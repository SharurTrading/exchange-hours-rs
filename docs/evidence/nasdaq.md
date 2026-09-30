<!-- SPDX-License-Identifier: MIT-0 -->

# `nasdaq` — evidence

- **Kind:** `Exchange`
- **Owner module:** [`equities.rs`](../../src/calendar/schedules/equities/us/equities.rs)<br>[`history.rs`](../../src/calendar/schedules/equities/us/history.rs)
- **Source sets:** [`US-NASDAQ-EQUITIES`](../schedules/sources.md#us-nasdaq-equities)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

Nasdaq Stock Market normal week, sourced to the 2010-01-01 floor; date-aware lookups retain the sourced 2013 07:00→04:00 early-open change, and the 07:00 grid below it is the operator's own floor-era statement record (see the Normal week section). The announced Night Session is monitored but unencoded pending Equity Data Plan readiness and a later Nasdaq filing, so current and future snapshots remain 04:00–20:00. **Systems in scope (2026-09-02):** the Nasdaq equities matching system (System Hours 04:00–20:00) is the envelope; The Nasdaq Options Market is `nasdaq_nom` and the FINRA/Nasdaq TRF is `finra_trf_carteret`. ACT, Weblink ACT 2.0, ACES (08:00–18:30, interior), the Nasdaq Testing Facility and index dissemination are excluded classes; Nasdaq Fixed Income and Nasdaq Futures belong to neither this SRO nor cash equity. No discrepancy.

## Revision rows

- 2013-03-18 — T1 — Nasdaq Equity Trader Alert 2013-21 — pre-market open moves from 07:00 to 04:00 ET; the operator's own SR-NASDAQ-2013-033 filing (34-69151) marks Rule 4120(b)(4)(B)'s in-force "[7:00] 4:00 a.m." text and states "NASDAQ will implement this proposal on March 18, 2013", so the effective day is stated twice at T1.

## Normal week

**The pre-2013 grid is the operator's own published history, at the floor.**
The 07:00–20:00 envelope below the sourced 2013-03-18 cutover rests on the
operator's own statements in its SEC rule filings and its own pre-2013
Trading Hours page:

- **Floor envelope (in force January 2010).** SR-NASDAQ-2010-008 (Release
  34-61521; notice at 75 FR 8156, 2010-02-23; filed 2010-01-15,
  `NQ-NW-2010-008`) quotes the rulebook text then in force in the operator's
  own electronic manual, and the in-force IM-5250-1 text states "Nasdaq market
  hours (7 a.m. to 8 p.m. ET)" four times, with the filing's own purpose
  statement repeating it once — the operator naming its own market-hours
  envelope at the floor. The parenthetical is unchanged text in the marked
  proposal; the amendment moves only the issuer pre-notification mechanics
  around it.
- **The interior grid, restated (2012–2013).** Four Nasdaq filings print the
  operator's own footnote citing Nasdaq Rule 4120(b)(4) "describing the three
  trading sessions on the Exchange": `(1) Pre-Market Session from 7 a.m. to
  9:30 a.m.; (2) Regular Market Session from 9:30 a.m. to 4 p.m. or 4:15 p.m.;
  and (3) Post-Market Session from 4 p.m. or 4:15 p.m. to 8 p.m.`
  (`NQ-NW-2012-1285`, 2012-01-24; `NQ-NW-2012-5367`, 2012-03-06;
  `NQ-NW-2012-21815`, 2012-09-05; `NQ-NW-2012-26253`, 2012-10-25), and the
  2013-02-28 approval order restates it again three weeks before the cutover
  (`NQ-NW-2013-04614`, with "7:00 a.m." spelled out). The operator's own
  Trading Hours page states the same grid in session language across all five
  of its surviving captures: "The NASDAQ Stock Market Trading Sessions
  (Eastern Time) Pre-Market Trading Hours from 7:00 a.m. to 9:30 a.m. Market
  Hours from 9:30 a.m. to 4:00 p.m. After-Market Hours from 4:00 p.m. to 8:00
  p.m. Quote and order-entry from 7:00 a.m. to 8:00 p.m."
  (`NQ-NW-2012-PAGE-20120325` through `-20121005`, captured 2012-03-25,
  2012-05-06, 2012-09-22, 2012-09-24 and 2012-10-05; no earlier capture
  survives — the page witnesses from March 2012, and the floor statement is
  the 2010-008 quoted rulebook text).
- **The outgoing state, immediately before the cutover.** SR-NASDAQ-2013-033
  ("the 4 a.m. Filing"; filed 2013-03-05, Release 34-69151, notice at 78 FR
  17464, `NQ-NW-2013-033-NOTICE` and the operator's own filing PDF
  `NQ-NW-2013-033-FILING`) states, in the Exchange's own Background: "NASDAQ's
  equities trading day is divided into three sessions: (1) the pre-market
  session which runs from 7:00 a.m. to 9:29:59 a.m.; (2) the regular session
  which runs from 9:30 a.m. to 4:00 p.m.; and (3) the post-market session
  which runs from 4:00:00:01 p.m. to 8:00 p.m." — second-precision boundaries
  in session language. Its Exhibit 5 prints the marked rule text
  "(B) Pre-Market Session means the trading session that begins at **[7:00]**
  4:00 a.m. and continues until 9:30 a.m." — the bracketed 7:00 is the
  in-force rulebook value the amendment deletes — and states "NASDAQ will
  implement this proposal on March 18, 2013", the same day the 2013-21 alert
  keys.

The ledger horizon is therefore the January-2010 floor: nothing is carried.

**Residual.** The envelope is stated at the floor (2010-01-15 in-force
rulebook text); the interior 9:30/16:00 boundaries are first restated in a
Federal Register document on 2012-01-24 (`NQ-NW-2012-1285` — the
4120(b)(4)-footnote practice begins there; the exact-phrase sweep over
"three trading sessions on the Exchange" returns no 2010–2011 document), and
no dated operator artifact restating the interior between the floor and that
date was found. The envelope's unchanged ends across that sub-span, the
operator's unchanged marked rule text at the 2013 filing, and the swept
absence of any session-hour change filing between 2010-01-01 and the keyed
2013-03-18 cutover carry the claim, and this record states exactly that
rather than a capture per year.

**The "or 4:15 p.m." alternative is per-series ETP designation, not the
venue grid.** Rule 4120(b)(4)(C)/(D) and Rule 4420(i)(7) state the regular
session runs "between 9:30 a.m. and either 4:00 p.m. or 4:15 p.m. for each
series of Portfolio Depository Receipts, as specified by Nasdaq" — a
per-series designation for ETP classes, which is product-level data outside
this identity's venue envelope (and out of the normal-week model's scope).
Every venue-level statement in the set — the 2013-03-21 Background twice, the
Trading Hours page, and the keyed 2013-03-18 row's alert — states the 4:00
p.m. close, so the crate's 09:30–16:00 regular session is the undisputed
intersection. The printed "4:00:00:01 p.m." post-market start and "9:29:59
a.m." pre-market end are the operator's own statement of the end-exclusive
boundaries the crate encodes; the 16:00:00 instant is the regular session's
own end-exclusive close, exactly as the sourced 2013-03-18 row already
encodes. Nasdaq's "Quote and order-entry from 7:00 a.m. to 8:00 p.m. Quotes
are open and firm from 7:00 a.m. to 8:00 p.m." (Trading Hours page) states
the same envelope the extended rules carry.

**The 2012–2013 FR footnotes and orders are the Exchange's own statements.**
Each Items I/II text is "prepared by the Exchange" inside its 19b-4 filing
and published verbatim by the SEC in the Federal Register, the same class the
NYSE Rule 51 statements are (LAW-PRIMARY-SOURCES: T1, the operator's own
statement).

## Holidays

**Coverage:** 2010-01-01..2026-12-31 (inclusive trade dates). Tier: T1 throughout.

**Service tier.** The identity is served: SharurPlatform routes Nasdaq cash equities (LAW-SERVICE-TIERS admission by consumer reach). The cadence is monthly per LAW-WATCH — holiday-bearing and the operator republishes the calendar yearly. The window ends at 2026-12-31 because the operator has published no 2027 holiday schedule: both live channels (the `nasdaqtrader.com` calendar page and the `nasdaq.com` holiday-schedule page with its PDF twin) carried only 2026 when read on 2026-09-27 UTC, so nothing past 2026 is claimed and nothing is withheld. Re-checked 2026-09-29 UTC: both channels were read again and their raw bytes carry zero 2027 dates (artifacts under `holidays/raw/nyse-nasdaq/forward-2027/nasdaq/` in the research store); **closing condition:** the operator's 2027 schedule page, at which point the window extends.

**Corpus and retrieval story (2026-09-27 UTC).** Every artifact is the operator's own page or alert, saved under `holidays/raw/nyse-nasdaq/nasdaq/` in the research store with its sha256 (`manifest.json` beside each directory). The cash-equity schedule is the operator's "U.S. Equity and Options Markets Holiday Schedule" page on `nasdaqtrader.com` (the operator's own trader channel): captured roughly monthly by the Wayback Machine 2010-2025, one replay per schedule year is saved, plus the live 2026 page and the live `nasdaq.com` holiday-schedule page and its PDF twin (which restate the 2026 grid with "1:00 p.m. ET" times). A December-of-year-Y capture normally carries the year Y+1 schedule, so row citations follow the schedule an artifact governs, and the Documents table records each artifact's stated window. The 2010 and 2011 pages print the cash market's early close as `Early Market Close* TBA` with the footnote "NASDAQ will continue to send alerts to notify customers of days when the Market will close early. Please refer to those alerts for full information, including system operating times"; from the 2012 page on, the cash market's own times are printed (`Early Close - U.S. 1:00 p.m.`).

**Early-close convention (set alongside `nyse.md` for the equities programme).** The sheet's own wording decides, per date. Where the sheet prints the cash market's early close with a time — every recovered date from 2012-07-03 through 2026-12-24 — it prints exactly one fact about this identity ("Early Close - U.S. 1:00 p.m.") and no continuing session, so the row is `EarlyClose{13:00}` and the whole envelope clips. The sheet's standing footnote defers "full information, including system operating times" to per-date alerts; no such alert was recoverable from the operator's own channels for any historical early-close date (attempts recorded in the store's `README.md`), so no row invents post-13:00 topology — a `ReplacementBlocks` row would fabricate instants no operator document prints. The sheet's option-product columns (1:00/1:15 p.m. for equity, index and currency options) are a different market and never key a row here; the sheet itself keeps them in separate columns.

**Unsourced dates (four).** LAW-NO-FABRICATED-DATES keeps a date out of the table rather than giving it an invented arrangement, and `Unsourced` keeps the window contiguous while refusing to call the date normal:

- **2010-11-26, 2011-11-25.** The operator's sheet states the early-market-close arrangement for the Nasdaq Stock Market but prints `TBA` instead of a time, deferring to alerts; the alerts are not archived (two retrieval attempts recorded). No scalar row is representable without the time, so the dates ship `Unsourced`.
- **2012-10-30.** Equity Trader Alert `ETA2012-44` states the 2012-10-29 closure unconditionally — "NASDAQ OMX will close all U.S. equity and derivatives exchanges ... on Monday, October 29th, due to Hurricane Sandy" — but for the Tuesday says only "it is likely that the markets will be closed ... will confirm this decision at a future time". The confirming alert (`ETA2012-45` era) is not recoverable from the archived channels (four attempts recorded), and a conditional clause never keys a row, so the date ships `Unsourced` while 2012-10-29 keys on the alert.
- **2025-01-09.** The National Day of Mourning for President Carter. Six retrieval attempts across the operator's own channels (trader alert ids, the `nasdaqtrader.com` home and RSS pages of January 2025, the `nasdaq.com` press center, the operator's own 2025 holiday-schedule page captures, and the 2025 trading-calendar PDF text layer) recovered no artifact that states the closure for this market — the operator's own 2025 sheet omits the day entirely. The date was a closure, and silence in the audited window would claim "audited normal", so it ships `Unsourced` with this closing condition: an operator artifact stating the 2025-01-09 closure for the Nasdaq Stock Market, which converts the row to `Closed` at T1.

**Gaps and residual risks.** The four `Unsourced` dates above are withheld from the audited window and any other date in it is audited from the operator's own sheets. The normal-week Night Session horizon note is unchanged and lives in the sections below.

### 2010

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2010-01-01 | closed | `Closed` | `NQ-HOL-2010` | T1 | event date 2010-01-01; the sheet prints no session for it |
| 2010-01-18 | closed | `Closed` | `NQ-HOL-2010` | T1 | event date 2010-01-18; the sheet prints no session for it |
| 2010-02-15 | closed | `Closed` | `NQ-HOL-2010` | T1 | event date 2010-02-15; the sheet prints no session for it |
| 2010-04-02 | closed | `Closed` | `NQ-HOL-2010` | T1 | event date 2010-04-02; the sheet prints no session for it |
| 2010-05-31 | closed | `Closed` | `NQ-HOL-2010` | T1 | event date 2010-05-31; the sheet prints no session for it |
| 2010-07-05 | closed | `Closed` | `NQ-HOL-2010` | T1 | event date 2010-07-05; the sheet prints no session for it |
| 2010-09-06 | closed | `Closed` | `NQ-HOL-2010` | T1 | event date 2010-09-06; the sheet prints no session for it |
| 2010-11-25 | closed | `Closed` | `NQ-HOL-2010` | T1 | event date 2010-11-25; the sheet prints no session for it |
| 2010-11-26 | unsourced | `TBA` / not printed | `NQ-HOL-2010` | T1 | event date 2010-11-26; operator states the arrangement but no recovered artifact states its time |
| 2010-12-24 | closed | `Closed` | `NQ-HOL-2010` | T1 | event date 2010-12-24; the sheet prints no session for it |

### 2011

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2011-01-17 | closed | `Closed` | `NQ-HOL-2011` | T1 | event date 2011-01-17; the sheet prints no session for it |
| 2011-02-21 | closed | `Closed` | `NQ-HOL-2011` | T1 | event date 2011-02-21; the sheet prints no session for it |
| 2011-04-22 | closed | `Closed` | `NQ-HOL-2011` | T1 | event date 2011-04-22; the sheet prints no session for it |
| 2011-05-30 | closed | `Closed` | `NQ-HOL-2011` | T1 | event date 2011-05-30; the sheet prints no session for it |
| 2011-07-04 | closed | `Closed` | `NQ-HOL-2011` | T1 | event date 2011-07-04; the sheet prints no session for it |
| 2011-09-05 | closed | `Closed` | `NQ-HOL-2011` | T1 | event date 2011-09-05; the sheet prints no session for it |
| 2011-11-24 | closed | `Closed` | `NQ-HOL-2011` | T1 | event date 2011-11-24; the sheet prints no session for it |
| 2011-11-25 | unsourced | `TBA` / not printed | `NQ-HOL-2011` | T1 | event date 2011-11-25; operator states the arrangement but no recovered artifact states its time |
| 2011-12-26 | closed | `Closed` | `NQ-HOL-2011` | T1 | event date 2011-12-26; the sheet prints no session for it |

### 2012

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2012-01-02 | closed | `Closed` | `NQ-HOL-2011` | T1 | event date 2012-01-02; the sheet prints no session for it |
| 2012-01-16 | closed | `Closed` | `NQ-HOL-2011` | T1 | event date 2012-01-16; the sheet prints no session for it |
| 2012-02-20 | closed | `Closed` | `NQ-HOL-2011` | T1 | event date 2012-02-20; the sheet prints no session for it |
| 2012-04-06 | closed | `Closed` | `NQ-HOL-2011` | T1 | event date 2012-04-06; the sheet prints no session for it |
| 2012-05-28 | closed | `Closed` | `NQ-HOL-2011` | T1 | event date 2012-05-28; the sheet prints no session for it |
| 2012-07-03 | early close | `Early Close - U.S. 1:00 p.m.` | `NQ-HOL-2012` | T1 | event date 2012-07-03; the sheet prints the 1:00 p.m. close and nothing after it |
| 2012-07-04 | closed | `Closed` | `NQ-HOL-2012` | T1 | event date 2012-07-04; the sheet prints no session for it |
| 2012-09-03 | closed | `Closed` | `NQ-HOL-2012` | T1 | event date 2012-09-03; the sheet prints no session for it |
| 2012-10-29 | closed | `Closed` | `NQ-SANDY-2012` | T1 | event date 2012-10-29; the sheet prints no session for it |
| 2012-10-30 | unsourced | `TBA` / not printed | `NQ-HOL-2012` | T1 | event date 2012-10-30; operator states the arrangement but no recovered artifact states its time |
| 2012-11-22 | closed | `Closed` | `NQ-HOL-2012` | T1 | event date 2012-11-22; the sheet prints no session for it |
| 2012-11-23 | early close | `Early Close - U.S. 1:00 p.m.` | `NQ-HOL-2012` | T1 | event date 2012-11-23; the sheet prints the 1:00 p.m. close and nothing after it |
| 2012-12-24 | early close | `Early Close - U.S. 1:00 p.m.` | `NQ-HOL-2012` | T1 | event date 2012-12-24; the sheet prints the 1:00 p.m. close and nothing after it |
| 2012-12-25 | closed | `Closed` | `NQ-HOL-2012` | T1 | event date 2012-12-25; the sheet prints no session for it |

### 2013

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2013-01-01 | closed | `Closed` | `NQ-HOL-2013` | T1 | event date 2013-01-01; the sheet prints no session for it |
| 2013-01-21 | closed | `Closed` | `NQ-HOL-2013` | T1 | event date 2013-01-21; the sheet prints no session for it |
| 2013-02-18 | closed | `Closed` | `NQ-HOL-2013` | T1 | event date 2013-02-18; the sheet prints no session for it |
| 2013-03-29 | closed | `Closed` | `NQ-HOL-2013` | T1 | event date 2013-03-29; the sheet prints no session for it |
| 2013-05-27 | closed | `Closed` | `NQ-HOL-2013` | T1 | event date 2013-05-27; the sheet prints no session for it |
| 2013-07-03 | early close | `Early Close - U.S. 1:00 p.m.` | `NQ-HOL-2013` | T1 | event date 2013-07-03; the sheet prints the 1:00 p.m. close and nothing after it |
| 2013-07-04 | closed | `Closed` | `NQ-HOL-2013` | T1 | event date 2013-07-04; the sheet prints no session for it |
| 2013-09-02 | closed | `Closed` | `NQ-HOL-2013` | T1 | event date 2013-09-02; the sheet prints no session for it |
| 2013-11-28 | closed | `Closed` | `NQ-HOL-2013` | T1 | event date 2013-11-28; the sheet prints no session for it |
| 2013-11-29 | early close | `Early Close - U.S. 1:00 p.m.` | `NQ-HOL-2013` | T1 | event date 2013-11-29; the sheet prints the 1:00 p.m. close and nothing after it |
| 2013-12-24 | early close | `Early Close - U.S. 1:00 p.m.` | `NQ-HOL-2013` | T1 | event date 2013-12-24; the sheet prints the 1:00 p.m. close and nothing after it |
| 2013-12-25 | closed | `Closed` | `NQ-HOL-2013` | T1 | event date 2013-12-25; the sheet prints no session for it |

### 2014

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2014-01-01 | closed | `Closed` | `NQ-HOL-2014` | T1 | event date 2014-01-01; the sheet prints no session for it |
| 2014-01-20 | closed | `Closed` | `NQ-HOL-2014` | T1 | event date 2014-01-20; the sheet prints no session for it |
| 2014-02-17 | closed | `Closed` | `NQ-HOL-2014` | T1 | event date 2014-02-17; the sheet prints no session for it |
| 2014-04-18 | closed | `Closed` | `NQ-HOL-2014` | T1 | event date 2014-04-18; the sheet prints no session for it |
| 2014-05-26 | closed | `Closed` | `NQ-HOL-2014` | T1 | event date 2014-05-26; the sheet prints no session for it |
| 2014-07-03 | early close | `Early Close - U.S. 1:00 p.m.` | `NQ-HOL-2014` | T1 | event date 2014-07-03; the sheet prints the 1:00 p.m. close and nothing after it |
| 2014-07-04 | closed | `Closed` | `NQ-HOL-2014` | T1 | event date 2014-07-04; the sheet prints no session for it |
| 2014-09-01 | closed | `Closed` | `NQ-HOL-2014` | T1 | event date 2014-09-01; the sheet prints no session for it |
| 2014-11-27 | closed | `Closed` | `NQ-HOL-2014` | T1 | event date 2014-11-27; the sheet prints no session for it |
| 2014-11-28 | early close | `Early Close - U.S. 1:00 p.m.` | `NQ-HOL-2014` | T1 | event date 2014-11-28; the sheet prints the 1:00 p.m. close and nothing after it |
| 2014-12-24 | early close | `Early Close - U.S. 1:00 p.m.` | `NQ-HOL-2014` | T1 | event date 2014-12-24; the sheet prints the 1:00 p.m. close and nothing after it |
| 2014-12-25 | closed | `Closed` | `NQ-HOL-2014` | T1 | event date 2014-12-25; the sheet prints no session for it |

### 2015

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2015-01-01 | closed | `Closed` | `NQ-HOL-2015` | T1 | event date 2015-01-01; the sheet prints no session for it |
| 2015-01-19 | closed | `Closed` | `NQ-HOL-2015` | T1 | event date 2015-01-19; the sheet prints no session for it |
| 2015-02-16 | closed | `Closed` | `NQ-HOL-2015` | T1 | event date 2015-02-16; the sheet prints no session for it |
| 2015-04-03 | closed | `Closed` | `NQ-HOL-2015` | T1 | event date 2015-04-03; the sheet prints no session for it |
| 2015-05-25 | closed | `Closed` | `NQ-HOL-2015` | T1 | event date 2015-05-25; the sheet prints no session for it |
| 2015-07-03 | closed | `Closed` | `NQ-HOL-2015` | T1 | event date 2015-07-03; the sheet prints no session for it |
| 2015-09-07 | closed | `Closed` | `NQ-HOL-2015` | T1 | event date 2015-09-07; the sheet prints no session for it |
| 2015-11-26 | closed | `Closed` | `NQ-HOL-2015` | T1 | event date 2015-11-26; the sheet prints no session for it |
| 2015-11-27 | early close | `Early Close - U.S. 1:00 p.m.` | `NQ-HOL-2015` | T1 | event date 2015-11-27; the sheet prints the 1:00 p.m. close and nothing after it |
| 2015-12-24 | early close | `Early Close - U.S. 1:00 p.m.` | `NQ-HOL-2015` | T1 | event date 2015-12-24; the sheet prints the 1:00 p.m. close and nothing after it |
| 2015-12-25 | closed | `Closed` | `NQ-HOL-2015` | T1 | event date 2015-12-25; the sheet prints no session for it |

### 2016

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2016-01-01 | closed | `Closed` | `NQ-HOL-2015` | T1 | event date 2016-01-01; the sheet prints no session for it |
| 2016-01-18 | closed | `Closed` | `NQ-HOL-2015` | T1 | event date 2016-01-18; the sheet prints no session for it |
| 2016-02-15 | closed | `Closed` | `NQ-HOL-2015` | T1 | event date 2016-02-15; the sheet prints no session for it |
| 2016-03-25 | closed | `Closed` | `NQ-HOL-2015` | T1 | event date 2016-03-25; the sheet prints no session for it |
| 2016-05-30 | closed | `Closed` | `NQ-HOL-2015` | T1 | event date 2016-05-30; the sheet prints no session for it |
| 2016-07-04 | closed | `Closed` | `NQ-HOL-2015` | T1 | event date 2016-07-04; the sheet prints no session for it |
| 2016-09-05 | closed | `Closed` | `NQ-HOL-2015` | T1 | event date 2016-09-05; the sheet prints no session for it |
| 2016-11-24 | closed | `Closed` | `NQ-HOL-2015` | T1 | event date 2016-11-24; the sheet prints no session for it |
| 2016-11-25 | early close | `Early Close - U.S. 1:00 p.m.` | `NQ-HOL-2015` | T1 | event date 2016-11-25; the sheet prints the 1:00 p.m. close and nothing after it |
| 2016-12-26 | closed | `Closed` | `NQ-HOL-2015` | T1 | event date 2016-12-26; the sheet prints no session for it |

### 2017

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2017-01-02 | closed | `Closed` | `NQ-HOL-2017` | T1 | event date 2017-01-02; the sheet prints no session for it |
| 2017-01-16 | closed | `Closed` | `NQ-HOL-2017` | T1 | event date 2017-01-16; the sheet prints no session for it |
| 2017-02-20 | closed | `Closed` | `NQ-HOL-2017` | T1 | event date 2017-02-20; the sheet prints no session for it |
| 2017-04-14 | closed | `Closed` | `NQ-HOL-2017` | T1 | event date 2017-04-14; the sheet prints no session for it |
| 2017-05-29 | closed | `Closed` | `NQ-HOL-2017` | T1 | event date 2017-05-29; the sheet prints no session for it |
| 2017-07-03 | early close | `Early Close - U.S. 1:00 p.m.` | `NQ-HOL-2017` | T1 | event date 2017-07-03; the sheet prints the 1:00 p.m. close and nothing after it |
| 2017-07-04 | closed | `Closed` | `NQ-HOL-2017` | T1 | event date 2017-07-04; the sheet prints no session for it |
| 2017-09-04 | closed | `Closed` | `NQ-HOL-2017` | T1 | event date 2017-09-04; the sheet prints no session for it |
| 2017-11-23 | closed | `Closed` | `NQ-HOL-2017` | T1 | event date 2017-11-23; the sheet prints no session for it |
| 2017-11-24 | early close | `Early Close - U.S. 1:00 p.m.` | `NQ-HOL-2017` | T1 | event date 2017-11-24; the sheet prints the 1:00 p.m. close and nothing after it |
| 2017-12-25 | closed | `Closed` | `NQ-HOL-2017` | T1 | event date 2017-12-25; the sheet prints no session for it |

### 2018

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2018-01-01 | closed | `Closed` | `NQ-HOL-2018` | T1 | event date 2018-01-01; the sheet prints no session for it |
| 2018-01-15 | closed | `Closed` | `NQ-HOL-2018` | T1 | event date 2018-01-15; the sheet prints no session for it |
| 2018-02-19 | closed | `Closed` | `NQ-HOL-2018` | T1 | event date 2018-02-19; the sheet prints no session for it |
| 2018-03-30 | closed | `Closed` | `NQ-HOL-2018` | T1 | event date 2018-03-30; the sheet prints no session for it |
| 2018-05-28 | closed | `Closed` | `NQ-HOL-2018` | T1 | event date 2018-05-28; the sheet prints no session for it |
| 2018-07-03 | early close | `Early Close - U.S. 1:00 p.m.` | `NQ-HOL-2018` | T1 | event date 2018-07-03; the sheet prints the 1:00 p.m. close and nothing after it |
| 2018-07-04 | closed | `Closed` | `NQ-HOL-2018` | T1 | event date 2018-07-04; the sheet prints no session for it |
| 2018-09-03 | closed | `Closed` | `NQ-HOL-2018` | T1 | event date 2018-09-03; the sheet prints no session for it |
| 2018-11-22 | closed | `Closed` | `NQ-HOL-2018` | T1 | event date 2018-11-22; the sheet prints no session for it |
| 2018-11-23 | early close | `Early Close - U.S. 1:00 p.m.` | `NQ-HOL-2018` | T1 | event date 2018-11-23; the sheet prints the 1:00 p.m. close and nothing after it |
| 2018-12-24 | early close | `Early Close - U.S. 1:00 p.m.` | `NQ-HOL-2018` | T1 | event date 2018-12-24; the sheet prints the 1:00 p.m. close and nothing after it |
| 2018-12-25 | closed | `Closed` | `NQ-HOL-2018` | T1 | event date 2018-12-25; the sheet prints no session for it |

### 2019

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2019-01-01 | closed | `Closed` | `NQ-HOL-2019` | T1 | event date 2019-01-01; the sheet prints no session for it |
| 2019-01-21 | closed | `Closed` | `NQ-HOL-2019` | T1 | event date 2019-01-21; the sheet prints no session for it |
| 2019-02-18 | closed | `Closed` | `NQ-HOL-2019` | T1 | event date 2019-02-18; the sheet prints no session for it |
| 2019-04-19 | closed | `Closed` | `NQ-HOL-2019` | T1 | event date 2019-04-19; the sheet prints no session for it |
| 2019-05-27 | closed | `Closed` | `NQ-HOL-2019` | T1 | event date 2019-05-27; the sheet prints no session for it |
| 2019-07-03 | early close | `Early Close - U.S. 1:00 p.m.` | `NQ-HOL-2019` | T1 | event date 2019-07-03; the sheet prints the 1:00 p.m. close and nothing after it |
| 2019-07-04 | closed | `Closed` | `NQ-HOL-2019` | T1 | event date 2019-07-04; the sheet prints no session for it |
| 2019-09-02 | closed | `Closed` | `NQ-HOL-2019` | T1 | event date 2019-09-02; the sheet prints no session for it |
| 2019-11-28 | closed | `Closed` | `NQ-HOL-2019` | T1 | event date 2019-11-28; the sheet prints no session for it |
| 2019-11-29 | early close | `Early Close - U.S. 1:00 p.m.` | `NQ-HOL-2019` | T1 | event date 2019-11-29; the sheet prints the 1:00 p.m. close and nothing after it |
| 2019-12-24 | early close | `Early Close - U.S. 1:00 p.m.` | `NQ-HOL-2019` | T1 | event date 2019-12-24; the sheet prints the 1:00 p.m. close and nothing after it |
| 2019-12-25 | closed | `Closed` | `NQ-HOL-2019` | T1 | event date 2019-12-25; the sheet prints no session for it |

### 2020

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2020-01-01 | closed | `Closed` | `NQ-HOL-2020` | T1 | event date 2020-01-01; the sheet prints no session for it |
| 2020-01-20 | closed | `Closed` | `NQ-HOL-2020` | T1 | event date 2020-01-20; the sheet prints no session for it |
| 2020-02-17 | closed | `Closed` | `NQ-HOL-2020` | T1 | event date 2020-02-17; the sheet prints no session for it |
| 2020-04-10 | closed | `Closed` | `NQ-HOL-2020` | T1 | event date 2020-04-10; the sheet prints no session for it |
| 2020-05-25 | closed | `Closed` | `NQ-HOL-2020` | T1 | event date 2020-05-25; the sheet prints no session for it |
| 2020-07-03 | closed | `Closed` | `NQ-HOL-2020` | T1 | event date 2020-07-03; the sheet prints no session for it |
| 2020-09-07 | closed | `Closed` | `NQ-HOL-2020` | T1 | event date 2020-09-07; the sheet prints no session for it |
| 2020-11-26 | closed | `Closed` | `NQ-HOL-2020` | T1 | event date 2020-11-26; the sheet prints no session for it |
| 2020-11-27 | early close | `Early Close - U.S. 1:00 p.m.` | `NQ-HOL-2020` | T1 | event date 2020-11-27; the sheet prints the 1:00 p.m. close and nothing after it |
| 2020-12-24 | early close | `Early Close - U.S. 1:00 p.m.` | `NQ-HOL-2020` | T1 | event date 2020-12-24; the sheet prints the 1:00 p.m. close and nothing after it |
| 2020-12-25 | closed | `Closed` | `NQ-HOL-2020` | T1 | event date 2020-12-25; the sheet prints no session for it |

### 2021

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2021-01-01 | closed | `Closed` | `NQ-HOL-2021` | T1 | event date 2021-01-01; the sheet prints no session for it |
| 2021-01-18 | closed | `Closed` | `NQ-HOL-2021` | T1 | event date 2021-01-18; the sheet prints no session for it |
| 2021-02-15 | closed | `Closed` | `NQ-HOL-2021` | T1 | event date 2021-02-15; the sheet prints no session for it |
| 2021-04-02 | closed | `Closed` | `NQ-HOL-2021` | T1 | event date 2021-04-02; the sheet prints no session for it |
| 2021-05-31 | closed | `Closed` | `NQ-HOL-2021` | T1 | event date 2021-05-31; the sheet prints no session for it |
| 2021-07-05 | closed | `Closed` | `NQ-HOL-2021` | T1 | event date 2021-07-05; the sheet prints no session for it |
| 2021-09-06 | closed | `Closed` | `NQ-HOL-2021` | T1 | event date 2021-09-06; the sheet prints no session for it |
| 2021-11-25 | closed | `Closed` | `NQ-HOL-2021` | T1 | event date 2021-11-25; the sheet prints no session for it |
| 2021-11-26 | early close | `Early Close - U.S. 1:00 p.m.` | `NQ-HOL-2021` | T1 | event date 2021-11-26; the sheet prints the 1:00 p.m. close and nothing after it |
| 2021-12-24 | closed | `Closed` | `NQ-HOL-2021` | T1 | event date 2021-12-24; the sheet prints no session for it |

### 2022

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2022-01-17 | closed | `Closed` | `NQ-HOL-2022` | T1 | event date 2022-01-17; the sheet prints no session for it |
| 2022-02-21 | closed | `Closed` | `NQ-HOL-2022` | T1 | event date 2022-02-21; the sheet prints no session for it |
| 2022-04-15 | closed | `Closed` | `NQ-HOL-2022` | T1 | event date 2022-04-15; the sheet prints no session for it |
| 2022-05-30 | closed | `Closed` | `NQ-HOL-2022` | T1 | event date 2022-05-30; the sheet prints no session for it |
| 2022-06-20 | closed | `Closed` | `NQ-HOL-2022` | T1 | event date 2022-06-20; the sheet prints no session for it |
| 2022-07-04 | closed | `Closed` | `NQ-HOL-2022` | T1 | event date 2022-07-04; the sheet prints no session for it |
| 2022-09-05 | closed | `Closed` | `NQ-HOL-2022` | T1 | event date 2022-09-05; the sheet prints no session for it |
| 2022-11-24 | closed | `Closed` | `NQ-HOL-2022` | T1 | event date 2022-11-24; the sheet prints no session for it |
| 2022-11-25 | early close | `Early Close - U.S. 1:00 p.m.` | `NQ-HOL-2022` | T1 | event date 2022-11-25; the sheet prints the 1:00 p.m. close and nothing after it |
| 2022-12-26 | closed | `Closed` | `NQ-HOL-2022` | T1 | event date 2022-12-26; the sheet prints no session for it |

### 2023

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2023-01-02 | closed | `Closed` | `NQ-HOL-2023` | T1 | event date 2023-01-02; the sheet prints no session for it |
| 2023-01-16 | closed | `Closed` | `NQ-HOL-2023` | T1 | event date 2023-01-16; the sheet prints no session for it |
| 2023-02-20 | closed | `Closed` | `NQ-HOL-2023` | T1 | event date 2023-02-20; the sheet prints no session for it |
| 2023-04-07 | closed | `Closed` | `NQ-HOL-2023` | T1 | event date 2023-04-07; the sheet prints no session for it |
| 2023-05-29 | closed | `Closed` | `NQ-HOL-2023` | T1 | event date 2023-05-29; the sheet prints no session for it |
| 2023-06-19 | closed | `Closed` | `NQ-HOL-2023` | T1 | event date 2023-06-19; the sheet prints no session for it |
| 2023-07-04 | closed | `Closed` | `NQ-HOL-2023` | T1 | event date 2023-07-04; the sheet prints no session for it |
| 2023-09-04 | closed | `Closed` | `NQ-HOL-2023` | T1 | event date 2023-09-04; the sheet prints no session for it |
| 2023-11-23 | closed | `Closed` | `NQ-HOL-2023` | T1 | event date 2023-11-23; the sheet prints no session for it |
| 2023-11-24 | early close | `Early Close - U.S. 1:00 p.m.` | `NQ-HOL-2023` | T1 | event date 2023-11-24; the sheet prints the 1:00 p.m. close and nothing after it |
| 2023-12-25 | closed | `Closed` | `NQ-HOL-2023` | T1 | event date 2023-12-25; the sheet prints no session for it |

### 2024

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2024-01-01 | closed | `Closed` | `NQ-HOL-2024` | T1 | event date 2024-01-01; the sheet prints no session for it |
| 2024-01-15 | closed | `Closed` | `NQ-HOL-2024` | T1 | event date 2024-01-15; the sheet prints no session for it |
| 2024-02-19 | closed | `Closed` | `NQ-HOL-2024` | T1 | event date 2024-02-19; the sheet prints no session for it |
| 2024-03-29 | closed | `Closed` | `NQ-HOL-2024` | T1 | event date 2024-03-29; the sheet prints no session for it |
| 2024-05-27 | closed | `Closed` | `NQ-HOL-2024` | T1 | event date 2024-05-27; the sheet prints no session for it |
| 2024-06-19 | closed | `Closed` | `NQ-HOL-2024` | T1 | event date 2024-06-19; the sheet prints no session for it |
| 2024-07-03 | early close | `Early Close - U.S. 1:00 p.m.` | `NQ-HOL-2024` | T1 | event date 2024-07-03; the sheet prints the 1:00 p.m. close and nothing after it |
| 2024-07-04 | closed | `Closed` | `NQ-HOL-2024` | T1 | event date 2024-07-04; the sheet prints no session for it |
| 2024-09-02 | closed | `Closed` | `NQ-HOL-2024` | T1 | event date 2024-09-02; the sheet prints no session for it |
| 2024-11-28 | closed | `Closed` | `NQ-HOL-2024` | T1 | event date 2024-11-28; the sheet prints no session for it |
| 2024-11-29 | early close | `Early Close - U.S. 1:00 p.m.` | `NQ-HOL-2024` | T1 | event date 2024-11-29; the sheet prints the 1:00 p.m. close and nothing after it |
| 2024-12-24 | early close | `Early Close - U.S. 1:00 p.m.` | `NQ-HOL-2024` | T1 | event date 2024-12-24; the sheet prints the 1:00 p.m. close and nothing after it |
| 2024-12-25 | closed | `Closed` | `NQ-HOL-2024` | T1 | event date 2024-12-25; the sheet prints no session for it |

### 2025

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2025-01-01 | closed | `Closed` | `NQ-HOL-2025` | T1 | event date 2025-01-01; the sheet prints no session for it |
| 2025-01-09 | unsourced | `TBA` / not printed | `NQ-HOL-2025` | T1 | event date 2025-01-09; operator states the arrangement but no recovered artifact states its time |
| 2025-01-20 | closed | `Closed` | `NQ-HOL-2025` | T1 | event date 2025-01-20; the sheet prints no session for it |
| 2025-02-17 | closed | `Closed` | `NQ-HOL-2025` | T1 | event date 2025-02-17; the sheet prints no session for it |
| 2025-04-18 | closed | `Closed` | `NQ-HOL-2025` | T1 | event date 2025-04-18; the sheet prints no session for it |
| 2025-05-26 | closed | `Closed` | `NQ-HOL-2025` | T1 | event date 2025-05-26; the sheet prints no session for it |
| 2025-06-19 | closed | `Closed` | `NQ-HOL-2025` | T1 | event date 2025-06-19; the sheet prints no session for it |
| 2025-07-03 | early close | `1:00 p.m. ET` | `NQ-HOL-2025` | T1 | the sheet's own table row: `July 3, 2025` / `Early Close* - U.S.` / `1:00 p.m.`; the `*` defers to the alert footnote ("Nasdaq will continue to send alerts…"), which states no time of its own |
| 2025-07-04 | closed | `Closed` | `NQ-HOL-2025` | T1 | event date 2025-07-04; the sheet prints no session for it |
| 2025-09-01 | closed | `Closed` | `NQ-HOL-2025` | T1 | event date 2025-09-01; the sheet prints no session for it |
| 2025-11-27 | closed | `Closed` | `NQ-HOL-2025` | T1 | event date 2025-11-27; the sheet prints no session for it |
| 2025-11-28 | early close | `Early Close - U.S. 1:00 p.m.` | `NQ-HOL-2025` | T1 | event date 2025-11-28; the sheet prints the 1:00 p.m. close and nothing after it |
| 2025-12-24 | early close | `Early Close - U.S. 1:00 p.m.` | `NQ-HOL-2025` | T1 | event date 2025-12-24; the sheet prints the 1:00 p.m. close and nothing after it |
| 2025-12-25 | closed | `Closed` | `NQ-HOL-2025` | T1 | event date 2025-12-25; the sheet prints no session for it |

### 2026

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2026-01-01 | closed | `Closed` | `NQ-HOL-2026` | T1 | event date 2026-01-01; the sheet prints no session for it |
| 2026-01-19 | closed | `Closed` | `NQ-HOL-2026` | T1 | event date 2026-01-19; the sheet prints no session for it |
| 2026-02-16 | closed | `Closed` | `NQ-HOL-2026` | T1 | event date 2026-02-16; the sheet prints no session for it |
| 2026-04-03 | closed | `Closed` | `NQ-HOL-2026` | T1 | event date 2026-04-03; the sheet prints no session for it |
| 2026-05-25 | closed | `Closed` | `NQ-HOL-2026` | T1 | event date 2026-05-25; the sheet prints no session for it |
| 2026-06-19 | closed | `Closed` | `NQ-HOL-2026` | T1 | event date 2026-06-19; the sheet prints no session for it |
| 2026-07-03 | closed | `Closed` | `NQ-HOL-2026` | T1 | event date 2026-07-03; the sheet prints no session for it |
| 2026-09-07 | closed | `Closed` | `NQ-HOL-2026` | T1 | event date 2026-09-07; the sheet prints no session for it |
| 2026-11-26 | closed | `Closed` | `NQ-HOL-2026` | T1 | event date 2026-11-26; the sheet prints no session for it |
| 2026-11-27 | early close | `Early Close - U.S. 1:00 p.m.` | `NQ-HOL-2026` | T1 | event date 2026-11-27; the sheet prints the 1:00 p.m. close and nothing after it |
| 2026-12-24 | early close | `Early Close - U.S. 1:00 p.m.` | `NQ-HOL-2026` | T1 | event date 2026-12-24; the sheet prints the 1:00 p.m. close and nothing after it |
| 2026-12-25 | closed | `Closed` | `NQ-HOL-2026` | T1 | event date 2026-12-25; the sheet prints no session for it |
### Documents

| Document | Window | Replay or service URL | Capture or retrieval, UTC | Tier | sha256 |
|---|---|---|---|---|---|
| `NQ-HOL-2010` | 2010-01-01 .. 2010-12-31 | <https://web.archive.org/web/20101005102049id_/https://nasdaqtrader.com/Trader.aspx?id=Calendar> | captured 2010-10-05, retrieved 2026-09-27 | T1 | `1b2ff2f4bcc789aff160c74d1d0562bc2006e6feb3f303aa851c1a9ebdda064a` |
| `NQ-HOL-2011` | 2011-01-01 .. 2012-12-31 | <https://web.archive.org/web/20111219004705id_/https://www.nasdaqtrader.com/Trader.aspx?id=Calendar> | captured 2011-12-19, retrieved 2026-09-27 | T1 | `4b09e9145e119ac20f3b81621035c9047f69951578811198ca9ffd0fde197864` |
| `NQ-HOL-2012` | 2012-01-01 .. 2012-12-31 | <https://web.archive.org/web/20121224035751id_/https://www.nasdaqtrader.com/Trader.aspx?id=Calendar> | captured 2012-12-24, retrieved 2026-09-27 | T1 | `3e1c9fe3fa051591bcb88378aa456ca2a2a5597b80aa30d2e1bbb215579a6080` |
| `NQ-HOL-2013` | 2013-01-01 .. 2013-12-31 | <https://web.archive.org/web/20131115174606id_/https://www.nasdaqtrader.com/Trader.aspx?id=Calendar> | captured 2013-11-15, retrieved 2026-09-27 | T1 | `2806df949fc5391a488c735956143d0e23fefa8406bbfba53cc01e0b06a50546` |
| `NQ-HOL-2014` | 2014-01-01 .. 2014-12-31 | <https://web.archive.org/web/20141230075257id_/https://www.nasdaqtrader.com/Trader.aspx?id=Calendar> | captured 2014-12-30, retrieved 2026-09-27 | T1 | `86a492fc29e1332fdd91860b5823ded5becd9965d239e21ff9554fafb23f188c` |
| `NQ-HOL-2015` | 2015-01-01 .. 2016-12-31 | <https://web.archive.org/web/20151231131937id_/https://www.nasdaqtrader.com/Trader.aspx?id=Calendar> | captured 2015-12-31, retrieved 2026-09-27 | T1 | `91edeba7820084a4b4b1bf687110fb6c1485d4e0b72454f35a38b6d9b16c5999` |
| `NQ-HOL-2017` | 2016-12-26 .. 2017-12-31 | <https://web.archive.org/web/20161221145852id_/https://www.nasdaqtrader.com/Trader.aspx?id=Calendar> | captured 2016-12-21, retrieved 2026-09-27 | T1 | `643f0d373e405990d80865b4d6122ed2ae3901be2e296777007b57e10faa9dfe` |
| `NQ-HOL-2018` | 2017-12-25 .. 2018-12-31 | <https://web.archive.org/web/20171226142407id_/https://www.nasdaqtrader.com/Trader.aspx?id=Calendar> | captured 2017-12-26, retrieved 2026-09-27 | T1 | `002fbc7f18fd55d49839f99cc6a8f961c5bfd8df500e26c1f55e6d76766893da` |
| `NQ-HOL-2019` | 2018-12-24 .. 2019-12-31 | <https://web.archive.org/web/20181226231700id_/https://www.nasdaqtrader.com/Trader.aspx?id=Calendar> | captured 2018-12-26, retrieved 2026-09-27 | T1 | `e52f480be17e8f305ec1fbadb19d62c4b8a0ee80d4ff13872e76b4651b1350cd` |
| `NQ-HOL-2020` | 2020-01-01 .. 2020-12-31 | <https://web.archive.org/web/20200704055213id_/https://www.nasdaqtrader.com/trader.aspx?id=calendar> | captured 2020-07-04, retrieved 2026-09-27 | T1 | `92b78f1c9027361c652a6d9d8683a32a5f9d48f4afdffdda866a85a2c616f3e7` |
| `NQ-HOL-2021` | 2020-12-24 .. 2021-12-31 | <https://web.archive.org/web/20201224000133id_/https://www.nasdaqtrader.com/Trader.aspx?id=Calendar> | captured 2020-12-24, retrieved 2026-09-27 | T1 | `2481d7f5acef0d8e19a92c80e4439e9cd4a69ae40312f97f8a4433f53bd49f22` |
| `NQ-HOL-2022` | 2021-12-24 .. 2022-12-31 | <https://web.archive.org/web/20211230181846id_/https://www.nasdaqtrader.com/Trader.aspx?id=Calendar> | captured 2021-12-30, retrieved 2026-09-27 | T1 | `9c7620406827968ec7953aa095e8d042a60b8e5b6c5571aa571f06c103ed2656` |
| `NQ-HOL-2023` | 2022-11-25 .. 2023-12-31 | <https://web.archive.org/web/20221231173848id_/https://www.nasdaqtrader.com/Trader.aspx?id=Calendar> | captured 2022-12-31, retrieved 2026-09-27 | T1 | `5c8d7a83c46515bdc8e4a9cda722917973ec4c5d5a728fcf8b406a5e89945c07` |
| `NQ-HOL-2024` | 2023-12-25 .. 2024-12-31 | <https://web.archive.org/web/20231225031448id_/https://www.nasdaqtrader.com/Trader.aspx?id=Calendar> | captured 2023-12-25, retrieved 2026-09-27 | T1 | `0c82875d72990b681343f55a3ca594a648bc42332ad22e528838bd0c820d1bc0` |
| `NQ-HOL-2025` | 2024-12-29 .. 2025-12-31 | <https://web.archive.org/web/20241229150248id_/https://www.nasdaqtrader.com/Trader.aspx?id=Calendar> | captured 2024-12-29, retrieved 2026-09-27 | T1 | `8b57f45eb7cc7e6aa373591d48be9b2e77ff3319500c107a0caebff9d2f32db9` |
| `NQ-HOL-2026` | 2026-01-01 .. 2026-12-31 | <https://www.nasdaqtrader.com/trader.aspx?id=calendar> | retrieved 2026-09-27 | T1 | `47db9c37c67e3bd095a5e596b02bf61b3ca3d49de48f45a9d4c7e3de5745270a` |
| `NQ-SANDY-2012` | 2012-10-29 | <https://www.nasdaqtrader.com/TraderNews.aspx?id=ETA2012-44> | captured 2012-10-29, retrieved 2026-09-27 | T1 | `04ff48074681a33f0a14e59411c75ffd34604b806955bdcaa60bad4f0f2a33cc` |
| `NQ-NW-2010-008` | 2010-01-15 .. 2013-03-17 (the in-force rulebook text the filing quotes; Normal-week envelope rows) | <https://www.federalregister.gov/documents/full_text/text/2010/02/23/2010-3394.txt> | retrieved 2026-09-30 | T1 | `8773a28b348bfcca3730a4229fba69516f937ae0a4da5dc6357a6636cfbcfe12` |
| `NQ-NW-2012-1285` | 2012-01-24 .. 2013-03-17 (the Rule 4120(b)(4) footnote; Normal-week rows) | <https://www.federalregister.gov/documents/full_text/text/2012/01/24/2012-1285.txt> | retrieved 2026-09-30 | T1 | `c986d71f5581694b153d114ba7687086414485006cd3b3735dccf4b6f850c6b6` |
| `NQ-NW-2012-5367` | 2012-03-06 .. 2013-03-17 (the Rule 4120(b)(4) footnote; Normal-week corroboration) | <https://www.federalregister.gov/documents/full_text/text/2012/03/06/2012-5367.txt> | retrieved 2026-09-30 | T1 | `72e8461f11b44c14e999e51f177e27c3a563b3cbcba2b3423f8058daa2ab6c76` |
| `NQ-NW-2012-21815` | 2012-09-05 .. 2013-03-17 (the Rule 4120(b)(4) footnote; Normal-week corroboration) | <https://www.federalregister.gov/documents/full_text/text/2012/09/05/2012-21815.txt> | retrieved 2026-09-30 | T1 | `dc16205617f178dd2ea1f56d8ced9489acb86d9ea2cacce3bb5c4b9f28935829` |
| `NQ-NW-2012-26253` | 2012-10-25 .. 2013-03-17 (the Rule 4120(b)(4) footnote; Normal-week corroboration) | <https://www.federalregister.gov/documents/full_text/text/2012/10/25/2012-26253.txt> | retrieved 2026-09-30 | T1 | `67bac20d532718052b3b5a79d23e2b04812ec46a4e9a5690349fa74d4fb8d5fa` |
| `NQ-NW-2012-PAGE-20120325` | 2012-03-25 .. 2013-03-17 (the pre-2013 Trading Hours page; Normal-week rows) | <https://web.archive.org/web/20120325161230id_/http://www.nasdaqomx.com/trading/marketplaces/tradinghours/> | Wayback `id_` replay of capture `20120325161230`, retrieved 2026-09-30 | T1 | `a79bb35c8fd5dd04a1dc450297df8f89afc00da17bc8c0035306539f70b77d10` |
| `NQ-NW-2012-PAGE-20120506` | 2012-05-06 .. 2013-03-17 (the pre-2013 Trading Hours page; Normal-week corroboration) | <https://web.archive.org/web/20120506131531id_/http://www.nasdaqomx.com/trading/marketplaces/tradinghours/> | Wayback `id_` replay of capture `20120506131531`, retrieved 2026-09-30 | T1 | `a1b8c6dc6ef6f8976cbbc6fb8815b216c27eba146a76ef26095d3ad4d684d6ae` |
| `NQ-NW-2012-PAGE-20120922` | 2012-09-22 .. 2013-03-17 (the pre-2013 Trading Hours page; Normal-week corroboration) | <https://web.archive.org/web/20120922200838id_/http://www.nasdaqomx.com/trading/marketplaces/tradinghours/> | Wayback `id_` replay of capture `20120922200838`, retrieved 2026-09-30 | T1 | `aea6d39bf6d8ef5f86adca4e75310cbe53200d5e10d67b07c3f611302bc6eaf3` |
| `NQ-NW-2012-PAGE-20120924` | 2012-09-24 .. 2013-03-17 (the pre-2013 Trading Hours page; Normal-week corroboration) | <https://web.archive.org/web/20120924001331id_/http://www.nasdaqomx.com/trading/marketplaces/tradinghours/> | Wayback `id_` replay of capture `20120924001331`, retrieved 2026-09-30 | T1 | `5b8b945d0f280e9a7e78261f9e31893b48d547d490e93a2f4bb20679453afb4c` |
| `NQ-NW-2012-PAGE-20121005` | 2012-10-05 .. 2013-03-17 (the pre-2013 Trading Hours page; Normal-week corroboration) | <https://web.archive.org/web/20121005120912id_/http://www.nasdaqomx.com/trading/marketplaces/tradinghours/> | Wayback `id_` replay of capture `20121005120912`, retrieved 2026-09-30 | T1 | `a0edbe4d3d3b57a9deddf466c7197224337cf355e83cb546be4b517998eb14c6` |
| `NQ-NW-2013-04614` | 2013-02-28 .. 2013-03-17 (the Rule 4120(b)(4) footnote; Normal-week corroboration) | <https://www.federalregister.gov/documents/full_text/text/2013/02/28/2013-04614.txt> | retrieved 2026-09-30 | T1 | `3bcf140b8fff571c2bd18ec77286cd244e8328f3f81650540629f36155093ef5` |
| `NQ-NW-2013-033-NOTICE` | 2013-03-05 .. 2013-03-17 (the outgoing three-session statement; Normal-week rows) | <https://www.federalregister.gov/documents/full_text/text/2013/03/21/2013-06479.txt> | retrieved 2026-09-30 | T1 | `41d83bdf0e3ad7c70efc66a9de79877b64632487709d142126eb6b3221f94c72` |
| `NQ-NW-2013-033-FILING` | 2013-03-05 .. 2013-03-17 (the marked Rule 4120(b)(4)(B) text and the implementation day; Normal-week rows) | <https://web.archive.org/web/20210115152527id_/https://listingcenter.nasdaq.com/assets/rulebook/nasdaq/filings/SR-NASDAQ-2013-033.pdf> | Wayback `id_` replay of capture `20210115152527`, retrieved 2026-09-30 | T1 | `fe71e4f64fff33fbe6d4fb54619b60a261570b4ccf209f15eef3825c570cafc8` |
| `NQ-NW-2013-13998` | no rows keyed (the conforming filing to "the 4 a.m. Filing"; rule 4120(c)(7)(B) "7 a.m." to "4:00 a.m."; Normal-week bounded-search record) | <https://www.federalregister.gov/documents/full_text/text/2013/06/13/2013-13998.txt> | retrieved 2026-09-30 | T1 | `db82e74989cc1950a094ef217b751b3f1cd0a0d4b5735cbeabfa612f20f06e90` |
| `NASDAQ-SYS-2010-A` | no rows keyed (the 2010 systems-hours inventory; Normal-week bounded-search record) | <https://web.archive.org/web/20100101193510id_/http://nasdaqtrader.com/content/TechnicalSupport/nasdaq_sys_hours.pdf> | Wayback `id_` replay of capture `20100101193510`, retrieved 2026-09-30 04:52 UTC | T1 | `57a5d35d99f03690814c2c74157fc5a2b2417b964e03cd3a1adae2803f4d8c81` |
| `NASDAQ-SYS-2010-B` | no rows keyed (the December 2010 edition, doc code Q10-0079; Normal-week bounded-search record) | <https://web.archive.org/web/20101230180554id_/http://ftp.nasdaqtrader.com/content/TechnicalSupport/nasdaq_sys_hours.pdf> | Wayback `id_` replay of capture `20101230180554`, retrieved 2026-09-30 04:52 UTC | T1 | `062e5fda19370bdf65abc22f2de32147df8b68566c4888355fcfa10cd5d80799` |

## Sources

Row review: 2026-08-22 (UTC) is the date the ledger row was last reviewed as a
whole. Per-source retrieval dates were not recorded before the 2026-09-12
migration; where a later targeted review, capture or document date is recorded
beside a source below, that date governs for that source, and dates are added
as each source is re-verified.

- <https://www.nasdaqtrader.com/trader.aspx?id=calendar> — the operator's U.S. Equity and Options Markets Holiday Schedule (live, states 2026); the 2010-2025 editions are read through the web archive, one replay per schedule year (digests in the `### Documents` tables above).
- <https://www.nasdaq.com/market-activity/stock-market-holiday-schedule> — the operator's consumer-facing holiday schedule and its PDF twin (live, states 2026).
- <https://listingcenter.nasdaq.com/rulebook/nasdaq/rules/Nasdaq%20Equity%202> — Nasdaq Equity 2, the rulebook provision behind the 04:00–20:00 System Hours.
- <https://www.nasdaqtrader.com/content/technicalsupport/nasdaq_sys_hours.pdf> — *Nasdaq Systems — Hours of Operation*, the operator's system inventory and phase table (2020 edition; read through the web archive, see the ledger's channel notes).
- <https://www.nasdaqtrader.com/TraderNews.aspx?id=ETA2013-21> — Nasdaq Equity Trader Alert 2013-21, the 04:00 pre-market open effective Monday 2013-03-18.
- <https://www.nasdaqtrader.com/TraderNews.aspx?id=ETA2026-46> — Nasdaq Equity Trader Alert 2026-46, the announced Night Session.
- <https://listingcenter.nasdaq.com/assets/rulebook/nasdaq/filings/SR-NASDAQ-2025-109_Approval.pdf> — the SEC approval order for the Night Session rule.
- The operator's `nasdaq_sys_hours.pdf` floor-era captures, 2010-01-01 and 2010-12-30 (Wayback replays, digests in the `### Documents` table) — the operator's systems-hours inventory of that era, whose Stock Market row prints Support Hours 07:00-20:00 and Market Hours 09:30-16:00; support hours are not a trading session (LAW-SESSION-NOT-EXPIRY), so they carry the record's negative rather than a row.
- The operator's SEC rule filings, read as Federal Register full texts (digests in the `### Documents` table): SR-NASDAQ-2010-008 (75 FR 8156) quoting the in-force rulebook text, the 2012-2013 filings printing the Rule 4120(b)(4) footnote (`2012-1285`, `2012-5367`, `2012-21815`, `2012-26253`, `2013-04614`), SR-NASDAQ-2013-033 (78 FR 17464) with its marked rule text and implementation day, and the conforming filing (2013-13998).
- The operator's own 19b-4 filing PDF for SR-NASDAQ-2013-033, served from `listingcenter.nasdaq.com/assets/rulebook/nasdaq/filings/` and read through the web archive (`NQ-NW-2013-033-FILING`).
- The operator's own pre-2013 Trading Hours page on `nasdaqomx.com`, one `id_` replay per surviving capture, 2012-03-25 through 2012-10-05 (`NQ-NW-2012-PAGE-*`).

## Gaps and residual risks

- **Horizon — discharged 2026-09-30 UTC.** The 07:00–20:00 grid below
  2013-03-18 was carried until the SEC-filing and archived-page sweep of
  2026-09-30 sourced it to the floor (see the Normal week section): the
  operator's own quoted in-force rulebook text at the floor
  (`NQ-NW-2010-008`), its Rule 4120(b)(4) statements through 2012-2013, its
  archived Trading Hours page, and SR-NASDAQ-2013-033's marked "[7:00]" rule
  text and named implementation day. The #231 horizon tracking for this scope
  is discharged; both of that tracker's scopes now stand discharged (`asx` on
  2026-09-30 — see that file). The residual is the floor-to-first-restatement
  sub-span stated in the Normal week section, not a carried interval: no date
  below the sourced horizon refuses.
