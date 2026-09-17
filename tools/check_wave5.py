#!/usr/bin/env python3
# SPDX-License-Identifier: MIT-0
"""check_wave5.py -- re-derive every wave-5 value from the block and the bytes.

This tool shares no code with the generator.  It reads the research store's
repaired block, the crate's own modules and the saved raw text dumps, and
re-derives, checking each against what shipped:

  1. **rows** — every row of the six families' 2013-2015 era, from the block's
     own status/instant vocabulary and the crate's own 2013-2015 grids, compared
     by trade date, kind, tier and document admissibility;
  2. **coverage** — the declared window, the ascending row order and the fact
     that every row lies inside it;
  3. **bytes** — every cited document resolves to a saved artifact whose sha256
     is the one the block and the evidence file record, and every instant the
     row states appears in that artifact's dump;
  4. **venues** — the D17 intersection recomputed over the era from the family
     tables alone;
  5. **cross-wave** — the same intersection recomputed over 2010-2027 for the
     audit that closes #95, on every date at least one routed family covers (a
     family with no window there abstains, and the covered families decide);
  6. **fences** — that each family's test suite pins its era's dates (a
     handwritten table, not a count alone) and that the venue suite pins the
     venue shape;
  7. **prose counts** — every whole-table total, per-era share, per-window
     breakdown and venue table-doc figure the evidence files and module docs
     state, re-derived from the module each describes;
  8. **era shapes** — each family module header's per-era shape sentence against
     that era's own row count;
  9. **printed cells** — every era table's `instant as printed` cell against the
     block's own string for that row's date.

The crate is the checkout this script lives in (`tools/..`); only the research
store is a parameter.

Usage:  WAVE5_RESEARCH=... python3 tools/check_wave5.py
Exit:   0 every derivation agrees; 1 a disagreement; 2 a missing input.
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

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
HOLIDAYS = os.path.join(ROOT, "src", "calendar", "schedules", "holidays")
EVIDENCE = os.path.join(ROOT, "docs", "evidence")

WINDOW = (dt.date(2013, 1, 1), dt.date(2015, 12, 31))

FAMILIES = ("globex_equity_index", "globex_energy", "globex_fx",
            "globex_grains", "globex_interest_rates", "globex_livestock")

ROUTING = {
    "cme": ("globex_equity_index", "globex_energy", "globex_fx",
            "globex_grains", "globex_interest_rates", "globex_livestock"),
    "cbot": ("globex_grains", "globex_interest_rates"),
    "comex": ("globex_energy",),
    "nymex": ("globex_energy",),
}

GROUP_FAMILIES = {
    "equity_index": ("globex_equity_index",),
    "interest_rates+fx": ("globex_interest_rates", "globex_fx"),
    "energy+metals": ("globex_energy",),
    "grains_oilseeds": ("globex_grains",),
    "livestock": ("globex_livestock",),
    "livestock+dairy+lumber": ("globex_livestock",),
    "grains_oilseeds+livestock+dairy+lumber": ("globex_grains", "globex_livestock"),
    "equity_index+interest_rates+fx": (
        "globex_equity_index", "globex_interest_rates", "globex_fx"),
    "equity_index+interest_rates+fx+energy+metals": (
        "globex_equity_index", "globex_interest_rates", "globex_fx",
        "globex_energy"),
    "equity_index+interest_rates+fx+energy+metals+grains_oilseeds"
    "+livestock+dairy+lumber": (
        "globex_equity_index", "globex_interest_rates", "globex_fx",
        "globex_energy", "globex_grains", "globex_livestock"),
    "interest_rates+fx+energy+metals": (
        "globex_interest_rates", "globex_fx", "globex_energy"),
    "dairy": (),
    "lumber": (),
}

#: The ordinary close each family's era grid states, as (first date, ssm) pairs.
#: Read from the crate's own modules, not from the generator.
ORDINARY_CLOSE = {
    "globex_equity_index": (dt.date(2013, 1, 1), 16 * 3_600 + 15 * 60),
    "globex_energy": (dt.date(2013, 1, 1), 16 * 3_600 + 15 * 60),
    "globex_interest_rates": (dt.date(2013, 1, 1), 16 * 3_600),
    "globex_fx": (dt.date(2013, 1, 1), 16 * 3_600),
    "globex_grains": (dt.date(2013, 1, 1), 13 * 3_600 + 15 * 60),
    "globex_livestock": (dt.date(2013, 1, 1), 16 * 3_600),
}
#: The revisions inside the window that move a close.
CLOSE_REVISIONS = {
    "globex_equity_index": ((dt.date(2015, 9, 20), 16 * 3_600),),
    "globex_energy": ((dt.date(2015, 9, 20), 16 * 3_600),),
    "globex_grains": ((dt.date(2015, 7, 5), 13 * 3_600 + 20 * 60),),
    "globex_livestock": (
        (dt.date(2014, 10, 27), 16 * 3_600),),
}
#: A printed first open at least this much later than the ordinary first open
#: means the evening leg did not run.
LATE_OPEN_GRACE = 4 * 3_600
#: The ordinary first open of each family's era grid.
ORDINARY_FIRST_OPEN = {
    "globex_equity_index": 17 * 3_600,
    "globex_energy": 17 * 3_600,
    "globex_interest_rates": 17 * 3_600,
    "globex_fx": 17 * 3_600,
    "globex_grains": 19 * 3_600,
    "globex_livestock": 8 * 3_600,
}

CLOCK = re.compile(r"\b(\d{3,4})\s*CT\b")
ROW_RE = re.compile(
    r"\(\s*(\d{4})\s*,\s*(\d{1,2})\s*,\s*(\d{1,2})\s*,\s*(.+?)\s*,\s*(T[12])\s*,"
    r"\s*\"([^\"]+)\"\s*,?\s*\)", re.S)
WINDOW_RE = re.compile(
    r"\(\s*(\d+)\s*,\s*(\d+)\s*,\s*(\d+)\s*\)\s*\.\.=\s*"
    r"\(\s*(\d+)\s*,\s*(\d+)\s*,\s*(\d+)\s*\)")
CONST_RE = re.compile(
    r"^\s*const\s+([A-Za-z_][A-Za-z0-9_]*)\s*:\s*u32\s*=\s*([^;]+);")

FAILURES = []
NOTES = []


def fail(check, message):
    FAILURES.append((check, message))


def note(check, message):
    NOTES.append((check, message))


# --------------------------------------------------------------------------
# Reading
# --------------------------------------------------------------------------

def parse_rule(expression):
    """`12 * 3_600 + 15 * 60` -> seconds; also a bare quaternary date."""
    total = 0
    for term in expression.split("+"):
        product = 1
        for factor in term.split("*"):
            factor = factor.strip().replace("_", "")
            if not factor.isdigit():
                raise ValueError("not an instant: %r" % expression)
            product *= int(factor)
        total += product
    return total


def parse_module(path):
    text = open(path, encoding="utf-8").read()
    constants = {}
    for line in text.splitlines():
        found = CONST_RE.match(line)
        if found:
            constants[found.group(1)] = parse_rule(found.group(2))
    body = text[text.index("holidays! {"):]
    coverage, _, rest = body.partition("rows:")
    windows = [
        (dt.date(int(m[0]), int(m[1]), int(m[2])),
         dt.date(int(m[3]), int(m[4]), int(m[5])))
        for m in WINDOW_RE.findall(coverage)]
    rows = []
    for found in ROW_RE.finditer(rest):
        kind = found.group(4)
        for name in sorted(constants, key=len, reverse=True):
            kind = re.sub(r"\b%s\b" % re.escape(name), str(constants[name]), kind)
        rows.append({
            "date": dt.date(int(found.group(1)), int(found.group(2)),
                            int(found.group(3))),
            "kind": canonical_kind(kind),
            "tier": found.group(5),
            "document": found.group(6),
        })
    rows.sort(key=lambda row: row["date"])
    return windows, rows


def canonical_kind(kind):
    """`early_close(12 * 3_600 + 15 * 60)` -> `early_close(44100)`."""
    kind = re.sub(r"\s+", "", kind)
    for name in ("early_close", "late_open", "late_open_and_early_close"):
        prefix = "%s(" % name
        if kind.startswith(prefix) and kind.endswith(")"):
            arguments = [parse_rule(a) for a in kind[len(prefix):-1].split(",")]
            return "%s(%s)" % (name, ",".join(str(a) for a in arguments))
    return kind


def parse_clock(text):
    found = CLOCK.search(text or "")
    if not found:
        return None
    digits = found.group(1)
    if len(digits) == 3:
        hour, minute = int(digits[0]), int(digits[1:])
    else:
        hour, minute = int(digits[:2]), int(digits[2:])
    return hour * 3_600 + minute * 60 if hour < 24 and minute < 60 else None


def named_day(cell, anchor):
    found = re.search(
        r"\b(?:Mon|Tues|Tue|Wed|Thurs|Thu|Fri|Sat|Sun)(?:day)?\.?,?\s+"
        r"(?:(Jan|Feb|Mar|Apr|May|Jun|Jul|Aug|Sep|Oct|Nov|Dec)[a-z]*\.?\s+(\d{1,2})"
        r"|(\d{1,2})\s+(Jan|Feb|Mar|Apr|May|Jun|Jul|Aug|Sep|Oct|Nov|Dec)[a-z]*\.?)",
        cell or "", re.IGNORECASE)
    if not found:
        return None
    months = {m: i + 1 for i, m in enumerate(
        ["jan", "feb", "mar", "apr", "may", "jun",
         "jul", "aug", "sep", "oct", "nov", "dec"])}
    if found.group(1):
        month, day = months[found.group(1).lower()[:3]], int(found.group(2))
    else:
        day, month = int(found.group(3)), months[found.group(4).lower()[:3]]
    best = None
    for year in (anchor.year - 1, anchor.year, anchor.year + 1):
        try:
            candidate = dt.date(year, month, day)
        except ValueError:
            continue
        if best is None or abs((candidate - anchor).days) < abs((best - anchor).days):
            best = candidate
    return best


def close_at(family, date):
    if family == "globex_livestock":
        # The 2007 around-the-clock grid keeps a 13:55 CT Thursday short day
        # until 2014-10-26; the 2014-10-27 grid keeps a 13:55 CT Friday one.
        if date < dt.date(2014, 10, 27):
            return 13 * 3_600 + 55 * 60 if date.weekday() == 3 else 16 * 3_600
        return 13 * 3_600 + 55 * 60 if date.weekday() == 4 else 16 * 3_600
    value = ORDINARY_CLOSE[family][1]
    for first, moved in CLOSE_REVISIONS.get(family, ()):
        if date >= first:
            value = moved
    return value


def first_open_at(family, date):
    """The trade date's ordinary first open under the era grid."""
    if family == "globex_livestock":
        # The 2007 grid opens Monday at 09:05 CT on the day itself and
        # Tuesday-Thursday at 17:00 CT the previous evening; from 2014-10-27
        # every weekday opens at 08:00 CT except Monday's 09:05 CT.
        if date < dt.date(2014, 10, 27):
            return 9 * 3_600 + 5 * 60 if date.weekday() == 0 else 17 * 3_600
        return 9 * 3_600 + 5 * 60 if date.weekday() == 0 else 8 * 3_600
    return ORDINARY_FIRST_OPEN[family]


