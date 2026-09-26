<!-- SPDX-License-Identifier: MIT-0 -->
<!-- Working note for the next agent; untracked, like the wave-4 prompt before it. -->
<!-- Revised 2026-09-17 08:50 UTC for the #97 landing; the wave-5 substance is unchanged. -->

# Next stage: stage 2.2 wave 5 — the CME 2013–2015 era (closes #88 and #95)

You are an autonomous coding agent working in `exchange-hours-rs` (a Rust crate that ships
sourced exchange session calendars). This is a handover: assume you have no memory of the
session that produced it. Read everything below before touching anything.

## State at hand-off (2026-09-17 08:50 UTC)

- `origin/main` = **`6fc7bd7`**, the wave-4 merge (PR #106, which closed #91). Wave 4's tooling,
  its evidence fences and `LAW-AGENT-ATTRIBUTION` are all on `main`.
- **#97** — the coverage-gate narrowing — is landed as **PR #108**
  (`coverage-gate-narrowing`, head `5d4547f`, base `main`): `MERGEABLE`, CI green
  (`quality` ~26 min, `msrv (1.95)` pass), six review threads all resolved, **awaiting the
  maintainer's merge**. It closes #97 and leaves #107 open. **Nothing in wave 5 depends on it**,
  so start whenever you are told to; if #108 has merged by then, your tree also carries the
  `[D, D + 1]` narrowing and its premise fence (which adds ~80 s to the suite). Say in your PR
  body which head you branched from.
- **The main checkout is stale** — it sits on `0a2de80`, one merge behind `origin/main`.
  `git fetch origin` before anything, and do the work in a fresh worktree:
  `git worktree add .worktrees/wave5 origin/main`.
- **The 2013–2015 era is still absent from every table, and #97 changed no session data.**
  `globex_equity_index`'s `coverage:` clause and its ledger `Holidays` cell both read
  `2010-01-01..2012-12-31, 2016-01-01..2018-12-31, 2019-01-01..2021-12-31,
  2022-01-01..2024-12-31, 2025-01-01..2027-12-31` — the same shape as the other five
  families with the gap, so nothing in stage 2 has moved since wave 4 merged. Confirm in one
  command each:
  `sed -n '/coverage:/,+1p' src/calendar/schedules/holidays/globex_equity_index.rs` and
  `grep -m1 globex_equity_index docs/schedules/verification.md`.
- This file is **untracked**: read it from the main checkout; do not add it to the PR.

## 0. Read these first, in this order

1. `AGENTS.md` — the charter and the law. Read it in full; it is short. **It carries
   `LAW-AGENT-ATTRIBUTION`** — every GitHub artifact an agent authors (a pull-request or issue
   title and body, a comment, a review, a review reply, an inline comment, a release note, any
   other post) **opens by naming the exact model that wrote it**, because the agent writes
   through the maintainer's login and speaks as the maintainer. Commit messages carry a
   `Model:` trailer. This is not optional; both #106 and #108 were reviewed against it.
2. `docs/plans/2026-09-12-path-to-release.md` §3 stage 2.2 — you are **wave 5, the last CME
   family wave**. Waves 1 (2010–2012), 2 (2016–2018), 3 (2022–2024) and 4 (2019–2021) have
   landed; wave 5 closes the CME family obligation and with it issue **#95**. §2.3 and §2.4 name
   what comes next, and this file's §8 repeats them.
3. `../exchange-hours-research/STATUS.md`, the last ~200 lines — the handover log. The wave-4
   section is the closest model for a *data* wave (repair-first, tooling, review rounds, traps);
   the **#97 section** is the model for a *production* change (a proved narrowing, a premise
   fence, an A/B measurement, two review rounds).
4. The landed wave PRs for the shape to copy: **#102** (wave 1), **#103** (wave 2), **#104**
   (wave 3, which also extended the venue tables), and **#106** (wave 4, the closest: it repaired
   its block first, moved editorial annotation to a `note` field, and added tables and fences).
   `git show --stat 6fc7bd7` for wave 4. **#108** is the model if you touch production code: one
   proof, one fence that holds the premise over the whole population, one measured A/B in the
   research store.

Wave 4's tools survive in `tools/` and are the templates you copy: `wave4_repair.py`,
`wave4_rows.py`, `check_wave4.py`, `encode_wave4.py`, `venue_intersection_wave4.py`,
`evidence_wave4.py`, with `tools/README.md` documenting the handoffs and the
`WAVE4_RESEARCH`-style required variable. Rename them for wave 5, re-point them, and keep the
variable name honest in the README.

