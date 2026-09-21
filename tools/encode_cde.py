# SPDX-License-Identifier: MIT-0
"""Encode the CDE 2021-2026 holiday block into the crate's holiday module.

    CDE_RESEARCH=../exchange-hours-research python3 tools/encode_cde.py

Reads `holidays/cde-2021-2026.json` (the adversarially verified block) and
writes `src/calendar/schedules/holidays/coinbase_derivatives.rs`. Every row's
trade date, kind and document id come from the block; the citation labels are
here because they are prose.
"""
import json
import os
import pathlib
import sys

ROOT = pathlib.Path(__file__).resolve().parent.parent
STORE = os.environ.get("CDE_RESEARCH")
if not STORE:
    sys.exit("CDE_RESEARCH must name the research store root (the directory holding holidays/)")
BLOCK = pathlib.Path(STORE) / "holidays" / "cde-2021-2026.json"
if not BLOCK.exists():
    sys.exit(f"{BLOCK} not found")

LABELS = {
    "2021-07-05": "Independence Day observed",
    "2021-09-06": "Labor Day",
    "2021-11-25": "Thanksgiving Day",
    "2021-11-26": "Thanksgiving half day: Equity Products 12:15 CT, Energy Products 12:45 CT",
    "2021-12-24": "Christmas Day observed",
    "2022-01-17": "Martin Luther King Jr. Day",
    "2022-02-21": "Presidents' Day",
    "2022-04-15": "Good Friday",
    "2022-05-30": "Memorial Day",
    "2022-06-20": "Juneteenth",
    "2022-07-04": "Independence Day",
    "2022-09-05": "Labor Day",
    "2022-11-24": "Thanksgiving Day; notice 22-10 is listed but its PDF is unreachable",
    "2022-11-25": "Thanksgiving half day; notice 22-10 is listed but its PDF is unreachable",
    "2022-12-26": "Christmas Day observed",
    "2023-01-02": "New Year's Day observed",
    "2023-01-16": "Martin Luther King Jr. Day",
    "2023-02-20": "Presidents' Day",
    "2023-04-07": "Good Friday",
    "2023-05-29": "Memorial Day",
    "2023-06-19": "Juneteenth",
    "2023-07-04": "Independence Day",
    "2023-09-04": "Labor Day",
    "2023-11-23": "Thanksgiving Day",
    "2023-11-24": "Thanksgiving half day: Equity 12:15 CT, Energy 12:45 CT, 23x5 Crypto 12:45 CT",
    "2023-12-25": "Christmas Day",
    "2024-01-01": "New Year's Day",
    "2024-01-15": "Martin Luther King Jr. Day",
    "2024-02-19": "Presidents' Day",
    "2024-03-29": "Good Friday",
    "2024-05-27": "Memorial Day",
    "2024-06-19": "Juneteenth",
    "2024-07-04": "Independence Day",
    "2024-09-02": "Labor Day",
    "2024-11-28": "Thanksgiving Day",
    "2024-11-29": "Thanksgiving half day; Energy, Metal and Crypto each print 13:45 CT",
    "2024-12-24": "Christmas Eve half day: Energy and Metal 12:45 CT; the 23x5 Crypto group trades to 16:00 CT",
    "2024-12-25": "Christmas Day",
    "2025-01-01": "New Year's Day",
    "2025-01-20": "Martin Luther King Jr. Day",
    "2025-02-17": "Presidents' Day",
    "2025-04-18": "Good Friday",
    "2025-05-26": "Memorial Day",
    "2025-06-19": "Juneteenth",
    "2025-07-04": "Independence Day",
    "2025-09-01": "Labor Day",
    "2025-11-27": "Thanksgiving Day",
    "2025-11-28": "Thanksgiving half day: Equity 12:15 CT, Energy & Metal 13:45 CT",
    "2025-12-24": "Christmas Eve half day: Equity 12:15 CT, Energy & Metal 12:45 CT",
    "2025-12-25": "Christmas Day",
    "2026-01-01": "New Year's Day, Energy & Metal and Equity closed for holiday",
    "2026-01-19": "Martin Luther King Jr. Day",
    "2026-02-16": "Presidents' Day",
    "2026-04-03": "Good Friday",
    "2026-05-25": "Memorial Day",
    "2026-06-19": "Juneteenth; the 23x5 and 24x5 tiers close while the 24x7 tier stays open",
    "2026-07-03": "Independence Day observed",
    "2026-09-07": "Labor Day",
}
# The 2026 rows already shipped carry these labels; keep them byte-stable.
KEEP = {
    "2026-01-01": "New Year's Day, Energy & Metal and Equity closed for holiday; trade date 1/2 opens 1/1 17:00 CT.",
    "2026-06-19": "Juneteenth; the 23x5 and 24x5 tiers close while Gold and Silver stay open with the 24x7 tier.",
}