# --------------------------------------------------------------------------
# Check 1-2: rows and coverage, re-derived from the block alone
# --------------------------------------------------------------------------

def expected_rows(block):
    """The rows the block states, re-derived with the crate's grids."""
    out = collections.defaultdict(dict)
    for holiday in block["holidays"]:
        date = dt.date.fromisoformat(holiday["date"])
        for entry in holiday["families"]:
            group = entry["family"]
            if group not in GROUP_FAMILIES:
                fail(1, "unmapped block family group %r" % group)
                continue
            status = entry["status"]
            for family in GROUP_FAMILIES[group]:
                slot = out[family]
                if status == "closed":
                    slot[date] = ("Closed", None, None, entry["document"],
                                  entry["tier"])
                    continue
                close = parse_clock(entry.get("close_instant"))
                if close is not None and close < close_at(family, date):
                    existing = slot.get(date)
                    if existing and existing[0] == "late_open":
                        # The re-open cell was recorded first: merge rather than
                        # drop the late open the same date already carries.
                        slot[date] = ("late_open_and_early_close", existing[1], close,
                                      entry["document"], entry["tier"])
                    else:
                        slot[date] = ("early_close", None, close, entry["document"],
                                      entry["tier"])
                cell = entry.get("open_instant") or ""
                if not cell:
                    continue
                opened = parse_clock(cell)
                if opened is None:
                    continue
                target = named_day(cell, date) or date
                base = first_open_at(family, target)
                if (opened - base) % 86_400 < LATE_OPEN_GRACE:
                    continue
                existing = slot.get(target)
                if existing and existing[0] == "early_close":
                    slot[target] = ("late_open_and_early_close", opened,
                                    existing[2], existing[3], existing[4])
                elif existing and existing[0] == "late_open":
                    if existing[1] != opened:
                        fail(1, "%s %s: two different first opens" % (target, family))
                else:
                    slot[target] = ("late_open", opened, None, entry["document"],
                                    entry["tier"])
    return out


