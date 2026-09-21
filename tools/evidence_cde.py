# SPDX-License-Identifier: MIT-0
"""Rewrite the `## Holidays` section of docs/evidence/coinbase_derivatives.md.

    CDE_RESEARCH=../exchange-hours-research python3 tools/evidence_cde.py

The operator statements are read from the OPS literal in
`holidays/raw/cde-2021-2025/build_block.py` - the same literal the block was
built from - so a quoted cell cannot drift between the block and the evidence
file. The `### Documents` table is the fixed six-column shape
`tests/schedule_documentation/evidence_files.rs` requires.
"""
import ast
import json
import os
import pathlib
import re
import sys

ROOT = pathlib.Path(__file__).resolve().parent.parent
STORE = os.environ.get("CDE_RESEARCH")
if not STORE:
    sys.exit("CDE_RESEARCH must name the research store root")
STORE = pathlib.Path(STORE)
RAW = STORE / "holidays" / "raw" / "cde-2021-2025"

block = json.loads((STORE / "holidays" / "cde-2021-2026.json").read_text())
prov = json.loads((RAW / "provenance.json").read_text())

# Lift the OPS literal out of the builder so both artifacts share one source.
source = (RAW / "build_block.py").read_text()
start = source.index("OPS = [")
end = source.index("\n]", start) + 2
OPS = ast.literal_eval(source[start + len("OPS = ") : end])

def flat(text):
    return re.sub(r"\s+", " ", text)

by_date = {}
for notice, event, group, status, cell, trade_date, kind in OPS:
    by_date.setdefault(trade_date, []).append(
        {
            "notice": notice,
            "event": event,
            "group": group,
            "status": status,
            "cell": flat(cell),
            "kind": kind,
        }
    )

# Each notice's own governed trade-date window, from the statements it carries.
windows = {}
for notice, event, group, status, cell, trade_date, kind in OPS:
    lo, hi = windows.get(notice, (trade_date, trade_date))
    windows[notice] = (min(lo, trade_date), max(hi, trade_date))

WEEKDAY = ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday", "Sunday"]
import datetime

def pretty(day):
    d = datetime.date.fromisoformat(day)
    return f"{WEEKDAY[d.weekday()]} {d.month}/{d.day}"

KIND_CELL = {"closed": "closed", "early_close": "early close", "unsourced": "unsourced"}

STAMP = re.compile(r"(\d{2})/(\d{2})\s+(\d{2}):(\d{2})\s+CT")


def earliest_stamp(cell):
    """The last `MM/DD HH:MM CT` on a printed cell, in local seconds."""
    hits = STAMP.findall(cell)
    assert hits, f"an early-close cell must print an instant: {cell!r}"
    _month, _day, hours, minutes = hits[-1]
    return int(hours) * 3_600 + int(minutes) * 60


rows = []
for r in block["crate_rows"]:
    day, kind, doc = r["trade_date"], r["kind"], r["document"]
    entries = by_date.get(day, [])
    # A block row must be reproducible from the operator statements: refuse to
    # render evidence for a row the saved notices do not carry.
    contributed = [e for e in entries if e["kind"] is not None]
    if kind == "unsourced":
        assert not contributed, (
            f"{day}: the block ships Unsourced but the notices carry "
            + ", ".join(f"{e['notice']}:{e['status']}" for e in contributed)
        )
    else:
        assert contributed, f"{day}: the block ships {kind} with no supporting notice"
        kinds = {e["kind"] for e in contributed}
        assert kinds == {kind}, f"{day}: the block ships {kind} but the notices derive {kinds}"
        if kind == "early_close":
            derived = min(earliest_stamp(e["cell"]) for e in contributed)
            assert derived == r["close_ssm"], (
                f"{day}: the block carries close_ssm {r['close_ssm']} but the earliest "
                f"printed close is {derived}"
            )
    if kind == "unsourced":
        printed = "&mdash; no status claimed"
        derived = (
            "notice `CDE-MN-22-10` (\"Market Notice - Thanksgiving Holiday Schedule 2022\") "
            "governs this date; the operator lists it but its PDF is unreachable, so the date is "
            "carried as not audited rather than claimed closed"
        )
    elif kind == "closed":
        closing = [e for e in entries if e["status"] == "closed"]
        assert closing, f"{day}: a closed row must quote the notice's own cell"
        if entries[0]["notice"] in ("21-03", "21-04"):
            printed = f"`{entries[0]['cell']}`"
            derived = (
                f"{pretty(day)} is the observed holiday the notice's subject names; markets are "
                f"closed from 16:00 CT the preceding Friday to 17:00 CT that Monday, and the "
                f"Monday-evening session that follows belongs to the next trade date"
            )
        else:
            # Name every group, not only the closing ones: a group that prints a
            # full session on the same grid is a session the venue row withholds.
            parts = []
            for e in sorted(entries, key=lambda e: e["group"]):
                if e["status"] == "closed":
                    parts.append(f"{e['group']} `{e['cell']}`")
                else:
                    parts.append(
                        f"{e['group']} prints a full session `{e['cell']}` on the same 23x5 clock"
                    )
            printed = "; ".join(parts)
            withheld = len(closing) < len(entries)
            derived = (
                f"CDE's own Trade Date column names {pretty(day)}; the venue row carries the "
                f"intersection, so the whole trade date is closed"
                + (
                    " and the session quoted above as printing is withheld"
                    if withheld
                    else ", its OPEN and CLOSE cells printing the quoted text"
                )
            )
    else:
        parts = [f"{e['group']} `{e['cell']}`" for e in entries if e["status"] == "early_close"]
        normal = [e for e in entries if e["status"] == "normal"]
        printed = "; ".join(parts)
        if normal:
            printed += "; " + "; ".join(
                f"{e['group']} trades to 16:00 CT (`{e['cell']}`)" for e in normal
            )
        groups = ", ".join(sorted({e["group"] for e in entries}))
        derived = (
            f"CDE's own Trade Date column names {pretty(day)}; the venue row takes the earliest "
            f"close among the groups on the 23x5 grid ({groups}), so it never reports a window in "
            f"which no product prints"
        )
    rows.append(f"| {day} | {KIND_CELL[kind]} | {printed} | `{doc}` | T1 | {derived} |")

