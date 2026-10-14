<!-- SPDX-License-Identifier: MIT-0 -->

# `ice_europe_gilts` — evidence

- **Kind:** `MarketHoursKey`
- **Owner module:** [`ice_europe_gilts.rs`](../../src/calendar/schedules/futures/international/ice_europe_gilts.rs)
- **Source sets:** [`ICE-EU-FIXED-INCOME`](../schedules/sources.md#ice-eu-fixed-income)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-10-14 UTC)

ICE Futures Europe gilt futures share one London clock — Monday–Friday
08:00–18:00 with a 06:03–08:00 pre-open — until circular 26/063 extends Long
and Medium Gilt alone to a 00:45 pre-open and 01:00–21:00 trading on
2026-07-06; the holiday layer is audited over six operator-era windows after
the 2010 full-year notice (the 2009 zip) and the IFEU schedule editions
2015–2017 and 2019–2020 closed as data on 2026-10-14 UTC, leaving 2018, the
2022/2023 interior months and the family layers of 2019–2020 recorded gaps.

## Revision rows

- 2026-07-06 — T1 — ICE Futures Europe circular 26/063 — the circular of 23 April 2026 states "Effective from Monday 06 July 2026 the revised daily trading hours for the Contracts will be as follows" and prints Long Gilt Futures and Medium Gilt Futures at Pre Open 00:45, Open 01:00, Close 21:00 UK time; Short and Ultra Long Gilt are not among "the Contracts".

## The normal week

The shared gilt grid through 2026-07-05 is the operator's own Long Gilt
product page. The Wayback capture of 2026-03-11 (`ICE-GILT-PAGE-20260311`)
and the capture of 2026-05-09 (`ICE-GILT-PAGE-20260509`) both print
"LONDON | 8:00 AM - 6:00 PM | 08:00 - 18:00" with Pre-Open 06:03, and the
live page (`ICE-GILT-PAGE-LIVE`, retrieved 2026-10-11 UTC) prints the same
pre-change grid for the Short, Medium and Ultra Long Gilt products that
circular 26/063 left unchanged. The page names all four gilt futures as this
contract's related products, so the family trades one clock in this era. The
operator's temporary-trading-times document for 25–30 October 2026
(`ICE-EU-DST-2026`) reprints the split in its own table: "Medium and Long
Gilt Futures … 01:00-21:00 GMT" against "Short, Ultra Long Gilt Futures and
Long Gilt Options … 08:00-18:00 GMT".

The pre-open phases are `order_entry`: the page's Pre-Open column and the
circular's "Pre Open (UK Time)" column both precede the printed Open, and
nothing matches before it. `regular` carries the trading window; `extended`
is empty; the sessions are same-day and never wrap.

**The 2026-07-06 split and the disputed pre-open quarter-hour.** Circular
26/063 covers exactly four contract families — Three Month SONIA Indexed
Futures, MPC Dated SONIA Futures, Long Gilt Futures and Medium Gilt Futures —
and states "there will be no change to the current settlement windows for any
of the Contracts; and the time at which trading ceases on the Last Trading Day
remains unchanged". This key therefore narrows to the Long and Medium scope on
the cutover day; Short and Ultra Long Gilt stay on 08:00–18:00 and are a
different clock from that day, outside this key. On the pre-open the
circular's own table prints 00:45 while the operator's live product page
prints Pre-Open 00:35; the timeline holds the window both state — 00:45–01:00 —
and withholds the disputed 00:35–00:45 quarter-hour as a residual. Closing
condition: a desk answer or the operator's next spec-page revision stating one
instant.

**LAW-SESSION-NOT-EXPIRY.** The circular's unchanged "settlement windows" and
the Last-Trading-Day cessation are fixing and expiry instants, never session
boundaries; the Christmas documents' "designated settlement periods" columns
(12:13–12:15 GMT and siblings) are likewise not encoded — only the "Trading
ceases" column states a session instant.

**Carried normal week.** The full 08:00–18:00 grid is witnessed at the two
2026 captures; the NYSE Liffe notices of 2009–2013 open the gilt rows at
08:00 on their half days (LON3220's attachment prints Long Gilt and
Short/Medium Gilt opening 08:00 GMT on 24 and 31 December 2009,
corroborating the open era) but state no full session grid, and the IFEU
editions are holiday documents that print no session grid either. Everything
below 2026-03-11 is therefore the carried baseline declared in
`sourcing.rs`, with the interim states unproven rather than asserted.

## Holidays

**Coverage:** 2010-01-01..2014-12-31, 2015-01-01..2017-12-31, 2019-01-01..2020-12-31, 2021-01-01..2021-12-31, 2022-12-23..2023-01-02, 2024-01-01..2026-12-31 (gilt trade dates, Europe/London; sterling-based-product closures under NYSE Liffe, the IFEU Migrated-Liffe Fixed Income editions, the exchange-wide Trading Schedule editions where the Migrated companion is unwitnessed, and the Fixed Income editions and Christmas documents thereafter).

The pattern is stable across every audited era: the exchange closes 1
January, Good Friday and Christmas Day outright; the gilt row additionally
closes on Easter Monday and the UK bank holidays (early May, spring bank
holiday, late August, and the in-lieu Boxing Day / New Year holidays), and
stays **open on 1 May when it falls in a weekday-only closure year** (the
2024, 2025 and 2026 editions print "Open" there; the 2015 edition prints no
gilt closure for Friday 1 May 2015 either — see the 2015 section).
Christmas-eve and New-Year-eve half days close at the instant the era's
document prints — 12:15 GMT in the LIFFE attachments (2010–2013), 13:00 GMT
on 2022-12-23, and 12:15 GMT on 2022-12-30 and both 2025 eves. The 2016 and
2017 editions mark their 18:00-GMT December-eve early closes with per-row
asterisks the gilt rows do not carry, so the gilt eves there are audited
normal. Rows are keyed to the London civil session date; these sessions are
same-day, so no wrap reassignment arises.

**Announced eves with unpublished instants ship `Unsourced`**: LON3770 says
the 2014 eves close "at various times commencing at midday" — no exact
instant — the 2015 edition marks the gilt rows themselves as partial trading
days without an instant, and the 2021, 2024 and 2026 schedule editions defer
their partial trading days to a circular the captured set does not hold.
Those dates are inside a window and expressly **not** audited normal.

**2019 and 2020 ride the exchange-wide Trading Schedule editions, whose
family-level rows the gilt family's own Migrated edition would carry; that
edition is absent for both years.** The exchange-level rows are the
editions' own "Closed" status lines (1 January, Good Friday, Christmas
Day) — exchange closures that include every IFEU contract. The dates the
editions name as UK public holidays (Easter Monday, May Day, spring and
August bank holidays, Boxing Day, 2020's VE-Day 8 May) and the December
eves their Christmas appendices print special-hours tables for are marked
`Unsourced`: the editions print no gilt arrangement on them, and the
Migrated-era witness (2015–2017, 2021, 2024–2026) closes gilts on every one
of the holiday dates, so an audited-normal claim there would be unwitnessed
and — on the 2015 asterisk precedent — the eves' silence witnesses nothing.

2018 and 2023 February–December sit inside no window (see the gaps below).
Both are the charter's bridged residuals in name only — session questions
answer nothing there because the normal week below the 2026-03-11 horizon
is carried, not sourced, so the bridge's sourced-week condition never
reaches; the holiday classification refuses typed.

### 2010

LON3203 (issue 13 October 2009, effective 1 January 2010, "TRADING DAYS IN
2010") is the operator's own unconditional full-year notice: §1 states the
London market "will be open on Monday to Friday throughout 2010 except on
the following days" — 1 January (New Year's Day), 2 April (Good Friday) and
5 April (Easter Monday) — and §2 states that on the UK bank holidays 3 May,
31 May, 30 August, 27 December and 28 December "NYSE Liffe's sterling based
products and UK-based commodity products will not be made available"; the
gilt futures are sterling-based products, so each date is the family's
closure. LON3226 (30 November 2009) confirms LON3203 announced the 2010
trading days. The December eves' arrangements (§3, "closing of its
derivatives contracts from midday GMT") key no row here — LON3351's
attachment supplies the precise gilt instants the four shipped rows cite.

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2010-01-01 | closed | − LON3203 §1 "Friday 1 January 2010 (New Year's Day)"; LON3220's summary row prints the date against "Closed." | `LIFFE-ZIP-2009` | T1 | LON3203 §1 market-closed list; same local civil trade date |
| 2010-04-02 | closed | − "Friday 2 April 2010 (Good Friday)" | `LIFFE-ZIP-2009` | T1 | LON3203 §1 market-closed list; same local civil trade date |
| 2010-04-05 | closed | − "Monday 5 April 2010 (Easter Monday)" | `LIFFE-ZIP-2009` | T1 | LON3203 §1 market-closed list; same local civil trade date |
| 2010-05-03 | closed | − sterling-based products not made available | `LIFFE-ZIP-2009` | T1 | LON3203 §2 UK bank-holiday list; same local civil trade date |
| 2010-05-31 | closed | − sterling-based products not made available | `LIFFE-ZIP-2009` | T1 | LON3203 §2 UK bank-holiday list; same local civil trade date |
| 2010-08-30 | closed | − sterling-based products not made available | `LIFFE-ZIP-2009` | T1 | LON3203 §2 UK bank-holiday list; same local civil trade date |
| 2010-12-24 | early close | 12:15 GMT — "Long Gilt F … Open 08:00 … Close 12:15" (attachment) | `LIFFE-ZIP-2010` | T1 | LON3351's summary half day, precise instant from its Attachment 1 gilt row; same London civil trade date |
| 2010-12-27 | closed | − "Sterling-based products … will not be made available for trading" | `LIFFE-ZIP-2010` | T1 | LON3351, holiday in lieu of Christmas; same local civil trade date |
| 2010-12-28 | closed | − "Sterling-based products … will not be made available for trading" | `LIFFE-ZIP-2010` | T1 | LON3351, holiday in lieu of Boxing Day; same local civil trade date |
| 2010-12-31 | early close | 12:15 GMT — attachment gilt row Close 12:15 | `LIFFE-ZIP-2010` | T1 | LON3351's summary half day with the attachment's precise instant; same London civil trade date |

### 2011

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2011-01-03 | closed | − sterling-based products not available | `LIFFE-ZIP-2010` | T1 | LON3352 §2 UK bank holiday list; same local civil trade date |
| 2011-04-22 | closed | − "Friday 22 April 2011 (Good Friday)" | `LIFFE-ZIP-2010` | T1 | LON3352 §1 market-closed list; same local civil trade date |
| 2011-04-25 | closed | − "Monday 25 April 2011 (Easter Monday)" | `LIFFE-ZIP-2010` | T1 | LON3352 §1 market-closed list; same local civil trade date |
| 2011-05-02 | closed | − sterling-based products not available | `LIFFE-ZIP-2010` | T1 | LON3352 §2 UK bank holiday list; same local civil trade date |
| 2011-05-30 | closed | − sterling-based products not available | `LIFFE-ZIP-2010` | T1 | LON3352 §2 UK bank holiday list; same local civil trade date |
| 2011-08-29 | closed | − sterling-based products not available | `LIFFE-ZIP-2010` | T1 | LON3352 §2 UK bank holiday list; same local civil trade date |
| 2011-12-23 | early close | 12:15 GMT — attachment gilt row Close 12:15 | `LIFFE-ZIP-2011` | T1 | LON3526's sterling half day with the attachment's precise instant; same London civil trade date |
| 2011-12-26 | closed | − "Market Closed (UK Public Holiday)" | `LIFFE-ZIP-2011` | T1 | LON3526 and LON3352 §1; same local civil trade date |
| 2011-12-27 | closed | − "Sterling-based products … will not be made available" | `LIFFE-ZIP-2011` | T1 | LON3526 in-lieu closure, restated by LON3352 §2; same local civil trade date |
| 2011-12-30 | early close | 12:15 GMT — attachment gilt row Close 12:15 | `LIFFE-ZIP-2011` | T1 | LON3526's sterling half day with the attachment's precise instant; same London civil trade date |

### 2012

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2012-01-02 | closed | − sterling-based products not available | `LIFFE-ZIP-2011` | T1 | LON3530 §2 UK bank holiday list; same local civil trade date |
| 2012-04-06 | closed | − "Friday 6 April 2012 (Good Friday)" | `LIFFE-ZIP-2011` | T1 | LON3530 §1 market-closed list; same local civil trade date |
| 2012-04-09 | closed | − "Monday 9 April 2012 (Easter Monday)" | `LIFFE-ZIP-2011` | T1 | LON3530 §1 market-closed list; same local civil trade date |
| 2012-05-07 | closed | − sterling-based products not available | `LIFFE-ZIP-2011` | T1 | LON3530 §2 UK bank holiday list; same local civil trade date |
| 2012-06-04 | closed | − sterling-based products not available (spring bank holiday) | `LIFFE-ZIP-2011` | T1 | LON3530 §2 UK bank holiday list; same local civil trade date |
| 2012-06-05 | closed | − sterling-based products not available (Jubilee holiday) | `LIFFE-ZIP-2011` | T1 | LON3530 §2 UK bank holiday list; same local civil trade date |
| 2012-08-27 | closed | − sterling-based products not available | `LIFFE-ZIP-2011` | T1 | LON3530 §2 UK bank holiday list; same local civil trade date |
| 2012-12-24 | early close | 12:15 GMT — attachment gilt row Close 12:15 | `LIFFE-ZIP-2012` | T1 | LON3655's summary half day with the attachment's precise instant; same London civil trade date |
| 2012-12-25 | closed | − "Market Closed (UK Public Holidays)" | `LIFFE-ZIP-2012` | T1 | LON3655 and LON3530 §1; same local civil trade date |
| 2012-12-26 | closed | − "Market Closed (UK Public Holidays)" | `LIFFE-ZIP-2012` | T1 | LON3655 and LON3530 §1; same local civil trade date |
| 2012-12-31 | early close | 12:15 GMT — attachment gilt row Close 12:15 | `LIFFE-ZIP-2012` | T1 | LON3655's summary half day with the attachment's precise instant; LON3664 amended only Euribor, not gilts |

### 2013

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2013-01-01 | closed | − "Tuesday 1 January 2013 (New Year's Day)" | `LIFFE-ZIP-2012` | T1 | LON3657 §1 market-closed list; same local civil trade date |
| 2013-03-29 | closed | − "Friday 29 March 2013 (Good Friday)" | `LIFFE-ZIP-2012` | T1 | LON3657 §1 market-closed list; same local civil trade date |
| 2013-04-01 | closed | − "Monday 1 April 2013 (Easter Monday)" | `LIFFE-ZIP-2012` | T1 | LON3657 §1 market-closed list; same local civil trade date |
| 2013-05-06 | closed | − sterling-based products not available | `LIFFE-ZIP-2012` | T1 | LON3657 §2 UK Public Holiday list; same local civil trade date |
| 2013-05-27 | closed | − sterling-based products not available | `LIFFE-ZIP-2012` | T1 | LON3657 §2 UK Public Holiday list; same local civil trade date |
| 2013-08-26 | closed | − sterling-based products not available | `LIFFE-ZIP-2012` | T1 | LON3657 §2 UK Public Holiday list; same local civil trade date |
| 2013-12-24 | early close | 12:15 GMT — attachment gilt row Close 12:15 | `LIFFE-ZIP-2013` | T1 | LON3762's summary half day with the attachment's precise instant; same London civil trade date |
| 2013-12-25 | closed | − "Market Closed (UK Public Holidays)" | `LIFFE-ZIP-2013` | T1 | LON3762 and LON3657 §1; same local civil trade date |
| 2013-12-26 | closed | − "Market Closed (UK Public Holidays)" | `LIFFE-ZIP-2013` | T1 | LON3762 and LON3657 §1; same local civil trade date |
| 2013-12-31 | early close | 12:15 GMT — attachment gilt row Close 12:15 | `LIFFE-ZIP-2013` | T1 | LON3762's summary half day with the attachment's precise instant; same London civil trade date |

### 2014

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2014-01-01 | closed | − "Wednesday 1 January 2014 (New Year's Day)" | `LIFFE-ZIP-2013` | T1 | LON3770 §1 market-closed list; same local civil trade date |
| 2014-04-18 | closed | − "Friday 18 April 2014 (Good Friday)" | `LIFFE-ZIP-2013` | T1 | LON3770 §1 market-closed list; same local civil trade date |
| 2014-04-21 | closed | − "Monday 21 April 2014 (Easter Monday)" | `LIFFE-ZIP-2013` | T1 | LON3770 §1 market-closed list; same local civil trade date |
| 2014-05-05 | closed | − sterling-based products not available | `LIFFE-ZIP-2013` | T1 | LON3770 §2 UK Public Holiday list; same local civil trade date |
| 2014-05-26 | closed | − sterling-based products not available | `LIFFE-ZIP-2013` | T1 | LON3770 §2 UK Public Holiday list; same local civil trade date |
| 2014-08-25 | closed | − sterling-based products not available | `LIFFE-ZIP-2013` | T1 | LON3770 §2 UK Public Holiday list; same local civil trade date |
| 2014-12-24 | unsourced | announced half day; "contracts will close at various times commencing at midday" | `LIFFE-ZIP-2013` | T1 | LON3770 §3 states the arrangement without an exact instant; the date is expressly not audited normal |
| 2014-12-25 | closed | − "Thursday 25 December 2014 (Christmas Day)" | `LIFFE-ZIP-2013` | T1 | LON3770 §1 market-closed list; same local civil trade date |
| 2014-12-26 | closed | − "Friday 26 December 2014 (Boxing Day)" | `LIFFE-ZIP-2013` | T1 | LON3770 §1 market-closed list; same local civil trade date |
| 2014-12-31 | unsourced | announced half day; "contracts will close at various times commencing at midday" | `LIFFE-ZIP-2013` | T1 | LON3770 §3 states the arrangement without an exact instant; the date is expressly not audited normal |

### 2015

The first witnessed Migrated-Liffe edition (`IFEU-FI-2015`, captured
2015-09-19, "IFEU Fixed Income and Equity Contracts: Trading Days in 2015")
prints the exchange's closure list — 1 January, 3 April (Good Friday) and
25 December — states "All Fixed Income Contracts will be closed for trading
on 6 April 2015" (Easter Monday), and marks the gilt row × across its bank
holiday columns "4 May, 25 May and 31 August" and "28 December". Its
footnote ties to the gilt rows themselves: "*Thursday 24 December 2015 and
Thursday 31 December 2015 will be partial trading days" — an arrangement
without an instant, so both eves ship `Unsourced`. On Friday 1 May 2015 the
edition's "avoidance of doubt" note names only Three Month Euribor, Three
Month Euroswiss and Euro Swapnote® as trading — the note's subject is those
three contracts (the 2017 edition prints the same note flipped to "will be
closed" for them, proving it tracks that trio, not the family), 1 May 2015
is neither an exchange-closure date nor a column of the availability table,
and the UK early-May holiday it prints is Monday 4 May. The gilt row for
1 May 2015 is therefore absent and the date is audited normal.

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2015-01-01 | closed | − "Thursday 1 January 2015 (New Year's Day)" | `IFEU-FI-2015` | T1 | The edition's exchange-closure list; same local civil trade date |
| 2015-04-03 | closed | − "Friday 3 April 2015 (Good Friday)" | `IFEU-FI-2015` | T1 | The edition's exchange-closure list; same local civil trade date |
| 2015-04-06 | closed | − "All Fixed Income Contracts will be closed for trading on 6 April 2015" | `IFEU-FI-2015` | T1 | The edition's FI-closure line, Easter Monday; same local civil trade date |
| 2015-05-04 | closed | − gilt row "×", 4 May column | `IFEU-FI-2015` | T1 | The edition's FI availability table, "4 May, 25 May and 31 August" column; same local civil trade date |
| 2015-05-25 | closed | − gilt row "×", 25 May column | `IFEU-FI-2015` | T1 | The edition's FI availability table; same local civil trade date |
| 2015-08-31 | closed | − gilt row "×", 31 August column | `IFEU-FI-2015` | T1 | The edition's FI availability table; same local civil trade date |
| 2015-12-24 | unsourced | announced partial trading day; gilt row asterisked "*Thursday 24 December 2015 … will be partial trading days" | `IFEU-FI-2015` | T1 | The edition marks the gilt rows partial without an instant; the date is expressly not audited normal |
| 2015-12-25 | closed | − "Friday 25 December 2015 (Christmas Day)" | `IFEU-FI-2015` | T1 | The edition's exchange-closure list; same local civil trade date |
| 2015-12-28 | closed | − gilt row "×", 28 December column | `IFEU-FI-2015` | T1 | The edition's FI availability table; same local civil trade date |
| 2015-12-31 | unsourced | announced partial trading day; gilt row asterisked "*Thursday 31 December 2015 … will be partial trading days" | `IFEU-FI-2015` | T1 | The edition marks the gilt rows partial without an instant; the date is expressly not audited normal |

### 2016

The April 2016 Migrated edition (`IFEU-FI-2016`, captured 2016-04-19;
the October and December captures print the identical closure set and gilt
rows, and the December capture adds the 2017 edition) prints the exchange's
closure list — 1 January, 25 March (Good Friday) and 26 December, the
operator's own "(Christmas Day)" label for the Monday — states "All Fixed
Income Contracts will be closed for trading on 28 March 2016" (Easter
Monday), and prints explicit "Closed" cells for the gilt row across its
"2 May, 30 May and 29 August" and "27 December" columns. The edition's
December footnote — "*Friday 23 December 2016 and Friday 30 December 2016
will be an early close at 18:00 GMT" — ties to the asterisked rows
(EURIBOR futures, Eris EUR, USD Swapnotes, Bunds) and the gilt rows carry
no asterisk, so both gilt eves are audited normal.

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2016-01-01 | closed | − "Friday 1 January 2016 (New Year's Day)" | `IFEU-FI-2016` | T1 | The edition's exchange-closure list; same local civil trade date |
| 2016-03-25 | closed | − "Friday 25 March 2016 (Good Friday)" | `IFEU-FI-2016` | T1 | The edition's exchange-closure list; same local civil trade date |
| 2016-03-28 | closed | − "All Fixed Income Contracts will be closed for trading on 28 March 2016" | `IFEU-FI-2016` | T1 | The edition's FI-closure line, Easter Monday; same local civil trade date |
| 2016-05-02 | closed | − gilt row "Closed", 2 May column | `IFEU-FI-2016` | T1 | The edition's FI availability table, "2 May, 30 May and 29 August" column; same local civil trade date |
| 2016-05-30 | closed | − gilt row "Closed", 30 May column | `IFEU-FI-2016` | T1 | The edition's FI availability table; same local civil trade date |
| 2016-08-29 | closed | − gilt row "Closed", 29 August column | `IFEU-FI-2016` | T1 | The edition's FI availability table; same local civil trade date |
| 2016-12-26 | closed | − "Monday 26 December 2016 (Christmas Day)" | `IFEU-FI-2016` | T1 | The edition's exchange-closure list; same local civil trade date |
| 2016-12-27 | closed | − gilt row "Closed", 27 December column | `IFEU-FI-2016` | T1 | The edition's FI availability table; same local civil trade date |

### 2017

The January 2017 Migrated edition (`IFEU-FI-2017`, captured 2017-01-26; the
July capture and the edition bundled in the December-2016 capture print the
identical closure set and gilt rows) omits 1 January — a Sunday — from its
closure list and instead marks the in-lieu Monday: its "2 January" column
prints the gilt row "Closed". The exchange-closure list names 14 April
(Good Friday) and 25 December; "All Fixed Income Contracts will be closed
for trading on 17 April 2017" (Easter Monday); and the availability table's
columns 1 May, 29 May, 28 August and 26 December each print the gilt row
"Closed". As in 2016 the December footnote's 18:00-GMT early close ties to
asterisked non-gilt rows only, so 22 and 29 December 2017 are audited
normal for the family.

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2017-01-02 | closed | − gilt row "Closed", 2 January column | `IFEU-FI-2017` | T1 | The edition's FI availability table; the in-lieu New Year holiday, same local civil trade date |
| 2017-04-14 | closed | − "Friday 14 April 2017 (Good Friday)" | `IFEU-FI-2017` | T1 | The edition's exchange-closure list; same local civil trade date |
| 2017-04-17 | closed | − "All Fixed Income Contracts will be closed for trading on 17 April 2017" | `IFEU-FI-2017` | T1 | The edition's FI-closure line, Easter Monday; same local civil trade date |
| 2017-05-01 | closed | − gilt row "Closed", 1 May column | `IFEU-FI-2017` | T1 | The edition's FI availability table; same local civil trade date |
| 2017-05-29 | closed | − gilt row "Closed", 29 May column | `IFEU-FI-2017` | T1 | The edition's FI availability table; same local civil trade date |
| 2017-08-28 | closed | − gilt row "Closed", 28 August column | `IFEU-FI-2017` | T1 | The edition's FI availability table; same local civil trade date |
| 2017-12-25 | closed | − "Monday 25 December 2017 (Christmas Day)" | `IFEU-FI-2017` | T1 | The edition's exchange-closure list; same local civil trade date |
| 2017-12-26 | closed | − gilt row "Closed", 26 December column | `IFEU-FI-2017` | T1 | The edition's FI availability table; same local civil trade date |

### 2019

No Migrated edition is witnessed for 2019, so the year rides the
exchange-wide `2019 Trading Schedule` (`IFEU-TS-2019`, "Last Update:
January 2019"; the May and June updates print the identical table). Its
"Closed" status lines — 1 January, 19 April (Good Friday) and 25 December
— are exchange closures that include the family, and those three rows key
`Closed`. The editions mark the remaining UK holidays (22 April, 6 May,
27 May, 26 August, 26 December) "TD" with commodity-only closure notes and
print no gilt row anywhere, and the same document's Christmas appendix
prints 24 and 31 December special-hours tables without a gilt row. The
family's own document class — which in every witnessed year closes gilts
on the UK holidays and sometimes marks the eves partial — is absent, so
those seven dates ship `Unsourced` rather than an unwitnessed
audited-normal claim; the witness for the holiday rows themselves would be
the 2019 Migrated edition.

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2019-01-01 | closed | − "New Year's Day" / "UK and US Holiday", status "Closed" | `IFEU-TS-2019` | T1 | The edition's exchange-wide status column; same local civil trade date |
| 2019-04-19 | closed | − "Good Friday" / "UK and US Holiday", status "Closed" | `IFEU-TS-2019` | T1 | The edition's exchange-wide status column; same local civil trade date |
| 2019-04-22 | unsourced | "Easter Monday" / "UK Holiday", status "TD"; the additional-information column names commodity closures only, no gilt row | `IFEU-TS-2019` | T1 | The edition names the date and prints no family arrangement; expressly not audited normal |
| 2019-05-06 | unsourced | "May Day" / "UK Holiday", status "TD"; commodity-only closure notes, no gilt row | `IFEU-TS-2019` | T1 | The edition names the date and prints no family arrangement; expressly not audited normal |
| 2019-05-27 | unsourced | "Spring Bank Holiday" / "Memorial Day" / "UK and US Holiday", status "D" (an 18:30 early close for named commodity families); no gilt row | `IFEU-TS-2019` | T1 | The edition names the date and prints no family arrangement; expressly not audited normal |
| 2019-08-26 | unsourced | "UK Holiday", status "TD"; commodity-only closure notes, no gilt row | `IFEU-TS-2019` | T1 | The edition names the date and prints no family arrangement; expressly not audited normal |
| 2019-12-24 | unsourced | the Christmas appendix prints a 24 December hours table with no gilt row | `IFEU-TS-2019` | T1 | The appendix names the date and prints no family arrangement; expressly not audited normal |
| 2019-12-25 | closed | − "Christmas Day" / "UK and U.S. holiday", status "Closed"; the appendix prints "The Exchange will be closed." | `IFEU-TS-2019` | T1 | The edition's exchange-wide status column and its appendix; same local civil trade date |
| 2019-12-26 | unsourced | "Boxing Day" / "UK Holiday", status "TD"; the appendix's open-except list names commodity contracts only | `IFEU-TS-2019` | T1 | The edition names the date and prints no family arrangement; expressly not audited normal |
| 2019-12-31 | unsourced | the Christmas appendix prints a 31 December hours table with no gilt row | `IFEU-TS-2019` | T1 | The appendix names the date and prints no family arrangement; expressly not audited normal |

### 2020

As in 2019 no Migrated edition is witnessed, so the year rides the
exchange-wide `2020 Trading Schedule` (`IFEU-TS-2020`, the December-2020
edition, "Last Update: December 2020"; the January-2020 update and the
next-year table of the 2019 editions print the same closure set). Its
"Closed" status lines are 1 January, 10 April (Good Friday) and 25
December; the remaining UK holidays — including 2020's moved VE-Day
holiday on Friday 8 May, printed "May Day" / "UK Holiday" with status
"TD" — carry commodity-only notes with no gilt row, and the edition's
Appendix A prints 24, 28 and 31 December Christmas tables without a gilt
row. Those seven dates ship `Unsourced` on the same reasoning as 2019.

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2020-01-01 | closed | − "New Year's Day" / "UK and US Holiday", status "Closed" | `IFEU-TS-2020` | T1 | The edition's exchange-wide status column; same local civil trade date |
| 2020-04-10 | closed | − "Good Friday" / "UK and US Holiday", status "Closed" | `IFEU-TS-2020` | T1 | The edition's exchange-wide status column; same local civil trade date |
| 2020-04-13 | unsourced | "Easter Monday" / "UK Holiday", status "TD"; commodity-only closure notes, no gilt row | `IFEU-TS-2020` | T1 | The edition names the date and prints no family arrangement; expressly not audited normal |
| 2020-05-08 | unsourced | "May Day" / "UK Holiday", status "TD" — the edition's May-Day row falls on the moved VE-Day holiday Friday; commodity-only closure notes, no gilt row | `IFEU-TS-2020` | T1 | The edition names the date and prints no family arrangement; expressly not audited normal |
| 2020-05-25 | unsourced | "Spring Bank Holiday" / "Memorial Day" / "UK and US Holiday", status "D"; no gilt row | `IFEU-TS-2020` | T1 | The edition names the date and prints no family arrangement; expressly not audited normal |
| 2020-08-31 | unsourced | "UK Holiday", status "TD"; commodity-only closure notes, no gilt row | `IFEU-TS-2020` | T1 | The edition names the date and prints no family arrangement; expressly not audited normal |
| 2020-12-24 | unsourced | Appendix A prints a 24 December hours table with no gilt row | `IFEU-TS-2020` | T1 | The appendix names the date and prints no family arrangement; expressly not audited normal |
| 2020-12-25 | closed | − "Christmas Day" / "UK and US Holiday", status "Closed"; Appendix A prints "The Exchange will be closed." | `IFEU-TS-2020` | T1 | The edition's exchange-wide status column and its appendix; same local civil trade date |
| 2020-12-28 | unsourced | "Boxing Day" / "UK Holiday", status "TD" with an Appendix A pointer; the appendix's open-except list names commodity contracts only | `IFEU-TS-2020` | T1 | The edition names the date and prints no family arrangement; expressly not audited normal |
| 2020-12-31 | unsourced | Appendix A prints a 31 December hours table with no gilt row | `IFEU-TS-2020` | T1 | The appendix names the date and prints no family arrangement; expressly not audited normal |

### 2021

The exchange-wide December-2020 edition (`IFEU-TS-2020`, above) prints a
preliminary 2021 Christmas appendix whose 27 and 28 December pages say
"The Exchange will be OPEN … except for" a commodity-only exception list —
gilts unlisted. That document is the earlier, exchange-wide statement
("Last Update: December 2020"); the September-2021 Migrated edition is the
later document of the family's own class, and its availability table
prints the gilt row "Closed" across explicit 27 and 28 December columns.
Under the charter's lineage rule the later family-specific document
governs the family, and the shipped rows follow it; the earlier
appendix's exception lists enumerate the commodity complex's special
hours, as its per-contract tables (no fixed-income row anywhere) show. The
appendix likewise prints no gilt row on the 24 and 31 December 2021 eves,
so no sourced instant exists for the two `Unsourced` rows below.

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2021-01-01 | closed | − "Friday 1 January 2021 (New Year's Day)" | `IFEU-FI-2021` | T1 | IFEU schedule exchange-closure list; same local civil trade date |
| 2021-04-02 | closed | − "Friday 2 April 2021 (Good Friday)" | `IFEU-FI-2021` | T1 | IFEU schedule exchange-closure list; same local civil trade date |
| 2021-04-05 | closed | − gilt row "Closed", 5 April column | `IFEU-FI-2021` | T1 | IFEU fixed-income availability table; same local civil trade date |
| 2021-05-03 | closed | − gilt row "Closed", 3 May column | `IFEU-FI-2021` | T1 | IFEU fixed-income availability table; same local civil trade date |
| 2021-05-31 | closed | − gilt row "Closed", 31 May column | `IFEU-FI-2021` | T1 | IFEU fixed-income availability table; same local civil trade date |
| 2021-08-30 | closed | − gilt row "Closed", 30 August column | `IFEU-FI-2021` | T1 | IFEU fixed-income availability table; same local civil trade date |
| 2021-12-24 | unsourced | announced partial trading day; "will be confirmed via an Exchange Circular" | `IFEU-FI-2021` | T1 | The edition names the partial day without an instant; the date is expressly not audited normal |
| 2021-12-27 | closed | − gilt row "Closed", 27 December column | `IFEU-FI-2021` | T1 | IFEU fixed-income availability table, Christmas in lieu; same local civil trade date |
| 2021-12-28 | closed | − gilt row "Closed", 28 December column | `IFEU-FI-2021` | T1 | IFEU fixed-income availability table, Boxing Day in lieu; same local civil trade date |
| 2021-12-31 | unsourced | announced partial trading day; "will be confirmed via an Exchange Circular" | `IFEU-FI-2021` | T1 | The edition names the partial day without an instant; the date is expressly not audited normal |

### 2022

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2022-12-23 | early close | 13:00 GMT — gilt rows "Trading ceases 13:00 (GMT)" | `IFEU-XMAS-2022` | T1 | The Christmas document's Friday 23 December page; same London civil trade date |
| 2022-12-26 | closed | − "The Exchange will be closed" | `IFEU-XMAS-2022` | T1 | The Christmas document's Monday 26 December page; same local civil trade date |
| 2022-12-27 | closed | − gilts listed among the contracts excepted from trading | `IFEU-XMAS-2022` | T1 | The Christmas document's Tuesday 27 December page; in-lieu closure, same local civil trade date |
| 2022-12-30 | early close | 12:15 GMT — gilt rows "Trading ceases 12:15 (GMT)" | `IFEU-XMAS-2022` | T1 | The Christmas document's Friday 30 December page; same London civil trade date |

### 2023

| 2023-01-02 | closed | − exchange "CLOSED for trading EXCEPT for the contracts listed below"; gilts not listed | `IFEU-XMAS-2022` | T1 | The Christmas document's Monday 02 January 2023 page; in-lieu closure, same local civil trade date |

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|

The rest of 2023 sits inside no window: the 2023 Fixed Income edition is not
in the captured set (the 2023-09-27 capture is the Oil/Utilities/Agricultural
edition), so its Good Friday, Easter Monday, May and Coronation and August
bank-holiday treatments are unaudited. Closing condition: the IFEU 2023 Fixed
Income edition.

### 2024

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2024-01-01 | closed | − "Monday 1 January 2024 (New Year's Day)" | `IFEU-FI-2024` | T1 | IFEU schedule exchange-closure list; same local civil trade date |
| 2024-03-29 | closed | − "Friday 29 March 2024 (Good Friday)" | `IFEU-FI-2024` | T1 | IFEU schedule exchange-closure list; same local civil trade date |
| 2024-04-01 | closed | − gilt row "Closed", 1 Apr column | `IFEU-FI-2024` | T1 | IFEU fixed-income availability table; same local civil trade date |
| 2024-05-06 | closed | − gilt row "Closed", 6 May column | `IFEU-FI-2024` | T1 | IFEU fixed-income availability table; same local civil trade date |
| 2024-05-27 | closed | − gilt row "Closed", 27 May column | `IFEU-FI-2024` | T1 | IFEU fixed-income availability table; same local civil trade date |
| 2024-08-26 | closed | − gilt row "Closed", 26 Aug column | `IFEU-FI-2024` | T1 | IFEU fixed-income availability table; same local civil trade date |
| 2024-12-24 | unsourced | announced partial trading day; "will be confirmed via an Exchange circular" | `IFEU-FI-2024` | T1 | The edition names the partial day without an instant; the date is expressly not audited normal |
| 2024-12-25 | closed | − "Wednesday 25 December 2024 (Christmas Day)" | `IFEU-FI-2024` | T1 | IFEU schedule exchange-closure list; same local civil trade date |
| 2024-12-26 | closed | − gilt row "Closed", 26 Dec column | `IFEU-FI-2024` | T1 | IFEU fixed-income availability table, Boxing Day in lieu; same local civil trade date |
| 2024-12-31 | unsourced | announced partial trading day; "will be confirmed via an Exchange circular" | `IFEU-FI-2024` | T1 | The edition names the partial day without an instant; the date is expressly not audited normal |

The January and November 2024 editions print identical closure sets and gilt
rows; the November edition (`IFEU-FI-2024`) is the cited artifact and the
January edition is corroboration in the store.

### 2025

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2025-01-01 | closed | − "Wednesday 1 January 2025 (New Year's Day)" | `IFEU-FI-2025` | T1 | IFEU schedule exchange-closure list; same local civil trade date |
| 2025-04-18 | closed | − "Friday 18 April 2025 (Good Friday)" | `IFEU-FI-2025` | T1 | IFEU schedule exchange-closure list; same local civil trade date |
| 2025-04-21 | closed | − gilt row "Closed", 21 April column | `IFEU-FI-2025` | T1 | IFEU fixed-income availability table; same local civil trade date |
| 2025-05-26 | closed | − gilt row "Closed", 26 May column | `IFEU-FI-2025` | T1 | IFEU fixed-income availability table; 5 May prints "Open" and is audited normal |
| 2025-08-25 | closed | − gilt row "Closed", 25 August column | `IFEU-FI-2025` | T1 | IFEU fixed-income availability table; same local civil trade date |
| 2025-12-24 | early close | 12:15 GMT — gilt rows "Trading ceases 12:15 (GMT)" | `IFEU-XMAS-2025` | T1 | The Christmas document's Wednesday 24 December page; same London civil trade date |
| 2025-12-25 | closed | − "The Exchange will be closed" | `IFEU-FI-2025` | T1 | The edition's Christmas Day closure list, restated by the Christmas document's Thursday 25 December page; same local civil trade date |
| 2025-12-26 | closed | − exchange "OPEN … for the following contracts" only; gilts not listed | `IFEU-XMAS-2025` | T1 | The Christmas document's Friday 26 December page; gilt closure by omission from the open list, same local civil trade date |
| 2025-12-31 | early close | 12:15 GMT — gilt rows "Trading ceases 12:15 (GMT)" | `IFEU-XMAS-2025` | T1 | The Christmas document's Wednesday 31 December page; same London civil trade date |

### 2026

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2026-01-01 | closed | − "Thursday 1 January 2026 (New Year's Day)" | `IFEU-FI-2026` | T1 | IFEU schedule exchange-closure list; same local civil trade date |
| 2026-04-03 | closed | − "Friday 03 April 2026 (Good Friday)" | `IFEU-FI-2026` | T1 | IFEU schedule exchange-closure list; same local civil trade date |
| 2026-04-06 | closed | − gilt row "Closed", 6 Apr column | `IFEU-FI-2026` | T1 | IFEU fixed-income availability table; same local civil trade date |
| 2026-05-04 | closed | − gilt row "Closed", 4 May column | `IFEU-FI-2026` | T1 | IFEU fixed-income availability table; 1 May prints "Open" and is audited normal |
| 2026-05-25 | closed | − gilt row "Closed", 25 May column | `IFEU-FI-2026` | T1 | IFEU fixed-income availability table; same local civil trade date |
| 2026-08-31 | closed | − gilt row "Closed", 31 Aug column | `IFEU-FI-2026` | T1 | IFEU fixed-income availability table; same local civil trade date |
| 2026-12-24 | unsourced | announced partial trading day; "will be confirmed via an Exchange circular" | `IFEU-FI-2026` | T1 | The edition names the partial day without an instant; the date is expressly not audited normal |
| 2026-12-25 | closed | − "Friday 25 December 2026 (Christmas Day)" | `IFEU-FI-2026` | T1 | IFEU schedule exchange-closure list; same local civil trade date |
| 2026-12-28 | closed | − gilt row "Closed", 28 Dec column | `IFEU-FI-2026` | T1 | IFEU fixed-income availability table, Boxing Day in lieu; same local civil trade date |
| 2026-12-31 | unsourced | announced partial trading day; "will be confirmed via an Exchange circular" | `IFEU-FI-2026` | T1 | The edition names the partial day without an instant; the date is expressly not audited normal |

### Documents

Artifacts live in the research store under
`normal-weeks/ice-gilts-2026-10-11/` (product pages, the circular, the live
August 2026 schedule and Christmas PDFs, the T2 workbook, the notice zips),
`holidays/raw/ice-europe-gilts-2010-2027/` (the archived schedule
editions, a second copy of the zips and the 2009 zip) and
`holidays/raw/ice-history-2026-10-13/` (the 2015–2020 editions, extracted
beside their PDFs, with `INDEX.md`, `INVENTORY.tsv` and `SHA256SUMS.txt`
fences). Digests below re-verified against the store bytes on 2026-10-14
UTC. The LIFFE ids resolve to the operator's annual notice-archive zips;
each zip's per-notice citations (LON3203, LON3351 and siblings) are named
in the year tables above and the quoted rows were read from the notice
PDFs inside the named zip.

| Document | Window | Replay or service URL | Capture or retrieval, UTC | Tier | sha256 |
|---|---|---|---|---|---|
| `LIFFE-ZIP-2009` | 2010-01-01..2010-08-30 (LON3203; LON3220/LON3226 corroborate) | <https://www.ice.com/publicdocs/nyse_liffe/historical_notices/2009.zip> | retrieved 2026-10-13 UTC | T1 | `b9c6610cf544bee19d4d2005c4dd3fe204fefb60f902dc56d79d29776ae8dbd1` |
| `LIFFE-ZIP-2010` | 2010-12-24..2011-12-30 (LON3351, LON3352) | <https://www.ice.com/publicdocs/nyse_liffe/historical_notices/2010.zip> | retrieved 2026-10-11 UTC | T1 | `31a87b7d07e69a40fbf89e7f13ca730286c48c2d349ec04e20a56a27d953271b` |
| `LIFFE-ZIP-2011` | 2011-12-23..2012-12-31 (LON3526, LON3530) | <https://www.ice.com/publicdocs/nyse_liffe/historical_notices/2011.zip> | retrieved 2026-10-11 UTC | T1 | `d1025dd65a551be278cd7d0bfb888d7b35340125624b8eeb93b960333dad71c9` |
| `LIFFE-ZIP-2012` | 2012-12-24..2013-08-26 (LON3655, LON3657) | <https://www.ice.com/publicdocs/nyse_liffe/historical_notices/2012.zip> | retrieved 2026-10-11 UTC | T1 | `da9498b4cdbbab44dc299f449edbd82a8d2344b45077b08c7e4bbe296b183084` |
| `LIFFE-ZIP-2013` | 2013-12-24..2014-12-31 (LON3762, LON3770) | <https://www.ice.com/publicdocs/nyse_liffe/historical_notices/2013.zip> | retrieved 2026-10-11 UTC | T1 | `905cfaaea0817a316af55024d45da84b0537ef5b613480297938f2b13adefb23` |
| `IFEU-FI-2015` | 2015-01-01..2015-12-31 | <https://web.archive.org/web/20150919192106id_/https://www.theice.com/publicdocs/futures/Trading_Schedule_Migrated_Liffe_Contracts.pdf> | Wayback `id_` replay of capture `20150919192106`, retrieved 2026-10-13 UTC | T1 | `617d811630e78aeac5cf075b15d0c64647828217456db65dd043cfad1295ecda` |
| `IFEU-FI-2016` | 2016-01-01..2016-12-31 | <https://web.archive.org/web/20160419003940id_/https://www.theice.com/publicdocs/futures/Trading_Schedule_Migrated_Liffe_Contracts.pdf> | Wayback `id_` replay of capture `20160419003940`, retrieved 2026-10-13 UTC | T1 | `a8651e82230609e1e824956c013e9e961e6964fa8353cbbb9c310db6816cc272` |
| `IFEU-FI-2017` | 2017-01-01..2017-12-31 | <https://web.archive.org/web/20170126164934id_/https://www.theice.com/publicdocs/futures/Trading_Schedule_Migrated_Liffe_Contracts.pdf> | Wayback `id_` replay of capture `20170126164934`, retrieved 2026-10-13 UTC | T1 | `406d9d23ecebe7938282d23ca6194e72c1a0a38b5685ca68a579dc61ec46eb21` |
| `IFEU-TS-2019` | 2019-01-01..2019-12-31 | <https://web.archive.org/web/20190103162112id_/https://www.theice.com/publicdocs/Trading_Schedule.pdf> | Wayback `id_` replay of capture `20190103162112`, retrieved 2026-10-13 UTC | T1 | `3b977b2311b169a00c541d0fe32a56216cea714a2f124b75b524e2f50799c9bb` |
| `IFEU-TS-2020` | 2020-01-01..2020-12-31 (the 2020 table and Appendix A; the preliminary 2021 appendix is lineage context) | <https://web.archive.org/web/20201231184142id_/https://www.theice.com/publicdocs/Trading_Schedule.pdf> | Wayback `id_` replay of capture `20201231184142`, retrieved 2026-10-13 UTC | T1 | `ee74ea06ed480239fd0c484c347d078a33e42e39d2f5196f0aee942085e948fb` |
| `IFEU-FI-2021` | 2021-01-01..2021-12-31 | <https://web.archive.org/web/20211022182425id_/https://www.ice.com/publicdocs/futures/Trading_Schedule_Migrated_Liffe_Contracts.pdf> | Wayback `id_` replay of capture `20211022182425`, retrieved 2026-10-11 UTC | T1 | `7e7b897601ed455406d6c61e6a20074e8af907915cf9550f6c29303b79fdefb8` |
| `IFEU-XMAS-2022` | 2022-12-23..2023-01-02 | <https://www.ice.com/publicdocs/futures/IFEU_Christmas_and_New_Year_Trading_Hours_2022.pdf> | retrieved 2026-10-11 UTC | T1 | `1dea6fe3b03798f3117919d707eeaca1753fd50fbf7b173626c2542105c0b950` |
| `IFEU-FI-2024` | 2024-01-01..2024-12-31 | <https://web.archive.org/web/20241203121313id_/https://www.ice.com/publicdocs/futures/Trading_Schedule_Migrated_Liffe_Contracts.pdf> | Wayback `id_` replay of capture `20241203121313`, retrieved 2026-10-11 UTC | T1 | `9fc431d63fed0b3d4b037b411b6ce5c1f26d616942f2b33de650c25860408894` |
| `IFEU-FI-2025` | 2025-01-01..2025-12-31 | <https://web.archive.org/web/20260728113518id_/https://www.ice.com/publicdocs/futures/Trading_Schedule_Migrated_Liffe_Contracts.pdf> | Wayback `id_` replay of capture `20260728113518`, retrieved 2026-10-11 UTC | T1 | `45d51a2c389a5d634f2c9404b930573e84b1c5c601eeaf98c7444f28d040377e` |
| `IFEU-XMAS-2025` | 2025-12-24..2025-12-31 | <https://www.ice.com/publicdocs/futures/IFEu_Christmas_New_Year_Trading_Hours_2025.pdf> | retrieved 2026-10-11 UTC | T1 | `ec7a173ea733a17c4e3fd12006bd0ea703bfce95b3f5c7e6cd8192f6a85c6cd0` |
| `IFEU-FI-2026` | 2026-01-01..2026-12-31 | <https://www.ice.com/publicdocs/futures/Trading_Schedule_Migrated_Liffe_Contracts.pdf> | retrieved 2026-10-11 UTC | T1 | `8947f6eb6659cbe3ecb8ed909562a1013c6ca788e9167c74811d4d52f5865ff6` |
| `ICE-CIRC-26-063` | the 2026-07-06 cutover | <https://www.ice.com/publicdocs/circulars/26063.pdf> | retrieved 2026-10-11 UTC | T1 | `385479b77d7c78467006909dafb69d8691edd507e94eb1c305c5b0f909d1c6f7` |
| `ICE-EU-DST-2026` | the 2026 split restated | <https://www.ice.com/publicdocs/futures/Trading_Schedule_Temporary_Trading_Hours_for_DST.pdf> | retrieved 2026-10-11 UTC | T1 | `c9ffb649c802465a7f5ceedef403140c36a3575966cd619e3d4a1526fe6d9579` |
| `ICE-GILT-PAGE-LIVE` | current Long Gilt grid | <https://www.ice.com/products/37650336/Long-Gilt-Future> | retrieved 2026-10-11 UTC | T1 | `01c6371a9e3211dd4ecb43b6d5d100250a99a9823600d5a2f247657213de481a` |
| `ICE-GILT-PAGE-20260311` | the pre-change grid | <https://web.archive.org/web/20260311095734id_/https://www.ice.com/products/37650336/Long-Gilt-Future> | Wayback `id_` replay of capture `20260311095734`, retrieved 2026-10-11 UTC | T1 | `9795400c3e148d7fea202e7ceb70c781d41eaf6df2a9c18b963b6e98c5c26759` |
| `ICE-GILT-PAGE-20260509` | the pre-change grid | <https://web.archive.org/web/20260509id_/https://www.ice.com/products/37650336> | Wayback replay of capture 2026-05-09, retrieved 2026-10-11 UTC | T1 | `59f523fff54e830a2473a8a0616dedada156849207def21b73cbe15e3969dad9` |
| `ICE-EU-XLSX-2026` | the T2 change tracker | <https://www.ice.com/publicdocs/futures/Excel_Trading_Schedule.xlsx> | retrieved 2026-10-11 UTC | T2 | `33afaf80680739bb20686af842e62f21f9af2e394abd0c49ee293a804e844d1b` |

## Sources

- <https://www.ice.com/products/37650336/Long-Gilt-Future> — the Long Gilt product page; the monitoring entry point for the family grid and its Pre-Open column.
- <https://www.ice.com/publicdocs/futures/Trading_Schedule_Migrated_Liffe_Contracts.pdf> — the IFEU Trading Schedule: Fixed Income, Equity and Indices, republished as editions; the holiday-table monitoring entry point.
- <https://www.theice.com/publicdocs/Trading_Schedule.pdf> — the exchange-wide annual Trading Schedule (the 2019/2020 era's witnessed class for this family; commodity-scoped per-contract tables, so it keys only the exchange-level closures here).
- <https://www.ice.com/publicdocs/futures/IFEu_Christmas_New_Year_Trading_Hours_2025.pdf> — the December Christmas/New Year trading-hours document series.
- <https://www.ice.com/publicdocs/circulars/26063.pdf> — ICE Futures Europe circular 26/063; the circulars index is <https://www.ice.com/futures-europe/circulars>.
- <https://www.ice.com/publicdocs/nyse_liffe/historical_notices/> — the NYSE Liffe historical-notice zip archive (2005–2014 offered).
- <https://www.ice.com/publicdocs/futures/Excel_Trading_Schedule.xlsx> — the operator's T2 Excel trading-schedule workbook, whose Change Tracker cites the governing circular per change.
- <https://www.ice.com/holiday-hours> — the ICE holiday-hours hub that links the schedule editions above (captured byte-identical 2026-10-10 and 2026-10-11 UTC).

## Gaps and residual risks

- **2018.** Inside no audited window: the archive holds no Trading
  Schedule capture of any kind between 2017-07-07 and 2019-01-03 (the store's
  CDX sweep enumerates every archived capture of every `Trading_Schedule*`
  URL 2015–2022; the Common Crawl 2018 indexes answered 502/504 for all nine
  queries — inconclusive, not negative). The year is bracketed by the
  witnessed 2017 and 2019 editions, but the normal week below the
  2026-03-11 horizon is carried, so no bridge reaches it and both layers
  refuse typed. Closing condition: a Common Crawl 2018 query for
  `theice.com/publicdocs/Trading_Schedule.pdf` when the service recovers,
  then a desk ask for the 2018 edition PDF.
- **2019 and 2020 family layer.** The exchange-level closures ship, but the
  Migrated-Liffe edition — the document class that prints the family's own
  availability rows — is not witnessed for either year, so every other
  holiday and both years' December eves carry `Unsourced` markers rather
  than audited-normal claims. Closing condition: the 2019 and 2020
  Migrated editions.
- **2022 January–November and 2023 February–December.** Inside no window: the
  2022 and 2023 Fixed Income editions are not in the captured set, so only
  the Christmas-document dates audited. Closing condition: those editions.
- **The 00:35 vs 00:45 pre-open.** The live product page prints Pre-Open
  00:35; circular 26/063 prints 00:45. The timeline serves 00:45–01:00 — the
  window both state — and the disputed 00:35–00:45 quarter-hour is withheld.
  Closing condition: a desk answer or the operator's next spec-page revision.
- **Announced eves with unpublished instants** (2014-12-24, 2014-12-31,
  2015-12-24, 2015-12-31, 2021-12-24, 2021-12-31, 2024-12-24, 2024-12-31,
  2026-12-24, 2026-12-31) are `Unsourced` rows, never audited normal.
  Closing condition per year: the operator's Christmas circular.
- **2027** is unpublished (the 2027 edition and the Christmas-2026 circular
  both 404 live, checked 2026-10-11 UTC). The audited window ends at
  2026-12-31; no 2027 date is asserted normal.
- The live product page names Medium, Short and Ultra Long Gilt as related
  products of the Long Gilt page and the shared-grid claim rests on that page
  family plus the DST document's split table; the Short/Ultra product pages
  were not separately captured. A separate capture would strengthen the
  shared-era claim but the October 2026 temporary-trading-times document
  states the split directly.
- **2006–2008 remain unpulled.** The live archive also serves the 2006,
  2007 and 2008 notice zips (the store's probe log records the 200-status
  responses); 2008 is banked (`nyse_liffe_2008.zip`, with LON3073 the 2009
  full-year notice), but the 2006/2007 zips were left in place. They predate
  nothing this key owes — the support floor is 2010-01-01 — and are recorded
  only so a later cross-family hunt knows the channel is open.
