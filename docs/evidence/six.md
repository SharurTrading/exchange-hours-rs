<!-- SPDX-License-Identifier: MIT-0 -->

# `six` — evidence

- **Kind:** `Exchange`
- **Owner module:** [`six.rs`](../../src/calendar/schedules/equities/europe/six.rs)
- **Source sets:** [`EU-SIX`](../schedules/sources.md#eu-six), [`EU-FESE-SECONDARY`](../schedules/sources.md#eu-fese-secondary)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

SIX shares January-2010 phases, including the two-minute randomized opening and closing edges, are operator-sourced; the 2020-06-22 Trading-At-Last launch is date-aware.

## Revision rows

- 2020-06-22 — T1 — SIX SMR8.2 participant readiness — Trading-At-Last added, 17:32–17:40, pushing post-trading back to 17:40.

## Holidays

**Coverage:** 2012-01-01..2017-12-31, 2018-01-01..2019-12-31, 2020-01-01..2024-12-31, 2025-01-01..2027-12-31 (inclusive venue-local trade dates in `Europe/Zurich`; tier T1 throughout).

The span between the 2010 floor and the first window — 2010-2011 — is unaudited and refused, not silent; see Gaps below.

The rows key on SIX's own `Trading Calendar` PDFs, one per year, each a Trading Guide page whose twelve month grids mark every non-trading day and whose legend reads: light shade `Saturday — Market Closed`, lighter shade `Sunday — Market Closed`, dark cell `Market Holiday — Market Closed`. Each year's rows cite that year's PDF: `SIX-TC-2012`..`SIX-TC-2017` (Wayback `id_` replays of the six-swiss-exchange.com `download/participants/regulation/trading_guides/trading_calendar_<year>.pdf` editions) and `SIX-TC-2020`..`SIX-TC-2024` (Wayback replays of the six-group.com `dam/.../trading-guides/trading-calendar-<year>.pdf` editions) for the backfilled years, and `SIX-TC-2025` (2025-05-05 capture of the `trading-guides-upcoming` edition), `SIX-TC-2026` and `SIX-TC-2027` (live download-centre editions; the 2027 file is marked `valid as of 1 July 2026`) for the activation window. The 2018-2019 grids key on `SIX-TG-2018`: the operator's own Trading Guide of 28 May 2018, the live education-path compilation whose `Trading Calendar 2018` and `Trading Calendar 2019` sections print both years' twelve-month grids under the same legend (its pages 25-26), found on 2026-10-03 UTC and closing the 2018-2019 half of #212 as data.

The dark cells were resolved from the PDFs' vector fills — the holiday fill is rgb ≈ (0.0, 0.17, 0.37) in the 2017-2027 editions (the 28 May 2018 guide's grids included) and rgb ≈ (0.84, 0.17, 0.12) red in the 2012-2016 editions — read cell by cell with the day number found inside the same cell rectangle (adjacent holidays print as one merged rectangle and resolve to each covered day) — and every resolved date was cross-checked against its weekday column in the grid (for example 2026-05-14 lands on the Thursday column of the May block), then re-read cell by cell from page renderings. Holidays that fall on a weekend are not dark-marked — the Saturday/Sunday shading already deletes them — and key no weekday row: the Swiss National Day 2026-08-01 and St. Stephen's Day 2026-12-26 (both Saturdays), and St. Berchtold Day 2027-01-02, Labour Day 2027-05-01, Swiss National Day 2027-08-01, Christmas Day 2027-12-25 and St. Stephen's Day 2027-12-26 (all weekend) are those cases, while no holiday of 2018 or 2019 falls on a weekend, so those two grids arise no such exclusion. The calendars print closures only — no half day, no late open and no intraday instant in any audited year — so `Closed` is the only kind the operator's own statement supports.

