<!-- SPDX-License-Identifier: MIT-0 -->

# `euronext_index_futures` — evidence

- **Kind:** `MarketHoursKey`
- **Owner module:** [`euronext_index_futures.rs`](../../src/calendar/schedules/futures/international/euronext_index_futures.rs)
- **Source sets:** [`EU-EURONEXT`](../schedules/sources.md#eu-euronext)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-10-13 UTC)

CAC 40 Index futures (`FCE`, Paris) and AEX Index futures (`FTI`, Amsterdam) print one identical Central Order Book grid — 08:00–22:00 CET/CEST, Monday to Friday, no lunch break — on the operator's own contract-specification pages; one family serves both because the two markets share the CET/CEST clock. The Large-in-Scale Facility beside the grid (07:30–22:00) is an off-book wholesale facility and is not modelled.

## Revision rows

None. The family's module carries no `revisions!` block: its normal week is one grid, and no banked artifact dates any earlier state to key a revision against. The grid's history is therefore carried, not sourced — the horizon cell and the floor gap below record exactly that, and dates below the horizon refuse through the coverage contract instead of serving an inferred history.

## Holidays

**Coverage:** 2010-01-01..2026-12-31 (inclusive trade dates in `Europe/Paris`; tier T1 throughout).

The operator publishes **one** holiday calendar per year covering its cash and derivatives markets together, so the family reads the same retrieved corpus the sibling Paris cash table reads, at the derivatives scope each generation states:

- **2010 and 2011** — the NYSE Euronext press releases open with the group-wide scope sentence (`NYSE Euronext's European Cash and Derivatives markets will be open Monday to Friday throughout 2010 except on the following days`), so their closure lists key derivatives rows directly. The 2011 release additionally enumerates the derivatives eves per product family and states `Continental and U.S. Individual Equity Futures and Options contracts and Index Products (excluding the FTSE 100 and FTSE 250 Index contracts) will be open for a full day` — the one year the eves are stated **for this family**, so 2011 ships no eve rows.
- **2012 and 2013** — the banked editions are the `Calendar of Cash Business Days` notice and the cash Info-Flash (their own sentences name the cash markets), and the 2013 restatement on the 2014 calendar page repeats that scope. **No banked artifact states the derivatives markets' 2012–2013 arrangements.** The rows below mark the dates the cash edition closes as candidates whose status is withheld (`unsourced`); a date beside them claims normal only on the operator's one-calendar-per-year practice, which the banked 2010, 2011 and 2014–2026 editions witness. The residual and its closer are recorded under *Gaps and residual risks*.
- **2014–2021** — the calendar pages' opening sentences cover cash **and** derivatives (`Euronext's Cash and Derivatives markets will be open … except on the following days`), so their closure lists key rows. The December eves are stated only as a group — `Contracts on the Derivatives Markets will close at various times, commencing at 13:55 CET` (13:55 from 2015; `midday` in 2014) — an envelope lower bound, not the index futures' close, so the eves ship `unsourced`. 2016 and 2017 state the eves closing `at the usual times` for `the Cash and Derivatives markets`, so those two years ship no eve rows.
- **2022–2026** — the per-market tables and the 2026 INFO-FLASH carry `Cash and Derivatives` in their titles and scope sentences with a Paris column (or Paris cell), so the Paris cells key the rows. The eves print `**Half Trading Day` while deferring the hours to an end-of-year appendix to the Euronext Instructions 4-01/4-03 Trading Manuals; the appendices the store holds state the **cash** manual's Paris equity segments (the 4-01 sheet), the 4-03 sheet carries the fund/NAV facility, and no banked artifact states the index futures' own half-day instants — so the eves ship `unsourced` exactly as the sibling cash table's 2026 rows do.
- **Rows of other markets never touch this family.** The Oslo-only, Dublin-only and Milan-only rows print `Full Trading Day` for Paris (and Amsterdam where the table has the column), so no row ships for them.

### Documents

| Document | Window | Replay or service URL | Capture or retrieval, UTC | Tier | sha256 |
|---|---|---|---|---|---|
| `EURONEXT-PR-2010` | 2010-01-01 .. 2010-12-31 (the 2010 holiday calendar and early closing dates) | <https://web.archive.org/web/20181111124554id_/https://www.euronext.com/sites/www.euronext.com/files/press_release_14_oct_2009.pdf> | Wayback `id_` replay of capture `20181111124554`, retrieved 2026-09-29 UTC | T1 | `736892c0c3712b1f0e530dbcae188464f26d24647a306d96c09cf01fb8ef4547` |
| `EURONEXT-PR-2011` | 2011-01-01 .. 2011-12-31 (the 2011 holiday calendar and early closing dates) | <https://web.archive.org/web/20181111124531id_/https://www.euronext.com/sites/www.euronext.com/files/holiday_calendar_and_early_closing_dates_for_european_markets_press_release.pdf> | Wayback `id_` replay of capture `20181111124531`, retrieved 2026-09-29 UTC | T1 | `5358973c8ce8d0197a917ac2e9aa7debc8341bc16b964b065f434a3bdd28f830` |
| `EURONEXT-TC-2012` | 2012-01-01 .. 2012-12-31 (notice PAR_20111114_07899_EUR, the 2012 Calendar of Cash Business Days) | <https://web.archive.org/web/20181111124523id_/https://www.euronext.com/sites/www.euronext.com/files/calendar_2012.pdf> | Wayback `id_` replay of capture `20181111124523`, retrieved 2026-09-29 UTC | T1 | `4da0f4141496d2d39a52379157ef1d1f98c5920a5f5eb353f7d2cd19952de035` |
| `EURONEXT-IF-2013` | 2013-01-01 .. 2013-12-31 (Info-Flash IFCA121112, the 2013 holiday calendar) | <https://web.archive.org/web/20181111124519id_/https://www.euronext.com/sites/www.euronext.com/files/ifca121112.pdf> | Wayback `id_` replay of capture `20181111124519`, retrieved 2026-09-29 UTC | T1 | `d2f2c26bf770378802c2ce738ee4c93fe04b5c458d1b7c828bdd2fc37fb9a27f` |
| `EURONEXT-HH-2014-01-12` | 2014-01-01 .. 2014-12-31 (the 2014 cash-markets calendar) | <https://web.archive.org/web/20140112152726id_/https://www.euronext.com/trading/trading-hours-and-holidays> | Wayback `id_` replay of capture `20140112152726`, retrieved 2026-09-29 06:17 UTC | T1 | `fd8329e9dda392f69d21ef350f3083e15d4cd79ef58e68e8c467bc0347239bb0` |
| `EURONEXT-HH-2015-03-22` | 2015-01-01 .. 2015-12-31 (the 2015 cash-markets calendar) | <https://web.archive.org/web/20150322042835id_/https://www.euronext.com/trading/trading-hours-and-holidays> | Wayback `id_` replay of capture `20150322042835`, retrieved 2026-09-29 06:17 UTC | T1 | `5794e756a358eb5c3402f1ef93feb661679d572cd790cfb1914a202777fece47` |
| `EURONEXT-HH-2016-03-04` | 2016-01-01 .. 2016-12-31 (the 2016 calendar) | <https://web.archive.org/web/20160304192247id_/https://www.euronext.com/en/trading-calendars-hours> | Wayback `id_` replay of capture `20160304192247`, retrieved 2026-09-29 06:22 UTC | T1 | `15aadf7933c975c82ef903e70f9d8366fe8787fe99ad1bb28dd3a0416b2595da` |
| `EURONEXT-HH-2017-08-05` | 2017-01-01 .. 2017-12-31 (the 2017 calendar) | <https://web.archive.org/web/20170805202027id_/https://www.euronext.com/en/trading-calendars-hours> | Wayback `id_` replay of capture `20170805202027`, retrieved 2026-09-29 06:23 UTC | T1 | `5267845e3bd2fe339dce61d878724cb846f1e3aaed8ec1514317600e76a7f9bc` |
| `EURONEXT-HH-2018-08-26` | 2018-01-01 .. 2018-12-31 (the 2018 calendar, plus the 2017 restatement) | <https://web.archive.org/web/20180826042950id_/https://www.euronext.com/en/trading-calendars-hours> | Wayback `id_` replay of capture `20180826042950`, retrieved 2026-09-29 06:23 UTC | T1 | `d213f40a8a53ebc2f0a24f27f65d7f17cc2f6e4961e2407b986be92248b93bca` |
| `EURONEXT-HH-2019-03-25` | 2019-01-01 .. 2019-12-31 (the 2019 calendar, plus the 2018 restatement) | <https://web.archive.org/web/20190325100632id_/https://www.euronext.com/en/trading-calendars-hours> | Wayback `id_` replay of capture `20190325100632`, retrieved 2026-09-29 06:23 UTC | T1 | `1af181cc6794aee7048c4c7ee8f82ea133fa35d2673f81eaef7f8918815f8457` |
| `EURONEXT-HH-2019-12-10` | 2019-01-01 .. 2020-12-31 (the 2019 restatement and the 2020 calendar) | <https://web.archive.org/web/20191210215605id_/http://live.euronext.com/en/resources/trading-hours-holidays> | Wayback `id_` replay of capture `20191210215605`, retrieved 2026-09-29 06:23 UTC | T1 | `35fbf47ffe78b754d080b27001f721eb4cee28dcfaacdc42123d614780f5a702` |
| `EURONEXT-HH-2020-12-05` | 2020-01-01 .. 2021-12-31 (the 2020 restatement and the 2021 per-market table) | <https://web.archive.org/web/20201205195829id_/https://live.euronext.com/en/resources/trading-hours-holidays> | Wayback `id_` replay of capture `20201205195829`, retrieved 2026-09-29 06:18 UTC | T1 | `1299b5f1048e91c927e1589eb0c36ea88dcfd6215f36b153fd35476bb3e64d9d` |
| `EURONEXT-HH-2022-05-23` | 2021-01-01 .. 2022-12-31 (the 2021 restatement and the 2022 per-market table) | <https://web.archive.org/web/20220523023131id_/https://live.euronext.com/en/resources/trading-hours-holidays> | Wayback `id_` replay of capture `20220523023131`, retrieved 2026-09-29 06:18 UTC | T1 | `c6dfa1fa31ae0a3ea7bcd09ab3707ffb50cc95ebc481334f01d0acd696b3348b` |
| `EURONEXT-HH-2023-11-27` | 2022-01-01 .. 2024-12-31 (the 2022 restatement, the 2023 and 2024 per-market tables) | <https://web.archive.org/web/20231127145642id_/https://live.euronext.com/en/resources/trading-hours-holidays> | Wayback `id_` replay of capture `20231127145642`, retrieved 2026-09-29 06:18 UTC | T1 | `550939767ce4869dc486c45e24f1b11c0276b6f53a8b4a5fa5f99e60c088a23c` |
| `EURONEXT-HH-2025-12-06` | 2025-01-01 .. 2025-12-31 (the page's 2025 table) | <https://web.archive.org/web/20251206154319id_/https://live.euronext.com/en/resources/trading-hours-holidays> | Wayback `id_` replay of capture `20251206154319`, retrieved 2026-09-28 01:50 UTC | T1 | `dc96c4f7f1e6a51cd2e6743385faa5cbc4156623ec45499c89e2ec3871398a18` |
| `EURONEXT-IF-2026` | 2026-01-01 .. 2026-12-31 | <https://connect2.euronext.com/sites/default/files/2025-11/IF251107CADE%202026%20Holiday%20Calendar%20for%20Euronexts%20Cash%20and%20Derivatives%20markets_1.pdf?VersionId=3ehe2.c9cMxv1K36.i6o4tz.5CAJmuw2> | retrieved 2026-09-28 01:09 | T1 | `617bc559510ef3a3d86d4a8b804422d0ba4c4dae43782a0a728d57a23fc4b1c5` |
| `EURONEXT-FCE-SPEC-2026-10-11` | the CAC 40 Index futures contract page (the family's Paris normal-week row) | <https://www.euronext.com/en/products/indices/paris/FCE> | retrieved 2026-10-11 | T1 | `02d46ecbdbf3e2c91e6fef6cc1ef87e05f55a3c87a4e95e98aae8a513b1f181a` |
| `EURONEXT-FTI-SPEC-2026-10-11` | the AEX Index futures contract page (the family's Amsterdam normal-week row) | <https://www.euronext.com/en/products/indices/amsterdam/FTI> | retrieved 2026-10-11 | T1 | `46196226abc331a593bf043723166cf10ac6a4e136f185e26a9adf49056bf646` |
| `EURONEXT-HH-LIVE-2026-10-11` | no rows keyed (the live page's 2026 table, re-read beside the contract pages) | <https://live.euronext.com/en/resources/trading-hours-holidays> | retrieved 2026-10-11 | T1 | `3537229c30525deecc5ce8d715db3ebb3f16416092d361cd1f2b440581b7fe3a` |

### 2010

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2010-01-01 | closed | New Year's Day: the group-wide list reaches `NYSE Euronext's European Cash and Derivatives markets` | `EURONEXT-PR-2010` | T1 | the operator's printed event date; one Paris civil day per trade date, so the conversion is the identity |
| 2010-04-02 | closed | Good Friday, the same list | `EURONEXT-PR-2010` | T1 | the operator's printed event date; one Paris civil day per trade date, so the conversion is the identity |
| 2010-04-05 | closed | Easter Monday, the same list | `EURONEXT-PR-2010` | T1 | the operator's printed event date; one Paris civil day per trade date, so the conversion is the identity |
| 2010-12-24 | unsourced | `cash markets will close at 1.00 pm GMT (2.00 pm CET) whilst NYSE Liffe will commence the closing of its derivatives contracts from midday GMT (1.00 pm CET)` — a derivatives arrangement whose per-product instants are unstated; no status claimed for the index futures' hours | `EURONEXT-PR-2010` | T1 | the operator's printed event date; one Paris civil day per trade date, so the conversion is the identity |
| 2010-12-31 | unsourced | the same derivatives closing-commencement sentence | `EURONEXT-PR-2010` | T1 | the operator's printed event date; one Paris civil day per trade date, so the conversion is the identity |

### 2011

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2011-04-22 | closed | Good Friday: the list reaches `… its Derivatives markets` | `EURONEXT-PR-2011` | T1 | the operator's printed event date; one Paris civil day per trade date, so the conversion is the identity |
| 2011-04-25 | closed | Easter Monday, the same list | `EURONEXT-PR-2011` | T1 | the operator's printed event date; one Paris civil day per trade date, so the conversion is the identity |
| 2011-12-26 | closed | the list's unnamed weekday closure (Boxing Day) | `EURONEXT-PR-2011` | T1 | the operator's printed event date; one Paris civil day per trade date, so the conversion is the identity |

### 2012

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2012-04-06 | unsourced | the banked 2012 edition is the `Calendar of Cash Business Days` notice, cash-markets scope; the derivatives arrangement is unstated, so the candidate date's status is withheld | `EURONEXT-TC-2012` | T1 | the operator's printed event date; one Paris civil day per trade date, so the conversion is the identity |
| 2012-04-09 | unsourced | the same cash-scope withholding | `EURONEXT-TC-2012` | T1 | the operator's printed event date; one Paris civil day per trade date, so the conversion is the identity |
| 2012-05-01 | unsourced | the same cash-scope withholding | `EURONEXT-TC-2012` | T1 | the operator's printed event date; one Paris civil day per trade date, so the conversion is the identity |
| 2012-12-24 | unsourced | the same cash-scope withholding (`the markets will close at 14:00 CET` is the cash scope) | `EURONEXT-TC-2012` | T1 | the operator's printed event date; one Paris civil day per trade date, so the conversion is the identity |
| 2012-12-25 | unsourced | the same cash-scope withholding | `EURONEXT-TC-2012` | T1 | the operator's printed event date; one Paris civil day per trade date, so the conversion is the identity |
| 2012-12-26 | unsourced | the same cash-scope withholding | `EURONEXT-TC-2012` | T1 | the operator's printed event date; one Paris civil day per trade date, so the conversion is the identity |
| 2012-12-31 | unsourced | the same cash-scope withholding | `EURONEXT-TC-2012` | T1 | the operator's printed event date; one Paris civil day per trade date, so the conversion is the identity |

### 2013

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2013-01-01 | unsourced | the banked 2013 editions state the cash-markets scope; the derivatives arrangement is unstated, so the candidate date's status is withheld | `EURONEXT-IF-2013` | T1 | the operator's printed event date; one Paris civil day per trade date, so the conversion is the identity |
| 2013-03-29 | unsourced | the same cash-scope withholding | `EURONEXT-IF-2013` | T1 | the operator's printed event date; one Paris civil day per trade date, so the conversion is the identity |
| 2013-04-01 | unsourced | the same cash-scope withholding | `EURONEXT-IF-2013` | T1 | the operator's printed event date; one Paris civil day per trade date, so the conversion is the identity |
| 2013-05-01 | unsourced | the same cash-scope withholding | `EURONEXT-IF-2013` | T1 | the operator's printed event date; one Paris civil day per trade date, so the conversion is the identity |
| 2013-12-24 | unsourced | the same cash-scope withholding (`close at 14:00 CET` is the cash scope) | `EURONEXT-IF-2013` | T1 | the operator's printed event date; one Paris civil day per trade date, so the conversion is the identity |
| 2013-12-25 | unsourced | the same cash-scope withholding | `EURONEXT-IF-2013` | T1 | the operator's printed event date; one Paris civil day per trade date, so the conversion is the identity |
| 2013-12-26 | unsourced | the same cash-scope withholding | `EURONEXT-IF-2013` | T1 | the operator's printed event date; one Paris civil day per trade date, so the conversion is the identity |
| 2013-12-31 | unsourced | the same cash-scope withholding | `EURONEXT-IF-2013` | T1 | the operator's printed event date; one Paris civil day per trade date, so the conversion is the identity |

### 2014

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2014-01-01 | closed | New Year's Day: the page's opening sentence covers `Cash markets … and its Derivatives markets` | `EURONEXT-HH-2014-01-12` | T1 | the operator's printed event date; one Paris civil day per trade date, so the conversion is the identity |
| 2014-04-18 | closed | Good Friday, the same list | `EURONEXT-HH-2014-01-12` | T1 | the operator's printed event date; one Paris civil day per trade date, so the conversion is the identity |
| 2014-04-21 | closed | Easter Monday, the same list | `EURONEXT-HH-2014-01-12` | T1 | the operator's printed event date; one Paris civil day per trade date, so the conversion is the identity |
| 2014-05-01 | closed | Labour Day, the same list | `EURONEXT-HH-2014-01-12` | T1 | the operator's printed event date; one Paris civil day per trade date, so the conversion is the identity |
| 2014-12-24 | unsourced | `Contracts on the Derivatives markets will close at various times, commencing at midday` — no per-product instant | `EURONEXT-HH-2014-01-12` | T1 | the operator's printed event date; one Paris civil day per trade date, so the conversion is the identity |
| 2014-12-25 | closed | Christmas Day, the same list | `EURONEXT-HH-2014-01-12` | T1 | the operator's printed event date; one Paris civil day per trade date, so the conversion is the identity |
| 2014-12-26 | closed | Boxing Day, the same list | `EURONEXT-HH-2014-01-12` | T1 | the operator's printed event date; one Paris civil day per trade date, so the conversion is the identity |
| 2014-12-31 | unsourced | the same derivatives various-times sentence | `EURONEXT-HH-2014-01-12` | T1 | the operator's printed event date; one Paris civil day per trade date, so the conversion is the identity |

### 2015

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2015-01-01 | closed | New Year's Day | `EURONEXT-HH-2015-03-22` | T1 | the operator's printed event date; one Paris civil day per trade date, so the conversion is the identity |
| 2015-04-03 | closed | Good Friday | `EURONEXT-HH-2015-03-22` | T1 | the operator's printed event date; one Paris civil day per trade date, so the conversion is the identity |
| 2015-04-06 | closed | Easter Monday | `EURONEXT-HH-2015-03-22` | T1 | the operator's printed event date; one Paris civil day per trade date, so the conversion is the identity |
| 2015-05-01 | closed | Labour Day | `EURONEXT-HH-2015-03-22` | T1 | the operator's printed event date; one Paris civil day per trade date, so the conversion is the identity |
| 2015-12-24 | unsourced | `Contracts on the Derivatives markets will close at various times, commencing at 13:55 CET` | `EURONEXT-HH-2015-03-22` | T1 | the operator's printed event date; one Paris civil day per trade date, so the conversion is the identity |
| 2015-12-25 | closed | Christmas Day | `EURONEXT-HH-2015-03-22` | T1 | the operator's printed event date; one Paris civil day per trade date, so the conversion is the identity |
| 2015-12-31 | closed | New Year's Eve, in the calendar's own closure list | `EURONEXT-HH-2015-03-22` | T1 | the operator's printed event date; one Paris civil day per trade date, so the conversion is the identity |

### 2016

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2016-01-01 | closed | New Year's Day: `Euronext's cash and derivatives markets will be open … except on the following days` | `EURONEXT-HH-2016-03-04` | T1 | the operator's printed event date; one Paris civil day per trade date, so the conversion is the identity |
| 2016-03-25 | closed | Good Friday, the same list | `EURONEXT-HH-2016-03-04` | T1 | the operator's printed event date; one Paris civil day per trade date, so the conversion is the identity |
| 2016-03-28 | closed | Easter Monday, the same list | `EURONEXT-HH-2016-03-04` | T1 | the operator's printed event date; one Paris civil day per trade date, so the conversion is the identity |
| 2016-12-26 | closed | Boxing Day; the eves print `trading on the Cash and Derivatives markets will close at the usual times`, so no eve rows | `EURONEXT-HH-2016-03-04` | T1 | the operator's printed event date; one Paris civil day per trade date, so the conversion is the identity |

### 2017

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2017-04-14 | closed | Good Friday: `Euronext's cash and derivatives markets will be open … except on the following days` | `EURONEXT-HH-2017-08-05` | T1 | the operator's printed event date; one Paris civil day per trade date, so the conversion is the identity |
| 2017-04-17 | closed | Easter Monday, the same list | `EURONEXT-HH-2017-08-05` | T1 | the operator's printed event date; one Paris civil day per trade date, so the conversion is the identity |
| 2017-05-01 | closed | Labour Day; 1 January fell on a Sunday and the calendar prints no substitute | `EURONEXT-HH-2017-08-05` | T1 | the operator's printed event date; one Paris civil day per trade date, so the conversion is the identity |
| 2017-12-25 | closed | Christmas Day, the same list | `EURONEXT-HH-2017-08-05` | T1 | the operator's printed event date; one Paris civil day per trade date, so the conversion is the identity |
| 2017-12-26 | closed | Boxing Day; the eves print `trading on the Cash and Derivatives markets will close at the usual times`, so no eve rows | `EURONEXT-HH-2017-08-05` | T1 | the operator's printed event date; one Paris civil day per trade date, so the conversion is the identity |

### 2018

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2018-01-01 | closed | New Year's Day: `Euronext's Cash and Derivatives markets will be open … except on the following days` | `EURONEXT-HH-2018-08-26` | T1 | the operator's printed event date; one Paris civil day per trade date, so the conversion is the identity |
| 2018-03-30 | closed | Good Friday, the same list | `EURONEXT-HH-2018-08-26` | T1 | the operator's printed event date; one Paris civil day per trade date, so the conversion is the identity |
| 2018-04-02 | closed | Easter Monday, the same list | `EURONEXT-HH-2018-08-26` | T1 | the operator's printed event date; one Paris civil day per trade date, so the conversion is the identity |
| 2018-05-01 | closed | Labour Day, the same list | `EURONEXT-HH-2018-08-26` | T1 | the operator's printed event date; one Paris civil day per trade date, so the conversion is the identity |
| 2018-12-24 | unsourced | `Contracts on the Derivatives markets will close at various times` — no per-product instant | `EURONEXT-HH-2018-08-26` | T1 | the operator's printed event date; one Paris civil day per trade date, so the conversion is the identity |
| 2018-12-25 | closed | Christmas Day, the same list | `EURONEXT-HH-2018-08-26` | T1 | the operator's printed event date; one Paris civil day per trade date, so the conversion is the identity |
| 2018-12-26 | closed | Boxing Day, the same list | `EURONEXT-HH-2018-08-26` | T1 | the operator's printed event date; one Paris civil day per trade date, so the conversion is the identity |
| 2018-12-31 | unsourced | the same derivatives various-times sentence | `EURONEXT-HH-2018-08-26` | T1 | the operator's printed event date; one Paris civil day per trade date, so the conversion is the identity |

### 2019

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2019-01-01 | closed | New Year's Day: `Euronext's Cash and Derivatives markets will be open … except on the following days` | `EURONEXT-HH-2019-03-25` | T1 | the operator's printed event date; one Paris civil day per trade date, so the conversion is the identity |
| 2019-04-19 | closed | Good Friday, the same list | `EURONEXT-HH-2019-03-25` | T1 | the operator's printed event date; one Paris civil day per trade date, so the conversion is the identity |
| 2019-04-22 | closed | Easter Monday, the same list | `EURONEXT-HH-2019-03-25` | T1 | the operator's printed event date; one Paris civil day per trade date, so the conversion is the identity |
| 2019-05-01 | closed | Labour Day, the same list | `EURONEXT-HH-2019-03-25` | T1 | the operator's printed event date; one Paris civil day per trade date, so the conversion is the identity |
| 2019-12-24 | unsourced | `Contracts on the Derivatives Markets will close at various times, commencing at 13:55 CET` | `EURONEXT-HH-2019-12-10` | T1 | the operator's printed event date; one Paris civil day per trade date, so the conversion is the identity |
| 2019-12-25 | closed | Christmas Day, the same list | `EURONEXT-HH-2019-03-25` | T1 | the operator's printed event date; one Paris civil day per trade date, so the conversion is the identity |
| 2019-12-26 | closed | Boxing Day, the same list | `EURONEXT-HH-2019-03-25` | T1 | the operator's printed event date; one Paris civil day per trade date, so the conversion is the identity |
| 2019-12-31 | unsourced | the same derivatives various-times sentence | `EURONEXT-HH-2019-12-10` | T1 | the operator's printed event date; one Paris civil day per trade date, so the conversion is the identity |

### 2020

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2020-01-01 | closed | New Year's Day: `The Euronext Cash Markets, Euronext Block, and the Euronext Derivatives Markets will be open … except on the following days` | `EURONEXT-HH-2019-12-10` | T1 | the operator's printed event date; one Paris civil day per trade date, so the conversion is the identity |
| 2020-04-10 | closed | Good Friday, the same list | `EURONEXT-HH-2019-12-10` | T1 | the operator's printed event date; one Paris civil day per trade date, so the conversion is the identity |
| 2020-04-13 | closed | Easter Monday, the same list | `EURONEXT-HH-2019-12-10` | T1 | the operator's printed event date; one Paris civil day per trade date, so the conversion is the identity |
| 2020-05-01 | closed | Labour Day, the same list | `EURONEXT-HH-2019-12-10` | T1 | the operator's printed event date; one Paris civil day per trade date, so the conversion is the identity |
| 2020-12-24 | unsourced | `Contracts on the Derivatives Markets will close at various times, commencing at 13:55 CET`; the detailed instants live in the operator's derivatives appendix, which is not banked | `EURONEXT-HH-2019-12-10` | T1 | the operator's printed event date; one Paris civil day per trade date, so the conversion is the identity |
| 2020-12-25 | closed | Christmas Day, the same list | `EURONEXT-HH-2019-12-10` | T1 | the operator's printed event date; one Paris civil day per trade date, so the conversion is the identity |
| 2020-12-31 | unsourced | the same derivatives various-times sentence | `EURONEXT-HH-2019-12-10` | T1 | the operator's printed event date; one Paris civil day per trade date, so the conversion is the identity |

### 2021

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2021-01-01 | closed | New Year's Day: the 2021 list's own scope sentence; the Dublin and Oslo rows are Dublin-only and Oslo-only | `EURONEXT-HH-2020-12-05` | T1 | the operator's printed event date; one Paris civil day per trade date, so the conversion is the identity |
| 2021-04-02 | closed | Good Friday, the same list | `EURONEXT-HH-2020-12-05` | T1 | the operator's printed event date; one Paris civil day per trade date, so the conversion is the identity |
| 2021-04-05 | closed | Easter Monday, the same list | `EURONEXT-HH-2020-12-05` | T1 | the operator's printed event date; one Paris civil day per trade date, so the conversion is the identity |
| 2021-12-24 | unsourced | `Contracts on the Derivatives Markets will close at various times, starting at 13:55 CET` | `EURONEXT-HH-2020-12-05` | T1 | the operator's printed event date; one Paris civil day per trade date, so the conversion is the identity |
| 2021-12-31 | unsourced | the same derivatives various-times sentence | `EURONEXT-HH-2020-12-05` | T1 | the operator's printed event date; one Paris civil day per trade date, so the conversion is the identity |

### 2022

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2022-04-15 | closed | Good Friday: the Paris cell of the shared per-market table | `EURONEXT-HH-2022-05-23` | T1 | the operator's printed event date; one Paris civil day per trade date, so the conversion is the identity |
| 2022-04-18 | closed | Easter Monday: the Paris cell | `EURONEXT-HH-2022-05-23` | T1 | the operator's printed event date; one Paris civil day per trade date, so the conversion is the identity |
| 2022-12-26 | closed | Stephen's Day / Boxing Day: the Paris cell; 12-23 and 12-30 print `Full Day Trading` for Paris (Dublin's substitutes), so no eve rows | `EURONEXT-HH-2022-05-23` | T1 | the operator's printed event date; one Paris civil day per trade date, so the conversion is the identity |

