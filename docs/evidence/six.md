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

## Sources

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
- **Dormant identity.** Reviewed on demand; gaps are recorded here rather than as issues.
