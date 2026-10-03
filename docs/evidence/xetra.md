<!-- SPDX-License-Identifier: MIT-0 -->

# `xetra` — evidence

- **Kind:** `Exchange`
- **Owner module:** [`xetra.rs`](../../src/calendar/schedules/equities/europe/xetra.rs)
- **Source sets:** [`EU-XETRA`](../schedules/sources.md#eu-xetra), [`EU-FESE-SECONDARY`](../schedules/sources.md#eu-fese-secondary)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

DAX constituent-share envelope with its January-2010 intraday auction, 2020-11-24 Trade-at-Close launch, and participant-restricted Extended Retail from 2025-12-01.

## Revision rows

- 2020-11-24 — T1 — Deutsche Börse Trade-at-Close press release — executable Trade-at-Close inserted after the DAX closing auction through 17:45, post-trading pushed back to 17:45.
- 2025-12-01 — T1 — Deutsche Börse Extended Xetra Retail circular — envelope opens at 07:00, Trade-at-Close ends 17:40, participant-restricted late retail through 22:00, post-trading to 22:05.

## Holidays

**Coverage:** 2010-01-01..2024-12-31, 2025-01-01..2027-12-31 (inclusive venue-local trade dates in `Europe/Berlin`; tier T1 throughout).

The rows key on Deutsche Börse's own cash-market trading calendar. For 2010-2024 each year's closure set is stated in one sentence on the operator's per-year `Trading calendar` PDF — at FWB® Frankfurter Wertpapierbörse "there will be trading Mondays to Fridays in \<year\>, with the exception of ..." — and 2010-2014 rows cite the `Trading Calendar <year>` grid editions (`DB-TC-PDF-<year>`, the `DB_HK_<year>` documents of the xetra.com and deutsche-boerse.com archives) while 2015-2024 rows cite the `xetra-trading-calendar-<year>` PDFs (`DB-TC-PDF-<year>`); the English dispatch page editions of 2012-2014 (`DB-TC-PAGE-<year>`) restate the same sentences and corroborate. For 2025-2027 the rows key on the `Non-trading days at Frankfurter Wertpapierbörse (FWB®) ... (Xetra and Börse Frankfurt)` table of the operator's `Trading calendar and trading hours` page and the per-year PDFs that restate each year's set in the same one-sentence form: the 2025-04-22 capture of the page (`DB-TC-PAGE-2025`) and the `Trading calendar 2025` PDF it links (`DB-TC-PDF-2025`) govern 2025; the live page (`DB-TC-PAGE`) and its `Trading calendar 2026` PDF (`DB-TC-PDF-2026`) govern 2026 and 2027.

Three readings of the historical sentences are the operator's own, not the crate's. First, a "settlement day" is a **closure**: the sentence places the day in the exception list — the market does not trade — while noting that settlement (Clearstream) runs; this is the same statement the modern `**` footnote prints as "No trading but settlement is open", and both eras ship `Closed`. Second, the sentence is the year's complete closure statement, so its omissions trade: 2011 names no 3 October (a Monday that year) and 2013 names no 3 October (a Thursday), so the exchange traded those German Unity Days, and from 2022 the sentences stop naming Whit Monday and German Unity Day altogether, so those days answer as ordinary trading days from 2022 on. Third, a weekend-falling exception date is deleted by the sentence's own "Mondays to Fridays" premise and keys no weekday row.

Two further readings govern 2025-2027. First, Christmas Eve and New Year's Eve are **closures**: the `**` footnote reads "No trading but settlement is open", which states that the market does not trade. Second, the live page names 2026's trading holidays — `Ascension Day (14 May 2026)`, `Whit Monday (25 May 2026)`, `Corpus Christi (4 June 2026)` — and states `Trading of shares and Exchange traded products on Frankfurt and Xetra ends on public holidays (Germany and State of Hesse) where trading takes place according to the FWB trading calendar at 20:00 CET`, so those three dates carry an early close at 20:00 with the printed instant. The 2025 capture words the same note over `Börse Frankfurt` only, so no 2025 Xetra early close is sourced and none ships; the 2025 trading holidays (Ascension 29 May, Whit Monday 9 June, Corpus Christi 19 June, German Unity Day 3 October) ship closures-free as ordinary days for this identity. The live page's conditional note `On December 30, 2026, deviating trading hours may apply` — echoed by the 2026 PDF's `*)` footnote `Trading hours may differ from normal trading days` — keys no row (LAW-NO-FABRICATED-DATES).

Each year section below quotes the year's sentence in full; the operator prints no named holiday beside the dates, so a row's instant column names the exception date and the sentence is the whole statement.

### 2010

The 2010 calendar sentence: at FWB® Frankfurter Wertpapierbörse “there will be trading Mondays to Fridays in 2010, with the exception of 1 January, 2 April, 5 April, 24 December and 31 December. 24 and 31 December are settlement days.”


| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2010-01-01 | closed | `1 January` — an exception date of the year's sentence | `DB-TC-PDF-2010` | T1 | the year's sentence deletes the day |
| 2010-04-02 | closed | `2 April` — an exception date of the year's sentence | `DB-TC-PDF-2010` | T1 | the year's sentence deletes the day |
| 2010-04-05 | closed | `5 April` — an exception date of the year's sentence | `DB-TC-PDF-2010` | T1 | the year's sentence deletes the day |
| 2010-12-24 | closed | `24 December` — an exception date of the year's sentence | `DB-TC-PDF-2010` | T1 | the year's sentence deletes the day and states it a settlement day: no trading, settlement only |
| 2010-12-31 | closed | `31 December` — an exception date of the year's sentence | `DB-TC-PDF-2010` | T1 | the year's sentence deletes the day and states it a settlement day: no trading, settlement only |
### 2011

The 2011 calendar sentence: at FWB® Frankfurter Wertpapierbörse “there will be trading Mondays to Fridays in 2011, with the exception of 22 April, 25 April and 26 December.”


| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2011-04-22 | closed | `22 April` — an exception date of the year's sentence | `DB-TC-PDF-2011` | T1 | the year's sentence deletes the day |
| 2011-04-25 | closed | `25 April` — an exception date of the year's sentence | `DB-TC-PDF-2011` | T1 | the year's sentence deletes the day |
| 2011-12-26 | closed | `26 December` — an exception date of the year's sentence | `DB-TC-PDF-2011` | T1 | the year's sentence deletes the day |
### 2012

The 2012 calendar sentence: at FWB® Frankfurter Wertpapierbörse “there will be trading Mondays to Fridays in 2012, with the exception of 6 April, 9 April, 1 May, 24 December, 25 December, 26 December and 31 December. 24 and 31 December are settlement days.”


| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2012-04-06 | closed | `6 April` — an exception date of the year's sentence | `DB-TC-PDF-2012` | T1 | the year's sentence deletes the day |
| 2012-04-09 | closed | `9 April` — an exception date of the year's sentence | `DB-TC-PDF-2012` | T1 | the year's sentence deletes the day |
| 2012-05-01 | closed | `1 May` — an exception date of the year's sentence | `DB-TC-PDF-2012` | T1 | the year's sentence deletes the day |
| 2012-12-24 | closed | `24 December` — an exception date of the year's sentence | `DB-TC-PDF-2012` | T1 | the year's sentence deletes the day and states it a settlement day: no trading, settlement only |
| 2012-12-25 | closed | `25 December` — an exception date of the year's sentence | `DB-TC-PDF-2012` | T1 | the year's sentence deletes the day |
| 2012-12-26 | closed | `26 December` — an exception date of the year's sentence | `DB-TC-PDF-2012` | T1 | the year's sentence deletes the day |
| 2012-12-31 | closed | `31 December` — an exception date of the year's sentence | `DB-TC-PDF-2012` | T1 | the year's sentence deletes the day and states it a settlement day: no trading, settlement only |
### 2013

The 2013 calendar sentence: at FWB® Frankfurter Wertpapierbörse “there will be trading Mondays to Fridays in 2013, with the exception of 1 January, 29 March, 1 April, 1 May, 24 December, 25 December, 26 December and 31 December. 24 and 31 December are settlement days.”


| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2013-01-01 | closed | `1 January` — an exception date of the year's sentence | `DB-TC-PDF-2013` | T1 | the year's sentence deletes the day |
| 2013-03-29 | closed | `29 March` — an exception date of the year's sentence | `DB-TC-PDF-2013` | T1 | the year's sentence deletes the day |
| 2013-04-01 | closed | `1 April` — an exception date of the year's sentence | `DB-TC-PDF-2013` | T1 | the year's sentence deletes the day |
| 2013-05-01 | closed | `1 May` — an exception date of the year's sentence | `DB-TC-PDF-2013` | T1 | the year's sentence deletes the day |
| 2013-12-24 | closed | `24 December` — an exception date of the year's sentence | `DB-TC-PDF-2013` | T1 | the year's sentence deletes the day and states it a settlement day: no trading, settlement only |
| 2013-12-25 | closed | `25 December` — an exception date of the year's sentence | `DB-TC-PDF-2013` | T1 | the year's sentence deletes the day |
| 2013-12-26 | closed | `26 December` — an exception date of the year's sentence | `DB-TC-PDF-2013` | T1 | the year's sentence deletes the day |
| 2013-12-31 | closed | `31 December` — an exception date of the year's sentence | `DB-TC-PDF-2013` | T1 | the year's sentence deletes the day and states it a settlement day: no trading, settlement only |
### 2014

The 2014 keyed document is the calendar’s German edition (`DB_HK_2014`); its sentence: „An der Frankfurter Wertpapierbörse (FWB®) wird im Jahr 2014 von montags bis freitags außer am 1. Januar, 18. April, 21. April, 1. Mai, 3. Oktober, 24. Dezember, 25. Dezember, 26. Dezember und am 31. Dezember gehandelt. Der 3. Oktober sowie der 24. und 31. Dezember sind Erfüllungstage.“. The English dispatch page edition (`DB-TC-PAGE-2014`) restates it: “at Frankfurter Wertpapierbörse (FWB®, the Frankfurt Stock Exchange) there will be trading Mondays to Fridays in 2014, with the exception of 1 January, 18 April, 21 April, 1 May, 3 October, 24 December, 25 December, 26 December and 31 December. 3 October, 24 December and 31 December are settlement days.”


| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2014-01-01 | closed | `1 January` — an exception date of the year's sentence | `DB-TC-PDF-2014` | T1 | the year's sentence deletes the day |
| 2014-04-18 | closed | `18 April` — an exception date of the year's sentence | `DB-TC-PDF-2014` | T1 | the year's sentence deletes the day |
| 2014-04-21 | closed | `21 April` — an exception date of the year's sentence | `DB-TC-PDF-2014` | T1 | the year's sentence deletes the day |
| 2014-05-01 | closed | `1 May` — an exception date of the year's sentence | `DB-TC-PDF-2014` | T1 | the year's sentence deletes the day |
| 2014-10-03 | closed | `3 October` — an exception date of the year's sentence | `DB-TC-PDF-2014` | T1 | the year's sentence deletes the day and states it a settlement day: no trading, settlement only |
| 2014-12-24 | closed | `24 December` — an exception date of the year's sentence | `DB-TC-PDF-2014` | T1 | the year's sentence deletes the day and states it a settlement day: no trading, settlement only |
| 2014-12-25 | closed | `25 December` — an exception date of the year's sentence | `DB-TC-PDF-2014` | T1 | the year's sentence deletes the day |
| 2014-12-26 | closed | `26 December` — an exception date of the year's sentence | `DB-TC-PDF-2014` | T1 | the year's sentence deletes the day |
| 2014-12-31 | closed | `31 December` — an exception date of the year's sentence | `DB-TC-PDF-2014` | T1 | the year's sentence deletes the day and states it a settlement day: no trading, settlement only |
### 2015

The 2015 calendar sentence: at FWB® Frankfurter Wertpapierbörse “there will be trading Mondays to Fridays in 2015, with the exception of 1 January, 3 April, 6 April, 1 May, 25 May, 24 December, 25 December and 31 December. 25 May, 24 December and 31 December are settlement days.”


| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2015-01-01 | closed | `1 January` — an exception date of the year's sentence | `DB-TC-PDF-2015` | T1 | the year's sentence deletes the day |
| 2015-04-03 | closed | `3 April` — an exception date of the year's sentence | `DB-TC-PDF-2015` | T1 | the year's sentence deletes the day |
| 2015-04-06 | closed | `6 April` — an exception date of the year's sentence | `DB-TC-PDF-2015` | T1 | the year's sentence deletes the day |
| 2015-05-01 | closed | `1 May` — an exception date of the year's sentence | `DB-TC-PDF-2015` | T1 | the year's sentence deletes the day |
| 2015-05-25 | closed | `25 May` — an exception date of the year's sentence | `DB-TC-PDF-2015` | T1 | the year's sentence deletes the day and states it a settlement day: no trading, settlement only |
| 2015-12-24 | closed | `24 December` — an exception date of the year's sentence | `DB-TC-PDF-2015` | T1 | the year's sentence deletes the day and states it a settlement day: no trading, settlement only |
| 2015-12-25 | closed | `25 December` — an exception date of the year's sentence | `DB-TC-PDF-2015` | T1 | the year's sentence deletes the day |
| 2015-12-31 | closed | `31 December` — an exception date of the year's sentence | `DB-TC-PDF-2015` | T1 | the year's sentence deletes the day and states it a settlement day: no trading, settlement only |
### 2016

The 2016 calendar sentence: at FWB® Frankfurter Wertpapierbörse “there will be trading Mondays to Fridays in 2016, with the exception of 1 January, 25 March, 28 March, 16 May, 3 October, 26 December. 16 May and 3 October are settlement days.”


| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2016-01-01 | closed | `1 January` — an exception date of the year's sentence | `DB-TC-PDF-2016` | T1 | the year's sentence deletes the day |
| 2016-03-25 | closed | `25 March` — an exception date of the year's sentence | `DB-TC-PDF-2016` | T1 | the year's sentence deletes the day |
| 2016-03-28 | closed | `28 March` — an exception date of the year's sentence | `DB-TC-PDF-2016` | T1 | the year's sentence deletes the day |
| 2016-05-16 | closed | `16 May` — an exception date of the year's sentence | `DB-TC-PDF-2016` | T1 | the year's sentence deletes the day and states it a settlement day: no trading, settlement only |
| 2016-10-03 | closed | `3 October` — an exception date of the year's sentence | `DB-TC-PDF-2016` | T1 | the year's sentence deletes the day and states it a settlement day: no trading, settlement only |
| 2016-12-26 | closed | `26 December` — an exception date of the year's sentence | `DB-TC-PDF-2016` | T1 | the year's sentence deletes the day |
### 2017

The 2017 calendar sentence: at FWB® Frankfurter Wertpapierbörse “there will be trading Mondays to Fridays in 2017, with the exception of 14 April, 17 April, 1 May, 5 June, 3 October, 31 October, 25 December and 26 December. 5 June, 3 October and 31 October are settlement days.”


| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2017-04-14 | closed | `14 April` — an exception date of the year's sentence | `DB-TC-PDF-2017` | T1 | the year's sentence deletes the day |
| 2017-04-17 | closed | `17 April` — an exception date of the year's sentence | `DB-TC-PDF-2017` | T1 | the year's sentence deletes the day |
| 2017-05-01 | closed | `1 May` — an exception date of the year's sentence | `DB-TC-PDF-2017` | T1 | the year's sentence deletes the day |
| 2017-06-05 | closed | `5 June` — an exception date of the year's sentence | `DB-TC-PDF-2017` | T1 | the year's sentence deletes the day and states it a settlement day: no trading, settlement only |
| 2017-10-03 | closed | `3 October` — an exception date of the year's sentence | `DB-TC-PDF-2017` | T1 | the year's sentence deletes the day and states it a settlement day: no trading, settlement only |
| 2017-10-31 | closed | `31 October` — an exception date of the year's sentence | `DB-TC-PDF-2017` | T1 | the year's sentence deletes the day and states it a settlement day: no trading, settlement only |
| 2017-12-25 | closed | `25 December` — an exception date of the year's sentence | `DB-TC-PDF-2017` | T1 | the year's sentence deletes the day |
| 2017-12-26 | closed | `26 December` — an exception date of the year's sentence | `DB-TC-PDF-2017` | T1 | the year's sentence deletes the day |
### 2018

The 2018 calendar sentence: at FWB® Frankfurter Wertpapierbörse “there will be trading Mondays to Fridays in 2018, with the exception of 1 January, 30 March, 2 April, 1 May, 21 May, 3 October, 24 December, 25 December, 26 December and 31 December. 21 May, 3 October, 24 December and 31 December are settlement days.”


| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2018-01-01 | closed | `1 January` — an exception date of the year's sentence | `DB-TC-PDF-2018` | T1 | the year's sentence deletes the day |
| 2018-03-30 | closed | `30 March` — an exception date of the year's sentence | `DB-TC-PDF-2018` | T1 | the year's sentence deletes the day |
| 2018-04-02 | closed | `2 April` — an exception date of the year's sentence | `DB-TC-PDF-2018` | T1 | the year's sentence deletes the day |
| 2018-05-01 | closed | `1 May` — an exception date of the year's sentence | `DB-TC-PDF-2018` | T1 | the year's sentence deletes the day |
| 2018-05-21 | closed | `21 May` — an exception date of the year's sentence | `DB-TC-PDF-2018` | T1 | the year's sentence deletes the day and states it a settlement day: no trading, settlement only |
| 2018-10-03 | closed | `3 October` — an exception date of the year's sentence | `DB-TC-PDF-2018` | T1 | the year's sentence deletes the day and states it a settlement day: no trading, settlement only |
| 2018-12-24 | closed | `24 December` — an exception date of the year's sentence | `DB-TC-PDF-2018` | T1 | the year's sentence deletes the day and states it a settlement day: no trading, settlement only |
| 2018-12-25 | closed | `25 December` — an exception date of the year's sentence | `DB-TC-PDF-2018` | T1 | the year's sentence deletes the day |
| 2018-12-26 | closed | `26 December` — an exception date of the year's sentence | `DB-TC-PDF-2018` | T1 | the year's sentence deletes the day |
| 2018-12-31 | closed | `31 December` — an exception date of the year's sentence | `DB-TC-PDF-2018` | T1 | the year's sentence deletes the day and states it a settlement day: no trading, settlement only |
### 2019

The 2019 calendar sentence: at FWB® Frankfurter Wertpapierbörse “there will be trading Mondays to Fridays in 2019, with the exception of 1 January, 19 April, 22 April, 1 May, 10 June, 3 October, 24 December, 25 December, 26 December and 31 December. 10 June, 3 October, 24 December and 31 December are settlement days.”


| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2019-01-01 | closed | `1 January` — an exception date of the year's sentence | `DB-TC-PDF-2019` | T1 | the year's sentence deletes the day |
| 2019-04-19 | closed | `19 April` — an exception date of the year's sentence | `DB-TC-PDF-2019` | T1 | the year's sentence deletes the day |
| 2019-04-22 | closed | `22 April` — an exception date of the year's sentence | `DB-TC-PDF-2019` | T1 | the year's sentence deletes the day |
| 2019-05-01 | closed | `1 May` — an exception date of the year's sentence | `DB-TC-PDF-2019` | T1 | the year's sentence deletes the day |
| 2019-06-10 | closed | `10 June` — an exception date of the year's sentence | `DB-TC-PDF-2019` | T1 | the year's sentence deletes the day and states it a settlement day: no trading, settlement only |
| 2019-10-03 | closed | `3 October` — an exception date of the year's sentence | `DB-TC-PDF-2019` | T1 | the year's sentence deletes the day and states it a settlement day: no trading, settlement only |
| 2019-12-24 | closed | `24 December` — an exception date of the year's sentence | `DB-TC-PDF-2019` | T1 | the year's sentence deletes the day and states it a settlement day: no trading, settlement only |
| 2019-12-25 | closed | `25 December` — an exception date of the year's sentence | `DB-TC-PDF-2019` | T1 | the year's sentence deletes the day |
| 2019-12-26 | closed | `26 December` — an exception date of the year's sentence | `DB-TC-PDF-2019` | T1 | the year's sentence deletes the day |
| 2019-12-31 | closed | `31 December` — an exception date of the year's sentence | `DB-TC-PDF-2019` | T1 | the year's sentence deletes the day and states it a settlement day: no trading, settlement only |
### 2020

The 2020 calendar sentence: at FWB® Frankfurter Wertpapierbörse “there will be trading Mondays to Fridays in 2020, with the exception of 1 January, 10 April, 13 April, 1 May, 1 June, 24 December, 25 December, and 31 December. 1 June, 24 December and 31 December are settlement days.”


| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2020-01-01 | closed | `1 January` — an exception date of the year's sentence | `DB-TC-PDF-2020` | T1 | the year's sentence deletes the day |
| 2020-04-10 | closed | `10 April` — an exception date of the year's sentence | `DB-TC-PDF-2020` | T1 | the year's sentence deletes the day |
| 2020-04-13 | closed | `13 April` — an exception date of the year's sentence | `DB-TC-PDF-2020` | T1 | the year's sentence deletes the day |
| 2020-05-01 | closed | `1 May` — an exception date of the year's sentence | `DB-TC-PDF-2020` | T1 | the year's sentence deletes the day |
| 2020-06-01 | closed | `1 June` — an exception date of the year's sentence | `DB-TC-PDF-2020` | T1 | the year's sentence deletes the day and states it a settlement day: no trading, settlement only |
| 2020-12-24 | closed | `24 December` — an exception date of the year's sentence | `DB-TC-PDF-2020` | T1 | the year's sentence deletes the day and states it a settlement day: no trading, settlement only |
| 2020-12-25 | closed | `25 December` — an exception date of the year's sentence | `DB-TC-PDF-2020` | T1 | the year's sentence deletes the day |
| 2020-12-31 | closed | `31 December` — an exception date of the year's sentence | `DB-TC-PDF-2020` | T1 | the year's sentence deletes the day and states it a settlement day: no trading, settlement only |
### 2021

The 2021 calendar sentence: at FWB® Frankfurter Wertpapierbörse “there will be trading Mondays to Fridays in 2021, with the exception of 1 January, 2 April, 5 April, 24 May, 24 December and 31 December. 24 May, 24 December and 31 December are settlement days.”


| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2021-01-01 | closed | `1 January` — an exception date of the year's sentence | `DB-TC-PDF-2021` | T1 | the year's sentence deletes the day |
| 2021-04-02 | closed | `2 April` — an exception date of the year's sentence | `DB-TC-PDF-2021` | T1 | the year's sentence deletes the day |
| 2021-04-05 | closed | `5 April` — an exception date of the year's sentence | `DB-TC-PDF-2021` | T1 | the year's sentence deletes the day |
| 2021-05-24 | closed | `24 May` — an exception date of the year's sentence | `DB-TC-PDF-2021` | T1 | the year's sentence deletes the day and states it a settlement day: no trading, settlement only |
| 2021-12-24 | closed | `24 December` — an exception date of the year's sentence | `DB-TC-PDF-2021` | T1 | the year's sentence deletes the day and states it a settlement day: no trading, settlement only |
| 2021-12-31 | closed | `31 December` — an exception date of the year's sentence | `DB-TC-PDF-2021` | T1 | the year's sentence deletes the day and states it a settlement day: no trading, settlement only |
### 2022

The 2022 calendar sentence: at FWB® Frankfurter Wertpapierbörse “there will be trading Mondays to Fridays in 2022, with the exception of 15 April, 18 April and 26 December.”


| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2022-04-15 | closed | `15 April` — an exception date of the year's sentence | `DB-TC-PDF-2022` | T1 | the year's sentence deletes the day |
| 2022-04-18 | closed | `18 April` — an exception date of the year's sentence | `DB-TC-PDF-2022` | T1 | the year's sentence deletes the day |
| 2022-12-26 | closed | `26 December` — an exception date of the year's sentence | `DB-TC-PDF-2022` | T1 | the year's sentence deletes the day |
### 2023

The 2023 calendar sentence: at FWB® Frankfurter Wertpapierbörse “there will be trading Mondays to Fridays in 2023, with the exception of 7 April, 10 April, 1 May, 25 December and 26 December.”


| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2023-04-07 | closed | `7 April` — an exception date of the year's sentence | `DB-TC-PDF-2023` | T1 | the year's sentence deletes the day |
| 2023-04-10 | closed | `10 April` — an exception date of the year's sentence | `DB-TC-PDF-2023` | T1 | the year's sentence deletes the day |
| 2023-05-01 | closed | `1 May` — an exception date of the year's sentence | `DB-TC-PDF-2023` | T1 | the year's sentence deletes the day |
| 2023-12-25 | closed | `25 December` — an exception date of the year's sentence | `DB-TC-PDF-2023` | T1 | the year's sentence deletes the day |
| 2023-12-26 | closed | `26 December` — an exception date of the year's sentence | `DB-TC-PDF-2023` | T1 | the year's sentence deletes the day |
### 2024

The 2024 calendar sentence: at FWB® Frankfurter Wertpapierbörse “there will be trading Mondays to Fridays in 2024, with the exception of 1 January, 29 March, 1 April, 1 May, 24 December, 25 December, 26 December and 31 December. 24 December and 31 December are settlement days.”


| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2024-01-01 | closed | `1 January` — an exception date of the year's sentence | `DB-TC-PDF-2024` | T1 | the year's sentence deletes the day |
| 2024-03-29 | closed | `29 March` — an exception date of the year's sentence | `DB-TC-PDF-2024` | T1 | the year's sentence deletes the day |
| 2024-04-01 | closed | `1 April` — an exception date of the year's sentence | `DB-TC-PDF-2024` | T1 | the year's sentence deletes the day |
| 2024-05-01 | closed | `1 May` — an exception date of the year's sentence | `DB-TC-PDF-2024` | T1 | the year's sentence deletes the day |
| 2024-12-24 | closed | `24 December` — an exception date of the year's sentence | `DB-TC-PDF-2024` | T1 | the year's sentence deletes the day and states it a settlement day: no trading, settlement only |
| 2024-12-25 | closed | `25 December` — an exception date of the year's sentence | `DB-TC-PDF-2024` | T1 | the year's sentence deletes the day |
| 2024-12-26 | closed | `26 December` — an exception date of the year's sentence | `DB-TC-PDF-2024` | T1 | the year's sentence deletes the day |
| 2024-12-31 | closed | `31 December` — an exception date of the year's sentence | `DB-TC-PDF-2024` | T1 | the year's sentence deletes the day and states it a settlement day: no trading, settlement only |

### 2025

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2025-01-01 | closed | `New Year's Day — Wednesday 01 Jan 2025` | `DB-TC-PDF-2025` | T1 | the PDF's sentence: trading in 2025 "with the exception of 1 January, 18 April, 21 April, 1 May, 24, 25, 26 and 31 December"; the page table corroborates |
| 2025-04-18 | closed | `Good Friday — Friday 18 Apr 2025` | `DB-TC-PDF-2025` | T1 | same sentence; the page table corroborates |
| 2025-04-21 | closed | `Easter Monday — Monday 21 Apr 2025` | `DB-TC-PDF-2025` | T1 | same sentence; the page table corroborates |
| 2025-05-01 | closed | `Labour Day — Thursday 01 May 2025` | `DB-TC-PDF-2025` | T1 | same sentence; the page table corroborates |
| 2025-12-24 | closed | `Christmas Eve** — Wednesday 24 Dec 2025` — `**` = "No trading but settlement is open" | `DB-TC-PDF-2025` | T1 | same sentence; the page's footnote states the day is not a trading day |
| 2025-12-25 | closed | `Christmas Day — Thursday 25 Dec 2025` | `DB-TC-PDF-2025` | T1 | same sentence; the page table corroborates |
| 2025-12-26 | closed | `Boxing Day — Friday 26 Dec 2025` | `DB-TC-PDF-2025` | T1 | same sentence; the page table corroborates |
| 2025-12-31 | closed | `New Year's Eve** — Wednesday 31 Dec 2025` — `**` = "No trading but settlement is open" | `DB-TC-PDF-2025` | T1 | same sentence; the page's footnote states the day is not a trading day |

### 2026

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2026-01-01 | closed | `New Year's Day — Thursday Jan 01, 2026`; the PDF: trading "with the exception of 1 January, 3 April, 6 April, 1 May, 24 December, 25 December and 31 December" | `DB-TC-PDF-2026` | T1 | the PDF's own closure sentence; the live page's 2026 column corroborates |
| 2026-04-03 | closed | `Good Friday — Friday Apr 03, 2026` | `DB-TC-PDF-2026` | T1 | the PDF's closure sentence |
| 2026-04-06 | closed | `Easter Monday — Monday Apr 06, 2026` | `DB-TC-PDF-2026` | T1 | the PDF's closure sentence |
| 2026-05-01 | closed | `Labor Day — Friday May 01, 2026` | `DB-TC-PDF-2026` | T1 | the PDF's closure sentence |
| 2026-05-14 | early close | `Ascension Day (14 May 2026)` trades, and `Trading of shares and Exchange traded products on Frankfurt and Xetra ends ... at 20:00 CET` | `DB-TC-PAGE` | T1 | the page names 2026-05-14 as a trading holiday and states the 20:00 shares close in the same note |
| 2026-05-25 | early close | `Whit Monday (25 May 2026)` trades; shares end at 20:00 CET | `DB-TC-PAGE` | T1 | same note, same rule |
| 2026-06-04 | early close | `Corpus Christi (4 June 2026)` trades; shares end at 20:00 CET | `DB-TC-PAGE` | T1 | same note, same rule |
| 2026-12-24 | closed | `Christmas Eve** — Thursday Dec 24, 2026` — `**` = "No trading but settlement is open" | `DB-TC-PDF-2026` | T1 | the PDF's closure sentence; the page's footnote states the day is not a trading day |
| 2026-12-25 | closed | `Christmas Day — Friday Dec 25, 2026` | `DB-TC-PDF-2026` | T1 | the PDF's closure sentence |
| 2026-12-31 | closed | `New Year's Eve** — Thursday Dec 31, 2026` — `**` = "No trading but settlement is open" | `DB-TC-PDF-2026` | T1 | the PDF's closure sentence; the page's footnote states the day is not a trading day |

### 2027

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2027-01-01 | closed | `New Year's Day — Friday Jan 01, 2027` | `DB-TC-PAGE` | T1 | the live page's 2027 column of the non-trading-days table |
| 2027-03-26 | closed | `Good Friday — Friday Mar 26, 2027` | `DB-TC-PAGE` | T1 | the live page's 2027 column |
| 2027-03-29 | closed | `Easter Monday — Monday Mar 29, 2027` | `DB-TC-PAGE` | T1 | the live page's 2027 column; Labour Day 2027 falls on Saturday and names no weekday closure |
| 2027-12-24 | closed | `Christmas Eve** — Friday Dec 24, 2027` — `**` = "No trading but settlement is open" | `DB-TC-PAGE` | T1 | the live page's 2027 column; Christmas Day (Saturday) and Boxing Day (Sunday) 2027 name no weekday closure |
| 2027-12-31 | closed | `New Year's Eve** — Friday Dec 31, 2027` — `**` = "No trading but settlement is open" | `DB-TC-PAGE` | T1 | the live page's 2027 column |

**Gaps.** The 2025-04-22 page capture words its 20:00 holiday-close note over `Börse Frankfurt` only, so the 2025 trading holidays carry no Xetra early close here; the 2027 trading holidays (Ascension, Whit Monday, Corpus Christi) are named by no retrieved artifact — the page's named list is scoped to `the year 2026` — so 2027-05-06, 2027-05-17 and 2027-05-27 ship as ordinary days and may in fact end at 20:00. Re-checked 2026-09-29 UTC with a fresh live read of the page (artifact under `holidays/raw/equities/xetra/forward-2027/`): the trading-holiday note is still scoped to `the year 2026`, the page still links only the `Trading calendar 2026` PDF, and no `Trading calendar 2027` edition is linked, so the three 2027 instants remain named by no artifact. Re-checked again 2026-10-02 UTC with a fresh live read of the page (artifact `db_trading_calendar_page.live-20261002T235000Z.html`, store `INDEX-recheck-2026-10-02.md` beside the earlier one): the note is still scoped to `the year 2026`, the only linked PDF is still `Trading calendar 2026`, and no 2027 edition is published. Re-checked again 2026-10-03 UTC with a fresh live read (artifact `db_trading_calendar_page.live-20261003T085958Z.html` under `holidays/raw/equities/xetra/recheck-2026-10/`, byte-identical to the 2026-10-02 read except one site-chrome script tag), with a CDX enumeration of every `trading-calendar` PDF on the operator's two hosts (2003-2026 editions only, no 2027) and a web search beside it: the state is unchanged, and no `Trading calendar 2027` edition exists at any of the three channels.

**The 2025 half of this gap closed on 2026-10-03 as the confirmation #200 names.** #200's closing condition for the missing 2025 Xetra early close is "an operator edition that extends the 20:00 note to Xetra for 2025 (or a confirmation that none existed)". Six operator editions of the 2025-vintage page have now been read and word the note over Börse Frankfurt only: the 2025-04-22 capture held since the 2025 wave, the Wayback replays of 2025-08-06, 2025-09-13 and 2025-10-29 (artifacts `wayback_xetra_page_2025*.html` under `recheck-2026-10/`), the German edition captured 2025-10-04 (`wayback_xetra_page_de_20251004035435.html`: `Der Handel an der Börse Frankfurt endet in Aktien und Exchange Traded Products ... um 20:00 Uhr MEZ`), and the operator group's own `eurexgroup.com` mirror of the same 2025-vintage page, live on 2026-10-03 (`eurexgroup_xetra_trading_calendar_page.live-20261003T090213Z.html`). Every one of the six also affirmatively names the four 2025 Xetra trading holidays — "on Xetra and Börse Frankfurt in the year 2025 regular stock exchange trading takes place on the following public holidays: Ascension Day (29 May 2025) Whit Monday (9 June 2025) Corpus Christi (19 June 2025) German Unity Day (3 October 2025)" — and the operator's circulars channel carries no 2025 holiday-hours circular (no listing-feed item is a 2025 holiday-hours circular (older items exist on the feeds; a CDX sweep of the circulars tree, not archived, finds none either)). So no 2025 artifact extends the 20:00 end to Xetra: the shipped ordinary-day rows state what every sourced state states, and Xetra's ordinary 17:35 close precedes 20:00 besides, so a 20:00 end could not have shortened the executable day. The 2025 half ships as it is.

**Closing condition (the 2027 half):** the operator's `Trading calendar 2027` edition or the page's 2027 edition, which names Xetra's own hours on 2027-05-06, 2027-05-17 and 2027-05-27; the live page is re-checked monthly per LAW-WATCH and the 2027 edition is the expected closer — the operator had already linked the `Trading calendar 2026` edition by its 2025-10-29 page capture (it is one of that capture's three downloads), so the 2027 edition's publication window is the coming weeks. Tracked as #200. Nothing in 2010-2024 is unresolved: every weekday exception date each year's sentence names ships a `Closed` row (104 rows across the fifteen years), weekend-falling exception dates key no weekday row, and no historical sentence states an intraday instant. Nothing else in 2025-2027 is unresolved: every closure the operator prints ships, and every other trade date in the window is audited normal.

