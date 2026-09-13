<!-- SPDX-License-Identifier: MIT-0 -->

# Path to the 1.0 release — a plan a smaller agent can execute

Written 2026-09-12 (UTC) at the end of the charter session; re-ordered 2026-09-13 (UTC)
on the maintainer's instruction that **everything is handled in order of importance
before the release, and every identity's holiday table runs from 2010 to whatever the
operator had published as of its inspection date, quick wins first**. This is the
hand-over: everything below is doable one bounded pull request at a time, by an agent
without the context of that session, following `AGENTS.md` (the charter) and this plan.

## 0. How to use this plan

1. Read `AGENTS.md` in full first, every time. It is short and it is the law.
2. Read the last twenty lines of `STATUS.md` in the research store
   (`$EXCHANGE_HOURS_RESEARCH`, default `../exchange-hours-research`) — it records what
   was in flight when the previous session stopped and where each artifact is.
3. Do one numbered item per pull request. A PR is sized to a working day
   (LAW-BOUNDED-WORK); split an item by year range if it is bigger. Where this plan says
   "repair X", X is named precisely and is a few hours of work. Where it says "retrieve",
   the unit is one research block: fetch the operator's own documents (live site first,
   then web-archive captures), save the bytes and their sha256 in the research store,
   have one independent reader try to refute every row against those bytes, then encode.
   Never fill a year from another year, a vendor, or the press.
4. Before every push: `cargo fmt --all --check && cargo clippy --all-targets -- -D warnings && cargo nextest run --all-targets && cargo test --doc && RUSTDOCFLAGS="-D warnings" cargo doc --no-deps && cargo deny check && cargo +1.95 check --all-targets`.
   Run it as a bash script (the shell is zsh; `PIPESTATUS` differs).
5. CodeRabbit reviews every PR. Fix what it finds when it is right; when it asks for a
   schedule change inside a documentation PR, record the point as a gap in the
   identity's evidence file (`docs/evidence/<owner>.md`) and say so in the reply;
   resolve every thread before merging.
6. Every date you write about the repository's own work is the UTC date from `date -u`
   (LAW-UTC-DATES). Exchange effective days stay venue-local.
7. Stop and ask the maintainer only at the points marked **DECISION**. The release
   itself waits until every stage before it is finished and reviewed; it is not started
   early for any reason.

## 1. Where things stand

- `main` is at the merge of #87 (ledger reshape + `docs/evidence/`). The charter (#84),
  the five dormant metals TAS keys (#83) and the fences (#78) are on main.
- #96 `holiday-tables` (this plan's PR) adds the holiday engine plus 427 rows over 22
  identities: the eight served CME families for 2025-01-01..2027-12-31 at T2, and CFE,
  Eurex, ICE Futures U.S. and Coinbase Derivatives from 2026 at T1. CI is green and every
  CodeRabbit thread is resolved at `8ae30df`.
- Fourteen issues are open. Every one is placed at a stage below; the index at the top
  of §3 is the map, and each issue carries a comment naming its place. #36 (the
  whole-ledger coverage audit) was closed on 2026-09-13: dormant rows are not audited
  under the charter.
- This plan is executed by one agent and reviewed by another at the end of each stage,
  following `AGENTS.md` **Reviewing a change**; the executing agent posts a stage
  summary on the PR that closes the stage, listing the issues closed and any gap
  recorded, and does not start the next stage until the review is done.
- Research store map: `architecture-review/` (the review and its four fact reports),
  `holidays/` (seven verified holiday-schedule blocks 2010–2027 as JSON, their verdicts,
  raw artifacts, and `DESIGN-holiday-tables.md`, the design the engine implements),
  `holiday-tables/` (implementation results), `ledger-reshape/`, `cme-globex/`,
  `sgx-pre2020/`, `ecbtc/`.
- The ledger's `Service` column is the authority on tiers (LAW-SERVICE-TIERS). Today it
  lists 16 served identities: the eight keys SharurPlatform maps
  (`globex_equity_index`, `globex_energy`, `globex_grains`, `globex_fx`,
  `globex_interest_rates`, `globex_livestock`, `globex_cryptocurrency`,
  `globex_nikkei_225_dollar`) and the venues `cme`, `cbot`, `comex`, `nymex`, `cfe`,
  `eurex`, `iceus` and `coinbase_derivatives`. Small Exchange (SMFE) is dormant: it has
  been closed since 2025-03-24. Every other identity is dormant.
- Holiday coverage today, from the ledger's `Holidays` column:

  | Tier | With a table | Without |
  |---|---|---|
  | Served | 12 (none reaches 2010) | 4: `cme`, `cbot`, `comex`, `nymex` |
  | Dormant | 10 (from 2026 only) | 106 |

