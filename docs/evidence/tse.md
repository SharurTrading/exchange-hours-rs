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

## Holidays

**Coverage:** 2025-01-01..2027-12-31 (inclusive trade dates; T1 throughout)

One artifact pair keys the block: the operator's own `Market Holidays` page, which prints the current and next year. The live page retrieved 2026-09-28 UTC (page state "Update : Feb. 06, 2026") carries the 2026 and 2027 tables; the 2025 table came from the same URL's Wayback `id_` replay at capture `20250923014239` (page state "Update : Mar. 07, 2025"), whose 2026 table matches the live page's 2026 table date for date. The page states the scope in one sentence: `JPX markets are closed on Saturdays, Sundays, national holidays, and on the dates indicated below.` The page's night-session and clearing paragraphs govern OSE/TOCOM derivatives and add nothing to the cash venue envelope.

The rows are exactly the printed dates that fall on a weekday. A printed holiday landing on a Saturday or Sunday removes no session beyond the normal week, so it ships no row: 2025-02-23 (Emperor's Birthday, Sunday), 2025-05-03 (Constitution Memorial Day, Saturday), 2025-05-04 (Greenery Day, Sunday), 2025-11-23 (Labor Thanksgiving Day, Sunday), 2026-05-03 (Constitution Memorial Day, Sunday), 2027-01-02 and 2027-01-03 (printed Market Holidays on a Saturday and a Sunday), and 2027-03-21 (Vernal Equinox, Sunday). The page's own observance rule — `National holidays that fall on a Sunday are observed on the closest following day that is not a national holiday` — is why 2025-02-24, 2025-05-06, 2025-11-24 and 2027-03-22 print as rows. It also states `September 22, 2026, is a holiday in accordance with Rule 3, Paragraph 3 of Act on National Holidays`, the citizen's holiday between the two September 2026 holidays. JPX prints no cash-market early close or late open on any date in this window, and every row below is a full closure.

### 2025

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2025-01-01 | closed | `Jan. 1 (Wed.) New Year's Day` | `JPX-HOL-2025` | T1 | Printed date 2025-01-01, a Wednesday; the weekday date is the trade date |
| 2025-01-02 | closed | `Jan. 2 (Thu.) Market Holiday` | `JPX-HOL-2025` | T1 | Printed date 2025-01-02, a Thursday |
| 2025-01-03 | closed | `Jan. 3 (Fri.) Market Holiday` | `JPX-HOL-2025` | T1 | Printed date 2025-01-03, a Friday |
| 2025-01-13 | closed | `Jan. 13 (Mon.) Coming of Age Day` | `JPX-HOL-2025` | T1 | Printed date 2025-01-13, a Monday |
| 2025-02-11 | closed | `Feb. 11 (Tue.) National Foundation Day` | `JPX-HOL-2025` | T1 | Printed date 2025-02-11, a Tuesday |
| 2025-02-24 | closed | `Feb. 24 (Mon.) Emperor's Birthday (Feb. 23) observed 1` | `JPX-HOL-2025` | T1 | Event date 2025-02-23 is a Sunday; the observance lands on Monday 2025-02-24 |
| 2025-03-20 | closed | `Mar. 20 (Thu.) Vernal Equinox` | `JPX-HOL-2025` | T1 | Printed date 2025-03-20, a Thursday |
| 2025-04-29 | closed | `Apr. 29 (Tue.) Showa Day` | `JPX-HOL-2025` | T1 | Printed date 2025-04-29, a Tuesday |
| 2025-05-05 | closed | `May 5 (Mon.) Children's Day` | `JPX-HOL-2025` | T1 | Printed date 2025-05-05, a Monday; May 3 and May 4 fall on the weekend and ship no rows |
| 2025-05-06 | closed | `May 6 (Tue.) Greenery Day (May 4) observed 1` | `JPX-HOL-2025` | T1 | Event date 2025-05-04 is a Sunday; the observance lands on Tuesday 2025-05-06 |
| 2025-07-21 | closed | `Jul. 21 (Mon.) Marine Day` | `JPX-HOL-2025` | T1 | Printed date 2025-07-21, a Monday |
| 2025-08-11 | closed | `Aug. 11 (Mon.) Mountain Day` | `JPX-HOL-2025` | T1 | Printed date 2025-08-11, a Monday |
| 2025-09-15 | closed | `Sep. 15 (Mon.) Respect for the Aged Day` | `JPX-HOL-2025` | T1 | Printed date 2025-09-15, a Monday |
| 2025-09-23 | closed | `Sep. 23 (Tue.) Autumnal Equinox` | `JPX-HOL-2025` | T1 | Printed date 2025-09-23, a Tuesday |
| 2025-10-13 | closed | `Oct. 13 (Mon.) Sports Day` | `JPX-HOL-2025` | T1 | Printed date 2025-10-13, a Monday |
| 2025-11-03 | closed | `Nov. 3 (Mon.) Culture Day` | `JPX-HOL-2025` | T1 | Printed date 2025-11-03, a Monday |
| 2025-11-24 | closed | `Nov. 24 (Mon.) Labor Thanksgiving Day (Nov. 23) observed 1` | `JPX-HOL-2025` | T1 | Event date 2025-11-23 is a Sunday; the observance lands on Monday 2025-11-24 |
| 2025-12-31 | closed | `Dec. 31 (Wed.) Market Holiday` | `JPX-HOL-2025` | T1 | Printed date 2025-12-31, a Wednesday |

### 2026

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2026-01-01 | closed | `Jan. 1 (Thu.) New Year's Day` | `JPX-HOL-2026-2027` | T1 | Printed date 2026-01-01, a Thursday |
| 2026-01-02 | closed | `Jan. 2 (Fri.) Market Holiday` | `JPX-HOL-2026-2027` | T1 | Printed date 2026-01-02, a Friday; the printed Jan. 3 Saturday Market Holiday ships no row |
| 2026-01-12 | closed | `Jan. 12 (Mon.) Coming of Age Day` | `JPX-HOL-2026-2027` | T1 | Printed date 2026-01-12, a Monday |
| 2026-02-11 | closed | `Feb. 11 (Wed.) National Foundation Day` | `JPX-HOL-2026-2027` | T1 | Printed date 2026-02-11, a Wednesday |
| 2026-02-23 | closed | `Feb. 23 (Mon.) Emperor's Birthday` | `JPX-HOL-2026-2027` | T1 | Printed date 2026-02-23, a Monday |
| 2026-03-20 | closed | `Mar. 20 (Fri.) Vernal Equinox` | `JPX-HOL-2026-2027` | T1 | Printed date 2026-03-20, a Friday |
| 2026-04-29 | closed | `Apr. 29 (Wed.) Showa Day` | `JPX-HOL-2026-2027` | T1 | Printed date 2026-04-29, a Wednesday |
| 2026-05-04 | closed | `May 4 (Mon.) Greenery Day` | `JPX-HOL-2026-2027` | T1 | Printed date 2026-05-04, a Monday; the printed May 3 Sunday Constitution Memorial Day ships no row |
| 2026-05-05 | closed | `May 5 (Tue.) Children's Day` | `JPX-HOL-2026-2027` | T1 | Printed date 2026-05-05, a Tuesday |
| 2026-05-06 | closed | `May 6 (Wed.) Constitution Memorial Day (May 3) observed 1` | `JPX-HOL-2026-2027` | T1 | Event date 2026-05-03 is a Sunday; the observance lands on Wednesday 2026-05-06 |
| 2026-07-20 | closed | `Jul. 20 (Mon.) Marine Day` | `JPX-HOL-2026-2027` | T1 | Printed date 2026-07-20, a Monday |
| 2026-08-11 | closed | `Aug. 11 (Tue.) Mountain Day` | `JPX-HOL-2026-2027` | T1 | Printed date 2026-08-11, a Tuesday |
| 2026-09-21 | closed | `Sep. 21 (Mon.) Respect for the Aged Day` | `JPX-HOL-2026-2027` | T1 | Printed date 2026-09-21, a Monday |
| 2026-09-22 | closed | `Sep. 22 (Tue.) Holiday 2` | `JPX-HOL-2026-2027` | T1 | Printed date 2026-09-22, a Tuesday; the page states it is `a holiday in accordance with Rule 3, Paragraph 3 of Act on National Holidays` |
| 2026-09-23 | closed | `Sep. 23 (Wed.) Autumnal Equinox` | `JPX-HOL-2026-2027` | T1 | Printed date 2026-09-23, a Wednesday |
| 2026-10-12 | closed | `Oct. 12 (Mon.) Sports Day` | `JPX-HOL-2026-2027` | T1 | Printed date 2026-10-12, a Monday |
| 2026-11-03 | closed | `Nov. 3 (Tue.) Culture Day` | `JPX-HOL-2026-2027` | T1 | Printed date 2026-11-03, a Tuesday |
| 2026-11-23 | closed | `Nov. 23 (Mon.) Labor Thanksgiving Day` | `JPX-HOL-2026-2027` | T1 | Printed date 2026-11-23, a Monday |
| 2026-12-31 | closed | `Dec. 31 (Thu.) Market Holiday` | `JPX-HOL-2026-2027` | T1 | Printed date 2026-12-31, a Thursday |

### 2027

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2027-01-01 | closed | `Jan. 1 (Fri.) New Year's Day` | `JPX-HOL-2026-2027` | T1 | Printed date 2027-01-01, a Friday; the printed Jan. 2 and Jan. 3 weekend Market Holidays ship no rows |
| 2027-01-11 | closed | `Jan. 11 (Mon.) Coming of Age Day` | `JPX-HOL-2026-2027` | T1 | Printed date 2027-01-11, a Monday |
| 2027-02-11 | closed | `Feb. 11 (Thu.) National Foundation Day` | `JPX-HOL-2026-2027` | T1 | Printed date 2027-02-11, a Thursday |
| 2027-02-23 | closed | `Feb. 23 (Tue.) Emperor's Birthday` | `JPX-HOL-2026-2027` | T1 | Printed date 2027-02-23, a Tuesday |
| 2027-03-22 | closed | `Mar. 22 (Mon.) Vernal Equinox (Mar. 21) observed 1` | `JPX-HOL-2026-2027` | T1 | Event date 2027-03-21 is a Sunday; the observance lands on Monday 2027-03-22 |
| 2027-04-29 | closed | `Apr. 29 (Thu.) Showa Day` | `JPX-HOL-2026-2027` | T1 | Printed date 2027-04-29, a Thursday |
| 2027-05-03 | closed | `May 3 (Mon.) Constitution Memorial Day` | `JPX-HOL-2026-2027` | T1 | Printed date 2027-05-03, a Monday |
| 2027-05-04 | closed | `May 4 (Tue.) Greenery Day` | `JPX-HOL-2026-2027` | T1 | Printed date 2027-05-04, a Tuesday |
| 2027-05-05 | closed | `May 5 (Wed.) Children's Day` | `JPX-HOL-2026-2027` | T1 | Printed date 2027-05-05, a Wednesday |
| 2027-07-19 | closed | `Jul. 19 (Mon.) Marine Day` | `JPX-HOL-2026-2027` | T1 | Printed date 2027-07-19, a Monday |
| 2027-08-11 | closed | `Aug. 11 (Wed.) Mountain Day` | `JPX-HOL-2026-2027` | T1 | Printed date 2027-08-11, a Wednesday |
| 2027-09-20 | closed | `Sep. 20 (Mon.) Respect for the Aged Day` | `JPX-HOL-2026-2027` | T1 | Printed date 2027-09-20, a Monday |
| 2027-09-23 | closed | `Sep. 23 (Thu.) Autumnal Equinox` | `JPX-HOL-2026-2027` | T1 | Printed date 2027-09-23, a Thursday |
| 2027-10-11 | closed | `Oct. 11 (Mon.) Sports Day` | `JPX-HOL-2026-2027` | T1 | Printed date 2027-10-11, a Monday |
| 2027-11-03 | closed | `Nov. 3 (Wed.) Culture Day` | `JPX-HOL-2026-2027` | T1 | Printed date 2027-11-03, a Wednesday |
| 2027-11-23 | closed | `Nov. 23 (Tue.) Labor Thanksgiving Day` | `JPX-HOL-2026-2027` | T1 | Printed date 2027-11-23, a Tuesday |
| 2027-12-31 | closed | `Dec. 31 (Fri.) Market Holiday` | `JPX-HOL-2026-2027` | T1 | Printed date 2027-12-31, a Friday |

### Documents

| Document | Window | Replay or service URL | Capture or retrieval, UTC | Tier | sha256 |
|---|---|---|---|---|---|
| `JPX-HOL-2025` | 2025-01-01..2027-12-31 | <https://web.archive.org/web/20250923014239id_/https://www.jpx.co.jp/english/corporate/about-jpx/calendar/index.html> | retrieved 2026-09-28 UTC (Wayback capture 2025-09-23, page state "Update : Mar. 07, 2025") | T1 | `b301e55c0d5e091602cabcb242dc28de76943e04db22115e4270b8a53de37877` |
| `JPX-HOL-2026-2027` | 2025-01-01..2027-12-31 | <https://www.jpx.co.jp/english/corporate/about-jpx/calendar/> | retrieved 2026-09-28 UTC (page state "Update : Feb. 06, 2026") | T1 | `32c6d13a925aff109c135947f2e809b5b9de1ae3d4dfb77b876f4a05d809090a` |

Both artifacts are saved in the research store under `holidays/raw/equities/tse/2025-2027/` with an `INDEX.md` carrying the same digests. The replayed capture is stored gzip-compressed exactly as served.

**Why the window stops at 2027-12-31 needs no gap row.** The operator's own publication pattern is the current and next year on one page; every printed 2027 date is unconditional and complete (both equinoxes are printed), so the window reaches the end of the published future and nothing inside it is withheld. Every other trade date in the window is a date the page does not modify and is audited normal. The page's own caveat — `Exchange holidays are subject to change due to changes to national holidays under Japan's Act on National Holidays` — is the standing residual risk recorded under Gaps below; no such change has occurred inside this window. Re-checked monthly per LAW-WATCH; the 2028 table extends the window when JPX prints it.


## Sources

Row review: 2026-08-24 (UTC) is the date the ledger row was last reviewed as a
whole. Per-source retrieval dates were not recorded before the 2026-09-12
migration; where a later targeted review, capture or document date is recorded
beside a source below, that date governs for that source, and dates are added
as each source is re-verified.

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

- **No holiday gap inside the audited window.** Every printed weekday closure of 2025-2027 ships a row and every other trade date in the window is audited normal; the window's end is the operator's own publication horizon (the current and next year), not a withholding. Closing condition for extension: JPX printing the 2028 table, which is re-checked monthly per LAW-WATCH.
- **The 2010-2024 holiday era is unmodelled.** The holiday table above was built from the operator's current-and-next-year page alone, per this wave's bounded scope, so dates before 2025-01-01 sit outside every audited window and the identity refuses them rather than claiming a holiday answer. What would close it: JPX's archived holiday pages per year (the same URL's Wayback captures) worked up the same way.
- **National-holiday drift.** The page itself warns `Exchange holidays are subject to change due to changes to national holidays under Japan's Act on National Holidays`; a mid-year legislative change could move a printed date, and a slipped or cancelled date is corrected as a schedule fix (LAW-NO-FABRICATED-DATES).
- The 2010 shareholder report does not state an exact pre-floor day for the November-2009 ToSTNeT tail change, so none is invented (LAW-NO-FABRICATED-DATES). Nothing below the January-2010 floor is reviewed in any case.
- ToSTNeT is classified `extended` so the `regular` rules continue to describe the central auction market. Not every security or order type is eligible for every phase.
- The 08:00–08:20 arrowhead acceptance window is `order_entry`: orders may be entered, amended and cancelled, no matching engine runs, and ToSTNeT-1 does not open until 08:20, so nothing can print in it.
