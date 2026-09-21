<!-- SPDX-License-Identifier: MIT-0 -->

# Path to release — complete 2025-onward calendars

Amended 2026-09-21 (UTC). This is the active staged development plan. It replaces
remaining instructions in the September 12–19 plan; that plan and its completed
wave records are preserved in the [historical archive](archive/2026-09-12-path-to-release-before-2025-amendment.md).
The historical archive and the untracked wave/stage handoff prompts are context,
not instructions to restart pre-2025 research. Follow [AGENTS.md](../../AGENTS.md).

## 1. Decision and current implementation

The maintainer adopted these decisions on 2026-09-21 UTC:

- The support floor is permanently **2025-01-01**. It does not advance each year;
  retain 2025 and every subsequent year as the history grows.
- Complete coverage is required for each consumer-reachable instrument's exact
  family or documented venue scope. Dormant completeness does not block release;
  complete a dormant scope before activating it. Broad venue intersections keep
  their explicit limitations and cannot replace a complete family calendar.
- Completeness includes normal-week changes, required order-entry phases, holidays,
  early/late boundaries, pauses, reopenings, extra sessions and trade-date changes.
  An audited window with an `Unsourced` date or a missing phase is not complete.
- Extend each scope through the operator's sufficiently specified, unconditional
  publications. An annual holiday name without session details, an indicative
  calendar or an unpublished year is not proof of ordinary trading. Preserve
  publication dates, exchange effective dates and UTC retrieval dates separately.
- Identity-backed date-aware queries must report unsupported coverage as an error.
  Missing evidence is neither an open market, a closed market, nor an absent session.
- Remove pre-2025 runtime history in Stage 5. Retain the sourced state in force at
  the floor, later revisions, seasonal behavior and boundary context. An older
  document may prove that state; 2025-01-01 is not an invented exchange cutover.
- Reuse captured evidence for 2025 onward. Do not delete research artifacts or
  rewrite Git history; do not repeat old retrieval waves or copy another year's
  holiday pattern into a missing year.

**Current state, inspected at `66b5f59` on 2026-09-21 UTC:** the crate still contains
pre-2025 history, existing infallible/`Option` query signatures, scalar built-in
holiday tables and a caller-owned replacement-session engine. No built-in
replacement-session data ships. The ledger's windows, gaps and counts remain
facts about those tables. This plan/charter PR implements none of Stages 1–7.

The pending manifest version is `1.0.0`; inspected repository tags end at `v0.2.2`.
Do not infer crates.io publication state from local tags. Stage 7 checks both:
use the pending `1.0.0` only if unpublished; a published 1.x contract requires a
major version for the approved breaking query migration. Never reuse a tag.

## 2. Execution contract and handoff

Read this section before each stage. The unit is **one bounded PR**, normally no
more than a working day. A stage with named sub-PRs is not permission to combine
them. Dependencies must be reviewed before dependent work begins. This plan is
intended to be executable without the conversation that produced it.

1. Read the charter, this plan, the stage's issue, the actual implementation and
   relevant tests. Read the research store's `STATUS.md` for the latest reviewed
   head, completed work and artifact corrections. Prefer current files and reviewed
   verdicts over stale handoff claims.
2. Use a fresh worktree based on the reviewed dependency commit. Preserve unrelated
   changes. The research root must be explicit: `EXCHANGE_HOURS_RESEARCH` points to
   the directory holding `STATUS.md` and `holidays/`. The main checkout's sibling
   is `/Users/agedvagabond/Developer/exchange-hours-research`; a managed worktree's
   `../exchange-hours-research` is usually wrong. Raw artifacts are not committed.
3. Read saved artifact bytes and their corrected indexes/verdicts before retrieving
   again. Follow the operator/live/archive channel order in the maintenance guide.
   A source refusal twice ends that channel attempt. Record the exact gap and
   closing condition; bounded effort does not waive the release completeness gate.