## 1. The task

**Wave 5: CME Group's holiday schedules for calendar years 2013, 2014 and 2015.**

Encode the crate's served families' holiday rows for **venue-local trade dates
2013-01-01 .. 2015-12-31** as a new audited era in each family's static holiday table, at the
tier each date's document warrants; extend the four CME venue tables (`cme`, `cbot`, `comex`,
`nymex`) over the same years by the D17 intersection rule; write the evidence; add the tests;
update the records; and open one PR that **closes #88 and #95**.

Six of the eight served families have this gap — `globex_equity_index`, `globex_energy`,
`globex_fx`, `globex_grains`, `globex_interest_rates`, `globex_livestock`.
`globex_cryptocurrency` has no era before 2019 (no product existed: block `missing[1]`) and
`globex_nikkei_225_dollar` none before 2016 (no Nikkei line exists on any 2013–2015 sheet:
block `missing[0]`), so neither declares this window — say so in their evidence files rather
than inventing one.

**After wave 5 the only CME family gap left is `globex_livestock` 2016–2018** (it never
declared that era because CME's schedule for it is not the one the other six are encoded
from). Name that gap in the PR body and in `globex_livestock.md`; if it needs work of its own,
open the issue and cite the number (LAW-FOLLOW-UPS-ARE-ISSUES).

## 2. The evidence you already have

The research store (`../exchange-hours-research`) holds the wave-5 block and its verdict:

- `holidays/cme-2013-2015.json` — the block: **72 dates, 346 family rows, 34 document codes,
  every row T1**; statuses 96 `closed`, 96 `early_close`, 121 `normal`, 25 `late_open`,
  8 `modified`. Round 0 is `holidays/cme-2013-2015.r0.json`.
- `holidays/cme-2013-2015.verify.json` — **round 2, `matches: false`, 12 discrepancies
  (5 load-bearing)**. **This is your repair list**, and its `fix` fields are the wording the
  verifier will accept. `holidays/cme-2013-2015.verify.r1.json` is the round-1 verdict it
  supersedes.
- Raw artifacts: `holidays/raw/cme-2013-2015/` (round 0: PDFs, `.xls`, `cdx_holiday_calendar.json`,
  `names.txt`, `INDEX.md`), `holidays/raw/cme-2013-2015-fix/` (round 1: 45 `.xls`/`.zip`,
  `cdx_all.json`, `todo.tsv`, `txt/`, `INDEX.md`), and `holidays/raw/cme-2013-2015-verify-r2/`
  (the verifier's own retrieval, `cdx/cdx-2013-files.json`, `cdx-2014-files.json`,
  `cdx-2015-files.json`).

**The blocker named in the plan is already cleared.** §3 stage 2.2 order 5 says "retrieve
`2013-4th-of-july-done.pdf` … no 2013-07-03 row ships before that document is read". The
round-2 verifier retrieved it and saved it:

```
holidays/raw/cme-2013-2015-verify-r2/new/2013-4th-of-july-done__20130717050333.pdf
  sha256 768c7813542459dbc5443e79e514bb12fd511d93284beeb880af167f6049e0f0, 73,249 bytes
  replay https://web.archive.org/web/20130717050333id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2013-4th-of-july-done.pdf
  footer "Last updated 7/2/2013";
holidays/raw/cme-2013-2015-verify-r2/txt/2013-4th-of-july-done__20130717050333.txt  (the dump)
```

That dump carries CME's later revision of the 2013 Independence Day schedule — read it first,
because it moves recorded statuses. It prints, under `Wednesday, July 3`:

- `1200 CT – Early close for Dairy`, `1200 CT – Early close for Lumber`,
  `1202 CT – Early close for Lumber Options`, `1215 CT – Early close for Livestock Futures & Options`;
- for grains: `1200 CT – Early CBOT & KCBT close`, `1215 CT – Early MGEX Wheat & Apple Juice close`,
  `1230 CT – Early CBOT Mini-Sized grain close`, `1230 CT – Early MGEX Indices close`;
- the equity line's `1215 CT –Early close`, the energy/rates/FX lines and the rest of the grid.

