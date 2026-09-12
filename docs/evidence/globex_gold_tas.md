<!-- SPDX-License-Identifier: MIT-0 -->

# `globex_gold_tas` — evidence

- **Kind:** `MarketHoursKey`
- **Owner module:** [`metals_tas.rs`](../../src/calendar/schedules/futures/us/metals_tas.rs)
- **Source sets:** [`US-CME-GROUP`](../schedules/sources.md#us-cme-group)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

**Gap: executable** — the daily close is carried across a multi-year observation gap with no cutover asserted, and a close instant governs a window where trades print. COMEX Gold Trading at Settlement, CME Globex `GCT`, security group TG. Listed by COMEX's own Regulation 40.6 self-certification, NYMEX/COMEX Submission 10-070 with SER S-5166 attached: "the launch of TAS pricing for the active month in each of Gold Futures (Chapter 113) and Silver Futures (Chapter 112) on Globex on April 11 (for trade date April 12)" — keyed to the local opening day, Sunday 2010-04-11. The pre-launch era is a sourced closure, not an unworked gap: RA0907-4's complete Globex TAS list is the eleven codes `WST` `RTI` `BHT` `HPT` `HHT` `BBT` `CLT` `HOT` `NGT` `RBT` `RET`, and RA1001-4 / RA1002-4 of 2010-02-02 reprints a complete list with no metals at all, so the pre-2010-11-01 Rule 524.A.2 sentence that supplies the energy-TAS floor era is inapplicable and nothing is carried back to January 2010. Window: Sunday-Thursday 17:00 CT wrapping to 12:30 CT with no Friday-evening reopen, from RA1006-4's "GCT COMEX Gold ... No-Activity Periods: 1:30 p.m.- 5:45 p.m. (ET) Monday- Thursday / 1:30 p.m. (ET) Friday- 5:15 p.m. (ET) Sunday" in session language ("the end of the trading session is defined by receipt of the security status message indicating that group is closed"), with the 17:00 open from CME's metals hours page and both carried back to the launch day. Every inter-trade-date gap is 4h30m, over the four-hour limit, so it is `Closed` rather than `Maintenance`. `regular` is empty in every era and that is a sourced absence: Rule 524.A enumerates the TAS execution routes as CME Globex plus Rule 526 block trades and Rule 538 EFP/EFR and names no other; all forty-five TAS records in CME's ProductSlate read floor "-" / floorVol "0" / venues "Globex ClearPort "; the ContractSpecs API's "Open Outcry:" venue label exists — it prints one for SOFR options — and is never attached to a TAS hours line; and the Daily Bulletin carries TAS as an ex-pit volume column, never a priced product row, which already held on the 2015-03-20 pit-era edition. **The 2009-2015 pit clause does not create a regular session.** NYMEX and COMEX Rule 524.A.1 read "TAS transactions executed in the pit must be made open and competitively pursuant to the requirements of Rule 521 during the hours designated for pit trading in the particular contract" from 2009-09-14 until a 2015-03-06..2016-08-09 bracket, and pit TAS was real for gold and silver — SER S-5166 heads its sections "Traded on. CME Globex and COMEX Pit" and dates the floor launch "on the COMEX trading floor on April 12". But that route had no window of its own, CME's advisories list it under the underlying's product code while the Globex route carries the TAS root's own code, and the hours page's pit column is unreliable per row: Gold TAS carries the gold pit window at every capture, Silver TAS none at any, and Copper TAS one cell of "08:10-13:00 ET" that is CME's error for the copper pit matched-order window. Encode no open-outcry window here. **Zero executable revision rows since launch, and that is sourced constancy rather than an unworked row**: 12:30 CT at RA1006-4, RA1101-4, RA1102-4 (sha256 a71fb96d6821ed4396be3c726c42beb9e299b55ae3f9e1de2a2b2706df193607) and RA1104-4, at eight captures of the metals hours page between 2011-10-29 and 2015-03-22, on the server-rendered specification 2019-11-18 to 2021-06-21, on the archived ContractSpecs API at 2021-07-10, 2022-08-26, 2023-06-17, 2025-06-04, 2025-10-16, 2026-05-27 and 2026-08-27, and live. The two dated revisions are order-entry only: 2011-04-10 (RA1104-4 staggers the queue to Sunday 16:18 / Monday-Thursday 16:48 CT) and 2012-04-15 (Globex notice 20120409 ends the stagger and restores 16:15 / 16:45 as one randomised minute per group; its adoption is observed on the hours page at 2012-09-14 and 2013-09-02, so the stale 2012-06-16 weekday cell is evidenced as staleness rather than asserted over). **The Sunday queue is the sourced intersection.** Its onset is sourced at 16:15 CT (RA1006-4 / RA1101-4 / RA1102-4), then staggered by RA1104-4, then 16:15 again (Globex notice 20120409), then 16:00 — undated, bracketed to (2012-04-15, 2012-06-16] by the hours page and stated by CME's client-systems wiki in 2025, with all twelve archived weekly notices from 2012-04-16 to 2012-06-25 silent on it. A knowledge boundary may only widen, so the row serves 16:15→17:00 and withholds 16:00-16:15, exactly as `globex_equity_index` does; the wiki's paragraph is word-for-word the 2012 notice with that one number changed, so it is one open question across four rows, tracked as issue #79. The weekday onset needs no withholding: 16:45 → the stagger → 16:45, all three states dated. **The observation gap** is 2015-03-22 to 2019-11-18 and is channel exhaustion, not a sampling choice: metals-hours.html has no 200-status capture after 2015-03-22 and the specification channel carried no TAS hours line until autumn 2019. **The 2015-09-20 DCM-wide move did not reach it.** Globex notice 20150817 moves closes that sat at 16:15 CT to 16:00 CT for CME Equity, CBOT Equity, COMEX, NYMEX and DME — its own stated day is Monday 2015-09-21, not a Sunday — and a 12:00-12:30 CT close cannot move to 16:00; the values either side of the observation gap are identical. The control is that the same notice demonstrably did move the metals outrights. Members `MGT` (2018-09-23), `QOT` and `1OT` (2025-07-27) are caller catalog data and date no revision. **Channel note.** The CFTC-hosted NYMEX and COMEX filings were fetched directly from cftc.gov and the notices, hours pages, specification captures and archived ContractSpecs payloads through web.archive.org `id_` replay, so neither carries the reader caveat; only the live ContractSpecs records and the live 2015 and 2025 notice pages were read as extracted text through a public reader in front of the cmegroup.com URLs, and no review date rests on that channel alone.

## Revision rows

- 2010-04-11 — T1 — COMEX Submission 10-070 with SER S-5166 — the launch grid, 17:00 CT to a 12:30 CT close with Sunday 16:15 and Monday-Thursday 16:45 CT queue onsets; keyed to the local Sunday opening day.
- 2011-04-10 — T1 — NYMEX & COMEX MRAN RA1104-4 — order-entry only: the queue is staggered to Sunday 16:18 and Monday-Thursday 16:48 CT.
- 2012-04-15 — T1 — CME Globex notice 20120409 — order-entry only: the stagger ends and 16:15 / 16:45 CT are restored as one randomised minute per group, so the row points back at the launch profile.

## Sources

Row review: 2026-09-12 (UTC) is the date the ledger row was last reviewed as a
whole. Per-source retrieval dates were not recorded before the 2026-09-12
migration; where a later targeted review, capture or document date is recorded
beside a source below, that date governs for that source, and dates are added
as each source is re-verified.

- <https://www.cftc.gov/sites/default/files/stellent/groups/public/@rulesandproducts/documents/ifdocs/rul031110nymexandcomex001.pdf> — NYMEX/COMEX Submission 10-070 with SER S-5166 attached — the gold and silver TAS launch, "on Globex on April 11 (for trade date April 12)".
- <https://web.archive.org/web/20120512095523id_/http://www.cmegroup.com/rulebook/files/SER-5542__10-12-22__TAS_COMEX.pdf> — COMEX SER-5542 of 2010-12-22 — the copper TAS launch, "effective Sunday, January 23, 2011 for trade date Monday, January 24, 2011".
- <https://www.cftc.gov/sites/default/files/stellent/groups/public/@rulesandproducts/documents/ifdocs/rul033111nymexandcomex001.pdf> — NYMEX & COMEX MRAN RA1104-4 — the 2011-04-10 staggered queue onsets.
- <https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20120409.html> — CME Globex notice 20120409 — the 2012-04-15 restoration of 16:15 / 16:45 CT as one randomised minute per group.
- <https://web.archive.org/web/20190716065836id_/https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20120409.html> — CME Globex notice 20120409, archived capture.
- <https://web.archive.org/web/20190722114429id_/https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20120402.html> — CME Globex notice 20120402.
- <https://www.cftc.gov/sites/default/files/stellent/groups/public/@rulesandproducts/documents/ifdocs/rul083109nymexandcomex001.pdf> — NYMEX & COMEX MRAN RA0907-4 — the complete pre-2010 Globex TAS list, with no metals.
- <https://www.cftc.gov/sites/default/files/stellent/groups/public/@rulesandproducts/documents/ifdocs/rul020210nymexandcomex001.pdf> — NYMEX & COMEX MRAN RA1001-4 / RA1002-4 of 2010-02-02 — a complete TAS list with no metals.
- <https://www.cftc.gov/sites/default/files/stellent/groups/public/@rulesandproducts/documents/ifdocs/rul102110nymexandcomex001.pdf> — NYMEX & COMEX MRAN RA1006-4 — the launch No-Activity Periods for GCT and SIT.
- <https://www.cftc.gov/sites/default/files/stellent/groups/public/@rulesandproducts/documents/ifdocs/rul010311nymexandcomex001.pdf> — NYMEX & COMEX MRAN RA1101-4 — HGT and the metals TAS No-Activity Periods.
- <https://www.cftc.gov/sites/default/files/stellent/groups/public/@rulesandproducts/documents/ifdocs/rul010611nymexandcomex001.pdf> — NYMEX & COMEX MRAN RA1102-4.
- <https://www.cftc.gov/sites/default/files/stellent/groups/public/@rulesandproducts/documents/ifdocs/rul101811nymexandcomex001.pdf> — NYMEX & COMEX MRAN RA1107-4 — enumerates the pit-eligible TAS contracts as gold and silver only.
- <https://www.cmegroup.com/rulebook/files/cme-group-Rule-524.pdf> — CME Group Rule 524 — the TAS execution routes.
- <https://web.archive.org/web/20111029073737id_/http://www.cmegroup.com/trading_hours/metals-hours.html> — CME metals trading-hours page — capture 2011-10-29.
- <https://web.archive.org/web/20120616193920id_/http://www.cmegroup.com/trading_hours/metals-hours.html> — CME metals trading-hours page — capture 2012-06-16, the upper bound of the Sunday-queue bracket.
- <https://web.archive.org/web/20120914235904id_/http://www.cmegroup.com/trading_hours/metals-hours.html> — CME metals trading-hours page — capture 2012-09-14.
- <https://web.archive.org/web/20130902013752id_/http://www.cmegroup.com/trading_hours/metals-hours.html> — CME metals trading-hours page — capture 2013-09-02.
- <https://web.archive.org/web/20150322121442id_/http://www.cmegroup.com/trading_hours/metals-hours.html> — CME metals trading-hours page — capture 2015-03-22, the last 200-status capture.
- <https://web.archive.org/web/20150906022503id_/http://www.cmegroup.com/trading/metals/precious/gold_contract_specifications.html> — CME gold contract specification — capture 2015-09-06.
- <https://web.archive.org/web/20151009145054id_/http://www.cmegroup.com/trading/metals/precious/gold_contract_specifications.html> — CME gold contract specification — capture 2015-10-09.
- <https://web.archive.org/web/20191118131805id_/http://www.cmegroup.com/trading/metals/precious/gold_contract_specifications.html> — CME gold contract specification — capture 2019-11-18, the first to carry a TAS hours line.
- <https://www.cmegroup.com/CmeWS/mvc/ContractSpecs/List/productId/437> — CME ContractSpecs API, productId 437 — the live TAS hours records.
- <https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20150817.html> — CME Globex notice 20150817 — the DCM-wide 2015 move, which did not reach these roots.
- <https://cmegroupclientsite.atlassian.net/wiki/spaces/EPICSANDBOX/pages/457223974/Trade+at+Settlement+-+TAS> — CME client-systems wiki, Trade at Settlement — the 2025 statement of the Sunday queue onset.
- <https://web.archive.org/web/20250909102259id_/https://www.cmegroup.com/education/articles-and-reports/trade-at-settlement-for-metals.html> — CME, Trade at Settlement for metals — capture 2025-09-09.
- <https://www.cmegroup.com/content/dam/cmegroup/market-regulation/rule-filings/2025/7/25-240.pdf> — CME rule filing 25-240 — the 2025-07-27 member listings.
- <https://www.cmegroup.com/notices/electronic-trading/2025/07/20250721.html> — CME Globex notice 20250721.
- <https://web.archive.org/web/20260214052421id_/https://www.cmegroup.com/notices/electronic-trading/2018/09/20180903.html> — CME Globex notice 20180903 — the control that CME writes a readiness clause when it means one.

## Gaps and residual risks

- **executable** — the observation gap is 2015-03-22 to 2019-11-18, and it is channel exhaustion rather than a sampling choice: metals-hours.html has no 200-status capture after 2015-03-22 and the specification channel carried no TAS hours line until autumn 2019. The close is carried across it with no cutover asserted, and a close instant governs a window where trades print. Closing condition: a CME artifact stating this root's window inside that interval. Dormant identity, so recorded here rather than opened as an issue.
- **order-entry** — the Sunday queue is the sourced intersection. Its onset is sourced at 16:15 CT (RA1006-4 / RA1101-4 / RA1102-4), then staggered by RA1104-4, then 16:15 again (Globex notice 20120409), then 16:00 — undated, bracketed to (2012-04-15, 2012-06-16] by the metals hours page and stated by CME's client-systems wiki in 2025, with all twelve archived weekly notices from 2012-04-16 to 2012-06-25 silent on it. A knowledge boundary may only widen, so the row serves 16:15 to 17:00 and withholds 16:00-16:15, exactly as `globex_equity_index` does; the wiki's paragraph is word-for-word the 2012 notice with that one number changed, so it is one open question across four rows, tracked as issue #79. The weekday onset needs no withholding: 16:45, the stagger, 16:45, all three states dated.
- **sourced absence** — `regular` is empty in every era. Rule 524.A enumerates the TAS execution routes as CME Globex plus Rule 526 block trades and Rule 538 EFP/EFR and names no other; all forty-five TAS records in CME's ProductSlate read floor "-" / floorVol "0" / venues "Globex ClearPort "; the ContractSpecs API's "Open Outcry:" venue label exists — it prints one for SOFR options — and is never attached to a TAS hours line; and the Daily Bulletin carries TAS as an ex-pit volume column, never a priced product row, which already held on the 2015-03-20 pit-era edition.
- **the 2009-2015 pit clause does not create a regular session** — NYMEX and COMEX Rule 524.A.1 read "TAS transactions executed in the pit must be made open and competitively pursuant to the requirements of Rule 521 during the hours designated for pit trading in the particular contract" from 2009-09-14 until a 2015-03-06..2016-08-09 bracket, and pit TAS was real for gold and silver (SER S-5166 heads its sections "Traded on. CME Globex and COMEX Pit" and dates the floor launch "on the COMEX trading floor on April 12"). But that route had no window of its own, CME's advisories list it under the underlying's product code while the Globex route carries the TAS root's own code, and the hours page's pit column is unreliable per row: Gold TAS carries the gold pit window at every capture, Silver TAS none at any, and Copper TAS one cell of "08:10-13:00 ET" that is CME's error for the copper pit matched-order window. No open-outcry window is encoded.
- **sourced constancy, not an unworked row** — zero executable revision rows since launch: 12:30 CT at RA1006-4, RA1101-4, RA1102-4 (sha256 a71fb96d6821ed4396be3c726c42beb9e299b55ae3f9e1de2a2b2706df193607) and RA1104-4, at eight captures of the metals hours page between 2011-10-29 and 2015-03-22, on the server-rendered specification 2019-11-18 to 2021-06-21, on the archived ContractSpecs API at 2021-07-10, 2022-08-26, 2023-06-17, 2025-06-04, 2025-10-16, 2026-05-27 and 2026-08-27, and live.
- **ruled out** — the 2015-09-20 DCM-wide move did not reach this root. Globex notice 20150817 moves closes that sat at 16:15 CT to 16:00 CT for CME Equity, CBOT Equity, COMEX, NYMEX and DME — its own stated day is Monday 2015-09-21, not a Sunday — and a 12:00-12:30 CT close cannot move to 16:00; the values either side of the observation gap are identical. The control is that the same notice demonstrably did move the metals outrights.
- **state classification** — every inter-trade-date gap is 4h30m, over the four-hour limit, so it is `Closed` rather than `Maintenance`.
- **catalog, not schedule** — members `MGT` (2018-09-23), `QOT` and `1OT` (2025-07-27) are caller catalog data and date no revision.
- **staleness, not a state** — the 2012-04-15 row's adoption is observed on the hours page at 2012-09-14 and 2013-09-02, so the stale 2012-06-16 weekday cell is evidenced as staleness rather than asserted over.
- **evidence tier** — the ledger row is `T1`: every window this key serves is stated by a CME operator document (the MRAN filings, the metals hours page, the contract specification captures). CME's live ContractSpecs API, a reference-data machine channel admissible at T2, corroborates the same instants at the 2026-09-12 review.
- **channel** — the CFTC-hosted NYMEX and COMEX filings were fetched directly from cftc.gov, and the notices, hours pages, specification captures and archived ContractSpecs payloads through web.archive.org `id_` replay, so neither carries the reader caveat. Only the live ContractSpecs records and the live 2015 and 2025 notice pages were read as extracted text through a public reader in front of the cmegroup.com URLs, and no review date rests on that channel alone.

> Anchor identity for [`metals_tas.rs`](../../src/calendar/schedules/futures/us/metals_tas.rs), which is shared with [`globex_silver_tas`](globex_silver_tas.md), [`globex_copper_tas`](globex_copper_tas.md).
> Its module narrative below is the one authoritative copy; each sharer's
> file links to it rather than duplicating it.

## Module narrative (moved from src/calendar/schedules/futures/us/metals_tas.rs on 2026-09-12 UTC)

COMEX Trading at Settlement books in America/Chicago, three keys on one
envelope shape and three histories: Gold TAS (CME Globex `GCT`, security
group TG), Silver TAS (`SIT`, group MT) and Copper TAS (`HGT`, group HT).
They share the MRAN chain that states their windows and both of the
order-entry revisions below, so they live in one module; they are three keys
because their closes differ, their launch days differ and their pit
eligibility differs. Platinum and palladium TAS are a separate lineage on a
separate exchange and live in `pgm_tas.rs`.

MEMBER LISTINGS ARE CALLER CATALOG DATA, NOT REVISIONS. Gold TAS covers
`GCT` from 2010-04-11, E-micro/Micro Gold `MGT` from 2018-09-23 (Globex
notice 20180903), and E-mini Gold `QOT` and 1-Ounce Gold `1OT` from
2025-07-27 (Globex notice 20250721, corroborated by COMEX Submission 25-240,
"effective on Sunday, July 27, 2025, for trade date Monday, July 28, 2025").
Silver TAS covers `SIT` and Micro Silver `MST`, also 2025-07-27. Copper TAS
covers `HGT`, Micro Copper `MHT` (2025-07-27) and the spot-month root `HG0`
— "known as TAS zero or TAS flat", in CME's own education article — whose
listing day is undated: CME's copper Product Code cell shows it by
2018-08-10 but demonstrably drops TAS codes that exist (it shows none at all
at 2015-10-08 and 2017-06-13, while `HGT` had traded since 2011-01-24), and
all seventy archived weekly Globex notices from 2017-06-05 to 2018-08-27
name it nowhere. Tracked as issue #80. Each of these joined a group that
already had a grid, so none dates a revision row here.
https://web.archive.org/web/20260214052421id_/https://www.cmegroup.com/notices/electronic-trading/2018/09/20180903.html
https://www.cmegroup.com/notices/electronic-trading/2025/07/20250721.html
https://www.cmegroup.com/content/dam/cmegroup/market-regulation/rule-filings/2025/7/25-240.pdf
https://web.archive.org/web/20250909102259id_/https://www.cmegroup.com/education/articles-and-reports/trade-at-settlement-for-metals.html

NO REGULAR SESSION, AND THE PIT CLAUSE DOES NOT CREATE ONE. Four channels
carry the negative for the TAS shape. CME, CBOT, NYMEX and COMEX Rule 524.A
enumerate the execution routes — CME Globex, Rule 526 block trades, Rule 538
EFP/EFR — and name no other. CME's ProductSlate carries forty-five TAS
product records and every one reads `floor "-"`, `floorVol "0"` and a venues
string of "Globex ClearPort " with no Floor component. The ContractSpecs API
does have an "Open Outcry:" venue label — it prints one for SOFR options —
and never attaches it to a TAS hours line. And the Daily Bulletin carries
TAS as a volume column inside the ex-pit breakdown ("TAS DAILY TOTALS"),
never as a priced product row, which already held on the 2015-03-20 pit-era
edition where RTH volume was live.

The 2009-2015 clause, recorded so the next reader does not "discover" it and
conclude the key is wrong: NYMEX & COMEX Rule 524.A.1, from its adoption on
2009-09-14 until a bracket of 2015-03-06..2016-08-09, read "TAS transactions
executed in the pit must be made open and competitively pursuant to the
requirements of Rule 521 during the hours designated for pit trading in the
particular contract". There WAS a pit-era COMEX TAS, for gold and silver
only: SER S-5166 heads its sections "A. Gold Futures (GC) (CME Globex TAS
code GCT) Traded on. CME Globex and COMEX Pit" and its cover letter dates the
floor launch one day after the electronic one, "on the COMEX trading floor on
April 12"; RA1107-4 confirms and adds that copper never was — "TAS
transactions are allowed in the active contract month in Gold and Silver
futures trading in the pit or on CME Globex and in the first active contract
month in Copper futures trading on CME Globex. TAS transactions are not
allowed in any pit-traded Copper futures contract month."