### 2023

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2023-04-07 | closed | Good Friday: the Paris cell | `EURONEXT-HH-2023-11-27` | T1 | the operator's printed event date; one Paris civil day per trade date, so the conversion is the identity |
| 2023-04-10 | closed | Easter Monday: the Paris cell | `EURONEXT-HH-2023-11-27` | T1 | the operator's printed event date; one Paris civil day per trade date, so the conversion is the identity |
| 2023-05-01 | closed | May Day: the Paris cell | `EURONEXT-HH-2023-11-27` | T1 | the operator's printed event date; one Paris civil day per trade date, so the conversion is the identity |
| 2023-12-25 | closed | Christmas: the Paris cell | `EURONEXT-HH-2023-11-27` | T1 | the operator's printed event date; one Paris civil day per trade date, so the conversion is the identity |
| 2023-12-26 | closed | St Stephens Day / Boxing Day: the Paris cell; 12-22 and 12-29 print `Full Day Trading` for Paris, so no eve rows | `EURONEXT-HH-2023-11-27` | T1 | the operator's printed event date; one Paris civil day per trade date, so the conversion is the identity |

### 2024

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2024-01-01 | closed | New Year's Day: the Paris cell of the 2024 `Cash and Derivatives` table | `EURONEXT-HH-2023-11-27` | T1 | the operator's printed event date; one Paris civil day per trade date, so the conversion is the identity |
| 2024-03-29 | closed | Good Friday: the Paris cell | `EURONEXT-HH-2023-11-27` | T1 | the operator's printed event date; one Paris civil day per trade date, so the conversion is the identity |
| 2024-04-01 | closed | Easter Monday: the Paris cell | `EURONEXT-HH-2023-11-27` | T1 | the operator's printed event date; one Paris civil day per trade date, so the conversion is the identity |
| 2024-05-01 | closed | Labour Day: the Paris cell | `EURONEXT-HH-2023-11-27` | T1 | the operator's printed event date; one Paris civil day per trade date, so the conversion is the identity |
| 2024-12-24 | unsourced | the Paris cell prints `**Half Trading Day`; the banked end-of-year appendix states the cash manual's Paris equity segments only, so no derivatives instant is claimed | `EURONEXT-HH-2023-11-27` | T1 | the operator's printed event date; one Paris civil day per trade date, so the conversion is the identity |
| 2024-12-25 | closed | Christmas: the Paris cell | `EURONEXT-HH-2023-11-27` | T1 | the operator's printed event date; one Paris civil day per trade date, so the conversion is the identity |
| 2024-12-26 | closed | St Stephens Day / Boxing Day: the Paris cell | `EURONEXT-HH-2023-11-27` | T1 | the operator's printed event date; one Paris civil day per trade date, so the conversion is the identity |
| 2024-12-31 | unsourced | the same `**Half Trading Day` Paris cell and unstated derivatives instants | `EURONEXT-HH-2023-11-27` | T1 | the operator's printed event date; one Paris civil day per trade date, so the conversion is the identity |

