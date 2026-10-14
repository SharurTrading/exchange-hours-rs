<!-- SPDX-License-Identifier: MIT-0 -->

# `bme_ibex` — evidence

- **Kind:** `MarketHoursKey`
- **Owner module:** [`bme_ibex.rs`](../../src/calendar/schedules/futures/international/bme_ibex.rs)
- **Source sets:** [`EU-MEFF`](../schedules/sources.md#eu-meff)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-10-14 UTC)

MEFF IBEX 35 futures (with Mini and Micro IBEX 35) run one same-day
Monday–Friday Madrid session whose 2017 and 2025 cutovers are operator-dated;
the 2010 floor grid is sourced from the operator's own product page and the
changeover into the 2017 grid is an undated span served as the sourced
intersection.

## Revision rows

- 2017-01-23 — T1 — MEFF extends trading in IBEX 35 and Mini IBEX futures by an hour — the operator's own same-day press release opens the 08:00 era: "Starting today … Trading then starts at 8:00 with a 5-minute auction from 7:55 h to 8:00 h … Trading Period IBEX 35 and Mini IBEX Futures 8:00 - 20:00 h".
- 2025-06-23 — T1 — MEFF notice 16/2025 — notice 16/2025 of 2 April 2025 states "on 23 June, the trading hours for IBEX 35 Futures, Mini IBEX Futures, and Micro Futures on the electronic order system will be extended until 10:00 p.m." and prints the row "7:55 – 8:00 | 8:00 – 22:00"; the reminder notice 25/2025 repeats it.

## The normal week

The floor profile is the operator's own product page, Wayback capture
2010-06-18 (`MEFF-PAGE-2010`): "AUCTION PERIOD — From 8:30 a.m. to 9:00 a.m."
and "TRADING HOURS — From 9:00 a.m. to 5:35 p.m.". The 2017 grid is restated
by the operator's archived Trading Hours page of 2017-11-23
(`MEFF-HOURS-2017`): "IBEX 35 y Mini IBEX | 7:55 – 8:00 | 8:00 – 20:00". The
current grid is the operator's live Trading Hours page ("Futures IBEX 35 y
Mini IBEX | 7:55 – 8:00 | 8:00 – 22:00") and live product page ("AUCTION
PERIOD — From 7:55 a.m. to 8:00 a.m. TRADING HOURS — From 8:00 a.m. to 22:00
p.m."), both `MEFF-HOURS-LIVE`/`MEFF-IBEX35-LIVE`, retrieved 2026-10-11 UTC.

Every era is one same-day session with no lunch or maintenance gap: `regular`
carries the whole trading period, the opening auction is `order_entry` (orders
entered, nothing matches before the open), and `extended` is empty. The
session never wraps past local midnight, so a trade date is its own civil day.

The operator's notices print an "additional random period of 30 seconds
maximum" on the opening auction (2017 press release; notice 16/2025). The
crate keeps the operator's published nominal 08:00 trading-period boundary —
the charter's exchange-level-boundaries convention, the randomized seconds
only moving an adjacent phase handoff.

**Undated changeover (the served-intersection residual).** The 2010 floor grid
closes 17:35 and the 2017 press release's own "extends by an hour" wording
implies a 20:00 close already existed before the 08:00 open moved, so at least
one interim change (17:35→20:00) took effect on an undated day between
2010-06-18 and 2017-01-23. A full CDX domain sweep of meff.es for 2010–2016
(3,958 URLs, retrieval 2026-10-11 UTC) found no surviving hours page or press
release for the interim states, and the 2013 Index Futures General Conditions
PDF 404s live and is unarchived. Under the charter's 2026-10-04 convention the
timeline serves the sourced intersection — the 09:00–17:35 window that holds
under every sourced state, with the auction 08:30–09:00 — from the floor to
2017-01-22, and withholds the disputed remainder (any earlier open, any later
close) until a dated artifact replaces it. Closing condition: the MEFF
Circular C-EX-DF series archive or a desk answer.

**The conditional 2024 slip.** Notice 7/2024 of 9 April 2024 announced the
22:00 extension "el lunes 1 de julio, si bien la fecha se confirmará una vez
la Comisión Nacional del Mercado de Valores haya procedido a la valoración del
nuevo horario" — a CNMV-conditional date, which under LAW-NO-FABRICATED-DATES
never keys a row. Notice 13/2024 of 30 May 2024 then announced "el retraso de
la ampliación de dicho horario de negociación", and notice 16/2025 dated the
change unconditionally to 2025-06-23. The crate encodes only that last,
unconditional day; no 2024 row exists.

**LAW-SESSION-NOT-EXPIRY.** Notice 16/2025's "Market end: 22:05", the
prearranged-trades and RFQ close at 20:15, and notice 49/2025's 13:54 VWAP
settlement window and 14:30 application-acceptance end are calculation,
clearing and post-trade instants, not session boundaries; none is encoded.
The 14:00 "Cierre de Negociación" of the Christmas eves **is** session
language and is encoded as a holiday-table early close.

## Holidays

**Coverage:** 2010-01-01..2011-12-31, 2014-01-01..2026-12-31 (MEFF derivatives
trade dates, Europe/Madrid; BME-wide operator calendars that name MEFF).

The closure set is constant across every audited year: 1 January, Good
Friday, Easter Monday, 1 May (when a weekday), 25 December, and 26 December in
lieu. The operator never closes for Epiphany, 12 October, 15 August, 1
November, 6 or 8 December — those days traded in every audited year. 24 and
31 December are 14:00 early closes in the years they fall on weekdays (2014,
2015, 2018–2020, 2024–2026); 2021 closed both fully; 2010 closed 31 December
fully while 24 December 2010 traded. The early-close instant is the
operator's "trade until 14:00 hrs CET" (2018–2020 releases, the live
calendar page) and notice 14/2024 and notice 49/2025's MEFF-specific "Cierre
de Negociación: 14:00 h". The rows are keyed to the operator's civil session
date, which for this same-day family is the trade date.