## 2. The release target

`Cargo.toml` says `1.0.0`, `CHANGELOG.md` has a `[1.0.0] - 2026-08-22` section, and no
`v1.0.0` tag exists (tags stop at `v0.2.2`): 1.0.0 was cut and never published, and a
large `[Unreleased]` section sits on top of it. **DECISION (maintainer):** release
everything as **1.0.0** on the release date (fold `[Unreleased]` into the pending
`[1.0.0]` section and re-date it — the repository's own housekeeping rule says a cut but
unpublished version absorbs its preparation), or bump to 1.1.0. The plan assumes 1.0.0.
Nothing in the unreleased work breaks the 1.0.0 API: the additions are keys, the three
holiday accessors, and documentation.

## 3. Stages, in order of importance

**Issue index.** Every open issue, its stage, and whether the tag waits for it.

| Issue | Stage | Blocks the tag |
|---|---|---|
| #95 CME venue intersections | 2.1, then every 2.2 wave | yes |
| #92 wave 2010–2012 fix round | 2.2 order 1 | yes |
| #89 wave 2016–2018 verbatims | 2.2 order 2 | yes |
| #90 wave 2022–2024 verbatims | 2.2 order 3 | yes |
| #91 wave 2019–2021 truncation | 2.2 order 4 | yes |
| #88 wave 2013–2015 retrieval | 2.2 order 5 | yes |
| #98 fixed `### Documents` shape for the non-CME venues | 2.3, or a doc-only PR before it | yes |
| #97 coverage-gate narrowing | 3, before stage 4 starts | yes |
| #79 equity-index 2012 Sunday queue | 3 | no (order-entry gap; stays open if unsourced) |
| #77 seasonal-key doc sentences | 3 | yes (doc-only) |
| #86 cutovers outside `revisions!` | 3 | yes (served venues `coinbase_derivatives`, `eurex`) |
| #94 `DayPolicy::may_affect` | 3, may follow the release | no (additive trait method) |
| #93 fifth `HolidayKind` | 3, may follow the release | no (`HolidayKind` is `#[non_exhaustive]`; the Saturday rows stay declared gaps) |
| #85 narrative debt | 4, drained by the PRs that touch each module | no (dormant) |

**The ordering rule.** Served before dormant; inside a tier, quick wins first, meaning
work whose evidence is already retrieved and verified comes before work that needs a new
retrieval. Nothing is tagged until stage 6's gate holds. Stages 3 and 5 are independent
of the holiday work and may interleave with it.

**Two definitions every holiday PR uses.**

- **Floor.** Coverage starts on 2010-01-01, or on the identity's first trading day if
  that is later (for example Coinbase Derivatives or MEMX). A venue that has stopped
  trading ends on its last trading day (Small Exchange: 2025-03-21). If the operator's
  own documents, archives included, do not reach the floor, the table starts where they
  do and the evidence file names the gap and what would close it (the amended
  LAW-HOLIDAY-SCOPE).
- **Published future as of the inspection date.** The inspection date is the UTC date
  the operator's calendar was last read for that identity; the `retrieved` stamps on the
  evidence file's documents carry it, and the `## Holidays` section's opening paragraph
  states it. Coverage ends on the last date the operator had published unconditionally
  at that inspection. A "preliminary", "indicative" or "to be announced" calendar is not
  published: Eurex 2027 is the precedent, and it does not ship.

### Stage 1 — Land #96

- Merge (squash) once CI and CodeRabbit are green on the head commit. Stage 1 is done
  when this file is on `main`.
- Background for later waves, in `holiday-tables/ASSEMBLE-result.md`: the gate window
  is `[D-1, D+19]`; nine `globex_cryptocurrency` Closed rows were withdrawn because the
  five-day era has no business-date roll (memo D9 is amended in the PR's Review
  section); CME document ids are `CME-SVC-<first eventDate>`.