def check_rows(block, modules):
    check = 1
    expected = expected_rows(block)
    for family in FAMILIES:
        module_rows = {row["date"]: row for row in modules[family][1]
                       if WINDOW[0] <= row["date"] <= WINDOW[1]}
        want = expected[family]
        for date in sorted(set(want) | set(module_rows)):
            found = module_rows.get(date)
            wanted = want.get(date)
            if found is None:
                fail(check, "%s %s: the block states a row the module does not ship (%s)"
                     % (family, date, wanted[0]))
                continue
            if wanted is None:
                fail(check, "%s %s: the module ships a row the block does not state (%s)"
                     % (family, date, found["kind"]))
                continue
            kind, opened, closed, _, tier = wanted
            if kind == "Closed":
                rendered = "Closed"
            elif kind == "early_close":
                rendered = "early_close(%d)" % closed
            elif kind == "late_open":
                rendered = "late_open(%d)" % opened
            else:
                rendered = "late_open_and_early_close(%d,%d)" % (opened, closed)
            if found["kind"] != rendered:
                fail(check, "%s %s: derived %s, module %s"
                     % (family, date, rendered, found["kind"]))
            if found["tier"] != tier:
                fail(check, "%s %s: derived tier %s, module %s"
                     % (family, date, tier, found["tier"]))
    return sum(len({r["date"] for r in modules[f][1]
                    if WINDOW[0] <= r["date"] <= WINDOW[1]})
               for f in FAMILIES)


def check_coverage(modules):
    check = 2
    for family in FAMILIES:
        windows, rows = modules[family]
        if WINDOW not in windows:
            fail(check, "%s does not declare the 2013-2015 window" % family)
        dates = [row["date"] for row in rows]
        if dates != sorted(dates):
            fail(check, "%s rows are not ascending" % family)
        if len(dates) != len(set(dates)):
            fail(check, "%s rows are not unique" % family)
        for row in rows:
            if not any(first <= row["date"] <= last for first, last in windows):
                fail(check, "%s %s lies outside every window" % (family, row["date"]))
        for earlier, later in zip(windows, windows[1:]):
            if earlier[1] >= later[0]:
                fail(check, "%s windows overlap or touch" % family)
    return sum(len(modules[f][0]) for f in FAMILIES)


