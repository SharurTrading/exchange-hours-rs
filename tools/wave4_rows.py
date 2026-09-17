#!/usr/bin/env python3
# SPDX-License-Identifier: MIT-0
"""wave4_rows.py -- turn the repaired cme-2019-2021 block into crate holiday rows.

The crate ships one static holiday table per CME product family, keyed by the
crate's own venue-local **trade date** (LAW-HOLIDAY-SCOPE, design memo D1). This
tool reads

  * the repaired wave-4 evidence block
    (`holidays/cme-2019-2021.r2.json`, the round-2 block the repair tool wrote),
  * the raw-artifact index (`holidays/raw/cme-2019-2021/INDEX.md`) and hash
    manifest (`shasum.txt`), which resolve every cited sheet to a URL, a
    capture, a sha256 and a tier, and
  * the saved `xlrd` text dumps (`holidays/raw/cme-2019-2021/text/`), which the
    repaired block's `verbatim` fields are quoted from,

and emits, for the eight crate families over venue-local trade dates
2019-01-01 .. 2021-12-31:

  tools/out/wave4/<family>.rows.rs       rows ready to paste into a `holidays!` table
  tools/out/wave4/<family>.evidence.md   the per-family, per-year evidence table
  tools/out/wave4/documents.md           one row per document id used by a row
  tools/out/wave4/DECISIONS.md           the grids, conversions, drops, questions
  tools/out/wave4/ROWS.json              the encoding plan (machine readable)

Design rules implemented here:

  * Only a date/status that changes an answer ships a row.  `normal` never
    ships; a printed instant equal to the family's ordinary instant for this
    era never ships.
  * Rows are keyed to the crate's venue-local trade date, never to the
    operator's event date.  A holiday's own event date is the trade date whose
    final close falls on it, because the sheet prints that date's close; a
    reopen printed for the *next* trade date is keyed to that date instead.
  * `closed` -> `Closed`;  `early_close` -> `early_close(<printed close>)`;
    `modified` -> decided from the printed instants; a printed open later than
    the trade date's ordinary first open -> `late_open(<printed open>)`.
  * The day-after rule: a closure that also removed the evening leg which would
    have carried the *next* trade date leaves that trade date's first open at
    the operator's printed day-session time, which is a `LateOpen` there.  Where
    that date also prints an early close the two merge into
    `late_open_and_early_close`.
  * Juneteenth 2019, 2020 and 2021 ship `Unsourced` in every family: no CME
    document covers those dates, and inside a contiguous window silence would
    read as audited normal — false for a date the operator later marks as a
    holiday.  The id cites the enumeration that proves the absence.
  * `dairy` and `lumber`, which CME prints on the same sheets but which have no
    crate key, are folded for *reporting* only: `dairy` against `globex_grains`
    and `lumber` against `globex_livestock`.  Their own rows are compared and
    every divergence is named in the crate family's evidence file; their
    instants never key a row (they run on their own clocks).

Stdlib only.  Deterministic: same bytes in, same text out.
"""

from __future__ import annotations

import argparse
import collections
import datetime as dt
import hashlib
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
    "globex_cryptocurrency",
    "globex_nikkei_225_dollar",
)

#: CME product groups the block carries that are not crate keys.  They are
#: printed on the same sheets and folded here for reporting only.
FOLD = {"dairy_same_page": "globex_grains", "lumber_same_page": "globex_livestock"}

GROUPS = FAMILIES + tuple(FOLD)

WINDOW = (dt.date(2019, 1, 1), dt.date(2021, 12, 31))

SECONDS_PER_DAY = 86_400

#: The Juneteenth dates that ship `Unsourced` in every family, each cited to
#: the operator's own consolidated bundle for its own year: that bundle's member
#: list is CME's account of every Globex holiday schedule it published that
#: year, and none of the three carries a Juneteenth sheet.  The 2021 date is a
#: Saturday and no family has a trade date there, so its row clips nothing; it
#: is keyed to the operator's own calendar date for the holiday rather than to
#: an observed date CME never states.
JUNETEENTH = {
    dt.date(2019, 6, 19): "2019-holiday-calendars.zip @2021-01-26T09:48:37Z",
    dt.date(2020, 6, 19): "2020-holiday-calendars.zip @2026-07-30T11:18:34Z",
    dt.date(2021, 6, 19): "2021-holiday-calendars.zip @2026-08-30T10:03:27Z",
}

#: Columbus Day and Veterans Day: named gaps, never rows.  CME published only
#: settlement and clearing advisories for these dates in these years.
NAMED_GAPS = [
    ("2019-10-14", "Columbus Day"), ("2019-11-11", "Veterans Day"),
    ("2020-10-12", "Columbus Day"), ("2020-11-11", "Veterans Day"),
    ("2021-10-11", "Columbus Day"), ("2021-11-11", "Veterans Day"),
]


class Grid:
    """The ordinary week of one product group over 2019-2021.

    ``wrapped`` is true when the trading day's first executable open falls on
    the preceding local date, which is what makes the crate's `LateOpen` rule
    interpret a value below ``first_open`` on the trade date itself.
    """

    def __init__(self, name, wrapped, first_open, close, description, source,
                 close_note="16:00 CT", day_open=None):
        self.name = name
        self.close_note = close_note
        self.wrapped = wrapped
        self.first_open = first_open
        self.close = close  # callable(date) -> ordinary final-close ssm
        self.description = description
        self.source = source
        #: The ordinary open of the matching day session, where the operator's
        #: own sheets print a reopen cell that names that session and no clock.
        self.day_open = day_open