# --- fixed-shape documents table -------------------------------------------
doc_lines = []
# The row ids, plus the three notices the evidence prose cites while keying no row.
cited = set(r["document"] for r in block["crate_rows"])
cited |= {"CDE-NOTICES-INDEX-2026-09-19", "CDE-MN-24-26", "CDE-MN-24-27", "CDE-MN-26-27"}
for doc_id in sorted(cited):
    if doc_id == "CDE-NOTICES-INDEX-2026-09-19":
        doc_lines.append(
            "| `CDE-NOTICES-INDEX-2026-09-19` | 2021-06-01 .. 2026-09-10 | "
            "<https://www.coinbase.com/derivatives/market-notices> | retrieved 2026-09-19 02:02 UTC "
            "via the public reader (direct coinbase.com returns 403); raw bytes "
            "`cde_market_notices_raw.html`, sha256 "
            "`54816ca2d73db15058f89afecb88861b13fe7f16c2a22bcc3092cc4ddc5d0dbc` | T1 | "
            "`54816ca2d73db15058f89afecb88861b13fe7f16c2a22bcc3092cc4ddc5d0dbc` |"
        )
        continue
    notice = doc_id.replace("CDE-MN-", "")
    v = prov.get(notice)
    if not v or v["status"] != "ok":
        sys.exit(f"{doc_id} has no resolved artifact in provenance.json")
    lo, hi = windows.get(notice, ("n/a", "n/a"))
    window = f"{lo} .. {hi}" if lo != "n/a" else "n/a"
    doc_lines.append(
        f"| `{doc_id}` | {window} | <{v['url']}> | retrieved 2026-09-19 02:02-02:45 UTC "
        f"({v['channel']}) | T1 | `{v['sha256']}` |"
    )

