<!-- SPDX-License-Identifier: MIT-0 -->

# Changelog

## [Unreleased]

### Added

- `ExchangeCalendar::with_last_known_week`: a caller-selected view for a consumer
  that must keep operating without a sourced answer. Wherever a default calendar
  would refuse a date at or above the support floor, it reads the weekly profile
  the identity knows there instead: above the last audited holiday window, every
  date of an identity with no table, withheld dates and carried eras. Audited rows
  keep their arrangement, including inside a search that crosses the last window.
  `coverage_on` reports each date it reads this way as
  `DateCoverage::NormalWeekOnly`, so a consumer can disclose it or decline to infer
  a closure from it. The default calendar's answers are unchanged.
- `ExchangeCalendar::with_next_known_week`: the forward-looking companion, for a
  consumer replaying history such as a backtest. A date below the support floor or
  below the identity's sourced normal week reads the next week the identity knows
  instead of refusing. Inside coverage and after it the view changes nothing; with
  both views no date refuses.

## [1.2.4] - 2026-11-01

### Fixed

- Replace NSE's withheld 8 November 2026 Muhurat date with its published
  capital-market session, including pre-open order entry, the CAS/non-CAS
  venue union and post-close trading until 16:00 IST.
- Preserve ICE softs' shortened Christmas Eve post-close queues and Cotton's
  published 28 December matching session through complete trade-date
  replacements, including the dependent 29 December queue arrangement.
- Model CME cryptocurrency October maintenance through complete trade-date
  exceptions with exact Saturday Pre-Open queues; detached snapshots keep
  their ordinary-week contract.

### Added

- Xetra's published 30 December 2026 replacement session: matching through
  14:10 CET and post-trading order entry through 15:00 CET.

## [1.2.3] - 2026-10-24

### Added

- Encode the 2026 ICE Futures U.S. Christmas notice of October 22, 2026: the
  softs families (Coffee, Cocoa, Cotton, FCOJ) close early at 13:05 New York
  on 2026-12-24 while Sugar 11 and 16 keep regular hours, the NYSE-stock-index
  and U.S. Dollar Index families close early at 13:15, and Coffee, Cocoa,
  Cotton and Sugar 11 open late at 08:00 on 2026-12-28 while FCOJ, Sugar 16
  and the index families trade regular hours; the `iceus` venue ships
  `Unsourced` on both dates, whose arrangements its families dispute. The
  notice also resolves the index families' and the venue's former
  2026-12-28 withholdings.
- Add NSE India's 2026-01-15 closure (Municipal Corporation Election,
  Maharashtra), keyed from the operator's holiday-master feed at T2, which
  the 2025-12-18 holiday-list banner predates; the crate previously answered
  the date as an ordinary Thursday.

## [1.2.2] - 2026-10-23

### Fixed

- Date the `oanda_fx`, `oanda_metals` and OANDA venue-clock normal-week
  changeover to opening day 2024-02-11 New York. Operator candle captures
  support the earlier grids through opening day 2024-02-08 and the current
  grids from the Sunday-opened session's Monday close; Friday and Saturday
  open no sessions, so the former intersection span is retired. The four
  nested-grid books and Singapore 30 retain their undated page-print bounds.
  Singapore 30's legacy night-close conflict is disclosed beside its served
  intersection. Holiday windows, typed coverage refusals and the remaining
  historical gaps in #332 are unchanged.
- Correct the CME Event Contracts on Bitcoin Futures five-day session to its
  own 15:00 CT close, including the May 29, 2026 transition session; market
  data confirms the tradable event contracts' close separately from the
  non-trading reference instruments' 16:00 CT clock (#360).