def _close_1600(_date):
    return 16 * 3600


WRAPPED_1700_1600 = (
    "one wrapped leg per trade date, 17:00 CT the previous evening into a "
    "16:00 CT close on the trade date"
)
GRAINS_GRID = (
    "19:00 CT the previous evening into 07:45 CT, then the 08:30-13:20 CT day "
    "session whose close is the trading day's final closing phase"
)
LIVESTOCK_GRID = (
    "one flat Monday-Friday 08:30-13:05 CT session inside a single civil day, "
    "with a 06:00 CT pre-open until 2020-05-31 and an 08:00 CT one after it"
)

#: Ordinary instants (+ where they were read) per product group, 2019-2021.
GRIDS = {
    # cme_group.rs CME_PROFILE_2015_09_20 (2015-09-20, notice 20150817) until
    # 2021-06-27, then CME_PROFILE_DATED_CURRENT (notice 20210621); both close
    # at 16:00 CT, only the 15:15-15:30 halt differs.
    "globex_equity_index": Grid(
        "globex_equity_index", True, 17 * 3600, _close_1600,
        WRAPPED_1700_1600,
        "cme_group.rs CME_PROFILE_2015_09_20 (2015-09-20, notice 20150817) / "
        "CME_PROFILE_DATED_CURRENT (2021-06-27, notice 20210621)",
    ),
    # energy_metals.rs ENERGY_METALS_DATED_CURRENT, revision 2015-09-20.
    "globex_energy": Grid(
        "globex_energy", True, 17 * 3600, _close_1600, WRAPPED_1700_1600,
        "energy_metals.rs ENERGY_METALS_DATED_CURRENT (2015-09-20, notice 20150907)",
    ),
    # fx.rs DATED_CURRENT, revision 2010-11-15 (grid unchanged since).
    "globex_fx": Grid(
        "globex_fx", True, 17 * 3600, _close_1600, WRAPPED_1700_1600,
        "fx.rs DATED_CURRENT (2010-11-15, notice 20101025)",
    ),
    # grains.rs DATED_CURRENT, revision 2015-07-05 (CME SER-7395R).
    "globex_grains": Grid(
        "globex_grains", True, 19 * 3600, lambda _date: 13 * 3600 + 20 * 60,
        GRAINS_GRID,
        "grains.rs DATED_CURRENT (2015-07-05, CME SER-7395R)",
        close_note="13:20 CT",
        day_open=8 * 3600 + 30 * 60,
    ),
    # interest_rates.rs PROFILE_2011_10_02, revision 2011-10-02.
    "globex_interest_rates": Grid(
        "globex_interest_rates", True, 17 * 3600, _close_1600, WRAPPED_1700_1600,
        "interest_rates.rs PROFILE_2011_10_02 (2011-10-02, notice 20110926)",
    ),
    # livestock.rs PROFILE_2016_06_06 / PROFILE_CURRENT (2020-05-31, SER-8599R):
    # the regular 08:30-13:05 CT session is unchanged across the era.
    "globex_livestock": Grid(
        "globex_livestock", False, 8 * 3600 + 30 * 60,
        lambda _date: 13 * 3600 + 5 * 60, LIVESTOCK_GRID,
        "livestock.rs PROFILE_2016_06_06 / PROFILE_CURRENT (2020-05-31, CME SER-8599R)",
        close_note="13:05 CT",
        day_open=8 * 3600 + 30 * 60,
    ),
    # cryptocurrency.rs FIVE_DAY, revision 2017-12-17 (CME SER-8051R).
    "globex_cryptocurrency": Grid(
        "globex_cryptocurrency", True, 17 * 3600, _close_1600, WRAPPED_1700_1600,
        "cryptocurrency.rs FIVE_DAY (2017-12-17, CME SER-8051R)",
    ),
    # cme_nikkei.rs NKD_CURRENT, revision 2015-09-20.
    "globex_nikkei_225_dollar": Grid(
        "globex_nikkei_225_dollar", True, 17 * 3600, _close_1600,
        WRAPPED_1700_1600,
        "cme_nikkei.rs NKD_CURRENT (2015-09-20, notice 20150817)",
    ),
    # Folded groups, described on their own published clock.  Only a status is
    # ever reported for them; their instants never key a crate row.
    "dairy_same_page": Grid(
        "dairy_same_page", True, 17 * 3600,
        lambda date: (13 * 3600 + 55 * 60) if date.weekday() == 4 else 16 * 3600,
        "one wrapped leg per trade date, 17:00 CT the previous evening into a "
        "16:00 CT close (13:55 CT on Fridays)",
        "block dairy rows (CME's Dairy product-group line); not a crate key",
        close_note="16:00 CT (13:55 CT on Fridays)",
    ),
    "lumber_same_page": Grid(
        "lumber_same_page", False, 9 * 3600, lambda _date: 15 * 3600 + 5 * 60,
        "one flat Monday-Friday 09:00-15:05 CT session inside a single civil "
        "day, with a 06:00 CT pre-open",
        "block lumber rows (CME's Lumber product-group line); not a crate key",
        close_note="15:05 CT",
        day_open=9 * 3600,
    ),
}

