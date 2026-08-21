<!-- SPDX-License-Identifier: MIT-0 -->

# Changelog

All notable changes to `exchange-hours` are documented in this file, starting
from the first tagged release. The format is based on
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and the crate adheres
to [Semantic Versioning](https://semver.org/spec/v2.0.0.html). Session-data
corrections (a venue's hours fixed against a primary source) go under
**Fixed**; new venues and new API surface under **Unreleased**/**Added**.

## [Unreleased]

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