**Your first act is to repair the block, install the repair in the store, and only then encode.**
Wave 4 is the precedent: `holidays/cme-2013-2015.r2.json` plus
`holidays/cme-2013-2015.repair.json`, with round 1 kept byte-identical.

### Shape warning that will cost you a day if you miss it

Unlike waves 3 and 4, this block's `families[]` entries are **grouped product lines**, not the
crate's family keys: `equity_index+interest_rates+fx`, `energy+metals`,
`grains_oilseeds+livestock+dairy+lumber`, `livestock+dairy+lumber`, `interest_rates+fx`, and
one nine-group row — CME states many holidays for whole product sections at once. Your
derivation must expand each group row onto the crate families it covers (with `dairy` and
`lumber` folding into `globex_grains` and `globex_livestock` for reporting only, as waves 3 and
4 did), and the block's own `coverage` paragraph names the mapping and the reconciliation. Read
it in full, and read the `Schema note` about the optional `other_statements` array (67 entries,
18 `superseded`, the rest `corroborating`) — those are the lineage records
LAW-PRIMARY-SOURCES requires and several of them are load-bearing for the repair.

## 3. The repair list, item by item

From `holidays/cme-2013-2015.verify.json` `discrepancies[]` — **read every entry in full
before you start**; what follows is the map, not a substitute.

**Load-bearing (block the rows):**

1. **2013-07-03 `livestock+dairy+lumber` records the wrong final status.** CME revised the
   schedule after the capture the block cites: the later revision is the
   `2013-4th-of-july-done.pdf` now in `-verify-r2/new/`, and it moves those groups from
   "Regular Close – Per each product schedule" into the explicit early closes listed in §2.
   Re-derive the row from that document, record the supersession with its lineage, and cite it.
2. **2013-07-03 `grains_oilseeds` quotes a superseded line as current.** The same revision
   prints `1215 CT – Early MGEX Wheat & Apple Juice close`; the recorded 1200/1230 CT values
   are unchanged between revisions, but the quoted line must be the revision's.
3. **The completeness claim is not met.** `missing[10]` and the fix `INDEX.md` say the
   enumeration accounted for every supersession; it covered only the `.xls`/`.zip` half. The
   status-200 PDF capture `20130717050333 …/2013-4th-of-july-done.pdf` sits in *both* rounds'
   saved CDX files and in neither download list. **Retrieve the 42 earlier distinct-digest
   captures of the cited PDF URLs** (compute the diff yourself from
   `raw/cme-2013-2015/cdx_holiday_calendar.json`, `raw/cme-2013-2015-fix/cdx_all.json` and
   `raw/cme-2013-2015-verify-r2/cdx/*.json` against the downloaded sets `names.txt` and
   `todo.tsv`), settle the two that carry different session values by lineage, and correct the
   claim to what you actually did.
4. **A recorded gap that the corpus already closes.** `missing[2]` says the Good Friday 2013
   livestock/dairy/lumber 13:55 CT zone is unknown. It is not: `X13GFPD`
   (`2013-good-friday-presidents-day.xls`), already cited in the file, settles it from 2013
   itself — sheet `Good Fri.`, header `Early Thur. Close`, rows `Dairy`/`Livestock`/`Lumber` =
   13:55, and the sheet's own Notes row gives the zone. Close the gap, cite it, do not infer.
5. **A stated evidential claim has a counterexample.** The fix `INDEX.md` says the
   Interest-Rate/FX row pairs are identical in every column across all 45 files; 57 of 58 are,
   and `X13GFPD` sheet `Good Fri.` r012/r014 is the exception. Correct the claim and the
   coverage clause, and keep the grouping (it survives on the other 57 plus the separate
   printed row labels).

**Moderate / minor (record hygiene, but a reviewer will check every one):**

6. Name the crate keys the documents *do* give distinct instants for and the block does not
   model (e.g. `X15ANN` sheet `Labor` r035 `Weather` 15:15, r020 `Mini-Sized Grains and
   Oilseeds` 13:45) as gaps with their closing condition, beside the two families that are
   silent outright.
7. The "28 newly cited documents" self-description counts wrong: only one 2013 multi-sheet
   workbook is cited.
8. The `2014-11-11` `other_statements[0].note` says the sheet is identical to the annual
   master's; the statement is the same, the sheet is not. Reword.
9. `2013-10-14` and `2013-11-11` drop the registered-trademark glyph from the quoted document
   titles (`Globex(R)`).