- **Bounded search, 2026-09-30 UTC (PAGE channel — closed negative).** The
  floor-era channels that could state the 07:00 open on the operator's trader
  site were checked and close without an admissible artifact:
  nasdaqtrader's own `nasdaq_sys_hours.pdf` survives at captures
  `20100101193510` and `20101230180554` (`NASDAQ-SYS-2010-A`,
  `NASDAQ-SYS-2010-B`), but its Stock Market row reads "Support Hours: 7:00
  a.m. – 8:00 p.m.; Market Hours: 9:30 a.m. – 4:00 p.m." — the help desk's
  staffing window, not a trading session, so it states neither the pre-market
  open nor the post-market close in session language; the nasdaqtrader
  `TradingHours` page has no 2010-2013 capture; `TraderNews.aspx?id=ETA2013-21`
  has no capture at all, so the alert's publication day stays unrecorded. The
  two PDF captures are held in the store and cited below so the negative is
  checkable. The SEC/rulebook channel named as the closing condition then
  produced the floor record the same day (the Normal week section).
- **Bounded search, 2026-09-30 UTC (SEC and archive channels — completed).**
  The Federal Register full-text sweep 2010-01-01..2013-12-31 ("pre-market
  session", "post-market session", "extend the pre-market", "Rule 4120",
  "three trading sessions on the Exchange") returns the keyed statements and
  no other session-hour change language; the 4120(b)(4)-footnote practice
  begins 2012-01-24. EDGAR full-text search ("pre-market session", any form,
  2010-01-01..2013-06-30) returns zero hits, so the Nasdaq 10-K channel
  states no session text in the era. The operator's electronic manual
  (`nasdaqomx.cchwallstreet.com`, the manual the filings mark against) has
  asset-only captures. The sweep records and the unkeyed bycatch (the BX
  start-time filing `2011-7109`, which corroborates `nasdaq_bx`'s keyed row,
  and six Nasdaq LLC filings that state no session hour) are indexed in the
  store under `holidays/raw/nyse-nasdaq/nasdaq/normal-week/INDEX.md`.
