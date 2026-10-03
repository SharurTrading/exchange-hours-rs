<!-- SPDX-License-Identifier: MIT-0 -->

# `nzx` — evidence

- **Kind:** `Exchange`
- **Owner module:** [`nzx.rs`](../../src/calendar/schedules/equities/apac/nzx.rs)
- **Source sets:** [`APAC-NZX`](../schedules/sources.md#apac-nzx)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

Main Board; deterministic edges for randomized auctions.

## Revision rows

- 2020-04-06 — T1 — NZX announcement 350919 — pre-open start moves 09:00 → 08:30.

## Normal week

**The pre-2020 grid is the operator's own print of 2010-01-05, and every later
page state corroborates it to the dated revision.** The operator's
`nzx.com/markets/key-dates/trading-hours` page as served 2010-01-05
(`NZX-KD-2010-01-05`) prints, for the NZSX & NZAX markets: Enquiry
8.00am-9.00am, Pre-open 9.00am-10.00am, Normal Trading 10.00am-4.45pm,
Pre-close 4.45pm-5.00pm, Adjust 5.00pm-5.30pm, closing Enquiry 5.30pm. That is
exactly the pre-2020 profile the module encodes: `regular` 10:00-16:45, the
tradeable Pre-open 09:00-09:59:30 in `extended`, and the order-entry Pre-close
16:45-16:59:30 with the ±30-second closing-uncross envelope to 17:00:30. The
same grid appears in the operator's 2010-12-29, 2011-12-19, 2012-05-04,
2013-01-16, 2013-05-16, 2014-01-27, 2014-10-20, 2015-01-13, 2015-04-27,
2017-06-23, 2018-08-24, 2019-03-25 and 2020-06-08 page states (all held in the
store, each cited in the holiday `### Documents` table), and the 2020-04-06
revision row keys the one change the era ends with. The ledger horizon is
therefore 2010-01-05, the capture day: below it the carried region contains no
trade date (2010-01-01 and 2010-01-04 are the sheet's own New Year closures and
2010-01-02/03 fall at the weekend).

**Corroboration across the floor.** The same page as served 2009-12-04
(`NZX-KD-2009-12-04`) prints the identical grid, so the state attested on
2010-01-05 did not begin there; the capture dates no 2010 day, so the horizon
stays at the capture rather than the floor. A 2009-era capture attests the
state on its own day only.

**What the era's sheets do not print.** The ±30-second uncross envelope (the
09:59:30-10:00:00 and 16:59:30-17:00:30 `extended` slices) rests on the
operator's Anatomy-of-a-Trading-Day randomisation statement already carried by
the normal-week profile; the 2010-2020 page states print the phase grid, not
the randomisation. That is the same disclosure the holiday section records for
the pre-2025 abbreviated days, and the envelope is the one instant family the
era's sheets do not state.


## Sources

Row review: 2026-08-22 (UTC) is the date the ledger row was last reviewed as a
whole. Per-source retrieval dates were not recorded before the 2026-09-12
migration; where a later targeted review, capture or document date is recorded
beside a source below, that date governs for that source, and dates are added
as each source is re-verified.

- <https://www.nzx.com/learning/help-reference/trading-hours> — NZX trading hours (live home of the holiday table since 2025; earlier homes and their captures are listed in the `### Documents` tables).
- <https://www.nzx.com/announcements/294559> — the operator's *"NZX Market Holidays – 2016/2017"* memorandum (19 December 2016), live at retrieval (2026-10-02 UTC); the `NZX-MEMO-2016-2017` artifact below.
- <https://www.nzx.com/announcements/312156> — the operator's *"NZX Market Holidays – 2017/2018"* memorandum (19 December 2017), retrieved live as corroboration the same day.
- `https://nzxfutures.com/system/downloads/15/` — the operator's derivatives-site download slot holding the holiday-memo series as PDFs (the site now 404s live; the editions above survive as Wayback `id_` replays, retrieved 2026-10-03 UTC and digested in the `### Documents` table).
- <https://www.nzx.com/learning/issuer-participant-resources/nzx-trading/anatomy-of-a-trading-day> — NZX Anatomy of a Trading Day. Of Pre-Open it says: "Orders can be placed, amended, and deleted. No trades execute until the opening auction. Off-market trades may be reported." Off-market reports print, so Pre-Open is tradeable `extended`, not order-entry-only.
- <https://www.nzx.com/announcements/350919> — NZX announcement 350919, the 2020-04-06 pre-open move.
- <https://www.nzx.com/announcements/353837> — NZX announcement 353837, making the initially temporary change indefinite.
- The operator's pre-2025 trading-hours pages and their Wayback `id_` replays — `nzx.com/markets/key-dates/trading-hours` (2010-2011), `nzx.com/markets/NZSX/trading_hours` (2011-2017), `nzx.com/Derivatives/trading_hours` (2017; found by the 2026-09-30 UTC re-sweep, artifact in the store's `cdx-retry-2026-09-30/`), `nzx.com/investing/nzx-trading-hours` (2018-2019), `nzx.com/services/nzx-trading/hours-boards` (2020-2024) — retrieved 2026-09-29 (UTC) as `holidays/raw/equities/nzx/2010-2024/` (research store), digests in the `### Documents` table below. Each page prints the operator's own `Market Closed or Abbreviated Trading` / `NZX Market Holidays` table and the era's normal and abbreviated phase grids.

## Holidays

**Coverage:** 2010-01-01..2027-01-04 (inclusive trade dates, one unbroken span since the former mid-year capture gap closed; recorded below). Tier: T1 throughout.

NZX prints no consolidated year sheets for most of the window. The operator's
own `Market Holidays & Trading Hours` statements are keyed to the page that
carried them at the time: the `nzx.com/markets/key-dates/trading-hours` page
(2010-2011) prints one "Market holidays & abbreviated trading days for <year>"
sheet per year; the `nzx.com/markets/NZSX/trading_hours` Main Board page
(2011-2017) prints a rolling "Market Closed or Abbreviated Trading" table of
upcoming closures, roughly twelve months deep; the
`nzx.com/investing/nzx-trading-hours` page (2018-2019) prints the current
year's full table; and the `nzx.com/services/nzx-trading/hours-boards` page
(2020-2024) prints the current year's full `NZX Market Holidays` table. Each
page also prints the era's own normal and abbreviated phase grids, from which
the replacement-block days below are restated. Every pre-2025 artifact is a
Wayback `id_` replay of the operator's own page — verbatim bytes of an
operator statement, so T1 (LAW-PUBLIC-SOURCES).

**The 2016-2017 capture gap, narrowed 2026-09-30 and again 2026-10-02.** The NZSX page's surviving
replays run 2015-04-27 (printing upcoming closures through ANZAC Day,
2016-04-25) and then 2017-06-23 (printing upcoming closures from Labour Day,
2017-10-23). The 2026-09-30 UTC re-sweep with the CDX service working (the
2026-09-29 sweeps ran while the service was failing; outputs in the store's
`holidays/raw/equities/nzx/cdx-retry-2026-09-30/`) surfaced one more in-span
operator page: the Derivatives trading-hours page of 2017-07-18
(`nzx.com/Derivatives/trading_hours`, `NZX-DX-2017-07-18`), whose own
`Market Closed or Abbreviated Trading` table — the same table the Main Board
page carries — prints the closures retrospectively from Good Friday 2017-04-14
through the 2018 arrangement, agreeing with the 2017-06-23 Main Board capture
on every shared date, so the 2017-04-14..2017-10-22 span keys to it and the
audited window runs unbroken from 2017-04-14. **The 2026-10-02 UTC pass found
the season's own operator artifact live:** NZX's memorandum *"NZX Market
Holidays – 2016/2017"* (announcement 294559, issued by NZX Client and Data
Services to all NZX Market Participants, 19 December 2016,
`NZX-MEMO-2016-2017`) is still served at
`https://www.nzx.com/announcements/294559` and prints the complete 2016/2017
table in session language — `23 December 2016 -  Abbreviated Trading`,
`26 December 2016 - Boxing Day - Closed`,
`27 December 2016 - Christmas Day Observed - Closed`,
`30 December 2016   - Abbreviated Trading`,
`2 January 2017 - New Year's Day Observed - Closed`,
`3 January 2017 - Day after New Year's Day Observed - Closed`,
`6 February 2017 - Waitangi Day - Closed`,
`14 April 2017 - Good Friday - Closed`,
`17 April 2017 - Easter Monday - Closed`,
`25 April 2017 - ANZAC Day - Closed`,
`5 June 2017 - Queen's Birthday - Closed` and
`23 October 2017 - Labour Day - Closed`. Its five `Closed` dates inside the
former gap key as rows (`NZX-MEMO-2016-2017`), its two abbreviated days do the
same as replacement days (see the abbreviated-grid note below), and its five
post-gap dates — Good Friday 2017-04-14 through Labour Day 2017-10-23 — agree
with the shipped `NZX-DX-2017-07-18` and `NZX-SX-2017-06-23` rows on every
shared date, so they corroborate the series and change nothing. The windows
re-join from 2016-12-23, the memorandum's own first printed day.