2012 and 2013 sit inside no window: the aspx calendar page has Wayback
captures only for 2010, 2011 and 2014–2016, the BME press-release series
begins with the 2016 calendar (20 November 2015), and the hunter's CDX sweep
found no 2012–2013 grid. Closing condition: the Circular C-EX-DF series or a
desk answer. The span is the charter's bridged residual — session questions
answer from the sourced normal week while the holiday classification refuses
typed.

### 2010

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2010-01-01 | closed | − struck festivo cell, 1 Enero | `MEFF-CAL-2010` | T1 | Operator's printed non-trading day; same local civil trade date |
| 2010-04-02 | closed | − struck festivo cell, 2 Abril (Good Friday) | `MEFF-CAL-2010` | T1 | Operator's printed non-trading day; same local civil trade date |
| 2010-04-05 | closed | − struck festivo cell, 5 Abril (Easter Monday) | `MEFF-CAL-2010` | T1 | Operator's printed non-trading day; same local civil trade date |
| 2010-12-31 | closed | − struck festivo cell, 31 Diciembre | `MEFF-CAL-2010` | T1 | Operator's printed non-trading day; 24 December 2010 carries no strike and traded |

### 2011

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2011-04-22 | closed | − `celdaFestivo` cell, 22 abril (Good Friday) | `MEFF-GRID-2011` | T1 | Operator's calendar page festivo cell; same local civil trade date |
| 2011-04-25 | closed | − `celdaFestivo` cell, 25 abril (Easter Monday) | `MEFF-GRID-2011` | T1 | Operator's calendar page festivo cell; same local civil trade date |
| 2011-12-26 | closed | − `celdaFestivo` cell, 26 diciembre | `MEFF-GRID-2011` | T1 | Operator's calendar page festivo cell; 24 and 31 December 2011 were Saturdays and carry no rows |

### 2014

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2014-01-01 | closed | − `celdaFestivo` cell, 1 enero | `MEFF-GRID-2014` | T1 | Operator's calendar page festivo cell; same local civil trade date |
| 2014-04-18 | closed | − `celdaFestivo` cell, 18 abril (Good Friday) | `MEFF-GRID-2014` | T1 | Operator's calendar page festivo cell; same local civil trade date |
| 2014-04-21 | closed | − `celdaFestivo` cell, 21 abril (Easter Monday) | `MEFF-GRID-2014` | T1 | Operator's calendar page festivo cell; same local civil trade date |
| 2014-05-01 | closed | − `celdaFestivo` cell, 1 mayo | `MEFF-GRID-2014` | T1 | Operator's calendar page festivo cell; same local civil trade date |
| 2014-12-24 | early close | 14:00 — "Los días 24 y 31 de diciembre serán hábiles hasta las 14h" | `MEFF-GRID-2014` | T1 | Operator's page note; same local civil trade date |
| 2014-12-25 | closed | − `celdaFestivo` cell, 25 diciembre | `MEFF-GRID-2014` | T1 | Operator's calendar page festivo cell; same local civil trade date |
| 2014-12-26 | closed | − `celdaFestivo` cell, 26 diciembre | `MEFF-GRID-2014` | T1 | Operator's calendar page festivo cell; same local civil trade date |
| 2014-12-31 | early close | 14:00 — "Los días 24 y 31 de diciembre serán hábiles hasta las 14h" | `MEFF-GRID-2014` | T1 | Operator's page note; same local civil trade date |