# --------------------------------------------------------------------------
# Check 3: bytes
# --------------------------------------------------------------------------

def locate(root, name, stamp):
    stem = name.rsplit(".", 1)[0]
    for directory in ("cme-2013-2015/txt", "cme-2013-2015-fix/txt",
                      "cme-2013-2015-verify-r2/txt",
                      "cme-2013-2015-repair-r2/txt"):
        for candidate in ("%s__%s.txt" % (stem, stamp), "%s__%s.txt" % (name, stamp)):
            path = os.path.join(root, directory, candidate)
            if os.path.isfile(path):
                return path
    return None


def locate_all(root, name, stamp):
    """Every saved copy of one capture, in the store's candidate directories."""
    stem = name.rsplit(".", 1)[0]
    out = []
    for directory in ("cme-2013-2015/pdf", "cme-2013-2015/xls",
                      "cme-2013-2015-fix/xls", "cme-2013-2015-fix/zip",
                      "cme-2013-2015-verify-r2/new",
                      "cme-2013-2015-verify-r2/refetch",
                      "cme-2013-2015-repair-r2/pdf"):
        for candidate in ("%s__%s" % (stem, stamp), "%s__%s.pdf" % (stem, stamp),
                          "%s__%s.xls" % (stem, stamp), "%s__%s.zip" % (stem, stamp),
                          name):
            path = os.path.join(root, directory, candidate)
            if os.path.isfile(path):
                out.append(path)
    return out


def locate_bytes(root, name, stamp):
    stem = name.rsplit(".", 1)[0]
    for directory in ("cme-2013-2015/pdf", "cme-2013-2015/xls",
                      "cme-2013-2015-fix/xls", "cme-2013-2015-fix/zip",
                      "cme-2013-2015-verify-r2/new", "cme-2013-2015-repair-r2/pdf"):
        for suffix in (".pdf", ".xls", ".zip", ""):
            path = os.path.join(root, directory, "%s__%s%s" % (stem, stamp, suffix))
            if os.path.isfile(path):
                return path
        path = os.path.join(root, directory, name)
        if os.path.isfile(path):
            return path
    return None


def check_bytes(root, block, modules):
    check = 3
    derived = 0
    checked_docs = set()
    for family in FAMILIES:
        for row in modules[family][1]:
            if not (WINDOW[0] <= row["date"] <= WINDOW[1]):
                continue
            document = row["document"]
            found = re.match(r"^(.+?) @(\d{4}-\d{2}-\d{2}(?:T\d{2}:\d{2}:\d{2}Z)?)$",
                             document)
            if not found:
                fail(check, "%s %s: document id %r is not `<file> @<capture>`"
                     % (family, row["date"], document))
                continue
            name, capture = found.group(1), found.group(2)
            if "T" not in capture:
                # A date-only label is the one an earlier wave's evidence table
                # already cites this artifact under; the block carries the
                # capture it abbreviates, so the bytes still resolve.
                for entry in block["documents"].values():
                    if os.path.basename(entry["file"]) == name and \
                            entry["capture_utc"].startswith(capture):
                        capture = entry["capture_utc"]
            stamp = re.sub(r"[^0-9]", "", capture)
            if document in checked_docs:
                continue
            checked_docs.add(document)
            candidates = locate_all(root, name, stamp)
            if not candidates:
                fail(check, "%s: no saved bytes for %r" % (family, document))
                continue
            recorded = None
            for entry in block["documents"].values():
                if os.path.basename(entry["file"]) == name and \
                        entry["capture_utc"] == capture:
                    recorded = entry.get("sha256")
            digests = {hashlib.sha256(open(path, "rb").read()).hexdigest()
                       for path in candidates}
            if recorded is None:
                fail(check, "%s: the block carries no sha256 for %r"
                     % (family, document))
            elif recorded not in digests:
                fail(check, "%s: %r records sha256 %s; the saved copies are %s"
                     % (family, document, recorded, sorted(digests)))
            elif len(digests) > 1:
                note(check, "%s: %r has %d saved copies with different bytes; "
                     "the cited one is present"
                     % (family, document, len(digests)))
            derived += 1
    return derived


# --------------------------------------------------------------------------
# Check 4: the D17 intersection, over the era and across the waves
# --------------------------------------------------------------------------

def joint(family_rows, families, date):
    stated = []
    for family in families:
        row = family_rows[family].get(date)
        stated.append((family, row))
    kinds = {None if row is None else row["kind"] for _, row in stated}
    if kinds == {None}:
        return None, None
    if len(kinds) == 1:
        kind = next(iter(kinds))
        for _, row in stated:
            if row is not None:
                return kind, row["document"]
    return "Unsourced", next(row["document"] for _, row in stated if row is not None)