### Documents

The 2010-2024 artifacts were retrieved on 2026-09-29 UTC and saved under `holidays/raw/equities/xetra/2010-2024/` in the research store; the 2025-2027 artifacts below them were retrieved on 2026-09-28 UTC and saved under `holidays/raw/equities/xetra/2025-2027/`. The store's `INDEX.md` carries the same digests. The `DB_HK_<year>` editions of 2010-2014 are one-page `Trading Calendar <year>` grids whose closure sentence sits under the year grids; the 2014 keyed edition is the German printing, the English capture of the same document not replaying (capture `20141031072236`), and the same year's English dispatch page edition (`DB-TC-PAGE-2014`) corroborates it.

| Document | Window | Replay or service URL | Capture or retrieval, UTC | Tier | sha256 |
|---|---|---|---|---|---|
| `DB-TC-PDF-2010` | 2010-01-01 .. 2010-12-31 | <https://web.archive.org/web/20100108053919id_/http://deutsche-boerse.com/dbag/dispatch/en/binary/gdb_content_pool/imported_files/public_files/10_downloads/11_about_us/DB_HK_2010.pdf> (capture `20100108053919`) | Wayback `id_` replay of capture `20100108053919`, retrieved 2026-09-29 UTC | T1 | `f7616bc10dac01267b5344e8bac889fa3cbda71dfbc2e6e061a30decd728e2cc` |
| `DB-TC-PDF-2011` | 2011-01-01 .. 2011-12-31 | <https://web.archive.org/web/20101031002657id_/http://deutsche-boerse.com/dbag/dispatch/en/binary/gdb_content_pool/imported_files/public_files/10_downloads/11_about_us/DB_HK_2011.pdf> (capture `20101031002657`) | Wayback `id_` replay of capture `20101031002657`, retrieved 2026-09-29 UTC | T1 | `5f7810d0531bff4b561349e19e6b21cbb89563e7eaeed77d924848659b0968cc` |
| `DB-TC-PDF-2012` | 2012-01-01 .. 2012-12-31 | <https://web.archive.org/web/20120710042704id_/http://xetra.com/xetra/dispatch/en/binary/gdb_content_pool/imported_files/public_files/10_downloads/11_about_us/DB_HK_2012.pdf> (capture `20120710042704`) | Wayback `id_` replay of capture `20120710042704`, retrieved 2026-09-29 UTC | T1 | `45cb1080b03b51d87dc81185c19f0f65a50b9bc835018fb850cc973633ac38d8` |
| `DB-TC-PAGE-2012` | 2012-01-01 .. 2012-12-31 | <https://web.archive.org/web/20120322133844id_/http://xetra.com/xetra/dispatch/en/kir/navigation/xetra/300_trading_clearing/200_trading_information/100_trading_calender> (capture `20120322133844`) | Wayback `id_` replay of capture `20120322133844`, retrieved 2026-09-29 UTC | T1 | `2db9ae4a30964157936587b4ea7771f3acce7e46d1da34fd154a5c1beed8e3a1` |
| `DB-TC-PDF-2013` | 2013-01-01 .. 2013-12-31 | <https://web.archive.org/web/20130606061445id_/http://xetra.com/xetra/dispatch/en/binary/gdb_content_pool/imported_files/public_files/10_downloads/11_about_us/DB_HK_2013.pdf> (capture `20130606061445`) | Wayback `id_` replay of capture `20130606061445`, retrieved 2026-09-29 UTC | T1 | `f203cb348d4ac9b162a75566f101e2cf9f3e58697f09716c0985778a78121010` |
| `DB-TC-PAGE-2013` | 2013-01-01 .. 2013-12-31 | <https://web.archive.org/web/20130111223039id_/http://xetra.com/xetra/dispatch/en/kir/navigation/xetra/300_trading_clearing/200_trading_information/100_trading_calender> (capture `20130111223039`) | Wayback `id_` replay of capture `20130111223039`, retrieved 2026-09-29 UTC | T1 | `c497915f37a7862af049199aa2ea3f236780b9f13bc202ce590d22764a4e129a` |
| `DB-TC-PDF-2014` | 2014-01-01 .. 2014-12-31 | <https://web.archive.org/web/20141031072230id_/http://xetra.com/xetra/dispatch/de/binary/gdb_content_pool/imported_files/public_files/10_downloads/11_about_us/DB_HK_2014.pdf> (capture `20141031072230`, German edition) | Wayback `id_` replay of capture `20141031072230`, retrieved 2026-09-29 UTC | T1 | `78ca94d1d7ea2e6ea5c670c36bfdd1cdb1f7181e7082b5354c9b0cdb1b293904` |
| `DB-TC-PAGE-2014` | 2014-01-01 .. 2014-12-31 | <https://web.archive.org/web/20140209211122id_/http://xetra.com/xetra/dispatch/en/kir/navigation/xetra/300_trading_clearing/200_trading_information/100_trading_calender> (capture `20140209211122`) | Wayback `id_` replay of capture `20140209211122`, retrieved 2026-09-29 UTC | T1 | `16afa058aba967270d71a61580a702a89902794a6962ff021d6b18fb4bf9af58` |
| `DB-TC-PDF-2015` | 2015-01-01 .. 2015-12-31 | <https://web.archive.org/web/20210307025839id_/https://www.xetra.com/resource/blob/252786/26a3eefdbb8b939be91ec1d3d1986347/data/trading-calendar-2015.pdf> (capture `20210307025839`) | Wayback `id_` replay of capture `20210307025839`, retrieved 2026-09-29 UTC | T1 | `de88e22fbe02148cd3f4f3fd1b60e91aa2a79ba06263660dea5f5754d7e9bfe9` |
| `DB-TC-PDF-2016` | 2016-01-01 .. 2016-12-31 | <https://web.archive.org/web/20210307022753id_/https://www.xetra.com/resource/blob/259998/85c710e77ed5d595695f5b1d6418eac7/data/trading-calendar-2016.pdf> (capture `20210307022753`) | Wayback `id_` replay of capture `20210307022753`, retrieved 2026-09-29 UTC | T1 | `685f132fa36e9c2a64cad212d2e8a79e8587f0fa90c5f84c891a20537a21c4de` |
| `DB-TC-PDF-2017` | 2017-01-01 .. 2017-12-31 | <https://web.archive.org/web/20210307024049id_/https://www.xetra.com/resource/blob/336572/247517da046f56e5b4dae39723f29b00/data/trading-calendar-2017.pdf> (capture `20210307024049`) | Wayback `id_` replay of capture `20210307024049`, retrieved 2026-09-29 UTC | T1 | `4dbfb46a4507ef84e87d2e1e6002a240951f6d4abc7cdb208c4dcae02d26c4a5` |
| `DB-TC-PDF-2018` | 2018-01-01 .. 2018-12-31 | <https://web.archive.org/web/20181009055128id_/http://www.xetra.com/blob/3232562/366c1c9be4ce7cdb923bbf246db1d1bf/data/xetra-trading-calendar-2018.pdf> (capture `20181009055128`) | Wayback `id_` replay of capture `20181009055128`, retrieved 2026-09-29 UTC | T1 | `6109fc3a393a3f7aa848ee681cd6182aed267c1795585001d25dd7261f52d9b2` |
| `DB-TC-PDF-2019` | 2019-01-01 .. 2019-12-31 | <https://web.archive.org/web/20191015105238id_/https://www.xetra.com/resource/blob/1406548/6de0eba301a5433abb110fa3c96a5778/data/xetra-trading-calendar-2019.pdf> (capture `20191015105238`) | Wayback `id_` replay of capture `20191015105238`, retrieved 2026-09-29 UTC | T1 | `14297839055d795ae9aa4c8a17f0428554710ffe6af9eb0ed10aa7534b553fca` |
| `DB-TC-PDF-2020` | 2020-01-01 .. 2020-12-31 | <https://web.archive.org/web/20200520112052id_/https://www.xetra.com/resource/blob/1665534/112856375187d3e93671aa3d731d09d3/data/xetra-trading-calendar-2020.pdf> (capture `20200520112052`) | Wayback `id_` replay of capture `20200520112052`, retrieved 2026-09-29 UTC | T1 | `6d7fc1b53875541344bcf589dc667739cb521273148588040210cc487d782d06` |
| `DB-TC-PDF-2021` | 2021-01-01 .. 2021-12-31 | <https://web.archive.org/web/20201127233243id_/https://www.xetra.com/resource/blob/2344982/ddbcd31a616628a7ffa09dc709467ec4/data/xetra-trading-calendar-2021.pdf> (capture `20201127233243`) | Wayback `id_` replay of capture `20201127233243`, retrieved 2026-09-29 UTC | T1 | `c74c7e9bc4ed1da058b42239c23c1fccba3a8bf473ede97761cb2a24fabcc59b` |
| `DB-TC-PDF-2022` | 2022-01-01 .. 2022-12-31 | <https://web.archive.org/web/20220120154425id_/https://www.xetra.com/resource/blob/2833162/2b3b8cd1230955e9b416c5ce6e3b5bf2/data/xetra-trading-calendar-2022.pdf> (capture `20220120154425`) | Wayback `id_` replay of capture `20220120154425`, retrieved 2026-09-29 UTC | T1 | `99dd0f35e98f62649a5c40e30ee996c7baa2580c22227264cbd04b5b2d3b69ef` |
| `DB-TC-PDF-2023` | 2023-01-01 .. 2023-12-31 | <https://web.archive.org/web/20230307164045id_/https://www.xetra.com/resource/blob/3317408/4c8fbfbfeea62fd44600f6fe3f14f84e/data/xetra-trading-calendar-2023.pdf> (capture `20230307164045`) | Wayback `id_` replay of capture `20230307164045`, retrieved 2026-09-29 UTC | T1 | `33a083ba124695c5793ddecafef34e9e717683b5ed761e832ee32f1e87ad509b` |
| `DB-TC-PDF-2024` | 2024-01-01 .. 2024-12-31 | <https://web.archive.org/web/20240613172628id_/https://www.xetra.com/resource/blob/3559262/98ebe1fde231df56c9f116bc766533b2/data/xetra-trading-calendar-2024.pdf> (capture `20240613172628`) | Wayback `id_` replay of capture `20240613172628`, retrieved 2026-09-29 UTC | T1 | `05d70c3a23ecbb701aeeaa37aa84817eb20895c7952467ff3469720c239386d9` |
| `DB-TC-PAGE-2025` | 2025-01-01 .. 2027-12-31 | <https://web.archive.org/web/20250422181518id_/https://www.xetra.com/xetra-en/trading/trading-calendar-and-trading-hours> (capture `20250422181518`) | Wayback `id_` replay of capture `20250422181518`, retrieved 2026-09-28 UTC | T1 | `44b6b2783375a17ac736ebc1c0247e0bae011f1a64c9ae125c602fea9ab41918` |
| `DB-TC-PDF-2025` | 2025-01-01 .. 2025-12-31 | <https://www.xetra.com/resource/blob/4064968/4079a2d5a9fec324905942b807b398ed/data/xetra-trading-calendar-2025.pdf> (2025 archive replay of the PDF linked from `DB-TC-PAGE-2025`) | retrieved 2026-09-28 UTC | T1 | `84c71bed702dd753f4272939f9c65d87ebd9afdfc89ff7917d545b66c3f8a8e5` |
| `DB-TC-PDF-2026` | 2026-01-01 .. 2026-12-31 | <https://www.cashmarket.deutsche-boerse.com/resource/blob/4481276/1b643791fcb4d60bdd7f25efad3f4626/data/deutsche-boerse-trading-calendar-2026.pdf> ("Trading calendar 2026") | retrieved 2026-09-28 01:28 UTC | T1 | `1edfc7b737ae1f5fe93179bfa9af59223b3c9d11cc8deddd127b386fb87e44b6` |
| `DB-TC-PAGE` | 2025-01-01 .. 2027-12-31 | <https://www.cashmarket.deutsche-boerse.com/cash-en/trading/trading-calendar-and-trading-hours> | retrieved 2026-09-28 01:28 UTC | T1 | `d70a8f5d54cb529103bf674ea8382702a3510800dac8b48965a43aa08ae4bca4` |

