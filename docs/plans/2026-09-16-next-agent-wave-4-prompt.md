# Next stage: stage 2.2 wave 4 — the CME 2019–2021 holiday era (issue #91)

You are an autonomous coding agent working in `exchange-hours-rs` (a Rust crate that ships
sourced exchange session calendars). This is a handover: assume you have no memory of the
session that produced it. Read everything below before touching anything.

## 0. Read these first, in this order

1. `AGENTS.md` — the charter, and the law. Every rule below is a citation of it. Read it in
   full; it is short.
2. `docs/plans/2026-09-12-path-to-release.md` §3 stage 2.2 — this task is **wave 4** of the
   five CME waves in that table. Waves 1 (2010–2012), 2 (2016–2018) and 3 (2022–2024) have
   landed; wave 5 (2013–2015, #88) is blocked on a retrieval and is not yours.
3. `../exchange-hours-research/STATUS.md`, last ~80 lines — the handover log, including a
   section on wave 3 that describes the exact workflow and the traps.
4. The three landed wave PRs, for the shape to copy: **#102** (wave 1), **#103** (wave 2),
   **#104** (wave 3, the closest model — it also extended the venue tables and carried the
   `dairy`/`lumber` fold). Read `gh pr view 104 --json body` and, more usefully,
   `git show --stat 0a2de80` and the diffs of the files it touched.

## 1. The task

**Wave 4: CME Group's holiday schedules for calendar years 2019, 2020 and 2021.**

Encode the eight served CME families' holiday rows for **venue-local trade dates
2019-01-01 .. 2021-12-31** as a new audited era in each family's static holiday table, at
the tier each date's document warrants, extend the four CME venue tables (`cme`, `cbot`,
`comex`, `nymex`) over the same years by the D17 intersection rule, write the evidence, add
the tests, update the records, and open one PR that **closes #91**.

The eight families: `globex_equity_index`, `globex_energy`, `globex_fx`, `globex_grains`,
`globex_interest_rates`, `globex_livestock`, `globex_cryptocurrency`,
`globex_nikkei_225_dollar`. Read `src/calendar/schedules/holidays/mod.rs` for the
`holidays!` macro and `HolidayKind`, and one family module in full to see what a landed era
looks like.

## 2. The evidence you already have (do not re-retrieve unless a named defect requires it)

The research store (`../exchange-hours-research`, beside the repo) holds the wave-4 block and
its verification:

- `holidays/cme-2019-2021.json` — the round-1 block: **44 holiday dates × 10 product groups**
  (`dairy_same_page` and `lumber_same_page` are product groups with no crate key; fold them
  into `globex_grains` and `globex_livestock` exactly as wave 3 did), **160 `closed`, 176
  `early_close`, 93 `normal`, 6 `late_open`, 5 `modified`**.
- `holidays/cme-2019-2021.r0.json` — round 0, kept verbatim. Do not edit it.
- `holidays/cme-2019-2021.verify.json` — the **round-2 verdict: FAIL, evidence discipline
  only. "No date, status, tier or instant VALUE is wrong anywhere in the 440 family rows,
  and none is inferred from another year."** For this wave, unlike waves 1–3, there are **no
  state defects to repair** — four evidence-discipline defects remain (N1–N4 below), each
  with a verbatim `fix` field in the verdict's `discrepancies` array. **Read that array in
  full; it is your repair list, and its `fix` text is the wording the verifier will accept.**
- Raw artifacts: `holidays/raw/cme-2019-2021/` (+`-fix/`), with `INDEX.md`, `shasum.txt` and
  `cdx/` crawls. All were retrieved on 2026-09-12 (UTC).

### The four defects, in the verifier's own terms

- **N1 (moderate, introduced by the fix round).** A false enumeration claim: three files
  assert that the archive holds no standalone 2020 Good Friday workbook, but it does —
  `2020-good-friday-schedule.xls`, capture `20241202115016` (2024-12-02T11:50:16Z), sha256
  `4b5f23b22f4dcfc3f933d6a3b3c1e3193172045165ebf24618e78a25dd334aed`, byte-identical to the
  ZIP member. The fix-round crawl was windowed `from=2019 to=2023` and could not see a 2024
  capture. **This is a served-identity false claim, so fix it in all three named files and
  re-run the prefix crawl with `from=2018 to=2027` before asserting any absence anywhere.**
- **N2 (minor).** Instant fields paraphrase CME's printed spacing outside the eleven fields
  the fix round rewrote — e.g. `9:00 CT` where the sheet prints `9:00CT`, and a `2021-07-05`
  grains open that drops the printed word `Regular`. The verdict lists every confirmed case
  with the exact correction.
- **N3 (minor).** `shasum.txt`'s preamble claims coverage it does not have: 67 files are
  hashed nowhere. Either hash all 170 or rewrite the preamble to the verifier's text.