HEADER = '''// SPDX-License-Identifier: MIT-0

//! Coinbase Derivatives holiday rows, from the venue's first trade date
//! 2021-06-28 through 2026-09-07.
//!
//! Keyed by the crate's own venue-local trade date in `America/Chicago`
//! (design memo D1). CDE's Market Notices are already keyed by **trade date**
//! and print the open, close and roll instants of the neighbouring dates
//! beside each one, so the conversion is the identity and the operator's own
//! table corroborates it row for row.
//!
//! The venue default this table attaches to is CDE's recurring 23x5 futures
//! grid - Sunday to Friday, 17:00-16:00 CT with the daily 16:00-17:00 break -
//! so every row is the **intersection** of the product groups the notices list
//! on that grid: where any group on it closes for the date the row is
//! `Closed`, and where none closes but any ends early the row carries the
//! **earliest** of those closes. Either way the venue never reports a window
//! in which no product on the grid can print, and each group's own printed
//! cell is quoted on the evidence row, so a session the venue row withholds
//! stays visible.
//!
//! That intersection has teeth on three dates. On 2023-06-19 and 2024-06-19
//! the notices close Equity and Energy - and Metal - while their `Crypto
//! Products` row prints a full `06/18 17:00 CT 06/19 16:00 CT` session; crypto
//! was still on this same 23x5 clock then, because CDE did not enable 24x7
//! trading until 2025-05-09. On 2025-06-19 a `23x5 Crypto` row prints that same
//! session beside the closed `Energy & Metal` row, with a separate `24x7
//! Crypto` row open as well. All three ship `Closed` and withhold a session
//! that did trade on this grid.
//!
//! The tiers off this grid - the 24x7 crypto tier from 2025-05-09, and a 24x5
//! equity-index PSF group from 2026 - claim no key and add no row of their own.
//! All of it is recorded in
//! [`docs/evidence/coinbase_derivatives.md`](../../../../../docs/evidence/coinbase_derivatives.md).
//!
//! Six dates carry an early close and two are `Unsourced`:
//! 2022-11-24 and 2022-11-25, whose notice 22-10 the operator lists but whose
//! PDF is unreachable (issue #112), so the crate declines to claim those dates
//! either way.
//! Trade date 2025-01-09 ships no row because notice 24-27 states it "will
//! observe a normal trading day", and notice 24-26 records an unplanned
//! technical early close rather than a published holiday schedule.
//!
//! Every row is **T1**, a numbered CDE Market Notice. Coverage runs from the
//! `FairX` launch day to 2026-09-07, the end of the window the first table
//! shipped; the notices for 2026-09-08 onward belong to the published-future
//! refresh, and CDE has published nothing for 2027.

use super::fences::early_close;
use super::{
    EvidenceTier::T1,
    HolidayKind::{Closed, Unsourced},
    HolidayTable, holidays,
};

/// The venue's built-in holiday rows and the window they were audited over.
// Evidence: docs/evidence/coinbase_derivatives.md
pub(crate) static TABLE: &HolidayTable = holidays! {
    coverage: [(2021, 6, 28) ..= (2026, 9, 7)],
    rows: [
'''

block = json.loads(BLOCK.read_text())
rows = block["crate_rows"]
seen = set()
lines = []
for r in rows:
    day = r["trade_date"]
    if day in seen:
        sys.exit(f"duplicate trade date in block: {day}")
    seen.add(day)
    year, month, date = (int(p) for p in day.split("-"))
    label = KEEP.get(day, LABELS.get(day))
    if label is None:
        sys.exit(f"no citation label for {day}; add one to LABELS")
    if r["kind"] == "closed":
        kind = "Closed"
    elif r["kind"] == "early_close":
        ssm = r["close_ssm"]
        hours, rest = divmod(ssm, 3_600)
        minutes = rest // 60
        kind = (
            f"early_close({hours} * 3_600)"
            if minutes == 0
            else f"early_close({hours} * 3_600 + {minutes} * 60)"
        )
    elif r["kind"] == "unsourced":
        kind = "Unsourced"
    else:
        sys.exit(f"{day}: unrepresentable kind {r['kind']!r}")
    # rustfmt wraps the long early_close rows; emit them already wrapped.
    prefix = f"        ({year}, {month}, {date}, {kind}, T1, \"{r['document']}\"),"
    if len(prefix) <= 100:
        body = prefix
    else:
        body = (
            f"        ({year}, {month}, {date}, {kind}, T1, \"{r['document']}\"),"
        )
    lines.append(f"        // {day} - T1 - {r['document']} - {label}")
    lines.append(body)
text = HEADER + "\n".join(lines) + "\n    ],\n};\n"
target = ROOT / "src" / "calendar" / "schedules" / "holidays" / "coinbase_derivatives.rs"
target.write_text(text)
kinds = {}
for r in rows:
    kinds[r["kind"]] = kinds.get(r["kind"], 0) + 1
print(f"wrote {target.relative_to(ROOT)}: {len(rows)} rows {kinds}")