It still gives these keys no regular session, on three grounds. The pit route
had no window of its own — Rule 524.A.1 ties it to the underlying's pit
hours. CME's own advisories list the pit route by the UNDERLYING's product
code under "Pit-Traded Contracts" and the electronic route by the TAS root's
own Globex code under "CME Globex Contracts", so the pit route is a pricing
convention inside the underlying's pit and the instrument these keys model is
the Globex TAS book. And CME's own pit column is unusable per row anyway: the
metals hours page gives Gold TAS an Open Outcry cell of "08:20-13:30 ET
(07:20-12:30 CT)" — the gold pit window — at every capture 2011-10-29 to
2015-03-22, gives Silver TAS none at any of them, and prints "08:10-13:00 ET"
for Copper TAS once, which is CME's error: that is the copper pit
MATCHED-ORDER window, stated as such by RA1005-4, RA1006-4 and RA1107-4
("Regular trading hours for open outcry trading in the Copper futures pit are
from 8:10a.m. until1:00 p.m. Eastern Time"), and the cell is empty from
2012-05-01 on. `energy_metals.rs` already ships `regular: &[]` for the
NYMEX/COMEX outrights in every era, so no NYMEX or COMEX pit is modelled as
regular anywhere in this crate. Do not encode a pit or open-outcry window
here.
https://www.cmegroup.com/rulebook/files/cme-group-Rule-524.pdf
https://www.cftc.gov/sites/default/files/stellent/groups/public/@rulesandproducts/documents/ifdocs/rul101811nymexandcomex001.pdf

THE PRE-LAUNCH ERA IS A SOURCED CLOSURE, NOT AN UNWORKED GAP, SO NOTHING IS
CARRIED BACK TO THE JANUARY-2010 FLOOR. RA0907-4 (2009-08-31) prints CME's
complete "CME Globex Contracts" TAS list as eleven codes — `WST`, `RTI`,
`BHT`, `HPT`, `HHT`, `BBT`, `CLT`, `HOT`, `NGT`, `RBT`, `RET` — and copper
appears in it only as a pit-traded MATCHED ORDER product, which is not TAS.
RA1001-4 / RA1002-4 (2010-02-02), one month after the audit floor, reprints
the complete list with fifteen codes and still no gold, silver or copper. So
the pre-2010-11-01 Rule 524.A.2 sentence that supplies the energy-TAS floor
era — "TAS transactions on Globex may take place at any time the applicable
contracts are available for trading on Globex" — is inapplicable here: gold
was not an applicable contract. RA1005-4 (2010-10-11) and RA1006-4
(2010-10-22) carry `GCT` and `SIT` rows and no `HGT` row, which closes
copper's own pre-launch era the same way.
https://www.cftc.gov/sites/default/files/stellent/groups/public/@rulesandproducts/documents/ifdocs/rul083109nymexandcomex001.pdf
https://www.cftc.gov/sites/default/files/stellent/groups/public/@rulesandproducts/documents/ifdocs/rul020210nymexandcomex001.pdf

LAUNCH DAYS, ALL UNCONDITIONAL AND KEYED TO THE LOCAL OPENING DAY. Gold and
silver: NYMEX/COMEX Submission 10-070 with COMEX Special Executive Report
S-5166 attached, a Regulation 40.6 self-certification whose cover letter
states "the launch of TAS pricing for the active month in each of Gold
Futures (Chapter 113) and Silver Futures (Chapter 112) on Globex on April 11
(for trade date April 12)". The grid opens 17:00 CT on the Sunday, so the row
is keyed to Sunday 2010-04-11. S-5166 is not separately retrievable at any
CME path; it survives as an attachment inside the CFTC filing, and is cited
that way. Copper: COMEX SER-5542 (2010-12-22), "effective Sunday, January 23,
2011 for trade date Monday, January 24, 2011 … COMEX will offer TAS in the
first and second active months for Copper Futures on CME Globex only" —
keyed to Sunday 2011-01-23, the same day RA1101-4 carries as its own
Effective Date while printing `HGT`'s window.
https://www.cftc.gov/sites/default/files/stellent/groups/public/@rulesandproducts/documents/ifdocs/rul031110nymexandcomex001.pdf
https://web.archive.org/web/20120512095523id_/http://www.cmegroup.com/rulebook/files/SER-5542__10-12-22__TAS_COMEX.pdf

THE WINDOW, IN CME'S OWN SESSION LANGUAGE. RA1006-4 is the earliest clean
statement, six months after the gold and silver launch, and it is a session
statement rather than a calculation window: "the end of the trading session
is defined by receipt of the security status message indicating that group is
closed". Its table reads "GCT COMEX Gold … No-Activity Periods: 1:30 p.m.-
5:45 p.m. (ET) Monday- Thursday / 1:30 p.m. (ET) Friday- 5:15 p.m. (ET)
Sunday" and "SIT COMEX Silver … 1:25 p.m.- 5:45 p.m. (ET) Monday -Thursday /
1:25 p.m. (ET) Friday-5:15p.m. (ET) Sunday"; RA1101-4 adds "HGT COMEX Copper
… 1:00 p.m.- 5:45 p.m. (ET) Monday- Thursday / 1:00 p.m. (ET)
Friday-5:15p.m. (ET) Sunday". So the closes are 12:30, 12:25 and 12:00 CT;
the weekday queue opens 16:45 CT and the Sunday queue 16:15 CT; and there is
NO Friday-evening reopen, because the dark run goes Friday close straight to
Sunday 17:15 ET. RA1102-4 (advisory 2011-01-07, effective 2011-01-23, sha256
a71fb96d6821ed4396be3c726c42beb9e299b55ae3f9e1de2a2b2706df193607) reprints
all three rows unchanged and is the fourth printing of the same table.

The 17:00 CT OPEN is not in the MRAN chain, which prints only dark windows.
It comes from CME's metals trading-hours page, whose Gold TAS row reads
"18:00-13:30 ET (17:00-12:30 ET)" — the page's own zone typo in the second
pair, preserved — with Silver TAS at 18:00-13:25 ET and Copper TAS at
18:00-13:00 ET, and it is carried back to each launch day with no revision
asserted, exactly as `AGENTS.md`'s carry-back convention provides.
https://www.cftc.gov/sites/default/files/stellent/groups/public/@rulesandproducts/documents/ifdocs/rul102110nymexandcomex001.pdf
https://www.cftc.gov/sites/default/files/stellent/groups/public/@rulesandproducts/documents/ifdocs/rul010311nymexandcomex001.pdf
https://www.cftc.gov/sites/default/files/stellent/groups/public/@rulesandproducts/documents/ifdocs/rul010611nymexandcomex001.pdf
https://web.archive.org/web/20111029073737id_/http://www.cmegroup.com/trading_hours/metals-hours.html

EVERY INTER-TRADE-DATE GAP EXCEEDS FOUR HOURS, SO IT IS `Closed`, NOT
`Maintenance`. Gold 12:30->17:00 is 4h30m, silver 4h35m, copper 5h. The
crate's convention retains an inter-trade-date gap as `Maintenance` only up
to four elapsed hours; none of these three qualifies. Stated here so nobody
"fixes" it later.

ZERO EXECUTABLE REVISION ROWS, LAUNCH TO TODAY, AND THAT IS SOURCED
CONSTANCY RATHER THAN AN UNWORKED ROW. Gold's 12:30 CT close is stated at
RA1006-4, RA1101-4, RA1102-4 and RA1104-4; on the metals hours page at eight
captures between 2011-10-29 and 2015-03-22; on the server-rendered contract
specification from 2019-11-18 to 2021-06-21 ("TAS: Sun-Fri 6:00 p.m. - 1:30
p.m. ET (5:00 - 12:30 CT)"); on CME's archived ContractSpecs API at
2021-07-10, 2022-08-26, 2023-06-17, 2025-06-04, 2025-10-16, 2026-05-27 and
2026-08-27; and live. Silver carries the same shape at 12:25 (seven archived
API captures 2021-07-10..2026-05-27 alone) and copper at 12:00. No source
names a change at any point, so no cutover is asserted.

OBSERVATION GAP TO RECORD, 2015-03-22 -> 2019-11-18. It is channel
exhaustion rather than a sampling choice: `metals-hours.html` has no
200-status capture after 2015-03-22 (301s thereafter), and the
contract-specification channel carried no TAS hours line until autumn 2019
(gold absent 2019-09-24, present 2019-11-18; copper absent 2019-07-20,
present 2019-12-20). This is why the row is `Partial` with an executable gap.
https://web.archive.org/web/20120914235904id_/http://www.cmegroup.com/trading_hours/metals-hours.html
https://web.archive.org/web/20150322121442id_/http://www.cmegroup.com/trading_hours/metals-hours.html
https://web.archive.org/web/20191118131805id_/http://www.cmegroup.com/trading/metals/precious/gold_contract_specifications.html
https://www.cmegroup.com/CmeWS/mvc/ContractSpecs/List/productId/437

THE 2015-09-20 DCM-WIDE MOVE DID NOT REACH METALS TAS, on two independent
arguments. Scope, in the notice's own words: Globex notice 20150817 says the
daily maintenance period begins fifteen minutes earlier and "the closing
times for the following markets will now occur 15 minutes earlier Monday
through Friday at 16:00 CT" for CME Equity, CBOT Equity, COMEX, NYMEX and
DME. A 12:30 / 12:25 / 12:00 CT close is not a 16:15 CT close and cannot move
to 16:00. And direct observation: the last TAS statement before the gap
(2015-03-22) and the first after it (2019-11-18) are the same values. Note
the notice's own stated day is Monday 2015-09-21 — 2015-09-20 was a Sunday —
so "trade date 2015-09-20" is wrong wherever it appears. The control that
makes these separate keys from `globex_energy`: the same notice demonstrably
DID move the metals outrights, gold's specification going from "5:15 p.m. …
45-minute break" on 2015-09-06 to "5:00 p.m. … 60-minute break" on
2015-10-09.
https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20150817.html
https://web.archive.org/web/20150906022503id_/http://www.cmegroup.com/trading/metals/precious/gold_contract_specifications.html
https://web.archive.org/web/20151009145054id_/http://www.cmegroup.com/trading/metals/precious/gold_contract_specifications.html

---

THE TWO DATED REVISIONS ARE BOTH ORDER-ENTRY ONLY, AND BOTH TOUCH ONLY THESE
THREE KEYS.

2011-04-10 (Sunday opening day; trade date Monday 2011-04-11) — RA1104-4:
"Effective on Sunday, April 10, 2011, for trade date Monday, April 11, 2011,
the pre-opening times for TAS trading on CME Globex will be as follows (all
times are in Eastern Time): … COMEX Gold Monday - Thursday at 5:48 p.m.,
Sunday at 5:18 p.m. … COMEX Silver Monday - Thursday at 5:49 p.m., Sunday at
5:19 p.m. … COMEX Copper Monday - Thursday at 5:50 p.m., Sunday at 5:20
p.m." — 16:48/16:18, 16:49/16:19 and 16:50/16:20 CT. The same table leaves
every close unchanged at 1:30 / 1:25 / 1:00 p.m. ET, and its stated purpose
is queue management: the revised times "are being implemented to stagger and
more evenly distribute order flow during the pre-opening time period".
Corroborated by the hours page's 17:18 / 17:48 ET cells at 2011-10-29 and
2012-05-01. RA1104-4 is the last MRAN edition to print per-product clock
times; RA1106-4 replaces the table with a security-status-message definition.
https://www.cftc.gov/sites/default/files/stellent/groups/public/@rulesandproducts/documents/ifdocs/rul033111nymexandcomex001.pdf

2012-04-15 (Sunday opening day; trade date Monday 2012-04-16) — CME Globex
notice 20120409, announced a week earlier in 20120402: "On Sunday, April 15
(trade date Monday, April 16) … CME Group will randomize the timing of each
TAS groups' pre-open state. … There will be a market pause … at 16:15:00 on
Sunday and 16:45:00 Monday through Thursday. TAS groups will then go into
pre-open on Sundays between 16:15:00 and 16:16:00 Central time (CT), and
Mondays through Thursdays between 16:45:00 and 16:46:00 CT." Unconditional,
day-level, and it ends the stagger: every TAS group returns to the shared
16:15 / 16:45 CT onsets. The crate encodes the nominal onset, never the
randomized second, which RA2302-5 confirms is still the live construction.
That the hours page still printed 16:48/16:49/16:50 on 2012-06-16 is page
staleness and is evidenced rather than asserted: the same URL prints
"Pre-Open Electronic Trading (Weekday) 17:45-17:46 ET (16:45-16:46 CT)" for
all three metals at 2012-09-14 and again at 2013-09-02, so the notice's
adoption is observed within five months.
https://web.archive.org/web/20190716065836id_/https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20120409.html
https://web.archive.org/web/20190722114429id_/https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20120402.html
https://web.archive.org/web/20130902013752id_/http://www.cmegroup.com/trading_hours/metals-hours.html

THE SUNDAY QUEUE SERVES THE SOURCED INTERSECTION, AND 16:00-16:15 IS
WITHHELD. The Sunday onset is sourced at 16:15 (RA1006-4, RA1101-4,
RA1102-4), then 16:18/16:19/16:20 (RA1104-4), then 16:15 again (the
2012-04-09 notice), then 16:00 — undated, bracketed to
(2012-04-15, 2012-06-16] by the hours page's "17:00-17:01 ET (16:00-16:01
CT)" cell of 2012-06-16, stated by CME's client-systems wiki in 2025, and
announced by none of the twelve archived weekly Globex notices covering
2012-04-16 to 2012-06-25. `AGENTS.md`'s "a knowledge boundary may only
widen" therefore holds the bound at its narrowest value across the whole
undated span: these keys serve 16:15->17:00 (16:18/16:19/16:20 inside the
2011-04-10 row) and withhold the 16:00-16:15 quarter-hour, exactly as
`globex_equity_index`'s Sunday Pre-Open already does — and it is almost
certainly the same CME-wide 2012 move, because the wiki's paragraph is
word-for-word the 2012-04-09 notice with that one number changed. Whoever
dates one dates all four rows; tracked as issue #79. The weekday onset needs
no withholding: 16:45 -> 16:48/16:49/16:50 -> 16:45, all three states dated.
https://cmegroupclientsite.atlassian.net/wiki/spaces/EPICSANDBOX/pages/457223974/Trade+at+Settlement+-+TAS
https://web.archive.org/web/20120616193920id_/http://www.cmegroup.com/trading_hours/metals-hours.html

---

Revision evidence — each row's day-level effective date, keyed to the local
Sunday opening day of the first session it governs, and the primary source
that states it (full quotations sit in the blocks above):
  2010-04-11 "COMEX Submission 10-070 with SER S-5166"
    https://www.cftc.gov/sites/default/files/stellent/groups/public/@rulesandproducts/documents/ifdocs/rul031110nymexandcomex001.pdf
  2011-01-23 "COMEX SER-5542"
    https://web.archive.org/web/20120512095523id_/http://www.cmegroup.com/rulebook/files/SER-5542__10-12-22__TAS_COMEX.pdf
  2011-04-10 "NYMEX & COMEX MRAN RA1104-4"
    https://www.cftc.gov/sites/default/files/stellent/groups/public/@rulesandproducts/documents/ifdocs/rul033111nymexandcomex001.pdf
  2012-04-15 "CME Globex notice 20120409"
    https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20120409.html

The 2012-04-15 rows point back at each key's launch profile because the
notice restores exactly the shared 16:15 / 16:45 CT onsets that RA1104-4
staggered; the tables are shared rather than copied so a future correction
cannot move one and not the other.
Evidence: docs/evidence/globex_gold_tas.md
