#!/usr/bin/env python3
# SPDX-License-Identifier: MIT-0
"""handle_unsourced_2026_10_09.py -- the 2026-10-09 globex holiday work-up.

Mechanical row surgery on the eight globex family holiday tables:

1. The three Juneteenth `Unsourced` rows (2019/2020/2021-06-19) are deleted in
   every family: the operator's own consolidated annual bundles enumerate every
   Globex holiday schedule published in those years and name no Juneteenth
   arrangement, so the dates are audited normal.
2. 2023-02-20 and 2023-04-07 are worked up from the operator's own summary
   sheets `files/presidents-day.pdf` and `files/good-friday.pdf` (already
   stored and cited by grains and interest rates) for the families whose rows
   the sheets state. FX and cryptocurrency state their ordinary Monday close on
   2023-02-20, so their `Unsourced` rows are deleted (audited normal).
   `globex_nikkei_225_dollar` keeps its markers: no read document prints a
   Nikkei line on those dates.

Stdlib only, deterministic, idempotent (a second run finds nothing to change).
"""

from __future__ import annotations

import re
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
TABLES = ROOT / "src/calendar/schedules/holidays"

PRESIDENTS = "files/presidents-day.pdf @2023-03-29T11:57:47Z"
GOOD_FRIDAY = "files/good-friday.pdf @2024-07-08T16:00:09Z"

# (file, date-tuple) -> action
#   "delete"                        row + comment removed (audited normal)
#   ("row", kind_expr, comment)     row replaced, comment replaced
ACTIONS: dict[tuple[str, str], tuple] = {}


def _juneteenth(year: str) -> None:
    for name in (
        "globex_cryptocurrency",
        "globex_energy",
        "globex_equity_index",
        "globex_fx",
        "globex_grains",
        "globex_interest_rates",
        "globex_livestock",
        "globex_nikkei_225_dollar",
    ):
        ACTIONS[(name, f"{year}, 6, 19")] = ("delete",)


for year in ("2019", "2020", "2021"):
    _juneteenth(year)

ACTIONS[("globex_energy", "2023, 2, 20")] = (
    "row",
    "early_close(13 * 3_600 + 30 * 60)",
    "T1 - " + PRESIDENTS + " - the printed Monday final close 13:30 CT is earlier than the ordinary 16:00 CT close.",
)
ACTIONS[("globex_equity_index", "2023, 2, 20")] = (
    "row",
    "early_close(12 * 3_600)",
    "T1 - " + PRESIDENTS + " - the printed Monday final close 12:00 CT is earlier than the ordinary 16:00 CT close.",
)
ACTIONS[("globex_livestock", "2023, 2, 20")] = (
    "row",
    "Closed",
    "T1 - " + PRESIDENTS + " - the sheet prints no Monday column: no session belongs to the trade date.",
)
ACTIONS[("globex_cryptocurrency", "2023, 2, 20")] = ("delete",)
ACTIONS[("globex_fx", "2023, 2, 20")] = ("delete",)

ACTIONS[("globex_energy", "2023, 4, 7")] = (
    "row",
    "Closed",
    "T1 - " + GOOD_FRIDAY + " - the sheet prints no Friday leg: no session belongs to the trade date.",
)
ACTIONS[("globex_equity_index", "2023, 4, 7")] = (
    "row",
    "early_close(8 * 3_600 + 15 * 60)",
    "T1 - " + GOOD_FRIDAY + " - the printed Friday final close 08:15 CT is earlier than the ordinary 16:00 CT close.",
)
ACTIONS[("globex_fx", "2023, 4, 7")] = (
    "row",
    "early_close(10 * 3_600 + 15 * 60)",
    "T1 - " + GOOD_FRIDAY + " - the printed Friday final close 10:15 CT is earlier than the ordinary 16:00 CT close.",
)
ACTIONS[("globex_cryptocurrency", "2023, 4, 7")] = (
    "row",
    "early_close(10 * 3_600 + 15 * 60)",
    "T1 - " + GOOD_FRIDAY + " - the printed Friday final close 10:15 CT is earlier than the ordinary 16:00 CT close.",
)
ACTIONS[("globex_livestock", "2023, 4, 7")] = (
    "row",
    "Closed",
    "T1 - " + GOOD_FRIDAY + " - the sheet prints no Friday leg: no session belongs to the trade date.",
)

ROW_RE = re.compile(r"^\s*\((?P<date>\d{4}, \d{1,2}, \d{1,2}), (?P<kind>Unsourced|early_close\([^)]*\)|Closed), T[12], \"(?P<doc>[^\"]+)\"\),\s*$")


def main() -> None:
    per_file: dict[str, list[tuple]] = {}
    for (name, date), action in ACTIONS.items():
        per_file.setdefault(name, []).append((date, action))

    changed = 0
    for name, items in sorted(per_file.items()):
        path = TABLES / f"{name}.rs"
        lines = path.read_text().splitlines(keepends=True)
        out: list[str] = []
        i = 0
        hits: list[str] = []
        while i < len(lines):
            line = lines[i]
            match = ROW_RE.match(line)
            if match and match.group("kind") == "Unsourced":
                date_key = match.group("date")
                action = next((a for d, a in items if d == date_key), None)
                if action is not None:
                    # The comment line directly above belongs to this row.
                    comment_at = len(out) - 1 if out and out[-1].lstrip().startswith("//") else None
                    comment = out[comment_at] if comment_at is not None else ""
                    if action[0] == "delete":
                        if comment_at is not None:
                            out.pop()
                        hits.append(f"{date_key}: deleted")
                        i += 1
                        continue
                    _, kind_expr, basis = action
                    tier, doc, why = basis.split(" - ", 2)
                    indent = line[: len(line) - len(line.lstrip())]
                    new_comment = f"{indent}// {date_key} - {tier} - {doc} - {why}\n"
                    new_row = f"{indent}({date_key}, {kind_expr}, {tier}, \"{doc}\"),\n"
                    if comment_at is not None:
                        out[comment_at] = new_comment
                    else:
                        out.append(new_comment)
                    out.append(new_row)
                    hits.append(f"{date_key}: -> {kind_expr}")
                    i += 1
                    continue
            out.append(line)
            i += 1
        if hits:
            path.write_text("".join(out))
            changed += len(hits)
            print(f"{name}.rs:")
            for hit in hits:
                print(f"  {hit}")
    print(f"total: {changed} row changes")


if __name__ == "__main__":
    main()
