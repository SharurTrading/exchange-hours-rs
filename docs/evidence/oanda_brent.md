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
ICE clock: the ICE specification prints weekday trading 01:00–23:00
London and 20:00–18:00 New York, with temporary US/UK DST arrangements
published separately. The 19:28 London settlement calculation is not a
session close. These underlying-contract statements do not replace
OANDA's printed blocks or key an operator-verified OANDA holiday row.

## Holidays

**Coverage:** 2024-02-21..2026-10-30 (inclusive trade dates; the operator's
own holiday page's dated sections, all times Eastern; converted to this
book's London clock at each date's own ET-London offset). Tier: T1
throughout.

Every converted scalar instant converts at five hours: none of these scalar instants falls
inside the March and late-October windows where America/New_York and
Europe/London disagree (in 2025-2026 those run roughly 2025-03-09..29,
2025-10-26..11-02, 2026-03-08..29 and 2026-10-25..11-01; the converted rows
cluster on 2025-05-26, 06-19, 07-04, 09-01, 11-27/28, 12-24/25/31 and their
2026 counterparts, all five-offset dates). The page has an explicit
`Regular trading hours` / `has regular hours` vocabulary for this book (MLK
and Presidents Day 2025 and 2026 print it), so a section that prints other
groups' rows and nothing for Brent **withholds** rather than affirms — six
2025 dates and four 2026 dates ship `Unsourced` on that rule; the 2024 audit below retains seven additional withholdings.