#: The label the compact sheet prints for each family's row. The evidence
#: tables name it, so a row that cites a combined Globex holiday sheet — every
#: compact sheet carries one line per product group — says which printed line
#: its instant comes from.
SHEET_LABEL = {
    "globex_equity_index": "Equity",
    "globex_energy": "Energy, Metals & DME",
    "globex_fx": "FX",
    "globex_grains": "Grain & Oilseed",
    "globex_interest_rates": "Interest Rate",
    "globex_livestock": "Livestock",
    "globex_cryptocurrency": "Bitcoin",
    "globex_nikkei_225_dollar": "Equity",
    "dairy_same_page": "Dairy",
    "lumber_same_page": "Lumber Futures&Options",
}

# --------------------------------------------------------------------------
# Document codes -> the crate's document id
# --------------------------------------------------------------------------

#: The annual bundle each document code belongs to, with its capture.
BUNDLE = {
    "2019": ("2019-holiday-calendars.zip", "2021-01-26T09:48:37Z", "20210126094837",
             "1e861e355238903b013c1288f4eb9e8026e6ddd5fc1001dfd2acdbfcc1832e05",
             "globex-trading-schedules/"),
    "2020": ("2020-holiday-calendars.zip", "2026-07-30T11:18:34Z", "20260730111834",
             "5263a4a5e9076bc0e69c7cd0e3fd9f82dc4d80b66f1b56f14070f08c1d762b59", ""),
    "2021": ("2021-holiday-calendars.zip", "2026-08-30T10:03:27Z", "20260830100327",
             "0ee0860a3a0e035eb9d079419aca3cafcc4256d296fa916647987c396dda8c59", ""),
}
BUNDLE_URL = (
    "https://www.cmegroup.com/tools-information/holiday-calendar/files/%s"
)

#: code -> (bundle year, compact member name).  The band 2021-ny belongs to the
#: 2020 bundle (its sheet covers 31 Dec 2020 into January 2021) and 2022-ny to
#: the 2021 one, exactly as the block's DOCUMENT-ID LEGEND says.
CODE_MEMBER = {
    "2019-mlk": ("2019", "2019-martin-luther-king-holiday-schedule-compact.xls"),
    "2019-presidents": ("2019", "2019-presidents-day-holiday-schedule-compact.xls"),
    "2019-goodfri": ("2019", "2019-good-friday-holiday-compact.xls"),
    "2019-memorial": ("2019", "2019-memorial-day-holiday-schedule-compact.xls"),
    "2019-july4": ("2019", "2019-4th-of-july-holiday-schedule-compact.xls"),
    "2019-labor": ("2019", "2019-labor-day-holiday-schedule-compact.xls"),
    "2019-thx": ("2019", "2019-thanksgiving-holiday-schedule-compact.xls"),
    "2019-xmas": ("2019", "2019-christmas-holiday-schedule-compact.xls"),
    "2019-ny": ("2019", "2019-new-years-holiday-schedule-compact.xls"),
    "2020-mlk": ("2020", "2020-martin-luther-king-holiday-schedule-compact.xls"),
    "2020-presidents": ("2020", "2020-presidents-day-holiday-schedule-compact.xls"),
    "2020-goodfri": ("2020", "2020-good-friday-holiday-compact.xls"),
    "2020-memorial": ("2020", "2020-memorial-day-holiday-schedule-compact.xls"),
    "2020-july4": ("2020", "2020-4th-of-july-holiday-schedule-compact.xls"),
    "2020-labor": ("2020", "2020-labor-day-holiday-schedule-compact.xls"),
    "2020-thx": ("2020", "2020-thanksgiving-holiday-schedule-compact.xls"),
    "2020-xmas": ("2020", "2020-christmas-holiday-schedule-compact.xls"),
    "2021-ny": ("2020", "2021-new-years-holiday-schedule-compact.xls"),
    "2021-mlk": ("2021", "2021-mlk-day-schedule-compact.xls"),
    "2021-presidents": ("2021", "2021-presidents-day-holiday-schedule-compact.xls"),
    "2021-goodfri": ("2021", "2021-good-friday-holiday-schedule-compact.xls"),
    "2021-memorial": ("2021", "2021-memorial-day-holiday-schedule-compact.xls"),
    "2021-july4": ("2021", "2021-independence-day-holiday-schedule-compact.xls"),
    "2021-labor": ("2021", "2021-labor-day-holiday-schedule-compact.xls"),
    "2021-thx": ("2021", "2021-thanksgiving-holiday-schedule-compact.xls"),
    "2021-xmas": ("2021", "2021-christmas-holiday-schedule-compact.xls"),
    "2022-ny": ("2021", "2022-new-years-holiday-schedule-compact.xls"),
}

