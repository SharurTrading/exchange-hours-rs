<!-- SPDX-License-Identifier: MIT-0 -->

# `sgx_equity_index_ntr_usd` — evidence

- **Kind:** `MarketHoursKey`
- **Owner module:** [`sgx_equity_index_more.rs`](../../src/calendar/schedules/futures/international/sgx_equity_index_more.rs)
- **Source sets:** [`APAC-SGX-DERIVATIVES`](../schedules/sources.md#apac-sgx-derivatives)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

**Gap: executable** — the family's first ten months of trading are under-reported, and the routines of its first era are carried back from a later payload. MSCI/FTSE NTR (USD) and global-index grid. Current T 07:25–18:30 and T+1 18:45 sessions sourced. **This family's sourced history now starts at the 2018 (Apr) calendar edition rather than the 2020 one** — the mirror image of the FTSE Taiwan boundary. That edition is served live from api2.sgx.com (PDF created 11 April 2018; never archived, fetched and hashed 2026-09-06) and lists fourteen rows on 07:25–18:30 / 19:00–04:45: thirteen NTR (USD) contracts including `NSG`, one of the two codes this key names, plus `EM`; `NSP` joins at the 2020 edition. The 2019 edition repeats them, and the content API (captures 2019-02-04, 2019-06-11, 2019-06-21 and 2020-01-09) states the routines the calendars exclude by footnote — T Pre-Opening 07:10–07:23 / Non-Cancel 07:23–07:25, Pre-Closing 18:30–18:34 / Non-Cancel 18:34–18:35, T+1 Pre-Opening 18:50–18:58 / Non-Cancel 18:58–19:00 — which are carried to the 2018 boundary; the 18:30–18:35 closing auction is an executable window carried back from 2019-02-04, the most aggressive claim in this row's table. **Keyed to Monday 2018-04-16**, the Monday after the edition's own creation date, not to its edition year: SGX's product change log records "Change of Trading Hours for EM and NTR suite" in an entry issued 29 December 2017 without a day, so keying at 1 January 2018 would carry the grid across an undated change with no second state to intersect, and a mid-week key on a sessionless-to-open boundary would report the family open from midnight on a leg that never opened. **Three eras**: from 2018-04-16 with the 04:45 close; from Monday 2019-11-11 with 05:15 on SGX's own change log (v6.9, issued 2019-10-07), as the Japan row records, confirmed by the 2020 edition and the 2020-01-09 payload; from 2025-04-07 the current grid on DT/AM 15 of 2025, corroborated by the change log's "(eff 7 Apr)". DT/AM 50 of 2024 names no NTR contract and does not split this row. **Partial** because 12 June 2017 — SGX's release launching the first four NTR (USD) futures — through 2018-04-15 is sessionless: both 2017 portal tables list 88 contracts and no NTR row, so nothing states the family's hours before the 2018 edition and that interval is under-reported rather than back-filled across the undated turn-of-2017 change. Routines after the 2020 payload are sourced to DT/AM 15 by all 22 archived content-API payloads of 2021–2024 and the product pages either side of 2024-11-04; the change log's entry issued 2019-08-14 (v6.5, "Editorial change Contracts_data: (T) session Closing hours and LTD Last Trade Time to 6:35pm") restates the closing routine's end the 2019 payloads print and names no day.

## Revision rows

- 2018-04-16 — T1 — SGX Derivatives Trading Calendar 2018 (Apr) edition, the first listing the NTR (USD) suite — T 07:25-18:30 and T+1 19:00-04:45 for thirteen NTR (USD) rows including NSG, plus EM; keyed to the Monday after the edition's own creation date of 11 April 2018.
- 2019-11-11 — T1 — SGX Derivatives Products Description change log v6.9: Effective 11 Nov, T+1 close 05:15 — T+1 close moves to 05:15, confirmed by the 2020 calendar edition and the content API's 2020-01-09 payload.
- 2025-04-07 — T1 — SGX-DT Circular DT/AM 15 of 2025 — the current grid, which moved the T+1 open to 18:45; DT/AM 50 of 2024 names no NTR contract.

## Sources

Retrieval dates: these sources were last opened on the row's reviewed-on date
(2026-09-06, UTC); per-source retrieval dates were not recorded before the
2026-09-12 migration and are added as each source is re-verified.

- <https://api2.sgx.com/sites/default/files/2018-05/SGX%20Derivatives%20Trading%20Calendar%202018%20%28Apr%29.pdf> — SGX Derivatives Trading Calendar 2018 (Apr) edition — PDF created 11 April 2018, never archived, fetched and hashed 2026-09-06; the first edition listing the NTR (USD) suite.
- <https://api2.sgx.com/sites/default/files/2019-01/2019%20DT%20Calendar.pdf> — SGX DT Calendar 2019 — repeats the 2018 rows.
- <https://web.archive.org/web/20190204200905id_/https://api2.sgx.com/content-api?queryId=9756cc24703868bca7da492a8e1aebd1268eaf70%3Aderivatives_products_list&variables=%7B%22limit%22%3A10000%2C%22lang%22%3A%22EN%22%7D> — SGX content API, capture 2019-02-04 — the routines the calendars exclude by footnote, T+1 close at 04:45.
- <https://web.archive.org/web/20200109051211id_/https://api2.sgx.com/content-api?queryId=ef44c5f861fc84577240761863bf1f842f189d9f%3Aderivatives_products_list&variables=%7B%22limit%22%3A10000%2C%22lang%22%3A%22EN%22%7D> — SGX content API, capture 2020-01-09 — the same routines with the 05:15 close.
- <https://api2.sgx.com/sites/default/files/2020-01/SGX%20Derivatives%20Trading%20Calendar%202020.pdf> — SGX Derivatives Trading Calendar 2020 — confirms the 05:15 close and carries the MSCI-branded suite.
- <https://api2.sgx.com/sites/default/files/2026-08/Derivatives+Products+Description+v17.6%20eff%2020260824,%2020260907.zip> — SGX Derivatives Products Description v17.6 — the operator's own dated product change log.
- <https://api2.sgx.com/sites/default/files/2026-01/SGX%20Calendar%202026_2.pdf> — SGX Calendar 2026 — the current trading-hours table, "7:25am 6:30pm 6:45pm 5:15am".
- <https://api2.sgx.com/sites/default/files/2025-07/DT%20Trading%20Calendar%202025%20%28updated%2031%20Jul%202025%29.pdf> — SGX DT Trading Calendar 2025 (updated 31 July 2025).
- <https://api2.sgx.com/sites/default/files/2025-01/SGX%20Calendar%202025.pdf> — SGX Calendar 2025.
- <https://www.sgx.com/derivatives/products/sgxsimsci> — SGX SiMSCI product page.
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

- <https://api2.sgx.com/sites/default/files/2021-01/SGX%20Derivatives%20Trading%20Calendar%202021.pdf> — SGX Derivatives Trading Calendar 2021 — T1. **(shared)**
- <https://api2.sgx.com/sites/default/files/2021-07/SGX_Derivatives%20Trading%20Calendar%202021%20%28Final%20-%20Jul%29.pdf> — SGX Derivatives Trading Calendar 2021 (Final - Jul) — T1. **(shared)**
- <https://api2.sgx.com/sites/default/files/2022-06/DT%20Trading%20Calendar%202022%20%28Final%29.pdf> — SGX DT Trading Calendar 2022 (Final) — no text layer; read from rendered pages — T1. **(shared)**
- <https://api2.sgx.com/sites/default/files/2024-01/SGX%20Calendar%202024.pdf> — SGX Calendar 2024 — T1. (No 2023 edition was located.) **(shared)**
- <https://web.archive.org/web/20190611051800id_/https://api2.sgx.com/content-api?queryId=5adaa923edc3b334f3d4a62a324e055c4be65025%3Aderivatives_products_list&variables=%7B%22limit%22%3A10000%2C%22lang%22%3A%22EN%22%7D> — SGX content API, capture 2019-06-11 — the day after the change log's stated 10 June; brackets the SiMSCI move (17:10 / 17:40 to 17:20 / 17:50) from above — T1 through a verbatim public mirror. **(shared)**
- <https://rulebook.sgx.com/rulebook/futures-trading-rules> — SGX Futures Trading Rules — thirty-six "Amended on 14 November 2016" annotations for the Titan system cutover, none of them Rule 4.1.5, which delegates hours to the contract specifications: the rulebook dates the system, not the grid. **(shared)**
- <https://www.sgx.com/titan-dt-dc-portal> — SGX Titan DT/DC portal — the operator index that corroborates each member-mirrored circular's issue date, and whose 2017-06-17 capture shows the 27 July 2016 newsletter listed only as "Titan DTDC Newsletter - New Feature Overview 2". **(shared)**

## Gaps and residual risks

- **executable** — 2017-06-12 through 2018-04-15 is reported sessionless although the family was trading. Both 2017 portal tables list 88 contracts and no NTR row, so nothing states the family's hours before the 2018 (Apr) edition, and the interval is under-reported rather than back-filled across the undated turn-of-2017 change SGX's change log records as "Change of Trading Hours for EM and NTR suite" (entry issued 29 December 2017, no day). Closing condition: an SGX artifact stating the family's hours between the 2017 launch and the 2018 edition. Dormant identity, so recorded here rather than opened as an issue.
- **executable** — the first era's routines are carried back from the content API's 2019-02-04 payload to the 2018-04-16 boundary; the 18:30-18:35 closing auction is an executable window carried in that way, the most aggressive claim in this row's table.
- **keying** — the row is keyed to Monday 2018-04-16 rather than 1 January 2018 because keying at the edition year would carry the grid across the undated turn-of-2017 change with no second state to intersect, and because a mid-week key on a sessionless-to-open boundary would report the family open from midnight on a leg that never opened.
- **membership grew, grid did not** — `NSP` joins at the 2020 edition, the FN* series at the 2021 edition and the MCN* series at the 2024 one. Every edition puts whichever contracts it lists on the identical pair and `NSG` is present in all of them, so the modelled grid is continuously sourced from the 2018 (Apr) edition.
- **editorial, not a revision** — the change log's entry issued 2019-08-14 (v6.5, "Editorial change Contracts_data: (T) session Closing hours and LTD Last Trade Time to 6:35pm") restates the closing routine's end the 2019 payloads already print and names no day.
- **sourced, not carried** — the routines between the content API's 2020-01-09 payload and DT/AM 15 of 2025 are sourced rather than carried: all 22 archived content-API payloads of 2021-2024 reproduce them to the minute, and SGX's server-rendered product pages corroborate DT/AM 50 of 2024 routine by routine across the content API's 2024-10-07 to 2025-04-07 capture gap.

> Shared module. The narrative for [`sgx_equity_index_more.rs`](../../src/calendar/schedules/futures/international/sgx_equity_index_more.rs)
> lives in [`sgx_equity_index_taiwan`](sgx_equity_index_taiwan.md#module-narrative-moved-from-srccalendarschedulesfuturesinternationalsgx_equity_index_morers-on-2026-09-12-utc).
> Sibling identities: [`sgx_equity_index_taiwan`](sgx_equity_index_taiwan.md).