**Wave 1 landed 2026-09-13 (UTC).** The eight verifier defects are repaired and
re-verified in the research store's block (the repaired copy and the repair
script are in `.wave6-scratch/`, which the PR removes; the committed record is
the evidence files' quotations). Outcome per family, over trade dates
2010-01-01 .. 2012-12-31, all **T1** from CME's own holiday-calendar PDFs:
`globex_equity_index` 28 rows, `globex_interest_rates` 27, `globex_fx` 45,
`globex_energy` 30, `globex_grains` 12, `globex_livestock` 11 — **153 rows**.
Each family's table then declares **two audited windows**, `2010-01-01 ..
2012-12-31` and `2025-01-01 .. 2027-12-31`: the intervening 2013-2024 interval is
audited by neither wave's documents, so it is named as the remaining stages'
work rather than counted as normal (`HolidayCoverage::windows`). The four CME
venue tables extend over the same years by the same D17 intersection rule:
COMEX and NYMEX carry 30 agreed rows each and nothing withheld, while CBOT
states nothing on 31 dates and CME on 49 — `Unsourced` rows, the disagreement
named per date in their evidence files. Over both eras CBOT therefore carries 71
rows (9 stated, 62 `Unsourced`) and CME 90 (9 stated, 81 `Unsourced`). `globex_cryptocurrency` is deliberately left at its 2025-2027 window —
CME listed no cryptocurrency product before 2017-12-17, so a 2010-2012 window
would assert an audit of years in which the family did not exist — and
`globex_nikkei_225_dollar` keeps its window because the era's sheets print one
Nikkei line. Three instant shapes the scalar vocabulary cannot state are named
as gaps rather than approximated: the Good Friday eves' 15:30/17:00 stated
re-opens, the 2012-07-03 equity 15:30 re-open, and the eighteen interest-rate
Monday holidays (#101, a normal-week finding the wave surfaced). `venues.rs` is
split into one module per venue because the added rows took it past the 500-line
reviewability guard; no row moved.

### Stage 2 — Served holidays to the floor (release-blocking)

The design memo (`holidays/DESIGN-holiday-tables.md`, §5.1–5.2) names, per block, the
evidence repairs that must happen before its rows ship. The verdict in
`<block>.verify.json` wins over `<block>.json` on any row. Each CME wave is: repair the
named items in the block's JSON, encode the families by copying the wave-1 pattern
(`src/calendar/schedules/holidays/<family>.rs`, the `## Holidays` section in
`docs/evidence/<family>.md`, the seven required tests per family in
`tests/futures_family_boundaries/holidays_<family>.rs`), run the gates, open the PR.

**2.1 CME venue tables for 2025–2027 — no retrieval.** Derive `cbot` (grains ∩ interest
rates), `comex` and `nymex` (the metals and energy halves of `globex_energy`) and `cme`
(the six families that route to it) from the family rows #96 already ships, by memo D17:
full closures agree across families; a date on which the families disagree emits
`HolidayKind::Unsourced` rather than a venue row, and the disagreement is named in the
venue's evidence file. This gives the four served
identities without a table their first rows and starts #95; the D17 agreement
assumption is audited wave by wave, so #95 closes with the last 2.2 wave.

**Landed 2026-09-13 (UTC)** (`holidays/venues.rs`, one module for the four tables).
The rule above was the one implemented; what follows is how its two operative phrases
were read, and both readings follow the `iceus` venue table that shipped first:

- **A disagreement ships `Unsourced`, not silence.** `HolidayCoverage` defines a date
  inside the window with no row as **audited normal**, so dropping a disputed date
  would make the crate positively claim the date was ordinary — false on every one of
  them. `HolidayKind::Unsourced` clips nothing and answers nothing while telling the
  caller the date is special. It was already in the vocabulary and already used by
  `iceus`; nothing in D17's intent is given up, and the reverse evidence fence
  (`every_evidence_holiday_line_exists_in_its_module`) requires the row to exist before
  a venue evidence file may list the date at all.
- **"Disagree" includes a family that states nothing.** A family with no row on a date
  on which another family states one has *audited the date normal*, which is a
  different answer, not a missing one. Counting those as disagreements is what makes
  the `no row in FX` dates unsourced; the closing condition is recorded in `cme.md`.

Result: `cme` 9 `Closed` + 32 `Unsourced`; `cbot` 9 + 31; `comex` and `nymex` 36 rows
each with no disagreement at all, because metals and energy are one key and CME prints
them as one product row. `cbot`'s ledger cadence moves `quarterly` → `monthly`
(LAW-WATCH: a served identity that ships a holiday table), and the README's
holiday-coverage count moves 22 → 26.

**2.2 CME family waves, most-ready first.** Every wave PR also extends the four venue
tables over its own years by the rule in 2.1, so there is no separate venue wave.