- **N4 (minor overall, load-bearing in two fields).** **314 `verbatim`/instant fields are
  hard-truncated mid-token** at a fixed character budget, contrary to the block's own
  `NOTATION` paragraph. Two named fields lose part of a printed instant
  (`2019-01-01 globex_grains open_instant`, `2020-12-25 globex_livestock open_instant`).
  **This is #91's headline item and the reason the issue exists**: an evidence file generated
  from these fields would quote the operator saying something the operator did not say.

### Your repair before encoding

Produce **`holidays/cme-2019-2021.r2.json`** — the round-2 repaired block — plus
**`holidays/cme-2019-2021.repair.json`** recording, per defect, the re-derivation, the
artifact and bytes, the command used and the corrected text. Keep round 1 byte-identical;
waves 1–3 all did this and the reviewer expects it. Re-emit the 314 truncated fields at full
length **from the saved bytes** (`raw/cme-2019-2021/…`), not by extending the truncated
strings from context. Where a field is genuinely a paraphrase problem rather than a
truncation, prefer the verifier's `fix` wording. Amend the `NOTATION` paragraph so it
describes what the file actually does afterwards.

**Write permission:** the research store is outside the workspace and writes to it are
usually denied by the sandbox. Attempt the write, and on denial retry the exact command with
`danger_full_access` and a one-sentence justification — that is the sanctioned path, and it
worked for the previous stage. Never leave the repair "staged" without installing it.

## 3. What the crate must ship

### 3.1 Rows

Derive the rows from the repaired block, one per date-and-status that **moves an answer**:
`normal` ships nothing; a printed instant equal to the family's ordinary instant for the era
ships nothing; `closed` → `HolidayKind::Closed`; `early_close` → `EarlyClose`; `late_open` →
`LateOpen`; a `closed` date whose printed reopen is the next trade date's day session → the
era's late-open shape. Rows are keyed to the crate's **venue-local trade date**, never the
operator's event date (LAW-HOLIDAY-SCOPE, design memo D1).

Three things the verdict and the block's own `missing` register already settle:

- **Juneteenth 2019, 2020 and 2021 ship `Unsourced`, in every family.** No CME document
  exists for those three dates; the channels tried are in the register. Inside a contiguous
  window, silence is the positive claim that a date was audited normal, which is false for a
  date the operator marked as a new holiday in 2021 and did not for 2019–2020 — read the
  register entry in full and state the reasoning per date in the evidence file. Cite whatever
  the register says was checked (a negative-control capture, or the enumeration) the way wave
  3 cited its three 2023 markers.
- **Columbus Day and Veterans Day 2019–2021** are named gaps in the register (CME published
  only settlement and clearing advisories, never a Globex trading schedule). Record them as
  gaps; a settlement notice is not session language (LAW-SESSION-NOT-EXPIRY).
- **`globex_nikkei_225_dollar`** has no outright Nikkei row on any 2019–2021 sheet; the block
  records the family as following the quoted `Equity`/`Equity Products` line. That is an
  interpretive step the evidence file must state and the verdict has accepted — do not
  silently key rows to the equity line without writing it down.

**The era's paper trail is T1** (CME's own holiday-calendar workbooks and compact sheets) for
everything except any date you mark `Unsourced` from a checked absence, which cites the
artifact that proves the absence (wave 3's three 2023 markers cite `CME-SVC-*` windows; here
the register names the CDX enumerations). Some 2019–2021 sheets are annual-bundle members
rather than standalone captures — cite the member path the way waves 2 and 3 cite
`2016-holiday-calendars.zip#…`, and resolve every id through the raw `INDEX.md`.

### 3.2 Venue tables

Extend `src/calendar/schedules/holidays/venues/{cme,cbot,comex,nymex}.rs` over 2019–2021 by
the D17 rule their module doc and `venues.rs` state: a date every routed family states the
same row ships that row; a date on which the covering families differ ships
`HolidayKind::Unsourced` and the disagreement is named per date in the venue's evidence file;
a date inside the era's window that no routed family states anything for is audited normal
and ships nothing; a family whose table does not cover the era **abstains** rather than
disputing. Routing: `cme` = equity index, energy, FX, grains, interest rates, livestock;
`cbot` = grains + interest rates; `comex`/`nymex` = energy alone. Nikkei and cryptocurrency
are **not** routed. `comex`/`nymex` carry `globex_energy`'s rows unchanged.

The venue fences in `tests/venue_sessions/holidays_cme_venues.rs` recompute the intersection
from the families' public answers, so they stay green if and only if the derivation is right.
Wave 3 left a reusable tool, `tools/venue_intersection.py` (`--check`, `--emit`,
`--emit-evidence`); copy it to a wave-4 name and re-point it at this era rather than writing
a new one, and run `--check` before you believe your table.

### 3.3 The rest of the change set, per wave 3

Copy wave 3's shape file for file:

