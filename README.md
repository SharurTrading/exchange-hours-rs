<!--
Copyright (C) 2026 Kevin Monaghan. All rights reserved.

This file is proprietary and confidential.
Unauthorized copying, use, modification, distribution, or disclosure of this file,
via any medium, is strictly prohibited except under a written agreement with the
copyright owner.
-->

# market-hours

`market-hours` owns SHARUR's exchange-level trading-hours calendar. It answers,
for a UTC instant, whether a venue is open, where the enclosing (or next) trading
session begins and ends, and where a bar of a given `CalendarResolution` should
start and close while respecting that session structure. It holds no runtime
state, performs no I/O, and carries no money, secrets, or floats.

## Overview

Every component that reasons about time-of-day — chart bar consolidation, the GUI
market-hours overlays, instrument-catalog session profiles — needs the same
answers about a venue's schedule: is it trading right now, when does this session
close, when does the next one open, and is "now" inside a maintenance break. Left
to each consumer, those rules drift apart and disagree at the edges (the
end-exclusive close, the Sunday-evening Globex open, the overnight wrap that
spills past midnight, the daily maintenance gap).

This crate is the one place that decides those things. It models trading hours at
the **exchange level** as a set of weekday-masked open/close slices in the
exchange's local time zone, then evaluates them entirely in UTC so callers never
have to think about time zones or DST. The model is deliberately a *normal-week*
one: holidays, early closes, and product-by-product variations are out of scope
(the `is_holiday` hook is a stub that always returns `false` today), so the crate
gives pragmatic exchange-level defaults rather than a contract-exact calendar.

The crate stops at *schedule arithmetic*. It does not own which venue a symbol
trades on, route anything, or read live state — it is a pure function library over
a `MarketHours` value the caller already holds.

## Architecture

A thin `lib.rs` declares one private `calendar` module and re-exports its public
surface (`PAT-THIN-01`); all logic and data live in `calendar`. The module is a
flat collection of value types, a large static table of venue profiles, and free
functions that evaluate them.

**Core types.**

| Type | Role |
|---|---|
| `Exchange` | Exhaustive venue identifier grouped by product family; serializes as `snake_case`. |
| `MarketHours` | A timezone-aware profile: `tz` plus `regular` + `extended` rule sets and the `has_daily_close` / `has_weekend_close` flags. Owns the open/closed and boundary query methods. |
| `SessionRule` | One schedule slice: a `[bool; 7]` weekday mask plus `open_ssm` / `close_ssm` (seconds since local midnight, close end-exclusive). |
| `SessionKind` | Selects `Regular`, `Extended`, or `Both` rule sets for a query. |
| `CalendarResolution` | The bar interval consumed by the candle-boundary helpers. |
| `MarketHoursKey` / `FuturesSessionProfile` | A named, statically-shared futures profile and the key that selects it via `session_profile`. |

**Time and session model.** Every public helper takes and returns
`chrono::DateTime<Utc>`. The exchange's `Tz` is used only internally — to map a
rule's seconds-since-local-midnight (SSM) onto a concrete instant and to interpret
"the day". Weekdays are indexed Monday = 0 through Sunday = 6, matching the `days`
mask. A rule with `open_ssm <= close_ssm` is a same-day session; a rule with
`open_ssm > close_ssm` **wraps** past local midnight and closes on the next local
day. All close comparisons are **end-exclusive**: an instant exactly at
`close_ssm` is closed, so adjacent sessions never overlap. A wrapped session from
"yesterday" may spill into today, but never across a holiday boundary (relevant
once `is_holiday` is implemented).

**DST resolution.** Local wall-clock instants are resolved with a deterministic
bias: opens take the **earliest** valid mapping, closes (being end-exclusive) take
the **latest**, and a wall-clock that falls in a spring-forward gap snaps to the
earliest valid instant *after* the gap via a bounded minute-by-minute search.
"Next open after `T`" comparisons run on resolved **instants**, not raw SSM, so
fall-back duplications do not produce anomalies.

