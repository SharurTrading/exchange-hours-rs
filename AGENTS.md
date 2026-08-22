<!-- SPDX-License-Identifier: MIT-0 -->

# AGENTS.md — conventions for humans and coding agents

This file defines the named laws referenced in code comments (they originated in
the Sharur v1 monolith this crate was extracted from; this is now their home)
and the rules any change to this repository must follow.

## Laws

- **LAW-DETERMINISM** — library code never reads a clock, performs I/O, or uses
  randomness. Same inputs, same answer, forever. Enforced structurally where
  possible: `chrono` is built with `default-features = false` so `Utc::now()`
  does not compile here. Do not add a dependency or code path that breaks this.
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
- **LAW-NO-FABRICATED-DATES** — a historical cutover exists only when a primary
  source states a **day-level** effective date. A real change without one is
  documented as a known gap (see IntelligentCross IQX and the central Euronext
  markets in their venue modules under `src/calendar/schedules/`),
  never given an invented date. Amendment history is recorded back to
  **January 2010**; earlier changes are out of scope by design.

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
- **Regular vs extended**: continuous trading is `regular`; auction call
  windows, order-entry-only phases, and post-close/trade-at-last sessions are
  `extended`. Lunch breaks are gaps between regular rules, not rules.
- **Absence is `None`.** Boundary queries return `Option`; never fabricate a
  degenerate session. No public code path may panic or hang.

## Structural rules

- `Exchange` and `MarketHoursKey` are `#[non_exhaustive]`: venue additions are
  minor releases, not breaking ones. Never remove that attribute, and never
  remove or rename a variant outside a major release.
- `Exchange`, `Exchange::ALL`, and `Exchange::as_str` are generated from
  **one table** (the `exchanges!` invocation in `exchange/mod.rs`, using the
  macro in `exchange/define.rs`): adding a venue is
  one new row, and neither `ALL` nor the name table can omit it. The compiler
  then forces the remaining in-crate exhaustive match, `hours_for_exchange`
  in `presets/current.rs` (no catch-all arm — the hours decision). The same
  edit must also reach the region list in `bulk.rs` when a bulk builder
  covers the venue, and `ALL_EXCHANGES` + `EXCHANGE_VARIANT_COUNT` in
  `tests/contract/session_invariants/identity_expectations.rs` — the test
  suite's independent expectation of the table's contents and order.
- One canonical `snake_case` name per venue, shared by serde, `as_str`,
  `Display`, and `FromStr`, and it is stable: a rename breaks persisted data.
  `FromStr` rejects unknown names with `ParseExchangeError` — never map bad
  input to `Exchange::Unknown`.
- Production files stay under 300 lines (500 is a hard stop — split by venue or
  operator family under `schedules/`). Test files are exempt.
- Schedule profile tables are `static` so `MarketHours` can borrow them
  allocation-free.

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
3. **Routing.** Update the no-catch-all `hours_for_exchange` match with an
   explicit current-profile decision. Add point-in-time routing only for
   primary-sourced revisions; document an unsourced gap instead of inventing a
   date. A cross-zone or otherwise recurring selector also needs date-aware
   `ExchangeCalendar` transition coverage.
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
   `CHANGELOG.md`.
8. **Verification.** Run the complete quality and MSRV commands below. A
   focused venue test is useful while iterating, but it does not replace the
   all-venue contracts.

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
  data corrections go under **Fixed**, new venues under **Added**. A PR that
  bumps the version in `Cargo.toml` is a release cut: that PR retitles
  `[Unreleased]` to the dated version section (as the 0.1.0 and 0.2.0
  release-prep PRs did), and the tag follows the merge.
- The README's version and MSRV badges read from crates.io, so they cannot
  drift; do not re-hardcode them. The Coverage table's venue counts still
  duplicate the enum — update them together. When a dependency or an
  observable behaviour goes away, grep `README.md` for it as well: the
  `tracing` removal left two stale claims there and shipped in `0.2.1`.
