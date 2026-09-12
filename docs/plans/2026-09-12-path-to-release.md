<!-- SPDX-License-Identifier: MIT-0 -->

# Path to the 1.0 release — a plan a smaller agent can execute

Written 2026-09-12 (UTC) at the end of the charter session. This is the hand-over:
everything below is doable one bounded pull request at a time, by an agent without the
context of that session, following `AGENTS.md` (the charter) and this plan.

## 0. How to use this plan

1. Read `AGENTS.md` in full first, every time. It is short and it is the law.
2. Read the last twenty lines of `STATUS.md` in the research store
   (`$EXCHANGE_HOURS_RESEARCH`, default `../exchange-hours-research`) — it records what
   was in flight when the previous session stopped and where each artifact is.
3. Do one stage per pull request. A PR is sized to a working day (LAW-BOUNDED-WORK). Do
   not launch multi-agent research programmes; where this plan says "repair X", X is
   named precisely and is a few hours of work.
4. Before every push: `cargo fmt --all --check && cargo clippy --all-targets -- -D warnings && cargo nextest run --all-targets && cargo test --doc && RUSTDOCFLAGS="-D warnings" cargo doc --no-deps && cargo deny check && cargo +1.95 check --all-targets`.
   Run it as a bash script (the shell is zsh; `PIPESTATUS` differs).
5. CodeRabbit reviews every PR. Fix what it finds when it is right; when it asks for a
   schedule change inside a documentation PR, record the point as a gap in the
   identity's evidence file (`docs/evidence/<owner>.md`) and say so in the reply;
   resolve every thread before merging.
6. Every date you write about the repository's own work is the UTC date from `date -u`
   (LAW-UTC-DATES). Exchange effective days stay venue-local.
7. Stop and ask the maintainer only at the points marked **DECISION**.

## 1. Where things stand

- `main` is at the merge of #87 (ledger reshape + `docs/evidence/`). The charter (#84),
  the five dormant metals TAS keys (#83) and the fences (#78) are on main.
- Open PR: #96 `holiday-tables`, base `main` — the holiday engine plus 427 rows over
  22 identities (the eight served CME families for 2025-01-01..2027-12-31 at T2; CFE,
  Eurex, ICE Futures U.S. and Coinbase Derivatives from 2026 at T1), 749 tests, review
  findings fixed in `c9a48cd`. See stage 1.
- Open issues: #79 (served-key gap: the undated 2012 Sunday Pre-Open move on
  `globex_equity_index`), #77 (`session_profile` docs are false for seasonal keys), #36
  (system-coverage audit, phase 1 done).
- Research store map: `architecture-review/` (the review and its four fact reports),
  `holidays/` (seven verified holiday-schedule blocks 2010–2027 as JSON, their verdicts,
  raw artifacts, and `DESIGN-holiday-tables.md`, the design the engine implements),
  `holiday-tables/` (implementation results), `ledger-reshape/`, `cme-globex/`,
  `sgx-pre2020/`, `ecbtc/`.
- Served identities (LAW-SERVICE-TIERS): the eight keys SharurPlatform maps —
  `globex_equity_index`, `globex_energy`, `globex_grains`, `globex_fx`,
  `globex_interest_rates`, `globex_livestock`, `globex_cryptocurrency`,
  `globex_nikkei_225_dollar` — and the venues its adapters admit: CME, CBOT, COMEX,
  NYMEX, CFE, Eurex, ICE Futures U.S., Coinbase Derivatives, SMFE. Everything else is
  dormant: kept, not worked on.

## 2. The release target

`Cargo.toml` says `1.0.0`, `CHANGELOG.md` has a `[1.0.0] - 2026-08-22` section, and no
`v1.0.0` tag exists (tags stop at `v0.2.2`): 1.0.0 was cut and never published, and a
large `[Unreleased]` section sits on top of it. **DECISION (maintainer):** release
everything as **1.0.0** on the release date (fold `[Unreleased]` into the pending
`[1.0.0]` section and re-date it — the repository's own housekeeping rule says a cut but
unpublished version absorbs its preparation), or bump to 1.1.0. The plan assumes 1.0.0.
Nothing in the unreleased work breaks the 1.0.0 API: the additions are keys, the three
holiday accessors, and documentation.

