<!-- SPDX-License-Identifier: MIT-0 -->

# `tse` — evidence

- **Kind:** `Exchange`
- **Owner module:** [`tse.rs`](../../src/calendar/schedules/equities/apac/tse.rs)
- **Source sets:** [`APAC-JPX`](../schedules/sources.md#apac-jpx)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

TSE venue union across arrowhead and ToSTNeT is 08:00–18:00 as of the 2026-08-24 review. Primary pre-scope evidence establishes the 08:00 tail at the January-2010 floor (Working Paper No.3) and through the post-2011 era (the November 2020 Investigation Report states order acceptance began as normal at 08:00); the 2011 phase change and exact 2024-11-05 ToSTNeT close extension are date-aware.

## Revision rows

- 2011-11-21 — T1 — JPX trading-hours transition table — arrowhead morning session extended from 11:00 to 11:30.
- 2024-11-05 — T1 — JPX news release 20241103-01 — upgraded arrowhead and ToSTNeT go live: the closing call runs to 15:30 with continuous matching to 15:25, and ToSTNeT single-stock/basket trading extends from 17:30 to 18:00.

## Holidays

**Coverage:** 2010-01-01..2027-12-31 (inclusive trade dates; T1 throughout)

One page keys the block: the operator's own holiday calendar, which prints the
current and next year. The 2025-2027 tables were keyed when the venue turned
served (2026-09-28 UTC); the 2010-2024 tables were keyed on 2026-09-29 UTC from
the same channel's archived editions, one Wayback capture per calendar year,
each saved in the research store. The editions change shape over the window:
the 2010-2014 tables are TSE's own page `tse.or.jp/english/about/calendar.html`
("Update : Dec. 30, 2009" through "Update : Jan. 06, 2014"), whose scope
sentence is `Tokyo Stock Exchange is open five days a week from Monday to
Friday. The market will be closed on the following national and observed
holidays, and the market holidays of Jan. 2, 3, and Dec. 31`; the 2015-2017
editions are JPX's `english/corporate/calendar/` page, whose scope sentence is
`Exchange is closed on every Saturday, Sunday and the following holidays:`;
from the 2018 edition on the page is today's
`english/corporate/about-jpx/calendar/`, whose scope sentence is `JPX markets
are closed on Saturdays, Sundays, national holidays, and on the dates indicated
below`.

The rows are exactly the printed dates that fall on a weekday. A printed
holiday landing on a Saturday or Sunday removes no session beyond the normal
week, so it ships no row — every edition from 2018 on prints the weekday beside
each date and the weekend prints are visible in the artifact (2020-02-23
(Sun.), 2018-05-05 (Sat.), 2022-01-01 (Sat.), 2023-12-31 (Sat.) and so on);
the earlier editions print no weekday and the civil calendar supplies it, the
same derivation the 2025-2027 tables use. The pages' own observance rule —
`National holidays that fall on a Sunday are observed on the following Monday.`
(2023/2024 editions; the earlier `Holiday` rows are the same observance) — is
why the `Holiday` and `... observed` rows below land on Mondays. Two weekday
rows ship from prose rather than the table: the 2010-2014 scope sentence states
`the market holidays of Jan. 2, 3, and Dec. 31` outright, and the 2010 and 2011
tables omit the weekday members of that trio (2010-12-31, a Friday; 2011-01-03,
a Monday), so both ship from the sentence that states them. From the 2012
edition the table prints the trio itself.

No edition in the window states a holiday-time early close, late open or
shortened cash-market session on any printed date, and every row below is a
full closure.

**Edition lineages.** Each year's rows are keyed to that year's own edition and
cross-checked against the prior edition's next-year table; the cross-checks
agree date for date except where the operator itself revised the projection:

- `JPX-HOL-2019`: the 2018 edition's 2019 table still prints Dec. 23
  (Emperor's Birthday) and omits the imperial-transition holidays; the 2019
  edition ("Update : Jan. 01, 2019") prints Apr. 30 `Abdication Day`, May 1
  `Accession Day`, May 2 `National Holiday` and Oct. 22 `Enthronement Ceremony
  Day` and omits Dec. 23. The later edition is the corrected statement and keys
  the 2019 rows.
- `JPX-HOL-2021`: the 2020 edition's 2021 table still carries the ordinary July
  and August grid; the 2021 edition ("Update : Jan. 07, 2021") prints the
  Tokyo-Olympics arrangement (Jul. 22 `Marine Day`, Jul. 23 `Sports Day`, Aug. 9
  `Mountain Day (Aug. 8) observed`) and no October Sports Day. The later
  edition keys the 2021 rows.
- `JPX-HOL-2020`: the 2019 edition's 2020 table labels May 5 `Accession Day`;
  the 2020 edition ("Update : Jan. 06, 2020") labels the same Tuesday
  `Children's Day`. Both state the closure; the 2020 edition's label keys the
  row.
- `JPX-HOL-2017`: the 2016 edition's 2017 table labels Jan. 2 `Exchange
  Holiday`; the 2017 edition labels the same Monday `Holiday` (New Year's Day,
  Jan. 1, being the Sunday it observes). Both state the closure; the 2017
  edition's label keys the row.
- `JPX-HOL-2023`: the 2023 edition's own 2023 table carries a broken-markup May
  3 cell (`Constitution Memorial Day/td>`); the same artifact prints May 4
  (Thu.) `Greenery Day` as its own row and the 2024 edition's 2023 table prints
  the pair cleanly. The 2023 rows for May 3 and May 4 rest on both artifacts.

### 2010

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2010-01-01 | closed | `Jan. 1 New Year's Day` | `TSE-CAL-2010` | T1 | Printed date 2010-01-01, a Friday |
| 2010-01-11 | closed | `Jan. 11 Coming of Age Day` | `TSE-CAL-2010` | T1 | Printed date 2010-01-11, a Monday |
| 2010-02-11 | closed | `Feb. 11 National Foundation Day` | `TSE-CAL-2010` | T1 | Printed date 2010-02-11, a Thursday |
| 2010-03-22 | closed | `Mar. 22 Holiday` | `TSE-CAL-2010` | T1 | Printed date 2010-03-22, a Monday; the page prints the date as a `Holiday` without naming the holiday |
| 2010-04-29 | closed | `Apr. 29 Showa Day` | `TSE-CAL-2010` | T1 | Printed date 2010-04-29, a Thursday |
| 2010-05-03 | closed | `May 3 Constitution Memorial Day` | `TSE-CAL-2010` | T1 | Printed date 2010-05-03, a Monday |
| 2010-05-04 | closed | `May 4 Greenery Day` | `TSE-CAL-2010` | T1 | Printed date 2010-05-04, a Tuesday |
| 2010-05-05 | closed | `May 5 Children's Day` | `TSE-CAL-2010` | T1 | Printed date 2010-05-05, a Wednesday |
| 2010-07-19 | closed | `Jul. 19 Marine Day` | `TSE-CAL-2010` | T1 | Printed date 2010-07-19, a Monday |
| 2010-09-20 | closed | `Sep. 20 Respect for the Aged Day` | `TSE-CAL-2010` | T1 | Printed date 2010-09-20, a Monday |
| 2010-09-23 | closed | `Sep. 23 Autumnal equinox` | `TSE-CAL-2010` | T1 | Printed date 2010-09-23, a Thursday |
| 2010-10-11 | closed | `Oct. 11 Health and Sports Day` | `TSE-CAL-2010` | T1 | Printed date 2010-10-11, a Monday |
| 2010-11-03 | closed | `Nov. 3 Culture Day` | `TSE-CAL-2010` | T1 | Printed date 2010-11-03, a Wednesday |
| 2010-11-23 | closed | `Nov. 23 Labor Thanksgiving Day` | `TSE-CAL-2010` | T1 | Printed date 2010-11-23, a Tuesday |
| 2010-12-23 | closed | `Dec. 23 Emperor's Birthday` | `TSE-CAL-2010` | T1 | Printed date 2010-12-23, a Thursday |
| 2010-12-31 | closed | `the market holidays of Jan. 2, 3, and Dec. 31` — the scope sentence; the Dec. 31 weekday | `TSE-CAL-2010` | T1 | Printed date 2010-12-31, a Friday; the TSE-era scope sentence states Jan. 2, Jan. 3 and Dec. 31 as market holidays and the year's own table omits this weekday date |

### 2011

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2011-01-03 | closed | `the market holidays of Jan. 2, 3, and Dec. 31` — the scope sentence; the Jan. 3 weekday | `TSE-CAL-2011` | T1 | Printed date 2011-01-03, a Monday; the TSE-era scope sentence states Jan. 2, Jan. 3 and Dec. 31 as market holidays and the year's own table omits this weekday date |
| 2011-01-10 | closed | `Jan. 10 Coming of Age Day` | `TSE-CAL-2011` | T1 | Printed date 2011-01-10, a Monday |
| 2011-02-11 | closed | `Feb. 11 National Foundation Day` | `TSE-CAL-2011` | T1 | Printed date 2011-02-11, a Friday |
| 2011-03-21 | closed | `Mar. 21 Vernal Equinox` | `TSE-CAL-2011` | T1 | Printed date 2011-03-21, a Monday |
| 2011-04-29 | closed | `Apr. 29 Showa Day` | `TSE-CAL-2011` | T1 | Printed date 2011-04-29, a Friday |
| 2011-05-03 | closed | `May 3 Constitution Memorial Day` | `TSE-CAL-2011` | T1 | Printed date 2011-05-03, a Tuesday |
| 2011-05-04 | closed | `May 4 Greenery Day` | `TSE-CAL-2011` | T1 | Printed date 2011-05-04, a Wednesday |
| 2011-05-05 | closed | `May 5 Children's Day` | `TSE-CAL-2011` | T1 | Printed date 2011-05-05, a Thursday |
| 2011-07-18 | closed | `Jul. 18 Marine Day` | `TSE-CAL-2011` | T1 | Printed date 2011-07-18, a Monday |
| 2011-09-19 | closed | `Sep. 19 Respect for the Aged Day` | `TSE-CAL-2011` | T1 | Printed date 2011-09-19, a Monday |
| 2011-09-23 | closed | `Sep. 23 Autumnal equinox` | `TSE-CAL-2011` | T1 | Printed date 2011-09-23, a Friday |
| 2011-10-10 | closed | `Oct. 10 Health and Sports Day` | `TSE-CAL-2011` | T1 | Printed date 2011-10-10, a Monday |
| 2011-11-03 | closed | `Nov. 3 Culture Day` | `TSE-CAL-2011` | T1 | Printed date 2011-11-03, a Thursday |
| 2011-11-23 | closed | `Nov. 23 Labor Thanksgiving Day` | `TSE-CAL-2011` | T1 | Printed date 2011-11-23, a Wednesday |
| 2011-12-23 | closed | `Dec. 23 Emperor's Birthday` | `TSE-CAL-2011` | T1 | Printed date 2011-12-23, a Friday |

### 2012

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2012-01-02 | closed | `Jan. 2 Holiday` | `TSE-CAL-2012` | T1 | Printed date 2012-01-02, a Monday; the page prints the date as a `Holiday` without naming the holiday |
| 2012-01-03 | closed | `Jan. 3 Exchange Holiday` | `TSE-CAL-2012` | T1 | Printed date 2012-01-03, a Tuesday |
| 2012-01-09 | closed | `Jan. 9 Coming of Age Day` | `TSE-CAL-2012` | T1 | Printed date 2012-01-09, a Monday |
| 2012-03-20 | closed | `Mar. 20 Vernal Equinox` | `TSE-CAL-2012` | T1 | Printed date 2012-03-20, a Tuesday |
| 2012-04-30 | closed | `Apr. 30 Holiday` | `TSE-CAL-2012` | T1 | Printed date 2012-04-30, a Monday; the page prints the date as a `Holiday` without naming the holiday |
| 2012-05-03 | closed | `May 3 Constitution Memorial Day` | `TSE-CAL-2012` | T1 | Printed date 2012-05-03, a Thursday |
| 2012-05-04 | closed | `May 4 Greenery Day` | `TSE-CAL-2012` | T1 | Printed date 2012-05-04, a Friday |
| 2012-07-16 | closed | `Jul. 16 Marine Day` | `TSE-CAL-2012` | T1 | Printed date 2012-07-16, a Monday |
| 2012-09-17 | closed | `Sep. 17 Respect for the Aged Day` | `TSE-CAL-2012` | T1 | Printed date 2012-09-17, a Monday |
| 2012-10-08 | closed | `Oct. 8 Health and Sports Day` | `TSE-CAL-2012` | T1 | Printed date 2012-10-08, a Monday |
| 2012-11-23 | closed | `Nov. 23 Labor Thanksgiving Day` | `TSE-CAL-2012` | T1 | Printed date 2012-11-23, a Friday |
| 2012-12-24 | closed | `Dec. 24 Holiday` | `TSE-CAL-2012` | T1 | Printed date 2012-12-24, a Monday; the page prints the date as a `Holiday` without naming the holiday |
| 2012-12-31 | closed | `Dec. 31 Exchange Holiday` | `TSE-CAL-2012` | T1 | Printed date 2012-12-31, a Monday |

### 2013

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2013-01-01 | closed | `Jan. 1 New Year's Day` | `TSE-CAL-2013` | T1 | Printed date 2013-01-01, a Tuesday |
| 2013-01-02 | closed | `Jan. 2, 3 Exchange Holiday` | `TSE-CAL-2013` | T1 | Printed date 2013-01-02, a Wednesday |
| 2013-01-03 | closed | `Jan. 3 Exchange Holiday` | `TSE-CAL-2013` | T1 | Printed date 2013-01-03, a Thursday |
| 2013-01-14 | closed | `Jan. 14 Coming of Age Day` | `TSE-CAL-2013` | T1 | Printed date 2013-01-14, a Monday |
| 2013-02-11 | closed | `Feb. 11 National Foundation Day` | `TSE-CAL-2013` | T1 | Printed date 2013-02-11, a Monday |
| 2013-03-20 | closed | `Mar. 20 Vernal Equinox` | `TSE-CAL-2013` | T1 | Printed date 2013-03-20, a Wednesday |
| 2013-04-29 | closed | `Apr. 29 Showa Day` | `TSE-CAL-2013` | T1 | Printed date 2013-04-29, a Monday |
| 2013-05-03 | closed | `May 3 Constitution Memorial Day` | `TSE-CAL-2013` | T1 | Printed date 2013-05-03, a Friday |
| 2013-05-06 | closed | `May 6 Holiday` | `TSE-CAL-2013` | T1 | Printed date 2013-05-06, a Monday; the page prints the date as a `Holiday` without naming the holiday |
| 2013-07-15 | closed | `Jul. 15 Marine Day` | `TSE-CAL-2013` | T1 | Printed date 2013-07-15, a Monday |
| 2013-09-16 | closed | `Sep. 16 Respect for the Aged Day` | `TSE-CAL-2013` | T1 | Printed date 2013-09-16, a Monday |
| 2013-09-23 | closed | `Sep. 23 Autumnal equinox` | `TSE-CAL-2013` | T1 | Printed date 2013-09-23, a Monday |
| 2013-10-14 | closed | `Oct. 14 Health and Sports Day` | `TSE-CAL-2013` | T1 | Printed date 2013-10-14, a Monday |
| 2013-11-04 | closed | `Nov. 4 Holiday` | `TSE-CAL-2013` | T1 | Printed date 2013-11-04, a Monday; the page prints the date as a `Holiday` without naming the holiday |
| 2013-12-23 | closed | `Dec. 23 Emperor's Birthday` | `TSE-CAL-2013` | T1 | Printed date 2013-12-23, a Monday |
| 2013-12-31 | closed | `Dec. 31 Exchange Holiday` | `TSE-CAL-2013` | T1 | Printed date 2013-12-31, a Tuesday |

### 2014

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2014-01-01 | closed | `Jan. 1 New Year's Day` | `TSE-CAL-2014` | T1 | Printed date 2014-01-01, a Wednesday |
| 2014-01-02 | closed | `Jan. 2, 3 Exchange Holiday` | `TSE-CAL-2014` | T1 | Printed date 2014-01-02, a Thursday |
| 2014-01-03 | closed | `Jan. 3 Exchange Holiday` | `TSE-CAL-2014` | T1 | Printed date 2014-01-03, a Friday |
| 2014-01-13 | closed | `Jan. 13 Coming of Age Day` | `TSE-CAL-2014` | T1 | Printed date 2014-01-13, a Monday |
| 2014-02-11 | closed | `Feb. 11 National Foundation Day` | `TSE-CAL-2014` | T1 | Printed date 2014-02-11, a Tuesday |
| 2014-03-21 | closed | `Mar. 21 Vernal Equinox` | `TSE-CAL-2014` | T1 | Printed date 2014-03-21, a Friday |
| 2014-04-29 | closed | `Apr. 29 Showa Day` | `TSE-CAL-2014` | T1 | Printed date 2014-04-29, a Tuesday |
| 2014-05-05 | closed | `May 5 Children's Day` | `TSE-CAL-2014` | T1 | Printed date 2014-05-05, a Monday |
| 2014-05-06 | closed | `May 6 Holiday` | `TSE-CAL-2014` | T1 | Printed date 2014-05-06, a Tuesday; the page prints the date as a `Holiday` without naming the holiday |
| 2014-07-21 | closed | `Jul. 21 Marine Day` | `TSE-CAL-2014` | T1 | Printed date 2014-07-21, a Monday |
| 2014-09-15 | closed | `Sep. 15 Respect for the Aged Day` | `TSE-CAL-2014` | T1 | Printed date 2014-09-15, a Monday |
| 2014-09-23 | closed | `Sep. 23 Autumnal equinox` | `TSE-CAL-2014` | T1 | Printed date 2014-09-23, a Tuesday |
| 2014-10-13 | closed | `Oct. 13 Health and Sports Day` | `TSE-CAL-2014` | T1 | Printed date 2014-10-13, a Monday |
| 2014-11-03 | closed | `Nov. 3 Culture Day` | `TSE-CAL-2014` | T1 | Printed date 2014-11-03, a Monday |
| 2014-11-24 | closed | `Nov. 24 Holiday` | `TSE-CAL-2014` | T1 | Printed date 2014-11-24, a Monday; the page prints the date as a `Holiday` without naming the holiday |
| 2014-12-23 | closed | `Dec. 23 Emperor's Birthday` | `TSE-CAL-2014` | T1 | Printed date 2014-12-23, a Tuesday |
| 2014-12-31 | closed | `Dec. 31 Exchange Holiday` | `TSE-CAL-2014` | T1 | Printed date 2014-12-31, a Wednesday |

### 2015

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2015-01-01 | closed | `Jan. 1 New Year's Day` | `JPX-HOL-2015` | T1 | Printed date 2015-01-01, a Thursday |
| 2015-01-02 | closed | `Jan. 2 Exchange Holiday` | `JPX-HOL-2015` | T1 | Printed date 2015-01-02, a Friday |
| 2015-01-12 | closed | `Jan. 12 Coming of Age Day` | `JPX-HOL-2015` | T1 | Printed date 2015-01-12, a Monday |
| 2015-02-11 | closed | `Feb. 11 National Foundation Day` | `JPX-HOL-2015` | T1 | Printed date 2015-02-11, a Wednesday |
| 2015-04-29 | closed | `Apr. 29 Showa Day` | `JPX-HOL-2015` | T1 | Printed date 2015-04-29, a Wednesday |
| 2015-05-04 | closed | `May 4 Greenery Day` | `JPX-HOL-2015` | T1 | Printed date 2015-05-04, a Monday |
| 2015-05-05 | closed | `May 5 Children's Day` | `JPX-HOL-2015` | T1 | Printed date 2015-05-05, a Tuesday |
| 2015-05-06 | closed | `May 6 Holiday` | `JPX-HOL-2015` | T1 | Printed date 2015-05-06, a Wednesday; the page prints the date as a `Holiday` without naming the holiday |
| 2015-07-20 | closed | `Jul. 20 Marine Day` | `JPX-HOL-2015` | T1 | Printed date 2015-07-20, a Monday |
| 2015-09-21 | closed | `Sep. 21 Respect for the Aged Day` | `JPX-HOL-2015` | T1 | Printed date 2015-09-21, a Monday |
| 2015-09-22 | closed | `Sep. 22 Holiday` | `JPX-HOL-2015` | T1 | Printed date 2015-09-22, a Tuesday; the page prints the date as a `Holiday` without naming the holiday |
| 2015-09-23 | closed | `Sep. 23 Autumnal equinox` | `JPX-HOL-2015` | T1 | Printed date 2015-09-23, a Wednesday |
| 2015-10-12 | closed | `Oct. 12 Health and Sports Day` | `JPX-HOL-2015` | T1 | Printed date 2015-10-12, a Monday |
| 2015-11-03 | closed | `Nov. 3 Culture Day` | `JPX-HOL-2015` | T1 | Printed date 2015-11-03, a Tuesday |
| 2015-11-23 | closed | `Nov. 23 Labor Thanksgiving Day` | `JPX-HOL-2015` | T1 | Printed date 2015-11-23, a Monday |
| 2015-12-23 | closed | `Dec. 23 Emperor's Birthday` | `JPX-HOL-2015` | T1 | Printed date 2015-12-23, a Wednesday |
| 2015-12-31 | closed | `Dec. 31 Exchange Holiday` | `JPX-HOL-2015` | T1 | Printed date 2015-12-31, a Thursday |

### 2016

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2016-01-01 | closed | `Jan. 1 New Year's Day` | `JPX-HOL-2016` | T1 | Printed date 2016-01-01, a Friday |
| 2016-01-11 | closed | `Jan. 11 Coming of Age Day` | `JPX-HOL-2016` | T1 | Printed date 2016-01-11, a Monday |
| 2016-02-11 | closed | `Feb. 11 National Foundation Day` | `JPX-HOL-2016` | T1 | Printed date 2016-02-11, a Thursday |
| 2016-03-21 | closed | `Mar. 21 Holiday` | `JPX-HOL-2016` | T1 | Printed date 2016-03-21, a Monday; the page prints the date as a `Holiday` without naming the holiday |
| 2016-04-29 | closed | `Apr. 29 Showa Day` | `JPX-HOL-2016` | T1 | Printed date 2016-04-29, a Friday |
| 2016-05-03 | closed | `May 3 Constitution Memorial Day` | `JPX-HOL-2016` | T1 | Printed date 2016-05-03, a Tuesday |
| 2016-05-04 | closed | `May 4 Greenery Day` | `JPX-HOL-2016` | T1 | Printed date 2016-05-04, a Wednesday |
| 2016-05-05 | closed | `May 5 Children's Day` | `JPX-HOL-2016` | T1 | Printed date 2016-05-05, a Thursday |
| 2016-07-18 | closed | `Jul. 18 Marine Day` | `JPX-HOL-2016` | T1 | Printed date 2016-07-18, a Monday |
| 2016-08-11 | closed | `Aug. 11 Mountain Day` | `JPX-HOL-2016` | T1 | Printed date 2016-08-11, a Thursday |
| 2016-09-19 | closed | `Sep. 19 Respect for the Aged Day` | `JPX-HOL-2016` | T1 | Printed date 2016-09-19, a Monday |
| 2016-09-22 | closed | `Sep. 22 Autumnal equinox` | `JPX-HOL-2016` | T1 | Printed date 2016-09-22, a Thursday |
| 2016-10-10 | closed | `Oct. 10 Health and Sports Day` | `JPX-HOL-2016` | T1 | Printed date 2016-10-10, a Monday |
| 2016-11-03 | closed | `Nov. 3 Culture Day` | `JPX-HOL-2016` | T1 | Printed date 2016-11-03, a Thursday |
| 2016-11-23 | closed | `Nov. 23 Labor Thanksgiving Day` | `JPX-HOL-2016` | T1 | Printed date 2016-11-23, a Wednesday |
| 2016-12-23 | closed | `Dec. 23 Emperor's Birthday` | `JPX-HOL-2016` | T1 | Printed date 2016-12-23, a Friday |

### 2017

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2017-01-02 | closed | `Jan. 2 Holiday` | `JPX-HOL-2017` | T1 | Printed date 2017-01-02, a Monday; the page prints the date as a `Holiday` without naming the holiday |
| 2017-01-03 | closed | `Jan. 3 Exchange Holiday` | `JPX-HOL-2017` | T1 | Printed date 2017-01-03, a Tuesday |
| 2017-01-09 | closed | `Jan. 9 Coming of Age Day` | `JPX-HOL-2017` | T1 | Printed date 2017-01-09, a Monday |
| 2017-03-20 | closed | `Mar. 20 Vernal Equinox` | `JPX-HOL-2017` | T1 | Printed date 2017-03-20, a Monday |
| 2017-05-03 | closed | `May 3 Constitution Memorial Day` | `JPX-HOL-2017` | T1 | Printed date 2017-05-03, a Wednesday |
| 2017-05-04 | closed | `May 4 Greenery Day` | `JPX-HOL-2017` | T1 | Printed date 2017-05-04, a Thursday |
| 2017-05-05 | closed | `May 5 Children's Day` | `JPX-HOL-2017` | T1 | Printed date 2017-05-05, a Friday |
| 2017-07-17 | closed | `Jul. 17 Marine Day` | `JPX-HOL-2017` | T1 | Printed date 2017-07-17, a Monday |
| 2017-08-11 | closed | `Aug. 11 Mountain Day` | `JPX-HOL-2017` | T1 | Printed date 2017-08-11, a Friday |
| 2017-09-18 | closed | `Sep. 18 Respect for the Aged Day` | `JPX-HOL-2017` | T1 | Printed date 2017-09-18, a Monday |
| 2017-10-09 | closed | `Oct. 9 Health and Sports Day` | `JPX-HOL-2017` | T1 | Printed date 2017-10-09, a Monday |
| 2017-11-03 | closed | `Nov. 3 Culture Day` | `JPX-HOL-2017` | T1 | Printed date 2017-11-03, a Friday |
| 2017-11-23 | closed | `Nov. 23 Labor Thanksgiving Day` | `JPX-HOL-2017` | T1 | Printed date 2017-11-23, a Thursday |

### 2018

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2018-01-01 | closed | `Jan. 1 (Mon.) New Year's Day` | `JPX-HOL-2018` | T1 | Printed date 2018-01-01, a Monday |
| 2018-01-02 | closed | `Jan. 2 (Tue.) Market Holiday` | `JPX-HOL-2018` | T1 | Printed date 2018-01-02, a Tuesday |
| 2018-01-03 | closed | `Jan. 3 (Wed.) Market Holiday` | `JPX-HOL-2018` | T1 | Printed date 2018-01-03, a Wednesday |
| 2018-01-08 | closed | `Jan. 8 (Mon.) Coming of Age Day` | `JPX-HOL-2018` | T1 | Printed date 2018-01-08, a Monday |
| 2018-02-12 | closed | `Feb. 12 (Mon.) National Foundation Day (Feb. 11) observed` | `JPX-HOL-2018` | T1 | Printed date 2018-02-12, a Monday; event date Feb. 11 is a Sunday and the page's own observance rule (`National holidays that fall on a Sunday are observed on the following Monday.`) is why the printed row lands on the Monday |
| 2018-03-21 | closed | `Mar. 21 (Wed.) Vernal Equinox` | `JPX-HOL-2018` | T1 | Printed date 2018-03-21, a Wednesday |
| 2018-04-30 | closed | `Apr. 30 (Mon.) Showa Day (Apr. 29) observed` | `JPX-HOL-2018` | T1 | Printed date 2018-04-30, a Monday; event date Apr. 29 is a Sunday and the page's own observance rule (`National holidays that fall on a Sunday are observed on the following Monday.`) is why the printed row lands on the Monday |
| 2018-05-03 | closed | `May 3 (Thu.) Constitution Memorial Day` | `JPX-HOL-2018` | T1 | Printed date 2018-05-03, a Thursday |
| 2018-05-04 | closed | `May 4 (Fri.) Greenery Day` | `JPX-HOL-2018` | T1 | Printed date 2018-05-04, a Friday |
| 2018-07-16 | closed | `Jul. 16 (Mon.) Marine Day` | `JPX-HOL-2018` | T1 | Printed date 2018-07-16, a Monday |
| 2018-09-17 | closed | `Sep. 17 (Mon.) Respect for the Aged Day` | `JPX-HOL-2018` | T1 | Printed date 2018-09-17, a Monday |
| 2018-09-24 | closed | `Sep. 24 (Mon.) Autumnal Equinox (Sep. 23) observed` | `JPX-HOL-2018` | T1 | Printed date 2018-09-24, a Monday; event date Sep. 23 is a Sunday and the page's own observance rule (`National holidays that fall on a Sunday are observed on the following Monday.`) is why the printed row lands on the Monday |
| 2018-10-08 | closed | `Oct. 8 (Mon.) Health and Sports Day` | `JPX-HOL-2018` | T1 | Printed date 2018-10-08, a Monday |
| 2018-11-23 | closed | `Nov. 23 (Fri.) Labor Thanksgiving Day` | `JPX-HOL-2018` | T1 | Printed date 2018-11-23, a Friday |
| 2018-12-24 | closed | `Dec. 24 (Mon.) Emperor's Birthday (Dec. 23) observed` | `JPX-HOL-2018` | T1 | Printed date 2018-12-24, a Monday; event date Dec. 23 is a Sunday and the page's own observance rule (`National holidays that fall on a Sunday are observed on the following Monday.`) is why the printed row lands on the Monday |
| 2018-12-31 | closed | `Dec. 31 (Mon.) Market Holiday` | `JPX-HOL-2018` | T1 | Printed date 2018-12-31, a Monday |

### 2019

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2019-01-01 | closed | `Jan. 1 (Tue.) New Year's Day` | `JPX-HOL-2019` | T1 | Printed date 2019-01-01, a Tuesday |
| 2019-01-02 | closed | `Jan. 2 (Wed.) Market Holiday` | `JPX-HOL-2019` | T1 | Printed date 2019-01-02, a Wednesday |
| 2019-01-03 | closed | `Jan. 3 (Thu.) Market Holiday` | `JPX-HOL-2019` | T1 | Printed date 2019-01-03, a Thursday |
| 2019-01-14 | closed | `Jan. 14 (Mon.) Coming of Age Day` | `JPX-HOL-2019` | T1 | Printed date 2019-01-14, a Monday |
| 2019-02-11 | closed | `Feb. 11 (Mon.) National Foundation Day` | `JPX-HOL-2019` | T1 | Printed date 2019-02-11, a Monday |
| 2019-03-21 | closed | `Mar. 21 (Thu.) Vernal Equinox` | `JPX-HOL-2019` | T1 | Printed date 2019-03-21, a Thursday |
| 2019-04-29 | closed | `Apr. 29 (Mon.) Showa Day` | `JPX-HOL-2019` | T1 | Printed date 2019-04-29, a Monday |
| 2019-04-30 | closed | `Apr. 30 (Tue.) Abdication Day` | `JPX-HOL-2019` | T1 | Printed date 2019-04-30, a Tuesday |
| 2019-05-01 | closed | `May 1 (Wed.) Accession Day` | `JPX-HOL-2019` | T1 | Printed date 2019-05-01, a Wednesday |
| 2019-05-02 | closed | `May 2 (Thu.) National Holiday` | `JPX-HOL-2019` | T1 | Printed date 2019-05-02, a Thursday |
| 2019-05-03 | closed | `May 3 (Fri.) Constitution Memorial Day` | `JPX-HOL-2019` | T1 | Printed date 2019-05-03, a Friday |
| 2019-05-06 | closed | `May 6 (Mon.) Children's Day (May 5) observed` | `JPX-HOL-2019` | T1 | Printed date 2019-05-06, a Monday; event date May 5 is a Sunday and the page's own observance rule (`National holidays that fall on a Sunday are observed on the following Monday.`) is why the printed row lands on the Monday |
| 2019-07-15 | closed | `Jul. 15 (Mon.) Marine Day` | `JPX-HOL-2019` | T1 | Printed date 2019-07-15, a Monday |
| 2019-08-12 | closed | `Aug. 12 (Mon.) Mountain Day (Aug. 11) observed` | `JPX-HOL-2019` | T1 | Printed date 2019-08-12, a Monday; event date Aug. 11 is a Sunday and the page's own observance rule (`National holidays that fall on a Sunday are observed on the following Monday.`) is why the printed row lands on the Monday |
| 2019-09-16 | closed | `Sep. 16 (Mon.) Respect for the Aged Day` | `JPX-HOL-2019` | T1 | Printed date 2019-09-16, a Monday |
| 2019-09-23 | closed | `Sep. 23 (Mon.) Autumnal Equinox` | `JPX-HOL-2019` | T1 | Printed date 2019-09-23, a Monday |
| 2019-10-14 | closed | `Oct. 14 (Mon.) Health and Sports Day` | `JPX-HOL-2019` | T1 | Printed date 2019-10-14, a Monday |
| 2019-10-22 | closed | `Oct. 22 (Tue.) Enthronement Ceremony Day` | `JPX-HOL-2019` | T1 | Printed date 2019-10-22, a Tuesday |
| 2019-11-04 | closed | `Nov. 4 (Mon.) Culture Day (Nov. 3) observed` | `JPX-HOL-2019` | T1 | Printed date 2019-11-04, a Monday; event date Nov. 3 is a Sunday and the page's own observance rule (`National holidays that fall on a Sunday are observed on the following Monday.`) is why the printed row lands on the Monday |
| 2019-12-31 | closed | `Dec. 31 (Tue.) Market Holiday` | `JPX-HOL-2019` | T1 | Printed date 2019-12-31, a Tuesday |

### 2020

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2020-01-01 | closed | `Jan. 1 (Wed.) New Year's Day` | `JPX-HOL-2020` | T1 | Printed date 2020-01-01, a Wednesday |
| 2020-01-02 | closed | `Jan. 2 (Thu.) Market Holiday` | `JPX-HOL-2020` | T1 | Printed date 2020-01-02, a Thursday |
| 2020-01-03 | closed | `Jan. 3 (Fri.) Market Holiday` | `JPX-HOL-2020` | T1 | Printed date 2020-01-03, a Friday |
| 2020-01-13 | closed | `Jan. 13 (Mon.) Coming of Age Day` | `JPX-HOL-2020` | T1 | Printed date 2020-01-13, a Monday |
| 2020-02-11 | closed | `Feb. 11 (Tue.) National Foundation Day` | `JPX-HOL-2020` | T1 | Printed date 2020-02-11, a Tuesday |
| 2020-02-24 | closed | `Feb. 24 (Mon.) Emperor's Birthday (Feb. 23) observed` | `JPX-HOL-2020` | T1 | Printed date 2020-02-24, a Monday; event date Feb. 23 is a Sunday and the page's own observance rule (`National holidays that fall on a Sunday are observed on the following Monday.`) is why the printed row lands on the Monday |
| 2020-03-20 | closed | `Mar. 20 (Fri.) Vernal Equinox` | `JPX-HOL-2020` | T1 | Printed date 2020-03-20, a Friday |
| 2020-04-29 | closed | `Apr. 29 (Wed.) Showa Day` | `JPX-HOL-2020` | T1 | Printed date 2020-04-29, a Wednesday |
| 2020-05-04 | closed | `May 4 (Mon.) Greenery Day` | `JPX-HOL-2020` | T1 | Printed date 2020-05-04, a Monday |
| 2020-05-05 | closed | `May 5 (Tue.) Children's Day` | `JPX-HOL-2020` | T1 | Printed date 2020-05-05, a Tuesday |
| 2020-05-06 | closed | `May 6 (Wed.) Constitution Memorial Day (May 3) observed` | `JPX-HOL-2020` | T1 | Printed date 2020-05-06, a Wednesday; event date May 3 is a Sunday and the page's own observance rule (`National holidays that fall on a Sunday are observed on the following Monday.`) is why the printed row lands on the Monday |
| 2020-07-23 | closed | `Jul. 23 (Thu.) Marine Day` | `JPX-HOL-2020` | T1 | Printed date 2020-07-23, a Thursday |
| 2020-07-24 | closed | `Jul. 24 (Fri.) Sports Day` | `JPX-HOL-2020` | T1 | Printed date 2020-07-24, a Friday |
| 2020-08-10 | closed | `Aug. 10 (Mon.) Mountain Day` | `JPX-HOL-2020` | T1 | Printed date 2020-08-10, a Monday |
| 2020-09-21 | closed | `Sep. 21 (Mon.) Respect for the Aged Day` | `JPX-HOL-2020` | T1 | Printed date 2020-09-21, a Monday |
| 2020-09-22 | closed | `Sep. 22 (Tue.) Autumnal Equinox` | `JPX-HOL-2020` | T1 | Printed date 2020-09-22, a Tuesday |
| 2020-11-03 | closed | `Nov. 3 (The.) Culture Day` | `JPX-HOL-2020` | T1 | Printed date 2020-11-03, a Tuesday |
| 2020-11-23 | closed | `Nov. 23 (Mon.) Labor Thanksgiving Day` | `JPX-HOL-2020` | T1 | Printed date 2020-11-23, a Monday |
| 2020-12-31 | closed | `Dec. 31 (Thu.) Market Holiday` | `JPX-HOL-2020` | T1 | Printed date 2020-12-31, a Thursday |

### 2021

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2021-01-01 | closed | `Jan. 1 (Fri.) New Year's Day` | `JPX-HOL-2021` | T1 | Printed date 2021-01-01, a Friday |
| 2021-01-11 | closed | `Jan. 11 (Mon.) Coming of Age Day` | `JPX-HOL-2021` | T1 | Printed date 2021-01-11, a Monday |
| 2021-02-11 | closed | `Feb. 11 (Thu.) National Foundation Day` | `JPX-HOL-2021` | T1 | Printed date 2021-02-11, a Thursday |
| 2021-02-23 | closed | `Feb. 23 (Tue.) Emperor's Birthday` | `JPX-HOL-2021` | T1 | Printed date 2021-02-23, a Tuesday |
| 2021-04-29 | closed | `Apr. 29 (Thu.) Showa Day` | `JPX-HOL-2021` | T1 | Printed date 2021-04-29, a Thursday |
| 2021-05-03 | closed | `May 3 (Mon.) Constitution Memorial Day` | `JPX-HOL-2021` | T1 | Printed date 2021-05-03, a Monday |
| 2021-05-04 | closed | `May 4 (Tue.) Greenery Day` | `JPX-HOL-2021` | T1 | Printed date 2021-05-04, a Tuesday |
| 2021-05-05 | closed | `May 5 (Wed.) Children's Day` | `JPX-HOL-2021` | T1 | Printed date 2021-05-05, a Wednesday |
| 2021-07-22 | closed | `Jul. 22 (Thu.) Marine Day` | `JPX-HOL-2021` | T1 | Printed date 2021-07-22, a Thursday |
| 2021-07-23 | closed | `Jul. 23 (Fri.) Sports Day` | `JPX-HOL-2021` | T1 | Printed date 2021-07-23, a Friday |
| 2021-08-09 | closed | `Aug. 9 (Mon.) Mountain Day (Aug. 8) observed` | `JPX-HOL-2021` | T1 | Printed date 2021-08-09, a Monday; event date Aug. 8 is a Sunday and the page's own observance rule (`National holidays that fall on a Sunday are observed on the following Monday.`) is why the printed row lands on the Monday |
| 2021-09-20 | closed | `Sep. 20 (Mon.) Respect for the Aged Day` | `JPX-HOL-2021` | T1 | Printed date 2021-09-20, a Monday |
| 2021-09-23 | closed | `Sep. 23 (Thu.) Autumnal Equinox` | `JPX-HOL-2021` | T1 | Printed date 2021-09-23, a Thursday |
| 2021-11-03 | closed | `Nov. 3 (Wed.) Culture Day` | `JPX-HOL-2021` | T1 | Printed date 2021-11-03, a Wednesday |
| 2021-11-23 | closed | `Nov. 23 (Tue.) Labor Thanksgiving Day` | `JPX-HOL-2021` | T1 | Printed date 2021-11-23, a Tuesday |
| 2021-12-31 | closed | `Dec. 31 (Fri.) Market Holiday` | `JPX-HOL-2021` | T1 | Printed date 2021-12-31, a Friday |

### 2022

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2022-01-03 | closed | `Jan. 3 (Mon.) Market Holiday` | `JPX-HOL-2022` | T1 | Printed date 2022-01-03, a Monday |
| 2022-01-10 | closed | `Jan. 10 (Mon.) Coming of Age Day` | `JPX-HOL-2022` | T1 | Printed date 2022-01-10, a Monday |
| 2022-02-11 | closed | `Feb. 11 (Fri.) National Foundation Day` | `JPX-HOL-2022` | T1 | Printed date 2022-02-11, a Friday |
| 2022-02-23 | closed | `Feb. 23 (Wed.) Emperor's Birthday` | `JPX-HOL-2022` | T1 | Printed date 2022-02-23, a Wednesday |
| 2022-03-21 | closed | `Mar. 21 (Mon.) Vernal Equinox` | `JPX-HOL-2022` | T1 | Printed date 2022-03-21, a Monday |
| 2022-04-29 | closed | `Apr. 29 (Fri.) Showa Day` | `JPX-HOL-2022` | T1 | Printed date 2022-04-29, a Friday |
| 2022-05-03 | closed | `May 3 (Tue.) Constitution Memorial Day` | `JPX-HOL-2022` | T1 | Printed date 2022-05-03, a Tuesday |
| 2022-05-04 | closed | `May 4 (Wed.) Greenery Day` | `JPX-HOL-2022` | T1 | Printed date 2022-05-04, a Wednesday |
| 2022-05-05 | closed | `May 5 (Thu.) Children's Day` | `JPX-HOL-2022` | T1 | Printed date 2022-05-05, a Thursday |
| 2022-07-18 | closed | `Jul. 18 (Mon.) Marine Day` | `JPX-HOL-2022` | T1 | Printed date 2022-07-18, a Monday |
| 2022-08-11 | closed | `Aug. 11 (Thu.) Mountain Day` | `JPX-HOL-2022` | T1 | Printed date 2022-08-11, a Thursday |
| 2022-09-19 | closed | `Sep. 19 (Mon.) Respect for the Aged Day` | `JPX-HOL-2022` | T1 | Printed date 2022-09-19, a Monday |
| 2022-09-23 | closed | `Sep. 23 (Fri.) Autumnal Equinox` | `JPX-HOL-2022` | T1 | Printed date 2022-09-23, a Friday |
| 2022-10-10 | closed | `Oct. 10 (Mon.) Sports Day` | `JPX-HOL-2022` | T1 | Printed date 2022-10-10, a Monday |
| 2022-11-03 | closed | `Nov. 3 (Thu.) Culture Day` | `JPX-HOL-2022` | T1 | Printed date 2022-11-03, a Thursday |
| 2022-11-23 | closed | `Nov. 23 (Wed.) Labor Thanksgiving Day` | `JPX-HOL-2022` | T1 | Printed date 2022-11-23, a Wednesday |

### 2023

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2023-01-02 | closed | `Jan. 2 (Mon.) New Year's Day (Jan. 1) observed` | `JPX-HOL-2023` | T1 | Printed date 2023-01-02, a Monday; event date Jan. 1 is a Sunday and the page's own observance rule (`National holidays that fall on a Sunday are observed on the following Monday.`) is why the printed row lands on the Monday |
| 2023-01-03 | closed | `Jan. 3 (Tue.) Market Holiday` | `JPX-HOL-2023` | T1 | Printed date 2023-01-03, a Tuesday |
| 2023-01-09 | closed | `Jan. 9 (Mon.) Coming of Age Day` | `JPX-HOL-2023` | T1 | Printed date 2023-01-09, a Monday |
| 2023-02-23 | closed | `Feb. 23 (Thu.) Emperor's Birthday` | `JPX-HOL-2023` | T1 | Printed date 2023-02-23, a Thursday |
| 2023-03-21 | closed | `Mar. 21 (Tue.) Vernal Equinox` | `JPX-HOL-2023` | T1 | Printed date 2023-03-21, a Tuesday |
| 2023-05-03 | closed | `May 3 (Wed.) Constitution Memorial Day` | `JPX-HOL-2023` | T1 | Printed date 2023-05-03, a Wednesday |
| 2023-05-04 | closed | `May 4 (Thu.) Greenery Day` | `JPX-HOL-2023` | T1 | Printed date 2023-05-04, a Thursday |
| 2023-05-05 | closed | `May 5 (Fri.) Children's Day` | `JPX-HOL-2023` | T1 | Printed date 2023-05-05, a Friday |
| 2023-07-17 | closed | `Jul. 17 (Mon.) Marine Day` | `JPX-HOL-2023` | T1 | Printed date 2023-07-17, a Monday |
| 2023-08-11 | closed | `Aug. 11 (Fri.) Mountain Day` | `JPX-HOL-2023` | T1 | Printed date 2023-08-11, a Friday |
| 2023-09-18 | closed | `Sep. 18 (Mon.) Respect for the Aged Day` | `JPX-HOL-2023` | T1 | Printed date 2023-09-18, a Monday |
| 2023-10-09 | closed | `Oct. 9 (Mon.) Sports Day` | `JPX-HOL-2023` | T1 | Printed date 2023-10-09, a Monday |
| 2023-11-03 | closed | `Nov. 3 (Fri.) Culture Day` | `JPX-HOL-2023` | T1 | Printed date 2023-11-03, a Friday |
| 2023-11-23 | closed | `Nov. 23 (Thu.) Labor Thanksgiving Day` | `JPX-HOL-2023` | T1 | Printed date 2023-11-23, a Thursday |

### 2024

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2024-01-01 | closed | `Jan. 1 (Mon.) New Year's Day` | `JPX-HOL-2024` | T1 | Printed date 2024-01-01, a Monday |
| 2024-01-02 | closed | `Jan. 2 (Tue.) Market Holiday` | `JPX-HOL-2024` | T1 | Printed date 2024-01-02, a Tuesday |
| 2024-01-03 | closed | `Jan. 3 (Wed.) Market Holiday` | `JPX-HOL-2024` | T1 | Printed date 2024-01-03, a Wednesday |
| 2024-01-08 | closed | `Jan. 8 (Mon.) Coming of Age Day` | `JPX-HOL-2024` | T1 | Printed date 2024-01-08, a Monday |
| 2024-02-12 | closed | `Feb. 12 (Mon.) National Foundation Day (Feb. 11) observed` | `JPX-HOL-2024` | T1 | Printed date 2024-02-12, a Monday; event date Feb. 11 is a Sunday and the page's own observance rule (`National holidays that fall on a Sunday are observed on the following Monday.`) is why the printed row lands on the Monday |
| 2024-02-23 | closed | `Feb. 23 (Fri.) Emperor's Birthday` | `JPX-HOL-2024` | T1 | Printed date 2024-02-23, a Friday |
| 2024-03-20 | closed | `Mar. 20 (Wed.) Vernal Equinox` | `JPX-HOL-2024` | T1 | Printed date 2024-03-20, a Wednesday |
| 2024-04-29 | closed | `Apr. 29 (Mon.) Showa Day` | `JPX-HOL-2024` | T1 | Printed date 2024-04-29, a Monday |
| 2024-05-03 | closed | `May 3 (Fri.) Constitution Memorial Day` | `JPX-HOL-2024` | T1 | Printed date 2024-05-03, a Friday |
| 2024-05-06 | closed | `May 6 (Mon.) Children's Day (May 5) observed` | `JPX-HOL-2024` | T1 | Printed date 2024-05-06, a Monday; event date May 5 is a Sunday and the page's own observance rule (`National holidays that fall on a Sunday are observed on the following Monday.`) is why the printed row lands on the Monday |
| 2024-07-15 | closed | `Jul. 15 (Mon.) Marine Day` | `JPX-HOL-2024` | T1 | Printed date 2024-07-15, a Monday |
| 2024-08-12 | closed | `Aug. 12 (Mon.) Mountain Day (Aug. 11) observed` | `JPX-HOL-2024` | T1 | Printed date 2024-08-12, a Monday; event date Aug. 11 is a Sunday and the page's own observance rule (`National holidays that fall on a Sunday are observed on the following Monday.`) is why the printed row lands on the Monday |
| 2024-09-16 | closed | `Sep. 16 (Mon.) Respect for the Aged Day` | `JPX-HOL-2024` | T1 | Printed date 2024-09-16, a Monday |
| 2024-09-23 | closed | `Sep. 23 (Mon.) Autumnal Equinox (Sep. 22) observed` | `JPX-HOL-2024` | T1 | Printed date 2024-09-23, a Monday; event date Sep. 22 is a Sunday and the page's own observance rule (`National holidays that fall on a Sunday are observed on the following Monday.`) is why the printed row lands on the Monday |
| 2024-10-14 | closed | `Oct. 14 (Mon.) Sports Day` | `JPX-HOL-2024` | T1 | Printed date 2024-10-14, a Monday |
| 2024-11-04 | closed | `Nov. 4 (Mon.) Culture Day (Nov. 3) observed` | `JPX-HOL-2024` | T1 | Printed date 2024-11-04, a Monday; event date Nov. 3 is a Sunday and the page's own observance rule (`National holidays that fall on a Sunday are observed on the following Monday.`) is why the printed row lands on the Monday |
| 2024-12-31 | closed | `Dec. 31 (Tue.) Market Holiday` | `JPX-HOL-2024` | T1 | Printed date 2024-12-31, a Tuesday |
### 2025

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2025-01-01 | closed | `Jan. 1 (Wed.) New Year's Day` | `JPX-HOL-2025` | T1 | Printed date 2025-01-01, a Wednesday; the weekday date is the trade date |
| 2025-01-02 | closed | `Jan. 2 (Thu.) Market Holiday` | `JPX-HOL-2025` | T1 | Printed date 2025-01-02, a Thursday |
| 2025-01-03 | closed | `Jan. 3 (Fri.) Market Holiday` | `JPX-HOL-2025` | T1 | Printed date 2025-01-03, a Friday |
| 2025-01-13 | closed | `Jan. 13 (Mon.) Coming of Age Day` | `JPX-HOL-2025` | T1 | Printed date 2025-01-13, a Monday |
| 2025-02-11 | closed | `Feb. 11 (Tue.) National Foundation Day` | `JPX-HOL-2025` | T1 | Printed date 2025-02-11, a Tuesday |
| 2025-02-24 | closed | `Feb. 24 (Mon.) Emperor's Birthday (Feb. 23) observed 1` | `JPX-HOL-2025` | T1 | Event date 2025-02-23 is a Sunday; the observance lands on Monday 2025-02-24 |
| 2025-03-20 | closed | `Mar. 20 (Thu.) Vernal Equinox` | `JPX-HOL-2025` | T1 | Printed date 2025-03-20, a Thursday |
| 2025-04-29 | closed | `Apr. 29 (Tue.) Showa Day` | `JPX-HOL-2025` | T1 | Printed date 2025-04-29, a Tuesday |
| 2025-05-05 | closed | `May 5 (Mon.) Children's Day` | `JPX-HOL-2025` | T1 | Printed date 2025-05-05, a Monday; May 3 and May 4 fall on the weekend and ship no rows |
| 2025-05-06 | closed | `May 6 (Tue.) Greenery Day (May 4) observed 1` | `JPX-HOL-2025` | T1 | Event date 2025-05-04 is a Sunday; the observance lands on Tuesday 2025-05-06 |
| 2025-07-21 | closed | `Jul. 21 (Mon.) Marine Day` | `JPX-HOL-2025` | T1 | Printed date 2025-07-21, a Monday |
| 2025-08-11 | closed | `Aug. 11 (Mon.) Mountain Day` | `JPX-HOL-2025` | T1 | Printed date 2025-08-11, a Monday |
| 2025-09-15 | closed | `Sep. 15 (Mon.) Respect for the Aged Day` | `JPX-HOL-2025` | T1 | Printed date 2025-09-15, a Monday |
| 2025-09-23 | closed | `Sep. 23 (Tue.) Autumnal Equinox` | `JPX-HOL-2025` | T1 | Printed date 2025-09-23, a Tuesday |
| 2025-10-13 | closed | `Oct. 13 (Mon.) Sports Day` | `JPX-HOL-2025` | T1 | Printed date 2025-10-13, a Monday |
| 2025-11-03 | closed | `Nov. 3 (Mon.) Culture Day` | `JPX-HOL-2025` | T1 | Printed date 2025-11-03, a Monday |
| 2025-11-24 | closed | `Nov. 24 (Mon.) Labor Thanksgiving Day (Nov. 23) observed 1` | `JPX-HOL-2025` | T1 | Event date 2025-11-23 is a Sunday; the observance lands on Monday 2025-11-24 |
| 2025-12-31 | closed | `Dec. 31 (Wed.) Market Holiday` | `JPX-HOL-2025` | T1 | Printed date 2025-12-31, a Wednesday |

### 2026

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2026-01-01 | closed | `Jan. 1 (Thu.) New Year's Day` | `JPX-HOL-2026-2027` | T1 | Printed date 2026-01-01, a Thursday |
| 2026-01-02 | closed | `Jan. 2 (Fri.) Market Holiday` | `JPX-HOL-2026-2027` | T1 | Printed date 2026-01-02, a Friday; the printed Jan. 3 Saturday Market Holiday ships no row |
| 2026-01-12 | closed | `Jan. 12 (Mon.) Coming of Age Day` | `JPX-HOL-2026-2027` | T1 | Printed date 2026-01-12, a Monday |
| 2026-02-11 | closed | `Feb. 11 (Wed.) National Foundation Day` | `JPX-HOL-2026-2027` | T1 | Printed date 2026-02-11, a Wednesday |
| 2026-02-23 | closed | `Feb. 23 (Mon.) Emperor's Birthday` | `JPX-HOL-2026-2027` | T1 | Printed date 2026-02-23, a Monday |
| 2026-03-20 | closed | `Mar. 20 (Fri.) Vernal Equinox` | `JPX-HOL-2026-2027` | T1 | Printed date 2026-03-20, a Friday |
| 2026-04-29 | closed | `Apr. 29 (Wed.) Showa Day` | `JPX-HOL-2026-2027` | T1 | Printed date 2026-04-29, a Wednesday |
| 2026-05-04 | closed | `May 4 (Mon.) Greenery Day` | `JPX-HOL-2026-2027` | T1 | Printed date 2026-05-04, a Monday; the printed May 3 Sunday Constitution Memorial Day ships no row |
| 2026-05-05 | closed | `May 5 (Tue.) Children's Day` | `JPX-HOL-2026-2027` | T1 | Printed date 2026-05-05, a Tuesday |
| 2026-05-06 | closed | `May 6 (Wed.) Constitution Memorial Day (May 3) observed 1` | `JPX-HOL-2026-2027` | T1 | Event date 2026-05-03 is a Sunday; the observance lands on Wednesday 2026-05-06 |
| 2026-07-20 | closed | `Jul. 20 (Mon.) Marine Day` | `JPX-HOL-2026-2027` | T1 | Printed date 2026-07-20, a Monday |
| 2026-08-11 | closed | `Aug. 11 (Tue.) Mountain Day` | `JPX-HOL-2026-2027` | T1 | Printed date 2026-08-11, a Tuesday |
| 2026-09-21 | closed | `Sep. 21 (Mon.) Respect for the Aged Day` | `JPX-HOL-2026-2027` | T1 | Printed date 2026-09-21, a Monday |
| 2026-09-22 | closed | `Sep. 22 (Tue.) Holiday 2` | `JPX-HOL-2026-2027` | T1 | Printed date 2026-09-22, a Tuesday; the page states it is `a holiday in accordance with Rule 3, Paragraph 3 of Act on National Holidays` |
| 2026-09-23 | closed | `Sep. 23 (Wed.) Autumnal Equinox` | `JPX-HOL-2026-2027` | T1 | Printed date 2026-09-23, a Wednesday |
| 2026-10-12 | closed | `Oct. 12 (Mon.) Sports Day` | `JPX-HOL-2026-2027` | T1 | Printed date 2026-10-12, a Monday |
| 2026-11-03 | closed | `Nov. 3 (Tue.) Culture Day` | `JPX-HOL-2026-2027` | T1 | Printed date 2026-11-03, a Tuesday |
| 2026-11-23 | closed | `Nov. 23 (Mon.) Labor Thanksgiving Day` | `JPX-HOL-2026-2027` | T1 | Printed date 2026-11-23, a Monday |
| 2026-12-31 | closed | `Dec. 31 (Thu.) Market Holiday` | `JPX-HOL-2026-2027` | T1 | Printed date 2026-12-31, a Thursday |

### 2027

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2027-01-01 | closed | `Jan. 1 (Fri.) New Year's Day` | `JPX-HOL-2026-2027` | T1 | Printed date 2027-01-01, a Friday; the printed Jan. 2 and Jan. 3 weekend Market Holidays ship no rows |
| 2027-01-11 | closed | `Jan. 11 (Mon.) Coming of Age Day` | `JPX-HOL-2026-2027` | T1 | Printed date 2027-01-11, a Monday |
| 2027-02-11 | closed | `Feb. 11 (Thu.) National Foundation Day` | `JPX-HOL-2026-2027` | T1 | Printed date 2027-02-11, a Thursday |
| 2027-02-23 | closed | `Feb. 23 (Tue.) Emperor's Birthday` | `JPX-HOL-2026-2027` | T1 | Printed date 2027-02-23, a Tuesday |
| 2027-03-22 | closed | `Mar. 22 (Mon.) Vernal Equinox (Mar. 21) observed 1` | `JPX-HOL-2026-2027` | T1 | Event date 2027-03-21 is a Sunday; the observance lands on Monday 2027-03-22 |
| 2027-04-29 | closed | `Apr. 29 (Thu.) Showa Day` | `JPX-HOL-2026-2027` | T1 | Printed date 2027-04-29, a Thursday |
| 2027-05-03 | closed | `May 3 (Mon.) Constitution Memorial Day` | `JPX-HOL-2026-2027` | T1 | Printed date 2027-05-03, a Monday |
| 2027-05-04 | closed | `May 4 (Tue.) Greenery Day` | `JPX-HOL-2026-2027` | T1 | Printed date 2027-05-04, a Tuesday |
| 2027-05-05 | closed | `May 5 (Wed.) Children's Day` | `JPX-HOL-2026-2027` | T1 | Printed date 2027-05-05, a Wednesday |
| 2027-07-19 | closed | `Jul. 19 (Mon.) Marine Day` | `JPX-HOL-2026-2027` | T1 | Printed date 2027-07-19, a Monday |
| 2027-08-11 | closed | `Aug. 11 (Wed.) Mountain Day` | `JPX-HOL-2026-2027` | T1 | Printed date 2027-08-11, a Wednesday |
| 2027-09-20 | closed | `Sep. 20 (Mon.) Respect for the Aged Day` | `JPX-HOL-2026-2027` | T1 | Printed date 2027-09-20, a Monday |
| 2027-09-23 | closed | `Sep. 23 (Thu.) Autumnal Equinox` | `JPX-HOL-2026-2027` | T1 | Printed date 2027-09-23, a Thursday |
| 2027-10-11 | closed | `Oct. 11 (Mon.) Sports Day` | `JPX-HOL-2026-2027` | T1 | Printed date 2027-10-11, a Monday |
| 2027-11-03 | closed | `Nov. 3 (Wed.) Culture Day` | `JPX-HOL-2026-2027` | T1 | Printed date 2027-11-03, a Wednesday |
| 2027-11-23 | closed | `Nov. 23 (Tue.) Labor Thanksgiving Day` | `JPX-HOL-2026-2027` | T1 | Printed date 2027-11-23, a Tuesday |
| 2027-12-31 | closed | `Dec. 31 (Fri.) Market Holiday` | `JPX-HOL-2026-2027` | T1 | Printed date 2027-12-31, a Friday |

### Documents

| Document | Window | Replay or service URL | Capture or retrieval, UTC | Tier | sha256 |
|---|---|---|---|---|---|
| `TSE-CAL-2010` | 2010-01-01..2027-12-31 | <https://web.archive.org/web/20100325235357id_/http://www.tse.or.jp/english/about/calendar.html> | Wayback `id_` replay of capture `20100325235357`, retrieved 2026-09-29 UTC (page state "Update : Dec. 30, 2009") | T1 | `84e86755d7cbf0df4bbabb2d37af8ba31c9eea7b50d12896231d70440090ea46` |
| `TSE-CAL-2011` | 2010-01-01..2027-12-31 | <https://web.archive.org/web/20110319172910id_/http://www.tse.or.jp/english/about/calendar.html> | Wayback `id_` replay of capture `20110319172910`, retrieved 2026-09-29 UTC (page state "Update : Jan. 04, 2011") | T1 | `2ebfe0698a3a7bb48d66ed5dcc93042b933fe9fdaa1cbf8a1048698418ed40c2` |
| `TSE-CAL-2012` | 2010-01-01..2027-12-31 | <https://web.archive.org/web/20120104081654id_/http://www.tse.or.jp/english/about/calendar.html> | Wayback `id_` replay of capture `20120104081654`, retrieved 2026-09-29 UTC (page state "Update : Jan. 04, 2012") | T1 | `af5ec14b87176d70accd462c6f83deb3065ea30ce4a1b5c51d97dbc17064422e` |
| `TSE-CAL-2013` | 2010-01-01..2027-12-31 | <https://web.archive.org/web/20130213034932id_/http://www.tse.or.jp/english/about/calendar.html> | Wayback `id_` replay of capture `20130213034932`, retrieved 2026-09-29 UTC (page state "Update : Jan. 04, 2013") | T1 | `2e1ad290bdbc601dd434e11aa770ff8b3e996fd0c2194141b35f484d6b1c9dac` |
| `TSE-CAL-2014` | 2010-01-01..2027-12-31 | <https://web.archive.org/web/20140212214059id_/http://www.tse.or.jp/english/about/calendar.html> | Wayback `id_` replay of capture `20140212214059`, retrieved 2026-09-29 UTC (page state "Update : Jan. 06, 2014") | T1 | `abb6c5a5a02f11ff20ee1f8595612af6892125ef6dcbe139184dab8bb9e5b46b` |
| `JPX-HOL-2015` | 2010-01-01..2027-12-31 | <https://web.archive.org/web/20150402033709id_/http://www.jpx.co.jp/english/corporate/calendar/> | Wayback `id_` replay of capture `20150402033709`, retrieved 2026-09-29 UTC | T1 | `daa6946c6b5efc5834f61b6e37fc76230a0ae3f124f5f2df2abac1d14b9d1845` |
| `JPX-HOL-2016` | 2010-01-01..2027-12-31 | <https://web.archive.org/web/20160106162134id_/http://www.jpx.co.jp/english/corporate/calendar/> | Wayback `id_` replay of capture `20160106162134`, retrieved 2026-09-29 UTC | T1 | `7559064a1dfd1ee857b8cdfff4eb1853ba2d22f5bf95c2c9b3321d40448a1ad1` |
| `JPX-HOL-2017` | 2010-01-01..2027-12-31 | <https://web.archive.org/web/20170506195702id_/http://www.jpx.co.jp/english/corporate/calendar/> | Wayback `id_` replay of capture `20170506195702`, retrieved 2026-09-29 UTC | T1 | `0a61b382e2a19bb8ce1d9fb181ec594a495caad0cb5d967cd65a56b7dfd19fa6` |
| `JPX-HOL-2018` | 2010-01-01..2027-12-31 | <https://web.archive.org/web/20180123165131id_/http://www.jpx.co.jp/english/corporate/calendar/> | Wayback `id_` replay of capture `20180123165131`, retrieved 2026-09-29 UTC (page state "Update : Jan. 04, 2018") | T1 | `54ad39c5376e551040a7704753d5edf2efe1e40349c39ee1c0845eb759f1796e` |
| `JPX-HOL-2019` | 2010-01-01..2027-12-31 | <https://web.archive.org/web/20190103232012id_/https://www.jpx.co.jp/english/corporate/about-jpx/calendar/> | Wayback `id_` replay of capture `20190103232012`, retrieved 2026-09-29 UTC (page state "Update : Jan. 01, 2019") | T1 | `f445fa0edfe84adb05e18982696d4656ab8ba8a3fb86eb4c92b2557f2711d2a2` |
| `JPX-HOL-2020` | 2010-01-01..2027-12-31 | <https://web.archive.org/web/20201112023210id_/https://www.jpx.co.jp/english/corporate/about-jpx/calendar/> | Wayback `id_` replay of capture `20201112023210`, retrieved 2026-09-29 UTC (page state "Update : Jan. 06, 2020") | T1 | `c8fb80b98ec01aaf6aad50f8b5cbf445bd14e8c0a4648c4c71e23c1d3cfef31c` |
| `JPX-HOL-2021` | 2010-01-01..2027-12-31 | <https://web.archive.org/web/20210417114538id_/https://www.jpx.co.jp/english/corporate/about-jpx/calendar/> | Wayback `id_` replay of capture `20210417114538`, retrieved 2026-09-29 UTC (page state "Update : Jan. 07, 2021") | T1 | `624872ba167e6559411676f75d7ca76f96867f573b643036a095663b7cfe1e32` |
| `JPX-HOL-2022` | 2010-01-01..2027-12-31 | <https://web.archive.org/web/20220707005934id_/https://www.jpx.co.jp/english/corporate/about-jpx/calendar/> | Wayback `id_` replay of capture `20220707005934`, retrieved 2026-09-29 UTC (page state "Update : Mar. 14, 2022") | T1 | `079c157ab703cfb2ecf4e1942313522167e941b6ee5b7026f8cb92eaf2b0205d` |
| `JPX-HOL-2023` | 2010-01-01..2027-12-31 | <https://web.archive.org/web/20230130070214id_/https://www.jpx.co.jp/english/corporate/about-jpx/calendar/> | Wayback `id_` replay of capture `20230130070214`, retrieved 2026-09-29 UTC (page state "Update : Sep. 02, 2022") | T1 | `34b5da5a09a9a9419ae147b4697726432741e242291839ef7fa6de207bb5e30b` |
| `JPX-HOL-2024` | 2010-01-01..2027-12-31 | <https://web.archive.org/web/20240213145440id_/https://www.jpx.co.jp/english/corporate/about-jpx/calendar/> | Wayback `id_` replay of capture `20240213145440`, retrieved 2026-09-29 UTC (page state "Update : Mar. 13, 2023") | T1 | `faad220eecb6f394cda4993ded099621a14f2e4b8d798f488b90905774edd9e8` |
| `JPX-HOL-2025` | 2025-01-01..2027-12-31 | <https://web.archive.org/web/20250923014239id_/https://www.jpx.co.jp/english/corporate/about-jpx/calendar/index.html> | Wayback `id_` replay of capture `20250923014239`, retrieved 2026-09-28 UTC (page state "Update : Mar. 07, 2025") | T1 | `b301e55c0d5e091602cabcb242dc28de76943e04db22115e4270b8a53de37877` |
| `JPX-HOL-2026-2027` | 2025-01-01..2027-12-31 | <https://www.jpx.co.jp/english/corporate/about-jpx/calendar/> | retrieved 2026-09-28 UTC (page state "Update : Feb. 06, 2026") | T1 | `32c6d13a925aff109c135947f2e809b5b9de1ae3d4dfb77b876f4a05d809090a` |

All seventeen artifacts are saved in the research store — the 2025-2027 pair under `holidays/raw/equities/tse/2025-2027/` and the fifteen 2010-2024 editions under `holidays/raw/equities/tse/2010-2024/`, each directory with an `INDEX.md` carrying the same digests. The 2025-2027 replay and the pre-2015 TSE-era captures are stored gzip-compressed exactly as replayed; the rest are stored exactly as served.

**Why the window runs 2010-01-01..2027-12-31.** The 2010 start is the charter support floor, and every edition from 2010 on states its own year complete (the equinoxes are printed in every edition), so the whole span answers from the operator's own calendars. **The forward end at 2027-12-31 needs no gap row.** The operator's own publication pattern is the current and next year on one page; every printed 2027 date is unconditional and complete (both equinoxes are printed), so the window reaches the end of the published future and nothing inside it is withheld. Every other trade date in the window is a date the page does not modify and is audited normal. The page's own caveat — `Exchange holidays are subject to change due to changes to national holidays under Japan's Act on National Holidays` — is the standing residual risk recorded under Gaps below; no such change has occurred inside this window. Re-checked monthly per LAW-WATCH; the 2028 table extends the window when JPX prints it.


## Sources

Row review: 2026-08-24 (UTC) is the date the ledger row was last reviewed as a
whole. Per-source retrieval dates were not recorded before the 2026-09-12
migration; where a later targeted review, capture or document date is recorded
beside a source below, that date governs for that source, and dates are added
as each source is re-verified.

- <https://www.jpx.co.jp/english/equities/trading/domestic/01.html> — JPX domestic equities trading hours: 09:00–11:30 and 12:30–15:30 auction-trading sessions, with order acceptance from 08:00 and 12:05.
- <https://www.jpx.co.jp/english/systems/equities-trading/01.html> — JPX equities trading systems; arrowhead continuous matching ends at 15:25 and the final five minutes are the closing call.
- <https://www.jpx.co.jp/english/systems/equities-trading/> — JPX equities trading systems index.
- <https://www.jpx.co.jp/english/equities/trading/tostnet/02.html> — JPX ToSTNeT hours: single-issue and basket trading (ToSTNeT-1) at 08:20–18:00 as of the 2026-08-24 review and 08:20–17:30 before the 2024-11-05 upgrade, which is why the earliest executable edge of the venue is 08:20 in both eras.
- <https://www.jpx.co.jp/english/corporate/news/news-releases/0020/b5b4pj000003xrsa-att/InvestigationReport.pdf> — Investigation Report of November 30, 2020 into the October 1, 2020 system failure: "Order acceptance began as normal at 08:00", which dates the 08:00 acceptance for the post-2011 profile.
- <https://www.jpx.co.jp/english/equities/trading/domestic/tvdivq0000006blj-att/tradinghours_eg.pdf> — JPX's official trading-hours transition table, dating the arrowhead morning extension to 2011-11-21.
- <https://www.jpx.co.jp/english/corporate/news/news-releases/1030/uorii50000002f2a-att/pressrelease_extension_of_trading_hours_en.pdf> — 2024 extension appendix, expressly changing ToSTNeT single-stock/basket trading to 18:00.
- <https://www.jpx.co.jp/english/corporate/news/news-releases/1030/20241103-01.html> — final go-live release confirming the upgraded arrowhead and ToSTNeT systems launched on 2024-11-05.
- <https://www.jpx.co.jp/english/corporate/investor-relations/shareholders/meeting/tvdivq000000958w-att/tse04.pdf> — 2010 shareholder report, establishing that ToSTNeT had already been extended to 17:30 in November 2009, before the January-2010 audit floor.
- <https://www.jpx.co.jp/corporate/research-study/working-paper/tvdivq0000008q5y-att/JPX_working_paper_No.3.pdf> — JPX Working Paper No.3, analysing the operator's own FLEX order-book data from 2010-01-04 and explicitly identifying orders entered from 08:00 outside the matching session.

## Gaps and residual risks

- **No holiday gap inside the audited window.** Every printed weekday closure of 2025-2027 ships a row and every other trade date in the window is audited normal; the window's end is the operator's own publication horizon (the current and next year), not a withholding. Closing condition for extension: JPX printing the 2028 table, which is re-checked monthly per LAW-WATCH.
- **No holiday gap inside 2010-2024 either.** Every printed weekday closure of 2010-2024 ships a row and every other trade date in the span is audited normal by that year's own edition; the edition lineages above record the places the operator itself revised a projection. The pre-2010 era sits below the support floor and is reviewed in no case.
- **National-holiday drift.** The page itself warns `Exchange holidays are subject to change due to changes to national holidays under Japan's Act on National Holidays`; a mid-year legislative change could move a printed date, and a slipped or cancelled date is corrected as a schedule fix (LAW-NO-FABRICATED-DATES).
- The 2010 shareholder report does not state an exact pre-floor day for the November-2009 ToSTNeT tail change, so none is invented (LAW-NO-FABRICATED-DATES). Nothing below the January-2010 floor is reviewed in any case.
- ToSTNeT is classified `extended` so the `regular` rules continue to describe the central auction market. Not every security or order type is eligible for every phase.
- The 08:00–08:20 arrowhead acceptance window is `order_entry`: orders may be entered, amended and cancelled, no matching engine runs, and ToSTNeT-1 does not open until 08:20, so nothing can print in it.
