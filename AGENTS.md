<!-- SPDX-License-Identifier: MIT-0 -->

# AGENTS.md — the charter, for humans and coding agents

This file states what the crate is for, the named laws that code comments and
ledger rows cite, and the rules any change to this repository must follow. It
was rewritten on 2026-09-12 (UTC) after an architectural review
(`docs/plans/2026-09-12-architectural-review.md`); law names were kept wherever
a law's intent survived so that existing citations stay valid.

## Purpose

`exchange-hours` is the live session calendar for the instruments its consumer
can route. Its consumer is SharurPlatform, which will support more markets over
time. For each supported identity the crate answers five questions, correctly
for today and for every instant back to the January-2010 floor: is the market
open now; where does this trading day begin and end; which trade date does an
instant belong to; when does the next session open; is this gap a closure. It
also answers which days are holidays or early closes for that identity
wherever the per-family holiday tables LAW-HOLIDAY-SCOPE calls for have
shipped; where they have not, holidays reach a calendar only through the
caller's `DayPolicy` overlay.
It is not an archive of exchange history for its own sake and it is not a public
reference work; every rule below is judged against those five questions and
the cost of keeping them true.

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
- **LAW-SERVICE-TIERS** — every `Exchange` and `MarketHoursKey` is either
  **served** or **dormant**, and the ledger says which. An identity is served
  when a consumer instrument can reach it: a root in the consumer's family map
  points at the key, or the consumer's adapters admit the venue. Everything
  else is dormant: kept, correct as of its last review, never deleted, and
  re-reviewed only when a consumer reaches it or the maintainer names it as a
  planned market. The obligations differ by tier and are stated in the table
  below. A new identity is admitted when a consumer can reach it or the
  maintainer names the market it is planned for; a shape no consumer can route
  is not modelled, however well it could be sourced. Trade-type variants of a
  family (TAS, TAM, BTIC, TACO, TMAC) are the worked example: the consumer maps
  a variant to its underlying family with a disclosed variant flag, and a
  variant earns its own key only when a consumer maps one. Research already
  done for an unbuilt key is kept in the research store, not discarded.

  | Obligation | Served | Dormant |
  |---|---|---|
  | Current schedule sourced at T1 or T2 | required | required at last review |
  | Dated history to the January-2010 floor | required | best-effort, labelled |
  | Holiday and early-close table | required, floor to published future | required after the served tier; refreshed on demand |
  | Review cadence (LAW-WATCH) | monthly if high-churn, 24/7 or holiday-bearing, else quarterly | on demand |
  | Follow-ups tracked as issues | required | recorded in the evidence file |