| Order | Years | State of the evidence (memo §5.1) | Do first |
|---|---|---|---|
| 1 | 2010–2012 (#92) | round 1 PASS; nothing load-bearing | fix the eight non-blocking defects (quoting convention, the `nikkei 2010-02-15` label, the 2012 Good Friday capture, the 2010-12-23 energy source, the count typo); this wave lands the January-2010 floor — **landed 2026-09-13 (UTC), see the wave note below** |
| 2 | 2016–2018 (#89) | one narrow load-bearing item; "no instant, status or family-level value is wrong" | correct the three Grains verbatims that quote the deleted MGEX Apple Juice row |
| 3 | 2022–2024 (#90) | two state-neutral load-bearing items | repair the 2023-11-23 grains verbatim and the zone-provenance sentence; ship 2023 MLK/Presidents'/Good Friday and the 13 Nikkei 2024 rows as `Unsourced` |
| 4 | 2019–2021 (#91) | no value wrong in 440 rows; evidence discipline only | re-extract the 314 hard-truncated verbatim fields from the saved bytes; withdraw the false "no standalone 2020 Good Friday workbook" claim; Juneteenth 2019–2021 is `Unsourced` |
| 5 | 2013–2015 (#88) | five load-bearing items; needs a retrieval | retrieve `2013-4th-of-july-done.pdf` (it changes 2013-07-03 for livestock, dairy and lumber) and the 42 unretrieved earlier captures, then fix the other named items; no 2013-07-03 row ships before that document is read |

**2.3 Served non-CME venues back to the floor — new retrieval.** Nothing below 2026 was
ever retrieved for these. One PR per venue, split by year range if it exceeds a day,
smallest history first. Each carries its dormant sibling keys, which read the same
operator documents, and each brings the venue's evidence files into the fixed
`### Documents` shape so the id fences cover them (#98; a doc-only PR may close #98
ahead of this):

| Order | Venue | Rides along |
|---|---|---|
| 1 | `coinbase_derivatives` (from its first trading day) | — |
| 2 | `cfe` | `cfe_vix` |
| 3 | `iceus` | `ice_us` and the six `ice_us_*` product keys |
| 4 | `eurex` | the `eurex` key and `eurex_fixed_income` |

**2.4 Refresh the published future.** Within the month of the release, re-inspect every
served operator and extend each table to what it has now published: CME 2028 once it
appears in the trading-hours service, Coinbase Derivatives' Thanksgiving and Christmas
2026 notices, ICE's and Cboe's next calendars, Eurex 2027 once it is final.

### Stage 3 — Served-key hygiene

- **Time-bound first:** `globex_cryptocurrency`'s 2026-09-19 Saturday row is a
  forward-dated **Scheduled** row. After that day, confirm against CME that it happened
  and clear the marker, or remove the row and record the correction (RELEASING.md
  step 5).
- **#79**: source the 2012 Sunday Pre-Open move (16:15→16:00 CT) for
  `globex_equity_index` from CME's own channels (the 2012 trading-hours captures are in
  the research store under `cme-globex/`), or leave the 16:00–16:15 quarter-hour
  withheld and keep the issue open. This is an order-entry gap; it does not affect
  `is_open`.
- **#77**: fix the `session_profile` and `hours_for_market_hours_key` doc sentences so
  they say which state a seasonal key's static table holds. Doc-only PR.
- **#97, before stage 4 starts**: prove the `[D, D+1]` narrowing of the coverage gate
  and re-measure. With 2010–2027 history nearly every day sits within 19 days of some
  row, so the sound `[D-1, D+19]` window opens the gate almost always and the memo's
  hot-path claim does not hold; the fence
  `the_coverage_gate_is_sound_for_every_shipped_row` must stay green.
- **#86**: give the evidence-day fence a second source of dated boundaries, so the
  cutovers `coinbase_derivatives` and `eurex` encode as constants are fenced like a
  `revisions!` row.
- **#94** and **#93** are additive API (a provided trait method; a variant on a
  `#[non_exhaustive]` enum). Do them if time allows; they may follow the release.

### Stage 4 — Dormant holidays to the floor (release-blocking, after stage 2)

Same definitions and the same one-block-per-PR unit. Dormant tables are refreshed on
demand after the release (LAW-WATCH); `holiday_coverage()` tells a caller where each
one ends.

**4.1 Quick wins from evidence already saved.**

- The eleven dormant CME Globex keys (`globex_*_tas`, `globex_mini_grains`,
  `globex_rough_rice`, `globex_weather`, `globex_event_contracts`,
  `globex_event_contracts_btc`, `globex_spot_quoted`) from the CME holiday corpus in
  `holidays/`, for every year that corpus prints the key's own products. Where it does
  not, retrieve; the memo already names Weather and Mini-Sized Grains as carrying
  distinct instants.
- `small_exchange`: its own notices, from 2010 or its first trading day to its last
  trading day, 2025-03-21.

**4.2 Shared calendars: one research block serves many identities.** Each operator's own
statement is the source for its own identities; one operator's calendar is never the
source for another's.

| Order | Group | Identities |
|---|---|---|
| 1 | U.S. equities and equity options | 40, in operator PRs: NYSE group, Nasdaq group, Cboe group, MIAX, MEMX, BOX, IEX, LTSE, TXSE, 24X, Blue Ocean ATS, FINRA TRF. Check options close times and the overnight venues cell by cell |
| 2 | SGX | the SGX derivatives identities (`sgx` and the five `sgx_equity_index_*` keys); `sgx_securities` from SGX's securities calendar |
| 3 | Euronext | the six `euronext_*` venues |
| 4 | ICE Europe group | `iceeu`, `ice_europe_commodities`, `ice_europe_financials`, `ice_endex`, `ice_abu_dhabi`, `ice_canada` |
| 5 | Nasdaq Nordic | `nasdaq_copenhagen`, `nasdaq_helsinki`, `nasdaq_stockholm` |
| 6 | Shared national calendars | `sse` and `szse`; `nse_india` and `bse_india` |

**4.3 Single venues, larger markets first:** `lse`, `xetra`, `eex`, `six`, `tse`,
`hkex`, `krx`, `twse`, `asx`, `tmx_australia`, `tsx`, `b3`, `bmv`, `bme`,
`borsa_istanbul`, `vienna`, `jse`, `tadawul`, `nzx`, `bursa_malaysia`, `set_thailand`,
`idx`, `pse`, `hose`.

Every stage-4 PR that touches a module listed in `NARRATIVE_DEBT` moves that module's
narrative into its evidence file in the same change (LAW-EVIDENCE-FILES); #85 closes
when the list is empty.

**4.4 No holidays by nature.** `always_open` and `unknown` are synthetic;
`binance_futures` trades 24/7 (confirm from Binance's own documents that it observes no
closures; a maintenance window is not a holiday). Each evidence file says so in a
`## Holidays` section, and its ledger `Holidays` cell stays `—`.

**Size, for planning.** Stage 2 is about a dozen PRs, half of them needing no new
retrieval. Stage 4 is roughly forty to fifty, almost all needing one.

### Stage 5 — Consumer-side tasks (SharurPlatform, not this repository)

Hand these to the platform; they are the charter's consumer contract. Only item 1 waits
for the tag.

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

### Stage 6 — The release (RELEASING.md, followed exactly)

0. **The holiday gate.** Every ledger row's `Holidays` cell starts at 2010-01-01, or at
   a later start its evidence file justifies (first trading day, or a named gap where
   the operator's archives stop), and ends at the published future of an inspection
   made in the release's month. The only rows allowed `—` are stage 4.4's. Stages 2, 3
   and 4 are done. If any row fails, do not tag.
1. Branch `release/1.0.0` from `main`; set the version in `Cargo.toml` and `Cargo.lock`
   (already 1.0.0 if the decision in §2 holds); move `[Unreleased]` into `[1.0.0]` with
   the release UTC date; restore an empty `[Unreleased]`; update comparison links.
2. README: installation and migration text; the coverage and assurance prose are derived
   by fences — run the suite and fix what the failure messages print. Do not advance the
   schedule-review cutoff unless every non-synthetic row was reviewed through the new
   date.
3. Clear or correct every **Scheduled** marker whose effective day has passed (stage 3).
4. Gates, then `cargo publish --dry-run --locked` and `cargo package --list --locked`
   (check that `docs/evidence/` and the holiday tables are in the package and that
   nothing repository-only leaked).
5. `cargo bench --bench calendar_queries` — informational; call out a regression. The
   holiday tables will have grown by an order of magnitude; the coverage gate should
   keep a no-row day at one binary search.
6. Open the release PR; merge; from a clean checkout of the merge commit rerun the gates
   and the dry run; `cargo owner --list exchange-hours` and `gh auth status`; tag
   `v1.0.0` annotated; `cargo publish --locked`; GitHub release with the changelog
   section. **DECISION (maintainer): the publish step needs the crates.io credential.**

### Stage 7 — After the release

- LAW-WATCH cadence: monthly for served identities that are high-churn, 24/7 or
  holiday-bearing (after stage 2 that is every served identity); dormant on demand. A
  review is: open the monitoring entry points in `docs/schedules/sources.md`, compare,
  bump the ledger row's reviewed-on (UTC) if unchanged, otherwise a schedule-fix PR.
- Each new holiday year: retrieve each served operator's calendar when it is published
  (CME publishes one to two years ahead), encode per the wave pattern, one PR.
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
