<!-- SPDX-License-Identifier: MIT-0 -->

# `tadawul` — evidence

- **Kind:** `Exchange`
- **Owner module:** [`tadawul.rs`](../../src/calendar/schedules/equities/africa_middle_east/tadawul.rs)
- **Source sets:** [`MIDEAST-TADAWUL`](../schedules/sources.md#mideast-tadawul)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

Main Market including sourced temporary 2020 regime. The pre-2016 opening-auction order window is omitted: no dated primary source states it for the 11:00-open eras.

## Revision rows

- 2013-06-29 — T3 — SPA news 7e453de27d — the trading week moves from Saturday–Wednesday to Sunday–Thursday, hours unchanged at 11:00–15:30.
- 2016-04-03 — T3 — SPA news 1484000 — hours move to 10:00–15:00.
- 2018-05-27 — T1 — Tadawul Statistical Report H1 2018 — a closing auction is added, 15:00–15:10.
- 2019-05-12 — T1 — Tadawul Statistical Report 2019 — trade at last is added, extending the close-side envelope to 15:20.
- 2020-03-26 — T1 — Saudi Exchange issuer news 6262 — temporary shortened hours: continuous 10:00–13:00 with the close-side envelope to 13:20.
- 2020-05-31 — T1 — Saudi Exchange resumption notice — normal trading hours resume.

## Sources

Row review: 2026-08-24 (UTC) is the date the ledger row was last reviewed as a
whole. Per-source retrieval dates were not recorded before the 2026-09-12
migration; where a later targeted review, capture or document date is recorded
beside a source below, that date governs for that source, and dates are added
as each source is re-verified.

- <https://www.saudiexchange.sa/wps/portal/saudiexchange/rules-guidance/capital-market-overview/trading-cycle-and-times?locale=en> — Saudi Exchange trading cycle and times: current Main Market phases — opening-auction orders 09:30–10:00, continuous trading 10:00–15:00, closing auction 15:00–15:10, trade at last 15:10–15:20. The table starts "Trading in Equities" at 10:00 and notes only that the market opens on a variable basis within 30 seconds after 10:00. Auction uncrosses can be randomized by up to 30 seconds; the static profile uses the published nominal boundaries.
- <https://www.spa.gov.sa/7e453de27d> — Saudi Press Agency release, the 2013-06-29 workweek change.
- <https://www.spa.gov.sa/1484000?lang=en&newsid=1484000> — Saudi Press Agency release, the 2016-04-03 hours change.
- <https://www.saudiexchange.sa/wps/wcm/connect/24ca438e-86a0-47d0-b8f4-65b4cbfebcdd/Saudi%2BStock%2BExchange%2B-Tadawul-%2CStatistical%2BReport%2B%E2%80%93%2BFirst%2BHalf%2B2018%2B-%2BUpdated.pdf> — Tadawul Statistical Report, first half 2018: the closing auction from 2018-05-27.
- <https://www.saudiexchange.sa/wps/wcm/connect/4657c15f-ef37-45c8-8423-09e2a5055ab7/Saudi%2BStock%2BExchange%2B%28Tadawul%29%2CStatistical%2BReport%2B%E2%80%93%2B%2B2019-%2BEn.pdf> — Tadawul Statistical Report 2019: trade at last from 2019-05-12.
- <https://www.saudiexchange.sa/wps/portal/saudiexchange/newsandreports/issuer-news/news-detail-wcm/?locale=en&newsId=6262> — Saudi Exchange issuer news 6262: temporary shortened hours applied 2020-03-26.
- <https://www.saudiexchange.sa/wps/portal/saudiexchange/newsandreports/issuer-news/news-detail-wcm/saudiexchangecontent/issuernews/issuernewsdetails/saudiexchange-announces-resumption-of-normal-trading-hours?locale=en> — Saudi Exchange resumption notice: normal trading hours resume from 2020-05-31.

## Gaps and residual risks

- **Raised in review of the ledger-reshape PR (#87), 2026-09-12 — the 2013-06-29 and 2016-04-03 rows are dated by T3 artifacts.** Both revision rows carry `T3` on their own lines and rest on Saudi Press Agency releases (SPA news 7e453de27d and SPA news 1484000). Under LAW-PRIMARY-SOURCES a dated change needs an unconditional day stated by the operator, and T3 may date a change **only** when it mirrors an operator document verbatim; nothing in the record shows either release reproduces a Saudi Exchange or CMA document verbatim, so as recorded these two rows are not admissible and the bullet above understates that as a tier note rather than a defect. The reshape PR moved this text out of the owner module and changed no schedule rule, revision row, profile or routing; both rows are served exactly as before. Closing condition: retrieve the underlying Tadawul or CMA announcement behind each date, or establish that the SPA text is a verbatim reprint of it — either promotes both rows to T1. If neither holds, the rows must be withdrawn and the two grids served as an undated intersection instead. Dormant identity, so recorded here rather than opened as an issue (LAW-FOLLOW-UPS-ARE-ISSUES).
- **executable, pre-2016 opening-auction window.** The pre-2016 grids carry no pre-opening phase. Today's trading-cycle table documents the 09:30–10:00 opening auction, but no dated primary source states the opening-auction order window for the 11:00-open eras; a 10:00–11:00 window would be an inference from the later auction's shape, so under LAW-PRIMARY-SOURCES it is omitted and reads closed. The 2016–2018 era likewise carries no order-entry schedule: the 09:30 queue is evidenced only by the current trading-cycle page, which states nothing about that era. Closing condition: a dated Saudi Exchange or CMA artifact stating the queue for its era. Neither old grid had any close-side phase, so their extended slices are empty too.
- **Tier of the 2013 and 2016 rows.** Both rest on Saudi Press Agency releases. SPA is the Kingdom's state news agency and carries official announcements, but nothing in the record establishes that either release reproduces an exchange or CMA document verbatim, so they are recorded here at T3 rather than T1. Under LAW-PRIMARY-SOURCES a dated change needs an unconditional day stated by the operator, and T3 may date a change only when it mirrors an operator document verbatim. Closing condition: retrieve the underlying Tadawul or CMA announcement for each date, or confirm that the SPA text is a verbatim reprint of it. If neither holds, both rows are T4-keyed and must be rebuilt or withdrawn.
- **Horizon carried below the first dated row.** The pre-2013 baseline — Saturday–Wednesday, 11:00–15:30 — cites no artifact of its own, and neither SPA citation carries a readable publication date, so no day can be read off the record from which the baseline is carried. The ledger horizon is therefore 2013-06-29, the first day at which this row's state is sourced, with everything below it carried. Closing condition: a dated Tadawul or CMA artifact printing the Saturday–Wednesday 11:00–15:30 grid, or a recorded retrieval and publication date for SPA release 7e453de27d; either would move the horizon earlier.
- **Interpretive step, order-entry classification.** The current 09:30–10:00 window is `order_entry`: it collects, amends and cancels opening-auction orders without any of them matching, and the first print is the 10:00 uncross. The closing auction and the trade-at-last tail both print — trade at last executes at the closing auction price — so both stay `extended`.
- **Source set has no monitoring feed.** `MIDEAST-TADAWUL` records that no consolidated schedule-notice feed is indexed; review means reopening the trading cycle and times page and the individual reports and notices above.
- **Dormant identity.** Reviewed on demand; gaps are recorded here rather than as issues.