### 2012

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2012-01-02 | closed | dark cell on 2 January (Monday column) — St. Berchtold Day | `SIX-TC-2012` | T1 | the calendar's own legend: `Market Holiday — Market Closed` |
| 2012-04-06 | closed | dark cell on 6 April (Friday column) — Good Friday | `SIX-TC-2012` | T1 | the calendar's own legend: `Market Holiday — Market Closed` |
| 2012-04-09 | closed | dark cell on 9 April (Monday column) — Easter Monday | `SIX-TC-2012` | T1 | the calendar's own legend: `Market Holiday — Market Closed` |
| 2012-05-01 | closed | dark cell on 1 May (Tuesday column) — Labour Day | `SIX-TC-2012` | T1 | the calendar's own legend: `Market Holiday — Market Closed` |
| 2012-05-17 | closed | dark cell on 17 May (Thursday column) — Ascension Day | `SIX-TC-2012` | T1 | the calendar's own legend: `Market Holiday — Market Closed` |
| 2012-05-28 | closed | dark cell on 28 May (Monday column) — Whit Monday | `SIX-TC-2012` | T1 | the calendar's own legend: `Market Holiday — Market Closed` |
| 2012-08-01 | closed | dark cell on 1 August (Wednesday column) — Swiss National Day | `SIX-TC-2012` | T1 | the calendar's own legend: `Market Holiday — Market Closed` |
| 2012-12-24 | closed | dark cell on 24 December (Monday column) — Christmas Eve | `SIX-TC-2012` | T1 | the calendar's own legend: `Market Holiday — Market Closed` |
| 2012-12-25 | closed | dark cell on 25 December (Tuesday column) — Christmas Day | `SIX-TC-2012` | T1 | the calendar's own legend: `Market Holiday — Market Closed` |
| 2012-12-26 | closed | dark cell on 26 December (Wednesday column) — St. Stephen's Day | `SIX-TC-2012` | T1 | the calendar's own legend: `Market Holiday — Market Closed` |
| 2012-12-31 | closed | dark cell on 31 December (Monday column) — New Year's Eve | `SIX-TC-2012` | T1 | the calendar's own legend: `Market Holiday — Market Closed` |
### 2013

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2013-01-01 | closed | dark cell on 1 January (Tuesday column) — New Year's Day | `SIX-TC-2013` | T1 | the calendar's own legend: `Market Holiday — Market Closed` |
| 2013-01-02 | closed | dark cell on 2 January (Wednesday column) — St. Berchtold Day | `SIX-TC-2013` | T1 | the calendar's own legend: `Market Holiday — Market Closed` |
| 2013-03-29 | closed | dark cell on 29 March (Friday column) — Good Friday | `SIX-TC-2013` | T1 | the calendar's own legend: `Market Holiday — Market Closed` |
| 2013-04-01 | closed | dark cell on 1 April (Monday column) — Easter Monday | `SIX-TC-2013` | T1 | the calendar's own legend: `Market Holiday — Market Closed` |
| 2013-05-01 | closed | dark cell on 1 May (Wednesday column) — Labour Day | `SIX-TC-2013` | T1 | the calendar's own legend: `Market Holiday — Market Closed` |
| 2013-05-09 | closed | dark cell on 9 May (Thursday column) — Ascension Day | `SIX-TC-2013` | T1 | the calendar's own legend: `Market Holiday — Market Closed` |
| 2013-05-20 | closed | dark cell on 20 May (Monday column) — Whit Monday | `SIX-TC-2013` | T1 | the calendar's own legend: `Market Holiday — Market Closed` |
| 2013-08-01 | closed | dark cell on 1 August (Thursday column) — Swiss National Day | `SIX-TC-2013` | T1 | the calendar's own legend: `Market Holiday — Market Closed` |
| 2013-12-24 | closed | dark cell on 24 December (Tuesday column) — Christmas Eve | `SIX-TC-2013` | T1 | the calendar's own legend: `Market Holiday — Market Closed` |
| 2013-12-25 | closed | dark cell on 25 December (Wednesday column) — Christmas Day | `SIX-TC-2013` | T1 | the calendar's own legend: `Market Holiday — Market Closed` |
| 2013-12-26 | closed | dark cell on 26 December (Thursday column) — St. Stephen's Day | `SIX-TC-2013` | T1 | the calendar's own legend: `Market Holiday — Market Closed` |
| 2013-12-31 | closed | dark cell on 31 December (Tuesday column) — New Year's Eve | `SIX-TC-2013` | T1 | the calendar's own legend: `Market Holiday — Market Closed` |
### 2014

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2014-01-01 | closed | dark cell on 1 January (Wednesday column) — New Year's Day | `SIX-TC-2014` | T1 | the calendar's own legend: `Market Holiday — Market Closed` |
| 2014-01-02 | closed | dark cell on 2 January (Thursday column) — St. Berchtold Day | `SIX-TC-2014` | T1 | the calendar's own legend: `Market Holiday — Market Closed` |
| 2014-04-18 | closed | dark cell on 18 April (Friday column) — Good Friday | `SIX-TC-2014` | T1 | the calendar's own legend: `Market Holiday — Market Closed` |
| 2014-04-21 | closed | dark cell on 21 April (Monday column) — Easter Monday | `SIX-TC-2014` | T1 | the calendar's own legend: `Market Holiday — Market Closed` |
| 2014-05-01 | closed | dark cell on 1 May (Thursday column) — Labour Day | `SIX-TC-2014` | T1 | the calendar's own legend: `Market Holiday — Market Closed` |
| 2014-05-29 | closed | dark cell on 29 May (Thursday column) — Ascension Day | `SIX-TC-2014` | T1 | the calendar's own legend: `Market Holiday — Market Closed` |
| 2014-06-09 | closed | dark cell on 9 June (Monday column) — Whit Monday | `SIX-TC-2014` | T1 | the calendar's own legend: `Market Holiday — Market Closed` |
| 2014-08-01 | closed | dark cell on 1 August (Friday column) — Swiss National Day | `SIX-TC-2014` | T1 | the calendar's own legend: `Market Holiday — Market Closed` |
| 2014-12-24 | closed | dark cell on 24 December (Wednesday column) — Christmas Eve | `SIX-TC-2014` | T1 | the calendar's own legend: `Market Holiday — Market Closed` |
| 2014-12-25 | closed | dark cell on 25 December (Thursday column) — Christmas Day | `SIX-TC-2014` | T1 | the calendar's own legend: `Market Holiday — Market Closed` |
| 2014-12-26 | closed | dark cell on 26 December (Friday column) — St. Stephen's Day | `SIX-TC-2014` | T1 | the calendar's own legend: `Market Holiday — Market Closed` |
| 2014-12-31 | closed | dark cell on 31 December (Wednesday column) — New Year's Eve | `SIX-TC-2014` | T1 | the calendar's own legend: `Market Holiday — Market Closed` |
### 2015

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2015-01-01 | closed | dark cell on 1 January (Thursday column) — New Year's Day | `SIX-TC-2015` | T1 | the calendar's own legend: `Market Holiday — Market Closed` |
| 2015-01-02 | closed | dark cell on 2 January (Friday column) — St. Berchtold Day | `SIX-TC-2015` | T1 | the calendar's own legend: `Market Holiday — Market Closed` |
| 2015-04-03 | closed | dark cell on 3 April (Friday column) — Good Friday | `SIX-TC-2015` | T1 | the calendar's own legend: `Market Holiday — Market Closed` |
| 2015-04-06 | closed | dark cell on 6 April (Monday column) — Easter Monday | `SIX-TC-2015` | T1 | the calendar's own legend: `Market Holiday — Market Closed` |
| 2015-05-01 | closed | dark cell on 1 May (Friday column) — Labour Day | `SIX-TC-2015` | T1 | the calendar's own legend: `Market Holiday — Market Closed` |
| 2015-05-14 | closed | dark cell on 14 May (Thursday column) — Ascension Day | `SIX-TC-2015` | T1 | the calendar's own legend: `Market Holiday — Market Closed` |
| 2015-05-25 | closed | dark cell on 25 May (Monday column) — Whit Monday | `SIX-TC-2015` | T1 | the calendar's own legend: `Market Holiday — Market Closed` |
| 2015-12-24 | closed | dark cell on 24 December (Thursday column) — Christmas Eve | `SIX-TC-2015` | T1 | the calendar's own legend: `Market Holiday — Market Closed` |
| 2015-12-25 | closed | dark cell on 25 December (Friday column) — Christmas Day | `SIX-TC-2015` | T1 | the calendar's own legend: `Market Holiday — Market Closed` |
| 2015-12-31 | closed | dark cell on 31 December (Thursday column) — New Year's Eve | `SIX-TC-2015` | T1 | the calendar's own legend: `Market Holiday — Market Closed` |
### 2016

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2016-01-01 | closed | dark cell on 1 January (Friday column) — New Year's Day | `SIX-TC-2016` | T1 | the calendar's own legend: `Market Holiday — Market Closed` |
| 2016-03-25 | closed | dark cell on 25 March (Friday column) — Good Friday | `SIX-TC-2016` | T1 | the calendar's own legend: `Market Holiday — Market Closed` |
| 2016-03-28 | closed | dark cell on 28 March (Monday column) — Easter Monday | `SIX-TC-2016` | T1 | the calendar's own legend: `Market Holiday — Market Closed` |
| 2016-05-05 | closed | dark cell on 5 May (Thursday column) — Ascension Day | `SIX-TC-2016` | T1 | the calendar's own legend: `Market Holiday — Market Closed` |
| 2016-05-16 | closed | dark cell on 16 May (Monday column) — Whit Monday | `SIX-TC-2016` | T1 | the calendar's own legend: `Market Holiday — Market Closed` |
| 2016-08-01 | closed | dark cell on 1 August (Monday column) — Swiss National Day | `SIX-TC-2016` | T1 | the calendar's own legend: `Market Holiday — Market Closed` |
| 2016-12-26 | closed | dark cell on 26 December (Monday column) — St. Stephen's Day | `SIX-TC-2016` | T1 | the calendar's own legend: `Market Holiday — Market Closed` |
### 2017

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2017-01-02 | closed | dark cell on 2 January (Monday column) — St. Berchtold Day | `SIX-TC-2017` | T1 | the calendar's own legend: `Market Holiday — Market Closed` |
| 2017-04-14 | closed | dark cell on 14 April (Friday column) — Good Friday | `SIX-TC-2017` | T1 | the calendar's own legend: `Market Holiday — Market Closed` |
| 2017-04-17 | closed | dark cell on 17 April (Monday column) — Easter Monday | `SIX-TC-2017` | T1 | the calendar's own legend: `Market Holiday — Market Closed` |
| 2017-05-01 | closed | dark cell on 1 May (Monday column) — Labour Day | `SIX-TC-2017` | T1 | the calendar's own legend: `Market Holiday — Market Closed` |
| 2017-05-25 | closed | dark cell on 25 May (Thursday column) — Ascension Day | `SIX-TC-2017` | T1 | the calendar's own legend: `Market Holiday — Market Closed` |
| 2017-06-05 | closed | dark cell on 5 June (Monday column) — Whit Monday | `SIX-TC-2017` | T1 | the calendar's own legend: `Market Holiday — Market Closed` |
| 2017-08-01 | closed | dark cell on 1 August (Tuesday column) — Swiss National Day | `SIX-TC-2017` | T1 | the calendar's own legend: `Market Holiday — Market Closed` |
| 2017-12-25 | closed | dark cell on 25 December (Monday column) — Christmas Day | `SIX-TC-2017` | T1 | the calendar's own legend: `Market Holiday — Market Closed` |
| 2017-12-26 | closed | dark cell on 26 December (Tuesday column) — St. Stephen's Day | `SIX-TC-2017` | T1 | the calendar's own legend: `Market Holiday — Market Closed` |
### 2018

