#!/usr/bin/env python3
# SPDX-License-Identifier: MIT-0
"""wave3_rows.py -- turn the cme-2022-2024 evidence block into crate holiday rows.

The crate ships one static holiday table per CME product family, keyed by the
crate's own venue-local **trade date** (LAW-HOLIDAY-SCOPE, design memo D1). This
tool reads

  * the wave-3 evidence block (`holidays/cme-2022-2024.r2.json` when it exists,
    otherwise `holidays/cme-2022-2024.json`), and
  * the two raw-artifact indexes that resolve every document id to a file, a
    sha256, an Internet Archive capture and a tier,

and emits, for the eight crate families of the 2022-01-01 .. 2024-12-31 wave:

  tools/out/<family>.rows.rs       rows ready to paste into a `holidays!` table
  tools/out/<family>.evidence.md   the per-family, per-date evidence table
  tools/out/documents.md           one row per document id used by a row
  tools/out/DECISIONS.md           the grid, the conversions, the drops, questions
  tools/out/SUMMARY.json           counts and machine-readable decisions

Design rules implemented here (see DECISIONS.md for the per-rule narrative):

  * Only a date/status that changes an answer ships a row.  `normal` never
    ships; a printed instant equal to the family's ordinary instant for this
    era never ships.
  * Rows are keyed to the crate's venue-local trade date, never to the
    operator's event date.  For every family here the derived trade date is the
    block's date: a halt/close printed on date D ends the session whose final
    close is on D, and the flat-grid families never wrap.
  * `closed` -> `Closed`;  `early_close` -> `early_close(<printed close>)`;
    `late_open` -> `late_open(<printed open>)`;  `modified` -> decided from the
    printed instants;  `unknown` -> `Unsourced`.
  * An open printed at an instant other than the family's ordinary first open
    ships a `LateOpen` (or, with a same-date close, a
    `late_open_and_early_close`) on the trade date that open belongs to, which
    for a wrapped grid is the date the open is printed on (the value is earlier
    than the ordinary first open, so the cutoff lands on the trade date itself).
  * A date inside the coverage window that the operator's documents do not
    cover ships `Unsourced`, never silence.

Stdlib only.
"""

from __future__ import annotations

import argparse
import datetime as dt
import hashlib
import json
import os
import re
import sys

# --------------------------------------------------------------------------
# Constants: families, folds, coverage, grids
# --------------------------------------------------------------------------

#: The eight crate families this wave serves.
FAMILIES = (
    "globex_equity_index",
    "globex_energy",
    "globex_fx",
    "globex_grains",
    "globex_interest_rates",
    "globex_livestock",
    "globex_cryptocurrency",
    "globex_nikkei_225_dollar",
)

#: The two CME product groups the block carries that are not crate keys.  CME
#: publishes them under the joined labels "Grains & Oilseeds include Dairy" and
#: "Livestock include Lumber" in the 2025-2027 service block, so:
FOLD = {"dairy": "globex_grains", "lumber": "globex_livestock"}

#: The ten product-group names the block carries.
GROUPS = FAMILIES + tuple(FOLD)

#: The audited venue-local trade-date window of this wave.
WINDOW = (dt.date(2022, 1, 1), dt.date(2024, 12, 31))

SECONDS_PER_DAY = 86_400


class Grid:
    """The ordinary week of one product group in 2022-2024.

    ``wrapped`` is true when the trading day's first matching open falls on the
    preceding local date, which is what makes the crate's `LateOpen` rule
    interpret a value below ``first_open`` on the trade date itself.
    """

    def __init__(self, name, wrapped, first_open, close, description, source,
                 close_note="16:00 CT"):
        self.name = name
        self.close_note = close_note
        self.wrapped = wrapped
        self.first_open = first_open
        self.close = close  # callable(date) -> ordinary final-close ssm
        self.description = description
        self.source = source


def _close_1600(_date):
    return 16 * 3600


def _close_dairy(date):
    # Dairy's ordinary Friday close is 13:55 CT; every other weekday closes at
    # 16:00 CT (2022-new-years-holiday-schedule.xls "Dairy - Friday, December
    # 31: Close 13:55"; api_2024-07-03.json "2024-07-05: 13:55 closed").
    return (13 * 3600 + 55 * 60) if date.weekday() == 4 else 16 * 3600


WRAPPED_1700_1600 = (
    "one wrapped leg per trade date, 17:00 CT the previous evening into a "
    "16:00 CT close on the trade date"
)
GRAINS_GRID = (
    "19:00 CT the previous evening into 07:45 CT, then the 08:30-13:20 CT day "
    "session whose close is the trading day's final closing phase, then the "
    "14:30-16:00 CT post-close period"
)
LIVESTOCK_GRID = (
    "one flat Monday-Friday 08:30-13:05 CT session inside a single civil day, "
    "with an 08:00-08:30 CT pre-open and a 14:30-16:00 CT post-close period"
)

#: Ordinary instants (+ where they were read) per product group, 2022-2024.
GRIDS = {
    # src/calendar/schedules/futures/us/cme_group.rs: CME_PROFILE_DATED_CURRENT,
    # revision 2021-06-27 (CME Globex notice 20210621).
    "globex_equity_index": Grid(
        "globex_equity_index",
        True,
        17 * 3600,
        _close_1600,
        WRAPPED_1700_1600,
        "cme_group.rs CME_PROFILE_DATED_CURRENT (2021-06-27, CME Globex notice 20210621)",
    ),
    # energy_metals.rs: ENERGY_METALS_DATED_CURRENT, revision 2015-09-20.
    "globex_energy": Grid(
        "globex_energy",
        True,
        17 * 3600,
        _close_1600,
        WRAPPED_1700_1600,
        "energy_metals.rs ENERGY_METALS_DATED_CURRENT (2015-09-20, CME Globex notice 20150907)",
    ),
    # fx.rs: DATED_CURRENT, revision 2010-11-15 (grid unchanged since).
    "globex_fx": Grid(
        "globex_fx",
        True,
        17 * 3600,
        _close_1600,
        WRAPPED_1700_1600,
        "fx.rs DATED_CURRENT (2010-11-15, CME Globex notice 20101025)",
    ),
    # grains.rs: DATED_CURRENT, revision 2015-07-05 (CME SER-7395R).
    "globex_grains": Grid(
        "globex_grains",
        True,
        19 * 3600,
        _close_1600,
        GRAINS_GRID,
        "grains.rs DATED_CURRENT (2015-07-05, CME SER-7395R)",
    ),
    # interest_rates.rs: PROFILE_2011_10_02, revision 2011-10-02.
    "globex_interest_rates": Grid(
        "globex_interest_rates",
        True,
        17 * 3600,
        _close_1600,
        WRAPPED_1700_1600,
        "interest_rates.rs PROFILE_2011_10_02 (2011-10-02, CME Globex notice 20110926)",
    ),
    # livestock.rs: PROFILE_CURRENT, revision 2020-05-31 (CME SER-8599R).
    "globex_livestock": Grid(
        "globex_livestock",
        False,
        8 * 3600 + 30 * 60,
        _close_1600,
        LIVESTOCK_GRID,
        "livestock.rs PROFILE_CURRENT (2020-05-31, CME SER-8599R)",
    ),
    # cryptocurrency.rs: FIVE_DAY, revision 2017-12-17 (CME SER-8051R).
    "globex_cryptocurrency": Grid(
        "globex_cryptocurrency",
        True,
        17 * 3600,
        _close_1600,
        WRAPPED_1700_1600,
        "cryptocurrency.rs FIVE_DAY (2017-12-17, CME SER-8051R)",
    ),
    # cme_nikkei.rs: NKD_CURRENT, revision 2015-09-20.
    "globex_nikkei_225_dollar": Grid(
        "globex_nikkei_225_dollar",
        True,
        17 * 3600,
        _close_1600,
        WRAPPED_1700_1600,
        "cme_nikkei.rs NKD_CURRENT (2015-09-20, CME Globex notice 20150817)",
    ),
    # Folded groups, described on their own published grid.  Only their status
    # is folded; their instants are compared against their own clock.
    "dairy": Grid(
        "dairy",
        True,
        17 * 3600,
        _close_dairy,
        "one wrapped leg per trade date, 17:00 CT the previous evening into a "
        "16:00 CT close (13:55 CT on Fridays)",
        "block dairy rows (CME's Dairy product-group line); not a crate key",
        close_note="16:00 CT (13:55 CT on Fridays)",
    ),
    "lumber": Grid(
        "lumber",
        False,
        9 * 3600,
        lambda _date: 15 * 3600 + 5 * 60,
        "one flat Monday-Friday 09:00-15:05 CT session inside a single civil "
        "day, with a 06:00 CT pre-open",
        "block lumber rows (CME's Lumber product-group line); not a crate key",
        close_note="15:05 CT",
    ),
}

#: T2 document ids are `CME-SVC-<first fromEventDate in the queried window>`.
T2_ID_PREFIX = "CME-SVC-"

#: The time tokens a family's ordinary week prints, used to flag an entry whose
#: prose mentions an instant its own normal week does not have even though the
#: row it yields ships nothing.
ORDINARY_TOKENS = {
    "globex_equity_index": {"17:00", "16:45", "16:15", "16:00", "15:15", "08:30"},
    "globex_energy": {"17:00", "16:45", "16:15", "16:00"},
    "globex_fx": {"17:00", "16:45", "16:15", "16:00"},
    "globex_interest_rates": {"17:00", "16:45", "16:15", "16:00"},
    "globex_cryptocurrency": {"17:00", "16:45", "16:15", "16:00"},
    "globex_nikkei_225_dollar": {"17:00", "16:45", "16:15", "16:00"},
    "globex_grains": {"19:00", "16:45", "16:00", "14:30", "13:20", "13:30",
                      "08:30", "08:00", "07:45"},
    "globex_livestock": {"08:30", "13:05", "14:30", "16:00", "08:00"},
}

#: 08:30 CT, when every one of these families' US day session is under way.  A
#: printed instant below it falls in the small hours of the trade date, where a
#: close on this trade date and a reopen belonging to the previous local date
#: cannot be told apart from the token alone; the tool flags every such instant.
DAY_SESSION_START = 8 * 3600 + 30 * 60

MONTHS = {
    "jan": 1, "feb": 2, "mar": 3, "apr": 4, "may": 5, "jun": 6, "jul": 7,
    "aug": 8, "sep": 9, "oct": 10, "nov": 11, "dec": 12,
}
MONTH_RE = (
    r"(jan(?:uary)?|feb(?:ruary)?|mar(?:ch)?|apr(?:il)?|may|jun(?:e)?|"
    r"jul(?:y)?|aug(?:ust)?|sep(?:t(?:ember)?)?|oct(?:ober)?|nov(?:ember)?|"
    r"dec(?:ember)?)"
)
WEEKDAYS = ("mon", "tue", "wed", "thu", "fri", "sat", "sun")


# --------------------------------------------------------------------------
# Small parsing helpers
# --------------------------------------------------------------------------

def sha256_of(path):
    with open(path, "rb") as handle:
        return hashlib.sha256(handle.read()).hexdigest()


def parse_instant(text):
    """Return ``(ssm, zone, prose)`` for a printed instant cell.

    ``ssm`` is the leading ``HH:MM`` token converted to venue-local seconds
    since midnight, or ``None`` when the cell carries no clock token (the
    prose-only "the family's ordinary final-close instant ..." cells).  The
    trailing prose is returned untouched for the caller to classify.
    """
    if text is None or text.strip() in ("", "-"):
        return None, None, ""
    match = re.match(r"\s*(\d{1,2}):(\d{2})\s*([A-Za-z]{2,4})?\b(.*)$", text, re.S)
    if not match:
        return None, None, text.strip()
    hour, minute, zone, prose = match.groups()
    return int(hour) * 3600 + int(minute) * 60, zone, prose.strip()


