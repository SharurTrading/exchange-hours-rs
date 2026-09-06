<!-- SPDX-License-Identifier: MIT-0 -->

# AGENTS.md — conventions for humans and coding agents

This file defines the named laws referenced in code comments and the rules any
change to this standalone repository must follow.

## Laws

- **LAW-DETERMINISM** — library code never reads a clock, performs I/O, or uses
  randomness. Same inputs, same answer, forever. Enforced structurally where
  possible: `chrono` is built with `default-features = false` so `Utc::now()`
  does not compile here. Do not add a dependency or code path that breaks this.
- **LAW-PANIC** — production queries are total: they do not panic, hang, or use
  unreachable fallbacks. Invalid raw `SessionRule` values may return
  unspecified answers, but still return normally; absence and bounded-search
  exhaustion are explicit.
- **STYLE-LOG** — library code emits no output. Diagnostics belong to callers;
  tests may print only when it makes a failure reproducible.
- **TEST-LAYOUT** — every test is an integration test over the public surface.
  Tests get nothing callers do not also get: no `#[cfg(test)]` back doors, no
  test-only methods on production types, no `pub(crate)` leaks for test access.
- **LAW-PRIMARY-SOURCES** — every session time in a profile table is backed by a
  primary source (the exchange's own site, rulebook, notice, or regulator
  circular), cited in a comment next to the table. Secondary sources corroborate
  only. Stable monitoring entry points live in
  `docs/schedules/sources.md`; exact effective-date evidence stays beside the
  table. A venue that merely coincides with another still gets its own named
  profile so a future divergence is a one-line edit.
- **LAW-PUBLIC-SOURCES** — every cited source must be publicly available
  without authentication. The crate encodes the schedule facts a published
  document states and cites it. Material behind a member portal or an
  authenticated feed is out of scope as a data source, though its existence and
  publication date may be cited as evidence that a change occurred when the
  contents are unavailable.
- **LAW-NO-FABRICATED-DATES** — a historical cutover exists only when a primary
  source states an **unconditional, day-level** effective date. A real change
  without one is documented as a known gap, never given an invented date. A
  future date contingent on a readiness filing, regulatory condition, or
  infrastructure rollout stays in the update-guide watch list and out of
  runtime selectors. An unconditional, fully sourced future date **may** be
  encoded ahead of its effective day, so instant-driven queries roll over
  with no release in between; the row's review date is the day the evidence
  was checked, and a slipped or cancelled change is corrected as a schedule
  fix. Day-level revision rows are keyed to the local
  **opening day** of the first session they govern, and a day-level revision
  boundary never splits a running session; a sourced change whose boundary
  falls at a stated intraday instant is an exact-instant cutover, never a
  day-level row rounded to local midnight. Amendment history is recorded back
  to **January 2010**; earlier changes are out of scope by design.