- `docs/evidence/<family>.md` — add `### 2019`, `### 2020`, `### 2021` under `## Holidays`,
  each row carrying the operator's own printed token and its document id; a `**Coverage:**`
  line that is **exactly** the module's window list, in order, comma-separated (a fence
  compares them); an era paragraph that says how many windows the family now declares and
  which intervals remain unaudited; a `### Documents` table in the fixed
  `| Document | Window | Replay or service URL | Capture or retrieval, UTC | Tier | sha256 |`
  shape for the ids this era cites; and a `### Gaps and residual risks, 2019-2021` prose
  block. Prose **is** data here: every count, date and instant must be one you recomputed.
- The four venue evidence files (`cme.md`, `cbot.md`, `comex.md`, `nymex.md`), same shape.
- `tests/futures_family_boundaries/holidays_globex_*.rs` — an era-wide sweep per family that
  walks every shipped row of the new era and asserts both sides of every instant, the
  closures' wrap removal, the `Unsourced` rows changing no answer (equal to
  `without_holidays()`), and the window's edges. Wave 3 added four such tests per family;
  match that.
- `docs/schedules/verification.md` — each affected row's `Holidays` cell gains
  `2019-01-01..2021-12-31` in date order (a fence derives the cell from the module's own
  `holiday_coverage()`, so a wrong cell fails the build). Check the `Reviewed on` and cadence
  cells still satisfy LAW-WATCH.
- `CHANGELOG.md` under `[Unreleased] / Added`, `docs/plans/2026-09-12-path-to-release.md` (mark
  wave 4 landed and record its note), and `../exchange-hours-research/STATUS.md`.
- `tools/` — a wave-4 generator/checker pair, following `tools/README.md`'s pattern
  (including its `WAVE3_RESEARCH`-style required variable, named for this wave). If you carry
  forward wave 3's scripts, rename the variable and update the README; do not leave a README
  that documents a variable the tools no longer read.

## 4. Method that worked for wave 3 (use it)

1. **Repair the evidence first, install it, then encode.** The crate's only product is the
   claim that its values are sourced, so a row may not ship from a block a verifier failed.
2. **Delegate the bulk conversions to subagents, keep the decisions yourself.** Give each
   subagent a self-contained brief naming the exact inputs, the exact output files, the rules
   it must implement, and the questions it must *ask* rather than guess. Reserve your own
   context for: what ships and what is a gap, the trade-date conversions, the fold, tier
   assignment, and the final gate.
3. **Cross-check every derived artifact with an independent implementation.** Wave 3's
   checker re-derived all 237 rows from the block and recomputed all four venue tables from
   the family modules; both found real disagreements with the generator that were then
   settled from the bytes. Do the same here, and write down which tool decided each disputed
   value.
4. **Mutate a shipped row** (flip a kind, an instant, a date) and confirm a test fails. If
   none does, the row is unfenced — say so and fix the fence before you push.
5. **Verify before pushing**, as one script, in this order:
   `cargo fmt --all --check && cargo clippy --all-targets -- -D warnings && cargo nextest run --all-targets && cargo test --doc && RUSTDOCFLAGS="-D warnings" cargo doc --no-deps && cargo deny check`,
   then `cargo +1.95 check --all-targets`. Two environment notes: `cargo deny check` cannot
   write its advisory lock under the sandbox's read-only `~/.cargo` — copy that database to a
   writable `CARGO_HOME` and run it there; and
   `the_coverage_gate_is_sound_for_every_shipped_row` takes **~21 minutes** once the tables
   cover four eras, so budget for it rather than killing it.

## 5. Branch, PR and review

- Branch from the current `origin/main` (which is `0a2de80`, the wave-3 merge) and work in a
  `git worktree` inside the repo (`.worktrees/wave4`) — **writes outside the workspace root
  are denied**, so a sibling path will not work.
- One PR, titled after the wave, whose body **closes #91**, summarizes the repair, states the
  row counts per family and shape, the venue counts, the gaps, and the verification results.
- **Expect review.** CodeRabbit plus a human reviewer left nineteen findings on wave 3 and
  every one was valid. Reproduce each against the modules before changing anything: check the
  arithmetic in any count it disputes, read the code it names, and when a suggested fix is
  wrong, say so with the measurement that shows it (one wave-3 suggestion broke a real test
  and had to be answered that way). Fix, push, reply to every thread, resolve every thread,
  and do not merge with a thread open.
- Do not mark the stage done until CI is green on the pushed head and the PR is `MERGEABLE`
  with zero unresolved threads. Then record the outcome in the research store's `STATUS.md`.

## 6. Boundaries

- One PR sized to a working day (LAW-BOUNDED-WORK). If the wave does not fit, split it by
  year — but the era's coverage window then declares only what shipped.
- Do not touch wave 5's documents, `docs/plans/` history below your own note, or another
  wave's block.
- A follow-up you name anywhere (PR body, commit message, review reply, evidence file) is
  either done in this change or opened as an issue **in the same change**, with the number
  cited where the follow-up is named (LAW-FOLLOW-UPS-ARE-ISSUES). Wave 3 opened #105 this way.
- If a discovered defect is outside your files, report it, open an issue, and leave the code
  alone rather than widening the PR.
