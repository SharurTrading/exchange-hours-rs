<!-- SPDX-License-Identifier: MIT-0 -->

# `six` — evidence

- **Kind:** `Exchange`
- **Owner module:** [`six.rs`](../../src/calendar/schedules/equities/europe/six.rs)
- **Source sets:** [`EU-SIX`](../schedules/sources.md#eu-six), [`EU-FESE-SECONDARY`](../schedules/sources.md#eu-fese-secondary)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

SIX shares January-2010 phases, including the two-minute randomized opening and closing edges, are operator-sourced; the 2020-06-22 Trading-At-Last launch is date-aware.

## Revision rows

- 2020-06-22 — T1 — SIX SMR8.2 participant readiness — Trading-At-Last added, 17:32–17:40, pushing post-trading back to 17:40.

## Holidays

**Coverage:** 2025-01-01..2027-12-31 (inclusive venue-local trade dates in `Europe/Zurich`; tier T1 throughout).

The rows key on SIX's own `Trading Calendar` PDFs, one per year, each a Trading Guide page whose twelve month grids mark every non-trading day and whose legend reads: light shade `Saturday — Market Closed`, lighter shade `Sunday — Market Closed`, dark cell `Market Holiday — Market Closed`. Each year's rows cite that year's PDF: `SIX-TC-2025` (2025-05-05 capture of the `trading-guides-upcoming` edition), `SIX-TC-2026` and `SIX-TC-2027` (live download-centre editions; the 2027 file is marked `valid as of 1 July 2026`).

The dark cells were resolved from the PDFs' vector fills — the holiday fill is rgb ≈ (0.0, 0.17, 0.37), read cell by cell with the day number found inside the same cell rectangle — and every resolved date was cross-checked against its weekday column in the grid (for example 2026-05-14 lands on the Thursday column of the May block). Holidays that fall on a weekend are not dark-marked — the Saturday/Sunday shading already deletes them — and key no weekday row: the Swiss National Day 2026-08-01 and St. Stephen's Day 2026-12-26 (both Saturdays), and St. Berchtold Day 2027-01-02, Labour Day 2027-05-01, Swiss National Day 2027-08-01, Christmas Day 2027-12-25 and St. Stephen's Day 2027-12-26 (all weekend) are those cases. The calendars print closures only — no half day, no late open and no intraday instant anywhere in the three years — so `Closed` is the only kind the operator's own statement supports.

### 2025

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2025-01-01 | closed | dark `Market Holiday` cell on 1 January (Wednesday column) | `SIX-TC-2025` | T1 | the calendar's own legend: `Market Holiday — Market Closed` |
| 2025-01-02 | closed | dark cell on 2 January (Thursday column) — St. Berchtold Day | `SIX-TC-2025` | T1 | same legend |
| 2025-04-18 | closed | dark cell on 18 April (Friday column) — Good Friday | `SIX-TC-2025` | T1 | same legend |
| 2025-04-21 | closed | dark cell on 21 April (Monday column) — Easter Monday | `SIX-TC-2025` | T1 | same legend |
| 2025-05-01 | closed | dark cell on 1 May (Thursday column) — Labour Day | `SIX-TC-2025` | T1 | same legend |
| 2025-05-29 | closed | dark cell on 29 May (Thursday column) — Ascension Day | `SIX-TC-2025` | T1 | same legend |
| 2025-06-09 | closed | dark cell on 9 June (Monday column) — Whit Monday | `SIX-TC-2025` | T1 | same legend |
| 2025-08-01 | closed | dark cell on 1 August (Friday column) — Swiss National Day | `SIX-TC-2025` | T1 | same legend |
| 2025-12-24 | closed | dark cell on 24 December (Wednesday column) — Christmas Eve | `SIX-TC-2025` | T1 | same legend |
| 2025-12-25 | closed | dark cell on 25 December (Thursday column) — Christmas Day | `SIX-TC-2025` | T1 | same legend |
| 2025-12-26 | closed | dark cell on 26 December (Friday column) — St. Stephen's Day | `SIX-TC-2025` | T1 | same legend |
| 2025-12-31 | closed | dark cell on 31 December (Wednesday column) — New Year's Eve | `SIX-TC-2025` | T1 | same legend |

### 2026

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2026-01-01 | closed | dark cell on 1 January (Thursday column) — New Year's Day | `SIX-TC-2026` | T1 | the calendar's own legend: `Market Holiday — Market Closed` |
| 2026-01-02 | closed | dark cell on 2 January (Friday column) — St. Berchtold Day | `SIX-TC-2026` | T1 | same legend |
| 2026-04-03 | closed | dark cell on 3 April (Friday column) — Good Friday | `SIX-TC-2026` | T1 | same legend |
| 2026-04-06 | closed | dark cell on 6 April (Monday column) — Easter Monday | `SIX-TC-2026` | T1 | same legend |
| 2026-05-01 | closed | dark cell on 1 May (Friday column) — Labour Day | `SIX-TC-2026` | T1 | same legend |
| 2026-05-14 | closed | dark cell on 14 May (Thursday column) — Ascension Day | `SIX-TC-2026` | T1 | same legend |
| 2026-05-25 | closed | dark cell on 25 May (Monday column) — Whit Monday | `SIX-TC-2026` | T1 | same legend; Swiss National Day (Saturday) and St. Stephen's Day (Saturday) 2026 are marked as a weekend, not as holidays |
| 2026-12-24 | closed | dark cell on 24 December (Wednesday column) — Christmas Eve | `SIX-TC-2026` | T1 | same legend |
| 2026-12-25 | closed | dark cell on 25 December (Thursday column) — Christmas Day | `SIX-TC-2026` | T1 | same legend |
| 2026-12-31 | closed | dark cell on 31 December (Thursday column) — New Year's Eve | `SIX-TC-2026` | T1 | same legend |

### 2027

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2027-01-01 | closed | dark cell on 1 January (Friday column) — New Year's Day | `SIX-TC-2027` | T1 | the calendar's own legend: `Market Holiday — Market Closed`; St. Berchtold Day 2027 (Saturday) is marked as a weekend |
| 2027-03-26 | closed | dark cell on 26 March (Friday column) — Good Friday | `SIX-TC-2027` | T1 | same legend |
| 2027-03-29 | closed | dark cell on 29 March (Monday column) — Easter Monday | `SIX-TC-2027` | T1 | same legend |
| 2027-05-06 | closed | dark cell on 6 May (Thursday column) — Ascension Day | `SIX-TC-2027` | T1 | same legend; Labour Day 2027 (Saturday) is marked as a weekend |
| 2027-05-17 | closed | dark cell on 17 May (Monday column) — Whit Monday | `SIX-TC-2027` | T1 | same legend |
| 2027-12-24 | closed | dark cell on 24 December (Friday column) — Christmas Eve | `SIX-TC-2027` | T1 | same legend; Christmas Day (Saturday), St. Stephen's Day (Sunday) and Swiss National Day 2027 (Sunday) are marked as a weekend |
| 2027-12-31 | closed | dark cell on 31 December (Friday column) — New Year's Eve | `SIX-TC-2027` | T1 | same legend |

**Gaps: none inside the window.** The three PDFs print complete closure sets for their years — every weekday the operator marks ships a row, and every other trade date in 2025-2027 is audited normal. No 2028 material is claimed.

### Documents

Every artifact was retrieved on 2026-09-28 UTC and saved under `holidays/raw/equities/six/2025-2027/` in the research store, whose `INDEX.md` carries the same digests.

| Document | Window | Replay or service URL | Capture or retrieval, UTC | Tier | sha256 |
|---|---|---|---|---|---|
| `SIX-TC-2025` | 2025-01-01 .. 2025-12-31 | <https://web.archive.org/web/20250505133302id_/https://www.six-group.com/dam/download/the-swiss-stock-exchange/trading/trading-provisions/regulation/trading-guides-upcoming/trading-calendar-2025.pdf> (capture `20250505133302`) | retrieved 2026-09-28 UTC | T1 | `0729de0a843ee2e22d50271d2bbc6fef8b031a133d392cd700f7db38878b1ed2` |
| `SIX-TC-2026` | 2026-01-01 .. 2026-12-31 | <https://www.six-group.com/dam/download/the-swiss-stock-exchange/trading/trading-provisions/regulation/trading-guides/trading-calendar-2026.pdf> | retrieved 2026-09-28 01:28 UTC | T1 | `70d1b87db3e65d487159f660e9daec2fc68c6cb53385483c0af7bf99591c9390` |
| `SIX-TC-2027` | 2027-01-01 .. 2027-12-31 | <https://www.six-group.com/dam/download/the-swiss-stock-exchange/trading/trading-provisions/regulation/trading-guides/trading-calendar-2027.pdf> ("valid as of 1 July 2026") | retrieved 2026-09-28 01:28 UTC | T1 | `cd2fdca6f0083709bd9100d30b10415b2f0fce0b0b74b54b7e73f901fb318037` |

## Sources

Row review: 2026-08-22 (UTC) is the date the ledger row was last reviewed as a
whole. Per-source retrieval dates were not recorded before the 2026-09-12
migration; where a later targeted review, capture or document date is recorded
beside a source below, that date governs for that source, and dates are added
as each source is re-verified.

- <https://www.six-group.com/en/products-services/the-swiss-stock-exchange/trading/trading-provisions/trading-hours.html> — SIX Group, "Trading hours": the current page confirms the two-minute opening slot.
- <https://www.six-group.com/dam/download/the-swiss-stock-exchange/trading/trading-provisions/regulation/trading-guides/trading-guide.pdf> — SIX Swiss Exchange Trading Guide. Blue Chip Shares: "Trading Hours 09:00 - 17:30 CET / Continuous Trading 09:00 - 17:20 CET / Closing Auction 17:20 - 17:30 CET / Trading-At-Last Start: 17:30 - 17:32 CET End: 17:40 CET". Its segment row is "Blue Chip Shares 06:00 09:00 17:20 17:30 17:30 17:40 22:00". The trading-period overview runs Pre-Opening from 06:00 "until Opening" and permits no immediate-execution time in force in it (Immediate or Cancel and Fill or Kill are "No" for both Pre-Opening and Post Trading).
- <https://www.six-group.com/dam/download/sites/education/preparatory-documentation/trading-module/trading-guide.pdf> — SIX Trading Guide valid from 2018-05-28: the same Blue Chip grid, including the two-minute randomized opening and closing auction windows.
- <https://web.archive.org/web/20081123115341id_/http://www.six-swiss-exchange.com/download/trading/regulation/directives/swx_dir01_en.pdf> — SIX Directive 1, effective 2007-09-07: exchange hours 06:00–22:00, pre-opening from 06:00 until the opening, post-trading from the close through 22:00, and pre-opening and post-trading separated from the trading phases of the exchange day.
- <https://web.archive.org/web/20090824132532id_/http://www.six-swiss-exchange.com:80/download/marketpulse/news/newsboard/product_guides/product_guide_equities_en.pdf> — SIX Equity Market Product Guide valid from 2009-07-22: the exact shares grid — continuous 09:00–17:20, closing auction 17:20–17:30, two-minute randomized opening and closing windows ending at 09:02 and 17:32.
- <https://www.six-group.com/dam/download/the-swiss-stock-exchange/trading/participation/SWXess-maintenance-releases/smr82_participant_readiness.pdf> — SIX SMR8.2 participant readiness: Trading-At-Last launched with SMR8.2 on 2020-06-22, with the added 17:30–17:40 phase.
- <https://www.six-group.com/en/products-services/the-swiss-stock-exchange/trading/download-center.html> — SIX download centre, the monitoring entry point.
- <https://www.fese.eu/app/uploads/2024/07/trading-hours-2025-1.pdf> — FESE 2025 trading-hours table, `EU-FESE-SECONDARY`: corroboration only.

## Gaps and residual risks

- **Scope.** `Exchange::Six` denotes the shares segments (Blue Chip / Mid-/Small-Cap). SIX does not follow the Xetra pattern: the 17:30–17:35 auction belongs to the ETF/ETP/Sponsored Funds segments only, which have no Trading-At-Last, and those segments are out of scope.
- **Interpretive step, randomized opening.** The Trading Guide's 09:00 opening is randomized over two minutes. The deterministic profile keeps the auction/pre-opening classification through 09:01:59 and starts regular trading at the latest possible edge, 09:02. Within that stretch the guide's own phase boundary applies: 06:00 until 09:00 is Pre-Opening (`order_entry`), 09:00–09:02 is the Opening auction (`extended`, because its uncross prints).
- **Interpretive step, order-entry classification.** Pre-Opening and Post Trading are `order_entry`: an At-the-Opening order entered during Pre-Opening only executes in the Opening Auction that follows, and Directive 1 separates both phases from the trading phases of the exchange day.
- **Time zone.** SIX labels its times "CET" year-round; they are local Zurich wall-clock, so `Europe::Zurich` (CET/CEST) is the correct zone, not a fixed offset.
- **Served identity, 2026-09-28 UTC.** The consumer's market clock routes its `SIX` and `SIX_CENTRE` sets to this venue, so the row is **served** and reviewed monthly per LAW-WATCH; the holiday window records **no gaps** — every closure the operator prints ships and no residual is withheld (LAW-SERVICE-TIERS, LAW-FOLLOW-UPS-ARE-ISSUES: nothing to track).
