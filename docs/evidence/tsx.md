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

**Coverage:** 2017-01-01..2026-12-31 (inclusive trade dates in `America/Toronto`; tier T1 throughout).

The operator's holiday statement is TMX Group's own "Calendar" page,
`tsx.com/en/trading/calendars-and-trading-hours/calendar` (found through the tsx.com sitemap),
one server-rendered page carrying the current year's "Stock Market Holidays - Stock Markets
Closed" list in full and the next year's from Q4. TSX runs no overnight session, so an event
date and its trade date are one civil day and the conversion is the identity. The 2025 and
2026 lists are read from the live retrieval of 2026-09-28; every earlier year is pinned by
Wayback `id_` captures of the same page across its two paths — the 2018 relaunch path
`tsx.com/trading/calendars-and-trading-hours/calendar` (named by the archived sitemap.xml of
2022-06-26; ten captures 2018-09-11..2023-12-15 pin 2017 through 2023, the 2018-09-11
capture's archive section restating the complete 2017 list) and the `/en/` path for 2024-2025
(the 2024-12-17 capture carries the complete 2024 list; the July 2024 states printed the list
without its Christmas Eve row, so that row keys to the December state — a knowledge boundary
may only widen).

**2027 is not published.** The page's newest section is 2026; TMX historically adds the next
year's calendar in Q4. Verified 2026-09-28: no 2027 list exists on the page or behind its
"Settlement Schedule" links. Nothing past 2026-12-31 is claimed; **closing condition:** the
operator's 2027 calendar section.

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
| 2024-01-01 | closed | `In Lieu of New Year's Day - Monday, January 1, 2024` | `TSX-CAL-2024-12-17` | T1 | the operator's own in-lieu print; the July 2024 states print the same ten closures |
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
own index, and `holidays/raw/equities/tsx/2010-2024/` holds the ten 2017-2025 captures above
(their sha256s in `SHA256SUMS.txt`, their retrieval stamps in that directory's `INDEX.md`).

## Gaps and residual risks

- **Interpretive step, order-entry classification.** The 07:00–09:30 Pre-Open is `order_entry`: no trade can match inside it, and the first print of the day is the 09:30 Market-on-Open cross that starts continuous trading.
- **Interpretive step, conditional extension.** The Price Movement Extension rule is modelled as the venue's maximum envelope. On ordinary days and symbols the 16:00–16:10 interval is cancel-only, but when the extension fires it is the delayed Market-on-Close cross for that symbol and it prints, so the window is not order-entry-only. The separate 16:10–16:15 Post Market Cancel Session is not modelled at all.
- **Interpretive step, the Christmas Eve half.** The footnote states two closes — `1:00 PM (TSX/TSXV) and 1:30 (ALPHA/ALPHA X/DRK)`. The row encodes the TSX/TSXV 13:00 close because this identity is the Toronto Stock Exchange cash-equity venue; the ALPHA/ALPHA X/DRK book systems are separate order books, not part of this row's scope, and no TSX-listed session is clipped by their later close.
- **No dated revision.** The reviewed grid holds for the whole audit window, so every instant resolves to the one profile. A sourced revision later replaces this with a real timeline row and needs no routing change. Closing condition for a future change: a TMX notice stating an unconditional day-level effective date.
- **Holiday coverage gap, 2010-01-01..2016-12-31 (tracked as [#221](https://github.com/SharurTrading/exchange-hours-rs/issues/221)).** No capture of any TSX holiday page reaches those years: the 2018 relaunch path's first capture is 2018-09-11 and its archive section restates only 2017, and the CDX sweeps of the candidate pre-relaunch URLs returned nothing recoverable — with the caveat that the Wayback CDX service was down for most of 2026-09-29 UTC, so the domain-wide `holiday`/`calendar` filters could not complete. Queries before 2017-01-01 refuse rather than answer. Closing condition: a capture of any pre-relaunch TSX/TMX holiday page dated 2010-2016, the domain-wide CDX filters re-run when the archive service is stable, or the operator's re-publication of a historical calendar.
- **Holiday horizon.** The audited holiday window stops at 2026-12-31 because the operator has published nothing past it (verified 2026-09-28). Closing condition: TMX's 2027 calendar section; the row is re-checked monthly per LAW-WATCH.