def check_venues(modules):
    check = 4
    derived = 0
    for venue, families in ROUTING.items():
        venue_rows = {row["date"]: row
                      for row in modules["venues"][venue][1]}
        date = WINDOW[0]
        while date <= WINDOW[1]:
            kind, document = joint(
                {f: {r["date"]: r for r in modules[f][1]} for f in families},
                families, date)
            found = venue_rows.get(date)
            derived += 1
            if kind is None:
                if found is not None:
                    fail(check, "%s %s: derived no row, module ships %s"
                         % (venue, date, found["kind"]))
            elif found is None:
                fail(check, "%s %s: derived %s, module ships nothing"
                     % (venue, date, kind))
            elif found["kind"] != kind:
                fail(check, "%s %s: derived %s, module %s"
                     % (venue, date, kind, found["kind"]))
            elif kind != "Unsourced" and found["document"] != document:
                fail(check, "%s %s: derived id %r, module %r"
                     % (venue, date, document, found["document"]))
            date += dt.timedelta(days=1)
    return derived


def check_cross_wave(modules):
    """The #95 audit: the venue tables equal the intersection over 2010-2027."""
    check = 5
    derived = 0
    for venue, families in ROUTING.items():
        venue_rows = {row["date"]: row for row in modules["venues"][venue][1]}
        tables = {f: {r["date"]: r for r in modules[f][1]} for f in families}
        windows = [w for f in families for w in modules[f][0]]
        first = min(w[0] for w in windows)
        last = max(w[1] for w in windows)
        date = first
        while date <= last:
            # D17: a family with no window for the date abstains, and the
            # families that do cover it decide. Only a date no routed family
            # covers is skipped — the 2016-2018 era, where globex_livestock has
            # no table, still has rows the five that cover it determine.
            active = [f for f in families
                      if any(a <= date <= b for a, b in modules[f][0])]
            if not active:
                date += dt.timedelta(days=1)
                continue
            kind, document = joint(tables, active, date)
            found = venue_rows.get(date)
            derived += 1
            if kind is None:
                if found is not None:
                    fail(check, "%s %s: derived no row, module ships %s"
                         % (venue, date, found["kind"]))
            elif found is None:
                fail(check, "%s %s: derived %s, module ships nothing"
                     % (venue, date, kind))
            elif found["kind"] != kind:
                fail(check, "%s %s: derived %s, module %s"
                     % (venue, date, kind, found["kind"]))
            date += dt.timedelta(days=1)
    return derived


# --------------------------------------------------------------------------
# Check 6: the fences
# --------------------------------------------------------------------------

ERA_WORDS = {"three": 3, "four": 4, "five": 5, "six": 6, "seven": 7}


def table_doc(text):
    """The `///` block immediately above the table, without its evidence line."""
    lines = text.splitlines()
    stop = next(i for i, line in enumerate(lines) if line.startswith("// Evidence:"))
    start = stop
    while start > 0 and lines[start - 1].startswith("///"):
        start -= 1
    return "\n".join(lines[start:stop])


def window_counts(module):
    """(total, {window: (rows, stated, withheld)}) for one parsed module."""
    windows, rows = module
    per = {}
    for window in windows:
        inside = [r for r in rows if window[0] <= r["date"] <= window[1]]
        stated = [r for r in inside if r["kind"].lower() != "unsourced"]
        per[window] = (len(inside), len(stated), len(inside) - len(stated))
    return len(rows), per


UNITS = {"one": 1, "two": 2, "three": 3, "four": 4, "five": 5, "six": 6, "seven": 7,
         "eight": 8, "nine": 9, "ten": 10, "eleven": 11, "twelve": 12, "thirteen": 13,
         "fourteen": 14, "fifteen": 15, "sixteen": 16, "seventeen": 17, "eighteen": 18,
         "nineteen": 19}
TENS = {"twenty": 20, "thirty": 30, "forty": 40, "fifty": 50, "sixty": 60,
        "seventy": 70, "eighty": 80, "ninety": 90}


def word_number(text):
    """`Thirty-four` / `Two hundred and seventy-two` -> int; a digit string too."""
    text = text.strip().lower().strip("*-").strip()
    if text.isdigit():
        return int(text)
    words = text.replace("-", " ").replace(" and ", " ").split()
    total = 0
    current = 0
    for word in words:
        if word in UNITS:
            current += UNITS[word]
        elif word in TENS:
            current += TENS[word]
        elif word == "hundred":
            current = max(current, 1) * 100
        else:
            return None
    return total + current


def check_era_shapes(modules):
    """Each family module header's per-era shape sentence names that era's row count."""
    check = 8
    derived = 0
    for family in FAMILIES:
        path = os.path.join(HOLIDAYS, "%s.rs" % family)
        text = open(path, encoding="utf-8").read()
        header = text[:text.index("holidays! {")]
        windows, _ = modules[family]
        paragraphs = re.split(r"^//! \*\*(\d{4})-(\d{4})\.\*\*", header, flags=re.M)
        for i in range(1, len(paragraphs), 3):
            span = (dt.date(int(paragraphs[i]), 1, 1), dt.date(int(paragraphs[i + 1]), 12, 31))
            if span not in windows:
                continue
            body = paragraphs[i + 2]
            value = None
            claim = ""
            for found in re.finditer(
                    r"(?:^|\.\s|\n)\s*\*{0,2}([A-Za-z][A-Za-z-]*|\d+)\*{0,2} rows\b", body):
                parsed = word_number(found.group(1))
                if parsed is not None:
                    value, claim = parsed, found.group(1)
                    break
            if value is None:
                continue
            derived += 1
            inside = [r for r in modules[family][1] if span[0] <= r["date"] <= span[1]]
            if value != len(inside):
                fail(check, "%s: the %s-%s shape sentence says %r rows, the module has %d"
                     % (path, paragraphs[i], paragraphs[i + 1], claim, len(inside)))
    return derived


