#!/usr/bin/env python3
# SPDX-License-Identifier: MIT-0
"""wave5_rows.py -- turn the repaired cme-2013-2015 block into crate holiday rows.

The crate ships one static holiday table per CME product family, keyed by the
crate's own venue-local **trade date** (LAW-HOLIDAY-SCOPE). This tool reads

  * the repaired wave-5 evidence block
    (`$WAVE5_RESEARCH/holidays/cme-2013-2015.r2.json`),
  * the saved xlrd text dumps under `holidays/raw/cme-2013-2015-fix/txt/` and
    the `pdftotext -layout` dumps under `holidays/raw/cme-2013-2015/txt/`,
    which every instant is re-read from, and
  * the crate's own 2013-2015 ordinary grids, restated below with the module
    and the revision each was read from,

and emits, for the six crate families that have the 2013-2015 gap, over
venue-local trade dates 2013-01-01 .. 2015-12-31:

  tools/out/wave5/<family>.rows.rs       rows ready to paste into a `holidays!` table
  tools/out/wave5/<family>.evidence.md   the per-family, per-year evidence table
  tools/out/wave5/documents.md           one row per document id used by a row
  tools/out/wave5/ROWS.json              the encoding plan (machine readable)
  tools/out/wave5/SUMMARY.json           counts, folds, drops, questions

Conversion rules, all of them applied by `derive_group`:

  * Only a date/status that changes an answer ships a row.  `normal` never
    ships; a printed instant equal to the family's ordinary instant for this
    era never ships.
  * A block entry names a **grouped product line**, not a crate key.  Each
    group is expanded onto the crate families it covers; `dairy` and `lumber`,
    which have no crate key, are folded into `globex_grains` and
    `globex_livestock` for *reporting* only, and never key a row.
  * Rows are keyed to the crate's venue-local trade date, never to the
    operator's event date: a holiday's own event date is the trade date whose
    final close falls on it, because the sheet prints that date's close.  A
    re-open printed for the *next* trade date is keyed to that date.
  * `closed` -> `Closed`;  `early_close` -> `EarlyClose(<printed close>)` when
    the printed close is earlier than the family's ordinary close;
    `late_open`/`modified` are decided from the printed instants.
  * A printed first open is a `LateOpen` on the trade date it carries when it
    is late enough to mean the evening leg did not run (see LATE_OPEN_GRACE).
    A routine evening re-open at or after the ordinary first open ships
    nothing: the re-open a holiday schedule prints on the eve is the *next*
    trade date's own open, and the sheet prints it on every date.

Stdlib only.  Deterministic: same bytes in, same text out.
"""

from __future__ import annotations

import argparse
import collections
import datetime as dt
import json
import os
import re
import sys

# --------------------------------------------------------------------------
# Families, folds, coverage, grids
# --------------------------------------------------------------------------

FAMILIES = (
    "globex_equity_index",
    "globex_energy",
    "globex_fx",
    "globex_grains",
    "globex_interest_rates",
    "globex_livestock",
)


#: Groups the crate does not key.  They are printed on the same sheets and
#: folded here for reporting only; their instants never key a crate row.
FOLD = {"dairy": "globex_grains", "lumber": "globex_livestock"}

GROUPS = FAMILIES + tuple(FOLD)

WINDOW = (dt.date(2013, 1, 1), dt.date(2015, 12, 31))
WINDOW_CELL = "2013-01-01 .. 2015-12-31"

#: A printed first open at least this much later than the trade date's ordinary
#: first open means the evening leg did not run and the row is a `LateOpen`.
#: A reopen inside the grace window is the routine evening open the sheet
#: prints on the eve of the next trade date, not a delayed session start.
LATE_OPEN_GRACE = 4 * 3_600

