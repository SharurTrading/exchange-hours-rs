<!-- SPDX-License-Identifier: MIT-0 -->

# `sgx_equity_index_china` — evidence

- **Kind:** `MarketHoursKey`
- **Owner module:** [`sgx_equity_index.rs`](../../src/calendar/schedules/futures/international/sgx_equity_index.rs)
- **Source sets:** [`APAC-SGX-DERIVATIVES`](../schedules/sources.md#apac-sgx-derivatives)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

**Gap: executable** — one undated move falls inside an era where trades print, and it is served as the intersection of the states sourced around it. FTSE China A50 (`CN`); the H50 futures and options (`FCH`, `FCHO`) appear in no pre-2020 SGX artifact and join the family's calendar listing at the 2021 edition on the same clock. "FTSE Xinhua China A50" on the oldest artifacts is the same contract under the index provider's former name, not a family change. **The one family with three distinct sourced states before 2013 — the only one whose T session moved, where Japan's and Singapore's two oldest states differ only in the T+1 leg — and the worked example of a boundary that may widen but never narrow.** SGX's pre-portal specification page captured 2009-02-27 prints the oldest: T "Opening 9.15am -11.35am / 1.00pm - 3.05pm" behind "Pre -Opening 9.00am -9.13 am / Non -Cancel Period 9.13 am -9.15 am", T+1 "Pre -Opening 3.35 pm - 3.38 pm / Non -Cancel Period 3.38 pm - 3.40 pm / Opening 3.40pm - 10.55pm", and no pre-closing routine. The ~2012 fragment prints 09:00–15:25 / 16:10–02:00 and the 2013-08-20 portal table 09:00–15:55 / 16:40–02:00; the 2013-08-20 page's own holiday tables still print the 15:25 close, which orders the two. Across the whole undated span the T+1 open is sourced at 15:40, 16:10, 16:40 and 17:00, so the floor row holds it at 17:00 — the narrowest sourced value — rather than granting an earlier open and withdrawing it at a capture, and only the T close, which widens, moves at the boundaries. From the floor: T 09:15–15:05 without the 2009 lunch break (withheld: closed under 2009, open under the later states), 09:00–09:15 as order entry (the 2009 queue, open from ~2012 — the phase both states support), T+1 17:00–22:55, and no closing routine (the 15:25–15:30 pair is closed under 2009). From Monday 2013-08-26 — the daytime widening to 15:55 and the overnight close running on to 02:00 on one boundary, keyed to the Monday after the capture because the row creates a wrapping close, at the cost of four trading days of the 15:55 close — T 09:00–15:55 with the T pre-opening routine the A50 specification captured 2013-08-20 states (Pre-Opening 8.45–8.58 / Non-Cancel 8.58–9.00) and its Pre-Closing/Non-Cancel pair 15:55–16:00, T+1 17:00–02:00 (the 16:30–16:40 queue withheld, its anchor held). From Monday 2017-07-10: T 09:00–16:30 and T+1 17:00–04:45 on the portal's 2017-07-05 and 2017-09-27 captures, with routines from the content API's 2019-01-16 A50 payload (Pre-Closing 4.30–4.34 / Non-Cancel 4.34–4.35; T+1 Pre-Opening 4.50–4.58 / Non-Cancel 4.58–5.00) — a knowledge boundary keyed to the Monday for the reason the Japan row gives. From Monday 2019-11-11 the 05:15 close on SGX's change log (v6.9, issued 2019-10-07), as the Japan row records, confirmed by the 2020 edition and the 2020-01-09 payload. From 2025-04-07 the current grid on DT/AM 15 of 2025, which moved the T+1 open to 16:45; DT/AM 50 of 2024 names no China contract and does not split this row. **Undated**: the two A50-specific T-session extensions before 2017, bracketed into 2016 by SGX's change log (#66). **Residual risks**: the undated 22:55 → 02:00 close moves of 2010 (the floor under-reports the T+1 leg to 2013-08-25 and never over-reports it), the undated removal of the lunch break and the 15:05 → 15:25 close between 2009 and ~2012, the 2016 window (bracketed to (2015-12-24, 2017-07-05) by the 2016 edition, with member notices placing the revision at the Titan launch of 2016-11-14, as the Japan row records); routines from the 2020 payload to DT/AM 15 are sourced by all 22 archived 2021–2024 payloads and the product pages either side of 2024-11-04.

## Revision rows

- 2013-08-26 — T1 — SGX portal Trading Hours table and FTSE China A50 specification, captures 2013-08-20, keyed to the Monday — T 09:00-15:55 with the sourced opening and closing routines, T+1 17:00-02:00; keyed to the Monday because the row creates a wrapping close.
- 2017-07-10 — T1 — SGX derivatives Trading Hours page, captures 2017-07-05 and 2017-09-27 — T 09:00-16:30 and T+1 17:00-04:45, routines from the content API's 2019-01-16 A50 payload; a knowledge boundary keyed to the Monday.
- 2019-11-11 — T1 — SGX Derivatives Products Description change log v6.9: Effective 11 Nov, T+1 close 05:15 — T+1 close moves to 05:15, confirmed by the 2020 calendar edition and the 2020-01-09 payload.
- 2025-04-07 — T1 — SGX-DT Circular DT/AM 15 of 2025 — the current grid, which moved the T+1 open to 16:45; DT/AM 50 of 2024 names no China contract and does not split this row.

## Sources

Row review: 2026-09-06 (UTC) is the date the ledger row was last reviewed as a
whole. Per-source retrieval dates were not recorded before the 2026-09-12
migration; where a later targeted review, capture or document date is recorded
beside a source below, that date governs for that source, and dates are added
as each source is re-verified.

- <https://www.sgx.com/derivatives/products/nikkei225futuresoptions> — SGX Nikkei 225 futures and options product page.
- <https://www.sgx.com/derivatives/products/chinaa50> — SGX FTSE China A50 product page.
- <https://www.sgx.com/derivatives/products/chinah50> — SGX FTSE China H50 product page.
- <https://www.sgx.com/derivatives/products/sgxsimsci> — SGX SiMSCI product page.
- <https://www.sgx.com/derivatives/products/sgxsti> — SGX Straits Times Index product page.
- <https://api2.sgx.com/sites/default/files/2026-01/SGX%20Calendar%202026_2.pdf> — SGX Calendar 2026 — the current trading-hours table.
- <https://api2.sgx.com/sites/default/files/2026-08/Derivatives+Products+Description+v17.6%20eff%2020260824,%2020260907.zip> — SGX Derivatives Products Description v17.6 — the operator's own dated product change log.
- <https://api2.sgx.com/sites/default/files/2025-07/DT%20Trading%20Calendar%202025%20%28updated%2031%20Jul%202025%29.pdf> — SGX DT Trading Calendar 2025 (updated 31 July 2025).
- <https://www.citicsf.com.hk/attachment?aid=95&uid=a1207308-0e3a-4a16-a869-a4d1b808a2b3> — SGX-DT Circular DT/AM 15 of 2025, served through a member mirror carrying SGX's own document metadata.

The historical artifacts below were recovered on 2026-09-12 from the
pre-reshape module comment
(`git show main:src/calendar/schedules/futures/international/sgx_equity_index/history.rs`),
which held the URL list this file's narrative relies on. Items marked
**(shared)** are cited by all five SGX equity-index evidence files —
[`japan`](sgx_equity_index_japan.md), [`china`](sgx_equity_index_china.md),
[`singapore`](sgx_equity_index_singapore.md),
[`taiwan`](sgx_equity_index_taiwan.md) and
[`ntr_usd`](sgx_equity_index_ntr_usd.md) — because the calendar editions, the
content-API payloads and the change-log workbook state every family's grid in
one document.

- <https://api2.sgx.com/sites/default/files/2018-05/SGX%20Derivatives%20Trading%20Calendar%202018%20%28Apr%29.pdf> — SGX Derivatives Trading Calendar 2018 (Apr) — PDF created 11 April 2018, served from api2.sgx.com and never archived; prints the 04:45 T+1 close and is the first edition listing the NTR (USD) suite — T1. **(shared)**
- <https://api2.sgx.com/sites/default/files/2019-01/2019%20DT%20Calendar.pdf> — SGX DT Calendar 2019 — "accurate as of 15 January 2019"; repeats the 2018 rows and the 04:45 T+1 close — T1. **(shared)**
- <https://api2.sgx.com/sites/default/files/2020-01/SGX%20Derivatives%20Trading%20Calendar%202020.pdf> — SGX Derivatives Trading Calendar 2020 — the first edition printing the 05:15 T+1 close — T1. **(shared)**
- <https://api2.sgx.com/sites/default/files/2021-01/SGX%20Derivatives%20Trading%20Calendar%202021.pdf> — SGX Derivatives Trading Calendar 2021 — T1. **(shared)**
- <https://api2.sgx.com/sites/default/files/2021-07/SGX_Derivatives%20Trading%20Calendar%202021%20%28Final%20-%20Jul%29.pdf> — SGX Derivatives Trading Calendar 2021 (Final - Jul) — T1. **(shared)**
- <https://api2.sgx.com/sites/default/files/2022-06/DT%20Trading%20Calendar%202022%20%28Final%29.pdf> — SGX DT Trading Calendar 2022 (Final) — no text layer; read from rendered pages — T1. **(shared)**
- <https://api2.sgx.com/sites/default/files/2024-01/SGX%20Calendar%202024.pdf> — SGX Calendar 2024 — T1. (No 2023 edition was located.) **(shared)**
- <https://api2.sgx.com/sites/default/files/2025-01/SGX%20Calendar%202025.pdf> — SGX Calendar 2025 — the edition that moves the Japan T close to 14:55 — T1. **(shared)**
- <https://web.archive.org/web/20190204200905id_/https://api2.sgx.com/content-api?queryId=9756cc24703868bca7da492a8e1aebd1268eaf70%3Aderivatives_products_list&variables=%7B%22limit%22%3A10000%2C%22lang%22%3A%22EN%22%7D> — SGX content API, capture 2019-02-04 — the per-family Pre-Opening / Non-Cancel / Pre-Closing routines the calendars exclude by footnote, with the 04:45 T+1 close — T1 through a verbatim public mirror. **(shared)**
- <https://web.archive.org/web/20190611051800id_/https://api2.sgx.com/content-api?queryId=5adaa923edc3b334f3d4a62a324e055c4be65025%3Aderivatives_products_list&variables=%7B%22limit%22%3A10000%2C%22lang%22%3A%22EN%22%7D> — SGX content API, capture 2019-06-11 — the day after the change log's stated 10 June; brackets the SiMSCI move (17:10 / 17:40 to 17:20 / 17:50) from above — T1 through a verbatim public mirror. **(shared)**
- <https://web.archive.org/web/20200109051211id_/https://api2.sgx.com/content-api?queryId=ef44c5f861fc84577240761863bf1f842f189d9f%3Aderivatives_products_list&variables=%7B%22limit%22%3A10000%2C%22lang%22%3A%22EN%22%7D> — SGX content API, capture 2020-01-09 — the same routines with the 05:15 close, stated inside the 2019-11-11 row's own interval — T1 through a verbatim public mirror. **(shared)**
- <https://rulebook.sgx.com/rulebook/futures-trading-rules> — SGX Futures Trading Rules — thirty-six "Amended on 14 November 2016" annotations for the Titan system cutover, none of them Rule 4.1.5, which delegates hours to the contract specifications: the rulebook dates the system, not the grid. **(shared)**
- <https://www.sgx.com/titan-dt-dc-portal> — SGX Titan DT/DC portal — the operator index that corroborates each member-mirrored circular's issue date, and whose 2017-06-17 capture shows the 27 July 2016 newsletter listed only as "Titan DTDC Newsletter - New Feature Overview 2". **(shared)**
- <https://web.archive.org/web/20130820090335id_/http://www.sgx.com/wps/portal/sgxweb/home/trading/derivatives/trading_hours_calendar> — SGX portal Trading Hours table, capture 2013-08-20 — state S0, the artifact the 2013-08-26 rows are keyed to (keyed to the Monday because the row creates a wrapping overnight close) — T1 through a verbatim public mirror.
- <https://web.archive.org/web/20170705000242id_/http://sgx.com/wps/portal/sgxweb_ch/home/trading/derivatives/trading_hours_calendar> — SGX portal Trading Hours table, capture 2017-07-05 — state A, the upper bound on the undated S0 to A move and the artifact behind the 2017-07-10 knowledge boundary — T1 through a verbatim public mirror.
- <https://web.archive.org/web/20170927124017id_/http://www.sgx.com/wps/portal/sgxweb/home/trading/derivatives/trading_hours_calendar> — SGX portal Trading Hours table, capture 2017-09-27 — byte-identical to the 2017-07-05 capture, which is why the 21 September 2017 "Change of Trading Hours" newsletter is not the move — T1 through a verbatim public mirror.
- <https://www.kgieworld.sg/docs/SGXDerivativesTradingCalendar2016_AUG.pdf> — SGX Derivatives Trading Calendar 2016 (Aug) — a verbatim SGX PDF (SGX cover and imprint, created 2016-08-31) served by KGI Futures (Singapore), an SGX-DT member; states "All dates and information are accurate as of 24 December 2015" and prints state S0, which is the lower bound of the S0 to A window — T1 through a verbatim member mirror, read at its as-of date and never its creation date.
- <https://web.archive.org/web/20090227040521id_/http://www.sgx.com:80/psv/derivatives/futures_options/equity_index/SGX_FTSE_Xinhua_China_A50_Index.shtml> — SGX pre-portal psv contract-specification page for the FTSE Xinhua China A50 Index, capture 2009-02-27 — state P: 09:15-11:35, 13:00-15:05 / 15:40-22:55, with "Pre -Opening 9.00am -9.13 am" and "Pre -Opening 3.35 pm - 3.38 pm / Non -Cancel Period 3.38 pm - 3.40 pm"; it prints no pre-closing routine, which is why `SGX_CHINA_FLOOR` serves no extended phase. Below the January-2010 floor — T1 through a verbatim public mirror.
- <https://web.archive.org/web/20190116144725id_/https://api2.sgx.com/content-api?queryId=e8c4b75927723d2bae18ec762abab178e0efcd9a%3Apage&variables=%7B%22path%22%3A%22%2Fderivatives%2Fproducts%2Fchinaa50%22%2C%22lang%22%3A%22EN%22%7D> — SGX content API, FTSE China A50 product page, capture 2019-01-16 — state B for this key — T1 through a verbatim public mirror.

## Gaps and residual risks

- **executable** — the two A50-specific T-session extensions before 2017 are undated, bracketed into 2016 by SGX's own change log (#66).
- **executable** — the removal of the 2009 lunch break and the 15:05 to 15:25 close move between the 2009 specification page and the ~2012 fragment are undated. The floor row withholds the lunch break (closed under 2009, open under the later states) and the 15:25-15:30 closing pair, and holds the T+1 open at 17:00 across the whole undated span because a knowledge boundary may only widen.
- **executable** — the pre-2017 move is undated. SGX's own Derivatives Products Description change log brackets it into 2016 (entries issued 2016-04-22 and 2016-07-15 carry no day; #66), the 2016 calendar edition served verbatim by KGI Futures and "accurate as of 24 December 2015" witnesses the older grid at that date, and the 2017-07-05 portal capture witnesses the newer one, so the move is bracketed to (2015-12-24, 2017-07-05) and only 2016 to mid-2017 is unwitnessed (the 2014, 2015 and later-2016 editions are unrecovered). Member notices, inadmissible for a row, put the market-wide revision at the Titan launch of Monday 2016-11-14 citing DT/AM 80 of 2016 Appendix 1, which no reachable copy holds. Closing condition: a reachable copy of DT/AM 80 of 2016 or another SGX artifact stating the day. Dormant identity, so recorded here rather than opened as an issue.
- **executable** — the T+1 close moved 22:55 to 02:00 between the 2009 specification page and the 2013 portal table on days SGX does not state; third-party press puts it at 01:00 from 2010-01-11 and 02:00 from 2010-08-30, which is T4 and inadmissible for a row. The floor row therefore under-reports the T+1 leg by up to three hours to 2013-08-25 and never over-reports it.
- **ordering** — the ~2012 fragment prints 09:00-15:25 / 16:10-02:00 and the 2013-08-20 portal table 09:00-15:55 / 16:40-02:00; the 2013-08-20 page's own holiday tables still print the 15:25 close, which orders the two.
- **naming** — "FTSE Xinhua China A50" on the oldest artifacts is the same contract under the index provider's former name, not a family change.
- **sourced, not carried** — the routines between the content API's 2020-01-09 payload and DT/AM 15 of 2025 are sourced rather than carried: all 22 archived content-API payloads of 2021-2024 reproduce them to the minute, and SGX's server-rendered product pages corroborate DT/AM 50 of 2024 routine by routine across the content API's 2024-10-07 to 2025-04-07 capture gap.

> Shared module. The narrative for [`sgx_equity_index.rs`](../../src/calendar/schedules/futures/international/sgx_equity_index.rs)
> lives in [`sgx_equity_index_japan`](sgx_equity_index_japan.md#module-narrative-moved-from-srccalendarschedulesfuturesinternationalsgx_equity_indexrs-on-2026-09-12-utc).
> Sibling identities: [`sgx_equity_index_japan`](sgx_equity_index_japan.md), [`sgx_equity_index_singapore`](sgx_equity_index_singapore.md).