### 2024

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2024-03-29 | closed | `— (closed)` | `OANDA-UK-CARTER-2025-WAYBACK` | T1 | The expressly named Brent/BCO statement controls; event date equals London trade date, and each stated instant has a five-hour offset. |
| 2024-04-01 | unsourced | `— (arrangement withheld)` | `OANDA-UK-CARTER-2025-WAYBACK` | T1 | Easter Monday UK section names other books; Brent arrangement unstated (#332) |
| 2024-05-27 | early close | `13:30 ET` | `OANDA-UK-CARTER-2025-WAYBACK` | T1 | The expressly named Brent/BCO statement controls; event date equals London trade date, and each stated instant has a five-hour offset. |
| 2024-05-28 | unsourced | `— (arrangement withheld)` | `OANDA-UK-CARTER-2025-WAYBACK` | T1 | Memorial Day Brent reopen unprinted; separate Commodities wording ambiguous (#332) |
| 2024-06-19 | early close | `13:30 ET` | `OANDA-UK-CARTER-2025-WAYBACK` | T1 | The expressly named Brent/BCO statement controls; event date equals London trade date, and each stated instant has a five-hour offset. |
| 2024-06-20 | unsourced | `— (arrangement withheld)` | `OANDA-UK-CARTER-2025-WAYBACK` | T1 | Juneteenth Brent reopen unprinted; separate Commodities wording ambiguous (#332) |
| 2024-07-03 | unsourced | `— (arrangement withheld)` | `OANDA-UK-CARTER-2025-WAYBACK` | T1 | Independence eve section names only indices; Brent silent (#332) |
| 2024-07-04 | early close | `13:30 ET` | `OANDA-UK-CARTER-2025-WAYBACK` | T1 | The expressly named Brent/BCO statement controls; event date equals London trade date, and each stated instant has a five-hour offset. |
| 2024-07-05 | unsourced | `— (arrangement withheld)` | `OANDA-UK-CARTER-2025-WAYBACK` | T1 | Independence Day Brent reopen unprinted (#332) |
| 2024-08-26 | unsourced | `— (arrangement withheld)` | `OANDA-UK-CARTER-2025-WAYBACK` | T1 | UK Summer Bank Holiday section names other books; Brent silent (#332) |
| 2024-09-02 | early close | `13:30 ET` | `OANDA-UK-CARTER-2025-WAYBACK` | T1 | The expressly named Brent/BCO statement controls; event date equals London trade date, and each stated instant has a five-hour offset. |
| 2024-09-03 | unsourced | `— (arrangement withheld)` | `OANDA-UK-CARTER-2025-WAYBACK` | T1 | Labor Day Brent reopen unprinted; separate Commodities wording ambiguous (#332) |
| 2024-11-28 | early close | `13:30 ET` | `OANDA-UK-CARTER-2025-WAYBACK` | T1 | The expressly named Brent/BCO statement controls; event date equals London trade date, and each stated instant has a five-hour offset. |
| 2024-11-29 | replacement blocks | `01:00; 20:00` (London conversion) | `OANDA-UK-CARTER-2025-WAYBACK` | T1 | The expressly named Brent/BCO statement controls. ET-to-London offset is five hours on this date; Thanksgiving reopens at 01:00 London next day and closes at 20:00 London. |
| 2024-12-24 | early close | `13:59 ET` | `OANDA-UK-CARTER-2025-WAYBACK` | T1 | The expressly named Brent/BCO statement controls; event date equals London trade date, and each stated instant has a five-hour offset. |
| 2024-12-25 | closed | `— (closed)` | `OANDA-UK-CARTER-2025-WAYBACK` | T1 | The expressly named Brent/BCO statement controls; event date equals London trade date, and each stated instant has a five-hour offset. |
| 2024-12-31 | early close | `14:59 ET` | `OANDA-UK-CARTER-2025-WAYBACK` | T1 | The expressly named Brent/BCO statement controls; event date equals London trade date, and each stated instant has a five-hour offset. |

### 2025

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2025-01-01 | closed | `Brent Crude Oil is closed` | `OANDA-UK-HOLIDAYS` | T1 | New Year's Day |
| 2025-01-09 | unsourced | — | `OANDA-UK-HOLIDAYS` | T1 | Carter's Day of Mourning: the section prints US index/bonds and Commodities rows, nothing for Brent; the book's regular-hours vocabulary makes silence a withholding |
| 2025-04-17 | unsourced | — | `OANDA-UK-HOLIDAYS` | T1 | Holy Thursday: same rule |
| 2025-04-18 | closed | `BCO is closed` | `OANDA-UK-HOLIDAYS` | T1 | Good Friday |
| 2025-04-21 | unsourced | — | `OANDA-UK-HOLIDAYS` | T1 | Easter Monday: section names other books but no Brent arrangement; regular-hours vocabulary makes silence a withholding (#332) |
| 2025-05-26 | early close | `closes early at 1:30 PM and reopen at 8:00 PM` | `OANDA-UK-HOLIDAYS` | T1 | Memorial Day, 1:30 PM ET = 18:30 London |
| 2025-05-27 | replacement blocks | `01:00; 21:59; 22:05; 22:59` | `OANDA-UK-HOLIDAYS` | T1 | Complete replacement: previous holiday's literal reopen plus this trade date's printed or normal close. |
| 2025-06-19 | early close | `closes early at 1:30 PM and reopens at 8:00 PM` | `OANDA-UK-HOLIDAYS` | T1 | Juneteenth, 18:30 London |
| 2025-06-20 | replacement blocks | `01:00; 21:59` | `OANDA-UK-HOLIDAYS` | T1 | Complete replacement: previous holiday's literal reopen plus this trade date's printed or normal close. |
| 2025-07-03 | unsourced | — | `OANDA-UK-HOLIDAYS` | T1 | Independence Day eve: the section prints US index and Commodities rows, nothing for Brent |
| 2025-07-04 | early close | `close early at 1:30 pm` | `OANDA-UK-HOLIDAYS` | T1 | Independence Day, 18:30 London |
| 2025-08-25 | unsourced | — | `OANDA-UK-HOLIDAYS` | T1 | UK Summer Bank Holiday: section names other books but no Brent arrangement; regular-hours vocabulary makes silence a withholding (#332) |
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
| 2026-04-06 | unsourced | — | `OANDA-UK-HOLIDAYS` | T1 | Easter Monday: section names other books but no Brent arrangement; regular-hours vocabulary makes silence a withholding (#332) |
| 2026-05-01 | unsourced | — | `OANDA-UK-HOLIDAYS` | T1 | Labour Day: section names other books but no Brent arrangement; regular-hours vocabulary makes silence a withholding (#332) |
| 2026-05-04 | unsourced | — | `OANDA-UK-HOLIDAYS` | T1 | Early May Bank Holiday: section names other books but no Brent arrangement; regular-hours vocabulary makes silence a withholding (#332) |
| 2026-05-25 | early close | `Early close at 13:29; reopen at 20:01` | `OANDA-UK-HOLIDAYS` | T1 | Memorial Day, 13:29 ET = 18:29 London; the reopen 20:01 ET = 01:01 London next day, this book's own evening open |
| 2026-06-19 | early close | `Early close at 13:29` | `OANDA-UK-HOLIDAYS` | T1 | Juneteenth, 18:29 London |
| 2026-07-03 | early close | `Early close at 13:29` | `OANDA-UK-HOLIDAYS` | T1 | Independence Day, 18:29 London |
| 2026-08-31 | unsourced | — | `OANDA-UK-HOLIDAYS` | T1 | UK Summer Bank Holiday: section names other books but no Brent arrangement; regular-hours vocabulary makes silence a withholding (#332) |
| 2026-09-07 | early close | `Early close at 13:29` | `OANDA-UK-HOLIDAYS` | T1 | Labor Day, 18:29 London |

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
govern here too. The archived 2024 rows additionally resolve
`OANDA-UK-CARTER-2025-WAYBACK` through
[`oanda_us_index_cfd.md`](oanda_us_index_cfd.md)'s Documents table,
sha256 `467313685d4d64a4b95e5112a58d0fa04583f0abe988b1478d41fddf8b04cf1c`.

The underlying comparison additionally cites the following primary artifact,
saved as `normal-weeks/oanda-underlying-policy-2026-10-12/ice-brent-product-219.html`
in the research store. It establishes ICE contract hours, not OANDA hours.

| Document ID | Window | URL | Retrieved | Tier | SHA256 |
|---|---|---|---|---|---|
| `ICE-BRENT-SPEC-2026-10-12` | current specification observed 2026-10-12; no historical cutover claimed | <https://www.ice.com/products/219/Brent-Crude-Futures> | 2026-10-12 UTC | T1 | `8b9ca9b5ddfbfe482e33f6a5f5055a5e803988a5ca84a01d613e36106c711f0b` |

## Gaps and residual risks

- **The pre-2024-02-21 normal week is carried, not sourced.** The 2022
  regime (`20:00 - 18:00`, Friday an hour early) differs and its changeover
  is undated; the span refuses on the date-aware surface. Closing condition:
  dated operator artifacts, or hunts closed negative (then the 2026-10-04
  intersection rule applies).
- The seventeen `Unsourced` dates close with UK-scoped operator artifacts
  printing the book's arrangements (#332); the page's own regular-hours
  vocabulary is the affirmative form to look for.
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

## Archived 2024 scope — 2026-10-11 UTC

The March 2025 UK archive explicitly names Brent/BCO in the 2024 Good Friday, Memorial, Juneteenth, Independence, Labor, Thanksgiving and Christmas/New Year sections. Its 2024-05-01 and 2024-05-06 BCO/USD `is open` statements affirm ordinary trading. The book's regular-hours vocabulary makes the 2024-04-01 Easter, 2024-07-03 Independence eve and 2024-08-26 UK bank-holiday sections' Brent silence a withholding. Four additional following dates (2024-05-28, 2024-06-20, 2024-07-05, 2024-09-03) withhold exact sessions: the preceding sections specify Brent's early close but no Brent reopen; separate `Commodities` late-open wording does not establish this book's interval. Closing condition: UK-scoped operator statements fixing these seven arrangements (#332). Thanksgiving alone explicitly prints Brent's 20:00 ET reopen and Friday 15:00 ET close, producing the 01:00–20:00 London replacement. All seven converted early-close instants fall on five-hour ET-to-London dates; Good Friday's closure needs no scalar conversion inside the March DST mismatch. Earlier dates are outside the 2024-02-21 holiday window; no pre-witness close is derived. The table has 47 rows, including seventeen withheld dates. All seventeen new rows cite the byte-identical Carter archive through oanda_us_index_cfd.md's Documents table.

## Consistent UK holiday scope — 2026-10-11 UTC

The same silence rule applies to Easter Monday 2025-04-21 and 2026-04-06, UK Summer Bank Holiday 2025-08-25 and 2026-08-31, Labour Day 2026-05-01 and Early May Bank Holiday 2026-05-04. These six sections name other books but do not affirm Brent trading. The corresponding May 2024 and May 2025 sections explicitly name BCO/USD as open, so those dates remain audited normal. The 2026-09-07 Brent statement prints only the 13:29 ET close; the section's 20:01 reopen is Agricultural wording and does not provide this book's reopening. No Brent reopen is attributed to that statement. Seven 2024, six 2025 and four 2026 dates are withheld; UK-scoped operator arrangements close those gaps (#332).
