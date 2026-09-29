<!-- SPDX-License-Identifier: MIT-0 -->

# `asx` — evidence

- **Kind:** `Exchange`
- **Owner module:** [`asx.rs`](../../src/calendar/schedules/equities/apac/asx.rs)
- **Source sets:** [`APAC-ASX`](../schedules/sources.md#apac-asx)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

Cash-market envelope; the pre-2025 staggered opening preserves both the earliest regular edge and latest randomized auction edge.

## Revision rows

- 2025-06-23 — T1 — ASX SR15 notice 0473.25.05 — Service Release 15: a single Opening Single Price Auction, which ASX's timetable prints at 09:59:00–09:59:45 with Normal Trading nominally from 09:59:45, replaces the five staggered symbol-group opens, and Post Close 16:11:00–16:21:30 is added. `ASX_PROFILE_CURRENT` encodes the resulting opening minute as one `extended` rule 09:59:00–10:00:00 and starts `regular` at 10:00:00; see the opening-edge note under Gaps.

## Sources

Row review: 2026-08-22 (UTC) is the date the ledger row was last reviewed as a
whole. Per-source retrieval dates were not recorded before the 2026-09-12
migration; where a later targeted review, capture or document date is recorded
beside a source below, that date governs for that source, and dates are added
as each source is re-verified.

- <https://www.asx.com.au/markets/market-resources/trading-hours-calendar/cash-market-trading-hours> — ASX cash-market trading hours, the current phase timetable: Pre-open 07:00:00–09:59:00, Opening Single Price Auction 09:59:00–09:59:45, Open (Normal Trading) 09:59:45–16:00:00, Pre-CSPA 16:00:00–16:10:00, Closing Single Price Auction 16:10:00–16:11:00, Post Close 16:11:00–16:21:30.
- <https://www.asxonline.com/public/notices/2025/may/0473.25.05.html> — ASX notice 0473.25.05, the Service Release 15 notice dating the change to 2025-06-23.
- <https://www.asxonline.com/content/dam/asxonline/public/notices/2025/april/asx-sr15asx-operating-rule-procedure-amendments.pdf> — SR15 marked operating-rule procedure amendments, which carry the pre-SR15 text: five symbol groups opening at nominal times from 10:00 through 10:09, each randomized by ±15 seconds, with the CSPA ending at 16:12.
- ASX Operating Rules Procedures Appendix 4013 — the operator's phase definitions
  for the cash market. No URL for this appendix is recorded anywhere in this
  repository — neither the owner module nor the `APAC-ASX` entry in
  [sources.md](../schedules/sources.md#apac-asx) carries one — so the appendix
  cannot be re-opened from a link here; re-verification goes through the
  `APAC-ASX` operating-rules entry point in
  [sources.md](../schedules/sources.md#apac-asx).
- <https://www.asx.com.au/markets/market-resources/trading-hours-calendar/cash-market-trading-hours/trading-calendar> — the operator's own annual trading calendar (this page's own holiday sheets), retrieved 2026-09-28 (UTC) as `holidays/raw/equities/asx/2025-2027/live_asx_cash_market_trading_calendar.html` (research store), sha256 `adb2344c…`; the live page renders the 2026 and 2027 sheets.

## Holidays

**Coverage:** 2025-01-01..2027-12-31 (inclusive trade dates). Tier: T1 throughout.

ASX server-renders one sheet per year on its own `Trading calendar` page under
cash-market trading hours (columns `Public holiday`, `Dates for <year>`,
`Trading day`, settlement activity, `Business day`). The live page retrieved
2026-09-28 (UTC) renders the **2026** and **2027** sheets; the **2025** sheet
comes from the page's own 2025-04-16 wayback replay (`id_` original bytes),
which renders `2025 Trading calendar` alone. All three years ASX publishes are
therefore audited and the window reaches 2027-12-31; there is no forward gap
to record inside the operator's publication horizon.

The sheet states each closure in its own `CLOSED` cell and each half day as
`CLOSE EARLY` with numbered footnotes that state the instant themselves:
`Normal trading ceases at 14:10 (Sydney time)`. The row therefore clips the
envelope at 14:10 — the Pre-CSPA, CSPA and Post Close blocks start after that
instant and are gone with it — and nothing about the reading is inferred: the
footnote is the sheet's own words. ANZAC Day 2027 is the one entry the module
deliberately does not row: the 2027 sheet prints `OPEN` against Monday
26 April with the footnote `Substitute for Sunday 25 April`, so that Monday is
an audited-normal trading day.

### 2025

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2025-01-01 | closed | `New Year's Day` — `Wednesday 1 January` — `CLOSED` | `ASX-CAL-2025` | T1 | ASX event date printed verbatim; `A Trading Day means ASX Trade is open for trading`, and no session belongs to the date |
| 2025-01-27 | closed | `Australia Day` — `Monday 27 January` — `CLOSED` | `ASX-CAL-2025` | T1 | ASX event date printed verbatim (the sheet's own observed date for Sunday 26 January) |
| 2025-04-18 | closed | `Good Friday` — `Friday 18 April` — `CLOSED` | `ASX-CAL-2025` | T1 | ASX event date printed verbatim |
| 2025-04-21 | closed | `Easter Monday` — `Monday 21 April` — `CLOSED` | `ASX-CAL-2025` | T1 | ASX event date printed verbatim |
| 2025-04-25 | closed | `ANZAC Day` — `Friday 25 April` — `CLOSED` | `ASX-CAL-2025` | T1 | ASX event date printed verbatim |
| 2025-06-09 | closed | `King's Birthday` — `Monday 9 June` — `CLOSED` | `ASX-CAL-2025` | T1 | ASX event date printed verbatim |
| 2025-12-24 | early close | `Last Business day before Christmas Day` — `Wednesday 24 December` — `CLOSE EARLY` `[3]` — footnote: `Normal trading ceases at 14:10 (Sydney time)` | `ASX-CAL-2025` | T1 | ASX trade date named verbatim; the row clips the envelope at the sheet's own 14:10 |
| 2025-12-25 | closed | `Christmas Day` — `Thursday 25 December` — `CLOSED` | `ASX-CAL-2025` | T1 | ASX event date printed verbatim |
| 2025-12-26 | closed | `Boxing Day` — `Friday 26 December` — `CLOSED` | `ASX-CAL-2025` | T1 | ASX event date printed verbatim |
| 2025-12-31 | early close | `Last Business day of the Year` — `Wednesday 31 December` — `CLOSE EARLY` `[4]` — 14:10 (Sydney time) | `ASX-CAL-2025` | T1 | same reading as 2025-12-24 |

### 2026

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2026-01-01 | closed | `New Year's Day` — `Thursday 1 January` — `CLOSED` | `ASX-CAL-LIVE` | T1 | ASX event date printed verbatim |
| 2026-01-26 | closed | `Australia Day` — `Monday 26 January` — `CLOSED` | `ASX-CAL-LIVE` | T1 | ASX event date printed verbatim |
| 2026-04-03 | closed | `Good Friday` — `Friday 3 April` — `CLOSED` | `ASX-CAL-LIVE` | T1 | ASX event date printed verbatim |
| 2026-04-06 | closed | `Easter Monday` — `Monday 6 April` — `CLOSED` | `ASX-CAL-LIVE` | T1 | ASX event date printed verbatim |
| 2026-04-25 | closed | `ANZAC Day` — `Saturday 25 April` — `CLOSED`, no substitute footnote | `ASX-CAL-LIVE` | T1 | the sheet prints the Saturday closure and states no substitution; the row restates a closure the Mon-Fri normal week already makes, so it changes no answer — recorded so the transcription matches the sheet line for line |
| 2026-06-08 | closed | `King's Birthday` — `Monday 8 June` — `CLOSED` | `ASX-CAL-LIVE` | T1 | ASX event date printed verbatim |
| 2026-12-24 | early close | `Last Business day before Christmas Day` — `Thursday 24 December` — `CLOSE EARLY` `[3]` — 14:10 (Sydney time) | `ASX-CAL-LIVE` | T1 | ASX trade date named verbatim |
| 2026-12-25 | closed | `Christmas Day` — `Friday 25 December` — `CLOSED` | `ASX-CAL-LIVE` | T1 | ASX event date printed verbatim |
| 2026-12-28 | closed | `Boxing Day` — `Monday 28 December` — `CLOSED` | `ASX-CAL-LIVE` | T1 | the sheet's own date for the Saturday 26 December holiday |
| 2026-12-31 | early close | `Last Business day of the Year` — `Thursday 31 December` — `CLOSE EARLY` `[4]` — 14:10 (Sydney time) | `ASX-CAL-LIVE` | T1 | same reading as 2026-12-24 |

### 2027

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2027-01-01 | closed | `New Year's Day` — `Friday 1 January` — `CLOSED` | `ASX-CAL-LIVE` | T1 | ASX event date printed verbatim |
| 2027-01-26 | closed | `Australia Day` — `Tuesday 26 January` — `CLOSED` | `ASX-CAL-LIVE` | T1 | ASX event date printed verbatim |
| 2027-03-26 | closed | `Good Friday` — `Friday 26 March` — `CLOSED` | `ASX-CAL-LIVE` | T1 | ASX event date printed verbatim |
| 2027-03-29 | closed | `Easter Monday` — `Monday 29 March` — `CLOSED` | `ASX-CAL-LIVE` | T1 | ASX event date printed verbatim |
| 2027-06-14 | closed | `King's Birthday` — `Monday 14 June` — `CLOSED` | `ASX-CAL-LIVE` | T1 | ASX event date printed verbatim |
| 2027-12-24 | early close | `Last Business day before Christmas Day` — `Friday 24 December` — `CLOSE EARLY` `[4]` — 14:10 (Sydney time) | `ASX-CAL-LIVE` | T1 | ASX trade date named verbatim |
| 2027-12-27 | closed | `Christmas Day` — `Monday 27 December` — `CLOSED` `[5]` — `Substitute for Saturday 25 December` | `ASX-CAL-LIVE` | T1 | the sheet's own substitute date, printed in the event row |
| 2027-12-28 | closed | `Boxing Day` — `Tuesday 28 December` — `CLOSED` `[6]` — `Substitute for Sunday 26 December` | `ASX-CAL-LIVE` | T1 | the sheet's own substitute date |
| 2027-12-31 | early close | `Last Business day of the Year` — `Friday 31 December` — `CLOSE EARLY` `[7]` — 14:10 (Sydney time) | `ASX-CAL-LIVE` | T1 | same reading as 2027-12-24 |

**ANZAC Day 2027 ships no row.** The 2027 sheet prints `ANZAC Day` /
`Monday 26 April` / `OPEN` `[3]` / Settlement / Settlement / `YES`, with the
footnote `Substitute for Sunday 25 April`. The sheet states a trading day, so
the Monday is audited normal: recording it as a row would claim a closure the
operator's own bytes deny. No other sheet entry is without a row; ASX's
standing note (`ASX Limited reserves the right to declare additional
Non-Business Days … without notice`) is the operator's own alteration clause
and is covered by the monthly watch.

### Documents

| Document | Window | Replay or service URL | Capture or retrieval, UTC | Tier | sha256 |
|---|---|---|---|---|---|
| `ASX-CAL-2025` | 2025-01-01 .. 2027-12-31 | <https://web.archive.org/web/20250416082951id_/https://www.asx.com.au/markets/market-resources/trading-hours-calendar/cash-market-trading-hours/trading-calendar> | Wayback `id_` replay of capture `20250416082951`, retrieved 2026-09-28 01:51 UTC | T1 | `d24de6d6f6ec1864480de6f2f75cf4a3650b30f6eda8daa354fd1bfa1302d66d` |
| `ASX-CAL-LIVE` | 2025-01-01 .. 2027-12-31 | <https://www.asx.com.au/markets/market-resources/trading-hours-calendar/cash-market-trading-hours/trading-calendar> | retrieved 2026-09-28 01:00 UTC | T1 | `adb2344ca5e13dcbfb8de9b0cf40334c992f4ffb660026b22bca7d21d964cd19` |

`ASX-CAL-2025`'s replayed page renders only the 2025 sheet; its window cell
names the table window it keys rows inside, not the years it prints. The
store's `holidays/raw/equities/asx/2025-2027/` also holds the calendar hub
page (`7ea4e039…`) through which the trading-calendar URL was located.

## Gaps and residual risks

- **horizon carried below the first dated row** — the pre-SR15 baseline rests only on the SR15 marked procedure amendments, whose publication day is not recorded in the repository (the notice index places it in April 2025). The ledger horizon is therefore 2025-06-23, the first day at which this row's state is sourced, with everything below it carried. Closing condition: read the marked amendments' own publication date, or find an earlier dated ASX procedure edition stating the staggered-open table; either would move the horizon earlier.
- **The opening edge, stated against what the module encodes.** ASX's cash-market timetable prints *nominal* boundaries: Opening Single Price Auction 09:59:00–09:59:45, then Open (Normal Trading) 09:59:45–16:00:00. `asx.rs` does not encode 09:59:45 as the start of `regular`. `ASX_EXTENDED_CURRENT` carries one rule over the whole opening minute, 09:59:00–10:00:00, and `ASX_REGULAR` runs 10:00:00–16:00:00, so the crate reports the market open from 09:59:00 — the auction matches, so a price can print there — and defers *continuous* trading to 10:00:00, the latest instant at which it can have begun. That is the conservative envelope AGENTS.md's *Exchange-level boundaries, not per-security auction outcomes* calls for: the uncross is randomised per security around the nominal 09:59:45 handoff, so naming any second inside 09:59:45–10:00:00 as the continuous-trading start would imply ticker-level uncross timing the exchange does not publish. Nothing is under-reported as closed by this choice; only the `regular`/`extended` split inside that minute is conservative.
- Both single-price auctions match, and in Post Close "ASX matches orders at the CSPA price", so the opening auction, the CSPA and Post Close are all tradeable `extended`: 09:59:00–10:00:00 on the open side and 16:10:00–16:21:30 on the close side, the latter merging CSPA 16:10–16:11 with Post Close 16:11–16:21:30 into one rule.
- Pre-open is `extended`, not `order_entry`: ASX Trade does not match in it, but overnight and overseas trades report until 09:45 and other allowable trades may be reported under the Operating Rules, so a price can print.
- The only order-entry-only window is Pre-CSPA 16:00–16:10, in which continuous matching ceases and only entry and amendment are accepted.
- **Pre-SR15 era, same convention.** `ASX_EXTENDED_PRE_2025_06_23` spans 09:59:45–10:09:15 — Group 1's nominal 10:00 open less its ±15-second randomization, through Group 5's nominal 10:09 plus the same — while `ASX_REGULAR` is shared with the current era and still starts at 10:00:00. The envelope therefore covers every group's possible open and the `regular` edge names Group 1's nominal transition, not any group's realised one.
