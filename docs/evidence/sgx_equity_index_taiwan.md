<!-- SPDX-License-Identifier: MIT-0 -->

# `sgx_equity_index_taiwan` — evidence

- **Kind:** `MarketHoursKey`
- **Owner module:** [`sgx_equity_index_more.rs`](../../src/calendar/schedules/futures/international/sgx_equity_index_more.rs)
- **Source sets:** [`APAC-SGX-DERIVATIVES`](../schedules/sources.md#apac-sgx-derivatives)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

**Gap: executable** — the family's first era rests on a captured catalogue payload rather than a dated hours document, and the first trading week is under-reported. FTSE Taiwan suite (`TWN`, `MTWN`, `TWNO`, `CTWN`/`PTWN`). Current T 08:45–13:45 and T+1 14:00 sessions sourced. Two eras: the 14:15 T+1 open from 2020-07-20, then 14:00 from 2025-04-07 on DT/AM 15 of 2025. **This family's boundary is its launch, not a calendar edition, and it is five months earlier than this crate previously recorded.** SGX announced the contract on 1 July 2020 and stated a launch of 20 July 2020, and SGX's own content API prints "SGX FTSE Taiwan Index Futures" (`TWN`) with T 08:45–13:45, T+1 14:15–05:15, the Pre-Closing/Non-Cancel pair 13:45–13:50 and the two opening routines 08:30–08:45 and 14:05–14:15, identically in its captures of 15 July, 4 November and 6 December 2020; the same endpoint captured 2 June 2020 lists no FTSE Taiwan contract, so SGX's own catalogue brackets the listing between those dates. Dates before 2020-07-20 are sessionless because the contracts did not exist, not because nothing survives: ten SGX hours-bearing artifacts from 2013-08-20 to 2020-01-09 list only the MSCI predecessors `TW` and `TWO` and the NTR sibling `NTW`, and `TWN` appears in the 2019 and 2020 editions only as the holiday country code for Taiwan (TWSE), which SGX renamed `TAI` in the 2021 edition once `TWN` became a product code. The predecessor's daytime bounds were identical and its T+1 open moved over the years; matching times are not evidence that the modelled contract existed, and that history belongs to a key this crate does not have. The 2021 calendar edition, previously this row's boundary, is now corroboration; all 22 archived content-API payloads of 2021–2024 and SGX's product pages either side of 2024-11-04 reproduce the launch grid to the minute, and DT/AM 15's appendices restate every routine window as unchanged or current. DT/AM 50 of 2024 names no Taiwan contract. **Partial** because the boundary's grid rests on the content API's captured payload rather than a document with its own date (mitigated by its byte-identical repetition across four months, two captures post-dating the launch, and DT/AM 15's restatement), because 15–19 July 2020 is reported closed while SGX's catalogue already carried the grid, and because the launch day is stated in the Chinese rendering of SGX's release (the English page is now an empty shell), corroborated by SGX's market update of 27 July 2020 reporting the first week of trading. `TWNO` trades continuously to 13:50 where `TWN` auctions 13:45–13:50 and is not separately modelled.

## Revision rows

- 2020-07-20 — T1 — SGX FTSE Taiwan launch, 20 July 2020; grid from the SGX content API, capture 2020-07-15 — launch grid T 08:45-13:45, T+1 14:15-05:15, Pre-Closing/Non-Cancel 13:45-13:50 and opening routines 08:30-08:45 and 14:05-14:15 — the launch day is T1 from SGX's release, the grid T2 from SGX's own content API.
- 2025-04-07 — T1 — SGX-DT Circular DT/AM 15 of 2025 — T+1 open moves to 14:00; DT/AM 50 of 2024 names no Taiwan contract.

## Sources

- <https://www.sgx.com/derivatives/products/nikkei225futuresoptions> — SGX Nikkei 225 futures and options product page.
- <https://www.sgx.com/derivatives/products/twnfc> — SGX FTSE Taiwan Index Futures product page.
- <https://www.sgx.com/asia-simplified/equity-derivatives> — SGX Asia simplified equity derivatives overview.
- <https://web.archive.org/web/20201030164207id_/https://www.sgx.com/zh-hans/media-centre/20200701-sgx-introduce-sgx-ftse-taiwan-index-futures> — SGX launch release of 1 July 2020, Chinese rendering — states the 20 July 2020 launch; the English page is now an empty shell.
- <https://web.archive.org/web/20200602051032id_/https://api2.sgx.com/content-api?queryId=00c0b9e1c305ecf3e85c714c61d2ecbe0d05faef%3Aderivatives_products_list&variables=%7B%22limit%22%3A10000%2C%22lang%22%3A%22EN%22%7D> — SGX content API, capture 2020-06-02 — MSCI Taiwan only, no FTSE Taiwan contract.
- <https://web.archive.org/web/20200715061901id_/https://api2.sgx.com/content-api?queryId=c23ca146c2d59c41265ea30280b202dbdaaaae81%3Aderivatives_products_list&variables=%7B%22limit%22%3A100%2C%22offset%22%3A0%2C%22lang%22%3A%22EN%22%7D> — SGX content API, capture 2020-07-15 — the FTSE Taiwan suite with its grid and routines.
- <https://api2.sgx.com/sites/default/files/2021-01/SGX%20Derivatives%20Trading%20Calendar%202021.pdf> — SGX Derivatives Trading Calendar 2021 — the first calendar listing the suite, now corroboration rather than the boundary.
- <https://api2.sgx.com/sites/default/files/2026-01/SGX%20Calendar%202026_2.pdf> — SGX Calendar 2026 — the current trading-hours table.
- <https://www.citicsf.com.hk/attachment?aid=95&uid=a1207308-0e3a-4a16-a869-a4d1b808a2b3> — SGX-DT Circular DT/AM 15 of 2025, served through a member mirror carrying SGX's own document metadata.

## Gaps and residual risks

- **executable** — the launch era's grid rests on SGX's content-API payload (a T2 machine channel) rather than a document with its own date. Mitigated by its byte-identical repetition across four months, two captures post-dating the launch and DT/AM 15's restatement of every routine window as unchanged or current. Closing condition: an SGX document with its own date stating the launch grid. Dormant identity, so recorded here rather than opened as an issue.
- **executable** — 15-19 July 2020 is reported closed while SGX's own catalogue already carried the grid, so the first trading week is under-reported.
- **launch day channel** — the launch day is stated in the Chinese rendering of SGX's release, corroborated by SGX's market update of 27 July 2020 reporting the first week of trading.
- **sessionless by fact, not by absence** — dates before 2020-07-20 are sessionless because the contracts did not exist: ten SGX hours-bearing artifacts from 2013-08-20 to 2020-01-09 list only the MSCI predecessors `TW` and `TWO` and the NTR sibling `NTW`, and `TWN` appears in the 2019 and 2020 editions only as the holiday country code for Taiwan, which SGX renamed `TAI` in the 2021 edition once `TWN` became a product code. The predecessor's history belongs to a key this crate does not have.
- **not modelled** — `TWNO` trades continuously to 13:50 where `TWN` auctions 13:45-13:50 and is not separately modelled.

> Anchor identity for [`sgx_equity_index_more.rs`](../../src/calendar/schedules/futures/international/sgx_equity_index_more.rs), which is shared with [`sgx_equity_index_ntr_usd`](sgx_equity_index_ntr_usd.md).
> Its module narrative below is the one authoritative copy; each sharer's
> file links to it rather than duplicating it.

## Module narrative (moved from src/calendar/schedules/futures/international/sgx_equity_index_more.rs on 2026-09-12 UTC)

T session trades continuously 08:45-13:45; the T+1 session reopens at 14:00
and runs to 05:15 the next calendar day, so it is encoded as a wrapping rule.
The Friday T+1 leg ends Saturday 05:15 and there is no Sunday session, so
both rules are Monday-Friday. SGX describes the combined result as "more than
20 hours of trading across Asia, Europe and U.S. hours", which the 14:00
through 05:15 wrap plus the daytime session reproduces.

https://www.sgx.com/derivatives/products/twnfc
https://www.sgx.com/asia-simplified/equity-derivatives
https://api2.sgx.com/sites/default/files/2026-01/SGX%20Calendar%202026_2.pdf

---

The two opening routines, "Pre - Opening: 8.30 am - 8.43 am / Non - Cancel:
8.43 am - 8.45 am" and the T+1 "Pre - Opening: 1.55 pm - 1.58 pm / Non -
Cancel: 1.58 pm - 2.00 pm", each contiguous pair merged into one window. The
options variant publishes a single "Order Cancellation" window over the same
spans - 08:30-08:45 and 13:55-14:00 - so these windows cover both. Neither
matches: the opening matches land on the 08:45 and 14:00 session opens that
already begin `regular` windows, so both windows are `order_entry`.

---

SGX EQUITY-INDEX HISTORY. The evidence, the calendar editions, the dated
cutovers, the one undated move and how it is served are recorded once in the
`sgx_equity_index::history` module; that note governs these two families
exactly as it governs the other three. In short: each family serves its
sourced states from its own knowledge boundary - Taiwan from its 2020-07-20
launch, NTR (USD) from the 2018 (Apr) calendar edition - with the 04:45 ->
05:15 T+1 close from Monday 2019-11-11 on SGX's own change log, and from
2025-04-07 the current grid applies on the authority of SGX-DT
Circular DT/AM 15 of 2025, which pulled both T+1 opens fifteen minutes
earlier. Routines are sourced for every era here from SGX's content API,
which states each family's Pre-Opening/Non-Cancel/Pre-Closing windows
(captures 2019-02-04, 2019-06-11, 2020-01-09 and 2020-07-15).

https://api2.sgx.com/sites/default/files/2025-01/SGX%20Calendar%202025.pdf
https://api2.sgx.com/sites/default/files/2025-07/DT%20Trading%20Calendar%202025%20%28updated%2031%20Jul%202025%29.pdf
https://api2.sgx.com/sites/default/files/2026-01/SGX%20Calendar%202026_2.pdf
https://www.citicsf.com.hk/attachment?aid=95&uid=a1207308-0e3a-4a16-a869-a4d1b808a2b3

---

2018-04-16 to 2019-11-10: the suite's first listing. The 2018 (Apr) calendar
edition (PDF created 2018-04-11) prints thirteen NTR (USD) rows, every one
"7.25am to 6.30pm / 7.00pm to 4.45am", and the 2019 edition repeats them;
neither 2017 portal table lists the suite. Keyed to the Monday after the
edition's own creation date rather than to its edition year: SGX's product
change log records "Change of Trading Hours for EM and NTR suite" in an
entry issued 2017-12-29 with no day, so a 1 January key would carry the grid
across an undated change with no second state to intersect, and the row
creates a wrapping overnight close, which the history note explains is why
it lands on a Monday. Routines as above, from the content API's 2019-02-04
payload; the change log's entry issued 2019-08-14, v6.5, "Editorial change
Contracts_data: (T) session Closing hours and LTD Last Trade Time to
6:35pm" for EM, NAU, NCH, NEA and the rest of the suite, restates the
closing routine's end (18:30 close, 18:30-18:35 pre-closing and non-cancel)
that the 2019 payloads already print, and names no day.

---

https://api2.sgx.com/sites/default/files/2025-07/DT%20Trading%20Calendar%202025%20%28updated%2031%20Jul%202025%29.pdf
TAIWAN'S KNOWLEDGE BOUNDARY IS ITS OWN LAUNCH, NOT AN EDITION. This family's
contracts do not exist in the 2020 calendar edition, which lists only the
MSCI Taiwan predecessors - "SGX MSCI Taiwan Index Futures" (TW), its options
(TWO) and its NTR (USD) sibling (NTW) - and the 2021 edition is the first
calendar to list "SGX FTSE Taiwan Index Futures" under TWN. But an edition is
one channel: SGX's own media release of 1 July 2020 states the launch day
(20 July 2020), and SGX's content API lists "SGX FTSE Taiwan Index Futures"
with its full grid and routines on 2020-07-15 while its 2020-06-02 payload
still carries only MSCI Taiwan. So the family's sourced history starts on its
stated launch day, 2020-07-20, on the grid the content API prints, and the
months between the launch and the 2021 edition are no longer sessionless.

The predecessor's hours were identical (T 08:45-13:45, T+1 14:15-05:15), so
starting at the 2020 edition would serve the right *times*. It would still be
wrong: this profile is scoped to the FTSE suite, and reporting those
contracts open before 20 July 2020 asserts a product SGX had not yet listed.
Dates before the launch are sessionless, which is exact rather than
conservative: the family was not trading.

Launch release (Chinese rendering; the English page is now an empty shell),
the content API's 2020-06-02 payload (MSCI Taiwan only), its 2020-07-15
payload (the FTSE suite with its grid and routines), the 2021 edition that
first lists the suite in a calendar, and DT/AM 15's appendices:
https://web.archive.org/web/20201030164207id_/https://www.sgx.com/zh-hans/media-centre/20200701-sgx-introduce-sgx-ftse-taiwan-index-futures
https://web.archive.org/web/20200602051032id_/https://api2.sgx.com/content-api?queryId=00c0b9e1c305ecf3e85c714c61d2ecbe0d05faef%3Aderivatives_products_list&variables=%7B%22limit%22%3A10000%2C%22lang%22%3A%22EN%22%7D
https://web.archive.org/web/20200715061901id_/https://api2.sgx.com/content-api?queryId=c23ca146c2d59c41265ea30280b202dbdaaaae81%3Aderivatives_products_list&variables=%7B%22limit%22%3A100%2C%22offset%22%3A0%2C%22lang%22%3A%22EN%22%7D
https://api2.sgx.com/sites/default/files/2021-01/SGX%20Derivatives%20Trading%20Calendar%202021.pdf
https://www.citicsf.com.hk/attachment?aid=95&uid=a1207308-0e3a-4a16-a869-a4d1b808a2b3
Evidence: docs/evidence/sgx_equity_index_taiwan.md

---

The widest of the five SGX equity-index grids: the T session trades
continuously 07:25-18:30, and the T+1 session reopens at 18:45 and runs to
05:15 the next calendar day, so it wraps. SGX's Calendar 2026 lists this
whole series uniformly as "7:25am 6:30pm 6:45pm 5:15am" (T start, T end, T+1
start, T+1 end), which is why one profile covers the entire NTR (USD) and
global-index family rather than one profile per contract code.

THE SUITE'S MEMBERSHIP GREW; ITS GRID DID NOT. The codes listed above are
today's. The 2018 (Apr) edition is the first to list the suite - thirteen
NTR (USD) rows including NSG, one of the two codes this key names, all on
one 07:25-18:30 / 19:00-04:45 pair; neither 2017 portal table has an NTR
row, and SGX's launch release of 12 June 2017 names four contracts and no
hours. NSP joins at the 2020 edition, which carries an MSCI-branded suite
(NJP, NTW, NSP); the FN* series appears from the 2021 edition and the MCN*
series from the 2024 one. Every edition puts whichever contracts it lists on
the identical pair, and NSG is present in all of them, so the grid this key
models is continuously sourced from the 2018 (Apr) edition. That is the difference from the FTSE Taiwan suite below, whose
boundary is a launch day rather than a first listing.

https://www.sgx.com/derivatives/products/sgxsimsci
https://api2.sgx.com/sites/default/files/2026-01/SGX%20Calendar%202026_2.pdf

---

Three rows: the knowledge boundary at the 2018 (Apr) edition, the 2019-11-11
row that carries the 05:15 close on the day SGX's change log states, then the
current grid on the effective day
stated by SGX-DT Circular DT/AM 15 of 2025, which moved this family's T+1
open from 19:00 to 18:45. Partial because the family traded from its 12 June
2017 launch with hours no artifact read states until the 2018 (Apr) edition.

The 2018 (Apr) edition that first lists the suite, the 2019 edition, the
content API payloads that state the routines (2019-02-04 at 04:45,
2020-01-09 at 05:15), the 2020 edition, the change log that dates the
05:15 close, and the current editions and circular:
https://api2.sgx.com/sites/default/files/2018-05/SGX%20Derivatives%20Trading%20Calendar%202018%20%28Apr%29.pdf
https://api2.sgx.com/sites/default/files/2019-01/2019%20DT%20Calendar.pdf
https://web.archive.org/web/20190204200905id_/https://api2.sgx.com/content-api?queryId=9756cc24703868bca7da492a8e1aebd1268eaf70%3Aderivatives_products_list&variables=%7B%22limit%22%3A10000%2C%22lang%22%3A%22EN%22%7D
https://web.archive.org/web/20200109051211id_/https://api2.sgx.com/content-api?queryId=ef44c5f861fc84577240761863bf1f842f189d9f%3Aderivatives_products_list&variables=%7B%22limit%22%3A10000%2C%22lang%22%3A%22EN%22%7D
https://api2.sgx.com/sites/default/files/2020-01/SGX%20Derivatives%20Trading%20Calendar%202020.pdf
https://api2.sgx.com/sites/default/files/2026-08/Derivatives+Products+Description+v17.6%20eff%2020260824,%2020260907.zip
https://api2.sgx.com/sites/default/files/2026-01/SGX%20Calendar%202026_2.pdf
https://api2.sgx.com/sites/default/files/2025-07/DT%20Trading%20Calendar%202025%20%28updated%2031%20Jul%202025%29.pdf
https://www.citicsf.com.hk/attachment?aid=95&uid=a1207308-0e3a-4a16-a869-a4d1b808a2b3
Evidence: docs/evidence/sgx_equity_index_ntr_usd.md