## 3. Stages

### Stage 1 — Land the holiday-tables PR

- #96 already targets `main` and is rebased on the merged reshape. Wait for CI; handle
  CodeRabbit (a +11k-line PR will draw many threads: fix the documentation ones, record
  schedule questions as gaps, resolve every thread); merge (squash, as every PR here is).
- Known review outcomes to read first in `holiday-tables/ASSEMBLE-result.md`: the gate
  window was widened to `[D-1, D+19]`; nine `globex_cryptocurrency` Closed rows were
  withdrawn because the five-day era has no business-date roll (memo D9 is amended in
  the PR's Review section); CME document ids are `CME-SVC-<first eventDate>`.
- Acceptance: golden file unchanged; the bench numbers in the PR body meet the design
  memo's §6.3 targets (the coverage gate makes a day with no row cost one binary search).
  If they do not, the engine is wrong, not the targets — fix before merging.
- This plan is `docs/plans/2026-09-12-path-to-release.md` in that PR.

### Stage 2 — Holiday waves back to the floor (one PR per wave)

The design memo (`holidays/DESIGN-holiday-tables.md`, §5.1–5.2) names, per block, the
few evidence repairs that must happen before its rows ship. Each wave is: repair those
items in the block's JSON (small, named), encode the served families from the JSON by
copying the wave-1 pattern (`src/calendar/schedules/holidays/<family>.rs`, the
`## Holidays` section in `docs/evidence/<family>.md`, the seven required tests per
family in `tests/futures_family_boundaries/holidays_<family>.rs`), run the gates, open
the PR. The verdict in `<block>.verify.json` wins over `<block>.json` on any row.

| Wave | Years | Repairs first (from memo §5.1) |
|---|---|---|
| 2 | 2022–2024 | the 2023-11-23 grains verbatim; the zone-provenance sentence; ship 2023 MLK/Presidents'/Good Friday and the 13 Nikkei 2024 rows as `Unsourced` |
| 3 | 2019–2021 | repair the 314 hard-truncated verbatim fields before writing evidence from them; withdraw the false "no standalone 2020 Good Friday workbook" claim; Juneteenth 2019–2021 is `Unsourced` |
| 4 | 2016–2018 | correct the three Grains verbatims that quote the deleted MGEX Apple Juice row |
| 5 | 2013–2015 | retrieve `2013-4th-of-july-done.pdf` (in the saved CDX lists; changes 2013-07-03 for livestock/dairy/lumber) and the 42 unretrieved earlier captures; then encode |
| 6 | 2010–2012 | fix the eight non-blocking defects the verifier listed (quoting convention, the `nikkei 2010-02-15` label, the 2012 Good Friday capture, the 2010-12-23 energy source); then encode — this closes the served-identity obligation to the January-2010 floor |
| 7 | venues | `CBOT` = grains ∩ interest rates; `COMEX`/`NYMEX` = the metals/energy halves; `CME` = the six routed families; a date on which families disagree ships no venue row and is a declared gap (memo D17) |

Non-CME venues (CFE, Eurex, ICE U.S., Coinbase Derivatives) shipped from 2026 in wave 1;
their history below 2026 was never retrieved and is best-effort (dormant-style) unless
the maintainer names it.

### Stage 3 — Served-key hygiene

- **#79**: source the 2012 Sunday Pre-Open move (16:15→16:00 CT) for `globex_equity_index`
  from CME's own channels (the 2012 trading-hours captures are already in the research
  store under `cme-globex/`; the operator's trading-hours service is T2), or leave the
  16:00–16:15 quarter-hour withheld and keep the issue open. This is an order-entry gap;
  it does not affect `is_open`.
- **#77**: fix the `session_profile` and `hours_for_market_hours_key` doc sentences so
  they say which state a seasonal key's static table holds. Doc-only PR.
- **#36**: under the charter the remaining 74 unaudited rows are dormant; narrow the issue
  to the served identities (all of which have been reviewed since 2026-08-22) and close
  it, or keep it as a dormant backlog. **DECISION (maintainer).**
- **Watch (LAW-WATCH)**: before the release, confirm every forward-dated row against the
  operator — today that is `globex_cryptocurrency`'s 2026-09-19 Saturday row — and clear
  or correct it (RELEASING.md step 5).

### Stage 4 — Consumer-side tasks (SharurPlatform, not this repository)

Hand these to the platform; they are the charter's consumer contract:

1. Pin the tagged release instead of a commit (`Cargo.toml` `exchange-hours = "=1.0.0"`).
2. Clamp the daily-chart backward walk to the contract's activation date
   (`crates/app/src/history.rs`, the walk in `window_reach.rs`); today it reaches 2007.
3. Add root tables for ICE Futures U.S. softs and Eurex fixed income — Rithmic admits
   those namespaces and the crate has the keys; today those instruments silently get
   NYSE FANG+ / FESX hours.
4. Map any trade-type variant it ever admits to its underlying family with the variant
   flag, never to a new key.
5. Decide how its `DayPolicy` seam layers over the crate's built-in holiday tables (the
   crate applies its table by default; the seam remains an override).