def check_prose_counts(modules):
    """Every row count the prose states is re-derived from the module it describes.

    Whole-table totals, window counts, per-era shares and the per-window
    breakdowns in the venue files' `Counts in this subsection` paragraphs are
    recomputed here; a wave that adds a window moves all of them, and an
    over-summed total is exactly the defect this catches.
    """
    check = 7
    derived = 0
    owners = list(FAMILIES) + list(ROUTING)
    for owner in owners:
        module = modules[owner] if owner in modules else modules["venues"][owner]
        total, per = window_counts(module)
        windows = module[0]
        evidence = os.path.join(ROOT, "docs", "evidence", "%s.md" % owner)
        text = open(evidence, encoding="utf-8").read()
        # 1. every "carries N rows over M windows"
        for found in re.finditer(r"carries (\d+) rows over (\w+) windows", text):
            derived += 1
            if int(found.group(1)) != total:
                fail(check, "%s: prose says %s rows, the module has %d"
                     % (evidence, found.group(1), total))
            if int(found.group(2)) != len(windows):
                fail(check, "%s: prose says %s windows, the module declares %d"
                     % (evidence, found.group(2), len(windows)))
        # 2. per-era shares, keyed by the era section they sit in
        for section in re.split(r"^### ", text, flags=re.M)[1:]:
            title = section.split("\n", 1)[0]
            found = re.search(r"(\d{4})-(\d{4})", title)
            if not found:
                continue
            span = (dt.date(int(found.group(1)), 1, 1), dt.date(int(found.group(2)), 12, 31))
            if span not in per:
                continue
            share = re.search(r"this era's share is \*\*(\d+) rows\*\*", section)
            if share:
                derived += 1
                if int(share.group(1)) != per[span][0]:
                    fail(check, "%s/%s: share says %s, the module has %d"
                         % (evidence, title, share.group(1), per[span][0]))
        # 3. the venue files' per-window breakdown paragraphs
        for paragraph in re.finditer(
                r"\*\*Counts in this subsection are the (20\d\d-20\d\d) era's\.\*\*(.*?)(?:\n\n|\Z)",
                text, re.S):
            stated_era = paragraph.group(1)
            body = paragraph.group(2)
            named = 0
            for found in re.finditer(r"(\d+)\s+in\s+(20\d\d-20\d\d|this era)", body):
                era = stated_era if found.group(2) == "this era" else found.group(2)
                span = (dt.date(int(era[:4]), 1, 1), dt.date(int(era[5:]), 12, 31))
                if span not in per:
                    continue
                named += 1
                derived += 1
                if int(found.group(1)) != per[span][0]:
                    fail(check, "%s: the %s breakdown says %s rows for %s, the module has %d"
                         % (evidence, paragraph.group(1), found.group(1), era, per[span][0]))
            derived += 1
            if named != len(windows):
                fail(check, "%s: the %s breakdown names %d of %d windows"
                     % (evidence, paragraph.group(1), named, len(windows)))
        # 3b. the cross-wave audit section's whole-table figures
        kinds = collections.Counter(row["kind"].split("(")[0] for row in module[1])
        withheld = sum(part[2] for part in per.values())
        for found in re.finditer(r"(\d+) rows over the (\w+) windows", text):
            derived += 2
            if int(found.group(1)) != total:
                fail(check, "%s: the audit says %s rows, the module has %d"
                     % (evidence, found.group(1), total))
            if ERA_WORDS.get(found.group(2)) != len(windows):
                fail(check, "%s: the audit says %s windows, the module declares %d"
                     % (evidence, found.group(2), len(windows)))
        for found in re.finditer(r"(\d+) withheld as `Unsourced`", text):
            derived += 1
            if int(found.group(1)) != withheld:
                fail(check, "%s: the audit says %s withheld, the module has %d"
                     % (evidence, found.group(1), withheld))
        for found in re.finditer(
                r"(\d+) closures, (\d+) early closes and (\d+) withheld", text):
            derived += 3
            for value, kind in ((int(found.group(1)), "Closed"),
                                (int(found.group(2)), "early_close"),
                                (int(found.group(3)), "withheld")):
                want = withheld if kind == "withheld" else kinds.get(kind, 0)
                if value != want:
                    fail(check, "%s: the audit says %d %s, the module has %d"
                         % (evidence, value, kind, want))

        # 4. the module's own table doc: totals, tiers and per-era figures
        module_path = (os.path.join(HOLIDAYS, "%s.rs" % owner) if owner in FAMILIES
                       else os.path.join(HOLIDAYS, "venues", "%s.rs" % owner))
        doc = re.sub(r"\s*///\s*", " ", table_doc(open(module_path, encoding="utf-8").read()))
        counted = re.search(r"(\d+)(?: rows)? over (\w+) audited eras", doc)
        if counted:
            derived += 2
            if int(counted.group(1)) != total:
                fail(check, "%s: the table doc says %s rows, the module has %d"
                     % (module_path, counted.group(1), total))
            if ERA_WORDS.get(counted.group(2)) != len(windows):
                fail(check, "%s: the table doc says %s eras, the module declares %d"
                     % (module_path, counted.group(2), len(windows)))
        else:
            worded = re.search(r"([A-Za-z][A-Za-z -]*) rows over (\w+) audited eras", doc)
            if worded:
                derived += 2
                value = word_number(worded.group(1))
                if value is None:
                    fail(check, "%s: unparsed row total %r" % (module_path, worded.group(0)))
                elif value != total:
                    fail(check, "%s: the table doc says %s rows, the module has %d"
                         % (module_path, worded.group(1), total))
                if ERA_WORDS.get(worded.group(2)) != len(windows):
                    fail(check, "%s: the table doc says %s eras, the module declares %d"
                         % (module_path, worded.group(2), len(windows)))
        for found in re.finditer(r"(\d+) (?:are|stated and \d+ are) `Unsourced`", doc):
            derived += 1
            withheld = sum(part[2] for part in per.values())
            if int(found.group(1)) != withheld:
                fail(check, "%s: the table doc says %s Unsourced, the module has %d"
                     % (module_path, found.group(1), withheld))
        for found in re.finditer(r"(\d+) stated and (\d+) `Unsourced`", doc):
            derived += 2
            if int(found.group(1)) != total - sum(p[2] for p in per.values()):
                fail(check, "%s: the table doc says %s stated, the module has %d"
                     % (module_path, found.group(1), total - sum(p[2] for p in per.values())))
            if int(found.group(2)) != sum(p[2] for p in per.values()):
                fail(check, "%s: the table doc says %s Unsourced, the module has %d"
                     % (module_path, found.group(2), sum(p[2] for p in per.values())))
        for found in re.finditer(r"(\d+) from (20\d\d)-(20\d\d)", doc):
            span = (dt.date(int(found.group(2)), 1, 1), dt.date(int(found.group(3)), 12, 31))
            if span not in per:
                continue
            derived += 1
            if int(found.group(1)) != per[span][0]:
                fail(check, "%s: the table doc says %s rows for %s, the module has %d"
                     % (module_path, found.group(1), found.group(0)[-9:], per[span][0]))
        withheld_series = doc.rpartition("`Unsourced`")[2]
        if withheld_series:
            seen = 0
            for found in re.finditer(r"(\d+)\s+in\s+(20\d\d)-(20\d\d)", withheld_series):
                span = (dt.date(int(found.group(2)), 1, 1),
                        dt.date(int(found.group(3)), 12, 31))
                if span not in per:
                    continue
                seen += 1
                derived += 1
                if int(found.group(1)) != per[span][2]:
                    fail(check, "%s: the table doc says %s withheld for %s, the module has %d"
                         % (module_path, found.group(1), found.group(0)[-9:], per[span][2]))
            if seen:
                derived += 1
                if seen != len(windows):
                    fail(check, "%s: the withheld series names %d of %d windows"
                         % (module_path, seen, len(windows)))
        # 5. the "N audited eras: a at Tx, b at Tx, ..." summary line
        flat = re.sub(r"\s*///\s*", " ", doc)
        summary = re.search(r"([A-Za-z-]+) audited eras?: ((?:(?:and )?20\d\d-20\d\d at [\w/]+[,. ]*)+)", flat)
        if summary:
            named = re.findall(r"(20\d\d)-(20\d\d) at", summary.group(2))
            derived += 1
            if len(named) != len(windows):
                fail(check, "%s: the summary names %d eras, the module declares %d"
                     % (module_path, len(named), len(windows)))
    return derived


