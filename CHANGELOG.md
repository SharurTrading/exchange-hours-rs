<!-- SPDX-License-Identifier: MIT-0 -->

# Changelog

All notable changes to `exchange-hours` are documented in this file, starting
from the first tagged release. The format is based on
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and the crate adheres
to [Semantic Versioning](https://semver.org/spec/v2.0.0.html). Session-data
corrections (a venue's hours fixed against a primary source) go under
**Fixed**; new venues and new API surface under **Unreleased**/**Added**.

## [Unreleased]

## [0.1.0] - 2026-08-20

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

[Unreleased]: https://github.com/SharurTrading/exchange-hours-rs/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/SharurTrading/exchange-hours-rs/releases/tag/v0.1.0
