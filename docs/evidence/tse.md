<!-- SPDX-License-Identifier: MIT-0 -->

# `tse` — evidence

- **Kind:** `Exchange`
- **Owner module:** [`tse.rs`](../../src/calendar/schedules/equities/apac/tse.rs)
- **Source sets:** [`APAC-JPX`](../schedules/sources.md#apac-jpx)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

TSE venue union across arrowhead and ToSTNeT is 08:00–18:00 as of the 2026-08-24 review. Primary pre-scope evidence establishes the 08:00 tail at the January-2010 floor (Working Paper No.3) and through the post-2011 era (the November 2020 Investigation Report states order acceptance began as normal at 08:00); the 2011 phase change and exact 2024-11-05 ToSTNeT close extension are date-aware.

## Revision rows

- 2011-11-21 — T1 — JPX trading-hours transition table — arrowhead morning session extended from 11:00 to 11:30.
- 2024-11-05 — T1 — JPX news release 20241103-01 — upgraded arrowhead and ToSTNeT go live: the closing call runs to 15:30 with continuous matching to 15:25, and ToSTNeT single-stock/basket trading extends from 17:30 to 18:00.

## Sources

Retrieval dates: these sources were last opened on the row's reviewed-on date
(2026-08-24, UTC); per-source retrieval dates were not recorded before the
2026-09-12 migration and are added as each source is re-verified.

- <https://www.jpx.co.jp/english/equities/trading/domestic/01.html> — JPX domestic equities trading hours: 09:00–11:30 and 12:30–15:30 auction-trading sessions, with order acceptance from 08:00 and 12:05.
- <https://www.jpx.co.jp/english/systems/equities-trading/01.html> — JPX equities trading systems; arrowhead continuous matching ends at 15:25 and the final five minutes are the closing call.
- <https://www.jpx.co.jp/english/systems/equities-trading/> — JPX equities trading systems index.
- <https://www.jpx.co.jp/english/equities/trading/tostnet/02.html> — JPX ToSTNeT hours: single-issue and basket trading (ToSTNeT-1) at 08:20–18:00 as of the 2026-08-24 review and 08:20–17:30 before the 2024-11-05 upgrade, which is why the earliest executable edge of the venue is 08:20 in both eras.
- <https://www.jpx.co.jp/english/corporate/news/news-releases/0020/b5b4pj000003xrsa-att/InvestigationReport.pdf> — Investigation Report of November 30, 2020 into the October 1, 2020 system failure: "Order acceptance began as normal at 08:00", which dates the 08:00 acceptance for the post-2011 profile.
- <https://www.jpx.co.jp/english/equities/trading/domestic/tvdivq0000006blj-att/tradinghours_eg.pdf> — JPX's official trading-hours transition table, dating the arrowhead morning extension to 2011-11-21.
- <https://www.jpx.co.jp/english/corporate/news/news-releases/1030/uorii50000002f2a-att/pressrelease_extension_of_trading_hours_en.pdf> — 2024 extension appendix, expressly changing ToSTNeT single-stock/basket trading to 18:00.
- <https://www.jpx.co.jp/english/corporate/news/news-releases/1030/20241103-01.html> — final go-live release confirming the upgraded arrowhead and ToSTNeT systems launched on 2024-11-05.
- <https://www.jpx.co.jp/english/corporate/investor-relations/shareholders/meeting/tvdivq000000958w-att/tse04.pdf> — 2010 shareholder report, establishing that ToSTNeT had already been extended to 17:30 in November 2009, before the January-2010 audit floor.
- <https://www.jpx.co.jp/corporate/research-study/working-paper/tvdivq0000008q5y-att/JPX_working_paper_No.3.pdf> — JPX Working Paper No.3, analysing the operator's own FLEX order-book data from 2010-01-04 and explicitly identifying orders entered from 08:00 outside the matching session.

## Gaps and residual risks

- The 2010 shareholder report does not state an exact pre-floor day for the November-2009 ToSTNeT tail change, so none is invented (LAW-NO-FABRICATED-DATES). Nothing below the January-2010 floor is reviewed in any case.
- ToSTNeT is classified `extended` so the `regular` rules continue to describe the central auction market. Not every security or order type is eligible for every phase.
- The 08:00–08:20 arrowhead acceptance window is `order_entry`: orders may be entered, amended and cancelled, no matching engine runs, and ToSTNeT-1 does not open until 08:20, so nothing can print in it.