## Sources

Row review: 2026-08-22 (UTC) is the date the ledger row was last reviewed as a
whole. Per-source retrieval dates were not recorded before the 2026-09-12
migration; where a later targeted review, capture or document date is recorded
beside a source below, that date governs for that source, and dates are added
as each source is re-verified.

- <https://cashmarket.deutsche-boerse.com/resource/blob/197910/0890768f3f753299e4c268b80fe7944d/data/207_08e.pdf> — Deutsche Börse circular 207/08, the January-2010-era market model.
- <https://www.cashmarket.deutsche-boerse.com/resource/blob/1431340/a23cc3ff15d46a3b649bd23f1618b928/data/091_18e.pdf> — Deutsche Börse circular 091/18. With 207/08 it brackets the January-2010 baseline and confirms the DAX grid remained: pre-trading from 07:30, opening auction 08:50–09:00, intraday auction 13:00–13:02, continuous trading to 17:30, closing auction to 17:35, order-entry-only post-trading through 20:30.
- <https://www.cashmarket.deutsche-boerse.com/resource/blob/31802/6ab37d564c2934a20766824e4284d608/data/2026_07_07_fwb_boersenordnung_en.pdf> — FWB Exchange Rules: § 67 makes pre-trading and post-trading Trading Periods distinct from the periods in which prices are determined, and § 67(2) states that "[d]uring the pre-trading period, the order book shall remain closed"; § 123 confines trading to 08:30–17:30 plus the closing auction and the Trade-at-Close period; § 123(2b) permits Extended Xetra Retail Service trading from 08:00 to 09:00 and through 22:00.
- <https://www.cashmarket.deutsche-boerse.com/cash-en/Stay-Informed/circulars-newsletters/deutsche-boerse-circulars/Introduction-of-T7-Release-9.0-1978838> — Deutsche Börse circular, T7 Release 9.0 entered production 2020-11-23.
- <https://www.cashmarket.deutsche-boerse.com/cash-en/Stay-Informed/newsroom/press-releases/Xetra-Trade-at-Close-enables-trading-at-the-official-closing-price-2346762> — Deutsche Börse factsheet and release: Trade-at-Close itself launched 2020-11-24, one day after the release went to production.
- <https://www.cashmarket.deutsche-boerse.com/cash-en/Stay-Informed/circulars-newsletters/deutsche-boerse-circulars/Introduction-of-the-Extended-Xetra-Retail-Service-early-and-late-trading-Planned-changes-to-the-trading-process-valid-from-1-December-2025-4793480> — Deutsche Börse circular, Extended Xetra Retail Service effective 2025-12-01.
- <https://www.cashmarket.deutsche-boerse.com/resource/blob/250890/24d50260d22cd63e0f600ae2543ca529/data/trading-parameters-xetra.pdf> — Xetra trading-parameter sheet: marks pre-trading and post-trading "(Book)", quotes no price for them, and runs the Retail Pre-Call/Retail-Call from 08:00.
- <https://www.cashmarket.deutsche-boerse.com/cash-en/trading/trading-calendar-and-trading-hours> — Xetra calendar and hours, the source set's current entry point.
- <https://www.cashmarket.deutsche-boerse.com/cash-en/trading/Xetra/continuous-trading-with-auctions> — Xetra continuous trading with auctions.
- <https://www.cashmarket.deutsche-boerse.com/cash-en/Stay-Informed/rules-and-regulations-for-the-fwb> — FWB rules and regulations, the monitoring entry point.
- <https://www.fese.eu/app/uploads/2024/07/trading-hours-2025-1.pdf> — FESE 2025 trading-hours table, `EU-FESE-SECONDARY`: corroboration only.