- **Watch item, not a gap.** The announced Night Session is monitored and
  unencoded. Nasdaq Equity Trader Alert 2026-46 announces **2026-12-06** as the
  date, but Nasdaq Equity 1 conditions commencement on Equity Data Plan
  readiness and a later Nasdaq readiness filing, so that day is conditional and
  not an unconditional effective day (LAW-NO-FABRICATED-DATES). A conditional
  future date stays in the watch list and out of runtime selectors, so it keys
  no revision row and current and future snapshots stay 04:00–20:00 until the
  readiness filing confirms it. The watch entry is
  [`Pending effective-date confirmations`](../schedules/updating.md#pending-effective-date-confirmations)
  in `updating.md`; `nasdaq_unconfirmed_night_session_is_not_encoded` in
  `tests/venue_sessions/nasdaq.rs` fences the non-encoding.
- **System coverage (2026-09-02).** No discrepancy. ACT, Weblink ACT 2.0, ACES,
  the Nasdaq Testing Facility and index dissemination are excluded classes;
  Nasdaq Fixed Income and Nasdaq Futures belong to neither this SRO nor cash
  equity. Dormant identity, so the residual items above are recorded here rather
  than opened as issues (LAW-FOLLOW-UPS-ARE-ISSUES).

## Module narrative (moved from src/calendar/schedules/equities/us/equities.rs on 2026-09-12 UTC)

Nasdaq, MEMX, and MIAX Pearl publish the 04:00–20:00 shape. Sources: Nasdaq
Equity Rules Equity 2 § 8; MEMX market-hours notice; MIAX Pearl Equities
alert 2024-11-13 and its trading-hours page.

Nasdaq operated 07:00–20:00 ET before moving its pre-market open to 04:00
effective 2013-03-18; the pre-2013 grid is sourced to the floor by the
operator's own SEC filings and its archived Trading Hours page (see the
Normal week section).
https://www.nasdaqtrader.com/TraderNews.aspx?id=ETA2013-21