#: The two served families that declare no 2013-2015 window, with the reason.
NO_WINDOW = {
    "globex_cryptocurrency": (
        "CME listed no cryptocurrency product before Bitcoin futures on "
        "2017-12-18, so there is no crypto line on any 2013-2015 sheet to "
        "source (block missing[1])."
    ),
    "globex_nikkei_225_dollar": (
        "No Nikkei-specific line exists on any CME Group holiday schedule for "
        "2013, 2014 or 2015; the undifferentiated CME & CBOT Equity Products "
        "heading is the operator's statement for this family, and it is "
        "modelled by globex_equity_index (block missing[0])."
    ),
}


class Grid:
    """The ordinary week of one product group over 2013-2015.

    ``first_open(date)`` is the trade date's ordinary first executable open,
    which for a wired overnight grid falls on the preceding local date.  A
    grid is **wrapped** when that instant sits on the day before the trade
    date: only then can a printed open be "late" by being *later in clock
    terms* than the ordinary first open.
    """

    def __init__(self, name, wrapped, first_open, close, description, source,
                 close_note, day_open=None):
        self.name = name
        self.wrapped = wrapped
        self.close_note = close_note
        self.description = description
        self.source = source
        self.day_open = day_open
        if callable(first_open):
            self._first_open = first_open
        else:
            self._first_open = lambda _date, value=first_open: value
        if callable(close):
            self._close = close
        else:
            self._close = lambda _date, value=close: value

    def first_open(self, date):
        return self._first_open(date)

    def ordinary_close(self, date):
        return self._close(date)


def _c(*parts):
    """Builds a fixed ssm: ``_c(16, 15)`` == 16:15 -> 58_500 seconds."""
    total = 0
    for i, value in enumerate(parts):
        total += value * (3_600 if i == 0 else 60 if i == 1 else 1)
    return total


GRID_1615 = Grid(
    "wrapped-1615", True, _c(17, 0), _c(16, 15),
    "one wrapped leg per trade date, 17:00 CT the previous evening into a "
    "16:15 CT close on the trade date",
    "cme_group.rs CME_PROFILE_2012_11_18 (2012-11-18, notice 20121022) / "
    "energy_metals.rs ENERGY_METALS_AT_2010_FLOOR (2010 floor)",
    "16:15 CT",
)
GRID_1600 = Grid(
    "wrapped-1600", True, _c(17, 0), _c(16, 0),
    "one wrapped leg per trade date, 17:00 CT the previous evening into a "
    "16:00 CT close on the trade date",
    "cme_group.rs CME_PROFILE_2015_09_20 (2015-09-20, notice 20150817) / "
    "energy_metals.rs ENERGY_METALS_DATED_CURRENT (2015-09-20, notice 20150907)",
    "16:00 CT",
)
GRID_RATES_FX = Grid(
    "wrapped-1600", True, _c(17, 0), _c(16, 0),
    "one wrapped leg per trade date, 17:00 CT the previous evening into a "
    "16:00 CT close on the trade date",
    "interest_rates.rs PROFILE_2011_10_02 (2011-10-02, notice 20110926) / "
    "fx.rs DATED_CURRENT (2010-11-15, notice 20101025)",
    "16:00 CT",
)
GRID_GRAINS_1315 = Grid(
    "grains-1315", True, _c(19, 0), _c(13, 15),
    "19:00 CT the previous evening into 07:45 CT, then the 08:30-13:15 CT day "
    "session whose close is the trading day's final closing phase",
    "grains.rs FROM_2013_04_07 (2013-04-07, CME SER-6617 and GCC notice "
    "2013-03-22) / FROM_2012_05_20 before it",
    "13:15 CT", day_open=_c(8, 30),
)
GRID_GRAINS_1320 = Grid(
    "grains-1320", True, _c(19, 0), _c(13, 20),
    "19:00 CT the previous evening into 07:45 CT, then the 08:30-13:20 CT day "
    "session whose close is the trading day's final closing phase",
    "grains.rs DATED_CURRENT (2015-07-05, CME SER-7395R)",
    "13:20 CT", day_open=_c(8, 30),
)


def _livestock_floor_first_open(date):
    # Monday opens 09:05 CT on the day itself; Tuesday-Thursday open 17:00 CT
    # on the previous evening, which is the same trading day, not a wrap.
    return _c(9, 5) if date.weekday() == 0 else _c(17, 0)