### 2015

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2015-01-01 | closed | − `celdaFestivo` cell, 1 enero | `MEFF-GRID-2015` | T1 | Operator's calendar page festivo cell; same local civil trade date |
| 2015-04-03 | closed | − `celdaFestivo` cell, 3 abril (Good Friday) | `MEFF-GRID-2015` | T1 | Operator's calendar page festivo cell; same local civil trade date |
| 2015-04-06 | closed | − `celdaFestivo` cell, 6 abril (Easter Monday) | `MEFF-GRID-2015` | T1 | Operator's calendar page festivo cell; same local civil trade date |
| 2015-05-01 | closed | − `celdaFestivo` cell, 1 mayo | `MEFF-GRID-2015` | T1 | Operator's calendar page festivo cell; same local civil trade date |
| 2015-12-24 | early close | 14:00 — "Los días 24 y 31 de diciembre serán hábiles hasta las 14h" | `MEFF-GRID-2015` | T1 | Operator's page note; same local civil trade date |
| 2015-12-25 | closed | − `celdaFestivo` cell, 25 diciembre | `MEFF-GRID-2015` | T1 | Operator's calendar page festivo cell; same local civil trade date |
| 2015-12-31 | early close | 14:00 — "Los días 24 y 31 de diciembre serán hábiles hasta las 14h" | `MEFF-GRID-2015` | T1 | Operator's page note; same local civil trade date |

### 2016

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2016-01-01 | closed | − `celdaFestivo` cell, 1 enero; release "Friday, January 1st" | `MEFF-GRID-2016` | T1 | Operator's calendar page festivo cell, corroborated by `BME-NOTA-2016`; same local civil trade date |
| 2016-03-25 | closed | − `celdaFestivo` cell, 25 marzo (Good Friday); release "Friday, March 25th" | `MEFF-GRID-2016` | T1 | Operator's calendar page festivo cell, corroborated by `BME-NOTA-2016`; same local civil trade date |
| 2016-03-28 | closed | − `celdaFestivo` cell, 28 marzo (Easter Monday); release "Monday, March 28th" | `MEFF-GRID-2016` | T1 | Operator's calendar page festivo cell, corroborated by `BME-NOTA-2016`; same local civil trade date |
| 2016-12-26 | closed | − `celdaFestivo` cell, 26 diciembre; release "Monday, December 26th" | `MEFF-GRID-2016` | T1 | Operator's calendar page festivo cell, corroborated by `BME-NOTA-2016`; 24/31 December 2016 were Saturdays |

### 2017

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2017-04-14 | closed | − "Friday, April 14th" (Good Friday) | `BME-NOTA-2017` | T1 | BME release names MEFF among the Spanish markets it closes; same local civil trade date |
| 2017-04-17 | closed | − "Monday, April 17th" (Easter Monday) | `BME-NOTA-2017` | T1 | BME release names MEFF among the Spanish markets it closes; same local civil trade date |
| 2017-05-01 | closed | − "Monday, May 1st" | `BME-NOTA-2017` | T1 | BME release names MEFF among the Spanish markets it closes; same local civil trade date |
| 2017-12-25 | closed | − "Monday, December 25th" | `BME-NOTA-2017` | T1 | BME release names MEFF among the Spanish markets it closes; 24/31 December 2017 were Sundays |
| 2017-12-26 | closed | − "Tuesday, December 26th" | `BME-NOTA-2017` | T1 | BME release names MEFF among the Spanish markets it closes; same local civil trade date |

### 2018

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2018-01-01 | closed | − "Monday, January 1st" | `BME-NOTA-2018` | T1 | BME release names MEFF among the Spanish markets it closes; same local civil trade date |
| 2018-03-30 | closed | − "Friday, March 30th" (Good Friday) | `BME-NOTA-2018` | T1 | BME release names MEFF among the Spanish markets it closes; same local civil trade date |
| 2018-04-02 | closed | − "Monday, April 2nd" (Easter Monday) | `BME-NOTA-2018` | T1 | BME release names MEFF among the Spanish markets it closes; same local civil trade date |
| 2018-05-01 | closed | − "Tuesday, May 1st" | `BME-NOTA-2018` | T1 | BME release names MEFF among the Spanish markets it closes; same local civil trade date |
| 2018-12-24 | early close | 14:00 — "On 24 and 31 December the BME markets will trade until 14:00 hrs CET" | `BME-NOTA-2018` | T1 | Operator release half-day clause; same local civil trade date |
| 2018-12-25 | closed | − "Tuesday, December 25th" | `BME-NOTA-2018` | T1 | BME release names MEFF among the Spanish markets it closes; same local civil trade date |
| 2018-12-26 | closed | − "Wednesday, December 26th" | `BME-NOTA-2018` | T1 | BME release names MEFF among the Spanish markets it closes; same local civil trade date |
| 2018-12-31 | early close | 14:00 — "On 24 and 31 December the BME markets will trade until 14:00 hrs CET" | `BME-NOTA-2018` | T1 | Operator release half-day clause; same local civil trade date |