### 2025

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2025-01-01 | closed | New Year's Day: the Paris cell of the 2025 `Cash and Derivatives` table | `EURONEXT-HH-2025-12-06` | T1 | the operator's printed event date; one Paris civil day per trade date, so the conversion is the identity |
| 2025-04-18 | closed | Good Friday: the Paris cell | `EURONEXT-HH-2025-12-06` | T1 | the operator's printed event date; one Paris civil day per trade date, so the conversion is the identity |
| 2025-04-21 | closed | Easter Monday: the Paris cell | `EURONEXT-HH-2025-12-06` | T1 | the operator's printed event date; one Paris civil day per trade date, so the conversion is the identity |
| 2025-05-01 | closed | Labour Day: the Paris cell | `EURONEXT-HH-2025-12-06` | T1 | the operator's printed event date; one Paris civil day per trade date, so the conversion is the identity |
| 2025-12-24 | unsourced | the Paris cell prints `**Half Trading Day`; the banked appendix's 4-03 sheet states the fund/NAV facility only, so no derivatives instant is claimed | `EURONEXT-HH-2025-12-06` | T1 | the operator's printed event date; one Paris civil day per trade date, so the conversion is the identity |
| 2025-12-25 | closed | Christmas: the Paris cell | `EURONEXT-HH-2025-12-06` | T1 | the operator's printed event date; one Paris civil day per trade date, so the conversion is the identity |
| 2025-12-26 | closed | St Stephen's Day / Boxing Day: the Paris cell | `EURONEXT-HH-2025-12-06` | T1 | the operator's printed event date; one Paris civil day per trade date, so the conversion is the identity |
| 2025-12-31 | unsourced | the same `**Half Trading Day` Paris cell and unstated derivatives instants | `EURONEXT-HH-2025-12-06` | T1 | the operator's printed event date; one Paris civil day per trade date, so the conversion is the identity |