Nasdaq BX, renamed Nasdaq Texas by the operator. This is not the unrelated
NYSE Texas venue, whose profile lives in `nyse.rs`. The stable public
identity here remains `nasdaq_bx`. The venue publishes 07:00–19:00 ET system
hours around the 09:30–16:00 core session.
An official 2009 circular proves an 08:00–19:00 January-2010 baseline, and
SR-BX-2011-016 proves the later 08:00→07:00 system-hours change, and Equity
Trader Alert 2011-20 makes its production date Monday 2011-04-18. A
March-2014 Nasdaq data notice independently confirms the 07:00 platform open.
https://www.nasdaqtrader.com/content/technicalsupport/nasdaq_sys_hours.pdf
https://www.nasdaqtrader.com/TraderNews.aspx?id=ETA2009-003
https://www.sec.gov/rules/sro/bx/2011/34-64105.pdf
https://www.nasdaqtrader.com/TraderNews.aspx?id=ETA2011-20
https://www.nasdaqtrader.com/TraderNews.aspx?id=dtn2014-08

Nasdaq PSX currently publishes 08:00–17:00 ET system hours. PSX launched
with a 09:00 ET start and kept the same 17:00 close before the 2010-12-13
expansion.
https://listingcenter.nasdaq.com/rulebook/phlx/rules/phlx-psx-legacy-3000
https://www.sec.gov/files/rules/sro/phlx/2010/34-63492.pdf