The year's grids ride in the operator's Trading Guide of 28 May 2018 (`SIX-TG-2018`), `Trading Calendar 2018` section.

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2018-01-01 | closed | dark cell on 1 January (Monday column) — New Year's Day | `SIX-TG-2018` | T1 | the calendar's own legend: `Market Holiday — Market Closed` |
| 2018-01-02 | closed | dark cell on 2 January (Tuesday column) — St. Berchtold Day | `SIX-TG-2018` | T1 | the calendar's own legend: `Market Holiday — Market Closed` |
| 2018-03-30 | closed | dark cell on 30 March (Friday column) — Good Friday | `SIX-TG-2018` | T1 | the calendar's own legend: `Market Holiday — Market Closed` |
| 2018-04-02 | closed | dark cell on 2 April (Monday column) — Easter Monday | `SIX-TG-2018` | T1 | the calendar's own legend: `Market Holiday — Market Closed` |
| 2018-05-01 | closed | dark cell on 1 May (Tuesday column) — Labour Day | `SIX-TG-2018` | T1 | the calendar's own legend: `Market Holiday — Market Closed` |
| 2018-05-10 | closed | dark cell on 10 May (Thursday column) — Ascension Day | `SIX-TG-2018` | T1 | the calendar's own legend: `Market Holiday — Market Closed` |
| 2018-05-21 | closed | dark cell on 21 May (Monday column) — Whit Monday | `SIX-TG-2018` | T1 | the calendar's own legend: `Market Holiday — Market Closed` |
| 2018-08-01 | closed | dark cell on 1 August (Wednesday column) — Swiss National Day | `SIX-TG-2018` | T1 | the calendar's own legend: `Market Holiday — Market Closed` |
| 2018-12-24 | closed | dark cell on 24 December (Monday column) — Christmas Eve | `SIX-TG-2018` | T1 | the calendar's own legend: `Market Holiday — Market Closed` |
| 2018-12-25 | closed | dark cell on 25 December (Tuesday column) — Christmas Day | `SIX-TG-2018` | T1 | the calendar's own legend: `Market Holiday — Market Closed` |
| 2018-12-26 | closed | dark cell on 26 December (Wednesday column) — St. Stephen's Day | `SIX-TG-2018` | T1 | the calendar's own legend: `Market Holiday — Market Closed` |
| 2018-12-31 | closed | dark cell on 31 December (Monday column) — New Year's Eve | `SIX-TG-2018` | T1 | the calendar's own legend: `Market Holiday — Market Closed` |

No SIX holiday of 2018 falls on a weekend, so the grid arises no weekend-fall exclusion; the year's twelve dark cells are the only non-weekend marks in it.
### 2019

The same guide's `Trading Calendar 2019` section (`SIX-TG-2018`).

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2019-01-01 | closed | dark cell on 1 January (Tuesday column) — New Year's Day | `SIX-TG-2018` | T1 | the calendar's own legend: `Market Holiday — Market Closed` |
| 2019-01-02 | closed | dark cell on 2 January (Wednesday column) — St. Berchtold Day | `SIX-TG-2018` | T1 | the calendar's own legend: `Market Holiday — Market Closed` |
| 2019-04-19 | closed | dark cell on 19 April (Friday column) — Good Friday | `SIX-TG-2018` | T1 | the calendar's own legend: `Market Holiday — Market Closed` |
| 2019-04-22 | closed | dark cell on 22 April (Monday column) — Easter Monday | `SIX-TG-2018` | T1 | the calendar's own legend: `Market Holiday — Market Closed` |
| 2019-05-01 | closed | dark cell on 1 May (Wednesday column) — Labour Day | `SIX-TG-2018` | T1 | the calendar's own legend: `Market Holiday — Market Closed` |
| 2019-05-30 | closed | dark cell on 30 May (Thursday column) — Ascension Day | `SIX-TG-2018` | T1 | the calendar's own legend: `Market Holiday — Market Closed` |
| 2019-06-10 | closed | dark cell on 10 June (Monday column) — Whit Monday | `SIX-TG-2018` | T1 | the calendar's own legend: `Market Holiday — Market Closed` |
| 2019-08-01 | closed | dark cell on 1 August (Thursday column) — Swiss National Day | `SIX-TG-2018` | T1 | the calendar's own legend: `Market Holiday — Market Closed` |
| 2019-12-24 | closed | dark cell on 24 December (Tuesday column) — Christmas Eve | `SIX-TG-2018` | T1 | the calendar's own legend: `Market Holiday — Market Closed` |
| 2019-12-25 | closed | dark cell on 25 December (Wednesday column) — Christmas Day | `SIX-TG-2018` | T1 | the calendar's own legend: `Market Holiday — Market Closed` |
| 2019-12-26 | closed | dark cell on 26 December (Thursday column) — St. Stephen's Day | `SIX-TG-2018` | T1 | the calendar's own legend: `Market Holiday — Market Closed` |
| 2019-12-31 | closed | dark cell on 31 December (Tuesday column) — New Year's Eve | `SIX-TG-2018` | T1 | the calendar's own legend: `Market Holiday — Market Closed` |

