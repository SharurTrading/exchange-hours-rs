#!/usr/bin/env python3
# SPDX-License-Identifier: MIT-0
"""Independent cross-checker for the wave-4 CME holiday tables (2019-2021).

This tool re-derives the crate's 2019-2021 Globex holiday rows from primary
inputs and reports every disagreement.  It shares no code with the generator
(`tools/wave4_rows.py`, which it must not read): its own parser reads CME's
own `.xls` dumps and the repaired evidence block, and its own rules turn a
block entry into an expected crate row.

Primary inputs (all under ``$WAVE4_RESEARCH``)::

    holidays/cme-2019-2021.r2.json                  the repaired evidence block
    holidays/raw/cme-2019-2021/text/*.txt           xlrd dumps of CME's sheets
    holidays/raw/cme-2019-2021/shasum.txt           artifact hashes
    holidays/raw/cme-2019-2021/{zip2019/globex-trading-schedules,zip2020,zip2021}/
    holidays/raw/cme-2019-2021/docs/

Under test (crate at the working directory)::

    src/calendar/schedules/holidays/globex_*.rs          family tables
    src/calendar/schedules/holidays/venues/{cme,cbot,comex,nymex}.rs
    src/calendar/schedules/futures/us/*.rs               ordinary grids
    docs/evidence/<owner>.md                             evidence records
    tests/...                                            committed fences

Run::

    WAVE4_RESEARCH=/path/to/exchange-hours-research python3 tools/check_wave4.py

Exits non-zero when any check fails.
"""

from __future__ import annotations

import hashlib
import json
import os
import re
import sys
from datetime import date, timedelta

# --------------------------------------------------------------------------
# Configuration
# --------------------------------------------------------------------------

FAMILIES = [
    "globex_equity_index",
    "globex_energy",
    "globex_fx",
    "globex_grains",
    "globex_interest_rates",
    "globex_livestock",
    "globex_cryptocurrency",
    "globex_nikkei_225_dollar",
]

FAMILY_MODULE = "src/calendar/schedules/holidays/{0}.rs"
VENUES = {
    "cme": [
        "globex_equity_index",
        "globex_energy",
        "globex_fx",
        "globex_grains",
        "globex_interest_rates",
        "globex_livestock",
    ],
    "cbot": ["globex_grains", "globex_interest_rates"],
    "comex": ["globex_energy"],
    "nymex": ["globex_energy"],
}
VENUE_MODULE = "src/calendar/schedules/holidays/venues/{0}.rs"
FAMILY_TEST = "tests/futures_family_boundaries/holidays_{0}.rs"
VENUE_TEST = "tests/venue_sessions/holidays_cme_venues.rs"

WINDOW = (date(2019, 1, 1), date(2021, 12, 31))

EXPECTED_FAMILY_COUNTS = {
    "globex_equity_index": 36,
    "globex_energy": 35,
    "globex_fx": 35,
    "globex_grains": 42,
    "globex_interest_rates": 35,
    "globex_livestock": 37,
    "globex_cryptocurrency": 36,
    "globex_nikkei_225_dollar": 36,
}
EXPECTED_VENUE_COUNTS = {"cme": 42, "cbot": 42, "comex": 35, "nymex": 35}

TIER = "T1"
BLOCK_TIER = "T1"

# The ordinary grid each family runs in 2019-2021, read from
# `src/calendar/schedules/futures/us/*.rs` at head:
#   * cme_group.rs      CME_REGULAR 08:30-15:15 + CME_EXTENDED_CURRENT
#                       15:15-16:00 / SUN_PLUS_MON_THU 17:00-08:30
#   * energy_metals.rs  ENERGY_METALS_EXTENDED_CURRENT SUN_PLUS_MON_THU 17:00-16:00
#   * fx.rs             MATCHING_GRID SUN_PLUS_MON_THU 17:00-16:00
#   * interest_rates.rs EXTENDED_1700_1600 SUN_PLUS_MON_THU 17:00-16:00
#   * cryptocurrency.rs FIVE_DAY_EXTENDED SUN_PLUS_MON_THU 17:00-16:00
#   * cme_nikkei.rs     NKD_REGULAR_CURRENT SUN_PLUS_MON_THU 17:00-16:00
#   * grains.rs         CBOT_EXTENDED_CURRENT 19:00-07:45 + CBOT_REGULAR_CURRENT
#                       08:30-13:20
#   * livestock.rs      REGULAR_CURRENT MON_FRI 08:30-13:05, no wrapped leg
#
# A wrapped family's first open for trade date D is the preceding calendar
# evening (Sunday for a Monday trade date); livestock's is on D itself.
H17 = 17 * 3600
H19 = 19 * 3600
H0830 = 8 * 3600 + 30 * 60
H1305 = 13 * 3600 + 5 * 60
H1320 = 13 * 3600 + 20 * 60
H1600 = 16 * 3600

ORDINARY = {
    # family: (wrapped, ordinary first open ssm, ordinary final close ssm,
    #          day-session open ssm used by a clockless "Regular" reopen)
    "globex_equity_index": (True, H17, H1600, H0830),
    "globex_energy": (True, H17, H1600, H0830),
    "globex_fx": (True, H17, H1600, H0830),
    "globex_grains": (True, H19, H1320, H0830),
    "globex_interest_rates": (True, H17, H1600, H0830),
    "globex_livestock": (False, H0830, H1305, H0830),
    "globex_cryptocurrency": (True, H17, H1600, H0830),
    "globex_nikkei_225_dollar": (True, H17, H1600, H0830),
}

# Juneteenth: no CME document exists for any of the three years, so each
# family ships `Unsourced` cited to that year's annual bundle.
JUNETEENTH = {
    date(2019, 6, 19): "2019-holiday-calendars.zip @2021-01-26T09:48:37Z",
    date(2020, 6, 19): "2020-holiday-calendars.zip @2026-07-30T11:18:34Z",
    date(2021, 6, 19): "2021-holiday-calendars.zip @2026-08-30T10:03:27Z",
}

# The block's short document codes, and the raw-store files they name.
CODE_KEYWORDS = {
    "ny": ("new-years",),
    "mlk": ("martin-luther-king", "mlk"),
    "presidents": ("presidents",),
    "goodfri": ("good-friday",),
    "memorial": ("memorial",),
    "july4": ("4th-of-july", "independence-day"),
    "labor": ("labor-day",),
    "thx": ("thanksgiving",),
    "xmas": ("christmas",),
}
# `2019-ny(DEC2018)` is the December-2018 supplement, outside the annual ZIP.
SUPPLEMENT = "docs/2019-new-years-compact-JAN2019.xls"