#: The December-2018 supplement, retrieved on its own.
SUPPLEMENT = {
    "code": "2019-ny(DEC2018)",
    "id": "2019-new-years-holiday-schedule-compact.xls @2018-01-07T04:13:43Z",
    "window": "2019-01-01 .. 2021-12-31",
    "url": "https://web.archive.org/web/20180107041343id_/"
           "http://www.cmegroup.com/tools-information/holiday-calendar/files/"
           "2019-new-years-holiday-schedule-compact.xls",
    "capture": "archive capture 2018-01-07T04:13:43Z",
    "tier": "T1",
    "sha256": "2684a5f1b3a9f65802f6911ca6089e2cb68c3cdf2520dfaf3cdcf3105328c188",
    "file": "docs/2019-new-years-compact-JAN2019.xls",
}

WINDOW_CELL = "2019-01-01 .. 2021-12-31"

# --------------------------------------------------------------------------
# Small helpers
# --------------------------------------------------------------------------

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


def sha256_of(path):
    with open(path, "rb") as handle:
        return hashlib.sha256(handle.read()).hexdigest()


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


def parse_clock(text):
    """The clock token of a printed cell, as seconds since midnight.

    Handles both CME spellings — `1700 CT / 2300 UTC` and `12:00` — and, where
    the cell prints a pre-open beside the open (`Pre-open 6:00CT /12:00 UTC Open
    8:30 CT /14:30 UTC`, `Monday 28 Dec, Open 8:30 CT`), reads the open: a
    standalone capital `Open` names the matching phase and the token after it,
    while `Pre-open` / `Pre Open` is order entry and never the boundary.  The
    value returned is the Central Time token, which every one of these sheets
    prints first.
    """
    if not text:
        return None
    tail = text
    match = re.search(r"\bOpen\b", text)
    if match:
        tail = text[match.end():]
    clock = re.search(r"(\d{1,2}):(\d{2})", tail)
    if clock:
        return int(clock.group(1)) * 3600 + int(clock.group(2)) * 60
    token = re.search(r"\b(\d{3,4})\b", tail)
    if token:
        digits = token.group(1).zfill(4)
        hours, minutes = int(digits[:2]), int(digits[2:])
        if hours < 24 and minutes < 60:
            return hours * 3600 + minutes * 60
    return None


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


def parse_day_label(text, anchor):
    """The local date a printed open names, if it names one at all."""
    if not text:
        return None
    match = re.search(
        r"\b(mon|tue|wed|thu|fri|sat|sun)[a-z]*\.?,?\s+(\d{1,2})\s+" + MONTH_RE,
        text, re.I)
    if match:
        return resolve_year(MONTHS[match.group(3)[:3].lower()], int(match.group(2)),
                            anchor)
    match = re.search(
        r"\b(mon|tue|wed|thu|fri|sat|sun)[a-z]*\.?,?\s+" + MONTH_RE + r"\.?\s+(\d{1,2})\b",
        text, re.I)
    if match:
        return resolve_year(MONTHS[match.group(2)[:3].lower()], int(match.group(3)),
                            anchor)
    return None


def printed_token(cell, ssm):
    """The exact printed fragment carrying ``ssm`` in ``cell``."""
    if cell:
        for match in re.finditer(r"\d{1,2}:\d{2}", cell):
            if parse_clock(match.group(0)) == ssm:
                return match.group(0)
        for match in re.finditer(r"\b\d{3,4}\b", cell):
            if parse_clock(match.group(0)) == ssm:
                return match.group(0)
    return ssm_token(ssm)


# --------------------------------------------------------------------------
# Derived row model
# --------------------------------------------------------------------------

class Row:
    """One derived crate row, before rendering."""

    def __init__(self, trade_date, group, kind, close_ssm=None, open_ssm=None,
                 entry=None, entry_date=None, role="", rationale="",
                 printed_open=None, printed_close=None, clause=None,
                 unsourced_reason=""):
        self.trade_date = trade_date
        self.group = group
        self.kind = kind  # Closed | EarlyClose | LateOpen | LateOpenAndEarlyClose | Unsourced
        self.close_ssm = close_ssm
        self.open_ssm = open_ssm
        self.entry = entry or {}
        self.entry_date = entry_date if entry_date is not None else trade_date
        self.role = role
        self.rationale = rationale
        self.printed_open = printed_open
        self.printed_close = printed_close
        self.clause = clause
        self.unsourced_reason = unsourced_reason
        self.doc_id = None

    def key(self):
        return (self.kind, self.open_ssm, self.close_ssm)

    def __repr__(self):
        return "Row(%s, %s, %s, open=%s, close=%s)" % (
            self.trade_date, self.group, self.kind, self.open_ssm, self.close_ssm)


def closure_token(entry, group):
    """The operator's own closure phrase for a row that prints no clock."""
    text = entry.get("verbatim", "")
    cells = [cell.strip() for cell in text.split(" | ")]
    for cell in cells:
        if re.search(r"\bClosed\b", cell):
            return cell
    return entry.get("status", "closed")