10. Observation, not a defect: 29 `*_instant` fields zero-pad three-digit printed clock times
    (`905 CT` → `0905 CT`). Wave 4's precedent (`verbatim` literal, instants as printed) is the
    guide; if you re-emit them, do it from the bytes.
11. The round-0 `INDEX.md` still carries the withdrawn `.xls`-twins sentence and the withdrawn
    categorical 2015 Labor Day claim, with no amendment pointer to the fix directory. Add the
    pointer.
12. The corroborating-statement rule is applied unevenly: `X13GFPD`'s `New Years` sheet covers
    2013-01-01/02 in full, and none of those ten rows carries the corroboration its siblings do.

Everything the verifier *confirmed* is listed in `confirmed_sound[]`; do not "fix" those, and
do not re-retrieve what is already saved except where an item above names a defect.

## 4. What the crate must ship

Copy wave 4's shape file for file (PR #106 is the reference):

1. **Rows** — one per date-and-status that moves an answer, keyed to the crate's venue-local
   **trade date**, never the operator's event date. `normal` ships nothing; a printed instant
   equal to the family's ordinary instant for the era ships nothing; `closed` → `Closed`;
   `early_close` → `EarlyClose`; a printed reopen later than the ordinary first open →
   `LateOpen` on the trade date it belongs to (on the day after a closure, when the operator's
   sheet shows the prior-evening leg did not run); both → `LateOpenAndEarlyClose`; `modified`
   decided from the printed instants. The 2013–2015 ordinary grids are the **current profiles'
   ancestors**: check `src/calendar/schedules/futures/us/*.rs` for what was in force in
   2013–2015 (the CME group had the 15:30-16:15 CT post-halt slice before 2015-09-20 and
   15:30-16:00 after; grains have their own day-session close; livestock and lumber are flat
   grids) — do not carry wave 4's grids across a revision boundary without reading the module.
2. **Venue tables** — `{cme,cbot,comex,nymex}` extended by D17 over 2013–2015 by the copied
   `venue_intersection_wave5.py` (`--check`, `--emit-evidence`, `--insert`). **The insertion
   helper in wave 4's tool had a bug that shipped: it inserted the block between a row and its
   own citation comment.** Copy the fixed version (it walks back over the row's `//` comment
   block) and confirm every venue row still has its citation line directly above it.
3. **Evidence files** — `docs/evidence/<family>.md` gains `### 2013`, `### 2014`, `### 2015`
   under `## Holidays`; a `**Coverage:**` line that is exactly the module's window list, in
   order, comma-separated (a fence compares them); an era paragraph naming how many windows the
   family declares and which intervals stay unaudited (recompute it); a `### Documents` table in
   the fixed six-column shape; and a `### Gaps and residual risks, 2013-2015` block. Every
   instant cell names the sheet's printed product line ("The cited sheet's `FX` line prints …"),
   and no cell ever contains a nested backtick or a literal `|` — wave 4's review caught both.
   Keep one id per artifact: the era's table owns its ids and the file's master
   `## Evidence documents` table carries only ids no era table carries (wave 4's review caught
   a deduplication that emptied four era tables).
4. **Tests** — per family, wave 4's four era tests plus the handwritten
   `ERA_2013_2015_ROWS` `(date, kind, tier)` table compared in order (the count sweep alone
   cannot see a row moved to another audited date with the same kind and instant — wave 4's
   review proved that with a mutation), and the venue fences in
   `tests/venue_sessions/holidays_cme_venues.rs`. Mutation-check one row per family.
5. **Records** — `docs/schedules/verification.md` (the six families' and four venues'
   `Holidays` cells gain `2013-01-01..2015-12-31` in date order; `Reviewed on` and the monthly
   cadence still satisfy LAW-WATCH), `CHANGELOG.md` under `[Unreleased] / Added`,
   the plan note, `tools/README.md`, and `../exchange-hours-research/STATUS.md`.
6. **Module docs** — each touched module's `//!` header must name every window it declares:
   `every_holiday_module_header_names_its_declared_windows` (added in wave 4) parses the
   `coverage:` clause and fails if a window's years are missing from the header. Wave 4's
   blocking review finding was exactly this.
7. **`#95` — the D17 audit closes with this wave.** Its ledger/evidence text says the venue
   intersection is audited wave by wave and closes when the last family wave lands; run the
   cross-wave audit over 2010–2027, record it in the four venue evidence files, and close #95
   with this PR (or say precisely what remains).