### 2019

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2019-01-01 | closed | − "Tuesday, January 1st" | `BME-NOTA-2019` | T1 | BME release names MEFF among the Spanish markets it closes; same local civil trade date |
| 2019-04-19 | closed | − "Friday, April 19th" (Good Friday) | `BME-NOTA-2019` | T1 | BME release names MEFF among the Spanish markets it closes; same local civil trade date |
| 2019-04-22 | closed | − "Monday, April 22nd" (Easter Monday) | `BME-NOTA-2019` | T1 | BME release names MEFF among the Spanish markets it closes; same local civil trade date |
| 2019-05-01 | closed | − "Wednesday, May 1st" | `BME-NOTA-2019` | T1 | BME release names MEFF among the Spanish markets it closes; same local civil trade date |
| 2019-12-24 | early close | 14:00 — "On 24 and 31 December the Markets will trade until 14:00 hrs CET" | `BME-NOTA-2019` | T1 | Operator release half-day clause; same local civil trade date |
| 2019-12-25 | closed | − "Wednesday, December 25th" | `BME-NOTA-2019` | T1 | BME release names MEFF among the Spanish markets it closes; same local civil trade date |
| 2019-12-26 | closed | − "Thursday, December 26th" | `BME-NOTA-2019` | T1 | BME release names MEFF among the Spanish markets it closes; same local civil trade date |
| 2019-12-31 | early close | 14:00 — "On 24 and 31 December the Markets will trade until 14:00 hrs CET" | `BME-NOTA-2019` | T1 | Operator release half-day clause; same local civil trade date |

### 2020

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2020-01-01 | closed | − "Wednesday, 1 January" | `BME-NOTA-2020` | T1 | BME release names MEFF among the Spanish markets it closes; same local civil trade date |
| 2020-04-10 | closed | − "Friday, 10 April" (Good Friday) | `BME-NOTA-2020` | T1 | BME release names MEFF among the Spanish markets it closes; same local civil trade date |
| 2020-04-13 | closed | − "Monday, 13 April" (Easter Monday) | `BME-NOTA-2020` | T1 | BME release names MEFF among the Spanish markets it closes; same local civil trade date |
| 2020-05-01 | closed | − "Friday, 1 May" | `BME-NOTA-2020` | T1 | BME release names MEFF among the Spanish markets it closes; same local civil trade date |
| 2020-12-24 | early close | 14:00 — "On Thursday 24 and Thursday 31 December the markets will trade until 14:00 hrs CET" | `BME-NOTA-2020` | T1 | Operator release half-day clause; same local civil trade date |
| 2020-12-25 | closed | − "Friday, 25 December" | `BME-NOTA-2020` | T1 | BME release names MEFF among the Spanish markets it closes; 26 December 2020 was a Saturday |
| 2020-12-31 | early close | 14:00 — "On Thursday 24 and Thursday 31 December the markets will trade until 14:00 hrs CET" | `BME-NOTA-2020` | T1 | Operator release half-day clause; same local civil trade date |

### 2021

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2021-01-01 | closed | − "Friday, 1 January" | `BME-NOTA-2021` | T1 | BME release names MEFF among the Spanish markets it closes; same local civil trade date |
| 2021-04-02 | closed | − "Friday, 2 April" (Good Friday) | `BME-NOTA-2021` | T1 | BME release names MEFF among the Spanish markets it closes; same local civil trade date |
| 2021-04-05 | closed | − "Monday, 5 April" (Easter Monday) | `BME-NOTA-2021` | T1 | BME release names MEFF among the Spanish markets it closes; same local civil trade date |
| 2021-12-24 | closed | − "Friday, 24 December" (full closure) | `BME-NOTA-2021` | T1 | BME release closes the day outright; 25 December 2021 was a Saturday |
| 2021-12-31 | closed | − "Friday, 31 December" (full closure) | `BME-NOTA-2021` | T1 | BME release closes the day outright; no half days in 2021 |

### 2022

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2022-04-15 | closed | − "Friday, 15 April" (Good Friday) | `BME-NOTA-2022` | T1 | BME release names MEFF among the Spanish markets it closes; same local civil trade date |
| 2022-04-18 | closed | − "Monday, 18 April" (Easter Monday) | `BME-NOTA-2022` | T1 | BME release names MEFF among the Spanish markets it closes; same local civil trade date |
| 2022-12-26 | closed | − "Monday, 26 December" (in lieu) | `BME-NOTA-2022` | T1 | BME release names MEFF among the Spanish markets it closes; 24/31 December 2022 were Saturdays, so no half days |