MEMX shortened its executable Post-Market Session from 20:00 to 17:00 ET
effective 2020-10-05, then restored the 20:00 close on 2023-02-01.
https://info.memxtrading.com/trader-alert-20-06-memx-market-hours-change/
https://info.memxtrading.com/trader-alert-23-04-memx-trading-hours-change/

## Module narrative (moved from src/calendar/schedules/equities/us/history.rs on 2026-09-12 UTC)

Nasdaq Equity Trader Alert 2013-21 moved the pre-market open from 07:00 to
04:00 ET effective Monday 2013-03-18. The operator's own SR-NASDAQ-2013-033
filing marks Rule 4120(b)(4)(B)'s in-force "[7:00]" text and states the same
implementation day. Future Night Session announcements are
monitored in the schedule update guide but are not selected until Nasdaq's
required readiness filing supplies an unconditional effective day.
https://www.nasdaqtrader.com/TraderNews.aspx?id=ETA2013-21
https://listingcenter.nasdaq.com/assets/rulebook/nasdaq/filings/SR-NASDAQ-2013-033.pdf

Nasdaq Equity Trader Alert 2011-20 states that BX began accepting and
executing orders at 07:00 ET on Monday 2011-04-18. The official launch alert
supplies the 08:00 ET predecessor open and unchanged 19:00 close.
https://www.nasdaqtrader.com/TraderNews.aspx?id=ETA2009-003
https://www.nasdaqtrader.com/TraderNews.aspx?id=ETA2011-20