## 5. Method that worked (wave 4, and wave 3 before it)

1. **Repair the evidence first, install it, then encode.** A row may not ship from a block a
   verifier failed. Write `r2.json` + `repair.json`, keep round 1 byte-identical, and prefer the
   verifier's own `fix` wording where it gives one. Re-derive from the saved bytes, never by
   extending a string from context.
2. **Delegate bulk conversions to subagents; keep the decisions yourself** — what ships, what is
   a gap, the trade-date conversions, the fold, the tier assignment, the final gate. Give each
   subagent a self-contained brief naming the inputs, the outputs, the rules and the questions
   it must ask rather than guess.
3. **Cross-check every derived artifact with an independent implementation.** Wave 4's
   `check_wave4.py` (13,528 re-derivations: rows, coverage, bytes, D17 venue intersection) was
   written by an agent that never read the generator, and it found real disagreements. Copy its
   structure; require it to pass before you push.
4. **Mutate a shipped row** (a kind, an instant, a date) and confirm a test fails. Wave 4's
   review showed the date case needed its own table; #108's showed that a *sweep* needs its
   population pinned independently (see §9).
5. **Verify before pushing**, as one script, in this order:
   `cargo fmt --all --check && cargo clippy --all-targets -- -D warnings && cargo nextest run --all-targets && cargo test --doc && RUSTDOCFLAGS="-D warnings" cargo doc --no-deps && cargo deny check`,
   then `cargo +1.95 check --all-targets`. Two environment notes: `cargo deny check` cannot
   write its advisory lock under the sandbox's read-only `~/.cargo` — copy that database to a
   writable `CARGO_HOME`; and the suite is dominated by
   `the_coverage_gate_is_sound_for_every_shipped_row` at **21–27 minutes** (1,245 s alone,
   1,350 s alongside the rest; it grows with every wave), with the premise fence about **80 s**
   on top. Budget for them rather than killing them; CI's `quality` job takes ~26 min.

## 6. Branch, PR and review

- Branch from `origin/main` and work in a `git worktree` inside the repo
  (`git worktree add .worktrees/wave5 origin/main`) — writes outside the workspace root may be
  denied, so a sibling path will not work.
- One PR, titled after the wave, whose body **closes #88 and #95**, summarizes the repair,
  states the row counts per family and shape, the venue counts, the gaps and the verification
  results — and **opens with `LAW-AGENT-ATTRIBUTION`**: *"Posted by an AI agent running as
  `<model>` (`<harness>`), using the maintainer's GitHub login."* Every comment, reply and
  commit trailer you write carries the same attribution. The maintainer's reviewer agent does
  this too; you will see it in the threads.
- **Expect two to four rounds.** Wave 4 drew seventeen items (fifteen inline threads plus two
  review-body comments, two of them blocking); #108 drew six across two rounds, from CodeRabbit
  *and* from the maintainer's reviewer agent, whose report is a top-level PR comment. Reproduce
  every finding against the modules before changing anything. **Both reviewers recompute every
  number you write** — the #108 review re-derived the whole proof from the source and re-ran the
  measurement script, and found three wrong counts in prose while clearing all seven committed
  files. When a suggested fix is wrong, answer with the measurement or the code that shows it;
  when it is right, say so and fix it. Reply to every thread, resolve every thread, and do not
  merge with one open.