def _livestock_floor_close(date):
    return _c(13, 55) if date.weekday() == 3 else _c(16, 0)


def _livestock_dated_first_open(date):
    return _c(9, 5) if date.weekday() == 0 else _c(8, 0)


def _livestock_dated_close(date):
    return _c(13, 55) if date.weekday() == 4 else _c(16, 0)


GRID_LIVESTOCK_FLOOR = Grid(
    "livestock-floor", False, _livestock_floor_first_open, _livestock_floor_close,
    "one flat session inside a single civil day: 09:05-16:00 CT on Mondays, "
    "17:00-16:00 CT on Tuesday-Thursday and 17:00-13:55 CT on Thursdays",
    "livestock.rs PROFILE_AT_2010_FLOOR (2007 around-the-clock schedule)",
    "16:00 CT (13:55 CT on Thursdays)", day_open=_c(9, 5),
)
GRID_LIVESTOCK_2014 = Grid(
    "livestock-2014", False, _livestock_dated_first_open, _livestock_dated_close,
    "one flat session inside a single civil day: 09:05-16:00 CT on Mondays, "
    "08:00-16:00 CT on Tuesday-Thursday and 08:00-13:55 CT on Fridays",
    "livestock.rs PROFILE_2014_10_27 (2014-10-27, CME SER-7194)",
    "16:00 CT (13:55 CT on Fridays)", day_open=_c(8, 0),
)

#: Family -> ((first trade date the grid governs, Grid), ...), ascending.
GRIDS = {
    "globex_equity_index": ((dt.date(2013, 1, 1), GRID_1615),
                            (dt.date(2015, 9, 20), GRID_1600)),
    "globex_energy": ((dt.date(2013, 1, 1), GRID_1615),
                      (dt.date(2015, 9, 20), GRID_1600)),
    "globex_interest_rates": ((dt.date(2013, 1, 1), GRID_RATES_FX),),
    "globex_fx": ((dt.date(2013, 1, 1), GRID_RATES_FX),),
    "globex_grains": ((dt.date(2013, 1, 1), GRID_GRAINS_1315),
                      (dt.date(2015, 7, 5), GRID_GRAINS_1320)),
    "globex_livestock": ((dt.date(2013, 1, 1), GRID_LIVESTOCK_FLOOR),
                         (dt.date(2014, 10, 27), GRID_LIVESTOCK_2014)),
}

#: Folded groups, described on their own published clock.
FOLD_GRIDS = {
    "dairy": Grid(
        "dairy", True, _c(17, 0),
        lambda date: _c(13, 55) if date.weekday() == 3 else _c(16, 0),
        "one wrapped leg per trade date, 17:00 CT the previous evening into a "
        "16:00 CT close (13:55 CT on Thursdays)",
        "block dairy rows (CME's Dairy product-group line); not a crate key",
        "16:00 CT (13:55 CT on Thursdays)",
    ),
    "lumber": Grid(
        "lumber", False, _c(9, 0), _c(15, 5),
        "one flat Monday-Friday 09:00-15:05 CT session inside a single civil day",
        "block lumber rows (CME's Lumber product-group line); not a crate key",
        "15:05 CT", day_open=_c(9, 0),
    ),
}

#: The printed line each crate family's instants come from.  A row that cites a
#: combined Globex holiday sheet names the line its instant was read off.
SHEET_LABEL = {
    "globex_equity_index": "Equity Products",
    "globex_energy": "Energy, Metals & DME Products",
    "globex_fx": "FX Products",
    "globex_grains": "Grain, Oilseed & MGEX Products",
    "globex_interest_rates": "Interest Rate Products",
    "globex_livestock": "Livestock",
    "dairy": "Dairy",
    "lumber": "Lumber",
}