## Gaps and residual risks

- **Scope.** The profile represents the liquid DAX constituent-share segment, not every Xetra instrument. Other segments have their own auction schedules and are out of scope.
- **Interpretive step, order-entry classification.** Pre-trading and post-trading are `order_entry` on § 67/§ 123 and on the parameter sheet's "(Book)" marking. Every auction call is `extended` whole because its price determination prints at the auction price.
- **Interpretive step, retail phases.** The 08:00–08:55 early retail and 17:40–22:00 late retail windows are participant-restricted, so they are `extended` rather than `regular`; only the unrestricted continuous phases stay `regular`. Each auction is modelled through its 30-second random end, so regular trading begins at the latest possible edge.
- **Served identity, 2026-09-28 UTC.** The consumer's market clock routes its `XETRA` and `XETRA_CENTRE` sets to this venue, so the row is **served** and reviewed monthly per LAW-WATCH; follow-ups are tracked as issues (#200) (LAW-SERVICE-TIERS, LAW-FOLLOW-UPS-ARE-ISSUES).

## Module narrative (moved from src/calendar/schedules/equities/europe/xetra.rs on 2026-10-02 UTC)

Each auction can end in a 30-second random period, so regular trading
begins only at the latest possible edge.
https://cashmarket.deutsche-boerse.com/resource/blob/197910/0890768f3f753299e4c268b80fe7944d/data/207_08e.pdf
https://www.cashmarket.deutsche-boerse.com/resource/blob/1431340/a23cc3ff15d46a3b649bd23f1618b928/data/091_18e.pdf

