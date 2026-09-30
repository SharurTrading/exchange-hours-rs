<!-- SPDX-License-Identifier: MIT-0 -->

# `nasdaq` — evidence

- **Kind:** `Exchange`
- **Owner module:** [`equities.rs`](../../src/calendar/schedules/equities/us/equities.rs)<br>[`history.rs`](../../src/calendar/schedules/equities/us/history.rs)
- **Source sets:** [`US-NASDAQ-EQUITIES`](../schedules/sources.md#us-nasdaq-equities)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

Nasdaq Stock Market normal week; date-aware lookups retain the sourced 2013 07:00→04:00 early-open change. The announced Night Session is monitored but unencoded pending Equity Data Plan readiness and a later Nasdaq filing, so current and future snapshots remain 04:00–20:00. **Systems in scope (2026-09-02):** the Nasdaq equities matching system (System Hours 04:00–20:00) is the envelope; The Nasdaq Options Market is `nasdaq_nom` and the FINRA/Nasdaq TRF is `finra_trf_carteret`. ACT, Weblink ACT 2.0, ACES (08:00–18:30, interior), the Nasdaq Testing Facility and index dissemination are excluded classes; Nasdaq Fixed Income and Nasdaq Futures belong to neither this SRO nor cash equity. No discrepancy.

## Revision rows

- 2013-03-18 — T1 — Nasdaq Equity Trader Alert 2013-21 — pre-market open moves from 07:00 to 04:00 ET.

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

## Gaps and residual risks

- **Horizon (carried interval).** The 07:00–20:00 grid below 2013-03-18 is
  carried, not separately sourced: Equity Trader Alert 2013-21 states the
  outgoing value when it dates the change, and no earlier Nasdaq artifact in the
  reviewed set states the pre-market open at a day level. The alert's own
  publication date is not recorded here, so the horizon is keyed to the
  effective day it states, 2013-03-18. Closing condition: a Nasdaq rulebook
  edition or trader alert that states the 07:00 System Hours open on a
  floor-era day. Tracked as #231, the carried-horizon tracker this scope shares
  with `asx` (LAW-FOLLOW-UPS-ARE-ISSUES).
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
effective 2013-03-18.
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
04:00 ET effective Monday 2013-03-18. Future Night Session announcements are
monitored in the schedule update guide but are not selected until Nasdaq's
required readiness filing supplies an unconditional effective day.
https://www.nasdaqtrader.com/TraderNews.aspx?id=ETA2013-21

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