**Profile storage.** Venue defaults are `static` `SessionRule` tables shared by
many venues (a private `StaticHoursProfile` for equities/options/EU venues, the
public-keyed `FuturesSessionProfile` for futures). `MarketHours.regular` /
`extended` are `Cow<'static, [SessionRule]>`, so a profile built from a static
table borrows it without allocating, while time-varying or empty profiles can own
a `Vec`. `hours_for_exchange_as_of` selects between historical revisions of a
profile (e.g. the CME 2016 short-window change, EUREX Asian hours from 2018) by
comparing `as_of` against hard-coded cutover dates.

**Ownership model.** The crate is stateless and allocation-light: queries borrow a
`&MarketHours` and return owned `DateTime<Utc>` results or tuples. There is no
shared mutable state, no `Arc`/`Mutex`, and `#![forbid(unsafe_code)]` is set.
Several builder and convenience entry points are retained behind
`#[allow(dead_code)]` for the test harness and downstream call sites; they remain
part of the public surface.

## Public API

Grouped by purpose. Every identifier is re-exported from the crate root.

- **Venue → hours:** `hours_for_exchange`, `hours_for_exchange_as_of`.
- **Open / closed queries** (on `MarketHours`): `is_open`, `is_open_with`,
  `is_open_regular`, `is_open_extended`, `is_maintenance`,
  `is_closed_all_day_in_calendar`, `is_closed_all_day_on`, `is_closed_all_day_at`,
  and `normal_week_open_seconds` for the unioned regular + extended schedule duration.
- **Session bounds:** `session_bounds`, `session_bounds_with`,
  `next_session_after`, `next_session_after_with`, `next_session_open_after`.
- **Candle / bar boundaries:** `candle_start`, `candle_end`, `candle_end_with`,
  `time_end_of_day`.
- **Named futures profiles:** `MarketHoursKey`, `session_profile`,
  `FuturesSessionProfile` (with its own `FuturesSessionProfile::is_open`).
- **Bulk builders:** `hours_for_all`, `hours_for_us_equities`,
  `hours_for_eu_equities`, `hours_map_for`, `hours_map_us_equities`,
  `hours_map_eu_equities`.
- **Static rule tables:** `US_EQUITY_REGULAR`, `US_EQUITY_EXTENDED`,
  `NYSE_TEXAS_EXTENDED`, `BLUE_OCEAN_EXTENDED`.
- **Core value types:** `Exchange`, `CalendarResolution`, `SessionRule`,
  `SessionKind`, `MarketHours`.

## Boundaries & invariants

What the crate guarantees and what callers must not do:

- **UTC in, UTC out.** Every query argument and result is `DateTime<Utc>`. The
  exchange `Tz` is an internal detail; callers never pass or receive local times.
  Pass instants, not wall-clock strings.
- **Closes are end-exclusive; wraps do not overlap.** An instant equal to a
  session close is closed, and a wrapping session (`open_ssm > close_ssm`) is the
  only way a session crosses local midnight. A maintainer must preserve both rules
  together — relaxing the end-exclusive comparison would make back-to-back
  sessions double-count.
- **Calendar opens come from sessions, not elapsed durations.** `candle_start`
  pairs daily/weekly/monthly closes with the first catalog session open in that
  trading period. The start may therefore fall on the preceding civil day,
  month, or year, and a provider close marker must be probed at `close - 1ns`.
- **The `Exchange` match is exhaustive.** `hours_for_exchange` has no catch-all
  arm, so adding an `Exchange` variant forces a deliberate profile decision rather
  than silently inheriting a default. `Exchange::Unknown` is the explicit fallback
  and maps to a 24×7 UTC profile with a one-shot `tracing::warn!`.
- **Always-open venues stay categorically separate from futures venues.** A 24×7
  venue (`Exchange::BinanceFutures`) is modeled as a single `0..86400` same-day
  rule active every weekday, with `has_daily_close` / `has_weekend_close` both
  `false`; it must never be folded into the CME-style daily-break profiles.
  Callers can tell the two apart by inspecting the session rules.
- **Serde wire format is stable.** `Exchange` and the rule/profile types serialize
  as `snake_case` strings; the `exchange_serde_snake_case_*` tests pin that form
  (`MarketHoursKey` likewise). A rename that changes a serialized string is a
  breaking change to persisted/transported data and must not be made casually.
