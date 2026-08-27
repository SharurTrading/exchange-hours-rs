<!-- SPDX-License-Identifier: MIT-0 -->

# Architecture

`exchange-hours` separates identity, sourced schedule data, schedule selection,
and time queries. That separation is intentional: adding a venue should not
require touching an algorithm, and adding a query should not require knowing
which venues have historical or seasonal rules.

## Data flow

```text
Exchange ── presets/ ───────────────┐
                                    ├─ schedules/<family>/<venue>.rs
MarketHoursKey ─ futures_profile.rs ┘  static tables + sources + revisions
                                               │
                                               ▼
                                          MarketHours
                                               │
                              ┌────────────────┴────────────────┐
                              │                                 │
                       fixed snapshot                  ExchangeCalendar
                                                      date-aware source
                                                                │
                                                       optional DayPolicy
                                                                │
                                                        PolicyCalendar
                              │                                 │
                              └────────────────┬────────────────┘
                                               ▼
                                      query/ shared engine
                               sessions, state, trade date, bars
```

All three public query surfaces use the same private engine. A fixed
`MarketHours` is borrowed exactly as supplied by the caller. An
`ExchangeCalendar` carries either an `Exchange` or a `MarketHoursKey` and
reselects a static profile for every candidate venue-local opening day. That is
required for historical cutovers, wrap sessions, and B3/BMV's New York offset
rules. `PolicyCalendar` borrows caller-owned trade-date boundary overrides and
feeds them into that same engine; it does not change `hours_at`.
`StaticDayPolicy` provides a validated allocation-free record table for
whole-date closures, late first opens, and early final closes. It is
deliberately not a replacement-session engine: holiday windows that
pause/reopen or alter regular and extended phases differently follow the
future provider contract in `docs/schedules/date-exceptions.md`.

## Source tree

```text
src/calendar/
  exchange/             Exchange table, canonical names, parsing
  exchange_calendar/    Date-aware value and its public method adapters
  futures_profile.rs    MarketHoursKey table and fixed/dated family routing
  policy.rs, policy/    DayPolicy overlay and public method adapters
  query/                One private implementation of every query algorithm
  schedules/            Sourced static rule tables and venue-local revisions
    equities/           Geographic folders are navigation only
    futures/            Operator/product-family tables where genuinely shared
    profile.rs          StaticHoursProfile and allocation-free adapter
    timeline.rs         Ordinary dated revisions and cross-zone helpers
  presets/
    historical.rs       The one exhaustive Exchange -> point-in-time routing match
  hours.rs              MarketHours value and fixed-snapshot method adapters
  state.rs              SessionState classification
  session.rs            Public fixed-session free-function adapters
  candle.rs             Public fixed-candle free-function adapters
  local_time.rs         The only local-wall-time -> UTC resolver
  rule.rs               SessionRule and SessionKind values
  resolution.rs         CalendarResolution value
  bulk.rs               Deliberate regional membership lists
```

Most modules are private and the crate root re-exports the stable public API.
Internal folders can therefore evolve without changing downstream import paths.

## Schedule ownership

A venue or product-family module owns:

- literal `SessionRule` slices and `StaticHoursProfile` values;
- the primary-source comments supporting every literal and the exact venue,
  segment, or product-family scope those literals describe;
- its `profile_at` selector — a trivial one-profile selector for venues with
  no recorded change, a dated `Revision` timeline otherwise, so a venue's
  first future revision is a purely local edit.

The repeatable evidence workflow is documented in
[Updating exchange schedules](docs/schedules/updating.md). The
[verification ledger](docs/schedules/verification.md) maps every public venue
to its owner and source-set IDs; the [source registry](docs/schedules/sources.md)
normalizes stable monitoring entry points. Exact dated evidence remains beside
the literals it proves. `MarketHoursKey` remains product-neutral: mapping a
symbol, root, product code, or MIC to a family belongs to the caller.

Use an operator-family module only when schedules genuinely share sourced rule
data. Mere equality today is not enough: unrelated venues retain named profiles
so one can diverge without untangling another.

Ordinary histories are ascending static `Revision` timelines. Temporary regimes
are a start row followed by a restoration row. B3 and BMV keep custom selectors
because their current grids depend on the UTC-offset relationship with New York;
those selectors still return the same static profile type.

## Deliberate independent fences

Some repeated wiring is a correctness check, not architecture debt. Keep these
independent:

- the `Exchange` declaration/name table;
- the `MarketHoursKey` declaration/name table;
- the exhaustive routing match in `presets/historical.rs`;
- fixed and dated product-family routing;
- regional membership in `bulk.rs`;
- the handwritten exchange/key lists, counts, and historical cutover tables in
  the integration contracts.

This means a new venue is intentionally not a literal one-file change. Its
schedule and future amendments have one owner module, while the compiler and
independent tests force explicit product and coverage decisions. The complete
checklist lives in [AGENTS.md](AGENTS.md#adding-or-revising-a-venue).

## Query invariants

`query/` has one private concrete context with fixed and date-aware schedule
sources plus an optional borrowed day policy. Do not add a second algorithm
implementation for a public facade. Extend the context only when a genuinely
different schedule source or overlay is needed.

The shared engine must preserve:

- UTC timestamps at every public boundary;
- end-exclusive closes;
- equal rule endpoints as one complete local-day session, never an empty rule;
- opening-day selection for wrap and future-session scans;
- earliest resolution for opens and latest resolution for closes;
- trade-date policy applied to the complete effective trading day, including
  its prior-evening wrapped portion, except for an explicitly sourced
  following-business-day assignment such as CME cryptocurrency's weekend
  roll-forward across a policy-closed Monday;
- one mutually exclusive state, with same-trade-date gaps as halts except for
  sourced short maintenance on a continuously traded-week profile, and only
  four-hour-bounded same-week inter-trade-date gaps as ordinary maintenance;
- bounded, total behavior at Chrono's minimum and maximum instants;
- `None` for unavailable boundaries rather than fabricated intervals.

## Tests

Integration-test roots are thin harnesses. Growing suites live in same-named
directories and are split by venue or behavior. Shared fixtures in
`tests/support/` use only the public crate surface, preserving TEST-LAYOUT.

The independent all-exchange and all-cutover contracts should never import
production schedule tables. Their duplication is what detects missing wiring,
wrong zones, or a selector that skipped a historical regime.
