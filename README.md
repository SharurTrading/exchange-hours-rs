<!--
Copyright (C) 2026 Kevin Monaghan. All rights reserved.

This file is proprietary and confidential.
Unauthorized copying, use, modification, distribution, or disclosure of this file,
via any medium, is strictly prohibited except under a written agreement with the
copyright owner.
-->

# exchange-hours

**Exchange trading hours, session boundaries, and calendar-aware bar boundaries — UTC in, UTC out.**

Three questions about a `chrono::DateTime<Utc>` instant: is this venue open, what are the
bounds of the containing (or next) trading session, and where does a bar of a given
`CalendarResolution` start and end without spanning a closed period. A pure function library
over a `MarketHours` value: no state, no I/O, no clocks, no floats, no secrets, no money, and
`#![forbid(unsafe_code)]`.

- **69 venues** — US equities/options, US and international futures, EU equities, always-open
  crypto — with point-in-time historical revisions for 8 of them.
- **Session queries** — open/closed by regular/extended/both, session bounds, next open, gaps.
- **Calendar-aware bar boundaries** — intraday bars clamp to the session and step over
  maintenance breaks; daily/weekly/monthly bars close at real session closes, not midnight.
- **DST correctness by construction** — local seconds-since-midnight rules, resolved to
  instants with an explicit, asymmetric bias (opens earliest, closes latest).

## Quick start

This is the compiled doctest in `src/lib.rs`, copied verbatim. CME equity-index futures trade
17:00→16:00 CT with a one-hour daily break; RTH runs 08:30–15:15 CT:

```rust
use chrono::{TimeZone, Utc};
use chrono_tz::US;
use exchange_hours::{
    CalendarResolution, Exchange, candle_end, hours_for_exchange, next_session_after,
    session_bounds,
};

let ct = |y, m, d, hh, mm| {
    US::Central
        .with_ymd_and_hms(y, m, d, hh, mm, 0)
        .single()
        .expect("valid CT instant")
        .with_timezone(&Utc)
};

let hours = hours_for_exchange(Exchange::Cme);

// Monday mid-morning sits inside the regular session.
let monday_10am = ct(2026, 4, 20, 10, 0);
assert!(hours.is_open_regular(monday_10am));
let (open, close) = session_bounds(&hours, monday_10am);
assert_eq!(open, ct(2026, 4, 20, 8, 30));
assert_eq!(close, ct(2026, 4, 20, 15, 15)); // end-exclusive

// 16:30 CT is the daily maintenance gap: closed, but reopening within 90 min.
let monday_evening = ct(2026, 4, 20, 16, 30);
assert!(!hours.is_open(monday_evening));
assert!(hours.is_maintenance(monday_evening));

// After Friday's close the next session is Sunday evening, not Saturday.
let friday_after_close = ct(2026, 4, 24, 16, 30);
let (next_open, _) = next_session_after(&hours, friday_after_close);
assert_eq!(next_open, ct(2026, 4, 26, 17, 0));

// Bar boundaries follow the same rules: a daily bar closes at the venue's
// session close, not at midnight.
let daily_close = candle_end(&hours, monday_10am, CalendarResolution::Daily);
assert_eq!(daily_close, ct(2026, 4, 20, 16, 0));
```

## Coverage

| Family | Variants | Local zone | Session shape |
|---|---|---|---|
| US equities (Reg NMS) | 17 | `America/New_York` | 09:30–16:00 regular; 04:00–09:30 and 16:00–20:00 extended. NYSE Texas, IntelligentCross, and the overnight-only Blue Ocean ATS differ. |
| FINRA TRFs | 3 | `America/New_York` | 08:00–20:00; no extended session. |
| US options | 18 | `America/New_York` | 09:30–16:00; Cboe C1 adds a 20:15→09:25 GTH wrap and a 16:15–17:00 curb. |
| CME Globex futures | 4 | `US/Central` | 17:00→16:00 wrap with a 60-minute daily break; CBOT grains keep their own day session. |
| Cboe Futures (CFE) | 1 | `US/Central` | RTH 08:30–15:15, 15:30–16:00 curb, 17:00→08:30 overnight wrap. |
| EU equities | 14 | 11 European zones | 09:00–17:30 continuous plus auction windows; LSE 08:00–16:30. |
| ICE complex & European energy | 9 | London / Amsterdam / Berlin / Dubai / New York | 01:00–23:00 local, or the ICE 20:00→18:00 ET wrap. |
| Asia-Pacific futures (SGX) | 1 | `Asia/Singapore` | 07:10–20:00 day session plus a T+1 wrap to 05:15. |
| Always-open venues, and the `Exchange::Unknown` fallback | 2 | `UTC` | 24×7; no daily close, no weekend close. `Unknown` also logs a one-shot `tracing::warn!`. |