def derive_group(entry, group, block_date, sink):
    """Derive every row one block entry states, plus its drops and questions."""
    grid = GRIDS[group]
    status = entry["status"]
    close_ssm = parse_clock(entry.get("close_instant"))
    open_ssm = parse_clock(entry.get("open_instant"))
    ordinary_close = grid.close(block_date)

    def record_close():
        """Return (shipped_ssm, drop_reason)."""
        if close_ssm is None:
            return None, "close cell carries no clock token (prose only)"
        if close_ssm == ordinary_close:
            return None, ("printed close %s CT equals the family's ordinary %s CT "
                          "final close"
                          % (ssm_token(close_ssm), ssm_token(ordinary_close)))
        if close_ssm > ordinary_close:
            sink["questions"].append(
                "%s %s: printed close %s CT is later than the ordinary %s CT "
                "final close and no late-close kind exists; the cell is %r."
                % (block_date, group, ssm_token(close_ssm), ssm_token(ordinary_close),
                   entry.get("close_instant")))
            return None, "printed close is later than the ordinary final close"
        return close_ssm, None

    def record_open():
        """Return (trade_date, ssm, note) for a boundary-moving open."""
        cell = entry.get("open_instant") or ""
        label = parse_day_label(cell, block_date)
        # A `late_open` entry is the operator's statement about its own trade
        # date; every other entry's printed open belongs to the next one.
        if status == "late_open":
            target = block_date
        else:
            target = label or next_weekday(block_date)
        if open_ssm is None:
            if status == "closed" and label and re.search(r"\bRegular\b", cell):
                if grid.day_open is None:
                    sink["questions"].append(
                        "%s %s: the reopen cell names the day session as regular "
                        "but this family's ordinary day-session open is not "
                        "modelled: %r" % (block_date, group, cell))
                    return None, None, "day-session open not modelled"
                if grid.wrapped:
                    return target, grid.day_open, (
                        "the operator's reopen cell names the day session as "
                        "regular on trade date %s, so the first trade there is the "
                        "ordinary %s CT day-session open — later than the family's "
                        "%s CT first open, which falls on the preceding local date"
                        % (iso(target), ssm_token(grid.day_open),
                           ssm_token(grid.first_open)))
                return target, None, (
                    "the operator's reopen cell names the day session as regular "
                    "on trade date %s, and this flat-grid family's ordinary open "
                    "is already that day session's own" % iso(target))
            return None, None, None
        if open_ssm >= grid.first_open:
            return target, None, (
                "printed open %s CT is the family's ordinary %s CT first open for "
                "the next trade date, so nothing moves"
                % (ssm_token(open_ssm), ssm_token(grid.first_open)))
        if not grid.wrapped and open_ssm == grid.first_open:
            return target, None, (
                "printed open %s CT is this flat-grid family's ordinary first open "
                "on the trade date itself" % ssm_token(open_ssm))
        if not grid.wrapped and open_ssm < grid.first_open:
            sink["questions"].append(
                "%s %s: printed open %s CT is earlier than the ordinary %s CT "
                "matching open on a flat grid; read as an order-entry-only "
                "deviation, which the scalar holiday vocabulary cannot state."
                % (block_date, group, ssm_token(open_ssm), ssm_token(grid.first_open)))
            return None, None, "open is an order-entry-only deviation"
        note = ("the prior-evening leg of trade date %s did not run: the "
                "operator prints the day session's own %s CT open, later than the "
                "ordinary %s CT first open"
                % (iso(target), ssm_token(open_ssm), ssm_token(grid.first_open)))
        return target, open_ssm, note

    if status == "normal":
        if close_ssm is not None and close_ssm != ordinary_close:
            sink["questions"].append(
                "%s %s: status `normal` but the entry prints close %r against the "
                "ordinary %s CT final close; no row ships."
                % (block_date, group, entry.get("close_instant"),
                   ssm_token(ordinary_close)))
        elif open_ssm is not None and open_ssm != grid.first_open and open_ssm >= grid.first_open:
            sink["questions"].append(
                "%s %s: status `normal` with a printed open %r later than the "
                "ordinary %s CT first open; no row ships."
                % (block_date, group, entry.get("open_instant"),
                   ssm_token(grid.first_open)))
        sink["drops"].append((block_date, group, "status normal", entry))
        return

    if status == "closed":
        sink["rows"].append(Row(
            block_date, group, "Closed", entry=entry, role="closed",
            entry_date=block_date,
            rationale="CME prints no session running through this date",
            printed_close=closure_token(entry, group)))
    else:
        shipped_close, drop_reason = record_close()
        if shipped_close is not None:
            sink["rows"].append(Row(
                block_date, group, "EarlyClose", close_ssm=shipped_close,
                entry=entry, role=status, entry_date=block_date,
                rationale="the printed final close %s CT is earlier than the "
                          "ordinary %s CT close"
                          % (ssm_token(shipped_close), ssm_token(ordinary_close)),
                printed_close=entry.get("close_instant")))
        elif drop_reason:
            sink["drops"].append((block_date, group, drop_reason, entry))

    target, shipped_open, note = record_open()
    if shipped_open is not None and target is not None:
        sink["rows"].append(Row(
            target, group, "LateOpen", open_ssm=shipped_open, entry=entry,
            role="reopen-of-" + status, entry_date=block_date, rationale=note,
            printed_open=entry.get("open_instant")))
    elif note:
        sink["drops"].append((target or block_date, group, "reopen: " + note, entry))