Nasdaq's launch alert dates PSX production to 2010-10-08. The initial rules
operated 09:00–17:00 ET; SR-Phlx-2010-172 explicitly identifies 2010-12-13
as the implementation date for the 08:00 ET opening.
Row evidence:
  2010-10-08 "Nasdaq Equity Trader Alert 2010-56"
    https://www.nasdaqtrader.com/TraderNews.aspx?id=ETA2010-56
  2010-12-13 "SEC SR-Phlx-2010-172"
    https://www.sec.gov/files/rules/sro/phlx/2010/34-63492.pdf

MEMX began live trading on 2020-09-21. It shortened the Post-Market Session
from 20:00 to 17:00 ET on 2020-10-05 and restored the 20:00 close on
2023-02-01. Its own 2025-06-06 retrospective identifies 2025-05-19 as the
actual launch of its 04:00 ET pre-market. The earlier rule filing proposed a
March date, so the exchange's stated production launch is the operative
boundary.
https://memx.com/insights/day-1
https://info.memxtrading.com/trader-alert-20-06-memx-market-hours-change/
https://www.sec.gov/files/rules/sro/memx/2023/34-96773.pdf
https://info.memxtrading.com/trader-alert-23-04-memx-trading-hours-change/
https://memx.com/insights/pre-market-share-gains-and-new-options-active-risk-feature
Row evidence:
  2020-09-21 "MEMX Day 1 retrospective"
    https://memx.com/insights/day-1
  2020-10-05 "MEMX trader alert 20-06"
    https://info.memxtrading.com/trader-alert-20-06-memx-market-hours-change/
  2023-02-01 "MEMX trader alert 23-04"
    https://info.memxtrading.com/trader-alert-23-04-memx-trading-hours-change/
    https://www.sec.gov/files/rules/sro/memx/2023/34-96773.pdf
  2025-05-19 "MEMX retrospective 2025-06-06"
    https://memx.com/insights/pre-market-share-gains-and-new-options-active-risk-feature

MIAX Pearl Equities launched on 2020-09-29. Regulatory Circular 2025-02
later made the Early Trading Session (04:00–09:30 ET) and Late Trading
Session (16:00–20:00 ET) available beginning 2025-02-20. Before that
amendment the exchange-level profile contains Regular Trading Hours only.
https://www.miaxglobal.com/company/markets/us-equities
https://www.miaxglobal.com/sites/default/files/circular-files/MIAX_Pearl_Equities_RC_2025_02_0.pdf
Row evidence:
  2020-09-29 "MIAX Pearl Equities launch notice"
    https://www.miaxglobal.com/company/markets/us-equities
  2025-02-20 "MIAX Pearl Regulatory Circular 2025-02"
    https://www.miaxglobal.com/sites/default/files/circular-files/MIAX_Pearl_Equities_RC_2025_02_0.pdf