Futures hours track the *product family*, not the venue: nine shared profiles are also addressable by `MarketHoursKey` via `session_profile` / `hours_for_market_hours_key`.

## Place in the system

A foundational leaf. It depends only on `chrono` + `chrono-tz` (instant/zone arithmetic and
the DST resolver), `serde` (snake_case (de)serialization of the public types), and `tracing`
(the `Unknown`-fallback warning) — no engine, transport, adapter, async, or credential crate,
and no `tokio`. Downstream, the Sharur platform consumes it as a git dependency: the
instrument catalog maps instrument roots onto `MarketHoursKey` and re-exports
`FuturesSessionProfile`, `SessionRule`, and `session_profile`; chart bar consolidation and the
GUI's session overlays reach `candle_start` / `candle_end` / `session_bounds` through that
catalog rather than hand-rolling hour arithmetic — one shared definition, so a change here
reaches every consumer at once.

## Boundaries & invariants

- **UTC in, UTC out.** Every argument and result is `DateTime<Utc>`; the exchange `Tz` is an
  internal detail and callers never pass or receive local times.
- **Closes are end-exclusive.** An instant equal to a close is *closed*. Relaxing this makes
  adjacent sessions overlap and every duration double-count at its boundary.
- **`open_ssm > close_ssm` means the session wraps** past local midnight — the only way a
  session crosses a day boundary, so any "is it open?" answer must consult yesterday's rules
  as well as today's.
- **Opens bias earliest, closes bias latest.** On an ambiguous fall-back hour both mappings
  are valid; the asymmetry keeps a session inclusive while its end-exclusive close lands on
  the true boundary, and a spring-forward gap snaps to the first instant after it. This is
  the one place a maintainer must not "simplify" to a single bias.
- **Calendar bar starts come from sessions, not durations.** `candle_start` pairs a
  daily/weekly/monthly close with the first session open in that period, so a start can fall
  on the preceding civil day, month, or year. Probe a provider close marker at `close - 1ns`.
- **The `Exchange` match is exhaustive.** `hours_for_exchange` has no catch-all arm, so a new
  variant forces a profile decision instead of silently inheriting a default.
- **Always-open venues stay categorically separate.** A 24×7 venue is a single `0..86400`
  rule on all seven days with `has_daily_close` / `has_weekend_close` both `false`; it is
  never folded into the CME-style daily-break profiles.
- **Serde wire format is stable.** `Exchange` and `MarketHoursKey` serialize as `snake_case`
  strings; a rename that changes a serialized string breaks persisted data.
- **Normal week only.** Holidays, early closes, half-days, and product-level variations are
  absent — `is_holiday` is a stub returning `false`, though the wrap and daily-close paths
  already route through it. Verify contract specs before trading on these defaults.

## Testing

Pure and stateless, so the validation class is property/deterministic-fixture, not workload.

- `tests/unit/market_hours.rs` (98) — bridged into the lib via `#[cfg(test)] #[path = …] mod
  tests;` in `lib.rs`, a v1 layout artifact: the suite reaches no crate-private item, so it
  would also compile unchanged as an integration test. Pins the normal-week baseline per venue
  family: published opens, the minute before, the end-exclusive close, overnight wraps,
  maintenance gaps, weekend boundaries (including SGX's Friday T+1 wrap into Saturday), the
  always-open contract, session bounds, and the `snake_case` serde forms.
- `tests/contract/session_invariants.rs` (4) — public-surface invariants over a self-contained
  `splitmix64` PRNG on a fixed seed set plus a pinned DST fixture: totality and determinism of
  every public query, `is_open == is_open_with(Both)`, maintenance implies closed, ordered
  bounds, candle ends never preceding their start, and a strictly-advancing
  `next_session_after` walk. Failures print seed and instant, so they are reproducible.
- `src/lib.rs` doctest (1) — the Quick start example above.

```bash
cargo fmt --check && cargo clippy --all-targets -- -D warnings && cargo test
```