**The former 2016-04-26..2016-12-22 gap closed 2026-10-03 UTC.** The season's
announcement-system edition — *"NZX Market Holidays – 2015/2016"* — was
established 2026-10-02 UTC to be purged from the operator's announcement
system (ids below ~292000-292500 answer 404) and never captured; the
2026-10-03 UTC round-2 hunt re-checked the remaining channels and holds the
negatives (probe artifacts in the store's `evidence-thread/`): the Wayback
domain sweep for 2016-04-26..2016-12-31 (266 captures — assets, one
securities page and the `companyresearch.nzx.com` subdomain only; the three
trading-hours pages have no 200-capture between 2015-04-28 and 2017-06-23,
so no rolling-table state ever printed the span), the live
`companyresearch.nzx.com` research portal (announcement views answer 410
Gone; its captures are 2008-2010), the live `announcements.nzx.com` platform
(recent-window only; old ids 404), the operator's static S3 export
`nzx-prod-s7fsd7f98s.s3-website-ap-southeast-2.amazonaws.com` (a 2019-era
prerender; listing denied), the Mondo Visione news index (every page from
2015-12-15 to 2015-12-31 and 2016-12-14 to 2016-12-23 scanned: NZX's
same-day media releases are carried, no holiday memorandum ever was), and
the ShareChat mirror (announcement pages 410/500 live; NZXO listings serve
an empty shell). The recovery came from the download system of the
operator's own derivatives site, `nzxfutures.com` — a host no earlier pass
had swept: its `/system/downloads/15/` slot holds the holiday-memo series as
PDFs, and Wayback capture `20170520051811` replays
`Market_Holidays_Memo.pdf` — *"NZX Dairy Derivatives Market Holidays –
2015/2016"*, NZX Client and Market Services, 20 November 2015
(`NZX-DD-2015-11-20`) — verbatim. Its complete season table prints, in
session language, `6 June 2016 Queen's Birthday Closed` and
`24 October 2016 Labour Day Closed`, and nothing else between `25 April
2016 ANZAC Day Closed` and `24 December 2016 Christmas Eve Abbreviated
Trading`; the two rows key to it and its completeness audits the ordinary
days between them, which re-joins the window into one unbroken
2010-01-01..2027-01-04 span. Issue
[#209](https://github.com/SharurTrading/exchange-hours-rs/issues/209) closes
with this data.

**Market scope.** The 2015/2016 edition is addressed to the dairy-derivatives
market — the announcement-system Main Board edition it parallels is the one
that was purged — so the rows it keys rest on the market-scope step this
file records and the reader can re-verify: the series' holiday grid is the
exchange-wide national-holiday grid, identical on every date any two of its
editions and the Main Board sheets share. The three surviving editions
prove it across the era: the 2013-09-17 derivatives memorandum prints
Queen's Birthday 2014-06-02 and Labour Day 2014-10-27 exactly as the Main
Board sheet `NZX-SX-2014-01-27` does; the 2015-01-16 dairy memorandum prints
ANZAC Observed 2015-04-27, Queen's Birthday 2015-06-01 and Labour Day
2015-10-26 exactly as `NZX-SX-2015-01-13` does; and the 2015/2016 edition's
every shared date — 2015-12-24, 25, 28, 31, 2016-01-01, 01-04, 02-08,
03-25, 03-28, 04-25, and 2016-12-26, 27 against the 2016/2017 memorandum —
matches the shipped Main Board rows verbatim. Two disclosed conventions
bound the step: the memos list trading-affecting dates only (a weekend
holiday such as Saturday 2016-02-06 prints no row — the Main Board sheet's
own Saturday row is untouched), and the one known cross-market divergence
is the December abbreviated days (dairy Christmas Eve 24 December 2016
against Main Board 23 December 2016), which sit outside the keyed span and
stay keyed to the Main Board memorandum alone.

**Abbreviated trading days are replacement days, not scalar early closes.**
The operator's own abbreviated column keeps a tradeable closing auction after
the shortened Normal Trading window: Pre-Close runs to the abbreviated close
and the closing uncross randomises within 30 seconds either side of it, exactly
as it does around 5:00pm on a full day (the same anatomy-of-a-trading-day
statement the normal-week profile cites for the ±30 second envelope). A scalar
`EarlyClose` clip cannot state that day: clipped at the Normal-Trading end it
deletes the auction prints (an executable window the operator keeps open), and
clipped at the uncross envelope's end it drags the order-entry-only Pre-Close
queue inside `is_open`, which the charter's order-entry rule forbids. Each
abbreviated day therefore ships as a replacement block set restating the
operator's grid block for block, with Enquiry and Adjust excluded exactly as
on a full day. Three grids are in force across the window, each printed by the
era's own page:

- **2010-2012** (`ERA_2010_ABBREVIATED_DAY_BLOCKS`): Pre-open 9:00am-10:00am
  (tradeable: off-market reports print), Normal Trading 10:00am-3:45pm,
  Pre-Close 3:45pm-4:00pm, Adjust 4:00pm-4:30pm — read from the 2010-01-05 and
  2011-12-19 pages' abbreviated columns. The Pre-Close slice stops at 3:59:30
  and the uncross envelope runs 3:59:30-4:00:30.
- **2013-2020** (`ERA_2013_ABBREVIATED_DAY_BLOCKS`): Pre-open 9:00am-10:00am,
  Normal Trading 10:00am-12:45pm, Pre-Close 12:45pm-1:00pm, Adjust
  1:00pm-1:30pm — read from the 2013-01-16 page onward. The Pre-Close slice
  stops at 12:59:30 and the uncross envelope runs 12:59:30-1:00:30. The
  2013-05-16 capture's own grid still shows the older 15:45 column; see the
  conflict note below. The 2016 abbreviated days the 2016/2017 memorandum keys
  (23 and 30 December) hold the same 12:45 grid at its narrowest sourced value
  across the capture span: the 2015-04-27 page before it and the 2017-06-23
  page after it print the identical 12:45 abbreviated column, and no capture
  survives between them (the 2017-07-18 page is the Equity Derivatives
  trading-hours page, whose abbreviated column is the derivatives grid — it
  corroborates the closures, not this grid) (AGENTS.md, *Prefer the sourced
  intersection to omission*).
- **2021 onward** (`ABBREVIATED_DAY_BLOCKS`): the same 12:45pm grid with the
  8:30am Pre-open, read from the 2021-01-12 page and the current table.

The block instants are the sheets' own phase boundaries; only the ±30 second
uncross envelopes are the operator's randomisation statement already carried by
the normal-week profile, and on the pre-2020 dates the profile itself is
carried, so the envelope rests on that carried convention rather than a 2010
operator statement — disclosed here because it is the one instant the era's
sheets do not print.

**The 2013 abbreviated-grid conflict, held at the narrowest bound.** The
operator's 2013-01-16 page prints the 12:45 abbreviated grid (Normal Trading
to 12:45pm) while its 2013-05-16 page — the capture whose holiday table keys
the 2013-12-24 and 2013-12-31 abbreviated days — still shows the older 15:45
column in its trading-hours grid, and every later capture (2014-01-27 onward)
prints 12:45 again. No operator statement dates the changeover between the
two grids, so the two 2013 abbreviated days hold the close at its narrowest
sourced value across the undated span, 12:45pm, and restate the 12:45 grid
the 2013-01-16 page prints (AGENTS.md, *Prefer the sourced intersection to
omission*). The 12:45pm-4:30pm span the two grids dispute ships as no session
rather than guessed at, so if the older grid actually governed those days the
answer understates the open window; the narrowest-bound rule prefers that
error to inventing the changeover day.

**The 2020 Pre-open conflict, held at the narrowest bound.** The operator's
2020-06-08 page still prints the 9:00am Pre-open (both columns) while its
2021-01-12 page prints 8:30am, and no capture survives between them; the
2020-04-06 announcement (350919) keyed the move and announcement 353837 made
it indefinite, but the interim state across December 2020 is not printed by
any surviving artifact. The two 2020 abbreviated days therefore hold the
Pre-open at 9:00am — the narrowest sourced value across the undated span
(AGENTS.md, *Prefer the sourced intersection to omission*) — and the disputed
8:30-9:00 hour stays out of the replacement blocks.

### 2010

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2010-01-01 | closed | `New Year's Day` — `Friday, 1 Jan` — `Closed` | `NZX-KD-2010-01-05` | T1 | NZX event date printed verbatim; no Main Board session belongs to the date |
| 2010-01-04 | closed | `New Year's Day Holiday` — `Mon, 4 Jan` — `Closed` | `NZX-KD-2010-01-05` | T1 | the sheet's own observed date for the Saturday 2 January holiday |
| 2010-02-06 | closed | `Waitangi Day Sat, 06 Feb` — `Closed` | `NZX-KD-2010-01-05` | T1 | NZX event date printed verbatim (a Saturday the sheet prints; it restates the normal week's closure) |
| 2010-04-01 | replacement blocks | `Thursday, 1 Apr` — `Abbreviated Trading`; abbreviated grid: Normal Trading `10.00am - 3.45pm`, Pre-close `3.45pm - 4.00pm`, Adjust `4.00pm - 4.30pm` | `NZX-KD-2010-01-05` | T1 | see the interpretive step — the row restates the operator's own abbreviated grid as one replacement day |
| 2010-04-02 | closed | `Good Friday: Fri, 02 Apr` — `Closed` | `NZX-KD-2010-01-05` | T1 | NZX event date printed verbatim |
| 2010-04-05 | closed | `Easter Monday: Mon, 05 Apr` — `Closed` | `NZX-KD-2010-01-05` | T1 | NZX event date printed verbatim |
| 2010-04-25 | closed | `ANZAC Day: Sun, 25 Apr` — `Closed` | `NZX-KD-2010-01-05` | T1 | NZX event date printed verbatim (a Sunday the sheet prints; it restates the normal week's closure) |
| 2010-06-07 | closed | `Queens Birthday: Mon, 7 Jun` — `Closed` | `NZX-KD-2010-01-05` | T1 | NZX event date printed verbatim |
| 2010-10-25 | closed | `Labour Day: Mon, 25 Oct` — `Closed` | `NZX-KD-2010-01-05` | T1 | NZX event date printed verbatim |
| 2010-12-24 | replacement blocks | `Friday, 24 Dec` — `Abbreviated Trading`; same abbreviated grid | `NZX-KD-2010-01-05` | T1 | same reading as 2010-04-01 |
| 2010-12-27 | closed | `Christmas Day: Mon, 27 Dec` — `Closed` | `NZX-KD-2010-01-05` | T1 | the sheet's own observed date for the Saturday 25 December holiday |
| 2010-12-28 | closed | `Boxing Day: Tue, 28 Dec` — `Closed` | `NZX-KD-2010-01-05` | T1 | the sheet's own observed date for the Sunday 26 December holiday |
| 2010-12-31 | replacement blocks | `Friday, 31 Dec` — `Abbreviated Trading` | `NZX-KD-2010-01-05` | T1 | same reading as 2010-04-01 |

### 2011

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2011-01-03 | closed | `New Year's Day: Monday, 03 Jan` — `Closed` | `NZX-KD-2010-12-29` | T1 | the sheet's own observed date for the Saturday 1 January holiday |
| 2011-01-04 | closed | `New Year's Day Holiday: Tuesday, 04 Jan` — `Closed` | `NZX-KD-2010-12-29` | T1 | the sheet's own observed date for the Sunday 2 January holiday |
| 2011-02-06 | closed | `Waitangi Day: Sunday, 06 Feb` — `Closed` | `NZX-KD-2010-12-29` | T1 | NZX event date printed verbatim (a Sunday the sheet prints; it restates the normal week's closure) |
| 2011-04-21 | replacement blocks | `Thursday, 21 Apr` — `Abbreviated Trading` | `NZX-KD-2010-12-29` | T1 | same reading as 2010-04-01 |
| 2011-04-22 | closed | `Good Friday: Friday, 22 Apr` — `Closed` | `NZX-KD-2010-12-29` | T1 | NZX event date printed verbatim |
| 2011-04-25 | closed | `Easter Monday: Monday, 25 Apr` and `ANZAC Day: Monday, 25 Apr` — `Closed` | `NZX-KD-2010-12-29` | T1 | NZX prints both events on one date; one row, the date printed verbatim |
| 2011-06-06 | closed | `Queens Birthday: Monday, 6 Jun` — `Closed` | `NZX-KD-2010-12-29` | T1 | NZX event date printed verbatim |
| 2011-10-24 | closed | `Labour Day: Monday, 24 Oct` — `Closed` | `NZX-KD-2010-12-29` | T1 | NZX event date printed verbatim |
| 2011-12-23 | replacement blocks | `Christmas Eve: Friday, 23 Dec` — `Abbreviated Trading` | `NZX-KD-2010-12-29` | T1 | same reading as 2010-04-01 |
| 2011-12-26 | closed | `Boxing Day: Monday, 26 Dec` — `Closed` | `NZX-KD-2010-12-29` | T1 | the sheet's own observed date for the Sunday 26 December holiday |
| 2011-12-27 | closed | `Christmas Day: Tuesday, 27 Dec` — `Closed` | `NZX-KD-2010-12-29` | T1 | the sheet's own observed date for the Saturday 25 December holiday |
| 2011-12-30 | replacement blocks | `Friday, 30 Dec` — `Abbreviated Trading` | `NZX-KD-2010-12-29` | T1 | same reading as 2010-04-01 |

### 2012

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2012-01-02 | closed | `2 Jan 2012 New Year` — `Closed` | `NZX-SX-2011-12-19` | T1 | the sheet's own observed date for the Sunday 1 January holiday |
| 2012-01-03 | closed | `3 Jan 2012 New Year` — `Closed` | `NZX-SX-2011-12-19` | T1 | the sheet's own observed date for the Monday 2 January holiday |
| 2012-02-06 | closed | `6 Feb 2012 Waitangi Day` — `Closed` | `NZX-SX-2011-12-19` | T1 | NZX event date printed verbatim |
| 2012-04-05 | replacement blocks | `5 Apr 2012` — `Abbreviated`; abbreviated grid: Normal Trading `10:00am - 3:45pm`, Pre-close `3:45pm - 4:00pm`, Adjust `4:00pm - 4:30pm` | `NZX-SX-2011-12-19` | T1 | same reading as 2010-04-01 |
| 2012-04-06 | closed | `6 Apr 2012 Good Friday` — `Closed` | `NZX-SX-2011-12-19` | T1 | NZX event date printed verbatim |
| 2012-04-09 | closed | `9 Apr 2012 Easter Monday` — `Closed` | `NZX-SX-2011-12-19` | T1 | NZX event date printed verbatim |
| 2012-04-25 | closed | `25 Apr 2012 Anzac Day` — `Closed` | `NZX-SX-2011-12-19` | T1 | NZX event date printed verbatim |
| 2012-06-04 | closed | `4 Jun 2012 Queen's Birthday` — `Closed` | `NZX-SX-2012-05-04` | T1 | NZX event date printed verbatim (the 2011-12-19 replay prints the same row) |
| 2012-10-22 | closed | `22 Oct 2012 Labour Day` — `Closed` | `NZX-SX-2012-05-04` | T1 | NZX event date printed verbatim (the 2011-12-19 replay prints the same row) |
| 2012-12-24 | replacement blocks | `24 Dec 2012` — `Abbreviated` | `NZX-SX-2012-05-04` | T1 | same reading as 2010-04-01 |
| 2012-12-25 | closed | `25 Dec 2012 Christmas Day` — `Closed` | `NZX-SX-2012-05-04` | T1 | NZX event date printed verbatim |
| 2012-12-26 | closed | `26 Dec 2012 Boxing Day` — `Closed` | `NZX-SX-2012-05-04` | T1 | NZX event date printed verbatim |
| 2012-12-31 | replacement blocks | `31 Dec 2012` — `Abbreviated` | `NZX-SX-2012-05-04` | T1 | same reading as 2010-04-01 |

### 2013

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2013-01-01 | closed | `1 Jan 2013 New Year` — `Closed` | `NZX-SX-2012-05-04` | T1 | NZX event date printed verbatim |
| 2013-01-02 | closed | `2 Jan 2013 New Year` — `Closed` | `NZX-SX-2012-05-04` | T1 | NZX event date printed verbatim |
| 2013-02-06 | closed | `6 Feb 2013 Waitangi Day` — `Closed` | `NZX-SX-2013-01-16` | T1 | NZX event date printed verbatim |
| 2013-03-29 | closed | `29 Mar 2013 Good Friday` — `Closed` | `NZX-SX-2013-01-16` | T1 | NZX event date printed verbatim |
| 2013-04-01 | closed | `1 Apr 2013 Easter Monday` — `Closed` | `NZX-SX-2013-01-16` | T1 | NZX event date printed verbatim |
| 2013-04-25 | closed | `25 Apr 2013 Anzac Day` — `Closed` | `NZX-SX-2013-01-16` | T1 | NZX event date printed verbatim |
| 2013-06-03 | closed | `3 Jun 2013 Queen's Birthday` — `Closed` | `NZX-SX-2013-01-16` | T1 | NZX event date printed verbatim (the 2013-05-16 replay prints the same row) |
| 2013-10-28 | closed | `28 Oct 2013 Labour Day` — `Closed` | `NZX-SX-2013-01-16` | T1 | NZX event date printed verbatim (the 2013-05-16 replay prints the same row) |
| 2013-12-24 | replacement blocks | `24 Dec 2013` — `Abbreviated` | `NZX-SX-2013-05-16` | T1 | the date is the sheet's own; the block instants read from the 2013-01-16 page's 12:45 abbreviated grid — Normal Trading `10:00am - 12:45pm`, Pre-close `12:45pm - 1:00pm`, Adjust `1:00pm - 1:30pm` — held at the narrowest bound across the 2013-05-16 capture's 15:45 column (see the conflict note) |
| 2013-12-25 | closed | `25 Dec 2013 Christmas Day` — `Closed` | `NZX-SX-2013-05-16` | T1 | NZX event date printed verbatim |
| 2013-12-26 | closed | `26 Dec 2013 Boxing Day` — `Closed` | `NZX-SX-2013-05-16` | T1 | NZX event date printed verbatim |
| 2013-12-31 | replacement blocks | `31 Dec 2013` — `Abbreviated` | `NZX-SX-2013-05-16` | T1 | same reading as 2013-12-24 |

### 2014

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2014-01-01 | closed | `1 Jan 2014 New Year` — `Closed` | `NZX-SX-2013-05-16` | T1 | NZX event date printed verbatim (the 2014-01-27 replay prints the same row) |
| 2014-01-02 | closed | `2 Jan 2014 New Year` — `Closed` | `NZX-SX-2013-05-16` | T1 | NZX event date printed verbatim (the 2014-01-27 replay prints the same row) |
| 2014-02-06 | closed | `6 Feb 2014 Waitangi Day` — `Closed` | `NZX-SX-2014-01-27` | T1 | NZX event date printed verbatim |
| 2014-04-18 | closed | `18 Apr 2014 Good Friday` — `Closed` | `NZX-SX-2014-01-27` | T1 | NZX event date printed verbatim |
| 2014-04-21 | closed | `21 Apr 2014 Easter Monday` — `Closed` | `NZX-SX-2014-01-27` | T1 | NZX event date printed verbatim |
| 2014-04-25 | closed | `25 Apr 2014 Anzac Day` — `Closed` | `NZX-SX-2014-01-27` | T1 | NZX event date printed verbatim |
| 2014-06-02 | closed | `2 Jun 2014 Queen's Birthday` — `Closed` | `NZX-SX-2014-01-27` | T1 | NZX event date printed verbatim |
| 2014-10-27 | closed | `27/10/2014 Labour Day` — `Closed` | `NZX-SX-2014-01-27` | T1 | NZX event date printed verbatim (the 2014-10-20 replay prints the same row) |
| 2014-12-24 | replacement blocks | `24/12/2014 Christmas Eve` — `Abbreviated` | `NZX-SX-2014-10-20` | T1 | the 12:45 abbreviated grid, same reading as 2013-12-24 |
| 2014-12-25 | closed | `25/12/2014 Christmas Day` — `Closed` | `NZX-SX-2014-10-20` | T1 | NZX event date printed verbatim |
| 2014-12-26 | closed | `26/12/2014 Boxing Day` — `Closed` | `NZX-SX-2014-10-20` | T1 | NZX event date printed verbatim |
| 2014-12-31 | replacement blocks | `31/12/2014 New Years Eve` — `Abbreviated` | `NZX-SX-2014-10-20` | T1 | same reading as 2013-12-24 |

### 2015

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2015-01-01 | closed | `01/01/2015 New Year` — `Closed` | `NZX-SX-2014-10-20` | T1 | NZX event date printed verbatim |
| 2015-01-02 | closed | `02/01/2015 New Year` — `Closed` | `NZX-SX-2014-10-20` | T1 | NZX event date printed verbatim |
| 2015-02-06 | closed | `06/02/2015 Waitangi Day` — `Closed` | `NZX-SX-2015-01-13` | T1 | NZX event date printed verbatim |
| 2015-04-03 | closed | `03/04/2015 Good Friday` — `Closed` | `NZX-SX-2015-01-13` | T1 | NZX event date printed verbatim |
| 2015-04-06 | closed | `06/04/2015 Easter Monday` — `Closed` | `NZX-SX-2015-01-13` | T1 | NZX event date printed verbatim |
| 2015-04-25 | closed | `25/04/2015 Anzac Day` — `Closed` | `NZX-SX-2015-01-13` | T1 | NZX event date printed verbatim (a Saturday the sheet prints) |
| 2015-04-27 | closed | `27/04/2015 Anzac Observance` — `Closed` | `NZX-SX-2015-01-13` | T1 | the sheet's own mondayised date |
| 2015-06-01 | closed | `01/06/2015 Queen's Birthday` — `Closed` | `NZX-SX-2015-01-13` | T1 | NZX event date printed verbatim (the 2015-04-27 replay prints the same row) |
| 2015-10-26 | closed | `26/10/2015 Labour Day` — `Closed` | `NZX-SX-2015-01-13` | T1 | NZX event date printed verbatim (the 2015-04-27 replay prints the same row) |
| 2015-12-24 | replacement blocks | `24/12/2015 Christmas Eve` — `Abbreviated` | `NZX-SX-2015-01-13` | T1 | the 12:45 abbreviated grid, same reading as 2013-12-24 |
| 2015-12-25 | closed | `25/12/2015 Christmas Day` — `Closed` | `NZX-SX-2015-01-13` | T1 | NZX event date printed verbatim |
| 2015-12-28 | closed | `28/12/2015 Boxing Day` — `Closed` | `NZX-SX-2015-01-13` | T1 | the sheet's own observed date for the Saturday 26 December holiday |
| 2015-12-31 | replacement blocks | `31/12/2015 New Years Eve` — `Abbreviated` | `NZX-SX-2015-01-13` | T1 | same reading as 2013-12-24 |

### 2016

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2016-01-01 | closed | `01/01/2016 New Year` — `Closed` | `NZX-SX-2015-04-27` | T1 | NZX event date printed verbatim |
| 2016-01-04 | closed | `04/01/2016 New Year` — `Closed` | `NZX-SX-2015-04-27` | T1 | the sheet's own observed date for the Saturday 2 January holiday |
| 2016-02-06 | closed | `06/02/2016 Waitangi Day` — `Closed` | `NZX-SX-2015-04-27` | T1 | NZX event date printed verbatim (a Saturday the sheet prints) |
| 2016-02-08 | closed | `08/02/2016 Waitangi Observance` — `Closed` | `NZX-SX-2015-04-27` | T1 | the sheet's own mondayised date |
| 2016-03-25 | closed | `25/03/2016 Good Friday` — `Closed` | `NZX-SX-2015-04-27` | T1 | NZX event date printed verbatim |
| 2016-03-28 | closed | `28/03/2016 Easter Monday` — `Closed` | `NZX-SX-2015-04-27` | T1 | NZX event date printed verbatim |
| 2016-04-25 | closed | `25/04/2016 Anzac Day` — `Closed` | `NZX-SX-2015-04-27` | T1 | NZX event date printed verbatim; the last date the NZSX page's surviving replays print before the operator's next artifact in the era |
| 2016-06-06 | closed | `6 June 2016 Queen's Birthday Closed` | `NZX-DD-2015-11-20` | T1 | NZX event date printed verbatim in the dairy-derivatives memorandum's season table (see the market-scope paragraph above) |
| 2016-10-24 | closed | `24 October 2016 Labour Day Closed` | `NZX-DD-2015-11-20` | T1 | NZX event date printed verbatim in the same season table |
| 2016-12-23 | replacement blocks | `23 December 2016 -  Abbreviated Trading` | `NZX-MEMO-2016-2017` | T1 | the memorandum's own date; the 12:45 abbreviated grid held at its narrowest sourced value across the 2015-04-27..2017-06-23 capture span (see the grid bullets above) |
| 2016-12-26 | closed | `26 December 2016 - Boxing Day - Closed` | `NZX-MEMO-2016-2017` | T1 | NZX event date printed verbatim; the memorandum's own first `Closed` row |
| 2016-12-27 | closed | `27 December 2016 - Christmas Day Observed - Closed` | `NZX-MEMO-2016-2017` | T1 | the memo's own observed date for the Sunday 25 December holiday |
| 2016-12-30 | replacement blocks | `30 December 2016   - Abbreviated Trading` | `NZX-MEMO-2016-2017` | T1 | same reading as 2016-12-23 |

The June and October rows come from the series' 2015/2016 edition on the
operator's derivatives site (recovered 2026-10-03 UTC, see the coverage
paragraph); the season list between them prints nothing else, so the days
without rows are audited normal.

### 2017

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2017-01-02 | closed | `2 January 2017 - New Year's Day Observed - Closed` | `NZX-MEMO-2016-2017` | T1 | the memo's own observed date for the Sunday 1 January holiday |
| 2017-01-03 | closed | `3 January 2017 - Day after New Year's Day Observed - Closed` | `NZX-MEMO-2016-2017` | T1 | the memo's own observed date for the Saturday 2 January holiday |
| 2017-02-06 | closed | `6 February 2017 - Waitangi Day - Closed` | `NZX-MEMO-2016-2017` | T1 | NZX event date printed verbatim; the memo's last date inside the former capture gap |
| 2017-04-14 | closed | `14/04/2017 Good Friday` — `Closed` | `NZX-DX-2017-07-18` | T1 | NZX event date printed verbatim; the Derivatives page table's own first row — the 2016/2017 memorandum prints the same row and covers every date before it from 2016-12-23 |
| 2017-04-17 | closed | `17/04/2017 Easter Monday` — `Closed` | `NZX-DX-2017-07-18` | T1 | NZX event date printed verbatim |
| 2017-04-25 | closed | `25/04/2017 Anzac Day` — `Closed` | `NZX-DX-2017-07-18` | T1 | NZX event date printed verbatim |
| 2017-06-05 | closed | `05/06/2017 Queens Birthday` — `Closed` | `NZX-DX-2017-07-18` | T1 | NZX event date printed verbatim (the operator's own unapostrophised print) |
| 2017-10-23 | closed | `23/10/2017 Labour Day` — `Closed` | `NZX-SX-2017-06-23` | T1 | NZX event date printed verbatim; the first date the next surviving Main Board artifact prints after the capture gap — the Derivatives artifact prints the same date |
| 2017-12-22 | replacement blocks | `22/12/2017 Christmas Eve` — `Abbreviated` | `NZX-SX-2017-06-23` | T1 | the 12:45 abbreviated grid, same reading as 2013-12-24 |
| 2017-12-25 | closed | `25/12/2017 Christmas Day` — `Closed` | `NZX-SX-2017-06-23` | T1 | NZX event date printed verbatim |
| 2017-12-26 | closed | `26/12/2017 Boxing Day` — `Closed` | `NZX-SX-2017-06-23` | T1 | NZX event date printed verbatim |
| 2017-12-29 | replacement blocks | `29/12/2017 New Years Eve` — `Abbreviated` | `NZX-SX-2017-06-23` | T1 | same reading as 2013-12-24 |

### 2018

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2018-01-01 | closed | `01/01/2018 New Years Day` — `Closed` | `NZX-TH-2018-08-24` | T1 | NZX event date printed verbatim |
| 2018-01-02 | closed | `02/01/2018 Day after New Year's Day` — `Closed` | `NZX-TH-2018-08-24` | T1 | NZX event date printed verbatim |
| 2018-02-06 | closed | `06/02/2018 Waitangi Day` — `Closed` | `NZX-TH-2018-08-24` | T1 | NZX event date printed verbatim |
| 2018-03-30 | closed | `30/03/2018 Good Friday` — `Closed` | `NZX-TH-2018-08-24` | T1 | NZX event date printed verbatim |
| 2018-04-02 | closed | `02/04/2018 Easter Monday` — `Closed` | `NZX-TH-2018-08-24` | T1 | NZX event date printed verbatim |
| 2018-04-25 | closed | `25/04/2018 Anzac Day` — `Closed` | `NZX-TH-2018-08-24` | T1 | NZX event date printed verbatim |
| 2018-06-04 | closed | `04/06/2018 Queen's Birthday` — `Closed` | `NZX-TH-2018-08-24` | T1 | NZX event date printed verbatim (the 2017-06-23 replay prints the same row) |
| 2018-10-22 | closed | `22/10/2018 Labour Day` — `Closed` | `NZX-TH-2018-08-24` | T1 | NZX event date printed verbatim |
| 2018-12-24 | replacement blocks | `24/12/2018 Business Day prior to Christmas Day` — `Abbreviated` | `NZX-TH-2018-08-24` | T1 | the 12:45 abbreviated grid, same reading as 2013-12-24 |
| 2018-12-25 | closed | `25/12/2018 Christmas Day` — `Closed` | `NZX-TH-2018-08-24` | T1 | NZX event date printed verbatim |
| 2018-12-26 | closed | `26/12/2018 Boxing Day` — `Closed` | `NZX-TH-2018-08-24` | T1 | NZX event date printed verbatim |
| 2018-12-31 | replacement blocks | `31/12/2018 Business Day prior to New Year's Day` — `Abbreviated` | `NZX-TH-2018-08-24` | T1 | same reading as 2013-12-24 |

### 2019

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2019-01-01 | closed | `01/01/2019 New Years Day` — `Closed` | `NZX-TH-2019-03-25` | T1 | NZX event date printed verbatim |
| 2019-01-02 | closed | `02/01/2019 Day after New Year's Day` — `Closed` | `NZX-TH-2019-03-25` | T1 | NZX event date printed verbatim |
| 2019-02-06 | closed | `06/02/2019 Waitangi Day` — `Closed` | `NZX-TH-2019-03-25` | T1 | NZX event date printed verbatim |
| 2019-04-19 | closed | `19/04/2019 Good Friday` — `Closed` | `NZX-TH-2019-03-25` | T1 | NZX event date printed verbatim |
| 2019-04-22 | closed | `22/04/2019 Easter Monday` — `Closed` | `NZX-TH-2019-03-25` | T1 | NZX event date printed verbatim |
| 2019-04-25 | closed | `25/04/2019 ANZAC Day` — `Closed` | `NZX-TH-2019-03-25` | T1 | NZX event date printed verbatim |
| 2019-06-03 | closed | `03/06/2019 Queen's Birthday` — `Closed` | `NZX-TH-2019-03-25` | T1 | NZX event date printed verbatim |
| 2019-10-28 | closed | `28/10/2019 Labour Day` — `Closed` | `NZX-TH-2019-03-25` | T1 | NZX event date printed verbatim |
| 2019-12-24 | replacement blocks | `24/12/2019 Business Day prior to Christmas Day` — `Abbreviated` | `NZX-TH-2019-03-25` | T1 | the 12:45 abbreviated grid, same reading as 2013-12-24 |
| 2019-12-25 | closed | `25/12/2019 Christmas Day` — `Closed` | `NZX-TH-2019-03-25` | T1 | NZX event date printed verbatim |
| 2019-12-26 | closed | `26/12/2019 Boxing Day` — `Closed` | `NZX-TH-2019-03-25` | T1 | NZX event date printed verbatim |
| 2019-12-31 | replacement blocks | `31/12/2019 Business Day prior to New Year's Day` — `Abbreviated` | `NZX-TH-2019-03-25` | T1 | same reading as 2013-12-24 |

### 2020

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2020-01-01 | closed | `01/01/2020 New Years Day` — `Closed` | `NZX-HB-2020-06-08` | T1 | NZX event date printed verbatim |
| 2020-01-02 | closed | `02/01/2020 Day after New Year's Day` — `Closed` | `NZX-HB-2020-06-08` | T1 | NZX event date printed verbatim |
| 2020-02-06 | closed | `06/02/2020 Waitangi Day` — `Closed` | `NZX-HB-2020-06-08` | T1 | NZX event date printed verbatim |
| 2020-04-10 | closed | `10/04/2020 Good Friday` — `Closed` | `NZX-HB-2020-06-08` | T1 | NZX event date printed verbatim |
| 2020-04-13 | closed | `13/04/2020 Easter Monday` — `Closed` | `NZX-HB-2020-06-08` | T1 | NZX event date printed verbatim |
| 2020-04-27 | closed | `27/04/2020 ANZAC Day observed` — `Closed` | `NZX-HB-2020-06-08` | T1 | the sheet's own observed date for the Saturday 25 April holiday; the sheet prints no Saturday row |
| 2020-06-01 | closed | `01/06/2020 Queen's Birthday` — `Closed` | `NZX-HB-2020-06-08` | T1 | NZX event date printed verbatim |
| 2020-10-26 | closed | `26/10/2020 Labour Day` — `Closed` | `NZX-HB-2020-06-08` | T1 | NZX event date printed verbatim |
| 2020-12-24 | replacement blocks | `24/12/2020 Business Day prior to Christmas Day` — `Abbreviated`; the 12:45 grid with the 9:00am Pre-open | `NZX-HB-2020-06-08` | T1 | the 2013-2020 grid; the Pre-open is the narrowest sourced value across the 2020-06-08 and 2021-01-12 replays (see the conflict note) |
| 2020-12-25 | closed | `25/12/2020 Christmas Day` — `Closed` | `NZX-HB-2020-06-08` | T1 | NZX event date printed verbatim |
| 2020-12-28 | closed | `28/12/2020 Boxing Day` — `Closed` | `NZX-HB-2020-06-08` | T1 | the sheet's own observed date for the Saturday 26 December holiday |
| 2020-12-31 | replacement blocks | `31/12/2020 Business Day Prior to New Year's Day` — `Abbreviated` | `NZX-HB-2021-01-12` | T1 | same reading as 2020-12-24 |

### 2021

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2021-01-01 | closed | `01/01/2021 New Years Day` — `Closed` | `NZX-HB-2021-01-12` | T1 | NZX event date printed verbatim |
| 2021-01-04 | closed | `04/01/2021 Day after New Year's Day` — `Closed` | `NZX-HB-2021-01-12` | T1 | the sheet's own observed date for the Saturday 2 January holiday |
| 2021-02-08 | closed | `08/02/2021 Waitangi Day` — `Closed` | `NZX-HB-2021-01-12` | T1 | the sheet's own observed date for the Saturday 6 February holiday; the sheet prints no Saturday row |
| 2021-04-02 | closed | `02/04/2021 Good Friday` — `Closed` | `NZX-HB-2021-01-12` | T1 | NZX event date printed verbatim |
| 2021-04-05 | closed | `05/04/2021 Easter Monday` — `Closed` | `NZX-HB-2021-01-12` | T1 | NZX event date printed verbatim |
| 2021-04-26 | closed | `26/04/2021 ANZAC Day observed` — `Closed` | `NZX-HB-2021-01-12` | T1 | the sheet's own observed date for the Sunday 25 April holiday |
| 2021-06-07 | closed | `07/06/2021 Queen's Birthday` — `Closed` | `NZX-HB-2021-01-12` | T1 | NZX event date printed verbatim |
| 2021-10-25 | closed | `25/10/2021 Labour Day` — `Closed` | `NZX-HB-2021-01-12` | T1 | NZX event date printed verbatim |
| 2021-12-24 | replacement blocks | `24/12/2021 Business Day prior to Christmas Day` — `Abbreviated`; the 8:30am grid | `NZX-HB-2021-01-12` | T1 | the 2021 grid: Pre-open `8:30am - 10:00am`, Normal Trading `10:00am - 12:45pm`, Pre-close `12:45pm - 1:00pm`, Adjust `1:00pm - 1:30pm` |
| 2021-12-27 | closed | `27/12/2021 Christmas Day` — `Closed` | `NZX-HB-2021-01-12` | T1 | the sheet's own observed date for the Saturday 25 December holiday |
| 2021-12-28 | closed | `28/12/2021 Boxing Day` — `Closed` | `NZX-HB-2021-01-12` | T1 | the sheet's own observed date for the Sunday 26 December holiday |
| 2021-12-31 | replacement blocks | `31/12/2021 Business Day Prior to New Year's Day` — `Abbreviated` | `NZX-HB-2021-01-12` | T1 | same reading as 2021-12-24 |

### 2022

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2022-01-03 | closed | `New Year's Day — Monday, 3 January 2022` — `Closed` | `NZX-HB-2022-02-01` | T1 | the sheet's own observed date for the Saturday 1 January holiday |
| 2022-01-04 | closed | `Day after New Year's Day — Tuesday, 4 January 2022` — `Closed` | `NZX-HB-2022-02-01` | T1 | the sheet's own observed date for the Sunday 2 January holiday |
| 2022-02-07 | closed | `Waitangi Day — Monday, 7 February 2022` — `Closed` | `NZX-HB-2022-02-01` | T1 | the sheet's own observed date for the Sunday 6 February holiday |
| 2022-04-15 | closed | `Good Friday — Friday, 15 April 2022` — `Closed` | `NZX-HB-2022-02-01` | T1 | NZX event date printed verbatim |
| 2022-04-18 | closed | `Easter Monday — Monday, 18 April 2022` — `Closed` | `NZX-HB-2022-02-01` | T1 | NZX event date printed verbatim |
| 2022-04-25 | closed | `ANZAC Day — Monday, 25 April 2022` — `Closed` | `NZX-HB-2022-02-01` | T1 | NZX event date printed verbatim |
| 2022-06-06 | closed | `Queen's Birthday — Monday, 6 June 2022` — `Closed` | `NZX-HB-2022-02-01` | T1 | NZX event date printed verbatim |
| 2022-06-24 | closed | `Matariki — Friday, 24 June 2022` — `Closed` | `NZX-HB-2022-02-01` | T1 | NZX event date printed verbatim; the holiday's first observance |
| 2022-09-26 | closed | `Queen Elizabeth II Memorial Day — Monday, 26 September` — `Closed`; footnote: `New Zealand's stock exchange (NZX) will be closed on 26 September, a national public holiday to honour the passing of Queen Elizabeth II. All NZX operations will resume on 27 September.` | `NZX-HB-2022-11-15` | T1 | the sheet's own unscheduled closure in session language |
| 2022-10-24 | closed | `Labour Day — Monday, 24 October 2022` — `Closed` | `NZX-HB-2022-11-15` | T1 | NZX event date printed verbatim |
| 2022-12-23 | replacement blocks | `Business Day Prior to Christmas Day — Friday, 23 December 2022` — `Abbreviated Trading*` | `NZX-HB-2022-11-15` | T1 | the 8:30am grid, same reading as 2021-12-24 |
| 2022-12-26 | closed | `Boxing Day — Monday, 26 December 2022` — `Closed` | `NZX-HB-2022-11-15` | T1 | the sheet's own observed date for the Monday 26 December holiday |
| 2022-12-27 | closed | `Christmas Day — Tuesday, 27 December 2022` — `Closed` | `NZX-HB-2022-11-15` | T1 | the sheet's own observed date for the Sunday 25 December holiday |
| 2022-12-30 | replacement blocks | `Business Day Prior to New Year's Day — Friday, 30 December 2022` — `Abbreviated Trading*` | `NZX-HB-2022-11-15` | T1 | same reading as 2021-12-24 |

### 2023

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2023-01-02 | closed | `New Year's Day — Monday, 2 January 2023` — `Closed` | `NZX-HB-2023-11-06` | T1 | the sheet's own observed date for the Sunday 1 January holiday |
| 2023-01-03 | closed | `Day after New Year's Day — Tuesday, 3 January 2023` — `Closed` | `NZX-HB-2023-11-06` | T1 | the sheet's own observed date for the Monday 2 January holiday |
| 2023-02-06 | closed | `Waitangi Day — Monday, 6 February 2023` — `Closed` | `NZX-HB-2023-11-06` | T1 | NZX event date printed verbatim |
| 2023-04-07 | closed | `Good Friday — Friday, 7 April 2023` — `Closed` | `NZX-HB-2023-11-06` | T1 | NZX event date printed verbatim |
| 2023-04-10 | closed | `Easter Monday — Monday, 10 April 2023` — `Closed` | `NZX-HB-2023-11-06` | T1 | NZX event date printed verbatim |
| 2023-04-25 | closed | `ANZAC Day — Tuesday, 25 April 2023` — `Closed` | `NZX-HB-2023-11-06` | T1 | NZX event date printed verbatim |
| 2023-06-05 | closed | `King's Birthday — Monday, 5 June 2023` — `Closed` | `NZX-HB-2023-11-06` | T1 | NZX event date printed verbatim |
| 2023-07-14 | closed | `Matariki — Friday, 14 July 2023` — `Closed` | `NZX-HB-2023-11-06` | T1 | NZX event date printed verbatim |
| 2023-10-23 | closed | `Labour Day — Monday, 23 October 2023` — `Closed` | `NZX-HB-2023-11-06` | T1 | NZX event date printed verbatim |
| 2023-12-22 | replacement blocks | `Business Day Prior to Christmas Day — Friday, 22 December 2023` — `Abbreviated Trading*` | `NZX-HB-2023-11-06` | T1 | the 8:30am grid, same reading as 2021-12-24 |
| 2023-12-25 | closed | `Christmas Day — Monday, 25 December 2023` — `Closed` | `NZX-HB-2023-11-06` | T1 | NZX event date printed verbatim |
| 2023-12-26 | closed | `Boxing Day — Tuesday, 26 December 2023` — `Closed` | `NZX-HB-2023-11-06` | T1 | NZX event date printed verbatim |
| 2023-12-29 | replacement blocks | `Business Day Prior to New Year's Day — Friday, 29 December 2023` — `Abbreviated Trading*` | `NZX-HB-2023-11-06` | T1 | same reading as 2021-12-24 |

### 2024

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2024-01-01 | closed | `New Year's Day — Monday, 1 January 2024` — `Closed` | `NZX-HB-2024-02-19` | T1 | NZX event date printed verbatim |
| 2024-01-02 | closed | `Day after New Year's Day — Tuesday, 2 January 2024` — `Closed` | `NZX-HB-2024-02-19` | T1 | NZX event date printed verbatim |
| 2024-02-06 | closed | `Waitangi Day — Tuesday, 6 February 2024` — `Closed` | `NZX-HB-2024-02-19` | T1 | NZX event date printed verbatim |
| 2024-03-29 | closed | `Good Friday — Friday, 29 March 2024` — `Closed` | `NZX-HB-2024-02-19` | T1 | NZX event date printed verbatim |
| 2024-04-01 | closed | `Easter Monday — Monday, 1 April 2024` — `Closed` | `NZX-HB-2024-02-19` | T1 | NZX event date printed verbatim |
| 2024-04-25 | closed | `ANZAC Day — Thursday, 25 April 2024` — `Closed` | `NZX-HB-2024-02-19` | T1 | NZX event date printed verbatim |
| 2024-06-03 | closed | `King's Birthday — Monday, 3 June 2024` — `Closed` | `NZX-HB-2024-02-19` | T1 | NZX event date printed verbatim |
| 2024-06-28 | closed | `Matariki — Friday, 28 June 2024` — `Closed` | `NZX-HB-2024-02-19` | T1 | NZX event date printed verbatim |
| 2024-10-28 | closed | `Labour Day — Monday, 28 October 2024` — `Closed` | `NZX-HB-2024-02-19` | T1 | NZX event date printed verbatim |
| 2024-12-24 | replacement blocks | `Business Day Prior to Christmas Day — Tuesday, 24 December 2024` — `Abbreviated Trading*` | `NZX-HB-2024-02-19` | T1 | the 8:30am grid, same reading as 2021-12-24 |
| 2024-12-25 | closed | `Christmas Day — Wednesday, 25 December 2024` — `Closed` | `NZX-HB-2024-02-19` | T1 | NZX event date printed verbatim |
| 2024-12-26 | closed | `Boxing Day — Thursday, 26 December 2024` — `Closed` | `NZX-HB-2024-02-19` | T1 | NZX event date printed verbatim |
| 2024-12-31 | replacement blocks | `Business Day Prior to New Year's Day — Tuesday, 31 December 2024` — `Abbreviated Trading*` | `NZX-HB-2024-02-19` | T1 | same reading as 2021-12-24 |

### 2025

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2025-01-01 | closed | `New Year's Day` — `Wednesday, 1 January 2025` — `Closed` | `NZX-TH-2024-12-16` | T1 | NZX event date printed verbatim; no Main Board session belongs to the date |
| 2025-01-02 | closed | `Day after New Year's Day` — `Thursday, 2 January 2025` — `Closed` | `NZX-TH-2024-12-16` | T1 | NZX event date printed verbatim |
| 2025-02-06 | closed | `Waitangi Day` — `Thursday, 6 February 2025` — `Closed` | `NZX-TH-2025-01-23` | T1 | NZX event date printed verbatim |
| 2025-04-18 | closed | `Good Friday` — `Friday, 18 April 2025` — `Closed` | `NZX-TH-2025-01-23` | T1 | NZX event date printed verbatim |
| 2025-04-21 | closed | `Easter Monday` — `Monday, 21 April 2025` — `Closed` | `NZX-TH-2025-01-23` | T1 | NZX event date printed verbatim |
| 2025-04-25 | closed | `ANZAC Day` — `Friday, 25 April 2025` — `Closed` | `NZX-TH-2025-01-23` | T1 | NZX event date printed verbatim |
| 2025-06-02 | closed | `King's Birthday` — `Monday, 2 June 2025` — `Closed` | `NZX-TH-2025-01-23` | T1 | NZX event date printed verbatim |
| 2025-06-20 | closed | `Matariki` — `Friday, 20 June 2025` — `Closed` | `NZX-TH-2025-01-23` | T1 | NZX event date printed verbatim |
| 2025-10-27 | closed | `Labour Day` — `Monday, 27 October 2025` — `Closed` | `NZX-TH-2025-01-23` | T1 | NZX event date printed verbatim |
| 2025-12-24 | replacement blocks | `Business Day Prior to Christmas Day` — `Wednesday, 24 December 2025` — `Abbreviated Trading*`; abbreviated grid: Normal Trading `10:00am - 12:45pm`, Pre-Close `12:45pm - 1:00pm`, Adjust `1:00pm - 1:30pm` | `NZX-TH-2025-01-23` | T1 | see the interpretive step — the row restates the operator's own abbreviated grid as one replacement day |
| 2025-12-25 | closed | `Christmas Day` — `Thursday, 25 December 2025` — `Closed` | `NZX-TH-2025-01-23` | T1 | NZX event date printed verbatim |
| 2025-12-26 | closed | `Boxing Day` — `Friday, 26 December 2025` — `Closed` | `NZX-TH-2025-01-23` | T1 | NZX event date printed verbatim |
| 2025-12-31 | replacement blocks | `Business Day Prior to New Year's Day` — `Wednesday, 31 December 2025` — `Abbreviated Trading*`; same abbreviated grid | `NZX-TH-2025-01-23` | T1 | same reading as 2025-12-24 |

### 2026

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2026-01-01 | closed | `New Year's Day` — `Thursday, 1 January 2026` — `Closed` | `NZX-TH-2025-01-23` | T1 | NZX event date printed verbatim |
| 2026-01-02 | closed | `Day after New Year's Day` — `Friday, 2 January 2026` — `Closed` | `NZX-TH-2025-01-23` | T1 | NZX event date printed verbatim |
| 2026-02-06 | closed | `Waitangi Day` — `Friday, 6 February 2026` — `Closed` | `NZX-TH-2026-02-03` | T1 | NZX event date printed verbatim |
| 2026-04-03 | closed | `Good Friday` — `Friday, 3 April 2026` — `Closed` | `NZX-TH-2026-02-03` | T1 | NZX event date printed verbatim |
| 2026-04-06 | closed | `Easter Monday` — `Monday, 6 April 2026` — `Closed` | `NZX-TH-2026-02-03` | T1 | NZX event date printed verbatim |
| 2026-04-27 | closed | `ANZAC Day` — `Monday, 27 April 2026` — `Closed` | `NZX-TH-2026-02-03` | T1 | the sheet mondayises the Saturday to its own printed Monday date |
| 2026-06-01 | closed | `King's Birthday` — `Monday, 1 June 2026` — `Closed` | `NZX-TH-2026-02-03` | T1 | NZX event date printed verbatim |
| 2026-07-10 | closed | `Matariki` — `Friday, 10 July 2026` — `Closed` | `NZX-TH-2026-02-03` | T1 | NZX event date printed verbatim |
| 2026-10-26 | closed | `Labour Day` — `Monday, 26 October 2026` — `Closed` | `NZX-TH-2026-02-03` | T1 | NZX event date printed verbatim |
| 2026-12-24 | replacement blocks | `Business Day Prior to Christmas Day` — `Thursday, 24 December 2026` — `Abbreviated Trading*` | `NZX-TH-2026-02-03` | T1 | same reading as 2025-12-24 |
| 2026-12-25 | closed | `Christmas Day` — `Friday, 25 December 2026` — `Closed` | `NZX-TH-2026-02-03` | T1 | NZX event date printed verbatim |
| 2026-12-28 | closed | `Boxing Day` — `Monday, 28 December 2026` — `Closed` | `NZX-TH-2026-02-03` | T1 | the sheet mondayises the Saturday to its own printed Monday date |
| 2026-12-31 | replacement blocks | `Business Day Prior to New Year's Day` — `Thursday, 31 December 2026` — `Abbreviated Trading*` | `NZX-TH-2026-02-03` | T1 | same reading as 2025-12-24 |

### 2027

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2027-01-01 | closed | `New Year's Day` — `Friday, 1 January 2027` — `Closed` | `NZX-TH-2026-02-03` | T1 | NZX event date printed verbatim |
| 2027-01-04 | closed | `Day after New Year's Day` — `Monday, 4 January 2027` — `Closed` | `NZX-TH-2026-02-03` | T1 | NZX event date printed verbatim; the operator's published horizon |

**Interpretive step — the abbreviated-trading days are replacement days, not
scalar early closes.** The operator's own abbreviated column keeps a tradeable
closing auction after the shortened Normal Trading window: Pre-Close runs to
the abbreviated close and the closing uncross randomises within 30 seconds
either side of it, exactly as it does around 5:00pm on a full day (the same
anatomy-of-a-trading-day statement the normal-week profile cites for the ±30
second envelope). An `EarlyClose` clip cannot state that day: clipped at the
Normal-Trading end it deletes the auction prints (an executable window the
operator keeps open), and clipped at the uncross envelope's end it drags the
order-entry-only Pre-Close queue inside `is_open`, which the charter's
order-entry rule forbids. Each abbreviated day therefore ships as a
replacement block set restating the operator's grid block for block — the
`extended` Pre-open (tradeable: off-market reports print), the `regular`
Normal Trading, the `order_entry` Pre-Close sliced at the earliest uncross
edge, and the `extended` closing uncross envelope — with Enquiry and Adjust
excluded exactly as on a full day. The block instants are the sheets' own
phase boundaries; only the ±30 second uncross envelope is the operator's
randomisation statement already carried by the normal-week profile.

**No 2027 rows past the horizon.** Waitangi Day 2027 (6 February) and every
later 2027 date are outside the operator's published table, so the window
ends at 2027-01-04 and the inventory records the forward gap. Dates inside
the windows with no row are audited normal: each operator page names every
holiday it observes in its span, and every printed date ships. Re-checked
2026-09-29 UTC: the live trading-hours page was read again and answered
byte-identical bytes (same sha256 `92071cd2…` as `NZX-TH-LIVE-2026-09-28`),
so the operator's table still ends at 2027-01-04, and web search finds no 2027
holiday-arrangement notice on the operator's channels. **Closing condition:**
the operator republishing its trading-hours table with later dates, at which
point the window extends to the table's own end. Re-checked monthly per
LAW-WATCH.

### Documents

| Document | Window | Replay or service URL | Capture or retrieval, UTC | Tier | sha256 |
|---|---|---|---|---|---|
| `NZX-KD-2010-01-05` | 2010-01-01 .. 2010-12-31 | <https://web.archive.org/web/20100105003154id_/https://www.nzx.com/markets/key-dates/trading-hours> | Wayback `id_` replay of capture `20100105003154`, retrieved 2026-09-29 03:18:58 UTC | T1 | `c01569901faa64d89de1ecb5e4f1ab319d3ad7ee13735389989e8813e5da0294` |
| `NZX-KD-2009-12-04` | no rows keyed (the same page's pre-floor state; Normal-week corroboration) | <https://web.archive.org/web/20091204184203id_/http://www.nzx.com/markets/key-dates/trading-hours> | Wayback `id_` replay of capture `20091204184203`, retrieved 2026-09-30 04:20 UTC | T1 | `f5a8c3f622dbcf6adf8c48f688988e8392ef0542d8a56ac54e29d63f5b033b54` |
| `NZX-KD-2010-12-29` | 2011-01-01 .. 2011-12-30 | <https://web.archive.org/web/20101229214809id_/https://www.nzx.com/markets/key-dates/trading-hours> | Wayback `id_` replay of capture `20101229214809`, retrieved 2026-09-29 03:18:59 UTC | T1 | `14aa8ba0dbc6d6b044181964302a7c54fb9457da8822766a98c8b190c3c58a23` |
| `NZX-SX-2011-12-19` | 2011-12-23 .. 2012-10-22 | <https://web.archive.org/web/20111219140445id_/https://www.nzx.com/markets/NZSX/trading_hours> | Wayback `id_` replay of capture `20111219140445`, retrieved 2026-09-29 03:28:46 UTC | T1 | `7085a9a8c7a0addae300a56e6c6b21b6c57731505c1d4052b6132a0819108872` |
| `NZX-SX-2012-05-04` | 2012-06-04 .. 2013-04-25 | <https://web.archive.org/web/20120504064253id_/https://www.nzx.com/markets/NZSX/trading_hours> | Wayback `id_` replay of capture `20120504064253`, retrieved 2026-09-29 03:27:01 UTC | T1 | `464bee01a8f7b760e5292cfaa3d4efbd17018d3c2ec96eb0e0142b706612d2ce` |
| `NZX-SX-2013-01-16` | 2013-02-06 .. 2014-01-02 | <https://web.archive.org/web/20130116204734id_/https://www.nzx.com/markets/NZSX/trading_hours> | Wayback `id_` replay of capture `20130116204734`, retrieved 2026-09-29 03:27:03 UTC | T1 | `092c07789047507b8c9147feb60d04b5c33b3f87ca9be5b4e841104be4808948` |
| `NZX-SX-2013-05-16` | 2013-06-03 .. 2014-04-25 | <https://web.archive.org/web/20130516103440id_/https://www.nzx.com/markets/NZSX/trading_hours> | Wayback `id_` replay of capture `20130516103440`, retrieved 2026-09-29 03:28:48 UTC | T1 | `d16c9e2e6fc34df8b3a54b7a3587f5b535c14a2b47aa6843dc3796857bfb4241` |
| `NZX-SX-2014-01-27` | 2014-02-06 .. 2015-01-02 | <https://web.archive.org/web/20140127101752id_/https://www.nzx.com/markets/NZSX/trading_hours> | Wayback `id_` replay of capture `20140127101752`, retrieved 2026-09-29 03:27:06 UTC | T1 | `88dc01a700388b64149a68274542e82f9af308c55047b6c6cd7ea26b339743aa` |
| `NZX-SX-2014-10-20` | 2014-10-27 .. 2015-06-01 | <https://web.archive.org/web/20141020180412id_/https://www.nzx.com/markets/NZSX/trading_hours> | Wayback `id_` replay of capture `20141020180412`, retrieved 2026-09-29 03:28:50 UTC | T1 | `e8d409f45da2a2d48c640547ea3249dc73ef43d8b27a20b609793f56087c52d5` |
| `NZX-SX-2015-01-13` | 2015-02-06 .. 2016-01-04 | <https://web.archive.org/web/20150113212520id_/https://www.nzx.com/markets/NZSX/trading_hours> | Wayback `id_` replay of capture `20150113212520`, retrieved 2026-09-29 03:27:08 UTC | T1 | `8ea588ca618faad2f742ad771bd2a0428469568b537bf26fbe67c04cf0dd1bc6` |
| `NZX-SX-2015-04-27` | 2015-06-01 .. 2016-04-25 | <https://web.archive.org/web/20150427144953id_/https://www.nzx.com/markets/NZSX/trading_hours> | Wayback `id_` replay of capture `20150427144953`, retrieved 2026-09-29 03:28:52 UTC | T1 | `293803e0f1a918423c4bad2d8cfbb3d4b8858d0d32d42c6701814168292c4411` |
| `NZX-DD-2015-11-20` | 2016-06-06 .. 2016-10-24 (the 2015/2016 season list: the two gap rows key from it; its other dates are covered by the Main Board artifacts that already keyed them) | <https://web.archive.org/web/20170520051811id_/http://www.nzxfutures.com:80/system/downloads/15/Market_Holidays_Memo.pdf?1464052847> | Wayback `id_` replay of capture `20170520051811` of the operator's own derivatives-site download, retrieved 2026-10-03T02:18:39 UTC; the memo's own date is 20 November 2015 | T1 | `032980ab3fe067d6a20ea2ca652555e120eb9d9b449a1070af9710f6c8913930` |
| `NZX-DD-2015-01-16` | no rows keyed (the series' 2015 edition; its ANZAC Observed 27 April, Queen's Birthday 1 June and Labour Day 26 October rows corroborate the market-scope paragraph) | <https://web.archive.org/web/20160208055659id_/http://nzxfutures.com/system/downloads/15/NZX%20Derivatives%20Market%20Holidays.pdf?1421371202> | Wayback `id_` replay of capture `20160208055659` of the same download slot, retrieved 2026-10-03T02:18:37 UTC; the memo's own date is 16 January 2015 | T1 | `3b8f184b71e36dbc610d88a750781fe7f9c71a83cc1ffe8a46cbc6f79843a0da` |
| `NZX-DX-2013-09-17` | no rows keyed (the series' 2013/2014 edition; its Queen's Birthday 2 June and Labour Day 27 October 2014 rows corroborate the market-scope paragraph) | <https://web.archive.org/web/20140331113945id_/http://www.nzxfutures.com/system/downloads/15/NZX%20Derivatives%20Market%20Holidays.pdf?1387250622> | Wayback `id_` replay of capture `20140331113945` of the same download slot, retrieved 2026-10-03T02:18:38 UTC; the memo's own date is 17 September 2013 | T1 | `b3289a28c96368242a7d5a96db013a717fad57ad27ed0766c979df406c1bfd1a` |
| `NZX-MEMO-2016-2017` | 2016-12-23 .. 2017-10-23 | <https://www.nzx.com/announcements/294559> | retrieved live 2026-10-02 UTC (full HTML saved in the store's `holidays/raw/equities/nzx/evidence-thread/`; the digest is the saved page's) | T1 | `b29fc81921bc93e50a7c7813d177759adfdcad290a62bfa0f30fa81f94a085f5` |
| `NZX-SX-2017-06-23` | 2017-10-23 .. 2018-06-04 | <https://web.archive.org/web/20170623165321id_/https://www.nzx.com/markets/NZSX/trading_hours> | Wayback `id_` replay of capture `20170623165321`, retrieved 2026-09-29 03:27:10 UTC | T1 | `aff08bd12590dea5f6035b51625dd1c3a3792dc7ae5359cdc1acd9835bb05fa1` |
| `NZX-DX-2017-07-18` | 2017-04-14 .. 2018-04-02 | <https://web.archive.org/web/20170718122509id_/https://nzx.com/Derivatives/trading_hours> | Wayback `id_` replay of capture `20170718122509`, retrieved 2026-09-30 02:25:08 UTC | T1 | `03f91bbe1706d7b2a3f050a134725891027f48a34ccdd64f239ada7ebb54580c` |
| `NZX-MEMO-2017-2018` | no rows keyed (corroborates the series and the December-2017 dates `NZX-SX-2017-06-23` keys) | <https://www.nzx.com/announcements/312156> | retrieved live 2026-10-02 UTC (full HTML saved beside the 2016/2017 memo in the store) | T1 | `4ad717b9da045e00ab3206b0804160ccebfe72f1ae852f4b5ac2731ac14d4659` |
| `NZX-TH-2018-08-24` | 2018-01-01 .. 2019-01-02 | <https://web.archive.org/web/20180824125304id_/https://www.nzx.com/investing/nzx-trading-hours> | Wayback `id_` replay of capture `20180824125304`, retrieved 2026-09-29 03:22:36 UTC | T1 | `45a19b7159f1d07a831d740d0f95da0bf84e19bb200b5585537e2c3822b90e94` |
| `NZX-TH-2019-03-25` | 2019-01-01 .. 2019-12-31 | <https://web.archive.org/web/20190325182628id_/https://www.nzx.com/investing/nzx-trading-hours> | Wayback `id_` replay of capture `20190325182628`, retrieved 2026-09-29 03:22:37 UTC | T1 | `c697dea53d89bfbd3be04aef9faf9366ed66720aada58a6eea05dec9ceb48634` |
| `NZX-HB-2020-06-08` | 2020-01-01 .. 2020-12-28 | <https://web.archive.org/web/20200608000152id_/https://www.nzx.com/services/nzx-trading/hours-boards> | Wayback `id_` replay of capture `20200608000152`, retrieved 2026-09-29 03:31:50 UTC | T1 | `d09833adaaa0241f13afb27cd9708324cc06701992f311e911f8e7f07f34c5f7` |
| `NZX-HB-2021-01-12` | 2020-12-24 .. 2022-01-04 | <https://web.archive.org/web/20210112222610id_/https://www.nzx.com/services/nzx-trading/hours-boards> | Wayback `id_` replay of capture `20210112222610`, retrieved 2026-09-29 03:33:35 UTC | T1 | `6feeadc3d903d2dc3968fb1ac6e3f7cac4c2d9fa435ff6fc312b32bd53e95706` |
| `NZX-HB-2022-02-01` | 2021-12-24 .. 2023-01-02 | <https://web.archive.org/web/20220201081954id_/https://www.nzx.com/services/nzx-trading/hours-boards> | Wayback `id_` replay of capture `20220201081954`, retrieved 2026-09-29 03:34:49 UTC | T1 | `cff34ef58e690fb695ce480429e39bf28145fbe7a8eca2dda46b948c40e3801c` |
| `NZX-HB-2022-11-15` | 2021-12-24 .. 2023-01-02 | <https://web.archive.org/web/20221115164810id_/https://www.nzx.com/services/nzx-trading/hours-boards> | Wayback `id_` replay of capture `20221115164810`, retrieved 2026-09-29 03:36:07 UTC | T1 | `02c27775b753fa68c3158e263df93af0ac86fbc50e071735f89ab7a18c297247` |
| `NZX-HB-2023-11-06` | 2022-12-23 .. 2024-01-02 | <https://web.archive.org/web/20231106024609id_/https://www.nzx.com/services/nzx-trading/hours-boards> | Wayback `id_` replay of capture `20231106024609`, retrieved 2026-09-29 03:35:41 UTC | T1 | `84e0a2b0cb78823a85ee0c520bc8b59cf3d75930df48ca491fd6401566641cb3` |
| `NZX-HB-2024-02-19` | 2024-01-01 .. 2025-01-02 | <https://web.archive.org/web/20240219104141id_/https://www.nzx.com/services/nzx-trading/hours-boards> | Wayback `id_` replay of capture `20240219104141`, retrieved 2026-09-29 03:36:09 UTC | T1 | `bc4ad05484098ec1a589e4a129681e46c7e75b572115edd0c49ae10a3d9c1608` |
| `NZX-TH-2024-12-16` | 2025-01-01 .. 2026-01-02 | <https://web.archive.org/web/20241216221746id_/https://www.nzx.com/investing/nzx-trading-hours> | Wayback `id_` replay of capture `20241216221746`, retrieved 2026-09-28 01:20 UTC | T1 | `27ead98734cede1f2b0e21d3132b20b5a4b6ea5ebd218b4fb4d17fe9394aa675` |
| `NZX-TH-2025-01-23` | 2025-01-01 .. 2026-01-02 | <https://web.archive.org/web/20250123031559id_/https://www.nzx.com/investing/nzx-trading-hours> | Wayback `id_` replay of capture `20250123031559`, retrieved 2026-09-28 01:19 UTC | T1 | `982f231b435dddba2473692163a2bcb60f47f89c1005a41d1745eb520c6e715a` |
| `NZX-TH-2026-02-03` | 2025-01-01 .. 2027-01-04 | <https://web.archive.org/web/20260203200136id_/https://new.nzx.com/investing/nzx-trading-hours> | Wayback `id_` replay of capture `20260203200136`, retrieved 2026-09-28 01:19 UTC | T1 | `0aa0508dadd8c30aeb914bd09f6e7db96afe84533a18350e4145f7b5b8dd6215` |
| `NZX-TH-LIVE-2026-09-28` | 2025-01-01 .. 2027-01-04 | <https://www.nzx.com/learning/help-reference/trading-hours> | retrieved 2026-09-28 01:10 UTC | T1 | `92071cd2c1186012fe977fc59af5f4b27de50a435a11b3298a078aa715391146` |

`NZX-TH-LIVE-2026-09-28` is no row's document: it is the horizon evidence, and
its table is cell-for-cell identical to `NZX-TH-2026-02-03`'s over the rows
they share. `NZX-MEMO-2017-2018` likewise keys no row: it is the series' next
edition, retrieved live beside the 2016/2017 memorandum on 2026-10-02 UTC, and
its printed dates agree with the `NZX-SX-2017-06-23` and `NZX-TH-2018-08-24`
rows on every shared date. The store's `holidays/raw/equities/nzx/2010-2024/` holds the
pre-2025 replays; `holidays/raw/equities/nzx/2025-2027/` holds the 2025-2027
replays (including the 2025-11-16 replay, `59c3798c…`, which still prints the
2025 table and corroborates the 2025/2026 boundary documents without keying
any row).

## Gaps and residual risks

- **the 2016 capture gap — closed 2026-10-03 UTC.** 2016-04-26..2016-12-22 is
  now inside the one audited window: the series' 2015/2016 edition
  (`NZX-DD-2015-11-20`, the operator's own derivatives-site holiday memo,
  Wayback replay) prints the span's two closures — Queen's Birthday
  2016-06-06 and Labour Day 2016-10-24 — and its complete season list audits
  the days between them normal, so [#209](https://github.com/SharurTrading/exchange-hours-rs/issues/209) closes with the data. The 2026-10-02 UTC
  negatives stand as the search record (the announcement-system edition is
  purged and never captured), and the 2026-10-03 UTC pass added the settled
  negatives named in the coverage paragraph — none of which produced a
  conflicting artifact. Residual risk: the keyed rows rest on the
  market-scope step recorded above (a dairy-addressed memo whose every date
  shared with a Main Board artifact agrees with it); a hypothetical
  Main Board-only arrangement inside the span that no operator artifact of
  the era printed would be invisible to every witness this file holds, which
  is the same residual every rolling-list window here carries.
- **the 2013 abbreviated-grid conflict** — the 2013-05-16 capture's trading-hours
  grid still shows the older 15:45 abbreviated column while the 2013-01-16 page
  before it and every capture from 2014-01-27 after it print 12:45; the two 2013
  abbreviated days hold the 12:45 close (the narrowest sourced value) and the
  disputed 12:45pm-4:30pm remainder ships as no session. **Closing condition:** a
  dated operator statement for the 15:45-to-12:45 grid changeover, or a capture
  whose grid and holiday table agree on the December 2013 days, keys the span's
  true grid.
- **horizon sourced from 2010-01-05** — the pre-2020 baseline is the operator's
  own key-dates trading-hours page print of 2010-01-05, corroborated at every
  later capture and ended by the dated 2020-04-06 revision (see the Normal week
  section). The carried region below it — 2010-01-01..2010-01-04 — contains no
  trade date. The 2020-06-08 capture still printing the
  9:00am Pre-open beside the announcement's 8:30am move remains the era's
  open question; the holiday table holds the 9:00am Pre-open on the two 2020
  abbreviated days (the narrowest sourced value) and the conflict is recorded
  in the coverage section.
- The closing uncross is randomised within 30 seconds either side of 17:00, so
  the tradeable window runs to 17:00:30; stopping at 17:00 would drop the half
  of the randomisation in which the official closing print most often occurs.
  On the pre-2025 abbreviated days the envelopes rest on the crate's carried
  ±30-second convention, not on a statement the era's sheets print (disclosed
  in the coverage section).
- The only order-entry-only phase is Pre-Close 16:45–16:59:30, which neither
  matches nor accepts reports. The slice stops 30 seconds short of the nominal
  17:00 boundary so the randomized uncross stays inside the tradeable window.
- Enquiry and Adjust do not accept automatically matched orders and are
  excluded from the envelope (AGENTS.md, *Cash-equity venue envelope*).

## Module narrative (moved from src/calendar/schedules/equities/apac/nzx.rs on 2026-10-02 UTC)

Both auction
uncrosses are randomized ±30 seconds around the nominal boundary; this
deterministic venue profile uses 10:00 and 17:00. Enquiry and Adjust do not
accept automatically matched orders and are excluded.
Sources:
https://www.nzx.com/learning/help-reference/trading-hours
https://www.nzx.com/learning/issuer-participant-resources/nzx-trading/anatomy-of-a-trading-day

---

These windows are shared by both revisions — only the pre-open
start moved in 2020, and that start now sits in the order-entry slice.
The closing uncross is randomised within 30 seconds EITHER SIDE of 17:00, so
the tradeable window runs to 17:00:30; stopping at 17:00 dropped the half of
the randomisation in which the official closing print most often occurs.