def merge_family(group_rows, sink):
    """Collapse ``{trade_date: [Row]}`` into one row per trade date."""
    merged = {}
    conflicts = []
    for trade_date, rows in sorted(group_rows.items()):
        closed = [row for row in rows if row.kind == "Closed"]
        closes = [row for row in rows if row.kind == "EarlyClose"]
        opens = [row for row in rows if row.kind == "LateOpen"]
        unsourced = [row for row in rows if row.kind == "Unsourced"]
        close_values = {row.close_ssm for row in closes}
        open_values = {row.open_ssm for row in opens}
        if len(close_values) > 1:
            conflicts.append((trade_date, "two early-close instants: %s"
                              % sorted(ssm_token(value) for value in close_values)))
            continue
        if len(open_values) > 1:
            conflicts.append((trade_date, "two late-open instants: %s"
                              % sorted(ssm_token(value) for value in open_values)))
            continue
        close = closes[0] if closes else None
        late = opens[0] if opens else None
        if unsourced:
            merged[trade_date] = unsourced[0]
            continue
        if closed and (close or late):
            conflicts.append((trade_date, "a closure and a boundary move on the "
                                          "same trade date"))
            merged[trade_date] = closed[0]
            continue
        if closed:
            merged[trade_date] = closed[0]
        elif close and late:
            merged[trade_date] = Row(
                trade_date, close.group, "LateOpenAndEarlyClose",
                close_ssm=close.close_ssm, open_ssm=late.open_ssm,
                entry=close.entry, role="combined", entry_date=close.entry_date,
                rationale=late.rationale + "; " + close.rationale,
                printed_open=late.printed_open, printed_close=close.printed_close)
        elif close:
            merged[trade_date] = close
        elif late:
            merged[trade_date] = late
    return merged, conflicts


# --------------------------------------------------------------------------
# Document ids
# --------------------------------------------------------------------------

def load_shasum(path):
    """`{extracted path: sha256}` from the raw store's manifest."""
    hashes = {}
    with open(path, "r", encoding="utf-8") as handle:
        for line in handle:
            if line.startswith("#"):
                continue
            parts = line.split()
            if len(parts) == 2:
                hashes[parts[1]] = parts[0]
    return hashes


def document_for(code, hashes):
    """The crate document id, URL, capture, tier and sha256 for a code."""
    if code == SUPPLEMENT["code"]:
        return dict(SUPPLEMENT)
    if code not in CODE_MEMBER:
        raise SystemExit("no member mapping for document code %r" % code)
    year, member = CODE_MEMBER[code]
    bundle, capture, stamp, bundle_sha, prefix = BUNDLE[year]
    extracted = "zip%s/%s%s" % (year, prefix, member)
    if extracted not in hashes:
        raise SystemExit("%s is not in shasum.txt" % extracted)
    return {
        "code": code,
        "id": "%s#%s%s @%s" % (bundle, prefix, member, capture),
        "window": WINDOW_CELL,
        "url": "https://web.archive.org/web/%sid_/%s"
               % (stamp, BUNDLE_URL % bundle),
        "capture": "archive capture %s" % capture,
        "tier": "T1",
        "sha256": hashes[extracted],
        "file": extracted,
        "member": member,
        "bundle_sha256": bundle_sha,
    }


# --------------------------------------------------------------------------
# Rendering
# --------------------------------------------------------------------------

def instant_cell(row):
    """The `instant as printed` evidence cell: operator tokens only.

    A quoted full-sheet row carries CME's own cell separator, a vertical bar,
    which would split this pipe-separated table; where that happens the bar is
    written `\u00b7` and named inside the quotation, so the cell stays checkable
    against the block.
    """

    def sanitize(value):
        if " | " in value:
            return (value.replace(" | ", " \u00b7 ")
                    + " (CME's cell separator is written as U+00B7 here)")
        return value

    if row.kind == "Unsourced":
        return "no CME document covers this date (see `%s`)" % row.doc_id
    printed = []
    if row.open_ssm is not None:
        printed.append(row.printed_open or (ssm_token(row.open_ssm) + " CT"))
    if row.close_ssm is not None:
        printed.append(row.printed_close or (ssm_token(row.close_ssm) + " CT"))
    if not printed:
        printed = [row.printed_close or "closed"]
    return " / ".join("`%s`" % sanitize(value) for value in printed)


def kind_words(row):
    if row.kind == "Closed":
        return "closed: no trade date"
    if row.kind == "Unsourced":
        return "unsourced: no operator document covers this date"
    if row.kind == "EarlyClose":
        return "early close %s CT" % ssm_token(row.close_ssm)
    if row.kind == "LateOpen":
        return "late open %s CT" % ssm_token(row.open_ssm)
    return "late open %s CT and early close %s CT" % (
        ssm_token(row.open_ssm), ssm_token(row.close_ssm))


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


def kind_rust(row):
    if row.kind == "Closed":
        return "Closed"
    if row.kind == "Unsourced":
        return "Unsourced"
    if row.kind == "EarlyClose":
        return "early_close(%s)" % ssm_expr(row.close_ssm)
    if row.kind == "LateOpen":
        return "late_open(%s)" % ssm_expr(row.open_ssm)
    return "late_open_and_early_close(%s, %s)" % (
        ssm_expr(row.open_ssm), ssm_expr(row.close_ssm))


