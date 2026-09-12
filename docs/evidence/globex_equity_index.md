<!-- SPDX-License-Identifier: MIT-0 -->

# `globex_equity_index` — evidence

- **Kind:** `MarketHoursKey`
- **Owner module:** [`cme_group.rs`](../../src/calendar/schedules/futures/us/cme_group.rs)
- **Source sets:** [`US-CME-GROUP`](../schedules/sources.md#us-cme-group)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

**Gap: order-entry** — the trading session is sourced; what is undated is a queue or post-close phase in which no trade can print. Scoped modern CME/CBOT U.S.-grid family, including YM/MYM but excluding full-size `SP`, NKD, BTIC, and TACO. Current RTH/ETH and queues are exact; sourced dated history retains the 2010 weekday-queue change and 2012/2015/2021 matching revisions, but omits the Sunday queue whose 16:15→16:00 move is bracketed to 2012-05-28..2012-06-07 by CME's own trading-hours captures, with both CME notice channels read in full across that window and silent on it. Dated profiles now carry the sourced Sunday 16:15–17:00 intersection from the January-2010 floor, so only the 16:00–16:15 quarter-hour remains withheld.

## Revision rows

- 2010-11-15 — T1 — CME Globex notice 20101025 — Monday–Thursday Pre-Open moves from 16:50 to 16:45 CT.
- 2012-11-18 — T1 — CME Globex notice 20121022 — new daily trading-hour schedule; post-halt slice becomes 15:30–16:15 CT including Fridays.
- 2015-09-20 — T1 — CME Globex notice 20150817 — CME Equity and CBOT Equity closes move 15 minutes earlier to 16:00 CT.
- 2021-06-27 — T1 — CME Globex notice 20210621 — the 15:15–15:30 CT halt is removed, producing the continuous 17:00–16:00 CT ETH envelope.
- 2026-08-22 — T1 — 2026-08-22 review: verified current, onset undated — knowledge-bound row widening the Sunday queue to the sourced current 16:00–17:00 CT Pre-Open.

## Sources

- <https://www.cmegroup.com/content/dam/cmegroup/education/modules/files/EQ240_EQ_for_AIT.pdf> — CME `EQ240` equity-index education module, the October-2009 product guide that supplies the complete audit-floor grid.
- <https://www.cmegroup.com/education/files/eq-trading-hours.pdf> — CME equity-index trading-hours sheet.
- <https://www.cmegroup.com/tools-information/lookups/advisories/clearing/Chadv12-423.html> — CME clearing advisory Chadv12-423, the 2012 trade-date boundary change.
- <https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20121022.html> — CME Globex notice 20121022, the 2012-11-18 revision's source.
- <https://www.cmegroup.com/tools-information/lookups/advisories/market-data/20121015.html> — CME market-data advisory 20121015, corroborating the 2012 change.
- <https://www.cmegroup.com/notices/clearing/2019/06/Chadv19-182.pdf> — CME clearing advisory Chadv19-182.
- <https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20101025.html> — CME Globex notice 20101025, the 2010-11-15 weekday Pre-Open move.
- <https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20150817.html> — CME Globex Notice #20150817 of 17 August 2015, the 2015-09-20 revision's source.
- <https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20150914.html> — CME Globex Notice #20150914, the "Effective this Monday" repeat of the same article.
- <https://www.cmegroup.com/notices/electronic-trading/2021/06/20210621.html> — CME Globex notice 20210621, the 2021-06-27 halt removal.
- <https://www.cmegroup.com/market-regulation/rule-filings/2021/6/21-244R_2.pdf> — CME rule filing 21-244R, corroborating the halt removal.
- <https://www.cmegroup.com/notices/ser/2022/02/SER-8921.pdf> — CME SER-8921, current-grid corroboration.
- <https://web.archive.org/web/20120503104328/http://www.cmegroup.com/trading_hours/equities-hours.html> — CME equities trading-hours page — capture 2012-05-03, Sunday Pre-Open still 16:15.
- <https://web.archive.org/web/20120616181609/http://www.cmegroup.com/trading_hours/equities-hours.html> — CME equities trading-hours page — capture 2012-06-16, Sunday Pre-Open already 16:00.
- <https://web.archive.org/web/20120511163357id_/http://www.cmegroup.com/trading_hours/index.html?show=Commodities> — CME trading-hours index — capture 2012-05-11.
- <https://web.archive.org/web/20120528102754id_/http://www.cmegroup.com/trading_hours/index.html> — CME trading-hours index — capture 2012-05-28, Sunday Pre-Open 16:15 platform-wide.
- <https://web.archive.org/web/20120607015831id_/http://www.cmegroup.com/trading_hours/> — CME trading-hours index — capture 2012-06-07, Sunday Pre-Open 16:00 platform-wide.
- <https://web.archive.org/web/20190820012118id_/https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20120521.html> — CME Globex Notice 2012-05-21 — read in full, silent on the Pre-Open.
- <https://web.archive.org/web/20190716070058id_/https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20120528.html> — CME Globex Notice 2012-05-28 — read in full, silent on the Pre-Open.
- <https://web.archive.org/web/20190720204402id_/https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20120604.html> — CME Globex Notice 2012-06-04 — read in full, silent on the Pre-Open.
- <https://web.archive.org/web/20120622070557id_/https://www.cmegroup.com/tools-information/lookups/advisories/market-data/20120528.html> — CME Market Data Notice 2012-05-28 — read in full, silent on the Pre-Open.

Official origin of the four trading-hours captures: <http://www.cmegroup.com/trading_hours/>.

## Gaps and residual risks

- **order-entry** — the Sunday Pre-Open's move from 16:15 to 16:00 CT has no operator-stated effective day. The 2026-08-31 review narrowed the bracket to 2012-05-28..2012-06-07 from CME's own trading-hours captures and read both CME dated notice channels in full across that window without finding an announcement, so the dated profiles serve the sourced 16:15–17:00 CT intersection and withhold only the 16:00–16:15 CT quarter-hour. Closing condition: a CME document that states the new Sunday Pre-Open in session language on a day-level effective date. Served identity, so tracked as an issue (LAW-FOLLOW-UPS-ARE-ISSUES).
- **residual risk** — the only Sunday inside the narrowed bracket is 2012-06-03; that is an observation about the bracket, not a source-stated effective day, so LAW-NO-FABRICATED-DATES keeps it out of the tables.
- **scope** — the family is the scoped modern CME/CBOT U.S.-grid set including YM and MYM; full-size `SP`, NKD, BTIC and TACO are excluded.

> Shared module. The narrative for
> [`cme_group.rs`](../../src/calendar/schedules/futures/us/cme_group.rs) lives in
> [`cme`](cme.md#module-narrative-moved-from-srccalendarschedulesfuturesuscme_grouprs-on-2026-09-12-utc).
> Sibling identities: [`cme`](cme.md).