- **LAW-SESSION-NOT-EXPIRY** — an instrument's **termination of trading,
  expiration, settlement, marker or fixing instant is never a session
  boundary**, and this crate does not model it at any resolution. The crate
  answers when a *market* accepts and matches orders. When an individual
  *contract* stops trading is instrument-lifecycle data, owned by the caller's
  catalog and its provider adapters, and a profile that encodes one has
  answered a question it was not asked.

  A daily close enters a profile only when the operator states it in **session
  language** — "trading halts", "trading ceases daily at", "a daily
  maintenance period from X to Y", a `TRADING HOURS` row, or a venue feed's
  own close/halt event. A **calculation window** ("the marker is calculated
  from trade data from 12:34-12:35"), an **order-entry cutoff**, or a
  **termination-of-trading row** is none of those, even when it is printed
  adjacent to the hours and even when it is the only time on the page.

  The tell is a table in which several products share one hours cell while
  each carries its own end time: the shared cell is the session and the
  per-product times are expiries. CME event contracts are the worked example —
  `SER-9624` lists five contracts with five different Termination-of-Trading
  times against a single `CME Globex` cell, and reading those five as daily
  closes would have produced six product families where the operator publishes
  one. Every trade-type shape surveyed in
  `docs/plans/2026-09-05-cme-trade-type-handoff.md` closes at an instant that
  is *also* a settlement or fixing instant for its asset class, so the
  coincidence proves nothing on its own.

  A source that states only an expiry leaves the session **unsourced**: withhold
  it under LAW-PRIMARY-SOURCES rather than promoting the expiry to a close.
- **LAW-HOLIDAY-SCOPE** — a change confined to a single trade date (or a
  bounded holiday run of dates) — an early final close, a late first open, or
  a full calendar-day closure — is a **holiday**, not a schedule. Holidays are
  caller-owned date exceptions: they belong to the `DayPolicy` overlay and the
  `docs/schedules/date-exceptions.md` provider contract, never to a profile
  table, revision row, or template edit. Normal-week templates and their dated
  revisions encode only real exchange behavior changes — sourced, persistent
  changes to the recurring week. Do not bend a template, add a revision row,
  or delete a valid phase to absorb a single-day event, and do not downgrade
  a genuine recurring-grid change to "just a holiday" to avoid the evidence
  work.

- **LAW-FOLLOW-UPS-ARE-ISSUES** — a follow-up named anywhere — in a pull
  request, a commit message, a review reply, a ledger row, a plan, or a
  research note — is either done in that same change or opened as a GitHub
  issue before the change merges, and the issue number is cited where the
  follow-up is named. "Separate small PR", "later", "out of scope here" and
  "not this PR" are not tracking; the issue is. This includes a source gap the
  change discovers in a neighbouring row (a notice that also touches another
  key's channel), a channel found exhausted, and a question a reviewer raised
  that was answered with "follow-up". A follow-up that is never opened is a
  claim that it was never needed, and the next reader will believe it.

## Modeling conventions

- **UTC in, UTC out.** Every *timestamp* crossing the public boundary is a
  `DateTime<Utc>`: a local time is never a parameter or a return value, and the
  venue's zone stays an internal detail. This constrains which time types may
  appear, not signatures in general — public functions legitimately take an
  `Exchange` and return a `MarketHours` with no timestamp involved.
  Session rules are seconds-since-local-midnight in the venue's own IANA zone.
- **Closes are end-exclusive.** An instant equal to a close is closed.
- **`open_ssm >= close_ssm` wraps** into the next local day. Equal endpoints
  encode one complete local-day span; omit a rule to express no session.
- **DST bias is asymmetric on purpose**: opens resolve earliest, closes latest.
  Never "simplify" this to a single bias.
- **Regular vs extended.** A venue's primary/core continuous session, or a
  derivatives operator's explicitly published regular-trading-hours (`RTH`)
  session, is `regular`. Electronic or overnight trading outside that RTH may
  be `extended` when the owner scope says so. Auction calls, pre-open and other
  order-entry-only windows, and post-close/trade-at-last sessions are always
  `extended`. Lunch breaks are gaps between regular rules, not rules.
- **Cash-equity venue envelope.** An `Exchange` cash-equity profile is the
  availability union of the venue's automated order-capable systems within the
  row's documented scope; it does not claim every listed security is eligible
  for every phase. Include executable and accepted order-entry phases. Exclude
  pure reporting, cancellation-only, enquiry, and administrative states, and
  systems that are separately modeled identities.
- **Product-neutral family selection.** `MarketHoursKey` names a sourced
  schedule family. This crate never maps symbols, roots, product codes, or MICs
  to keys; that belongs to a caller's instrument catalog. A venue-keyed default
  is not permission to use that clock for every product listed there.
- **Trade dates and state.** A containing session's trade date is normally the
  venue-local date of its final close. Same-trade-date gaps are `Halt`, except
  that a sourced continuously traded-week profile can retain an
  operator-designated gap of no more than four hours as `Maintenance`.
  Inter-trade-date gaps no longer than four elapsed hours within one ISO week
  are also `Maintenance`; longer gaps are `Closed`. `is_maintenance` must
  remain exactly the maintenance case of `session_state`.
- **Caller-owned day overrides.** Built-in profiles contain normal-week data,
  not holidays (LAW-HOLIDAY-SCOPE: a single-trade-date early close, late
  open, or closure is always a date exception, never a template change).
  `DayPolicy` is keyed by trade date and overlays closed dates,
  early final closes, and late first opens on queries; `StaticDayPolicy`
  standardizes validated hard-coded records. Neither mutates the profile
  returned by `hours_at`. A closed date normally removes its complete trading
  day, including the prior-evening wrap. Preserve a sourced family exception
  when the operator assigns continuous weekend trading to the following open
  business date; CME cryptocurrency rolls a closed Monday's weekend block into
  Tuesday instead of deleting it. A special day that changes internal phase
  topology is not representable by scalar boundaries: do not force it into a
  normal profile or delete a valid phase. Follow
  `docs/schedules/date-exceptions.md` for the richer provider contract.
- **Exchange-level boundaries, not per-security auction outcomes.** When an
  operator publishes a nominal phase boundary but randomizes the actual
  auction uncross per security or trading group, the venue owner documents the
  deterministic representation it uses. Prefer the published nominal boundary
  when the randomized seconds only move an adjacent phase handoff and do not
  change the exchange-wide open/closed envelope; use a conservative envelope
  when that is the profile's stated venue-level scope. Never imply exact
  ticker-level uncross timing.
- **Below the January-2010 floor, the earliest sourced profile stands.**
  `select_revision` returns a venue's baseline for any date before its first
  revision, so an instant before the audit floor resolves to that venue's oldest
  profile on record rather than to an absence — which for a launch-dated venue is
  its pre-launch closure, and for others the earliest grid the crate holds. That
  is deliberate, decided 2026-09-01: the profile returned is the oldest state the
  crate has sourced for that venue, and the caller
  asked for a date this crate never undertook to model — amendment history is
  recorded back to January 2010 and earlier changes are out of scope by design.
  Do not add a lower bound to the timelines. Callers needing pre-2010 behaviour
  must not read these answers as sourced; nothing below the floor is reviewed,
  and the profile is simply the oldest one on record.
- **Executable windows are the priority.** A gap in a phase where a trade can
  print — `regular` or `extended` — is materially more serious than a gap in an
  `order_entry` window. The first changes whether the crate says a market was
  tradeable; the second only changes whether orders could be queued ahead of an
  open that is itself modelled correctly. Two obligations follow. When ordering
  evidence work, close executable-hours gaps first, however many more rows the
  queue-onset gaps touch — row count is not impact. When recording a gap, say
  which kind it is: "this row is `Partial`" carries very different weight for a
  missing trading session than for an undated queue start, and a reader cannot
  tell them apart from the basis label alone.
- **Carry the earliest sourced state back to the audit floor.** When a phase is
  primary-sourced at some instant and no primary source names a cutover between
  the January-2010 floor and that instant, extend it backwards to the floor
  rather than modelling the interval as sessionless. This asserts *no* revision
  row, so it does not fabricate a date — it declines to invent one, exactly as
  ICE Sugar's August-2011 baseline is carried back. Modelling a venue that was
  demonstrably trading as closed is a worse error than carrying its earliest
  sourced grid: absence is a claim too. Where a third-party (non-primary) source
  attests a change inside the carried interval, carry the state anyway and state
  the residual risk beside the table.
- **Prefer the sourced intersection to omission.** When a phase's endpoints are
  sourced at two different values and only the changeover day is undated, serve
  the window that holds under *every* sourced state and withhold only the
  disputed remainder. CME's Sunday Pre-Open is the worked example: sourced at
  16:15 at the floor and 16:00 currently, so 16:15-17:00 is carried from the
  floor with no cutover asserted and only the 16:00-16:15 quarter-hour waits on
  the undated 2012 move. Dropping the whole phase because part of it is disputed
  under-reports the venue far more than the uncertainty warrants. The same rule
  governs a *source conflict* — two primaries describing one period differently,
  neither superseding the other: serve what both state, withhold only what they
  dispute, and record the conflict beside the table. `ECBTC` after 2026-05-29 is
  that worked example: SER-9740R and CME's own client-systems wiki disagree by
  an hour on the weekday close and agree on everything else, so its key serves
  the agreed 16:02→15:00 CT window. Establish the order of the statements first
  (a page's version history, a document's own date), and do not count a later
  document that merely reproduces an earlier one — a rule filing reprinting a
  SER as its exhibit — as a second statement or a tie-breaker.
- **A knowledge boundary is the first source that lists the modelled product,**
  not merely the earliest source that survives. A document only sources a family
  if that family appears in it. SGX's FTSE Taiwan suite is the worked example,
  and the worked example of getting this half right: the 2020 calendar edition
  is the earliest calendar that survives, but it lists only the MSCI Taiwan
  predecessors, so the row cannot start there - and it does not start at the
  2021 edition either, because SGX's own content API lists the FTSE suite with
  its full grid from July 2020 and SGX's own release states the launch day,
  five months before that edition. The predecessor's hours were identical,
  which is exactly why the first half is easy to miss; one channel's earliest
  surviving document is not the family's earliest source, which is exactly why
  the second half is. Check the contract set, not just the grid, before keying
  a row to an edition - and check the other channels before concluding an
  edition is the earliest source at all.
- **An operator's own dated change log is a primary source for an effective
  day.** The law asks whether a primary source states an unconditional,
  day-level effective date, not what genre of document carries it. A
  product-parameter change log the operator authors, dates in the file and
  serves publicly satisfies that test when four things hold: the entry is the
  operator's (authored and issue-dated in the document itself, not by a
  reader); the effective day is stated inside that same entry - in the same
  cell, or in a header row that scopes the items below it within one entry
  whose structure is encoded in the file (border-partitioned rows with zero
  merges) and that is the operator's own recurring convention there; it is
  stated in session language; and the log is calibrated against at least one
  cutover the crate already holds from a circular. SGX's *Derivatives Products
  Description* `read_me` sheet is the worked example: its entries "(eff 4
  Nov)" and "(eff 7 Apr)" match DT/AM 50 of 2024 and DT/AM 15 of 2025 to the
  day, so its "Amended trading hours for SGP, SGPO and ST eff 10 Jun" (issued
  2019-05-21) and "Effective 11 Nov: … (T+1) session Closing hours to 5:15am
  all T+1 traded contracts" (issued 2019-10-07) date the two 2019 SGX moves
  that no reachable circular states. Record the interpretive step beside the
  row where a header scopes the items rather than sharing their cell. A bare
  day-and-month in an operator's log means the entry's own year, provided the
  log spells the year out whenever a day sits near a year boundary - SGX does
  ("eff 22 Dec 18" issued 2018-12-18; "eff 21 Jan 19" issued 2019-01-07; "eff
  13 Jan 2020" issued 2019-12-23), and every bare day in its sheet falls
  within months of its issue date, so its "eff 10 Jun" of 2019-05-21 is
  2019-06-10; a log that never spells a year gives the reader none, and the
  entry fails the same-entry test. What this
  does not admit: a change log a reader must date from formatting alone, an
  entry whose day must be inferred from a month, filename or directory, or a
  member firm's own summary of the operator's log.
- **A knowledge boundary may only widen.** Intersecting per interval can make a
  bound move *inward* at a boundary that asserts no change - three sourced
  states in sequence, the middle one directly witnessed on a date, will do it.
  Do not encode that. Knowledge does not shrink: a boundary that widens says
  "from here we have sourced this much", and one that narrows says a window
  that was open became closed, which is a claim about the market that
  LAW-NO-FABRICATED-DATES requires a stated day for. Where the per-interval
  intersections would narrow a bound, hold it at its narrowest value across
  the whole undated span and withhold the disputed remainder from the earlier
  interval, exactly as CME's Sunday Pre-Open withholds 16:00-16:15 rather than
  granting it early and withdrawing it at the capture that first shows the
  later state. SGX's FTSE China A50 is the worked example: its T+1 open is
  sourced at 16:10, 16:40 and 17:00 across three undated states, so 17:00 is
  served from the floor and only its T close, which widens, is keyed at the
  boundaries. The same asymmetry decides the day: a row that lengthens or
  creates a wrapping overnight close is keyed to the following Monday, because
  the selector reads the query instant's local date and a mid-week key would
  report the previous evening's leg running past the close that was in force
  when it opened - the running session LAW-NO-FABRICATED-DATES says a boundary
  must not split. A row that changes only daytime bounds, or that narrows an
  overnight open, keeps its artifact's own date.
- **"Unsourced" means "not worked up", never "no source exists".** Say a row is
  unmodelled before an era and say what would close it; do not write that
  nothing survives unless the predecessor channels have actually been searched.
  Operators retire sites, and the web archive keeps what the live site dropped.
  This repository shipped "no edition of the calendar survives before it" for
  SGX while the operator's own pre-2020 hours pages sat in the archive stating a
  materially different grid. A sessionless era is a safe answer either way, but
  the stated reason has to be true.
- **Absence is `None`.** Boundary queries return `Option`; never fabricate a
  degenerate session. No public code path may panic or hang.

## Structural rules

- `Exchange` and `MarketHoursKey` are `#[non_exhaustive]`: venue additions are
  minor releases, not breaking ones. Never remove that attribute, and never
  remove or rename a variant outside a major release. A breaking identity or
  wire-format change requires explicit authorization, an appropriate SemVer
  release, a changelog migration note, and exhaustive
  production/test/documentation cleanup; never perform one as drive-by cleanup.
- `Exchange`, `Exchange::ALL`, and `Exchange::as_str` are generated from
  **one table** (the `exchanges!` invocation in `exchange/mod.rs`, using the
  macro in `exchange/define.rs`): adding a venue is
  one new row, and neither `ALL` nor the name table can omit it. The compiler
  then forces the remaining in-crate exhaustive match — `hours_for_exchange`
  in `presets/historical.rs` (no catch-all arm — the routing decision: every
  venue names its `profile_at` selector). The same
  edit must also reach the region list in `bulk.rs` when a bulk builder
  covers the venue, and `ALL_EXCHANGES` + `EXCHANGE_VARIANT_COUNT` in
  `tests/contract/session_invariants/identity_expectations.rs` — the test
  suite's independent expectation of the table's contents and order.
- `MarketHoursKey`, `MarketHoursKey::ALL`, `MarketHoursKey::as_str`, and its
  serde implementations likewise come from the single `market_hours_keys!`
  table in `futures_profile.rs` / `futures_profile/key_serde.rs`. A new key
  must also reach the handwritten key expectations, verification ledger, and
  public profile tests; never generate those independent fences from `ALL`.
- One canonical `snake_case` name per identity, shared by serde, `as_str`,
  `Display`, and `FromStr`, and it is stable: a rename breaks persisted data.
  `Exchange` and `MarketHoursKey` Serde use their canonical strings in every
  format; never restore derive-generated enum ordinals, because inserting or
  removing a variant would silently remap binary payloads. Their `FromStr`
  implementations reject unknown names with `ParseExchangeError` or
  `ParseMarketHoursKeyError`—never map bad input to `Exchange::Unknown`.
- Production source files stay cohesive and reviewable, ordinarily at or below
  500 lines. This is a source-reviewability guard, not a repository or packaged-
  crate size limit: split independent responsibilities (for example, by venue or
  operator family under `schedules/`), but do not fragment a coherent module
  merely to satisfy a smaller arbitrary line count. Test files are exempt.
- Schedule profile tables are `static` so `MarketHours` can borrow them
  allocation-free.
- `ExchangeCalendar` represents either `CalendarSource::Exchange` or
  `CalendarSource::MarketHoursKey`. Both sources must support the same
  date-aware query surface. Keep the calendar `Copy + Send + Sync + 'static`
  and built-in hot-path queries allocation-free over bounded rule scans.
- Identity-dependent topology belongs on the date-aware identity calendar. A
  detached `MarketHours` snapshot must remain exactly the caller-supplied rule
  set; never guess a family from coincident rules to coalesce sessions or assign
  a special trade date. Always-open calendars have no final close and return no
  trade date.
- **Instant-only selection.** Every public entry point that resolves an
  identity to a schedule for a moment in time requires the caller's instant —
  `hours_for_exchange(exchange, as_of)`, `hours_for_market_hours_key(key,
  as_of)`, and the bulk builders all carry it — and there is no clock-less
  "current" routing between eras. `session_profile` remains the one static
  current-*table* accessor (it selects no era and equals the timelines'
  selection at any instant on or after the knowledge-bound rows). This is
  what makes backtest and live one code path; do not reintroduce a second
  selector. The public fixed-snapshot query adapters (`session_bounds*`,
  `candle_*`, `next_session_*`) remain compatibility contracts: do not remove
  or silently redirect them while adding calendar methods.

## Adding or revising a venue

Treat the following as one change set. The repeated expectations in production,
tests, and documentation are deliberate coverage fences; do not generate the
handwritten test lists from production data.

1. **Identity.** Add the canonical enum row in the `exchanges!` table, with a
   stable `snake_case` wire name and public variant documentation. Never rename
   an existing row as part of an hours correction.
2. **Schedule data.** Add or revise the venue-owned static profile beside its
   primary-source citations. Start from the venue's source-set IDs in
   `docs/schedules/verification.md` and follow
   `docs/schedules/updating.md`. Give a venue its own named profile even when
   its hours currently coincide with another venue. Keep historical revisions
   and their day-level effective-date sources with the same venue family.
3. **Routing.** Add the venue's arm to the no-catch-all `hours_for_exchange`
   match in `presets/historical.rs`, dispatching to its `profile_at` selector.
   Add dated revision rows only for primary-sourced, unconditional revisions;
   document an unsourced gap as a knowledge-bound row at the review date
   instead of inventing a date, and monitor a conditional future announcement
   without routing it. A cross-zone or otherwise recurring selector also needs
   date-aware `ExchangeCalendar` transition coverage.
4. **Regional membership.** Add the venue to the appropriate `bulk.rs` region
   list when a built-in bulk builder should include it, preserving that list's
   documented stable order.
5. **Independent contracts.** Update `ALL_EXCHANGES` and
   `EXCHANGE_VARIANT_COUNT` in
   `tests/contract/session_invariants/identity_expectations.rs`. Add every
   observable dated profile change to `HISTORICAL_CUTOVERS` in
   `tests/contract/session_invariants/historical_expectations.rs`, including
   restoration boundaries for temporary schedules. Add a source-stated
   intraday boundary to `HISTORICAL_INSTANT_CUTOVERS` instead of rounding it to
   local midnight. For a wrap, record the local opening day (often Sunday for
   a Monday trade-date change). These lists must remain handwritten so they
   can catch production omissions.
6. **Public-surface tests.** Add a per-venue baseline for the published open,
   the instant before it, regular/extended classification, every lunch or
   maintenance gap, the end-exclusive close, the weekend boundary, and the
   serde form. Test both sides of every recorded cutover at venue-local
   midnight. Put growing suites behind a thin top-level integration harness and
   feature/venue submodules; shared fixtures may use only public APIs.
7. **User-facing records.** Update the README Coverage counts and relevant
   limitations, the venue row in `docs/schedules/verification.md`, and its
   source sets when necessary. Advance the README repository cutoff only after
   every real venue row has been reviewed through that date. Record a new venue
   under `[Unreleased]` / **Added** or a schedule correction under **Fixed** in
   `CHANGELOG.md`. If a version has already been cut but is not yet tagged or
   published, record final preparation fixes in that pending version section.
8. **Verification.** Run the complete quality and MSRV commands below. A
   focused venue test is useful while iterating, but it does not replace the
   all-venue contracts.

## Adding or revising a product-family key

Treat a `MarketHoursKey` change with the same evidence discipline as a venue:

1. Add the stable canonical identity to the single `market_hours_keys!` table;
   never add an in-crate symbol-to-key mapper.
2. Give the family its own sourced static profile and history. A member product
   listing after the family clock began remains caller catalog data unless it
   changes the family schedule itself.
3. Update the key's `hours_for_market_hours_key` routing arm and the date-aware
   `calendar_for_market_hours_key` surface; never borrow an unrelated family's
   history merely because today's hours coincide.
4. Update every handwritten key list, wire/serde fence, baseline and cutover
   test, verification row, source registry, README count/scope, and changelog.
5. Document any venue-keyed compatibility default that uses the profile and
   explicitly warn which other products it does not cover. Unsupported
   families remain rejected rather than mapped to the nearest key.
6. Run the complete quality, MSRV, documentation, and publish-dry-run gates.

## Lints and toolchain

- **Clippy pedantic is the floor**, plus restriction lints that encode the laws:
  `unwrap_used` / `expect_used` / `panic` / `todo` / `unreachable` (LAW-PANIC),
  `print_stdout` / `print_stderr` / `dbg_macro`, and the `disallowed_*` family
  (LAW-DETERMINISM). Configured in `Cargo.toml` `[lints]` and `clippy.toml`.
  CI runs `-D warnings`, so a pedantic warning fails the build.
- **`missing_docs` is denied** — every public item, including every enum
  variant, carries a doc comment.
- **Suppressions are `#[expect(..., reason = "...")]`, never bare `#[allow]`**,
  and only where the lint is wrong for a stated reason (see the const-eval
  `panic` in `schedules/timeline.rs`). Tests are exempt from the panic-family
  lints via the `allow-*-in-tests` switches in `clippy.toml`.
- **Toolchain** is pinned in `rust-toolchain.toml` to the version the consuming
  platform builds with. That is the *build* toolchain; the *minimum supported*
  version is `rust-version` in `Cargo.toml` and is exercised by the `msrv` CI
  job. `rust-version` (1.95) is deliberately above the language floor the code
  actually requires (1.88, set by its `let`-chains). Raising either is a
  deliberate change: update `rust-version`, the CI matrix, and the README badge
  together.
- **CI runs on GitHub-hosted runners only** — `runs-on` names an explicit image
  (`ubuntu-24.04`), never a self-hosted label.
- **Dependencies** are checked by `cargo deny` against the allow-list in
  `deny.toml`; a new dependency whose licence is not listed fails the gate.

## Verification

Run before claiming any change is done — this is every check the `quality` CI
job runs, in the same order:

```bash
cargo fmt --all --check && cargo clippy --all-targets -- -D warnings && cargo nextest run --all-targets && cargo test --doc && RUSTDOCFLAGS="-D warnings" cargo doc --no-deps && cargo deny check
```

The MSRV job builds on a second toolchain, so it is a separate command:

```bash
cargo +1.95 check --all-targets
```

New venues need per-venue baseline tests (published open, minute before, the
end-exclusive close, lunch/maintenance gaps, weekend boundary, serde form) and
`as_of` tests on both sides of every recorded cutover.

## Housekeeping

- Record user-visible changes under `[Unreleased]` in `CHANGELOG.md`; session
  data corrections go under **Fixed**, new venues under **Added**. If a version
  has been cut but not tagged or published, final preparation fixes belong in
  that pending version section. A PR that bumps the version in `Cargo.toml` is
  a release cut: that PR retitles
  `[Unreleased]` to the dated version section (as the 0.1.0 and 0.2.0
  release-prep PRs did), and the tag follows the merge. Follow
  [`RELEASING.md`](RELEASING.md) for the cut, tag, publish, and verification
  sequence.
- The README's version and MSRV badges read from crates.io, so they cannot
  drift; do not re-hardcode them. The Coverage table's market-identity counts still
  duplicate the enum — update them together. When a dependency or an
  observable behaviour goes away, grep `README.md` for it as well: the
  `tracing` removal left two stale claims there and shipped in `0.2.1`.