As in 2018, no SIX holiday of 2019 falls on a weekend; the year's twelve dark cells are the only non-weekend marks in its grid.
### 2020

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2020-01-01 | closed | dark cell on 1 January (Wednesday column) — New Year's Day | `SIX-TC-2020` | T1 | the calendar's own legend: `Market Holiday — Market Closed` |
| 2020-01-02 | closed | dark cell on 2 January (Thursday column) — St. Berchtold Day | `SIX-TC-2020` | T1 | the calendar's own legend: `Market Holiday — Market Closed` |
| 2020-04-10 | closed | dark cell on 10 April (Friday column) — Good Friday | `SIX-TC-2020` | T1 | the calendar's own legend: `Market Holiday — Market Closed` |
| 2020-04-13 | closed | dark cell on 13 April (Monday column) — Easter Monday | `SIX-TC-2020` | T1 | the calendar's own legend: `Market Holiday — Market Closed` |
| 2020-05-01 | closed | dark cell on 1 May (Friday column) — Labour Day | `SIX-TC-2020` | T1 | the calendar's own legend: `Market Holiday — Market Closed` |
| 2020-05-21 | closed | dark cell on 21 May (Thursday column) — Ascension Day | `SIX-TC-2020` | T1 | the calendar's own legend: `Market Holiday — Market Closed` |
| 2020-06-01 | closed | dark cell on 1 June (Monday column) — Whit Monday | `SIX-TC-2020` | T1 | the calendar's own legend: `Market Holiday — Market Closed` |
| 2020-12-24 | closed | dark cell on 24 December (Thursday column) — Christmas Eve | `SIX-TC-2020` | T1 | the calendar's own legend: `Market Holiday — Market Closed` |
| 2020-12-25 | closed | dark cell on 25 December (Friday column) — Christmas Day | `SIX-TC-2020` | T1 | the calendar's own legend: `Market Holiday — Market Closed` |
| 2020-12-31 | closed | dark cell on 31 December (Thursday column) — New Year's Eve | `SIX-TC-2020` | T1 | the calendar's own legend: `Market Holiday — Market Closed` |
### 2021

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2021-01-01 | closed | dark cell on 1 January (Friday column) — New Year's Day | `SIX-TC-2021` | T1 | the calendar's own legend: `Market Holiday — Market Closed` |
| 2021-04-02 | closed | dark cell on 2 April (Friday column) — Good Friday | `SIX-TC-2021` | T1 | the calendar's own legend: `Market Holiday — Market Closed` |
| 2021-04-05 | closed | dark cell on 5 April (Monday column) — Easter Monday | `SIX-TC-2021` | T1 | the calendar's own legend: `Market Holiday — Market Closed` |
| 2021-05-13 | closed | dark cell on 13 May (Thursday column) — Ascension Day | `SIX-TC-2021` | T1 | the calendar's own legend: `Market Holiday — Market Closed` |
| 2021-05-24 | closed | dark cell on 24 May (Monday column) — Whit Monday | `SIX-TC-2021` | T1 | the calendar's own legend: `Market Holiday — Market Closed` |
| 2021-12-24 | closed | dark cell on 24 December (Friday column) — Christmas Eve | `SIX-TC-2021` | T1 | the calendar's own legend: `Market Holiday — Market Closed` |
| 2021-12-31 | closed | dark cell on 31 December (Friday column) — New Year's Eve | `SIX-TC-2021` | T1 | the calendar's own legend: `Market Holiday — Market Closed` |
### 2022

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2022-04-15 | closed | dark cell on 15 April (Friday column) — Good Friday | `SIX-TC-2022` | T1 | the calendar's own legend: `Market Holiday — Market Closed` |
| 2022-04-18 | closed | dark cell on 18 April (Monday column) — Easter Monday | `SIX-TC-2022` | T1 | the calendar's own legend: `Market Holiday — Market Closed` |
| 2022-05-26 | closed | dark cell on 26 May (Thursday column) — Ascension Day | `SIX-TC-2022` | T1 | the calendar's own legend: `Market Holiday — Market Closed` |
| 2022-06-06 | closed | dark cell on 6 June (Monday column) — Whit Monday | `SIX-TC-2022` | T1 | the calendar's own legend: `Market Holiday — Market Closed` |
| 2022-08-01 | closed | dark cell on 1 August (Monday column) — Swiss National Day | `SIX-TC-2022` | T1 | the calendar's own legend: `Market Holiday — Market Closed` |
| 2022-12-26 | closed | dark cell on 26 December (Monday column) — St. Stephen's Day | `SIX-TC-2022` | T1 | the calendar's own legend: `Market Holiday — Market Closed` |
### 2023

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2023-01-02 | closed | dark cell on 2 January (Monday column) — St. Berchtold Day | `SIX-TC-2023` | T1 | the calendar's own legend: `Market Holiday — Market Closed` |
| 2023-04-07 | closed | dark cell on 7 April (Friday column) — Good Friday | `SIX-TC-2023` | T1 | the calendar's own legend: `Market Holiday — Market Closed` |
| 2023-04-10 | closed | dark cell on 10 April (Monday column) — Easter Monday | `SIX-TC-2023` | T1 | the calendar's own legend: `Market Holiday — Market Closed` |
| 2023-05-01 | closed | dark cell on 1 May (Monday column) — Labour Day | `SIX-TC-2023` | T1 | the calendar's own legend: `Market Holiday — Market Closed` |
| 2023-05-18 | closed | dark cell on 18 May (Thursday column) — Ascension Day | `SIX-TC-2023` | T1 | the calendar's own legend: `Market Holiday — Market Closed` |
| 2023-05-29 | closed | dark cell on 29 May (Monday column) — Whit Monday | `SIX-TC-2023` | T1 | the calendar's own legend: `Market Holiday — Market Closed` |
| 2023-08-01 | closed | dark cell on 1 August (Tuesday column) — Swiss National Day | `SIX-TC-2023` | T1 | the calendar's own legend: `Market Holiday — Market Closed` |
| 2023-12-25 | closed | dark cell on 25 December (Monday column) — Christmas Day | `SIX-TC-2023` | T1 | the calendar's own legend: `Market Holiday — Market Closed` |
| 2023-12-26 | closed | dark cell on 26 December (Tuesday column) — St. Stephen's Day | `SIX-TC-2023` | T1 | the calendar's own legend: `Market Holiday — Market Closed` |
### 2024

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2024-01-01 | closed | dark cell on 1 January (Monday column) — New Year's Day | `SIX-TC-2024` | T1 | the calendar's own legend: `Market Holiday — Market Closed` |
| 2024-01-02 | closed | dark cell on 2 January (Tuesday column) — St. Berchtold Day | `SIX-TC-2024` | T1 | the calendar's own legend: `Market Holiday — Market Closed` |
| 2024-03-29 | closed | dark cell on 29 March (Friday column) — Good Friday | `SIX-TC-2024` | T1 | the calendar's own legend: `Market Holiday — Market Closed` |
| 2024-04-01 | closed | dark cell on 1 April (Monday column) — Easter Monday | `SIX-TC-2024` | T1 | the calendar's own legend: `Market Holiday — Market Closed` |
| 2024-05-01 | closed | dark cell on 1 May (Wednesday column) — Labour Day | `SIX-TC-2024` | T1 | the calendar's own legend: `Market Holiday — Market Closed` |
| 2024-05-09 | closed | dark cell on 9 May (Thursday column) — Ascension Day | `SIX-TC-2024` | T1 | the calendar's own legend: `Market Holiday — Market Closed` |
| 2024-05-20 | closed | dark cell on 20 May (Monday column) — Whit Monday | `SIX-TC-2024` | T1 | the calendar's own legend: `Market Holiday — Market Closed` |
| 2024-08-01 | closed | dark cell on 1 August (Thursday column) — Swiss National Day | `SIX-TC-2024` | T1 | the calendar's own legend: `Market Holiday — Market Closed` |
| 2024-12-24 | closed | dark cell on 24 December (Tuesday column) — Christmas Eve | `SIX-TC-2024` | T1 | the calendar's own legend: `Market Holiday — Market Closed` |
| 2024-12-25 | closed | dark cell on 25 December (Wednesday column) — Christmas Day | `SIX-TC-2024` | T1 | the calendar's own legend: `Market Holiday — Market Closed` |
| 2024-12-26 | closed | dark cell on 26 December (Thursday column) — St. Stephen's Day | `SIX-TC-2024` | T1 | the calendar's own legend: `Market Holiday — Market Closed` |
| 2024-12-31 | closed | dark cell on 31 December (Tuesday column) — New Year's Eve | `SIX-TC-2024` | T1 | the calendar's own legend: `Market Holiday — Market Closed` |

### 2025

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2025-01-01 | closed | dark `Market Holiday` cell on 1 January (Wednesday column) | `SIX-TC-2025` | T1 | the calendar's own legend: `Market Holiday — Market Closed` |
| 2025-01-02 | closed | dark cell on 2 January (Thursday column) — St. Berchtold Day | `SIX-TC-2025` | T1 | same legend |
| 2025-04-18 | closed | dark cell on 18 April (Friday column) — Good Friday | `SIX-TC-2025` | T1 | same legend |
| 2025-04-21 | closed | dark cell on 21 April (Monday column) — Easter Monday | `SIX-TC-2025` | T1 | same legend |
| 2025-05-01 | closed | dark cell on 1 May (Thursday column) — Labour Day | `SIX-TC-2025` | T1 | same legend |
| 2025-05-29 | closed | dark cell on 29 May (Thursday column) — Ascension Day | `SIX-TC-2025` | T1 | same legend |
| 2025-06-09 | closed | dark cell on 9 June (Monday column) — Whit Monday | `SIX-TC-2025` | T1 | same legend |
| 2025-08-01 | closed | dark cell on 1 August (Friday column) — Swiss National Day | `SIX-TC-2025` | T1 | same legend |
| 2025-12-24 | closed | dark cell on 24 December (Wednesday column) — Christmas Eve | `SIX-TC-2025` | T1 | same legend |
| 2025-12-25 | closed | dark cell on 25 December (Thursday column) — Christmas Day | `SIX-TC-2025` | T1 | same legend |
| 2025-12-26 | closed | dark cell on 26 December (Friday column) — St. Stephen's Day | `SIX-TC-2025` | T1 | same legend |
| 2025-12-31 | closed | dark cell on 31 December (Wednesday column) — New Year's Eve | `SIX-TC-2025` | T1 | same legend |