4. Keep every PR compiling, with its own tests, evidence and documentation. Run
   [the full verification chain](#verification-for-every-pr) before declaring it
   complete. Tests exercise only the public surface. Benchmark engine changes.
5. Follow the charter's independent review procedure against the exact PR head.
   Resolve blocking findings and re-run affected checks after fixes. The implementing
   agent must not call its own source reading an independent review. No repeated
   multi-agent research cycles are required.
6. Before handing off, update the stage issue and research-store `STATUS.md` with:
   stage/sub-PR; branch and PR; exact head and review verdict; artifacts and digests;
   checks and results; acceptance items met; remaining issue numbers; next stage and
   its entry conditions. Name the exact runtime model in GitHub posts and commit
   trailers under LAW-AGENT-ATTRIBUTION, never in product files.
7. A stage is complete only when all its acceptance conditions hold. An issue may
   track a gap without resolving it. Stop dependent work on an unresolved blocker;
   continue only independent work already authorized by the stage.

### Verification for every PR

Run the quality checks in this order, then the separate MSRV check:

```bash
cargo fmt --all --check
cargo clippy --all-targets -- -D warnings
cargo nextest run --all-targets
cargo test --doc
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps
cargo deny check
cargo +1.95 check --all-targets
```

Run the shell as bash with `set -euo pipefail`, so the first failure stops the
chain and no later success masks it. For engine changes also run
`cargo bench --bench calendar_queries` and record the comparison. Use the full
[release procedure](../../RELEASING.md) for package/tag/publish checks in Stage 7.

## 3. Stage and issue index

| Stage | Tracking issue | PR units | Depends on | Release gate |
|---|---|---|---|---|
| 0 — plan and charter | this documentation PR | one documentation PR | maintainer decisions above | plan reviewed |
| 1 — evidence/coverage inventory | [#114](https://github.com/SharurTrading/exchange-hours-rs/issues/114) | one inventory PR | Stage 0 | yes |
| 2 — explicit coverage contract | [#115](https://github.com/SharurTrading/exchange-hours-rs/issues/115) | 2A metadata/types; 2B query/adaptor migration | Stage 1 | yes |
| 3 — built-in special sessions | [#93](https://github.com/SharurTrading/exchange-hours-rs/issues/93) | one engine PR; data follows in Stage 4 | Stage 2 | yes |
| 4 — complete served data | [#116](https://github.com/SharurTrading/exchange-hours-rs/issues/116) | one operator/family per PR | Stages 1–2; Stage 3 for blocks | yes |
| 5 — remove obsolete history | [#117](https://github.com/SharurTrading/exchange-hours-rs/issues/117) | CME; other served; dormant cleanup PRs | Stages 1–2; preserve Stage 4 corrections | yes |
| 6 — consumer migration | [#118](https://github.com/SharurTrading/exchange-hours-rs/issues/118) | separate SharurPlatform PR | Stage 2; validate final Stage 4–5 candidate | yes, candidate before tag; published pin after |
| 7 — release verification | [#119](https://github.com/SharurTrading/exchange-hours-rs/issues/119) | release PR, then published consumer pin | reviewed Stages 1–6 | final gate |

Stage 4 scalar/source work and Stage 5 pruning may proceed independently after
Stage 2, provided each PR rebases on any overlapping reviewed change. Do not split
one day's exception implementation between scalar and replacement layers across
PRs. Stage 6 validates with the release candidate before publication and pins the
published version afterwards, avoiding a dependency cycle with Stage 7.

Existing issues retain these dispositions; links to completed historic work live
in the archived plan. This table changes priority, not the truth of a report.

| Issue | New disposition | Blocks the new release |
|---|---|---|
| [#93](https://github.com/SharurTrading/exchange-hours-rs/issues/93) special-session blocks | Stage 3; exact data in Stage 4 | yes |
| [#98](https://github.com/SharurTrading/exchange-hours-rs/issues/98) non-CME document-id tables | Stage 4, for remaining touched evidence | yes for served evidence |
| [#77](https://github.com/SharurTrading/exchange-hours-rs/issues/77) seasonal snapshot documentation | Stage 2B; name the static state and retain seasonal tests | yes |
| [#79](https://github.com/SharurTrading/exchange-hours-rs/issues/79) undated 2012 Sunday queue | Stage 1/4 baseline audit; no search for the old cutover solely for its own sake | yes if a 2025-onward phase is still withheld; otherwise historical only |
| [#86](https://github.com/SharurTrading/exchange-hours-rs/issues/86) cutovers outside `revisions!` | Stage 2B/5 audit all retained post-floor or boundary-context selectors | yes for any retained unfenced boundary |
| [#105](https://github.com/SharurTrading/exchange-hours-rs/issues/105) dairy/lumber scope mismatch | Stage 1/4/6 check actual consumer mapping and 2025-onward rows | yes if a served instrument uses the wrong family; otherwise dormant evidence |
| [#85](https://github.com/SharurTrading/exchange-hours-rs/issues/85) narrative debt | move narrative when Stage 4/5 touches its module | only the touched-module obligation |
| [#94](https://github.com/SharurTrading/exchange-hours-rs/issues/94) overlay coverage optimization | keep as a separate performance follow-up | no, unless a correctness defect is found |
| [#107](https://github.com/SharurTrading/exchange-hours-rs/issues/107) per-occurrence optimization/benchmark target | preserve soundness fences; benchmark in Stage 2/3/5 | no numeric performance target is newly imposed |
| [#89](https://github.com/SharurTrading/exchange-hours-rs/issues/89) pre-2025 quotations | superseded research scope; preserve disclosure until Stage 5 removes the affected shipped claims | no new retrieval; [#117](https://github.com/SharurTrading/exchange-hours-rs/issues/117) owns removal |
| [#101](https://github.com/SharurTrading/exchange-hours-rs/issues/101) 2010–2012 interest-rate holiday grid | superseded research scope; Stage 5 removes the affected era | no new retrieval; [#117](https://github.com/SharurTrading/exchange-hours-rs/issues/117) owns removal |
| [#110](https://github.com/SharurTrading/exchange-hours-rs/issues/110) 2016–2018 livestock encoding | superseded; do not encode another old era | no |
| [#112](https://github.com/SharurTrading/exchange-hours-rs/issues/112) 2022/2024 Coinbase source defects | superseded research scope; preserve disclosure until Stage 5 removes affected rows | no new retrieval; [#117](https://github.com/SharurTrading/exchange-hours-rs/issues/117) owns removal |

Do not close historical issues as “fixed” when scope merely changed. Keep their
existing bodies and artifact references. Stage 5 may close them as superseded once
no affected shipped claim remains. Do not close mixed-scope #79/#86/#105 by date alone.

## 4. Stage 0 — plan PR only

**Entry / inputs:** the maintainer decisions above, the current charter/release
plan, actual source and ledger state, and current issue bodies.

**Edits:** amend this plan and the charter; align maintenance and release guidance;
add explicit transition notices where current APIs differ from the target. Preserve
all runtime data and coverage counts. Preserve completed stages in the archive.
Create/link #114–#119, reuse #93, and record the issue dispositions above.

**Tests / acceptance:** documentation links and stage dependencies are consistent;
all future work has an issue; current capabilities are not described as already
migrated; no Rust, runtime table, consumer or package-version change is present;
full verification passes. Open and attach the attributed PR and leave it unmerged.
The next agent begins Stage 1 only after plan review.

**Handoff:** PR/head, checks, review status, issue map and explicit “Stage 1 next”.

## 5. Stage 1 — inventory saved evidence (#114)

**Entry:** reviewed Stage 0. **Inputs:** actual consumer family/admission maps;
`docs/schedules/verification.md`; owner modules, holiday tables and evidence files;
the following paths relative to `EXCHANGE_HOURS_RESEARCH`:

| Saved input | Reuse boundary |
|---|---|
| `holidays/cme-2025-2027.json`, its `.r0`/`.r2` and `.verify`/`.verify.r1` siblings | use corrections/verdicts to identify authoritative rows; do not assume a suffix alone means final |
| `holidays/raw/cme-2025-2027-fix/` and `holidays/raw/cme-2025-2027-repair/` | resolve cited bytes, indexes and digests before trusting transformed rows |
| `holidays/cde-2021-2026.json`, its `.verify.json`, `holidays/raw/cde-2021-2025/` | only 2025 onward contributes historical coverage; retain earlier baseline evidence when needed |
| `holidays/cfe-eurex-ice-cde-smfe-2026-2027.json`, verdicts, raw/fix indexes | reuse each scope's actual saved years; retrieve missing 2025 material separately |
| `holidays/DESIGN-holiday-tables.md`, `holiday-tables/DECISIONS.md`, owner evidence files | distinguish implemented scalar behavior from the approved block/error work |

**One inventory PR:** add `docs/schedules/coverage-2025.md` with one row per served
instrument scope: canonical identity, consumer routing evidence, baseline source,
normal-week and holiday audited ranges, actually complete ranges, missing/disputed
dates or phases, special-session needs, publication horizon, document ids and
artifact locations/digests, and closing issues. Identify no-holiday and synthetic
scopes explicitly. Keep lengthy quotations in owner evidence files. Check #79 and
#105 for effects that survive the new floor; do not copy old served counts.

**Tests / acceptance:** resolve every cited inventory artifact and its digest;
compare inventory membership with the ledger and actual consumer map; account for
`Unsourced` rows and narrative gaps separately from outer coverage bounds. Record
missing source bytes as missing, not verified. No runtime changes. Full gates pass.

**Handoff:** inventory commit, evidence corrections, missing inputs/issues, and the
specific identity list Stage 2 coverage metadata must cover.

## 6. Stage 2 — coverage errors (#115)

**Entry:** reviewed inventory. **Inputs:** `ExchangeCalendar`, `PolicyCalendar`,
`CalendarSource`, `QueryContext`, holiday/exception coverage, identity profile
routing, bulk builders, and the public session/candle/period adapters.

**2A — metadata and error types, one PR.** Add documented, non-exhaustive public
coverage/error types. Store sourced normal-week ranges separately from complete
calendar ranges and known internal gaps. Reuse sorted static data; an empty holiday
table needs an explicit audited-normal/no-holiday assertion, not fabricated rows.
Metadata includes the identity and supported ranges; errors distinguish unsupported
coverage from bounded search exhaustion. Inspectable metadata is not permission to
return a fabricated schedule.

**2B — query and adapter migration, one compiling PR.** Identity-backed date-aware
queries return `Result<existing_value, CalendarQueryError>`; preserve existing
`Option` inside `Ok` where it means genuine absence. Propagate errors through status,
orders, boundaries, trade dates, next sessions, candles, weeks/months and caller
layers. Do not swallow errors with `.ok()`, `unwrap_or(false)` or a default grid.
Migrate all in-crate callers, examples and applicable tests together. Resolve #77's
static-season documentation and #86's retained-boundary fence obligations.

The implementation contract is:

- The instant-query floor is venue-local midnight at the start of 2025; existing
  date-keyed APIs use 2025-01-01 in their documented local-date domain. Public
  timestamps remain UTC. Do not impose a single UTC midnight on every venue.
- Retain enough earlier internal context to answer an in-range query's full
  session/gap; a returned session opening may precede the floor. Direct earlier
  identity queries still error. Neither a floor nor an upper bound truncates a
  real session to manufacture a result.
- Check dates needed to establish the answer, not just the supplied instant.
  Searches cannot skip an unknown date to claim the next known session is next.
  Period bounds needing unknown days error. Do not reject on speculative internal
  probes unrelated to the result; fence the actual dependency window.
- `hours_at`/`hours_for_*` and bulk snapshot selection check normal-week coverage
  only and retain their no-holiday contract. Detached caller-supplied `MarketHours`
  queries remain unchanged. `session_profile` remains a static table accessor.
  Resolve invariant timezone metadata without querying a pre-floor epoch snapshot.
- `without_holidays` explicitly selects the normal-week contract, not complete
  calendar coverage. `DayPolicy` alone cannot certify unknown dates. A correctly
  scoped full caller replacement may supply a missing date above the floor;
  `KnownNormal` still needs a sourced baseline. Caller data never improves the
  built-in ledger, and no overlay bypasses the identity floor.
- Synthetic calendars keep their explicitly documented mathematical semantics;
  real no-holiday calendars require evidence. Preserve sourced pre-launch closures
  within the supported date domain as known closures, not missing evidence.

**Tests / acceptance:** public integration cases for before/at the floor in opposing
zones, crossing sessions, post-close queues, future bounds, internal gaps, lookahead,
periods, overlays and synthetic/no-holiday scopes. Supported results remain stable.
Keep `Copy + Send + Sync`, deterministic total functions and bounded allocation-free
built-in queries. Both PRs pass full gates; 2B includes benchmark comparison.

**Handoff:** complete signature migration map, error semantics, tested boundary
cases and the supported metadata fixtures Stage 3 and the consumer will use.

## 7. Stage 3 — built-in special sessions (#93)

**Entry:** reviewed Stage 2. **Inputs:** `ExceptionBlock`, `SessionExceptionSource`,
`StaticSessionExceptions`, `query/replacement.rs`, `HolidayKind`, holiday macro/fences,
and the exact unresolved 2025-onward shapes in the inventory and #93.

**One engine PR:** extend built-in holiday data with a static replacement-block kind,
reusing the existing replacement resolver and validation. Each row retains its tier
and document id. Built-in blocks replace the complete trade date; explicit caller
`Closed`/`ReplaceSessions` wins over that arrangement, then `DayPolicy` clips the
chosen result. Do not apply a scalar row a second time to an already replaced date.
`without_holidays` detaches all built-in date exceptions. Add fixtures, not invented
exchange data; operator-specific rows land in Stage 4 with their evidence.

**Tests / acceptance:** public `session_exceptions` and holiday/policy suites cover
pause/reopen, regular-only closure, added Saturday sessions, multi-day and reassigned
trade dates, order-entry changes, DST and precedence. Validate block ordering, bounds,
scope and citations. Every query family observes the same replacement. Full gates and
benchmark comparison pass. An unstated halt instant remains a source gap even when
the new representation could encode a hypothetical value.

**Handoff:** supported block format, fixture coverage, performance result and exact
inventory rows now representable; unresolved evidence stays linked to #116.

## 8. Stage 4 — complete served calendars (#116)

**Entry:** reviewed inventory and Stage 2; Stage 3 before any block rows. **Inputs:**
Stage 1's exact scopes and artifacts, current family/venue modules, source registry,
operator publications and per-family public integration suites.

**PR order:** (1) one CME family per PR, reusing its captured 2025–2027 artifacts;
(2) Coinbase 2025 onward and new notices; (3) CFE 2025 and refreshed future;
(4) ICE Futures U.S. 2025 and refreshed future; (5) Eurex 2025 and refreshed future.
A shared operator PR may carry sibling keys using the same evidence if bounded.
For every PR:

- Reconcile published normal-week changes, baseline and required phases as well as
  every holiday arrangement. Include known one-off market/session exceptions within
  the documented scope; per-instrument expiry and ticker-level events remain out.
- Encode only exact sourced scalar or replacement rows; resolve gaps rather than
  expanding a window over them. Update complete coverage only when all its claims
  hold. #98 covers the remaining non-CME document-id tables. #105 requires exact
  product-scope handling; shared branding is not proof of matching hours.
- Keep broad venue intersections derived from agreed family data and explicitly
  partial on disagreement. Do not weaken their meaning to make a release counter pass.
- Preserve every artifact id/digest and write evidence, tests, ledger cells and
  CHANGELOG together. Reinspect future publications; do not assume all operators
  expose the same next year or final session details.

**Tests / acceptance:** independently derive row expectations from cited bytes;
cover every retained cutover, holiday kind, required phase, end-exclusive boundary,
wrap, weekend and trade-date consequence. Mutation-check each changed behavior.
Every served instrument scope is complete from the floor/later launch through its
stated unconditional publication horizon, including today as of its review. A shorter
historical window or unresolved in-window gap blocks completion; record the precise
missing source instead of passing the gate. Full checks pass per PR.

**Handoff:** per-scope complete windows, verified publication horizon, reviewed heads
and remaining issues; do not mark the parent stage complete before every scope passes.

## 9. Stage 5 — remove obsolete history (#117)

**Entry:** reviewed Stage 1 baseline inventory and Stage 2 errors. **Inputs:** retained
profiles/seasonal selectors, revision and exact-instant fences, golden grids, owner
evidence, coverage records and any overlapping Stage 4 corrections.

**Separate cleanup PRs:** CME owners, other served owners, then dormant owners; split
further by cohesive module if required. Retain the state sourced at the 2025 floor,
all later changes and enough context for complete New Year session/gap answers.
Remove obsolete earlier runtime profiles/revisions and historical identity coverage
expectations. Do not rename identities, rewrite wire names, freeze seasonal behavior,
reset a real launch, or add a fictitious January-2025 exchange revision. Retain cited
older documents needed for the baseline and all raw research. Move touched narrative
under #85 and reconcile retained dated selectors under #86.

**Tests / acceptance:** preserve independently captured in-range public results across
representative normal weeks, all retained cutovers/holiday rows and boundary context;
justify intentional corrections with separate sources. Test unsupported earlier
identity requests. Retain generic date-arithmetic and detached-snapshot fixtures even
when their dates precede 2025. Update golden/history fences only for genuinely removed
coverage claims. Remove affected shipped claims before retiring #89/#101/#112 as
superseded; do not implement #110's old encoding task. Check #79's surviving baseline
consequences separately. Full gates and relevant benchmarks pass per PR.

**Handoff:** owners pruned, retained baseline citations and boundary dependencies,
closed/surviving issue numbers, regression results and remaining owners.

## 10. Stage 6 — consumer migration (#118)

**Entry:** reviewed Stage 2; final validation uses the Stage 4/5 candidate. **Inputs:**
the actual SharurPlatform adapter maps, query call sites, listing windows and history
walks. This work belongs in a separate consumer PR, not this repository's plan PR.

**Edits:** map each reachable instrument to a complete exact family/documented scope;
resolve still-present ICE/Eurex routing gaps. Propagate coverage errors to an explicit
unavailable-data path; do not translate them into market closure or silently skip
history. Clamp walks to both listing bounds and supported calendar bounds. Preserve
trade-type disclosure. Use a temporary local candidate dependency only for validation;
the final consumer dependency must pin the published version.

**Tests / acceptance:** consumer tests cover live and historical successful answers,
out-of-coverage and listing boundaries, future search exhaustion and exact family
selection. Validate the candidate before the release; link the consumer PR/head in
#118/#119. Stage 7 publishes, then completes the tagged dependency pin and verifies
it in the consumer. Do not require an unpublished tag to exist before candidate tests.

**Handoff:** consumer PR/head, candidate crate head, validation results and the exact
pin change waiting for publication.

## 11. Stage 7 — release verification (#119)

**Entry:** reviewed Stages 1–5 and the Stage 6 candidate migration. **Inputs:** their
handoffs, current ledger/coverage inventory, operator publication horizons, outstanding
issue dispositions and [RELEASING.md](../../RELEASING.md).

**Release PR:** re-inspect every served operator in the release month; incorporate
new sufficiently specified future publications and errata. Confirm elapsed Scheduled
rows, including any old marker still present after pruning. Update actual coverage,
README and API migration documentation; do not advance a global review date merely
because this narrower programme finished. Verify registry and tag state before
choosing the breaking release version described in section 1.

**Release gate:** every actual consumer instrument maps to a complete scope from
2025/later launch through its stated publication horizon, covering the inspection
present. No unresolved required phase, normal-week or holiday gap may be hidden by
an outer window, an issue, a caller overlay or a broad venue intersection. Dormant
completion is not required; Stage 5's obsolete-runtime cleanup is required. Track
#94/#107 as performance follow-ups without inventing a new numeric release target.

**Tests / acceptance:** full quality/MSRV checks, `cargo publish --dry-run --locked`,
`cargo package --list --locked`, engine benchmarks and candidate consumer validation.
Follow the existing reviewed-release/clean-checkout/authentication/tag/publish sequence;
never tag from the plan PR or an unreviewed implementation branch. Verify the actual
published crate/docs, finish Stage 6's version pin and run consumer checks again.

**Handoff:** release version, crate/consumer heads, tag, publication verification,
per-scope horizons and remaining non-blocking issue links. Monthly served-calendar
reviews and updates as final operator publications arrive continue under LAW-WATCH;
no year is dropped from the 2025-onward history.