MONTHS = {
    "jan": 1, "feb": 2, "mar": 3, "apr": 4, "may": 5, "jun": 6,
    "jul": 7, "aug": 8, "sep": 9, "sept": 9, "oct": 10, "nov": 11, "dec": 12,
}
WEEKDAYS = ["mon", "tue", "wed", "thu", "fri", "sat", "sun"]

RE_WEEKDAY_DAY_MONTH = re.compile(
    r"\b(Mon|Tue|Wed|Thu|Fri|Sat|Sun)[a-z]*\.?,?\s+(\d{1,2})\s+"
    r"(Jan|Feb|Mar|Apr|May|Jun|Jul|Aug|Sept?|Oct|Nov|Dec)[a-z]*",
    re.I,
)
RE_WEEKDAY_MONTH_DAY = re.compile(
    r"\b(Mon|Tue|Wed|Thu|Fri|Sat|Sun)[a-z]*\.?,?\s+"
    r"(Jan|Feb|Mar|Apr|May|Jun|Jul|Aug|Sept?|Oct|Nov|Dec)[a-z]*\.?,?\s+(\d{1,2})\b",
    re.I,
)
RE_WEEKDAY = re.compile(r"\b(Mon|Tue|Wed|Thu|Fri|Sat|Sun)[a-z]*\b", re.I)
RE_CLOCK = re.compile(r"(\d{1,2}):(\d{2})|(\d{3,4})\b")
RE_OPEN_WORD = re.compile(r"(?<!-)(?<!Pre)\bOpen\b")

# --------------------------------------------------------------------------
# Small date / time helpers
# --------------------------------------------------------------------------


def ssm(hh, mm=0):
    return hh * 3600 + mm * 60


