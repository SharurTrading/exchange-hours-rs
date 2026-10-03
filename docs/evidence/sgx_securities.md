<!-- SPDX-License-Identifier: MIT-0 -->

# `sgx_securities` — evidence

- **Kind:** `Exchange`
- **Owner module:** [`sgx.rs`](../../src/calendar/schedules/equities/apac/sgx.rs)
- **Source sets:** [`APAC-SGX-SECURITIES`](../schedules/sources.md#apac-sgx-securities)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

SGX-ST normal week; sourced 2011/2017/2019 phases.

## Revision rows

- 2011-08-01 — T1 — SGX-ST Rules 2011-08-01 — continuous all-day trading 09:00–17:00; the midday break is removed.
- 2017-11-13 — T1 — SGX announcement 2017-07-18 — the midday break returns: regular 09:00–12:00 and 13:00–17:00 with a 12:00–13:00 routine.
- 2019-06-03 — T1 — SGX announcement 2019-05-14 — Trade at Close extends the closing tail to 17:16.

## Normal week

**The carried baseline's instants are the operator's own print.** The
`wps/wcm/connect` securities Trading Hours page as served 2009-05-14
(`SGX-TH-2009-05-14`) — the only surviving capture of any pre-2011 securities
hours page — states: "Trading sessions are held daily from Mondays to Fridays
between 9.00am – 12.30pm and 2.00pm - 5.00pm. In addition, there is an Pre-Open
Routine (8.30am – 9.00am) and Pre-Close Routine (5.00pm – 5.06pm)." That is
exactly the pre-2011-08-01 baseline the module encodes: `regular`
09:00-12:30/14:00-17:00, the morning Pre-Open 08:30-09:00, and the 17:00-17:06
Pre-Close/Non-Close tail.

**Why the horizon stays at 2011-08-01.** The capture is pre-floor: it dates an
observation on 2009-05-14 and no capture of any securities trading-hours page
survives for 2010-01-01..2011-07-31 (the domain-wide CDX sweeps recorded above
were re-checked 2026-09-30 UTC against the `wps/wcm/connect` path family; the
Trading Hours page's own capture list holds exactly one row, this one). An
observation before the support floor does not source a day inside the claimed
interval, so the era's dates stay carried even though its grid is no longer
unattested. Closing condition, unchanged in substance and now named to the
page family: a capture of the securities Trading Hours page (or a dated SGX-ST
rulebook or practice-note edition) dated inside 2010-01-01..2011-07-31 would
move the horizon to its day.

## Sources

Row review: 2026-08-22 (UTC) is the date the ledger row was last reviewed as a
whole. Per-source retrieval dates were not recorded before the 2026-09-12
migration; where a later targeted review, capture or document date is recorded
beside a source below, that date governs for that source, and dates are added
as each source is re-verified.

- <https://rulebook.sgx.com/rulebook/regulatory-notice-821-trading-hours-market-phases-application-market-phases-and-principles> — SGX-ST Regulatory Notice 8.2.1. Every routine has a Pre-Open/Pre-Close Phase that "allows order entry, order modification and withdrawal of orders but no matching of orders", and a Non-Cancel Phase in which "all existing orders that can be matched are matched at a single price".
- <https://rulebook.sgx.com/sites/default/files/net_file_store/SGX_ST_Rules_August_1_2011.pdf> — SGX-ST Rules as at 2011-08-01, introducing continuous all-day trading. Its Practice Note 8.2.1 carries the pre-2017 routine boundaries used by the two oldest profiles: Pre-Open 08:30–08:59 / Non-Cancel 08:59–09:00, lunch-break Adjust 12:30–13:59 with no matching and its 13:59–14:00 match, and Pre-Close 17:00–17:05 / Non-Cancel 17:05–17:06.
- <https://links.sgx.com/1.0.0/corporate-announcements/AYXNAX3DG8RCFZT7/20170718_SGX_to_adjust_equities_market_structure_after_supportive_feedback.pdf> — SGX announcement of 2017-07-18, restoring the midday break from 2017-11-13.
- <https://links.sgx.com/1.0.0/corporate-announcements/46OQY4VBYIHO4ARN/20190514_SGX_to_launch_securities_market_trade_at_close_session_on_3_June.pdf> — SGX announcement of 2019-05-14, launching Trade at Close on 2019-06-03.
- <https://www.sgx.com/stock-exchange/trading> — the operator's own securities-market trading page (this page's own holiday and half-day statements), read 2026-09-28 (UTC) as bytes from SGX's content API: `holidays/raw/equities/sgx_securities/2025-2027/api2_content-api_stock-exchange_trading.json` (research store), sha256 `45dbdc61…`. The live page is an SPA shell to non-JS clients (`live_sgx_trading_hours_holidays.html`, sha256 `d87ad3e0…`); the JSON above is the feed the page itself renders.
- <https://www.mom.gov.sg/employment-practices/public-holidays> — the Singapore holiday calendar the operator designates (page last updated 19 June 2026), retrieved 2026-09-28 (UTC) as `holidays/raw/equities/sgx_securities/2025-2027/live_mom_public_holidays.html`, sha256 `a4f175a7…`; it prints the gazetted 2025, 2026 and 2027 dates.

## Holidays

**Coverage:** 2014-01-01..2019-12-31, 2020-01-01..2020-01-01, 2025-01-01..2026-12-31 (inclusive trade dates; the spans between the windows are the capture gaps recorded below). Tiers: T1 for the 2014 to 2020 rows, T2 for the 2025 and 2026 rows.

**The 2014-2020 rows come from the operator's own securities
`Trading Hours & Calendar` page** (`sgx.com/wps/portal/sgxweb/home/trading/
securities/trading_hours_calendar`), whose Wayback `id_` replays — verbatim
bytes of an operator statement, so T1 (LAW-PUBLIC-SOURCES) — were retrieved
2026-09-29 (UTC) as `holidays/raw/equities/sgx_securities/2010-2024/`
(research store), digests in the `### Documents` table below. The page
prints one `Public Holidays <year>` table per year with the operator's own
markers: `*` — `The following Monday will be a public holiday` — keys the
Sunday substitutions, and `#` — `The preceding day is a half-day trading
day` — keys the half days. Weekend holidays the sheet marks with neither
marker close no weekday and ship no row.

**The 2016-2017 half-day grid, and what is held where.** The page's
`Half Day Trading` column prints the half-day routine in force:
`Pre-Open 0830 0858 – 59*`, `Non-Cancel 0858 – 59* 0900`, `Trading Open
0900 1230`, `Pre-Close 1230 1234-35*`, `Non-Cancel 1234-35* 1236`,
`Close 1236` — identical in both surviving replays (2017-09-27 and
2018-12-23). The seven pre-2025 half days ship as replacement block sets
restating that printed grid, not scalar early closes, for the same reason
the 2025-2026 rows do: the closing Non-Cancel match is tradeable after the
trading close.

**Two disclosures the printed record forces.**

- **The 2019 Trade-at-Close question.** The operator's own 2019-05-14
  announcement launched Trade at Close on 2019-06-03 and moved the full-day
  close to 17:16; the current page's half-day grid carries the 12:06-12:16
  Trade-at-Close tail. But no surviving artifact prints the half-day grid
  as it stood after 2019-06-03 — the last printed pre-2025 grid is the
  2018-12-23 replay's 12:36-close grid — so whether the 2019-12-24 and
  2019-12-31 half days closed at 12:36 (the printed grid) or gained a
  Trade-at-Close tail (as the full day did) is unstated. The rows hold the
  printed grid, the only sourced shape, and the disputed remainder ships as
  no session rather than guessed.
- **The 2014-2016 half days are unstated.** The 2014, 2015 and 2016 sheets
  print no `#` markers, no half-day legend and no half-day grid, so no
  half-day row ships for those years. The dates without rows inside the
  2014-2016 windows are audited normal **for closures**; that the eves'
  treatment is unstated is a recorded gap below.

**The capture gaps.** No capture of any SGX securities trading-hours page —
the `wps/portal/sgxweb` securities page, its `marketplace`-portal
predecessor, the `www2`-era pages, or the current content-API page —
survives in the Wayback index for 2010-01-01..2013-12-31 or for
2020-01-02..2024-12-31. No operator statement prints those closures, so the
table claims nothing for the spans: the coverage windows stop at 2019-12-31,
hold only 2020-01-01 (the one 2020 date the 2019 sheet prints), and resume at
2025-01-01, and queries inside the gaps refuse rather than answer.
**Closing condition:** a surviving operator artifact printing the spans'
closures (a 2010-2013 or 2020-2024 capture or export of the operator's own
calendar page, or an annual securities trading-schedule notice). Tracked as
[#213](https://github.com/SharurTrading/exchange-hours-rs/issues/213).

**The bounded searches, recorded.** Checked 2026-09-29 UTC by domain-wide
CDX sweeps over sgx.com in both eras, and re-run with fresh eyes on
2026-09-30 UTC (artifacts under `holidays/raw/equities/sgx_securities/
2010-2013-retry/` and `.../2020-2024-retry/` in the research store):

- **2010-2013.** The earlier record said the marketplace-portal predecessor
  page was never captured; that was too strong. The
  `wps/wcm/connect/mp_en/site/trading_on_sgx/securities_market/` family was
  captured — in May 2009 only: the `Securities Trading Calendar` page
  (capture 20090522132802), the `2009 public holidays` sheet (20090514003600)
  and the `2008 publicholidays` sheet (20090523091800), each retrieved and
  digested in the store. They print only the 2008 and 2009 holiday tables and
  the pre-2011 sessions grid; no capture of the family exists after May 2009,
  the `wps/portal/sgxweb` securities page's first capture is 2014-08-21
  (its own capture list, re-enumerated 2026-09-30 UTC), a 3 000-urlkey
  domain-wide sweep of sgx.com 2010-2013 holds no securities trading-schedule
  page or annual circular, and `marketplace.sgx.com` has zero captures.
- **2020-2024.** The wps page's own captures remain React SPA shells
  (re-verified on the 2021-12-24, 2022-05-25 and 2024-03-02 replays). The
  shell's own data channel is the operator's content API, and the archive
  answers for it too: the content-api page query for the wps
  `trading_hours_calendar` path was captured exactly once (2022-09-13
  17:17:07 UTC) and its complete body is `{"data":{"route":null}}` — the
  retired path served no content; the current site's
  `/stock-exchange/trading` content-api query first appears at capture
  2026-07-25; and a domain-wide enumeration of `api2.sgx.com` (1 499
  urlkeys) holds no other calendar or holiday query shape. The JSON channel
  is a negative on the same terms as the page shells.

**Tier.** The 2025-2026 rows key at T2 because the artifact behind them is
the operator's own machine channel read as bytes (LAW-PRIMARY-SOURCES),
which carries both the designation sentence and the half-day schedule; the
closure *dates* are printed on the designated MOM page, the government's own
gazetted list; no member firm, vendor or press restatement touches any row.
The 2014-2020 rows key at T1: their artifact is the operator's own page as
verbatim `id_` replays.

### 2014

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2014-01-01 | closed | `1 Jan 2014 Wednesday New Year's Day` | `SGX-CAL-2014` | T1 | the operator's own table, date printed verbatim |
| 2014-01-31 | closed | `31 Jan 2014 Friday Chinese New Year` | `SGX-CAL-2014` | T1 | the operator's own table; the Saturday 1 February holiday closes no weekday and ships no row |
| 2014-04-18 | closed | `18 Apr 2014 Friday Good Friday` | `SGX-CAL-2014` | T1 | the operator's own table |
| 2014-05-01 | closed | `1 May 2014 Thursday Labour Day` | `SGX-CAL-2014` | T1 | the operator's own table |
| 2014-05-13 | closed | `13 May 2014 Tuesday Vesak Day` | `SGX-CAL-2014` | T1 | the operator's own table |
| 2014-07-28 | closed | `28 Jul 2014 Monday Hari Raya Puasa` | `SGX-CAL-2014` | T1 | the operator's own table |
| 2014-10-06 | closed | `5 Oct 2014* Sunday Hari Raya Haji`; footnote: `As Hari Raya Haji falls on Sunday 5 October 2014, the next day, Monday 6 October 2014, will be a public holiday.` | `SGX-CAL-2014` | T1 | the operator's own substitution sentence keys the Monday |
| 2014-10-22 | closed | `22 Oct 2014** Wednesday Deepavali`; footnote: `The Hindu Advisory Board (HAB) has confirmed that Deepavali will fall on 22 October 2014 (Wednesday) instead of 23 October 2014 (Thursday).` | `SGX-CAL-2014` | T1 | the operator's own confirmed-date footnote |
| 2014-12-25 | closed | `25 Dec 2014 Thursday Christmas Day` | `SGX-CAL-2014` | T1 | the operator's own table |

The 2014 sheet prints no half-day markers or half-day grid; the eves'
treatment is the recorded gap below. The Saturday 9 August National Day
closes no weekday and ships no row.

### 2015

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2015-01-01 | closed | `1-Jan-15 Thursday New Year's Day` | `SGX-CAL-2015` | T1 | the operator's own table |
| 2015-02-19 | closed | `19-Feb-15 Thursday Chinese New Year` | `SGX-CAL-2015` | T1 | the operator's own table |
| 2015-02-20 | closed | `20-Feb-15 Friday Chinese New Year` | `SGX-CAL-2015` | T1 | the operator's own table |
| 2015-04-03 | closed | `3-Apr-15 Friday Good Friday` | `SGX-CAL-2015` | T1 | the operator's own table |
| 2015-05-01 | closed | `1-May-15 Friday Labour Day` | `SGX-CAL-2015` | T1 | the operator's own table |
| 2015-06-01 | closed | `1-Jun-15 Monday Vesak Day` | `SGX-CAL-2015` | T1 | the operator's own table |
| 2015-07-17 | closed | `17-Jul-15 Friday Hari Raya Puasa` | `SGX-CAL-2015` | T1 | the operator's own table |
| 2015-08-07 | closed | `7-Aug-15 Friday SG50 Public Holiday` | `SGX-CAL-2015` | T1 | the operator's own one-off SG50 holiday |
| 2015-08-10 | closed | `9 Aug 2015 * Sunday National Day`; legend: `The following Monday will be a public holiday` | `SGX-CAL-2015` | T1 | the operator's own substitution marker keys the Monday |
| 2015-09-11 | closed | `11-Sep-15 Friday Polling Day` | `SGX-CAL-2015` | T1 | the operator's own table |
| 2015-09-24 | closed | `24-Sep-15 Thursday Hari Raya Haji` | `SGX-CAL-2015` | T1 | the operator's own table |
| 2015-11-10 | closed | `10 Nov 2015 ** Tuesday Deepavali`; footnote records the date as subject to the Hindu Almanac's reconfirmation | `SGX-CAL-2015` | T1 | the operator's own printed date |
| 2015-12-25 | closed | `25-Dec-15 Friday Christmas Day` | `SGX-CAL-2015` | T1 | the operator's own table |

The 2015-09-24 replay and the 2016-01-08 replay print the identical 2015
table; the former is the keying document, the latter corroborates it. The
2015 sheet prints no half-day markers or grid (the recorded gap below).

### 2016

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2016-01-01 | closed | `1-Jan-16 Friday New Year's Day` | `SGX-CAL-2016` | T1 | the operator's own table |
| 2016-02-08 | closed | `8-Feb-16 Monday Chinese New Year` | `SGX-CAL-2016` | T1 | the operator's own table |
| 2016-02-09 | closed | `9-Feb-16 Tuesday Chinese New Year` | `SGX-CAL-2016` | T1 | the operator's own table |
| 2016-03-25 | closed | `25-Mar-16 Friday Good Friday` | `SGX-CAL-2016` | T1 | the operator's own table |
| 2016-05-02 | closed | `1 May 2016* Sunday Labour Day`; legend: `The following Monday will be a public holiday` | `SGX-CAL-2016` | T1 | the operator's own substitution marker keys the Monday |
| 2016-07-06 | closed | `6-Jul-16 Wednesday Hari Raya Puasa` | `SGX-CAL-2016` | T1 | the operator's own table |
| 2016-08-09 | closed | `9-Aug-16 Tuesday National Day` | `SGX-CAL-2016` | T1 | the operator's own table |
| 2016-09-12 | closed | `12-Sep-16 Monday Hari Raya Haji` | `SGX-CAL-2016` | T1 | the operator's own table |
| 2016-12-26 | closed | `25 Dec 2016* Sunday Christmas Day`; legend: `The following Monday will be a public holiday` | `SGX-CAL-2016` | T1 | the operator's own substitution marker keys the Monday |

The Saturday 21 May Vesak Day and Saturday 29 October Deepavali close no
weekday and ship no row. The 2016 sheet prints no half-day markers or grid
(the recorded gap below).

### 2017

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2017-01-02 | closed | `1-Jan-17* Sunday New Year's Day`; legend: `The following Monday will be a public holiday` | `SGX-CAL-2017` | T1 | the operator's own substitution marker keys the Monday |
| 2017-01-27 | replacement blocks | `28-Jan-17# Saturday Chinese New Year`; footnote: `The preceding Friday, 27-Jan-17, is a half-day trading day`; the page's `Half Day Trading` grid: Pre-Open `0830 0858 – 59*`, Non-Cancel `0858 – 59* 0900`, `Trading Open 0900 1230`, Pre-Close `1230 1234-35*`, Non-Cancel `1234-35* 1236`, `Close 1236` | `SGX-CAL-2017` | T1 | the operator's own half-day marker and printed grid; the row restates the grid as one replacement day |
| 2017-01-30 | closed | `29-Jan-17* Sunday Chinese New Year`; legend: `The following Monday will be a public holiday` | `SGX-CAL-2017` | T1 | the operator's own substitution marker keys the Monday |
| 2017-04-14 | closed | `14-Apr-17 Friday Good Friday` | `SGX-CAL-2017` | T1 | the operator's own table |
| 2017-05-01 | closed | `1-May-17 Monday Labour Day` | `SGX-CAL-2017` | T1 | the operator's own table |
| 2017-05-10 | closed | `10-May-17 Wednesday Vesak Day` | `SGX-CAL-2017` | T1 | the operator's own table |
| 2017-06-26 | closed | `25-Jun-17* Sunday Hari Raya Puasa`; legend: `The following Monday will be a public holiday` | `SGX-CAL-2017` | T1 | the operator's own substitution marker keys the Monday |
| 2017-08-09 | closed | `9-Aug-17 Wednesday National Day` | `SGX-CAL-2017` | T1 | the operator's own table |
| 2017-09-01 | closed | `1-Sep-17 Friday Hari Raya Haji` | `SGX-CAL-2017` | T1 | the operator's own table |
| 2017-10-18 | closed | `18-Oct-17 Wednesday Deepavali` | `SGX-CAL-2017` | T1 | the operator's own table |
| 2017-12-25 | closed | `25-Dec-17 Monday Christmas Day` | `SGX-CAL-2017` | T1 | the operator's own table; no `#` marker, the eve being a Sunday |

The 2017 sheet was captured 2017-09-27, when the restored midday-break
structure (effective 2017-11-13) was already the page's printed full-day
grid, and its `Half Day Trading` column prints the 12:30/12:36 grid.

### 2018

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2018-01-01 | closed | `1-Jan-18 Monday New Year's Day` | `SGX-CAL-2018-2019` | T1 | the operator's own table |
| 2018-02-15 | replacement blocks | `16-Feb-18 # Friday Chinese New Year`; footnote: `The preceding day is a half-day trading day`; the page's `Half Day Trading` grid, `Trading Open 0900 1230` … `Close 1236` | `SGX-CAL-2018-2019` | T1 | the operator's own half-day marker and printed grid |
| 2018-02-16 | closed | `16-Feb-18 Friday Chinese New Year` | `SGX-CAL-2018-2019` | T1 | the operator's own table; the Saturday 17 February holiday closes no weekday |
| 2018-03-30 | closed | `30-Mar-18 Friday Good Friday` | `SGX-CAL-2018-2019` | T1 | the operator's own table |
| 2018-05-01 | closed | `1-May-18 Tuesday Labour Day` | `SGX-CAL-2018-2019` | T1 | the operator's own table |
| 2018-05-29 | closed | `29-May-18 Tuesday Vesak Day` | `SGX-CAL-2018-2019` | T1 | the operator's own table |
| 2018-06-15 | closed | `15-Jun-18 Friday Hari Raya Puasa` | `SGX-CAL-2018-2019` | T1 | the operator's own table |
| 2018-08-09 | closed | `9-Aug-18 Thursday National Day` | `SGX-CAL-2018-2019` | T1 | the operator's own table |
| 2018-08-22 | closed | `22-Aug-18 Wednesday Hari Raya Haji` | `SGX-CAL-2018-2019` | T1 | the operator's own table |
| 2018-11-06 | closed | `6-Nov-18 Tuesday Deepavali` | `SGX-CAL-2018-2019` | T1 | the operator's own table |
| 2018-12-24 | replacement blocks | `25-Dec-18 # Tuesday Christmas Day`; footnote: `The preceding day is a half-day trading day` | `SGX-CAL-2018-2019` | T1 | the operator's own half-day marker and printed grid |
| 2018-12-25 | closed | `25-Dec-18 Tuesday Christmas Day` | `SGX-CAL-2018-2019` | T1 | the operator's own table |
| 2018-12-31 | replacement blocks | `1-Jan-19 # Tuesday New Year's Day`; `# The preceding day is a half-day trading day` | `SGX-CAL-2018-2019` | T1 | the 2019 table's own half-day marker keys the preceding Monday |

### 2019

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2019-01-01 | closed | `1-Jan-19 Tuesday New Year's Day` | `SGX-CAL-2018-2019` | T1 | the operator's own table |
| 2019-02-04 | replacement blocks | `5-Feb-19 # Tuesday Chinese New Year`; `6-Feb-19 Wednesday`; footnote: `The preceding day is a half-day trading day` | `SGX-CAL-2018-2019` | T1 | the operator's own half-day marker and printed grid |
| 2019-02-05 | closed | `5-Feb-19 Tuesday Chinese New Year` | `SGX-CAL-2018-2019` | T1 | the operator's own table |
| 2019-02-06 | closed | `6-Feb-19 Wednesday Chinese New Year` | `SGX-CAL-2018-2019` | T1 | the operator's own table |
| 2019-04-19 | closed | `19-Apr-19 Friday Good Friday` | `SGX-CAL-2018-2019` | T1 | the operator's own table |
| 2019-05-01 | closed | `1-May-19 Wednesday Labour Day` | `SGX-CAL-2018-2019` | T1 | the operator's own table |
| 2019-05-20 | closed | `19-May-19* Sunday Vesak Day`; legend: `The following Monday will be a public holiday` | `SGX-CAL-2018-2019` | T1 | the operator's own substitution marker keys the Monday |
| 2019-06-05 | closed | `5-Jun-19 Wednesday Hari Raya Puasa` | `SGX-CAL-2018-2019` | T1 | the operator's own table |
| 2019-08-09 | closed | `9-Aug-19 Friday National Day` | `SGX-CAL-2018-2019` | T1 | the operator's own table |
| 2019-08-12 | closed | `11-Aug-19* Sunday Hari Raya Haji`; legend: `The following Monday will be a public holiday` | `SGX-CAL-2018-2019` | T1 | the operator's own substitution marker keys the Monday |
| 2019-10-28 | closed | `27-Oct-19* Sunday Deepavali`; legend: `The following Monday will be a public holiday` | `SGX-CAL-2018-2019` | T1 | the operator's own substitution marker keys the Monday |
| 2019-12-24 | replacement blocks | `25-Dec-19 # Wednesday Christmas Day`; footnote: `The preceding day is a half-day trading day`; the printed half-day grid held (see the Trade-at-Close disclosure) | `SGX-CAL-2018-2019` | T1 | the operator's own half-day marker and printed grid |
| 2019-12-25 | closed | `25-Dec-19 Wednesday Christmas Day` | `SGX-CAL-2018-2019` | T1 | the operator's own table |
| 2019-12-31 | replacement blocks | `1-Jan-20 # Wednesday New Year's Day`; `# The preceding day is a half-day trading day`; the printed half-day grid held (see the Trade-at-Close disclosure) | `SGX-CAL-2018-2019` | T1 | the 2020 table's own half-day marker keys the preceding Tuesday |

### 2020

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2020-01-01 | closed | `1-Jan-20 Wednesday New Year's Day` | `SGX-CAL-2018-2019` | T1 | the one 2020 date the sheet's 2020 table prints; no later 2020 table survives |

The live `www.sgx.com` holiday pages render no server-side content, so the
operator's statements were read as bytes from SGX's own content API
(`api2.sgx.com/content-api`), the CMS feed the page renders, for
`/stock-exchange/trading` — the securities-market page. That artifact carries
three load-bearing statements in the operator's own words:

1. **The closure calendar is designated.** `SGX follows the Singapore holiday
   calendar available on the Ministry of Manpower website.` — so a gazetted
   Singapore holiday falling on an SGX-ST trading day is an SGX-ST closure,
   and MOM's printed gazetted list (saved beside the operator artifact) is
   the calendar the operator names.
2. **The half-day phase grid.** On half days the operator prints Opening
   Routine as usual, `Trading Open: 09:00am - 12:00pm`, Closing Routine
   `Pre-Close: 12:00pm – 12:04pm/12:05pm*`, `Non-Cancel: 12:04pm/12:05pm* –
   12:06pm`, `Trade at Close: 12:06pm – 12:16pm`, `Close: 12:16pm` — the same
   phase structure as a full day compressed to a 12:16 close.
3. **The half-day dates.** `The Eve of Chinese New Year, Eve of Christmas and
   Eve of New Year in 2025 & 2026 fall on business days. As such, there will
   be half-day trading in 2025 & 2026.` with the six dates printed in the
   page's own table.

**Why the window ends at 2026-12-31.** The operator's own sheet is printed for
2025 & 2026 only. MOM has gazetted 2027, but the operator has not stated the
2027 treatment of the Chinese New Year, Christmas and New Year eves — whether
any of them is a half day is exactly the fact statement 3 exists to make — so
a 2027 date without a row could not be audited normal and the window cannot
honestly extend. Re-checked 2026-09-29 UTC: the `SGX-ST-SCHED` content-api
endpoint was read again and answered byte-identical bytes (same sha256
`45dbdc61…`), so the sheet still scopes itself to 2025 & 2026 and the
2027-01-01..2027-12-31 arrangement remains unpublished. Re-checked again
2026-10-02 UTC (artifact
`api2_content-api_stock-exchange_trading.live-20261002T235448Z.json` and
`INDEX-recheck-2026-10-02.md` under `holidays/raw/equities/sgx_securities/2025-2027/`):
the endpoint answered changed bytes — editorial deltas elsewhere on the page
(market-maker programme wording and a board-lot FAQ that mentions a February
2027 implementation) — and the holiday sheet is content-identical: the
half-day statement still scopes itself to `2025 & 2026` and names no 2027
dates. **Closing condition:**
SGX's next annual securities schedule naming the 2027 half days, at which
point the window extends. Re-checked monthly per LAW-WATCH.

**Tier.** The rows key at T2 because the artifact behind every row is the
operator's own machine channel read as bytes (LAW-PRIMARY-SOURCES), which
carries both the designation sentence and the half-day schedule. The closure
*dates* are printed on the designated MOM page, the government's own gazetted
list; no member firm, vendor or press restatement touches any row.

### 2025

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2025-01-01 | closed | MOM: `1 January 2025 Wednesday New Year's Day`; SGX designates the MOM calendar | `SGX-ST-SCHED` | T2 | the operator's designation resolved against MOM's printed gazetted date |
| 2025-01-28 | replacement blocks | SGX: `28 Jan 2025 Tuesday Eve of Chinese New Year`; half-day `Close: 12:16pm` | `SGX-ST-SCHED` | T2 | SGX trade date named verbatim; the row restates the day's complete session structure as a replacement-block set keyed to the operator's printed half-day close |
| 2025-01-29 | closed | MOM: `29 January 2025 Wednesday Chinese New Year` | `SGX-ST-SCHED` | T2 | designation + MOM print |
| 2025-01-30 | closed | MOM: `30 January 2025 Thursday Chinese New Year` | `SGX-ST-SCHED` | T2 | designation + MOM print |
| 2025-03-31 | closed | MOM: `31 March 2025 Monday Hari Raya Puasa` | `SGX-ST-SCHED` | T2 | designation + MOM print |
| 2025-04-18 | closed | MOM: `18 April 2025 Friday Good Friday` | `SGX-ST-SCHED` | T2 | designation + MOM print |
| 2025-05-01 | closed | MOM: `1 May 2025 Thursday Labour Day` | `SGX-ST-SCHED` | T2 | designation + MOM print |
| 2025-05-12 | closed | MOM: `12 May 2025 Monday Vesak Day` | `SGX-ST-SCHED` | T2 | designation + MOM print |
| 2025-10-20 | closed | MOM: `20 October 2025 Monday Deepavali` | `SGX-ST-SCHED` | T2 | designation + MOM print |
| 2025-12-24 | replacement blocks | SGX: `24 Dec 2025 Wednesday Eve of Christmas`; half-day `Close: 12:16pm` | `SGX-ST-SCHED` | T2 | SGX trade date named verbatim |
| 2025-12-25 | closed | MOM: `25 December 2025 Thursday Christmas Day` | `SGX-ST-SCHED` | T2 | designation + MOM print |
| 2025-12-31 | replacement blocks | SGX: `31 Dec 2025 Wednesday Eve of New Year`; half-day `Close: 12:16pm` | `SGX-ST-SCHED` | T2 | SGX trade date named verbatim |

The 2025 gazetted holidays that fall on a Saturday — Polling Day 3 May, Hari
Raya Haji 7 June, National Day 9 August — print no row: they are outside the
Mon-Fri trading week, and the operator's designation closes no weekday for
them. MOM's own Sunday-substitution sentences never arise in 2025.

### 2026

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2026-01-01 | closed | MOM: `1 January 2026 Thursday New Year's Day` | `SGX-ST-SCHED` | T2 | designation + MOM print |
| 2026-02-16 | replacement blocks | SGX: `16 Feb 2026 Monday Eve of Chinese New Year`; half-day `Close: 12:16pm` | `SGX-ST-SCHED` | T2 | SGX trade date named verbatim |
| 2026-02-17 | closed | MOM: `17 February 2026 Tuesday Chinese New Year` | `SGX-ST-SCHED` | T2 | designation + MOM print |
| 2026-02-18 | closed | MOM: `18 February 2026 Wednesday Chinese New Year` | `SGX-ST-SCHED` | T2 | designation + MOM print |
| 2026-04-03 | closed | MOM: `3 April 2026 Friday Good Friday` | `SGX-ST-SCHED` | T2 | designation + MOM print |
| 2026-05-01 | closed | MOM: `1 May 2026 Friday Labour Day` | `SGX-ST-SCHED` | T2 | designation + MOM print |
| 2026-05-27 | closed | MOM: `27 May 2026 Wednesday Hari Raya Haji` | `SGX-ST-SCHED` | T2 | designation + MOM print |
| 2026-06-01 | closed | MOM: `31 May 2026 Sunday Vesak Day` — `Monday, 1 June 2026, will be a public holiday if your rest day falls on 31 May 2026.` | `SGX-ST-SCHED` | T2 | MOM's own Sunday-substitution sentence moves the holiday to the Monday the exchange does not trade |
| 2026-08-10 | closed | MOM: `9 August 2026 Sunday National Day` — `Monday, 10 August 2026, will be a public holiday …` | `SGX-ST-SCHED` | T2 | MOM's Sunday-substitution sentence |
| 2026-11-09 | closed | MOM: `8 November 2026 Sunday Deepavali` — `Monday, 9 November 2026, will be a public holiday …` | `SGX-ST-SCHED` | T2 | MOM's Sunday-substitution sentence |
| 2026-12-24 | replacement blocks | SGX: `24 Dec 2026 Thursday Eve of Christmas`; half-day `Close: 12:16pm` | `SGX-ST-SCHED` | T2 | SGX trade date named verbatim |
| 2026-12-25 | closed | MOM: `25 December 2026 Friday Christmas Day` | `SGX-ST-SCHED` | T2 | designation + MOM print |
| 2026-12-31 | replacement blocks | SGX: `31 Dec 2026 Thursday Eve of New Year`; half-day `Close: 12:16pm` | `SGX-ST-SCHED` | T2 | SGX trade date named verbatim |

`Hari Raya Puasa 2026` falls on Saturday 21 March (MOM prints it with no
substitution sentence), so it closes no weekday and ships no row.

**The Sunday substitutions are MOM's own sentences, not inference.** MOM
words each as conditional on the employee's rest day; the exchange reads a
market closure because the substituted day is itself a gazetted public
holiday under Singapore's Holidays Act arrangement the calendar publishes,
and the operator's designation adopts the calendar whole. Each substituted
Monday above quotes MOM's sentence in full beside it so a reader can check
the derivation from the saved bytes.

**The half-day rows state the sheet exactly.** Every matching phase the
operator prints on a half day — the morning session to 12:00, the closing
Non-Cancel to 12:06 and Trade at Close to 12:16 — ends at or before the
printed `Close: 12:16pm`. The rows ship as `ReplacementBlocks` restating the
complete half-day structure rather than a scalar early close, because a
scalar clip on the envelope's final close would drag the order-entry-only
Pre-Close into `is_open`: the restated midday order-entry slice
(12:00-12:04) only ever marks order acceptance where the sheet prints
Pre-Close order entry, and the closing routine runs to the printed 12:16.

### Documents

| Document | Window | Replay or service URL | Capture or retrieval, UTC | Tier | sha256 |
|---|---|---|---|---|---|
| `SGX-CAL-2014` | 2014-01-01 .. 2014-12-25 | <https://web.archive.org/web/20140821082648id_/http://www.sgx.com/wps/portal/sgxweb/home/trading/securities/trading_hours_calendar> | Wayback `id_` replay of capture `20140821082648`, retrieved 2026-09-29 05:09:17 UTC | T1 | `1025d2182f31d1d73a058a59ef757b82530ddecfd8df17da6b3c53ebb18b3e33` |
| `SGX-CAL-2015` | 2015-01-01 .. 2015-12-25 | <https://web.archive.org/web/20150924101024id_/http://www.sgx.com/wps/portal/sgxweb/home/trading/securities/trading_hours_calendar> | Wayback `id_` replay of capture `20150924101024`, retrieved 2026-09-29 05:09:19 UTC | T1 | `21c74bf1c2261e6118b8b6b822ed92651aaeffd04cd52ab746e99d06ef2b23de` |
| `SGX-CAL-2016` | 2016-01-01 .. 2016-12-26 | <https://web.archive.org/web/20160108072436id_/http://www.sgx.com/wps/portal/sgxweb/home/trading/securities/trading_hours_calendar> | Wayback `id_` replay of capture `20160108072436`, retrieved 2026-09-29 05:09:22 UTC | T1 | `e51c3933d83c95acd3bfe5a0c16ca55df28aea4484fe8e88349884b0b8783309` |
| `SGX-CAL-2017` | 2017-01-02 .. 2017-12-25 | <https://web.archive.org/web/20170927130119id_/http://www.sgx.com/wps/portal/sgxweb/home/trading/securities/trading_hours_calendar> | Wayback `id_` replay of capture `20170927130119`, retrieved 2026-09-29 05:09:24 UTC | T1 | `3256466c4917117ac0cd318f0dd55d7e4513c5d3b967b77d59c48079ce2baaa9` |
| `SGX-CAL-2018-2019` | 2018-01-01 .. 2020-01-01 | <https://web.archive.org/web/20181223141708id_/http://www.sgx.com/wps/portal/sgxweb/home/trading/securities/trading_hours_calendar> | Wayback `id_` replay of capture `20181223141708`, retrieved 2026-09-29 05:09:54 UTC | T1 | `818e043305ce491a2279d98d4279a26eed2b6d6aae61df9afca7ec3bdbf516ba` |
| `SGX-ST-SCHED` | 2025-01-01 .. 2026-12-31 | <https://api2.sgx.com/content-api?queryId=dd24dd8e5b3ef52e535a662e01b58d76471f335e%3Apage&variables=%7B%22path%22%3A%22%2Fstock-exchange%2Ftrading%22%2C%22lang%22%3A%22EN%22%7D> | retrieved 2026-09-28 01:59 UTC | T2 | `45dbdc61d808b4f72bb8bbddb198107f08759a20b85b288271d6d5a0326facd7` |
| `SGX-MOM-CAL-2025-2026` | 2025-01-01 .. 2026-12-31 | <https://www.mom.gov.sg/employment-practices/public-holidays> | retrieved 2026-09-28 01:48 UTC | T1 | `a4f175a7d33222b91f1c8c2f84e6d1e75f0c0b15e265addb478b73e3e65dcf3d` |
| `SGX-TH-2009-05-14` | no rows keyed (the pre-floor securities Trading Hours page; Normal-week corroboration) | <https://web.archive.org/web/20090514003555id_/http://www.sgx.com:80/wps/wcm/connect/mp_en/site/trading_on_sgx/securities_market/securities_trading_and_settlement/Trading+Hours?> | Wayback `id_` replay of capture `20090514003555`, retrieved 2026-09-30 04:47 UTC | T1 | `0dd72053814bf349b1177d300027cd4bfae160cc4fd75c22afa824d5f7099bf1` |

`SGX-ST-SCHED` is the document id the 2025-2026 rows cite; `SGX-MOM-CAL-
2025-2026` is the designated calendar those closure dates are read from,
recorded so each closure date's bytes resolve, and no row keys on it alone.
The 2014-2020 rows cite the five pre-2025 replays above, each a capture of
the operator's own securities `Trading Hours & Calendar` page printing that
year's (or years') `Public Holidays` table, the operator's `*`/`#` legend
and, from 2017, the `Half Day Trading` grid. The store's
`holidays/raw/equities/sgx_securities/2025-2027/` also holds the live SPA
shell and the SGX Group desk calendar PDF (derivatives day notes; context
only) — neither keys a row. The 2020-2024 captures of the wps path are SPA
shells and key nothing.

## Gaps and residual risks

- **the 2010-2013 capture gap** — no capture of any SGX securities
  trading-hours page survives in the Wayback index for 2010-01-01..2013-12-31
  (checked 2026-09-29, UTC, CDX sweeps over the whole sgx.com domain), so no
  operator statement prints those closures and the table claims nothing
  there; queries inside the gap refuse. **Closing condition:** a surviving
  operator artifact printing the span (a capture or export of the operator's
  own calendar page, or an annual securities trading-schedule notice).
  Tracked as [#213](https://github.com/SharurTrading/exchange-hours-rs/issues/213).
- **the 2020-2024 capture gap** — the same for 2020-01-02..2024-12-31: the
  wps path's 2020-2024 captures are SPA shells whose bytes carry no holiday
  content, the `stock-exchange/trading` page's own captures begin in 2025,
  and no other operator calendar page was captured in the era. Only
  2020-01-01 (keyed by the 2019 sheet's 2020 table) answers. **Closing
  condition:** a surviving operator artifact printing the span's closures.
  Tracked as [#213](https://github.com/SharurTrading/exchange-hours-rs/issues/213).
- **the 2014-2016 half-day treatment is unstated** — the 2014, 2015 and
  2016 sheets print no `#` markers, half-day legend or half-day grid, so no
  half-day rows ship for those years and the within-window no-row claim is
  scoped to closures. **Closing condition:** a 2014-2016 artifact printing
  the operator's half-day treatment (a sheet edition with the legend, or a
  participant notice).
- **the 2019 Trade-at-Close half-day question** — no surviving artifact
  prints the half-day grid as it stood after the 2019-06-03 Trade-at-Close
  launch, so the 2019-12-24 and 2019-12-31 rows hold the last printed grid
  (close 12:36) and the disputed tail ships as no session. **Closing
  condition:** a post-June-2019 artifact printing the half-day grid.
- **horizon carried below the first dated row** — the pre-2011-08-01 session bounds (09:00–12:30 and 14:00–17:00) are attested by the operator's own Trading Hours page only at a pre-floor capture (2009-05-14, see the Normal week section): the grid is no longer unattested, but no artifact dated inside 2010-01-01..2011-07-31 prints it, so the era's dates stay carried. The ledger horizon remains 2011-08-01, the first day at which this row's state is sourced, with everything below it carried. Closing condition: a capture of the securities Trading Hours page (or a dated pre-2011 SGX-ST rulebook or practice-note edition) dated inside 2010-01-01..2011-07-31, which would move the horizon to its day.
- Current routine ends are randomized: Pre-Open ends 08:58–08:59 and 12:58–12:59, Pre-Close ends 17:04–17:05. Each order-entry slice stops at the earliest possible end so no matching time is claimed as order entry.
- Trade at Close matches at the Equilibrium Price and is therefore tradeable throughout its window.

## Module narrative (moved from src/calendar/schedules/equities/apac/sgx.rs on 2026-10-02 UTC)

Only the Non-Cancel Phase can print, so each routine is split at the earliest
possible Non-Cancel start. Trade at Close matches at the Equilibrium Price and
is tradeable throughout.
https://rulebook.sgx.com/rulebook/regulatory-notice-821-trading-hours-market-phases-application-market-phases-and-principles

---

The 2011-08-01 practice note carries the pre-2017 routine boundaries used by
the two oldest profiles: Pre-Open 08:30–08:59 / Non-Cancel 08:59–09:00,
lunch-break Adjust 12:30–13:59 with no matching and its 13:59–14:00 match,
and Pre-Close 17:00–17:05 / Non-Cancel 17:05–17:06.
https://rulebook.sgx.com/sites/default/files/net_file_store/SGX_ST_Rules_August_1_2011.pdf
https://links.sgx.com/1.0.0/corporate-announcements/AYXNAX3DG8RCFZT7/20170718_SGX_to_adjust_equities_market_structure_after_supportive_feedback.pdf
https://links.sgx.com/1.0.0/corporate-announcements/46OQY4VBYIHO4ARN/20190514_SGX_to_launch_securities_market_trade_at_close_session_on_3_June.pdf