### 2023

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2023-04-07 | closed | − "Friday, 7 April" (Good Friday) | `BME-NOTA-2023` | T1 | BME release names MEFF among the Spanish markets it closes; same local civil trade date |
| 2023-04-10 | closed | − "Monday, 10 April" (Easter Monday) | `BME-NOTA-2023` | T1 | BME release names MEFF among the Spanish markets it closes; same local civil trade date |
| 2023-05-01 | closed | − "Monday, 1 May" | `BME-NOTA-2023` | T1 | BME release names MEFF among the Spanish markets it closes; same local civil trade date |
| 2023-12-25 | closed | − "Monday, 25 December" | `BME-NOTA-2023` | T1 | BME release names MEFF among the Spanish markets it closes; 24/31 December 2023 were Sundays |
| 2023-12-26 | closed | − "Tuesday, 26 December" | `BME-NOTA-2023` | T1 | BME release names MEFF among the Spanish markets it closes; same local civil trade date |

### 2024

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2024-01-01 | closed | − "Monday, 1 January" | `BME-NOTA-2024` | T1 | BME release names MEFF among the Spanish markets it closes; same local civil trade date |
| 2024-03-29 | closed | − "Friday, 29 March" (Good Friday) | `BME-NOTA-2024` | T1 | BME release names MEFF among the Spanish markets it closes; same local civil trade date |
| 2024-04-01 | closed | − "Monday, 1 April" (Easter Monday) | `BME-NOTA-2024` | T1 | BME release names MEFF among the Spanish markets it closes; same local civil trade date |
| 2024-05-01 | closed | − "Wednesday, 1 May" | `BME-NOTA-2024` | T1 | BME release names MEFF among the Spanish markets it closes; same local civil trade date |
| 2024-12-24 | early close | 14:00 — "Cierre de Negociación (horario corto y horario extendido): 14:00 h" | `MEFF-NOTICE-14-2024` | T1 | MEFF notice states the special session close in session language; same local civil trade date |
| 2024-12-25 | closed | − "Wednesday, 25 December" | `BME-NOTA-2024` | T1 | BME release names MEFF among the Spanish markets it closes; same local civil trade date |
| 2024-12-26 | closed | − "Thursday, 26 December" | `BME-NOTA-2024` | T1 | BME release names MEFF among the Spanish markets it closes; same local civil trade date |
| 2024-12-31 | early close | 14:00 — "Cierre de Negociación (horario corto y horario extendido): 14:00 h" | `MEFF-NOTICE-14-2024` | T1 | MEFF notice states the special session close in session language; same local civil trade date |

### 2025

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2025-01-01 | closed | − "Wednesday, 1 January" | `BME-NOTA-2025` | T1 | BME release names MEFF among the Spanish markets it closes; same local civil trade date |
| 2025-04-18 | closed | − "Friday, 18 April" (Good Friday) | `BME-NOTA-2025` | T1 | BME release names MEFF among the Spanish markets it closes; same local civil trade date |
| 2025-04-21 | closed | − "Monday, 21 April" (Easter Monday) | `BME-NOTA-2025` | T1 | BME release names MEFF among the Spanish markets it closes; same local civil trade date |
| 2025-05-01 | closed | − "Thursday, 1 May" | `BME-NOTA-2025` | T1 | BME release names MEFF among the Spanish markets it closes; same local civil trade date |
| 2025-12-24 | early close | 14:00 — "Cierre de Negociación: 14:00h" | `MEFF-NOTICE-49-2025` | T1 | MEFF notice states the 24 December session close in session language; same local civil trade date |
| 2025-12-25 | closed | − "Thursday, 25 December" | `BME-NOTA-2025` | T1 | BME release names MEFF among the Spanish markets it closes; same local civil trade date |
| 2025-12-26 | closed | − "Friday, 26 December" | `BME-NOTA-2025` | T1 | BME release names MEFF among the Spanish markets it closes; same local civil trade date |
| 2025-12-31 | early close | 14:00 — "Cierre de Negociación: 14:00h" | `MEFF-NOTICE-49-2025` | T1 | MEFF notice states the 31 December session close in session language; same local civil trade date |

### 2026

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2026-01-01 | closed | − "Thursday, 1 January" | `BME-NOTA-2026` | T1 | BME release names MEFF among the Spanish markets it closes; same local civil trade date |
| 2026-04-03 | closed | − "Friday, 3 April" (Good Friday) | `BME-NOTA-2026` | T1 | BME release names MEFF among the Spanish markets it closes; same local civil trade date |
| 2026-04-06 | closed | − "Monday, 6 April" (Easter Monday) | `BME-NOTA-2026` | T1 | BME release names MEFF among the Spanish markets it closes; same local civil trade date |
| 2026-05-01 | closed | − "Friday, 1 May" | `BME-NOTA-2026` | T1 | BME release names MEFF among the Spanish markets it closes; same local civil trade date |
| 2026-12-24 | early close | 14:00 — "On Thursday, 24 December and Thursday, 31 December, the market will be open until 2:00 pm" | `BME-NOTA-2026` | T1 | Operator release half-day clause; same local civil trade date |
| 2026-12-25 | closed | − "Friday, 25 December" | `BME-NOTA-2026` | T1 | BME release names MEFF among the Spanish markets it closes; 26 December 2026 is a Saturday |
| 2026-12-31 | early close | 14:00 — "On Thursday, 24 December and Thursday, 31 December, the market will be open until 2:00 pm" | `BME-NOTA-2026` | T1 | Operator release half-day clause; same local civil trade date |

