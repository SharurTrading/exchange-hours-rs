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
  crypto — with point-in-time historical revisions for 10 of them.
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

// Monday mid-morning sits inside the regular session. Boundary queries
// return `Option`: `None` means no matching session exists in the bounded
// search horizon (e.g. a pre-go-live date) or the interval is zero.
let monday_10am = ct(2026, 4, 20, 10, 0);
assert!(hours.is_open_regular(monday_10am));
let (open, close) = session_bounds(&hours, monday_10am).expect("CME trades this week");
assert_eq!(open, ct(2026, 4, 20, 8, 30));
assert_eq!(close, ct(2026, 4, 20, 15, 15)); // end-exclusive

// 16:30 CT is the daily maintenance gap: closed, but reopening within 90 min.
let monday_evening = ct(2026, 4, 20, 16, 30);
assert!(!hours.is_open(monday_evening));
assert!(hours.is_maintenance(monday_evening));

// After Friday's close the next session is Sunday evening, not Saturday.
let friday_after_close = ct(2026, 4, 24, 16, 30);
let (next_open, _) = next_session_after(&hours, friday_after_close).expect("reopens Sunday");
assert_eq!(next_open, ct(2026, 4, 26, 17, 0));

// Bar boundaries follow the same rules: a daily bar closes at the venue's
// session close, not at midnight.
let daily_close = candle_end(&hours, monday_10am, CalendarResolution::Daily);
assert_eq!(daily_close, Some(ct(2026, 4, 20, 16, 0)));
```

## Coverage

| Family | Variants | Local zone | Session shape |
|---|---|---|---|
| US equities (Reg NMS) | 17 | `America/New_York` | 09:30–16:00 regular everywhere; extended hours differ by venue — early opens 04:00 (Nasdaq×3, NYSE Arca, Cboe BZX/EDGX, MEMX, MIAX Pearl) or 07:00 (NYSE American/National/Texas, Cboe BYX/EDGA), NYSE itself is core-only, IEX runs 08:00–17:00 System Hours, IntelligentCross accepts orders from 09:00, and Blue Ocean ATS is overnight-only. |
| FINRA TRFs | 3 | `America/New_York` | 08:00–20:00; no extended session. |
| US options | 18 | `America/New_York` | 09:30–16:00; Cboe C1 adds a 20:15→09:25 GTH wrap and a 16:15–17:00 curb. |
| CME Globex futures | 4 | `US/Central` | 17:00→16:00 wrap with a 60-minute daily break; CBOT grains keep their own day session. |
| Cboe Futures (CFE) | 1 | `US/Central` | RTH 08:30–15:00 flowing seamlessly into post-settlement 15:00–16:00, plus a 17:00→08:30 overnight wrap. |
| EU equities | 14 | 11 European zones | 09:00–17:30 continuous as the continental default, with real divergence: LSE 08:00–16:30 (+ Closing Price Crossing to 16:40), SIX to 17:20, Euronext Dublin to 17:28, Nasdaq Stockholm to 17:25, Copenhagen to 16:55, Helsinki 10:00–18:25 EET; post-close trading-at-last windows on Euronext, Xetra, BME, Vienna, and SIX. |
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

- **UTC in, UTC out.** Every timestamp argument and timestamp result is `DateTime<Utc>`; the exchange `Tz` is an
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
  absent — `is_holiday` is a stub returning `false`, though every query path already routes
  through it under one session-existence contract. Verify contract specs before trading on
  these defaults.
- **No panics, and absence is `None`.** The public surface is total, and boundary queries
  (`session_bounds*`, `next_session_after*`, `candle_start*`/`candle_end*`,
  `time_end_of_day`) return `Option`: a profile with no session of the requested kind in
  the bounded search horizon (a venue before its go-live date), or a zero intraday
  interval, is `None` — never a fabricated degenerate pair that could leak downstream as a
  real session. Predicates report closed/not-in-maintenance/closed-all-day.
- **`SessionRule` has a stated domain.** `SessionRule::new` / `validate` enforce
  `open_ssm < 86_400`, `close_ssm <= 86_400`, a non-empty interval, and at least one
  enabled weekday; every shipped table is fence-checked against the same domain.

## Testing

Pure and stateless, so the validation class is property/deterministic-fixture, not workload.
Every test is an integration test over the public surface: the crate exposes nothing to tests
that callers do not also get (TEST-LAYOUT).

- `tests/venue_sessions.rs` — the per-venue baseline: published opens, the minute before, the
  end-exclusive close, overnight wraps, maintenance gaps, weekend boundaries (including SGX's
  Friday T+1 wrap into Saturday), always-open venues, bounds, the `snake_case` serde forms,
  and source-cited pins for the venue-data corrections (NYSE-family and Cboe early sessions,
  Nordic closes, European post-close windows).
- `tests/no_session_contract.rs` — the `None` contract for profiles with no rules.
- `tests/rule_validation.rs` — the `SessionRule` domain: what `new`/`validate` accept and reject.
- `tests/contract/session_invariants.rs` — properties that must hold for **every** venue and
  instant. A `splitmix64` PRNG on a fixed seed set plus a pinned DST fixture cover totality and
  determinism, maintenance implies closed, ordered bounds, and a strictly-advancing
  `next_session_after` walk. A grid sweep — every `Exchange`, every `SessionKind`, an hourly
  reference week, every rule boundary ±1s, and four DST transitions — pins the cross-query
  fence: `is_open_with(t, kind)` always agrees with whether `session_bounds_with(kind, t)`
  contains `t`, and a closed instant's bounds are exactly its next session. Failures print
  venue, kind, and instant in UTC and venue-local time.
- `src/lib.rs` doctest — the Quick start example above.

```bash
cargo fmt --check && cargo clippy --all-targets -- -D warnings && cargo test
```
