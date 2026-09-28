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

**Coverage:** 2025-01-01..2026-12-31 (inclusive trade dates; T1 throughout)

Two artifacts key the block, both the operator's own annual closure-arrangement notices: 上证公告〔2024〕38号 (`关于上海证券交易所2025年部分节假日休市安排的通知`, published 2024-12-23) states every 2025 closure, and 上证公告〔2025〕45号 (`关于上海证券交易所2026年部分节假日休市安排的通知`, published 2025-12-22) states every 2026 closure. 上证公告〔2025〕36号 (`关于2025年国庆节、中秋节休市安排的公告`, published 2025-09-25) restates the October 2025 block verbatim and corroborates it; no row rests on it alone. Each notice prints its closures as event-date ranges in session language — `休市` ("market closed") — for example `（六）国庆节、中秋节：10月1日（星期三）至10月8日（星期三）休市，10月9日（星期四）起照常开市`.

A row ships for each **weekday** a printed range covers. The days inside a range that fall on a Saturday or Sunday are already closed by the normal week, so they ship no rows, and the notices' `另外，X为周末休市` clauses — the national working-weekend swaps (调休), e.g. `1月26日（星期日）、2月8日（星期六）为周末休市` for the 2025 Spring Festival — leave those days ordinary weekend closures for the stock market with no session to encode, so none is invented. The exchange prints no early close, late open or weekend session for the cash market in either year, and every row below is a full closure.

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
| `SSE-NOTICE-2024-38` | 2025-01-01..2026-12-31 | <https://www.sse.com.cn/disclosure/announcement/general/c/c_20241223_10767108.shtml> | retrieved 2026-09-28 UTC | T1 | `b88e5777d868192c156b85f7514fafcba8638a8bcec0cd80190b2e67769f5fff` |
| `SSE-NOTICE-2025-36` | 2025-01-01..2026-12-31 | <https://www.sse.com.cn/disclosure/announcement/general/c/c_20250925_10792976.shtml> | retrieved 2026-09-28 UTC | T1 | `b587eadba8fc24d097028039278b5fa7a1fccb1ab6ed69ce619c8fb5d686ce54` |
| `SSE-NOTICE-2025-45` | 2025-01-01..2026-12-31 | <https://www.sse.com.cn/disclosure/announcement/general/c/c_20251222_10802507.shtml> | retrieved 2026-09-28 UTC | T1 | `4e260b815ce1309175ed994e6490a4ccd51b998a5f92bcdc41cec00fc6823dc7` |

All three artifacts are saved in the research store under `holidays/raw/equities/sse/2025-2027/` with an `INDEX.md` carrying the same digests.

**Why the window stops at 2026-12-31 is the operator's horizon, not a withholding.** SSE publishes the next year's arrangement each December; the 2026 notice was published 2025-12-22, and no `关于2027年部分节假日休市安排的通知` exists as of the 2026-09-28 retrieval (web search found none; the SSE announcement search and English calendar channels refused that day, see Gaps). **Closing condition:** publication of 上交所's 2027 notice, which extends the window to 2027-12-31. Re-checked monthly per LAW-WATCH.


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
- **The 2010-2024 holiday era is unmodelled.** The holiday table above was built from the two current annual notices alone, per this wave's bounded scope, so dates before 2025-01-01 sit outside every audited window and the identity refuses them rather than claiming a holiday answer. What would close it: the archived annual notices (上证公告 holiday-arrangement notices per year) worked up the same way.
- **National-schedule dependence.** Each notice states `根据中国证监会有关通知要求` — the closures follow the State Council's civil holiday arrangement. A later State Council adjustment would move a printed date; a slipped or cancelled date is corrected as a schedule fix (LAW-NO-FABRICATED-DATES). The notices also announce pre-holiday system tests (`有关春节、国庆节测试事宜，本所将另行通知`); a test is not a session and nothing is modelled for one.
- **Retrieval channels.** The English trading-calendar page (`english.sse.com.cn/start/trading/schedule/calendar/`) refused twice with HTTP 403 on 2026-09-28 and the Internet Archive holds only 404 captures of it; the SSE announcement search API errored. The Chinese general-announcement notices above are the channel that works and is cited here.
- Block and fixed-price phases are `extended` by convention; not every security is eligible for them.
- The generic fixed-price expansion is deliberately not a revision row: it changed eligibility inside the existing venue envelope, not the exchange-level close. Its effective day is **undated here**. A `2026-07-06` date stood in this file, in `sse.rs` and in the ledger row before the 2026-09-12 reshape; no artifact in this repository, in the owner module's pre-move comment, or in the research store cites it, so under LAW-NO-FABRICATED-DATES it is removed rather than kept unsourced. Closing condition: an SSE notice or rule stating the expansion's effective day; the expansion changes no exchange-level boundary, so nothing in the profile depends on it.
- The pre-2018 profile's 15:00–15:30 block window is carried from the historical block-trading rule; the ledger records the venue union as January-2010-on for that reason.