#: Block family groups -> (crate families they cover, folded groups reported).
GROUP_MAP = {
    "equity_index": (("globex_equity_index",), ()),
    "interest_rates+fx": (("globex_interest_rates", "globex_fx"), ()),
    "energy+metals": (("globex_energy",), ()),
    "grains_oilseeds": (("globex_grains",), ()),
    "livestock": (("globex_livestock",), ()),
    # CME prints Livestock, Dairy and Lumber as one section: the machinery and
    # the livestock clock key the crate row, and the dairy/lumber instants are
    # reported beside it for the fold.
    "livestock+dairy+lumber": (("globex_livestock",), ("dairy", "lumber")),
    "grains_oilseeds+livestock+dairy+lumber": (
        ("globex_grains", "globex_livestock"), ()),
    "equity_index+interest_rates+fx": (
        ("globex_equity_index", "globex_interest_rates", "globex_fx"), ()),
    "equity_index+interest_rates+fx+energy+metals": (
        ("globex_equity_index", "globex_interest_rates", "globex_fx",
         "globex_energy"), ()),
    "equity_index+interest_rates+fx+energy+metals+grains_oilseeds"
    "+livestock+dairy+lumber": (
        ("globex_equity_index", "globex_interest_rates", "globex_fx",
         "globex_energy", "globex_grains", "globex_livestock"), ()),
    # An eve record CME prints for the whole exchange but that carries only the
    # evening re-open of the next trade date.
    "interest_rates+fx+energy+metals": (
        ("globex_interest_rates", "globex_fx", "globex_energy"), ()),
    # Folded groups on their own, where CME states only that line's own
    # status (2013-07-04 and 2013-12-25).
    "dairy": ((), ("dairy",)),
    "lumber": ((), ("lumber",)),
}

# --------------------------------------------------------------------------
# Clock parsing
# --------------------------------------------------------------------------

CLOCK_RE = re.compile(r"\b(\d{3,4})\s*CT\b")
DAY_LABEL_RE = re.compile(
    r"\b(?:Mon|Tues|Tue|Wed|Thurs|Thu|Fri|Sat|Sun)(?:day)?\.?,?\s+"
    r"(?:(Jan|Feb|Mar|Apr|May|Jun|Jul|Aug|Sep|Oct|Nov|Dec)[a-z]*\.?\s+(\d{1,2})"
    r"|(\d{1,2})\s+(Jan|Feb|Mar|Apr|May|Jun|Jul|Aug|Sep|Oct|Nov|Dec)[a-z]*\.?)",
    re.IGNORECASE,
)
MONTHS = {m: i + 1 for i, m in enumerate(
    ["jan", "feb", "mar", "apr", "may", "jun",
     "jul", "aug", "sep", "oct", "nov", "dec"])}


def parse_clock(text):
    match = CLOCK_RE.search(text or "")
    if not match:
        return None
    digits = match.group(1)
    if len(digits) == 3:
        hour, minute = int(digits[0]), int(digits[1:])
    else:
        hour, minute = int(digits[:2]), int(digits[2:])
    if hour > 23 or minute > 59:
        return None
    return hour * 3_600 + minute * 60