- Do not mark the stage done until CI is green on the pushed head and the PR is `MERGEABLE`
  with zero unresolved threads. Then record the outcome in the research store's `STATUS.md`
  (head hash, verdict, defects, the reviewer's findings and what you did with each) and leave
  the merge to the maintainer.

## 7. Boundaries

- One PR sized to a working day (LAW-BOUNDED-WORK). If wave 5 does not fit, split it by year —
  but then each family's coverage window must declare only what actually shipped.
- Do not touch another wave's block, `docs/plans/` history below your own note, or the wave-4
  and #97 branches. `holidays/cme-2019-2021.*` is frozen once #106 has merged, and
  `holidays/cme-2013-2015.*` becomes frozen once your PR merges.
- A follow-up you name anywhere (PR body, commit message, review reply, evidence file) is
  either done in this change or opened as an issue **in the same change**, with the number cited
  where the follow-up is named (LAW-FOLLOW-UPS-ARE-ISSUES).
- If a discovered defect is outside your files, report it, open an issue, and leave the code
  alone rather than widening the PR.

## 8. What comes after wave 5

- **2.3 — served non-CME venues back to the floor (new retrieval)**: one PR per venue, smallest
  history first — `coinbase_derivatives`, then `cfe` (rides `cfe_vix`), then `iceus` (rides
  `ice_us` and the six `ice_us_*` keys), then `eurex` (rides `eurex_fixed_income`) — each
  bringing its evidence files into the fixed `### Documents` shape, which is **#98**.
- **2.4 — refresh the published future** within the month of the release (CME 2028 once the
  service carries it, Coinbase Derivatives' late-2026 notices, ICE/Cboe next calendars, Eurex
  2027 once final).
- **Stage 3** — served-key hygiene. **#97 landed 2026-09-17 (UTC) as PR #108**: the coverage
  gate reads `[D, D + 1]` for every self-dated occurrence, a premise fence
  (`every_shipped_session_occurrence_is_dated_by_its_own_open_or_the_next_day`, ~80 s) holds the
  claim over all 128 close-dated identities, and the year scan went from 12.8× to 2.8× the
  table-less control (`holiday-tables/BENCH-wave1.md` §8; note §8.5's corrected share,
  2,074/35,040 = 5.9 % expensive, 94.1 % exit). Still open there: **#79** (the undated 2012
  Sunday Pre-Open move), **#77** (seasonal-key doc sentences), **#86** (cutovers outside
  `revisions!`), **#94** and **#93** (additive API, may follow the release).
- **Stage 4** — dormant holidays to the floor; **stage 5** — consumer-side tasks in
  SharurPlatform; **stage 6** — the release (`RELEASING.md`, followed exactly).
- Open follow-ups to keep sight of: **#98** (fixed `### Documents` shape), **#107** (the gate's
  per-instant cost on a row's own date, and the per-occurrence window), **#105** (a consumer
  that maps `dairy`/`lumber`), **#101** (`globex_interest_rates`' 2010–2012 holiday-Monday
  grid), **#89** (three wave-4 MGEX Apple Juice verbatims quote a row CME deleted), **#85**
  (narrative debt), and the CME gaps named in §1.

## 9. Traps that cost waves 4 and 5 real time

- **A bounded search's `None` is not "never".** `next_session_open_after` looks fourteen
  venue-local days ahead, and the #108 premise fence's loop `break`ed on its `None`: a
  launch-dated identity's pre-launch span ended the sweep, so it silently covered 83 of its 128
  identities. Step a fixed amount forward and keep asking.
- **Pin a sweep's population independently of the thing it sweeps.** `swept == derived.len()`
  reads the same source twice and holds even when an identity disappears from both; #108's
  final guard is a literal count (the ledger's 132 rows less the four excluded conventions)
  plus an occurrence floor. Do the same for any era sweep you write.
- **A single-line scan misses rows written across several lines.** #108's bench memo listed
  nine of the module's twelve 2026 rows because three were formatted one field per line, and a
  reviewer recomputed it. Parse the module or the block with a real reader; never grep a
  regex over lines for a count you are going to publish.
- **Every number you write in prose is data.** Recompute it from the module or the block, and
  prefer a number a reader can re-derive.
- **The research store is outside the workspace.** Writes there may be denied; if a write is
  denied, retry that exact command with `danger_full_access` and a one-sentence justification,
  and never leave a repair "staged" without installing it.
- **One artifact, one document id.** The fences `every_artifact_carries_one_document_id` and
  `every_cited_document_id_is_resolved_exactly_once` both bite: a combined CME sheet serves
  several families under one id, and the same id may appear in exactly one table per evidence
  file.
- **A blank line between a table's header and its `|---|` separator makes the evidence parser
  read zero rows and skip the whole file**, which silently disables the id fences. Wave 4 found
  201 duplicated id rows hiding behind exactly that.
- **Nested backticks and literal `|` inside an evidence cell** break the table: wave 4's review
  caught both.
- **The `holidays!` macro fails the build** on a non-ascending row, a row outside every window,
  an empty document id, or a tier below T1 — read the error, it names the rule.
- **If you touch production code**, measure it: #108's `#[inline(never)]` on an order-entry
  scan was worth 20–57 % on the multi-candidate queries, and the only way that was found was an
  A/B on the benchmark with one attribute removed. An unmeasured "obvious" change to a hot path
  is a guess.
