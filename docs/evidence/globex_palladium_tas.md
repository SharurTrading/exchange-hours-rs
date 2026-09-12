<!-- SPDX-License-Identifier: MIT-0 -->

# `globex_palladium_tas` — evidence

- **Kind:** `MarketHoursKey`
- **Owner module:** [`pgm_tas.rs`](../../src/calendar/schedules/futures/us/pgm_tas.rs)
- **Source sets:** [`US-CME-GROUP`](../schedules/sources.md#us-cme-group)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

**Gap: executable** — the window is carried across a five-year archive gap in its only surviving channel with no cutover asserted, and the close governs a window where trades print. NYMEX Palladium Trading at Settlement, CME Globex `PAT`, security group PX. Listed by CME Globex notice 20181112: "Palladium TAS - This Week. Effective this Sunday, November 18 (trade date Monday, November 19), Palladium TAS will be listed for trading on CME Globex and for submission for clearing via CME ClearPort." Unconditional, and the 20180903 notice is the control that CME writes "pending completion of all regulatory review periods" when it means it. Keyed to the local opening day, Sunday 2018-11-18. **One era, no revision of any kind.** Window: Sunday-Thursday 17:00 CT wrapping to 12:00 CT with no Friday-evening reopen, from the specification's "TAS: Sunday - Friday 6:00 p.m. - 1:00 p.m. (5:00 p.m. - Noon CT)" at 2020-11-26 and 2021-05-13, the same instants at 2019-07-17, and "TAS: Sunday - Friday 5:00 p.m. - Noon CT" on the archived ContractSpecs API at 2026-05-27 and live. The inter-trade-date gap is 5h, so `Closed` rather than `Maintenance`. **There is no 16:00-17:00 CT daily break**: the 2019 specification's "with a 60-minute break each day beginning at 4:00 p.m.(CT)" clause is the outright row's clause restated one line down with the CT/ET pair swapped — a book closed at 12:00 CT cannot break at 16:00 — and CME had deleted it by 2020-11-26 with the instants untouched. `regular` is empty in every era and that is a sourced absence: Rule 524.A enumerates the TAS execution routes as CME Globex plus Rule 526 block trades and Rule 538 EFP/EFR and names no other; all forty-five TAS records in CME's ProductSlate read floor "-" / floorVol "0" / venues "Globex ClearPort "; the ContractSpecs API's "Open Outcry:" venue label exists — it prints one for SOFR options — and is never attached to a TAS hours line; and the Daily Bulletin carries TAS as an ex-pit volume column, never a priced product row, which already held on the 2015-03-20 pit-era edition. The 2009-2015 NYMEX pit clause never reached this root. **The queue withholds nothing**, for the same reason as platinum: it launched after the undated 2012 Sunday move, so Sunday 16:00-17:00 and Monday-Thursday 16:45-17:00 CT are its earliest sourced onsets, carried back from CME's client-systems wiki with no cutover asserted. **The observation gap is 2021-05-13 to 2026-05-27**, an archive limit: productId 445 — which is palladium, not platinum, the reverse of the obvious guess — has exactly one 200-status capture of that path. **Do not substitute `globex_copper_tas`** on the coincident 12:00 CT close: different exchange, different security group, launches seven years apart. **Channel note.** The CFTC-hosted NYMEX and COMEX filings were fetched directly from cftc.gov and the notices, hours pages, specification captures and archived ContractSpecs payloads through web.archive.org `id_` replay, so neither carries the reader caveat; only the live ContractSpecs records and the live 2015 and 2025 notice pages were read as extracted text through a public reader in front of the cmegroup.com URLs, and no review date rests on that channel alone.

## Revision rows

- 2018-11-18 — T1 — CME Globex notice 20181112 — the launch grid, 17:00 CT to a 12:00 CT close with Sunday 16:00-17:00 and Monday-Thursday 16:45-17:00 CT queues; keyed to the local Sunday opening day.

## Sources

Row review: 2026-09-12 (UTC) is the date the ledger row was last reviewed as a
whole. Per-source retrieval dates were not recorded before the 2026-09-12
migration; where a later targeted review, capture or document date is recorded
beside a source below, that date governs for that source, and dates are added
as each source is re-verified.

- <https://www.cmegroup.com/notices/electronic-trading/2017/05/20170508.html> — CME Globex notice 20170508 — "Effective Sunday, May 21 (trade date Monday, May 22), Platinum futures TAS will be listed for trading on CME Globex".
- <https://web.archive.org/web/20260214165515id_/https://www.cmegroup.com/notices/electronic-trading/2017/05/20170508.html> — CME Globex notice 20170508, archived capture.
- <https://www.cmegroup.com/notices/electronic-trading/2018/11/20181112.html> — CME Globex notice 20181112 — "Effective this Sunday, November 18 (trade date Monday, November 19), Palladium TAS will be listed for trading on CME Globex".
- <https://web.archive.org/web/20260218023215id_/https://www.cmegroup.com/notices/electronic-trading/2018/11/20181112.html> — CME Globex notice 20181112, archived capture.
- <https://web.archive.org/web/20170303065057id_/http://www.cmegroup.com/trading/metals/precious/platinum_contract_specifications.html> — CME platinum contract specification — capture 2017-03-03, the string TAS appears zero times.
- <https://web.archive.org/web/20170523204255id_/http://www.cmegroup.com/trading/metals/precious/platinum_contract_specifications.html> — CME platinum contract specification — capture 2017-05-23, "TAS: PLT" and "Trading ceases daily at 1:05 PM ET.".
- <https://web.archive.org/web/20190717122952id_/http://www.cmegroup.com/trading/metals/precious/platinum_contract_specifications.html> — CME platinum contract specification — capture 2019-07-17.
- <https://web.archive.org/web/20210411151252id_/http://www.cmegroup.com/trading/metals/precious/platinum_contract_specifications.html> — CME platinum contract specification — capture 2021-04-11, "TAS: Sunday - Friday 6:00 p.m. - 1:05 p.m. (5:00 p.m. - 12:05 p.m. CT)".
- <https://web.archive.org/web/20201126020132id_/http://www.cmegroup.com/trading/metals/precious/palladium_contract_specifications.html> — CME palladium contract specification — capture 2020-11-26, "TAS: Sunday - Friday 6:00 p.m. - 1:00 p.m. (5:00 p.m. - Noon CT)".
- <https://www.cmegroup.com/CmeWS/mvc/ContractSpecs/List/productId/446> — CME ContractSpecs API, productId 446 — platinum.
- <https://www.cmegroup.com/CmeWS/mvc/ContractSpecs/List/productId/445> — CME ContractSpecs API, productId 445 — palladium.
- <https://www.cftc.gov/sites/default/files/stellent/groups/public/@rulesandproducts/documents/ifdocs/rul101811nymexandcomex001.pdf> — NYMEX & COMEX MRAN RA1107-4 — enumerates the pit-eligible TAS contracts as gold and silver only.
- <https://www.cmegroup.com/rulebook/files/cme-group-Rule-524.pdf> — CME Group Rule 524 — the TAS execution routes.
- <https://cmegroupclientsite.atlassian.net/wiki/spaces/EPICSANDBOX/pages/457223974/Trade+at+Settlement+-+TAS> — CME client-systems wiki, Trade at Settlement — Sunday 16:00-16:01 and Monday-Thursday 16:45-16:46 CT.

## Gaps and residual risks

- **executable** — the observation gap is 2021-05-13 to 2026-05-27, an archive limit: productId 445 — which is palladium, not platinum, the reverse of the obvious guess — has exactly one 200-status capture of that path. Both ends state the same instants and no source names a change. Closing condition: a CME artifact stating this root's window inside that interval. Dormant identity, so recorded here rather than opened as an issue.
- **resolved misreading** — there is no 16:00-17:00 CT daily break. The 2019 specification's "with a 60-minute break each day beginning at 4:00 p.m.(CT)" clause is the outright row's clause restated one line down with the CT/ET pair swapped — a book closed at 12:00 CT cannot break at 16:00 — and CME had deleted it by 2020-11-26 with the instants untouched.
- **order-entry** — the queue withholds nothing, for the same reason as platinum: the root launched after the undated 2012 Sunday move, so Sunday 16:00-17:00 and Monday-Thursday 16:45-17:00 CT are its earliest sourced onsets, carried back from CME's client-systems wiki with no cutover asserted.
- **sourced absence** — `regular` is empty in every era. Rule 524.A enumerates the TAS execution routes as CME Globex plus Rule 526 block trades and Rule 538 EFP/EFR and names no other; all forty-five TAS records in CME's ProductSlate read floor "-" / floorVol "0" / venues "Globex ClearPort "; the ContractSpecs API's "Open Outcry:" venue label exists — it prints one for SOFR options — and is never attached to a TAS hours line; and the Daily Bulletin carries TAS as an ex-pit volume column, never a priced product row, which already held on the 2015-03-20 pit-era edition.
- **the 2009-2015 NYMEX pit clause never reached this root.**
- **state classification** — the inter-trade-date gap is 5h, so it is `Closed` rather than `Maintenance`.
- **do not substitute `globex_copper_tas`** on the coincident 12:00 CT close: different exchange, different security group, launches seven years apart.
- **evidence tier** — the ledger row is `T1`: every window this key serves is stated by a CME operator document (the listing notice and the contract specification captures). CME's live ContractSpecs API, a reference-data machine channel admissible at T2, corroborates the same instants at the 2026-09-12 review.
- **channel** — the CFTC-hosted NYMEX and COMEX filings were fetched directly from cftc.gov, and the notices, hours pages, specification captures and archived ContractSpecs payloads through web.archive.org `id_` replay, so neither carries the reader caveat. Only the live ContractSpecs records and the live 2015 and 2025 notice pages were read as extracted text through a public reader in front of the cmegroup.com URLs, and no review date rests on that channel alone.

> Shared module. The narrative for [`pgm_tas.rs`](../../src/calendar/schedules/futures/us/pgm_tas.rs)
> lives in [`globex_platinum_tas`](globex_platinum_tas.md#module-narrative-moved-from-srccalendarschedulesfuturesuspgm_tasrs-on-2026-09-12-utc).
> Sibling identities: [`globex_platinum_tas`](globex_platinum_tas.md).
