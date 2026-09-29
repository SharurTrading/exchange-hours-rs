<!-- SPDX-License-Identifier: MIT-0 -->

# `nzx` — evidence

- **Kind:** `Exchange`
- **Owner module:** [`nzx.rs`](../../src/calendar/schedules/equities/apac/nzx.rs)
- **Source sets:** [`APAC-NZX`](../schedules/sources.md#apac-nzx)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

Main Board; deterministic edges for randomized auctions.

## Revision rows

- 2020-04-06 — T1 — NZX announcement 350919 — pre-open start moves 09:00 → 08:30.

## Sources

Row review: 2026-08-22 (UTC) is the date the ledger row was last reviewed as a
whole. Per-source retrieval dates were not recorded before the 2026-09-12
migration; where a later targeted review, capture or document date is recorded
beside a source below, that date governs for that source, and dates are added
as each source is re-verified.

- <https://www.nzx.com/learning/help-reference/trading-hours> — NZX trading hours.
- <https://www.nzx.com/learning/issuer-participant-resources/nzx-trading/anatomy-of-a-trading-day> — NZX Anatomy of a Trading Day. Of Pre-Open it says: "Orders can be placed, amended, and deleted. No trades execute until the opening auction. Off-market trades may be reported." Off-market reports print, so Pre-Open is tradeable `extended`, not order-entry-only.
- <https://www.nzx.com/announcements/350919> — NZX announcement 350919, the 2020-04-06 pre-open move.
- <https://www.nzx.com/announcements/353837> — NZX announcement 353837, making the initially temporary change indefinite.
- <https://www.nzx.com/learning/help-reference/trading-hours> (this page's own holiday table) — retrieved 2026-09-28 (UTC) as `holidays/raw/equities/nzx/2025-2027/live_nzx_trading_hours_and_holidays.html` (research store), sha256 `92071cd2…`; it is the horizon evidence: its rolling table prints Waitangi Day 2026-02-06 through the Day after New Year's Day 2027-01-04 and nothing past that.

## Holidays

**Coverage:** 2025-01-01..2027-01-04 (inclusive trade dates, the operator's published rolling horizon). Tier: T1 throughout.

NZX prints no consolidated year sheets. The operator's own `NZX Market
Holidays & Trading Hours` page carries one rolling table of roughly the next
thirteen months (columns `Event`, `Date`, `Market Status`), so the rows below
are read from successive captures of that one page: the **2024-12-16** replay
prints Business Day Prior to Christmas Day 2024 through the Day after New
Year's Day 2026 and is the only artifact that prints the 2025-01-01 and
2025-01-02 rows; the **2025-01-23** replay prints Waitangi Day 2025 through
the Day after New Year's Day 2026 and keys the rest of 2025 plus the first two
2026 dates; the **2026-02-03** replay prints Waitangi Day 2026 through the Day
after New Year's Day 2027 and keys 2026 from Waitangi onward. The live page
(retrieved 2026-09-28) prints the identical 2026-02-06..2027-01-04 rows and is
the horizon evidence. The 2025 rows the 2025-01-23 replay and the 2024-12-16
replay print in common are identical cell for cell, so the boundary between
the two documents is corroborated on both sides. The wayback replays are
verbatim captures of the operator's own page (`id_` original bytes).

NZX has published nothing past 2027-01-04: verified 2026-09-28 (UTC), the
live table's last row is the Day after New Year's Day, Monday 4 January 2027.
Nothing is withheld by this crate — there is no later NZX schedule yet.
**Closing condition:** NZX's next table refresh, at which point the window
extends. Re-checked monthly per LAW-WATCH.

The `Market Status` column states `Closed` for a full closure and
`Abbreviated Trading*` for the two pre-holiday business days each year (the
footnote: `*Note not applicable to SGX-NZX Dairy Derivatives market` — the
abbreviated grid applies to the NZX Main Board this crate serves). Where
applicable, public holidays will be "mondayised" (the page's own preamble),
which is why ANZAC Day 2026 and Boxing Day 2026 print on Mondays.

### 2025

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2025-01-01 | closed | `New Year's Day` — `Wednesday, 1 January 2025` — `Closed` | `NZX-TH-2024-12-16` | T1 | NZX event date printed verbatim; no Main Board session belongs to the date |
| 2025-01-02 | closed | `Day after New Year's Day` — `Thursday, 2 January 2025` — `Closed` | `NZX-TH-2024-12-16` | T1 | NZX event date printed verbatim |
| 2025-02-06 | closed | `Waitangi Day` — `Thursday, 6 February 2025` — `Closed` | `NZX-TH-2025-01-23` | T1 | NZX event date printed verbatim |
| 2025-04-18 | closed | `Good Friday` — `Friday, 18 April 2025` — `Closed` | `NZX-TH-2025-01-23` | T1 | NZX event date printed verbatim |
| 2025-04-21 | closed | `Easter Monday` — `Monday, 21 April 2025` — `Closed` | `NZX-TH-2025-01-23` | T1 | NZX event date printed verbatim |
| 2025-04-25 | closed | `ANZAC Day` — `Friday, 25 April 2025` — `Closed` | `NZX-TH-2025-01-23` | T1 | NZX event date printed verbatim |
| 2025-06-02 | closed | `King's Birthday` — `Monday, 2 June 2025` — `Closed` | `NZX-TH-2025-01-23` | T1 | NZX event date printed verbatim |
| 2025-06-20 | closed | `Matariki` — `Friday, 20 June 2025` — `Closed` | `NZX-TH-2025-01-23` | T1 | NZX event date printed verbatim |
| 2025-10-27 | closed | `Labour Day` — `Monday, 27 October 2025` — `Closed` | `NZX-TH-2025-01-23` | T1 | NZX event date printed verbatim |
| 2025-12-24 | replacement blocks | `Business Day Prior to Christmas Day` — `Wednesday, 24 December 2025` — `Abbreviated Trading*`; abbreviated grid: Normal Trading `10:00am - 12:45pm`, Pre-Close `12:45pm - 1:00pm`, Adjust `1:00pm - 1:30pm` | `NZX-TH-2025-01-23` | T1 | see the interpretive step below — the row restates the operator's own abbreviated grid as one replacement day |
| 2025-12-25 | closed | `Christmas Day` — `Thursday, 25 December 2025` — `Closed` | `NZX-TH-2025-01-23` | T1 | NZX event date printed verbatim |
| 2025-12-26 | closed | `Boxing Day` — `Friday, 26 December 2025` — `Closed` | `NZX-TH-2025-01-23` | T1 | NZX event date printed verbatim |
| 2025-12-31 | replacement blocks | `Business Day Prior to New Year's Day` — `Wednesday, 31 December 2025` — `Abbreviated Trading*`; same abbreviated grid | `NZX-TH-2025-01-23` | T1 | same reading as 2025-12-24 |

### 2026

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2026-01-01 | closed | `New Year's Day` — `Thursday, 1 January 2026` — `Closed` | `NZX-TH-2025-01-23` | T1 | NZX event date printed verbatim |
| 2026-01-02 | closed | `Day after New Year's Day` — `Friday, 2 January 2026` — `Closed` | `NZX-TH-2025-01-23` | T1 | NZX event date printed verbatim |
| 2026-02-06 | closed | `Waitangi Day` — `Friday, 6 February 2026` — `Closed` | `NZX-TH-2026-02-03` | T1 | NZX event date printed verbatim |
| 2026-04-03 | closed | `Good Friday` — `Friday, 3 April 2026` — `Closed` | `NZX-TH-2026-02-03` | T1 | NZX event date printed verbatim |
| 2026-04-06 | closed | `Easter Monday` — `Monday, 6 April 2026` — `Closed` | `NZX-TH-2026-02-03` | T1 | NZX event date printed verbatim |
| 2026-04-27 | closed | `ANZAC Day` — `Monday, 27 April 2026` — `Closed` | `NZX-TH-2026-02-03` | T1 | the sheet mondayises the Saturday to its own printed Monday date |
| 2026-06-01 | closed | `King's Birthday` — `Monday, 1 June 2026` — `Closed` | `NZX-TH-2026-02-03` | T1 | NZX event date printed verbatim |
| 2026-07-10 | closed | `Matariki` — `Friday, 10 July 2026` — `Closed` | `NZX-TH-2026-02-03` | T1 | NZX event date printed verbatim |
| 2026-10-26 | closed | `Labour Day` — `Monday, 26 October 2026` — `Closed` | `NZX-TH-2026-02-03` | T1 | NZX event date printed verbatim |
| 2026-12-24 | replacement blocks | `Business Day Prior to Christmas Day` — `Thursday, 24 December 2026` — `Abbreviated Trading*` | `NZX-TH-2026-02-03` | T1 | same reading as 2025-12-24 |
| 2026-12-25 | closed | `Christmas Day` — `Friday, 25 December 2026` — `Closed` | `NZX-TH-2026-02-03` | T1 | NZX event date printed verbatim |
| 2026-12-28 | closed | `Boxing Day` — `Monday, 28 December 2026` — `Closed` | `NZX-TH-2026-02-03` | T1 | the sheet mondayises the Saturday to its own printed Monday date |
| 2026-12-31 | replacement blocks | `Business Day Prior to New Year's Day` — `Thursday, 31 December 2026` — `Abbreviated Trading*` | `NZX-TH-2026-02-03` | T1 | same reading as 2025-12-24 |

### 2027

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2027-01-01 | closed | `New Year's Day` — `Friday, 1 January 2027` — `Closed` | `NZX-TH-2026-02-03` | T1 | NZX event date printed verbatim |
| 2027-01-04 | closed | `Day after New Year's Day` — `Monday, 4 January 2027` — `Closed` | `NZX-TH-2026-02-03` | T1 | NZX event date printed verbatim; the operator's published horizon |

**Interpretive step — the abbreviated-trading days are replacement days, not
scalar early closes.** The operator's own abbreviated column keeps a tradeable
closing auction after the shortened Normal Trading window: Pre-Close runs
12:45pm-1:00pm and the closing uncross randomises within 30 seconds either
side of 1:00pm, exactly as it does around 5:00pm on a full day (the same
anatomy-of-a-trading-day statement the normal-week profile cites for the
±30 second envelope). An `EarlyClose` clip cannot state that day: clipped at
12:45 it deletes the auction prints (an executable window the operator
keeps open), and clipped at 13:00:30 it drags the order-entry-only Pre-Close
queue inside `is_open`, which the charter's order-entry rule forbids. Each
abbreviated day therefore ships as a replacement block set restating the
operator's grid block for block — `extended` 08:30-10:00 (Pre-open, tradeable:
off-market reports print), `regular` 10:00-12:45 (Normal Trading),
`order_entry` 12:45-12:59:30 (Pre-Close, sliced at the earliest uncross edge),
`extended` 12:59:30-13:00:30 (the closing uncross envelope) — with Enquiry and
Adjust excluded exactly as on a full day. The block instants are the sheet's
own phase boundaries; only the ±30 second uncross envelope is the operator's
randomisation statement already carried by the normal-week profile.

**No 2027 rows past the horizon.** Waitangi Day 2027 (6 February) and every
later 2027 date are outside the operator's published table, so the window
ends at 2027-01-04 and the inventory records the forward gap. Dates inside
the window with no row are audited normal: the operator's page names every
holiday it observes, and 2026's table (Waitangi 2026 through the horizon)
names thirteen — all thirteen shipped, none withheld.

### Documents

| Document | Window | Replay or service URL | Capture or retrieval, UTC | Tier | sha256 |
|---|---|---|---|---|---|
| `NZX-TH-2024-12-16` | 2025-01-01 .. 2026-01-02 | <https://web.archive.org/web/20241216221746id_/https://www.nzx.com/investing/nzx-trading-hours> | Wayback `id_` replay of capture `20241216221746`, retrieved 2026-09-28 01:20 UTC | T1 | `27ead98734cede1f2b0e21d3132b20b5a4b6ea5ebd218b4fb4d17fe9394aa675` |
| `NZX-TH-2025-01-23` | 2025-01-01 .. 2026-01-02 | <https://web.archive.org/web/20250123031559id_/https://www.nzx.com/investing/nzx-trading-hours> | Wayback `id_` replay of capture `20250123031559`, retrieved 2026-09-28 01:19 UTC | T1 | `982f231b435dddba2473692163a2bcb60f47f89c1005a41d1745eb520c6e715a` |
| `NZX-TH-2026-02-03` | 2025-01-01 .. 2027-01-04 | <https://web.archive.org/web/20260203200136id_/https://new.nzx.com/investing/nzx-trading-hours> | Wayback `id_` replay of capture `20260203200136`, retrieved 2026-09-28 01:19 UTC | T1 | `0aa0508dadd8c30aeb914bd09f6e7db96afe84533a18350e4145f7b5b8dd6215` |
| `NZX-TH-LIVE-2026-09-28` | 2025-01-01 .. 2027-01-04 | <https://www.nzx.com/learning/help-reference/trading-hours> | retrieved 2026-09-28 01:10 UTC | T1 | `92071cd2c1186012fe977fc59af5f4b27de50a435a11b3298a078aa715391146` |

`NZX-TH-LIVE-2026-09-28` is no row's document: it is the horizon evidence, and
its table is cell-for-cell identical to `NZX-TH-2026-02-03`'s over the rows
they share. The store's `holidays/raw/equities/nzx/2025-2027/` also holds the
2025-11-16 replay (`59c3798c…`), which still prints the 2025 table and
corroborates the 2025/2026 boundary documents without keying any row.

## Gaps and residual risks

- **horizon carried below the first dated row** — the pre-2020 baseline rests only on NZX announcement 350919, whose publication day is not recorded in the repository (an NZX announcement number is not a date). The ledger horizon is therefore 2020-04-06, the first day at which this row's state is sourced, with everything below it carried. Closing condition: read the announcement's own publication date, or find a dated pre-2020 NZX trading-hours page; either would move the horizon earlier.
- The closing uncross is randomised within 30 seconds either side of 17:00, so the tradeable window runs to 17:00:30; stopping at 17:00 would drop the half of the randomisation in which the official closing print most often occurs.
- The only order-entry-only phase is Pre-Close 16:45–16:59:30, which neither matches nor accepts reports. The slice stops 30 seconds short of the nominal 17:00 boundary so the randomized uncross stays inside the tradeable window.
- Enquiry and Adjust do not accept automatically matched orders and are excluded from the envelope (AGENTS.md, *Cash-equity venue envelope*).
