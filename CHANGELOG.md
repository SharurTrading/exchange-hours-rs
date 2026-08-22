<!-- SPDX-License-Identifier: MIT-0 -->

# Changelog

All notable changes to `exchange-hours` are documented in this file, starting
from the first tagged release. The format is based on
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and the crate adheres
to [Semantic Versioning](https://semver.org/spec/v2.0.0.html). Session-data
corrections (a venue's hours fixed against a primary source) go under
**Fixed**; new venues and new API surface under **Unreleased**/**Added**.

## [Unreleased]

### Added

- **23 major cash-equity venues:** ASX, TMX Australia, NZX, TSE, NSE India,
  BSE India, HKEX, SGX Securities, Bursa Malaysia, SET Thailand, IDX, PSE,
  HOSE, SSE, SZSE, KRX, TWSE, Borsa Istanbul, TSX, JSE, Tadawul, B3, and BMV.
  This brings the crate to 92 exchange variants.
- **The additive `ExchangeCalendar` API** for date-aware predicates, session
  scans, maintenance/all-day queries, and candle boundaries. Existing
  `MarketHours` functions and signatures remain unchanged; B3/BMV calendars
  reselect their published grid on every candidate trading day.
- **`hours_for_market_hours_key_as_of`** for primary-sourced point-in-time
  futures product-family snapshots, reusing the same dated tables as the
  corresponding exchange profiles.
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
  registry, and repeatable update guide make freshness, scope, and known
  evidence gaps auditable without overstating future accuracy.

### Changed

- **Equal `SessionRule` endpoints now mean a complete local-day session.** A
  rule such as Sunday `18:00→18:00` preserves one continuous session through
  Monday 18:00, including exact `session_bounds`; absence is represented by
  omitting the rule. This intentionally breaking pre-1.0 correction removes
  `SessionRuleError::EmptyInterval`.

### Fixed

- **Bursa Malaysia, IDX, HOSE, and BMV January-2010 history.** Bursa's morning
  continuous session now runs through 12:30 under the pre-audit-floor v2 manual
  and its dated successors. IDX now includes its archived 09:10–09:30 pre-open
  before the 2013 expansion. HOSE retains the operator's exact January-2010
  phase table plus its sourced September 2010, March 2012, and July 2013
  revisions. BMV preserves the exact March 2010 early interval and uses the
  operator's prospective New York-alignment rule from November 1, 2010 through
  a date-aware Mexico City/New York offset selector.
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
  Sunday opening day. The four remaining Euronext historical gaps stay
  documented rather than receiving invented cutovers.
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
  for DAX shares. Remaining older-history gaps stay explicit in the verification
  ledger.
- **Nasdaq-family equity schedules and history.** Nasdaq BX/Texas now preserves
  its sourced January-2010 08:00–19:00 ET grid and exact April 18, 2011 move to
  07:00–19:00, while PSX uses 08:00–17:00 and is
  closed before its sourced October 8, 2010 launch; its launch-day 09:00 open
  moves to 08:00 on the sourced December 13, 2010 date. Nasdaq Stock Market
  history now retains the 07:00 pre-market before its sourced March 18, 2013
  move to 04:00. Date-aware lookups add Nasdaq's 21:00–04:00 Night Session
  from 2026-12-06, preserving its fixed current snapshot.
- **Cboe US-equity launch and phase history.** BZX now retains its January-2010
  08:00–17:00 ET baseline; BYX, EDGA, and EDGX are closed before their sourced
  2010 exchange launches. Each venue applies its own published May 2016 move
  from 08:00 to 07:00, and BZX/BYX apply their distinct 2018 moves to a 20:00
  close. Existing 2021 EDGX and 2025 BZX 04:00 opens remain date-aware. EDGX's
  announced 21:00–04:00 session begins Sunday 2026-12-06 for the December 7
  business date, preserving the fixed current snapshot and 20:00–21:00 pause.
- **NYSE-family equity history.** Arca's 04:00–20:00 grid is now explicitly
  supported at the January-2010 audit floor. American retains its core-only
  continuous session until the sourced July 24, 2017 Pillar launch. National
  now preserves its legacy 08:00–18:30 and 08:00–20:00 grids, the sourced
  May 16, 2014 move to a 17:00 close, the 2014–2015 and 2017–2018 dormant
  intervals, the 2015 08:00–17:00 relaunch, and the May 21, 2018 Pillar
  relaunch.
- **IEX and US ATS identity/history.** The `iex` exchange identity is closed
  before its sourced August 19, 2016 first production-symbol launch instead of
  inheriting predecessor-ATS history. IntelligentCross IQX now uses its live
  and first public 2019 SEC ATS-N hours without treating the operator's January
  17, 2018 company commencement as an ATS launch; an archived 2018 operator FAQ
  now verifies launch-era hours. Primary filings bracket first live processing
  to August/September 2018 but do not state a day. Blue Ocean is
  closed before its October 5, 2021 production launch and is explicitly scoped
  to the production ATS service rather than its earlier beta/testing phase;
  its primary-sourced new-order trading window is 20:00–04:00. The live
  filing's sub-minute resting-book cleanup is outside that stated scope.
- **Primary-sourced US listed-equity-options profiles.** All 18 identifiers now
  model the published 09:30–16:00 regular-session envelope for ordinary
  individual-stock options, excluding ETF/ETN/index/FLEX/floor-only and
  venue-designated extended classes. Eleven post-2010 venues are closed before
  exact sourced launch dates; the other seven have pre-floor primary baselines.
- **Euronext cash-market clocks and pre-open phases.** Paris, Amsterdam,
  Brussels, Lisbon, Dublin, and Milan now use the operator's published 07:30
  CET pre-open and conservative 30-second randomized-opening envelope. Lisbon
  and Dublin retain their venue IANA zones while translating Euronext's
  Central-European clock one hour earlier locally, eliminating the former
  one-hour UTC shift. Legacy Euronext markets change pre-opening on the sourced
  March 20, 2023 date, while Milan changes on March 27. Milan's complete history
  is recorded; Dublin now preserves its operator-published pre-floor timetable
  through successive ISE order-book models and the 2018 calendar. Four legacy
  markets retain one explicit gap because Euronext's 2015
  randomization-introduction notice misprints its effective year and its
  successor does not repair the onset day.
- **CME equity-index historical closes.** The pre-November-18, 2012 profile no
  longer receives the later 15:30–16:15 CT post-halt session. That extension
  begins on CME's published Sunday effective date, and the later
  16:15→16:00 close change begins on September 21, 2015 rather than the
  unrelated March 4, 2016 boundary.
- **MEMX and MIAX Pearl Equities launch and early-session history.** Both
  venues are now closed before their sourced September 2020 production
  launches. MEMX then retains its 07:00 ET pre-market before the exchange's
  May 19, 2025 production launch of 04:00 trading; MIAX Pearl retains its
  regular-hours-only profile before its sourced February 20, 2025 Early and
  Late Trading Session launch.
- **CBOT grain/oilseed history.** The January 2010 profile now uses the
  published 18:00–07:15 and 09:30–13:15 CT split sessions. The sourced
  17:00–14:00 continuous electronic regime begins Sunday May 20, 2012, and
  CME's 19:00–07:45 / 08:30–13:15 regime begins Sunday April 7, 2013 rather
  than one day late. The 13:20 close still begins Sunday July 5, 2015 for the
  July 6 trade date.
- **COMEX, NYMEX, CME FX, and ICE U.S. product scopes.** COMEX Gold and NYMEX
  benchmark-energy profiles now preserve the sourced January-2010 16:15 CT
  close and the September 2015 move to 16:00. CME FX's unchanged 17:00→16:00
  grid is fenced by its pre-audit-floor February 2009 revision and primary
  2010, 2018, 2020, and current operator snapshots, and ICE U.S.
  now represents NYSE FANG+ Index Futures with its exact November 2017 launch
  and Sunday-session shape instead of an uncited venue-wide default.
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
  and classifies its 15:15–15:30, Sunday 16:00–17:00, and weekday 16:45–17:00
  order-entry queues as extended rather than closed.
  The January-2010 08:30–15:15 baseline plus the December 2010 07:20 and
  September 2011 07:00 morning extensions are now date-aware, completing the
  VIX-futures normal-week history from the audit floor.
- **FINRA TRF session classification and 2026 opening-time history.** Carteret,
  Chicago, and the FINRA/NYSE TRF now classify 09:30–16:00 ET as regular and
  their before/after-RTH reporting windows as extended. Their point-in-time
  profiles retain the former 08:00 ET system open before March 30, 2026 and
  apply FINRA's 04:00 ET open from that effective date. Chicago is now closed
  before its sourced September 10, 2018 facility launch; that launch was
  test-security-only through September 21. A second sourced
  revision adds the scheduled Sunday 21:00–Friday 20:00 operation, including
  Monday–Thursday 20:00–21:00 pauses, from December 6, 2026; FINRA states that
  implementation moves with the SIP rollout if that anticipated date changes.
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

[Unreleased]: https://github.com/SharurTrading/exchange-hours-rs/compare/v0.2.2...HEAD
[0.2.2]: https://github.com/SharurTrading/exchange-hours-rs/compare/v0.2.1...v0.2.2
[0.2.1]: https://github.com/SharurTrading/exchange-hours-rs/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/SharurTrading/exchange-hours-rs/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/SharurTrading/exchange-hours-rs/releases/tag/v0.1.0
