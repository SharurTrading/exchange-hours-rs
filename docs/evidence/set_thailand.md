<!-- SPDX-License-Identifier: MIT-0 -->

# `set_thailand` — evidence

- **Kind:** `Exchange`
- **Owner module:** [`set.rs`](../../src/calendar/schedules/equities/apac/set.rs)
- **Source sets:** [`APAC-SET`](../schedules/sources.md#apac-set)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

Venue union retains sourced auction edges and, from 2025-05-06, eligible Europe/US DR continuous trading through lunch plus 19:00→03:00 night trading. A launch-day bridge prevents a pre-launch post-midnight phase; night trading carries its prior opening date through the 03:00 final close.

## Revision rows

- 2024-03-25 — T1 — SET notification 86864800 — the ordinary afternoon session moves 30 minutes earlier, opening 14:00 instead of 14:30.
- 2025-05-06 — T1 — SET notification 95921400 — night-session launch day; a one-day transition profile prevents the generic Tuesday 02:45–03:00 tail from appearing before launch.
- 2025-05-07 — T1 — SET notification 95921400 — the complete recurring week, with DR continuous trading through lunch and 19:00→03:00 night trading.

## Sources

Row review: 2026-08-22 (UTC) is the date the ledger row was last reviewed as a
whole. Per-source retrieval dates were not recorded before the 2026-09-12
migration; where a later targeted review, capture or document date is recorded
beside a source below, that date governs for that source, and dates are added
as each source is re-verified.

- <https://www.set.or.th/en/market/information/trading-procedure/trading-hours> — SET published trading procedure. Each pre-open phase has a randomized end T: Pre-open I runs 09:30–T1 (T1 random 09:55–10:00), Pre-open II 13:30–T2 (T2 random 13:55–14:00) and the night Pre-open 18:45–T4 (T4 random 18:55–19:00); each is listed as order entry only, an auction order-collection phase with no matching and no trade reports. Pre-close 16:30–T3 (T3 random 16:35–16:40) and Off-hour T3–17:00 both admit Trade Reports.
- <https://www.set.or.th/en/market/news-and-alert/newsdetails?id=95921400&symbol=SET> — SET notification 95921400, the 2025-05-06 night-session and continuous-DR launch.
- <https://www.set.or.th/en/market/news-and-alert/newsdetails?id=86864800&symbol=SET> — SET notification 86864800, the 2024-03-25 afternoon-session move.

## Gaps and residual risks

- **horizon carried below the first dated row** — the pre-2024-03-25 baseline rests only on SET notification 86864800 and the living trading-procedure page; neither is dated in the repository below 2024, and no artifact attests the grid at or near the January-2010 floor. The ledger horizon is therefore 2024-03-25, the first day at which this row's state is sourced, with everything below it carried. Closing condition: a dated SET trading-procedure edition or notification covering the pre-2024 afternoon session, which would move the horizon earlier.
- The night Pre-close 02:45–T5 (T5 random 02:55–03:00) is left tradeable: the closing uncross falls inside it and the crate has no primary source ruling out trade reports in the night pre-close.
- Not every listed security is eligible for every phase; the row is a venue-availability envelope, and the consumer's map decides which products may use it.
- The next-local-day tail belongs to the prior opening day's trade date, which is a deliberate family convention rather than a generic rule.