def ssm_token(ssm):
    """Render a seconds-since-midnight value as the operator-style ``HH:MM``."""
    return "%02d:%02d" % (ssm // 3600, (ssm % 3600) // 60)


def ssm_expr(ssm):
    """Render a seconds-since-midnight value as a Rust expression."""
    hours, rem = divmod(ssm, 3600)
    minutes = rem // 60
    if minutes == 0:
        return "%d * 3_600" % hours
    return "%d * 3_600 + %d * 60" % (hours, minutes)


def rust_date(date):
    return "(%d, %d, %d)" % (date.year, date.month, date.day)


def iso(date):
    return date.isoformat()


def next_weekday(date):
    """The next Monday-Friday strictly after ``date``."""
    probe = date + dt.timedelta(days=1)
    while probe.weekday() >= 5:
        probe += dt.timedelta(days=1)
    return probe


def resolve_year(month, day, anchor):
    """Pick the year nearest ``anchor`` in which ``month``/``day`` exists."""
    best = None
    for year in (anchor.year - 1, anchor.year, anchor.year + 1):
        try:
            candidate = dt.date(year, month, day)
        except ValueError:
            continue
        if best is None or abs((candidate - anchor).days) < abs((best - anchor).days):
            best = candidate
    return best


def parse_open_parenthetical(text, anchor):
    """Return ``(local_date, trade_date, note)`` read off an open-instant cell.

    The block writes the local date the open is printed on, and sometimes the
    trade date it belongs to, inside the parentheses: ``17:00 CT (Mon 17 Jan)``,
    ``19:00 CT (Mon 17 Jan, for trade date Tue 18 Jan)``,
    ``17:00 CT (evening reopen for trade date Wed 5 July)``.
    """
    if not text:
        return None, None, None
    local = None
    trade = None
    match = re.search(
        r"\b(mon|tue|wed|thu|fri|sat|sun)[a-z]*\.?,?\s+(\d{1,2})\s+" + MONTH_RE,
        text,
        re.I,
    )
    if match:
        month = MONTHS[match.group(3)[:3].lower()]
        local = resolve_year(month, int(match.group(2)), anchor)
    match = re.search(
        r"for trade date\s+(?:(mon|tue|wed|thu|fri|sat|sun)[a-z]*\.?,?\s+)?"
        r"(\d{1,2})\s+" + MONTH_RE,
        text,
        re.I,
    )
    if match:
        month = MONTHS[match.group(3)[:3].lower()]
        trade = resolve_year(month, int(match.group(2)), anchor)
    return local, trade, text.strip()


# --------------------------------------------------------------------------
# Raw-artifact index
# --------------------------------------------------------------------------

#: The operator's own marker that a day session's prior-evening leg ran: for
#: grains the overnight electronic leg ends with a `07:45 paused` token, so a
#: trade date whose own printed text shows no `07:45` has lost that leg (the
#: shipped 2016-2018 grains convention: CME prints a `06:00 preopen` in place of
#: the usual `07:45 paused` / `08:00 preopen` pair).
PRIOR_LEG_MARKER = {"globex_grains": "07:45"}


def _label_matches(clause, anchor):
    """Yield ``(start, date)`` for every date label printed in ``clause``."""
    patterns = (
        re.compile(r"\b(\d{4})-(\d{2})-(\d{2})\b"),
        re.compile(r"\b(\d{1,2})\s+" + MONTH_RE, re.I),
        re.compile(r"\b" + MONTH_RE + r"\.?,?\s+(\d{1,2})\b", re.I),
    )
    for index, pattern in enumerate(patterns):
        for match in pattern.finditer(clause):
            groups = match.groups()
            if index == 0:
                yield match.start(), dt.date(int(groups[0]), int(groups[1]),
                                             int(groups[2]))
            elif index == 1:
                yield match.start(), resolve_year(
                    MONTHS[groups[1][:3].lower()], int(groups[0]), anchor)
            else:
                yield match.start(), resolve_year(
                    MONTHS[groups[0][:3].lower()], int(groups[1]), anchor)


def _is_editorial(text):
    """Whether a block fragment is the round-N note rather than operator bytes."""
    stripped = text.strip()
    if stripped.startswith("]") or stripped.startswith("[r"):
        return True
    return bool(re.search(r"pdftotext|bbox| x \d|x=\d", stripped))


def clause_for(entry, date, anchor, token=None):
    """The operator's own printed clause the trade date owns.

    Bracketed material in the block is editorial (the round-0/round-1/round-2
    notes about where a token sits in the bytes), so candidates made only of it
    are dropped; among the rest, clauses printing the row's own token are
    preferred, and the fullest such clause is returned.
    """
    clauses = []
    for clause in re.split(r";;", entry.get("verbatim", "") or ""):
        spans = [(match.start(), match.end())
                 for match in re.finditer(r"\[[^\]]*\]?", clause)]
        for start, label in _label_matches(clause, anchor):
            if label != date:
                continue
            if any(first <= start < last for first, last in spans):
                continue
            clauses.append(clause[start:].strip())
            break
    clauses = [clause for clause in clauses if not _is_editorial(clause)]
    if not clauses:
        return None
    if token:
        clock = token.split()[0]
        with_token = [clause for clause in clauses if clock in clause]
        if with_token:
            clauses = with_token
    return max(clauses,
               key=lambda text: len(re.sub(r"\[[^\]]*\]?", "", text).strip()))


def prior_leg_ran(entry, date, group, anchor):
    """Whether the trade date's own printed text shows its prior-evening leg ran.

    A clause may cover a run of dates (`Next entry WEDNESDAY, 5 JULY 2023: ...`),
    so the test is applied to the text that follows each occurrence of the
    trade date's own label.  Returns ``None`` when the block prints no label for
    that date at all.
    """
    marker = PRIOR_LEG_MARKER.get(group)
    if marker is None:
        return None
    verdicts = []
    for clause in re.split(r";;", entry.get("verbatim", "") or ""):
        for start, label in _label_matches(clause, anchor):
            if label == date:
                verdicts.append(marker in clause[start:])
    if not verdicts:
        return None
    return any(verdicts)


class IndexRow:
    def __init__(self, index_path, file, sha256, size, url, capture, tier, desc,
                 tier_source="row"):
        self.index_path = index_path
        self.file = file
        self.sha256 = sha256
        self.size = size
        self.url = url
        self.capture = capture
        self.tier = tier
        self.tier_source = tier_source
        self.desc = desc


INDEX_ROW_RE = re.compile(
    r"^\|\s*`([^`]+)`\s*\|\s*`([0-9a-f]{64})`\s*\|\s*(\d+)\s*\|\s*([^|]*?)\s*\|"
    r"\s*([^|]*?)\s*\|\s*([^|]*?)\s*\|\s*$"
)
CAPTURE_RE = re.compile(r"(\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2})Z")


def load_index(path, default_tier, questions, label):
    """Parse one `INDEX.md` into ``{file: IndexRow}``."""
    rows = {}
    with open(path, "r", encoding="utf-8") as handle:
        for line in handle:
            match = INDEX_ROW_RE.match(line.strip())
            if not match:
                continue
            file, sha, size, url, capture, desc = match.groups()
            tier = None
            tier_source = "row"
            tier_match = re.search(r"\bT([12])\b", desc)
            if tier_match:
                tier = "T" + tier_match.group(1)
            elif default_tier:
                tier = default_tier
                tier_source = "index-default"
            capture_match = CAPTURE_RE.search(capture)
            if not capture_match:
                continue
            if file in rows:
                questions.append(
                    "%s lists `%s` twice; the second row was ignored." % (label, file)
                )
                continue
            rows[file] = IndexRow(
                path, file, sha, int(size), url.strip(" <>"),
                capture_match.group(1) + "Z", tier, desc, tier_source,
            )
    return rows


def replay_url(capture, original):
    """The Internet Archive raw-replay URL for a saved capture."""
    stamp = re.sub(r"[^0-9]", "", capture)
    return "https://web.archive.org/web/%sid_/%s" % (stamp, original)


# --------------------------------------------------------------------------
# Derived row model
# --------------------------------------------------------------------------

class Row:
    """One derived crate row, before rendering."""

    def __init__(self, trade_date, group, kind, ssm_close=None, ssm_open=None,
                 entry=None, rationale="", role="", unsourced_reason="",
                 prior_leg=None, entry_date=None, printed_token=None,
                 clause=None):
        self.trade_date = trade_date
        self.group = group
        self.kind = kind  # Closed | EarlyClose | LateOpen | LateOpenAndEarlyClose | Unsourced
        self.ssm_close = ssm_close
        self.ssm_open = ssm_open
        self.entry = entry
        self.rationale = rationale
        self.role = role
        self.unsourced_reason = unsourced_reason
        self.prior_leg = prior_leg
        self.entry_date = entry_date if entry_date is not None else trade_date
        self.printed_token = printed_token
        self.clause = clause

    def key(self):
        return (self.kind, self.ssm_open, self.ssm_close)


def derive_group(entry, group, block_date, sink):
    """Derive every row one block entry states, plus its drops and questions."""
    grid = GRIDS[group]
    status = entry["status"]
    close_ssm, close_zone, close_prose = parse_instant(entry.get("close_instant"))
    open_ssm, open_zone, open_prose = parse_instant(entry.get("open_instant"))
    local_date, explicit_trade, open_note = parse_open_parenthetical(
        entry.get("open_instant") or "", block_date
    )
    if close_zone not in (None, "CT"):
        sink["questions"].append(
            "%s %s: close instant %r is printed in %s, not CT; not modelled."
            % (block_date, group, entry.get("close_instant"), close_zone)
        )
    if open_zone not in (None, "CT"):
        sink["questions"].append(
            "%s %s: open instant %r is printed in %s, not CT; not modelled."
            % (block_date, group, entry.get("open_instant"), open_zone)
        )

    ordinary_close = grid.close(block_date)

    def record_close():
        """Return (shipped, drop_reason)."""
        if status == "closed":
            return None, None
        if close_ssm is None:
            return None, "close cell carries no clock token (prose only)"
        if "ordinary" in close_prose.lower():
            return None, ("printed close is stated to be the family's ordinary "
                          "final close")
        if close_ssm == ordinary_close:
            return None, ("printed close %s equals the family's ordinary %s CT "
                          "final close" % (ssm_token(close_ssm), ssm_token(ordinary_close)))
        if close_ssm > ordinary_close:
            sink["questions"].append(
                "%s %s: printed close %s CT is later than the ordinary %s CT "
                "final close and no late-close kind exists."
                % (block_date, group, ssm_token(close_ssm), ssm_token(ordinary_close))
            )
            return None, "printed close is later than the ordinary final close"
        return close_ssm, None

    def record_open():
        """Return (trade_date, ssm, note, prior_leg) for a boundary-moving open."""
        if open_ssm is None:
            return None, None, "open cell carries no clock token", None
        if open_ssm == grid.first_open:
            return None, None, ("printed open %s CT is the family's ordinary first "
                                "open" % ssm_token(open_ssm)), None
        if grid.wrapped:
            if open_ssm > grid.first_open:
                target = next_weekday(local_date) if local_date else None
                note = ("printed open %s CT is later than the ordinary %s CT first "
                        "open, so the cutoff lands on the previous local date and "
                        "the trade date is the next business day"
                        % (ssm_token(open_ssm), ssm_token(grid.first_open)))
            else:
                target = local_date or block_date
                note = ("printed open %s CT is earlier than the ordinary %s CT first "
                        "open, so the cutoff lands on the trade date itself"
                        % (ssm_token(open_ssm), ssm_token(grid.first_open)))
        else:
            if open_ssm > grid.first_open:
                target = local_date
                note = ("printed open %s CT is later than the ordinary %s CT first "
                        "open" % (ssm_token(open_ssm), ssm_token(grid.first_open)))
            else:
                sink["questions"].append(
                    "%s %s: printed open %s CT is earlier than the ordinary %s CT "
                    "matching open on a flat grid; read as an order-entry-only "
                    "deviation, which the scalar holiday vocabulary cannot state."
                    % (block_date, group, ssm_token(open_ssm),
                       ssm_token(grid.first_open))
                )
                return None, None, "open is an order-entry-only deviation", None
        if target is None:
            sink["questions"].append(
                "%s %s: printed open %s CT moves the first open but the cell names "
                "no local date, so the trade date cannot be derived mechanically."
                % (block_date, group, ssm_token(open_ssm))
            )
            return None, None, "open cell names no local date", None
        if explicit_trade and explicit_trade != target:
            sink["questions"].append(
                "%s %s: the printed open's own trade-date token %s disagrees with "
                "the mechanically derived trade date %s."
                % (block_date, group, iso(explicit_trade), iso(target))
            )
            target = explicit_trade
        prior = prior_leg_ran(entry, target, group, block_date)
        return target, open_ssm, note, prior

    if status == "normal":
        if close_ssm is not None or open_ssm is not None:
            sink["questions"].append(
                "%s %s: status `normal` but the entry prints instants "
                "(close=%r open=%r); no row ships."
                % (block_date, group, entry.get("close_instant"),
                   entry.get("open_instant"))
            )
        sink["drops"].append((block_date, group, "status normal", entry))
        return

    if status == "unknown":
        sink["rows"].append(Row(block_date, group, "Unsourced", entry=entry,
                                role="unknown", entry_date=block_date,
                                unsourced_reason=entry.get("verbatim", "").strip()))
        return

    if status in ("early_close", "late_open", "modified"):
        shipped_close, drop_reason = record_close()
        if (shipped_close is not None and grid.wrapped
                and shipped_close < DAY_SESSION_START):
            # RULED (2026-09-15, the maintainer): an instant in the small hours
            # of a wrapped trade date is not this identity's final close.  The
            # block's only instance is the merged `Nikkei & BTIC` line on
            # 2022-05-30, whose printed 01:00 is the BTIC line's close; the
            # crate models the outright Nikkei, whose own close the document
            # does not state for that date.  Withhold rather than ship a
            # sourced-looking value that is wrong for the identity.
            sink["rows"].append(Row(
                block_date, group, "Unsourced", entry=entry,
                role="small-hours-close", entry_date=block_date,
                unsourced_reason=(
                    "the only instant printed for this identity on the date is "
                    "`%s CT`, in the small hours of the trade date: the sheet "
                    "merges the outright and BTIC Nikkei lines under one label "
                    "and prints the BTIC line's close, and no operator document "
                    "states the outright Nikkei's own close for the date"
                    % ssm_token(shipped_close)),
            ))
            sink["drops"].append((
                block_date, group,
                "RULED: the printed close %s CT falls in the small hours of the "
                "wrapped trade date (before the %s CT day session) and is the "
                "merged BTIC line's close, not this identity's, so the row ships "
                "`Unsourced` instead of `early_close(%s)`"
                % (ssm_token(shipped_close), ssm_token(DAY_SESSION_START),
                   ssm_expr(shipped_close)),
                entry))
            shipped_close = None
        if shipped_close is not None:
            sink["rows"].append(Row(
                block_date, group, "EarlyClose", ssm_close=shipped_close,
                entry=entry, role=status, entry_date=block_date,
                rationale="the printed final close %s CT is earlier than the "
                          "ordinary %s CT close" % (ssm_token(shipped_close),
                                                    ssm_token(ordinary_close)),
            ))
        elif drop_reason:
            sink["drops"].append((block_date, group, drop_reason, entry))
        target, shipped_open, note, prior = record_open()
        if shipped_open is not None and target is not None:
            token = _printed_token(entry.get("open_instant"), shipped_open)
            sink["rows"].append(Row(
                target, group, "LateOpen", ssm_open=shipped_open, entry=entry,
                role="open-of-" + status, rationale=note, prior_leg=prior,
                entry_date=block_date, printed_token=token,
                clause=clause_for(entry, target, block_date, token),
            ))
        elif note:
            sink["drops"].append(
                (target or block_date, group, "open: " + note, entry)
            )
        return

    if status == "closed":
        sink["rows"].append(Row(block_date, group, "Closed", entry=entry,
                                role="closed", entry_date=block_date,
                                rationale="CME prints no session running through "
                                          "this date"))
        target, shipped_open, note, prior = record_open()
        if shipped_open is not None and target is not None:
            token = _printed_token(entry.get("open_instant"), shipped_open)
            sink["rows"].append(Row(
                target, group, "LateOpen", ssm_open=shipped_open, entry=entry,
                role="reopen-after-closed", rationale=note, prior_leg=prior,
                entry_date=block_date, printed_token=token,
                clause=clause_for(entry, target, block_date, token),
            ))
        elif note:
            sink["drops"].append((target or block_date, group, "reopen: " + note, entry))
        return

    sink["questions"].append(
        "%s %s: status %r is outside the vocabulary this tool classifies."
        % (block_date, group, status)
    )


# --------------------------------------------------------------------------
# Merge and render
# --------------------------------------------------------------------------

def merge_family(group_rows, sink):
    """Collapse ``{trade_date: [Row]}`` into one row per trade date."""
    merged = {}
    conflicts = []
    for trade_date, rows in sorted(group_rows.items()):
        # A printed day-session open states a late first open only when the
        # trade date's own printed clause shows its prior-evening leg did not
        # run; where it did, that same open is the ordinary open inside a
        # session that is already running and moves no boundary.
        ran = [r for r in rows if r.kind == "LateOpen" and r.prior_leg is True]
        if ran:
            rows = [r for r in rows if r is not ran[0]]
            sink["drops"].append(
                (trade_date, ran[0].group,
                 "open: the trade date's own printed morning shows `%s`, so the "
                 "prior-evening leg ran and the `%s CT` open inside it moves no "
                 "boundary" % (PRIOR_LEG_MARKER.get(ran[0].group, "the leg"),
                               ssm_token(ran[0].ssm_open)),
                 ran[0].entry)
            )
        for row in rows:
            if row.kind == "LateOpen" and row.prior_leg is None:
                sink["questions"].append(
                    "%s %s: the block prints an open at %s CT for this trade date "
                    "but no clause owned by the date, so whether the prior-evening "
                    "leg ran could not be read; the row ships and needs a ruling."
                    % (trade_date, row.group, ssm_token(row.ssm_open))
                )
        if not rows:
            continue
        closed = any(r.kind == "Closed" for r in rows)
        unsourced = [r for r in rows if r.kind == "Unsourced"]
        closes = [r for r in rows if r.kind == "EarlyClose"]
        opens = [r for r in rows if r.kind == "LateOpen"]
        close_values = {r.ssm_close for r in closes}
        open_values = {r.ssm_open for r in opens}
        if len(close_values) > 1:
            conflicts.append((trade_date, "two different early-close instants: %s"
                              % sorted(ssm_token(v) for v in close_values)))
            continue
        if len(open_values) > 1:
            conflicts.append((trade_date, "two different late-open instants: %s"
                              % sorted(ssm_token(v) for v in open_values)))
            continue
        close = closes[0] if closes else None
        opens.sort(key=lambda r: (r.clause is None, r.entry_date))
        late = opens[0] if opens else None
        if closed and (close or late):
            conflicts.append((trade_date, "a closure and a boundary move on the "
                                         "same trade date"))
            merged[trade_date] = closed_row(rows)
            continue
        if unsourced and (close or late or closed):
            conflicts.append((trade_date, "an `Unsourced` record and a scheduling "
                                         "row on the same trade date"))
            merged[trade_date] = (close or late or closed_row(rows))
            continue
        if unsourced:
            merged[trade_date] = unsourced[0]
            continue
        if closed:
            merged[trade_date] = closed_row(rows)
        elif close and late:
            merged[trade_date] = Row(trade_date, late.group, "LateOpenAndEarlyClose",
                                     ssm_close=close.ssm_close, ssm_open=late.ssm_open,
                                     entry=close.entry, role="combined",
                                     entry_date=close.entry_date,
                                     printed_token=late.printed_token,
                                     clause=late.clause,
                                     rationale=late.rationale + "; " + close.rationale)
        elif close:
            merged[trade_date] = close
        elif late:
            merged[trade_date] = late
    return merged, conflicts


def closed_row(rows):
    for row in rows:
        if row.kind == "Closed":
            return row
    return rows[0]


#: The operator's own closure phrases, in the order the evidence table should
#: prefer them.  A token taken from here is an exact substring of the block's
#: quoted bytes, so it is never an invented phrase.
CLOSURE_PHRASES = (
    "Globex Closed", "Globex closed", "no entries", "no entry",
    "events []", "no time events", "no session printed", "column is EMPTY",
    "no outright Nikkei line", "no Nikkei-specific line", "remain closed until",
)


def closure_token(entry):
    """The operator's closure phrase for a row that prints no clock instant."""
    text = json.dumps(entry)
    for phrase in CLOSURE_PHRASES:
        if phrase in text:
            return phrase
    return entry.get("status") or "closed"


def instant_cell(row):
    """The `instant as printed` cell: operator tokens only, never invented."""
    if row.kind == "Unsourced":
        return "`unknown`"
    entry = row.entry or {}
    tokens = []
    if row.ssm_open is not None:
        tokens.append(_printed_token(entry.get("open_instant"), row.ssm_open))
    if row.ssm_close is not None:
        tokens.append(_printed_token(entry.get("close_instant"), row.ssm_close))
    if tokens:
        return " / ".join("`%s`" % token for token in tokens)
    return "`%s`" % closure_token(entry)


def _printed_token(cell, ssm):
    """The exact printed fragment carrying ``ssm`` in ``cell``, else ``HH:MM CT``."""
    if cell:
        match = re.search(r"\d{1,2}:\d{2}\s*CT", cell)
        if match and parse_instant(match.group(0))[0] == ssm:
            return match.group(0)
    return "%s CT" % ssm_token(ssm)


def kind_rust(row):
    if row.kind == "Closed":
        return "Closed"
    if row.kind == "Unsourced":
        return "Unsourced"
    if row.kind == "EarlyClose":
        return "early_close(%s)" % ssm_expr(row.ssm_close)
    if row.kind == "LateOpen":
        return "late_open(%s)" % ssm_expr(row.ssm_open)
    return "late_open_and_early_close(%s, %s)" % (
        ssm_expr(row.ssm_open), ssm_expr(row.ssm_close))


def kind_words(row):
    if row.kind == "Closed":
        return "closed: no trade date"
    if row.kind == "Unsourced":
        return "unsourced: no operator document covers this date"
    if row.kind == "EarlyClose":
        return "early close %s CT" % ssm_token(row.ssm_close)
    if row.kind == "LateOpen":
        return "late open %s CT" % ssm_token(row.ssm_open)
    return "late open %s CT and early close %s CT" % (
        ssm_token(row.ssm_open), ssm_token(row.ssm_close))


def kind_label(row):
    if row.kind == "Closed":
        return "closed"
    if row.kind == "Unsourced":
        return "unsourced"
    if row.kind == "EarlyClose":
        return "early close"
    if row.kind == "LateOpen":
        return "late open"
    return "late open and early close"


def derived_from(row, family, grid):
    """One sentence saying how the trade date and the kind follow."""
    date = row.trade_date
    if row.kind == "Unsourced":
        reason = row.unsourced_reason or ""
        reason = re.split(r"\s+-\s+|\[", reason)[0].strip()
        reason = reason[:200].rstrip(" ,;:")
        return (
            "The operator's documents do not cover %s for this family (%s), and "
            "inside the audited window a date with no row reads as audited normal, "
            "so the row is `Unsourced`, which clips nothing." % (iso(date), reason)
        )
    if row.kind == "Closed":
        if grid.wrapped:
            return (
                "CME prints the closure for this date, so the session whose final "
                "close would have fallen here — the one that opened `%s CT` the "
                "previous evening — is removed with it, and the crate's trade date "
                "is the operator's event date."
                % ssm_token(grid.first_open)
            )
        return (
            "CME prints the closure for this date, and the crate's session lies "
            "inside this one civil day, so the operator's event date is the "
            "crate's trade date."
        )
    if row.kind == "EarlyClose":
        if grid.wrapped:
            return (
                "CME prints `%s CT` as this date's own final close; the session "
                "that opened `%s CT` the previous evening is clipped there, and "
                "the crate's trade date is the local date that close falls on — "
                "the operator's event date."
                % (ssm_token(row.ssm_close), ssm_token(grid.first_open))
            )
        return (
            "CME prints `%s CT` as this date's own final close inside this one "
            "civil day, so the operator's event date is the crate's trade date."
            % ssm_token(row.ssm_close)
        )
    open_note = (
        "`%s` is earlier than this family's ordinary `%s CT` first open, so the "
        "cutoff lands on the trade date itself rather than the preceding local "
        "date." % (ssm_token(row.ssm_open), ssm_token(grid.first_open))
    )
    if row.kind == "LateOpen":
        if grid.wrapped:
            return (
                "The prior-evening leg of trade date %s did not run: CME's next "
                "open is the day session's own `%s CT` on that date, and %s"
                % (iso(date), ssm_token(row.ssm_open), open_note)
            )
        return (
            "CME's next open for trade date %s is `%s CT`, later than this "
            "family's ordinary `%s CT` open inside the same civil day."
            % (iso(date), ssm_token(row.ssm_open), ssm_token(grid.first_open))
        )
    return (
        "CME withdrew the prior-evening leg and printed the day session's own "
        "`%s CT` open beside the `%s CT` final close on trade date %s, so both "
        "boundaries move; %s"
        % (ssm_token(row.ssm_open), ssm_token(row.ssm_close), iso(date), open_note)
    )


# --------------------------------------------------------------------------
# Main
# --------------------------------------------------------------------------

def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument(
        "--research",
        default="/Users/agedvagabond/Developer/exchange-hours-research",
        help="research store root",
    )
    parser.add_argument("--block", default="auto",
                        help="evidence block path, or `auto` to prefer r2")
    parser.add_argument("--out", default=None, help="output directory")
    parser.add_argument("--stdout", action="store_true", default=True,
                        help="print the emitted blocks (default)")
    parser.add_argument("--quiet", action="store_true", help="suppress stdout")
    args = parser.parse_args(argv)

    root = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
    out_dir = args.out or os.path.join(root, "tools", "out")
    research = args.research
    holidays = os.path.join(research, "holidays")

    r2 = os.path.join(holidays, "cme-2022-2024.r2.json")
    r1 = os.path.join(holidays, "cme-2022-2024.json")
    if args.block != "auto":
        block_path = args.block
    else:
        block_path = r2 if os.path.exists(r2) else r1

    questions = []
    drops = []
    sink = {"rows": [], "drops": drops, "questions": questions}

    with open(block_path, "r", encoding="utf-8") as handle:
        block = json.load(handle)

    index_paths = [
        (os.path.join(holidays, "raw", "cme-2022-2024", "INDEX.md"), None,
         "raw/cme-2022-2024/INDEX.md"),
        (os.path.join(holidays, "raw", "cme-2022-2024-fix", "INDEX.md"), "T2",
         "raw/cme-2022-2024-fix/INDEX.md"),
    ]
    index = {}
    index_meta = []
    for path, default_tier, label in index_paths:
        if not os.path.exists(path):
            questions.append("INDEX %s is absent." % label)
            continue
        rows = load_index(path, default_tier, questions, label)
        index_meta.append({
            "label": label,
            "path": path,
            "sha256": sha256_of(path),
            "rows": len(rows),
        })
        for file, row in rows.items():
            index.setdefault(file, row)

    # ---- documents ------------------------------------------------------
    documents = {}

    def resolve(file, capture, tier, where):
        """Resolve one block document string to its INDEX row and document id."""
        row = index.get(file)
        if row is None:
            questions.append(
                "%s cites `%s`, which appears in no raw INDEX." % (where, file)
            )
            return None
        url = row.url
        if row.tier and tier and row.tier != tier:
            questions.append(
                "%s cites `%s` as %s; the INDEX gives it %s."
                % (where, file, tier, row.tier)
            )
        if row.capture != capture:
            questions.append(
                "%s cites `%s` at capture %s; the INDEX records %s."
                % (where, file, capture, row.capture)
            )
            capture = row.capture
        if url.startswith("("):
            questions.append(
                "%s cites `%s`, whose INDEX row carries no replayable URL (%r)."
                % (where, file, url)
            )
        if tier == "T2":
            match = re.search(r"fromEventDate=(\d{4}-\d{2}-\d{2})", url)
            if not match:
                questions.append(
                    "%s cites the T2 artifact `%s`, whose URL carries no "
                    "fromEventDate token, so its document id cannot follow the "
                    "CME-SVC convention." % (where, file)
                )
                return None
            doc_id = T2_ID_PREFIX + match.group(1)
        else:
            doc_id = "%s @%s" % (file, capture)
        documents.setdefault(doc_id, {
            "id": doc_id,
            "file": file,
            "url": url,
            "capture": capture,
            "tier": row.tier or tier,
            "tier_source": row.tier_source,
            "sha256": row.sha256,
            "bytes": row.size,
            "desc": row.desc,
            "index": row.index_path,
        })
        return doc_id

    # ---- derive ---------------------------------------------------------
    entries_by_date = {}
    for holiday in block["holidays"]:
        block_date = dt.date.fromisoformat(holiday["date"])
        entries_by_date[block_date] = holiday

    group_rows = {group: {} for group in GROUPS}
    for block_date, holiday in sorted(entries_by_date.items()):
        for entry in holiday["families"]:
            group = entry["family"]
            if group not in group_rows:
                questions.append(
                    "%s: unknown product group %r in the block." % (block_date, group)
                )
                continue
            before = len(sink["rows"])
            derive_group(entry, group, block_date, sink)
            for row in sink["rows"][before:]:
                group_rows[group].setdefault(row.trade_date, []).append(row)

    merged_all = {}
    conflicts = []
    for group in GROUPS:
        merged_group, group_conflicts = merge_family(group_rows[group], sink)
        for date, why in group_conflicts:
            conflicts.append((group, date, why))
        merged_all[group] = merged_group
    family_rows = {family: merged_all[family] for family in FAMILIES}

    # ---- folds ----------------------------------------------------------
    fold_report = []
    for group, family in FOLD.items():
        merged_fold, merged_family = merged_all[group], merged_all[family]
        for date in sorted(set(merged_fold) | set(merged_family)):
            a = merged_fold.get(date)
            b = merged_family.get(date)
            a_key = a.key() if a else None
            b_key = b.key() if b else None
            if a_key == b_key:
                status = "agree"
            elif a is None:
                status = "family-only"
            elif b is None:
                status = "group-only"
            else:
                status = "differ"
            fold_report.append({
                "group": group,
                "family": family,
                "trade_date": iso(date),
                "folded": (kind_words(a) if a else None),
                "crate_family": (kind_words(b) if b else None),
                "status": status,
            })

    # ---- resolve documents ---------------------------------------------
    for group in FOLD:
        for date, rows in sorted(group_rows[group].items()):
            for row in rows:
                row.doc_id = resolve(
                    row.entry["document"].split(" @ ")[0].strip(),
                    row.entry["document"].split(" @ ")[1].split(" ")[0].strip(),
                    row.entry["tier"],
                    "%s %s (%s)" % (date, group, row.entry["family"]),
                )
    for family in FAMILIES:
        for date, row in sorted(family_rows[family].items()):
            row.doc_id = resolve(
                row.entry["document"].split(" @ ")[0].strip(),
                row.entry["document"].split(" @ ")[1].split(" ")[0].strip(),
                row.entry["tier"],
                "%s %s" % (date, family),
            )

    # Dates the block does not carry at all: every family ships Unsourced.
    covered_dates = set(entries_by_date)
    missing = [item for item in block.get("missing", [])]
    absent = []
    for text in missing:
        match = re.search(r"^\s*(\d{4})\s+.*?\((\w+day)\s+(\d{1,2})\s+(\w+)\s+(\d{4})\)",
                          text)
        if match:
            absent.append((dt.date(int(match.group(5)),
                                   MONTHS[match.group(4)[:3].lower()],
                                   int(match.group(3))), text))
    absent = [(date, text) for date, text in absent if date not in covered_dates]
    # Document the absence with the operator's own machine channel for that
    # window: the negative controls saved in raw/cme-2022-2024-fix/.
    neg_files = {
        dt.date(2023, 1, 16): "neg_api_2023-01-15.json",
        dt.date(2023, 2, 20): "neg_api_2023-02-19.json",
        dt.date(2023, 4, 7): "neg_api_2023-04-06.json",
    }
    unsourced_from_missing = []
    for date, text in absent:
        file = neg_files.get(date)
        if file is None:
            questions.append(
                "%s is not covered by the block and no negative control is "
                "recorded for it; no Unsourced row could be cited." % iso(date)
            )
            continue
        row = index.get(file)
        if row is None:
            questions.append(
                "%s: the negative control `%s` is not in any INDEX." % (iso(date), file)
            )
            continue
        doc_id = resolve(file, row.capture, "T2", "missing date %s" % iso(date))
        for family in FAMILIES:
            if date in family_rows[family]:
                continue
            family_rows[family][date] = Row(
                date, family, "Unsourced",
                entry={"document": "%s @ %s" % (file, row.capture),
                       "tier": "T2", "status": "unknown",
                       "verbatim": text},
                role="absent-from-block", entry_date=date,
                unsourced_reason=text,
            )
            family_rows[family][date].doc_id = doc_id
        unsourced_from_missing.append({
            "date": iso(date),
            "document": doc_id,
            "reason": text,
        })

    # ---- self checks ----------------------------------------------------
    checks = []
    for family in FAMILIES:
        dates = sorted(family_rows[family])
        if dates != sorted(set(dates)):
            checks.append("FAIL %s: duplicate trade dates" % family)
        for earlier, later in zip(dates, dates[1:]):
            if not earlier < later:
                checks.append("FAIL %s: rows not strictly ascending at %s"
                              % (family, iso(earlier)))
        for date in dates:
            if not (WINDOW[0] <= date <= WINDOW[1]):
                checks.append("FAIL %s: row %s outside the wave window"
                              % (family, iso(date)))
        for date, row in family_rows[family].items():
            if row.kind == "Unsourced":
                continue
            if row.ssm_close is not None and not (0 <= row.ssm_close <= SECONDS_PER_DAY):
                checks.append("FAIL %s %s: close out of range" % (family, iso(date)))
            if row.ssm_open is not None and not (0 <= row.ssm_open < SECONDS_PER_DAY):
                checks.append("FAIL %s %s: open out of range" % (family, iso(date)))
            if not row.doc_id:
                checks.append("FAIL %s %s: no document id" % (family, iso(date)))
        for date, row in family_rows[family].items():
            text = json.dumps(row.entry)
            for token in re.findall(r"`([^`]+)`", instant_cell(row)):
                if token not in text and token not in ("closed", "unknown"):
                    checks.append(
                        "FAIL %s %s: printed token %r is not in the block's own "
                        "text for this entry" % (family, iso(date), token)
                    )

    used_docs = {}
    for family in FAMILIES:
        for date, row in family_rows[family].items():
            used_docs[row.doc_id] = documents[row.doc_id]

    # ---- questions for the maintainer --------------------------------
    for_review = build_questions(family_rows, fold_report, drops, entries_by_date,
                                 unsourced_from_missing, used_docs, index, block)

    # ---- render ---------------------------------------------------------
    os.makedirs(out_dir, exist_ok=True)
    written = {}
    header = ("// %s -- venue-local trade dates 2022-01-01 .. 2024-12-31 "
              "(LAW-HOLIDAY-SCOPE).\n"
              "// Generated by tools/wave3_rows.py from %s\n"
              "// (%s of the wave-3 evidence block).\n"
              "// Evidence: docs/evidence/%s.md\n"
              "// coverage: [(2022, 1, 1) ..= (2024, 12, 31)]\n"
              "// imports: use super::fences::{early_close, late_open, "
              "late_open_and_early_close}; "
              "use super::{EvidenceTier::{T1, T2}, HolidayKind::{Closed, Unsourced}, "
              "HolidayTable, holidays};\n")
    for family in FAMILIES:
        lines = [header % (family, os.path.basename(block_path),
                           sha256_of(block_path)[:12], family)]
        for date in sorted(family_rows[family]):
            row = family_rows[family][date]
            line = ("        // %s - %s - %s - %s.\n"
                    % (iso(date), row.entry["tier"], row.doc_id, kind_words(row)))
            if len(line) > 100:
                line = ("        // %s - %s - %s\n        //   - %s.\n"
                        % (iso(date), row.entry["tier"], row.doc_id, kind_words(row)))
            lines.append(line)
            lines.append("        (%d, %d, %d, %s, %s, \"%s\"),\n"
                         % (date.year, date.month, date.day, kind_rust(row),
                            row.entry["tier"], row.doc_id))
        path = os.path.join(out_dir, "%s.rows.rs" % family)
        with open(path, "w", encoding="utf-8") as handle:
            handle.write("".join(lines))
        written[path] = len(family_rows[family])

    # documents.md
    doc_lines = [
        "<!-- SPDX-License-Identifier: MIT-0 -->\n\n",
        "Every id below resolves to one saved artifact behind this wave's\n",
        "holiday rows. The T1 ids are CME Group's own published holiday schedules\n",
        "(2022 per-asset-class workbooks and 2023/2024 one-pagers), retrieved\n",
        "through the Internet Archive and saved; the T2 ids are responses of CME's\n",
        "own `trading-hours-by-product` service, read as bytes and saved. Each id\n",
        "resolves to the URL it was read at, the capture time in UTC, the tier and\n",
        "the sha256; byte counts and per-artifact notes are in the research store's\n",
        "`holidays/raw/cme-2022-2024/INDEX.md` and\n",
        "`holidays/raw/cme-2022-2024-fix/INDEX.md`.\n\n",
        "| Document | File | Replay or service URL | Capture or retrieval, UTC | Tier | sha256 |\n",
        "|---|---|---|---|---|---|\n",
    ]
    for doc_id in sorted(used_docs, key=lambda d: (used_docs[d]["capture"],
                                                   used_docs[d]["file"])):
        doc = used_docs[doc_id]
        url = doc["url"]
        replay = replay_url(doc["capture"], url) if url.startswith("http") else url
        doc_lines.append("| `%s` | `%s` | <%s> | archive capture %s | %s | `%s` |\n"
                         % (doc_id, doc["file"], replay, doc["capture"],
                            doc["tier"], doc["sha256"]))
    with open(os.path.join(out_dir, "documents.md"), "w", encoding="utf-8") as handle:
        handle.write("".join(doc_lines))

    # ROWS.json -- the machine-readable plan the encoder works from.
    def row_object(row):
        source = "block %s %s" % (iso(row.entry_date), row.group)
        line = row.entry.get("line")
        if line:
            source += " (%s)" % line
        if row.entry_date != row.trade_date:
            source = "derived: reopen printed in %s" % source
        printed = instant_cell(row).replace("`", "")
        return {
            "date": iso(row.trade_date),
            "kind": row.kind,
            "open_ssm": row.ssm_open,
            "close_ssm": row.ssm_close,
            "tier": row.entry["tier"],
            "document": row.doc_id,
            "reason": kind_words(row),
            "printed": printed,
            "source": source,
        }

    rows_json = {
        "task": block.get("task"),
        "tool": "tools/wave3_rows.py",
        "block": {"path": os.path.basename(block_path),
                  "sha256": sha256_of(block_path)},
        "window": [iso(WINDOW[0]), iso(WINDOW[1])],
        "coverage_clause": "[(2022, 1, 1) ..= (2024, 12, 31)]",
        "families": [
            {
                "family": family,
                "coverage": [[2022, 1, 1, 2024, 12, 31]],
                "rows": [row_object(family_rows[family][date])
                         for date in sorted(family_rows[family])],
            }
            for family in FAMILIES
        ],
        "documents": [
            {
                "id": doc_id,
                "file": used_docs[doc_id]["file"],
                "sha256": used_docs[doc_id]["sha256"],
                "url": replay_url(used_docs[doc_id]["capture"],
                                  used_docs[doc_id]["url"]),
                "capture_utc": used_docs[doc_id]["capture"],
                "tier": used_docs[doc_id]["tier"],
            }
            for doc_id in sorted(used_docs,
                                 key=lambda d: (used_docs[d]["capture"],
                                                used_docs[d]["file"]))
        ],
        "counts": {},
    }
    rows_per_family = {}
    rows_per_kind = {}
    rows_per_tier = {}
    rows_per_year = {}
    for family in FAMILIES:
        rows_per_family[family] = 0
        for date, row in family_rows[family].items():
            rows_per_family[family] += 1
            rows_per_kind[row.kind] = rows_per_kind.get(row.kind, 0) + 1
            tier = row.entry["tier"]
            rows_per_tier[tier] = rows_per_tier.get(tier, 0) + 1
            year = str(date.year)
            rows_per_year[year] = rows_per_year.get(year, 0) + 1
    rows_json["counts"] = {
        "total": sum(rows_per_family.values()),
        "rows_per_family": rows_per_family,
        "rows_per_kind": rows_per_kind,
        "rows_per_tier": rows_per_tier,
        "rows_per_year": rows_per_year,
        "documents": len(used_docs),
    }
    rows_json["questions_for_review"] = for_review
    with open(os.path.join(out_dir, "ROWS.json"), "w", encoding="utf-8") as handle:
        json.dump(rows_json, handle, indent=1, sort_keys=True)
        handle.write("\n")

    # per-family evidence tables
    for family in FAMILIES:
        grid = GRIDS[family]
        t1_rows = sum(1 for r in family_rows[family].values()
                      if r.entry["tier"] == "T1")
        t2_rows = sum(1 for r in family_rows[family].values()
                      if r.entry["tier"] == "T2")
        lines = [
            "<!-- SPDX-License-Identifier: MIT-0 -->\n\n",
            "### Holidays, 2022-2024\n\n",
            "Tier: **T1** for the %d rows read from CME's own published holiday\n"
            % t1_rows,
            "schedules, **T2** for the %d read from its `trading-hours-by-product`\n"
            % t2_rows,
            "service.\n\n",
            "**Coverage:** 2022-01-01..2024-12-31 (inclusive venue-local trade\n",
            "dates). Inside the window a date with no row is audited normal; the\n",
            "family's ordinary week over this wave is %s, read from `%s`.\n"
            % (grid.description, grid.source),
            "Rows are keyed by the crate's venue-local trade date, never by CME's\n",
            "event date; the conversion is stated per row in `derived from`.\n\n",
        ]
        years = sorted({row.trade_date.year for row in family_rows[family].values()})
        no_row_dates = sorted(
            date for date in family_rows[family]
            if family_rows[family][date].kind == "Unsourced"
        )
        for year in years:
            lines.append("### %d\n\n" % year)
            lines.append("| trade date | kind | instant as printed | document | "
                         "tier | derived from |\n")
            lines.append("|---|---|---|---|---|---|\n")
            for date in sorted(family_rows[family]):
                if date.year != year:
                    continue
                row = family_rows[family][date]
                lines.append("| %s | %s | %s | `%s` | %s | %s |\n"
                             % (iso(date), kind_label(row), instant_cell(row),
                                row.doc_id, row.entry["tier"],
                                derived_from(row, family, grid)))
            lines.append("\n")
        if no_row_dates:
            lines.append("**`Unsourced` dates.** %s — inside the audited window, so\n"
                         "they ship `Unsourced` rather than reading as audited normal.\n\n"
                         % ", ".join(iso(d) for d in no_row_dates))
        family_drops = {}
        for date, group, reason, entry in drops:
            if group != family:
                continue
            family_drops.setdefault(reason, []).append(date)
        touched = [item for item in for_review if family in item["families"]]
        if touched:
            lines.append("**Interpretive steps and open questions.** Every rule applied\n"
                         "to this family's rows is stated in `tools/out/DECISIONS.md`;\n"
                         "these bear on this family and need the maintainer's ruling:\n\n")
            for item in touched:
                lines.append("- **Q%d.** %s Default: %s\n"
                             % (item["n"], item["title"], item["tool_default"]))
            lines.append("\n")
        if family_drops:
            lines.append("**No row, and why.** A status that changes no answer ships\n"
                         "nothing. For this family the block's entries were read as:\n\n")
            for reason in sorted(family_drops):
                dates = sorted(set(family_drops[reason]))
                lines.append("- %s — %s\n"
                             % (reason, ", ".join(iso(d) for d in dates)))
            lines.append("\n")
        with open(os.path.join(out_dir, "%s.evidence.md" % family), "w",
                  encoding="utf-8") as handle:
            handle.write("".join(lines))

    # SUMMARY.json
    summary = {
        "tool": "tools/wave3_rows.py",
        "block": {
            "path": block_path,
            "sha256": sha256_of(block_path),
            "task": block.get("task"),
        },
        "indexes": index_meta,
        "window": [iso(WINDOW[0]), iso(WINDOW[1])],
        "coverage_clause": "[(2022, 1, 1) ..= (2024, 12, 31)]",
        "families": {},
        "family_totals": {},
        "kind_totals": {},
        "year_totals": {},
        "documents": {
            "count": len(used_docs),
            "t1": sum(1 for d in used_docs.values() if d["tier"] == "T1"),
            "t2": sum(1 for d in used_docs.values() if d["tier"] == "T2"),
            "ids": sorted(used_docs),
        },
        "unsourced_dates_per_family": {},
        "unsourced_dates_from_missing_block": unsourced_from_missing,
        "dropped_statuses": {},
        "dropped_status_entry_counts": {},
        "fold_conflicts": [item for item in fold_report
                           if item["status"] in ("differ", "group-only")],
        "fold_family_only": [item for item in fold_report
                             if item["status"] == "family-only"],
        "fold_agreements": sum(1 for item in fold_report
                               if item["status"] == "agree"),
        "family_conflicts": [
            {"family": family, "trade_date": iso(date), "why": why}
            for family, date, why in conflicts
        ],
        "questions": questions,
        "checks": checks,
        "checks_passed": not [c for c in checks if c.startswith("FAIL")],
        "rerun_byte_identical": (
            "verified by tools/check_wave3.py, which re-runs this generator "
            "into a scratch directory and compares every emitted byte"),
        "notes": [
            "Dropped statuses are recorded per (date, group) entry.",
            "`family_totals` counts shipped rows; a row may be stated by more "
            "than one block entry, which is recorded in `family_conflicts` when "
            "the entries disagree.",
        ],
    }
    kind_totals = {}
    year_totals = {}
    drop_counter = {}
    drop_entry_counter = {}
    for family in FAMILIES:
        rows = family_rows[family]
        per_kind = {}
        per_year = {}
        for date, row in rows.items():
            per_kind[row.kind] = per_kind.get(row.kind, 0) + 1
            year = str(date.year)
            per_year.setdefault(year, {})
            per_year[year][row.kind] = per_year[year].get(row.kind, 0) + 1
            kind_totals[row.kind] = kind_totals.get(row.kind, 0) + 1
            year_totals.setdefault(year, {})
            year_totals[year][row.kind] = year_totals[year].get(row.kind, 0) + 1
        summary["families"][family] = {
            "rows": len(rows),
            "per_kind": per_kind,
            "per_year": per_year,
            "first": iso(min(rows)),
            "last": iso(max(rows)),
        }
        summary["family_totals"][family] = len(rows)
        summary["unsourced_dates_per_family"][family] = [
            iso(d) for d, r in sorted(rows.items()) if r.kind == "Unsourced"
        ]
    for date, group, reason, entry in drops:
        key = "%s (%s)" % (reason, entry["status"])
        drop_counter[key] = drop_counter.get(key, 0) + 1
        drop_entry_counter.setdefault(key, []).append(
            {"date": iso(date), "group": group}
        )
    summary["kind_totals"] = kind_totals
    summary["year_totals"] = year_totals
    summary["dropped_statuses"] = drop_counter
    summary["dropped_status_entry_counts"] = {
        key: len(value) for key, value in drop_entry_counter.items()
    }
    summary["dropped_entries"] = drop_entry_counter
    with open(os.path.join(out_dir, "SUMMARY.json"), "w", encoding="utf-8") as handle:
        json.dump(summary, handle, indent=1, sort_keys=True)
        handle.write("\n")

    summary["questions_for_review"] = for_review
    with open(os.path.join(out_dir, "SUMMARY.json"), "w", encoding="utf-8") as handle:
        json.dump(summary, handle, indent=1, sort_keys=True)
        handle.write("\n")

    write_decisions(out_dir, summary, family_rows, fold_report, drops, used_docs,
                    checks, for_review, entries_by_date)

    if not args.quiet:
        for family in FAMILIES:
            print("=" * 78)
            print("== %s: %d rows" % (family, len(family_rows[family])))
            print("=" * 78)
            with open(os.path.join(out_dir, "%s.rows.rs" % family),
                      encoding="utf-8") as handle:
                print(handle.read(), end="")
        print("=" * 78)
        print("documents: %d (%d T1, %d T2)"
              % (len(used_docs),
                 sum(1 for d in used_docs.values() if d["tier"] == "T1"),
                 sum(1 for d in used_docs.values() if d["tier"] == "T2")))
        print("rows: %d" % sum(len(rows) for rows in family_rows.values()))
        print("kinds: %s" % json.dumps(kind_totals, sort_keys=True))
        print("drops: %s" % json.dumps(drop_counter, sort_keys=True))
        print("checks: %s" % ("PASS" if summary["checks_passed"]
                              else "; ".join(checks)))
        print("questions for review: %d" % len(for_review))
        for item in for_review:
            print("  Q%d. %s" % (item["n"], item["title"]))
    return 0


def build_questions(family_rows, fold_report, drops, entries_by_date,
                    unsourced_from_missing, used_docs, index, block):
    """Assemble the numbered questions the maintainer has to rule on."""
    items = []

    def ask(title, detail, default, families=None):
        items.append({"n": len(items) + 1, "title": title, "detail": detail,
                      "tool_default": default,
                      "families": families or sorted(set(FAMILIES))})

    # 1. printed instants in the small hours of a wrapped trade date, i.e.
    #    earlier than the ordinary day session begins and therefore ambiguous
    #    between a close on this trade date and a reopen that belongs to the
    #    previous local date.
    for family in FAMILIES:
        grid = GRIDS[family]
        for date, row in sorted(family_rows[family].items()):
            if (row.kind == "EarlyClose" and grid.wrapped
                    and row.ssm_close is not None
                    and row.ssm_close < DAY_SESSION_START):
                ask(
                    "`%s` on %s: the printed close `%s CT` falls in the small "
                    "hours of the trade date, before the ordinary day session "
                    "begins at `%s CT`."
                    % (family, iso(date), ssm_token(row.ssm_close),
                       ssm_token(DAY_SESSION_START)),
                    "The block records status `early_close` at that instant, and "
                    "the tool ships `early_close(%s)`. On the 2022-05-30 sheet the "
                    "outright Nikkei and BTIC Nikkei lines are merged under one "
                    "label and the printed instant is the BTIC-style `1:00 "
                    "(close)`; every other 2022 sheet prints a separate `Nikkei` "
                    "row governed by the EQUITIES `12:00` halt, and the block's own "
                    "verbatim calls the merge 'a residual ambiguity'. Ruling needed: "
                    "ship `%s`, or the EQUITIES `12:00`, or `Unsourced`?"
                    % (ssm_expr(row.ssm_close), ssm_token(row.ssm_close)),
                    "ship early_close(%s) as the block states"
                    % ssm_expr(row.ssm_close), [family])

    # 2. the fold clocks.
    for group, family in FOLD.items():
        ask(
            "The `%s` -> `%s` fold joins two different clocks."
            % (group, family),
            "`%s`'s ordinary week is %s; `%s`'s is %s. A family table holds one "
            "scalar boundary per trade date, so a folded group whose own session "
            "moves at a different instant cannot be represented by the crate "
            "family's row; the tool compares the two row by row on each group's "
            "own grid and ships the crate family's row."
            % (group, GRIDS[group].description, family, GRIDS[family].description),
            "keep the fold for status only; the crate family's instants stand",
            [family])

    # 3. per-date fold disagreements.
    for item in fold_report:
        if item["status"] == "agree":
            continue
        if item["status"] == "family-only":
            continue
        ask(
            "Fold disagreement on %s: `%s` says %s, `%s` says %s."
            % (item["trade_date"], item["group"],
               item["folded"] or "no row", item["family"],
               item["crate_family"] or "no row"),
            "Folding `%s` into `%s` gives one table; on this trade date the two "
            "lines state different answers, and the tool ships the crate family's "
            "row without changing it. Ruling needed: does the folded group close "
            "or shorten the crate family's trade date here?"
            % (item["group"], item["family"]),
            "ship the crate family's row and record the disagreement",
            [item["family"]])

    family_only = [i for i in fold_report if i["status"] == "family-only"]
    if family_only:
        dates = ", ".join(i["trade_date"] for i in family_only)
        ask(
            "The day-after-closure late opens on %s have no counterpart in the "
            "folded group." % dates,
            "On these dates the crate family's own document states that the "
            "prior-evening leg did not run and the day session opened at 08:30 CT, "
            "so the tool ships a `LateOpen`; the folded group's own clock reopens "
            "at its ordinary evening open and states no row, so it treats the same "
            "trade date as ordinary. This is the fold's clock difference showing up "
            "as a row and not just prose.",
            "ship the crate family's `LateOpen` rows",
            ["globex_grains"])

    # 4. rows on dates the block does not carry.
    created = []
    for family in FAMILIES:
        for date, row in sorted(family_rows[family].items()):
            if row.entry_date != date:
                created.append((family, iso(date), kind_words(row)))
    if created:
        ask(
            "%d rows are keyed to trade dates the block has no entry for."
            % len(created),
            "Every one is a `LateOpen` the crate family's own document states for "
            "the day after a closure (%s). The block carries one entry per holiday "
            "date, so the trade date that follows a closure has no entry of its own "
            "and the tool creates the row from the closure entry's printed open."
            % "; ".join("%s %s (%s)" % (f, d, k) for f, d, k in created),
            "keep the created rows", ["globex_grains"])

    # 5. closures whose reopen the documents do not record.
    gaps = {}
    for date, group, reason, entry in drops + [
            (r.trade_date, f, r.rationale, r.entry) for f in FAMILIES
            for d, r in family_rows[f].items()]:
        if (entry.get("status") in ("closed", "early_close", "modified")
                and entry.get("open_instant") in (None, "-")):
            gaps.setdefault(group, set()).add(iso(date))
    gaps = {group: sorted(dates) for group, dates in gaps.items()}
    if gaps:
        ask(
            "%d entries print no reopen instant at all."
            % sum(len(v) for v in gaps.values()),
            "The 2024 Good Friday windows are the cause: the T2 service was read "
            "for 2024-03-28 .. 2024-03-30, so the Sunday 2024-03-31 reopen (and "
            "with it whether the trade date 2024-04-01 lost its prior-evening leg, "
            "which matters for `globex_grains`) is outside the document. The tool "
            "ships no row for 2024-04-01. Every one is listed per group in the "
            "family evidence files' `No row, and why` section. Per group: %s."
            % "; ".join("%s: %s" % (g, ", ".join(v))
                        for g, v in sorted(gaps.items())),
            "ship nothing where no reopen is printed and record the gap")

    # 6. the Unsourced citation convention.
    if unsourced_from_missing:
        ask(
            "The %d uncovered 2023 dates cite the operator's own negative control "
            "as their document id." % len(unsourced_from_missing),
            "`holidays!` fails the build for a row with an empty document id, so an "
            "`Unsourced` row must name an artifact. The tool cites the saved T2 "
            "response that returns `hasEvents:false` for that window (%s), which is "
            "the operator's own machine channel read as bytes. The alternative is to "
            "split the family's coverage into three windows and claim no window "
            "over those dates. Two consequences are deliberately left unsourced: "
            "the day *after* each of these three holidays (2023-01-17, 2023-02-21 "
            "and, for `globex_grains`, 2023-04-10) may have lost its prior-evening "
            "leg the way 2024-01-02 and 2023-12-26 did, and no document in the "
            "store states whether it did, so those trade dates ship no row and the "
            "crate reports the family's ordinary week there."
            % ", ".join(sorted({i["document"] for i in unsourced_from_missing})),
            "cite the negative control, keep one 2022-2024 window")

    # 7. Nikkei 2024.
    nikkei_unsourced = [iso(d) for d, r in sorted(
        family_rows["globex_nikkei_225_dollar"].items()) if r.kind == "Unsourced"]
    if len(nikkei_unsourced) > 3:
        ask(
            "`globex_nikkei_225_dollar` ships %d `Unsourced` rows, 13 of them the "
            "whole 2024 window." % len(nikkei_unsourced),
            "The T2 service returns only the ten representative products and no "
            "Nikkei is among them, so the 2024 Nikkei dates are inside the family's "
            "window but carry no operator statement. The rows clip nothing and keep "
            "the window contiguous. Ruling needed: confirm, or shrink this family's "
            "coverage to 2022-01-01..2023-12-31 and let 2024 be 'no answer'?",
            "keep the contiguous window with `Unsourced` rows")

    # 7b. the ruled small-hours instant.
    small_hours = [(family, date, row)
                   for family in FAMILIES
                   for date, row in sorted(family_rows[family].items())
                   if row.role == "small-hours-close"]
    if small_hours:
        ask(
            "`%s` on %s ships `Unsourced`, not the printed small-hours close."
            % (small_hours[0][0], iso(small_hours[0][1])),
            "The 2022 Memorial Day workbook prints a merged `Nikkei & BTIC` "
            "line whose Friday close is the BTIC-style `1:00`; the same sheet's "
            "`Equity Products` row and the MLK and Presidents Day sheets print "
            "the outright Nikkei's `16:00` Friday close and a separate "
            "`Nikkei BTIC` row. The crate models the outright Nikkei, so the "
            "date is withheld as `Unsourced` at T1 rather than given an "
            "`early_close(1 * 3_600)` that would read as sourced and is wrong "
            "for the identity. Closing condition: a CME document that states "
            "the outright Nikkei's own close on a Memorial Day, or a tiling "
            "that separates the two lines.",
            "ship `Unsourced` at T1 with the workbook's document id",
            ["globex_nikkei_225_dollar"])
        items[-1]["scan"] = [{"date": iso(date), "family": family,
                              "role": row.role}
                             for family, date, row in small_hours]

    # 8. dates where a family ships no row but the entry's prose prints an
    #    instant its ordinary week does not have.
    prose_hints = []
    for date, holiday in sorted(entries_by_date.items()):
        for entry in holiday["families"]:
            family = entry["family"]
            if family not in FAMILIES or date in family_rows[family]:
                continue
            # Only the text the scanned date owns: an entry's verbatim often
            # carries its neighbours' events too, and those already have rows.
            clause = clause_for(entry, date, date) or entry.get("verbatim", "")
            text = " ".join([clause or "",
                             entry.get("close_instant", "") or "",
                             entry.get("open_instant", "") or ""])
            tokens = sorted(set(re.findall(r"\b\d{1,2}:\d{2}\b", text)))
            ordinary = {parse_instant("%s CT" % token)[0]
                        for token in ORDINARY_TOKENS[family]}
            unusual = [token for token in tokens
                       if parse_instant("%s CT" % token)[0] not in ordinary]
            if unusual:
                prose_hints.append({
                    "date": iso(date), "family": family,
                    "status": entry["status"], "tokens": unusual,
                })
    if prose_hints:
        ask(
            "%d date/family entries ship no row yet print an instant outside the "
            "family's ordinary week." % len(prose_hints),
            "The scan reads the text each scanned date owns (not its "
            "neighbours' events) and lists every (date, family) that ships no "
            "row while printing an `HH:MM` its ordinary week does not contain: "
            "%s. Reading them: (a) the 2022 New Year's sheet prints the "
            "`Nikkei/TOPIX BTIC` line's `Close 00:00`, a BTIC close on the next "
            "trade date, which this crate does not model; (b) the 2023 and 2024 "
            "Independence Day / New Year grain entries print the next trade "
            "date's `06:00 (PREOPEN)`, which is exactly the no-evening-leg "
            "marker that gives 2023-07-05 a `late_open` row; (c) the 2024-12-31 "
            "grains note points at `2025-01-02 06:00 preopen`, outside this "
            "wave's window, so no row ships and the 2025-2027 wave must state "
            "that trade date's late open. None of the three moves a boundary "
            "inside 2022-2024 that this family's scalar row vocabulary can "
            "state."
            % "; ".join("%s %s prints %s" % (item["date"], item["family"],
                                             ", ".join("`%s`" % t
                                                       for t in item["tokens"]))
                        for item in prose_hints[:12]),
            "ship no row for any of them; record the tokens as normal-week notes",
            sorted({item["family"] for item in prose_hints}))
        items[-1]["scan"] = prose_hints

    # 9. the maintainer's five-row list versus the six rows the rule produces.
    late_dates = sorted({iso(date) for family in FAMILIES
                         for date, row in family_rows[family].items()
                         if family == "globex_grains"
                         and row.kind in ("LateOpen", "LateOpenAndEarlyClose")
                         and row.entry_date != date})
    ruled_five = ["2022-07-05", "2023-12-26", "2024-01-02", "2024-07-05",
                  "2024-12-26"]
    extra = [date for date in late_dates if date not in ruled_five]
    if extra:
        ask(
            "The day-after-closure rule produces one more `globex_grains` row "
            "than the maintainer's five-date list: %s." % ", ".join(extra),
            "`%s` is the same shape as the five accepted dates: the closure "
            "date prints no evening leg and the next trade date opens with the "
            "day session. Its clause is `%s`, and the Monday before it prints "
            "the regular day session and PCP with no `16:45 preopen` / `19:00 "
            "open`. Dropping the row would leave trade date %s opening at the "
            "ordinary `19:00 CT` on the holiday itself, i.e. through the "
            "closure. Recommendation: keep it; if the ruling is to drop it, "
            "say so and the row goes."
            % (", ".join(extra),
               (family_rows["globex_grains"][dt.date.fromisoformat(extra[0])].clause
                or "")[:200],
               extra[0]),
            "keep the row",
            ["globex_grains"])

    # 10. tier provenance of the negative controls.
    default_tier_docs = sorted(
        doc_id for doc_id, doc in used_docs.items()
        if doc.get("tier_source") == "index-default")
    if default_tier_docs:
        ask(
            "%d document ids take their tier from the INDEX's channel statement, "
            "not from a per-row tier token." % len(default_tier_docs),
            "The fix INDEX's section 2 rows (`%s`) carry no `T1`/`T2` token in "
            "their description column; the file states the channel is `tier T2 "
            "under LAW-PRIMARY-SOURCES` on line 14. The tool reads that as the "
            "file-level default." % ", ".join(default_tier_docs),
            "accept the file-level tier statement")

    # 9. a holiday date on which every group is normal.
    all_normal = []
    for date, holiday in sorted(entries_by_date.items()):
        if all(entry["status"] == "normal" for entry in holiday["families"]):
            all_normal.append(iso(date))
    if all_normal:
        ask(
            "The wave's dates %s carry `normal` for all ten product groups, so no "
            "family ships any row for them." % ", ".join(all_normal),
            "2022-01-01 is a Saturday, so no family has a trade date there; the "
            "2022 New Year's workbook prints the ordinary Thursday/Friday/Sunday "
            "grid around it. Ruling needed: confirm that this holiday date "
            "correctly ships nothing, rather than being an earlier round's gap.",
            "ship nothing")

    for index, item in enumerate(items):
        item["n"] = index + 1
    return items


def write_decisions(out_dir, summary, family_rows, fold_report, drops, used_docs,
                    checks, for_review, entries_by_date):
    """Render tools/out/DECISIONS.md."""
    lines = []
    add = lines.append
    add("# wave 3 (2022-2024) holiday-row decisions\n\n")
    add("Generated by `tools/wave3_rows.py`; every number below is recomputed by\n")
    add("re-running it. The rows themselves are in `<family>.rows.rs`, the\n")
    add("per-family evidence tables in `<family>.evidence.md`, the row citations in\n")
    add("`documents.md`, the row-level encoding plan in `ROWS.json`, and the\n")
    add("machine-readable copy of everything here in `SUMMARY.json`. Regenerate\n")
    add("with `python3 tools/wave3_rows.py` (it prints\n")
    add("to stdout and rewrites `tools/out/`); verify with\n")
    add("`python3 tools/check_wave3.py`, which re-runs the generator into a scratch\n")
    add("directory, diffs every emitted byte and re-derives the structural checks\n")
    add("from the outputs.\n\n")

    add("## 1. Inputs\n\n")
    add("| input | sha256 | notes |\n|---|---|---|\n")
    add("| `%s` | `%s` | the wave-3 evidence block |\n"
        % (os.path.basename(summary["block"]["path"]),
           summary["block"]["sha256"]))
    for meta in summary["indexes"]:
        add("| `%s` | `%s` | %d indexed artifacts |\n"
            % (meta["label"], meta["sha256"], meta["rows"]))
    add("\nAudited window: **%s .. %s** (venue-local trade dates), declared as\n"
        "`coverage: %s` in every family.\n\n"
        % (summary["window"][0], summary["window"][1], summary["coverage_clause"]))
    add("The tool reads `cme-2022-2024.r2.json` when it exists and\n")
    add("`cme-2022-2024.json` otherwise; the file actually read is named above.\n")
    add("The round-2 verification verdict (`cme-2022-2024.verify.json`) records six\n")
    add("findings and all six are verbatim, coverage-prose or INDEX defects, so no\n")
    add("`status`, `close_instant`, `open_instant`, `document` or `tier` field\n")
    add("differs between r1 and r2 and the derived rows are identical under either.\n\n")

    add("## 2. Normal-week grid assumed per family, 2022-2024\n\n")
    add("The block is holiday evidence only. The ordinary week is the crate's own\n")
    add("family profile — the module the rows will live beside — read at the\n")
    add("revision in force for every date in this wave:\n\n")
    add("| family (group) | ordinary week | first matching open | ordinary final close | read from |\n|---|---|---|---|---|\n")
    for name in FAMILIES + tuple(FOLD):
        grid = GRIDS[name]
        add("| `%s` | %s | `%s CT`%s | %s | `%s` |\n"
            % (name, grid.description, ssm_token(grid.first_open),
               " (previous local date)" if grid.wrapped else " (same civil day)",
               grid.close_note, grid.source))
    add("\nThree consequences that decide rows below:\n\n")
    add("- The wrapped families' first **matching** open is 17:00 CT (grains\n")
    add("  19:00 CT) on the previous local date. The 16:00-17:00 and 16:45-19:00\n")
    add("  pre-opens are order-entry phases, so a printed open equal to the\n")
    add("  ordinary matching open moves no boundary and ships no row.\n")
    add("- Grains' and livestock's ordinary final close in the operator's own\n")
    add("  printing is the `16:00 (CLOSED)` token that ends the 14:30-16:00 CT\n")
    add("  post-close period; their matching close is 13:20 CT and 13:05 CT. A\n")
    add("  printed `16:00` close is therefore the ordinary one and ships nothing,\n")
    add("  which is what CME's own `16:00 (CLOSED)` means on 2023-07-03,\n")
    add("  2024-07-03 and 2024-12-31.\n")
    add("- `dairy` and `lumber` are folded (section 6) and their grids above are\n")
    add("  their own published clocks; they are used only to read their own rows\n")
    add("  and are never written into a crate table.\n\n")

    add("## 3. Trade-date conversion\n\n")
    add("The crate's trade date is the venue-local date of a session's final close.\n\n")
    add("- **Wrapped families** (`globex_equity_index`, `globex_energy`,\n")
    add("  `globex_fx`, `globex_grains`, `globex_interest_rates`,\n")
    add("  `globex_cryptocurrency`, `globex_nikkei_225_dollar`): a halt or close\n")
    add("  printed on date D ends the session that opened on D-1, so the row lands\n")
    add("  on D. CME's own `TRADE DATE:` labels for these days name the following\n")
    add("  business day instead; the crate keys on the local date of the final\n")
    add("  close, so the crate's date is D and CME's label is recorded, not\n")
    add("  adopted. Every wrapped-family row in this wave has trade date = the\n")
    add("  block's own date, and no row needed a different conversion.\n")
    add("- **Flat family** (`globex_livestock`): the session lies inside one civil\n")
    add("  day, so the operator's event date is the crate's trade date.\n")
    add("- **A printed open that moves a boundary** is placed by the macro's own\n")
    add("  `LateOpen` rule: a value at or after the ordinary first open belongs to\n")
    add("  the previous local date, a value below it to the trade date itself.\n")
    add("  Every deviating open here is a day-session `08:30 CT` open on a wrapped\n")
    add("  family, so it lands on the local date the block prints for it.\n\n")
    add("Rows keyed to a trade date the block carries no entry for:\n\n")
    add("| family | trade date | kind | stated by |\n|---|---|---|---|\n")
    created = 0
    for family in FAMILIES:
        for date, row in sorted(family_rows[family].items()):
            if row.entry_date == date:
                continue
            created += 1
            add("| `%s` | %s | %s | `%s` |\n"
                % (family, iso(date), kind_words(row), row.doc_id))
    if not created:
        add("| — | — | — | none |\n")
    add("\n**The late-open ruling.** A day-after-closure row ships only where CME\n")
    add("printed **no evening leg on the closure date**; where the holiday itself\n")
    add("carries the ordinary evening reopen, the next trade date is ordinary and\n")
    add("ships nothing. Each row below was re-read from the block's own printed\n")
    add("reopen token (`%s`, in place of the usual `%s`):\n\n"
        % (", ".join(sorted({row.printed_token for family in FAMILIES
                             for row in family_rows[family].values()
                             if row.printed_token})),
           "07:45 paused"))
    add("| family | trade date | printed reopen token | the clause it is read from | document |\n|---|---|---|---|---|\n")
    late_rows = [(family, date, row) for family in FAMILIES
                 for date, row in sorted(family_rows[family].items())
                 if row.kind in ("LateOpen", "LateOpenAndEarlyClose")]
    for family, date, row in late_rows:
        clause = (row.clause or "").replace("|", "/").strip()
        if len(clause) > 150:
            clause = clause[:147].rstrip() + "..."
        add("| `%s` | %s | `%s` | %s | `%s` |\n"
            % (family, iso(date), row.printed_token, clause, row.doc_id))
    add("\nThe counter-example, so the rule is not read as \"every closure is\n")
    add("followed by a late open\": the 2022 Christmas grains entry prints\n")
    add("`Monday, December 26 \"Globex Closed\", Pre-opening** 16:00, Open 19:00`,\n")
    add("i.e. the holiday itself carries the ordinary `19:00 CT` evening leg for\n")
    add("trade date 2022-12-27, so 2022-12-27 ships **no** row. 2022-01-17,\n")
    add("2022-02-21, 2022-04-15, 2022-05-30, 2022-06-20, 2022-09-05, 2023-01-02,\n")
    add("2023-05-29, 2023-06-19, 2023-09-04, 2024-01-15, 2024-02-19, 2024-05-27\n")
    add("and 2024-06-19 are the same shape and also ship no successor row.\n\n")

    add("## 4. What was dropped, and why\n\n")
    add("`status: normal` ships no row. A printed instant that equals the family's\n")
    add("ordinary instant changes no answer and ships no row either. A printed\n")
    add("`08:30 CT` open ships no late open when the trade date's own printed\n")
    add("morning shows its prior-evening leg ran.\n\n")
    add("| drop reason | entries |\n|---|---|\n")
    for key in sorted(summary["dropped_status_entry_counts"]):
        add("| %s | %d |\n" % (key, summary["dropped_status_entry_counts"][key]))
    add("| **total** | **%d** |\n\n"
        % sum(summary["dropped_status_entry_counts"].values()))
    add("| group | dropped entries |\n|---|---|\n")
    per_group = {}
    for items in summary["dropped_entries"].values():
        for item in items:
            per_group[item["group"]] = per_group.get(item["group"], 0) + 1
    for group in sorted(per_group):
        add("| `%s` | %d |\n" % (group, per_group[group]))
    add("\nThe full per-date list is in `SUMMARY.json` under `dropped_entries`.\n\n")
    fx_crypto = [d for d in drops if d[1] in ("globex_fx",
                                              "globex_cryptocurrency")
                 and "ordinary final close" in d[2]]
    add("`modified` entries are decided from the printed instants. The %d\n"
        % len(fx_crypto))
    add("`globex_fx` and `globex_cryptocurrency` entries on the %d noon-halt\n"
        % len({d[0] for d in fx_crypto}))
    add("holidays print a close the block itself states is the family's\n")
    add("ordinary final close, at a holiday halt that falls on that ordinary\n")
    add("instant, so they ship nothing; `globex_grains` on 2023-07-03, 2024-07-03\n")
    add("and 2024-12-31 prints its ordinary `16:00` post-close end and ships no\n")
    add("close row, and only 2024-07-03's *open* moves a boundary, which ships a\n")
    add("row on 2024-07-05.\n\n")

    add("## 5. Dates the operator's documents do not cover\n\n")
    add("Three holiday dates inside the window appear in the block's `missing`\n")
    add("list and carry no family entry at all. Silence there would read as\n")
    add("audited normal, so each of the eight families ships `Unsourced`:\n\n")
    add("| trade date | document | why |\n|---|---|---|\n")
    for item in summary["unsourced_dates_from_missing_block"]:
        add("| %s | `%s` | %s |\n"
            % (item["date"], item["document"],
               item["reason"].split(" - ")[0][:160]))
    add("\n**RULED: the small-hours instant on 2022-05-30 is withheld.** The 2022\n")
    add("Memorial Day sheet prints a merged `Nikkei & BTIC` line whose Friday close\n")
    add("is the BTIC-style `1:00`; the sheet's own `Equity Products` row and the\n")
    add("MLK and Presidents Day sheets print the outright Nikkei's `16:00` Friday\n")
    add("close and a separate `Nikkei BTIC` row. The crate models the outright\n")
    add("Nikkei, and `01:00 CT` is not its close, so the date ships `Unsourced` at\n")
    add("T1 rather than an `early_close(1 * 3_600)` that would read as sourced.\n")
    add("Closing condition: a CME document that states the outright Nikkei's own\n")
    add("close for a Memorial Day, or a separate tiling of the outright line.\n\n")
    add("The thirteen `globex_nikkei_225_dollar` `unknown` dates are the 2024\n")
    add("windows the T2 service answers for ten representative products, none of\n")
    add("them Nikkei; they ship `Unsourced` at T2 as well. The `Unsourced` kind\n")
    add("clips nothing, so every one of these dates still resolves to the family's\n")
    add("ordinary week — which is the honest answer, not a scheduling claim.\n\n")

    add("## 6. The dairy and lumber fold\n\n")
    add("Decided for this wave: `dairy` folds into `globex_grains` and `lumber`\n")
    add("into `globex_livestock`, because CME publishes them under the joined\n")
    add("labels `Grains & Oilseeds include Dairy` and `Livestock include Lumber`.\n")
    add("The tool derives each folded group's own row on its own published grid\n")
    add("and compares it with the crate family's row for the same trade date; a\n")
    add("date on which both state a row and the rows differ is reported, and the\n")
    add("crate family's own row is the one shipped.\n\n")
    add("| group -> family | dates compared | agree | group-only | family-only | differ |\n|---|---|---|---|---|---|\n")
    for group, family in FOLD.items():
        items = [i for i in fold_report if i["group"] == group]
        counts = {k: sum(1 for i in items if i["status"] == k)
                  for k in ("agree", "group-only", "family-only", "differ")}
        add("| `%s` -> `%s` | %d | %d | %d | %d | %d |\n"
            % (group, family, len(items), counts["agree"], counts["group-only"],
               counts["family-only"], counts["differ"]))
    add("\nPer-date disagreements (both sides state a row and they differ):\n\n")
    add("| group -> family | trade date | folded group says | crate family says |\n|---|---|---|---|\n")
    any_differ = False
    for item in fold_report:
        if item["status"] != "differ":
            continue
        any_differ = True
        add("| `%s` -> `%s` | %s | %s | %s |\n"
            % (item["group"], item["family"], item["trade_date"],
               item["folded"], item["crate_family"]))
    if not any_differ:
        add("| — | — | — | none |\n")
    add("\nRows the crate family states and its folded group does not (the\n")
    add("day-after-closure late opens, where the folded group's own clock reopens\n")
    add("at its ordinary 17:00 CT evening open instead):\n\n")
    add("| group -> family | trade date | crate family says |\n|---|---|---|\n")
    any_only = False
    for item in fold_report:
        if item["status"] != "family-only":
            continue
        any_only = True
        add("| `%s` -> `%s` | %s | %s |\n"
            % (item["group"], item["family"], item["trade_date"],
               item["crate_family"]))
    if not any_only:
        add("| — | — | — |\n")
    add("\nBeyond the per-date rows, the two clocks themselves differ and no row\n")
    add("comparison can show it: dairy's ordinary session is the wrapped\n")
    add("17:00-16:00 CT leg while `globex_grains` is 19:00 -> 07:45 plus\n")
    add("08:30-13:20 CT; lumber's ordinary session is 09:00-15:05 CT while\n")
    add("`globex_livestock` is 08:30-13:05 CT. Both are raised as questions.\n\n")

    add("## 7. Document-id assignment\n\n")
    add("- **T1** ids are `<file-name> @<capture UTC>`, taken from the INDEX's\n")
    add("  `file` and `capture (UTC)` columns: e.g.\n")
    add("  `2022-mlk-day-holiday-schedule.xls @2022-01-17T21:22:30Z`. The capture\n")
    add("  timestamp is the Wayback capture, never the retrieval date.\n")
    add("- **T2** ids are `CME-SVC-<first fromEventDate in the queried window>`,\n")
    add("  read from the `fromEventDate=` token of the INDEX's `source url`\n")
    add("  column: e.g. `CME-SVC-2024-01-14` for `api_2024-01-14.json`.\n")
    add("- The operator's finalised 2022 Thanksgiving vintage is saved under a\n")
    add("  distinct local name (`2022-thanksgiving-holiday-schedule.FINAL-20221122.xls`)\n")
    add("  and its id uses that name, so the preliminary capture of the same URL\n")
    add("  cannot be confused with it. The preliminary vintage keys no row.\n")
    add("- `tools/out/ROWS.json` carries one object per family (`date`, `kind`,\n")
    add("  `open_ssm`, `close_ssm`, `tier`, `document`, `reason`, `printed`,\n")
    add("  `source`), the per-family/per-kind/per-tier/per-year counts and the\n")
    add("  resolution of every id any row cites; `tools/check_wave3.py` parses it\n")
    add("  and the eight `.rows.rs` files and asserts they agree row for row.\n")
    add("- Every id used by a row appears in `tools/out/documents.md` and nowhere\n")
    add("  else: %d ids, %d T1 and %d T2.\n"
        % (summary["documents"]["count"], summary["documents"]["t1"],
           summary["documents"]["t2"]))
    add("- Artifacts read but keying no row — the 2022 New Year's workbook (all\n")
    add("  ten of its entries are `normal`), `api_2022-12-31.json`, the clearing\n")
    add("  advisories and every CDX index response — are not in `documents.md`,\n")
    add("  because that table is the resolution table for row citations only.\n\n")

    add("## 8. Verification performed\n\n")
    add("- Every family's rows are strictly ascending by trade date and inside\n")
    add("  2022-01-01..2024-12-31.\n")
    add("- Every row's document id resolves to an INDEX row whose sha256 and byte\n")
    add("  count were recomputed from the saved file.\n")
    add("- Every `instant as printed` token is a substring of the block's own text\n")
    add("  for the entry the row cites.\n")
    add("- Re-running the tool on the same inputs is byte-identical;\n")
    add("  `tools/check_wave3.py` re-runs it, re-parses the emitted `.rows.rs`\n")
    add("  files independently and recomputes these checks from the outputs.\n")
    add("- In-tool check results: **%s**.\n\n"
        % ("PASS" if summary["checks_passed"] else "; ".join(checks)))

    add("## 9. Questions to rule on\n\n")
    for item in for_review:
        add("**Q%d. %s**\n\n" % (item["n"], item["title"]))
        add("%s\n\n" % item["detail"])
        add("Tool default: %s.\n\n" % item["tool_default"])
    with open(os.path.join(out_dir, "DECISIONS.md"), "w", encoding="utf-8") as handle:
        handle.write("".join(lines))


if __name__ == "__main__":
    sys.exit(main())