### Documents

Artifacts live in the research store under
`normal-weeks/meff-ibex-2026-10-11/` (current pages, notices, releases,
2026 calendar) and `holidays/raw/meff-2010-2027/` (the 2010–2016 history
captures). Digests below re-verified against the store bytes on 2026-10-14
UTC.

| Document | Window | Replay or service URL | Capture or retrieval, UTC | Tier | sha256 |
|---|---|---|---|---|---|
| `MEFF-CAL-2010` | 2010-01-01..2010-12-31 | <https://web.archive.org/web/20100407173147id_/http://www.meff.es/docs/calendario.pdf> | Wayback `id_` replay of capture `20100407173147`, retrieved 2026-10-11 UTC | T1 | `b30829193bd4c494481f2f4d70dde72946d021cb6486bafb4912549fa2da19e1` |
| `MEFF-PAGE-2010` | 2010 floor grid | <https://web.archive.org/web/20100618194342id_/http://www.meff.es/aspx/Comun/Pagina.aspx?l1=Financiero&f=FuturosIBEX35&id=ing> | Wayback `id_` replay of capture `20100618194342`, retrieved 2026-10-11 UTC | T1 | `f359cefd2ba3ca1d71b6b9f4f88b62f18280437ab2f58016fb3cf27f13005905` |
| `MEFF-GRID-2011` | 2011-01-01..2011-12-31 | <https://web.archive.org/web/20110114201916id_/https://www.meff.es/aspx/Financiero/Calendario.aspx> | Wayback `id_` replay of capture `20110114201916`, retrieved 2026-10-11 UTC | T1 | `09998dc402675fe11f5c6132543c604454b5831d6dc0df56c70a97f753361ba4` |
| `MEFF-GRID-2014` | 2014-01-01..2014-12-31 | <https://web.archive.org/web/20140208082956id_/https://www.meff.es/aspx/Financiero/Calendario.aspx> | Wayback `id_` replay of capture `20140208082956`, retrieved 2026-10-11 UTC | T1 | `d67ae94cc981fdeba21b8b3b3b358ec7e24613c88ce334fada53bef7ecec2194` |
| `MEFF-GRID-2015` | 2015-01-01..2015-12-31 | <https://web.archive.org/web/20150317010050id_/https://www.meff.es/aspx/Financiero/Calendario.aspx> | Wayback `id_` replay of capture `20150317010050`, retrieved 2026-10-11 UTC | T1 | `05303365d083a95f15ab4b815657e82e90e8991599627cdb4c87bddbe1937246` |
| `MEFF-GRID-2016` | 2016-01-01..2016-12-31 | <https://web.archive.org/web/20160314163810id_/https://www.meff.es/aspx/Financiero/Calendario.aspx> | Wayback `id_` replay of capture `20160314163810`, retrieved 2026-10-11 UTC | T1 | `8ee2b08de4bdf3b0ebc01ba4b96a268f142f96c45f57323efbf274226e5b2f91` |
| `BME-NOTA-2016` | 2016-01-01..2016-12-31 | <https://web.archive.org/web/20200923220252id_/https://www.meff.es/ing/Press/Press-Release/20151120/nota_20151120_2/TRADING_CALENDAR_FOR_2016> | Wayback `id_` replay of capture `20200923220252`, retrieved 2026-10-11 UTC | T1 | `9917b37ff2e6095843f51349b4a15fa93ff1a8171052031687cf6f7ce2d59313` |
| `BME-NOTA-2017` | 2017-01-01..2017-12-31 | <https://web.archive.org/web/20200927150131id_/https://www.meff.es/ing/Press/Press-Release/20161117/nota_20161117_2/Trading_calendar_for_2017> | Wayback `id_` replay of capture `20200927150131`, retrieved 2026-10-11 UTC | T1 | `5bbde6ce1cdc7aea89ce2cd887c1dcd7f2573796e8f7e5cbad0f83c94e2c88e5` |
| `MEFF-PR-2017-01-23` | the 2017-01-23 cutover | <https://www.meff.es/ing/Press/Press-Release/20170123/nota_20170123_2/MEFF_extends_trading_in_IBEX_35_and_Mini_IBEX_futures_by_an_hour> | retrieved 2026-10-11 UTC | T1 | `27fcd880d5a7be61bf7c8482d40824dd089481ce41b6a69207a20fe572562ef8` |
| `BME-NOTA-2018` | 2018-01-01..2018-12-31 | <https://web.archive.org/web/20171125220031id_/https://www.meff.es/ing/Press/Press-Release/20171121/nota_20171121_2/2018_Trading_Calendar> | Wayback `id_` replay of capture `20171125220031`, retrieved 2026-10-11 UTC | T1 | `72f0e80f3fe5b82f8eb34fd966eb6af879111f4e207c905d8da07f16fff9bed7` |
| `BME-NOTA-2019` | 2019-01-01..2019-12-31 | <https://web.archive.org/web/20200923211746id_/https://www.meff.es/ing/Press/Press-Release/20181121/nota_20181121_4/Spanish_stock_markets_2019_trading_calendar> | Wayback `id_` replay of capture `20200923211746`, retrieved 2026-10-11 UTC | T1 | `277c6790ee6c0f4c7350085ccca8b97671632addcf1bc9470a6b24de7c223eee` |
| `BME-NOTA-2020` | 2020-01-01..2020-12-31 | <https://web.archive.org/web/20200929082445id_/https://www.meff.es/ing/Press/Press-Release/20191127/nota_20191127_2/Spanish_Securities_Exchanges_2020_Trading_Calendar> | Wayback `id_` replay of capture `20200929082445`, retrieved 2026-10-11 UTC | T1 | `00a251c344c812e3d3bc49035e31cf5ec65cbf07d1abbe2bce3964ea143f9b27` |
| `BME-NOTA-2021` | 2021-01-01..2021-12-31 | <https://web.archive.org/web/20201205151514id_/https://www.meff.es/ing/Press/Press-Release/20201119/nota_20201119_2/Spanish_securities_markets__2021_trading_calendar> | Wayback `id_` replay of capture `20201205151514`, retrieved 2026-10-11 UTC | T1 | `44e647b30cd69ddbd5f22f8a27c3090caa230febace8ea55ac9bab7d1e41ad60` |
| `BME-NOTA-2022` | 2022-01-01..2022-12-31 | <https://web.archive.org/web/20211207062222id_/https://www.meff.es/ing/Press/Press-Release/20211124/nota_20211124_4/Spanish_securities_markets__2022_trading_calendar> | Wayback `id_` replay of capture `20211207062222`, retrieved 2026-10-11 UTC | T1 | `cf49149d707f6e05162ca08bd17fde3b312fe6530a323298b15f62c3c30b596e` |
| `BME-NOTA-2023` | 2023-01-01..2023-12-31 | <https://web.archive.org/web/20221206111133id_/https://www.meff.es/ing/Press/Press-Release/20221124/nota_20221124_2/Spanish_securities_markets__2023_trading_calendar> | Wayback `id_` replay of capture `20221206111133`, retrieved 2026-10-11 UTC | T1 | `f98d71a55e6e29e5ee87b272d131c1f1166d340e9da439231535f2acad115146` |
| `BME-NOTA-2024` | 2024-01-01..2024-12-31 | <https://web.archive.org/web/20240304210050id_/https://www.meff.es/ing/Press/Press-Release/20231123/nota_20231123_4/Spanish_financial_markets__2024_trading_calendar> | Wayback `id_` replay of capture `20240304210050`, retrieved 2026-10-11 UTC | T1 | `a05cd270f18da1bcfef9cc9a0d6aef18e261dcc7dc68b56d9cfc47fe98bf893d` |
| `MEFF-NOTICE-7-2024` | the conditional 2024 slip | <https://www.meff.es/docs/comunicadosMercado/2024/04/Comunicado-2407-Ampliacion-de-horario-de-negociacion-de-los-contratos.pdf> | retrieved 2026-10-11 UTC | T1 | `fba050afcb45e2f837dcbe12ed47f80986af338b70302f6318a772563066a3c9` |
| `MEFF-NOTICE-13-2024` | the conditional 2024 slip | <https://www.meff.es/docs/comunicadosMercado/2024/05/Comunicado-2413-Ampliacion-de-horario-de-negociacion.pdf> | retrieved 2026-10-11 UTC | T1 | `9b79893b8b87480b5fec24ed65e8c1e533dcb7032190feb9173d3588a8dff3f3` |
| `MEFF-NOTICE-14-2024` | 2024-12-24 and 2024-12-31 | <https://www.meff.es/docs/comunicadosMercado/2024/06/Comunicado-2414-Horario-de-las-sesiones-24-y-31-de-diciembre.pdf> | retrieved 2026-10-11 UTC | T1 | `b61ba149bcd8b7b2502fd5b2ec7e7dd879c73872149231c5abf422742fb90d91` |
| `MEFF-NOTICE-16-2025` | the 2025-06-23 cutover | <https://www.meff.es/docs/comunicadosMercado/2025/04/Notice-2516-Extended-time-schedule.pdf> | retrieved 2026-10-11 UTC | T1 | `c9591084d82bfd57c5957f105b797f2f1933c63d13b5bcb738be6dc30ee440bc` |
| `MEFF-NOTICE-49-2025` | 2025-12-24 and 2025-12-31 | <https://www.meff.es/docs/comunicadosMercado/2025/11/Comunicado-2549-Horario-de-las-sesiones-24-y-31-de-diciembre.pdf> | retrieved 2026-10-11 UTC | T1 | `52c040aef322385c2cd5baf822a4814253fbe1bcfa91303c849b93f7812e8742` |
| `BME-NOTA-2025` | 2025-01-01..2025-12-31 | <https://www.bolsasymercados.es/en/media/press-releases/2024/spanish-financial-markets-2025-trading-calendar.html> | retrieved 2026-10-11 UTC | T1 | `81a0507d6ec7d46bd7521b08a4262f9057abab491c7daf985102420ee5731d51` |
| `BME-NOTA-2026` | 2026-01-01..2026-12-31 | <https://www.bolsasymercados.es/en/media/press-releases/2025/spanish-financial-markets-2026-trading-calendar.html> | retrieved 2026-10-11 UTC | T1 | `13ea1b88ca96472cf3335328542af31e820e665363bbb1e684faec7db640f319` |
| `MEFF-HOURS-2017` | the 2017 grid restated | <https://web.archive.org/web/20171123132817id_/https://www.meff.es/ing/Trading-Hours> | Wayback `id_` replay of capture `20171123132817`, retrieved 2026-10-11 UTC | T1 | `977009c9dd81c143b391973a1c82974dc3afd712a093596f17cce4485a6c0efc` |
| `MEFF-HOURS-LIVE` | current grid | <https://www.meff.es/ing/Trading-Hours> | retrieved 2026-10-11 UTC | T1 | `1289aa60e8d785a78ddd368d57c97d5507e30d5cc7432eb56054edbab218b553` |
| `MEFF-IBEX35-LIVE` | current grid | <https://www.meff.es/ing/Financial-Derivatives/Futures-on-IBEX35> | retrieved 2026-10-11 UTC | T1 | `a84f9bd0be545d1882760c1f030c691f75b13797fdce9dfc0789a794511bbef0` |
| `MEFF-CALENDAR-LIVE` | the 14:00 eve note | <https://www.meff.es/ing/Trading-Calendar> | retrieved 2026-10-11 UTC | T1 | `b9c2a07a58d49ad87b03d6eba0a8311816e42a705974dbedbdc5d8823a39500f` |