def fmt_ssm(value):
    return "{:02d}:{:02d}".format(value // 3600, (value % 3600) // 60)


def kind_text(kind):
    """Renders an internal kind tuple the way a crate row spells it."""
    if kind is None:
        return "nothing"
    tag = kind[0]
    if tag in ("Closed", "Unsourced"):
        return tag
    if tag == "EarlyClose":
        return "early_close({})".format(fmt_ssm(kind[1]))
    if tag == "LateOpen":
        return "late_open({})".format(fmt_ssm(kind[1]))
    return "late_open_and_early_close({}, {})".format(fmt_ssm(kind[1]), fmt_ssm(kind[2]))


def next_business_day(day):
    candidate = day + timedelta(days=1)
    while candidate.weekday() >= 5:
        candidate += timedelta(days=1)
    return candidate


def next_weekday(day, weekday_index):
    delta = (weekday_index - day.weekday()) % 7
    return day + timedelta(days=delta if delta else 7)


def nearest_year(month, day_number, reference):
    best = None
    for year in (reference.year - 1, reference.year, reference.year + 1):
        try:
            candidate = date(year, month, day_number)
        except ValueError:
            continue
        distance = abs((candidate - reference).days)
        if best is None or distance < best[0] or (distance == best[0] and candidate > best[1]):
            best = (distance, candidate)
    return None if best is None else best[1]


def ordinary_first_open(family, trade_date):
    """The instant a normal week's first open carries `trade_date`."""
    wrapped, open_ssm, _, _ = ORDINARY[family]
    if wrapped:
        return (trade_date - timedelta(days=1), open_ssm)
    return (trade_date, open_ssm)


# --------------------------------------------------------------------------
# Collision counters
# --------------------------------------------------------------------------


class Report:
    def __init__(self):
        self.failures = []
        self.not_present = []
        self.notes = []
        self.derivations = 0

    def fail(self, check, message):
        self.failures.append((check, message))

    def info(self, check, message):
        self.notes.append((check, message))

    def missing(self, message):
        self.not_present.append(message)


# --------------------------------------------------------------------------
# Crate module parsing -- the `holidays! { coverage: [...], rows: [...] }` table
# --------------------------------------------------------------------------

RE_ROW = re.compile(
    r"\(\s*(\d{4})\s*,\s*(\d{1,2})\s*,\s*(\d{1,2})\s*,\s*(.+?)\s*,\s*(T[12])\s*,"
    r'\s*"([^"]*)"\s*\)',
    re.S,
)
RE_WINDOW = re.compile(
    r"\(\s*(\d{4})\s*,\s*(\d{1,2})\s*,\s*(\d{1,2})\s*\)\s*\.\.=\s*"
    r"\(\s*(\d{4})\s*,\s*(\d{1,2})\s*,\s*(\d{1,2})\s*\)"
)
RE_CONST = re.compile(r"const\s+([A-Z][A-Z0-9_]*)\s*:\s*u32\s*=\s*([^;]+);")

RE_TOKEN = re.compile(r"\s*(\d[\d_]*|[A-Za-z_][A-Za-z0-9_]*|[()+\-*/])")


def eval_const_expr(expr, consts, seen=None):
    """Evaluates the small unsigned arithmetic the tables use, without `eval`."""
    tokens = RE_TOKEN.findall(expr)
    position = [0]
    seen = seen or frozenset()

    def peek():
        return tokens[position[0]] if position[0] < len(tokens) else None

    def take():
        token = peek()
        position[0] += 1
        return token

    def primary():
        token = take()
        if token is None:
            raise ValueError("unexpected end of expression: {!r}".format(expr))
        if token == "(":
            value = add()
            if take() != ")":
                raise ValueError("unbalanced parentheses in {!r}".format(expr))
            return value
        if token[0].isdigit():
            return int(token.replace("_", ""))
        if token in consts:
            if token in seen:
                raise ValueError("cyclic constant {!r}".format(token))
            return eval_const_expr(consts[token], consts, seen | {token})
        raise ValueError("unknown token {!r} in {!r}".format(token, expr))

    def unary():
        if peek() == "-":
            take()
            return -unary()
        return primary()

    def multiply():
        value = unary()
        while peek() in ("*", "/"):
            operator = take()
            right = unary()
            value = value * right if operator == "*" else value // right
        return value

    def add():
        value = multiply()
        while peek() in ("+", "-"):
            operator = take()
            right = multiply()
            value = value + right if operator == "+" else value - right
        return value

    result = add()
    if position[0] != len(tokens):
        raise ValueError("trailing tokens in {!r}".format(expr))
    return result


def parse_kind(source, consts):
    text = " ".join(source.split())
    text = text.replace("HolidayKind::", "")
    if text == "Closed":
        return ("Closed",)
    if text == "Unsourced":
        return ("Unsourced",)
    match = re.fullmatch(r"(early_close|late_open|late_open_and_early_close)\((.*)\)", text)
    if not match:
        raise ValueError("unrecognised holiday kind {!r}".format(text))
    args = [eval_const_expr(part, consts) for part in match.group(2).split(",")]
    if match.group(1) == "early_close":
        return ("EarlyClose", args[0])
    if match.group(1) == "late_open":
        return ("LateOpen", args[0])
    return ("LateOpenAndEarlyClose", args[0], args[1])


class ModuleTable:
    def __init__(self, owner, windows, rows, anomalies):
        self.owner = owner
        self.windows = windows          # [(first date, last date), ...]
        self.rows = rows                # [Row, ...] ascending by trade date
        self.anomalies = anomalies      # parser complaints, reported as failures

    def contains(self, day):
        return any(first <= day <= last for first, last in self.windows)

    def row_on(self, day):
        for row in self.rows:
            if row.trade_date == day:
                return row
        return None

    def rows_in(self, first, last):
        return [row for row in self.rows if first <= row.trade_date <= last]


class Row:
    __slots__ = ("trade_date", "kind", "tier", "document")

    def __init__(self, trade_date, kind, tier, document):
        self.trade_date = trade_date
        self.kind = kind
        self.tier = tier
        self.document = document

    def key(self):
        return (self.kind, self.tier)


def parse_module(path):
    with open(path, encoding="utf-8") as handle:
        text = handle.read()
    consts = {}
    for match in RE_CONST.finditer(text):
        consts[match.group(1)] = match.group(2)
    windows = []
    marker = text.find("coverage:")
    rows_at = text.find("rows: [")
    if marker < 0 or rows_at < 0:
        return None
    for match in RE_WINDOW.finditer(text[marker:rows_at]):
        windows.append(
            (
                date(int(match.group(1)), int(match.group(2)), int(match.group(3))),
                date(int(match.group(4)), int(match.group(5)), int(match.group(6))),
            )
        )
    end = text.find("\n};", rows_at)
    region = text[rows_at:end if end > 0 else len(text)]
    rows = []
    for match in RE_ROW.finditer(region):
        trade_date = date(int(match.group(1)), int(match.group(2)), int(match.group(3)))
        kind = parse_kind(match.group(4), consts)
        rows.append(Row(trade_date, kind, match.group(5), match.group(6)))
    if not rows:
        return None
    anomalies = []
    # A row tuple starts with three integers; if the parser found fewer rows
    # than the region holds, it dropped one, and every row comparison over this
    # table would be vacuously satisfied.
    starts = len(re.findall(r"\(\s*\d{4}\s*,\s*\d{1,2}\s*,\s*\d{1,2}\s*,", region))
    if starts != len(rows):
        anomalies.append(
            "{}: parsed {} rows but the table region holds {} row tuples".format(
                os.path.basename(path), len(rows), starts
            )
        )
    if end < 0:
        anomalies.append(
            "{}: no `}};` terminates the table; the row region may be truncated".format(
                os.path.basename(path)
            )
        )
    return ModuleTable(os.path.basename(path), windows, rows, anomalies)


# --------------------------------------------------------------------------
# The evidence block
# --------------------------------------------------------------------------


class Entry:
    """One family's row inside one block entry."""

    def __init__(self, day, block_name, family, status, verbatim, document,
                 open_instant, close_instant):
        self.date = day
        self.block_name = block_name
        self.family = family
        self.status = status
        self.verbatim = verbatim
        self.document = document
        self.open_instant = open_instant
        self.close_instant = close_instant

    def source(self):
        return "block {} {} status={} document={} verbatim={!r}".format(
            self.date.isoformat(), self.family, self.status, self.document,
            self.verbatim[:120],
        )


def load_block(path, report):
    with open(path, encoding="utf-8") as handle:
        data = json.load(handle)
    entries = []
    for holiday in data["holidays"]:
        day = date.fromisoformat(holiday["date"])
        for family in holiday["families"]:
            entries.append(
                Entry(
                    day,
                    holiday["name"],
                    family["family"],
                    family["status"],
                    family["verbatim"],
                    family["document"],
                    family.get("open_instant"),
                    family.get("close_instant"),
                )
            )
    return data, entries


# --------------------------------------------------------------------------
# Re-derivation rules
# --------------------------------------------------------------------------


def named_date(cell, reference):
    """The trade date a reopen cell names, if it names one."""
    match = RE_WEEKDAY_DAY_MONTH.search(cell)
    if match:
        month = MONTHS[match.group(3).lower()[:4].rstrip(".")]
        day_number = int(match.group(2))
        return (
            nearest_year(month, day_number, reference),
            match.group(1).lower()[:3],
        )
    match = RE_WEEKDAY_MONTH_DAY.search(cell)
    if match:
        month = MONTHS[match.group(2).lower()[:4].rstrip(".")]
        day_number = int(match.group(3))
        return (
            nearest_year(month, day_number, reference),
            match.group(1).lower()[:3],
        )
    match = RE_WEEKDAY.search(cell)
    if match:
        weekday = WEEKDAYS.index(match.group(1).lower()[:3])
        return (next_weekday(reference, weekday), match.group(1).lower()[:3])
    return (None, None)


def clock_in(cell, prefer_open=False):
    """The CT wall clock a cell prints, in seconds since local midnight.

    A reopen cell may print a pre-open before the open (`Pre-open 6:00CT ...
    Open 8:30 CT`); the leg's clock is the one after the word `Open`, so
    `prefer_open` skips to it. `Pre-Open` is a different word and never
    matches.
    """
    offset = 0
    if prefer_open:
        match = RE_OPEN_WORD.search(cell)
        if match:
            offset = match.end()
    found = RE_CLOCK.search(cell, offset)
    if not found:
        return None
    if found.group(1) is not None:
        return ssm(int(found.group(1)), int(found.group(2)))
    digits = found.group(3)
    if len(digits) == 3:
        digits = "0" + digits
    return ssm(int(digits[:2]), int(digits[2:]))


def derive_reopen(family, cell, day):
    """Resolves a printed reopen into `(trade_date, instant)`.

    A cell that names a day belongs to that day; failing a day name it is the
    entry's own trade date.  An evening clock (>= 12:00) opens the *next*
    business day's trade date, because that is the leg it carries; a day-session
    clock carries the named day itself.  A cell that prints no clock but names
    the next day session's `Regular` open means that day session's own open.
    """
    named, _weekday = named_date(cell, day)
    base = named if named is not None else day
    clock = clock_in(cell, prefer_open=True)
    if clock is None:
        return (base, ORDINARY[family][3])
    if clock >= ssm(12):
        return (next_business_day(base), clock)
    return (base, clock)


def derive_family_rows(family, entries, report):
    """Re-derives every expected row for one family, from the block alone."""
    expected = {}

    def add(day, kind, source):
        existing = expected.get(day)
        if existing is None:
            expected[day] = (kind, source)
            return
        old_kind, old_source = existing
        merged = None
        if old_kind[0] in ("LateOpen", "EarlyClose") and kind[0] in ("LateOpen", "EarlyClose"):
            if old_kind[0] == kind[0]:
                if old_kind == kind:
                    return
                report.fail(
                    "1",
                    "{} {}: block states two different {} rows on one date ({} at {} "
                    "vs {} at {})".format(
                        day.isoformat(), family, kind[0], kind_text(old_kind),
                        old_source.date.isoformat(), kind_text(kind),
                        source.date.isoformat(),
                    ),
                )
                return
            late = old_kind if old_kind[0] == "LateOpen" else kind
            early = old_kind if old_kind[0] == "EarlyClose" else kind
            merged = ("LateOpenAndEarlyClose", late[1], early[1])
        if merged is None:
            report.fail(
                "1",
                "{} {}: derived {} conflicts with {} already derived from {}".format(
                    day.isoformat(), family, kind_text(kind), kind_text(old_kind),
                    old_source.source(),
                ),
            )
            return
        expected[day] = (merged, source)

    ordinary_close = ORDINARY[family][2]

    for entry in entries:
        if entry.family != family:
            continue
        day = entry.date
        report.derivations += 1
        if entry.status == "closed":
            add(day, ("Closed",), entry)
        if entry.close_instant is not None:
            printed = clock_in(entry.close_instant)
            if printed is None:
                report.fail(
                    "1",
                    "{} {}: close cell prints no clock: {!r}".format(
                        day.isoformat(), family, entry.close_instant[:120]
                    ),
                )
            else:
                if entry.status == "normal" and printed < ordinary_close:
                    report.fail(
                        "1",
                        "{} {}: status normal prints a final close {} earlier than the "
                        "ordinary {} — status and instant disagree ({})".format(
                            day.isoformat(), family, fmt_ssm(printed),
                            fmt_ssm(ordinary_close), entry.source(),
                        ),
                    )
                elif entry.status != "normal" and printed < ordinary_close:
                    add(day, ("EarlyClose", printed), entry)
        if entry.open_instant is not None:
            named, weekday = named_date(entry.open_instant, day)
            if named is not None and weekday is not None:
                if WEEKDAYS[named.weekday()] != weekday:
                    report.info(
                        "1",
                        "{} {}: reopen cell names {} but {} is a {} ({!r})".format(
                            day.isoformat(), family, weekday, named.isoformat(),
                            WEEKDAYS[named.weekday()], entry.open_instant[:90],
                        ),
                    )
            trade_date, clock = derive_reopen(family, entry.open_instant, day)
            base = day if named is None else named
            printed_instant = (base, clock)
            ordinary = ordinary_first_open(family, trade_date)
            later = printed_instant > ordinary
            if entry.status == "normal" and later:
                report.fail(
                    "1",
                    "{} {}: status normal prints a reopen {} later than the ordinary "
                    "first open {} for trade date {} — status and instant disagree "
                    "({})".format(
                        day.isoformat(), family, fmt_ssm(clock),
                        fmt_ssm(ordinary[1]), trade_date.isoformat(), entry.source(),
                    ),
                )
            elif entry.status != "normal" and later:
                add(trade_date, ("LateOpen", clock), entry)

    for day, document in JUNETEENTH.items():
        report.derivations += 1
        expected[day] = (("Unsourced",), _JuneteenthSource(day, document))

    return expected


class _JuneteenthSource:
    """Stands in for a block entry on a date CME published nothing for."""

    def __init__(self, day, document):
        self.date = day
        self.document = document

    def source(self):
        return "Juneteenth {} ({}) — no operator schedule exists".format(
            self.date.isoformat(), self.document
        )


# --------------------------------------------------------------------------
# Document-id resolution
# --------------------------------------------------------------------------


class RawStore:
    def __init__(self, root):
        self.root = root
        self.text = {}          # dump file name -> contents
        self.text_by_base = {}  # source .xls basename -> [dump contents, ...]
        self.shasum = {}        # path relative to the raw root -> sha256
        self.paths = set()      # every hashed path
        self.by_basename = {}   # basename -> [path, ...]
        self._load()

    def _load(self):
        text_dir = os.path.join(self.root, "text")
        if os.path.isdir(text_dir):
            for name in sorted(os.listdir(text_dir)):
                if name.endswith(".txt"):
                    with open(os.path.join(text_dir, name), encoding="utf-8",
                              errors="replace") as handle:
                        contents = handle.read()
                    self.text[name] = contents
                    # Dump names are the source path with its separators folded
                    # (`docs__X.txt`, `zip2019_globex-trading-schedules__X.txt`),
                    # so the source file name is the segment after the last fold.
                    base = name[:-4].rsplit("__", 1)[-1]
                    self.text_by_base.setdefault(base, []).append(contents)
        shasum_path = os.path.join(self.root, "shasum.txt")
        if os.path.isfile(shasum_path):
            with open(shasum_path, encoding="utf-8") as handle:
                for line in handle:
                    match = re.match(r"^([0-9a-f]{64})\s+(.+?)\s*$", line)
                    if match:
                        self.shasum[match.group(2)] = match.group(1)
                        self.paths.add(match.group(2))
                        self.by_basename.setdefault(
                            os.path.basename(match.group(2)), []
                        ).append(match.group(2))

    def split_id(self, document):
        if " @" in document:
            name, _, stamp = document.rpartition(" @")
            return name, stamp
        return document, ""

    def resolve(self, document):
        """Resolves a crate document id to a path under the raw store."""
        name, _stamp = self.split_id(document)
        if ".zip#" in name:
            archive, member = name.split(".zip#", 1)
            year = archive[:4]
            # The 2019 bundle names its members with their directory
            # (`globex-trading-schedules/...`); 2020 and 2021 name them bare.
            directory = "zip2019" if year == "2019" else "zip" + year
            candidates = [directory + "/" + member]
        elif name == "2019-new-years-holiday-schedule-compact.xls":
            # The December-2018 supplement CME published before the 2019 bundle.
            candidates = [SUPPLEMENT]
        else:
            candidates = self.by_basename.get(name, [])
        for candidate in candidates:
            if candidate in self.paths:
                return candidate
        return None

    def dumps_for(self, path):
        """Every plain-text dump of the workbook at `path`."""
        return self.text_by_base.get(os.path.basename(path)[:-4], [])

    def code_candidates(self, code):
        """The raw-store paths a block document code can name.

        A code is `<year>-<key>`; the sheet it names is the file whose own name
        begins with that year and carries the holiday's keyword.  The year is
        the sheet's, not the bundle's, so `2021-ny` is the `2021-new-years...`
        member of the 2020 ZIP and `2022-ny` the like-named member of 2021's.
        """
        if code.endswith("(DEC2018)"):
            return [SUPPLEMENT]
        match = re.match(r"^(\d{4})-([a-z0-9]+)$", code)
        if not match:
            return []
        year, key = match.group(1), match.group(2)
        keywords = CODE_KEYWORDS.get(key)
        if keywords is None:
            return []
        found = []
        for path in sorted(self.paths):
            base = os.path.basename(path)
            if not base.startswith(year + "-"):
                continue
            if any(keyword in base for keyword in keywords):
                found.append(path)
        return found

    def matches_code(self, code, document):
        path = self.resolve(document)
        if path is None:
            return False
        return any(path == candidate for candidate in self.code_candidates(code))


# --------------------------------------------------------------------------
# Checks
# --------------------------------------------------------------------------


def window_days():
    day = WINDOW[0]
    while day <= WINDOW[1]:
        yield day
        day += timedelta(days=1)


def check_rows(entries, tables, raw, report):
    """Check 1 — row-for-row re-derivation for the eight families."""
    counts = {}
    for family in FAMILIES:
        expected = derive_family_rows(family, entries, report)
        for day in sorted(expected):
            if not (WINDOW[0] <= day <= WINDOW[1]):
                report.info(
                    "1",
                    "{} {}: a rule outside the wave's window derives {} ({})".format(
                        day.isoformat(), family, kind_text(expected[day][0]),
                        expected[day][1].source(),
                    ),
                )
        table = tables[family]
        actual = {}
        for row in table.rows_in(*WINDOW):
            actual[row.trade_date] = row
        counts[family] = len(actual)

        for day in window_days():
            report.derivations += 1
            want = expected.get(day)
            have = actual.get(day)
            want_kind = None if want is None else want[0]
            if want is None and have is None:
                continue
            if want_kind != (None if have is None else have.kind):
                report.fail(
                    "1",
                    "{} {}: derived {} | module {}".format(
                        day.isoformat(), family, kind_text(want_kind),
                        "nothing" if have is None else kind_text(have.kind),
                    )
                    + " | input: {}".format(
                        want[1].source() if want is not None else "no block entry and no rule fires"
                    ),
                )
                continue
            if have is None:
                continue
            if have.tier != BLOCK_TIER:
                report.fail(
                    "1",
                    "{} {}: derived tier {} | module {} | input: {}".format(
                        day.isoformat(), family, BLOCK_TIER, have.tier, want[1].source()
                    ),
                )
            if want is not None and isinstance(want[1], _JuneteenthSource):
                if have.document != want[1].document:
                    report.fail(
                        "1",
                        "{} {}: derived document {!r} | module {!r} | input: {}".format(
                            day.isoformat(), family, want[1].document, have.document,
                            want[1].source(),
                        ),
                    )
            elif want is not None and not raw.matches_code(
                getattr(want[1], "document", ""), have.document
            ):
                report.fail(
                    "1",
                    "{} {}: derived document (block code {}) | module {!r} | input: {}".format(
                        day.isoformat(), family, getattr(want[1], "document", "?"),
                        have.document, want[1].source(),
                    ),
                )
        if counts[family] != EXPECTED_FAMILY_COUNTS[family]:
            report.fail(
                "1",
                "{}: module ships {} rows in 2019-2021, expected {}".format(
                    family, counts[family], EXPECTED_FAMILY_COUNTS[family]
                ),
            )
    return counts


def check_coverage(tables, venue_tables, report):
    """Check 2 — the declared window and every row inside it."""
    for owner, table in list(tables.items()) + list(venue_tables.items()):
        for anomaly in table.anomalies:
            report.fail("2", anomaly)
        declared = any(
            first <= WINDOW[0] and last >= WINDOW[1] for first, last in table.windows
        )
        if not declared:
            report.fail(
                "2",
                "{}: declares no (2019-01-01 ..= 2021-12-31) window; windows are {}".format(
                    owner,
                    ", ".join(
                        "{}..{}".format(a.isoformat(), b.isoformat())
                        for a, b in table.windows
                    ),
                ),
            )
        for row in table.rows:
            if not table.contains(row.trade_date):
                report.fail(
                    "2",
                    "{}: row {} lies outside every declared window".format(
                        owner, row.trade_date.isoformat()
                    ),
                )
        for index in range(1, len(table.rows)):
            if table.rows[index].trade_date <= table.rows[index - 1].trade_date:
                report.fail(
                    "2",
                    "{}: rows are not strictly ascending at {} (after {})".format(
                        owner,
                        table.rows[index].trade_date.isoformat(),
                        table.rows[index - 1].trade_date.isoformat(),
                    ),
                )


def check_bytes(entries, raw, report):
    """Check 3 — the block's verbatims in CME's bytes, and the cited documents."""
    seen_documents = {}
    for entry in entries:
        if entry.family == "globex_nikkei_225_dollar":
            continue        # the block records prose for this family, not a row
        candidates = raw.code_candidates(entry.document)
        if not candidates:
            report.fail(
                "3",
                "{} {}: block document code {!r} names no raw-store file".format(
                    entry.date.isoformat(), entry.family, entry.document
                ),
            )
            continue
        hits = [
            path
            for path in candidates
            if any(entry.verbatim in text for text in raw.dumps_for(path))
        ]
        if hits:
            continue
        # The dumps pad an empty cell with two spaces where the block renders it
        # with one; that is the dumper's spacing, not the operator's bytes, so a
        # whitespace-only difference is reported but not counted as a failure.
        flat = " ".join(entry.verbatim.split())
        if any(
            flat in " ".join(text.split())
            for path in candidates
            for text in raw.dumps_for(path)
        ):
            report.info(
                "3",
                "{} {}: verbatim present in {} but with different whitespace only "
                "(the block collapses an empty cell's padding)".format(
                    entry.date.isoformat(), entry.family, entry.document
                ),
            )
            continue
        if "…" in entry.verbatim or "..." in entry.verbatim:
            cause = "the block elided cells with an ellipsis; the printed row is not reproduced"
        elif " — " in entry.verbatim:
            cause = "the block appends editorial commentary after the printed row"
        elif "(" in entry.verbatim:
            cause = "the block appends a parenthetical annotation to the printed row"
        else:
            cause = "no candidate sheet holds this string"
        report.fail(
            "3",
            "{} {}: verbatim not in the bytes of {} [{}]: {!r}".format(
                entry.date.isoformat(), entry.family, entry.document, cause,
                entry.verbatim[:120],
            ),
        )

    for family in FAMILIES:
        module = FAMILY_MODULE.format(family)
        table = parse_module(module)
        if table is None:
            report.missing("{}: no holiday table found (not yet present)".format(module))
            continue
        for row in table.rows_in(*WINDOW):
            seen_documents.setdefault(row.document, set()).add(family)
    for venue in VENUES:
        module = VENUE_MODULE.format(venue)
        table = parse_module(module)
        if table is None:
            report.missing("{}: no holiday table found (not yet present)".format(module))
            continue
        for row in table.rows_in(*WINDOW):
            seen_documents.setdefault(row.document, set()).add(venue)

    evidence_hashes = {}
    for owner in FAMILIES + list(VENUES):
        path = os.path.join("docs", "evidence", owner + ".md")
        if not os.path.isfile(path):
            report.missing(
                "{}: evidence file absent — citations unverified (not yet present)".format(
                    path
                )
            )
            continue
        with open(path, encoding="utf-8") as handle:
            text = handle.read()
        rows = 0
        for line in text.splitlines():
            if not line.startswith("|"):
                continue
            cells = [cell.strip() for cell in line.strip().strip("|").split("|")]
            if len(cells) < 3:
                continue
            match = re.match(r"^`(.+?)`$", cells[0])
            digest = re.search(r"\b([0-9a-f]{64})\b", cells[-1])
            if match and digest:
                evidence_hashes.setdefault(match.group(1), {})[owner] = digest.group(1)
                rows += 1
        if rows == 0:
            report.missing(
                "{}: no Documents table with sha256 values (not yet present)".format(path)
            )

    for document in sorted(seen_documents):
        owners = ", ".join(sorted(seen_documents[document]))
        resolved = raw.resolve(document)
        if resolved is None:
            report.fail(
                "3",
                "cited document {!r} ({}) resolves to no path under the raw store".format(
                    document, owners
                ),
            )
            continue
        expected_hash = raw.shasum.get(resolved)
        if expected_hash is None:
            report.fail(
                "3",
                "cited document {!r} ({}) resolves to {}, which shasum.txt does not "
                "hash".format(document, owners, resolved),
            )
            continue
        recorded = evidence_hashes.get(document)
        if recorded is None:
            report.fail(
                "3",
                "cited document {!r} ({}) has no sha256 row in any evidence file".format(
                    document, owners
                ),
            )
        else:
            for owner, digest in sorted(recorded.items()):
                if digest != expected_hash:
                    report.fail(
                        "3",
                        "document {!r} ({}): evidence/{}.md records sha256 {} | "
                        "shasum.txt has {} for {}".format(
                            document, owners, owner, digest, expected_hash, resolved
                        ),
                    )
        actual_path = os.path.join(raw.root, resolved)
        if os.path.isfile(actual_path):
            digest = hashlib.sha256()
            with open(actual_path, "rb") as handle:
                for block in iter(lambda: handle.read(1 << 20), b""):
                    digest.update(block)
            if digest.hexdigest() != expected_hash:
                report.fail(
                    "3",
                    "{} has sha256 {} | shasum.txt says {} ".format(
                        resolved, digest.hexdigest(), expected_hash
                    ),
                )
        else:
            report.missing(
                "{}: hashed path absent from the research store (not yet present)".format(
                    resolved
                )
            )


def venue_rows_from_families(family_tables, routed):
    """D17: the intersection of the routed families' tables, 2019-2021."""
    expected = {}
    documents = {}
    day = WINDOW[0]
    while day <= WINDOW[1]:
        covering = [f for f in routed if family_tables[f].contains(day)]
        if covering:
            stated = [(f, family_tables[f].row_on(day)) for f in covering]
            present = [(f, row) for f, row in stated if row is not None]
            documents[day] = {row.document for _f, row in present}
            if present:
                keys = {row.key() for _f, row in present}
                if len(present) == len(stated) and len(keys) == 1:
                    # Every routed family states the same row: the venue ships it.
                    expected[day] = present[0][1]
                else:
                    # A family that covers the date and holds no row has audited it
                    # normal, which is an answer and disputes a row; a date where
                    # the stated rows differ ships `Unsourced` rather than one
                    # family's instant.
                    expected[day] = Row(day, ("Unsourced",), TIER, None)
        day += timedelta(days=1)
    return expected, documents


def check_venues(tables, venue_tables, report):
    """Check 4 — the venue tables recomputed from the eight family modules."""
    counts = {}
    for venue, routed in VENUES.items():
        expected, documents = venue_rows_from_families(tables, routed)
        table = venue_tables[venue]
        actual = {row.trade_date: row for row in table.rows_in(*WINDOW)}
        counts[venue] = len(actual)
        for day in window_days():
            report.derivations += 1
            want = expected.get(day)
            have = actual.get(day)
            if want is None and have is None:
                continue
            if want is None or have is None or want.kind != have.kind:
                report.fail(
                    "4",
                    "{} {}: derived {} | module {} | input: {} families' rows on this "
                    "date state {}".format(
                        day.isoformat(), venue, kind_text(None if want is None else want.kind),
                        kind_text(None if have is None else have.kind),
                        len(routed),
                        " | ".join(
                            "{}: {}".format(
                                f,
                                kind_text(
                                    None
                                    if tables[f].row_on(day) is None
                                    else tables[f].row_on(day).kind
                                ),
                            )
                            for f in routed
                            if tables[f].contains(day)
                        ),
                    ),
                )
                continue
            if want.tier != have.tier:
                report.fail(
                    "4",
                    "{} {}: derived tier {} | module {} | input: {}".format(
                        day.isoformat(), venue, want.tier, have.tier,
                        "the routed families' own tiers",
                    ),
                )
            cited = documents.get(day, set())
            if have.document not in cited:
                report.fail(
                    "4",
                    "{} {}: module cites {!r}, which no routed family cites on this "
                    "date | derived from the routed families' rows | family documents: "
                    "{}".format(
                        day.isoformat(), venue, have.document,
                        ", ".join(sorted(cited)) if cited else "none",
                    ),
                )
        if counts[venue] != EXPECTED_VENUE_COUNTS[venue]:
            report.fail(
                "4",
                "{}: module ships {} rows in 2019-2021, expected {}".format(
                    venue, counts[venue], EXPECTED_VENUE_COUNTS[venue]
                ),
            )
    return counts


RE_TEST_FN = re.compile(r"^fn\s+([a-z0-9_]+)\s*\(")
RE_DATE_LITERAL = re.compile(
    r"day\(\s*\(\s*(\d{4})\s*,\s*(\d{1,2})\s*,\s*(\d{1,2})\s*\)\s*\)"
    r"|day\(\s*(\d{4})\s*,\s*(\d{1,2})\s*,\s*(\d{1,2})\s*\)"
    r"|\(\s*(\d{4})\s*,\s*(\d{1,2})\s*,\s*(\d{1,2})\s*\)"
)
RE_PINNING_ASSERT = re.compile(
    r"holiday_on|HolidayKind|is_closed_trade_date|is_closed_all_day_on|"
    r"session_bounds|is_open|holiday_coverage|candle_end|trade_date"
)
RE_ABSENCE = re.compile(r"\bNone\b|is_none\(\)|!coverage\.contains|assert!\(!")
RE_ERA_ROWS = re.compile(r"assert_eq!\(\s*rows\s*,\s*(\d+)")
RE_COUNT_TUPLE = re.compile(
    r"assert_eq!\(\s*\(\s*[a-z_0-9]+(?:\s*,\s*[a-z_0-9]+)+\s*\)\s*,"
)


def test_functions(path):
    """Splits a test file into `(name, first_line, [(line_number, text)])`."""
    with open(path, encoding="utf-8") as handle:
        lines = handle.readlines()
    starts = [
        (index, RE_TEST_FN.match(line).group(1))
        for index, line in enumerate(lines)
        if RE_TEST_FN.match(line)
    ]
    functions = []
    for position, (index, name) in enumerate(starts):
        end = len(lines)
        for scan in range(index + 1, len(lines)):
            if lines[scan].startswith("}") and len(lines[scan].strip()) == 1:
                end = scan + 1
                break
            if scan in [start for start, _ in starts[position + 1:position + 2]]:
                end = scan
                break
        functions.append((name, index + 1, list(enumerate(lines[index:end], start=index))))
    return functions


def check_mutation(tables, report):
    """Check 5 — which committed test would fail if one shipped row were flipped.

    The answer is read from the committed tests, not from a test run: a row is
    *pinned* by a per-date assertion on its date and kind, is covered by an
    era-wide count sweep whose exact counters a flipped kind or instant moves,
    or is asserted *absent* by a stale test that the current module already
    fails.
    """
    answers = {}
    for family in FAMILIES:
        table = tables[family]
        rows = table.rows_in(*WINDOW)
        chosen = None
        for wanted in ("Closed", "EarlyClose", "LateOpen", "Unsourced"):
            for row in rows:
                if row.kind[0] == wanted:
                    chosen = row
                    break
            if chosen is not None:
                break
        if chosen is None:
            answers[family] = (None, "no 2019-2021 row ships")
            continue
        target = chosen.trade_date
        verdict = None
        for path in (FAMILY_TEST.format(family), VENUE_TEST):
            if not os.path.isfile(path):
                continue
            for name, first_line, body in test_functions(path):
                dates = {}
                for number, text in body:
                    for match in RE_DATE_LITERAL.finditer(text):
                        groups = [group for group in match.groups() if group]
                        if len(groups) == 3:
                            dates.setdefault(
                                date(int(groups[0]), int(groups[1]), int(groups[2])),
                                [],
                            ).append(number)
                if target not in dates:
                    continue
                body_text = "".join(text for _n, text in body)
                numbers = dates[target]
                pinned = None
                stale = None
                for number in numbers:
                    near = "".join(
                        text for _n, text in body if number - 1 <= _n <= number + 1
                    )
                    wide = "".join(
                        text for _n, text in body if number - 3 <= _n <= number + 3
                    )
                    if RE_ABSENCE.search(wide):
                        stale = stale or number
                    elif RE_PINNING_ASSERT.search(near):
                        pinned = pinned or number
                if pinned is not None:
                    candidate = (3, path, name, first_line, pinned, "pin")
                elif RE_ERA_ROWS.search(body_text) or RE_COUNT_TUPLE.search(body_text):
                    declared = RE_ERA_ROWS.search(body_text)
                    expected = int(declared.group(1)) if declared else None
                    if expected is None or expected == len(rows):
                        candidate = (2, path, name, first_line, numbers[0], "count")
                    else:
                        candidate = (
                            1, path, name, first_line, numbers[0],
                            "count (stale: declares {} rows, module ships {})".format(
                                expected, len(rows)
                            ),
                        )
                elif stale is not None:
                    candidate = (1, path, name, first_line, stale, "stale-absence")
                else:
                    continue
                if verdict is None or candidate[0] > verdict[0]:
                    verdict = candidate
        if verdict is None:
            summary = "{} {} -> NO FENCE: no committed test asserts this row".format(
                target.isoformat(), kind_text(chosen.kind)
            )
        elif verdict[5] == "stale-absence":
            _rank, path, name, first_line, number, _how = verdict
            summary = (
                "{} {} -> NO FENCE for the flip: the only committed assertion naming "
                "this date is a stale absence check at {}:{} {} — it asserts the date "
                "has NO row and therefore already fails on the current module".format(
                    target.isoformat(), kind_text(chosen.kind), path, number, name
                )
            )
        else:
            _rank, path, name, first_line, number, how = verdict
            summary = "{} {} -> {}:{} {} ({})".format(
                target.isoformat(), kind_text(chosen.kind), path, number, name, how
            )
            if how == "count":
                summary += (
                    " — an era-wide sweep asserting the era's exact row counts and "
                    "instants, so a flipped kind or instant moves a counter; a "
                    "document-id flip alone would not"
                )
            elif how.startswith("count (stale"):
                summary += " — the test already fails on the current module"
        answers[family] = (chosen, summary)
    return answers


# --------------------------------------------------------------------------
# Entry point
# --------------------------------------------------------------------------


def main(argv):
    root = os.environ.get("WAVE4_RESEARCH")
    if not root:
        sys.stderr.write(
            "check_wave4: WAVE4_RESEARCH is unset; set it to the research-store "
            "root (the directory holding holidays/)\n"
        )
        return 2
    if not os.path.isdir(root):
        sys.stderr.write(
            "check_wave4: WAVE4_RESEARCH={!r} is not a directory\n".format(root)
        )
        return 2

    # The crate under test is the checkout holding this tool; `WAVE4_CRATE`
    # overrides it, which is how the tool's own sensitivity is self-tested
    # against a mutated copy.
    crate = os.environ.get("WAVE4_CRATE") or os.path.dirname(
        os.path.dirname(os.path.abspath(__file__))
    )
    os.chdir(crate)

    raw_root = os.path.join(root, "holidays", "raw", "cme-2019-2021")
    block_path = os.path.join(root, "holidays", "cme-2019-2021.r2.json")
    for path in (raw_root, block_path):
        if not os.path.exists(path):
            sys.stderr.write("check_wave4: primary input missing: {}\n".format(path))
            return 2

    report = Report()
    raw = RawStore(raw_root)
    _block, entries = load_block(block_path, report)

    family_tables = {}
    for family in FAMILIES:
        path = FAMILY_MODULE.format(family)
        table = parse_module(path)
        if table is None:
            report.missing("{}: no holiday table found (not yet present)".format(path))
        else:
            family_tables[family] = table
    venue_tables = {}
    for venue in VENUES:
        path = VENUE_MODULE.format(venue)
        table = parse_module(path)
        if table is None:
            report.missing("{}: no holiday table found (not yet present)".format(path))
        else:
            venue_tables[venue] = table

    print("wave-4 cross-check — CME Globex holiday tables, 2019-2021")
    print("research store: {}".format(root))
    print("crate:          {}".format(crate))
    print("block:          {} entries, {} family entries ({})".format(
        len({entry.date for entry in entries}),
        len(entries),
        ", ".join(
            "{}={}".format(family, sum(1 for e in entries if e.family == family))
            for family in FAMILIES
        ),
    ))
    print("modules checked (sha256[0:12]):")
    for owner in FAMILIES + list(VENUES):
        path = (
            FAMILY_MODULE.format(owner) if owner in FAMILIES else VENUE_MODULE.format(owner)
        )
        if os.path.isfile(path):
            with open(path, "rb") as handle:
                digest = hashlib.sha256(handle.read()).hexdigest()[:12]
            print("  {:<40} {}".format(path, digest))
    print("")

    family_counts = {}
    if len(family_tables) == len(FAMILIES):
        family_counts = check_rows(entries, family_tables, raw, report)
        check_coverage(family_tables, venue_tables, report)
        check_venues(family_tables, venue_tables, report)
    else:
        report.fail(
            "1", "not every family module parsed; row checks skipped: {}".format(
                ", ".join(sorted(set(FAMILIES) - set(family_tables)))
            ),
        )
    check_bytes(entries, raw, report)
    mutation = {}
    if len(family_tables) == len(FAMILIES):
        mutation = check_mutation(family_tables, report)

    for check, message in report.failures:
        print("[FAIL {}] {}".format(check, message))
    for message in report.not_present:
        print("[NOT-YET-PRESENT] {}".format(message))
    for check, message in report.notes:
        print("[NOTE {}] {}".format(check, message))

    print("")
    print("counts per family (module rows in 2019-01-01..2021-12-31):")
    for family in FAMILIES:
        if family in family_counts:
            print(
                "  {:<26} module {:>3}   expected {:>3}   {}".format(
                    family, family_counts[family], EXPECTED_FAMILY_COUNTS[family],
                    "ok" if family_counts[family] == EXPECTED_FAMILY_COUNTS[family] else "MISMATCH",
                )
            )
    print("counts per venue (module rows in 2019-01-01..2021-12-31):")
    for venue in VENUES:
        in_window = [
            row for row in venue_tables.get(venue).rows_in(*WINDOW)
        ] if venue in venue_tables else []
        print(
            "  {:<26} module {:>3}   expected {:>3}   {}".format(
                venue, len(in_window), EXPECTED_VENUE_COUNTS[venue],
                "ok" if len(in_window) == EXPECTED_VENUE_COUNTS[venue] else "MISMATCH",
            )
        )

    print("")
    print("mutation sanity (one shipped row per family):")
    for family in FAMILIES:
        if family in mutation:
            _chosen, message = mutation[family]
            print("  {:<26} {}".format(family, message))

    by_check = {}
    for check, _message in report.failures:
        by_check[check] = by_check.get(check, 0) + 1
    causes = {}
    for check, message in report.failures:
        if check != "3" or "verbatim not in the bytes" not in message:
            continue
        for cause in ("elided", "editorial commentary", "parenthetical",
                      "no candidate sheet"):
            if cause in message:
                causes[cause] = causes.get(cause, 0) + 1
                break
    print("")
    print("failures by check: " + (
        ", ".join(
            "{}={}".format(check, by_check.get(check, 0))
            for check in ("1", "2", "3", "4")
        )
    ) + "   not-yet-present items={}".format(len(report.not_present)))
    if causes:
        print("check-3 causes: " + ", ".join(
            "{}={}".format(cause, causes[cause]) for cause in sorted(causes)
        ))
    if report.failures:
        print(
            "RESULT: FAIL — {} disagreement(s); {} re-derivations performed".format(
                len(report.failures), report.derivations
            )
        )
        return 1
    print(
        "RESULT: PASS — {} re-derivations performed, every derived row agrees with "
        "the modules".format(report.derivations)
    )
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
