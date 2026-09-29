<!-- SPDX-License-Identifier: MIT-0 -->

# `sse` — evidence

- **Kind:** `Exchange`
- **Owner module:** [`sse.rs`](../../src/calendar/schedules/equities/apac/sse.rs)
- **Source sets:** [`APAC-CHINA-CASH`](../schedules/sources.md#apac-china-cash)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

Venue union includes the 15:00–15:30 block-order/trading phase already operative at the January-2010 floor. Later STAR eligibility does not create a new outer-envelope cutover.

## Revision rows

- 2018-08-20 — T1 — SSE news release 4947833 — the 14:57–15:00 closing call auction is added, so continuous trading ends 14:57.

## Holidays

**Coverage:** 2011-01-01..2026-12-31 (inclusive trade dates; T1 throughout)

Fourteen notices key the block, all the operator's own annual
closure-arrangement notices: 关于2011年全年休市安排的通知 (dated 2010-12-27)
onward, one per arrangement year through 关于上海证券交易所2024年部分节假日休市
安排的通知 (上证公告〔2023〕47号, dated 2023-12-26), then 上证公告〔2024〕38号 for
2025 and 上证公告〔2025〕45号 for 2026, with 上证公告〔2025〕36号 restating the 2025
October block verbatim — plus one dated operator adjustment, 上证公告〔2020〕6号
(关于调整2020年春节休市相关安排的公告, dated 2020-01-27), which extends the 2020
Spring Festival closure. The 2025-2026 rows were keyed when the venue turned
served (2026-09-28 UTC); the 2011-2024 rows were keyed on 2026-09-29 UTC. The
2011, 2012 and 2013 notices were retrieved live from the operator's own
media-center reprints (`aboutus/mediacenter/hotandd/`), where SSE republishes
its pre-2015 notices verbatim; the 2014-2024 notices are the announcement
channel's Wayback `id_` replays, and the 2020 extension is the same channel's
Wayback `id_` replay. Each notice prints its closures as event-date
ranges in session language — `休市` ("market closed") — for example 上证公告
〔2018〕39号's `（一）元旦：2018年12月30日（星期日）至2019年1月1日（星期二）休市，1月2日
（星期三）起照常开市`.

A row ships for each **weekday** a printed range covers. The days inside a
range that fall on a Saturday or Sunday are already closed by the normal week,
so they ship no rows, and the notices' `另外，X为周末休市` clauses — the national
working-weekend swaps (调休) — leave those days ordinary weekend closures for
the stock market with no session to encode, so none is invented. Every printed
date's own weekday name was checked against the civil calendar while deriving.
A range may reach back into the prior December (the 2019, 2023 and 2024
notices' 元旦 legs): its weekday legs ship with the notice that states them, so
2018-12-31 keys from 上证公告〔2018〕39号, published 2018-12-20, twelve days before
the trade date.

**The 2020 Spring Festival extension.** The annual notice 上证公告〔2019〕65号
states `春节：1月24日（星期五）至1月30日（星期四）休市，1月31日（星期五）起照常开市`.
On 2020-01-27 the operator's 上证公告〔2020〕6号 (关于调整2020年春节休市相关安排的
公告) states `延长2020年春节休市至2月2日（星期日），2月3日（星期一）正常开市` — an
unconditional, day-level adjustment of the earlier notice under `根据《国务院
办公厅关于延长2020年春节假期的通知》，经中国证监会批准`. The lineage is the
operator's own: the extension names the notice it adjusts, so the extension
governs the overlap and the annual notice's 1月31日 reopening clause is
superseded. Against the adjusted closure (1月24日 through 2月2日), the only
weekday the extension adds over the annual notice is 2020-01-31 — 2月1日 and
2月2日 fall on the weekend — and that row keys from `SSE-NOTICE-2020-6`.

The exchange prints no early close, late open or weekend
session for the cash market in any year of the window, and every row below is
a full closure.

### 2011

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2011-01-03 | closed | `元旦：1月1日（星期六）至1月3日（星期一）为节假日休市` | `SSE-NOTICE-2011` | T1 | Range 1月1日（星期六）至1月3日（星期一）; the range's own last day is a Monday |
| 2011-02-02 | closed | `春节：2月2日（星期三）至2月8日（星期二）为节假日休市` | `SSE-NOTICE-2011` | T1 | Range 2月2日（星期三）至2月8日（星期二）; weekday leg; 2011-02-05, 2011-02-06 are the weekend inside the range |
| 2011-02-03 | closed | `春节：2月2日（星期三）至2月8日（星期二）为节假日休市` | `SSE-NOTICE-2011` | T1 | Range 2月2日（星期三）至2月8日（星期二）; weekday leg; 2011-02-05, 2011-02-06 are the weekend inside the range |
| 2011-02-04 | closed | `春节：2月2日（星期三）至2月8日（星期二）为节假日休市` | `SSE-NOTICE-2011` | T1 | Range 2月2日（星期三）至2月8日（星期二）; weekday leg; 2011-02-05, 2011-02-06 are the weekend inside the range |
| 2011-02-07 | closed | `春节：2月2日（星期三）至2月8日（星期二）为节假日休市` | `SSE-NOTICE-2011` | T1 | Range 2月2日（星期三）至2月8日（星期二）; weekday leg; 2011-02-05, 2011-02-06 are the weekend inside the range |
| 2011-02-08 | closed | `春节：2月2日（星期三）至2月8日（星期二）为节假日休市` | `SSE-NOTICE-2011` | T1 | Range 2月2日（星期三）至2月8日（星期二）; the range's own last day is a Tuesday |
| 2011-04-04 | closed | `清明节：4月3日（星期日）至4月5日（星期二）为节假日休市` | `SSE-NOTICE-2011` | T1 | Range 4月3日（星期日）至4月5日（星期二）; weekday leg; 2011-04-03 are the weekend inside the range |
| 2011-04-05 | closed | `清明节：4月3日（星期日）至4月5日（星期二）为节假日休市` | `SSE-NOTICE-2011` | T1 | Range 4月3日（星期日）至4月5日（星期二）; the range's own last day is a Tuesday |
| 2011-05-02 | closed | `劳动节：4月30日（星期六）至5月2日（星期一）为节假日休市` | `SSE-NOTICE-2011` | T1 | Range 4月30日（星期六）至5月2日（星期一）; the range's own last day is a Monday |
| 2011-06-06 | closed | `端午节：6月4日（星期六）至6月6日（星期一）为节假日休市` | `SSE-NOTICE-2011` | T1 | Range 6月4日（星期六）至6月6日（星期一）; the range's own last day is a Monday |
| 2011-09-12 | closed | `中秋节：9月10日（星期六）至9月12日（星期一）为节假日休市` | `SSE-NOTICE-2011` | T1 | Range 9月10日（星期六）至9月12日（星期一）; the range's own last day is a Monday |
| 2011-10-03 | closed | `国庆节：10月1日（星期六）至10月7日（星期五）为节假日休市` | `SSE-NOTICE-2011` | T1 | Range 10月1日（星期六）至10月7日（星期五）; weekday leg; 2011-10-01, 2011-10-02 are the weekend inside the range |
| 2011-10-04 | closed | `国庆节：10月1日（星期六）至10月7日（星期五）为节假日休市` | `SSE-NOTICE-2011` | T1 | Range 10月1日（星期六）至10月7日（星期五）; weekday leg; 2011-10-01, 2011-10-02 are the weekend inside the range |
| 2011-10-05 | closed | `国庆节：10月1日（星期六）至10月7日（星期五）为节假日休市` | `SSE-NOTICE-2011` | T1 | Range 10月1日（星期六）至10月7日（星期五）; weekday leg; 2011-10-01, 2011-10-02 are the weekend inside the range |
| 2011-10-06 | closed | `国庆节：10月1日（星期六）至10月7日（星期五）为节假日休市` | `SSE-NOTICE-2011` | T1 | Range 10月1日（星期六）至10月7日（星期五）; weekday leg; 2011-10-01, 2011-10-02 are the weekend inside the range |
| 2011-10-07 | closed | `国庆节：10月1日（星期六）至10月7日（星期五）为节假日休市` | `SSE-NOTICE-2011` | T1 | Range 10月1日（星期六）至10月7日（星期五）; the range's own last day is a Friday |

### 2012

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2012-01-02 | closed | `元旦：1月1日（星期日）至1月3日（星期二）休市` | `SSE-NOTICE-2012` | T1 | Range 1月1日（星期日）至1月3日（星期二）; weekday leg; 2012-01-01 are the weekend inside the range |
| 2012-01-03 | closed | `元旦：1月1日（星期日）至1月3日（星期二）休市` | `SSE-NOTICE-2012` | T1 | Range 1月1日（星期日）至1月3日（星期二）; the range's own last day is a Tuesday |
| 2012-01-23 | closed | `春节：1月22日（星期日）至1月28日（星期六）休市` | `SSE-NOTICE-2012` | T1 | Range 1月22日（星期日）至1月28日（星期六）; weekday leg; 2012-01-22, 2012-01-28 are the weekend inside the range |
| 2012-01-24 | closed | `春节：1月22日（星期日）至1月28日（星期六）休市` | `SSE-NOTICE-2012` | T1 | Range 1月22日（星期日）至1月28日（星期六）; weekday leg; 2012-01-22, 2012-01-28 are the weekend inside the range |
| 2012-01-25 | closed | `春节：1月22日（星期日）至1月28日（星期六）休市` | `SSE-NOTICE-2012` | T1 | Range 1月22日（星期日）至1月28日（星期六）; weekday leg; 2012-01-22, 2012-01-28 are the weekend inside the range |
| 2012-01-26 | closed | `春节：1月22日（星期日）至1月28日（星期六）休市` | `SSE-NOTICE-2012` | T1 | Range 1月22日（星期日）至1月28日（星期六）; weekday leg; 2012-01-22, 2012-01-28 are the weekend inside the range |
| 2012-01-27 | closed | `春节：1月22日（星期日）至1月28日（星期六）休市` | `SSE-NOTICE-2012` | T1 | Range 1月22日（星期日）至1月28日（星期六）; weekday leg; 2012-01-22, 2012-01-28 are the weekend inside the range |
| 2012-04-02 | closed | `清明节：4月2日（星期一）至4月4日（星期三）休市` | `SSE-NOTICE-2012` | T1 | Range 4月2日（星期一）至4月4日（星期三）; weekday leg |
| 2012-04-03 | closed | `清明节：4月2日（星期一）至4月4日（星期三）休市` | `SSE-NOTICE-2012` | T1 | Range 4月2日（星期一）至4月4日（星期三）; weekday leg |
| 2012-04-04 | closed | `清明节：4月2日（星期一）至4月4日（星期三）休市` | `SSE-NOTICE-2012` | T1 | Range 4月2日（星期一）至4月4日（星期三）; the range's own last day is a Wednesday |
| 2012-04-30 | closed | `劳动节：4月29日（星期日）至5月1日（星期二）休市` | `SSE-NOTICE-2012` | T1 | Range 4月29日（星期日）至5月1日（星期二）; weekday leg; 2012-04-29 are the weekend inside the range |
| 2012-05-01 | closed | `劳动节：4月29日（星期日）至5月1日（星期二）休市` | `SSE-NOTICE-2012` | T1 | Range 4月29日（星期日）至5月1日（星期二）; the range's own last day is a Tuesday |
| 2012-06-22 | closed | `端午节：6月22日（星期五）至6月24日（星期日）休市` | `SSE-NOTICE-2012` | T1 | Range 6月22日（星期五）至6月24日（星期日）; weekday leg; 2012-06-23, 2012-06-24 are the weekend inside the range |
| 2012-10-01 | closed | `中秋节、国庆节：9月30日（星期日）至10月7日（星期日）休市` | `SSE-NOTICE-2012` | T1 | Range 9月30日（星期日）至10月7日（星期日）; weekday leg; 2012-09-30, 2012-10-06, 2012-10-07 are the weekend inside the range |
| 2012-10-02 | closed | `中秋节、国庆节：9月30日（星期日）至10月7日（星期日）休市` | `SSE-NOTICE-2012` | T1 | Range 9月30日（星期日）至10月7日（星期日）; weekday leg; 2012-09-30, 2012-10-06, 2012-10-07 are the weekend inside the range |
| 2012-10-03 | closed | `中秋节、国庆节：9月30日（星期日）至10月7日（星期日）休市` | `SSE-NOTICE-2012` | T1 | Range 9月30日（星期日）至10月7日（星期日）; weekday leg; 2012-09-30, 2012-10-06, 2012-10-07 are the weekend inside the range |
| 2012-10-04 | closed | `中秋节、国庆节：9月30日（星期日）至10月7日（星期日）休市` | `SSE-NOTICE-2012` | T1 | Range 9月30日（星期日）至10月7日（星期日）; weekday leg; 2012-09-30, 2012-10-06, 2012-10-07 are the weekend inside the range |
| 2012-10-05 | closed | `中秋节、国庆节：9月30日（星期日）至10月7日（星期日）休市` | `SSE-NOTICE-2012` | T1 | Range 9月30日（星期日）至10月7日（星期日）; weekday leg; 2012-09-30, 2012-10-06, 2012-10-07 are the weekend inside the range |

### 2013

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2013-01-01 | closed | `元旦：1月1日（星期二）至1月3日（星期四）休市` | `SSE-NOTICE-2013` | T1 | Range 1月1日（星期二）至1月3日（星期四）; weekday leg |
| 2013-01-02 | closed | `元旦：1月1日（星期二）至1月3日（星期四）休市` | `SSE-NOTICE-2013` | T1 | Range 1月1日（星期二）至1月3日（星期四）; weekday leg |
| 2013-01-03 | closed | `元旦：1月1日（星期二）至1月3日（星期四）休市` | `SSE-NOTICE-2013` | T1 | Range 1月1日（星期二）至1月3日（星期四）; the range's own last day is a Thursday |
| 2013-02-11 | closed | `春节：2月9日（星期六）至2月15日（星期五）休市` | `SSE-NOTICE-2013` | T1 | Range 2月9日（星期六）至2月15日（星期五）; weekday leg; 2013-02-09, 2013-02-10 are the weekend inside the range |
| 2013-02-12 | closed | `春节：2月9日（星期六）至2月15日（星期五）休市` | `SSE-NOTICE-2013` | T1 | Range 2月9日（星期六）至2月15日（星期五）; weekday leg; 2013-02-09, 2013-02-10 are the weekend inside the range |
| 2013-02-13 | closed | `春节：2月9日（星期六）至2月15日（星期五）休市` | `SSE-NOTICE-2013` | T1 | Range 2月9日（星期六）至2月15日（星期五）; weekday leg; 2013-02-09, 2013-02-10 are the weekend inside the range |
| 2013-02-14 | closed | `春节：2月9日（星期六）至2月15日（星期五）休市` | `SSE-NOTICE-2013` | T1 | Range 2月9日（星期六）至2月15日（星期五）; weekday leg; 2013-02-09, 2013-02-10 are the weekend inside the range |
| 2013-02-15 | closed | `春节：2月9日（星期六）至2月15日（星期五）休市` | `SSE-NOTICE-2013` | T1 | Range 2月9日（星期六）至2月15日（星期五）; the range's own last day is a Friday |
| 2013-04-04 | closed | `清明节：4月4日（星期四）至4月6日（星期六）休市` | `SSE-NOTICE-2013` | T1 | Range 4月4日（星期四）至4月6日（星期六）; weekday leg; 2013-04-06 are the weekend inside the range |
| 2013-04-05 | closed | `清明节：4月4日（星期四）至4月6日（星期六）休市` | `SSE-NOTICE-2013` | T1 | Range 4月4日（星期四）至4月6日（星期六）; weekday leg; 2013-04-06 are the weekend inside the range |
| 2013-04-29 | closed | `劳动节：4月29日（星期一）至5月1日（星期三）休市` | `SSE-NOTICE-2013` | T1 | Range 4月29日（星期一）至5月1日（星期三）; weekday leg |
| 2013-04-30 | closed | `劳动节：4月29日（星期一）至5月1日（星期三）休市` | `SSE-NOTICE-2013` | T1 | Range 4月29日（星期一）至5月1日（星期三）; weekday leg |
| 2013-05-01 | closed | `劳动节：4月29日（星期一）至5月1日（星期三）休市` | `SSE-NOTICE-2013` | T1 | Range 4月29日（星期一）至5月1日（星期三）; the range's own last day is a Wednesday |
| 2013-06-10 | closed | `端午节：6月10日（星期一）至6月12日（星期三）休市` | `SSE-NOTICE-2013` | T1 | Range 6月10日（星期一）至6月12日（星期三）; weekday leg |
| 2013-06-11 | closed | `端午节：6月10日（星期一）至6月12日（星期三）休市` | `SSE-NOTICE-2013` | T1 | Range 6月10日（星期一）至6月12日（星期三）; weekday leg |
| 2013-06-12 | closed | `端午节：6月10日（星期一）至6月12日（星期三）休市` | `SSE-NOTICE-2013` | T1 | Range 6月10日（星期一）至6月12日（星期三）; the range's own last day is a Wednesday |
| 2013-09-19 | closed | `中秋节：9月19日（星期四）至9月21日（星期六）休市` | `SSE-NOTICE-2013` | T1 | Range 9月19日（星期四）至9月21日（星期六）; weekday leg; 2013-09-21 are the weekend inside the range |
| 2013-09-20 | closed | `中秋节：9月19日（星期四）至9月21日（星期六）休市` | `SSE-NOTICE-2013` | T1 | Range 9月19日（星期四）至9月21日（星期六）; weekday leg; 2013-09-21 are the weekend inside the range |
| 2013-10-01 | closed | `国庆节：10月1日（星期二）至10月7日（星期一）休市` | `SSE-NOTICE-2013` | T1 | Range 10月1日（星期二）至10月7日（星期一）; weekday leg; 2013-10-05, 2013-10-06 are the weekend inside the range |
| 2013-10-02 | closed | `国庆节：10月1日（星期二）至10月7日（星期一）休市` | `SSE-NOTICE-2013` | T1 | Range 10月1日（星期二）至10月7日（星期一）; weekday leg; 2013-10-05, 2013-10-06 are the weekend inside the range |
| 2013-10-03 | closed | `国庆节：10月1日（星期二）至10月7日（星期一）休市` | `SSE-NOTICE-2013` | T1 | Range 10月1日（星期二）至10月7日（星期一）; weekday leg; 2013-10-05, 2013-10-06 are the weekend inside the range |
| 2013-10-04 | closed | `国庆节：10月1日（星期二）至10月7日（星期一）休市` | `SSE-NOTICE-2013` | T1 | Range 10月1日（星期二）至10月7日（星期一）; weekday leg; 2013-10-05, 2013-10-06 are the weekend inside the range |
| 2013-10-07 | closed | `国庆节：10月1日（星期二）至10月7日（星期一）休市` | `SSE-NOTICE-2013` | T1 | Range 10月1日（星期二）至10月7日（星期一）; the range's own last day is a Monday |

### 2014

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2014-01-01 | closed | `元旦：1月1日（星期三）休市` | `SSE-NOTICE-2014` | T1 | Event date 2014-01-01, a Wednesday; the single-day range's weekday is the trade date |
| 2014-01-31 | closed | `春节：1月31日（星期五）至2月6日（星期四）休市` | `SSE-NOTICE-2014` | T1 | Range 1月31日（星期五）至2月6日（星期四）; weekday leg; 2014-02-01, 2014-02-02 are the weekend inside the range |
| 2014-02-03 | closed | `春节：1月31日（星期五）至2月6日（星期四）休市` | `SSE-NOTICE-2014` | T1 | Range 1月31日（星期五）至2月6日（星期四）; weekday leg; 2014-02-01, 2014-02-02 are the weekend inside the range |
| 2014-02-04 | closed | `春节：1月31日（星期五）至2月6日（星期四）休市` | `SSE-NOTICE-2014` | T1 | Range 1月31日（星期五）至2月6日（星期四）; weekday leg; 2014-02-01, 2014-02-02 are the weekend inside the range |
| 2014-02-05 | closed | `春节：1月31日（星期五）至2月6日（星期四）休市` | `SSE-NOTICE-2014` | T1 | Range 1月31日（星期五）至2月6日（星期四）; weekday leg; 2014-02-01, 2014-02-02 are the weekend inside the range |
| 2014-02-06 | closed | `春节：1月31日（星期五）至2月6日（星期四）休市` | `SSE-NOTICE-2014` | T1 | Range 1月31日（星期五）至2月6日（星期四）; the range's own last day is a Thursday |
| 2014-04-07 | closed | `清明节：4月7日（星期一）休市` | `SSE-NOTICE-2014` | T1 | Event date 2014-04-07, a Monday; the single-day range's weekday is the trade date |
| 2014-05-01 | closed | `劳动节：5月1日（星期四）至5月3日（星期六）休市` | `SSE-NOTICE-2014` | T1 | Range 5月1日（星期四）至5月3日（星期六）; weekday leg; 2014-05-03 are the weekend inside the range |
| 2014-05-02 | closed | `劳动节：5月1日（星期四）至5月3日（星期六）休市` | `SSE-NOTICE-2014` | T1 | Range 5月1日（星期四）至5月3日（星期六）; weekday leg; 2014-05-03 are the weekend inside the range |
| 2014-06-02 | closed | `端午节：6月2日（星期一）休市` | `SSE-NOTICE-2014` | T1 | Event date 2014-06-02, a Monday; the single-day range's weekday is the trade date |
| 2014-09-08 | closed | `中秋节：9月8日（星期一）休市` | `SSE-NOTICE-2014` | T1 | Event date 2014-09-08, a Monday; the single-day range's weekday is the trade date |
| 2014-10-01 | closed | `国庆节：10月1日（星期三）至10月7日（星期二）休市` | `SSE-NOTICE-2014` | T1 | Range 10月1日（星期三）至10月7日（星期二）; weekday leg; 2014-10-04, 2014-10-05 are the weekend inside the range |
| 2014-10-02 | closed | `国庆节：10月1日（星期三）至10月7日（星期二）休市` | `SSE-NOTICE-2014` | T1 | Range 10月1日（星期三）至10月7日（星期二）; weekday leg; 2014-10-04, 2014-10-05 are the weekend inside the range |
| 2014-10-03 | closed | `国庆节：10月1日（星期三）至10月7日（星期二）休市` | `SSE-NOTICE-2014` | T1 | Range 10月1日（星期三）至10月7日（星期二）; weekday leg; 2014-10-04, 2014-10-05 are the weekend inside the range |
| 2014-10-06 | closed | `国庆节：10月1日（星期三）至10月7日（星期二）休市` | `SSE-NOTICE-2014` | T1 | Range 10月1日（星期三）至10月7日（星期二）; weekday leg; 2014-10-04, 2014-10-05 are the weekend inside the range |
| 2014-10-07 | closed | `国庆节：10月1日（星期三）至10月7日（星期二）休市` | `SSE-NOTICE-2014` | T1 | Range 10月1日（星期三）至10月7日（星期二）; the range's own last day is a Tuesday |

### 2015

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2015-01-01 | closed | `元旦：1月1日（星期四）至1月3日（星期六）休市` | `SSE-NOTICE-2014-15` | T1 | Range 1月1日（星期四）至1月3日（星期六）; weekday leg; 2015-01-03 are the weekend inside the range |
| 2015-01-02 | closed | `元旦：1月1日（星期四）至1月3日（星期六）休市` | `SSE-NOTICE-2014-15` | T1 | Range 1月1日（星期四）至1月3日（星期六）; weekday leg; 2015-01-03 are the weekend inside the range |
| 2015-02-18 | closed | `春节：2月18日（星期三）至2月24日（星期二）休市` | `SSE-NOTICE-2014-15` | T1 | Range 2月18日（星期三）至2月24日（星期二）; weekday leg; 2015-02-21, 2015-02-22 are the weekend inside the range |
| 2015-02-19 | closed | `春节：2月18日（星期三）至2月24日（星期二）休市` | `SSE-NOTICE-2014-15` | T1 | Range 2月18日（星期三）至2月24日（星期二）; weekday leg; 2015-02-21, 2015-02-22 are the weekend inside the range |
| 2015-02-20 | closed | `春节：2月18日（星期三）至2月24日（星期二）休市` | `SSE-NOTICE-2014-15` | T1 | Range 2月18日（星期三）至2月24日（星期二）; weekday leg; 2015-02-21, 2015-02-22 are the weekend inside the range |
| 2015-02-23 | closed | `春节：2月18日（星期三）至2月24日（星期二）休市` | `SSE-NOTICE-2014-15` | T1 | Range 2月18日（星期三）至2月24日（星期二）; weekday leg; 2015-02-21, 2015-02-22 are the weekend inside the range |
| 2015-02-24 | closed | `春节：2月18日（星期三）至2月24日（星期二）休市` | `SSE-NOTICE-2014-15` | T1 | Range 2月18日（星期三）至2月24日（星期二）; the range's own last day is a Tuesday |
| 2015-04-06 | closed | `清明节：4月5日（星期日）至4月6日（星期一）休市` | `SSE-NOTICE-2014-15` | T1 | Range 4月5日（星期日）至4月6日（星期一）; the range's own last day is a Monday |
| 2015-05-01 | closed | `劳动节：5月1日（星期五）至5月3日（星期日）休市` | `SSE-NOTICE-2014-15` | T1 | Range 5月1日（星期五）至5月3日（星期日）; weekday leg; 2015-05-02, 2015-05-03 are the weekend inside the range |
| 2015-06-22 | closed | `端午节：6月20日（星期六）至6月22日（星期一）休市` | `SSE-NOTICE-2014-15` | T1 | Range 6月20日（星期六）至6月22日（星期一）; the range's own last day is a Monday |
| 2015-10-01 | closed | `国庆节：10月1日（星期四）至10月7日（星期三）休市` | `SSE-NOTICE-2014-15` | T1 | Range 10月1日（星期四）至10月7日（星期三）; weekday leg; 2015-10-03, 2015-10-04 are the weekend inside the range |
| 2015-10-02 | closed | `国庆节：10月1日（星期四）至10月7日（星期三）休市` | `SSE-NOTICE-2014-15` | T1 | Range 10月1日（星期四）至10月7日（星期三）; weekday leg; 2015-10-03, 2015-10-04 are the weekend inside the range |
| 2015-10-05 | closed | `国庆节：10月1日（星期四）至10月7日（星期三）休市` | `SSE-NOTICE-2014-15` | T1 | Range 10月1日（星期四）至10月7日（星期三）; weekday leg; 2015-10-03, 2015-10-04 are the weekend inside the range |
| 2015-10-06 | closed | `国庆节：10月1日（星期四）至10月7日（星期三）休市` | `SSE-NOTICE-2014-15` | T1 | Range 10月1日（星期四）至10月7日（星期三）; weekday leg; 2015-10-03, 2015-10-04 are the weekend inside the range |
| 2015-10-07 | closed | `国庆节：10月1日（星期四）至10月7日（星期三）休市` | `SSE-NOTICE-2014-15` | T1 | Range 10月1日（星期四）至10月7日（星期三）; the range's own last day is a Wednesday |

### 2016

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2016-01-01 | closed | `元旦：1月1日（星期五）至1月3日（星期日）休市` | `SSE-NOTICE-2015-36` | T1 | Range 1月1日（星期五）至1月3日（星期日）; weekday leg; 2016-01-02, 2016-01-03 are the weekend inside the range |
| 2016-02-08 | closed | `春节：2月7日（星期日）至2月13日（星期六）休市` | `SSE-NOTICE-2015-36` | T1 | Range 2月7日（星期日）至2月13日（星期六）; weekday leg; 2016-02-07, 2016-02-13 are the weekend inside the range |
| 2016-02-09 | closed | `春节：2月7日（星期日）至2月13日（星期六）休市` | `SSE-NOTICE-2015-36` | T1 | Range 2月7日（星期日）至2月13日（星期六）; weekday leg; 2016-02-07, 2016-02-13 are the weekend inside the range |
| 2016-02-10 | closed | `春节：2月7日（星期日）至2月13日（星期六）休市` | `SSE-NOTICE-2015-36` | T1 | Range 2月7日（星期日）至2月13日（星期六）; weekday leg; 2016-02-07, 2016-02-13 are the weekend inside the range |
| 2016-02-11 | closed | `春节：2月7日（星期日）至2月13日（星期六）休市` | `SSE-NOTICE-2015-36` | T1 | Range 2月7日（星期日）至2月13日（星期六）; weekday leg; 2016-02-07, 2016-02-13 are the weekend inside the range |
| 2016-02-12 | closed | `春节：2月7日（星期日）至2月13日（星期六）休市` | `SSE-NOTICE-2015-36` | T1 | Range 2月7日（星期日）至2月13日（星期六）; weekday leg; 2016-02-07, 2016-02-13 are the weekend inside the range |
| 2016-04-04 | closed | `清明节：4月2日（星期六）至4月4日（星期一）休市` | `SSE-NOTICE-2015-36` | T1 | Range 4月2日（星期六）至4月4日（星期一）; the range's own last day is a Monday |
| 2016-05-02 | closed | `劳动节：4月30日（星期六）至5月2日（星期一）休市` | `SSE-NOTICE-2015-36` | T1 | Range 4月30日（星期六）至5月2日（星期一）; the range's own last day is a Monday |
| 2016-06-09 | closed | `端午节：6月9日（星期四）至6月11日（星期六）休市` | `SSE-NOTICE-2015-36` | T1 | Range 6月9日（星期四）至6月11日（星期六）; weekday leg; 2016-06-11 are the weekend inside the range |
| 2016-06-10 | closed | `端午节：6月9日（星期四）至6月11日（星期六）休市` | `SSE-NOTICE-2015-36` | T1 | Range 6月9日（星期四）至6月11日（星期六）; weekday leg; 2016-06-11 are the weekend inside the range |
| 2016-09-15 | closed | `中秋节：9月15日（星期四）至9月17日（星期六）休市` | `SSE-NOTICE-2015-36` | T1 | Range 9月15日（星期四）至9月17日（星期六）; weekday leg; 2016-09-17 are the weekend inside the range |
| 2016-09-16 | closed | `中秋节：9月15日（星期四）至9月17日（星期六）休市` | `SSE-NOTICE-2015-36` | T1 | Range 9月15日（星期四）至9月17日（星期六）; weekday leg; 2016-09-17 are the weekend inside the range |
| 2016-10-03 | closed | `国庆节：10月1日（星期六）至10月7日（星期五）休市` | `SSE-NOTICE-2015-36` | T1 | Range 10月1日（星期六）至10月7日（星期五）; weekday leg; 2016-10-01, 2016-10-02 are the weekend inside the range |
| 2016-10-04 | closed | `国庆节：10月1日（星期六）至10月7日（星期五）休市` | `SSE-NOTICE-2015-36` | T1 | Range 10月1日（星期六）至10月7日（星期五）; weekday leg; 2016-10-01, 2016-10-02 are the weekend inside the range |
| 2016-10-05 | closed | `国庆节：10月1日（星期六）至10月7日（星期五）休市` | `SSE-NOTICE-2015-36` | T1 | Range 10月1日（星期六）至10月7日（星期五）; weekday leg; 2016-10-01, 2016-10-02 are the weekend inside the range |
| 2016-10-06 | closed | `国庆节：10月1日（星期六）至10月7日（星期五）休市` | `SSE-NOTICE-2015-36` | T1 | Range 10月1日（星期六）至10月7日（星期五）; weekday leg; 2016-10-01, 2016-10-02 are the weekend inside the range |
| 2016-10-07 | closed | `国庆节：10月1日（星期六）至10月7日（星期五）休市` | `SSE-NOTICE-2015-36` | T1 | Range 10月1日（星期六）至10月7日（星期五）; the range's own last day is a Friday |

### 2017

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2017-01-02 | closed | `元旦：1月1日（星期日）至1月2日（星期一）休市` | `SSE-NOTICE-2016-25` | T1 | Range 1月1日（星期日）至1月2日（星期一）; the range's own last day is a Monday |
| 2017-01-27 | closed | `春节：1月27日（星期五）至2月2日（星期四）休市` | `SSE-NOTICE-2016-25` | T1 | Range 1月27日（星期五）至2月2日（星期四）; weekday leg; 2017-01-28, 2017-01-29 are the weekend inside the range |
| 2017-01-30 | closed | `春节：1月27日（星期五）至2月2日（星期四）休市` | `SSE-NOTICE-2016-25` | T1 | Range 1月27日（星期五）至2月2日（星期四）; weekday leg; 2017-01-28, 2017-01-29 are the weekend inside the range |
| 2017-01-31 | closed | `春节：1月27日（星期五）至2月2日（星期四）休市` | `SSE-NOTICE-2016-25` | T1 | Range 1月27日（星期五）至2月2日（星期四）; weekday leg; 2017-01-28, 2017-01-29 are the weekend inside the range |
| 2017-02-01 | closed | `春节：1月27日（星期五）至2月2日（星期四）休市` | `SSE-NOTICE-2016-25` | T1 | Range 1月27日（星期五）至2月2日（星期四）; weekday leg; 2017-01-28, 2017-01-29 are the weekend inside the range |
| 2017-02-02 | closed | `春节：1月27日（星期五）至2月2日（星期四）休市` | `SSE-NOTICE-2016-25` | T1 | Range 1月27日（星期五）至2月2日（星期四）; the range's own last day is a Thursday |
| 2017-04-03 | closed | `清明节：4月2日（星期日）至4月4日（星期二）休市` | `SSE-NOTICE-2016-25` | T1 | Range 4月2日（星期日）至4月4日（星期二）; weekday leg; 2017-04-02 are the weekend inside the range |
| 2017-04-04 | closed | `清明节：4月2日（星期日）至4月4日（星期二）休市` | `SSE-NOTICE-2016-25` | T1 | Range 4月2日（星期日）至4月4日（星期二）; the range's own last day is a Tuesday |
| 2017-05-01 | closed | `劳动节：4月29日（星期六）至5月1日（星期一）休市` | `SSE-NOTICE-2016-25` | T1 | Range 4月29日（星期六）至5月1日（星期一）; the range's own last day is a Monday |
| 2017-05-29 | closed | `端午节：5月28日（星期日）至5月30日（星期二）休市` | `SSE-NOTICE-2016-25` | T1 | Range 5月28日（星期日）至5月30日（星期二）; weekday leg; 2017-05-28 are the weekend inside the range |
| 2017-05-30 | closed | `端午节：5月28日（星期日）至5月30日（星期二）休市` | `SSE-NOTICE-2016-25` | T1 | Range 5月28日（星期日）至5月30日（星期二）; the range's own last day is a Tuesday |
| 2017-10-02 | closed | `中秋节、国庆节：10月1日（星期日）至10月8日（星期日）休市` | `SSE-NOTICE-2016-25` | T1 | Range 10月1日（星期日）至10月8日（星期日）; weekday leg; 2017-10-01, 2017-10-07, 2017-10-08 are the weekend inside the range |
| 2017-10-03 | closed | `中秋节、国庆节：10月1日（星期日）至10月8日（星期日）休市` | `SSE-NOTICE-2016-25` | T1 | Range 10月1日（星期日）至10月8日（星期日）; weekday leg; 2017-10-01, 2017-10-07, 2017-10-08 are the weekend inside the range |
| 2017-10-04 | closed | `中秋节、国庆节：10月1日（星期日）至10月8日（星期日）休市` | `SSE-NOTICE-2016-25` | T1 | Range 10月1日（星期日）至10月8日（星期日）; weekday leg; 2017-10-01, 2017-10-07, 2017-10-08 are the weekend inside the range |
| 2017-10-05 | closed | `中秋节、国庆节：10月1日（星期日）至10月8日（星期日）休市` | `SSE-NOTICE-2016-25` | T1 | Range 10月1日（星期日）至10月8日（星期日）; weekday leg; 2017-10-01, 2017-10-07, 2017-10-08 are the weekend inside the range |
| 2017-10-06 | closed | `中秋节、国庆节：10月1日（星期日）至10月8日（星期日）休市` | `SSE-NOTICE-2016-25` | T1 | Range 10月1日（星期日）至10月8日（星期日）; weekday leg; 2017-10-01, 2017-10-07, 2017-10-08 are the weekend inside the range |

### 2018

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2018-01-01 | closed | `元旦：1月1日（星期一）休市` | `SSE-NOTICE-2017-26` | T1 | Event date 2018-01-01, a Monday; the single-day range's weekday is the trade date |
| 2018-02-15 | closed | `春节：2月15日（星期四）至2月21日（星期三）休市` | `SSE-NOTICE-2017-26` | T1 | Range 2月15日（星期四）至2月21日（星期三）; weekday leg; 2018-02-17, 2018-02-18 are the weekend inside the range |
| 2018-02-16 | closed | `春节：2月15日（星期四）至2月21日（星期三）休市` | `SSE-NOTICE-2017-26` | T1 | Range 2月15日（星期四）至2月21日（星期三）; weekday leg; 2018-02-17, 2018-02-18 are the weekend inside the range |
| 2018-02-19 | closed | `春节：2月15日（星期四）至2月21日（星期三）休市` | `SSE-NOTICE-2017-26` | T1 | Range 2月15日（星期四）至2月21日（星期三）; weekday leg; 2018-02-17, 2018-02-18 are the weekend inside the range |
| 2018-02-20 | closed | `春节：2月15日（星期四）至2月21日（星期三）休市` | `SSE-NOTICE-2017-26` | T1 | Range 2月15日（星期四）至2月21日（星期三）; weekday leg; 2018-02-17, 2018-02-18 are the weekend inside the range |
| 2018-02-21 | closed | `春节：2月15日（星期四）至2月21日（星期三）休市` | `SSE-NOTICE-2017-26` | T1 | Range 2月15日（星期四）至2月21日（星期三）; the range's own last day is a Wednesday |
| 2018-04-05 | closed | `清明节：4月5日（星期四）至4月7日（星期六）休市` | `SSE-NOTICE-2017-26` | T1 | Range 4月5日（星期四）至4月7日（星期六）; weekday leg; 2018-04-07 are the weekend inside the range |
| 2018-04-06 | closed | `清明节：4月5日（星期四）至4月7日（星期六）休市` | `SSE-NOTICE-2017-26` | T1 | Range 4月5日（星期四）至4月7日（星期六）; weekday leg; 2018-04-07 are the weekend inside the range |
| 2018-04-30 | closed | `劳动节：4月29日（星期日）至5月1日（星期二）休市` | `SSE-NOTICE-2017-26` | T1 | Range 4月29日（星期日）至5月1日（星期二）; weekday leg; 2018-04-29 are the weekend inside the range |
| 2018-05-01 | closed | `劳动节：4月29日（星期日）至5月1日（星期二）休市` | `SSE-NOTICE-2017-26` | T1 | Range 4月29日（星期日）至5月1日（星期二）; the range's own last day is a Tuesday |
| 2018-06-18 | closed | `端午节：6月16日（星期六）至6月18日（星期一）休市` | `SSE-NOTICE-2017-26` | T1 | Range 6月16日（星期六）至6月18日（星期一）; the range's own last day is a Monday |
| 2018-09-24 | closed | `中秋节：9月22日（星期六）至9月24日（星期一）休市` | `SSE-NOTICE-2017-26` | T1 | Range 9月22日（星期六）至9月24日（星期一）; the range's own last day is a Monday |
| 2018-10-01 | closed | `国庆节：10月1日（星期一）至10月7日（星期日）休市` | `SSE-NOTICE-2017-26` | T1 | Range 10月1日（星期一）至10月7日（星期日）; weekday leg; 2018-10-06, 2018-10-07 are the weekend inside the range |
| 2018-10-02 | closed | `国庆节：10月1日（星期一）至10月7日（星期日）休市` | `SSE-NOTICE-2017-26` | T1 | Range 10月1日（星期一）至10月7日（星期日）; weekday leg; 2018-10-06, 2018-10-07 are the weekend inside the range |
| 2018-10-03 | closed | `国庆节：10月1日（星期一）至10月7日（星期日）休市` | `SSE-NOTICE-2017-26` | T1 | Range 10月1日（星期一）至10月7日（星期日）; weekday leg; 2018-10-06, 2018-10-07 are the weekend inside the range |
| 2018-10-04 | closed | `国庆节：10月1日（星期一）至10月7日（星期日）休市` | `SSE-NOTICE-2017-26` | T1 | Range 10月1日（星期一）至10月7日（星期日）; weekday leg; 2018-10-06, 2018-10-07 are the weekend inside the range |
| 2018-10-05 | closed | `国庆节：10月1日（星期一）至10月7日（星期日）休市` | `SSE-NOTICE-2017-26` | T1 | Range 10月1日（星期一）至10月7日（星期日）; weekday leg; 2018-10-06, 2018-10-07 are the weekend inside the range |
| 2018-12-31 | closed | `元旦：2018年12月30日（星期日）至2019年1月1日（星期二）休市` | `SSE-NOTICE-2018-39` | T1 | Range 12月30日（星期日）至1月1日（星期二）; weekday leg; 2018-12-30 are the weekend inside the range |

### 2019

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2019-01-01 | closed | `元旦：2018年12月30日（星期日）至2019年1月1日（星期二）休市` | `SSE-NOTICE-2018-39` | T1 | Range 12月30日（星期日）至1月1日（星期二）; the range's own last day is a Tuesday |
| 2019-02-04 | closed | `春节：2月4日（星期一）至2月10日（星期日）休市` | `SSE-NOTICE-2018-39` | T1 | Range 2月4日（星期一）至2月10日（星期日）; weekday leg; 2019-02-09, 2019-02-10 are the weekend inside the range |
| 2019-02-05 | closed | `春节：2月4日（星期一）至2月10日（星期日）休市` | `SSE-NOTICE-2018-39` | T1 | Range 2月4日（星期一）至2月10日（星期日）; weekday leg; 2019-02-09, 2019-02-10 are the weekend inside the range |
| 2019-02-06 | closed | `春节：2月4日（星期一）至2月10日（星期日）休市` | `SSE-NOTICE-2018-39` | T1 | Range 2月4日（星期一）至2月10日（星期日）; weekday leg; 2019-02-09, 2019-02-10 are the weekend inside the range |
| 2019-02-07 | closed | `春节：2月4日（星期一）至2月10日（星期日）休市` | `SSE-NOTICE-2018-39` | T1 | Range 2月4日（星期一）至2月10日（星期日）; weekday leg; 2019-02-09, 2019-02-10 are the weekend inside the range |
| 2019-02-08 | closed | `春节：2月4日（星期一）至2月10日（星期日）休市` | `SSE-NOTICE-2018-39` | T1 | Range 2月4日（星期一）至2月10日（星期日）; weekday leg; 2019-02-09, 2019-02-10 are the weekend inside the range |
| 2019-04-05 | closed | `清明节：4月5日（星期五）至4月7日（星期日）休市` | `SSE-NOTICE-2018-39` | T1 | Range 4月5日（星期五）至4月7日（星期日）; weekday leg; 2019-04-06, 2019-04-07 are the weekend inside the range |
| 2019-05-01 | closed | `劳动节：5月1日（星期三）休市` | `SSE-NOTICE-2018-39` | T1 | Event date 2019-05-01, a Wednesday; the single-day range's weekday is the trade date |
| 2019-06-07 | closed | `端午节：6月7日（星期五）至6月9日（星期日）休市` | `SSE-NOTICE-2018-39` | T1 | Range 6月7日（星期五）至6月9日（星期日）; weekday leg; 2019-06-08, 2019-06-09 are the weekend inside the range |
| 2019-09-13 | closed | `中秋节：9月13日（星期五）至9月15日（星期日）休市` | `SSE-NOTICE-2018-39` | T1 | Range 9月13日（星期五）至9月15日（星期日）; weekday leg; 2019-09-14, 2019-09-15 are the weekend inside the range |
| 2019-10-01 | closed | `国庆节：10月1日（星期二）至10月7日（星期一）休市` | `SSE-NOTICE-2018-39` | T1 | Range 10月1日（星期二）至10月7日（星期一）; weekday leg; 2019-10-05, 2019-10-06 are the weekend inside the range |
| 2019-10-02 | closed | `国庆节：10月1日（星期二）至10月7日（星期一）休市` | `SSE-NOTICE-2018-39` | T1 | Range 10月1日（星期二）至10月7日（星期一）; weekday leg; 2019-10-05, 2019-10-06 are the weekend inside the range |
| 2019-10-03 | closed | `国庆节：10月1日（星期二）至10月7日（星期一）休市` | `SSE-NOTICE-2018-39` | T1 | Range 10月1日（星期二）至10月7日（星期一）; weekday leg; 2019-10-05, 2019-10-06 are the weekend inside the range |
| 2019-10-04 | closed | `国庆节：10月1日（星期二）至10月7日（星期一）休市` | `SSE-NOTICE-2018-39` | T1 | Range 10月1日（星期二）至10月7日（星期一）; weekday leg; 2019-10-05, 2019-10-06 are the weekend inside the range |
| 2019-10-07 | closed | `国庆节：10月1日（星期二）至10月7日（星期一）休市` | `SSE-NOTICE-2018-39` | T1 | Range 10月1日（星期二）至10月7日（星期一）; the range's own last day is a Monday |

### 2020

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2020-01-01 | closed | `元旦：1月1日（星期三）休市` | `SSE-NOTICE-2019-65` | T1 | Event date 2020-01-01, a Wednesday; the single-day range's weekday is the trade date |
| 2020-01-24 | closed | `春节：1月24日（星期五）至1月30日（星期四）休市` | `SSE-NOTICE-2019-65` | T1 | Range 1月24日（星期五）至1月30日（星期四）; weekday leg; 2020-01-25, 2020-01-26 are the weekend inside the range |
| 2020-01-27 | closed | `春节：1月24日（星期五）至1月30日（星期四）休市` | `SSE-NOTICE-2019-65` | T1 | Range 1月24日（星期五）至1月30日（星期四）; weekday leg; 2020-01-25, 2020-01-26 are the weekend inside the range |
| 2020-01-28 | closed | `春节：1月24日（星期五）至1月30日（星期四）休市` | `SSE-NOTICE-2019-65` | T1 | Range 1月24日（星期五）至1月30日（星期四）; weekday leg; 2020-01-25, 2020-01-26 are the weekend inside the range |
| 2020-01-29 | closed | `春节：1月24日（星期五）至1月30日（星期四）休市` | `SSE-NOTICE-2019-65` | T1 | Range 1月24日（星期五）至1月30日（星期四）; weekday leg; 2020-01-25, 2020-01-26 are the weekend inside the range |
| 2020-01-30 | closed | `春节：1月24日（星期五）至1月30日（星期四）休市` | `SSE-NOTICE-2019-65` | T1 | Range 1月24日（星期五）至1月30日（星期四）; the range's own last day is a Thursday |
| 2020-01-31 | closed | `延长2020年春节休市至2月2日（星期日），2月3日（星期一）正常开市` | `SSE-NOTICE-2020-6` | T1 | The extension notice lengthens the Spring Festival closure past 2月2日; against the annual notice's superseded `1月31日（星期五）起照常开市` clause the extension's only newly closed weekday is 2020-01-31 (2月1日、2月2日 fall on the weekend) |
| 2020-04-06 | closed | `清明节：4月4日（星期六）至4月6日（星期一）休市` | `SSE-NOTICE-2019-65` | T1 | Range 4月4日（星期六）至4月6日（星期一）; the range's own last day is a Monday |
| 2020-05-01 | closed | `劳动节：5月1日（星期五）至5月5日（星期二）休市` | `SSE-NOTICE-2019-65` | T1 | Range 5月1日（星期五）至5月5日（星期二）; weekday leg; 2020-05-02, 2020-05-03 are the weekend inside the range |
| 2020-05-04 | closed | `劳动节：5月1日（星期五）至5月5日（星期二）休市` | `SSE-NOTICE-2019-65` | T1 | Range 5月1日（星期五）至5月5日（星期二）; weekday leg; 2020-05-02, 2020-05-03 are the weekend inside the range |
| 2020-05-05 | closed | `劳动节：5月1日（星期五）至5月5日（星期二）休市` | `SSE-NOTICE-2019-65` | T1 | Range 5月1日（星期五）至5月5日（星期二）; the range's own last day is a Tuesday |
| 2020-06-25 | closed | `端午节：6月25日（星期四）至6月27日（星期六）休市` | `SSE-NOTICE-2019-65` | T1 | Range 6月25日（星期四）至6月27日（星期六）; weekday leg; 2020-06-27 are the weekend inside the range |
| 2020-06-26 | closed | `端午节：6月25日（星期四）至6月27日（星期六）休市` | `SSE-NOTICE-2019-65` | T1 | Range 6月25日（星期四）至6月27日（星期六）; weekday leg; 2020-06-27 are the weekend inside the range |
| 2020-10-01 | closed | `国庆节、中秋节：10月1日（星期四）至10月8日（星期四）休市` | `SSE-NOTICE-2019-65` | T1 | Range 10月1日（星期四）至10月8日（星期四）; weekday leg; 2020-10-03, 2020-10-04 are the weekend inside the range |
| 2020-10-02 | closed | `国庆节、中秋节：10月1日（星期四）至10月8日（星期四）休市` | `SSE-NOTICE-2019-65` | T1 | Range 10月1日（星期四）至10月8日（星期四）; weekday leg; 2020-10-03, 2020-10-04 are the weekend inside the range |
| 2020-10-05 | closed | `国庆节、中秋节：10月1日（星期四）至10月8日（星期四）休市` | `SSE-NOTICE-2019-65` | T1 | Range 10月1日（星期四）至10月8日（星期四）; weekday leg; 2020-10-03, 2020-10-04 are the weekend inside the range |
| 2020-10-06 | closed | `国庆节、中秋节：10月1日（星期四）至10月8日（星期四）休市` | `SSE-NOTICE-2019-65` | T1 | Range 10月1日（星期四）至10月8日（星期四）; weekday leg; 2020-10-03, 2020-10-04 are the weekend inside the range |
| 2020-10-07 | closed | `国庆节、中秋节：10月1日（星期四）至10月8日（星期四）休市` | `SSE-NOTICE-2019-65` | T1 | Range 10月1日（星期四）至10月8日（星期四）; weekday leg; 2020-10-03, 2020-10-04 are the weekend inside the range |
| 2020-10-08 | closed | `国庆节、中秋节：10月1日（星期四）至10月8日（星期四）休市` | `SSE-NOTICE-2019-65` | T1 | Range 10月1日（星期四）至10月8日（星期四）; the range's own last day is a Thursday |

### 2021

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2021-01-01 | closed | `元旦：1月1日（星期五）至1月3日（星期日）休市` | `SSE-NOTICE-2020-48` | T1 | Range 1月1日（星期五）至1月3日（星期日）; weekday leg; 2021-01-02, 2021-01-03 are the weekend inside the range |
| 2021-02-11 | closed | `春节：2月11日（星期四）至2月17日（星期三）休市` | `SSE-NOTICE-2020-48` | T1 | Range 2月11日（星期四）至2月17日（星期三）; weekday leg; 2021-02-13, 2021-02-14 are the weekend inside the range |
| 2021-02-12 | closed | `春节：2月11日（星期四）至2月17日（星期三）休市` | `SSE-NOTICE-2020-48` | T1 | Range 2月11日（星期四）至2月17日（星期三）; weekday leg; 2021-02-13, 2021-02-14 are the weekend inside the range |
| 2021-02-15 | closed | `春节：2月11日（星期四）至2月17日（星期三）休市` | `SSE-NOTICE-2020-48` | T1 | Range 2月11日（星期四）至2月17日（星期三）; weekday leg; 2021-02-13, 2021-02-14 are the weekend inside the range |
| 2021-02-16 | closed | `春节：2月11日（星期四）至2月17日（星期三）休市` | `SSE-NOTICE-2020-48` | T1 | Range 2月11日（星期四）至2月17日（星期三）; weekday leg; 2021-02-13, 2021-02-14 are the weekend inside the range |
| 2021-02-17 | closed | `春节：2月11日（星期四）至2月17日（星期三）休市` | `SSE-NOTICE-2020-48` | T1 | Range 2月11日（星期四）至2月17日（星期三）; the range's own last day is a Wednesday |
| 2021-04-05 | closed | `清明节：4月3日（星期六）至4月5日（星期一）休市` | `SSE-NOTICE-2020-48` | T1 | Range 4月3日（星期六）至4月5日（星期一）; the range's own last day is a Monday |
| 2021-05-03 | closed | `劳动节：5月1日（星期六）至5月5日（星期三）休市` | `SSE-NOTICE-2020-48` | T1 | Range 5月1日（星期六）至5月5日（星期三）; weekday leg; 2021-05-01, 2021-05-02 are the weekend inside the range |
| 2021-05-04 | closed | `劳动节：5月1日（星期六）至5月5日（星期三）休市` | `SSE-NOTICE-2020-48` | T1 | Range 5月1日（星期六）至5月5日（星期三）; weekday leg; 2021-05-01, 2021-05-02 are the weekend inside the range |
| 2021-05-05 | closed | `劳动节：5月1日（星期六）至5月5日（星期三）休市` | `SSE-NOTICE-2020-48` | T1 | Range 5月1日（星期六）至5月5日（星期三）; the range's own last day is a Wednesday |
| 2021-06-14 | closed | `端午节：6月12日（星期六）至6月14日（星期一）休市` | `SSE-NOTICE-2020-48` | T1 | Range 6月12日（星期六）至6月14日（星期一）; the range's own last day is a Monday |
| 2021-09-20 | closed | `中秋节：9月19日（星期日）至9月21日（星期二）休市` | `SSE-NOTICE-2020-48` | T1 | Range 9月19日（星期日）至9月21日（星期二）; weekday leg; 2021-09-19 are the weekend inside the range |
| 2021-09-21 | closed | `中秋节：9月19日（星期日）至9月21日（星期二）休市` | `SSE-NOTICE-2020-48` | T1 | Range 9月19日（星期日）至9月21日（星期二）; the range's own last day is a Tuesday |
| 2021-10-01 | closed | `国庆节：10月1日（星期五）至10月7日（星期四）休市` | `SSE-NOTICE-2020-48` | T1 | Range 10月1日（星期五）至10月7日（星期四）; weekday leg; 2021-10-02, 2021-10-03 are the weekend inside the range |
| 2021-10-04 | closed | `国庆节：10月1日（星期五）至10月7日（星期四）休市` | `SSE-NOTICE-2020-48` | T1 | Range 10月1日（星期五）至10月7日（星期四）; weekday leg; 2021-10-02, 2021-10-03 are the weekend inside the range |
| 2021-10-05 | closed | `国庆节：10月1日（星期五）至10月7日（星期四）休市` | `SSE-NOTICE-2020-48` | T1 | Range 10月1日（星期五）至10月7日（星期四）; weekday leg; 2021-10-02, 2021-10-03 are the weekend inside the range |
| 2021-10-06 | closed | `国庆节：10月1日（星期五）至10月7日（星期四）休市` | `SSE-NOTICE-2020-48` | T1 | Range 10月1日（星期五）至10月7日（星期四）; weekday leg; 2021-10-02, 2021-10-03 are the weekend inside the range |
| 2021-10-07 | closed | `国庆节：10月1日（星期五）至10月7日（星期四）休市` | `SSE-NOTICE-2020-48` | T1 | Range 10月1日（星期五）至10月7日（星期四）; the range's own last day is a Thursday |

### 2022

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2022-01-03 | closed | `元旦：1月1日（星期六）至1月3日（星期一）休市` | `SSE-NOTICE-2021-37` | T1 | Range 1月1日（星期六）至1月3日（星期一）; the range's own last day is a Monday |
| 2022-01-31 | closed | `春节：1月31日（星期一）至2月6日（星期日）休市` | `SSE-NOTICE-2021-37` | T1 | Range 1月31日（星期一）至2月6日（星期日）; weekday leg; 2022-02-05, 2022-02-06 are the weekend inside the range |
| 2022-02-01 | closed | `春节：1月31日（星期一）至2月6日（星期日）休市` | `SSE-NOTICE-2021-37` | T1 | Range 1月31日（星期一）至2月6日（星期日）; weekday leg; 2022-02-05, 2022-02-06 are the weekend inside the range |
| 2022-02-02 | closed | `春节：1月31日（星期一）至2月6日（星期日）休市` | `SSE-NOTICE-2021-37` | T1 | Range 1月31日（星期一）至2月6日（星期日）; weekday leg; 2022-02-05, 2022-02-06 are the weekend inside the range |
| 2022-02-03 | closed | `春节：1月31日（星期一）至2月6日（星期日）休市` | `SSE-NOTICE-2021-37` | T1 | Range 1月31日（星期一）至2月6日（星期日）; weekday leg; 2022-02-05, 2022-02-06 are the weekend inside the range |
| 2022-02-04 | closed | `春节：1月31日（星期一）至2月6日（星期日）休市` | `SSE-NOTICE-2021-37` | T1 | Range 1月31日（星期一）至2月6日（星期日）; weekday leg; 2022-02-05, 2022-02-06 are the weekend inside the range |
| 2022-04-04 | closed | `清明节：4月3日（星期日）至4月5日（星期二）休市` | `SSE-NOTICE-2021-37` | T1 | Range 4月3日（星期日）至4月5日（星期二）; weekday leg; 2022-04-03 are the weekend inside the range |
| 2022-04-05 | closed | `清明节：4月3日（星期日）至4月5日（星期二）休市` | `SSE-NOTICE-2021-37` | T1 | Range 4月3日（星期日）至4月5日（星期二）; the range's own last day is a Tuesday |
| 2022-05-02 | closed | `劳动节：4月30日（星期六）至5月4日（星期三）休市` | `SSE-NOTICE-2021-37` | T1 | Range 4月30日（星期六）至5月4日（星期三）; weekday leg; 2022-04-30, 2022-05-01 are the weekend inside the range |
| 2022-05-03 | closed | `劳动节：4月30日（星期六）至5月4日（星期三）休市` | `SSE-NOTICE-2021-37` | T1 | Range 4月30日（星期六）至5月4日（星期三）; weekday leg; 2022-04-30, 2022-05-01 are the weekend inside the range |
| 2022-05-04 | closed | `劳动节：4月30日（星期六）至5月4日（星期三）休市` | `SSE-NOTICE-2021-37` | T1 | Range 4月30日（星期六）至5月4日（星期三）; the range's own last day is a Wednesday |
| 2022-06-03 | closed | `端午节：6月3日（星期五）至6月5日（星期日）休市` | `SSE-NOTICE-2021-37` | T1 | Range 6月3日（星期五）至6月5日（星期日）; weekday leg; 2022-06-04, 2022-06-05 are the weekend inside the range |
| 2022-09-12 | closed | `中秋节：9月10日（星期六）至9月12日（星期一）休市` | `SSE-NOTICE-2021-37` | T1 | Range 9月10日（星期六）至9月12日（星期一）; the range's own last day is a Monday |
| 2022-10-03 | closed | `国庆节：10月1日（星期六）至10月7日（星期五）休市` | `SSE-NOTICE-2021-37` | T1 | Range 10月1日（星期六）至10月7日（星期五）; weekday leg; 2022-10-01, 2022-10-02 are the weekend inside the range |
| 2022-10-04 | closed | `国庆节：10月1日（星期六）至10月7日（星期五）休市` | `SSE-NOTICE-2021-37` | T1 | Range 10月1日（星期六）至10月7日（星期五）; weekday leg; 2022-10-01, 2022-10-02 are the weekend inside the range |
| 2022-10-05 | closed | `国庆节：10月1日（星期六）至10月7日（星期五）休市` | `SSE-NOTICE-2021-37` | T1 | Range 10月1日（星期六）至10月7日（星期五）; weekday leg; 2022-10-01, 2022-10-02 are the weekend inside the range |
| 2022-10-06 | closed | `国庆节：10月1日（星期六）至10月7日（星期五）休市` | `SSE-NOTICE-2021-37` | T1 | Range 10月1日（星期六）至10月7日（星期五）; weekday leg; 2022-10-01, 2022-10-02 are the weekend inside the range |
| 2022-10-07 | closed | `国庆节：10月1日（星期六）至10月7日（星期五）休市` | `SSE-NOTICE-2021-37` | T1 | Range 10月1日（星期六）至10月7日（星期五）; the range's own last day is a Friday |

### 2023

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2023-01-02 | closed | `元旦：2022年12月31日（星期六）至2023年1月2日（星期一）休市` | `SSE-NOTICE-2022-51` | T1 | Range 12月31日（星期六）至1月2日（星期一）; the range's own last day is a Monday |
| 2023-01-23 | closed | `春节：1月21日（星期六）至1月27日（星期五）休市` | `SSE-NOTICE-2022-51` | T1 | Range 1月21日（星期六）至1月27日（星期五）; weekday leg; 2023-01-21, 2023-01-22 are the weekend inside the range |
| 2023-01-24 | closed | `春节：1月21日（星期六）至1月27日（星期五）休市` | `SSE-NOTICE-2022-51` | T1 | Range 1月21日（星期六）至1月27日（星期五）; weekday leg; 2023-01-21, 2023-01-22 are the weekend inside the range |
| 2023-01-25 | closed | `春节：1月21日（星期六）至1月27日（星期五）休市` | `SSE-NOTICE-2022-51` | T1 | Range 1月21日（星期六）至1月27日（星期五）; weekday leg; 2023-01-21, 2023-01-22 are the weekend inside the range |
| 2023-01-26 | closed | `春节：1月21日（星期六）至1月27日（星期五）休市` | `SSE-NOTICE-2022-51` | T1 | Range 1月21日（星期六）至1月27日（星期五）; weekday leg; 2023-01-21, 2023-01-22 are the weekend inside the range |
| 2023-01-27 | closed | `春节：1月21日（星期六）至1月27日（星期五）休市` | `SSE-NOTICE-2022-51` | T1 | Range 1月21日（星期六）至1月27日（星期五）; the range's own last day is a Friday |
| 2023-04-05 | closed | `清明节：4月5日（星期三）休市` | `SSE-NOTICE-2022-51` | T1 | Event date 2023-04-05, a Wednesday; the single-day range's weekday is the trade date |
| 2023-05-01 | closed | `劳动节：4月29日（星期六）至5月3日（星期三）休市` | `SSE-NOTICE-2022-51` | T1 | Range 4月29日（星期六）至5月3日（星期三）; weekday leg; 2023-04-29, 2023-04-30 are the weekend inside the range |
| 2023-05-02 | closed | `劳动节：4月29日（星期六）至5月3日（星期三）休市` | `SSE-NOTICE-2022-51` | T1 | Range 4月29日（星期六）至5月3日（星期三）; weekday leg; 2023-04-29, 2023-04-30 are the weekend inside the range |
| 2023-05-03 | closed | `劳动节：4月29日（星期六）至5月3日（星期三）休市` | `SSE-NOTICE-2022-51` | T1 | Range 4月29日（星期六）至5月3日（星期三）; the range's own last day is a Wednesday |
| 2023-06-22 | closed | `端午节：6月22日（星期四）至6月24日（星期六）休市` | `SSE-NOTICE-2022-51` | T1 | Range 6月22日（星期四）至6月24日（星期六）; weekday leg; 2023-06-24 are the weekend inside the range |
| 2023-06-23 | closed | `端午节：6月22日（星期四）至6月24日（星期六）休市` | `SSE-NOTICE-2022-51` | T1 | Range 6月22日（星期四）至6月24日（星期六）; weekday leg; 2023-06-24 are the weekend inside the range |
| 2023-09-29 | closed | `中秋节、国庆节：9月29日（星期五）至10月6日（星期五）休市` | `SSE-NOTICE-2022-51` | T1 | Range 9月29日（星期五）至10月6日（星期五）; weekday leg; 2023-09-30, 2023-10-01 are the weekend inside the range |
| 2023-10-02 | closed | `中秋节、国庆节：9月29日（星期五）至10月6日（星期五）休市` | `SSE-NOTICE-2022-51` | T1 | Range 9月29日（星期五）至10月6日（星期五）; weekday leg; 2023-09-30, 2023-10-01 are the weekend inside the range |
| 2023-10-03 | closed | `中秋节、国庆节：9月29日（星期五）至10月6日（星期五）休市` | `SSE-NOTICE-2022-51` | T1 | Range 9月29日（星期五）至10月6日（星期五）; weekday leg; 2023-09-30, 2023-10-01 are the weekend inside the range |
| 2023-10-04 | closed | `中秋节、国庆节：9月29日（星期五）至10月6日（星期五）休市` | `SSE-NOTICE-2022-51` | T1 | Range 9月29日（星期五）至10月6日（星期五）; weekday leg; 2023-09-30, 2023-10-01 are the weekend inside the range |
| 2023-10-05 | closed | `中秋节、国庆节：9月29日（星期五）至10月6日（星期五）休市` | `SSE-NOTICE-2022-51` | T1 | Range 9月29日（星期五）至10月6日（星期五）; weekday leg; 2023-09-30, 2023-10-01 are the weekend inside the range |
| 2023-10-06 | closed | `中秋节、国庆节：9月29日（星期五）至10月6日（星期五）休市` | `SSE-NOTICE-2022-51` | T1 | Range 9月29日（星期五）至10月6日（星期五）; the range's own last day is a Friday |

### 2024

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2024-01-01 | closed | `元旦：2023年12月30日（星期六）至2024年1月1日（星期一）休市` | `SSE-NOTICE-2023-47` | T1 | Range 12月30日（星期六）至1月1日（星期一）; the range's own last day is a Monday |
| 2024-02-09 | closed | `春节：2月9日（星期五）至2月17日（星期六）休市` | `SSE-NOTICE-2023-47` | T1 | Range 2月9日（星期五）至2月17日（星期六）; weekday leg; 2024-02-10, 2024-02-11, 2024-02-17 are the weekend inside the range |
| 2024-02-12 | closed | `春节：2月9日（星期五）至2月17日（星期六）休市` | `SSE-NOTICE-2023-47` | T1 | Range 2月9日（星期五）至2月17日（星期六）; weekday leg; 2024-02-10, 2024-02-11, 2024-02-17 are the weekend inside the range |
| 2024-02-13 | closed | `春节：2月9日（星期五）至2月17日（星期六）休市` | `SSE-NOTICE-2023-47` | T1 | Range 2月9日（星期五）至2月17日（星期六）; weekday leg; 2024-02-10, 2024-02-11, 2024-02-17 are the weekend inside the range |
| 2024-02-14 | closed | `春节：2月9日（星期五）至2月17日（星期六）休市` | `SSE-NOTICE-2023-47` | T1 | Range 2月9日（星期五）至2月17日（星期六）; weekday leg; 2024-02-10, 2024-02-11, 2024-02-17 are the weekend inside the range |
| 2024-02-15 | closed | `春节：2月9日（星期五）至2月17日（星期六）休市` | `SSE-NOTICE-2023-47` | T1 | Range 2月9日（星期五）至2月17日（星期六）; weekday leg; 2024-02-10, 2024-02-11, 2024-02-17 are the weekend inside the range |
| 2024-02-16 | closed | `春节：2月9日（星期五）至2月17日（星期六）休市` | `SSE-NOTICE-2023-47` | T1 | Range 2月9日（星期五）至2月17日（星期六）; weekday leg; 2024-02-10, 2024-02-11, 2024-02-17 are the weekend inside the range |
| 2024-04-04 | closed | `清明节：4月4日（星期四）至4月6日（星期六）休市` | `SSE-NOTICE-2023-47` | T1 | Range 4月4日（星期四）至4月6日（星期六）; weekday leg; 2024-04-06 are the weekend inside the range |
| 2024-04-05 | closed | `清明节：4月4日（星期四）至4月6日（星期六）休市` | `SSE-NOTICE-2023-47` | T1 | Range 4月4日（星期四）至4月6日（星期六）; weekday leg; 2024-04-06 are the weekend inside the range |
| 2024-05-01 | closed | `劳动节：5月1日（星期三）至5月5日（星期日）休市` | `SSE-NOTICE-2023-47` | T1 | Range 5月1日（星期三）至5月5日（星期日）; weekday leg; 2024-05-04, 2024-05-05 are the weekend inside the range |
| 2024-05-02 | closed | `劳动节：5月1日（星期三）至5月5日（星期日）休市` | `SSE-NOTICE-2023-47` | T1 | Range 5月1日（星期三）至5月5日（星期日）; weekday leg; 2024-05-04, 2024-05-05 are the weekend inside the range |
| 2024-05-03 | closed | `劳动节：5月1日（星期三）至5月5日（星期日）休市` | `SSE-NOTICE-2023-47` | T1 | Range 5月1日（星期三）至5月5日（星期日）; weekday leg; 2024-05-04, 2024-05-05 are the weekend inside the range |
| 2024-06-10 | closed | `端午节：6月10日（星期一）休市` | `SSE-NOTICE-2023-47` | T1 | Event date 2024-06-10, a Monday; the single-day range's weekday is the trade date |
| 2024-09-16 | closed | `中秋节：9月15日（星期日）至9月17日（星期二）休市` | `SSE-NOTICE-2023-47` | T1 | Range 9月15日（星期日）至9月17日（星期二）; weekday leg; 2024-09-15 are the weekend inside the range |
| 2024-09-17 | closed | `中秋节：9月15日（星期日）至9月17日（星期二）休市` | `SSE-NOTICE-2023-47` | T1 | Range 9月15日（星期日）至9月17日（星期二）; the range's own last day is a Tuesday |
| 2024-10-01 | closed | `国庆节：10月1日（星期二）至10月7日（星期一）休市` | `SSE-NOTICE-2023-47` | T1 | Range 10月1日（星期二）至10月7日（星期一）; weekday leg; 2024-10-05, 2024-10-06 are the weekend inside the range |
| 2024-10-02 | closed | `国庆节：10月1日（星期二）至10月7日（星期一）休市` | `SSE-NOTICE-2023-47` | T1 | Range 10月1日（星期二）至10月7日（星期一）; weekday leg; 2024-10-05, 2024-10-06 are the weekend inside the range |
| 2024-10-03 | closed | `国庆节：10月1日（星期二）至10月7日（星期一）休市` | `SSE-NOTICE-2023-47` | T1 | Range 10月1日（星期二）至10月7日（星期一）; weekday leg; 2024-10-05, 2024-10-06 are the weekend inside the range |
| 2024-10-04 | closed | `国庆节：10月1日（星期二）至10月7日（星期一）休市` | `SSE-NOTICE-2023-47` | T1 | Range 10月1日（星期二）至10月7日（星期一）; weekday leg; 2024-10-05, 2024-10-06 are the weekend inside the range |
| 2024-10-07 | closed | `国庆节：10月1日（星期二）至10月7日（星期一）休市` | `SSE-NOTICE-2023-47` | T1 | Range 10月1日（星期二）至10月7日（星期一）; the range's own last day is a Monday |
### 2025

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2025-01-01 | closed | `（一）元旦：1月1日（星期三）休市，1月2日（星期四）起照常开市` | `SSE-NOTICE-2024-38` | T1 | Event date 2025-01-01, a Wednesday; the single-day range's weekday is the trade date |
| 2025-01-28 | closed | `（二）春节：1月28日（星期二）至2月4日（星期二）休市，2月5日（星期三）起照常开市` | `SSE-NOTICE-2024-38` | T1 | Range 2025-01-28..2025-02-04; the weekday legs are the rows, Feb 1-2 being the weekend inside the range |
| 2025-01-29 | closed | `（二）春节：1月28日（星期二）至2月4日（星期二）休市` | `SSE-NOTICE-2024-38` | T1 | Range 2025-01-28..2025-02-04, weekday leg |
| 2025-01-30 | closed | `（二）春节：1月28日（星期二）至2月4日（星期二）休市` | `SSE-NOTICE-2024-38` | T1 | Range 2025-01-28..2025-02-04, weekday leg |
| 2025-01-31 | closed | `（二）春节：1月28日（星期二）至2月4日（星期二）休市` | `SSE-NOTICE-2024-38` | T1 | Range 2025-01-28..2025-02-04, weekday leg |
| 2025-02-03 | closed | `（二）春节：1月28日（星期二）至2月4日（星期二）休市` | `SSE-NOTICE-2024-38` | T1 | Range 2025-01-28..2025-02-04, weekday leg |
| 2025-02-04 | closed | `（二）春节：1月28日（星期二）至2月4日（星期二）休市` | `SSE-NOTICE-2024-38` | T1 | Range 2025-01-28..2025-02-04; the range's own last day is a Tuesday |
| 2025-04-04 | closed | `（三）清明节：4月4日（星期五）至4月6日（星期日）休市，4月7日（星期一）起照常开市` | `SSE-NOTICE-2024-38` | T1 | Range 2025-04-04..2025-04-06; the Friday is the range's only weekday |
| 2025-05-01 | closed | `（四）劳动节：5月1日（星期四）至5月5日（星期一）休市，5月6日（星期二）起照常开市` | `SSE-NOTICE-2024-38` | T1 | Range 2025-05-01..2025-05-05; weekday legs May 1-2 and May 5 |
| 2025-05-02 | closed | `（四）劳动节：5月1日（星期四）至5月5日（星期一）休市` | `SSE-NOTICE-2024-38` | T1 | Range 2025-05-01..2025-05-05, weekday leg |
| 2025-05-05 | closed | `（四）劳动节：5月1日（星期四）至5月5日（星期一）休市` | `SSE-NOTICE-2024-38` | T1 | Range 2025-05-01..2025-05-05; the range's own last day is a Monday |
| 2025-06-02 | closed | `（五）端午节：5月31日（星期六）至6月2日（星期一）休市，6月3日（星期二）起照常开市` | `SSE-NOTICE-2024-38` | T1 | Range 2025-05-31..2025-06-02; the Monday is the range's only weekday |
| 2025-10-01 | closed | `（六）国庆节、中秋节：10月1日（星期三）至10月8日（星期三）休市，10月9日（星期四）起照常开市` | `SSE-NOTICE-2024-38` | T1 | Range 2025-10-01..2025-10-08; weekday legs Oct 1-3 and Oct 6-8, Oct 4-5 being the weekend inside the range. Restated verbatim by `SSE-NOTICE-2025-36` |
| 2025-10-02 | closed | `（六）国庆节、中秋节：10月1日（星期三）至10月8日（星期三）休市` | `SSE-NOTICE-2024-38` | T1 | Range 2025-10-01..2025-10-08, weekday leg |
| 2025-10-03 | closed | `（六）国庆节、中秋节：10月1日（星期三）至10月8日（星期三）休市` | `SSE-NOTICE-2024-38` | T1 | Range 2025-10-01..2025-10-08, weekday leg |
| 2025-10-06 | closed | `（六）国庆节、中秋节：10月1日（星期三）至10月8日（星期三）休市` | `SSE-NOTICE-2024-38` | T1 | Range 2025-10-01..2025-10-08, weekday leg |
| 2025-10-07 | closed | `（六）国庆节、中秋节：10月1日（星期三）至10月8日（星期三）休市` | `SSE-NOTICE-2024-38` | T1 | Range 2025-10-01..2025-10-08, weekday leg |
| 2025-10-08 | closed | `（六）国庆节、中秋节：10月1日（星期三）至10月8日（星期三）休市` | `SSE-NOTICE-2024-38` | T1 | Range 2025-10-01..2025-10-08; the range's own last day is a Wednesday |

### 2026

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2026-01-01 | closed | `（一）元旦：1月1日（星期四）至1月3日（星期六）休市，1月5日（星期一）起照常开市。另外，1月4日（星期日）为周末休市` | `SSE-NOTICE-2025-45` | T1 | Range 2026-01-01..2026-01-03; the weekday legs are Jan 1-2, Jan 3 being a Saturday and Jan 4 a stated weekend closure |
| 2026-01-02 | closed | `（一）元旦：1月1日（星期四）至1月3日（星期六）休市` | `SSE-NOTICE-2025-45` | T1 | Range 2026-01-01..2026-01-03, weekday leg |
| 2026-02-16 | closed | `（二）春节：2月15日（星期日）至2月23日（星期一）休市，2月24日（星期二）起照常开市。另外，2月14日（星期六）、2月28日（星期六）为周末休市` | `SSE-NOTICE-2025-45` | T1 | Range 2026-02-15..2026-02-23; weekday legs Feb 16-20 and Feb 23, Feb 21-22 being the weekend inside the range |
| 2026-02-17 | closed | `（二）春节：2月15日（星期日）至2月23日（星期一）休市` | `SSE-NOTICE-2025-45` | T1 | Range 2026-02-15..2026-02-23, weekday leg |
| 2026-02-18 | closed | `（二）春节：2月15日（星期日）至2月23日（星期一）休市` | `SSE-NOTICE-2025-45` | T1 | Range 2026-02-15..2026-02-23, weekday leg |
| 2026-02-19 | closed | `（二）春节：2月15日（星期日）至2月23日（星期一）休市` | `SSE-NOTICE-2025-45` | T1 | Range 2026-02-15..2026-02-23, weekday leg |
| 2026-02-20 | closed | `（二）春节：2月15日（星期日）至2月23日（星期一）休市` | `SSE-NOTICE-2025-45` | T1 | Range 2026-02-15..2026-02-23, weekday leg |
| 2026-02-23 | closed | `（二）春节：2月15日（星期日）至2月23日（星期一）休市` | `SSE-NOTICE-2025-45` | T1 | Range 2026-02-15..2026-02-23; the range's own last day is a Monday |
| 2026-04-06 | closed | `（三）清明节：4月4日（星期六）至4月6日（星期一）休市，4月7日（星期二）起照常开市` | `SSE-NOTICE-2025-45` | T1 | Range 2026-04-04..2026-04-06; the Monday is the range's only weekday |
| 2026-05-01 | closed | `（四）劳动节：5月1日（星期五）至5月5日（星期二）休市，5月6日（星期三）起照常开市。另外，5月9日（星期六）为周末休市` | `SSE-NOTICE-2025-45` | T1 | Range 2026-05-01..2026-05-05; weekday legs May 1 and May 4-5 |
| 2026-05-04 | closed | `（四）劳动节：5月1日（星期五）至5月5日（星期二）休市` | `SSE-NOTICE-2025-45` | T1 | Range 2026-05-01..2026-05-05, weekday leg |
| 2026-05-05 | closed | `（四）劳动节：5月1日（星期五）至5月5日（星期二）休市` | `SSE-NOTICE-2025-45` | T1 | Range 2026-05-01..2026-05-05; the range's own last day is a Tuesday |
| 2026-06-19 | closed | `（五）端午节：6月19日（星期五）至6月21日（星期日）休市，6月22日（星期一）起照常开市` | `SSE-NOTICE-2025-45` | T1 | Range 2026-06-19..2026-06-21; the Friday is the range's only weekday |
| 2026-09-25 | closed | `（六）中秋节：9月25日（星期五）至9月27日（星期日）休市，9月28日（星期一）起照常开市` | `SSE-NOTICE-2025-45` | T1 | Range 2026-09-25..2026-09-27; the Friday is the range's only weekday |
| 2026-10-01 | closed | `（七）国庆节：10月1日（星期四）至10月7日（星期三）休市，10月8日（星期四）起照常开市。另外，9月20日（星期日）、10月10日（星期六）为周末休市` | `SSE-NOTICE-2025-45` | T1 | Range 2026-10-01..2026-10-07; weekday legs Oct 1-2 and Oct 5-7, Oct 3-4 being the weekend inside the range |
| 2026-10-02 | closed | `（七）国庆节：10月1日（星期四）至10月7日（星期三）休市` | `SSE-NOTICE-2025-45` | T1 | Range 2026-10-01..2026-10-07, weekday leg |
| 2026-10-05 | closed | `（七）国庆节：10月1日（星期四）至10月7日（星期三）休市` | `SSE-NOTICE-2025-45` | T1 | Range 2026-10-01..2026-10-07, weekday leg |
| 2026-10-06 | closed | `（七）国庆节：10月1日（星期四）至10月7日（星期三）休市` | `SSE-NOTICE-2025-45` | T1 | Range 2026-10-01..2026-10-07, weekday leg |
| 2026-10-07 | closed | `（七）国庆节：10月1日（星期四）至10月7日（星期三）休市` | `SSE-NOTICE-2025-45` | T1 | Range 2026-10-01..2026-10-07; the range's own last day is a Wednesday |

### Documents

| Document | Window | Replay or service URL | Capture or retrieval, UTC | Tier | sha256 |
|---|---|---|---|---|---|
| `SSE-NOTICE-2011` | 2011-01-01..2026-12-31 | <https://www.sse.com.cn/aboutus/mediacenter/hotandd/c/c_20150912_3988440.shtml> | retrieved 2026-09-29 UTC | T1 | `0b497d63c091daedb1e843dc98dec4efaf4fd770397971489e3f06f9b982e5c3` |
| `SSE-NOTICE-2012` | 2011-01-01..2026-12-31 | <https://www.sse.com.cn/aboutus/mediacenter/hotandd/c/c_20150912_3988508.shtml> | retrieved 2026-09-29 UTC | T1 | `b5133c270876754f1be91f78ae725919387c919beee667e59d845bd3d6080071` |
| `SSE-NOTICE-2013` | 2011-01-01..2026-12-31 | <https://www.sse.com.cn/aboutus/mediacenter/hotandd/c/c_20150912_3988635.shtml> | retrieved 2026-09-29 UTC | T1 | `35fc93671853b08297396868d80cfb335f806753b52130283efd2cb1cb3a0388` |
| `SSE-NOTICE-2014` | 2011-01-01..2026-12-31 | <https://web.archive.org/web/20140123073842id_/http://www.sse.com.cn/disclosure/announcement/general/c/c_20131226_3759639.shtml> | Wayback `id_` replay of capture `20140123073842`, retrieved 2026-09-29 UTC | T1 | `a1e6ca92063dcbea83526038bf5c7b52c041056a0decf840c5089935ab37d3a7` |
| `SSE-NOTICE-2014-15` | 2011-01-01..2026-12-31 | <https://web.archive.org/web/20141226235211id_/http://www.sse.com.cn/disclosure/announcement/general/c/c_20141224_3868137.shtml> | Wayback `id_` replay of capture `20141226235211`, retrieved 2026-09-29 UTC | T1 | `586d0bebf1ae88ab3732a4737816e4798504e2e7c4420a27d2e054f0f5b04c82` |
| `SSE-NOTICE-2015-36` | 2011-01-01..2026-12-31 | <https://web.archive.org/web/20161006130154id_/http://www.sse.com.cn/disclosure/announcement/general/c/c_20151224_4027676.shtml> | Wayback `id_` replay of capture `20161006130154`, retrieved 2026-09-29 UTC | T1 | `306ccb9222959309e556c3c5aa31e3ac64c639f36f5311d59a5b0252c21bdb3b` |
| `SSE-NOTICE-2016-25` | 2011-01-01..2026-12-31 | <https://web.archive.org/web/20170112192001id_/http://www.sse.com.cn/disclosure/announcement/general/c/c_20161222_4218613.shtml> | Wayback `id_` replay of capture `20170112192001`, retrieved 2026-09-29 UTC | T1 | `1608df61b5a3326975e742cbd3c8e9ab49d38fb748b292b5b4bcd57aae0a0cf3` |
| `SSE-NOTICE-2017-26` | 2011-01-01..2026-12-31 | <https://web.archive.org/web/20180101000930id_/http://www.sse.com.cn/disclosure/announcement/general/c/c_20171222_4438363.shtml> | Wayback `id_` replay of capture `20180101000930`, retrieved 2026-09-29 UTC | T1 | `ea360857f3dfc47d2f4003bca996e768aa3cbc813bc4750ecddc0c3ff17d900f` |
| `SSE-NOTICE-2018-39` | 2011-01-01..2026-12-31 | <https://web.archive.org/web/20190102063036id_/http://www.sse.com.cn/disclosure/announcement/general/c/c_20181220_4696473.shtml> | Wayback `id_` replay of capture `20190102063036`, retrieved 2026-09-29 UTC | T1 | `2873948d9003000d2cde91b926559dffb70721cca9220e23988fa80856ab584d` |
| `SSE-NOTICE-2019-65` | 2011-01-01..2026-12-31 | <https://web.archive.org/web/20191224162145id_/http://www.sse.com.cn/disclosure/announcement/general/c/c_20191220_4969627.shtml> | Wayback `id_` replay of capture `20191224162145`, retrieved 2026-09-29 UTC | T1 | `8c1f7e47852db24a1e9f63c9dcd46f4cf2d3fc96bec4ef45b3579ffc02fa8329` |
| `SSE-NOTICE-2020-6` | 2011-01-01..2026-12-31 | <https://web.archive.org/web/20200211105610id_/http://www.sse.com.cn/disclosure/announcement/general/c/c_20200127_4991582.shtml> | Wayback `id_` replay of capture `20200211105610`, retrieved 2026-09-29 UTC | T1 | `8695945221e5bd11bb25fd90273a909aad5a1e56aa8207584be1ec137fb4d92c` |
| `SSE-NOTICE-2020-48` | 2011-01-01..2026-12-31 | <https://web.archive.org/web/20210418025946id_/http://www.sse.com.cn/disclosure/announcement/general/c/c_20201224_5286949.shtml> | Wayback `id_` replay of capture `20210418025946`, retrieved 2026-09-29 UTC | T1 | `813744c6a57289996cca6260a19f8026fade6d066d809b68d7a899afec1270aa` |
| `SSE-NOTICE-2021-37` | 2011-01-01..2026-12-31 | <https://web.archive.org/web/20220518224459id_/http://www.sse.com.cn/disclosure/announcement/general/c/c_20211220_5662606.shtml> | Wayback `id_` replay of capture `20220518224459`, retrieved 2026-09-29 UTC | T1 | `494784607a6df3b052cfe3e78737285c06b6cf7019e5a2c56599173f3d60cff5` |
| `SSE-NOTICE-2022-51` | 2011-01-01..2026-12-31 | <https://web.archive.org/web/20230521054030id_/http://www.sse.com.cn/disclosure/announcement/general/c/c_20221227_5714458.shtml> | Wayback `id_` replay of capture `20230521054030`, retrieved 2026-09-29 UTC | T1 | `e40fc641b82ba8c10df25006c8e0ce8a2022ff399963fbe03297d5679b7c95f6` |
| `SSE-NOTICE-2023-47` | 2011-01-01..2026-12-31 | <https://web.archive.org/web/20240715201653id_/https://www.sse.com.cn/disclosure/announcement/general/c/c_20231226_5733939.shtml> | Wayback `id_` replay of capture `20240715201653`, retrieved 2026-09-29 UTC | T1 | `0309cec151e28e77790bd545e43e3d16945d359eaad561b828094b0933ec3245` |
| `SSE-EN-HOL-2013` | 2011-01-01..2026-12-31 | <https://web.archive.org/web/20130320173745id_/http://english.sse.com.cn/aboutsse/holiday/> | Wayback `id_` replay of capture `20130320173745`, retrieved 2026-09-29 UTC | T1 | `ea4e9de5ecc8a64c2b3d9f4932b3d6b590ea06c7fb8351c9c5a8a1f305605ed8` |
| `SSE-NOTICE-2024-38` | 2025-01-01..2026-12-31 | <https://www.sse.com.cn/disclosure/announcement/general/c/c_20241223_10767108.shtml> | retrieved 2026-09-28 UTC | T1 | `b88e5777d868192c156b85f7514fafcba8638a8bcec0cd80190b2e67769f5fff` |
| `SSE-NOTICE-2025-36` | 2025-01-01..2026-12-31 | <https://www.sse.com.cn/disclosure/announcement/general/c/c_20250925_10792976.shtml> | retrieved 2026-09-28 UTC | T1 | `b587eadba8fc24d097028039278b5fa7a1fccb1ab6ed69ce619c8fb5d686ce54` |
| `SSE-NOTICE-2025-45` | 2025-01-01..2026-12-31 | <https://www.sse.com.cn/disclosure/announcement/general/c/c_20251222_10802507.shtml> | retrieved 2026-09-28 UTC | T1 | `4e260b815ce1309175ed994e6490a4ccd51b998a5f92bcdc41cec00fc6823dc7` |

All nineteen artifacts are saved in the research store — the 2025-2027 notices under `holidays/raw/equities/sse/2025-2027/` and the fifteen 2011-2024 notices (plus the English 2013 schedule page that corroborates `SSE-NOTICE-2013` and the 2020 extension notice `SSE-NOTICE-2020-6`) under `holidays/raw/equities/sse/2010-2024/`, each directory with an `INDEX.md` carrying the same digests.

**Why the window runs 2011-01-01..2026-12-31.** The 2011 start is the earliest arrangement the operator's own channel still serves: SSE's media-center reprints begin with the 2011 notice, and the 2010 annual notice (上证交字〔2009〕42号, published 2009-12-23) is neither live on sse.com.cn nor captured in the Internet Archive, so 2010 sits outside every audited window and refuses rather than answers (see Gaps). Every notice from 2011 on states its year complete. **The forward end at 2026-12-31 is the operator's horizon, not a withholding.** SSE publishes the next year's arrangement each December; the 2026 notice was published 2025-12-22, and no `关于2027年部分节假日休市安排的通知` exists as of the 2026-09-28 retrieval (web search found none; the SSE announcement search and English calendar channels refused that day, see Gaps). **Closing condition:** publication of 上交所's 2027 notice, which extends the window to 2027-12-31. Re-checked monthly per LAW-WATCH.


## Sources

Row review: 2026-08-22 (UTC) is the date the ledger row was last reviewed as a
whole. Per-source retrieval dates were not recorded before the 2026-09-12
migration; where a later targeted review, capture or document date is recorded
beside a source below, that date governs for that source, and dates are added
as each source is re-verified.

- <https://www.sse.com.cn/lawandrules/sselawsrules2025/stocks/exchange/c/c_20260424_10816482.shtml> — current SSE trading rule, including the 15:00–15:30 block and fixed-price phase.
- <https://www.sse.com.cn/lawandrules/sselawsrules2025/repeal/rules/c/c_20120918_10785158.shtml> — historical SSE block-trading rule, establishing that declarations have been accepted and confirmed through 15:30 since before the January-2010 audit floor.
- <https://english.sse.com.cn/news/newsrelease/c/4947833.shtml> — SSE news release 4947833, adding the closing call auction on 2018-08-20.

## Gaps and residual risks

- **No holiday gap inside the audited window.** Every weekday closure the two notices print for 2025-2026 ships a row and every other trade date in the window is audited normal; the window's end is the operator's own publication horizon (the annual December notice), not a withholding. Closing condition for extension: the 2027 notice, re-checked monthly per LAW-WATCH.
- **The 2010 arrangement is unrecovered.** The annual notice 上证交字〔2009〕42号 (published 2009-12-23) is not among the media-center reprints (which begin with the 2011 notice) and no capture of the pre-2015 announcement pages exists in the Internet Archive; the surviving reprints of the 2010 per-holiday notices found by search are third-party pages, which are T3/T4 and key no row (LAW-PRIMARY-SOURCES). 2010-01-01..2010-12-31 therefore sits outside every audited window and the identity refuses those dates rather than claiming an answer. Closing condition: an operator-hosted copy of the 2010 annual notice or of the per-holiday notices that together state the year, live or archived, worked up the same way.
- **National-schedule dependence.** Each notice states `根据中国证监会有关通知要求` — the closures follow the State Council's civil holiday arrangement. A later State Council adjustment would move a printed date; a slipped or cancelled date is corrected as a schedule fix (LAW-NO-FABRICATED-DATES). The notices also announce pre-holiday system tests (`有关春节、国庆节测试事宜，本所将另行通知`); a test is not a session and nothing is modelled for one.
- **Retrieval channels.** The English trading-calendar page (`english.sse.com.cn/start/trading/schedule/calendar/`) refused twice with HTTP 403 on 2026-09-28 and the Internet Archive holds only 404 captures of it; the SSE announcement search API errored. The Chinese general-announcement notices above are the channel that works and is cited here.
- Block and fixed-price phases are `extended` by convention; not every security is eligible for them.
- The generic fixed-price expansion is deliberately not a revision row: it changed eligibility inside the existing venue envelope, not the exchange-level close. Its effective day is **undated here**. A `2026-07-06` date stood in this file, in `sse.rs` and in the ledger row before the 2026-09-12 reshape; no artifact in this repository, in the owner module's pre-move comment, or in the research store cites it, so under LAW-NO-FABRICATED-DATES it is removed rather than kept unsourced. Closing condition: an SSE notice or rule stating the expansion's effective day; the expansion changes no exchange-level boundary, so nothing in the profile depends on it.
- The pre-2018 profile's 15:00–15:30 block window is carried from the historical block-trading rule; the ledger records the venue union as January-2010-on for that reason.
