<!-- SPDX-License-Identifier: MIT-0 -->

# Updating exchange schedules

This is the repeatable workflow for reviewing or changing a built-in schedule.
It complements the mandatory venue checklist in
[AGENTS.md](../../AGENTS.md#adding-or-revising-a-venue).

The supporting records are:

- [verification.md](verification.md): one row per public exchange and
  `MarketHoursKey`, with its owner, evidence basis, review date, and known
  gaps;
- [sources.md](sources.md): stable operator/rulebook/notice entry points keyed
  by source-set ID;
- [date-exceptions.md](date-exceptions.md): the boundary-overlay contract and
  evidence requirements for future complete holiday calendars;
- [audit-2026-08-22.md](audit-2026-08-22.md): the dated repository-wide
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
from that calculation. Product-family key rows carry their own review dates and
are counted separately; adding a key does not retroactively change the
exchange-wide cutoff.

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

**Channel access notes, recorded 2026-08-31.** Several operators cannot be read
by automated retrieval, which changes how a review must be run:

- **CME Group** returns an explicit anti-scraping block citing its Data Terms of
  Use. Read cmegroup.com by hand, or take archived copies. Its Globex and Market
  Data notice digests are weekly Monday pages under
  `tools-information/lookups/advisories/{electronic-trading,market-data}/YYYYMMDD.html`,
  and its rule-filing PDFs are not archived.
- **SGX** serves its circular archive as a JavaScript app with no reachable
  public API — the regco.sgx.com CMS now answers the `/circulars` route with
  `null` — and its Titan DTDC newsletters, where hours changes are announced,
  are password-locked member documents. Three channels are productive instead.
  Its *Derivatives Trading Calendar* PDFs under
  `api2.sgx.com/sites/default/files/YYYY-MM/` are static, readable, and state
  per-contract session bounds. The public [Titan DT/DC
  portal](https://www.sgx.com/titan-dt-dc-portal) page lists every member
  document's title, version and **release date**, which dates a change's
  announcement even when its contents are locked. And `DT/AM` circulars, which
  do carry `with effect from` dates, exist as verbatim copies on SGX trading
  members' public sites when no SGX page links them; treat the mirror as a
  delivery channel, cite the circular number, date and title with it, and
  re-verify from SGX if a reachable copy appears. A fourth channel covers the
  years the live site dropped: SGX's **retired** portal is archived, and
  `sgx.com/wps/wcm/connect/` captures carry both the pre-2020 derivatives
  trading-hours pages and the circulars of that era. Look there before
  recording a pre-2020 era as unsourceable — one such claim shipped in this
  repository and was wrong.
- **ICE**: `ice.com/publicdocs/circulars/{YY}{NNN}.pdf` is enumerable but is ICE
  Futures *Europe*. ICE Futures U.S. notices have descriptive filenames under
  `publicdocs/futures_us/exchange_notices/` and are only thinly archived.
- **Nasdaq**: `nasdaqtrader.com` serves a bot-protection interstitial;
  `nasdaq.com/docs/*SystemSettings` serves the same content as static PDFs.
- The **Federal Register full-text API** covers every SEC self-regulatory
  filing and is the most productive channel for US equity and options venues.

Search engines and industry summaries can locate evidence, but they do not
establish a schedule. Final literals and day-level cutovers require an exchange,
operator, or regulator source under LAW-PRIMARY-SOURCES.

Prefer a living rulebook/current-hours page plus a dated notice. A mutable page
shows today's state; the dated artifact proves when a historical row began. If
an official document is available only through an archive, cite both its
official origin and the archive delivery URL.

## 2. Compare every modeled field

Work the executable phases first. A discrepancy in a `regular` or `extended`
window changes whether the crate reports the market as tradeable; a discrepancy
in an `order_entry` window only changes whether orders could be queued before an
open that is already modelled correctly. Both are worth fixing, but the first
outranks the second no matter how many rows the second touches.

Check the complete model rather than only the headline open and close:

- IANA time zone and trading weekdays;
- primary/core continuous or published RTH (`regular`) phases, and any
  electronic/overnight continuous trading that the owner scope classifies as
  `extended`;
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
- whether a venue-keyed default and a product-family key cover the same exact
  scope (never assume that they do);
- trade-date semantics and whether a session spans more than one local
  midnight; use identity-aware calendar coalescing when adjacent static pieces
  are only storage for one sourced multi-day block. If the public model still
  cannot preserve an operator's exact continuous bound, state that remaining
  limitation instead of implying it can;
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
- **One-off holiday, halt, weather closure, or half-day:** never force it into
  the normal-week profile. A whole trade-date closure, later first open, or
  earlier final close can use a sourced `DayPolicy`/`StaticDayPolicy` record.
  If regular and extended phases change differently, the day pauses and
  reopens, or its trade-date assignment changes, the scalar overlay is not
  exact; follow [date-exceptions.md](date-exceptions.md) and wait for a complete
  replacement-session provider instead of deleting valid trading.
- **Per-security randomized auction uncross:** this is microstructure, not an
  exchange closure. Use the operator's published nominal phase boundary when
  the random delay only shifts an adjacent auction/continuous handoff and the
  exchange remains available throughout. If a venue profile instead promises
  a conservative maximum auction envelope, retain and document that choice.
  Do not claim ticker-level uncross timing.
- **Product-specific variation:** either narrow the documented profile scope or
  add a distinct product/venue model; do not silently widen an envelope.
- **Cash-equity venue union:** compare every automated order-capable system
  owned by the modeled exchange, including queues and block/crossing phases.
  The profile may be open when only a subset of securities is eligible. Exclude
  pure reporting, cancellation-only, enquiry, and administrative states and a
  system represented by another `Exchange` identity.
- **Product-family identity:** add or revise a `MarketHoursKey`; do not map
  symbols, roots, product codes, or MICs inside this crate. A product that
  joins an already-live family normally has a caller-owned listing date, not a
  revision of the shared family clock. If its hours differ, it needs a separate
  key.
- **Holiday or special-day data:** keep it out of the built-in normal-week
  tables. A caller supplies sourced boundary-level records through `DayPolicy`
  or `StaticDayPolicy`. A closed date normally removes its complete trading
  day, including a prior-evening wrap. Preserve a different
  following-business-day assignment only when the operator sources it; CME
  cryptocurrency weekend trading rolls into Tuesday when policy closes Monday.
  Do not describe this scalar overlay as a complete holiday calendar: phase
  replacement, coverage status, and evidence finality are separate requirements
  documented in
  [date-exceptions.md](date-exceptions.md).

Equal `SessionRule` endpoints represent one complete local-day session. Use
that shape when a sourced session opens and closes at the same wall-clock time
on consecutive days; omit the rule when no session exists.

Future-effective notices can be encoded immediately only when their effective
day is unconditional and fully sourced. The row's review date is still the day
the evidence was checked, not the notice's effective date. When an announced
launch remains conditional on regulatory or infrastructure readiness, keep it
out of runtime selectors and track it below until every condition is confirmed.

The pre-coding loop, once a day is confirmed unconditional:

- encode the revision row ahead of its effective day; add its boundary to the
  handwritten `HISTORICAL_CUTOVERS` (or `HISTORICAL_INSTANT_CUTOVERS`) list
  with both-sides tests; mark the ledger row `Scheduled`;
- every query is instant-driven, so the row is inert until its effective
  instant — callers roll over with no release in between;
- in the first release pass after the effective date, confirm the change took
  effect and clear the `Scheduled` marker;
- if the operator slips or cancels the change, remove the row in a patch
  release and record it under **Fixed** in `CHANGELOG.md` — the same
  remediation as any wrong schedule row.

### Knowledge-bound rows

A venue or family whose verified-current profile carries a phase no reviewed
source can date — an order-acceptance queue, a PCP window, an early session —
records that gap as a **knowledge-bound row**: the final timeline row, dated
at the repository review that verified the phase (2026-08-22 in the current
tables), routing to the verified-current profile with the citation label
`"<date> review: verified current, onset undated"`. Instants before the row
resolve to the conservative dated grid; instants on or after it resolve to the
verified-current grid. The row makes no onset claim: its date never moves
forward, and only a sourced onset day replaces it.

### Pending effective-date confirmations

- **Nasdaq — 2026-12-06:** [ETA2026-46](https://www.nasdaqtrader.com/TraderNews.aspx?id=ETA2026-46)
  announces the Night Session date, while [Nasdaq Equity 1](https://listingcenter.nasdaq.com/assets/RuleBook/Nasdaq/rules/Nasdaq%20Equity%201.html)
  conditions commencement on Equity Data Plan readiness and a later Nasdaq
  readiness filing. Add the revision only after that filing confirms the day.
- **Cboe EDGX — 2026-12-06:** the [opening-process specification](https://www.cboe.com/document/tech-spec/document/technical-specifications/cboe-titanium-u.s.-equities-opening-process)
  announces the first overnight opening for the 2026-12-07 business date. The
  [SEC approval order](https://www.sec.gov/files/rules/sro/cboeedgx/2026/34-105587.pdf)
  confirms that commencement still requires Equity Data Plan
  readiness and a later EDGX readiness filing. Add the revision only after
  those remaining conditions confirm the day.
- **FINRA TRFs — 2026-12-06:** recheck the Securities Information Processor
  (SIP) Amendment launch against [SR-FINRA-2026-015](https://www.finra.org/sites/default/files/2026-07/SR-FINRA-2026-015.pdf)
  before this date. The announced Sunday-through-Friday reporting regime is not
  modeled while its day remains conditional; add all three TRF revisions only
  when the SIP implementation day is confirmed.
- **NYSE Arca — target 2026-12-06:** the operator's
  [extended-hours program](https://www.nyse.com/trade/equities/extended-hours-trading)
  remains conditional on SIP, DTCC, and exchange-readiness milestones. Do not
  extend the Arca runtime profile until the operator confirms production
  readiness and the actual opening day.
- **MEMX — target 2026-12-06:** the operator's
  [23×5 FAQ](https://info.memxtrading.com/equities-trading-resources/23-5-faq/)
  conditions launch on SEC, DTCC, SIP, and primary-listing-market readiness.
  Keep the existing 04:00–20:00 profile until every condition and the live day
  are confirmed.
- **24X overnight phase — no unconditional day:** the
  [SEC order](https://www.sec.gov/files/rules/exorders/2026/34-106061.pdf)
  records the proposed 21:00–04:00 phase and its remaining conditions. The
  existing live daytime venue is modeled separately; do not infer an overnight
  cutover from the approval or an announced target.
- **MX2 Options — target 2026-09-14:** monitor the operator's
  [phased go-live announcement](https://memx.com/insights/september-2026-go-live-date-for-mx2-options).
  Add an `mx2_options` identity only after the first production tranche is
  confirmed live and its ordinary-product order phases are sourced.
- **IEX Options — target 2026-10-02:** monitor the
  [IEX Options resources](https://www.iex.io/options/resources). Add an
  `iex_options` identity only after the phased production launch and supported
  order envelope are confirmed.
- **Green Impact Exchange — no unconditional day:** monitor the
  [GIX operator site](https://www.tradegix.com/). Registration or an H2 target
  is not a live-session boundary; add a `gix` identity only after a sourced
  production day and hours are available.
- **Nasdaq MRX Options 3C — awaiting operative alert:** the approved additional
  sessions remain unencoded until Nasdaq publishes the required trader alert
  with the production day and final phase table.

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

For a `MarketHoursKey`, also update the single key table, handwritten key
expectations, canonical Serde/parse/display contracts, fixed and dated routing,
the verification row, and date-aware key-calendar parity tests. Document any
venue default that happens to use the family, while warning that the default is
not a venue-wide schedule.

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
   **Added** in `CHANGELOG.md`. If a version has already been cut but is not yet
   tagged or published, put final preparation fixes in that pending version
   section.

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
