<!-- SPDX-License-Identifier: MIT-0 -->

# `asx_spi_200`

ASX SPI 200 index futures (AP, ASX 24 — formerly SFE/SYCOM), `Australia/Sydney`.

- **Kind:** `MarketHoursKey`
- **Admission:** served — the consumer's OANDA activation maps AU200 to this
  family (SharurPlatform), so a consumer instrument reaches the key.

## Ledger basis (moved from docs/schedules/verification.md on 2026-10-14 UTC)

The verification row's basis, horizon and cadence live in
[the ledger](../schedules/verification.md); this file holds the quotations,
interpretive steps and residual risks behind the row. The normal week is
constant since the January-2010 floor with one seasonal selector the
operator's own table states; the holiday table is the family's own ASX 24
column of the yearly trading calendars 2010–2027, converted to the crate's
trade dates.

### Normal week

The operator's trading-hours table is a two-column seasonal selector, printed
identically in every captured state 2010→2026:

> "Trading hours - Australian Eastern Standard Time / Australian Eastern
> Daylight Time … Code | Contract | For period from second Sunday in March to
> first Sunday in November | For period from first Sunday in November to
> second Sunday in March … AP SPI 200™ Index Futures and Options 5.10pm -
> 7.00am and 9.50am - 4.30pm | 5.10pm - 8.00am and 9.50am - 4.30pm"

(`www2` index-derivatives trading-hours page, captures 2020–2022; the same
two-column shape prints on the 2011 ASX 24 trading-hours page, the 2010 SYCOM
hours PDF, the 2012 ASX Trade24 hours PDF and the 2015/2018 SPI contract
specifications.) The pre-open footnote is the operator's own:

> "* Pre-open starts 10 minutes prior to trading for all contracts."

(SYCOM Trading Hours PDF, capture 2010-11-20.)

The weekly shape the calendars imply: the day session opens Monday to Friday;
the night session opens Sunday to Friday evenings, so the week's final close
is the Friday-evening leg on the Saturday morning. Evidence: the 2015 calendar
marks ANZAC Day Saturday 25 April 2015 "Normal trading hours apply for ASX
24" (the only sessions a Saturday can host are the Friday-evening leg's),
while the 2020 and 2021 calendars mark the ANZAC weekend "ASX 24: Closed" —
which closes that leg — and the Sunday-night closure leaves Monday opening
with the day session alone.

### Seasonal selector

The selector is the operator's own column pair: US daylight time (from the
second Sunday in March to the first Sunday in November) closes the night
session at 07:00 Sydney; the standard column closes it at 08:00 Sydney. The
crate computes the New York local date of the night session's open instant
(17:10 Sydney on the opening day) and reads the column off it. A fixed
day-delta would misclassify the transition Sundays: the US switch lands at
18:00 Sydney on the second Sunday of March and the first Sunday of November,
mid-way through the Sunday-night session, and the operator's columns are
day-level periods.

### Trade-date conversion for the holiday rows

