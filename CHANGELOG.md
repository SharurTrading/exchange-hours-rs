<!-- SPDX-License-Identifier: MIT-0 -->

# Changelog

All notable changes to `exchange-hours` are documented in this file, starting
from the first tagged release. The format is based on
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and the crate adheres
to [Semantic Versioning](https://semver.org/spec/v2.0.0.html). Session-data
corrections (a venue's hours fixed against a primary source) go under
**Fixed**; new venues and new API surface under **Unreleased**/**Added**.

## [Unreleased]

## [1.0.0] - 2026-08-22

First stable release. Version 1.0 establishes the canonical string identities,
normal-week schedule scope, date-aware calendar surface, and primary-source
maintenance contract described in the README and schedule verification ledger.

### Added

- **23 major cash-equity venues:** ASX, TMX Australia, NZX, TSE, NSE India,
  BSE India, HKEX, SGX Securities, Bursa Malaysia, SET Thailand, IDX, PSE,
  HOSE, SSE, SZSE, KRX, TWSE, Borsa Istanbul, TSX, JSE, Tadawul, B3, and BMV.
  Together with the US additions below and pre-v1 IQX cleanup, this brings the crate to 93
  source-backed market identities plus the synthetic `Exchange::Unknown`
  fallback (94 `Exchange` variants total).
- **Three live US national exchanges:** LTSE (`ltse`), 24X National Exchange
  (`24x`), and Texas Stock Exchange (`txse`), each with its primary-sourced
  production launch boundary, current accepted-order envelope, regional bulk
  membership, and stable canonical identity. Conditional 24X overnight hours
  remain on the watch list rather than activating early.
- **The additive `ExchangeCalendar` API** for date-aware predicates, session
  scans, maintenance/all-day queries, and candle boundaries. Existing
  `MarketHours` functions and signatures remain unchanged; B3/BMV calendars
  reselect their published grid on every candidate trading day.
- **Date-aware product-family calendars.** `calendar_for_market_hours_key`
  gives every `MarketHoursKey` the complete calendar query surface and
  reselects sourced revisions for each candidate opening day. `CalendarSource`,
  `ExchangeCalendar::source`, `exchange`, and `market_hours_key` preserve
  whether a calendar represents a venue or a product family.
- **Three primary-sourced CME Group family keys:**
  `GlobexInterestRates` (`globex_interest_rates`) for CBOT Treasuries/Fed Funds
  and CME SOFR, `GlobexLivestock` (`globex_livestock`) for LE/GF/HE, and
  `GlobexCryptocurrency` (`globex_cryptocurrency`) for CME non-spot-quoted
  cryptocurrency futures. Each owns its dated family history rather than
  borrowing another product family's clock. There are now 12 keys total: 11
  operator-derived families plus synthetic `AlwaysOpen`.
- **Caller-supplied trade-date overrides.** `DayPolicy` and
  `PolicyCalendar` apply closed dates, early final closes, and late first opens
  to all status, session, candle, and closed-day scans without bundling holiday
  data or modifying `hours_at`. `StaticDayPolicy` and `DayOverride` add a
  validated, allocation-free hard-coded record format; `NoPolicy` preserves
  normal-week behavior. Complex holiday phase replacements remain explicitly
  outside this scalar boundary API rather than being approximated.
- **Trade-date and one-shot state queries.** `trade_date` and
  `is_closed_trade_date` are available on date-aware and policy-aware
  calendars. Profiles without a trade-date concept return no trade date and
  report every trade date closed because no session can be assigned to one;
  their civil-day availability remains available separately. `SessionState`
  distinguishes regular, extended, halt, maintenance, and closed states;
  `is_maintenance` is exactly its maintenance case.
- **A Criterion query benchmark** covering `is_open`, `session_bounds`, daily
  `candle_end`, `trade_date`, and closed-gap `session_state` for
  `GlobexEquityIndex`, plus documented allocation and value/thread-trait
  performance contracts.
- **`hours_for_market_hours_key_as_of`** for primary-sourced point-in-time
  futures product-family snapshots, reusing the same dated tables as the
  corresponding exchange profiles.
- **A complete `MarketHoursKey` identity surface:** `ALL`, `as_str`, `Display`,
  `FromStr`, `Ord`, and `PartialOrd` share the canonical string table used by Serde;
  `ParseMarketHoursKeyError` preserves rejected input for callers.
- **Point-in-time APAC and global-equity schedules back to the January 2010
  audit floor.** Every encoded cutover has a primary-source, day-level
  effective date; temporary pandemic schedules and PSE's two full closure
  dates are preserved. B3 preserves every explicit grid back to January 2010
  before switching to its modern cross-zone rule; BMV preserves its exact 2010
  spring exception before applying the operator's prospective New York-alignment
  policy from November 2010.
- **Regional bulk builders** for Asia-Pacific and other major global equities:
  `hours_for_apac_equities`, `hours_map_apac_equities`,
  `hours_for_global_equities`, and `hours_map_global_equities`.
- **Schedule-maintenance documentation:** the README now publishes an explicit
  source-review cutoff and machine-checked assurance counts, while a dated
  audit report, per-exchange verification ledger, normalized official-source
  registry, and repeatable update guide make freshness, scope, and evidence
  status auditable without overstating future accuracy.

### Changed

- **Breaking: `ExchangeCalendar::exchange()` now returns `Option<Exchange>`.**
  Venue calendars return `Some(exchange)` and product-family calendars return
  `None`. Callers that accept both identity kinds should match on
  `ExchangeCalendar::source()`; family-specific callers can use
  `market_hours_key()`.
- **Breaking (behavioral): maintenance uses a four-hour bound and trade-date
  semantics.** A same-trade-date gap is now `Halt`; an inter-trade-date gap is
  `Maintenance` only when its complete elapsed span is at most four hours and
  both dates are in one ISO week. A sourced continuously traded-week profile
  can also retain an operator-designated gap within that bound as maintenance;
  this covers CME cryptocurrency's Saturday window even though trading on both
  sides carries one trade date. Longer gaps such as CBOT grains' afternoon
  closure and policy-created early-close gaps are `Closed`.
- **Product-family selection is explicit.** The crate does not map symbols,
  roots, product codes, or MICs to keys. Venue-keyed compatibility defaults are
  documented, and callers are warned not to use them for products outside the
  named family. Nine unmodeled families—including CME NKD and the requested
  ICE/Eurex/SGX expansions—remain rejected rather than receiving guessed
  substitutes.
- **Equal `SessionRule` endpoints now mean a complete local-day session.** A
  rule such as Sunday `18:00→18:00` preserves one continuous session through
  Monday 18:00, including exact `session_bounds`; absence is represented by
  omitting the rule. This intentionally breaking 1.0-boundary correction removes
  `SessionRuleError::EmptyInterval`.
- **Canonical string Serde for `Exchange` and `MarketHoursKey`.** Both public
  identity enums now use their stable `snake_case` strings in every format.
  This replaces derive-generated enum ordinals in non-self-describing formats,
  so a future variant insertion or removal cannot silently decode as another
  venue or product family. JSON remains unchanged. Binary payloads written by
  earlier crate versions require a one-time migration to the canonical string
  representation.

### Removed

- **`Exchange::IntelligentcrossIqx` and the `intelligentcross_iqx` wire name.**
  Official materials identify IntelligentCross as an ATS and `IQX` as its
  market-data feed, while available primary evidence cannot state the ATS's
  exact first-live day. Removing the unused, misleading pre-v1 identity avoids
  returning today's schedule for dates before the venue existed. Persisted
  `intelligentcross_iqx` values must be removed or migrated by callers. There
  is no replacement `Exchange` variant: applications that still need the ATS
  must keep an external mapping rather than converting it to another venue or
  to `Exchange::Unknown`.
- **Four raw US schedule slices are no longer public:**
  `US_EQUITY_REGULAR`, `US_EQUITY_EXTENDED`, `NYSE_TEXAS_EXTENDED`, and
  `BLUE_OCEAN_EXTENDED` were implementation details that could bypass
  venue/date routing. Use `hours_for_exchange`, `hours_for_exchange_as_of`, or
  `calendar_for_exchange` and the public query APIs instead.

### Fixed

- **Exact CME cryptocurrency weekend semantics.** The key-backed calendar now
  joins adjacent one-midnight storage pieces into the two physical session
  blocks around Saturday maintenance and assigns both to the following open
  business date. Ordinarily that is Monday and the daily bar is Friday
  16:01 Pre-Open→Monday 16:00 CT; when caller policy closes Monday, weekend trading
  remains open, receives Tuesday's trade date, and the bar ends Tuesday at
  16:00. Friday 16:00 remains the final weekly close before the next trade-date
  week enters Pre-Open at 16:01. The weekday 16:01–16:02 and Saturday
  03:45–04:00 order-entry phases are now correctly `Extended`; the preceding
  closed portions remain `Maintenance`. Detached fixed snapshots preserve exact open/closed
  state but retain table-piece bounds because only the key calendar carries
  the family identity required for coalescing, weekly boundaries, and trade
  dates.
- **Civil-time discontinuities in public queries.** A rule occurrence whose
  local open and close both collapse onto the same instant in a DST gap is now
  omitted instead of producing a zero-width session or candle. Partially
  skipped rules resolve to the first real wall-clock second, and a wholly
  skipped civil date is correctly treated as an empty, closed calendar day.
- **Conditional future schedules are held until confirmed.** Announced Nasdaq,
  EDGX, NYSE Arca, MEMX, 24X, and FINRA TRF overnight expansions are monitored
  in the update guide but do not activate from provisional target dates. MX2
  Options, IEX Options, MRX's additional sessions, and GIX are likewise watched
  without premature identities or selectors. Runtime coverage will change only
  after the required readiness, regulatory, clearing, SIP, and production-day
  confirmations.
- **Bursa Malaysia, IDX, HOSE, and BMV January-2010 history.** Bursa's morning
  continuous session now runs through 12:30 under the pre-audit-floor v2 manual
  and its dated successors. IDX now includes its archived 09:10–09:30 pre-open
  before the 2013 expansion. HOSE retains the operator's exact January-2010
  phase table plus its sourced September 2010, March 2012, and July 2013
  revisions. BMV preserves the exact March 2010 early interval and uses the
  operator's prospective New York-alignment rule from November 1, 2010 through
  a date-aware Mexico City/New York offset selector.
- **Venue-union phases across APAC, China, and Mexico.** HKEX now treats its
  executable Extended Morning Session as continuous Regular trading and dates
  CAS from 2016-07-25. SET includes the exact 2025-05-06 DR night launch and
  post-midnight trade-date boundary. TSE/TWSE include ToSTNeT/block tails;
  SSE/SZSE retain 15:00–15:30 block trading from the audit floor; HOSE retains
  every era's put-through tail; IDX restores Negotiated Market through 16:30;
  and BMV preserves its sourced HD/ID `:06`, `:10`, and `:20` transitions.
  Security eligibility may be narrower than these exchange-availability unions.
- **Calendar candle boundaries now honor the profile's close semantics.**
  Always-open profiles no longer receive artificial daily, weekly, or monthly
  closes; adjacent phase handoffs are ignored when the requested session kind
  remains open; and a venue's first launch-day candle start no longer requires
  a pre-launch close.
- **ASX's pre-Service Release 15 staggered opening envelope.** The historical
  profile now keeps the opening-auction phase through the sourced latest Group
  5 transition at 10:09:15 Sydney time while retaining 10:00 as the earliest
  continuous-trading edge. The current post-June 23, 2025 profile is unchanged.
- **Primary-sourced European cash-equity phases and ICE Canada history.** LSE
  SETS now includes pre-trading, conservative randomized opening and midday
  auction edges, and its sourced 2012 closing-price crossing session. SIX and BME now use
  the operators' conservative two-minute and 30-second randomized opening
  edges, respectively; their pre-TAL closing auctions retain the same sourced
  maximum envelopes. Nasdaq Stockholm, Helsinki, and Copenhagen now use the
  market model's five-second randomized opening edge. SIX and BME retain
  their pre-TAL profiles before the sourced June 22, 2020 and December 4, 2023
  launches; Vienna now includes complete 2010/2017/2019/2020 ATX history. LSE,
  Xetra, SIX, BME, all three Nasdaq Nordic markets, Vienna, Euronext Milan, and
  Euronext Dublin now have complete primary-sourced histories for their stated
  scopes. ICE Futures Canada
  now preserves the sourced January-2010 19:00 pre-open / 20:00–13:15 CT
  continuous grid, its 2011 opening change, 2012 and 2013 close revisions,
  and the 2016 move to 13:20 before closing on the 2018 IFUS transfer's actual
  Sunday opening day.
- **Product-scoped international derivatives schedules.** Eurex benchmark
  futures now preserve the January-2010 07:30 pre-trading / 07:50 continuous
  grid and classify the fixed-UTC Asian pre-trading/opening auction separately
  from post-2018 continuous trading across CET/CEST; EEX is
  narrowed to Nordic Zonal Power Futures with its 2024 launch; ICE Europe
  identities now model Brent and FTSE 100 Futures. ICE Endex preserves the
  transferred Dutch TTF contract's sourced 2013 WebICE grid, 2026 opening eve,
  and recurring DST rule; IFAD models Murban
  in its New-York-locked schedule. SGX is narrowed to Three-Month SORA Futures
  with its sourced 2024 launch and auction gaps. Binance Futures is narrowed to
  USDⓈ-M perpetuals and is closed before the exact sourced 2019-09-13 04:00 UTC
  platform launch. Xetra now includes the sourced 2025 Extended Retail envelope
  for DAX shares. Each profile's exact product scope and historical boundary are
  recorded in the verification ledger.
- **Nasdaq-family equity schedules and history.** Nasdaq BX/Texas now preserves
  its sourced January-2010 08:00–19:00 ET grid and exact April 18, 2011 move to
  07:00–19:00, while PSX uses 08:00–17:00 and is
  closed before its sourced October 8, 2010 launch; its launch-day 09:00 open
  moves to 08:00 on the sourced December 13, 2010 date. Nasdaq Stock Market
  history now retains the 07:00 pre-market before its sourced March 18, 2013
  move to 04:00. Nasdaq's announced Night Session remains monitored but is not
  encoded until the operator files its final readiness confirmation.
- **Cboe US-equity accepted-order envelopes.** BZX and BYX retain their exact
  2014-12-02/2014-12-01 06:00 queue onsets; BZX moves to a 02:30 queue on
  2025-05-01. EDGX retains its exact 2021 queue changes. EDGA/EDGX still begin
  on their 2010-07-02 first-symbol production day, but an undated original
  queue onset is now disclosed as Partial instead of being silently excluded.
- **NYSE-family accepted-order envelopes and Texas continuity.** Current
  profiles now include the operator-published 06:30 queues for NYSE, American,
  National, and Texas and 02:30 for Arca. NYSE Texas is correctly modeled as
  the continuing CHX/NYSE Chicago identity: 07:00–17:00 at the January-2010
  floor, Pillar from 2019-11-04, and a non-substantive 2025 rename—not a new
  2025 launch. Incomplete staged/legacy queue histories remain Partial.
- **IEX and Blue Ocean ATS identity/history.** The `iex` exchange identity is closed
  before its sourced August 19, 2016 first production-symbol launch instead of
  inheriting predecessor-ATS history. Blue Ocean is
  closed before its October 5, 2021 production launch and is explicitly scoped
  to the production ATS service rather than its earlier beta/testing phase;
  its primary-sourced new-order trading window is 20:00–04:00. The live
  filing's sub-minute resting-book cleanup is outside that stated scope.
- **Primary-sourced US listed-equity-options queues.** Seventeen identifiers now
  include their current generic 06:00, 07:00, or 07:30 order-acceptance phase
  before the 09:30–16:00 ordinary-stock-options RTH; MEMX correctly rejects
  pre-09:30 orders. Exact launch history remains, while each unavailable queue
  onset is explicitly Partial instead of receiving an inferred date.
- **Euronext cash-market clocks and pre-open phases.** Paris, Amsterdam,
  Brussels, Lisbon, Dublin, and Milan now use the operator's published 07:30
  CET pre-open. Paris, Amsterdam, Brussels, and Lisbon use the published
  nominal 09:00 continuous-session start and 17:35 Trading-at-Last handoff;
  the per-security 0–30-second randomized auction uncross is explicitly outside
  the exchange-level schedule. Lisbon
  and Dublin retain their venue IANA zones while translating Euronext's
  Central-European clock one hour earlier locally, eliminating the former
  one-hour UTC shift. Legacy Euronext markets change pre-opening on the sourced
  March 20, 2023 date, while Milan changes on March 27. Milan's complete history
  is recorded; Dublin now preserves its operator-published pre-floor timetable
  through successive ISE order-book models and the 2018 calendar. All six rows
  now have complete history within their documented exchange-level scopes.
- **CME equity-index matching, queues, and scope.** The pre-November-18, 2012
  profile no longer receives the later 15:30–16:15 CT post-halt session. That
  extension begins on CME's published Sunday effective date, the 16:15→16:00
  close change begins on September 20, 2015's opening day, and the 15:15–15:30
  halt is removed from the exact 2021-06-27 opening. The fixed-current profile
  now includes the Sunday 16:00 and weekday 16:45 Pre-Open queues. The dated
  history retains the exact 2010 weekday-queue change but omits the Sunday
  queue where its 16:15→16:00 onset day is unavailable, so the family is
  explicitly Partial. Its scope excludes full-size `SP`, NKD, BTIC, and TACO.
- **MEMX and MIAX Pearl Equities launch and early-session history.** Both
  venues are now closed before their sourced September 2020 production
  launches. MEMX retains its exact 2020-10-05 shortening to a 17:00 close,
  2023-02-01 restoration to 20:00, and 07:00 pre-market before the exchange's
  May 19, 2025 production launch of 04:00 trading; MIAX Pearl retains its
  regular-hours-only profile before its sourced February 20, 2025 Early and
  Late Trading Session launch.
- **CBOT standard-size grain/oilseed history and accepted-order phases.** The
  January-2010 profile now uses the published matching, morning Pre-Open, and
  PCP phases; PCP expands on 2010-04-19 and the morning queue moves to 08:00 on
  2011-12-27. The sourced 17:00–14:00 electronic regime begins Sunday
  2012-05-20, CME's 19:00–07:45 / 08:30–13:15 regime begins Sunday 2013-04-07,
  and the 13:20 close begins Sunday 2015-07-05. The fixed-current profile also
  includes the published 16:00/16:45 evening queues and 14:30–16:00 PCP.
  Their post-2012 onset chain is incomplete, so those phases are omitted from
  the affected dated eras and the history is Partial. Mini grains are excluded.
- **CME Group interest-rate, livestock, and cryptocurrency family history.**
  Interest rates retain the January-2010 17:30→16:00 CT matching grid and
  queues, the exact 2010 weekday-queue change, and the 2011-10-02 move to a
  17:00 match open. Livestock retains its overnight baseline, the 2014-10-27
  removal of evening trading, the 2016-02-29 move to 08:30–13:05, and the exact
  2020 move to an 08:00 Pre-Open; its fixed-current profile also includes PCP.
  Cryptocurrency is closed before the 2017-12-17 Bitcoin launch, preserves the
  five-day matching era, and switches at the exact 2026-05-29 24/7 transition
  with its 16:01 weekday and 03:45 Saturday Pre-Open phases, plus the temporary
  2026-08-01 maintenance extension and restoration. All three histories are
  Partial only for named older PCP/Pre-Open onset gaps; no date is inferred.
- **COMEX, NYMEX, CME FX, and ICE U.S. product scopes and queues.** The shared
  NYMEX energy/PGM and COMEX metals profile preserves the sourced January-2010
  16:15 CT close and September 2015 move to 16:00, while its fixed-current
  snapshot includes Sunday 16:00 and weekday 16:45 Pre-Open. Its scope is NYMEX
  CL/MCL/QM, NG/MNG/QG, HO/RB/BZ, and PL/PA plus COMEX GC/MGC, SI/SIL, and
  HG/MHG, excluding alternate session types and differently specified products.
  Standard-grid CME FX retains its 17:00→16:00 matching history and exact 2010
  weekday-queue change; current Sunday Pre-Open is included. Both CME histories
  are Partial only because the Sunday queue's earlier onset lacks a source day.
  ICE U.S. now represents NYSE FANG+ Index Futures with its exact November 2017
  launch, 30-minute Pre-Open queues as Extended, and matching as Regular instead
  of an uncited venue-wide default.
- **CFE's 2013 phased extensions, 2014 launch, and 2018 system-migration
  history.** The profile now retains CFE's 07:00 CT morning open before the
  sourced October 28, 2013 launch of its Monday–Thursday 15:29–15:30 pre-open
  and 15:30–16:15 session, then moves the morning open to 02:00 on the sourced
  November 4 second phase while retaining that pre-open.
  The nearly 24-hour profile starts on the sourced Sunday, June 22, 2014 launch
  rather than June 1, including its exact 16:15–17:00 Sunday pre-open. Its
  continuous Monday–Thursday 15:30–08:30 ETH comes directly from CFE-2014-010.
  A further sourced revision on February 25, 2018
  restores the 16:00–17:00 CT daily break introduced with CFE's system migration
  and classifies its 15:15–15:30 and later opening queues as extended rather
  than closed. Randomized queue starts use the sourced conservative latest edge
  of three seconds from 2018-02-25 and six seconds from 2018-08-12.
  The January-2010 08:30–15:15 baseline plus the December 2010 07:20 and
  September 2011 07:00 morning extensions are now date-aware, completing the
  VIX-futures normal-week history from the audit floor.
- **FINRA TRF session classification and 2026 opening-time history.** Carteret,
  Chicago, and the FINRA/NYSE TRF now classify 09:30–16:00 ET as regular and
  their before/after-RTH reporting windows as extended. Their point-in-time
  profiles retain the former 08:00 ET system open before March 30, 2026 and
  apply FINRA's 04:00 ET open from that effective date. Chicago is now closed
  before its sourced September 10, 2018 facility launch; that launch was
  test-security-only through September 21. FINRA's later overnight expansion
  remains monitored but unencoded while implementation follows the SIP rollout.
- **Calendar queries remain total at Chrono's representational bounds.** Local
  date resolution and forward/backward scans now use checked, inward-bounded
  arithmetic, and a bar that cannot advance past `DateTime<Utc>::MAX_UTC`
  returns `None` instead of overflowing.
- **Oversized intraday intervals clamp to the enclosing session close** instead
  of overflowing, and candle starts now exist exactly when their paired ends
  do. Weekly/monthly walks also retain the last representable valid close.

## [0.2.2] - 2026-08-21

Documentation only; no code, no API, and no dependency change. The published
`0.2.1` README carried claims that `0.2.1` itself had made false.

### Fixed

- **The README no longer says the crate logs through `tracing`.** Two claims
  survived the dependency's removal in `0.2.1` and shipped with it: the
  coverage table's always-open row stated that `Exchange::Unknown` "also logs a
  one-shot `tracing::warn!`", and the dependency paragraph listed `tracing`
  itself as a dependency. Neither has been true since `0.2.1`.
- **The version and MSRV badges are read from crates.io** rather than
  hardcoded. The version badge had been left at `0.2.0` across the `0.2.1`
  release — the same class of duplicated fact that `AGENTS.md` warns about —
  and pointed at a yanked version from the crate's own front page. Both badges
  now track the published crate and cannot drift again.

## [0.2.1] - 2026-08-21

A dependency removal and the lint attribute that fell out of it. No change to
any returned value. `0.2.0` is yanked in favour of this release.

### Removed

- **`tracing` is no longer a dependency.** It backed a single one-shot
  `warn!`, fired when a caller passed `Exchange::Unknown` to
  `hours_for_exchange` and received the documented 24×7 UTC fallback. That
  variant is reachable only deliberately — it is not a `Default`, and neither
  `FromStr` nor serde will produce it from an unrecognized name (both error) —
  so the warning reported a choice the caller had just made, to a subscriber
  most consumers of a pure-computation crate never install. The dependency
  tree is now `chrono`, `chrono-tz`, and `serde` alone — 25 locked packages
  down to 19, since `tracing-attributes` also pinned a second `syn` major
  alongside the one `serde_derive` uses. Returned values are unchanged:
  `hours_for_exchange(Exchange::Unknown)` still yields the 24×7 UTC fallback,
  as the contract suite's always-open invariant pins. Only the log line is
  gone.

### Changed

- `hours_for_exchange` is now `#[must_use]`, matching every other query in the
  crate. It was the sole public query without the attribute: the removed
  `warn!` was a side effect, so `clippy::must_use_candidate` never fired on
  it. **This is visible downstream** — a caller that discards the returned
  `MarketHours` now gets an `unused_must_use` warning, which is an error under
  `#![deny(warnings)]`. The signature and the returned value are unchanged, so
  any call that uses its result is unaffected.

## [0.2.0] - 2026-08-20 [YANKED]

API corrections published the same day as `0.1.0`. Both versions are now
**yanked** — `0.1.0` as superseded by this release, `0.2.0` in turn by
`0.2.1`, which is the release to use. The breaking changes below are therefore
breaking only against versions no longer available for selection; they remain
listed here because they are real and shipped.

### Changed

- **Breaking:** `Exchange` is `#[non_exhaustive]`, mirroring `MarketHoursKey`.
  Dependents must match it with a wildcard arm; in exchange, future venue
  additions are minor releases instead of breaking ones. Enumerate the
  variants of the compiled version via the new `Exchange::ALL`.
- **Breaking (behavioral):** the day's last intraday bar now ends at the daily
  close itself. Previously, when a `Minutes`/`Hours` bar ended exactly at a
  daily close followed by a maintenance break, `candle_end` snapped the end to
  the next session open (CME's last bar reported 17:00 CT instead of 16:00) —
  a V1-inherited exception that contradicted the crate's own end-exclusive
  close convention by counting closed time as bar time. Intraday bars now
  simply clamp to the enclosing session close, everywhere.
- **Breaking (behavioral):** `is_maintenance` now classifies the enclosing
  gap, not the distance to the reopen. It is true exactly when `t` lies inside
  a closed gap between two sessions whose whole close-to-reopen span is
  shorter than six hours. The old rule — closed and reopening within 90
  minutes — missed the front of every break longer than its threshold (the
  first 30 minutes of ICE's two-hour break, the first half of Eurex's
  three-hour gap, almost four hours of CBOT grains' 13:20→19:00 CT afternoon)
  and wrongly flagged pre-open windows (CME Sunday 15:30–17:00 CT, US
  equities 02:30–04:00 ET) that belong to overnight or weekend closures.

### Added

- `Exchange::ALL` — every variant, in declaration (`Ord`) order.
- `Exchange::as_str`, `Display`, and `FromStr` — one canonical `snake_case`
  name per venue, identical to the serde wire form, so string-keyed callers
  parse (`"nyse_arca".parse::<Exchange>()`) instead of pattern-matching. An
  unrecognized name is a `ParseExchangeError` (new public type) carrying the
  offending input, never a silent `Exchange::Unknown`.

## [0.1.0] - 2026-08-20 [YANKED]

Initial release.

### Added

- **69 venues** across US equities and options, US and international futures,
  EU equities, and always-open crypto, each schedule transcribed from a primary
  source and cited beside its table.
- **Session queries** — `is_open` / `is_open_with` over regular, extended, or
  both; `session_bounds`; `next_session_after`; `is_maintenance`;
  `is_closed_all_day_*`; `normal_week_open_seconds`.
- **Calendar-aware bar boundaries** — `candle_start` / `candle_end` across
  intraday, daily, weekly, and monthly `CalendarResolution`, clamped to real
  sessions so no bar spans a closed period and a daily bar closes at the
  session close rather than at midnight.
- **Point-in-time hours** — `hours_for_exchange_as_of` returns the schedule in
  effect at an instant, with cutovers compared in the venue's own local zone.
  A cutover exists only where a primary source states a day-level effective
  date; changes that are real but undated are recorded as known gaps rather
  than given invented dates.
- **Shared futures profiles** addressable by product family through
  `MarketHoursKey`, via `session_profile` / `hours_for_market_hours_key`.
- **Bulk builders** with deterministic `BTreeMap` iteration order.
- **Validated `SessionRule` construction** (`new` / `validate`) over a stated
  seconds-since-midnight domain, fence-checked against every shipped table.

### Notes

- Timestamps are `chrono::DateTime<Utc>` in and out; a venue's local zone and
  its DST rules stay internal. Opens resolve to the earliest valid instant and
  closes to the latest, so a session stays inclusive while its end-exclusive
  close lands on the true boundary.
- Boundary queries return `Option`. `None` means no session of the requested
  kind exists in the search horizon — a venue queried before its go-live date,
  or a zero-length interval — never a fabricated degenerate pair.
- This is a **normal-week** model: holidays, early closes, half-days, and
  product-level variations are out of scope. The profiles are exchange-level
  defaults and a best-effort model, not an authority. Have a human verify a
  venue against its currently published schedule before trading on it.
- No runtime state, no I/O, no clock reads, no floats, and
  `#![forbid(unsafe_code)]`.

[Unreleased]: https://github.com/SharurTrading/exchange-hours-rs/compare/v1.0.0...HEAD
[1.0.0]: https://github.com/SharurTrading/exchange-hours-rs/compare/v0.2.2...v1.0.0
[0.2.2]: https://github.com/SharurTrading/exchange-hours-rs/compare/v0.2.1...v0.2.2
[0.2.1]: https://github.com/SharurTrading/exchange-hours-rs/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/SharurTrading/exchange-hours-rs/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/SharurTrading/exchange-hours-rs/releases/tag/v0.1.0
