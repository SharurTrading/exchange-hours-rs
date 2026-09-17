#!/usr/bin/env python3
# SPDX-License-Identifier: MIT-0
"""encode_wave5.py -- splice the wave-5 rows into the six family modules.

Reads the encoding plan wave5_rows.py wrote and rewrites, in place,

    src/calendar/schedules/holidays/globex_equity_index.rs
    src/calendar/schedules/holidays/globex_energy.rs
    src/calendar/schedules/holidays/globex_fx.rs
    src/calendar/schedules/holidays/globex_grains.rs
    src/calendar/schedules/holidays/globex_interest_rates.rs
    src/calendar/schedules/holidays/globex_livestock.rs

with three mechanical edits, none of which touches prose:

  1. the `coverage:` clause gains `(2013, 1, 1) ..= (2015, 12, 31)` in
     ascending order;
  2. each new row is inserted above the first row whose year is on or after
     2013, so the table stays strictly ascending — the anchor is the *comment
     block* above that row, never the row itself, because a row's citation line
     sits directly above it;
  3. every kind the plan uses must already be spelled in the module above the
     `holidays!` block, so a new kind cannot arrive unseen.

Not idempotent by design: a second run would duplicate the rows.  It refuses
instead, and says so.

Usage:  python3 tools/encode_wave5.py . tools/out/wave5/ROWS.json
"""

from __future__ import annotations

import json
import pathlib
import re
import sys

ROOT = None

ORDER = (
    "globex_equity_index",
    "globex_energy",
    "globex_fx",
    "globex_grains",
    "globex_interest_rates",
    "globex_livestock",
)

ERA = ((2013, 1, 1), (2015, 12, 31))

WINDOW_RE = re.compile(
    r"\(\s*(\d+)\s*,\s*(\d+)\s*,\s*(\d+)\s*\)\s*\.\.=\s*"
    r"\(\s*(\d+)\s*,\s*(\d+)\s*,\s*(\d+)\s*\)")

KIND_EXPR = {
    "Closed": lambda row: "Closed",
    "Unsourced": lambda row: "Unsourced",
    "EarlyClose": lambda row: "early_close(%s)" % ssm(row["close_ssm"]),
    "LateOpen": lambda row: "late_open(%s)" % ssm(row["open_ssm"]),
    "LateOpenAndEarlyClose": lambda row: "late_open_and_early_close(%s, %s)" % (
        ssm(row["open_ssm"]), ssm(row["close_ssm"])),
}

KIND_SYMBOL = {
    "Closed": "Closed",
    "Unsourced": "Unsourced",
    "EarlyClose": "early_close",
    "LateOpen": "late_open",
    "LateOpenAndEarlyClose": "late_open_and_early_close",
}


def ssm(value):
    if value is None:
        raise ValueError("a row needs the instant its kind states")
    hours, rest = divmod(value, 3_600)
    minutes, seconds = divmod(rest, 60)
    if seconds:
        raise ValueError("sub-minute instant %d" % value)
    parts = []
    if hours:
        parts.append("%d * 3_600" % hours)
    if minutes:
        parts.append("%d * 60" % minutes)
    return " + ".join(parts) if parts else "0"


def render(row):
    date = row["date"]
    year, month, day = (int(part) for part in date.split("-"))
    return (
        "        // %s - %s - %s - %s.\n"
        '        (%d, %d, %d, %s, %s, "%s"),\n'
        % (date, row["tier"], row["document"], row["reason"].strip().rstrip("."),
           year, month, day, KIND_EXPR[row["kind"]](row), row["tier"],
           row["document"]))


def main(argv):
    if len(argv) != 3:
        print(__doc__)
        return 2
    root = pathlib.Path(argv[1]).resolve()
    plan_path = pathlib.Path(argv[2])
    if not plan_path.is_absolute():
        plan_path = (pathlib.Path.cwd() / plan_path).resolve()
    with plan_path.open() as handle:
        plan = json.load(handle)

    for family in ORDER:
        rows = plan["families"][family]
        path = root / "src" / "calendar" / "schedules" / "holidays" / ("%s.rs" % family)
        text = path.read_text()
        if re.search(r"^        \(2013, ", text, re.M):
            raise SystemExit("%s already holds 2013 rows; refusing to duplicate"
                             % path.name)

        # 1. the coverage clause
        match = re.search(r"coverage: \[([^\]]*)\],", text)
        if not match:
            raise SystemExit("%s: no coverage clause" % path.name)
        windows = [
            ((int(m[0]), int(m[1]), int(m[2])), (int(m[3]), int(m[4]), int(m[5])))
            for m in WINDOW_RE.findall(match.group(1))
        ]
        if not windows:
            raise SystemExit("%s: no windows parsed" % path.name)
        windows.append(ERA)
        windows.sort()
        for earlier, later in zip(windows, windows[1:]):
            if earlier[1] >= later[0]:
                raise SystemExit("%s: the era overlaps an existing window"
                                 % path.name)
        listing = ", ".join(
            "(%d, %d, %d) ..= (%d, %d, %d)" % (first + last)
            for first, last in windows)
        text = (text[:match.start()]
                + "coverage: [%s]," % listing
                + text[match.end():])

        # 2. every kind must already be spelled above the table
        prefix = text[:text.index("holidays! {")]
        for kind in {row["kind"] for row in rows}:
            symbol = KIND_SYMBOL[kind]
            if symbol not in prefix:
                raise SystemExit("%s: %s is not imported above the table"
                                 % (path.name, symbol))

        # 3. the rows, above the first row of 2013 or later and above its
        #    citation comment block
        lines = text.splitlines(keepends=True)
        anchor = None
        for index, line in enumerate(lines):
            found = re.match(r"^        \((\d{4}), ", line)
            if found and int(found.group(1)) >= ERA[0][0]:
                anchor = index
                break
        if anchor is None:
            raise SystemExit("%s: no row at or after %d to anchor on"
                             % (path.name, ERA[0][0]))
        while anchor > 0 and lines[anchor - 1].lstrip().startswith("//"):
            anchor -= 1
        block = "".join(render(row) for row in rows)
        lines.insert(anchor, block)
        path.write_text("".join(lines))
        print("%-32s +%3d rows, %d windows" % (path.name, len(rows), len(windows)))
    return 0


if __name__ == "__main__":
    raise SystemExit(main(sys.argv))