---

and § 67(2) states that "[d]uring the
pre-trading period, the order book shall remain closed" (the Specialist
carve-out applies to the Continuous Auction, not to Xetra's order book).
§ 123 confines trading to 08:30-17:30 plus the closing auction and the
Trade-at-Close period, so nothing can match before or after those phases.
The operator's trading-parameter sheet likewise marks both phases "(Book)",
i.e. order-book maintenance only, and quotes no price for them.
https://www.cashmarket.deutsche-boerse.com/resource/blob/31802/6ab37d564c2934a20766824e4284d608/data/2026_07_07_fwb_boersenordnung_en.pdf

---

The DAX envelope now
begins at 07:00, Trade-at-Close ends at 17:40, participant-restricted late
retail trading continues through 22:00, and post-trading ends at 22:05. The
non-continuous and participant-restricted retail phases are `extended`, the
order-entry-only pre- and post-trading phases are `order_entry`, and only the
unrestricted continuous phases stay `regular`.
https://www.cashmarket.deutsche-boerse.com/cash-en/Stay-Informed/circulars-newsletters/deutsche-boerse-circulars/Introduction-of-the-Extended-Xetra-Retail-Service-early-and-late-trading-Planned-changes-to-the-trading-process-valid-from-1-December-2025-4793480
https://www.cashmarket.deutsche-boerse.com/resource/blob/250890/24d50260d22cd63e0f600ae2543ca529/data/trading-parameters-xetra.pdf
