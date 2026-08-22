<!-- SPDX-License-Identifier: MIT-0 -->

# exchange-hours

[![CI](https://github.com/SharurTrading/exchange-hours-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/SharurTrading/exchange-hours-rs/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/exchange-hours.svg)](https://crates.io/crates/exchange-hours)
![Edition](https://img.shields.io/badge/edition-2024-orange)
![MSRV](https://img.shields.io/crates/msrv/exchange-hours.svg)
![Unsafe](https://img.shields.io/badge/unsafe-forbidden-success)
[![License](https://img.shields.io/badge/license-MIT--0-blue)](LICENSE)

**A Rust library that knows when exchanges are open — and where sessions and chart bars
begin and end.**

Give it a UTC timestamp (`chrono::DateTime<Utc>`) and an exchange, and it answers three
questions:

1. **Is the market open right now?** — regular hours, extended hours, or either.
2. **When does the current session start and end?** — and if the market is closed, when
   the next session opens.
3. **Where do chart bars begin and end?** — intraday, daily, weekly, and monthly bar
   boundaries that follow the exchange's real sessions, so a bar never spans a closed
   period and a daily bar closes at the session close, not at midnight.

Everything is a pure function over a `MarketHours` snapshot or date-aware
`ExchangeCalendar`: no state, no I/O, no clock reads, no floats, and
`#![forbid(unsafe_code)]`. Timestamps go in as UTC and come out as UTC — each
exchange's local time zone, including its DST quirks, is handled internally.
The internal ownership and extension model is documented in
[ARCHITECTURE.md](ARCHITECTURE.md).

- **91 exchange identifiers** — US equities/options, US and international futures, EU and Asia-Pacific
  equities, other major global cash markets, and always-open crypto — with independently
  fenced point-in-time revisions wherever primary evidence states a day-level boundary.
- **Session queries** — open/closed by regular/extended/both, session bounds, next open, gaps.
- **Calendar-aware bar boundaries** — intraday bars clamp to the session close so no bar
  spans a closed period; the day's last bar ends at the daily close itself (CME 16:00 CT,
  never the 17:00 reopen), and daily/weekly/monthly bars close at real session closes,
  not midnight.
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
// search horizon (for example, a pre-go-live date).
let monday_10am = ct(2026, 4, 20, 10, 0);
assert!(hours.is_open_regular(monday_10am));
let (open, close) = session_bounds(&hours, monday_10am).expect("CME trades this week");
assert_eq!(open, ct(2026, 4, 20, 8, 30));
assert_eq!(close, ct(2026, 4, 20, 15, 15)); // end-exclusive

// 16:30 CT is the daily maintenance break: closed, inside a gap between
// two sessions (16:00→17:00) shorter than six hours end to end.
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

For a venue whose grid can change between dates, keep an `ExchangeCalendar`
instead of carrying one fixed snapshot:

```rust
use chrono::{TimeZone, Utc};
use exchange_hours::{Exchange, calendar_for_exchange};

let bmv = calendar_for_exchange(Exchange::Bmv);
// All predicates, session scans, and candle methods take UTC instants and
// reselect BMV's normal/early grid for each candidate trading date.
let instant = Utc.with_ymd_and_hms(2026, 8, 19, 16, 0, 0)
    .single()
    .expect("valid UTC instant");
let snapshot_for_one_instant = bmv.hours_at(instant);
assert_eq!(snapshot_for_one_instant.exchange, Exchange::Bmv);
```

## Coverage

| Family | Variants | Local zone | Session shape |
|---|---|---|---|
| US equities and ATS | 16 | `America/New_York` | 09:30–16:00 regular on matching venues; modeled extended hours differ by venue. Nasdaq is 04:00–20:00 today, Nasdaq BX/Texas is 07:00–19:00, and PSX is 08:00–17:00. Date-aware profiles add Nasdaq and EDGX 21:00–04:00 sessions from 2026-12-06 while fixed current snapshots remain unchanged. NYSE Tape A is core-only, IEX runs 08:00–17:00 System Hours, and Blue Ocean's production new-order ATS window is 20:00–04:00. |
| FINRA TRFs | 3 | `America/New_York` | 09:30–16:00 regular; outside-RTH reporting is extended (04:00–20:00 system envelope from 2026-03-30, with the sourced overnight expansion tracked from 2026-12-06). |
| US options | 18 | `America/New_York` | Each venue models the primary-sourced 09:30–16:00 regular-session envelope for ordinary individual-stock options, with exact closed-before-launch history where the venue began after January 2010. ETF, ETN, index, FLEX, floor-only, and venue-designated extended-hours classes are outside this deliberately narrow scope. |
| CME Globex futures | 4 | `US/Central` | CME equity-index futures use the current 17:00→16:00 envelope with their 15:15–15:30 halt; date-aware history retains the sourced 2012 and 2015 close changes. CBOT grains keep distinct sourced 2010, 2012, 2013, and 2015 regimes. |
| Cboe Futures (CFE) | 1 | `US/Central` | RTH 08:30–15:00 flows into post-settlement 15:00–16:00; order-entry queues run Sunday 16:00–17:00 and Monday–Thursday 16:45–17:00 before the 17:00→08:30 overnight wrap. |
| EU equities | 14 | 11 European zones | 09:00–17:30 continuous as the continental default, with venue-owned phases: Xetra's DAX-share envelope includes participant-restricted Extended Retail from 07:00 to 22:00; LSE SETS includes 07:00 pre-trading, randomized opening/noon auctions, and CPX to 16:40; central Euronext profiles use the published nominal phase boundaries and exclude per-security randomized uncross seconds; SIX, BME, Vienna, and Nasdaq Nordic books keep their own phases and clocks. |
| Asia-Pacific equities | 17 | 14 IANA zones | ASX, TMX Australia, NZX, TSE, NSE India, BSE India, HKEX, SGX Securities, Bursa Malaysia, SET, IDX, PSE, HOSE, SSE, SZSE, KRX, and TWSE. Lunch breaks, auctions, and post-close windows stay venue-specific. |
| Other major global equities | 6 | Toronto / Istanbul / Johannesburg / Riyadh / São Paulo / Mexico City | TSX, Borsa Istanbul, JSE's main/liquid ZA01 segment, Tadawul, B3, and BMV, including their pre-open, closing-auction, and trade-at-last phases. B3/BMV grids are date-aware because they follow New York's offset relationship. |
| ICE complex & European energy | 9 | London / Amsterdam / Berlin / Dubai / New York / Winnipeg | Named product-family profiles: Eurex FESX/FDAX/FDXM, EEX Nordic Zonal Power, ICE FANG+, Brent, FTSE 100, Dutch TTF, Murban, and legacy Canola. Date-aware profiles preserve product launches, FTSE extensions, Eurex's fixed-UTC Asian pre-trading/auction and continuous phases, Endex's 2026 extension/DST rule, Murban's New-York-locked grid, and Canola's sourced 2010–2018 eras. |
| Asia-Pacific futures (SGX) | 1 | `Asia/Singapore` | Three-Month SORA Futures: continuous 07:25–17:55 and 18:15→05:15, with opening/closing phases and a 18:00–18:05 gap. Closed before the 2024-07-29 launch. |
| Always-open venues, and the `Exchange::Unknown` fallback | 2 | `UTC` | Binance USDⓈ-M perpetuals are normally 24×7 after their exact 2019-09-13 04:00 UTC launch; `Unknown` is always-open library policy. |

Futures hours track the *product family*, not the venue: nine shared profiles are also addressable by `MarketHoursKey`. Fixed snapshots use `session_profile` / `hours_for_market_hours_key`; sourced dated revisions use `hours_for_market_hours_key_as_of`.

## Schedule assurance

**Repository-wide review completed:** `2026-08-21`

**Primary-source-verified current profiles:** `90 of 90` real exchange
identifiers, within each row's documented normal-week scope.

**Complete sourced history since January 2010:** `90 of 90` real exchange
identifiers.

**Venue-specific profiles requiring reconciliation:** `0 of 90` real exchange
identifiers.

Every real exchange identifier was compared with its official current-hours or
rulebook material and its notice/evidence channel. All 90 current profiles are
primary-supported within their stated scope, and all 90 **Primary** rows have
no known modeled-history gap since January 2010 or their sourced launch. No
real row remains **Partial**, **Secondary**, **Pragmatic**, or **Known issue**.
`Exchange::Unknown` is synthetic and is not one of the 90 real identifiers.

The key surface was audited separately:
**Primary-source-verified current key snapshots:** `8 of 8` operator-derived
`MarketHoursKey` values. The key API provides both fixed-current snapshots and
an `as_of` selector for sourced histories. All eight operator-derived key rows
are **Primary**.

These are backward-looking evidence statements, not promises that an exchange
will remain unchanged after the review date. They cover recurring weekday
phases, time zones, lunch and maintenance gaps, and weekend boundaries. They
exclude holidays, half-days, one-off closures, halts, severe-weather exceptions,
and product-specific variations outside a row's stated scope. The full method,
corrections, exclusions, and confidence levels are recorded in the
[2026-08-21 schedule audit](docs/schedules/audit-2026-08-21.md).

The guarantee is exchange/segment/product-family level, never ticker-level
microstructure. When an auction uncross is randomized per security, the row
states whether the deterministic profile uses the operator's nominal phase
boundary or a conservative venue envelope; it does not predict that day's
per-security uncross second.

The [source-set registry](docs/schedules/sources.md) records the stable official
pages and notice/evidence entry points to check for each exchange. The
repeatable process is in
[Updating exchange schedules](docs/schedules/updating.md). Exact historical
notices remain cited beside the Rust table they support.

## Historical amendments

`hours_for_exchange` returns a fixed default snapshot, while
`hours_for_exchange_as_of` returns the fixed snapshot selected at a UTC instant. For
queries that span dates, `calendar_for_exchange` is the authoritative entry point: its
session and candle methods resolve the applicable profile again for every candidate
trading day.

- **Recorded changes only.** A venue gets a historical cutover only when a primary source
  (exchange notice, rulebook amendment, press release) states a day-level effective date.
  Real changes without a sourced date are documented as known gaps rather than given
  invented dates. The current verification ledger has no such gap within its stated
  exchange/product scopes. For Paris, Amsterdam, Brussels, and Lisbon, Euronext's
  per-security 0–30-second auction uncross delay is outside the exchange-level schedule,
  so those profiles use the operator's published nominal phase boundaries. Dublin and
  Milan retain their documented conservative latest-edge envelopes. Every choice
  preserves the exact venue-wide open/closed envelope. IEX and Blue Ocean have sourced
  production launch boundaries, and B3's explicit older grids are fully recorded to
  January 2010.
- **Cutover semantics.** Date-only changes are compared in the venue's **own local zone**.
  The new profile applies from venue-local midnight on its session opening day—often Sunday
  for a Monday trade-date change. When a primary source states an exact intraday instant,
  that instant is preserved rather than rounded to midnight; one nanosecond before it sees
  the old profile.
- **How far back.** The aim is to record every session-defining amendment back to
  **January 2010**; changes before that are out of scope by design. Below a venue's oldest
  recorded profile, `hours_for_exchange_as_of` keeps returning that oldest profile — it
  does not manufacture an older row or change. Venues with no recorded change return current
  hours at every `as_of`.

B3 and BMV use recurring cross-zone selection. B3 chooses its short or long cash-equity
grid from the New York−São Paulo UTC-offset difference; BMV chooses its early or normal
grid from the New York−Mexico City difference. This covers the mismatch weeks created by
the countries' former DST calendars and the post-abolition rules without hard-coding yearly
US transition dates. A `MarketHours` returned for one date remains a fixed snapshot, so use
`calendar_for_exchange` rather than carrying that snapshot across a transition.

## Best effort — validate before production use

Primary-sourced tables and dated revisions carry their citations beside the data and are
pinned by tests. Every real current profile is source-supported within the product or
segment scope stated in the ledger, with complete January-2010-or-launch history at that
scope. This crate is a **best-effort model, not an authority**:
exchanges amend hours on short notice, publish product-level exceptions, and run holiday
and half-day schedules that this normal-week model deliberately omits. Before trading on
any venue's hours in production, have a human verify the profile against the exchange's
currently published schedule and the relevant contract specifications.

## Place in the system

A foundational leaf. It depends only on `chrono` + `chrono-tz` (instant/zone arithmetic and
the DST resolver) and `serde` (snake_case (de)serialization of the public types) — no logging
facade, engine, transport, adapter, async, or credential crate, and no `tokio`. Downstream, the Sharur platform consumes it as a git dependency: the
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
- **`open_ssm >= close_ssm` means the session wraps** into the next local day. Equal
  endpoints encode one complete local-day span, so a Sunday `18:00→18:00` rule is one
  continuous session through Monday 18:00. Omit the rule to express no session. Any "is it
  open?" answer must therefore consult yesterday's rules as well as today's.
- **Opens bias earliest, closes bias latest.** On an ambiguous fall-back hour both mappings
  are valid; the asymmetry keeps a session inclusive while its end-exclusive close lands on
  the true boundary, and a spring-forward gap snaps to the first instant after it. This is
  the one place a maintainer must not "simplify" to a single bias.
- **Calendar bar starts come from sessions, not durations.** `candle_start` pairs a
  daily/weekly/monthly close with the first session open in that period, so a start can fall
  on the preceding civil day, month, or year. Probe a provider close marker at `close - 1ns`.
- **The `Exchange` matches are exhaustive inside the crate.** `hours_for_exchange` and
  `Exchange::as_str` have no catch-all arms, so a new variant forces a profile decision and
  a canonical name instead of silently inheriting defaults. The enum itself is
  `#[non_exhaustive]` (like `MarketHoursKey`), so venue additions are not breaking changes
  for dependents — match it with a wildcard arm, and enumerate via `Exchange::ALL`.
- **Always-open venues stay categorically separate.** A 24×7 venue is a single `0..86400`
  rule on all seven days with `has_daily_close` / `has_weekend_close` both `false`; it is
  never folded into the CME-style daily-break profiles.
- **One canonical name per venue, and it is stable.** `Exchange` and `MarketHoursKey`
  serialize as `snake_case` strings in every Serde format, and `Exchange` exposes
  the same names directly:
  `as_str`, `Display`, and `FromStr` (`"nyse_arca".parse::<Exchange>()`), so string-keyed
  callers parse instead of pattern-matching. An unrecognized name is a `ParseExchangeError`,
  never a silent `Exchange::Unknown`. A rename that changes one of these strings breaks
  persisted data. Neither identity enum uses variant ordinals, so adding or removing an
  enum row cannot silently reinterpret another venue or product family.
- **Pre-1.0 IQX migration.** The retired `intelligentcross_iqx` value has no replacement
  `Exchange` variant. Remove persisted entries that used it, or keep an application-level
  mapping if the IntelligentCross ATS is still required; never translate it to another
  venue or to `Exchange::Unknown`. Earlier binary ordinal payloads for `Exchange` and
  `MarketHoursKey` must be decoded with their original crate version and rewritten using
  the canonical string representation.
- **Normal week only.** Holidays, early closes, half-days, and product-level variations are
  absent — `is_holiday` is a stub returning `false`, though every query path already routes
  through it under one session-existence contract. Verify contract specs before trading on
  a profile outside its explicitly stated scope.
- **No panics, and absence is `None`.** The public surface is total, and boundary queries
  (`session_bounds*`, `next_session_after*`, `candle_start*`/`candle_end*`,
  `time_end_of_day`) return `Option`: a profile with no session of the requested kind in
  the bounded search horizon (a venue before its go-live date), or a zero-duration candle
  resolution, is `None` — never a fabricated degenerate pair that could leak downstream as
  a real session. Predicates report closed/not-in-maintenance/closed-all-day.
- **`SessionRule` has a stated domain.** `SessionRule::new` / `validate` enforce
  `open_ssm < 86_400`, `close_ssm <= 86_400`, and at least one enabled weekday; every
  shipped table is fence-checked against the same domain. Equal endpoints are intentional
  complete-local-day rules, not empty intervals.

## Testing

Pure and stateless, so the validation class is property/deterministic-fixture, not workload.
Every test is an integration test over the public surface: the crate exposes nothing to tests
that callers do not also get (TEST-LAYOUT, see [AGENTS.md](AGENTS.md)).

- `tests/venue_sessions.rs` and `tests/venue_sessions/` — a thin harness over focused futures,
  equities, candle, wrap, bounds, and correction modules. Together they pin published opens,
  end-exclusive closes, overnight wraps, maintenance gaps, weekend boundaries, always-open
  venues, serde forms, and source-cited data corrections.
- `tests/apac_equities.rs` and `tests/apac_equities/` — a thin integration harness with
  separate modules for current boundaries, regional amendment history, and APAC bulk/name
  contracts across all 17 cash-equity venues.
- `tests/global_equities.rs` and `tests/global_equities/` — a thin harness over current
  baselines, amendment history, and the global bulk/name contract for TSX, Borsa Istanbul,
  JSE, Tadawul, B3, and BMV.
- `tests/schedule_documentation.rs` — keeps all 91 exchange rows and nine reusable
  `MarketHoursKey` rows in canonical order, validates their review metadata and owner/source
  links, requires both current and notice/evidence channels for every source set, rejects
  orphaned source sets, and prevents the README freshness date from drifting from the ledger.
- `tests/seasonal_calendars.rs` and `tests/seasonal_calendars/` — a thin integration harness
  split into B3, BMV, transition-scan, candle/weekend, Chrono-edge, and compatibility-contract
  modules. The contracts include all-fixed-venue `MarketHours`/`ExchangeCalendar` parity and
  a daily history scan proving venue time zones remain stable.
- `tests/support/` — fixture construction shared by integration targets; it uses only the
  public crate surface and provides no test-only access to production internals.
- `tests/no_session_contract.rs` — the `None` contract for profiles with no rules.
- `tests/rule_validation.rs` — the `SessionRule` domain: what `new`/`validate` accept and reject.
- `tests/contract/session_invariants.rs` and `tests/contract/session_invariants/` — a thin,
  explicitly configured harness over handwritten identity/history expectations and focused
  property modules. A fixed-seed `splitmix64` sweep plus pinned DST fixtures cover totality,
  determinism, maintenance, ordered bounds, advancing session walks, and the all-venue
  cross-query fence. The production tables are never imported into these expectations.
- `src/lib.rs` doctest — the Quick start example above.

The CI gate, in order — `cargo fmt --all --check`, `cargo clippy --all-targets -- -D warnings`
(clippy **pedantic** plus the panic-family and determinism restriction lints, configured in
`Cargo.toml` and `clippy.toml`), `cargo nextest run`, `cargo test --doc`, `cargo doc` with
warnings denied, and `cargo deny check` for dependency licences and advisories. A second job
builds on the declared MSRV so that floor is exercised rather than asserted. The build
toolchain is pinned in `rust-toolchain.toml`. Run everything the `quality` job runs:

```bash
cargo fmt --all --check && cargo clippy --all-targets -- -D warnings && cargo nextest run --all-targets && cargo test --doc && RUSTDOCFLAGS="-D warnings" cargo doc --no-deps && cargo deny check
```

The MSRV job needs a second toolchain, so it is a separate command:

```bash
cargo +1.95 check --all-targets
```

## License

[MIT-0](LICENSE) (MIT No Attribution): use, copy, modify, and redistribute freely,
with no attribution required and no warranty of any kind — see the best-effort
disclaimer above before trading on these tables.