### 2026

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2026-01-01 | closed | New Year's Day: the INFO-FLASH Paris cell (`2026 Holiday Calendar for Euronext's Cash and Derivatives markets`) | `EURONEXT-IF-2026` | T1 | the operator's printed event date; one Paris civil day per trade date, so the conversion is the identity |
| 2026-04-03 | closed | Good Friday: the INFO-FLASH Paris cell | `EURONEXT-IF-2026` | T1 | the operator's printed event date; one Paris civil day per trade date, so the conversion is the identity |
| 2026-04-06 | closed | Easter Monday: the INFO-FLASH Paris cell | `EURONEXT-IF-2026` | T1 | the operator's printed event date; one Paris civil day per trade date, so the conversion is the identity |
| 2026-05-01 | closed | Labour Day: the INFO-FLASH Paris cell | `EURONEXT-IF-2026` | T1 | the operator's printed event date; one Paris civil day per trade date, so the conversion is the identity |
| 2026-12-24 | unsourced | Paris `**Half Trading Day` with the hours announced and unstated (`To be announced`); no instant is claimed | `EURONEXT-IF-2026` | T1 | the operator's printed event date; one Paris civil day per trade date, so the conversion is the identity |
| 2026-12-25 | closed | Christmas: the INFO-FLASH Paris cell | `EURONEXT-IF-2026` | T1 | the operator's printed event date; one Paris civil day per trade date, so the conversion is the identity |
| 2026-12-31 | unsourced | Paris `**Half Trading Day` with the hours announced and unstated; no instant is claimed | `EURONEXT-IF-2026` | T1 | the operator's printed event date; one Paris civil day per trade date, so the conversion is the identity |

The sixteen shared rows are copied byte for byte from [euronext_paris](euronext_paris.md) § Documents, whose store paths and retrieval stamps they inherit; the three index-futures rows above are the store's `normal-weeks/euronext-index-futures-2026-10-11/` artifacts (store PR #15, retrieval 2026-10-11).

## Sources

- The two contract-specification pages (`EURONEXT-FCE-SPEC-2026-10-11`, `EURONEXT-FTI-SPEC-2026-10-11`) — T1, the family's normal-week sources and its monitoring entry points; each states `Trading hours Central Order Book: 08:00 - 22:00 CET/CEST`, `Large-in-Scale Facility: 07:30 - 22:00 CET/CEST`, and the clearing note that the Day session `[07:30 - 18:30]` and Evening session `[18:30 - 22:00]` clear on trade day T.
- The [trading-hours-holidays page](https://live.euronext.com/en/resources/trading-hours-holidays) (`EURONEXT-HH-LIVE-2026-10-11`) — T1, the holiday calendar's living entry point, read beside the contract pages on 2026-10-11.
- The shared per-year calendar corpus, resolved in the Documents table above — T1.

## Gaps and residual risks

- **The Amsterdam (and Paris) normal-week floor: the family's horizon is the 2026-10-11 retrieval.** The two contract pages are the only banked artifacts that state the CAC 40 and AEX index-futures grid, and neither dates its own state: no banked artifact witnesses any earlier grid for either market, so every date below 2026-10-11 refuses through the coverage contract (`NormalWeekCarried`) instead of serving the current grid as an inferred history. This is the Amsterdam normal-week floor gap #332 records; the Paris side has the same shape and is disclosed here beside it. Named closer: an archived contract-specification or rulebook edition for either market that prints the Central Order Book hours with its own date (the operator's Euronext Instructions 4-03 editions, or a Wayback capture of the contract pages); each such artifact moves the horizon and keys a dated history. Admission under LAW-SERVICE-TIERS never improves this evidence: the family serves current routing, and its historical span stays the explicit refusal.
- **The 2012–2013 derivatives scope.** The banked 2012–2013 documents are the cash-markets notice and Info-Flash; the derivatives markets' own calendars for those two years (NYSE Liffe-era documents) are not banked, so the candidate dates ship `unsourced` and the residual is this disclosure. Named closer: the derivatives (Liffe) holiday calendar for 2012 and 2013 — an archived Liffe notice or the group calendar at a derivatives-named scope — which would key the years' rows or verify them normal.
- **The December half-day instants.** From 2014 the derivatives eves are stated only as a group closing `at various times` from a lower bound, and from 2022 the per-market cells defer to the end-of-year appendices whose banked editions state the cash segments only; the index futures' own half-day closes are therefore `unsourced` on every eve that has one (2010, 2014–2015, 2018–2021, 2024–2026). Named closer: a derivatives-scoped end-of-year appendix (an Instructions 4-03 edition printing the index-derivatives segments) or an archived derivatives-hours notice per year.
- **The Large-in-Scale Facility is out of scope, recorded here.** The contract pages state `Large-in-Scale Facility: 07:30 - 22:00 CET/CEST` beside the Central Order Book row. LiSF is Euronext's off-book wholesale facility for large-in-scale trades — the same kind of off-book system the Eurex families exclude when they decline to model TES/OTC block columns — so the family's session is the Central Order Book's 08:00–22:00 grid. The pages' clearing note names a Day session `[07:30 - 18:30]` and an Evening session `[18:30 - 22:00]` in its clearing sentence; the Trading-hours row that states the order book's own hours is the family's session language, and the 07:30 leg the clearing sentence reflects is the LiSF open.
- **The Euronext Block and Oslo/Dublin/Milan rows.** The scope sentences name Euronext Block beside the cash and derivatives markets; Block is a separate trading venue, not this family's market, and its exclusion is the documented scope. The Oslo, Dublin and Milan rows print `Full Trading Day` for Paris and are not keyed.
- **Holiday horizon.** The audited window stops at 2026-12-31: the operator has published nothing for 2027 (verified 2026-09-28 in the sibling file's sweep, re-read 2026-10-11). Closing condition: Euronext's 2027 calendar; the row is re-checked monthly per LAW-WATCH.
- **The 2026 INFO-FLASH's printed header date reads `07 November 2026`** while its publishing path and file name say 2025-11-07; both are recorded as printed and neither dates any row — the document is cited only for its table cells, exactly as the sibling file records.