section = f"""## Holidays

**Coverage:** 2021-06-28..2026-09-07 (inclusive trade dates). Tier: T1 throughout.

The venue profile is CDE's recurring 23x5 futures grid, so every row is the **intersection** of
the product groups the operator's notices list on that grid. Where any group on it closes for the
date the row is **closed**; where none closes but any ends early the row is an **early close** at
the earliest printed instant. Either way the venue never reports a window in which no product on
the grid can print, and every group's own printed cell is quoted on the row, so a session the
venue row withholds stays visible.

Three closures exercise the first half of that rule and each names the session it withholds. On
2023-06-19 and 2024-06-19 the notices close Equity and Energy (and, in 2024, Metal) while their
`Crypto Products` row prints a full `06/18 17:00 CT 06/19 16:00 CT` session — and crypto was on
**this same 23x5 clock** then, because CDE did not enable 24x7 trading until 2025-05-09. On
2025-06-19 a `23x5 Crypto` row prints that same session beside the closed `Energy & Metal` row,
with a separate `24x7 Crypto` row open as well. All three ship `closed` in the module; the module
doc comment says so too.

The window opens at the venue's first trade date, FairX's launch Monday 2021-06-28, and closes at
2026-09-07, where the table the crate first shipped stopped; the notices for 2026-09-08 onward
belong to the published-future refresh.

On a half day the groups can print different instants. Ordinary half days are the Friday after
Thanksgiving, Christmas Eve and New Year's Eve: in 2021-11-26, 2023-11-24, 2025-11-28 and
2025-12-24 the Equity group printed 12:15 CT while Energy, and later Metals, printed 12:45 or
13:45 CT; in 2024-12-24 Energy and Metal printed 12:45 CT while the 23x5 Crypto group traded to
16:00 CT. **The row carries the earliest instant**, because a venue row must never report a window
in which no product on the grid can print. The other group's instant is quoted in the same line,
so the residual under-report is visible: for 2024-12-24 the 23x5 Crypto group really traded to
16:00 CT, and for 2025-11-28 the Energy & Metal group really traded to 13:45 CT.

### Documents

| Document | Window | Replay or service URL | Capture or retrieval, UTC | Tier | sha256 |
|---|---|---|---|---|---|
{chr(10).join(doc_lines)}

All bytes are in the research store under `holidays/raw/cde-2021-2025/` with a URL, UTC retrieval
time and sha256 per artifact in its `INDEX.md`; the operator statements behind every row are the
`OPS` literal in that directory's `build_block.py`, and the verified result is
`holidays/cde-2021-2026.json`.

### 2021

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
{chr(10).join(r for r in rows if r.startswith("| 2021"))}

### 2022

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
{chr(10).join(r for r in rows if r.startswith("| 2022"))}

### 2023

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
{chr(10).join(r for r in rows if r.startswith("| 2023"))}

### 2024

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
{chr(10).join(r for r in rows if r.startswith("| 2024"))}

### 2025

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
{chr(10).join(r for r in rows if r.startswith("| 2025"))}

### 2026

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
{chr(10).join(r for r in rows if r.startswith("| 2026"))}

### Gaps and residual risks, 2021-2026

- **Notice 24-12 is headed "IN DRAFT".** It is the only notice in the corpus whose own
  published PDF carries that watermark, and it is the notice behind trade date 2024-06-19. The
  operator lists it at T1 with a posted date of 06/05/2024 and its grid is unambiguous, and a
  closure shipped in error errs toward closed rather than toward reporting a window in which no
  product traded, so the row stands. Residual risk: a later final revision is not held. Closing
  condition: a non-draft copy of notice 24-12. Tracked as issue #112.

- **Trade dates 2022-11-24 and 2022-11-25 ship `Unsourced`.** The operator's own listing carries
  notice 22-10, "Market Notice - Thanksgiving Holiday Schedule 2022", category Holiday, posted
  12/12/2022. Its href points at
  `info.fairx.com/coinbase-derivatives-market-notice-22-10-thanksgiving-holiday-schedule-2022`,
  where the host no longer completes a TLS handshake, and the Wayback Machine holds no capture of
  it: a CDX prefix query over `assets.ctfassets.net/k3n74unfin40*` returns 144 `Market_Notice`
  artifacts spanning ids 21-01 through 26-13, and none of them is 22-10, and a query over `info.fairx.com*`
  returns exactly one unrelated 2022 capture. `Unsourced` clips nothing, so the 23x5 grid applies
  unchanged; the crate simply declines to certify the date. Closing condition: any surviving copy
  of notice 22-10, or a later notice that restates the outgoing 2022 schedule. Tracked as issue
  #112.
- **Trade date 2021-12-31 is carried as audited normal and ships no row.** The operator published
  no notice for it: the 2021 listing runs 21-01 to 21-07 with no gaps, and 21-07 (issued
  2021-12-21, the last 2021 notice) covers Christmas only. New Year's Day 2022 fell on a Saturday
  and the 23x5 grid has no Saturday session. Residual risk: a closure the operator never
  published would be under-reported here.
- **Notice 24-26 keys no row.** "On December 24, 2024, Coinbase Derivatives Crypto Futures markets
  closed early at 14:30 CT due to a technical issue." That is an unplanned venue incident, not a
  published holiday schedule, and it names the crypto tier; trade date 2024-12-24 takes its row
  from notice 24-23, the published Christmas schedule.
- **Notice 24-27 keys no row.** It states that the 2025-01-09 session (a National Day of Mourning)
  "will observe a normal trading day", so 2025-01-09 is an audited-normal date.
- **The half-day rows under-report the later-closing groups.** See the `## Holidays` preamble: the
  venue row carries the earliest instant on 2021-11-26, 2023-11-24, 2024-12-24, 2025-11-28 and
  2025-12-24, five of the six early closes. The sixth, 2024-11-29, is not a disagreement: all
  three groups printed 13:45 CT, so nothing is under-reported there. A caller trading a group that prints a later close on those dates
  should use its own key; no key is claimed for the 24x7 crypto tier or the 24x5 equity-index PSF
  group.
- **Trade date 2021-09-03 is carried as audited normal.** Notice 21-04 says markets are "closed
  from 16:00 CT Friday, September 2nd", but 2 September 2021 was a Thursday and the Friday before
  Labor Day was the 3rd; the notice's subject ("Labor Day Observed Monday, September 6th, 2021"),
  its reopening sentence and the identical construction in notice 21-03 ("closed from 16:00 CT
  Friday, July 2nd") all place the closure on the Monday. The operator's date is a typo; the
  affected trade date is 2021-09-06, which ships `closed`.
- **The 2026-09-08 onward notices are not retrieved here.** They belong to the published-future
  refresh, not to this window; the last row this table ships is 2026-09-07.
"""

target = ROOT / "docs" / "evidence" / "coinbase_derivatives.md"
text = target.read_text()
head, rest = text.split("## Holidays", 1)
_, tail = rest.split("\n## Sources", 1)
target.write_text(head + section + "\n## Sources" + tail)
print(f"rewrote the Holidays section: {len(rows)} rows, {len(doc_lines)} documents")