- **Parity across modes (`SH-PAR-01`).** The calendar is mode-agnostic — the same
  `MarketHours` answers identically for live, sandbox, and backtest. There is no
  per-mode fork in this crate.
- **No secrets, no money, no floats.** This surface carries no credentials
  (`SH-SEC-01`) and no monetary values, so the exact-decimal rule (`SH-DEC-01`)
  does not arise; the crate reasons purely about time. `#![forbid(unsafe_code)]`
  holds crate-wide.

## Place in the system

`market-hours` is a foundational, internal contract crate. It depends only on
`chrono` + `chrono-tz` (instant/zone arithmetic and the DST resolver), `serde`
(snake_case (de)serialization of the public types), and `tracing` (the
`Unknown`-fallback warning). It pulls in no engine, transport, adapter, async, or
credential crate, and it is `tokio`-free.

Within SHARUR it sits at the market-data / instrument foundation.

- **`instrument-catalog`** re-exports `FuturesSessionProfile`, `MarketHoursKey`,
  `SessionRule`, and `session_profile` at its crate root, and layers its own
  instrument-root-to-profile-key selection on top of these normal-week profiles.
  Its `FuturesExchange` enum is a *distinct* catalog-owned type, not this crate's
  `Exchange`.
- **`gui`** consumes root-session decisions through `instrument-catalog` rather
  than defining its own market-hours wrapper or test copy.

Because it is a leaf of the calendar layer, a change to its session semantics
ripples outward to those consumers; its open/closed and boundary contract is the
stable surface they build on.

## Design notes

- **Why SSM + a wrap flag instead of explicit datetimes.** Storing each session as
  a weekday mask plus seconds-since-local-midnight keeps the static profile tables
  compact and DST-agnostic — the same rule applies every week, and the local→UTC
  mapping (including DST) is computed at query time. The `open_ssm > close_ssm`
  convention encodes overnight sessions without a separate "spans midnight" field.
- **Why opens bias earliest and closes bias latest.** On an ambiguous fall-back
  hour both mappings are valid; biasing opens early and closes late keeps a
  session maximally inclusive and end-exclusive at its true boundary, so a bar that
  ends "at the close" lands on the later instant. The spring-forward gap is handled
  by stepping forward to the first representable instant. This is the one place a
  maintainer must not "simplify" to a single bias.
- **Why maintenance is a heuristic, not a rule.** `is_maintenance` returns `true`
  when the market is closed *and* the next session opens within 90 minutes, which
  captures daily breaks (e.g. CME 16:00–17:00 CT) without modeling them as data.
  The intraday candle helpers (`candle_end_with` for `Minutes`/`Hours`) reuse it:
  when a bar would end exactly at a venue's daily close and a maintenance gap
  follows, the bar end snaps forward to the next session open so bars do not
  terminate at the start of the break.
- **Why daily/weekly closes are computed, not stored.** `Resolution::Daily`
  resolves to the latest session close occurring on a local calendar day (across
  same-day and wrap sessions), and `Resolution::Weekly` walks daily closes forward
  until the next one falls in a different ISO week. This keeps "end of day/week"
  consistent with the same session rules that drive open/closed queries.
- **Why `hours_for_exchange_as_of` carries historical revisions.** Several venues
  changed their published hours (CME short window in 2016, EUREX Asian hours in
  2018, CFE overnight in 2014, NYSE Texas / Blue Ocean go-live dates, the CBOT
  2013 reduction, the Cboe C1 GTH change in 2024). Point-in-time backtests need the
  hours that were in effect then, so the `as_of` selector exists alongside the
  "current profile" `hours_for_exchange`.
- **Why many items carry `#[allow(dead_code)]`.** Bulk builders, the
  equity-by-region maps, and a few convenience entry points are exercised by the
  test harness and kept available for downstream callers even when no in-crate
  caller reaches them; each annotation states that reason inline (`STACK-CI-01`).

## Testing

The crate is a pure, stateless schedule library, so its validation class is
property/deterministic-fixture rather than a stateful workload. Tests are
package-owned under `tests/`:

- `tests/unit/market_hours.rs` — the private unit suite, bridged into the lib
  via `#[cfg(test)] #[path = "../tests/unit/market_hours.rs"] mod tests;` in
  `lib.rs` (`TEST-UNIT-01`). It builds UTC instants through small per-zone
  helpers (`ct`, `et`, `cet`, `lon`, `sgt`, `utc`) and pins the normal-week
  baseline for each futures family.
- `tests/contract/session_invariants.rs` — the public-contract invariant and
  deterministic-workload suite over the re-exported API only (`TEST-CONTRACT-01`).

`tests/unit/market_hours.rs` failure modes covered:

- **Per-venue session edges** for CME, CBOT, COMEX, NYMEX, EUREX, ICEUS, ICEEU,
  SGX, and CFE — open at the published open, closed one minute before, the
  end-exclusive close, the overnight wrap into the next day, and the daily
  maintenance/break gaps (asserted via `is_open*` and `is_maintenance`).
- **Named futures profiles** — every `MarketHoursKey` resolves to a non-empty
  `FuturesSessionProfile`, with direct checks for Globex equity-index maintenance,
  Globex energy wraps, and the structural `AlwaysOpen` profile.
- **Weekend boundaries** — Friday "no overnight", Saturday closed, and the Sunday
  re-open, plus `is_closed_all_day_on` across the futures set (including SGX's
  Friday T+1 wrap that legitimately extends into Saturday morning).
- **Always-open venues** — `Exchange::BinanceFutures` open every day with no
  maintenance and no daily/weekend close, and structurally distinct from CME.
- **Session bounds** — `session_bounds` returns the correct RTH/day window and
  `next_session_after` jumps from a Friday close to the Sunday open.
- **Serde stability** — representative `Exchange` variants round-trip through the
  exact `snake_case` strings (`nasdaq_bx`, `cboe_options_c1`,
  `ice_europe_commodities`, `binance_futures`, `intelligentcross_iqx`).

`tests/contract/session_invariants.rs` drives a self-contained `splitmix64` PRNG
over a fixed seed set (no external dependency) and asserts structural invariants
that must hold for every venue and instant. Any failure prints the seed,
iteration/step index, the enabled venue/resolution operation space, the venue,
and the offending UTC instant, so it is exactly reproducible
(`TEST-DETERMINISM-01`, `TEST-INVARIANT-01`). Invariants proved:

- **Totality + determinism** — every public query is panic-free and returns the
  same answer when called twice for the same input, including across the
  spring-forward / fall-back DST transition instants pinned in
  `dst_transition_queries_are_stable_and_total`.
- **`is_open` equals `is_open_with(Both)`**, and **maintenance implies closed**
  (`is_maintenance` is never true while `is_open` is true).
- **Ordered bounds** — `session_bounds` and `next_session_after` never return a
  close before its open, the next session never opens in the past, and
  `next_session_open_after` agrees with `next_session_after().open`.
- **Candle ends never precede the bar start**, and `Seconds(s)` is a pure
  `t + s` offset.
- **Strictly-advancing session walk** — repeatedly advancing by
  `next_session_after(..).open` yields a strictly increasing, progress-making
  sequence (the function never stalls, moves backward, or hits the degenerate
  no-session fallback for a trading venue).
- **Always-open venues** (`BinanceFutures`, `Unknown`) never close and are never
  in maintenance.

Because the crate owns no runtime state, transport, or reconnect logic, the
`TEST-FAULT-01` tier-resilience adverse classes do not apply; the
package-appropriate edge inputs (DST transitions, weekend/maintenance gaps,
wrap-past-midnight sessions, and the `Unknown` fallback) are covered by the
seeded sampling and the pinned DST fixture.

Validation commands:

```bash
cargo nextest run -p market-hours --all-features --no-fail-fast
cargo doc -p market-hours --all-features --no-deps
```

The workspace acceptance gate remains `bash scripts/ci/offline_quality.sh`.

## Status

`version = "0.1.0"`, `publish = false`. Internal path dependency;
`instrument-catalog` depends on it by path. Holiday calendars, early-close /
half-day schedules, product-level hour variations, and venue-specific expiry
pauses are deliberately deferred — the `is_holiday` hook is a stub and the
profiles are exchange-level normal-week defaults. The serialized `snake_case`
forms and the open/closed + boundary contract are the stable surface downstream
crates build on.