## Sources

- <https://www.meff.es/ing/Trading-Hours> — the operator's current hours table; the monitoring entry point for the family grid.
- <https://www.meff.es/ing/Financial-Derivatives/Futures-on-IBEX35> — the IBEX 35 futures product page.
- <https://www.meff.es/ing/Trading-Calendar> — the live trading calendar with the 24/31 December 14:00 note.
- <https://www.meff.es/docs/comunicadosMercado/> — the MEFF market-notice archive by year and month (notices 7/2024, 13/2024, 14/2024, 16/2025, 49/2025 cited above).
- <https://www.meff.es/ing/Press/Press-Releases> — the operator press-release index (the 2017-01-23 extension release and the yearly BME trading-calendar releases).
- <https://www.bolsasymercados.es/en/media/press-releases> — BME's press-room index, where each November release names the next year's closure set for all BME markets including MEFF.

## Gaps and residual risks

- **Undated 2010→2017 changeover.** Served as the sourced intersection
  (09:00–17:35, auction 08:30–09:00) across the whole span; the disputed
  remainder — any earlier open or later close the interim states may have
  printed — is withheld. Closing condition: a dated MEFF Circular C-EX-DF
  artifact or a desk answer (searched negative 2026-10-11 UTC, CDX sweep of
  3,958 meff.es URLs for 2010–2016).
- **2012–2013 holiday grids.** Inside no audited window: the aspx calendar
  page's Wayback captures skip those years and the BME release series begins
  with 2016. Session questions answer from the sourced normal week; the
  holiday classification refuses typed. Closing condition: the Circular
  C-EX-DF series or a desk answer.
- **Notice 10/2025** is named in notice 16/2025's related-notices chain but
  was found neither live nor archived (recorded 2026-10-11 UTC). It is a
  member-test-environment notice of the same 22:00 schedule per its siblings
  12/2025 and 15/2025 and keys no row. Closing condition: the MEFF circular
  archive or desk.
- **2027** is unpublished: the BME release under the predictable slug 404s
  (checked 2026-10-11 UTC; prior years' editions appeared each November). The
  audited window ends at 2026-12-31 and no 2027 date is asserted normal.
- The 2010 calendario PDF is a wall calendar read from its struck cells; the
  24 December 2010 cell carries no strike (the diagonal crossing that region
  is the 25/26 weekend strike), so the day traded. The 2026-10-11 hunter
  report and this file's 400-dpi re-read agree.
