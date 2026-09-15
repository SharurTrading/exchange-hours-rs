#!/usr/bin/env python3
"""Encode the wave-3 (2022-2024) holiday rows into the eight family modules.

Usage: encode_rows.py <worktree-root> <rows.json>

Mechanical only: it sets each module's coverage clause, extends its
`use super::{...HolidayKind...}` list when the rows need `Unsourced`, and
inserts the new row tuples (with their citation comments) after the module's
last pre-2022 row. It does not touch module doc prose.
"""
import json
import re
import sys

ORDER = [
    "globex_equity_index",
    "globex_energy",
    "globex_fx",
    "globex_grains",
    "globex_interest_rates",
    "globex_livestock",
    "globex_cryptocurrency",
    "globex_nikkei_225_dollar",
]

KIND_EXPR = {
    "Closed": "Closed",
    "Unsourced": "Unsourced",
    "EarlyClose": lambda r: "early_close(%s)" % ssm(r["close_ssm"]),
    "LateOpen": lambda r: "late_open(%s)" % ssm(r["open_ssm"]),
    "LateOpenAndEarlyClose": lambda r: "late_open_and_early_close(%s, %s)"
    % (ssm(r["open_ssm"]), ssm(r["close_ssm"])),
}


def ssm(value):
    """Renders seconds-since-midnight the way the shipped modules do."""
    hours, rem = divmod(int(value), 3600)
    minutes, seconds = divmod(rem, 60)
    if seconds:
        raise SystemExit("unexpected seconds in %r" % value)
    term = "%d * 3_600" % hours if hours else ""
    if minutes:
        term = (term + " + %d * 60" % minutes) if term else "%d * 60" % minutes
    return term or "0"


def kind_of(row):
    expr = KIND_EXPR[row["kind"]]
    return expr(row) if callable(expr) else expr


def render(rows):
    out = []
    for row in rows:
        year, month, day = (int(part) for part in row["date"].split("-"))
        document = row["document"]
        reason = row.get("reason", "").strip().rstrip(".")
        out.append(
            "        // %s - %s - %s - %s."
            % (row["date"], row["tier"], document, reason)
        )
        out.append(
            '        (%d, %d, %d, %s, %s, "%s"),'
            % (year, month, day, kind_of(row), row["tier"], document)
        )
    return "\n".join(out)


def main():
    root = sys.argv[1].rstrip("/")
    plan = json.load(open(sys.argv[2]))
    families = {entry["family"]: entry for entry in plan["families"]}

    for family in ORDER:
        path = "%s/src/calendar/schedules/holidays/%s.rs" % (root, family)
        text = open(path).read()
        entry = families[family]
        rows = entry["rows"]

        # 1. Coverage clause: union the module's existing windows with the
        #    wave's own, in ascending order, never replacing what shipped.
        wave = [tuple(w) for w in (entry.get("coverage") or [[2022, 1, 1, 2024, 12, 31]])]
        match = re.search(r"coverage: \[([^\]]*)\],", text)
        if match is None:
            raise SystemExit("%s: no coverage clause" % path)
        existing = [
            tuple(int(x) for x in window)
            for window in re.findall(
                r"\(\s*(\d{4})\s*,\s*(\d{1,2})\s*,\s*(\d{1,2})\s*\)\s*\.\.=\s*"
                r"\(\s*(\d{4})\s*,\s*(\d{1,2})\s*,\s*(\d{1,2})\s*\)",
                match.group(1),
            )
        ]
        merged = sorted({window for window in existing + wave})
        for first, second in zip(merged, merged[1:]):
            if first[3:] >= second[:3]:
                raise SystemExit("%s: overlapping windows %r %r" % (path, first, second))
        clause = ", ".join(
            "(%d, %d, %d) ..= (%d, %d, %d)" % window for window in merged
        )
        text = text[: match.start()] + "coverage: [%s]," % clause + text[match.end() :]
        print("%-26s windows %s" % (family, [w for w in merged]))

        # 2. `Unsourced` in scope, when a row needs it. Each module imports
        #    `HolidayKind` in one of three shapes, listed here so a module that
        #    changes shape fails loudly rather than silently acquiring a second
        #    import. Only header lines are touched, never a row tuple.
        if any(row["kind"] == "Unsourced" for row in rows):
            shapes = [
                ("    HolidayKind::Closed,", "    HolidayKind::{Closed, Unsourced},"),
                (
                    "    HolidayKind::Closed,",
                    "    HolidayKind::{Closed, Unsourced},",
                ),
                ("use super::HolidayKind::Closed;", "use super::HolidayKind::{Closed, Unsourced};"),
                (
                    "use super::{EvidenceTier::T2, HolidayKind, HolidayTable, holidays};",
                    "use super::{\n"
                    "    EvidenceTier::{T1, T2},\n"
                    "    HolidayKind,\n"
                    "    HolidayKind::{Closed, Unsourced},\n"
                    "    HolidayTable, holidays,\n"
                    "};",
                ),
            ]
            lines = text.split("\n")
            for index, line in enumerate(lines):
                for needle, replacement in shapes:
                    if line == needle:
                        lines[index] = replacement
                        break
                else:
                    continue
                break
            else:
                raise SystemExit(
                    "%s: rows need Unsourced but no known import shape matched" % path
                )
            text = "\n".join(lines)

        # 3. Insert the rows after the last row dated before 2022-01-01.
        row_re = re.compile(r"^        \(2\d{3}, \d+, \d+, .*$", re.M)
        matches = list(row_re.finditer(text))
        last = None
        for match in matches:
            year = int(match.group(0).strip()[1:5])
            if year < 2022:
                last = match
        if last is None:
            # A family whose window opens after 2022 (globex_cryptocurrency)
            # has no earlier row: the new era goes in first.
            marker = re.search(r"rows: \[\n", text)
            if marker is None:
                raise SystemExit("%s: no rows: [ marker" % path)
            insert_at = marker.end()
        else:
            insert_at = last.end()
        block = "\n" + render(rows)
        text = text[:insert_at] + block + text[insert_at:]
        open(path, "w").write(text)
        print("%-26s +%d rows" % (family, len(rows)))


if __name__ == "__main__":
    main()