- **LAW-PRIMARY-SOURCES** — every session time, every dated change and every
  holiday in this crate is backed by evidence at a recorded **tier**:
  - **T1**, the operator's own statement — rulebook, notice, circular,
    specification page, product change log, holiday calendar — read from the
    operator or from a verbatim public mirror of the operator's document.
  - **T2**, the operator's own machine channel — a session-schedule feed, a
    trading-hours service, a reference-data API — read as bytes and saved.
  - **T3**, a restatement by a member firm, vendor, data provider or index
    publisher.
  - **T4**, press and everything else.

  A current schedule needs T1 or T2. A dated change needs an unconditional day
  stated by the operator: at T1 in session language, or at T2 when the feed
  carries the operator's own trade dates on both sides of the boundary. T3 may
  date a change only when it mirrors an operator document verbatim, and T3 may
  corroborate; T4 never keys a row and is recorded only as residual risk. A
  venue feed's labelled close or halt event is session language unless the
  operator's prose contradicts it; an open-and-close pair that no operator prose
  mentions is not. A conditional launch clause on a day now past is discharged
  when a later operator artifact witnesses the new state, and the discharge is
  recorded beside the row. An operator's own dated change log is T1 when the
  entry is the operator's, the day is stated inside that entry or a header that
  scopes it, it is in session language, and the log is calibrated against a
  change the crate already holds from another T1 document; a bare day-and-month
  means the occurrence nearest the entry's issue date. When two operator
  statements conflict, establish their lineage first (a page's version history,
  a document's own date, whether one merely reprints the other); serve what
  both state and withhold only what they dispute, and record both. A row's tier
  and document id live beside the row (LAW-EVIDENCE-FILES). A venue that merely
  coincides with another still gets its own named profile.
- **LAW-PUBLIC-SOURCES** — public sources are preferred and are the only ones
  a reader can re-verify, so cite them wherever they exist. A source behind
  authentication is admissible only as **T2**, only when it is the operator's
  own channel, and only when the retrieved artifact is saved in the research
  store with its retrieval date and quoted in the evidence file. A document on
  a member portal is admissible as T1 through a verbatim public mirror; the
  portal's index entry alone proves that a document exists and when it was
  published, not what it says.
- **LAW-NO-FABRICATED-DATES** — a historical cutover exists only when the
  evidence states an **unconditional, day-level** effective date at the tier
  LAW-PRIMARY-SOURCES requires. A real change without one is documented as a
  known gap, never given an invented date. A future date contingent on a
  readiness filing, regulatory condition, or infrastructure rollout stays in
  the watch list and out of runtime selectors. An unconditional, fully sourced
  future date **may** be encoded ahead of its effective day, so instant-driven
  queries roll over with no release in between; the row's review date is the
  UTC date the evidence was checked (LAW-UTC-DATES), and a slipped or cancelled
  change is corrected as a schedule fix. Day-level revision rows are keyed to
  the local **opening day** of the first session they govern, and a day-level
  revision boundary never splits a running session; a sourced change whose
  boundary falls at a stated intraday instant is an exact-instant cutover,
  never a day-level row rounded to local midnight. Amendment history is
  recorded back to **January 2010**; earlier changes are out of scope by
  design.
- **LAW-UTC-DATES** — every date the repository records about its own work is
  the UTC calendar date on which that work happened: a ledger `Reviewed on`, a
  knowledge-bound row's date and citation label, an audit or ledger amendment
  note, a CHANGELOG release date. Never use the author's local date or the
  commit's timezone offset. The crate is UTC in, UTC out, and its records keep
  the same clock, so a date never depends on where a maintainer works or
  whether they move; a recorded date later than the current UTC date is
  future-dated and wrong. This governs the repository's own records only — an
  exchange's effective dates remain venue-local civil dates keyed to the
  opening day (LAW-NO-FABRICATED-DATES).
- **LAW-SESSION-NOT-EXPIRY** — an instrument's **termination of trading,
  expiration, settlement, marker or fixing instant is never a session
  boundary**, and this crate does not model it at any resolution. The crate
  answers when a *market* accepts and matches orders. When an individual
  *contract* stops trading is instrument-lifecycle data, owned by the caller's
  catalog and its provider adapters. A daily close enters a profile only when
  the operator states it in **session language** — "trading halts", "trading
  ceases daily at", "a daily maintenance period from X to Y", a `TRADING
  HOURS` row, or a venue feed's own close or halt event. A **calculation
  window**, an **order-entry cutoff**, or a **termination-of-trading row** is
  none of those, even when it is printed adjacent to the hours. The tell is a
  table in which several products share one hours cell while each carries its
  own end time: the shared cell is the session and the per-product times are
  expiries (CME's `SER-9624` event contracts are the worked example). A close
  that coincides with a settlement or fixing instant is still a close when a
  T1 statement or a T2 feed observes it as one; the coincidence neither proves
  nor disqualifies it. A source that states only an expiry leaves the session
  unsourced.
- **LAW-HOLIDAY-SCOPE** — a change confined to a single trade date or a
  bounded run of dates — an early final close, a late first open, or a full
  calendar-day closure — is a **holiday**, never a schedule: it never bends a
  normal-week template, adds a revision row, or deletes a valid phase, and a
  genuine recurring-grid change is never downgraded to "just a holiday" to
  avoid the evidence work. Holidays are **in scope** for this crate. They live
  in per-family date tables under `schedules/` (data, not templates),
  sourced from the operator's own published holiday calendar at T1 or its own
  machine channel at T2, at the tier LAW-PRIMARY-SOURCES requires and with that
  tier carried in the row rather than only in a comment. The **target** is
  the January-2010 floor, or the identity's first trading day if later, to
  whatever the operator had published unconditionally as of the table's
  inspection date, for every identity: served identities first, dormant ones
  after them. It is an obligation the tables are built towards, not a
  statement of what ships. Where the operator's own documents, archives
  included, do not reach the floor, the table starts where they do and the
  evidence file names the gap; an identity whose operator observes no holidays
  says so in its evidence file instead of shipping a table. Once a family's table ships, the
  built-in calendars apply it by default. **What ships — which identities have
  a table, and over which trade-date window — is the `Holidays` column of the
  verification ledger**, derived by a fence from each identity's own
  `holiday_coverage()`, which returns the audited window;
  an identity with no table carries no holiday data and the caller's
  `DayPolicy` overlay is its only holiday layer. Inside a table's window a date
  with no row is audited normal; outside it the crate has no holiday answer at
  all. A holiday table entry records the date, the kind (closed, early
  close at an instant, late open at an instant), and its document id. A special
  day that changes internal phase topology is not representable by scalar
  boundaries and is recorded as a gap. `DayPolicy` remains the caller's overlay
  for what the crate does not carry, layered above the built-in table once
  there is one. Holiday
  lookups are bounded, allocation-free and cheap enough to sit on the
  consumer's hot path.
- **LAW-EVIDENCE-FILES** — narrative evidence lives in
  `docs/evidence/<owner>.md`, one file per venue or key, and never in a source
  module. A schedule module carries its rule data and, beside each revision
  row, one line: the effective day, the tier, the document id and a short
  label, plus a link to the evidence file. The evidence file holds the
  quotations, URLs, retrieval dates, conflicts, interpretive steps and residual
  risks, in enough detail that a reader can re-verify every row from the bytes
  in the research store. A fence checks that every row's day appears in its
  evidence file. The verification ledger row is a fixed shape — key, owner,
  tier, service tier, horizon, reviewed-on, gap kind and at most three
  sentences of basis — and never a narrative. A module touched for any reason
  moves its remaining narrative out in the same change; a new module never
  carries one.
- **LAW-BOUNDED-WORK** — adding or revising an identity is one pull request
  sized to a working day: the current schedule at T1 or T2, the dated changes
  the service tier requires, the holiday table, the tests, the evidence file,
  the ledger row and the registration list. It is not a research programme: no
  multi-agent adversarial cycles, no channel tunnelled around after it has
  refused twice, no archaeology beyond what the tier obliges. What a bounded
  task cannot source is recorded as a gap with what would close it, and for a
  served identity that gap becomes an issue (LAW-FOLLOW-UPS-ARE-ISSUES). The
  research store beside the repository holds retrieved artifacts and working
  notes; only the evidence file is committed.
- **LAW-WATCH** — a served identity is reviewed on a cadence recorded in its
  ledger row: monthly for a family that has changed within the last year,
  trades on a 24/7 grid, or ships a built-in holiday table (the operator
  republishes its holiday calendar yearly and issues errata), quarterly
  otherwise. A forward-dated row carries a
  confirm-by date and is confirmed against the operator before its effective
  day. Every schedule change ships in a tagged release with a CHANGELOG entry,
  so the consumer pins a version, never a commit. The monitoring entry points in
  `docs/schedules/sources.md` are the watch list.
- **LAW-FOLLOW-UPS-ARE-ISSUES** — a follow-up named anywhere — in a pull
  request, a commit message, a review reply, a ledger row, a plan, or a
  research note — is either done in that same change or opened as a GitHub
  issue before the change merges, and the issue number is cited where the
  follow-up is named. For a dormant identity the same obligation is met by
  recording the gap and its closing condition in the evidence file. A
  follow-up that is never recorded is a claim that it was never needed.

## The consumer contract

SharurPlatform owns the map from instrument roots to `MarketHoursKey` and from
venue namespaces to `Exchange`; this crate never maps symbols, roots, product
codes or MICs. The consumer must cover every venue namespace its adapters admit
with a family map, must mark a trade-type variant with its variant flag when it
maps one to its underlying family, must clamp historical walks to the
instrument's own listing window, and must depend on a tagged release. The
crate's ledger tells the consumer which identities are served, which are
dormant, and each identity's horizon and holiday coverage.

## Modeling conventions

- **UTC in, UTC out.** Every *timestamp* crossing the public boundary is a
  `DateTime<Utc>`: a local time is never a parameter or a return value, and the
  venue's zone stays an internal detail. Session rules are seconds-since-local-
  midnight in the venue's own IANA zone.
- **Closes are end-exclusive.** An instant equal to a close is closed.
- **`open_ssm >= close_ssm` wraps** into the next local day. Equal endpoints
  encode one complete local-day span; omit a rule to express no session.
- **DST bias is asymmetric on purpose**: opens resolve earliest, closes latest.
  Never "simplify" this to a single bias.
- **Boundaries, trade date and `order_entry` are required; `regular` is
  declared.** The session boundaries (the union of every executable phase) and
  the trade date are what every consumer purpose depends on. `order_entry` is
  required because it keeps a queue out of `is_open`. `regular` is an
  operator's explicitly published regular-trading-hours session where one
  exists and is otherwise empty with a one-sentence declaration beside the
  table; an empty `regular` needs no further proof. Auction calls, pre-open and
  other order-entry-only windows are `order_entry`; post-close and
  trade-at-last sessions are `extended`. Lunch breaks are gaps, not rules.
- **Cash-equity venue envelope.** An `Exchange` cash-equity profile is the
  availability union of the venue's automated order-capable systems within the
  row's documented scope. Include executable and accepted order-entry phases.
  Exclude pure reporting, cancellation-only, enquiry and administrative states,
  and systems that are separately modelled identities.
- **Product-neutral family selection.** `MarketHoursKey` names a sourced
  schedule family. A venue-keyed default is not permission to use that clock
  for every product listed there; the consumer's map decides.
- **Trade dates and state.** A containing session's trade date is normally the
  venue-local date of its final close. Same-trade-date gaps are `Halt`, except
  that a sourced continuously traded-week profile can retain an
  operator-designated gap of no more than four hours as `Maintenance`.
  Inter-trade-date gaps no longer than four elapsed hours within one ISO week
  are also `Maintenance`; longer gaps are `Closed`. The four-hour bound and
  the ISO-week test are **crate policy**, chosen so a consumer can compress
  maintenance out of a chart and never a halt; they are tested as policy, not
  sourced per identity. `is_maintenance` must remain exactly the maintenance
  case of `session_state`.
- **Family holiday exceptions.** A closed date normally removes its complete
  trading day, including the prior-evening wrap. Preserve a sourced family
  exception when the operator assigns continuous weekend trading to the
  following open business date; CME cryptocurrency rolls a closed Monday's
  weekend block into Tuesday instead of deleting it.
- **Exchange-level boundaries, not per-security auction outcomes.** When an
  operator publishes a nominal phase boundary but randomizes the actual
  uncross per security or group, prefer the published nominal boundary when
  the randomized seconds only move an adjacent phase handoff; use a
  conservative envelope when that is the profile's stated scope. Never imply
  exact ticker-level uncross timing.
- **Below the January-2010 floor, the earliest sourced profile stands.**
  `select_revision` returns a venue's baseline for any date before its first
  revision, so an instant before the floor resolves to the oldest profile on
  record — for a launch-dated identity its pre-launch closure, for others the
  earliest grid the crate holds. Do not add a lower bound to the timelines.
  Nothing below the floor is reviewed.
- **Executable windows are the priority.** A gap in a phase where a trade can
  print is materially more serious than a gap in an `order_entry` window.
  Close executable-hours gaps first, and when recording a gap say which kind
  it is.
- **Carry the earliest sourced state back to the floor.** When a phase is
  sourced at some instant and no admissible source names a cutover between the
  floor and that instant, extend it backwards rather than modelling the
  interval as sessionless; this asserts no revision row. Where a lower-tier
  source attests a change inside the carried interval, carry the state anyway
  and record the residual risk in the evidence file. Each identity's ledger row
  states its **horizon**: the date below which its rows are carried rather
  than sourced.
- **Prefer the sourced intersection to omission.** When a phase's endpoints
  are sourced at two values and only the changeover day is undated, serve the
  window that holds under every sourced state and withhold only the disputed
  remainder. The same rule governs a source conflict that lineage cannot
  settle. A knowledge boundary may only **widen**: hold a bound at its
  narrowest sourced value across the whole undated span rather than granting it
  early and withdrawing it at a capture. A row that lengthens or creates a
  wrapping overnight close is keyed to the following Monday, so a mid-week key
  never reports the previous evening's leg running past the close in force
  when it opened; a row that changes only daytime bounds keeps its artifact's
  date.
- **A knowledge boundary is the first source that lists the modelled
  product**, not merely the earliest source that survives. Check the contract
  set, not just the grid, before keying a row to an edition.
- **"Unsourced" means "not worked up", never "no source exists".** Say a row
  is unmodelled before an era and say what would close it; do not write that
  nothing survives unless the predecessor channels have actually been searched.
- **Absence is `None`.** Boundary queries return `Option`; never fabricate a
  degenerate session. No public code path may panic or hang.

## Structural rules

- `Exchange` and `MarketHoursKey` are `#[non_exhaustive]`: identity additions
  are minor releases, not breaking ones. Never remove that attribute, and never
  remove or rename a variant outside a major release. A dormant identity is
  kept for exactly this reason: its wire name may already be persisted by the
  consumer. A breaking identity or wire-format change requires explicit
  authorization, an appropriate SemVer release, a changelog migration note and
  exhaustive cleanup; never perform one as drive-by cleanup.
- `Exchange`, `Exchange::ALL`, and `Exchange::as_str` are generated from
  **one table** (the `exchanges!` invocation in `exchange/mod.rs`): adding a
  venue is one new row, and neither `ALL` nor the name table can omit it. The
  compiler then forces the remaining in-crate exhaustive match —
  `hours_for_exchange` in `presets/historical.rs` (no catch-all arm). The same
  edit must also reach the region list in `bulk.rs` when a bulk builder covers
  the venue, and `ALL_EXCHANGES` + `EXCHANGE_VARIANT_COUNT` in
  `tests/contract/session_invariants/identity_expectations.rs`.
- `MarketHoursKey`, `MarketHoursKey::ALL`, `MarketHoursKey::as_str`, and its
  serde implementations likewise come from the single `market_hours_keys!`
  table in `futures_profile.rs` / `futures_profile/key_serde.rs`. A new key
  must also reach the handwritten key expectations, the verification ledger and
  the public profile tests; those fences are compared against `ALL`, never
  generated from it.
- One canonical `snake_case` name per identity, shared by serde, `as_str`,
  `Display`, and `FromStr`, and it is stable: a rename breaks persisted data.
  Serde uses the canonical strings in every format; never restore
  derive-generated enum ordinals. `FromStr` rejects unknown names with
  `ParseExchangeError` or `ParseMarketHoursKeyError`.
- Production source files stay cohesive and reviewable, ordinarily at or below
  500 lines of **code**. This is a source-reviewability guard, not a size
  limit: split independent responsibilities, but do not fragment a coherent
  module to satisfy a smaller count, and never split a module to make room for
  narrative — narrative belongs in the evidence file (LAW-EVIDENCE-FILES).
  Test files are exempt.
- Schedule profile tables and holiday tables are `static` so calendars can
  borrow them allocation-free.
- `ExchangeCalendar` represents either `CalendarSource::Exchange` or
  `CalendarSource::MarketHoursKey`. Both sources must support the same
  date-aware query surface. Keep the calendar `Copy + Send + Sync + 'static`
  and built-in hot-path queries allocation-free over bounded scans; holiday
  lookup is a binary search over a sorted static table.
- Identity-dependent topology belongs on the date-aware identity calendar. A
  detached `MarketHours` snapshot must remain exactly the caller-supplied rule
  set; never guess a family from coincident rules. Always-open calendars have
  no final close and return no trade date.
- **Instant-only selection.** Every public entry point that resolves an
  identity to a schedule for a moment in time requires the caller's instant,
  and there is no clock-less "current" routing between eras. `session_profile`
  remains the one static current-*table* accessor; it selects no era and, for
  a seasonal or cross-zone identity, is documented as the state it holds. This
  is what makes backtest and live one code path; do not reintroduce a second
  selector. The public fixed-snapshot query adapters remain compatibility
  contracts.

## Adding or revising an identity

Treat the following as one change set, sized to LAW-BOUNDED-WORK. The repeated
expectations in production, tests and documentation are deliberate coverage
fences; compare them against production data, never generate them from it.

1. **Admission.** Say why the identity is served or which planned market it
   belongs to (LAW-SERVICE-TIERS). Record the service tier in the ledger row.
2. **Identity.** Add the canonical enum row in the `exchanges!` or
   `market_hours_keys!` table, with a stable `snake_case` wire name and public
   variant documentation. Never rename an existing row as part of an hours
   correction.
3. **Schedule data.** Add or revise the owner's static profile with one
   citation line per row and a link to `docs/evidence/<owner>.md`, which holds
   the evidence at its tier. Follow `docs/schedules/updating.md`. Give an
   identity its own named profile even when its hours coincide with another.
   Add dated revision rows only for unconditional, day-level changes at the
   required tier; record an unsourced gap in the evidence file (and as an
   issue for a served identity) instead of inventing a date.
4. **Holiday table.** Add the family's holiday and early-close table from the
   operator's published calendar, to the depth the service tier requires.
5. **Routing.** Add the arm to the no-catch-all `hours_for_exchange` or
   `hours_for_market_hours_key` match, dispatching to the `profile_at`
   selector; the date-aware `calendar_for_exchange` /
   `calendar_for_market_hours_key` surface needs no arm but every history,
   cutover and boundary test exercises the identity through it. A cross-zone
   or seasonal selector also needs date-aware transition coverage.
6. **Regional membership.** Add a venue to the appropriate `bulk.rs` region
   list when a built-in bulk builder should include it.
7. **Independent contracts.** Update `ALL_EXCHANGES` / `EXCHANGE_VARIANT_COUNT`
   or the handwritten key lists, and add every observable dated change to
   `HISTORICAL_CUTOVERS` (or `HISTORICAL_INSTANT_CUTOVERS` for a stated
   intraday boundary), recording the local opening day for a wrap.
8. **Public-surface tests.** Add a per-identity baseline for the published
   open and the instant before it, the phase classification, every lunch or
   maintenance gap, the end-exclusive close, the weekend boundary, at least one
   holiday and one early close where the table has them, and the serde form.
   Test both sides of every recorded cutover at venue-local midnight and
   mutation-check each cutover fence. Put growing suites behind a thin
   top-level harness and per-identity submodules.
9. **User-facing records.** Update the ledger row in
   `docs/schedules/verification.md` (fixed shape, service tier, horizon,
   cadence), the README coverage counts (derived by the fences), the source
   set in `docs/schedules/sources.md` when a new monitoring entry point is
   needed, and `CHANGELOG.md` under `[Unreleased]`.
10. **Verification.** Run the complete quality and MSRV commands below.

## Lints and toolchain

- **Clippy pedantic is the floor**, plus restriction lints that encode the laws:
  `unwrap_used` / `expect_used` / `panic` / `todo` / `unreachable` (LAW-PANIC),
  `print_stdout` / `print_stderr` / `dbg_macro`, and the `disallowed_*` family
  (LAW-DETERMINISM). Configured in `Cargo.toml` `[lints]` and `clippy.toml`.
  CI runs `-D warnings`, so a pedantic warning fails the build.
- **`missing_docs` is denied** — every public item, including every enum
  variant, carries a doc comment.
- **Suppressions are `#[expect(..., reason = "...")]`, never bare `#[allow]`**,
  and only where the lint is wrong for a stated reason. Tests are exempt from
  the panic-family lints via the `allow-*-in-tests` switches in `clippy.toml`.
- **Toolchain** is pinned in `rust-toolchain.toml` to the version the consuming
  platform builds with; the *minimum supported* version is `rust-version` in
  `Cargo.toml` (1.95) and is exercised by the `msrv` CI job. Raising either is
  a deliberate change: update `rust-version`, the CI matrix, and the README
  badge together.
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

## Housekeeping

- Record user-visible changes under `[Unreleased]` in `CHANGELOG.md`; session
  data corrections go under **Fixed**, new identities under **Added**, holiday
  table additions under **Added** with the years covered. A PR that bumps the
  version in `Cargo.toml` is a release cut: it retitles `[Unreleased]` to the
  dated version section, and the tag follows the merge. Follow
  [`RELEASING.md`](RELEASING.md) for the cut, tag, publish, and verification
  sequence. Every schedule change reaches the consumer only through a tagged
  release (LAW-WATCH).
- The README's version and MSRV badges read from crates.io; do not
  re-hardcode them. Every count the README states is derived by a fence in
  `tests/schedule_documentation/`; when a claim in the README stops being
  derivable, delete the claim rather than hand-maintaining it.
