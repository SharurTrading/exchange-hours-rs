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

## Gaps and residual risks

- **horizon carried below the first dated row** — the pre-SR15 baseline rests only on the SR15 marked procedure amendments, whose publication day is not recorded in the repository (the notice index places it in April 2025). The ledger horizon is therefore 2025-06-23, the first day at which this row's state is sourced, with everything below it carried. Closing condition: read the marked amendments' own publication date, or find an earlier dated ASX procedure edition stating the staggered-open table; either would move the horizon earlier.
- **The opening edge, stated against what the module encodes.** ASX's cash-market timetable prints *nominal* boundaries: Opening Single Price Auction 09:59:00–09:59:45, then Open (Normal Trading) 09:59:45–16:00:00. `asx.rs` does not encode 09:59:45 as the start of `regular`. `ASX_EXTENDED_CURRENT` carries one rule over the whole opening minute, 09:59:00–10:00:00, and `ASX_REGULAR` runs 10:00:00–16:00:00, so the crate reports the market open from 09:59:00 — the auction matches, so a price can print there — and defers *continuous* trading to 10:00:00, the latest instant at which it can have begun. That is the conservative envelope AGENTS.md's *Exchange-level boundaries, not per-security auction outcomes* calls for: the uncross is randomised per security around the nominal 09:59:45 handoff, so naming any second inside 09:59:45–10:00:00 as the continuous-trading start would imply ticker-level uncross timing the exchange does not publish. Nothing is under-reported as closed by this choice; only the `regular`/`extended` split inside that minute is conservative.
- Both single-price auctions match, and in Post Close "ASX matches orders at the CSPA price", so the opening auction, the CSPA and Post Close are all tradeable `extended`: 09:59:00–10:00:00 on the open side and 16:10:00–16:21:30 on the close side, the latter merging CSPA 16:10–16:11 with Post Close 16:11–16:21:30 into one rule.
- Pre-open is `extended`, not `order_entry`: ASX Trade does not match in it, but overnight and overseas trades report until 09:45 and other allowable trades may be reported under the Operating Rules, so a price can print.
- The only order-entry-only window is Pre-CSPA 16:00–16:10, in which continuous matching ceases and only entry and amendment are accepted.
- **Pre-SR15 era, same convention.** `ASX_EXTENDED_PRE_2025_06_23` spans 09:59:45–10:09:15 — Group 1's nominal 10:00 open less its ±15-second randomization, through Group 5's nominal 10:09 plus the same — while `ASX_REGULAR` is shared with the current era and still starts at 10:00:00. The envelope therefore covers every group's possible open and the `regular` edge names Group 1's nominal transition, not any group's realised one.