The operator's calendar keys each row to its civil date: the Day Session
column is that date's day session, and the Night Session column ("Night
Session to open at 5:10pm/5:00pm") is the session opening that evening. The
crate keys holiday rows to the trade date — the local date of the trading
day's final close — so the night session opening on date D belongs to trade
date D+1 (or the Saturday after a Friday open). Every row below records that
conversion in its derivation cell. The 2010-era SFE tables print one clause
per deviating session ("SFE: Closed", "Day Session Closed / Night Session to
open at 5:10pm", "Day Session Open"): a single "Day Session Open" clause
(2010-12-28) states the day session only, and the night session — the
institution the legend calls "Normal Trading Hours" — is read as normal; the
reading is disclosed under Gaps.

## Revision rows

None.

The normal week has no dated change: the two-column grid prints identically
in every captured state from the January-2010 floor to today, so the timeline
is one baseline selected by the seasonal column and no `revisions!` block
exists. The nightly reopen printed inside the holiday rows ("Night Session to
open at 5:00pm" on the two 2016 rows captured 2015-11-20, against 5:10pm
everywhere else) is per-date data — the 2016-01-27 and 2016-04-26 trade dates
carry replacement blocks with the 17:00 open — and is never a revision.

## Sources

- The live ASX 24 trading calendar (2026+2027 tables) and its Wayback
  captures 2010–2025 — `holidays/raw/asx24-2010-2027/` in the research store
  (`SharurTrading/exchange-hours-evidence`), with the row-by-row extraction
  `asx24-rows-extraction.md` (354 operator rows 2010–2027, taxonomy-mapped;
  store PR #15).
- The SYCOM Trading Hours PDF (capture 2010-11-20) and ASX Trade24 Trading
  Hours PDF (capture 2012-04-13), the 2011 ASX 24 trading-hours page and the
  `www2` index-derivatives trading-hours pages (captures 2020–2022) —
  `normal-weeks/asx-spi-2026-10-11/` in the research store.
- ONO (Overnight Options) footnotes in the calendars are options-only
  sessions and are never futures rows; no row below encodes one.

## Gaps and residual risks

- **The 2016 night-open question.** The 2016 calendar (captured 2015-11-20)
  prints "Night Session to open at 5:00pm (Sydney Time)" inside the 26 January
  and 25 April rows and 5:10pm inside the 13 June row of the same capture. The
  crate serves the 5:10pm open everywhere (every other artifact, 2010–2026,
  prints 5:10pm), encodes the two stated 5:00pm evenings as replacement
  blocks on their trade dates (2016-01-27, 2016-04-26), and discloses the
  residual: if the night open stood at 17:00 across an uncaptured 2016 span,
  ordinary 2016 evenings in that span are answered closed 17:00–17:10 where
  trading may have run. Closer: a dated ASX notice or a capture inside the
  span.
- **Generic early-close rows.** The Christmas/New Year early closes are
  "Equity Products to close at 2:30pm" rows — a generic ASX 24 products row,
  never SPI-named (no capture prints an SPI-named holiday row). The equity
  leg is the SPI-relevant clause; the reading is the extraction's.
- **2011-04-26 conflict resolved to the later capture.** The December-2010
  capture printed the Easter-Monday substitute "ASX 24: Closed"; the
  May-2011 capture prints "Day Session Closed / Night Session to Open at
  5:10pm". The crate keys the later capture (the family's service tier
  follows the operator's own revision).
- **2022-12-23 and 2022-12-30** print "Night Session Closed" only in the
  December-2022 capture (the July-2022 capture printed normal trading); the
  later capture keys the rows.
- **2024 rows are carried from the 2023-12-11 capture**: every direct 2024
  capture in the store is an Incapsula block page.
- The informational rows (NZ holidays, and US holidays from 2016, printing
  "Normal Trading Hours") ship no crate row: they state no change.

## Holidays

**Coverage:** 2010-01-01..2027-12-31, 2028-01-01..2028-01-01 (ASX 24 trading
calendar rows, SFE-era 2010 through the live 2026+2027 state; every date
inside the window was audited from its year's calendar, and a date without a
row is audited normal. The one-day 2028 window covers the final New Year
evening leg, whose trade date the 2027 calendar's "Night Session Closed" row
removes.)

### Documents

| Document | Window | Replay or service URL | Capture or retrieval, UTC | Tier | sha256 |
|---|---|---|---|---|---|
| `ASX24-CAL-2010` | SFE trading calendar 2010 (+ Dec 2009 – 2011) | <https://web.archive.org/web/20100128100216id_/http://www.asx.com.au/about/operational/trading_calendar/sfe/2010.htm> | 2010-01-28T10:02:16Z | T1 | `33e6a1022d20caca6aa854105630f75db9687274e66fcfc308cf31da89971fa5` |
| `ASX24-CAL-2011` | ASX 24 trading calendar 2011 | <https://web.archive.org/web/20110516085943id_/http://www.asx.com.au/about/asx-24-trading-calendar-2011.htm> | 2011-05-16T08:59:43Z | T1 | `09a9b0917058f1d978631423921e60583b487332c17df4cad13778c61db24ef1` |
| `ASX24-CAL-2012` | ASX 24 trading calendar 2012 | <https://web.archive.org/web/20150906003836id_/http://www.asx.com.au/about/asx-24-trading-calendar-2012.htm> | 2015-09-06T00:38:36Z | T1 | `66f47f571cd762256be01be5224439ada526872c0451732b0d52be897a960a5d` |
| `ASX24-CAL-2013` | ASX 24 trading calendar 2013 | <https://web.archive.org/web/20131007091353id_/http://www.asx.com.au/about/asx-24-trading-calendar-2013.htm> | 2013-10-07T09:13:53Z | T1 | `29b899c04424e57a15dcd457750efbac3c235f62a6555fa469a4be9d4c0ee7b2` |
| `ASX24-CAL-2014` | ASX 24 trading calendar 2014 | <https://web.archive.org/web/20131223094428id_/http://www.asx.com.au/about/asx-24-trading-calendar-2014.htm> | 2013-12-23T09:44:28Z | T1 | `63059e3345d1ff6e8bd80ab1b030638a16aaf54bdc687f1bd555aecacc6ebdf3` |
| `ASX24-CAL-2015` | ASX 24 trading calendar 2015 | <https://web.archive.org/web/20141130043626id_/http://www.asx.com.au/about/asx-24-trading-calendar-2015.htm> | 2014-11-30T04:36:26Z | T1 | `d57fec12951f15bcaff09fb6b70066fa817ab4dd4c594970cc00e8a63c877f0e` |
| `ASX24-CAL-2016` | ASX 24 trading calendar 2016 | <https://web.archive.org/web/20151120050041id_/http://www.asx.com.au/about/asx-24-trading-calendar-2016.htm> | 2015-11-20T05:00:41Z | T1 | `6513de2ec8fa277f20f10bd4fbf45a1f40978bd02cc57c792b6f274f5e5ca2cb` |
| `ASX24-CAL-2017` | ASX 24 trading calendar 2017 | <https://web.archive.org/web/20170103192507id_/http://www.asx.com.au/about/asx-24-trading-calendar-2017.htm> | 2017-01-03T19:25:07Z | T1 | `784a6ea88409ccd4195aff0a497bd019d590f4aae1490723f5c856c79950a020` |
| `ASX24-CAL-2018` | ASX 24 trading calendar 2018 | <https://web.archive.org/web/20171222063723id_/http://www.asx.com.au/about/asx-24-trading-calendar-2018.htm> | 2017-12-22T06:37:23Z | T1 | `126a8822ac1e0a4a3a9d8c2475ffb0e960286b147374aaa6eebaafdf1104255a` |
| `ASX24-CAL-2019` | ASX 24 trading calendar 2019 | <https://web.archive.org/web/20190821020021id_/https://www.asx.com.au/about/asx-24-trading-calendar-2019.htm> | 2019-08-21T02:00:21Z | T1 | `0bff174ebac4878dd31370744b10ad5f449e03879e87b2fc87febf0bc7730dde` |
| `ASX24-CAL-2020` | ASX 24 trading calendar 2020 | <https://web.archive.org/web/20191022200826id_/https://www.asx.com.au/about/asx-24-trading-calendar-2020.htm> | 2019-10-22T20:08:26Z | T1 | `d68560045f518dbf7ec32b5d2aed2f7776bf256cd1b046c702c078ce53120e90` |
| `ASX24-LIVE-2021-01` | live calendar (2021 table) | <https://web.archive.org/web/20210117034448id_/https://www2.asx.com.au/content/asx/home/markets/market-resources/asx-24-trading-calendar.html> | 2021-01-17T03:44:48Z | T1 | `b29e57a13f6c975f20396637167d2824eb0e1bf60d228a89e4214ce295f55d6f` |
| `ASX24-LIVE-2022-12` | live calendar (2023 + revised 2022 tables) | <https://web.archive.org/web/20221225022559id_/https://www2.asx.com.au/content/asx/home/markets/market-resources/asx-24-trading-calendar.html> | 2022-12-25T02:25:59Z | T1 | `48998b376b07a12c64b246e6d8ba9aa2f23a5a5c81d543e6560ea119a8d3b606` |
| `ASX24-LIVE-2023-12` | live calendar (2024 + 2023 tables) | <https://web.archive.org/web/20231211225115id_/https://www.asx.com.au/markets/market-resources/asx-24-trading-calendar> | 2023-12-11T22:51:15Z | T1 | `2817e96d912def34697a37f29389c163a5ce9ef6efe3a04bd791326f21d6929a` |
| `ASX24-LIVE-2025-03` | live calendar (2025 table) | <https://web.archive.org/web/20250316225953id_/https://www.asx.com.au/markets/market-resources/asx-24-trading-calendar> | 2025-03-16T22:59:53Z | T1 | `464ac2136830bd8100b6b2713f75ce751ff5a652f86838cab441ad5ef6a5a1ba` |
| `ASX24-LIVE-2026-10` | live calendar (2026 + 2027 tables) | <https://www.asx.com.au/markets/market-resources/asx-24-trading-calendar> | retrieved 2026-10-11 UTC | T1 | `2baa937bd806874d1618c65f07848d178bb9c1a51216c62090493b79d3d1832a` |
| `ASX-SYCOM-HOURS-2010` | SYCOM trading hours (night/day grid + pre-open footnote) | <https://web.archive.org/web/20101120044224id_/https://www.asx.com.au/products/derivatives/trading-hours-sycom.htm> | 2010-11-20T04:42:24Z | T1 | `de2497f634bcab792514d49e4518727b45ccd6c9d8f5f4378220b565b4d94e1c` |
| `ASX-TRADE24-HOURS-2012` | ASX Trade24 trading hours (same two-column selector) | <https://web.archive.org/web/20120413123746id_/https://www.asx.com.au/products/derivatives/asx-trade24-trading-hours.htm> | 2012-04-13T12:37:46Z | T1 | `60a870279d22505199b4a2b104e1eefcaef149381f7bf34a4d99fe7fa90ddb89` |
| `ASX-IDX-DERIV-HOURS-2020` | index-derivatives trading hours (current selector) | <https://web.archive.org/web/20201025203044id_/https://www2.asx.com.au/markets/market-resources/trading-hours-calendars/index-derivatives> | 2020-10-25T20:30:44Z | T1 | `108572fa21b005576efe2e17a01e4bcc75139bec5a0f2cf7a4b83b3bfedd234e` |

The row-by-row extraction of all 354 operator rows and the per-year word-level
diffs live beside the artifacts in the research store
(`holidays/raw/asx24-2010-2027/asx24-rows-extraction.md`).

### 2010

| 2010-01-01 | closed | — | `ASX24-CAL-2010` | T1 | operator Closed/Non Trading Day (day and night sessions) |
| 2010-01-02 | closed | — | `ASX24-CAL-2010` | T1 | the operator closed the Friday evening's night session, whose trade date is the Saturday |
| 2010-01-19 | replacement blocks | blocks orderentry(-1, 61200, 61800); regular(-1, 61800, 0); orderentry(0, 34800, 35400); regular(0, 35400, 59400) | `ASX24-CAL-2010` | T1 | Night session to cease trading at midnight (US policy): the night session opening on the US holiday ceased at midnight Sydney, so its trade date (the following day) runs that shortened leg and then its own day |
| 2010-01-26 | early close | 08:00 | `ASX24-CAL-2010` | T1 | Day Session Closed / Night Session to open at 5:10pm: the day session is removed and the morning night leg ends at its normal 08:00 close |
| 2010-02-06 | closed | — | `ASX24-CAL-2010` | T1 | operator Closed/Non Trading Day (day and night sessions) |
| 2010-02-16 | replacement blocks | blocks orderentry(-1, 61200, 61800); regular(-1, 61800, 0); orderentry(0, 34800, 35400); regular(0, 35400, 59400) | `ASX24-CAL-2010` | T1 | Night session to cease trading at midnight (US policy): the night session opening on the US holiday ceased at midnight Sydney, so its trade date (the following day) runs that shortened leg and then its own day |
| 2010-04-02 | replacement blocks | blocks orderentry(-1, 61200, 61800); regular(-1, 61800, 0) | `ASX24-CAL-2010` | T1 | Day Before Good Friday: the night session opening on the eve ceased at midnight Sydney (operator row) and Good Friday itself is Closed/Non Trading Day, so the trade date runs that shortened evening leg and no day session |
| 2010-04-03 | closed | — | `ASX24-CAL-2010` | T1 | the operator closed the Friday evening's night session, whose trade date is the Saturday |
| 2010-04-05 | closed | — | `ASX24-CAL-2010` | T1 | operator Closed/Non Trading Day (day and night sessions) |
| 2010-04-06 | replacement blocks | blocks 09:40-09:50; 09:50-16:30 | `ASX24-CAL-2010` | T1 | the operator closed the holiday evening's night session; the late open removes the wrapped leg and keeps the following day's own sessions |
| 2010-04-26 | early close | 07:00 | `ASX24-CAL-2010` | T1 | Day Session Closed / Night Session to open at 5:10pm: the day session is removed and the morning night leg ends at its normal 07:00 close |
| 2010-06-01 | replacement blocks | blocks orderentry(-1, 61200, 61800); regular(-1, 61800, 0); orderentry(0, 34800, 35400); regular(0, 35400, 59400) | `ASX24-CAL-2010` | T1 | Night session to cease trading at midnight (US policy): the night session opening on the US holiday ceased at midnight Sydney, so its trade date (the following day) runs that shortened leg and then its own day |
| 2010-06-14 | early close | 07:00 | `ASX24-CAL-2010` | T1 | Day Session Closed / Night Session to open at 5:10pm: the day session is removed and the morning night leg ends at its normal 07:00 close |
| 2010-07-06 | replacement blocks | blocks orderentry(-1, 61200, 61800); regular(-1, 61800, 0); orderentry(0, 34800, 35400); regular(0, 35400, 59400) | `ASX24-CAL-2010` | T1 | Night session to cease trading at midnight (US policy): the night session opening on the US holiday ceased at midnight Sydney, so its trade date (the following day) runs that shortened leg and then its own day |
| 2010-09-07 | replacement blocks | blocks orderentry(-1, 61200, 61800); regular(-1, 61800, 0); orderentry(0, 34800, 35400); regular(0, 35400, 59400) | `ASX24-CAL-2010` | T1 | Night session to cease trading at midnight (US policy): the night session opening on the US holiday ceased at midnight Sydney, so its trade date (the following day) runs that shortened leg and then its own day |
| 2010-11-26 | replacement blocks | blocks orderentry(-1, 61200, 61800); regular(-1, 61800, 0); orderentry(0, 34800, 35400); regular(0, 35400, 59400) | `ASX24-CAL-2010` | T1 | Night session to cease trading at midnight (US policy): the night session opening on the US holiday ceased at midnight Sydney, so its trade date (the following day) runs that shortened leg and then its own day |
| 2010-12-24 | early close | 14:30 | `ASX24-CAL-2010` | T1 | Equity Products to close at 2:30pm and Night Session Closed: the day session closes 14:30 and the evening night session does not open |
| 2010-12-25 | closed | — | `ASX24-CAL-2010` | T1 | operator Closed/Non Trading Day (day and night sessions) |
| 2010-12-26 | closed | — | `ASX24-CAL-2010` | T1 | operator Closed/Non Trading Day (day and night sessions) |
| 2010-12-27 | closed | — | `ASX24-CAL-2010` | T1 | operator Closed/Non Trading Day (day and night sessions) |
| 2010-12-28 | replacement blocks | blocks 09:40-09:50; 09:50-16:30 | `ASX24-CAL-2010` | T1 | the operator closed the holiday evening's night session; the late open removes the wrapped leg and keeps the following day's own sessions |
| 2010-12-31 | early close | 14:30 | `ASX24-CAL-2010` | T1 | Equity Products to close at 2:30pm and Night Session Closed: the day session closes 14:30 and the evening night session does not open |

### 2011

| 2011-01-01 | closed | — | `ASX24-CAL-2010` | T1 | the closed Friday night session's trade date is the Saturday |
| 2011-01-03 | closed | — | `ASX24-CAL-2011` | T1 | operator Closed/Non Trading Day (day and night sessions) |
| 2011-01-04 | replacement blocks | blocks 09:40-09:50; 09:50-16:30 | `ASX24-CAL-2011` | T1 | the operator closed the holiday evening's night session; the late open removes the wrapped leg and keeps the following day's own sessions |
| 2011-01-18 | replacement blocks | blocks orderentry(-1, 61200, 61800); regular(-1, 61800, 0); orderentry(0, 34800, 35400); regular(0, 35400, 59400) | `ASX24-CAL-2011` | T1 | Night session to cease trading at midnight (US policy): the night session opening on the US holiday ceased at midnight Sydney, so its trade date (the following day) runs that shortened leg and then its own day |
| 2011-01-26 | early close | 08:00 | `ASX24-CAL-2011` | T1 | Day Session Closed / Night Session to open at 5:10pm: the day session is removed and the morning night leg ends at its normal 08:00 close |
| 2011-02-06 | closed | — | `ASX24-CAL-2011` | T1 | operator Closed/Non Trading Day (day and night sessions) |
| 2011-02-07 | replacement blocks | blocks 09:40-09:50; 09:50-16:30 | `ASX24-CAL-2011` | T1 | the operator closed the Sunday evening's night session; the late open removes the wrapped leg and keeps Monday's own sessions |
| 2011-02-22 | replacement blocks | blocks orderentry(-1, 61200, 61800); regular(-1, 61800, 0); orderentry(0, 34800, 35400); regular(0, 35400, 59400) | `ASX24-CAL-2011` | T1 | Night session to cease trading at midnight (US policy): the night session opening on the US holiday ceased at midnight Sydney, so its trade date (the following day) runs that shortened leg and then its own day |
| 2011-04-22 | replacement blocks | blocks orderentry(-1, 61200, 61800); regular(-1, 61800, 0) | `ASX24-CAL-2011` | T1 | Day Before Good Friday: the night session opening on the eve ceased at midnight Sydney (operator row) and Good Friday itself is Closed/Non Trading Day, so the trade date runs that shortened evening leg and no day session |
| 2011-04-23 | closed | — | `ASX24-CAL-2011` | T1 | the operator closed the Friday evening's night session, whose trade date is the Saturday |
| 2011-04-25 | closed | — | `ASX24-CAL-2011` | T1 | operator Closed/Non Trading Day (day and night sessions) |
| 2011-04-26 | closed | — | `ASX24-CAL-2011` | T1 | the operator closed the day session and the prior evening's night session (the previous day was itself closed, so no morning leg ran); the holiday evening's own night session is the next trade date's opening leg and ran |
| 2011-05-31 | replacement blocks | blocks orderentry(-1, 61200, 61800); regular(-1, 61800, 0); orderentry(0, 34800, 35400); regular(0, 35400, 59400) | `ASX24-CAL-2011` | T1 | Night session to cease trading at midnight (US policy): the night session opening on the US holiday ceased at midnight Sydney, so its trade date (the following day) runs that shortened leg and then its own day |
| 2011-06-13 | early close | 07:00 | `ASX24-CAL-2011` | T1 | Day Session Closed / Night Session to open at 5:10pm: the day session is removed and the morning night leg ends at its normal 07:00 close |
| 2011-07-05 | replacement blocks | blocks orderentry(-1, 61200, 61800); regular(-1, 61800, 0); orderentry(0, 34800, 35400); regular(0, 35400, 59400) | `ASX24-CAL-2011` | T1 | Night session to cease trading at midnight (US policy): the night session opening on the US holiday ceased at midnight Sydney, so its trade date (the following day) runs that shortened leg and then its own day |
| 2011-09-06 | replacement blocks | blocks orderentry(-1, 61200, 61800); regular(-1, 61800, 0); orderentry(0, 34800, 35400); regular(0, 35400, 59400) | `ASX24-CAL-2011` | T1 | Night session to cease trading at midnight (US policy): the night session opening on the US holiday ceased at midnight Sydney, so its trade date (the following day) runs that shortened leg and then its own day |
| 2011-11-25 | replacement blocks | blocks orderentry(-1, 61200, 61800); regular(-1, 61800, 0); orderentry(0, 34800, 35400); regular(0, 35400, 59400) | `ASX24-CAL-2011` | T1 | Night session to cease trading at midnight (US policy): the night session opening on the US holiday ceased at midnight Sydney, so its trade date (the following day) runs that shortened leg and then its own day |
| 2011-12-23 | early close | 14:30 | `ASX24-CAL-2011` | T1 | Equity Products to close at 2:30pm and Night Session Closed: the day session closes 14:30 and the evening night session does not open |
| 2011-12-24 | closed | — | `ASX24-CAL-2011` | T1 | the closed Friday night session's trade date is the Saturday |
| 2011-12-25 | closed | — | `ASX24-CAL-2011` | T1 | operator Non Trading Day (the Sunday keeps its operator-stated row; no session opens into it) |
| 2011-12-26 | closed | — | `ASX24-CAL-2011` | T1 | operator Closed/Non Trading Day (day and night sessions) |
| 2011-12-27 | closed | — | `ASX24-CAL-2011` | T1 | operator Closed/Non Trading Day (day and night sessions) |
| 2011-12-28 | replacement blocks | blocks 09:40-09:50; 09:50-16:30 | `ASX24-CAL-2011` | T1 | the operator closed the holiday evening's night session; the late open removes the wrapped leg and keeps the following day's own sessions |
| 2011-12-30 | early close | 14:30 | `ASX24-CAL-2011` | T1 | Equity Products to close at 2:30pm and Night Session Closed: the day session closes 14:30 and the evening night session does not open |
| 2011-12-31 | closed | — | `ASX24-CAL-2011` | T1 | the closed Friday night session's trade date is the Saturday |

### 2012

| 2012-01-02 | closed | — | `ASX24-CAL-2012` | T1 | operator Closed/Non Trading Day (day and night sessions) |
| 2012-01-03 | replacement blocks | blocks 09:40-09:50; 09:50-16:30 | `ASX24-CAL-2012` | T1 | the operator closed the holiday evening's night session; the late open removes the wrapped leg and keeps the following day's own sessions |
| 2012-01-17 | replacement blocks | blocks orderentry(-1, 61200, 61800); regular(-1, 61800, 0); orderentry(0, 34800, 35400); regular(0, 35400, 59400) | `ASX24-CAL-2012` | T1 | Night session to cease trading at midnight (US policy): the night session opening on the US holiday ceased at midnight Sydney, so its trade date (the following day) runs that shortened leg and then its own day |
| 2012-01-26 | early close | 08:00 | `ASX24-CAL-2012` | T1 | Day Session Closed / Night Session to open at 5:10pm: the day session is removed and the morning night leg ends at its normal 08:00 close |
| 2012-02-21 | replacement blocks | blocks orderentry(-1, 61200, 61800); regular(-1, 61800, 0); orderentry(0, 34800, 35400); regular(0, 35400, 59400) | `ASX24-CAL-2012` | T1 | Night session to cease trading at midnight (US policy): the night session opening on the US holiday ceased at midnight Sydney, so its trade date (the following day) runs that shortened leg and then its own day |
| 2012-04-06 | replacement blocks | blocks orderentry(-1, 61200, 61800); regular(-1, 61800, 0) | `ASX24-CAL-2012` | T1 | Day Before Good Friday: the night session opening on the eve ceased at midnight Sydney (operator row) and Good Friday itself is Closed/Non Trading Day, so the trade date runs that shortened evening leg and no day session |
| 2012-04-07 | closed | — | `ASX24-CAL-2012` | T1 | the operator closed the Friday evening's night session, whose trade date is the Saturday |
| 2012-04-09 | closed | — | `ASX24-CAL-2012` | T1 | operator Closed/Non Trading Day (day and night sessions) |
| 2012-04-10 | replacement blocks | blocks 09:40-09:50; 09:50-16:30 | `ASX24-CAL-2012` | T1 | the operator closed the holiday evening's night session; the late open removes the wrapped leg and keeps the following day's own sessions |
| 2012-04-25 | early close | 07:00 | `ASX24-CAL-2012` | T1 | Day Session Closed / Night Session to open at 5:10pm: the day session is removed and the morning night leg ends at its normal 07:00 close |
| 2012-05-29 | replacement blocks | blocks orderentry(-1, 61200, 61800); regular(-1, 61800, 0); orderentry(0, 34800, 35400); regular(0, 35400, 59400) | `ASX24-CAL-2012` | T1 | Night session to cease trading at midnight (US policy): the night session opening on the US holiday ceased at midnight Sydney, so its trade date (the following day) runs that shortened leg and then its own day |
| 2012-06-11 | early close | 07:00 | `ASX24-CAL-2012` | T1 | Day Session Closed / Night Session to open at 5:10pm: the day session is removed and the morning night leg ends at its normal 07:00 close |
| 2012-07-05 | replacement blocks | blocks orderentry(-1, 61200, 61800); regular(-1, 61800, 0); orderentry(0, 34800, 35400); regular(0, 35400, 59400) | `ASX24-CAL-2012` | T1 | Night session to cease trading at midnight (US policy): the night session opening on the US holiday ceased at midnight Sydney, so its trade date (the following day) runs that shortened leg and then its own day |
| 2012-09-04 | replacement blocks | blocks orderentry(-1, 61200, 61800); regular(-1, 61800, 0); orderentry(0, 34800, 35400); regular(0, 35400, 59400) | `ASX24-CAL-2012` | T1 | Night session to cease trading at midnight (US policy): the night session opening on the US holiday ceased at midnight Sydney, so its trade date (the following day) runs that shortened leg and then its own day |
| 2012-11-23 | replacement blocks | blocks orderentry(-1, 61200, 61800); regular(-1, 61800, 0); orderentry(0, 34800, 35400); regular(0, 35400, 59400) | `ASX24-CAL-2012` | T1 | Night session to cease trading at midnight (US policy): the night session opening on the US holiday ceased at midnight Sydney, so its trade date (the following day) runs that shortened leg and then its own day |
| 2012-12-24 | early close | 14:30 | `ASX24-CAL-2012` | T1 | Equity Products to close at 2:30pm and Night Session Closed: the day session closes 14:30 and the evening night session does not open |
| 2012-12-25 | closed | — | `ASX24-CAL-2012` | T1 | operator Closed/Non Trading Day (day and night sessions) |
| 2012-12-26 | closed | — | `ASX24-CAL-2012` | T1 | operator Closed/Non Trading Day (day and night sessions) |
| 2012-12-27 | replacement blocks | blocks 09:40-09:50; 09:50-16:30 | `ASX24-CAL-2012` | T1 | the operator closed the holiday evening's night session; the late open removes the wrapped leg and keeps the following day's own sessions |
| 2012-12-31 | early close | 14:30 | `ASX24-CAL-2012` | T1 | Equity Products to close at 2:30pm and Night Session Closed: the day session closes 14:30 and the evening night session does not open |

### 2013

| 2013-01-01 | closed | — | `ASX24-CAL-2013` | T1 | operator Closed/Non Trading Day (day and night sessions) |
| 2013-01-02 | replacement blocks | blocks 09:40-09:50; 09:50-16:30 | `ASX24-CAL-2013` | T1 | the operator closed the holiday evening's night session; the late open removes the wrapped leg and keeps the following day's own sessions |
| 2013-01-22 | replacement blocks | blocks orderentry(-1, 61200, 61800); regular(-1, 61800, 0); orderentry(0, 34800, 35400); regular(0, 35400, 59400) | `ASX24-CAL-2013` | T1 | Night session to cease trading at midnight (US policy): the night session opening on the US holiday ceased at midnight Sydney, so its trade date (the following day) runs that shortened leg and then its own day |
| 2013-01-28 | early close | 08:00 | `ASX24-CAL-2013` | T1 | Day Session Closed / Night Session to open at 5:10pm: the day session is removed and the morning night leg ends at its normal 08:00 close |
| 2013-02-19 | replacement blocks | blocks orderentry(-1, 61200, 61800); regular(-1, 61800, 0); orderentry(0, 34800, 35400); regular(0, 35400, 59400) | `ASX24-CAL-2013` | T1 | Night session to cease trading at midnight (US policy): the night session opening on the US holiday ceased at midnight Sydney, so its trade date (the following day) runs that shortened leg and then its own day |
| 2013-03-29 | replacement blocks | blocks orderentry(-1, 61200, 61800); regular(-1, 61800, 0) | `ASX24-CAL-2013` | T1 | Day Before Good Friday: the night session opening on the eve ceased at midnight Sydney (operator row) and Good Friday itself is Closed/Non Trading Day, so the trade date runs that shortened evening leg and no day session |
| 2013-03-30 | closed | — | `ASX24-CAL-2013` | T1 | the operator closed the Friday evening's night session, whose trade date is the Saturday |
| 2013-04-01 | closed | — | `ASX24-CAL-2013` | T1 | operator Closed/Non Trading Day (day and night sessions) |
| 2013-04-02 | replacement blocks | blocks 09:40-09:50; 09:50-16:30 | `ASX24-CAL-2013` | T1 | the operator closed the holiday evening's night session; the late open removes the wrapped leg and keeps the following day's own sessions |
| 2013-04-25 | early close | 07:00 | `ASX24-CAL-2013` | T1 | Day Session Closed / Night Session to open at 5:10pm: the day session is removed and the morning night leg ends at its normal 07:00 close |
| 2013-05-28 | replacement blocks | blocks orderentry(-1, 61200, 61800); regular(-1, 61800, 0); orderentry(0, 34800, 35400); regular(0, 35400, 59400) | `ASX24-CAL-2013` | T1 | Night session to cease trading at midnight (US policy): the night session opening on the US holiday ceased at midnight Sydney, so its trade date (the following day) runs that shortened leg and then its own day |
| 2013-06-10 | early close | 07:00 | `ASX24-CAL-2013` | T1 | Day Session Closed / Night Session to open at 5:10pm: the day session is removed and the morning night leg ends at its normal 07:00 close |
| 2013-07-05 | replacement blocks | blocks orderentry(-1, 61200, 61800); regular(-1, 61800, 0); orderentry(0, 34800, 35400); regular(0, 35400, 59400) | `ASX24-CAL-2013` | T1 | Night session to cease trading at midnight (US policy): the night session opening on the US holiday ceased at midnight Sydney, so its trade date (the following day) runs that shortened leg and then its own day |
| 2013-09-03 | replacement blocks | blocks orderentry(-1, 61200, 61800); regular(-1, 61800, 0); orderentry(0, 34800, 35400); regular(0, 35400, 59400) | `ASX24-CAL-2013` | T1 | Night session to cease trading at midnight (US policy): the night session opening on the US holiday ceased at midnight Sydney, so its trade date (the following day) runs that shortened leg and then its own day |
| 2013-11-29 | replacement blocks | blocks orderentry(-1, 61200, 61800); regular(-1, 61800, 0); orderentry(0, 34800, 35400); regular(0, 35400, 59400) | `ASX24-CAL-2013` | T1 | Night session to cease trading at midnight (US policy): the night session opening on the US holiday ceased at midnight Sydney, so its trade date (the following day) runs that shortened leg and then its own day |
| 2013-12-24 | early close | 14:30 | `ASX24-CAL-2013` | T1 | Equity Products to close at 2:30pm and Night Session Closed: the day session closes 14:30 and the evening night session does not open |
| 2013-12-25 | closed | — | `ASX24-CAL-2013` | T1 | operator Closed/Non Trading Day (day and night sessions) |
| 2013-12-26 | closed | — | `ASX24-CAL-2013` | T1 | operator Closed/Non Trading Day (day and night sessions) |
| 2013-12-27 | replacement blocks | blocks 09:40-09:50; 09:50-16:30 | `ASX24-CAL-2013` | T1 | the operator closed the holiday evening's night session; the late open removes the wrapped leg and keeps the following day's own sessions |
| 2013-12-31 | early close | 14:30 | `ASX24-CAL-2013` | T1 | Equity Products to close at 2:30pm and Night Session Closed: the day session closes 14:30 and the evening night session does not open |

### 2014

| 2014-01-01 | closed | — | `ASX24-CAL-2014` | T1 | operator Closed/Non Trading Day (day and night sessions) |
| 2014-01-02 | replacement blocks | blocks 09:40-09:50; 09:50-16:30 | `ASX24-CAL-2014` | T1 | the operator closed the holiday evening's night session; the late open removes the wrapped leg and keeps the following day's own sessions |
| 2014-01-21 | replacement blocks | blocks orderentry(-1, 61200, 61800); regular(-1, 61800, 0); orderentry(0, 34800, 35400); regular(0, 35400, 59400) | `ASX24-CAL-2014` | T1 | Night session to cease trading at midnight (US policy): the night session opening on the US holiday ceased at midnight Sydney, so its trade date (the following day) runs that shortened leg and then its own day |
| 2014-01-27 | early close | 08:00 | `ASX24-CAL-2014` | T1 | Day Session Closed / Night Session to open at 5:10pm: the day session is removed and the morning night leg ends at its normal 08:00 close |
| 2014-02-18 | replacement blocks | blocks orderentry(-1, 61200, 61800); regular(-1, 61800, 0); orderentry(0, 34800, 35400); regular(0, 35400, 59400) | `ASX24-CAL-2014` | T1 | Night session to cease trading at midnight (US policy): the night session opening on the US holiday ceased at midnight Sydney, so its trade date (the following day) runs that shortened leg and then its own day |
| 2014-04-18 | replacement blocks | blocks orderentry(-1, 61200, 61800); regular(-1, 61800, 0) | `ASX24-CAL-2014` | T1 | Day Before Good Friday: the night session opening on the eve ceased at midnight Sydney (operator row) and Good Friday itself is Closed/Non Trading Day, so the trade date runs that shortened evening leg and no day session |
| 2014-04-19 | closed | — | `ASX24-CAL-2014` | T1 | the operator closed the Friday evening's night session, whose trade date is the Saturday |
| 2014-04-21 | closed | — | `ASX24-CAL-2014` | T1 | operator Closed/Non Trading Day (day and night sessions) |
| 2014-04-22 | replacement blocks | blocks 09:40-09:50; 09:50-16:30 | `ASX24-CAL-2014` | T1 | the operator closed the holiday evening's night session; the late open removes the wrapped leg and keeps the following day's own sessions |
| 2014-04-25 | early close | 07:00 | `ASX24-CAL-2014` | T1 | Day Session Closed / Night Session to open at 5:10pm: the day session is removed and the morning night leg ends at its normal 07:00 close |
| 2014-05-27 | replacement blocks | blocks orderentry(-1, 61200, 61800); regular(-1, 61800, 0); orderentry(0, 34800, 35400); regular(0, 35400, 59400) | `ASX24-CAL-2014` | T1 | Night session to cease trading at midnight (US policy): the night session opening on the US holiday ceased at midnight Sydney, so its trade date (the following day) runs that shortened leg and then its own day |
| 2014-06-09 | early close | 07:00 | `ASX24-CAL-2014` | T1 | Day Session Closed / Night Session to open at 5:10pm: the day session is removed and the morning night leg ends at its normal 07:00 close |
| 2014-07-05 | replacement blocks | blocks orderentry(-1, 61200, 61800); regular(-1, 61800, 0) | `ASX24-CAL-2014` | T1 | Night session to cease trading at midnight (US policy): the night session opening on the US holiday ceased at midnight Sydney, so its trade date (the following day) runs that shortened leg and then its own day alone (a Saturday) |
| 2014-09-02 | replacement blocks | blocks orderentry(-1, 61200, 61800); regular(-1, 61800, 0); orderentry(0, 34800, 35400); regular(0, 35400, 59400) | `ASX24-CAL-2014` | T1 | Night session to cease trading at midnight (US policy): the night session opening on the US holiday ceased at midnight Sydney, so its trade date (the following day) runs that shortened leg and then its own day |
| 2014-11-28 | replacement blocks | blocks orderentry(-1, 61200, 61800); regular(-1, 61800, 0); orderentry(0, 34800, 35400); regular(0, 35400, 59400) | `ASX24-CAL-2014` | T1 | Night session to cease trading at midnight (US policy): the night session opening on the US holiday ceased at midnight Sydney, so its trade date (the following day) runs that shortened leg and then its own day |
| 2014-12-24 | early close | 14:30 | `ASX24-CAL-2014` | T1 | Equity Products to close at 2:30pm and Night Session Closed: the day session closes 14:30 and the evening night session does not open |
| 2014-12-25 | closed | — | `ASX24-CAL-2014` | T1 | operator Closed/Non Trading Day (day and night sessions) |
| 2014-12-26 | closed | — | `ASX24-CAL-2014` | T1 | operator Closed/Non Trading Day (day and night sessions) |
| 2014-12-27 | closed | — | `ASX24-CAL-2014` | T1 | the operator closed the Friday evening's night session, whose trade date is the Saturday |
| 2014-12-31 | early close | 14:30 | `ASX24-CAL-2014` | T1 | Equity Products to close at 2:30pm and Night Session Closed: the day session closes 14:30 and the evening night session does not open |

### 2015

| 2015-01-01 | closed | — | `ASX24-CAL-2015` | T1 | operator Closed/Non Trading Day (day and night sessions) |
| 2015-01-02 | replacement blocks | blocks 09:40-09:50; 09:50-16:30 | `ASX24-CAL-2015` | T1 | the operator closed the holiday evening's night session; the late open removes the wrapped leg and keeps the following day's own sessions |
| 2015-01-20 | replacement blocks | blocks orderentry(-1, 61200, 61800); regular(-1, 61800, 0); orderentry(0, 34800, 35400); regular(0, 35400, 59400) | `ASX24-CAL-2015` | T1 | Night session to cease trading at midnight (US policy): the night session opening on the US holiday ceased at midnight Sydney, so its trade date (the following day) runs that shortened leg and then its own day |
| 2015-01-26 | early close | 08:00 | `ASX24-CAL-2015` | T1 | Day Session Closed / Night Session to open at 5:10pm: the day session is removed and the morning night leg ends at its normal 08:00 close |
| 2015-02-17 | replacement blocks | blocks orderentry(-1, 61200, 61800); regular(-1, 61800, 0); orderentry(0, 34800, 35400); regular(0, 35400, 59400) | `ASX24-CAL-2015` | T1 | Night session to cease trading at midnight (US policy): the night session opening on the US holiday ceased at midnight Sydney, so its trade date (the following day) runs that shortened leg and then its own day |
| 2015-04-03 | replacement blocks | blocks orderentry(-1, 61200, 61800); regular(-1, 61800, 0) | `ASX24-CAL-2015` | T1 | Day Before Good Friday: the night session opening on the eve ceased at midnight Sydney (operator row) and Good Friday itself is Closed/Non Trading Day, so the trade date runs that shortened evening leg and no day session |
| 2015-04-04 | closed | — | `ASX24-CAL-2015` | T1 | the operator closed the Friday evening's night session, whose trade date is the Saturday |
| 2015-04-06 | closed | — | `ASX24-CAL-2015` | T1 | operator Closed/Non Trading Day (day and night sessions) |
| 2015-04-07 | replacement blocks | blocks 09:40-09:50; 09:50-16:30 | `ASX24-CAL-2015` | T1 | the operator closed the holiday evening's night session; the late open removes the wrapped leg and keeps the following day's own sessions |
| 2015-05-26 | replacement blocks | blocks orderentry(-1, 61200, 61800); regular(-1, 61800, 0); orderentry(0, 34800, 35400); regular(0, 35400, 59400) | `ASX24-CAL-2015` | T1 | Night session to cease trading at midnight (US policy): the night session opening on the US holiday ceased at midnight Sydney, so its trade date (the following day) runs that shortened leg and then its own day |
| 2015-06-08 | early close | 07:00 | `ASX24-CAL-2015` | T1 | Day Session Closed / Night Session to open at 5:10pm: the day session is removed and the morning night leg ends at its normal 07:00 close |
| 2015-07-04 | replacement blocks | blocks orderentry(-1, 61200, 61800); regular(-1, 61800, 0) | `ASX24-CAL-2015` | T1 | Night session to cease trading at midnight (US policy): the night session opening on the US holiday ceased at midnight Sydney, so its trade date (the following day) runs that shortened leg and then its own day alone (a Saturday) |
| 2015-09-08 | replacement blocks | blocks orderentry(-1, 61200, 61800); regular(-1, 61800, 0); orderentry(0, 34800, 35400); regular(0, 35400, 59400) | `ASX24-CAL-2015` | T1 | Night session to cease trading at midnight (US policy): the night session opening on the US holiday ceased at midnight Sydney, so its trade date (the following day) runs that shortened leg and then its own day |
| 2015-11-27 | replacement blocks | blocks orderentry(-1, 61200, 61800); regular(-1, 61800, 0); orderentry(0, 34800, 35400); regular(0, 35400, 59400) | `ASX24-CAL-2015` | T1 | Night session to cease trading at midnight (US policy): the night session opening on the US holiday ceased at midnight Sydney, so its trade date (the following day) runs that shortened leg and then its own day |
| 2015-12-24 | early close | 14:30 | `ASX24-CAL-2015` | T1 | Equity Products to close at 2:30pm and Night Session Closed: the day session closes 14:30 and the evening night session does not open |
| 2015-12-25 | closed | — | `ASX24-CAL-2015` | T1 | operator Closed/Non Trading Day (day and night sessions) |
| 2015-12-26 | closed | — | `ASX24-CAL-2015` | T1 | the operator closed the Friday evening's night session, whose trade date is the Saturday |
| 2015-12-28 | closed | — | `ASX24-CAL-2015` | T1 | operator Closed/Non Trading Day (day and night sessions) |
| 2015-12-29 | replacement blocks | blocks 09:40-09:50; 09:50-16:30 | `ASX24-CAL-2015` | T1 | the operator closed the holiday evening's night session; the late open removes the wrapped leg and keeps the following day's own sessions |
| 2015-12-31 | early close | 14:30 | `ASX24-CAL-2015` | T1 | Equity Products to close at 2:30pm and Night Session Closed: the day session closes 14:30 and the evening night session does not open |

### 2016

| 2016-01-01 | closed | — | `ASX24-CAL-2016` | T1 | operator Closed/Non Trading Day (day and night sessions) |
| 2016-01-02 | closed | — | `ASX24-CAL-2016` | T1 | the operator closed the Friday evening's night session, whose trade date is the Saturday |
| 2016-01-26 | early close | 08:00 | `ASX24-CAL-2016` | T1 | Day Session Closed / Night Session to open at 5:00pm: the day session is removed and the morning night leg ends at its normal 08:00 close |
| 2016-01-27 | replacement blocks | blocks orderentry(-1, 60600, 61200); regular(-1, 61200, 28800); orderentry(0, 34800, 35400); regular(0, 35400, 59400) | `ASX24-CAL-2016` | T1 | the capture prints the night session opening 5:00pm on the holiday, so the next trade date's opening leg opened at 17:00 and its own day ran normally |
| 2016-03-25 | replacement blocks | blocks orderentry(-1, 61200, 61800); regular(-1, 61800, 0) | `ASX24-CAL-2016` | T1 | Day Before Good Friday: the night session opening on the eve ceased at midnight Sydney (operator row) and Good Friday itself is Closed/Non Trading Day, so the trade date runs that shortened evening leg and no day session |
| 2016-03-26 | closed | — | `ASX24-CAL-2016` | T1 | the operator closed the Friday evening's night session, whose trade date is the Saturday |
| 2016-03-28 | closed | — | `ASX24-CAL-2016` | T1 | operator Closed/Non Trading Day (day and night sessions) |
| 2016-03-29 | replacement blocks | blocks 09:40-09:50; 09:50-16:30 | `ASX24-CAL-2016` | T1 | the operator closed the holiday evening's night session; the late open removes the wrapped leg and keeps the following day's own sessions |
| 2016-04-25 | early close | 07:00 | `ASX24-CAL-2016` | T1 | Day Session Closed / Night Session to open at 5:00pm: the day session is removed and the morning night leg ends at its normal 07:00 close |
| 2016-04-26 | replacement blocks | blocks orderentry(-1, 60600, 61200); regular(-1, 61200, 25200); orderentry(0, 34800, 35400); regular(0, 35400, 59400) | `ASX24-CAL-2016` | T1 | the capture prints the night session opening 5:00pm on the holiday, so the next trade date's opening leg opened at 17:00 (US daylight time, a 07:00 close) and its own day ran normally |
| 2016-06-13 | early close | 07:00 | `ASX24-CAL-2016` | T1 | Day Session Closed / Night Session to open at 5:10pm: the day session is removed and the morning night leg ends at its normal 07:00 close |
| 2016-12-23 | early close | 14:30 | `ASX24-CAL-2016` | T1 | Equity Products to close at 2:30pm and Night Session Closed: the day session closes 14:30 and the evening night session does not open |
| 2016-12-24 | closed | — | `ASX24-CAL-2016` | T1 | the closed Friday night session's trade date is the Saturday |
| 2016-12-26 | closed | — | `ASX24-CAL-2016` | T1 | operator Closed/Non Trading Day (day and night sessions) |
| 2016-12-27 | closed | — | `ASX24-CAL-2016` | T1 | operator Closed/Non Trading Day (day and night sessions) |
| 2016-12-28 | replacement blocks | blocks 09:40-09:50; 09:50-16:30 | `ASX24-CAL-2016` | T1 | the operator closed the holiday evening's night session; the late open removes the wrapped leg and keeps the following day's own sessions |
| 2016-12-30 | early close | 14:30 | `ASX24-CAL-2016` | T1 | Equity Products to close at 2:30pm and Night Session Closed: the day session closes 14:30 and the evening night session does not open |
| 2016-12-31 | closed | — | `ASX24-CAL-2016` | T1 | the closed Friday night session's trade date is the Saturday |

### 2017

| 2017-01-02 | closed | — | `ASX24-CAL-2017` | T1 | operator Closed/Non Trading Day (day and night sessions) |
| 2017-01-03 | replacement blocks | blocks 09:40-09:50; 09:50-16:30 | `ASX24-CAL-2017` | T1 | the operator closed the holiday evening's night session; the late open removes the wrapped leg and keeps the following day's own sessions |
| 2017-01-26 | early close | 08:00 | `ASX24-CAL-2017` | T1 | Day Session Closed / Night Session to open at 5:10pm: the day session is removed and the morning night leg ends at its normal 08:00 close |
| 2017-04-14 | replacement blocks | blocks orderentry(-1, 61200, 61800); regular(-1, 61800, 0) | `ASX24-CAL-2017` | T1 | Day Before Good Friday: the night session opening on the eve ceased at midnight Sydney (operator row) and Good Friday itself is Closed/Non Trading Day, so the trade date runs that shortened evening leg and no day session |
| 2017-04-15 | closed | — | `ASX24-CAL-2017` | T1 | the operator closed the Friday evening's night session, whose trade date is the Saturday |
| 2017-04-17 | closed | — | `ASX24-CAL-2017` | T1 | operator Closed/Non Trading Day (day and night sessions) |
| 2017-04-18 | replacement blocks | blocks 09:40-09:50; 09:50-16:30 | `ASX24-CAL-2017` | T1 | the operator closed the holiday evening's night session; the late open removes the wrapped leg and keeps the following day's own sessions |
| 2017-04-25 | early close | 07:00 | `ASX24-CAL-2017` | T1 | Day Session Closed / Night Session to open at 5:10pm: the day session is removed and the morning night leg ends at its normal 07:00 close |
| 2017-06-12 | early close | 07:00 | `ASX24-CAL-2017` | T1 | Day Session Closed / Night Session to open at 5:10pm: the day session is removed and the morning night leg ends at its normal 07:00 close |
| 2017-12-23 | closed | — | `ASX24-CAL-2017` | T1 | Day Session Normal / Night Session Closed: the closed Friday night session's trade date is the Saturday |
| 2017-12-25 | closed | — | `ASX24-CAL-2017` | T1 | operator Closed/Non Trading Day (day and night sessions) |
| 2017-12-26 | closed | — | `ASX24-CAL-2017` | T1 | operator Closed/Non Trading Day (day and night sessions) |
| 2017-12-27 | replacement blocks | blocks 09:40-09:50; 09:50-16:30 | `ASX24-CAL-2017` | T1 | the operator closed the holiday evening's night session; the late open removes the wrapped leg and keeps the following day's own sessions |
| 2017-12-30 | closed | — | `ASX24-CAL-2017` | T1 | Day Session Normal / Night Session Closed: the closed Friday night session's trade date is the Saturday |

### 2018

| 2018-01-01 | closed | — | `ASX24-CAL-2018` | T1 | operator Closed/Non Trading Day (day and night sessions) |
| 2018-01-02 | replacement blocks | blocks 09:40-09:50; 09:50-16:30 | `ASX24-CAL-2018` | T1 | the operator closed the holiday evening's night session; the late open removes the wrapped leg and keeps the following day's own sessions |
| 2018-01-26 | early close | 08:00 | `ASX24-CAL-2018` | T1 | Day Session Closed / Night Session to open at 5:10pm: the day session is removed and the morning night leg ends at its normal 08:00 close |
| 2018-03-30 | replacement blocks | blocks orderentry(-1, 61200, 61800); regular(-1, 61800, 0) | `ASX24-CAL-2018` | T1 | Day Before Good Friday: the night session opening on the eve ceased at midnight Sydney (operator row) and Good Friday itself is Closed/Non Trading Day, so the trade date runs that shortened evening leg and no day session |
| 2018-03-31 | closed | — | `ASX24-CAL-2018` | T1 | the operator closed the Friday evening's night session, whose trade date is the Saturday |
| 2018-04-02 | closed | — | `ASX24-CAL-2018` | T1 | operator Closed/Non Trading Day (day and night sessions) |
| 2018-04-03 | replacement blocks | blocks 09:40-09:50; 09:50-16:30 | `ASX24-CAL-2018` | T1 | the operator closed the holiday evening's night session; the late open removes the wrapped leg and keeps the following day's own sessions |
| 2018-04-25 | early close | 07:00 | `ASX24-CAL-2018` | T1 | Day Session Closed / Night Session to open at 5:10pm: the day session is removed and the morning night leg ends at its normal 07:00 close |
| 2018-06-11 | early close | 07:00 | `ASX24-CAL-2018` | T1 | Day Session Closed / Night Session to open at 5:10pm: the day session is removed and the morning night leg ends at its normal 07:00 close |
| 2018-12-24 | early close | 14:30 | `ASX24-CAL-2018` | T1 | Equity Products to close at 2:30pm and Night Session Closed: the day session closes 14:30 and the evening night session does not open |
| 2018-12-25 | closed | — | `ASX24-CAL-2018` | T1 | operator Closed/Non Trading Day (day and night sessions) |
| 2018-12-26 | closed | — | `ASX24-CAL-2018` | T1 | operator Closed/Non Trading Day (day and night sessions) |
| 2018-12-27 | replacement blocks | blocks 09:40-09:50; 09:50-16:30 | `ASX24-CAL-2018` | T1 | the operator closed the holiday evening's night session; the late open removes the wrapped leg and keeps the following day's own sessions |
| 2018-12-31 | early close | 14:30 | `ASX24-CAL-2018` | T1 | Equity Products to close at 2:30pm and Night Session Closed: the day session closes 14:30 and the evening night session does not open |

### 2019

| 2019-01-01 | closed | — | `ASX24-CAL-2019` | T1 | operator Closed/Non Trading Day (day and night sessions) |
| 2019-01-02 | replacement blocks | blocks 09:40-09:50; 09:50-16:30 | `ASX24-CAL-2019` | T1 | the operator closed the holiday evening's night session; the late open removes the wrapped leg and keeps the following day's own sessions |
| 2019-01-28 | early close | 08:00 | `ASX24-CAL-2019` | T1 | Day Session Closed / Night Session to open at 5:10pm: the day session is removed and the morning night leg ends at its normal 08:00 close |
| 2019-04-19 | replacement blocks | blocks orderentry(-1, 61200, 61800); regular(-1, 61800, 0) | `ASX24-CAL-2019` | T1 | Day Before Good Friday: the night session opening on the eve ceased at midnight Sydney (operator row) and Good Friday itself is Closed/Non Trading Day, so the trade date runs that shortened evening leg and no day session |
| 2019-04-20 | closed | — | `ASX24-CAL-2019` | T1 | the operator closed the Friday evening's night session, whose trade date is the Saturday |
| 2019-04-22 | closed | — | `ASX24-CAL-2019` | T1 | operator Closed/Non Trading Day (day and night sessions) |
| 2019-04-23 | replacement blocks | blocks 09:40-09:50; 09:50-16:30 | `ASX24-CAL-2019` | T1 | the operator closed the holiday evening's night session; the late open removes the wrapped leg and keeps the following day's own sessions |
| 2019-04-25 | early close | 07:00 | `ASX24-CAL-2019` | T1 | Day Session Closed / Night Session to open at 5:10pm: the day session is removed and the morning night leg ends at its normal 07:00 close |
| 2019-06-10 | early close | 07:00 | `ASX24-CAL-2019` | T1 | Day Session Closed / Night Session to open at 5:10pm: the day session is removed and the morning night leg ends at its normal 07:00 close |
| 2019-12-24 | early close | 14:30 | `ASX24-CAL-2019` | T1 | Equity Products to close at 2:30pm and Night Session Closed: the day session closes 14:30 and the evening night session does not open |
| 2019-12-25 | closed | — | `ASX24-CAL-2019` | T1 | operator Closed/Non Trading Day (day and night sessions) |
| 2019-12-26 | closed | — | `ASX24-CAL-2019` | T1 | operator Closed/Non Trading Day (day and night sessions) |
| 2019-12-27 | replacement blocks | blocks 09:40-09:50; 09:50-16:30 | `ASX24-CAL-2019` | T1 | the operator closed the holiday evening's night session; the late open removes the wrapped leg and keeps the following day's own sessions |
| 2019-12-31 | early close | 14:30 | `ASX24-CAL-2019` | T1 | Equity Products to close at 2:30pm and Night Session Closed: the day session closes 14:30 and the evening night session does not open |

### 2020

| 2020-01-01 | closed | — | `ASX24-CAL-2020` | T1 | operator Closed/Non Trading Day (day and night sessions) |
| 2020-01-02 | replacement blocks | blocks 09:40-09:50; 09:50-16:30 | `ASX24-CAL-2020` | T1 | the operator closed the holiday evening's night session; the late open removes the wrapped leg and keeps the following day's own sessions |
| 2020-01-27 | early close | 08:00 | `ASX24-CAL-2020` | T1 | Day Session Closed / Night Session to open at 5:10pm: the day session is removed and the morning night leg ends at its normal 08:00 close |
| 2020-04-10 | replacement blocks | blocks orderentry(-1, 61200, 61800); regular(-1, 61800, 0) | `ASX24-CAL-2020` | T1 | Day Before Good Friday: the night session opening on the eve ceased at midnight Sydney (operator row) and Good Friday itself is Closed/Non Trading Day, so the trade date runs that shortened evening leg and no day session |
| 2020-04-11 | closed | — | `ASX24-CAL-2020` | T1 | the operator closed the Friday evening's night session, whose trade date is the Saturday |
| 2020-04-13 | closed | — | `ASX24-CAL-2020` | T1 | operator Closed/Non Trading Day (day and night sessions) |
| 2020-04-14 | replacement blocks | blocks 09:40-09:50; 09:50-16:30 | `ASX24-CAL-2020` | T1 | the operator closed the holiday evening's night session; the late open removes the wrapped leg and keeps the following day's own sessions |
| 2020-04-25 | closed | — | `ASX24-CAL-2020` | T1 | operator Closed/Non Trading Day (day and night sessions) |
| 2020-06-08 | early close | 07:00 | `ASX24-CAL-2020` | T1 | Day Session Closed / Night Session to open at 5:10pm: the day session is removed and the morning night leg ends at its normal 07:00 close |
| 2020-12-24 | early close | 14:30 | `ASX24-CAL-2020` | T1 | Equity Products to close at 2:30pm and Night Session Closed: the day session closes 14:30 and the evening night session does not open |
| 2020-12-25 | closed | — | `ASX24-CAL-2020` | T1 | operator Closed/Non Trading Day (day and night sessions) |
| 2020-12-26 | closed | — | `ASX24-CAL-2020` | T1 | the operator closed the Friday evening's night session, whose trade date is the Saturday |
| 2020-12-28 | closed | — | `ASX24-CAL-2020` | T1 | operator Closed/Non Trading Day (day and night sessions) |
| 2020-12-29 | replacement blocks | blocks 09:40-09:50; 09:50-16:30 | `ASX24-CAL-2020` | T1 | the operator closed the holiday evening's night session; the late open removes the wrapped leg and keeps the following day's own sessions |
| 2020-12-31 | early close | 14:30 | `ASX24-CAL-2020` | T1 | Equity Products to close at 2:30pm and Night Session Closed: the day session closes 14:30 and the evening night session does not open |

### 2021

| 2021-01-01 | closed | — | `ASX24-LIVE-2021-01` | T1 | operator Closed/Non Trading Day (day and night sessions) |
| 2021-01-02 | closed | — | `ASX24-LIVE-2021-01` | T1 | the operator closed the Friday evening's night session, whose trade date is the Saturday |
| 2021-01-26 | early close | 08:00 | `ASX24-LIVE-2021-01` | T1 | Day Session Closed / Night Session to open at 5:10pm: the day session is removed and the morning night leg ends at its normal 08:00 close |
| 2021-04-02 | replacement blocks | blocks orderentry(-1, 61200, 61800); regular(-1, 61800, 0) | `ASX24-LIVE-2021-01` | T1 | Day Before Good Friday: the night session opening on the eve ceased at midnight Sydney (operator row) and Good Friday itself is Closed/Non Trading Day, so the trade date runs that shortened evening leg and no day session |
| 2021-04-03 | closed | — | `ASX24-LIVE-2021-01` | T1 | the operator closed the Friday evening's night session, whose trade date is the Saturday |
| 2021-04-05 | closed | — | `ASX24-LIVE-2021-01` | T1 | operator Closed/Non Trading Day (day and night sessions) |
| 2021-04-06 | replacement blocks | blocks 09:40-09:50; 09:50-16:30 | `ASX24-LIVE-2021-01` | T1 | the operator closed the holiday evening's night session; the late open removes the wrapped leg and keeps the following day's own sessions |
| 2021-04-25 | closed | — | `ASX24-LIVE-2021-01` | T1 | operator Closed/Non Trading Day (day and night sessions) |
| 2021-04-26 | replacement blocks | blocks 09:40-09:50; 09:50-16:30 | `ASX24-LIVE-2021-01` | T1 | the operator closed the Sunday evening's night session; the late open removes the wrapped leg and keeps Monday's own sessions |
| 2021-06-14 | early close | 07:00 | `ASX24-LIVE-2021-01` | T1 | Day Session Closed / Night Session to open at 5:10pm: the day session is removed and the morning night leg ends at its normal 07:00 close |
| 2021-12-24 | early close | 14:30 | `ASX24-LIVE-2021-01` | T1 | Equity Products to close at 2:30pm and Night Session Closed: the day session closes 14:30 and the evening night session does not open |
| 2021-12-25 | closed | — | `ASX24-LIVE-2021-01` | T1 | the closed Friday night session's trade date is the Saturday |
| 2021-12-27 | closed | — | `ASX24-LIVE-2021-01` | T1 | operator Closed/Non Trading Day (day and night sessions) |
| 2021-12-28 | closed | — | `ASX24-LIVE-2021-01` | T1 | operator Closed/Non Trading Day (day and night sessions) |
| 2021-12-29 | replacement blocks | blocks 09:40-09:50; 09:50-16:30 | `ASX24-LIVE-2021-01` | T1 | the operator closed the holiday evening's night session; the late open removes the wrapped leg and keeps the following day's own sessions |
| 2021-12-31 | early close | 14:30 | `ASX24-LIVE-2021-01` | T1 | Equity Products to close at 2:30pm and Night Session Closed: the day session closes 14:30 and the evening night session does not open |

### 2022

| 2022-01-01 | closed | — | `ASX24-LIVE-2021-01` | T1 | the closed Friday night session's trade date is the Saturday |
| 2022-01-03 | closed | — | `ASX24-LIVE-2022-12` | T1 | operator Closed/Non Trading Day (day and night sessions) |
| 2022-01-04 | replacement blocks | blocks 09:40-09:50; 09:50-16:30 | `ASX24-LIVE-2022-12` | T1 | the operator closed the holiday evening's night session; the late open removes the wrapped leg and keeps the following day's own sessions |
| 2022-01-26 | early close | 08:00 | `ASX24-LIVE-2022-12` | T1 | Day Session Closed / Night Session to open at 5:10pm: the day session is removed and the morning night leg ends at its normal 08:00 close |
| 2022-04-15 | replacement blocks | blocks orderentry(-1, 61200, 61800); regular(-1, 61800, 0) | `ASX24-LIVE-2022-12` | T1 | Day Before Good Friday: the night session opening on the eve ceased at midnight Sydney (operator row) and Good Friday itself is Closed/Non Trading Day, so the trade date runs that shortened evening leg and no day session |
| 2022-04-16 | closed | — | `ASX24-LIVE-2022-12` | T1 | the operator closed the Friday evening's night session, whose trade date is the Saturday |
| 2022-04-18 | closed | — | `ASX24-LIVE-2022-12` | T1 | operator Closed/Non Trading Day (day and night sessions) |
| 2022-04-19 | replacement blocks | blocks 09:40-09:50; 09:50-16:30 | `ASX24-LIVE-2022-12` | T1 | the operator closed the holiday evening's night session; the late open removes the wrapped leg and keeps the following day's own sessions |
| 2022-04-25 | early close | 07:00 | `ASX24-LIVE-2022-12` | T1 | Day Session Closed / Night Session to open at 5:10pm: the day session is removed and the morning night leg ends at its normal 07:00 close |
| 2022-06-13 | early close | 07:00 | `ASX24-LIVE-2022-12` | T1 | Day Session Closed / Night Session to open at 5:10pm: the day session is removed and the morning night leg ends at its normal 07:00 close |
| 2022-09-22 | early close | 07:00 | `ASX24-LIVE-2022-12` | T1 | Day Session Closed / Night Session to open at 5:10pm: the day session is removed and the morning night leg ends at its normal 07:00 close |
| 2022-12-24 | closed | — | `ASX24-LIVE-2022-12` | T1 | Day Session Normal / Night Session Closed: the closed Friday night session's trade date is the Saturday |
| 2022-12-26 | closed | — | `ASX24-LIVE-2022-12` | T1 | operator Closed/Non Trading Day (day and night sessions) |
| 2022-12-27 | closed | — | `ASX24-LIVE-2022-12` | T1 | operator Closed/Non Trading Day (day and night sessions) |
| 2022-12-28 | replacement blocks | blocks 09:40-09:50; 09:50-16:30 | `ASX24-LIVE-2022-12` | T1 | the operator closed the holiday evening's night session; the late open removes the wrapped leg and keeps the following day's own sessions |
| 2022-12-31 | closed | — | `ASX24-LIVE-2022-12` | T1 | Day Session Normal / Night Session Closed: the closed Friday night session's trade date is the Saturday |

### 2023

| 2023-01-02 | closed | — | `ASX24-LIVE-2022-12` | T1 | operator Closed/Non Trading Day (day and night sessions) |
| 2023-01-03 | replacement blocks | blocks 09:40-09:50; 09:50-16:30 | `ASX24-LIVE-2022-12` | T1 | the operator closed the holiday evening's night session; the late open removes the wrapped leg and keeps the following day's own sessions |
| 2023-01-26 | early close | 08:00 | `ASX24-LIVE-2022-12` | T1 | Day Session Closed / Night Session to open at 5:10pm: the day session is removed and the morning night leg ends at its normal 08:00 close |
| 2023-04-07 | replacement blocks | blocks orderentry(-1, 61200, 61800); regular(-1, 61800, 0) | `ASX24-LIVE-2022-12` | T1 | Day Before Good Friday: the night session opening on the eve ceased at midnight Sydney (operator row) and Good Friday itself is Closed/Non Trading Day, so the trade date runs that shortened evening leg and no day session |
| 2023-04-08 | closed | — | `ASX24-LIVE-2022-12` | T1 | the operator closed the Friday evening's night session, whose trade date is the Saturday |
| 2023-04-10 | closed | — | `ASX24-LIVE-2022-12` | T1 | operator Closed/Non Trading Day (day and night sessions) |
| 2023-04-11 | replacement blocks | blocks 09:40-09:50; 09:50-16:30 | `ASX24-LIVE-2022-12` | T1 | the operator closed the holiday evening's night session; the late open removes the wrapped leg and keeps the following day's own sessions |
| 2023-04-25 | early close | 07:00 | `ASX24-LIVE-2022-12` | T1 | Day Session Closed / Night Session to open at 5:10pm: the day session is removed and the morning night leg ends at its normal 07:00 close |
| 2023-06-12 | early close | 07:00 | `ASX24-LIVE-2022-12` | T1 | Day Session Closed / Night Session to open at 5:10pm: the day session is removed and the morning night leg ends at its normal 07:00 close |
| 2023-12-23 | closed | — | `ASX24-LIVE-2022-12` | T1 | Day Session Normal / Night Session Closed: the closed Friday night session's trade date is the Saturday |
| 2023-12-25 | closed | — | `ASX24-LIVE-2022-12` | T1 | operator Closed/Non Trading Day (day and night sessions) |
| 2023-12-26 | closed | — | `ASX24-LIVE-2022-12` | T1 | operator Closed/Non Trading Day (day and night sessions) |
| 2023-12-27 | replacement blocks | blocks 09:40-09:50; 09:50-16:30 | `ASX24-LIVE-2022-12` | T1 | the operator closed the holiday evening's night session; the late open removes the wrapped leg and keeps the following day's own sessions |
| 2023-12-30 | closed | — | `ASX24-LIVE-2022-12` | T1 | Day Session Normal / Night Session Closed: the closed Friday night session's trade date is the Saturday |

### 2024

| 2024-01-01 | closed | — | `ASX24-LIVE-2023-12` | T1 | operator Closed/Non Trading Day (day and night sessions) |
| 2024-01-02 | replacement blocks | blocks 09:40-09:50; 09:50-16:30 | `ASX24-LIVE-2023-12` | T1 | the operator closed the holiday evening's night session; the late open removes the wrapped leg and keeps the following day's own sessions |
| 2024-01-26 | early close | 08:00 | `ASX24-LIVE-2023-12` | T1 | Day Session Closed / Night Session to open at 5:10pm: the day session is removed and the morning night leg ends at its normal 08:00 close |
| 2024-03-29 | replacement blocks | blocks orderentry(-1, 61200, 61800); regular(-1, 61800, 0) | `ASX24-LIVE-2023-12` | T1 | Day Before Good Friday: the night session opening on the eve ceased at midnight Sydney (operator row) and Good Friday itself is Closed/Non Trading Day, so the trade date runs that shortened evening leg and no day session |
| 2024-03-30 | closed | — | `ASX24-LIVE-2023-12` | T1 | the operator closed the Friday evening's night session, whose trade date is the Saturday |
| 2024-04-01 | closed | — | `ASX24-LIVE-2023-12` | T1 | operator Closed/Non Trading Day (day and night sessions) |
| 2024-04-02 | replacement blocks | blocks 09:40-09:50; 09:50-16:30 | `ASX24-LIVE-2023-12` | T1 | the operator closed the holiday evening's night session; the late open removes the wrapped leg and keeps the following day's own sessions |
| 2024-04-25 | early close | 07:00 | `ASX24-LIVE-2023-12` | T1 | Day Session Closed / Night Session to open at 5:10pm: the day session is removed and the morning night leg ends at its normal 07:00 close |
| 2024-06-10 | early close | 07:00 | `ASX24-LIVE-2023-12` | T1 | Day Session Closed / Night Session to open at 5:10pm: the day session is removed and the morning night leg ends at its normal 07:00 close |
| 2024-12-25 | closed | — | `ASX24-LIVE-2023-12` | T1 | operator Closed/Non Trading Day (day and night sessions) |
| 2024-12-26 | closed | — | `ASX24-LIVE-2023-12` | T1 | operator Closed/Non Trading Day (day and night sessions) |
| 2024-12-27 | replacement blocks | blocks 09:40-09:50; 09:50-16:30 | `ASX24-LIVE-2023-12` | T1 | the operator closed the holiday evening's night session; the late open removes the wrapped leg and keeps the following day's own sessions |

### 2025

| 2025-01-01 | closed | — | `ASX24-LIVE-2025-03` | T1 | operator Closed/Non Trading Day (day and night sessions) |
| 2025-01-02 | replacement blocks | blocks 09:40-09:50; 09:50-16:30 | `ASX24-LIVE-2025-03` | T1 | the operator closed the holiday evening's night session; the late open removes the wrapped leg and keeps the following day's own sessions |
| 2025-01-27 | early close | 08:00 | `ASX24-LIVE-2025-03` | T1 | Day Session Closed / Night Session to open at 5:10pm: the day session is removed and the morning night leg ends at its normal 08:00 close |
| 2025-04-18 | replacement blocks | blocks orderentry(-1, 61200, 61800); regular(-1, 61800, 0) | `ASX24-LIVE-2025-03` | T1 | Day Before Good Friday: the night session opening on the eve ceased at midnight Sydney (operator row) and Good Friday itself is Closed/Non Trading Day, so the trade date runs that shortened evening leg and no day session |
| 2025-04-19 | closed | — | `ASX24-LIVE-2025-03` | T1 | the operator closed the Friday evening's night session, whose trade date is the Saturday |
| 2025-04-21 | closed | — | `ASX24-LIVE-2025-03` | T1 | operator Closed/Non Trading Day (day and night sessions) |
| 2025-04-22 | replacement blocks | blocks 09:40-09:50; 09:50-16:30 | `ASX24-LIVE-2025-03` | T1 | the operator closed the holiday evening's night session; the late open removes the wrapped leg and keeps the following day's own sessions |
| 2025-04-25 | early close | 07:00 | `ASX24-LIVE-2025-03` | T1 | Day Session Closed / Night Session to open at 5:10pm: the day session is removed and the morning night leg ends at its normal 07:00 close |
| 2025-06-09 | early close | 07:00 | `ASX24-LIVE-2025-03` | T1 | Day Session Closed / Night Session to open at 5:10pm: the day session is removed and the morning night leg ends at its normal 07:00 close |
| 2025-12-24 | early close | 14:30 | `ASX24-LIVE-2025-03` | T1 | Equity Products to close at 2:30pm and Night Session Closed: the day session closes 14:30 and the evening night session does not open |
| 2025-12-25 | closed | — | `ASX24-LIVE-2025-03` | T1 | operator Closed/Non Trading Day (day and night sessions) |
| 2025-12-26 | closed | — | `ASX24-LIVE-2025-03` | T1 | operator Closed/Non Trading Day (day and night sessions) |
| 2025-12-27 | closed | — | `ASX24-LIVE-2025-03` | T1 | the operator closed the Friday evening's night session, whose trade date is the Saturday |
| 2025-12-31 | early close | 14:30 | `ASX24-LIVE-2025-03` | T1 | Equity Products to close at 2:30pm and Night Session Closed: the day session closes 14:30 and the evening night session does not open |

### 2026

| 2026-01-01 | closed | — | `ASX24-LIVE-2026-10` | T1 | operator Closed/Non Trading Day (day and night sessions) |
| 2026-01-02 | replacement blocks | blocks 09:40-09:50; 09:50-16:30 | `ASX24-LIVE-2026-10` | T1 | the operator closed the holiday evening's night session; the late open removes the wrapped leg and keeps the following day's own sessions |
| 2026-01-26 | early close | 08:00 | `ASX24-LIVE-2026-10` | T1 | Day Session Closed / Night Session to open at 5:10pm: the day session is removed and the morning night leg ends at its normal 08:00 close |
| 2026-04-03 | replacement blocks | blocks orderentry(-1, 61200, 61800); regular(-1, 61800, 0) | `ASX24-LIVE-2026-10` | T1 | Day Before Good Friday: the night session opening on the eve ceased at midnight Sydney (operator row) and Good Friday itself is Closed/Non Trading Day, so the trade date runs that shortened evening leg and no day session |
| 2026-04-04 | closed | — | `ASX24-LIVE-2026-10` | T1 | the operator closed the Friday evening's night session, whose trade date is the Saturday |
| 2026-04-06 | closed | — | `ASX24-LIVE-2026-10` | T1 | operator Closed/Non Trading Day (day and night sessions) |
| 2026-04-07 | replacement blocks | blocks 09:40-09:50; 09:50-16:30 | `ASX24-LIVE-2026-10` | T1 | the operator closed the holiday evening's night session; the late open removes the wrapped leg and keeps the following day's own sessions |
| 2026-06-08 | early close | 07:00 | `ASX24-LIVE-2026-10` | T1 | Day Session Closed / Night Session to open at 5:10pm: the day session is removed and the morning night leg ends at its normal 07:00 close |
| 2026-12-24 | early close | 14:30 | `ASX24-LIVE-2026-10` | T1 | Equity Products to close at 2:30pm and Night Session Closed: the day session closes 14:30 and the evening night session does not open |
| 2026-12-25 | closed | — | `ASX24-LIVE-2026-10` | T1 | operator Closed/Non Trading Day (day and night sessions) |
| 2026-12-26 | closed | — | `ASX24-LIVE-2026-10` | T1 | the operator closed the Friday evening's night session, whose trade date is the Saturday |
| 2026-12-28 | closed | — | `ASX24-LIVE-2026-10` | T1 | operator Closed/Non Trading Day (day and night sessions) |
| 2026-12-29 | replacement blocks | blocks 09:40-09:50; 09:50-16:30 | `ASX24-LIVE-2026-10` | T1 | the operator closed the holiday evening's night session; the late open removes the wrapped leg and keeps the following day's own sessions |
| 2026-12-31 | early close | 14:30 | `ASX24-LIVE-2026-10` | T1 | Equity Products to close at 2:30pm and Night Session Closed: the day session closes 14:30 and the evening night session does not open |

### 2027

| 2027-01-01 | closed | — | `ASX24-LIVE-2026-10` | T1 | operator Closed/Non Trading Day (day and night sessions) |
| 2027-01-02 | closed | — | `ASX24-LIVE-2026-10` | T1 | the operator closed the Friday evening's night session, whose trade date is the Saturday |
| 2027-01-26 | early close | 08:00 | `ASX24-LIVE-2026-10` | T1 | Day Session Closed / Night Session to open at 5:10pm: the day session is removed and the morning night leg ends at its normal 08:00 close |
| 2027-03-26 | replacement blocks | blocks orderentry(-1, 61200, 61800); regular(-1, 61800, 0) | `ASX24-LIVE-2026-10` | T1 | Day Before Good Friday: the night session opening on the eve ceased at midnight Sydney (operator row) and Good Friday itself is Closed/Non Trading Day, so the trade date runs that shortened evening leg and no day session |
| 2027-03-27 | closed | — | `ASX24-LIVE-2026-10` | T1 | the operator closed the Friday evening's night session, whose trade date is the Saturday |
| 2027-03-29 | closed | — | `ASX24-LIVE-2026-10` | T1 | operator Closed/Non Trading Day (day and night sessions) |
| 2027-03-30 | replacement blocks | blocks 09:40-09:50; 09:50-16:30 | `ASX24-LIVE-2026-10` | T1 | the operator closed the holiday evening's night session; the late open removes the wrapped leg and keeps the following day's own sessions |
| 2027-06-14 | early close | 07:00 | `ASX24-LIVE-2026-10` | T1 | Day Session Closed / Night Session to open at 5:10pm: the day session is removed and the morning night leg ends at its normal 07:00 close |
| 2027-12-24 | early close | 14:30 | `ASX24-LIVE-2026-10` | T1 | Equity Products to close at 2:30pm and Night Session Closed: the day session closes 14:30 and the evening night session does not open |
| 2027-12-25 | closed | — | `ASX24-LIVE-2026-10` | T1 | the closed Friday night session's trade date is the Saturday |
| 2027-12-27 | closed | — | `ASX24-LIVE-2026-10` | T1 | operator Closed/Non Trading Day (day and night sessions) |
| 2027-12-28 | closed | — | `ASX24-LIVE-2026-10` | T1 | operator Closed/Non Trading Day (day and night sessions) |
| 2027-12-29 | replacement blocks | blocks 09:40-09:50; 09:50-16:30 | `ASX24-LIVE-2026-10` | T1 | the operator closed the holiday evening's night session; the late open removes the wrapped leg and keeps the following day's own sessions |
| 2027-12-31 | early close | 14:30 | `ASX24-LIVE-2026-10` | T1 | Equity Products to close at 2:30pm and Night Session Closed: the day session closes 14:30 and the evening night session does not open |

### 2028

| 2028-01-01 | closed | — | `ASX24-LIVE-2026-10` | T1 | the operator closed the 31 December 2027 evening night session, whose trade date is 1 January 2028 (audited from the 2027 calendar) |