### 2026

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2026-01-01 | closed | dark cell on 1 January (Thursday column) — New Year's Day | `SIX-TC-2026` | T1 | the calendar's own legend: `Market Holiday — Market Closed` |
| 2026-01-02 | closed | dark cell on 2 January (Friday column) — St. Berchtold Day | `SIX-TC-2026` | T1 | same legend |
| 2026-04-03 | closed | dark cell on 3 April (Friday column) — Good Friday | `SIX-TC-2026` | T1 | same legend |
| 2026-04-06 | closed | dark cell on 6 April (Monday column) — Easter Monday | `SIX-TC-2026` | T1 | same legend |
| 2026-05-01 | closed | dark cell on 1 May (Friday column) — Labour Day | `SIX-TC-2026` | T1 | same legend |
| 2026-05-14 | closed | dark cell on 14 May (Thursday column) — Ascension Day | `SIX-TC-2026` | T1 | same legend |
| 2026-05-25 | closed | dark cell on 25 May (Monday column) — Whit Monday | `SIX-TC-2026` | T1 | same legend; Swiss National Day (Saturday) and St. Stephen's Day (Saturday) 2026 are marked as a weekend, not as holidays |
| 2026-12-24 | closed | dark cell on 24 December (Wednesday column) — Christmas Eve | `SIX-TC-2026` | T1 | same legend |
| 2026-12-25 | closed | dark cell on 25 December (Thursday column) — Christmas Day | `SIX-TC-2026` | T1 | same legend |
| 2026-12-31 | closed | dark cell on 31 December (Thursday column) — New Year's Eve | `SIX-TC-2026` | T1 | same legend |

### 2027

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2027-01-01 | closed | dark cell on 1 January (Friday column) — New Year's Day | `SIX-TC-2027` | T1 | the calendar's own legend: `Market Holiday — Market Closed`; St. Berchtold Day 2027 (Saturday) is marked as a weekend |
| 2027-03-26 | closed | dark cell on 26 March (Friday column) — Good Friday | `SIX-TC-2027` | T1 | same legend |
| 2027-03-29 | closed | dark cell on 29 March (Monday column) — Easter Monday | `SIX-TC-2027` | T1 | same legend |
| 2027-05-06 | closed | dark cell on 6 May (Thursday column) — Ascension Day | `SIX-TC-2027` | T1 | same legend; Labour Day 2027 (Saturday) is marked as a weekend |
| 2027-05-17 | closed | dark cell on 17 May (Monday column) — Whit Monday | `SIX-TC-2027` | T1 | same legend |
| 2027-12-24 | closed | dark cell on 24 December (Friday column) — Christmas Eve | `SIX-TC-2027` | T1 | same legend; Christmas Day (Saturday), St. Stephen's Day (Sunday) and Swiss National Day 2027 (Sunday) are marked as a weekend |
| 2027-12-31 | closed | dark cell on 31 December (Friday column) — New Year's Eve | `SIX-TC-2027` | T1 | same legend |

