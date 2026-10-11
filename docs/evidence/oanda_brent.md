<!-- SPDX-License-Identifier: MIT-0 -->

# `oanda_brent` — evidence

- **Kind:** `MarketHoursKey`
- **Owner module:** [`oanda.rs`](../../src/calendar/schedules/futures/international/oanda.rs)
- **Source sets:** [`CFD-OANDA`](../schedules/sources.md#cfd-oanda)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-10-10 UTC)

OANDA's own Brent book, anchored to London by the page's own dagger note:
opens Sunday 23:01 and closes Friday 21:59 London time, with the six-minute
21:59-22:05 break and the two-hour 22:59-01:01 break Monday-Thursday.

## Revision rows

None. The module carries one sourced grid and no `revisions!` block; there is
no dated, day-level change to record, and a dormant identity owes its current
schedule rather than a worked-up history (LAW-BOUNDED-WORK).

## The normal week

The page's timezone column names New York, but the dagger note states the
real clock in London terms:

> `†Brent Crude Oil starts at 23:01 London time on Sunday and closes at 21:59 London time on Friday. There will be a six-minute break from 21:59 - 22:05 London time Mon-Thu, and two-hours break from 22:59 - 01:01 London time Mon-Thu.`

The note is unconditional and complete (both endpoints, both breaks, both
clocks named), so the grid anchors to `Europe/London`: Sunday 23:01 wrapping
to Monday 21:59 with the two London-time breaks Monday-Thursday, then
weeknights 01:01-21:59, Friday 01:01-21:59 ending the week. The 2024 Wayback
capture corroborates with its own footnote form:

> `†Brent Crude oil: Starts at 23:01 London time on a Sunday / Closes at 21:59 London time Friday / has a break from 21:59- 22:05`

The 2022 capture shows the earlier regime — `Sun - Fri 20:00 - 18:00*` with
`*Brent Crude Oil closes an hour earlier on Fridays` — which the horizon
excludes (see Dated selectors). The current shape is OANDA's own, not the
ICE clock: ICE's own Brent futures day runs to 19:00 London with its own
window structure, and no ICE statement keys a row here.

## Holidays

**Coverage:** 2025-01-01..2026-10-30 (inclusive trade dates; the operator's
own holiday page's dated sections, all times Eastern; converted to this
book's London clock at each date's own ET-London offset). Tier: T1
throughout.

Every printed instant converts at five hours: none of these holidays falls
inside the March and late-October windows where America/New_York and
Europe/London disagree (in 2025-2026 those run roughly 2025-03-09..29,
2025-10-26..11-02, 2026-03-08..29 and 2026-10-25..11-01; the converted rows
cluster on 2025-05-26, 06-19, 07-04, 09-01, 11-27/28, 12-24/25/31 and their
2026 counterparts, all five-offset dates). The page has an explicit
`Regular trading hours` / `has regular hours` vocabulary for this book (MLK
and Presidents Day 2025 and 2026 print it), so a section that prints other
groups' rows and nothing for Brent **withholds** rather than affirms — four
dates ship `Unsourced` on that rule.

### 2025

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2025-01-01 | closed | `Brent Crude Oil is closed` | `OANDA-UK-HOLIDAYS` | T1 | New Year's Day |
| 2025-01-09 | unsourced | — | `OANDA-UK-HOLIDAYS` | T1 | Carter's Day of Mourning: the section prints US index/bonds and Commodities rows, nothing for Brent; the book's regular-hours vocabulary makes silence a withholding |
| 2025-04-17 | unsourced | — | `OANDA-UK-HOLIDAYS` | T1 | Holy Thursday: same rule |
| 2025-04-18 | closed | `BCO is closed` | `OANDA-UK-HOLIDAYS` | T1 | Good Friday |
| 2025-05-26 | early close | `closes early at 1:30 PM and reopen at 8:00 PM` | `OANDA-UK-HOLIDAYS` | T1 | Memorial Day, 1:30 PM ET = 18:30 London |
| 2025-05-27 | replacement blocks | `01:00; 21:59; 22:05; 22:59` | `OANDA-UK-HOLIDAYS` | T1 | Complete replacement: previous holiday's literal reopen plus this trade date's printed or normal close. |
| 2025-06-19 | early close | `closes early at 1:30 PM and reopens at 8:00 PM` | `OANDA-UK-HOLIDAYS` | T1 | Juneteenth, 18:30 London |
| 2025-06-20 | replacement blocks | `01:00; 21:59` | `OANDA-UK-HOLIDAYS` | T1 | Complete replacement: previous holiday's literal reopen plus this trade date's printed or normal close. |
| 2025-07-03 | unsourced | — | `OANDA-UK-HOLIDAYS` | T1 | Independence Day eve: the section prints US index and Commodities rows, nothing for Brent |
| 2025-07-04 | early close | `close early at 1:30 pm` | `OANDA-UK-HOLIDAYS` | T1 | Independence Day, 18:30 London |
| 2025-09-01 | early close | `Close early at 1:30 PM, reopen at 8:00 PM` | `OANDA-UK-HOLIDAYS` | T1 | Labor Day, 18:30 London |
| 2025-09-02 | replacement blocks | `01:00; 21:59; 22:05; 22:59` | `OANDA-UK-HOLIDAYS` | T1 | Complete replacement: previous holiday's literal reopen plus this trade date's printed or normal close. |
| 2025-11-26 | unsourced | — | `OANDA-UK-HOLIDAYS` | T1 | Thanksgiving eve: the section prints one Agricultural row, Brent silent |
| 2025-11-27 | early close | `Close early at 1:30 PM, reopen at 8:00 PM` | `OANDA-UK-HOLIDAYS` | T1 | Thanksgiving, 18:30 London |
| 2025-11-28 | replacement blocks | `01:00; 20:00` | `OANDA-UK-HOLIDAYS` | T1 | Complete replacement: previous holiday's literal reopen plus this trade date's printed or normal close. |
| 2025-12-24 | early close | `Brent Crude Oil: close at 13:59` | `OANDA-UK-HOLIDAYS` | T1 | Christmas Eve, 13:59 ET = 18:59 London |
| 2025-12-25 | closed | `Brent Crude Oil: closed` | `OANDA-UK-HOLIDAYS` | T1 | Christmas Day |
| 2025-12-31 | early close | `Brent Crude Oil: close at 14:59` | `OANDA-UK-HOLIDAYS` | T1 | New Year's Eve, 14:59 ET = 19:59 London |

### 2026

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2026-01-01 | closed | `Brent Crude Oil: closed` | `OANDA-UK-HOLIDAYS` | T1 | New Year's Day |
| 2026-04-03 | closed | `Brent Crude Oil: Closed` | `OANDA-UK-HOLIDAYS` | T1 | Good Friday |
| 2026-05-25 | early close | `Early close at 13:29; reopen at 20:01` | `OANDA-UK-HOLIDAYS` | T1 | Memorial Day, 13:29 ET = 18:29 London; the reopen 20:01 ET = 01:01 London next day, this book's own evening open |
| 2026-06-19 | early close | `Early close at 13:29` | `OANDA-UK-HOLIDAYS` | T1 | Juneteenth, 18:29 London |
| 2026-07-03 | early close | `Early close at 13:29` | `OANDA-UK-HOLIDAYS` | T1 | Independence Day, 18:29 London |
| 2026-09-07 | early close | `Early close at 13:29; reopen at 20:01` | `OANDA-UK-HOLIDAYS` | T1 | Labor Day, 18:29 London |

## Dated selectors

None. One sourced grid; `sourcing.rs` declares the normal week witnessed from
**2024-02-21**, the operator's archived page print whose footnote states the
current London grid. The 2022-05-18 capture prints the earlier `20:00 -
18:00` New-York regime with its Friday-early-close asterisk, so the
changeover is undated and the span below the 2024 witness is carried, not
served.

## Sources

- <https://www.oanda.com/uk-en/trading/hours-of-operation/> — the operator's
  hours page, captured 2026-10-10 22:36 UTC and rendered by the maintainer to
  PDF the same UTC date.
- <https://www.oanda.com/uk-en/trading/holiday-trading-hours> — the
  operator's UK holiday page, captured 2026-10-10 22:44 UTC.

### Documents

The Documents table governing this row is
[`oanda_fx.md`](oanda_fx.md)'s, which cites the hours page (PDF and HTML),
the holiday page, and the 2024 and 2022 Wayback captures; the digests there
govern here too.

## Gaps and residual risks

- **The pre-2024-02-21 normal week is carried, not sourced.** The 2022
  regime (`20:00 - 18:00`, Friday an hour early) differs and its changeover
  is undated; the span refuses on the date-aware surface. Closing condition:
  dated operator artifacts, or hunts closed negative (then the 2026-10-04
  intersection rule applies).
- The four `Unsourced` dates close with an OANDA desk answer or a capture
  printing the book's arrangement (the page's own regular-hours vocabulary
  is the affirmative form to look for).
- The ET-to-London conversion rests on every row being a five-offset date;
  a future holiday landing inside a DST-disagreement window needs its own
  offset check before conversion.

## Publication audit — 2026-10-11 UTC

The source's minute values are literal. The 2025 18:00 ET US reopens and
20:00 ET Brent reopens are not rounded to 18:01 or 20:01. Evening opens
attach to the following trade date. Replacement blocks retain the entire
trading day, including any next-day early close and Brent's late segment.

Monday's daytime session begins on Sunday at 23:01 London; there is no
additional Monday 01:01 opening. The 01:01 rule applies Tuesday–Friday.
The Monday 22:05–22:59 segment remains separate.

Pending-order storage outside the printed trading windows is a broker facility,
not a timed session phase; see [the shared scope decision](oanda_fx.md#pending-orders-and-session-scope--2026-10-11-utc) (#333).
