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
  only. A venue that merely coincides with another still gets its own named
  profile so a future divergence is a one-line edit.
- **LAW-NO-FABRICATED-DATES** — a historical cutover exists only when a primary
  source states a **day-level** effective date. A real change without one is
  documented as a known gap (see MEMX / MIAX Pearl in `presets/historical.rs`),
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
- **`open_ssm > close_ssm` wraps** past local midnight — the only way a session
  crosses a day boundary.
- **DST bias is asymmetric on purpose**: opens resolve earliest, closes latest.
  Never "simplify" this to a single bias.
- **Regular vs extended**: continuous trading is `regular`; auction call
  windows, order-entry-only phases, and post-close/trade-at-last sessions are
  `extended`. Lunch breaks are gaps between regular rules, not rules.
- **Absence is `None`.** Boundary queries return `Option`; never fabricate a
  degenerate session. No public code path may panic or hang.

## Structural rules

- The `Exchange` match in `presets/current.rs` is exhaustive with **no
  catch-all arm** — adding a variant must be a compile error until its hours
  are decided. Keep it that way.
- A new `Exchange` variant must also be added to `ALL_EXCHANGES`, `is_listed`,
  and `EXCHANGE_VARIANT_COUNT` in `tests/contract/session_invariants.rs`, and
  to the region list in `exchange.rs` if a bulk builder covers it.
- Serde wire format is stable: variants serialize as `snake_case` strings;
  renames that change a serialized string break persisted data.
- Production files stay under 300 lines (500 is a hard stop — split by operator
  family, as `profiles/equities_eu/` does). Test files are exempt.
- Profile tables are `static` so `MarketHours` can borrow them allocation-free.

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
  `panic` in `presets/historical.rs`). Tests are exempt from the panic-family
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
  data corrections go under **Fixed**, new venues under **Added**.
- The README badges (version, MSRV) and the Coverage table's venue counts are
  duplicated facts — update them together with `Cargo.toml` and the enum.
