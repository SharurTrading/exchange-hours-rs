<!-- SPDX-License-Identifier: MIT-0 -->

# `sgx_equity_index_japan` — evidence

- **Kind:** `MarketHoursKey`
- **Owner module:** [`sgx_equity_index.rs`](../../src/calendar/schedules/futures/international/sgx_equity_index.rs)
- **Source sets:** [`APAC-SGX-DERIVATIVES`](../schedules/sources.md#apac-sgx-derivatives)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

**Gap: executable** — one undated move falls inside an era where trades print, and it is served as the intersection of the states sourced around it. Nikkei 225 suite (`NK`, `NS`, `NU`, `NC`, `NR`, `ND`, `EJP`, `EJRT`); the modelled clock is the NK/NU index-futures grid, and `NKO`'s continuous T phase ends five minutes later in every era and is not modelled, as `FCHO` is not on the China grid. **Six eras, five of them dated.** From the January-2010 floor the key serves the intersection of the two oldest states SGX published: its pre-portal specification page captured 2009-03-08 ("Opening 7.45 am - 2.25 pm", "Pre-Closing 2.25 pm- 2.29 pm / Non-Cancel Period 2. 29 pm - 2.30 pm", T+1 "Pre -Opening 3.15 pm - 3.28 pm / Non -Cancel Period 3.28 pm - 3.30pm / Opening 3.30 pm - 10.55 pm", T pre-open "7.30am -7.43 am" / "7.43am -7.45 am") and the portal's Trading Hours table captured 2013-08-20 (07:45–14:25 / 15:15–02:00, routines excluded by footnote). The T session and its 14:25–14:30 closing routine are identical in both; the T+1 leg moved (open 15:30 → 15:15, close 22:55 → 02:00) on days no SGX artifact states, so the floor serves T+1 15:30–22:55 with 15:15–15:30 as order entry (a queue in 2009, open in 2013) and the T pre-open queue 07:30–07:45, whose 07:45 anchor never moved. The 2009 page is below the floor, but the changes it bounds fall inside the audit window; this is the timeline baseline under the carry-back convention and asserts no revision row. From Monday 2013-08-26, the 2013 table's state — T+1 15:15–02:00 — keyed to the Monday after the capture because it creates a wrapping overnight close (22:55 the same day to 02:00); its T+1 queue is withheld (length unstated, anchor moved). From Monday 2017-07-10, T 07:30–14:25 and T+1 14:55–04:45, the grid the portal printed byte-identically on 2017-07-05 and 2017-09-27, with routines from SGX's content API (capture 2019-02-04: Pre-Opening 7.15–7.28 / Non-Cancel 7.28–7.30; Pre-Closing 2.25–2.29 / Non-Cancel 2.29–2.30; T+1 Pre-Opening 2.45–2.53 / Non-Cancel 2.53–2.55). That row is a knowledge boundary keyed to the Monday after the capture, not a cutover: the move itself is undated, SGX's product change log brackets it into 2016 (entries issued 2016-04-22 and 2016-07-15 without a day; #66), and a mid-week key would have reported the previous evening's leg running past the 02:00 close it opened under. From Monday 2019-11-11 the same grid with the 05:15 close, on SGX's own Derivatives Products Description change log (entry issued 2019-10-07, v6.9: "Effective 11 Nov:" / "(T+1) session Closing hours to 5:15am all T+1 traded contracts"), confirmed by the 2020 edition and the content API's 2020-01-09 payload, which states every routine inside that row's interval. From Monday 2024-11-04, T 07:30–14:55 and T+1 15:25–05:15 with Pre-Closing 14:55–15:00 and T+1 Pre-Opening 15:15–15:25, on SGX-DT Circular DT/AM 50 of 2024 (9 September 2024, "with effect from Monday, 4 November 2024, the T session trading hours for SGX Nikkei derivatives and SGX FTSE Blossom Japan Index Futures will be extended by 30 minutes", enumerating NK, NKO, NR, ND, NU, NS, NC, EJRT and EJP with a Current/Revised table of every routine) — read from a verbatim member mirror (Fubon Futures, via the web archive) carrying SGX's own document metadata, the channel DT/AM 15 came through (#62). From 2025-04-07 the current grid on DT/AM 15 of 2025. **The one undated move** is the 02:00 → 04:45 close and 07:45 → 07:30 open between the 2013 table and the 2017 captures, bounded above by the 2017-07-05 capture and below by nothing SGX states to the day (#66). **The 2019-11-11 day comes from SGX's own dated change log**, admitted as a primary source under the convention `AGENTS.md` records (operator-authored and issue-dated in the file, session language, calibrated: its "(eff 4 Nov)" and "(eff 7 Apr)" entries match DT/AM 50 and DT/AM 15 to the day); the one interpretive step — a header row scoping the items below it inside one file-encoded entry — is SGX's own convention in that column, and the day sits inside the SGX-artifact bracket (content API 2019-06-21 at 04:45; api2 2019-11 and 2019-12 factsheets at 05:15; Annual Report 2020: "November 2019"). The channel was ruled out on 2026-09-05 as cell-border formatting and admitted on 2026-09-06 once the OOXML showed the partition is the document's own structure (#45). **Residual risks, stated.** The T+1 close moved 22:55 → 02:00 between the 2009 page and the 2013 table on days SGX does not state (third-party press: 01:00 from 2010-01-11, 02:00 from 2010-08-30, inadmissible for a row), so the floor row under-reports the T+1 leg by up to three hours to 2013-08-25 and never over-reports it; SGX's 2016 calendar edition, served verbatim by KGI Futures and "accurate as of 24 December 2015", witnesses the 2013 grid at that date, so the undated move is bracketed into (2015-12-24, 2017-07-05) and only 2016 to mid-2017 is unwitnessed (the 2014, 2015 and later-2016 editions are unrecovered); member notices, inadmissible for a row, put the Nikkei T open at 07:30 from 2016-07-11 and the market-wide revision at the Titan launch of Monday 2016-11-14, citing DT/AM 80 of 2016 Appendix 1, which no reachable copy holds (#66); the USD and Mini Nikkei contracts closed 14:30 with no pre-closing block in 2009, and the Mini Nikkei and Dividend Point contracts ran their own later closes before joining the NK grid (by 2017-07-05 and the 2018 edition respectively). The 21 September 2017 "Change of Trading Hours" newsletter is not this family's move: captures straddle it and are identical, and the change log's entry for it (issued 2017-10-05) names no day. Routines between the 2020 payload and DT/AM 50 are sourced, not carried: all 22 archived content-API payloads of 2021–2024 reproduce them to the minute, and SGX's server-rendered product pages corroborate DT/AM 50 routine by routine across the content API's 2024-10-07 to 2025-04-07 capture gap (`?cc=NK` at 2024-10-07 before, `?cc=NU` at 2024-11-14 after).

## Revision rows

- 2013-08-26 — T1 — SGX portal Trading Hours table, capture 2013-08-20, keyed to the Monday — T+1 becomes 15:15-02:00; keyed to the Monday after the capture because the row creates a wrapping overnight close, and the T+1 queue is withheld.
- 2017-07-10 — T1 — SGX derivatives Trading Hours page, captures 2017-07-05 and 2017-09-27 — T 07:30-14:25 and T+1 14:55-04:45 with routines from SGX's content API (capture 2019-02-04); a knowledge boundary keyed to the Monday, not a cutover.
- 2019-11-11 — T1 — SGX Derivatives Products Description change log v6.9: Effective 11 Nov, T+1 close 05:15 — T+1 close moves to 05:15, confirmed by the 2020 calendar edition and the content API's 2020-01-09 payload.
- 2024-11-04 — T1 — SGX-DT Circular DT/AM 50 of 2024 — T extended 30 minutes to 07:30-14:55 and T+1 15:25-05:15, with Pre-Closing 14:55-15:00 and T+1 Pre-Opening 15:15-15:25.
- 2025-04-07 — T1 — SGX-DT Circular DT/AM 15 of 2025 — the current grid.

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
- <https://web.archive.org/web/20090308012135id_/http://sgx.com:80/psv/derivatives/futures_options/equity_index/SGX_Nikkei_225_Index.shtml> — SGX pre-portal psv contract-specification page for the Nikkei 225 Index, capture 2009-03-08 — state P: 07:45-14:25 / 15:30-22:55, with "Pre -Opening 7.30am -7.43 am / Non -Cancel Period 7.43am -7.45 am" and "Pre-Closing 2.25 pm- 2.29 pm / Non-Cancel Period 2. 29 pm - 2.30 pm". Below the January-2010 floor, and used only for the intersection the floor row serves — T1 through a verbatim public mirror.
- <https://web.archive.org/web/20241007180652id_/https://www.sgx.com/derivatives/products/nikkei225futuresoptions?cc=NK> — SGX server-rendered Nikkei product page, `cc=NK`, capture 2024-10-07 — still prints "Opening : 7.30 am - 2.25 pm ... Opening : 2.55 pm - 5.15 am", the pre-2024-11-04 routines, across the content API's 2024-10-07 to 2025-04-07 capture gap — T1 through a verbatim public mirror.
- <https://web.archive.org/web/20241114152443id_/https://www.sgx.com/derivatives/products/nikkei225futuresoptions?cc=NU> — SGX server-rendered Nikkei product page, `cc=NU`, capture 2024-11-14 — prints DT/AM 50's Revised column routine by routine, corroborating the 2024-11-04 row — T1 through a verbatim public mirror.
- <https://web.archive.org/web/20241114183232id_/https://www.fubon.com/futures/wcm/home/bulletin/bulletin_20240912_137396/SGXChange.pdf> — SGX-DT Circular DT/AM 50 of 2024, "Extension of T-session for SGX Japan Derivatives and Intraday Margin Cycle 2 Timing Change", 9 September 2024, signed Leno Lee, SVP Trading and Clearing Services, on Singapore Exchange Derivatives Trading Limited letterhead — "with effect from Monday, 4 November 2024", listing NK, NKO, NR, ND, NU, NS, NC, EJRT and EJP with a Current to Revised table of every routine. Served through a verbatim member mirror (Fubon Futures) whose file carries SGX's own Word metadata (created and last saved 2024-09-09 17:53 Singapore time, the circular's dateline) — T1 through a verbatim member mirror. It names no China, Singapore, Taiwan or NTR contract, which is why only the Japan key splits there.

## Gaps and residual risks

- **executable** — the 02:00 to 04:45 close and the 07:45 to 07:30 open moved between the 2013 portal table and the 2017 captures on a day no SGX artifact states. The row serves the intersection and keys the knowledge boundary to Monday 2017-07-10.
- **executable** — the pre-2017 move is undated. SGX's own Derivatives Products Description change log brackets it into 2016 (entries issued 2016-04-22 and 2016-07-15 carry no day; #66), the 2016 calendar edition served verbatim by KGI Futures and "accurate as of 24 December 2015" witnesses the older grid at that date, and the 2017-07-05 portal capture witnesses the newer one, so the move is bracketed to (2015-12-24, 2017-07-05) and only 2016 to mid-2017 is unwitnessed (the 2014, 2015 and later-2016 editions are unrecovered). Member notices, inadmissible for a row, put the market-wide revision at the Titan launch of Monday 2016-11-14 citing DT/AM 80 of 2016 Appendix 1, which no reachable copy holds. Closing condition: a reachable copy of DT/AM 80 of 2016 or another SGX artifact stating the day. Dormant identity, so recorded here rather than opened as an issue.
- **executable** — the T+1 close moved 22:55 to 02:00 between the 2009 specification page and the 2013 portal table on days SGX does not state; third-party press puts it at 01:00 from 2010-01-11 and 02:00 from 2010-08-30, which is T4 and inadmissible for a row. The floor row therefore under-reports the T+1 leg by up to three hours to 2013-08-25 and never over-reports it.
- **interpretive step, recorded** — the 2019-11-11 day comes from SGX's own dated product change log, admitted under the convention AGENTS.md records: operator-authored and issue-dated in the file, in session language, and calibrated (its "(eff 4 Nov)" and "(eff 7 Apr)" entries match DT/AM 50 and DT/AM 15 to the day). The one interpretive step — a header row scoping the items below it inside one file-encoded entry — is SGX's own convention in that column, and the day sits inside the SGX-artifact bracket (content API 2019-06-21 at 04:45; api2 2019-11 and 2019-12 factsheets at 05:15; Annual Report 2020: "November 2019"). The channel was ruled out on 2026-09-05 as cell-border formatting and admitted on 2026-09-06 once the OOXML showed the partition is the document's own structure (#45).
- **channel** — DT/AM 50 of 2024 was read from a verbatim member mirror (Fubon Futures, via the web archive) carrying SGX's own document metadata, admissible as T1 under LAW-PUBLIC-SOURCES; the channel DT/AM 15 came through (#62).
- **not modelled** — `NKO`'s continuous T phase ends five minutes later in every era and is not modelled, as `FCHO` is not on the China grid. The USD and Mini Nikkei contracts closed 14:30 with no pre-closing block in 2009, and the Mini Nikkei and Dividend Point contracts ran their own later closes before joining the NK grid (by 2017-07-05 and the 2018 edition respectively).
- **ruled out** — the 21 September 2017 "Change of Trading Hours" newsletter is not this family's move: captures straddle it and are identical, and the change log's entry for it (issued 2017-10-05) names no day.
- **sourced, not carried** — the routines between the content API's 2020-01-09 payload and DT/AM 15 of 2025 are sourced rather than carried: all 22 archived content-API payloads of 2021-2024 reproduce them to the minute, and SGX's server-rendered product pages corroborate DT/AM 50 of 2024 routine by routine across the content API's 2024-10-07 to 2025-04-07 capture gap.

> Anchor identity for [`sgx_equity_index.rs`](../../src/calendar/schedules/futures/international/sgx_equity_index.rs), which is shared with [`sgx_equity_index_china`](sgx_equity_index_china.md), [`sgx_equity_index_singapore`](sgx_equity_index_singapore.md).
> Its module narrative below is the one authoritative copy; each sharer's
> file links to it rather than duplicating it.

## Module narrative (moved from src/calendar/schedules/futures/international/sgx_equity_index.rs on 2026-09-12 UTC)

Two executable phases per trade date. The T session trades continuously
07:30-14:55; the T+1 (night) session reopens at 15:10 and runs to 05:15 the
following calendar day, so it is encoded as a wrapping rule. The Friday T+1
session therefore ends Saturday 05:15 and no Sunday session exists, which is
why both rules are Monday-Friday. The 14:55-15:00 closing routine matches at
a single price rather than trading continuously, so it is modelled as an
extended phase, not as part of the continuous T session.

https://www.sgx.com/derivatives/products/nikkei225futuresoptions
https://api2.sgx.com/sites/default/files/2026-01/SGX%20Calendar%202026_2.pdf

---

WHY THESE ROWS STAY PARTIAL. Every move inside the modelled window is now
dated - two by circulars, two by SGX's own product-catalogue change log,
admitted under the convention `AGENTS.md` records - but the S0 -> A move
that the 2017-07-10 boundary bounds from above is not, and the floor eras
rest on carry-back; the `history` module records each.

DIRECTION OF THE ERROR. Before 2026-08-31 these rows carried the 2026-09-06 review's grid to
the January-2010 floor across every move, which made them the only rows in
the crate that could **over**-report. They no longer can: every undated move
is approached from the conservative side, every dated move begins on its
stated day, and every boundary that creates or lengthens a wrapping overnight close —
2013-08-26 (22:55 the same day to 02:00), 2017-07-10 and 2018-04-16 (02:00 or
sessionless to 04:45) and 2019-11-11 (04:45 to 05:15, the Monday SGX itself
chose) — falls on a Monday so no
evening leg runs past the close in force when it opened. Like every other
Partial row in this crate they err toward Closed, which is the safe
direction for an order router. Nothing remains in the other direction:
third-party press attests the T+1 close moving 22:55 -> 01:00 on
2010-01-11 and 01:00 -> 02:00 on 2010-08-30, both inadmissible for a row,
and the floor serves 22:55 to 2013-08-25, so those days are under-reported
by up to three hours too and never over-reported.

https://api2.sgx.com/sites/default/files/2026-01/SGX%20Calendar%202026_2.pdf
https://api2.sgx.com/sites/default/files/2026-08/Derivatives+Products+Description+v17.6%20eff%2020260824,%2020260907.zip
https://api2.sgx.com/sites/default/files/2025-07/DT%20Trading%20Calendar%202025%20%28updated%2031%20Jul%202025%29.pdf
Evidence: docs/evidence/sgx_equity_index_japan.md

---

T session trades continuously 09:00-16:30; the T+1 session reopens at 16:45
and runs to 05:15 the next calendar day, so it wraps. SGX's own A50 page
notes the contract "is available for trading everyday other than New Year's
Day", but that describes holiday coverage, not a weekend session: the T+1
leg still starts on a Monday-Friday trade date and the Friday leg ends
Saturday 05:15, so both rules stay Monday-Friday.

https://www.sgx.com/derivatives/products/chinaa50
https://www.sgx.com/derivatives/products/chinah50
https://api2.sgx.com/sites/default/files/2026-01/SGX%20Calendar%202026_2.pdf

---

Four rows, for the reasons recorded in the history note: the floor grid is
this key's baseline; 2013-08-26 widens the T close and creates the wrapping T+1
close on the 2013-08-20 portal table and specification; 2017-07-10 is the State-A knowledge boundary; the
2019-11-11 row, dated by SGX's change log, carries the 05:15 close and the
routines; and the current grid begins on the stated effective day of SGX-DT
Circular DT/AM 15 of 2025, which moved this family's T+1 open from 17:00 to
16:45. Partial because the S0 -> A move is undated.

https://api2.sgx.com/sites/default/files/2026-01/SGX%20Calendar%202026_2.pdf
https://api2.sgx.com/sites/default/files/2026-08/Derivatives+Products+Description+v17.6%20eff%2020260824,%2020260907.zip
https://api2.sgx.com/sites/default/files/2025-07/DT%20Trading%20Calendar%202025%20%28updated%2031%20Jul%202025%29.pdf
Evidence: docs/evidence/sgx_equity_index_china.md

---

T session trades continuously 08:30-17:20; the T+1 session reopens at 17:35
and runs to 05:15 the next calendar day, so it wraps. SGX MSCI Singapore NTR
(USD) futures (NSG, NSP) do not share this grid and are modelled separately
in the sibling module.

https://www.sgx.com/derivatives/products/sgxsimsci
https://www.sgx.com/derivatives/products/sgxsti
https://api2.sgx.com/sites/default/files/2026-01/SGX%20Calendar%202026_2.pdf

---

The two opening routines, "Pre - Opening: 8:15 am - 8:28 am / Non - Cancel:
8:28 am - 8:30 am" and "Pre - Opening: 5:30 pm - 5:33 pm / Non - Cancel: 5:33
pm - 5:35 pm", each contiguous pair merged into one window. The options
variant (CSGP) publishes a single "Order Cancellation" window over the same
spans - 08:15-08:30 and 17:30-17:35 - so these windows cover futures and
options alike. Neither matches: the opening matches land on the 08:30 and
17:35 session opens that already begin `regular` windows, so both windows are
`order_entry`.

---

Five rows: the floor grid is this key's baseline; 2013-08-26 creates the wrapping
T+1 close on the 2013-08-20 table; 2017-07-10 is the State-A knowledge
boundary; 2019-06-10 is the SiMSCI move SGX's change log dates;
the 2019-11-11 row, dated by the same log, carries the 05:15 close; and the
current grid begins on the stated effective day of SGX-DT Circular DT/AM 15
of 2025, which moved this family's T+1 open from 17:50 to 17:35. Partial
because the S0 -> A move is undated.

https://api2.sgx.com/sites/default/files/2026-01/SGX%20Calendar%202026_2.pdf
https://api2.sgx.com/sites/default/files/2026-08/Derivatives+Products+Description+v17.6%20eff%2020260824,%2020260907.zip
https://api2.sgx.com/sites/default/files/2025-07/DT%20Trading%20Calendar%202025%20%28updated%2031%20Jul%202025%29.pdf
Evidence: docs/evidence/sgx_equity_index_singapore.md

## Module narrative (moved from src/calendar/schedules/futures/international/sgx_equity_index/history.rs on 2026-10-01 UTC)

THE 2019-11-11 ROWS. The 05:15 T+1 close from the day SGX's change log
states; session bounds confirmed by the 2020 edition and routines from the
content API's 2020-01-09 payload, which states them for every family on this
grid: Japan "Pre -Opening : 7.15 am - 7.28 am / Non -Cancel : 7.28 am -
7.30 am / Opening : 7.30 am - 2.25 pm / Pre-Closing : 2.25 pm - 2.29 pm /
Non-Cancel : 2. 29 pm - 2.30 pm // Pre -Opening : 2.45 pm - 2.53 pm / Non
-Cancel : 2.53 pm - 2.55 pm / Opening : 2.55 pm - 5.15 am"; China "Pre -
Opening: 8.45 am - 8.58 am / ... Opening: 9.00 am - 4.30 pm / Pre - Closing:
4.30 pm - 4.34 pm / Non - Cancel: 4.34 pm - 4.35 pm // Pre - Opening: 4.50
pm - 4.58 pm / Non - Cancel: 4.58 pm - 5.00 pm / Opening: 5.00 pm - 5.15
am"; Singapore "Pre - Opening: 8:15 am - 8:28 am / ... Opening: 8:30 am -
5:20 pm / Pre - Closing: 5:20 pm - 5:24 pm / Non - Cancel: 5:24 pm - 5:25 pm
// Pre - Opening: 5:40 pm - 5:48 pm / Non - Cancel: 5:48 pm - 5:50 pm /
Opening: 5:50 pm - 5:15 am". Each Pre-Opening/Non-Cancel pair is one
order-entry window and each Pre-Closing/Non-Cancel pair one extended window.

---

SGX EQUITY-INDEX HISTORY.

THE CALENDAR EDITIONS. Nine published files of SGX's Derivatives Trading
Calendar — static, readable PDFs under api2.sgx.com/sites/default/files/, of
which eight are distinct documents — state these grids from 2020 on:

  edition                    Japan T / T+1        China  SiMSCI  Taiwan  NTR
  2020, 2021-01, 2021-07,
  2022-06, 2024              07:30-14:25 / 14:55  17:00  17:50   14:15   19:00
  2025-01                    07:30-14:55 / 15:25  17:00  17:50   14:15   19:00
  2025-07 (= 2025-11), 2026  07:30-14:55 / 15:10  16:45  17:35   14:00   18:45

(2025-11/DT Trading Calendar 2025.pdf is byte-identical to 2025-07/DT Trading
Calendar 2025 (updated 31 Jul 2025).pdf - verified by digest - so those two
files are one edition. The 2022 edition has no text layer and was read from
rendered pages; no 2023 edition was located.) Two earlier editions are still
served from the same host although the web archive never captured them: the
2018 (Apr) edition (PDF created 2018-04-11) and the 2019 edition ("accurate
as of 15 January 2019"). Both print the 04:45 T+1 close.

ALL FOUR MOVES INSIDE THE MODELLED WINDOW ARE DATED - TWO BY CIRCULARS, TWO BY
SGX'S OWN CHANGE LOG.

2025-04-07. SGX-DT Circular DT/AM - 15 of 2025, "Revision of T+1 Session
Trading Hours for SGX Equity Index Futures/Options, Dividend Index Futures
and United States Single Stock Futures (US SSFs)", 24 February 2025: "with
effect from Monday, [7] April 2025" the T+1 pre-open routine comes forward
ten minutes and shortens to five, with "no change to the T session trading
hours". Its Appendix A lists every affected contract's current and revised
Pre-Opening/Non-Cancel/Opening times, which are exactly the current grids.

2024-11-04. SGX-DT Circular DT/AM - 50 of 2024, "Extension of T-session for
SGX Japan Derivatives and Intraday Margin Cycle 2 Timing Change", 9
September 2024, signed Leno Lee, SVP Trading and Clearing Services, on
Singapore Exchange Derivatives Trading Limited letterhead: "with effect from
Monday, 4 November 2024, the T session trading hours for SGX Nikkei
derivatives and SGX FTSE Blossom Japan Index Futures will be extended by 30
minutes", listing NK, NKO, NR, ND, NU, NS, NC, EJRT and EJP, with a
Current -> Revised table of every routine (T Opening 7.30 am - 2.25 pm ->
2.55 pm; Pre-Closing 2.25-2.29 -> 2.55-2.59; T+1 Pre-Opening 2.45-2.53 ->
3.15-3.23, Opening 2.55 pm -> 3.25 pm - 5.15 am). It names no China,
Singapore, Taiwan or NTR contract, so only the Japan key splits there. Read
from a verbatim member mirror (Fubon Futures), whose file carries SGX's own
Word metadata (created and last saved 2024-09-09 17:53 Singapore time, the
circular's dateline) - the same channel DT/AM 15 came through, see below.

2019-06-10 AND 2019-11-11. SGX's "Derivatives Products Description" workbook,
served from api2.sgx.com, carries a dated, SGX-authored change log (sheet
read_me, header "Issue Date (mm/dd/yyyy) | Version Number | Authors | Change
Description", 167 dated entries from 2016-02-19, zero merged cells,
multi-row entries partitioned by border styles in the file). Its entry
issued 2019-05-21, v6.1, is one cell: "Amended trading hours for SGP, SGPO
and ST eff 10 Jun". Its entry issued 2019-10-07, v6.9, is five rows (r86 to
r90): one editorial item, then "Effective 11 Nov:" / "Editorial change
Contracts_mainmenu:" / "(T+1) session Closing hours to 5:15am all T+1
traded contracts" / "Editorial change session_mainmenu: SURV_INT to 5:15,
REMOVE_DAY_ORDERS to 5:20, ... CLOSE to 5:30 all T+1 traded contracts",
the header scoping the three rows under it.
Its later entries "(eff 4 Nov)" (issued 2024-11-04) and "(eff 7 Apr)"
(issued 2025-03-19) match DT/AM 50 and DT/AM 15 to the day. The log is
admitted as a primary source for those two effective days under the
convention `AGENTS.md` records: operator-authored, dated in the file,
session language, calibrated. The one interpretive step is on v6.9, where
"Effective 11 Nov:" is a header row scoping the items below it inside one
file-encoded entry - SGX's own recurring convention in that column: of its
six "Effective <day>:" headers, four share the items' cell (rows 60, 61, 62
and 79) and two stand on their own row (83 and 87). A bare day-and-month
means the occurrence of that day nearest the entry's issue date: every one
of the sheet's 195 bare day-and-month tokens resolves that way to within
eleven weeks of its issue date (-77 to +60 days), 193 of them in the issue
year, and the two that cross are both in the entry issued 2024-12-31 (r166,
"eff 6 Jan", "eff 20 Jan" - 2025). SGX spelled the year at the 2018/19 and
2019/20 turns ("eff 22 Dec 18" and "eff 21 Jan 19" issued 2018-12-18, r68;
"eff 13 Jan 2020" issued 2019-12-23, r94) but not at 2024/25, so the
warrant is the clustering, not a spelling habit. Both
days sit inside SGX-artifact brackets: the SiMSCI move between
the content API's 2019-02-04 payload (17:10 / 17:40) and its 2019-06-11
payload (17:20 / 17:50), the day after the stated 10 June; the 05:15 close
between the content API's 2019-06-21 payload (04:45) and SGX factsheets in
the api2 2019-11 and 2019-12 directories (05:15), with SGX's Annual Report
2020 saying "Our extension of trading hours in November 2019". Both stated
days are Mondays, which is why neither needs the rounding applied below.
The channel was ruled out on 2026-09-05 as "bound only by cell-border
formatting"; parsing the OOXML showed the partition is the document's own
structure, and the ruling was reversed on 2026-09-06 (#45).

BEFORE THE 2020 EDITION: THE STATES SGX PUBLISHED. All Singapore time, T and
T+1 session bounds; routines excluded from the portal tables per their own
footnote, stated on the 2009 pages.

  P   2009-02/03 SGX's pre-portal psv contract-specification pages
              (sgx.com/psv/derivatives/futures_options/equity_index/
              *.shtml, archived 2006 to 2009-03-27 and never again): Nikkei
              07:45-14:25 / 15:30-22:55; A50 09:15-11:35, 13:00-15:05 /
              15:40-22:55; MSCI S'pore and STI 08:30-17:10 / 18:15-22:55;
              MSCI Taiwan 08:45-13:45 / 14:45-22:55. Each page states both
              pre-opening routines - the T one as 13 minutes plus a
              2-minute non-cancel on every page (Nikkei "Pre -Opening
              7.30am -7.43 am / Non -Cancel Period 7.43am -7.45 am"; SiMSCI
              "Pre -Opening 8.15am -8.28 am / Non -Cancel Period 8.28am
              -8.30 am"; A50 "Pre -Opening 9.00am -9.13 am"), the T+1 one
              likewise (SiMSCI "Pre -Opening 6.00 pm – 6.13 pm / Non -Cancel
              Period 6.13 pm – 6.15pm") except the A50's ("Pre -Opening
              3.35 pm - 3.38 pm / Non -Cancel Period 3.38 pm - 3.40 pm") -
              and every page but the A50's states the same 4+1 pre-closing
              routine the later pages print (Nikkei "Pre-Closing 2.25 pm-
              2.29 pm / Non-Cancel Period 2. 29 pm - 2.30 pm"; SiMSCI
              "Pre-Closing 5.10 pm-5.14 pm / Non-Cancel Period 5.14 pm-
              5.15 pm"; MSCI Taiwan "Pre-Closing 1.45 pm - 1.49 pm / Non
              -Cancel Period 1. 49pm - 1.50 pm"). The A50 page prints none
              - its T block ends "1.00pm - 3.05pm" and "T+1 session" begins
              - which is why `SGX_CHINA_FLOOR` serves no extended phase.
              Below the January-2010 floor.
  W   ~2012   Nikkei 07:45-14:25 / 15:15-02:00; A50 09:00-15:25 / 16:10-02:00;
              MSCI S'pore 08:30-17:10 / 18:15-02:00; MSCI Taiwan 08:45-13:45 /
              14:35-02:00. An orphaned wcm/connect fragment captured
              2018-07-11 whose contract set ("S&P CNX Nifty", "FTSE Xinhua",
              "MSCI Asia Apex 50") dates its content to about 2012. Not
              uniformly older than P row by row: its Straits Times open
              (07:55) and its A50 grid without a lunch break both post-date
              February 2009, when SGX's own pages print 08:30 and the break.
  S0  2013-08-20 portal Trading Hours table: as W except A50 09:00-15:55 /
              16:40-02:00. The same page's holiday tables still print the
              A50 at 15:25, which orders W before S0. Its Notes assert a
              pre-opening routine and non-cancel period before both
              sessions of every equity-index future and state no length,
              referring to specification leaves the archive holds only for
              the A50 (captured the same day: 08:45-08:58 / 08:58-09:00,
              16:30-16:38 / 16:38-16:40). Witnessed unchanged as of
              2015-12-24 by SGX's Derivatives Trading Calendar 2016 - a
              verbatim SGX PDF (SGX cover and imprint, created 2016-08-31)
              served by KGI Futures, an SGX-DT member, that prints the S0
              grid for every family and states "All dates and information
              are accurate as of 24 December 2015". Read at its as-of
              date, never its creation date: SGX regenerated a stale
              edition eight months on.
  A   2017-07-05 and 2017-09-27 portal captures, byte-identical: Nikkei
              07:30-14:25 / 14:55-04:45; A50 09:00-16:30 / 17:00-04:45; MSCI
              S'pore 08:30-17:10 / 17:40-04:45; MSCI Taiwan 08:45-13:45 /
              14:15-04:45. No NTR row.
  B   2018 (Apr) and 2019 editions, content API 2019-01-16 and 2019-02-04:
              as A, adding NTR (USD) 07:25-18:30 / 19:00-04:45.
  B'  content API 2019-06-11: as B except SiMSCI 08:30-17:20 / 17:50-04:45.
  C   content API 2020-01-09 and the 2020 edition: as B' with 05:15.

A CAPTURE DATES THE OBSERVATION, NEVER THE STATE. State A was live on
2017-07-05 while the W fragment, a year older in content, was still being
served in 2018; SGX ran two content trees. So the 2017-07-05 capture is an
upper bound on state A's onset and can only under-date it - and SGX's
change log agrees: its entries of 2016-04-22 ("Equity Index & Dividend Index
products T/T+1 gap increased to 15mins", the fifteen-minute gap state A
leaves between each closing routine and its T+1 pre-open) and 2016-07-15
("Amendments to NK suite, CH, CHO and CN trading hours") place the move in
2016, though neither states a day (#66). The lower bound is the 2016
calendar edition's as-of date, 2015-12-24, so the S0 -> A window is
(2015-12-24, 2017-07-05), with both change-log entries, the Titan DT/DC
newsletter SGX released on 27 July 2016 and the Titan launch itself inside
it. That newsletter's subject is a later label: SGX's Titan portal index
captured 2017-06-17 lists the 27 Jul 2016 row only as "Titan DTDC
Newsletter - New Feature Overview 2", and the title "Extended Trading
Hours, Price Limits and Trade at Settlement" appears only in the 2018-12
re-upload the current index serves; the file is password-locked either
way. SGX's Futures
Trading Rules carry thirty-six dated annotations for that system cutover
("Amended on 14 November 2016" and its variants), none of them Rule 4.1.5,
which delegates hours to the contract specifications; the rulebook dates
the system, not the grid. The
21 September 2017 "Change of
Trading Hours" newsletter is not the move: captures straddle it and are
identical, and the change log's own entry for it (issued 2017-10-05, v3.3,
"Change of Trading Hours") states no day either.

HOW THE ROWS ARE KEYED. Japan, China and Singapore have members listed with
a grid in the oldest artifact, so their floor state is carried to the
January-2010 floor as each key's `select_revision` baseline - it asserts no
revision row - under the carry-back convention. That floor state is the
intersection of P and S0, not S0 alone: the P pages are below the floor,
but the changes they bound (the T+1 close 22:55 -> 02:00 on every key, the
Nikkei T+1 open 15:30 -> 15:15, the A50's lunch break and its 15:05 close)
fall inside the audit window on days no SGX artifact states, so from the
floor to the 2013 witness each key serves what both states hold - T+1 to
22:55; Nikkei T+1 from 15:30 with 15:15-15:30 as order entry, the phase P
(a queue) and S0 (open) both support; the A50 without its lunch break, to
15:05, with 09:00-09:15 as order entry for the same reason and its T+1 open
held at 17:00 (below). The 2009 pre-open queues are served where their
anchor did not move (Nikkei T 07:30-07:45; SiMSCI 08:15-08:30 and
18:00-18:15) and withheld where it did (A50 T+1, whose 15:35-15:40 sits
inside the floor's closed gap between the 15:05 T close and the 17:00 T+1
open); the Nikkei's T+1 queue 15:15-15:30 and the A50's T queue
09:00-09:15 each coincide with an order-entry window the intersection
already serves, so the carrying rule decides nothing there. The S0 state
then arrives as a dated row on all three keys, keyed to Monday 2013-08-26
rather than the Tuesday capture because it creates a wrapping overnight
close - the floor's T+1 leg ends 22:55 on its own local day; this row runs
it to 02:00 - and `AGENTS.md` routes a row that lengthens or creates one to
the Monday. Its T+1 queues part three ways. SiMSCI's 18:00-18:15 carries:
its 18:15 anchor did not move. Japan's is withheld because the portal
table states no length, the archive holds no Nikkei leaf of that day, and
the anchor moved 15:30 -> 15:15. The A50's is withheld although the leaf
captured the same day states it exactly (16:30-16:38 / 16:38-16:40, the
leaf that sources the 08:45-09:00 queue this row does serve): it anchors
to the 16:40 open the row does not serve, the T+1 open being held at 17:00
under the widen-only rule, and granting it would have to be withdrawn at
the 2017-07-10 boundary, whose queue is 16:50-17:00.
FTSE Taiwan and the NTR (USD) suite did not exist then and stay sessionless
below their own first listing. The State-A boundary is a knowledge boundary, keyed to Monday
2017-07-10 rather than the Wednesday capture: a boundary that lengthens a
wrapping overnight close, keyed mid-week, would report the previous
evening's leg running past the close in force when it opened - the running
session LAW-NO-FABRICATED-DATES says a boundary must not split - and no leg
wraps into a Monday morning. The NTR boundary moves to Monday 2018-04-16
for the same reason; the 2019-11-11 rows, which lengthen the close from
04:45 to 05:15, need no rounding because the day SGX states is a Monday, as
is 2019-06-10. A knowledge boundary may widen what is served and never
narrow it: the A50's T+1 open is sourced at 15:40, 16:10, 16:40 and 17:00
across four undated states, so 17:00 is held from the floor and only its T
close, which widens, moves at the boundaries.

ROUTINES. The calendars print session bounds only, but SGX's content API
prints every family's Pre-Opening/Non-Cancel/Pre-Closing windows, and its
2020-01-09 payload states them inside the 2019-11-11 row's own interval - so
those rows carry them on all five keys, as do the earlier eras those
payloads, the 2013 A50 leaf and the 2009 pages source. They are sourced, not
carried, through the interval to 2025: all 22 archived payloads of 2021 to
2024 (at least one per half-year, 2021-01-05 to 2024-10-07, across
rotating query hashes) reproduce the 2020-01-09 strings for every key to
the minute, and the edits SGX made to the catalogue in that span (the US
single-stock futures' T+1 session of 2023-11-27; the removal of the Nifty
rows) touch no equity-index row.
DT/AM 50 states Japan's routines on both sides of 2024-11-04, and SGX's own
server-rendered product pages corroborate it routine by routine across the
content API's 2024-10-07 to 2025-04-07 capture gap: nikkei225futuresoptions
?cc=NK at 2024-10-07 still prints "Opening : 7.30 am - 2.25 pm ... Opening
: 2.55 pm - 5.15 am" and ?cc=NU at 2024-11-14 prints the circular's Revised
column ("Opening : 7.30 am - 2.55 pm / Pre-Closing : 2.55 pm - 2.59 pm /
Non-Cancel : 2.59 pm - 3.00 pm / ... Pre-Opening : 3.15 pm - 3.23 pm /
Non-Cancel : 3.23 pm - 3.25 pm / Opening : 3.25 pm - 5.15 am"), while the
China, Singapore, Taiwan and NTR pages are identical either side of the day.
DT/AM 15 states every family's routines from 2025-04-07, and the content
API's 2025-04-07 payload prints them.

RESIDUAL RISKS, STATED. (1) The T+1 close moved from 22:55 to 02:00
between the P pages and the 2013 table on days SGX does not state;
third-party press puts 22:55 -> 01:00 on 2010-01-11 and 01:00 -> 02:00 on
2010-08-30, both inside the floor interval and inadmissible for a row. The
floor rows serve 22:55 to 2013-08-25, so they under-report the T+1 leg by
up to three hours across that interval and never over-report it - the side
the intersection convention chooses. (2) The 2014, 2015 and later-2016
calendar editions lived in a WCM store that is in neither the archive nor
the live site; the August-2016 edition survives on a member mirror and
witnesses S0 as of 2015-12-24, so only 2016 to mid-2017 is unwitnessed and
the intersection covers the sourced endpoints only. Member notices,
inadmissible for a row, date two events inside that window: the Nikkei
suite's T open 07:45 -> 07:30 "with effect from 11 July 2016" (Phillip
Futures, 1 July 2016) and the market-wide hours revision "with effect from
14 November 2016", the Titan DT/DC launch (KGI Futures and Phillip Futures,
November 2016), citing SGX Circular DT/AM 80 of 2016, Appendix 1, whose
SGX copy and mirror are both dead and unarchived. Were that appendix
recovered, the three 2017-07-10 boundaries would become a cutover keyed to
Monday 2016-11-14 and the keys would stop under-reporting about eight
months of the longer T+1 leg; until then the day is a risk, not a row.
(3) NKO's T
close is five minutes later than NK's in every era and is not modelled, as
FCHO is not on the China grid. (4) The Mini Nikkei (NS) and Dividend Point
(ND) contracts ran their own later closes in P (NU and NS to 14:30, with
no pre-closing block), W, S0 and - for ND - A before joining the NK grid
(NS by 2017-07-05, ND by the 2018 edition), and the Straits Times Index
future opened 07:55 rather than 08:30 in W before sharing the SiMSCI row
from S0 on; each family's clock is its index future's grid throughout.

CHANNELS. SGX publishes no DT/AM circular at a publicly reachable sgx.com
address (regco.sgx.com's /circulars route answers `null`; the api2 file
store is not listable), so both circulars above were read from verbatim
member-hosted copies - SGX letterhead, circular number, signatory, both
appendices - CITIC Futures International for DT/AM 15 and Fubon Futures
(via the web archive) for DT/AM 50, and KGI Futures (Singapore) for the
2016 calendar edition and, verified on DT/AM 103 of 2020, for circulars;
SGX's own Titan DT/DC portal corroborates each issue date. The retired portal's pages and SGX's pre-portal psv pages
are archived; the 2018 and 2019 calendars, the content API's payloads and
the server-rendered product pages are SGX-served.

https://api2.sgx.com/sites/default/files/2018-05/SGX%20Derivatives%20Trading%20Calendar%202018%20%28Apr%29.pdf
https://api2.sgx.com/sites/default/files/2019-01/2019%20DT%20Calendar.pdf
https://api2.sgx.com/sites/default/files/2020-01/SGX%20Derivatives%20Trading%20Calendar%202020.pdf
https://api2.sgx.com/sites/default/files/2021-01/SGX%20Derivatives%20Trading%20Calendar%202021.pdf
https://api2.sgx.com/sites/default/files/2021-07/SGX_Derivatives%20Trading%20Calendar%202021%20%28Final%20-%20Jul%29.pdf
https://api2.sgx.com/sites/default/files/2022-06/DT%20Trading%20Calendar%202022%20%28Final%29.pdf
https://api2.sgx.com/sites/default/files/2024-01/SGX%20Calendar%202024.pdf
https://api2.sgx.com/sites/default/files/2025-01/SGX%20Calendar%202025.pdf
https://api2.sgx.com/sites/default/files/2025-07/DT%20Trading%20Calendar%202025%20%28updated%2031%20Jul%202025%29.pdf
https://api2.sgx.com/sites/default/files/2026-01/SGX%20Calendar%202026_2.pdf
https://web.archive.org/web/20090308012135id_/http://sgx.com:80/psv/derivatives/futures_options/equity_index/SGX_Nikkei_225_Index.shtml
https://web.archive.org/web/20090227040521id_/http://www.sgx.com:80/psv/derivatives/futures_options/equity_index/SGX_FTSE_Xinhua_China_A50_Index.shtml
https://web.archive.org/web/20090308120909id_/http://www.sgx.com:80/psv/derivatives/futures_options/equity_index/SGX_MSCI_Singapore_Index.shtml
https://web.archive.org/web/20090220005028id_/http://sgx.com:80/psv/derivatives/futures_options/equity_index/SGX_Straits_Times_Index.shtml
https://web.archive.org/web/20130820090335id_/http://www.sgx.com/wps/portal/sgxweb/home/trading/derivatives/trading_hours_calendar
https://web.archive.org/web/20170705000242id_/http://sgx.com/wps/portal/sgxweb_ch/home/trading/derivatives/trading_hours_calendar
https://web.archive.org/web/20170927124017id_/http://www.sgx.com/wps/portal/sgxweb/home/trading/derivatives/trading_hours_calendar
https://web.archive.org/web/20190116144725id_/https://api2.sgx.com/content-api?queryId=e8c4b75927723d2bae18ec762abab178e0efcd9a%3Apage&variables=%7B%22path%22%3A%22%2Fderivatives%2Fproducts%2Fchinaa50%22%2C%22lang%22%3A%22EN%22%7D
https://web.archive.org/web/20190204200905id_/https://api2.sgx.com/content-api?queryId=9756cc24703868bca7da492a8e1aebd1268eaf70%3Aderivatives_products_list&variables=%7B%22limit%22%3A10000%2C%22lang%22%3A%22EN%22%7D
https://web.archive.org/web/20190611051800id_/https://api2.sgx.com/content-api?queryId=5adaa923edc3b334f3d4a62a324e055c4be65025%3Aderivatives_products_list&variables=%7B%22limit%22%3A10000%2C%22lang%22%3A%22EN%22%7D
https://web.archive.org/web/20200109051211id_/https://api2.sgx.com/content-api?queryId=ef44c5f861fc84577240761863bf1f842f189d9f%3Aderivatives_products_list&variables=%7B%22limit%22%3A10000%2C%22lang%22%3A%22EN%22%7D
https://web.archive.org/web/20241007180652id_/https://www.sgx.com/derivatives/products/nikkei225futuresoptions?cc=NK
https://web.archive.org/web/20241114152443id_/https://www.sgx.com/derivatives/products/nikkei225futuresoptions?cc=NU
https://web.archive.org/web/20241114183232id_/https://www.fubon.com/futures/wcm/home/bulletin/bulletin_20240912_137396/SGXChange.pdf
https://www.citicsf.com.hk/attachment?aid=95&uid=a1207308-0e3a-4a16-a869-a4d1b808a2b3
https://www.kgieworld.sg/docs/SGXDerivativesTradingCalendar2016_AUG.pdf
https://rulebook.sgx.com/rulebook/futures-trading-rules
https://www.sgx.com/titan-dt-dc-portal
https://api2.sgx.com/sites/default/files/2026-08/Derivatives+Products+Description+v17.6%20eff%2020260824,%2020260907.zip

## Module narrative (moved from src/calendar/schedules/futures/international/sgx_equity_index/eras.rs on 2026-10-01 UTC)

2019-06-10. SGX's Derivatives Products Description change log, entry
issued 2019-05-21 (v6.1): "Amended trading hours for SGP, SGPO and ST eff 10
Jun" - a single cell, no scoping step; the year is the entry's own. The
grid it moved to is the one the content API prints on 2019-06-11, the next
day (revisit 2019-06-21): "Pre - Opening: 8:15 am - 8:28 am / Non - Cancel:
8:28 am - 8:30 am / Opening: 8:30 am - 5:20 pm / Pre - Closing: 5:20 pm -
5:24 pm / Non - Cancel: 5:24 pm - 5:25 pm // Pre - Opening: 5:40 pm - 5:48
pm / Non - Cancel: 5:48 pm - 5:50 pm / Opening: 5:50 pm - 4:45 am", and the
state before it is the one the API printed on 2019-02-04 (17:10 / 17:40).

---

2017-07-10. Portal table, captures 2017-07-05 and 2017-09-27: "SGX MSCI
Singapore Index Futures | 8.30am to 5.10 pm | 5.40 pm to 4.45 am". Routines
from the content API (capture 2019-02-04: "Pre - Opening: 8:15 am - 8:28 am
/ Non - Cancel: 8:28 am - 8:30 am / Opening: 8:30 am - 5:10 pm / Pre -
Closing: 5:10 pm - 5:14 pm / Non - Cancel: 5:14 pm - 5:15 pm // Pre -
Opening: 5:30 pm - 5:38 pm / Non - Cancel: 5:38 pm - 5:40 pm / Opening: 5:40
pm - 4:45 am").

---

THE FLOOR: the intersection of SGX's 2009 specification pages and the
2013-08-20 table. The SiMSCI page captured 2009-03-08 (the STI page of
2009-02-20 is identical): "T Session: Pre -Opening 8.15am -8.28 am / Non
-Cancel Period 8.28am -8.30 am / Opening 8.30am-5.10 pm / Pre-Closing 5.10
pm-5.14 pm / Non-Cancel Period 5.14 pm- 5.15 pm // T+1 Session: Pre
-Opening 6.00 pm – 6.13 pm / Non -Cancel Period 6.13 pm – 6.15pm / Opening
6.15 pm - 10.55pm". The 2013 table: "SGX MSCI Singapore Index Futures / SGX
Straits Times Index Futures | 8.30am to 5.10pm | 6.15pm to 2.00am". Only the
T+1 close moved (22:55 -> 02:00), undated, so the floor serves T+1
18:15-22:55; both queues are stated in 2009 on opens that did not move
(the T queue 08:15-08:30 is, to the minute, what SGX still prints in 2020)
and are carried with them. The ~2012 fragment prints the same bounds for
MSCI Singapore and a 07:55 open for the Straits Times future.
https://web.archive.org/web/20090308120909id_/http://www.sgx.com:80/psv/derivatives/futures_options/equity_index/SGX_MSCI_Singapore_Index.shtml
https://web.archive.org/web/20090220005028id_/http://sgx.com:80/psv/derivatives/futures_options/equity_index/SGX_Straits_Times_Index.shtml

---

2017-07-10. Portal table, captures 2017-07-05 and 2017-09-27: "FTSE China
A50 Index Futures | 9:00 am to 4:30 pm | 5.00 pm to 4.45 am". Routines from
the content API (capture 2019-01-16, chinaa50 page: "Pre - Opening: 8.45 am
- 8.58 am / Non - Cancel: 8.58 am - 9.00 am / Opening: 9.00 am - 4.30 pm /
Pre - Closing: 4.30 pm - 4.34 pm / Non - Cancel: 4.34 pm - 4.35 pm // Pre -
Opening: 4.50 pm - 4.58 pm / Non - Cancel: 4.58 pm - 5.00 pm / Opening: 5.00
pm - 4.45 am").

---

2013-08-26. The 2013-08-20 table's state, with the routines its
specification captured the same day states ("Pre - Opening 8.45 am - 8.58
am / Non - Cancel 8.58 am - 9.00 am / Opening 9.00 am - 3.55 pm / Pre -
Closing 3.55 pm - 3.59 pm / Non - Cancel 3.59 pm - 4.00 pm"), keyed to the
following Monday because it creates a wrapping overnight close (the
floor's T+1 leg ends 22:55 the same day; this row runs it to 02:00) as well
as widening the T close; the T+1 open stays held at 17:00, so the leaf's
16:30-16:40 queue, anchored to a 16:40 open this row does not serve, is
withheld.

---

THE FLOOR: the intersection of three sourced states. SGX's pre-portal A50
specification page, captured 2009-02-27: "T session Pre -Opening 9.00am
-9.13 am / Non -Cancel Period 9.13 am -9.15 am / Opening 9.15am -11.35am /
1.00pm - 3.05pm // T+1 session Pre -Opening 3.35 pm - 3.38 pm / Non -Cancel
Period 3.38 pm - 3.40 pm / Opening 3.40pm - 10.55pm". The ~2012 fragment:
"FTSE Xinhua China A50 | 9.00 am - 3.25 pm | 4.10 pm - 2.00 am". The
2013-08-20 table: "SGX FTSE China A50 Index Futures | 9.00am to 3.55pm |
4.40pm to 2.00am" (its holiday tables still print 3.25 pm, ordering the
fragment before it). Every changeover is undated, so the floor serves what
all three hold: the T session without the 2009 lunch break's 11:35-13:00
and only to 15:05, the T+1 open held at 17:00 - the narrowest value sourced
anywhere in the undated span (15:40, 16:10, 16:40, 17:00), because a
knowledge boundary may widen but never narrow - and the T+1 close 22:55.
09:00-09:15 is a queue in 2009 and matching later, so order entry is the
phase all support; the 15:25-15:30 closing routine is closed in 2009 and
withheld.
https://web.archive.org/web/20090227040521id_/http://www.sgx.com:80/psv/derivatives/futures_options/equity_index/SGX_FTSE_Xinhua_China_A50_Index.shtml
https://web.archive.org/web/20180711020353id_/http://www.sgx.com/wps/wcm/connect/mp_en/site/trading_on_sgx/derivatives_market/derivatives_trading_hours_and_calendar/Trading+Hours?%20noCache=1531274630984.837727.133108399

---

2024-11-04. SGX-DT Circular No. DT/AM – 50 of 2024, "Extension of T-session
for SGX Japan Derivatives and Intraday Margin Cycle 2 Timing Change", 9
September 2024, signed Leno Lee, SVP Trading and Clearing Services, on
Singapore Exchange Derivatives Trading Limited letterhead: "with effect from
Monday, 4 November 2024, the T session trading hours for SGX Nikkei
derivatives and SGX FTSE Blossom Japan Index Futures will be extended by 30
minutes". Its "Revised Trading Hours" column: "Pre-Opening: 7.15 am – 7.28
am / Non-Cancel: 7.28 am – 7.30 am / Opening: 7.30 am – 2.55 pm /
Pre-Closing: 2.55 pm – 2.59 pm / Non-Cancel: 2.59 pm – 3.00 pm //
Pre-Opening: 3.15 pm – 3.23 pm / Non-Cancel: 3.23 pm – 3.25 pm / Opening:
3.25 pm – 5.15 am". Read from the verbatim member-hosted copy (Fubon
Futures, via the web archive; the file carries SGX's own document metadata,
created and last saved 2024-09-09 17:53 Singapore time), as SGX publishes no
circular at a publicly reachable address - see `history`.
https://web.archive.org/web/20241114183232id_/https://www.fubon.com/futures/wcm/home/bulletin/bulletin_20240912_137396/SGXChange.pdf

---

2017-07-10. The portal's Trading Hours table captured 2017-07-05 (08:02 SGT,
before that day's open) and again 2017-09-27, byte-identical: "SGX Nikkei
225 Index Futures | 7.30 am to 2.25 pm | 2.55 pm to 4.45 am". SGX's
content API states the routines on the same grid (capture 2019-02-04:
"Pre -Opening : 7.15 am - 7.28 am / Non -Cancel : 7.28 am - 7.30 am /
Opening : 7.30 am - 2.25 pm / Pre-Closing : 2.25 pm - 2.29 pm / Non-Cancel :
2.29 pm - 2.30 pm // Pre -Opening : 2.45 pm - 2.53 pm / Non -Cancel : 2.53 pm
- 2.55 pm / Opening : 2.55 pm - 4.45 am"). A knowledge boundary keyed to the
Monday after the capture, not a cutover: see `history`.
https://web.archive.org/web/20170705000242id_/http://sgx.com/wps/portal/sgxweb_ch/home/trading/derivatives/trading_hours_calendar
https://web.archive.org/web/20170927124017id_/http://www.sgx.com/wps/portal/sgxweb/home/trading/derivatives/trading_hours_calendar

---

2013-08-26. The 2013-08-20 table's state - T+1 15:15-02:00 - keyed to the
following Monday because it creates a wrapping overnight close - the
floor's T+1 leg closes 22:55 on its own local day - per the convention
`AGENTS.md` records ("lengthens or creates"): a capture is a knowledge
boundary, never a cutover, and a mid-week key would report Monday's leg
running past the 22:55 it opened under. The T queue is carried on its unchanged 07:45
anchor; the T+1 queue is not, because its anchor moved and the 2013 page
states no length (#65).

---

THE FLOOR: the intersection of the two oldest states SGX published for the
NK grid this key models. SGX's own pre-portal NK specification page,
captured 2009-03-08: "T
Session: Pre -Opening 7.30am -7.43 am / Non -Cancel Period 7.43am -7.45 am
/ Opening 7.45 am - 2.25 pm / Pre-Closing 2.25 pm- 2.29 pm / Non-Cancel
Period 2. 29 pm - 2.30 pm // T+1 Session: Pre -Opening 3.15 pm - 3.28 pm /
Non -Cancel Period 3.28 pm - 3.30pm / Opening 3.30 pm - 10.55 pm". The
portal's Trading Hours table captured 2013-08-20: "SGX Nikkei 225 Index
Futures / SGX USD Nikkei 225 Index Futures | 7.45am to 2.25pm | 3.15pm to
2.00am", routines excluded by its footnote. The T session bounds are
identical in both, and the closing routine is NK's alone in 2009 - SGX's
separate NU page of 2009-02-27 prints "Opening 7.45 am - 2.30 pm" with no
pre-closing block, the divergence `history` records as residual risk (4);
the 2013 table is the first of the two that puts NK and NU on one row. The
T+1 leg moved (open 15:30 -> 15:15,
close 22:55 -> 02:00) on days no SGX artifact states, so from the
January-2010 floor to the 2013 witness the key serves what both hold: T+1
15:30-22:55. The T pre-open queue 07:30-07:45 is stated in 2009 on the same
07:45 open the floor serves and carried with it; 15:15-15:30 is a queue in
2009 and matching in 2013, so order entry is the phase both support. This
is the timeline baseline: it asserts no revision row, and serves below the
floor too. Residual risk: third-party press attests 22:55 -> 01:00 on
2010-01-11 and 01:00 -> 02:00 on 2010-08-30, inside this interval and
inadmissible for a row; the intersection under-reports the T+1 leg after
those days and never over-reports it.
https://web.archive.org/web/20090308012135id_/http://sgx.com:80/psv/derivatives/futures_options/equity_index/SGX_Nikkei_225_Index.shtml
https://web.archive.org/web/20130820090335id_/http://www.sgx.com/wps/portal/sgxweb/home/trading/derivatives/trading_hours_calendar
