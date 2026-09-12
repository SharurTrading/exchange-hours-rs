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