def derived_from(row, grid):
    """One sentence saying how the trade date and the kind follow."""
    if row.kind == "Unsourced":
        return row.unsourced_reason
    if row.kind == "Closed":
        if grid.wrapped:
            return (
                "CME prints the closure on this date, so the session whose final "
                "close would have fallen here — the leg that opened the previous "
                "evening at %s CT — is removed with it, and the crate's trade date "
                "is the operator's own event date." % ssm_token(grid.first_open))
        return (
            "CME prints the closure on this date, and this family's session lies "
            "inside one civil day, so the operator's event date is the crate's "
            "trade date.")
    if row.kind == "EarlyClose":
        if grid.wrapped:
            return (
                "The cited sheet's `%s` line prints `%s CT` as this date's own final "
                "close, so the leg that opened the previous evening at %s CT is "
                "clipped there and the crate's trade date is the date that close "
                "falls on — the operator's event date."
                % (SHEET_LABEL[row.group], ssm_token(row.close_ssm),
                   ssm_token(grid.first_open)))
        return (
            "The cited sheet's `%s` line prints `%s CT` as this date's own final "
            "close inside one civil day, so the operator's event date is the "
            "crate's trade date."
            % (SHEET_LABEL[row.group], ssm_token(row.close_ssm)))
    if row.kind == "LateOpen":
        return (
            "CME's next open for trade date %s is the day session's own `%s CT` on "
            "that date, later than the family's ordinary `%s CT` first open, which "
            "falls on the preceding local date: the leg that would have carried "
            "this trade date did not run. `%s` is earlier than that ordinary open, "
            "so the crate's cutoff lands on the trade date itself."
            % (iso(row.trade_date), ssm_token(row.open_ssm),
               ssm_token(grid.first_open), ssm_token(row.open_ssm)))
    return (
        "CME withdrew the prior-evening leg and printed the day session's own `%s "
        "CT` open beside the `%s CT` final close on trade date %s, so both "
        "boundaries move and the crate keys one row carrying both."
        % (ssm_token(row.open_ssm), ssm_token(row.close_ssm), iso(row.trade_date)))


def render_rust(rows):
    out = []
    for row in rows:
        document = row.doc_id
        reason = row.rationale.strip().rstrip(".")
        out.append("        // %s - %s - %s - %s." % (iso(row.trade_date), row.tier,
                                                      document, reason))
        out.append('        (%d, %d, %d, %s, %s, "%s"),'
                   % (row.trade_date.year, row.trade_date.month, row.trade_date.day,
                      kind_rust(row), row.tier, document))
    return "\n".join(out)


def render_evidence(family, rows):
    """The `### <year>` tables of the family's evidence file, in file shape."""
    blocks = []
    for year in (2019, 2020, 2021):
        table = [
            "### %d" % year, "",
            "| trade date | kind | instant as printed | document | tier | derived from |",
            "|---|---|---|---|---|---|",
        ]
        for row in rows:
            if row.trade_date.year != year:
                continue
            table.append("| %s | %s | %s | `%s` | %s | %s |" % (
                iso(row.trade_date), kind_label(row), instant_cell(row),
                row.doc_id, row.tier, derived_from(row, GRIDS[row.group])))
        blocks.append("\n".join(table))
    return "\n\n".join(blocks)


# --------------------------------------------------------------------------
# Main
# --------------------------------------------------------------------------

