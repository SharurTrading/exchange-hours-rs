<!-- SPDX-License-Identifier: MIT-0 -->

# `sgx_equity_index_singapore` — evidence

- **Kind:** `MarketHoursKey`
- **Owner module:** [`sgx_equity_index.rs`](../../src/calendar/schedules/futures/international/sgx_equity_index.rs)
- **Source sets:** [`APAC-SGX-DERIVATIVES`](../schedules/sources.md#apac-sgx-derivatives)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

**Gap: executable** — one undated move falls inside an era where trades print, and it is served as the intersection of the states sourced around it. SiMSCI and Straits Times Index (`SGP`, `SGPO`/`CSGP`, `ST`), modelled on the SGP futures grid. The family is a continuous rename, not a succession: the 2018 and 2019 editions call it "MSCI Singapore Free", the 2020 edition drops "Free", and the 2021 edition — which removes every other MSCI-branded SGX equity-index contract — keeps SGP, SGPO and NSP MSCI-branded. **Six eras, five of them dated.** From the January-2010 floor, the intersection of SGX's pre-portal specification pages (SiMSCI captured 2009-03-08 and STI 2009-02-20, identical: "Opening 8.30am-5.10 pm", "Pre-Closing 5.10 pm-5.14 pm / Non-Cancel Period 5.14 pm- 5.15 pm", T+1 "Opening 6.15 pm - 10.55pm", queues "8.15am -8.28 am / 8.28am -8.30 am" and "6.00 pm – 6.13 pm / 6.13 pm – 6.15pm") with the 2013-08-20 portal table ("SGX MSCI Singapore Index Futures / SGX Straits Times Index Futures" on one row, 08:30–17:10 / 18:15–02:00): T 08:30–17:10 with the 17:10–17:15 closing routine, T+1 18:15–22:55, and both queues 08:15–08:30 and 18:00–18:15, whose anchors never moved — the timeline baseline. From Monday 2013-08-26, T+1 to 02:00, keyed to the Monday after the capture because the row creates a wrapping close (22:55 the same day to 02:00). From Monday 2017-07-10, T 08:30–17:10 and T+1 17:40–04:45 on the portal's 2017 captures, routines from the content API's 2019-02-04 payload (Pre-Opening 8:15–8:28 / Non-Cancel 8:28–8:30; Pre-Closing 5:10–5:14 / Non-Cancel 5:14–5:15; T+1 Pre-Opening 5:30–5:38 / Non-Cancel 5:38–5:40). From Monday 2019-06-10, T 08:30–17:20 and T+1 17:50–04:45 with the 17:20–17:25 closing routine, on SGX's own change log (entry issued 2019-05-21, v6.1: "Amended trading hours for SGP, SGPO and ST eff 10 Jun" — one cell, no scoping step), bracketed by the content API's 2019-02-04 payload (17:10 / 17:40) and its 2019-06-11 payload (17:20 / 17:50), the next day. From Monday 2019-11-11 the 05:15 close on the same log (v6.9), as the Japan row records. From 2025-04-07 the current grid on DT/AM 15 of 2025, which moved the T+1 open to 17:35. **Undated**: the pre-2017 move (#66). **Residual risks**: the undated 22:55 → 02:00 close moves of 2010 (the floor under-reports the T+1 leg to 2013-08-25 and never over-reports it), the 2016 window (bracketed and attested as the Japan row records), and the ~2012 fragment's 07:55 Straits Times open, which post-dates SGX's February-2009 page (08:30) and is not carried; routines from the 2020 payload to DT/AM 15 are sourced by all 22 archived 2021–2024 payloads and the product pages either side of 2024-11-04.

## Revision rows

- 2013-08-26 — T1 — SGX portal Trading Hours table, capture 2013-08-20, keyed to the Monday — T+1 close runs on to 02:00; keyed to the Monday because the row creates a wrapping close.
- 2017-07-10 — T1 — SGX derivatives Trading Hours page, captures 2017-07-05 and 2017-09-27 — T 08:30-17:10 and T+1 17:40-04:45, routines from the content API's 2019-02-04 payload.
- 2019-06-10 — T1 — SGX Derivatives Products Description change log v6.1: SGP, SGPO and ST eff 10 Jun — T 08:30-17:20 and T+1 17:50-04:45 with the 17:20-17:25 closing routine, bracketed by the content API's 2019-02-04 and 2019-06-11 payloads.
- 2019-11-11 — T1 — SGX Derivatives Products Description change log v6.9: Effective 11 Nov, T+1 close 05:15 — T+1 close moves to 05:15.
- 2025-04-07 — T1 — SGX-DT Circular DT/AM 15 of 2025 — the current grid, which moved the T+1 open to 17:35.

## Sources

Retrieval dates: these sources were last opened on the row's reviewed-on date
(2026-09-06, UTC); per-source retrieval dates were not recorded before the
2026-09-12 migration and are added as each source is re-verified.

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
- <https://web.archive.org/web/20090308120909id_/http://www.sgx.com:80/psv/derivatives/futures_options/equity_index/SGX_MSCI_Singapore_Index.shtml> — SGX pre-portal psv contract-specification page for the MSCI Singapore Index, capture 2009-03-08 — state P: 08:30-17:10 / 18:15-22:55, with "Pre -Opening 8.15am -8.28 am / Non -Cancel Period 8.28am -8.30 am", "Pre -Opening 6.00 pm – 6.13 pm / Non -Cancel Period 6.13 pm – 6.15pm" and "Pre-Closing 5.10 pm-5.14 pm / Non-Cancel Period 5.14 pm-5.15 pm". Below the January-2010 floor — T1 through a verbatim public mirror.
- <https://web.archive.org/web/20090220005028id_/http://sgx.com:80/psv/derivatives/futures_options/equity_index/SGX_Straits_Times_Index.shtml> — SGX pre-portal psv contract-specification page for the Straits Times Index, capture 2009-02-20 — state P, 08:30-17:10 / 18:15-22:55, which is why the ~2012 fragment's 07:55 open is not carried. Below the January-2010 floor — T1 through a verbatim public mirror.

## Gaps and residual risks

- **executable** — the pre-2017 move is undated (#66). SGX's own Derivatives Products Description change log brackets it into 2016 (entries issued 2016-04-22 and 2016-07-15 carry no day; #66), the 2016 calendar edition served verbatim by KGI Futures and "accurate as of 24 December 2015" witnesses the older grid at that date, and the 2017-07-05 portal capture witnesses the newer one, so the move is bracketed to (2015-12-24, 2017-07-05) and only 2016 to mid-2017 is unwitnessed (the 2014, 2015 and later-2016 editions are unrecovered). Member notices, inadmissible for a row, put the market-wide revision at the Titan launch of Monday 2016-11-14 citing DT/AM 80 of 2016 Appendix 1, which no reachable copy holds. Closing condition: a reachable copy of DT/AM 80 of 2016 or another SGX artifact stating the day. Dormant identity, so recorded here rather than opened as an issue.
- **executable** — the T+1 close moved 22:55 to 02:00 between the 2009 specification page and the 2013 portal table on days SGX does not state; third-party press puts it at 01:00 from 2010-01-11 and 02:00 from 2010-08-30, which is T4 and inadmissible for a row. The floor row therefore under-reports the T+1 leg by up to three hours to 2013-08-25 and never over-reports it.
- **not carried** — the ~2012 fragment's 07:55 Straits Times open post-dates SGX's February-2009 page (08:30) and is not carried.
- **calibration** — the 2019-06-10 change-log entry is one cell with no scoping step ("Amended trading hours for SGP, SGPO and ST eff 10 Jun") and is bracketed to the day by the content API's 2019-02-04 payload (17:10 / 17:40) and its 2019-06-11 payload (17:20 / 17:50), the next day.
- **sourced, not carried** — the routines between the content API's 2020-01-09 payload and DT/AM 15 of 2025 are sourced rather than carried: all 22 archived content-API payloads of 2021-2024 reproduce them to the minute, and SGX's server-rendered product pages corroborate DT/AM 50 of 2024 routine by routine across the content API's 2024-10-07 to 2025-04-07 capture gap.

> Shared module. The narrative for [`sgx_equity_index.rs`](../../src/calendar/schedules/futures/international/sgx_equity_index.rs)
> lives in [`sgx_equity_index_japan`](sgx_equity_index_japan.md#module-narrative-moved-from-srccalendarschedulesfuturesinternationalsgx_equity_indexrs-on-2026-09-12-utc).
> Sibling identities: [`sgx_equity_index_japan`](sgx_equity_index_japan.md), [`sgx_equity_index_china`](sgx_equity_index_china.md).
