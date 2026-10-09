#!/usr/bin/env python3
# SPDX-License-Identifier: MIT-0
"""venue_clock_mirror_2026_10_09.py -- regenerate the four CME venue tables.

The 2026-10-09 amendment completes the profile-clock rule: the venue ships its
clock family's row verbatim -- kind, tier and document id -- wherever the clock
states one, ships no row where the clock audits the date normal, and inherits
the clock's own `Unsourced` marker where the clock withholds. Each table is
therefore a byte-level mirror of one family table's coverage and rows:

    cbot  <- globex_grains          comex <- globex_energy
    cme   <- globex_equity_index    nymex <- globex_energy

Stdlib only, deterministic.
"""

from __future__ import annotations

import re
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
HOLIDAYS = ROOT / "src/calendar/schedules/holidays"

PER_ERA = re.compile(
    r"\((\d{4}), (\d{1,2}), (\d{1,2})\) ..= \((\d{4}), (\d{1,2}), (\d{1,2})\)"
)

VENUES = [
    ("cbot", "CBOT", "globex_grains", "the grain grid"),
    ("cme", "CME", "globex_equity_index", "the equity-index grid"),
    ("comex", "COMEX", "globex_energy", "the energy grid"),
    ("nymex", "NYMEX", "globex_energy", "the energy grid"),
]


def parse_family(family: str) -> dict:
    text = (HOLIDAYS / f"{family}.rs").read_text()
    start = text.index("holidays! {")
    body = text[start:]
    coverage = re.search(r"coverage: \[(.*?)\],", body, re.S).group(1)
    rows_start = body.index("rows: [") + len("rows: [")
    rows_end = body.index("\n    ],", rows_start)
    rows = body[rows_start:rows_end]
    eras = [
        ((int(a), int(b), int(c)), (int(d), int(e), int(f)))
        for a, b, c, d, e, f in PER_ERA.findall(coverage)
    ]
    counts = []
    for (ay, am, ad), (by, bm, bd) in eras:
        first = ay * 10000 + am * 100 + ad
        last = by * 10000 + bm * 100 + bd
        n = 0
        unsourced = 0
        for line in rows.splitlines():
            m = re.match(r"\s*\((\d{4}), (\d{1,2}), (\d{1,2}),", line)
            if not m:
                continue
            key = int(m[1]) * 10000 + int(m[2]) * 100 + int(m[3])
            if first <= key <= last:
                n += 1
                if "Unsourced" in line:
                    unsourced += 1
        counts.append((f"{ay}-{by}", n, unsourced))
    helpers = sorted(
        {h for h in ("early_close", "late_open", "late_open_and_early_close") if re.search(rf"\b{h}\(", rows)}
    )
    consts = set(re.findall(r"\b([A-Z][A-Z0-9_]+)\b", rows))
    declared = set(re.findall(r"^pub\(crate\) (?:const|static) ([A-Z][A-Z0-9_]+)", text, re.M))
    constants = sorted({c for c in consts if c.endswith("_BLOCKS") or c in declared})
    kinds = sorted(set(re.findall(r"\b(Closed|ReplacementBlocks|Unsourced)\b", rows)))
    return {
        "coverage": coverage,
        "rows": rows,
        "counts": counts,
        "helpers": helpers,
        "constants": constants,
        "kinds": kinds,
    }


def human_counts(counts: list) -> str:
    parts = [f"{era} {n}" for era, n in counts]
    return ", ".join(parts[:-1]) + f" and {parts[-1]}"