**Gaps.** The two years **2010-2011 are an unaudited span**: the operator's own Trading Calendar for them is archived on no retrievable operator channel, so the coverage window leaves the span out entirely and a date-aware query inside it refuses with the coverage contract rather than answering from silence. (The 2018-2019 half of the former four-year gap closed as data on 2026-10-03 UTC — see the Extended 2026-10-03 note below — and its rows are the `SIX-TG-2018` tables above.) What was searched, per LAW-BOUNDED-WORK, across both halves of the former gap: the six-swiss-exchange.com crawls of the per-year `trading_calendar_<year>.pdf` series (2012 is the earliest capture, 2011-12-10; the domain's PDF crawls end 2017-11); the era's stable URL `download/participants/regulation/trading_guides/trading_calendar_en.pdf` (captures only from 2022, redirects); the archived `trading_calendar/calendar/<year>/grid_*.pdf` and `participants/.../calendar/<year>/grid_*.pdf` files of 2010-2011, which are the operator's **settlement** calendars — their own prose reads "The SIX Swiss Exchange settlement calendar shows the days on which national banks are closed" and points to the trading calendar as a different document — so they key nothing (a settlement holiday is not a trading day); and the six-group.com side, where the `exchanges/*/trading_calendar/calendar/2019/grid_en.pdf` captures are the **Currency Holiday Calendar 2019** and the `dam/.../trading-guides/` series begins at the 2020 calendar. The settlement and currency artifacts are kept in the research store as `extra_*` files with their digests. **Retry 2026-09-30 UTC with the CDX service working** (outputs and fetches in the store's `cdx-retry-2026-09-30/` directory): the direct `trading_calendar_2010|2011.pdf` and `trading-calendar-2018|2019.pdf` URL sweeps all return empty; the domain-wide `trading_calendar` filter of six-swiss-exchange.com shows the PDF series starting at the 2012 edition (capture 2011-12-10) and the `trading-calendar` filter of six-group.com starts at 2020 (capture 2020-11-28); the 266-key domain-wide `calendar` enumeration of six-swiss-exchange.com holds only the settlement/currency grids for 2010-2011 (re-fetched and re-ruled out: their own sentence is `Any dates not included in the settlement calendar are considered normal trading days`); and the 2019-era six-group.com pages witness the 2019 Trading Calendar's existence — the 2019-04-11 trading-and-settlement page and the 2019-10-18 landing page both link `trading_calendar_2019.pdf` with the words `shows on which days there is no trading on SIX Swiss Exchange` — but no capture of the PDF itself survives, and the live URL now redirects to the products home page while the `dam/.../trading-calendar-2018|2019.pdf` paths return 404 (checked 2026-09-30 UTC). The search record is complete; the caveat that the CDX service was unavailable for the first sweep is closed. **Extended 2026-10-02 UTC (store: `evidence-thread/probes-2026-10-02/`):** the witnessed hrefs are capture-empty on every non-Wayback channel too — archive.today `newest` 404s for `trading_calendar_2018|2019.pdf` and `trading_guide.pdf` on the `/exchanges/` prefix, arquivo.pt timemaps are empty, and the Memento TimeTravel aggregator no longer resolves to its API (DNS now parks on GitHub Pages). The Trading Guide mechanism itself was re-derived: the captured full-guide editions (2015-12-16 "valid as of 26 October 2015", 2016-05-27 and 2016-09-07 "valid as of 1 March 2016") print the 2015-2017 calendars, all sourced, and the six-group `dam/.../trading-guide.pdf` captures begin 2020-10-24; the operators' own 2019-11-19 guides pages enumerate the 2019-era file set — `trading_calendar_2019.pdf`, `trading_calendar_2020.pdf`, `trading_guide.pdf` and the part documents. (That pass concluded the mechanism "cannot help" and left "no unprobed calendar-bearing name"; corrected 2026-10-03 UTC — the education-path compilation probed the same day IS a guide edition that prints grids, and its grids had been misread as absent.) The 2010-2011 side gained the same closure: the full `/download/` tree of 2008-2012 (268 urlkeys) holds no calendar-named artifact beyond `trading_calendar_2012.pdf`, `guide_ttc.pdf` is the Trade Type Code overview, the 2008 order-book trading guide carries no calendar section, and the live download centre publishes no historical calendar. Common Crawl was mid-outage for this pass (the index host 504s on every endpoint, then 307s to dead trailing-slash forms, checked 07:37-08:45 UTC); the earlier pass's seven-index prefix queries stand as the completed CC record. **Extended 2026-10-03 UTC (store: `hunt-3-2026-10-03/`; corrected the same day after the PR #270 review re-read the saved bytes):** the education channel — the one operator-document family not yet tested — **closes the 2018-2019 half as data**. The live education-path `trading-guide.pdf` (`https://www.six-group.com/dam/download/sites/education/preparatory-documentation/trading-module/trading-guide.pdf`, "Trading Guide of 28 May 2018", 28 pages, saved as `six-edu-trading-guide.live-20261003.pdf`, sha256 `f459e1fba545bd057c72ef2f53ebc0540a7004c8419e9db82b05bdac58bee245`) — the parts compilation this file's Sources already cited for the 2018-05-28 normal-week grid — carries **`Trading Calendar 2018` and `Trading Calendar 2019`** sections on its pages 25-26: all twelve months per year, weekday-headered grids under the operator's own footer (`SIX Swiss Exchange AG, P.O. Box, CH-8021 Zurich`; the copyright line `© SIX Swiss Exchange Ltd, 2018` is the page-28 imprint), with the legend `Saturday — Market Closed / Sunday — Market Closed / Market Holiday — Market Closed` (the em dash is this record's transcription convention; the grids' cells carry no dash glyph). The hunt's first read of this artifact had recorded it as having "no calendar section at all"; that sentence was wrong against its own cited bytes (the review's second retrieval found the sections), the store's hunt-3 `INDEX.md` is corrected beside it, and the grids' 24 dark cells key the 2018 and 2019 rows above — cell derivation and both months' full vectors recorded in the store's `guide-calendar-cells-20261003.txt`. The same pass's archived education module stays a negative, restated to what its saved bytes support: `trading-on-ssx-module-1-trading-en.pdf` (April 2019 edition, capture 2019-11-19) is truncated at exactly 1 048 576 bytes (both saved module PDFs are; `x-archive-orig-content-length: 1048576` against a crawler-recorded original length that is not itself archived), no text extraction recovers from the saved bytes — `pdftotext` cannot read the xref and PyMuPDF refuses the file; probe saved as the store's `module1-extraction-attempt-20261003.txt` — so the module keys nothing from saved evidence, and the first pass's interactive reads of it (a complete table of contents naming no calendar chapter, the pointer sentence about the trading calendar, the 2 813 845 crawler-recorded length) were session observations that are not archived in the store and are not relied on here. The same pass closed the remaining URL surfaces: the six-swiss-exchange.com `/exchanges/…` doubled-prefix form of the trading_guides path (zero captures across the whole prefix, `cdx_sixswiss_exchanges_tg.txt`), the German-language per-year names `trading_calendar_2018|2019_de.pdf` (zero captures in the pass's session record, which is not archived as a dump; no witness page names a German per-year file — the 2019-04-11 and 2019-10-18 pages link only the `grid_en.pdf` currency/settlement grids and `trading_calendar_2019.pdf`), the six-group.com media/news paths 2017-2020 (`cdx_sixgroup_media_calendar_2017-2020.txt`: 12 rows — three `trading-currency-holiday-calendar` pages, four `environment-calendar` pages and their factsheet, one teaser image, and three `interbank-clearing/…/bankholidays.html` 2016 news pages — three different instruments, none the exchange's trading calendar and no calendar announcement), the operator's 2019 annual report (retrieved whole, `six-ar2019.txt`: zero holiday content; a statistics brochure's trading-days count could not key dates under LAW-NO-FABRICATED-DATES in any case), and the re-runnable Common Crawl queries (CC-MAIN-2018-13 and 2018-30: no captures for the trading_guides prefix; CC-MAIN-2019-09: only a 301-redirect record; 2019-35 timed out once, re-runnable). Web search re-run with the literal per-year filenames surfaces only other exchanges' same-named documents. **Extended 2026-10-04 UTC (store: `edu-2010-2011-2026-10-04/`)** — the education/documentation trees of the 2010-2011 era themselves, the German/French name variants, the archived guides index pages of 2010-2012, and the official-notices channel, one pass each, all negative for row data but the witness chain now exact: the era's guides index pages captured February and May 2010 (`trading_guides_de.html` 20100203141257, `_en.html` 20100525145312, `_fr.html` 20100525144834) and the 2010-01-31/2010-11-15 trading-and-settlement pages all witness the 2010 and 2011 calendars as the stable name `download/participants/regulation/trading_guides/trading_calendar_en.pdf`, updated in place until the 2012-edition rename, and that URL's captures are exactly four empty 2022 redirects (uncollapsed CDX across all URL forms) — no 2010 or 2011 bite on Wayback, archive.today ("No results"), arquivo.pt (empty timemap, zero text hits) or live (both the six-swiss and six-group `/exchanges/` forms 301 to the products home page, checked 2026-10-04 UTC). The same pages name the Trading Guide edition of 11 January 2010 — the archive path's dated part editions `trading_guide_2010_01_11_en.pdf` and siblings — which by the proven two-year pattern (the 2015-12-16 guide prints 2015 and 2016; the 2016-09-07 guide prints 2016 and 2017) would print the 2010-2011 grids; the whole `regulation/archive/` path holds exactly one Wayback capture (an empty 301 of `on_order_book_2010_01_11_en.pdf` at `20250528090810`), and its six-group mirror the same. The education/training sweeps of the three domains (`educat|training|ausbildung|dokument` 2006-2013) surface only careers pages, trader-education pages and the SWX-era course modules (`swx.com/download/trading/training/1_*..3_*_{de,en,fr}.pdf`, all 2006 captures; six-swiss `education/preparation/1_1_new_issues_de.pdf` and `1_2_dept_sec_de.pdf`, 2009) — no calendar carrier; `kalender`/`calendrier`/`feiertag` are zero on six-swiss-exchange.com and the swx.com calendar hits are the settlement/currency grids already ruled out. The official-messages channel is exhausted with complete title lists: all 85 messages of 2010 (capture 20110108) and all 71 of 2011 (capture 20120610) plus 2012 H1 carry no calendar or holiday announcement, and the dated `swx_message_*.pdf` files were themselves never captured. Web search surfaces only Eurex's own 2011 Handelskalender (another operator's document, concerned with Eurex-listed Swiss products, not the SIX cash-market calendar). The one re-runnable machine channel left: Common Crawl's early indices (2010-2014), 504-outage-blocked on every attempt of this pass (00:28-00:42 UTC), the same outage pattern as 2026-10-02. **Closing condition (narrowed):** bytes of `trading_calendar_en.pdf` as served 2010-2011, or of the 11 January 2010 guide edition, from a future capture of the named URLs (the Common Crawl early-index queries recorded in the store) or a human request to the operator's desk; tracked as #212, whose 2018-2019 half is closed as data by this change. Inside the audited windows nothing is unresolved: every weekday the operator marks ships a row, weekend-falling holidays key no row, and every other trade date is audited normal. Re-checked 2026-09-29 UTC for the forward wave: the live 2027 PDF re-read byte-identically (same sha256 `cd2fdca6…`) and the 2028-edition URL returned the operator's own 404 page, so the horizon stays 2027-12-31 (artifact under `holidays/raw/equities/six/forward-2027/`). Re-checked again 2026-10-03 UTC for the forward wave (artifacts and `INDEX-recheck-2026-10-03.md` under `holidays/raw/equities/six/forward-2028/`): no 2028 edition exists — the `trading-guides/` and `trading-guides-upcoming/` `trading-calendar-2028.pdf` URLs both return the operator's own 404 page and the download centre's document list carries no calendar beyond 2025 — and the 2026 and 2027 PDFs' own cited live URLs have been withdrawn since the 2026-09-29 check, both now returning that 404 page; the rows' bytes stand as retrieved and digested in the store's `2025-2027/` directory, the 2026 URL's Wayback captures run through 2026-04-20, and the 2025 PDF remains live byte-identical (sha256 `0729de0a…`), so this is a watch-channel change, not a data change, and the horizon stays 2027-12-31.

### Documents

The 2012-2024 artifacts were retrieved on 2026-09-29 UTC and saved under `holidays/raw/equities/six/2010-2024/` in the research store; the 2025-2027 artifacts below them were retrieved on 2026-09-28 UTC and saved under `holidays/raw/equities/six/2025-2027/`; the `SIX-TG-2018` guide edition was retrieved on 2026-10-03 09:03 UTC and saved under `holidays/raw/equities/six/hunt-3-2026-10-03/`. The store's `INDEX.md` carries the same digests. The 2012-2016 editions print the holiday cells in red and the 2017-2027 editions in the dark blue the legend shows; the derivation reads each edition's own holiday fill.

| Document | Window | Replay or service URL | Capture or retrieval, UTC | Tier | sha256 |
|---|---|---|---|---|---|
| `SIX-TC-2012` | 2012-01-01 .. 2012-12-31 | <https://web.archive.org/web/20111210134722id_/http://www.six-swiss-exchange.com/download/participants/regulation/trading_guides/trading_calendar_2012.pdf> (capture `20111210134722`, "valid as 1st February 2011") | Wayback `id_` replay of capture `20111210134722`, retrieved 2026-09-29 UTC | T1 | `ade86266f26231fe8b4516e22e57383c85dbee4627ac39dd05f25005d011c55c` |
| `SIX-TC-2013` | 2013-01-01 .. 2013-12-31 | <https://web.archive.org/web/20130512203837id_/http://www.six-swiss-exchange.com/download/participants/regulation/trading_guides/trading_calendar_2013.pdf> (capture `20130512203837`) | Wayback `id_` replay of capture `20130512203837`, retrieved 2026-09-29 UTC | T1 | `5c92de028b17ec4f5e1dca7c8de1b89545baebaa43cb8ba6cf14f10f493e4d1d` |
| `SIX-TC-2014` | 2014-01-01 .. 2014-12-31 | <https://web.archive.org/web/20150501074534id_/http://www.six-swiss-exchange.com/download/participants/regulation/trading_guides/trading_calendar_2014.pdf> (capture `20150501074534`) | Wayback `id_` replay of capture `20150501074534`, retrieved 2026-09-29 UTC | T1 | `289a37b97f79b1087522c4a8fcbf5074cd77e9ca1fd0994872eeed01d839173b` |
| `SIX-TC-2015` | 2015-01-01 .. 2015-12-31 | <https://web.archive.org/web/20150501134212id_/http://www.six-swiss-exchange.com/download/participants/regulation/trading_guides/trading_calendar_2015.pdf> (capture `20150501134212`) | Wayback `id_` replay of capture `20150501134212`, retrieved 2026-09-29 UTC | T1 | `2959a3ad929de0d29f35b4dd1c33589a893179144489ddcfad3ec047f4149d12` |
| `SIX-TC-2016` | 2016-01-01 .. 2016-12-31 | <https://web.archive.org/web/20151123020429id_/http://www.six-swiss-exchange.com/download/participants/regulation/trading_guides/trading_calendar_2016.pdf> (capture `20151123020429`) | Wayback `id_` replay of capture `20151123020429`, retrieved 2026-09-29 UTC | T1 | `aa48b5680e39bab2e56da6b774f60ecd72363e6e5d0fe1192a1bdd22de20f115` |
| `SIX-TC-2017` | 2017-01-01 .. 2017-12-31 | <https://web.archive.org/web/20160905165931id_/http://www.six-swiss-exchange.com/download/participants/regulation/trading_guides/trading_calendar_2017.pdf> (capture `20160905165931`) | Wayback `id_` replay of capture `20160905165931`, retrieved 2026-09-29 UTC | T1 | `ecc5835379b7b6b4679699c313a36377c4470e75e2f2038af14733ea9c33ac4b` |
| `SIX-TG-2018` | 2018-01-01 .. 2019-12-31 | <https://www.six-group.com/dam/download/sites/education/preparatory-documentation/trading-module/trading-guide.pdf> ("Trading Guide of 28 May 2018"; the 2018 and 2019 grids are its pages 25-26) | retrieved 2026-10-03 09:03 UTC | T1 | `f459e1fba545bd057c72ef2f53ebc0540a7004c8419e9db82b05bdac58bee245` |
| `SIX-TC-2020` | 2020-01-01 .. 2020-12-31 | <https://web.archive.org/web/20201128083137id_/https://www.six-group.com/dam/download/the-swiss-stock-exchange/trading/trading-provisions/regulation/trading-guides/trading-calendar-2020.pdf> (capture `20201128083137`, Trading Guide page 25) | Wayback `id_` replay of capture `20201128083137`, retrieved 2026-09-29 UTC | T1 | `32e3208bedaefee731785ea7bec7b68fa67d638c1830df1cf969eec2e42405f6` |
| `SIX-TC-2021` | 2021-01-01 .. 2021-12-31 | <https://web.archive.org/web/20201128070709id_/https://www.six-group.com/dam/download/the-swiss-stock-exchange/trading/trading-provisions/regulation/trading-guides/trading-calendar-2021.pdf> (capture `20201128070709`) | Wayback `id_` replay of capture `20201128070709`, retrieved 2026-09-29 UTC | T1 | `3a2a357028a3f5b2e0c71629d57dc48d06a261219885733c6cdf12c53d9d8315` |
| `SIX-TC-2022` | 2022-01-01 .. 2022-12-31 | <https://web.archive.org/web/20210512075300id_/https://www.six-group.com/dam/download/the-swiss-stock-exchange/trading/trading-provisions/regulation/trading-guides/trading-calendar-2022.pdf> (capture `20210512075300`, Trading Guide page 32) | Wayback `id_` replay of capture `20210512075300`, retrieved 2026-09-29 UTC | T1 | `0f9ea4971affece53f0aa677511e2243f5a7c7342f4be448ce3ff717881b1ff7` |
| `SIX-TC-2023` | 2023-01-01 .. 2023-12-31 | <https://web.archive.org/web/20220120192614id_/https://www.six-group.com/dam/download/the-swiss-stock-exchange/trading/trading-provisions/regulation/trading-guides/trading-calendar-2023.pdf> (capture `20220120192614`) | Wayback `id_` replay of capture `20220120192614`, retrieved 2026-09-29 UTC | T1 | `3cf8ec89525e5a009837ba2d0b5daba506a8a3ce3ed92d8d084e20df1dc10fba` |
| `SIX-TC-2024` | 2024-01-01 .. 2024-12-31 | <https://web.archive.org/web/20230923182818id_/https://www.six-group.com/dam/download/the-swiss-stock-exchange/trading/trading-provisions/regulation/trading-guides/trading-calendar-2024.pdf> (capture `20230923182818`, Trading Guide page 38) | Wayback `id_` replay of capture `20230923182818`, retrieved 2026-09-29 UTC | T1 | `77d8781dc074fbe4ffc9e6e238054e6b1c0f09c99749227ed2011a259a567b37` |
| `SIX-TC-2025` | 2025-01-01 .. 2025-12-31 | <https://web.archive.org/web/20250505133302id_/https://www.six-group.com/dam/download/the-swiss-stock-exchange/trading/trading-provisions/regulation/trading-guides-upcoming/trading-calendar-2025.pdf> (capture `20250505133302`) | Wayback `id_` replay of capture `20250505133302`, retrieved 2026-09-28 UTC | T1 | `0729de0a843ee2e22d50271d2bbc6fef8b031a133d392cd700f7db38878b1ed2` |
| `SIX-TC-2026` | 2026-01-01 .. 2026-12-31 | <https://www.six-group.com/dam/download/the-swiss-stock-exchange/trading/trading-provisions/regulation/trading-guides/trading-calendar-2026.pdf> | retrieved 2026-09-28 01:28 UTC | T1 | `70d1b87db3e65d487159f660e9daec2fc68c6cb53385483c0af7bf99591c9390` |
| `SIX-TC-2027` | 2027-01-01 .. 2027-12-31 | <https://www.six-group.com/dam/download/the-swiss-stock-exchange/trading/trading-provisions/regulation/trading-guides/trading-calendar-2027.pdf> ("valid as of 1 July 2026") | retrieved 2026-09-28 01:28 UTC | T1 | `cd2fdca6f0083709bd9100d30b10415b2f0fce0b0b74b54b7e73f901fb318037` |

## Sources

Row review: 2026-08-22 (UTC) is the date the ledger row was last reviewed as a
whole. Per-source retrieval dates were not recorded before the 2026-09-12
migration; where a later targeted review, capture or document date is recorded
beside a source below, that date governs for that source, and dates are added
as each source is re-verified.

- <https://www.six-group.com/en/products-services/the-swiss-stock-exchange/trading/trading-provisions/trading-hours.html> — SIX Group, "Trading hours": the current page confirms the two-minute opening slot.
- <https://www.six-group.com/dam/download/the-swiss-stock-exchange/trading/trading-provisions/regulation/trading-guides/trading-guide.pdf> — SIX Swiss Exchange Trading Guide. Blue Chip Shares: "Trading Hours 09:00 - 17:30 CET / Continuous Trading 09:00 - 17:20 CET / Closing Auction 17:20 - 17:30 CET / Trading-At-Last Start: 17:30 - 17:32 CET End: 17:40 CET". Its segment row is "Blue Chip Shares 06:00 09:00 17:20 17:30 17:30 17:40 22:00". The trading-period overview runs Pre-Opening from 06:00 "until Opening" and permits no immediate-execution time in force in it (Immediate or Cancel and Fill or Kill are "No" for both Pre-Opening and Post Trading).
- <https://www.six-group.com/dam/download/sites/education/preparatory-documentation/trading-module/trading-guide.pdf> — SIX Trading Guide valid from 2018-05-28: the same Blue Chip grid, including the two-minute randomized opening and closing auction windows.
- <https://web.archive.org/web/20081123115341id_/http://www.six-swiss-exchange.com/download/trading/regulation/directives/swx_dir01_en.pdf> — SIX Directive 1, effective 2007-09-07: exchange hours 06:00–22:00, pre-opening from 06:00 until the opening, post-trading from the close through 22:00, and pre-opening and post-trading separated from the trading phases of the exchange day.
- <https://web.archive.org/web/20090824132532id_/http://www.six-swiss-exchange.com:80/download/marketpulse/news/newsboard/product_guides/product_guide_equities_en.pdf> — SIX Equity Market Product Guide valid from 2009-07-22: the exact shares grid — continuous 09:00–17:20, closing auction 17:20–17:30, two-minute randomized opening and closing windows ending at 09:02 and 17:32.
- <https://www.six-group.com/dam/download/the-swiss-stock-exchange/trading/participation/SWXess-maintenance-releases/smr82_participant_readiness.pdf> — SIX SMR8.2 participant readiness: Trading-At-Last launched with SMR8.2 on 2020-06-22, with the added 17:30–17:40 phase.
- <https://www.six-group.com/en/products-services/the-swiss-stock-exchange/trading/download-center.html> — SIX download centre, the monitoring entry point.
- <https://www.fese.eu/app/uploads/2024/07/trading-hours-2025-1.pdf> — FESE 2025 trading-hours table, `EU-FESE-SECONDARY`: corroboration only.

## Gaps and residual risks

- **Scope.** `Exchange::Six` denotes the shares segments (Blue Chip / Mid-/Small-Cap). SIX does not follow the Xetra pattern: the 17:30–17:35 auction belongs to the ETF/ETP/Sponsored Funds segments only, which have no Trading-At-Last, and those segments are out of scope.
- **Interpretive step, randomized opening.** The Trading Guide's 09:00 opening is randomized over two minutes. The deterministic profile keeps the auction/pre-opening classification through 09:01:59 and starts regular trading at the latest possible edge, 09:02. Within that stretch the guide's own phase boundary applies: 06:00 until 09:00 is Pre-Opening (`order_entry`), 09:00–09:02 is the Opening auction (`extended`, because its uncross prints).
- **Interpretive step, order-entry classification.** Pre-Opening and Post Trading are `order_entry`: an At-the-Opening order entered during Pre-Opening only executes in the Opening Auction that follows, and Directive 1 separates both phases from the trading phases of the exchange day.
- **Time zone.** SIX labels its times "CET" year-round; they are local Zurich wall-clock, so `Europe::Zurich` (CET/CEST) is the correct zone, not a fixed offset.
- **Served identity, 2026-09-28 UTC.** The consumer's market clock routes its `SIX` and `SIX_CENTRE` sets to this venue, so the row is **served** and reviewed monthly per LAW-WATCH; the holiday windows are complete for 2012-2027, the 2010-2011 span is the recorded gap with its closing condition (#212; the gap's 2018-2019 half closed as data on 2026-10-03 UTC through the 28 May 2018 guide's own year grids) (LAW-SERVICE-TIERS, LAW-FOLLOW-UPS-ARE-ISSUES).

## Module narrative (moved from src/calendar/schedules/equities/europe/six.rs on 2026-10-01 UTC)

Pre-Opening and Post Trading are order-entry-only. The Trading Guide's
trading-period overview runs Pre-Opening from 06:00 "until Opening" and
permits no immediate-execution time in force in it (Immediate or Cancel and
Fill or Kill are "No" for both Pre-Opening and Post Trading); an At-the-
Opening order entered during Pre-Opening only executes in the Opening
Auction that follows. Directive 1 likewise separates pre-opening and
post-trading from the trading phases of the exchange day.
https://www.six-group.com/dam/download/the-swiss-stock-exchange/trading/trading-provisions/regulation/trading-guides/trading-guide.pdf
https://web.archive.org/web/20081123115341id_/http://www.six-swiss-exchange.com/download/trading/regulation/directives/swx_dir01_en.pdf

---

SIX Swiss Exchange — shares segments (Blue Chip / Mid-/Small-Cap), which is
what `Exchange::Six` denotes. SIX does NOT follow the Xetra pattern:
continuous trading ends at 17:20, the closing auction starts at 17:20 and
can uncross as late as 17:32, and Trading-At-Last then runs to 17:40,
followed by order-entry-only post-trading through 22:00. The 17:30–17:35
auction belongs to the ETF/ETP/Sponsored Funds segments only, which have no
TAL.

The Trading Guide's 09:00 opening is randomized over two minutes. The
deterministic profile therefore keeps the auction/pre-opening classification
through 09:01:59 and starts regular trading at the latest possible edge,
09:02. Within that stretch the guide's own phase boundary applies: 06:00
until 09:00 is Pre-Opening (`order_entry`), 09:00-09:02 is the Opening
auction (`extended`, because its uncross prints). Its segment row is "Blue Chip Shares 06:00 09:00 17:20 17:30 17:30
17:40 22:00"; the current page confirms the two-minute opening slot.

Trading Guide, Blue Chip Shares: "Trading Hours 09:00 - 17:30 CET /
Continuous Trading 09:00 - 17:20 CET / Closing Auction 17:20 - 17:30 CET /
Trading-At-Last Start: 17:30 - 17:32 CET End: 17:40 CET".
Sources: SIX Group, "Trading hours"
(https://www.six-group.com/en/products-services/the-swiss-stock-exchange/trading/trading-provisions/trading-hours.html)
and the SIX Swiss Exchange Trading Guide
(https://www.six-group.com/dam/download/the-swiss-stock-exchange/trading/trading-provisions/regulation/trading-guides/trading-guide.pdf).
SIX's official guide valid from 2018-05-28 records the same Blue Chip grid,
including the two-minute randomized opening and closing auction windows.
https://www.six-group.com/dam/download/sites/education/preparatory-documentation/trading-module/trading-guide.pdf

The January-2010 baseline is independently established by operator archives.
Directive 1, effective 2007-09-07, gives exchange hours 06:00-22:00,
pre-opening from 06:00 until the opening, and post-trading from the close
through 22:00. The Equity Market Product Guide valid from 2009-07-22 gives
the exact shares grid used below: continuous 09:00-17:20, closing auction
17:20-17:30, and two-minute randomized opening and closing windows ending at
09:02 and 17:32 respectively.
https://web.archive.org/web/20081123115341id_/http://www.six-swiss-exchange.com/download/trading/regulation/directives/swx_dir01_en.pdf
https://web.archive.org/web/20090824132532id_/http://www.six-swiss-exchange.com:80/download/marketpulse/news/newsboard/product_guides/product_guide_equities_en.pdf

SIX labels its times "CET" year-round; they are local Zurich wall-clock, so
`Europe::Zurich` (CET/CEST) is the correct zone, not a fixed offset.