def clock_cell(ssm):
    if ssm is None:
        return "—"
    hour, rest = divmod(ssm, 3_600)
    return "%d:%02d CT" % (hour, rest // 60)


def named_day(cell, anchor):
    match = DAY_LABEL_RE.search(cell or "")
    if not match:
        return None
    if match.group(1):
        month, day = MONTHS[match.group(1).lower()[:3]], int(match.group(2))
    else:
        day, month = int(match.group(3)), MONTHS[match.group(4).lower()[:3]]
    best = None
    for year in (anchor.year - 1, anchor.year, anchor.year + 1):
        try:
            candidate = dt.date(year, month, day)
        except ValueError:
            continue
        if best is None or abs((candidate - anchor).days) < abs((best - anchor).days):
            best = candidate
    return best


def next_weekday(date):
    probe = date + dt.timedelta(days=1)
    while probe.weekday() >= 5:
        probe += dt.timedelta(days=1)
    return probe


def grid_at(family, date):
    chosen = GRIDS[family][0][1]
    for first, grid in GRIDS[family]:
        if date >= first:
            chosen = grid
    return chosen


# --------------------------------------------------------------------------
# Derivation
# --------------------------------------------------------------------------

class Row:
    """One crate holiday row under construction."""

    def __init__(self, family, trade_date, kind, open_ssm, close_ssm, tier,
                 document, reason, printed_open=None, printed_close=None,
                 group=None):
        self.family = family
        self.trade_date = trade_date
        self.kind = kind
        self.open_ssm = open_ssm
        self.close_ssm = close_ssm
        self.tier = tier
        self.document = document
        self.reason = reason
        self.printed_open = printed_open
        self.printed_close = printed_close
        self.group = group

    def key(self):
        return (self.kind, self.open_ssm, self.close_ssm)


#: A clock followed by a close word, on the same printed line.
CLOSE_TOKEN_RE = re.compile(
    r"\b(\d{3,4})\s*CT\b[^\n]{0,80}?"
    r"\b(?:Regular|Early|regular|early)[^\n]{0,20}?close", re.IGNORECASE)


def ordinary_close_token(entry):
    """The close clock a `normal` row's own verbatim line prints, or None.

    Only a clock the same line calls a close counts: a holiday schedule prints
    the *next* trade date's evening re-open beside the close, and that clock is
    not this date's.
    """
    match = CLOSE_TOKEN_RE.search(entry.get("verbatim", ""))
    return parse_clock(match.group(1) + " CT") if match else None


def closure_token(entry):
    for token in re.split(r"[\/\n]", entry.get("verbatim", "")):
        if re.search(r"\bclosed\b", token, re.IGNORECASE):
            return token.strip()
    return entry.get("status", "")


def derive_group(entry, group, crate_families, folded, sink):
    """Expands one block row onto the crate families the group covers."""
    date = dt.date.fromisoformat(entry["date"])
    status = entry["status"]
    document = entry["document"]
    tier = entry["tier"]
    close = parse_clock(entry.get("close_instant"))
    reopens = list(entry.get("reopens") or [])
    if entry.get("open_instant"):
        # The round-2 repaired block folds a single re-open back into
        # `open_instant`; the derivation reads whichever shape is present.
        reopens.insert(0, entry["open_instant"])

    for family in crate_families:
        grid = grid_at(family, date)
        ordinary_close = grid.ordinary_close(date)
        rows = sink.setdefault(family, {}).setdefault(date, [])

        if status == "normal":
            sink["_drops"].append((entry["date"], group, family, "status normal"))
            printed = ordinary_close_token(entry)
            if printed is not None and printed != ordinary_close:
                sink["_questions"].append(
                    "%s %s: the block records `normal` but its cited line prints "
                    "%s against the family's ordinary %s"
                    % (entry["date"], family, clock_cell(printed),
                       clock_cell(ordinary_close)))
            continue

        if status == "closed":
            rows.append(Row(
                family, date, "Closed", None, None, tier, document,
                "CME prints no session running through this date",
                printed_close=closure_token(entry), group=group))
            continue

        if close is not None and close != ordinary_close:
            if close > ordinary_close:
                sink["_questions"].append(
                    "%s %s: printed close %s is later than the family's "
                    "ordinary %s, which no holiday kind can state"
                    % (entry["date"], family, clock_cell(close),
                       clock_cell(ordinary_close)))
            else:
                rows.append(Row(
                    family, date, "EarlyClose", None, close, tier, document,
                    "the printed final close %s is earlier than the family's "
                    "ordinary %s" % (clock_cell(close),
                                     clock_cell(ordinary_close)),
                    printed_close=entry.get("close_instant"), group=group))

        for cell in reopens:
            open_ssm = parse_clock(cell)
            if open_ssm is None:
                continue
            # A re-open cell with a day label names the trade date it carries;
            # one without a label is the entry's own date.  The `for trade
            # date ...` prose forms always carry a label, so no case needs a
            # guess at a following weekday.
            target = named_day(cell, date) or date
            first_open = grid_at(family, target).first_open(target)
            delta = (open_ssm - first_open) % (24 * 3_600)
            if delta < LATE_OPEN_GRACE:
                sink["_drops"].append(
                    (entry["date"], group, family,
                     "re-open %s is not late enough to be a session start"
                     % clock_cell(open_ssm)))
                continue
            rows.append(Row(
                family, target, "LateOpen", open_ssm, None, tier, document,
                "the trade date's first open is %s: the evening leg that would "
                "have opened earlier did not run" % clock_cell(open_ssm),
                printed_open=cell, group=group))

    for group_name in folded:
        fold_entry(entry, group_name, sink)


def fold_entry(entry, group_name, sink):
    """Reports a folded group's own status against the family it folds to."""
    date = dt.date.fromisoformat(entry["date"])
    grid = FOLD_GRIDS[group_name]
    close = parse_clock(entry.get("close_instant"))
    status = entry["status"]
    if status == "closed":
        reported = "closed"
    elif close is not None and close != grid.ordinary_close(date):
        reported = "early close " + clock_cell(close)
    else:
        reported = status
    sink["_fold"].append({
        "group": group_name,
        "crate_family": FOLD[group_name],
        "trade_date": entry["date"],
        "status": status,
        "reported": reported,
    })


def merge_family(rows_by_date, family, sink):
    """Collapses one family's rows for a trade date into at most one row."""
    merged = []
    for date in sorted(rows_by_date):
        entries = rows_by_date[date]
        closes = [r for r in entries if r.kind == "EarlyClose"]
        opens = [r for r in entries if r.kind == "LateOpen"]
        closed = [r for r in entries if r.kind == "Closed"]
        if len({r.close_ssm for r in closes}) > 1:
            sink["_conflicts"].append(
                "%s %s: two different printed early closes (%s)"
                % (date, family, ", ".join(sorted(
                    clock_cell(r.close_ssm) for r in closes))))
            continue
        if len({r.open_ssm for r in opens}) > 1:
            sink["_conflicts"].append(
                "%s %s: two different printed first opens (%s)"
                % (date, family, ", ".join(sorted(
                    clock_cell(r.open_ssm) for r in opens))))
            continue
        if closed:
            if closes or opens:
                sink["_conflicts"].append(
                    "%s %s: a closure and a boundary move on one trade date"
                    % (date, family))
            merged.append(closed[0])
            continue
        if closes and opens:
            merged.append(Row(
                family, date, "LateOpenAndEarlyClose", opens[0].open_ssm,
                closes[0].close_ssm, closes[0].tier, closes[0].document,
                "%s; %s" % (opens[0].reason, closes[0].reason),
                printed_open=opens[0].printed_open,
                printed_close=closes[0].printed_close))
        elif closes:
            merged.append(closes[0])
        elif opens:
            merged.append(opens[0])
    merged.sort(key=lambda r: r.trade_date)
    return merged


# --------------------------------------------------------------------------
# Document ids
# --------------------------------------------------------------------------

def document_id(code, block):
    """The artifact's id: the full `<file> @<RFC 3339 capture>` form.

    `every_document_id_resolves_to_one_artifact` resolves an id to one
    `(window, sha256)` pair and to one `(file, sha256)` pair, so a second,
    date-only label for the same artifact is rejected outright. Every id this
    era writes is therefore the full capture form, and the era's `### Documents`
    table resolves each of them exactly once.
    """
    entry = block["documents"][code]
    # `capture_label` is set for an artifact an earlier wave already cites under
    # another label; every other document uses its full capture time.
    label = entry.get("capture_label", entry["capture_utc"])
    return "%s @%s" % (os.path.basename(entry["file"]), label)


def row_code(block, document):
    """The block document code behind a rendered document id.

    Two documents can share a filename (CME republished several URLs), so the
    lookup is by the whole `<file> @<capture>` id and never by filename alone.
    """
    matches = [code for code in block["documents"]
               if document_id(code, block) == document
               or "%s @%s" % (os.path.basename(block["documents"][code]["file"]),
                              block["documents"][code]["capture_utc"]) == document]
    if len(matches) != 1:
        raise SystemExit("document id %r resolves to %d block codes"
                         % (document, len(matches)))
    return matches[0]


def document_rows(block, used):
    out = []
    for code in sorted(used):
        entry = block["documents"][code]
        out.append({
            "id": document_id(code, block),
            "code": code,
            "file": os.path.basename(entry["file"]),
            "url": entry.get("replay_url") or entry.get("original_url"),
            "capture": entry["capture_utc"],
            "tier": entry.get("tier", "T1"),
            "sha256": entry.get("sha256"),
        })
    return out


# --------------------------------------------------------------------------
# Emit
# --------------------------------------------------------------------------

def ssm_expr(ssm):
    if ssm is None:
        raise ValueError("no instant")
    hours, rest = divmod(ssm, 3_600)
    minutes, seconds = divmod(rest, 60)
    if seconds:
        raise ValueError("sub-minute instant %d" % ssm)
    parts = []
    if hours:
        parts.append("%d * 3_600" % hours)
    if minutes:
        parts.append("%d * 60" % minutes)
    return " + ".join(parts) if parts else "0"


KIND_EXPR = {
    "Closed": lambda r: "Closed",
    "Unsourced": lambda r: "Unsourced",
    "EarlyClose": lambda r: "early_close(%s)" % ssm_expr(r.close_ssm),
    "LateOpen": lambda r: "late_open(%s)" % ssm_expr(r.open_ssm),
    "LateOpenAndEarlyClose": lambda r: "late_open_and_early_close(%s, %s)" % (
        ssm_expr(r.open_ssm), ssm_expr(r.close_ssm)),
}

KIND_WORDS = {
    "Closed": "closed",
    "Unsourced": "unsourced",
    "EarlyClose": "early close",
    "LateOpen": "late open",
    "LateOpenAndEarlyClose": "late open and early close",
}


def render_rust(row, block):
    document = document_id(row.document, block)
    comment = "        // %s - %s - %s - %s." % (
        row.trade_date.isoformat(), row.tier, document,
        row.reason.strip().rstrip("."))
    tuple_line = '        (%d, %d, %d, %s, %s, "%s"),' % (
        row.trade_date.year, row.trade_date.month, row.trade_date.day,
        KIND_EXPR[row.kind](row), row.tier, document)
    return comment + "\n" + tuple_line


def sanitize(text):
    """No evidence cell may contain a pipe or a backtick of its own."""
    return (text or "").replace("|", "·").replace("`", "")


def instant_cell(row):
    tokens = []
    if row.printed_close:
        tokens.append("`%s`" % sanitize(row.printed_close))
    if row.printed_open:
        tokens.append("`%s`" % sanitize(row.printed_open))
    return " / ".join(tokens) if tokens else "—"


def render_evidence(rows, year, note_by_date, block):
    lines = ["### %d" % year, "",
             "| trade date | kind | instant as printed | document | tier | derived from |",
             "|---|---|---|---|---|---|"]
    for row in rows:
        if row.trade_date.year != year:
            continue
        derived = sanitize(row.reason)
        extra = note_by_date.get(row.trade_date)
        if extra:
            derived = "%s %s" % (derived, sanitize(extra))
        lines.append("| %s | %s | %s | `%s` | %s | %s |" % (
            row.trade_date.isoformat(), KIND_WORDS[row.kind],
            instant_cell(row), document_id(row.document, block), row.tier,
            derived))
    return "\n".join(lines)


def render_documents(documents):
    lines = [
        "| Document | Window | Replay or service URL | Capture or retrieval, UTC | Tier | sha256 |",
        "|---|---|---|---|---|---|"]
    for doc in documents:
        lines.append("| `%s` | %s | <%s> | %s | %s | `%s` |" % (
            doc["id"], WINDOW_CELL, doc["url"], doc["capture"], doc["tier"],
            doc["sha256"] or "—"))
    return "\n".join(lines)


def main(argv=None):
    parser = argparse.ArgumentParser()
    parser.add_argument("--research", default=os.environ.get("WAVE5_RESEARCH"))
    parser.add_argument("--out", default="tools/out/wave5")
    args = parser.parse_args(argv)
    if not args.research:
        print("WAVE5_RESEARCH must name the research store root", file=sys.stderr)
        return 2

    block_path = os.path.join(args.research, "holidays", "cme-2013-2015.r2.json")
    with open(block_path) as handle:
        block = json.load(handle)

    sink = {"_drops": [], "_questions": [], "_conflicts": [], "_fold": []}
    for family in FAMILIES:
        sink[family] = collections.defaultdict(list)
    for holiday in block["holidays"]:
        for family_row in holiday["families"]:
            group = family_row["family"]
            if group not in GROUP_MAP:
                raise SystemExit("unmapped block family group %r" % group)
            crate_families, folded = GROUP_MAP[group]
            entry = dict(family_row)
            entry["date"] = holiday["date"]
            entry["name"] = holiday["name"]
            derive_group(entry, group, crate_families, folded, sink)

    os.makedirs(args.out, exist_ok=True)
    plan = {"window": [WINDOW[0].isoformat(), WINDOW[1].isoformat()],
            "families": {}}
    summary = {"window": WINDOW_CELL, "families": {}, "total_rows": 0,
               "documents": 0, "conflicts": sink["_conflicts"],
               "fold": sorted(sink["_fold"],
                              key=lambda f: (f["group"], f["trade_date"])),
               "drops": len(sink["_drops"]), "questions": sink["_questions"],
               "no_window": NO_WINDOW, "sheet_label": SHEET_LABEL}
    used_codes = set()

    for family in FAMILIES:
        rows = merge_family(sink[family], family, sink)
        note_by_date = {r.trade_date: None for r in rows}
        used = {row.document for row in rows}
        plan_rows = []
        for row in rows:
            plan_rows.append({
                "date": row.trade_date.isoformat(),
                "kind": row.kind,
                "open_ssm": row.open_ssm,
                "close_ssm": row.close_ssm,
                "tier": row.tier,
                "document": document_id(row.document, block),
                "reason": row.reason,
            })
        used_codes |= used
        plan["families"][family] = plan_rows
        summary["families"][family] = {
            "rows": len(rows),
            "kinds": dict(collections.Counter(r.kind for r in rows)),
            "tiers": dict(collections.Counter(r.tier for r in rows)),
            "grid": grid_at(family, dt.date(2013, 6, 3)).name,
        }
        summary["total_rows"] += len(rows)

        with open(os.path.join(args.out, "%s.rows.rs" % family), "w") as handle:
            handle.write("\n".join(
                render_rust(r, block) for r in rows) + "\n")
        with open(os.path.join(args.out, "%s.evidence.md" % family), "w") as handle:
            handle.write("\n\n".join(
                render_evidence(rows, year, note_by_date, block)
                for year in (2013, 2014, 2015)) + "\n")

    documents = document_rows(block, used_codes)
    summary["documents"] = len(documents)
    with open(os.path.join(args.out, "documents.md"), "w") as handle:
        handle.write(render_documents(documents) + "\n")
    with open(os.path.join(args.out, "ROWS.json"), "w") as handle:
        json.dump(plan, handle, indent=1)
        handle.write("\n")
    with open(os.path.join(args.out, "SUMMARY.json"), "w") as handle:
        json.dump(summary, handle, indent=1)
        handle.write("\n")

    print("window %s" % WINDOW_CELL)
    for family in FAMILIES:
        info = summary["families"][family]
        print("  %-24s %3d rows  %s" % (family, info["rows"], info["kinds"]))
    print("total %d rows, %d documents, %d drops, %d conflicts, %d questions"
          % (summary["total_rows"], summary["documents"], summary["drops"],
             len(summary["conflicts"]), len(summary["questions"])))
    for question in summary["questions"]:
        print("  QUESTION %s" % question)
    for conflict in summary["conflicts"]:
        print("  CONFLICT %s" % conflict)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