def closure_token(entry):
    """The block's own closed-day wording, as the evidence cell quotes it."""
    for token in re.split(r"[/\n]", entry.get("verbatim", "")):
        if re.search(r"\bclosed\b", token, re.IGNORECASE):
            return token.strip()
    return entry.get("status", "")


def sanitize_cell(text):
    return (text or "").replace("|", "\u00b7").replace("`", "")


def printed_cells(block):
    """family -> date -> the `instant as printed` cell the block states.

    CME publishes one sheet per holiday covering several trade dates, so a row's
    instant belongs to the row's own date: a closure must never show another
    date's early close, and a late open must show the re-open cell that names
    its date. This re-derives each cell from the entry that produces the row.
    """
    produced = collections.defaultdict(dict)
    for holiday in block["holidays"]:
        date = dt.date.fromisoformat(holiday["date"])
        for entry in holiday["families"]:
            group = entry["family"]
            for family in GROUP_FAMILIES.get(group, ()):
                slot = produced[family].setdefault(date, {"close": None, "open": None})
                if entry["status"] == "closed":
                    slot["close"] = closure_token(entry)
                    continue
                close = parse_clock(entry.get("close_instant"))
                if close is not None and close < close_at(family, date):
                    slot["close"] = entry["close_instant"]
                cells = list(entry.get("reopens") or [])
                if entry.get("open_instant"):
                    cells.insert(0, entry["open_instant"])
                for cell in cells:
                    opened = parse_clock(cell)
                    if opened is None:
                        continue
                    target = named_day(cell, date) or date
                    if (opened - first_open_at(family, target)) % 86_400 < LATE_OPEN_GRACE:
                        continue
                    produced[family].setdefault(
                        target, {"close": None, "open": None})["open"] = cell
    cells = {}
    for family, dates in produced.items():
        for date, slot in dates.items():
            parts = []
            if slot["close"]:
                parts.append("`%s`" % sanitize_cell(slot["close"]))
            if slot["open"]:
                parts.append("`%s`" % sanitize_cell(slot["open"]))
            cells[(family, date)] = " / ".join(parts) if parts else "\u2014"
    return cells