- Apply the published October 3, 24 and 31, 2026 maintenance extensions to
  the Bitcoin event-contract snapshot, including the 15:30 CT FIA-drill
  reopen on October 24 (#360).
- Retain cryptocurrency's sourced September 19 and October 3, 2026 Saturday
  Pre-Open queues at 07:45 and 04:45 CT. Order acceptance now answers inside
  them; matching and trading-bar starts remain at 08:00 and 05:00 CT (#360).
- Align the gold, silver and copper TAS current snapshots with their dated
  Sunday 16:00 CT Pre-Open and correct the four CME venue holiday-table
  header counts, fenced against the public rows (#360).

- Store built-in holiday coverage windows as the validated date pairs the
  `holidays!` macro constructs, so `HolidayCoverage::windows` and
  `HolidayCoverage::contains` answer from real dates with no endpoint
  conversion. An invalid window endpoint now fails the build where each
  window is constructed, the same const fence a row's trade date passes
  through, and the former outer-coverage substitution and `false`-on-invalid
  fallback branches are gone (#365). Every audited window and every public
  coverage result is unchanged.
- Repair the fifteen private-item rustdoc diagnostics: unresolved intra-doc
  links now resolve in their real scopes and the redundant explicit link
  targets are removed, documentation bytes only (#366).

## [1.2.1] - 2026-10-16

### Added

- `globex_nikkei_225_dollar`'s sixteen withheld holiday dates (2022-2024) ship as
  market-data-confirmed rows: twelve-hour early closes at 12:00 CT on nine dates,
  12:15 CT final closes on 2024-11-29 and 2024-12-24, the 08:15 CT Good Friday
  2023 close, full closures on 2024-03-29 and 2024-12-25, with 2024-07-03 and
  2024-12-31 audited normal — holiday classification now answers instead of
  refusing on every date in the table's windows, and the served Globex surface
  carries no `Unsourced` row anywhere.
- The metals TAS keys (`globex_gold_tas`, `globex_silver_tas`,
  `globex_copper_tas`) serve the witnessed 16:00:00 CT Sunday Pre-Open onset
  from 2016-01-03, moving their #79 quarter-hour residual to
  2012-04-15..2016-01-02 — the same witness that moved the four standard
  families, recorded for the PGM keys as confirmation of the weekday queue and
  a machine-channel silence on the wiki's Sunday half.
- Market-data records close six further recorded residuals: the platinum TAS
  carried-back 17:00 CT open is witnessed from 2018; the spot-quoted family's
  unsampleable interior is witnessed unchanged at four weeks; the event-contract
  grid is witnessed current through the archive's end (2026-04-24); the mini
  grains era's queue onsets are witnessed from 2016-01-03 (morning 08:00, PCP
  14:30 — ten minutes earlier than the advisory's 14:40, recorded — evening
  16:45); Nikkei's five 2025 merged trade dates are re-witnessed against the
  contract's own sessions; and the cryptocurrency 2025 closure-evening queues
  and full closures are witnessed as shipped. The weather-options Globex
  migration gains an extended negative: the archive's instrument universe
  carries no weather-option records, so the bracket keeps its dated-artifact
  closer.
- The four standard CME families' Sunday Pre-Open serves the witnessed 16:00:00 CT
  onset from 2016-01-03: the market-data sweep witnessed every Sunday of
  2016-01-03..2026-04-05 opening the queue at exactly 16:00 CT, so the knowledge-
  bound row moves down from its 2026-08-22 review date and the disputed
  16:00-16:15 CT quarter-hour shrinks to the Sundays of 2012-05-28..2016-01-02
  (`globex_equity_index`, `globex_energy`, `globex_fx`, `globex_interest_rates`,
  the three CME venues, and `globex_silver_100oz` by reference).
- `globex_cryptocurrency`'s first holiday window opens at the 2017-12-17 launch:
  the launch-to-2018 span was audited day by day on market data, shipping
  thirteen rows (four closures, six 12:00 CT early closes, three 12:15 CT final
  closes — including 2018-07-03, which the day-by-day audit itself surfaced),
  and the five-day era now serves its witnessed Sunday 16:00-17:00 CT and
  Monday-Thursday 16:45-17:00 CT pre-open queues from the launch, closing the
  #123 residual; the pre-2019 holiday-classification refusals are gone and the
  below-first bridged span shrinks to the pre-launch closure.
- The livestock published-table gaps closed on market-data confirmation: the
  14:30-16:00 CT post-close queue ran across the whole November 2016..March
  2020 capture omission, and the 06:00-08:30 CT morning queue opened on every
  sampled weekday of its carried span from 2016-03-02. The gold, silver and
  copper TAS carried closes, the platinum and palladium TAS carried windows,
  and the event-contract family's carried 16:00 CT daily close are likewise
  witnessed constant across their recorded gaps, closing six dormant recorded
  gaps as `MARKET-DATA` evidence records.
- A markdown lint gate: `markdownlint-cli2`, pinned at 0.23.3, configured in
  `.markdownlint-cli2.jsonc` with each rule deviation's reason and enforced by
  a `markdown-lint` CI job; the tracked markdown it flagged is fixed.
- Record the 2026-10-15 (UTC) enumeration re-sweep of the CME 2023-01-16
  (Martin Luther King Jr. Day) gap in the seven served Globex family evidence
  files, and close the marker under the maintainer's direction: the sweep
  enumerated whole Wayback directories instead of guessing filenames — no
  Globex-hours MLK-2023 document exists under any filename — and the four MLK
  artifacts the operator's old holiday directory holds state no session hours.
  The 2023-01-16 arrangements are then confirmed with market data (the exact
  session instants, retained privately) and encoded: 12:00 CT early closes in
  `globex_equity_index`, `globex_interest_rates` and `globex_nikkei_225_dollar`,
  a 13:30 CT early close in `globex_energy`, full closures in `globex_grains`
  and `globex_livestock`, and audited-normal Mondays in `globex_fx` and
  `globex_cryptocurrency` — with the 2023-01-17 prior-evening-leg gap
  confirmed run beside them. The four CME venue tables regenerate to the
  profile-clock rule with no inherited marker left, and the venue evidence
  files record the recomputed audit. Every confirming artifact is retained
  privately and cited only as `MARKET-DATA`.

## [1.2.0] - 2026-10-14

### Added

- The 2026-10-31 Saturday maintenance-window extension for the 24/7 markets
  (CME Globex notice 20261005: the window runs 02:00-05:00 CT that Saturday and
  reverts to the 02:00-04:00 standard, November 7 reopening at 04:00) now ships
  as forward-dated one-day rows in the two families the October 3 and October 24
  extensions (notice 20260921) already govern: `globex_cryptocurrency` and
  `globex_silver_100oz`. The three-hour October 31 window classifies
  `Maintenance`; the dormant `globex_event_contracts_btc` records the
  class-wide extension beside its evidence rather than encoding it, per its
  dormant tier. Awaiting the operator's confirmation on the day (LAW-WATCH).
- LSE holiday-table extension through the operator's published 2028 rows and
  2029-01-01: the audited window moves from 2010-01-01..2027-12-31 to
  2010-01-01..2029-01-01 with eleven new rows — 2028-01-03, 2028-04-14,
  2028-04-17, 2028-05-01, 2028-05-29, 2028-08-28, 2028-12-25 and 2028-12-26
  closed, 2028-12-22 and 2028-12-29 half days closing at 12:30 London time,
  and 2029-01-01 closed — read from the operator's own business-days table,
  which the 2026-10-13 monitoring sweep re-read live byte-identically (2026
  and 2027 rows unchanged).
- Two served product-family keys. `eurex_smi` serves Eurex SMI® index
  futures (`FSMI`) on the Zurich clock: continuous trading 07:50-17:27 CET
  from the January-2010 floor until the operator's Contract Specifications
  amendment of 2010-07-05 extended it to 07:50-22:00 (a dated T1 cutover,
  re-witnessed by the Trading Calendar editions 2011-2026), with Pre-Trading
  and Post-Trading order-entry phases and a built-in holiday table (2010-2026)
  of the all-derivatives rows plus the Swiss-scope rows the operator's
  editions key to the SMI scope. `euronext_index_futures` serves Euronext CAC
  40 (`FCE`) and AEX (`FTI`) index futures on their shared Central Order Book
  grid, 08:00-22:00 CET/CEST Monday to Friday, with a built-in holiday table
  (2010-2026) read from the operator's shared cash-and-derivatives calendars.
  Both ship with evidence files, ledger rows (served, monthly cadence) and
  public-surface tests.
- Two new `MarketHoursKey` families, both served under the maintainer's OANDA
  programme with monthly review cadences and built-in holiday tables.
  `bme_ibex` carries MEFF IBEX 35 index futures (with Mini and Micro IBEX 35)
  in Europe/Madrid: the 2010 floor grid 09:00-17:35 with an 08:30-09:00
  opening auction, the operator-dated 2017-01-23 press-release move to
  08:00-20:00 (auction 07:55-08:00), and notice 16/2025's unconditional
  2025-06-23 close extension to 22:00 — notice 7/2024's CNMV-conditional
  2024-07-01 date never keyed a row and its slip is recorded. Holiday tables
  audit 2010-2011 and 2014-2026 (90 rows: closures, 14:00 Christmas-eve and
  New-Year-eve early closes, 2021's two full closures), with the 2012-2013
  grids and the 2010-2016 interim hours disclosed as gaps.
  `ice_europe_gilts` carries ICE Futures Europe Long and Medium Gilt futures
  in Europe/London: the shared 08:00-18:00 grid with a 06:03-08:00 pre-open,
  and circular 26/063's unconditional 2026-07-06 extension of Long and Medium
  alone to 01:00-21:00 with a 00:45-01:00 pre-open (Short and Ultra Long Gilt
  stay 08:00-18:00 outside the key). Holiday tables audit six operator-era
  windows (141 rows across the LIFFE notices 2010-2014, the IFEU Migrated
  editions 2015-2017 and 2021-2026 and the exchange-wide Trading Schedule
  editions 2019-2020, including 12:15 and 13:00 Christmas-eve closes), with
  2018, the 2022/2023 mid-year months and the unwitnessed 2019-2020
  Migrated-edition layer disclosed as gaps, and the announced but untimed
  Christmas eves of 2014, 2015, 2021, 2024 and 2026 shipping `Unsourced`.
- Two served product-family keys, each with its built-in holiday table and a
  served ledger row: `asx_spi_200` (ASX SPI 200 index futures on the Sydney
  ASX 24 grid — day session 09:50-16:30, night session 17:10 to a 07:00/08:00
  close selected by the operator's own US-daylight columns, ten-minute
  pre-opens, holiday years 2010-2027) and `hkex_index_futures` (HKEX Hang
  Seng index futures — the dated 2011 and 2012 trading-hours phases, the
  2013-04-08 T+1 launch per DMD/022/13, three bracketed-undated after-hours
  cutovers served as knowledge bounds, holiday years 2010-2027 with the
  half-day eves and the witnessed UK/US AHT suspensions).

### Fixed

- Correct the `ice_us_sugar` evidence record. The operator's own "Sugar No. 11
  / No. 16" brochure (document date 22 March 2010) prints the carried
  `HOURS  0330 EST TO 1400EST` spec row inside the previously unwitnessed
  2010-2011 span, and the operator's Chinese Sugar introduction (printed
  12 December 2011) re-witnesses the same grid — superseding the evidence
  file's, ledger basis's and module comment's earlier claim that the interval
  could not be sourced and the August-2011 carry-back was the terminal answer.
  No served hour moves: the witnessed grid equals the baseline that already
  shipped, so this is an evidence correction, not a schedule change. The same
  wave's dated 2011-05-02 late open at 07:30 NY (Exchange Notice ExNot 041111)
  is recorded in the evidence file as witnessed pre-window data rather than
  encoded as a row, because the family's audited holiday window starts
  2025-01-01 and a lone 2011 row would assert audited normal for every other
  date of an extended window.
- Seven OANDA books now answer their witnessed normal-week history instead of
  refusing everything below the 2024-02-21 page print: `oanda_fx` and the
  venue default from 2021-12-08, `oanda_metals` from 2021-12-08,
  `oanda_us_index_cfd` from 2021-08-04, `oanda_us_bond_cfd` and
  `oanda_us_commodity_cfd` from 2021-03-03, and `oanda_china50_cfd` with
  `oanda_singapore30_cfd` from 2021-03-03. The undated changeovers between
  the operator's last earlier-grid print (2023-06-05) and first current-grid
  print (2024-02-21) serve the two witnessed grids' sourced intersection per
  the charter's 2026-10-04 rule, with each disputed remainder disclosed in
  its evidence file; dates in the witnessed spans sit below the first audited
  holiday windows, so their holiday classification refuses typed while
  session questions answer. `oanda_brent` keeps its carried span: its
  New-York-anchored legacy grid and London dagger grid are not nested, so no
  single static grid holds under both states.

## [1.1.0] - 2026-10-12

### Fixed

- OANDA TRY-pair date-aware queries now answer the witnessed current grid within
  the operator's Forex holiday audit, 2026-10-10 through 2026-10-30. Earlier
  history and unpublished later dates retain explicit coverage errors.

- Correct the Brent evidence comparison to ICE's printed trading hours;
  its settlement calculation is not a session close. OANDA rules are unchanged.

- BME opening and closing auction calls now accept orders without reporting
  the market open, on normal days and half days. The sourced uncross windows
  and Trading-at-Last remain executable.
- LSE's five withheld 2025 spring/summer holiday dates now answer as sourced
  closures, recovered from a verbatim public mirror of the operator table.
- The US-index key's exact holiday scope is restricted to its four US
  indices. Japan 225's two listings share the normal clock but require
  independent holiday-scope evidence before exact consumer routing (#332).
- The current OANDA `***` instruction follows each row's named local zone:
  Sydney for Australia 200's marked close and Frankfurt for the German,
  Euro 50 and Bund marked openings. The older guide's US-season Australian
  selector belongs to its older grid; its historical change date stays
  unsourced (#332).
- Carter's Day 2025 index close resolves to 09:30 ET from the UK page's
  dated 24-hour context and stated 18:00 reopening. The following trade
  date preserves that literal 18:00 open; Memorial's incomplete minute
  and the remaining historical/scope gaps stay withheld (#332).
- OANDA holiday instants now preserve the operator's exact minutes: the
  missing MLK 2025 closes, New Year 18:04 reopens, 2025 US 18:00 and Brent
  20:00 ET reopens, and Holy Thursday's 17:00 ET close (#330).
- Bond CFDs retain ordinary trading on the index-only Independence Eve
  notice. The incomplete Memorial Day 2026 reopen refuses the following
  index/bond trade date until the operator clarifies it (#332).
- Brent's Monday session bounds include the Sunday 23:01 London opening
  (#331). Replacement-session searches select the nearest chronological
  occurrence across normal rules and replacement blocks (#329).

- The OANDA order-acceptance scope is explicit: off-hours pending-order
  storage is a broker facility outside the published session phases (#333).

### Added

- Nasdaq Copenhagen, Helsinki and Stockholm cash holiday tables for 2026–2027, including Stockholm's half-day phases through 13:30 local time.
- Broker-native OANDA China A50 and Singapore 30 CFD keys with sourced
  Singapore day/night blocks and a zero-exception 2026-10-10..2026-10-30
  holiday window. Earlier normal-week gaps, unaudited holiday classification
  and later unpublished dates remain explicit; the sourced-intersection bridge
  may answer pre-window session questions.
- Euronext Brussels and Lisbon cash holidays for 2026: five sourced closures
  and two withheld year-end half days per market whose hours remain unpublished.
  Earlier holiday history and 2027 are outside these audits.

- Archived OANDA UK FX and spot-metals holidays from 2024-02-21 through
  2024-12-31: Christmas FX closure and fourteen literal metals arrangements,
  including Thanksgiving and Christmas reopens. Earlier normal-week history
  remains unresolved under #332.
- Archived OANDA UK US-index and Treasury-CFD holiday arrangements for 2024,
  preserving literal reopens and retaining the omitted Treasury Christmas Eve
  arrangement as a typed gap.
- Archived OANDA UK commodity and Brent holiday arrangements from
  2024-02-21 through 2024-12-31, preserving the separate group scopes and
  retaining unpublished Brent reopenings and special-day hours as typed gaps.
- Euronext Amsterdam's own 2010-2026 holiday table: 108 rows, with the
  two 2026 December half-day instants explicitly withheld until publication.
- BME principal-share cash holidays from 2010 through 2026, with complete
  half-day phase replacements and typed gaps for unpublished 2026 year-end
  timetables.
- `Exchange::Oanda` and the first two OANDA-book keys, `oanda_fx` (all FX
  products except TRY pairs: Sunday 17:05 to Friday 16:59 New York time with
  the six-minute 16:59–17:05 daily break) and `oanda_try_fx` (a daily
  Monday–Friday 08:00–17:00 Central European session with an overnight gap).
  The books trade straight through the daily maintenance halts of the futures
  clocks they hedge on, so they are book keys, not futures mappings. The
  `oanda_fx` holiday table ships four sourced trade-date closures
  (2024-12-25, 2025-01-01, 2025-12-25, 2026-01-01) over its
  2024-02-21..2026-10-30 audited window, and the venue's table mirrors it —
  the first wave of the OANDA-listed market coverage programme
  (SharurPlatform#1046, oanda-rs#30).
- Four additional OANDA-book keys for the UK hours page's spot metals,
  US-anchored CFDs and Brent listings:
  `oanda_metals` (XAU/XAG and crosses, Sunday 18:05 to Friday 16:59 New York
  time), `oanda_us_index_cfd` and `oanda_us_commodity_cfd` (four US indices and five
  US energy/commodity-metals listings on one continuous clock — Chicago
  17:01 to 15:59, the same instants as New York 18:01 to 16:59 — split into
  two keys because the operator's holiday page gives different early closes), and
  `oanda_brent` (London-anchored by the page's own dagger note, Sunday 23:01
  to Friday 21:59 with its two stated breaks). The broker-family holiday tables include archived 2024 arrangements
  and current published rows through 2026-10-30, within each key's documented
  audited windows; unpublished arrangements retain typed withholdings.

- `oanda_us_bond_cfd` separates the four Treasury listings from the index
  book. Good Friday 2026 closes bonds at 11:14 ET and indices at 09:14 ET;
  Carter's Day 2025 closes bonds at 13:15 ET. Historical OANDA gaps remain
  disclosed in #332 under the dated current-operation activation scope.

### Changed

- Activate the three previously admitted Eurex/IFUS keys plus nine reached
  OANDA book keys and seven reached cash-equity scopes. Holiday-bearing served
  scopes receive monthly review. The ledger now records 52 served and 91 dormant
  rows; the unchanged 2025 inventory sample records 34 complete, 10 incomplete
  and eight without 2025 holiday coverage. The dated OANDA release-scope
  exception prioritizes current/published future operation while retaining
  historical gaps in the single issue #332 and preserving typed coverage errors.

## [1.0.2] - 2026-10-10

### Changed

- The four CME venue holiday tables (`cme`, `cbot`, `comex`, `nymex`) now
  carry their profile clock's rows verbatim under the 2026-10-09 charter
  amendment completing the profile-clock rule (#153, #242): `cbot` mirrors
  `globex_grains`, `cme` mirrors `globex_equity_index`, and `comex`/`nymex`
  mirror `globex_energy`. Every holiday date a venue's clock answers now
  states its actual arrangement — full close, early close, late open or
  replacement day — where the earlier intersection rule withheld it as
  `Unsourced` because two routed families print different instants; the
  dissenting family's row stays in its own family table and the venue
  evidence files name the disagreements. CBOT's Thanksgiving now answers
  closed instead of refusing as unsourced.

### Fixed

- The ICE Futures U.S. holiday notices the earlier waves recorded as
  unretrieved or nonexistent were recovered on 2026-10-09 UTC through the
  operator's own notices listing service (the machine channel behind the
  client-side notices page, found by reading the page's JS bundle): the 2025
  Independence Day notice, the 2025 Christmas notice, the 2026 MLK notice and
  the 2026 Presidents Day notice. The 2025 dates now state their instants —
  FANG+ closes at 13:15 NY on 2025-07-03 and 2025-12-24 and 13:00 NY on
  2025-07-04, the dollar index at 13:00 on 2025-07-04 and 13:45 on
  2025-12-24, and 2025-12-26 is audited normal for both index families per
  the Christmas notice's `Regular Hours`; Cotton gains rows for its own
  2025-07-07 late open at 08:00 NY, the 2025-12-24 early close at 13:05 NY
  and the 2025-12-26 late open at 07:30 NY, Coffee, Cocoa and Sugar 11 gain
  the same Christmas rows, and FCOJ gains the 2025-12-24 early close. The
  2026 MLK and Presidents Day notices (issued December 2025 — the earlier
  "no such notice exists" record was an archive-channel failure) give FANG+
  the 13:00 NY early close on 2026-01-19 and 2026-02-16 and leave the dollar
  index regular both days. The `iceus` venue gains one `Unsourced` row
  (2025-07-07, Cotton's late open) and its counts move to 49 rows / 42
  withheld; 2026-12-28 stays withheld, no 2026 Christmas notice having
  issued at the re-check (#168).
- The `cfe` 2017-07-03 withholding closed as data on 2026-10-10 UTC: the
  controlling circular CFEIC17-022 ("Modified Trading Hours for Independence
  Day Holiday", June 16, 2017) was recovered from Common Crawl CC-MAIN-2017-34
  after the `cfecirculars.com` channel proved dead (NXDOMAIN, zero Wayback and
  zero Common Crawl captures) and the operator's own circulars index in the
  Wayback Machine named it; it states `Trading in all CFE products will close
  at 12:15 p.m. on Monday, July 3, 2017`, so the date ships `early close
  12:15 CT` and the `cfe` audited windows ship zero `Unsourced` dates.
- The `lse` five-date 2025 gap and the `cme` 2023-01-16 marker survived their
  next channels, recorded negative: the LSEG business-days page's only
  2024-2025 captures (2024-05-19, 2025-08-07) are client-side shells stating
  no dates, and the CME mirror sweep found no holiday or trading-hours page
  anywhere in the archived `cmegroup.cn`, a parked `cmegroup.fr`, empty
  `education`/`advisories` paths, and an HTTP 500 from the one further
  archived trading-hours-service capture querying a January 2023 window.
- The ICE monitoring entry point moved: `ice.com/trading-hours` 404s live and
  the section navigation now names `ice.com/holiday-hours`, whose IFUS entry
  re-fetched byte-identical to the held calendar (verified 2026-10-10 UTC).

- The `nasdaq` venue's four withheld holiday dates closed as data on the
  operator's own Equity Trader Alerts (retrieved 2026-10-09 UTC): 2010-11-26
  and 2011-11-25 now ship `early close 13:00` keyed to alerts 2010-73 and
  2011-54, whose Early Closing Schedules print the Nasdaq day session and
  closing cross at 1:00 p.m. (both alerts still serve live from the
  operator's Trader News pages); 2012-10-30 ships `closed` keyed to alert
  2012-45, which states the Hurricane Sandy Tuesday closure unconditionally;
  and 2025-01-09 ships `closed` keyed to alert 2025-1 (Wayback replay of
  capture 20250122151135), which states the National Day of Mourning closure.
  No date inside the `nasdaq` audited window is withheld any more, so the
  identity reads complete to 2026-12-31.
- The `nse_india` Muhurat hunt's 2026-10-09 UTC re-check is recorded:
  completed apex-host Wayback CDX sweeps (the prior pass's two apex files
  were error pages), the per-year issuance windows 2011/2012-2014/2017/2019
  negative again, the `/content/cmtr/` path family empty, and the operator's
  own live circulars channel listing 414 CM circulars 2026-09-22..2026-10-09
  with no Muhurat item — the seven withheld Muhurat dates stay withheld and
  the 2026-11-08 watch is re-dated. Ledger rows re-reviewed 2026-10-09.

- The globex futures families' remaining not-worked-up holiday markers are
  resolved where the operator's own documents state the arrangement: the
  three Juneteenth `Unsourced` markers (2019, 2020 and 2021) are deleted in
  every family — CME's own consolidated annual bundles name no Juneteenth
  arrangement in those years, so the dates are audited normal — and
  2023-02-20 and 2023-04-07 are worked up at T1 from CME's own summary
  sheets `files/presidents-day.pdf` and `files/good-friday.pdf` for
  `globex_energy`, `globex_equity_index`, `globex_livestock`,
  `globex_cryptocurrency` (Good Friday only; its Presidents' Day sheet
  prints the ordinary close) and `globex_fx` (same shape). 2023-01-16 stays
  a recorded gap in every family (no operator document states it; channels
  re-enumerated) and `globex_nikkei_225_dollar` keeps its 2022-05-30 and
  thirteen 2024 markers (no `THBP-B` capture exists in the archive — swept
  2026-10-09). Ledger rows re-reviewed 2026-10-09.

## 1.0.1 — 2026-10-08

### Removed

- The crate archive no longer ships the repository's one-off Python
  verification tooling under `tools/`. The package carries the library, its
  evidence, and the maintenance documents only.

## 1.0.0 — 2026-10-08

Initial release.
