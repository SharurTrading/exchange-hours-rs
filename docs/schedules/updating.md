<!-- SPDX-License-Identifier: MIT-0 -->

# Updating exchange schedules

This is the repeatable workflow for reviewing or changing a built-in schedule.
It complements the mandatory venue checklist in
[AGENTS.md](../../AGENTS.md#adding-or-revising-a-venue).

The supporting records are:

- [verification.md](verification.md): one row per public exchange, with its
  owner, evidence basis, review date, and known gaps;
- [sources.md](sources.md): stable operator/rulebook/notice entry points keyed
  by source-set ID;
- [audit-2026-08-21.md](audit-2026-08-21.md): the dated repository-wide
  assurance result, method, corrections, and exclusions for the current cutoff.

Exact historical notices and effective-date evidence remain beside the Rust
tables. The documentation is a monitoring index, not a substitute for adjacent
source citations.

## What the dates mean

`Reviewed on` is the day a human opened every required source set and compared
the modeled normal-week boundaries with the published material. It is not an
expiry date and does not promise that an exchange will remain unchanged after
that day.

The repository source-review cutoff is the oldest `Reviewed on` value among
non-synthetic `Exchange` rows. A partial review advances only the affected
rows. Advance the README cutoff only after every non-synthetic `Exchange` row
has been reviewed through the new date. `Unknown` is synthetic and is excluded
from that calculation.

The evidence basis does not improve merely because a row was reviewed:

- **Primary** — current boundaries are supported by an exchange, operator, or
  regulator source, with no known modeled-history gap since January 2010.
- **Partial** — current data has primary support, but a modeled historical era
  or exact cutover remains unsourced.
- **Secondary** — the best captured evidence is corroborating rather than
  primary.
- **Pragmatic** — an intentionally broad exchange/product-family default.
- **Known issue** — current primary material conflicts with, supersedes, or no
  longer identifies the modeled venue/profile; reconcile it before relying on
  the schedule.
- **Synthetic** — a library policy rather than a real venue schedule.

If a required source is inaccessible, record that fact and do not advance the
row's review date.

## 1. Open the monitoring sources

Start from the venue's source-set IDs in [verification.md](verification.md).
For every referenced set:

1. Open the current-hours page or living rulebook.
2. Open the official notice/evidence channel or entry point. When no stable
   feed exists, follow the exact adjacent artifacts recorded in the owner
   module and reacquire newer operator evidence from the documented entry point.
3. Review everything published since the venue's `Reviewed on` date, including
   announcements with a future effective date.
4. Confirm that the source still covers the same venue, board, product family,
   and phase semantics represented by the profile.

Search engines and industry summaries can locate evidence, but they do not
establish a schedule. Final literals and day-level cutovers require an exchange,
operator, or regulator source under LAW-PRIMARY-SOURCES.

Prefer a living rulebook/current-hours page plus a dated notice. A mutable page
shows today's state; the dated artifact proves when a historical row began. If
an official document is available only through an archive, cite both its
official origin and the archive delivery URL.

## 2. Compare every modeled field

Check the complete model rather than only the headline open and close:

- IANA time zone and trading weekdays;
- continuous (`regular`) phases;
- auctions, order-entry windows, and trade-at-last (`extended`) phases;
- lunch, maintenance, and transition gaps;
- sessions wrapping past local midnight;
- full-local-day sessions represented by equal open/close endpoints;
- fixed-UTC or foreign-market references expressed as seasonally changing
  venue-local times;
- daily-close and weekend-close flags;
- randomized auction boundaries and the documented deterministic choice:
  published nominal phase edge when only a per-security handoff varies, or a
  conservative venue envelope when that is the profile's stated scope;
- product, segment, or security-eligibility limitations;
- cancellation-only, reporting-only, negotiated, or administrative phases
  that the profile intentionally excludes.

Record the comparison in the owner module next to the literal rules. Keep the
source registry concise: stable monitoring entry points belong there, while
exact evidence belongs with the code it supports.

## 3. Classify the change

- **Current change with an exact effective date:** add a new static profile and
  a venue-local `Revision` row. For a wrapped session, key the revision to its
  local **opening day**: a Monday trade-date change that opens Sunday evening
  normally switches on Sunday.
- **Change at an exact intraday instant:** compare the full UTC instant in the
  venue selector and add it to the independent instant-cutover contract; do
  not round it to local midnight.
- **Current change without an exact effective date:** correct the current
  profile if supported, but document a historical gap; never invent a date.
- **Temporary regime:** add both the start and restoration rows only when each
  boundary has a primary day-level source.
- **Recurring/cross-zone rule:** use a date-aware selector such as B3/BMV and
  test every transition; do not freeze one seasonal snapshot. If the current
  profile format can express only a static local-time approximation, keep the
  ledger basis pragmatic and state which season differs.
- **One-off holiday, halt, weather closure, or half-day:** document it if useful,
  but do not force it into the normal-week profile.
- **Per-security randomized auction uncross:** this is microstructure, not an
  exchange closure. Use the operator's published nominal phase boundary when
  the random delay only shifts an adjacent auction/continuous handoff and the
  exchange remains available throughout. If a venue profile instead promises
  a conservative maximum auction envelope, retain and document that choice.
  Do not claim ticker-level uncross timing.
- **Product-specific variation:** either narrow the documented profile scope or
  add a distinct product/venue model; do not silently widen an envelope.

Equal `SessionRule` endpoints represent one complete local-day session. Use
that shape when a sourced session opens and closes at the same wall-clock time
on consecutive days; omit the rule when no session exists.

Future-effective notices can be encoded immediately when fully sourced. The
row's review date is still the day the evidence was checked, not the notice's
effective date. When an announced launch remains conditional on regulatory or
infrastructure readiness, label the encoded revision as an announced plan and
keep it below until the operator confirms that every condition has been met.

### Pending effective-date confirmations

- **Nasdaq — 2026-12-06:** [ETA2026-46](https://www.nasdaqtrader.com/TraderNews.aspx?id=ETA2026-46)
  announces the Night Session date, while [Nasdaq Equity 1](https://listingcenter.nasdaq.com/assets/RuleBook/Nasdaq/rules/Nasdaq%20Equity%201.html)
  conditions commencement on Equity Data Plan readiness and a later Nasdaq
  readiness filing. Recheck that filing and move the revision if it changes the
  launch.
- **Cboe EDGX — 2026-12-06:** the [opening-process specification](https://www.cboe.com/document/tech-spec/document/technical-specifications/cboe-titanium-u.s.-equities-opening-process)
  announces the first overnight opening for the 2026-12-07 business date, but
  [SR-CboeEDGX-2026-019 Amendment No. 1](https://cdn.cboe.com/resources/regulation/rule_filings/pending/2026/SR-CboeEDGX-2026-019-Amendment-No-1.pdf)
  conditions commencement on regulatory completion, Equity Data Plan readiness,
  and a later EDGX readiness filing. Move the revision if those conditions
  change the launch.
- **FINRA TRFs — 2026-12-06:** recheck the Securities Information Processor
  (SIP) Amendment launch against [SR-FINRA-2026-015](https://www.finra.org/sites/default/files/2026-07/SR-FINRA-2026-015.pdf)
  before this date. The modeled Sunday-through-Friday reporting regime is the
  announced plan and is conditional on that rollout; if the SIP implementation
  moves, move the Carteret, Chicago, and NYSE TRF revisions with it.

## 4. Implement in the owner module

For an existing venue, its file under `src/calendar/schedules/` owns current
rules, history, caveats, and citations. Ordinary revisions use the shared
timeline helper; exceptional recurrence stays local to the venue.

Then follow every independent fence in AGENTS.md:

- exhaustive current routing and historical routing;
- bulk membership when relevant;
- handwritten exchange identity and cutover expectations;
- baseline and both-sides-of-cutover integration tests;
- README coverage, this ledger, and the changelog.

Update the source set only when the authority changes or a more stable official
entry point is found. Update code citations and the registry in the same change.

## 5. Update freshness records

After the comparison is complete:

1. Update the venue row's source sets, evidence basis, review date, history
   status, and scope note.
2. Add any newly found gap rather than hiding it behind a stronger status.
3. Advance the repository cutoff in [verification.md](verification.md) and the
   README only if every non-synthetic `Exchange` row has reached that date.
4. For a repository-wide review, create a new dated audit report, update the
   README link, and update the documentation contract's included audit path.
   The contract derives Primary, Partial, Pragmatic, and Known-issue counts from
   the ledger; do not copy forward old assurance numbers without that check.
5. Record schedule changes under `[Unreleased]` / **Fixed** and new venues under
   **Added** in `CHANGELOG.md`.

Emergency reviews normally advance one venue only; the repository-wide cutoff
therefore remains unchanged until the lagging rows catch up.

## 6. Verify

Run the full repository gate:

```bash
cargo fmt --all --check
cargo clippy --all-targets -- -D warnings
cargo nextest run --all-targets
cargo test --doc
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps
cargo deny check
cargo +1.95 check --all-targets
```

Also open every link added or changed in `sources.md`. A successful HTTP
response is not verification: a human must confirm that the page still states
the modeled boundaries and scope.