def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--research", default=os.environ.get("WAVE4_RESEARCH"))
    parser.add_argument("--out", default="tools/out/wave4")
    arguments = parser.parse_args(argv)
    if not arguments.research:
        raise SystemExit("--research or WAVE4_RESEARCH must name the research store")
    research = arguments.research.rstrip("/")
    store = os.path.join(research, "holidays")
    raw = os.path.join(store, "raw", "cme-2019-2021")
    out = arguments.out.rstrip("/")
    os.makedirs(out, exist_ok=True)

    with open(os.path.join(store, "cme-2019-2021.r2.json"), "r", encoding="utf-8") as handle:
        block = json.load(handle)
    hashes = load_shasum(os.path.join(raw, "shasum.txt"))

    sink = {"rows": [], "drops": [], "questions": []}
    group_rows = collections.defaultdict(lambda: collections.defaultdict(list))
    entries_by_date = collections.defaultdict(dict)
    for holiday in block["holidays"]:
        block_date = dt.date.fromisoformat(holiday["date"])
        for entry in holiday["families"]:
            group = entry["family"]
            entries_by_date[block_date][group] = entry
            before = len(sink["rows"])
            derive_group(entry, group, block_date, sink)
            for row in sink["rows"][before:]:
                group_rows[group][row.trade_date].append(row)

    # Juneteenth: one `Unsourced` row per family per date, cited to the
    # enumeration that shows no CME document covers it.
    for group in GROUPS:
        for date, bundle_id in JUNETEENTH.items():
            group_rows[group][date].append(Row(
                date, group, "Unsourced", entry={"verbatim": "", "status": "unknown"},
                entry_date=date, role="juneteenth",
                unsourced_reason=(
                    "The operator published no Juneteenth schedule for 2019, 2020 "
                    "or 2021: the year's own consolidated bundle `%s` — CME's "
                    "account of every Globex holiday schedule it published that "
                    "year — carries no Juneteenth sheet, the archived "
                    "holiday-calendar.html index pages name none, and a fresh "
                    "2018-2027 prefix CDX enumeration finds no `juneteenth` "
                    "filename before 2022. Silence inside the window would read as "
                    "audited normal on a date the operator later marks as a "
                    "holiday, so the row is `Unsourced`, which clips nothing."
                    % bundle_id)))

    merged_all = {}
    conflicts = []
    for group in GROUPS:
        merged, group_conflicts = merge_family(group_rows[group], sink)
        for date, why in group_conflicts:
            conflicts.append((group, date, why))
        merged_all[group] = merged

    # The fold: reporting only.
    fold_report = []
    for group, family in FOLD.items():
        for date in sorted(set(merged_all[group]) | set(merged_all[family])):
            folded = merged_all[group].get(date)
            crate = merged_all[family].get(date)
            if not WINDOW[0] <= date <= WINDOW[1]:
                continue
            if folded is None and crate is None:
                continue
            fold_report.append({
                "group": group, "family": family, "trade_date": iso(date),
                "folded": kind_words(folded) if folded else None,
                "crate_family": kind_words(crate) if crate else None,
                "status": ("agree" if (folded is None) == (crate is None)
                           and folded is not None and folded.key() == crate.key()
                           else ("group-only" if crate is None
                                 else ("family-only" if folded is None else "differ"))),
            })

    # Resolve document ids for every shipped row.
    documents = {}
    for group in GROUPS:
        if group not in FAMILIES:
            continue
        for date, row in sorted(merged_all[group].items()):
            if row.kind == "Unsourced" and row.role == "juneteenth":
                row.doc_id = JUNETEENTH[row.trade_date]
                row.tier = "T1"
                year = "%d" % row.trade_date.year
                bundle, capture, stamp, bundle_sha, _ = BUNDLE[year]
                documents[row.doc_id] = {
                    "id": row.doc_id, "window": WINDOW_CELL,
                    "url": "https://web.archive.org/web/%sid_/%s"
                           % (stamp, BUNDLE_URL % bundle),
                    "capture": "archive capture %s" % capture,
                    "tier": "T1", "sha256": bundle_sha,
                }
                continue
            document = document_for(row.entry["document"], hashes)
            row.doc_id = document["id"]
            row.tier = document["tier"]
            documents[document["id"]] = document

    rows_by_family = {
        family: [merged_all[family][date] for date in sorted(merged_all[family])]
        for family in FAMILIES
    }

    # Outputs.
    counts = {}
    for family in FAMILIES:
        rows = rows_by_family[family]
        kinds = collections.Counter(row.kind for row in rows)
        counts[family] = {"rows": len(rows), "kinds": dict(kinds),
                          "tiers": dict(collections.Counter(row.tier for row in rows))}
        with open(os.path.join(out, "%s.rows.rs" % family), "w", encoding="utf-8") as handle:
            handle.write(render_rust(rows) + "\n")
        with open(os.path.join(out, "%s.evidence.md" % family), "w", encoding="utf-8") as handle:
            handle.write(render_evidence(family, rows) + "\n")

    with open(os.path.join(out, "documents.md"), "w", encoding="utf-8") as handle:
        handle.write("| Document | Window | Replay or service URL | Capture or retrieval, "
                     "UTC | Tier | sha256 |\n|---|---|---|---|---|---|\n")
        for document in sorted(documents.values(), key=lambda item: item["id"]):
            handle.write("| `%s` | %s | <%s> | %s | %s | `%s` |\n" % (
                document["id"], document["window"], document["url"],
                document["capture"], document["tier"], document["sha256"]))

    summary = {
        "window": [iso(WINDOW[0]), iso(WINDOW[1])],
        "families": counts,
        "total_rows": sum(item["rows"] for item in counts.values()),
        "documents": len(documents),
        "conflicts": [{"group": group, "date": iso(date), "why": why}
                      for group, date, why in conflicts],
        "fold": fold_report,
        "drops": len(sink["drops"]),
        "questions": sink["questions"],
    }
    with open(os.path.join(out, "SUMMARY.json"), "w", encoding="utf-8") as handle:
        json.dump(summary, handle, indent=2, ensure_ascii=False)
        handle.write("\n")
    with open(os.path.join(out, "ROWS.json"), "w", encoding="utf-8") as handle:
        json.dump({
            "window": [iso(WINDOW[0]), iso(WINDOW[1])],
            "families": {
                family: [{
                    "date": iso(row.trade_date), "kind": row.kind,
                    "open_ssm": row.open_ssm, "close_ssm": row.close_ssm,
                    "tier": row.tier, "document": row.doc_id,
                    "reason": row.rationale.strip().rstrip("."),
                } for row in rows_by_family[family]]
                for family in FAMILIES
            },
        }, handle, indent=2, ensure_ascii=False)
        handle.write("\n")

    print("rows per family:", {family: counts[family]["rows"] for family in FAMILIES})
    print("kinds:", {family: counts[family]["kinds"] for family in FAMILIES})
    print("documents:", len(documents), "conflicts:", len(conflicts),
          "drops:", len(sink["drops"]), "questions:", len(sink["questions"]))
    for question in sink["questions"]:
        print("  QUESTION:", question)
    for group, date, why in conflicts:
        print("  CONFLICT:", group, iso(date), why)
    return 0


if __name__ == "__main__":
    sys.exit(main())