### Stage 5 — The release (RELEASING.md, followed exactly)

1. Branch `release/1.0.0` from `main`; set the version in `Cargo.toml` and `Cargo.lock`
   (already 1.0.0 if the decision in §2 holds); move `[Unreleased]` into `[1.0.0]` with
   the release UTC date; restore an empty `[Unreleased]`; update comparison links.
2. README: installation and migration text; the coverage and assurance prose are derived
   by fences — run the suite and fix what the failure messages print. Do not advance the
   schedule-review cutoff unless every non-synthetic row was reviewed through the new
   date (it was not; leave it).
3. Clear or correct every **Scheduled** marker whose effective day has passed (stage 3).
4. Gates, then `cargo publish --dry-run --locked` and `cargo package --list --locked`
   (check that `docs/evidence/` and the holiday tables are in the package and that
   nothing repository-only leaked).
5. `cargo bench --bench calendar_queries` — informational; call out a regression.
6. Open the release PR; merge; from a clean checkout of the merge commit rerun the gates
   and the dry run; `cargo owner --list exchange-hours` and `gh auth status`; tag
   `v1.0.0` annotated; `cargo publish --locked`; GitHub release with the changelog
   section. **DECISION (maintainer): the publish step needs the crates.io credential.**

### Stage 6 — After the release

- LAW-WATCH cadence: monthly review of `globex_cryptocurrency` and any family that
  changed within the year; quarterly for the other served identities; dormant on demand.
  A review is: open the monitoring entry points in `docs/schedules/sources.md`, compare,
  bump the ledger row's reviewed-on (UTC) if unchanged, otherwise a schedule-fix PR.
- Each new holiday year: retrieve the operator calendar when published (CME publishes
  one to two years ahead), encode per the wave pattern, one PR.
- Keep the research store as the artifact cache; commit only evidence files.

## 4. Things that bite

- A `|` inside a ledger cell breaks the row splitter; use " / ".
- Every count in README and the ledger prose is derived by a fence; the failure message
  prints the expected string — copy it, do not hand-count.
- Production files ≤ 500 lines of code, functions ≤ 100 lines (clippy fails the build).
- `///` doc comments need backticks around identifiers (`doc_markdown`).
- A revision row needs an operator-stated, unconditional, day-level date (T1 or T2);
  a capture dates the observation, never the state; rows are keyed to the local opening
  day (Sunday for a Monday trade date on a 17:00 CT grid).
- Never edit the working tree while a background agent owns it; use a worktree.
- The research store is local; a fresh clone has none of it, and the plan still works
  from the evidence files alone.
