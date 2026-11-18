<!-- SPDX-License-Identifier: MIT-0 -->

# `eurex` — evidence

- **Kind:** `MarketHoursKey`
- **Owner module:** [`europe.rs`](../../src/calendar/schedules/futures/international/europe.rs)
- **Source sets:** [`EU-EUREX`](../schedules/sources.md#eu-eurex)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

FESX/FDAX/FDXM benchmark-index futures; the dated key API reuses the sourced January-2010 baseline, 2018 extension, and recurring CET/CEST phase selection.

## Revision rows

None. `europe.rs` carries no `revisions!` block for this identity; its 2018-12-10 Asian-hours cutover and the seasonal CET/CEST choice are encoded as comparisons inside `eurex_profile_at` instead.

The key selects through the same `eurex_profile_at` function as the `eurex`
exchange row: a date comparison against the 2018-12-10 Asian-hours cutover,
then a UTC-offset comparison that picks the CET or CEST phase table. Eurex
circular 088/2018 states the cutover day unconditionally at T1, so the date is
sourced even though it is not carried as a tuple.

## Dated selectors

Day-level boundaries this identity's `profile_at` selects on directly, outside
any `revisions!` block. They are invisible to the module-declaration fences, so
they are recorded here in revision-row grammar and checked against
`HISTORICAL_CUTOVERS` / `HISTORICAL_INSTANT_CUTOVERS` in
`tests/contract/session_invariants/historical_expectations.rs`.

- 2018-12-10 — T1 — Eurex circular 088/2018 (`EUREX_ASIAN_HOURS`) — the Asian-hours open. The key selects through the same `eurex_profile_at` function as the `eurex` exchange row, so the boundary is shared; the seasonal CET/CEST table is a UTC-offset comparison beside it and asserts no day.

## Holidays

**Coverage:** 2010-01-01..2030-12-31 (inclusive trade dates). Tier: T1 throughout.

The key shares the `eurex` exchange row's `TABLE`: the all-derivatives closures plus the dated German-scope closures the 2014, 2016, 2017 and 2018 editions state for the German equity and equity-index products, which are FDAX and FDXM inside the family's documented scope. The `eurex_fixed_income` key ships its own all-derivatives-only `FIXED_INCOME` table, because the operator's German-scope notes never name fixed income (see [eurex_fixed_income](eurex_fixed_income.md)).

**Documents.**

- `EUREX-HOLREG-2026` — Eurex “Holiday regulations”, § 2026, day by day. <https://www.eurex.com/ex-en/trade/trading-calendar/holiday-regulations> (raw bytes retrieved 2026-09-12 04:19 UTC, sha256 `7b28acd2d2fb126c01461ef5a4ae11fe93e8b6f318001c6304bbce0fc78f3821`) — **T1**. Corroborated by the Trading Calendar 2026 PDF, p.2 “Overview of holidays by countries” <https://www.eurex.com/resource/blob/4873184/0ca7669a8cb9a2f917d99a801fb3f2de/data/tradingcalendar_2026_en.pdf> (retrieved 2026-09-12 04:18 UTC, sha256 `b0796b42819b38c0757d727d9b789360ba84cd0d45cea215544f86342158ac65`, PDF CreationDate 2026-07-02).
- `EUREX-HOLREG-2027-2030` — the same page's `Non-trading days at Eurex 2026 - 2030` table, 2027-2030 columns. <https://www.eurex.com/ex-en/trade/trading-calendar/holiday-regulations> (raw bytes retrieved 2026-11-18 08:47:32 UTC, sha256 `47e905aed14cabcb657e363bade142fb8d2861523c052ecfe3b4e3b6a873f8ae`) — **T1**.

The 2027-2030 rows key the Holiday regulations page's own multi-year table — the same unconditional operator statement whose § 2025 and § 2026 day-by-day sections key 2025 and 2026. The resolution of the earlier indicative-page conflict, the 2026 cross-check, the stability comparison and the country-scope residual are recorded in [eurex](eurex.md) and inherited here; the key ships exactly the rows the exchange row's table ships for these years. All bytes, with each artifact's URL, UTC retrieval time and sha256, are in the research store under `holidays/raw/cfe-eurex-ice-cde-smfe-2026-2027/INDEX.md`, `holidays/raw/cfe-eurex-ice-cde-smfe-2026-2027-fix/INDEX.md` and `holidays/raw/eurex-2027-2030/INDEX.md`; the normalised result is `holidays/cfe-eurex-ice-cde-smfe-2026-2027.json`, verified `matches: true` with zero discrepancies in its round-2 adversarial verdict.

### 2010

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2010-01-01 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: January 1, April 2, April 5` | `EUREX-CAL-2010` | T1 | Eurex event date 2010-01-01 (New Year's Day); one Berlin civil day per trade date, so the conversion is the identity |
| 2010-04-02 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: January 1, April 2, April 5` | `EUREX-CAL-2010` | T1 | Eurex event date 2010-04-02 (Good Friday); one Berlin civil day per trade date, so the conversion is the identity |
| 2010-04-05 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: January 1, April 2, April 5` | `EUREX-CAL-2010` | T1 | Eurex event date 2010-04-05 (Easter Monday); one Berlin civil day per trade date, so the conversion is the identity |
| 2010-12-24 | closed | `Eurex is closed for trading in all derivatives: December 24, December 31` — a full trading closure; clearing stays open | `EUREX-CAL-2010` | T1 | Eurex event date 2010-12-24 (Christmas Eve); one Berlin civil day per trade date, so the conversion is the identity |
| 2010-12-31 | closed | `Eurex is closed for trading in all derivatives: December 24, December 31` — a full trading closure; clearing stays open | `EUREX-CAL-2010` | T1 | Eurex event date 2010-12-31 (New Year's Eve); one Berlin civil day per trade date, so the conversion is the identity |

### 2011

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2011-04-22 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: April 22, April 25, December 26` | `EUREX-CAL-2011` | T1 | Eurex event date 2011-04-22 (Good Friday); one Berlin civil day per trade date, so the conversion is the identity |
| 2011-04-25 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: April 22, April 25, December 26` | `EUREX-CAL-2011` | T1 | Eurex event date 2011-04-25 (Easter Monday); one Berlin civil day per trade date, so the conversion is the identity |
| 2011-12-26 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: April 22, April 25, December 26` | `EUREX-CAL-2011` | T1 | Eurex event date 2011-12-26 (Boxing Day); one Berlin civil day per trade date, so the conversion is the identity |

### 2012

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2012-04-06 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: April 6, April 9, May 1, December 25, December 26` | `EUREX-CAL-2012` | T1 | Eurex event date 2012-04-06 (Good Friday); one Berlin civil day per trade date, so the conversion is the identity |
| 2012-04-09 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: April 6, April 9, May 1, December 25, December 26` | `EUREX-CAL-2012` | T1 | Eurex event date 2012-04-09 (Easter Monday); one Berlin civil day per trade date, so the conversion is the identity |
| 2012-05-01 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: April 6, April 9, May 1, December 25, December 26` | `EUREX-CAL-2012` | T1 | Eurex event date 2012-05-01 (Labour Day); one Berlin civil day per trade date, so the conversion is the identity |
| 2012-12-24 | closed | `Eurex is closed for trading in all derivatives: December 24, December 31` — a full trading closure; clearing stays open | `EUREX-CAL-2012` | T1 | Eurex event date 2012-12-24 (Christmas Eve); one Berlin civil day per trade date, so the conversion is the identity |
| 2012-12-25 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: April 6, April 9, May 1, December 25, December 26` | `EUREX-CAL-2012` | T1 | Eurex event date 2012-12-25 (Christmas Day); one Berlin civil day per trade date, so the conversion is the identity |
| 2012-12-26 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: April 6, April 9, May 1, December 25, December 26` | `EUREX-CAL-2012` | T1 | Eurex event date 2012-12-26 (Boxing Day); one Berlin civil day per trade date, so the conversion is the identity |
| 2012-12-31 | closed | `Eurex is closed for trading in all derivatives: December 24, December 31` — a full trading closure; clearing stays open | `EUREX-CAL-2012` | T1 | Eurex event date 2012-12-31 (New Year's Eve); one Berlin civil day per trade date, so the conversion is the identity |

### 2013

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2013-01-01 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: January 1, March 29, April 1, May 1, December 25, December 26` | `EUREX-CAL-2013` | T1 | Eurex event date 2013-01-01 (New Year's Day); one Berlin civil day per trade date, so the conversion is the identity |
| 2013-03-29 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: January 1, March 29, April 1, May 1, December 25, December 26` | `EUREX-CAL-2013` | T1 | Eurex event date 2013-03-29 (Good Friday); one Berlin civil day per trade date, so the conversion is the identity |
| 2013-04-01 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: January 1, March 29, April 1, May 1, December 25, December 26` | `EUREX-CAL-2013` | T1 | Eurex event date 2013-04-01 (Easter Monday); one Berlin civil day per trade date, so the conversion is the identity |
| 2013-05-01 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: January 1, March 29, April 1, May 1, December 25, December 26` | `EUREX-CAL-2013` | T1 | Eurex event date 2013-05-01 (Labour Day); one Berlin civil day per trade date, so the conversion is the identity |
| 2013-12-24 | closed | `Eurex is closed for trading in all derivatives: December 24, December 31` — a full trading closure; clearing stays open | `EUREX-CAL-2013` | T1 | Eurex event date 2013-12-24 (Christmas Eve); one Berlin civil day per trade date, so the conversion is the identity |
| 2013-12-25 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: January 1, March 29, April 1, May 1, December 25, December 26` | `EUREX-CAL-2013` | T1 | Eurex event date 2013-12-25 (Christmas Day); one Berlin civil day per trade date, so the conversion is the identity |
| 2013-12-26 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: January 1, March 29, April 1, May 1, December 25, December 26` | `EUREX-CAL-2013` | T1 | Eurex event date 2013-12-26 (Boxing Day); one Berlin civil day per trade date, so the conversion is the identity |
| 2013-12-31 | closed | `Eurex is closed for trading in all derivatives: December 24, December 31` — a full trading closure; clearing stays open | `EUREX-CAL-2013` | T1 | Eurex event date 2013-12-31 (New Year's Eve); one Berlin civil day per trade date, so the conversion is the identity |

### 2014

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2014-01-01 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: 1 January, 18 April, 21 April, 1 May, 25 December, 26 December` | `EUREX-CAL-2014` | T1 | Eurex event date 2014-01-01 (New Year's Day); one Berlin civil day per trade date, so the conversion is the identity |
| 2014-04-18 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: 1 January, 18 April, 21 April, 1 May, 25 December, 26 December` | `EUREX-CAL-2014` | T1 | Eurex event date 2014-04-18 (Good Friday); one Berlin civil day per trade date, so the conversion is the identity |
| 2014-04-21 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: 1 January, 18 April, 21 April, 1 May, 25 December, 26 December` | `EUREX-CAL-2014` | T1 | Eurex event date 2014-04-21 (Easter Monday); one Berlin civil day per trade date, so the conversion is the identity |
| 2014-05-01 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: 1 January, 18 April, 21 April, 1 May, 25 December, 26 December` | `EUREX-CAL-2014` | T1 | Eurex event date 2014-05-01 (Labour Day); one Berlin civil day per trade date, so the conversion is the identity |
| 2014-10-03 | closed | `Eurex is closed for trading and exercise in German equity and equity index derivatives as well as ETF and ETC derivatives, which are based on Xetra® listings: 3 October` — the German scope, so FDAX and FDXM close | `EUREX-CAL-2014` | T1 | Eurex event date 2014-10-03 (German Unity Day); one Berlin civil day per trade date, so the conversion is the identity |
| 2014-12-24 | closed | `Eurex is closed for trading in all derivatives: 24 December, 31 December` — a full trading closure; clearing stays open | `EUREX-CAL-2014` | T1 | Eurex event date 2014-12-24 (Christmas Eve); one Berlin civil day per trade date, so the conversion is the identity |
| 2014-12-25 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: 1 January, 18 April, 21 April, 1 May, 25 December, 26 December` | `EUREX-CAL-2014` | T1 | Eurex event date 2014-12-25 (Christmas Day); one Berlin civil day per trade date, so the conversion is the identity |
| 2014-12-26 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: 1 January, 18 April, 21 April, 1 May, 25 December, 26 December` | `EUREX-CAL-2014` | T1 | Eurex event date 2014-12-26 (Boxing Day); one Berlin civil day per trade date, so the conversion is the identity |
| 2014-12-31 | closed | `Eurex is closed for trading in all derivatives: 24 December, 31 December` — a full trading closure; clearing stays open | `EUREX-CAL-2014` | T1 | Eurex event date 2014-12-31 (New Year's Eve); one Berlin civil day per trade date, so the conversion is the identity |

### 2015

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2015-01-01 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: 1 January, 3 April, 6 April, 1 May, 25 December, 26 December` | `EUREX-CAL-2015` | T1 | Eurex event date 2015-01-01 (New Year's Day); one Berlin civil day per trade date, so the conversion is the identity |
| 2015-04-03 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: 1 January, 3 April, 6 April, 1 May, 25 December, 26 December` | `EUREX-CAL-2015` | T1 | Eurex event date 2015-04-03 (Good Friday); one Berlin civil day per trade date, so the conversion is the identity |
| 2015-04-06 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: 1 January, 3 April, 6 April, 1 May, 25 December, 26 December` | `EUREX-CAL-2015` | T1 | Eurex event date 2015-04-06 (Easter Monday); one Berlin civil day per trade date, so the conversion is the identity |
| 2015-05-01 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: 1 January, 3 April, 6 April, 1 May, 25 December, 26 December` | `EUREX-CAL-2015` | T1 | Eurex event date 2015-05-01 (Labour Day); one Berlin civil day per trade date, so the conversion is the identity |
| 2015-05-25 | closed | `Eurex is closed for trading in all derivatives: 25 May, 24 December, 31 December` — a full trading closure; clearing stays open | `EUREX-CAL-2015` | T1 | Eurex event date 2015-05-25 (Whit Monday); one Berlin civil day per trade date, so the conversion is the identity |
| 2015-12-24 | closed | `Eurex is closed for trading in all derivatives: 25 May, 24 December, 31 December` — a full trading closure; clearing stays open | `EUREX-CAL-2015` | T1 | Eurex event date 2015-12-24 (Christmas Eve); one Berlin civil day per trade date, so the conversion is the identity |
| 2015-12-25 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: 1 January, 3 April, 6 April, 1 May, 25 December, 26 December` | `EUREX-CAL-2015` | T1 | Eurex event date 2015-12-25 (Christmas Day); one Berlin civil day per trade date, so the conversion is the identity |
| 2015-12-26 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: 1 January, 3 April, 6 April, 1 May, 25 December, 26 December` | `EUREX-CAL-2015` | T1 | Eurex event date 2015-12-26 (Boxing Day); one Berlin civil day per trade date, so the conversion is the identity |
| 2015-12-31 | closed | `Eurex is closed for trading in all derivatives: 25 May, 24 December, 31 December` — a full trading closure; clearing stays open | `EUREX-CAL-2015` | T1 | Eurex event date 2015-12-31 (New Year's Eve); one Berlin civil day per trade date, so the conversion is the identity |

### 2016

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2016-01-01 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: 1 January, 25 March, 28 March, 26 December` | `EUREX-CAL-2016` | T1 | Eurex event date 2016-01-01 (New Year's Day); one Berlin civil day per trade date, so the conversion is the identity |
| 2016-03-25 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: 1 January, 25 March, 28 March, 26 December` | `EUREX-CAL-2016` | T1 | Eurex event date 2016-03-25 (Good Friday); one Berlin civil day per trade date, so the conversion is the identity |
| 2016-03-28 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: 1 January, 25 March, 28 March, 26 December` | `EUREX-CAL-2016` | T1 | Eurex event date 2016-03-28 (Easter Monday); one Berlin civil day per trade date, so the conversion is the identity |
| 2016-05-16 | closed | `Eurex is closed for trading and exercise in German equity and equity index derivatives as well as ETF and ETC derivatives, which are based on Xetra® listings: 16 May, 3 October` — the German scope, so FDAX and FDXM close | `EUREX-CAL-2016` | T1 | Eurex event date 2016-05-16 (Whit Monday); one Berlin civil day per trade date, so the conversion is the identity |
| 2016-10-03 | closed | `Eurex is closed for trading and exercise in German equity and equity index derivatives as well as ETF and ETC derivatives, which are based on Xetra® listings: 16 May, 3 October` — the German scope, so FDAX and FDXM close | `EUREX-CAL-2016` | T1 | Eurex event date 2016-10-03 (German Unity Day); one Berlin civil day per trade date, so the conversion is the identity |
| 2016-12-26 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: 1 January, 25 March, 28 March, 26 December` | `EUREX-CAL-2016` | T1 | Eurex event date 2016-12-26 (Boxing Day); one Berlin civil day per trade date, so the conversion is the identity |

### 2017

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2017-04-14 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: 14 April, 17 April, 1 May, 25 December, 26 December` | `EUREX-CAL-2017` | T1 | Eurex event date 2017-04-14 (Good Friday); one Berlin civil day per trade date, so the conversion is the identity |
| 2017-04-17 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: 14 April, 17 April, 1 May, 25 December, 26 December` | `EUREX-CAL-2017` | T1 | Eurex event date 2017-04-17 (Easter Monday); one Berlin civil day per trade date, so the conversion is the identity |
| 2017-05-01 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: 14 April, 17 April, 1 May, 25 December, 26 December` | `EUREX-CAL-2017` | T1 | Eurex event date 2017-05-01 (Labour Day); one Berlin civil day per trade date, so the conversion is the identity |
| 2017-06-05 | closed | `Eurex is closed for trading and exercise in German equity and equity index derivatives as well as ETF and ETC derivatives, which are based on Xetra® listings: 5 June, 3 October, 31 October` — the German scope, so FDAX and FDXM close | `EUREX-CAL-2017` | T1 | Eurex event date 2017-06-05 (Whit Monday); one Berlin civil day per trade date, so the conversion is the identity |
| 2017-10-03 | closed | `Eurex is closed for trading and exercise in German equity and equity index derivatives as well as ETF and ETC derivatives, which are based on Xetra® listings: 5 June, 3 October, 31 October` — the German scope, so FDAX and FDXM close | `EUREX-CAL-2017` | T1 | Eurex event date 2017-10-03 (German Unity Day); one Berlin civil day per trade date, so the conversion is the identity |
| 2017-10-31 | closed | `Eurex is closed for trading and exercise in German equity and equity index derivatives as well as ETF and ETC derivatives, which are based on Xetra® listings: 5 June, 3 October, 31 October` — the German scope, so FDAX and FDXM close | `EUREX-CAL-2017` | T1 | Eurex event date 2017-10-31 (Reformation Day, the one-off 500th-anniversary German holiday); one Berlin civil day per trade date, so the conversion is the identity |
| 2017-12-25 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: 14 April, 17 April, 1 May, 25 December, 26 December` | `EUREX-CAL-2017` | T1 | Eurex event date 2017-12-25 (Christmas Day); one Berlin civil day per trade date, so the conversion is the identity |
| 2017-12-26 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: 14 April, 17 April, 1 May, 25 December, 26 December` | `EUREX-CAL-2017` | T1 | Eurex event date 2017-12-26 (Boxing Day); one Berlin civil day per trade date, so the conversion is the identity |

### 2018

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2018-01-01 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: 1 January, 30 March, 2 April, 1 May, 25 December, 26 December` | `EUREX-CAL-2018` | T1 | Eurex event date 2018-01-01 (New Year's Day); one Berlin civil day per trade date, so the conversion is the identity |
| 2018-03-30 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: 1 January, 30 March, 2 April, 1 May, 25 December, 26 December` | `EUREX-CAL-2018` | T1 | Eurex event date 2018-03-30 (Good Friday); one Berlin civil day per trade date, so the conversion is the identity |
| 2018-04-02 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: 1 January, 30 March, 2 April, 1 May, 25 December, 26 December` | `EUREX-CAL-2018` | T1 | Eurex event date 2018-04-02 (Easter Monday); one Berlin civil day per trade date, so the conversion is the identity |
| 2018-05-01 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: 1 January, 30 March, 2 April, 1 May, 25 December, 26 December` | `EUREX-CAL-2018` | T1 | Eurex event date 2018-05-01 (Labour Day); one Berlin civil day per trade date, so the conversion is the identity |
| 2018-05-21 | closed | `Eurex is closed for trading and exercise in German equity and equity index derivatives as well as ETF and ETC derivatives, which are based on Xetra® listings: 21 May, 3 October` — the German scope, so FDAX and FDXM close | `EUREX-CAL-2018` | T1 | Eurex event date 2018-05-21 (Whit Monday); one Berlin civil day per trade date, so the conversion is the identity |
| 2018-10-03 | closed | `Eurex is closed for trading and exercise in German equity and equity index derivatives as well as ETF and ETC derivatives, which are based on Xetra® listings: 21 May, 3 October` — the German scope, so FDAX and FDXM close | `EUREX-CAL-2018` | T1 | Eurex event date 2018-10-03 (German Unity Day); one Berlin civil day per trade date, so the conversion is the identity |
| 2018-12-24 | closed | `Eurex is closed for trading in all derivatives: 24 December, 31 December` — a full trading closure; clearing stays open | `EUREX-CAL-2018` | T1 | Eurex event date 2018-12-24 (Christmas Eve); one Berlin civil day per trade date, so the conversion is the identity |
| 2018-12-25 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: 1 January, 30 March, 2 April, 1 May, 25 December, 26 December` | `EUREX-CAL-2018` | T1 | Eurex event date 2018-12-25 (Christmas Day); one Berlin civil day per trade date, so the conversion is the identity |
| 2018-12-26 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: 1 January, 30 March, 2 April, 1 May, 25 December, 26 December` | `EUREX-CAL-2018` | T1 | Eurex event date 2018-12-26 (Boxing Day); one Berlin civil day per trade date, so the conversion is the identity |
| 2018-12-31 | closed | `Eurex is closed for trading in all derivatives: 24 December, 31 December` — a full trading closure; clearing stays open | `EUREX-CAL-2018` | T1 | Eurex event date 2018-12-31 (New Year's Eve); one Berlin civil day per trade date, so the conversion is the identity |

### 2019

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2019-01-01 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: 1 January, 19 April, 22 April, 1 May, 25 December, 26 December` | `EUREX-CAL-2019` | T1 | Eurex event date 2019-01-01 (New Year's Day); one Berlin civil day per trade date, so the conversion is the identity |
| 2019-04-19 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: 1 January, 19 April, 22 April, 1 May, 25 December, 26 December` | `EUREX-CAL-2019` | T1 | Eurex event date 2019-04-19 (Good Friday); one Berlin civil day per trade date, so the conversion is the identity |
| 2019-04-22 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: 1 January, 19 April, 22 April, 1 May, 25 December, 26 December` | `EUREX-CAL-2019` | T1 | Eurex event date 2019-04-22 (Easter Monday); one Berlin civil day per trade date, so the conversion is the identity |
| 2019-05-01 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: 1 January, 19 April, 22 April, 1 May, 25 December, 26 December` | `EUREX-CAL-2019` | T1 | Eurex event date 2019-05-01 (Labour Day); one Berlin civil day per trade date, so the conversion is the identity |
| 2019-12-24 | closed | `Eurex is closed for trading in all derivatives: 24 December, 31 December` — a full trading closure; clearing stays open | `EUREX-CAL-2019` | T1 | Eurex event date 2019-12-24 (Christmas Eve); one Berlin civil day per trade date, so the conversion is the identity |
| 2019-12-25 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: 1 January, 19 April, 22 April, 1 May, 25 December, 26 December` | `EUREX-CAL-2019` | T1 | Eurex event date 2019-12-25 (Christmas Day); one Berlin civil day per trade date, so the conversion is the identity |
| 2019-12-26 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: 1 January, 19 April, 22 April, 1 May, 25 December, 26 December` | `EUREX-CAL-2019` | T1 | Eurex event date 2019-12-26 (Boxing Day); one Berlin civil day per trade date, so the conversion is the identity |
| 2019-12-31 | closed | `Eurex is closed for trading in all derivatives: 24 December, 31 December` — a full trading closure; clearing stays open | `EUREX-CAL-2019` | T1 | Eurex event date 2019-12-31 (New Year's Eve); one Berlin civil day per trade date, so the conversion is the identity |

### 2020

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2020-01-01 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: 1 January, 10 April, 13 April, 1 May, 25 December` | `EUREX-CAL-2020` | T1 | Eurex event date 2020-01-01 (New Year's Day); one Berlin civil day per trade date, so the conversion is the identity |
| 2020-04-10 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: 1 January, 10 April, 13 April, 1 May, 25 December` | `EUREX-CAL-2020` | T1 | Eurex event date 2020-04-10 (Good Friday); one Berlin civil day per trade date, so the conversion is the identity |
| 2020-04-13 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: 1 January, 10 April, 13 April, 1 May, 25 December` | `EUREX-CAL-2020` | T1 | Eurex event date 2020-04-13 (Easter Monday); one Berlin civil day per trade date, so the conversion is the identity |
| 2020-05-01 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: 1 January, 10 April, 13 April, 1 May, 25 December` | `EUREX-CAL-2020` | T1 | Eurex event date 2020-05-01 (Labour Day); one Berlin civil day per trade date, so the conversion is the identity |
| 2020-12-24 | closed | `Eurex is closed for trading in all derivatives: 24 December, 31 December` — a full trading closure; clearing stays open | `EUREX-CAL-2020` | T1 | Eurex event date 2020-12-24 (Christmas Eve); one Berlin civil day per trade date, so the conversion is the identity |
| 2020-12-25 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: 1 January, 10 April, 13 April, 1 May, 25 December` | `EUREX-CAL-2020` | T1 | Eurex event date 2020-12-25 (Christmas Day); one Berlin civil day per trade date, so the conversion is the identity |
| 2020-12-31 | closed | `Eurex is closed for trading in all derivatives: 24 December, 31 December` — a full trading closure; clearing stays open | `EUREX-CAL-2020` | T1 | Eurex event date 2020-12-31 (New Year's Eve); one Berlin civil day per trade date, so the conversion is the identity |

### 2021

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2021-01-01 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: 1 January, 2 April, 5 April` | `EUREX-CAL-2021` | T1 | Eurex event date 2021-01-01 (New Year's Day); one Berlin civil day per trade date, so the conversion is the identity |
| 2021-04-02 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: 1 January, 2 April, 5 April` | `EUREX-CAL-2021` | T1 | Eurex event date 2021-04-02 (Good Friday); one Berlin civil day per trade date, so the conversion is the identity |
| 2021-04-05 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: 1 January, 2 April, 5 April` | `EUREX-CAL-2021` | T1 | Eurex event date 2021-04-05 (Easter Monday); one Berlin civil day per trade date, so the conversion is the identity |
| 2021-12-24 | closed | `Eurex is closed for trading in all derivatives: 24 December, 31 December` — a full trading closure; clearing stays open | `EUREX-CAL-2021` | T1 | Eurex event date 2021-12-24 (Christmas Eve); one Berlin civil day per trade date, so the conversion is the identity |
| 2021-12-31 | closed | `Eurex is closed for trading in all derivatives: 24 December, 31 December` — a full trading closure; clearing stays open | `EUREX-CAL-2021` | T1 | Eurex event date 2021-12-31 (New Year's Eve); one Berlin civil day per trade date, so the conversion is the identity |

### 2022

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2022-04-15 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: 15 April, 18 April, 26 December` | `EUREX-CAL-2022` | T1 | Eurex event date 2022-04-15 (Good Friday); one Berlin civil day per trade date, so the conversion is the identity |
| 2022-04-18 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: 15 April, 18 April, 26 December` | `EUREX-CAL-2022` | T1 | Eurex event date 2022-04-18 (Easter Monday); one Berlin civil day per trade date, so the conversion is the identity |
| 2022-12-26 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: 15 April, 18 April, 26 December` | `EUREX-CAL-2022` | T1 | Eurex event date 2022-12-26 (Boxing Day); one Berlin civil day per trade date, so the conversion is the identity |

### 2023

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2023-04-07 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: 7 April, 10 April, 1 May, 25 December, 26 December` | `EUREX-CAL-2023` | T1 | Eurex event date 2023-04-07 (Good Friday); one Berlin civil day per trade date, so the conversion is the identity |
| 2023-04-10 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: 7 April, 10 April, 1 May, 25 December, 26 December` | `EUREX-CAL-2023` | T1 | Eurex event date 2023-04-10 (Easter Monday); one Berlin civil day per trade date, so the conversion is the identity |
| 2023-05-01 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: 7 April, 10 April, 1 May, 25 December, 26 December` | `EUREX-CAL-2023` | T1 | Eurex event date 2023-05-01 (Labour Day); one Berlin civil day per trade date, so the conversion is the identity |
| 2023-12-25 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: 7 April, 10 April, 1 May, 25 December, 26 December` | `EUREX-CAL-2023` | T1 | Eurex event date 2023-12-25 (Christmas Day); one Berlin civil day per trade date, so the conversion is the identity |
| 2023-12-26 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: 7 April, 10 April, 1 May, 25 December, 26 December` | `EUREX-CAL-2023` | T1 | Eurex event date 2023-12-26 (Boxing Day); one Berlin civil day per trade date, so the conversion is the identity |

### 2024

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2024-01-01 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: 1 January, 29 March, 1 April, 1 May, 25 December, 26 December` | `EUREX-CAL-2024` | T1 | Eurex event date 2024-01-01 (New Year's Day); one Berlin civil day per trade date, so the conversion is the identity |
| 2024-03-29 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: 1 January, 29 March, 1 April, 1 May, 25 December, 26 December` | `EUREX-CAL-2024` | T1 | Eurex event date 2024-03-29 (Good Friday); one Berlin civil day per trade date, so the conversion is the identity |
| 2024-04-01 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: 1 January, 29 March, 1 April, 1 May, 25 December, 26 December` | `EUREX-CAL-2024` | T1 | Eurex event date 2024-04-01 (Easter Monday); one Berlin civil day per trade date, so the conversion is the identity |
| 2024-05-01 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: 1 January, 29 March, 1 April, 1 May, 25 December, 26 December` | `EUREX-CAL-2024` | T1 | Eurex event date 2024-05-01 (Labour Day); one Berlin civil day per trade date, so the conversion is the identity |
| 2024-12-24 | closed | `Eurex is closed for trading in all derivatives: 24 December, 31 December` — a full trading closure; clearing stays open | `EUREX-CAL-2024` | T1 | Eurex event date 2024-12-24 (Christmas Eve); one Berlin civil day per trade date, so the conversion is the identity |
| 2024-12-25 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: 1 January, 29 March, 1 April, 1 May, 25 December, 26 December` | `EUREX-CAL-2024` | T1 | Eurex event date 2024-12-25 (Christmas Day); one Berlin civil day per trade date, so the conversion is the identity |
| 2024-12-26 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: 1 January, 29 March, 1 April, 1 May, 25 December, 26 December` | `EUREX-CAL-2024` | T1 | Eurex event date 2024-12-26 (Boxing Day); one Berlin civil day per trade date, so the conversion is the identity |
| 2024-12-31 | closed | `Eurex is closed for trading in all derivatives: 24 December, 31 December` — a full trading closure; clearing stays open | `EUREX-CAL-2024` | T1 | Eurex event date 2024-12-31 (New Year's Eve); one Berlin civil day per trade date, so the conversion is the identity |

### 2025

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2025-01-01 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives.` | `EUREX-HOLREG-2025` | T1 | Eurex event date 2025-01-01 (New Year's Day); one Berlin civil day per trade date, so the conversion is the identity |
| 2025-04-18 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives.` | `EUREX-HOLREG-2025` | T1 | Eurex event date 2025-04-18 (Good Friday) |
| 2025-04-21 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives.` | `EUREX-HOLREG-2025` | T1 | Eurex event date 2025-04-21 (Easter Monday) |
| 2025-05-01 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives.` | `EUREX-HOLREG-2025` | T1 | Eurex event date 2025-05-01 (Labour Day) |
| 2025-12-24 | closed | `Eurex is closed for trading in all derivatives.` — a full trading closure; clearing stays open | `EUREX-HOLREG-2025` | T1 | Eurex event date 2025-12-24 (Christmas Eve) |
| 2025-12-25 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives.` | `EUREX-HOLREG-2025` | T1 | Eurex event date 2025-12-25 (Christmas Day) |
| 2025-12-26 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives.` | `EUREX-HOLREG-2025` | T1 | Eurex event date 2025-12-26 (Boxing Day) |
| 2025-12-31 | closed | `Eurex is closed for trading in all derivatives.` — a full trading closure; clearing stays open | `EUREX-HOLREG-2025` | T1 | Eurex event date 2025-12-31 (New Year's Eve) |

### 2026

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2026-01-01 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives.` | `EUREX-HOLREG-2026` | T1 | Eurex event date 2026-01-01 (New Year's Day); one Berlin civil day per trade date, so the conversion is the identity |
| 2026-04-03 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives.` | `EUREX-HOLREG-2026` | T1 | Eurex event date 2026-04-03 (Good Friday) |
| 2026-04-06 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives.` | `EUREX-HOLREG-2026` | T1 | Eurex event date 2026-04-06 (Easter Monday) |
| 2026-05-01 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives.` | `EUREX-HOLREG-2026` | T1 | Eurex event date 2026-05-01 (Labour Day) |
| 2026-12-24 | closed | `Eurex is closed for trading in all derivatives: 24 December, 31 December` — a full trading closure; clearing stays open | `EUREX-HOLREG-2026` | T1 | Eurex event date 2026-12-24 (Christmas Eve) |
| 2026-12-25 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives.` | `EUREX-HOLREG-2026` | T1 | Eurex event date 2026-12-25 (Christmas Day) |
| 2026-12-31 | closed | `Eurex is closed for trading in all derivatives: 24 December, 31 December` — a full trading closure; clearing stays open | `EUREX-HOLREG-2026` | T1 | Eurex event date 2026-12-31 (New Year's Eve) |

**Gaps, 2026:** none remain. The former "**Eurex 2027 does not ship**" state was superseded on 2026-11-18 UTC by the resolution recorded in [eurex](eurex.md): the Holiday regulations page's own `Non-trading days at Eurex 2026 - 2030` table keys 2027-2030 and the indicative CSV remains non-keying. The Trading Calendar 2026 PDF's German-scope `to be announced` line verified to **no closures** on 2026-10-05 UTC (#157, retired), and no 2027-2030 country-scope section exists on the page at all, so no German-scope addition is encodable for the extension years.

**Interpretive steps, 2026:** None. Eurex states its closures as whole days for “all derivatives”, every phase the crate models runs inside one Berlin civil day, and Eurex publishes no early close — 24 and 31 December are full trading closures with clearing open, not half days. So each event date is its own trade date and each row is `Closed`.

### 2027

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2027-01-01 | closed | `New Year's Day — Friday 01 Jan 2027` | `EUREX-HOLREG-2027-2030` | T1 | Eurex event date 2027-01-01 (New Year's Day); one Berlin civil day per trade date, so the conversion is the identity |
| 2027-03-26 | closed | `Good Friday — Friday 26 Mar 2027` | `EUREX-HOLREG-2027-2030` | T1 | Eurex event date 2027-03-26 (Good Friday); one Berlin civil day per trade date, so the conversion is the identity |
| 2027-03-29 | closed | `Easter Monday — Monday 29 Mar 2027` | `EUREX-HOLREG-2027-2030` | T1 | Eurex event date 2027-03-29 (Easter Monday); one Berlin civil day per trade date, so the conversion is the identity |
| 2027-12-24 | closed | `Christmas Eve* — Friday 24 Dec 2027` — a full trading closure | `EUREX-HOLREG-2027-2030` | T1 | Eurex event date 2027-12-24 (Christmas Eve); one Berlin civil day per trade date, so the conversion is the identity |
| 2027-12-31 | closed | `New Year's Eve* — Friday 31 Dec 2027` — a full trading closure | `EUREX-HOLREG-2027-2030` | T1 | Eurex event date 2027-12-31 (New Year's Eve); one Berlin civil day per trade date, so the conversion is the identity |

**Gaps, 2027:** none in the Eurex-wide set. The table's weekend entries (`Saturday 01 May 2027` Labour Day, `Saturday 25 Dec 2027` Christmas Day, `Sunday 26 Dec 2027` Boxing Day) key no weekday row; no country-scope section exists for 2027 (see [eurex](eurex.md)).

### 2028

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2028-04-14 | closed | `Good Friday — Friday 14 Apr 2028` | `EUREX-HOLREG-2027-2030` | T1 | Eurex event date 2028-04-14 (Good Friday); one Berlin civil day per trade date, so the conversion is the identity |
| 2028-04-17 | closed | `Easter Monday — Monday 17 Apr 2028` | `EUREX-HOLREG-2027-2030` | T1 | Eurex event date 2028-04-17 (Easter Monday); one Berlin civil day per trade date, so the conversion is the identity |
| 2028-05-01 | closed | `Labour Day — Monday 01 May 2028` | `EUREX-HOLREG-2027-2030` | T1 | Eurex event date 2028-05-01 (Labour Day); one Berlin civil day per trade date, so the conversion is the identity |
| 2028-12-25 | closed | `Christmas Day — Monday 25 Dec 2028` | `EUREX-HOLREG-2027-2030` | T1 | Eurex event date 2028-12-25 (Christmas Day); one Berlin civil day per trade date, so the conversion is the identity |
| 2028-12-26 | closed | `Boxing Day — Tuesday 26 Dec 2028` | `EUREX-HOLREG-2027-2030` | T1 | Eurex event date 2028-12-26 (Boxing Day); one Berlin civil day per trade date, so the conversion is the identity |

**Gaps, 2028:** none in the Eurex-wide set. The table's weekend entries (`Saturday 01 Jan 2028` New Year's Day, `Sunday 24 Dec 2028` Christmas Eve\*, `Sunday 31 Dec 2028` New Year's Eve\*) key no weekday row; no country-scope section exists for 2028.

### 2029

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2029-01-01 | closed | `New Year's Day — Monday 01 Jan 2029` | `EUREX-HOLREG-2027-2030` | T1 | Eurex event date 2029-01-01 (New Year's Day); one Berlin civil day per trade date, so the conversion is the identity |
| 2029-03-30 | closed | `Good Friday — Friday 30 Mar 2029` | `EUREX-HOLREG-2027-2030` | T1 | Eurex event date 2029-03-30 (Good Friday); one Berlin civil day per trade date, so the conversion is the identity |
| 2029-04-02 | closed | `Easter Monday — Monday 02 Apr 2029` | `EUREX-HOLREG-2027-2030` | T1 | Eurex event date 2029-04-02 (Easter Monday); one Berlin civil day per trade date, so the conversion is the identity |
| 2029-05-01 | closed | `Labour Day — Tuesday 01 May 2029` | `EUREX-HOLREG-2027-2030` | T1 | Eurex event date 2029-05-01 (Labour Day); one Berlin civil day per trade date, so the conversion is the identity |
| 2029-12-24 | closed | `Christmas Eve* — Monday 24 Dec 2029` — a full trading closure | `EUREX-HOLREG-2027-2030` | T1 | Eurex event date 2029-12-24 (Christmas Eve); one Berlin civil day per trade date, so the conversion is the identity |
| 2029-12-25 | closed | `Christmas Day — Tuesday 25 Dec 2029` | `EUREX-HOLREG-2027-2030` | T1 | Eurex event date 2029-12-25 (Christmas Day); one Berlin civil day per trade date, so the conversion is the identity |
| 2029-12-26 | closed | `Boxing Day — Wednesday 26 Dec 2029` | `EUREX-HOLREG-2027-2030` | T1 | Eurex event date 2029-12-26 (Boxing Day); one Berlin civil day per trade date, so the conversion is the identity |
| 2029-12-31 | closed | `New Year's Eve* — Monday 31 Dec 2029` — a full trading closure | `EUREX-HOLREG-2027-2030` | T1 | Eurex event date 2029-12-31 (New Year's Eve); one Berlin civil day per trade date, so the conversion is the identity |

**Gaps, 2029:** none. All eight of the table's 2029 dates fall on weekdays and ship; no country-scope section exists for 2029.

### 2030

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2030-01-01 | closed | `New Year's Day — Tuesday 01 Jan 2030` | `EUREX-HOLREG-2027-2030` | T1 | Eurex event date 2030-01-01 (New Year's Day); one Berlin civil day per trade date, so the conversion is the identity |
| 2030-04-19 | closed | `Good Friday — Friday 19 Apr 2030` | `EUREX-HOLREG-2027-2030` | T1 | Eurex event date 2030-04-19 (Good Friday); one Berlin civil day per trade date, so the conversion is the identity |
| 2030-04-22 | closed | `Easter Monday — Monday 22 Apr 2030` | `EUREX-HOLREG-2027-2030` | T1 | Eurex event date 2030-04-22 (Easter Monday); one Berlin civil day per trade date, so the conversion is the identity |
| 2030-05-01 | closed | `Labour Day — Wednesday 01 May 2030` | `EUREX-HOLREG-2027-2030` | T1 | Eurex event date 2030-05-01 (Labour Day); one Berlin civil day per trade date, so the conversion is the identity |
| 2030-12-24 | closed | `Christmas Eve* — Tuesday 24 Dec 2030` — a full trading closure | `EUREX-HOLREG-2027-2030` | T1 | Eurex event date 2030-12-24 (Christmas Eve); one Berlin civil day per trade date, so the conversion is the identity |
| 2030-12-25 | closed | `Christmas Day — Wednesday 25 Dec 2030` | `EUREX-HOLREG-2027-2030` | T1 | Eurex event date 2030-12-25 (Christmas Day); one Berlin civil day per trade date, so the conversion is the identity |
| 2030-12-26 | closed | `Boxing Day — Thursday 26 Dec 2030` | `EUREX-HOLREG-2027-2030` | T1 | Eurex event date 2030-12-26 (Boxing Day); one Berlin civil day per trade date, so the conversion is the identity |
| 2030-12-31 | closed | `New Year's Eve* — Tuesday 31 Dec 2030` — a full trading closure | `EUREX-HOLREG-2027-2030` | T1 | Eurex event date 2030-12-31 (New Year's Eve); one Berlin civil day per trade date, so the conversion is the identity |

**Gaps, 2030:** none. All eight of the table's 2030 dates fall on weekdays and ship; no country-scope section exists for 2030.

## Sources

Row review: 2026-08-22 (UTC) is the date the ledger row was last reviewed as a
whole. Per-source retrieval dates were not recorded before the 2026-09-12
migration; where a later targeted review, capture or document date is recorded
beside a source below, that date governs for that source, and dates are added
as each source is re-verified.

The row rests on the same `EU-EUREX`
documents as the `eurex` exchange row; the full annotated list is in
[`eurex.md`](eurex.md#sources). The documents that key the modelled states are:

- <https://www.eurex.com/resource/blob/296888/978b08fe3a240a0b4a8fb62a2647a197/data/cs_history_26102009_en.pdf.pdf> — the official 2009 archived contract specifications, pinning the grid in force at the January-2010 floor — T1.
- <https://www.eurex.com/resource/blob/1412768/e61a2c41d65ad165af7909002223b943/data/er18088e.pdf> — Eurex circular 088/2018 and its Annex C, effective 2018-12-10 — T1.
- <https://www.eurex.com/resource/blob/1448250/29a4179e4d28742af5d0ee85f9af89f8/data/Eurex%20Asian%20Trading%20Hours_Nov%202018.pdf> — the Asian-hours launch phase diagram, which splits pre-trading from the opening auction — T1.
- <https://www.eurex.com/resource/blob/2824010/3b94b95cdf5f31cc635294659a5e9786/data/2026_05_04_eurex_d_kontraktspezifikationen_annexe_en.pdf> — the current contract-specification Annex C — T1.
- <https://www.eurex.com/ex-en/trade/trading-hours> — Eurex trading hours, the current-schedule monitoring entry point — T1.

## Gaps and residual risks

- **No dated-history gap.** The 2009 archived specification predates the
  January-2010 floor, so the baseline is sourced through the floor and the
  horizon is the floor itself.
- **The key is seasonal, so it must be queried with an instant.** The CET and
  CEST phase tables differ by an hour at the open, and the Asian-hours open is a
  fixed 00:00 UTC instant rather than a fixed local time. A detached fixed
  snapshot carries whichever table it was built from and has no seasonal
  behaviour of its own.
- **Scope.** The key names the FESX/FDAX/FDXM benchmark-index family, not a
  venue-wide Eurex clock. Eurex fixed-income futures have their own key and
  their own dated revisions.
- **Dormant identity (LAW-SERVICE-TIERS).** A venue namespace reaches an
  `Exchange`, never a key, so no SharurPlatform adapter can reach this row and
  no family-map root points at it. It is reviewed on demand, and any future gap
  is recorded here rather than opened as an issue.

> Shared module. The narrative for
> [`europe.rs`](../../src/calendar/schedules/futures/international/europe.rs)
> lives in [`eurex`](eurex.md#module-narrative-moved-from-srccalendarschedulesfuturesinternationaleuropers-on-2026-09-12-utc).
> Sibling identity: [`eex`](eex.md).