def docstring(venue: str, clock: str, family: str, grid: str, info: dict) -> str:
    total = sum(n for _, n, _ in info["counts"])
    unsourced_total = sum(u for _, _, u in info["counts"])
    eras = human_counts([(era, n) for era, n, _ in info["counts"]])
    unsourced_note = (
        ""
        if unsourced_total == 0
        else (
            f" The table carries **{unsourced_total} `Unsourced` row"
            f"{'s' if unsourced_total != 1 else ''}** — the clock family's own"
            " not-worked-up markers, inherited because the venue answers every"
            " date its clock answers and withholds every date its clock"
            " withholds; each marker's closing condition is recorded beside the"
            " family's row and in the family's evidence file."
        )
    )
    return f"""//! The `Exchange::{clock_title(venue)}` table holiday rows — the clock family `{family}`, carried verbatim.
//!
//! Derived, not retrieved. The 2026-10-09 amendment completes the
//! profile-clock rule (AGENTS.md, LAW-HOLIDAY-SCOPE; the 2026-09-30 decision
//! on issue #153, retired artefacts by #242): the venue speaks for its profile
//! clock — {grid} — so wherever the clock states a row the venue ships that
//! row verbatim (kind, tier and document id), wherever the clock audits the
//! date normal the venue ships no row, and wherever the clock itself withholds
//! the venue inherits the marker. A routed family that prints a different
//! arrangement on a date the clock answers keeps its own sourced row in its
//! own family table, where the consumer's exact-family routing (#118) reads
//! it; the venue's evidence file names those disagreements.
//!
//! Coverage is {total} rows over the six audited eras: {eras}.{unsourced_note}
//!
//! Evidence: docs/evidence/{venue}.md, and the family's evidence file for
//! every retrieved row.
"""


def clock_title(venue: str) -> str:
    return {"cbot": "Cbot", "cme": "Cme", "comex": "Comex", "nymex": "Nymex"}[venue]


def emit(venue: str, static_name: str, family: str, grid: str) -> None:
    info = parse_family(family)
    imports = ["EvidenceTier::{T1, T2}", "HolidayTable", "holidays"]
    kind_imports = []
    for kind in ("Closed", "LateOpenAndEarlyClose", "ReplacementBlocks", "Unsourced"):
        if kind in info["kinds"]:
            kind_imports.append(kind)
    if info["kinds"]:
        imports.insert(1, "HolidayKind::{%s}" % ", ".join(kind_imports))
    if "early_close" in str(info["helpers"]):
        pass
    helper_line = ""
    if info["helpers"]:
        helper_line = "    fences::{%s},\n" % ", ".join(info["helpers"])
    constants_line = ""
    if info["constants"]:
        constants_line = "    %s::{%s},\n" % (
            family,
            ",\n        ".join(info["constants"]),
        )
    import_block = "use super::super::{\n    %s\n};\n" % "\n    ".join(
        [i + "," for i in imports]
    )
    # house layout: group fences + family constants into the same use tree
    import_block = (
        "use super::super::{\n"
        + "".join(f"    {i},\n" for i in imports)
        + helper_line
        + constants_line
        + "};\n"
    )
    content = (
        "// SPDX-License-Identifier: MIT-0\n\n"
        + docstring(venue, static_name, family, grid, info)
        + import_block
        + f"\n/// The `Exchange::{clock_title(venue)}` table: the `{family}` rows, carried verbatim.\n"
        f"///\n"
        f"/// Every row below is the profile clock's own row on that trade date —\n"
        f"/// kind, tier and document id unchanged — because the venue answers from\n"
        f"/// its clock. The clock's evidence file cites every artifact.\n"
        f"// Evidence: docs/evidence/{venue}.md\n"
        f"pub(crate) static {static_name}: &HolidayTable = holidays! {{\n"
        f"    coverage: [{info['coverage']}],\n"
        f"    rows: [{info['rows']}\n    ],\n"
        f"}};\n"
    )
    (HOLIDAYS / "venues" / f"{venue}.rs").write_text(content)
    print(f"{venue}.rs <- {family}.rs: {sum(n for _, n, _ in info['counts'])} rows, "
          f"{sum(u for _, _, u in info['counts'])} inherited Unsourced")


def main() -> None:
    for venue, static_name, family, grid in VENUES:
        emit(venue, static_name, family, grid)


if __name__ == "__main__":
    main()
