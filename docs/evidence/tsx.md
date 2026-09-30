<!-- SPDX-License-Identifier: MIT-0 -->

# `tsx` — evidence

- **Kind:** `Exchange`
- **Owner module:** [`tsx.rs`](../../src/calendar/schedules/equities/americas/tsx.rs)
- **Source sets:** [`AMER-TSX`](../schedules/sources.md#amer-tsx)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

Cash equities; conditional MOC extension represented by its maximum envelope.

## Revision rows

None. `tsx.rs` holds a single static profile with no dated revision row.

## Sources

Row review: 2026-08-22 (UTC) is the date the ledger row was last reviewed as a
whole. Per-source retrieval dates were not recorded before the 2026-09-12
migration; where a later targeted review, capture or document date is recorded
beside a source below, that date governs for that source, and dates are added
as each source is re-verified.

- <https://www.tsx.com/en/trading/calendars-and-trading-hours/trading-hours> — TSX trading hours: orders accepted from 07:00, continuous trading 09:30–16:00, the conditional Market-on-Close Price Movement Extension through 16:10, and Extended Trading at the last sale price 16:15–17:00. The venue's session table describes Pre-Open as a phase in which orders may be entered but will not be executed.
- <https://www.osc.ca/sites/default/files/pdfs/bulletins/oscb_20050114_2802.pdf> — Ontario Securities Commission Bulletin of 2005-01-14, volume 28 issue 2: the regulator record establishing that both the Price Movement Extension and the last-sale session existed before the January-2010 history floor.

## Holidays

**Coverage:** 2010-01-01..2011-10-10, 2012-01-03..2012-12-31, 2013-01-01..2013-08-18, 2014-01-02..2014-07-01, 2017-01-01..2026-12-31
(inclusive trade dates in `America/Toronto`; tier T1 throughout).

The operator's holiday statement for 2017-2026 is TMX Group's own "Calendar"
page, `tsx.com/en/trading/calendars-and-trading-hours/calendar` (found through
the tsx.com sitemap), one server-rendered page carrying the current year's
"Stock Market Holidays - Stock Markets Closed" list in full and the next
year's from Q4. TSX runs no overnight session, so an event date and its trade
date are one civil day and the conversion is the identity. The 2025 and 2026
lists are read from the live retrieval of 2026-09-28; every earlier year is
pinned by Wayback `id_` captures of the same page across its two paths — the
2018 relaunch path `tsx.com/trading/calendars-and-trading-hours/calendar`
(named by the archived sitemap.xml of 2022-06-26; seven captures
2018-09-11..2023-12-15 pin 2017 through 2023, the 2018-09-11 capture's archive
section restating the complete 2017 list) and the `/en/` path for 2024-2025
(the 2024-12-17 capture carries the complete 2024 list; the July 2024 states
printed the list without its Christmas Eve row, so that row keys to the
December state — a knowledge boundary may only widen).

The operator's holiday statement for 2010-2014 is its own **news releases** on
`tmx.com` (found 2026-09-30 UTC by domain-wide Wayback CDX sweeps of `tsx.com`
2010-2016 — 5 910 collapsed url keys — and `tmx.com` 2010-2016 — 15 931 —
plus an all-time `news_releases` filter sweep enumerating 1 789 capture rows;
per-URL sweeps of `tmxgroup.com`, `tsx.ca` and `mx-group.com` returned
nothing). The practice is one release per market closure — *Toronto Stock
Exchange, TSX Venture Exchange and Montreal Exchange closed for <holiday>* —
naming the exchange and an unconditional date in session language, plus a
December **Holiday (Operating) Schedule** release stating the year-end
arrangement in full. Within a recovered window a date without a row is
audited normal on that basis: the all-time sweep enumerated every closure
notice the archive holds, the year's arrangement is the union of its notices,
and the archive's own bulk sweeps of the release tree mark how far that
enumeration reaches — the 2012-05-19 and 2012-09-20 sweeps caught every 2012
release from February through July, the 2013-08-18 sweep caught every 2013
release through the Civic notice, and the 2014-07-14 sweep caught every
2014-published release back to March. The 2012 window runs to 2012-12-31
because the schedule's own TSX table prints `Monday, December 31, 2012 Open`;
the 2012 and 2014 windows open past the New Year arrangements (2012-01-02,
2014-01-01) whose announcing releases the archive never captured; the 2013
window ends at the last witnessed tree state. The spans between the windows
ship no data and refuse.

**2027 is not published.** The page's newest section is 2026; TMX historically
adds the next year's calendar in Q4. Verified 2026-09-28: no 2027 list exists
on the page or behind its "Settlement Schedule" links. Nothing past 2026-12-31
is claimed; **closing condition:** the operator's 2027 calendar section.

**The Christmas Eve half days are the operator's own sentences.** From 2024 on each Christmas
Eve entry carries `*`, footnoted `* Closing at 1:00 PM (TSX/TSXV) and 1:30 (ALPHA/ALPHA
X/DRK)`; 2018-2020 print the same instant as a page sentence, `Markets will close at 1:00 PM
on December 24th, <year>.`, and 2021's January capture printed `*Markets will close at 1:00pm
on December 24th, 2021 subject to Board Approval` — a conditional the 2022-01-28 capture's
archive section witnesses as discharged (`Christmas Eve - Friday, December 24, 2021 ** ...
TSX and TSX Venture Exchange will close early at 1:00 p.m.`), so the 2021 row keys to that
later artifact and the discharge is recorded beside it. This identity's scope is the Toronto
Stock Exchange cash-equity market, whose close is the printed 1:00 PM TSX/TSXV instant; the
1:30 half belongs to the ALPHA/ALPHA X/DRK book systems, which are separate trading venues
outside this row. No other date in any audited list carries an early close, late open or
half-day marking, and the 2022 and 2023 lists print no Christmas Eve line (24 December fell
on a weekend both years).

**The U.S. holidays are settlement data, not closures.** The page's "U.S. Holidays" block
(MLK Day, Memorial Day, Juneteenth, Independence Day or its in-lieu, U.S. Thanksgiving) is
footnoted `** U.S. Holidays with Special Settlement for Issues trading in USD` — a settlement
arrangement for USD issues, not a TSX trading closure, so none of those dates is encoded
(LAW-SESSION-NOT-EXPIRY).

**The 2010-2014 releases are the operator's session language.** Each release names Toronto
Stock Exchange and states an unconditional day: *closed for <holiday> on Monday, February 15,
2010*, or the schedule tables' `Closed` / `Open until 1:00 p.m. (EST)` rows. The 1:00 p.m.
EST Christmas Eve rows are early closes at 13:00 Toronto time — December is EST in Toronto —
the same instant the calendar page's later footnote prints. The 2012-07-12 "stampede" release
was checked and is a market-close ceremony, not a closure; the U.S.-style closures the
releases name for NGX or MX do not touch these rows' TSX scope.

### 2010

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2010-01-01 | closed | `Friday, January 1, 2010 Closed` (TSX/TSXV schedule row) — `January 1, 2010, for New Year's Day` | `TSX-REL-2009-12-02` | T1 | the schedule release's own TSX/TSXV table row; trade date is the same civil day |
| 2010-02-15 | closed | `will be closed for Family Day on Monday February 15, 2010` | `TSX-REL-2010-02-08` | T1 | the operator's own printed date |
| 2010-04-02 | closed | `will be closed on Friday, April 2, 2010, for Good Friday` | `TSX-REL-2010-03-26` | T1 | the operator's own printed date |
| 2010-05-24 | closed | `will be closed on Monday, May 24, 2010 for Victoria Day` | `TSX-REL-2010-05-17` | T1 | the operator's own printed date |
| 2010-08-02 | closed | `will be closed on Monday, August 2, 2010 for the Civic Holiday` | `TSX-REL-2010-07-27` | T1 | the operator's own printed date |
| 2010-09-06 | closed | `will be closed on Monday, September 6, 2010 for Labour Day` | `TSX-REL-2010-09-01` | T1 | the operator's own printed date |
| 2010-10-11 | closed | `will be closed for the Thanksgiving holiday on Monday, October 11, 2010` | `TSX-REL-2010-10-01` | T1 | the operator's own printed date |
| 2010-12-24 | early close | `Friday, December 24, 2010 Open until 1:00 p.m. (EST)` (TSX/TSXV schedule row) | `TSX-REL-2010-11-08` | T1 | the schedule row is the day's final close, 13:00 Toronto time |
| 2010-12-27 | closed | `Monday, December 27, 2010 (In lieu of Christmas Day) Closed` | `TSX-REL-2010-11-08` | T1 | the operator's own in-lieu print |
| 2010-12-28 | closed | `Tuesday, December 28, 2010 (In lieu of Boxing Day) Closed` | `TSX-REL-2010-11-08` | T1 | the operator's own in-lieu print |

### 2011

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2011-01-03 | closed | `Monday, January 3, 2011 (In lieu of New Year's Day) Closed` | `TSX-REL-2010-11-08` | T1 | the operator's own in-lieu print |
| 2011-02-21 | closed | `will be closed for the Family Day holiday on Monday, February 21` | `TMX-REL-2011-02-16` | T1 | the operator's own printed date; the release names Toronto Stock Exchange |
| 2011-04-22 | closed | `will be closed on Friday, April 22, 2011, for Good Friday` | `TMX-REL-2011-04-14` | T1 | the operator's own printed date |
| 2011-05-23 | closed | `will be closed for the Victoria Day holiday on Monday, May 23, 2011` | `TMX-REL-2011-05-18` | T1 | the operator's own printed date |
| 2011-07-01 | closed | `will be closed on Friday, July 1, 2011 for Canada Day` | `TMX-REL-2011-06-22` | T1 | the operator's own printed date |
| 2011-08-01 | closed | `will be closed on Monday, August 1, 2011 for the Civic Holiday` | `TMX-REL-2011-07-25` | T1 | the operator's own printed date |
| 2011-09-05 | closed | `will be closed on Monday, September 5, 2011 for the Labour Day holiday` | `TMX-REL-2011-08-30` | T1 | the operator's own printed date |
| 2011-10-10 | closed | `will be closed on Monday, October 10, 2011 for the Thanksgiving holiday` | `TMX-REL-2011-09-30` | T1 | the operator's own printed date |

### 2012

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2012-02-20 | closed | `will be closed for Family Day on Monday, February 20, 2012` | `TMX-REL-2012-02-13` | T1 | the operator's own printed date; the release names Toronto Stock Exchange |
| 2012-04-06 | closed | `will be closed on Friday, April 6, 2012 for Good Friday` | `TMX-REL-2012-03-30` | T1 | the operator's own printed date |
| 2012-05-21 | closed | `will be closed on Monday, May 21, 2012 for the Victoria Day holiday` | `TMX-REL-2012-05-14` | T1 | the operator's own printed date |
| 2012-07-02 | closed | `will be closed on Monday, July 2, 2012, for the Canada Day holiday` | `TMX-REL-2012-06-22` | T1 | the operator's own printed date; 1 July fell on a Sunday and the release names the Monday |
| 2012-08-06 | closed | `will be closed on Monday, August 6, 2012 for the Civic Holiday` | `TMX-REL-2012-07-31` | T1 | the operator's own printed date |
| 2012-09-03 | closed | `will be closed on Monday, September 3, 2012 for the Labour Day holiday` | `TMX-REL-2012-08-24` | T1 | the operator's own printed date |
| 2012-10-08 | closed | `will be closed on Monday, October 8, 2012 for the Thanksgiving holiday` | `TMX-REL-2012-09-28` | T1 | the operator's own printed date |
| 2012-12-24 | early close | `Monday, December 24, 2012 Open until 1:00 p.m. (EST)` (TSX/TSXV row; TMX Select's 1:30 p.m. half is a separate book) | `TMX-REL-2012-11-28` | T1 | the schedule row is the day's final close, 13:00 Toronto time |
| 2012-12-25 | closed | `will be closed on Tuesday, December 25, 2012 for Christmas Day` | `TMX-REL-2012-11-28` | T1 | the operator's own printed date |
| 2012-12-26 | closed | `will be closed ... on Wednesday, December 26, 2012 for Boxing Day` | `TMX-REL-2012-11-28` | T1 | the operator's own printed date |

### 2013

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2013-01-01 | closed | `will be closed ... on Tuesday, January 1, 2013 for New Year's Day` | `TMX-REL-2012-11-28` | T1 | the operator's own printed date |
| 2013-02-18 | closed | `will be closed on Monday, February 18, 2013 for Family Day` | `TMX-REL-2013-01-30` | T1 | the operator's own printed date; the release names Toronto Stock Exchange |
| 2013-03-29 | closed | `will be closed on Friday, March 29, 2013 for Good Friday` | `TMX-REL-2013-03-20` | T1 | the operator's own printed date |
| 2013-05-20 | closed | `will be closed on Monday, May 20, 2013 for the Victoria Day holiday` | `TMX-REL-2013-05-13` | T1 | the operator's own printed date |
| 2013-07-01 | closed | `will be closed on Monday, July 1, 2013, for the Canada Day holiday` | `TMX-REL-2013-06-24` | T1 | the operator's own printed date |
| 2013-08-05 | closed | `will be closed on Monday, August 5 2013 for the Civic Holiday` | `TMX-REL-2013-07-26` | T1 | the operator's own printed date |

### 2014

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2014-02-17 | closed | `will be closed on Monday, February 17, 2014 for Family Day` | `TMX-REL-2014-02-07` | T1 | the operator's own printed date; the release names Toronto Stock Exchange |
| 2014-04-18 | closed | `will be closed on Friday, April 18 for Good Friday` | `TMX-REL-2014-04-09` | T1 | the operator's own printed date; 18 April 2014 is the Friday |
| 2014-05-19 | closed | `will be closed on Monday, May 19, 2014 for the Victoria Day holiday` | `TMX-REL-2014-05-13` | T1 | the operator's own printed date |
| 2014-07-01 | closed | `will be closed on Tuesday, July 1, 2014, for the Canada Day holiday` | `TMX-REL-2014-06-23` | T1 | the operator's own printed date |

### 2017

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2017-01-02 | closed | `New Year's Day - Monday, January 2, 2017 *` — `in lieu of New Years Day, Sunday January 1` | `TSX-CAL-2018-09-11` | T1 | the operator's own in-lieu print; trade date is the same civil day |
| 2017-02-20 | closed | `Family Day - Monday, February 20, 2017` | `TSX-CAL-2018-09-11` | T1 | the operator's own printed date |
| 2017-04-14 | closed | `Good Friday - Friday, April 14, 2017` | `TSX-CAL-2018-09-11` | T1 | the operator's own printed date |
| 2017-05-22 | closed | `Victoria Day - Monday, May 22, 2017` | `TSX-CAL-2018-09-11` | T1 | the operator's own printed date |
| 2017-07-03 | closed | `Canada Day - Monday, July 3, 2017 **` — `in lieu of Canada Day, Saturday July 1` | `TSX-CAL-2018-09-11` | T1 | the operator's own in-lieu print |
| 2017-08-07 | closed | `Civic Holiday - Monday, August 7, 2017` | `TSX-CAL-2018-09-11` | T1 | the operator's own printed date |
| 2017-09-04 | closed | `Labour Day - Monday, September 4, 2017` | `TSX-CAL-2018-09-11` | T1 | the operator's own printed date |
| 2017-10-09 | closed | `Thanksgiving Day - Monday, October 9, 2017` | `TSX-CAL-2018-09-11` | T1 | the operator's own printed date |
| 2017-12-25 | closed | `Christmas Day - Monday, December 25, 2017` | `TSX-CAL-2018-09-11` | T1 | the operator's own printed date |
| 2017-12-26 | closed | `Boxing Day - Tuesday, December 26, 2017` | `TSX-CAL-2018-09-11` | T1 | the operator's own printed date; 24 December 2017 was a Sunday and the list prints no Christmas Eve line |

### 2018

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2018-01-01 | closed | `New Year's Day - Monday, January 1, 2018` | `TSX-CAL-2018-09-11` | T1 | the operator's own printed date; corroborated by the 2019-08-20 capture's archive section |
| 2018-02-19 | closed | `Family Day - Monday, February 19, 2018` | `TSX-CAL-2018-09-11` | T1 | the operator's own printed date |
| 2018-03-30 | closed | `Good Friday - Friday, March 30, 2018` | `TSX-CAL-2018-09-11` | T1 | the operator's own printed date |
| 2018-05-21 | closed | `Victoria Day - Monday, May 21, 2018` | `TSX-CAL-2018-09-11` | T1 | the operator's own printed date |
| 2018-07-02 | closed | `Canada Day - Monday, July 2, 2018 *` — `in lieu of Canada Day, Sunday, July 1, 2018` | `TSX-CAL-2018-09-11` | T1 | the operator's own in-lieu print |
| 2018-08-06 | closed | `Civic Holiday - Monday, August 6, 2018` | `TSX-CAL-2018-09-11` | T1 | the operator's own printed date |
| 2018-09-03 | closed | `Labour Day - Monday, September 3, 2018` | `TSX-CAL-2018-09-11` | T1 | the operator's own printed date |
| 2018-10-08 | closed | `Thanksgiving Day - Monday, October 8, 2018` | `TSX-CAL-2018-09-11` | T1 | the operator's own printed date |
| 2018-12-24 | early close | `Markets will close at 1:00 PM on December 24th, 2018.` | `TSX-CAL-2018-09-11` | T1 | the page's own sentence is the day's final close, 13:00 Toronto time; corroborated by the 2019-08-20 capture |
| 2018-12-25 | closed | `Christmas Day - Tuesday, December 25, 2018` | `TSX-CAL-2018-09-11` | T1 | the operator's own printed date |
| 2018-12-26 | closed | `Boxing Day - Wednesday, December 26, 2018` | `TSX-CAL-2018-09-11` | T1 | the operator's own printed date |

### 2019

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2019-01-01 | closed | `New Year's Day - Tuesday, January 1, 2019` | `TSX-CAL-2019-08-20` | T1 | the operator's own printed date; corroborated by the 2020-03-29 capture |
| 2019-02-18 | closed | `Family Day - Monday, February 18, 2019` | `TSX-CAL-2019-08-20` | T1 | the operator's own printed date |
| 2019-04-19 | closed | `Good Friday - Friday, April 19, 2019` | `TSX-CAL-2019-08-20` | T1 | the operator's own printed date |
| 2019-05-20 | closed | `Victoria Day - Monday, May 20, 2019` | `TSX-CAL-2019-08-20` | T1 | the operator's own printed date |
| 2019-07-01 | closed | `Canada Day - Monday, July 1, 2019` | `TSX-CAL-2019-08-20` | T1 | the operator's own printed date |
| 2019-08-05 | closed | `Civic Holiday - Monday, August 5, 2019` | `TSX-CAL-2019-08-20` | T1 | the operator's own printed date |
| 2019-09-02 | closed | `Labour Day - Monday, September 2, 2019` | `TSX-CAL-2019-08-20` | T1 | the operator's own printed date |
| 2019-10-14 | closed | `Thanksgiving Day - Monday, October 14, 2019` | `TSX-CAL-2019-08-20` | T1 | the operator's own printed date |
| 2019-12-24 | early close | `Markets will close at 1:00 PM on December 24th, 2019.` | `TSX-CAL-2019-08-20` | T1 | the page's own sentence; corroborated by the 2020-03-29 capture |
| 2019-12-25 | closed | `Christmas Day - Wednesday, December 25, 2019` | `TSX-CAL-2019-08-20` | T1 | the operator's own printed date |
| 2019-12-26 | closed | `Boxing Day - Thursday, December 26, 2019` | `TSX-CAL-2019-08-20` | T1 | the operator's own printed date |

### 2020

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2020-01-01 | closed | `New Year's Day - Wednesday, January 1, 2020` | `TSX-CAL-2020-03-29` | T1 | the operator's own printed date; restated by the 2021-01-25 capture |
| 2020-02-17 | closed | `Family Day - Monday, February 17, 2020` | `TSX-CAL-2020-03-29` | T1 | the operator's own printed date |
| 2020-04-10 | closed | `Good Friday - Friday, April 10, 2020` | `TSX-CAL-2020-03-29` | T1 | the operator's own printed date |
| 2020-05-18 | closed | `Victoria Day - Monday, May 18, 2020` | `TSX-CAL-2020-03-29` | T1 | the operator's own printed date |
| 2020-07-01 | closed | `Canada Day - Wednesday, July 1, 2020` | `TSX-CAL-2020-03-29` | T1 | the operator's own printed date |
| 2020-08-03 | closed | `Civic Holiday - Monday, August 3, 2020` | `TSX-CAL-2020-03-29` | T1 | the operator's own printed date |
| 2020-09-07 | closed | `Labour Day - Monday, September 7, 2020` | `TSX-CAL-2020-03-29` | T1 | the operator's own printed date |
| 2020-10-12 | closed | `Thanksgiving Day - Monday, October 12, 2020` | `TSX-CAL-2020-03-29` | T1 | the operator's own printed date |
| 2020-12-24 | early close | `Markets will close at 1:00 PM on December 24th, 2020.` | `TSX-CAL-2020-03-29` | T1 | the page's own sentence; restated by the 2021-01-25 capture |
| 2020-12-25 | closed | `Christmas Day - Friday, December 25, 2020` | `TSX-CAL-2020-03-29` | T1 | the operator's own printed date |
| 2020-12-28 | closed | `In Lieu of Boxing Day - Monday, December 28, 2020` | `TSX-CAL-2020-03-29` | T1 | the operator's own in-lieu print |

### 2021

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2021-01-01 | closed | `New Year's Day - Friday January 1, 2021` | `TSX-CAL-2021-01-25` | T1 | the operator's own printed date; corroborated by the 2022-01-28 capture's archive section |
| 2021-02-15 | closed | `Family Day - Monday, February 15, 2021` | `TSX-CAL-2021-01-25` | T1 | the operator's own printed date |
| 2021-04-02 | closed | `Good Friday - Friday, April 2, 2021` | `TSX-CAL-2021-01-25` | T1 | the operator's own printed date |
| 2021-05-24 | closed | `Victoria Day - Monday, May 24, 2021` | `TSX-CAL-2021-01-25` | T1 | the operator's own printed date |
| 2021-07-01 | closed | `Canada Day - Thursday, July 1, 2021` | `TSX-CAL-2021-01-25` | T1 | the operator's own printed date |
| 2021-08-02 | closed | `Civic Holiday - Monday, August 2, 2021` | `TSX-CAL-2021-01-25` | T1 | the operator's own printed date |
| 2021-09-06 | closed | `Labour Day - Monday, September 6, 2021` | `TSX-CAL-2021-01-25` | T1 | the operator's own printed date |
| 2021-10-11 | closed | `Thanksgiving Day - Monday, October 11, 2021` | `TSX-CAL-2021-01-25` | T1 | the operator's own printed date |
| 2021-12-24 | early close | `Christmas Eve - Friday, December 24, 2021 **` — `TSX and TSX Venture Exchange will close early at 1:00 p.m. and TSX Alpha Exchange will close at 1:30 p.m.` | `TSX-CAL-2022-01-28` | T1 | the January capture printed the half day `subject to Board Approval`; this later state of the page witnesses the discharged condition, so the row keys to it and the discharge is recorded here |
| 2021-12-27 | closed | `In Lieu of Christmas Day - Monday, December 27, 2021` | `TSX-CAL-2021-01-25` | T1 | the operator's own in-lieu print |
| 2021-12-28 | closed | `In Lieu of Boxing Day - Tuesday, December 28, 2021` | `TSX-CAL-2021-01-25` | T1 | the operator's own in-lieu print |

### 2022

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2022-01-03 | closed | `In Lieu of New Year's Day - Monday, January 3, 2022` | `TSX-CAL-2022-01-28` | T1 | the operator's own in-lieu print; restated by the 2023-01-16 capture |
| 2022-02-21 | closed | `Family Day - Monday, February 21, 2022` | `TSX-CAL-2022-01-28` | T1 | the operator's own printed date |
| 2022-04-15 | closed | `Good Friday - Friday, April 15, 2022` | `TSX-CAL-2022-01-28` | T1 | the operator's own printed date |
| 2022-05-23 | closed | `Victoria Day - Monday, May 23, 2022` | `TSX-CAL-2022-01-28` | T1 | the operator's own printed date |
| 2022-07-01 | closed | `Canada Day - Friday, July 1, 2022` | `TSX-CAL-2022-01-28` | T1 | the operator's own printed date |
| 2022-08-01 | closed | `Civic Holiday - Monday, August 1, 2022` | `TSX-CAL-2022-01-28` | T1 | the operator's own printed date |
| 2022-09-05 | closed | `Labour Day - Monday, September 5, 2022` | `TSX-CAL-2022-01-28` | T1 | the operator's own printed date |
| 2022-10-10 | closed | `Thanksgiving Day - Monday, October 10, 2022` | `TSX-CAL-2022-01-28` | T1 | the operator's own printed date |
| 2022-12-26 | closed | `In Lieu of Christmas Day - Monday, December 26, 2022` | `TSX-CAL-2022-01-28` | T1 | the operator's own in-lieu print; 24 December was a Saturday and the list prints no Christmas Eve line |
| 2022-12-27 | closed | `In Lieu of Boxing Day - Tuesday, December 27, 2022` | `TSX-CAL-2022-01-28` | T1 | the operator's own in-lieu print |

### 2023

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2023-01-02 | closed | `In Lieu of New Year's Day - Monday, January 2, 2023` | `TSX-CAL-2023-01-16` | T1 | the operator's own in-lieu print; restated by the 2023-12-15 capture |
| 2023-02-20 | closed | `Family Day - Monday, February 20, 2023` | `TSX-CAL-2023-01-16` | T1 | the operator's own printed date |
| 2023-04-07 | closed | `Good Friday - Friday, April 7, 2023` | `TSX-CAL-2023-01-16` | T1 | the operator's own printed date |
| 2023-05-22 | closed | `Victoria Day - Monday, May 22, 2023` | `TSX-CAL-2023-01-16` | T1 | the operator's own printed date |
| 2023-07-03 | closed | `Canada Day - Monday, July 3, 2023` | `TSX-CAL-2023-01-16` | T1 | the operator's own printed date |
| 2023-08-07 | closed | `Civic Holiday - Monday, August 7, 2023` | `TSX-CAL-2023-01-16` | T1 | the operator's own printed date |
| 2023-09-04 | closed | `Labour Day - Monday, September 4, 2023` | `TSX-CAL-2023-01-16` | T1 | the operator's own printed date |
| 2023-10-09 | closed | `Thanksgiving Day - Monday, October 9, 2023` | `TSX-CAL-2023-01-16` | T1 | the operator's own printed date |
| 2023-12-25 | closed | `Christmas Day - Monday, December 25, 2023` | `TSX-CAL-2023-01-16` | T1 | the operator's own printed date; 24 December was a Sunday and the list prints no Christmas Eve line |
| 2023-12-26 | closed | `Boxing Day - Tuesday, December 26, 2023` | `TSX-CAL-2023-01-16` | T1 | the operator's own printed date |

### 2024

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2024-01-01 | closed | `New Year's Day - Monday, January 1, 2024` | `TSX-CAL-2024-12-17` | T1 | the operator's own printed date; the July 2024 state prints the same closure under the label `In Lieu of New Year's Day` |
| 2024-02-19 | closed | `Family Day - Monday, February 19, 2024` | `TSX-CAL-2024-12-17` | T1 | the operator's own printed date; corroborated by the 2025-01-24 capture |
| 2024-03-29 | closed | `Good Friday - Friday, March 29, 2024` | `TSX-CAL-2024-12-17` | T1 | the operator's own printed date |
| 2024-05-20 | closed | `Victoria Day - Monday, May 20, 2024` | `TSX-CAL-2024-12-17` | T1 | the operator's own printed date |
| 2024-07-01 | closed | `Canada Day - Monday, July 1, 2024` | `TSX-CAL-2024-12-17` | T1 | the operator's own printed date |
| 2024-08-05 | closed | `Civic Holiday - Monday, August 5, 2024` | `TSX-CAL-2024-12-17` | T1 | the operator's own printed date |
| 2024-09-02 | closed | `Labour Day - Monday, September 2, 2024` | `TSX-CAL-2024-12-17` | T1 | the operator's own printed date |
| 2024-10-14 | closed | `Thanksgiving Day - Monday, October 14, 2024` | `TSX-CAL-2024-12-17` | T1 | the operator's own printed date |
| 2024-12-24 | early close | `Christmas Eve - Tuesday, December 24th, 2024*` — `* Closing at 1:00 PM (TSX/TSXV) and 1:30 (ALPHA/ALPHA X/DRK)` | `TSX-CAL-2024-12-17` | T1 | the July states printed the list without this row; the December state adds it, so the row keys to the December capture |
| 2024-12-25 | closed | `Christmas Day - Wednesday, December 25, 2024` | `TSX-CAL-2024-12-17` | T1 | the operator's own printed date |
| 2024-12-26 | closed | `Boxing Day - Thursday, December 26, 2024` | `TSX-CAL-2024-12-17` | T1 | the operator's own printed date |
### 2025

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2025-01-01 | closed | `New Year's Day - Wednesday, January 1, 2025` | `TSX-CAL-2026-09-28` | T1 | the operator's own printed date; trade date is the same civil day |
| 2025-02-17 | closed | `Family Day - Monday, February 17, 2025` | `TSX-CAL-2026-09-28` | T1 | the operator's own printed date; trade date is the same civil day |
| 2025-04-18 | closed | `Good Friday - Friday, April 18, 2025` | `TSX-CAL-2026-09-28` | T1 | the operator's own printed date; trade date is the same civil day |
| 2025-05-19 | closed | `Victoria Day - Monday, May 19, 2025` | `TSX-CAL-2026-09-28` | T1 | the operator's own printed date; trade date is the same civil day |
| 2025-07-01 | closed | `Canada Day - Tuesday, July 1, 2025` | `TSX-CAL-2026-09-28` | T1 | the operator's own printed date; trade date is the same civil day |
| 2025-08-04 | closed | `Civic Holiday - Monday, August 4, 2025` | `TSX-CAL-2026-09-28` | T1 | the operator's own printed date; trade date is the same civil day |
| 2025-09-01 | closed | `Labour Day - Monday, September 1, 2025` | `TSX-CAL-2026-09-28` | T1 | the operator's own printed date; trade date is the same civil day |
| 2025-10-13 | closed | `Thanksgiving Day - Monday, October 13, 2025` | `TSX-CAL-2026-09-28` | T1 | the operator's own printed date; trade date is the same civil day |
| 2025-12-24 | early close | `Christmas Eve - Wednesday, December 24, 2025*` — `* Closing at 1:00 PM (TSX/TSXV) and 1:30 (ALPHA/ALPHA X/DRK)` | `TSX-CAL-2026-09-28` | T1 | the printed event date; the TSX/TSXV half of the footnote is the day's final close, 13:00 Toronto time |
| 2025-12-25 | closed | `Christmas Day - Thursday, December 25, 2025` | `TSX-CAL-2026-09-28` | T1 | the operator's own printed date; trade date is the same civil day |
| 2025-12-26 | closed | `Boxing Day - Friday, December 26, 2025` | `TSX-CAL-2026-09-28` | T1 | the operator's own printed date; trade date is the same civil day |

### 2026

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2026-01-01 | closed | `New Year's Day - Thursday, January 1, 2026` | `TSX-CAL-2026-09-28` | T1 | the operator's own printed date; trade date is the same civil day |
| 2026-02-16 | closed | `Family Day - Monday, February 16, 2026` | `TSX-CAL-2026-09-28` | T1 | the operator's own printed date; trade date is the same civil day |
| 2026-04-03 | closed | `Good Friday - Friday, April 3, 2026` | `TSX-CAL-2026-09-28` | T1 | the operator's own printed date; trade date is the same civil day |
| 2026-05-18 | closed | `Victoria Day - Monday, May 18, 2026` | `TSX-CAL-2026-09-28` | T1 | the operator's own printed date; trade date is the same civil day |
| 2026-07-01 | closed | `Canada Day - Wednesday, July 1, 2026` | `TSX-CAL-2026-09-28` | T1 | the operator's own printed date; trade date is the same civil day |
| 2026-08-03 | closed | `Civic Holiday - Monday, August 3, 2026` | `TSX-CAL-2026-09-28` | T1 | the operator's own printed date; trade date is the same civil day |
| 2026-09-07 | closed | `Labour Day - Monday, September 7, 2026` | `TSX-CAL-2026-09-28` | T1 | the operator's own printed date; trade date is the same civil day |
| 2026-10-12 | closed | `Thanksgiving Day - Monday, October 12, 2026` | `TSX-CAL-2026-09-28` | T1 | the operator's own printed date; trade date is the same civil day |
| 2026-12-24 | early close | `Christmas Eve - Thursday, December 24, 2026*` — `* Closing at 1:00 PM (TSX/TSXV) and 1:30 (ALPHA/ALPHA X/DRK)` | `TSX-CAL-2026-09-28` | T1 | the printed event date; the TSX/TSXV half of the footnote is the day's final close, 13:00 Toronto time |
| 2026-12-25 | closed | `Christmas Day - Friday, December 25, 2026` | `TSX-CAL-2026-09-28` | T1 | the operator's own printed date; trade date is the same civil day |
| 2026-12-28 | closed | `In Lieu of Boxing Day - Monday, December 28, 2026` | `TSX-CAL-2026-09-28` | T1 | the operator's own in-lieu print (Boxing Day falls on the Saturday); trade date is the same civil day |

### Documents

| Document | Window | Replay or service URL | Capture or retrieval, UTC | Tier | sha256 |
|---|---|---|---|---|---|
| `TSX-REL-2009-12-02` | 2010-01-01 (the 2009 year-end schedule; the 2010 New Year row) | <https://web.archive.org/web/20100102012424id_/http://tmx.com/en/news_events/news_releases/12-2-2009_TSX-HolidaySchedule.html> | Wayback `id_` replay of capture `20100102012424`, retrieved 2026-09-30T01:08:53Z | T1 | `dbdb1446a5d181939826645009031d1458006a1b66a98870ce8c1f3ac4c73476` |
| `TSX-REL-2010-02-08` | 2010-02-15 | <https://web.archive.org/web/20101201061455id_/http://tmx.com/en/news_events/news_releases/2-8-2010_TSX-FamilyDay.html> | Wayback `id_` replay of capture `20101201061455`, retrieved 2026-09-30T01:13:06Z | T1 | `99c92378efb0599618876aae83a900e14986e5b2d947752e4024649caeff389e` |
| `TSX-REL-2010-03-26` | 2010-04-02 | <https://web.archive.org/web/20101201055224id_/http://tmx.com/en/news_events/news_releases/3-26-2010_TSX-GoodFriday.html> | Wayback `id_` replay of capture `20101201055224`, retrieved 2026-09-30T01:13:12Z | T1 | `c55f771c8be44d1a7b437186e9b936beb1d4f5e4989a5a15a068c57a9b0eaa8f` |
| `TSX-REL-2010-05-17` | 2010-05-24 | <https://web.archive.org/web/20101201061525id_/http://tmx.com/en/news_events/news_releases/5-17-2010_TSX-VictoriaDayHoliday.html> | Wayback `id_` replay of capture `20101201061525`, retrieved 2026-09-30T01:13:15Z | T1 | `14f5e7a4307066f6f1f56719e039761e13ef2b277951da9a5485e3ef59a3773d` |
| `TSX-REL-2010-07-27` | 2010-08-02 | <https://web.archive.org/web/20101201060128id_/http://tmx.com/en/news_events/news_releases/7-27-2010_TSX-CivicHoliday.html> | Wayback `id_` replay of capture `20101201060128`, retrieved 2026-09-30T01:13:17Z | T1 | `cd0a65f0c297831f3661c7dfd2ad3723ef18e8dc4cfa0f1cb43b71fc401af262` |
| `TSX-REL-2010-09-01` | 2010-09-06 | <https://web.archive.org/web/20101201055935id_/http://tmx.com/en/news_events/news_releases/9-1-2010_TSX-LabourDay.html> | Wayback `id_` replay of capture `20101201055935`, retrieved 2026-09-30T01:13:20Z | T1 | `229e7ba92355ee6b1eb00615e99342389b6959ed740c83cc5cad7e5e64bbae0f` |
| `TSX-REL-2010-10-01` | 2010-10-11 | <https://web.archive.org/web/20101201055415id_/http://tmx.com/en/news_events/news_releases/10-1-2010_TSX-Thanksgiving.html> | Wayback `id_` replay of capture `20101201055415`, retrieved 2026-09-30T01:13:23Z | T1 | `c4bf7babbcfcfe09ce89a3e516e011cf7f9a2e5d6423ff5ea62c3a340645a5f1` |
| `TSX-REL-2010-11-08` | 2010-12-24 .. 2011-01-03 (the year-end schedule and the 2011 New Year in-lieu) | <https://web.archive.org/web/20101201004014id_/http://tmx.com/en/news_events/news_releases/11-8-2010_TSX-HolidaySchedule.html> | Wayback `id_` replay of capture `20101201004014`, retrieved 2026-09-30T01:08:57Z | T1 | `8b54f5bf49aa6c1bdc71d06627ffbfa97504f0533c55fa1f70ca6924188835b0` |
| `TMX-REL-2011-02-16` | 2011-02-21 | <https://web.archive.org/web/20111010045533id_/http://tmx.com/en/news_events/news/news_releases/2011/2-16-2011_TMXGroup-familyday.html> | Wayback `id_` replay of capture `20111010045533`, retrieved 2026-09-30T01:13:50Z | T1 | `77b2fa392b5806d04914dcbeb4acf949db17e448b8b61e4be6aa06d6cedd33f1` |
| `TMX-REL-2011-04-14` | 2011-04-22 | <https://web.archive.org/web/20111010050037id_/http://tmx.com/en/news_events/news/news_releases/2011/4-14-2011_TMXGroup-goodfriday.html> | Wayback `id_` replay of capture `20111010050037`, retrieved 2026-09-30T01:13:53Z | T1 | `fd09b2180bcaa1ed4650586f40f9c4115880edbeb1adee682f2c9947d0dad71f` |
| `TMX-REL-2011-05-18` | 2011-05-23 | <https://web.archive.org/web/20111010030017id_/http://tmx.com/en/news_events/news/news_releases/2011/5-18-2011_TMXGroup-victoriaday.html> | Wayback `id_` replay of capture `20111010030017`, retrieved 2026-09-30T01:13:55Z | T1 | `560a7795ec488db2c343ecf82df6ec22a089f742fbedca93cf17e6833df26887` |
| `TMX-REL-2011-06-22` | 2011-07-01 | <https://web.archive.org/web/20111010023004id_/http://tmx.com/en/news_events/news/news_releases/2011/6-22-2011_TMXGroup-canada_day_june11.html> | Wayback `id_` replay of capture `20111010023004`, retrieved 2026-09-30T01:13:58Z | T1 | `d91efe4cce21c18fbe42472a336e4e170c2cdfbb5de1d72c3f96700c6c40c451` |
| `TMX-REL-2011-07-25` | 2011-08-01 | <https://web.archive.org/web/20110814063225id_/http://www.tmx.com/en/news_events/news/news_releases/2011/7-25-2011_TMXGroup-closed_civic_holiday.html> | Wayback `id_` replay of capture `20110814063225`, retrieved 2026-09-30T01:14:01Z | T1 | `deb2f47e5681a619026cec6fd8f1cae7f8befb4de5b98017dae10b91d6930b75` |
| `TMX-REL-2011-08-30` | 2011-09-05 | <https://web.archive.org/web/20111010050329id_/http://tmx.com/en/news_events/news/news_releases/2011/8-30-2011_TMXGroup-ClosedLabourDay.html> | Wayback `id_` replay of capture `20111010050329`, retrieved 2026-09-30T01:14:04Z | T1 | `03f58237dc628ac8380c214d7fe8805ebe6783dd3be57f142f14e1cd7eb87381` |
| `TMX-REL-2011-09-30` | 2011-10-10 | <https://web.archive.org/web/20111010050414id_/http://tmx.com/en/news_events/news/news_releases/2011/9-30-2011_TMXGroup-thanksgiving_closure.html> | Wayback `id_` replay of capture `20111010050414`, retrieved 2026-09-30T01:14:06Z | T1 | `08b9c25f4cfbddc0172d118d4d9e74e338a5c94b254e21be601b8fbeb84ca2d1` |
| `TMX-REL-2012-02-13` | 2012-02-20 | <https://web.archive.org/web/20120519053604id_/http://www.tmx.com/en/news_events/news/news_releases/2012/2-13-2012_TMXGroup-family_day_closed.html> | Wayback `id_` replay of capture `20120519053604`, retrieved 2026-09-30T01:14:22Z | T1 | `3511f55c6985caddb8b989bbeeb49b74ab82bcc25c6cead792b0672eb6100d3a` |
| `TMX-REL-2012-03-30` | 2012-04-06 | <https://web.archive.org/web/20120519054000id_/http://www.tmx.com/en/news_events/news/news_releases/2012/3_30_2012_TMX_market_closed_good_friday.html> | Wayback `id_` replay of capture `20120519054000`, retrieved 2026-09-30T01:14:24Z | T1 | `ac7ff82fb169b8321b3a879d1d3b2199f1a8cfb7a03ac0485e567ad5f6ed265e` |
| `TMX-REL-2012-05-14` | 2012-05-21 | <https://web.archive.org/web/20120920075441id_/http://tmx.com/en/news_events/news/news_releases/2012/5-14-2012_TMXGroup-closed_victoria_day.html> | Wayback `id_` replay of capture `20120920075441`, retrieved 2026-09-30T01:14:27Z | T1 | `f001d2ded95adc971db430accf42005f199b63c08d9cfd9b17c96d141586d343` |
| `TMX-REL-2012-06-22` | 2012-07-02 | <https://web.archive.org/web/20120920080159id_/http://tmx.com/en/news_events/news/news_releases/2012/6-22-2012_TMXGroup-closed_canada_day.html> | Wayback `id_` replay of capture `20120920080159`, retrieved 2026-09-30T01:14:30Z | T1 | `6b1e110ed0566e18e7ebdfe813c074905d5aecc9ddac9e724d2280e85a667c78` |
| `TMX-REL-2012-07-31` | 2012-08-06 | <https://web.archive.org/web/20120920075800id_/http://tmx.com/en/news_events/news/news_releases/2012/7-31-2012_TMXGroup-CivicHoliday.html> | Wayback `id_` replay of capture `20120920075800`, retrieved 2026-09-30T01:14:34Z | T1 | `7254d09fac43077b21e748c180a1441d1b909163c0304c2b9d28a6d2e16b2849` |
| `TMX-REL-2012-08-24` | 2012-09-03 | <https://web.archive.org/web/20120920080314id_/http://tmx.com/en/news_events/news/news_releases/2012/8-24-2012_TMXGroup-LabourDay.html> | Wayback `id_` replay of capture `20120920080314`, retrieved 2026-09-30T01:14:37Z | T1 | `0dacf78717bbf90dc1e95172e9983da5b6453fc898e747cb30fd211d29b34972` |
| `TMX-REL-2012-09-28` | 2012-10-08 | <https://web.archive.org/web/20121102153750id_/http://www.tmx.com/en/news_events/news/news_releases/2012/9-28-2012_TMXGroup-Thanksgiving.html> | Wayback `id_` replay of capture `20121102153750`, retrieved 2026-09-30T01:14:40Z | T1 | `8a8f792d6571d9757751e386627b84318113588ef04cdae868abd038079bd79e` |
| `TMX-REL-2012-11-28` | 2012-12-24 .. 2013-01-01 (the year-end operating schedule and the 2013 New Year row) | <https://web.archive.org/web/20121211200307id_/http://tmx.com/en/news_events/news/news_releases/2012/11-28-2012_TMXGroup-TMX-Group-Holiday-Operating-Schedule.html> | Wayback `id_` replay of capture `20121211200307`, retrieved 2026-09-30T01:09:01Z | T1 | `87c4561409393d7af23b022310d510aa1be832124d822417f70a0125b7dff4e3` |
| `TMX-REL-2013-01-30` | 2013-02-18 | <https://web.archive.org/web/20130818173444id_/http://tmx.com/en/news_events/news/news_releases/2013/1-30-2013_TMXGroup-family-day.html> | Wayback `id_` replay of capture `20130818173444`, retrieved 2026-09-30T01:16:19Z | T1 | `489abb206bc91beff137a2ff245ff4bb5a66bd402ae4ce0a9ae057cabd4f0a45` |
| `TMX-REL-2013-03-20` | 2013-03-29 | <https://web.archive.org/web/20130818193156id_/http://tmx.com/en/news_events/news/news_releases/2013/3-20-0-2013_TMXGroup-GoodFridayMarketClosed.html> | Wayback `id_` replay of capture `20130818193156`, retrieved 2026-09-30T01:16:19Z | T1 | `c9602651e5a7c17ebfca12c218c94bb5996b4ddbf42c67724245f4cc1a3c1b7c` |
| `TMX-REL-2013-05-13` | 2013-05-20 | <https://web.archive.org/web/20130818195457id_/http://tmx.com/en/news_events/news/news_releases/2013/5-13-2013_TMXGroup-VictoriaDay.html> | Wayback `id_` replay of capture `20130818195457`, retrieved 2026-09-30T01:16:36Z | T1 | `fab3414560d151534a4f29afd4ed9b2c54f34d133dbf952eb3e69a96265daa2f` |
| `TMX-REL-2013-06-24` | 2013-07-01 | <https://web.archive.org/web/20130818165636id_/http://tmx.com/en/news_events/news/news_releases/2013/6-24-2013_TMXGroup-CanadaDay.html> | Wayback `id_` replay of capture `20130818165636`, retrieved 2026-09-30T01:16:36Z | T1 | `36986092a2786c7ca8c39e9783bbede8ce023b55248ed1a63d8f76e7ee1520b7` |
| `TMX-REL-2013-07-26` | 2013-08-05 | <https://web.archive.org/web/20130806002842id_/http://tmx.com/en/news_events/news/news_releases/2013/7-26-2013_TMXGroup-Civic-Holiday.html> | Wayback `id_` replay of capture `20130806002842`, retrieved 2026-09-30T01:16:37Z | T1 | `31cc2e04eb09a17e3ef0bd9efc01c3061fd955d7337cee85ecc76c88f17df050` |
| `TMX-REL-2014-02-07` | 2014-02-17 | <https://web.archive.org/web/20140209142427id_/http://tmx.com/en/news_events/news/news_releases/2014/02-07-2014_TMXGroup-ClosedFamilyDay.html> | Wayback `id_` replay of capture `20140209142427`, retrieved 2026-09-30T01:16:49Z | T1 | `9e74fd275dc50fa4c0b07f7047907232afee9d69a54a99c8491f70bf75ef2275` |
| `TMX-REL-2014-04-09` | 2014-04-18 | <https://web.archive.org/web/20140714210509id_/http://tmx.com/en/news_events/news/news_releases/2014/04-09-2014_TMXGroup-GoodFriday.html> | Wayback `id_` replay of capture `20140714210509`, retrieved 2026-09-30T01:17:05Z | T1 | `5088e2cd4742fa79a404d8a4180c6a50e508cfa714a31de672b89354c847963d` |
| `TMX-REL-2014-05-13` | 2014-05-19 | <https://web.archive.org/web/20140714225156id_/http://tmx.com/en/news_events/news/news_releases/2014/05-13-2014_TMXGroup-VictoriaDay.html> | Wayback `id_` replay of capture `20140714225156`, retrieved 2026-09-30T01:17:08Z | T1 | `2cb61d1d1ef0159d5a8ed0d39af5809c92b66fda945a0fc451019924a1913d99` |
| `TMX-REL-2014-06-23` | 2014-07-01 | <https://web.archive.org/web/20140629102827id_/http://tmx.com/en/news_events/news/news_releases/2014/06-23-2014_TMXGroup-CanadaDayClosing.html> | Wayback `id_` replay of capture `20140629102827`, retrieved 2026-09-30T01:17:09Z | T1 | `737155026e5e036501b83723503d91378a8fc3afe36c0c7b74fa150c59c37cf1` |
| `TSX-CAL-2018-09-11` | 2017-01-02 .. 2018-12-26 (the 2018 list and the 2017 archive restatement) | <https://web.archive.org/web/20180911094940id_/https://www.tsx.com/trading/calendars-and-trading-hours/calendar> | Wayback `id_` replay of capture `20180911094940`, retrieved 2026-09-29T07:15:33Z | T1 | `37d3840568474ae4cb6087fbdec04f8b51efadb4c31e852bdebda82da722ac64` |
| `TSX-CAL-2019-08-20` | 2019-01-01 .. 2019-12-26 (the 2019 list; the 2018 list corroborated) | <https://web.archive.org/web/20190820094447id_/https://www.tsx.com/trading/calendars-and-trading-hours/calendar> | Wayback `id_` replay of capture `20190820094447`, retrieved 2026-09-29T07:15:34Z | T1 | `ad37450736210f7a61146262080d049edcc03c45126c039a21aeefee0930b455` |
| `TSX-CAL-2020-03-29` | 2019-01-01 .. 2020-12-28 (the 2019 list corroborated; the 2020 list) | <https://web.archive.org/web/20200329124856id_/https://www.tsx.com/trading/calendars-and-trading-hours/calendar> | Wayback `id_` replay of capture `20200329124856`, retrieved 2026-09-29T07:15:35Z | T1 | `5bf587ca7151937f62038caf52393b596b79becc7b219e0b8312ee77a93281bc` |
| `TSX-CAL-2021-01-25` | 2020-01-01 .. 2021-12-28 (the 2020 list corroborated; the 2021 list with the conditional half-day sentence) | <https://web.archive.org/web/20210125185115id_/https://www.tsx.com/trading/calendars-and-trading-hours/calendar> | Wayback `id_` replay of capture `20210125185115`, retrieved 2026-09-29T07:15:36Z | T1 | `403bb510bb4f0e7f2b1e95ac3cda02635fb5e8ca98e3feb234eaf38f37fac11f` |
| `TSX-CAL-2022-01-28` | 2021-12-24 and 2022-01-03 .. 2022-12-27 (the discharged 2021 half day; the 2022 list; the 2021 list corroborated) | <https://web.archive.org/web/20220128061626id_/https://www.tsx.com/trading/calendars-and-trading-hours/calendar> | Wayback `id_` replay of capture `20220128061626`, retrieved 2026-09-29T07:15:37Z | T1 | `31201ebfadab3d971d3bd81e993a6889811ee6ddb1fcc5d395b863bfaa0d9342` |
| `TSX-CAL-2023-01-16` | 2022-01-03 .. 2023-12-26 (the 2022 list corroborated; the 2023 list) | <https://web.archive.org/web/20230116070339id_/https://www.tsx.com/trading/calendars-and-trading-hours/calendar> | Wayback `id_` replay of capture `20230116070339`, retrieved 2026-09-29T07:15:38Z | T1 | `6c5aeb7b127247bf48d8a3caa63db770d9e3fa74458f5fb22555dc3708d15ac3` |
| `TSX-CAL-2023-12-15` | 2023-01-02 .. 2024-12-26 (the 2023 list corroborated; the pre-eve 2024 list) | <https://web.archive.org/web/20231215123703id_/https://www.tsx.com/trading/calendars-and-trading-hours/calendar> | Wayback `id_` replay of capture `20231215123703`, retrieved 2026-09-29T07:15:39Z | T1 | `560a04b11b3dfe733142022373ea75b2952d954b7ccc08e0f0303f60740f9652` |
| `TSX-CAL-2024-07-23` | 2024-01-01 .. 2024-12-26 (the pre-eve 2024 list) | <https://web.archive.org/web/20240723084106id_/https://www.tsx.com/en/trading/calendars-and-trading-hours/calendar> | Wayback `id_` replay of capture `20240723084106`, retrieved 2026-09-29T07:15:56Z | T1 | `5739e5d2d1a1a73bad33ade6c7af443e9a9f3c1a1d996486775408e2f9dee52b` |
| `TSX-CAL-2024-12-17` | 2024-01-01 .. 2025-12-31 (the updated 2024 list with the Christmas Eve row; the 2025 list) | <https://web.archive.org/web/20241217084235id_/https://www.tsx.com/en/trading/calendars-and-trading-hours/calendar> | Wayback `id_` replay of capture `20241217084235`, retrieved 2026-09-29T07:16:12Z | T1 | `02c0cc75ea2fae867884d53fa6b237b7dcfea10509dbc3b9707e06658096eff8` |
| `TSX-CAL-2025-01-24` | 2024-01-01 .. 2025-12-31 (both lists corroborated) | <https://web.archive.org/web/20250124071909id_/https://www.tsx.com/en/trading/calendars-and-trading-hours/calendar> | Wayback `id_` replay of capture `20250124071909`, retrieved 2026-09-29T07:16:27Z | T1 | `e327500acbb2d9d5da49478c8f1ec8242eba10324624f5ea2d72af52cbb861b9` |
| `TSX-CAL-2026-09-28` | 2025-01-01 .. 2026-12-31 | <https://www.tsx.com/en/trading/calendars-and-trading-hours/calendar> | retrieved 2026-09-28 01:06 | T1 | `983d3108683e59f3b6045ffd862e699113316a14960f54880ef33a627718e7d2` |

`TSX-CAL-2019-08-20`, `TSX-CAL-2020-03-29`, `TSX-CAL-2021-01-25`, `TSX-CAL-2023-01-16`,
`TSX-CAL-2023-12-15`, `TSX-CAL-2024-07-23` and `TSX-CAL-2025-01-24` corroborate the captures
before them on every shared row and key no row of their own beyond what the tables above
assign. The store's `holidays/raw/equities/tsx/2025-2027/` holds the live artifact with its
own index, `holidays/raw/equities/tsx/2010-2024/` holds the ten 2017-2025 captures above
(their sha256s in `SHA256SUMS.txt`, their retrieval stamps in that directory's `INDEX.md`),
and `holidays/raw/equities/tsx/cdx-retry-2026-09-30/` holds the 2026-09-30 sweep outputs and
the thirty-two release replays above with the same sha and index discipline.

## Gaps and residual risks

- **Interpretive step, order-entry classification.** The 07:00–09:30 Pre-Open is `order_entry`: no trade can match inside it, and the first print of the day is the 09:30 Market-on-Open cross that starts continuous trading.
- **Interpretive step, conditional extension.** The Price Movement Extension rule is modelled as the venue's maximum envelope. On ordinary days and symbols the 16:00–16:10 interval is cancel-only, but when the extension fires it is the delayed Market-on-Close cross for that symbol and it prints, so the window is not order-entry-only. The separate 16:10–16:15 Post Market Cancel Session is not modelled at all.
- **Interpretive step, the Christmas Eve half.** The footnote states two closes — `1:00 PM (TSX/TSXV) and 1:30 (ALPHA/ALPHA X/DRK)`. The row encodes the TSX/TSXV 13:00 close because this identity is the Toronto Stock Exchange cash-equity venue; the ALPHA/ALPHA X/DRK book systems are separate order books, not part of this row's scope, and no TSX-listed session is clipped by their later close.
- **No dated revision.** The reviewed grid holds for the whole audit window, so every instant resolves to the one profile. A sourced revision later replaces this with a real timeline row and needs no routing change. Closing condition for a future change: a TMX notice stating an unconditional day-level effective date.
- **Holiday coverage gaps inside 2011-2016 (tracked as [#221](https://github.com/SharurTrading/exchange-hours-rs/issues/221)).** The 2026-09-30 UTC sweeps (CDX service working; outputs in the store's `cdx-retry-2026-09-30/` directory) recovered 2010-2014 as far as the operator's own per-holiday news releases reach, but three spans survive in no operator artifact: **2011-10-11..2012-01-02** — the 2011 year-end arrangement and the 2012 New Year in-lieu rode releases the archive never captured (the all-time `news_releases` sweep of `tmx.com` holds no November-December 2011 or January 2012 holiday release; the 2011 news tree's last witnessed state is the 2011-10-10 bulk catch) — then **2013-08-19..2014-01-01** (no 2013 Labour Day, Thanksgiving or Christmas release and no 2014 New Year's Day release survives; the 2013 tree's last witnessed state is the 2013-08-18 bulk catch, and the 2014 New Year announcement would have been a 2013-published release outside the 2014 tree sweep) and **2014-07-02..2016-12-31** — the release practice stops in the archive after the 2014-06-23 Canada Day notice, and the `tsx.com`-era site of 2015-2016 carries no holiday page capture at all (the domain-wide `holiday` filter of `tsx.com` 2010-2016 returns zero url keys). Queries inside those spans refuse rather than answer. Residual risk: a release could have been published and never captured — the audited-normal claims inside the windows reach exactly as far as the witnessed bulk sweeps named above. Closing conditions: a capture of any of the missing releases or of any 2015-2016 TSX/TMX holiday page, or the operator's re-publication of a historical calendar.
- **Holiday horizon.** The audited holiday window stops at 2026-12-31 because the operator has published nothing past it (verified 2026-09-28). Closing condition: TMX's 2027 calendar section; the row is re-checked monthly per LAW-WATCH.