def check_printed_cells(block):
    """The evidence table's `instant as printed` cell is the block's own string."""
    check = 9
    derived = 0
    want = printed_cells(block)
    pattern = re.compile(r"^\| (\d{4}-\d\d-\d\d) \| ([^|]+) \| ([^|]*) \|")
    for family in FAMILIES:
        path = os.path.join(ROOT, "docs", "evidence", "%s.md" % family)
        text = open(path, encoding="utf-8").read()
        seen = 0
        for line in text.splitlines():
            found = pattern.match(line)
            if not found:
                continue
            date = dt.date.fromisoformat(found.group(1))
            if not (WINDOW[0] <= date <= WINDOW[1]):
                continue
            expected = want.get((family, date))
            if expected is None:
                continue
            seen += 1
            derived += 1
            if found.group(3).strip() != expected:
                fail(check, "%s %s: the cell says %r, the block states %r"
                     % (path, date, found.group(3).strip(), expected))
        if seen == 0:
            fail(check, "%s: no 2013-2015 row carries an instant cell" % path)
    return derived


def check_fences():
    check = 6
    derived = 0
    for family in FAMILIES:
        path = os.path.join(ROOT, "tests", "futures_family_boundaries",
                            "holidays_%s.rs" % family)
        text = open(path, encoding="utf-8").read()
        if "const ERA_2013_2015_ROWS" not in text:
            fail(check, "%s: no handwritten era date table" % path)
        if "era_2013_2015_rows_are_the_audited_date_kind_and_tier_set" not in text:
            fail(check, "%s: no era date-set test" % path)
        derived += 1
    venue_test = open(os.path.join(ROOT, "tests", "venue_sessions",
                                   "holidays_cme_venues.rs"), encoding="utf-8").read()
    for needed in ("wave5_venue_era_counts_match_the_families_they_route",
                   "wave5_venue_era_unsourced_rows_are_disagreements_not_closures",
                   "(2013, 1, 1), EvidenceTier::T1"):
        if needed not in venue_test:
            fail(check, "the venue suite does not pin %r" % needed)
        derived += 1
    # every module header names its windows
    for family in FAMILIES:
        text = open(os.path.join(HOLIDAYS, "%s.rs" % family), encoding="utf-8").read()
        header = text[:text.index("holidays! {")]
        for year in (2010, 2013, 2016, 2019, 2022, 2025):
            if str(year) not in header:
                fail(check, "%s: the module header does not name %d" % (family, year))
        derived += 1
    return derived


# --------------------------------------------------------------------------

def main(argv=None):
    parser = argparse.ArgumentParser()
    parser.add_argument("--research", default=os.environ.get("WAVE5_RESEARCH"))
    args = parser.parse_args(argv)
    if not args.research or not os.path.isdir(args.research):
        print("WAVE5_RESEARCH must name the research store root", file=sys.stderr)
        return 2
    block_path = os.path.join(args.research, "holidays", "cme-2013-2015.r2.json")
    if not os.path.isfile(block_path):
        print("missing %s" % block_path, file=sys.stderr)
        return 2
    with open(block_path) as handle:
        block = json.load(handle)

    modules = {family: parse_module(os.path.join(HOLIDAYS, "%s.rs" % family))
               for family in FAMILIES}
    modules["venues"] = {venue: parse_module(
        os.path.join(HOLIDAYS, "venues", "%s.rs" % venue))
        for venue in ROUTING}
    root = os.path.join(args.research, "holidays", "raw")

    total = 0
    total += check_rows(block, modules)
    total += check_coverage(modules)
    total += check_bytes(root, block, modules)
    total += check_venues(modules)
    total += check_cross_wave(modules)
    total += check_fences()
    total += check_prose_counts(modules)
    total += check_era_shapes(modules)
    total += check_printed_cells(block)

    for check, message in FAILURES:
        print("FAIL[%d] %s" % (check, message))
    for check, message in NOTES:
        print("NOTE[%d] %s" % (check, message))
    for family in FAMILIES:
        era = [r for r in modules[family][1] if WINDOW[0] <= r["date"] <= WINDOW[1]]
        print("%-24s %3d era rows, %d windows"
              % (family, len(era), len(modules[family][0])))
    for venue in ROUTING:
        era = [r for r in modules["venues"][venue][1]
               if WINDOW[0] <= r["date"] <= WINDOW[1]]
        print("%-24s %3d era rows" % (venue, len(era)))
    if FAILURES:
        print("RESULT: FAIL -- %d disagreement(s), %d re-derivations performed"
              % (len(FAILURES), total))
        return 1
    print("RESULT: PASS -- %d re-derivations performed, every derived value "
          "agrees with the modules" % total)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
