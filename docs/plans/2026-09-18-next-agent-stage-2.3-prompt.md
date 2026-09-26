<!-- SPDX-License-Identifier: MIT-0 -->
<!-- Working note for the next agent; untracked, like the wave-4 and wave-5 prompts before it. -->
<!-- Written 2026-09-18 UTC against main @ 453410e (PR #109 merged). -->

# Next stage: stage 2.3 — the four served non-CME venues back to the floor

You are an autonomous coding agent working in `exchange-hours-rs` (a Rust crate that ships
sourced exchange session calendars). This is a handover: assume you have no memory of the
session that produced it. Read everything below before touching anything.

Stage 2.3 is **four pull requests, not one** — one per venue, smallest history first. This brief
covers the whole stage; §4 is the executable instruction set for the first PR, and §5 is the
shape every later one repeats. Do not try to land all four in one change (LAW-BOUNDED-WORK).

## State at hand-off (2026-09-18 UTC)

- `origin/main` = **`453410e`**, the squash commit of **PR #109**
  (`CME holiday tables: the 2013-2015 wave closes the CME family obligation (#88, #95)`),
  merged by the maintainer at 2026-09-18T13:08:54Z. CI is green on `main`: the `push` run
  re-ran the whole chain on the merge commit and took 30m34s, with `quality` and `msrv (1.95)`
  both passing. Wave 5 closed **#88** and **#95**; **#110** (`globex_livestock`'s 2016-2018 era)
  stays open as its recorded follow-up.
- **The main checkout is on `main` and clean** except for three untracked files: the wave-4
  prompt, the wave-5 prompt, and this one. `git fetch origin` anyway, and do the work in a fresh
  worktree (§8). This file is untracked: read it from the main checkout, do not add it to a PR.
- **Stage 2.2 is done apart from #110.** Stage 2.3 is the next thing in the plan.
- **The four venues ship 2026-only holiday tables, and nothing below 2026 was ever retrieved for
  any of them.** Confirm each in one command:

  ```bash
  sed -n '/coverage:/,+1p' src/calendar/schedules/holidays/coinbase_derivatives.rs
  sed -n '/coverage:/,+1p' src/calendar/schedules/holidays/cfe.rs
  sed -n '/coverage:/,+1p' src/calendar/schedules/holidays/eurex.rs
  sed -n '/coverage:/,+1p' src/calendar/schedules/holidays/ice_us.rs
  grep -m1 '^| `coinbase_derivatives`' docs/schedules/verification.md
  ```

  As of this hand-off, exactly:

  | Venue module | Static(s) | Rows | Coverage window today |
  |---|---|---|---|
  | `cfe.rs` | `TABLE` | 12 | `2026-01-01..2026-12-31` |
  | `coinbase_derivatives.rs` | `TABLE` | 8 | `2026-01-01..2026-09-07` |
  | `eurex.rs` | `TABLE` | 7 | `2026-01-01..2026-12-31` |
  | `ice_us.rs` | six statics (`SUGAR_COFFEE_COCOA`, `ORANGE_JUICE`, `COTTON`, `FANG`, `DOLLAR_INDEX`, `VENUE`) | 21 / 20 / 21 / 22 / 21 / 24 | all `2026-01-01..2028-01-03` |

  The ledger's `Holidays` cells mirror those windows today. **Every one of them is a claim about
  the only years anyone audited**, so none of them is wrong — they are simply short.

## 0. Read these first, in this order

1. `AGENTS.md` — the charter and the law, in full; it is short. **It carries
   `LAW-AGENT-ATTRIBUTION`**: every GitHub artifact an agent authors (a pull-request or issue
   title and body, a comment, a review, a review reply, an inline comment, a release note, any
   other post) **opens by naming the exact model that wrote it**, because the agent writes
   through the maintainer's login and speaks as the maintainer. Commit messages carry a
   `Model:` trailer. This is not optional.
2. `docs/plans/2026-09-12-path-to-release.md` — **§3 stage 2.3 at lines 391-408** is your
   specification, including the order table and the floors; §2.4 and Stage 3 name what follows.
3. `../exchange-hours-research/STATUS.md` — the handover log. Read the **wave-5 section
   (line ~961 onward)**, its two review sections and its merge note: it is the closest model for a
   *retrieval* wave, and it records how the block was repaired, reviewed and merged.
4. **`docs/plans/2026-09-17-next-agent-wave-5-prompt.md`** — the prompt this one mirrors. Its
   §§5-7 (method, branch/PR/review, boundaries) and its §9 (traps) apply here almost unchanged;
   do not re-derive them, read them.
5. The landed wave PRs for the shape to copy: **#102** (wave 1), **#103** (wave 2), **#104**
   (wave 3, which also extended venue tables), **#106** (wave 4) and **#109** (wave 5, the merge
   on your base). `git show --stat 453410e`.
6. `holiday-tables/DESIGN-holiday-tables.md` §3.2 and `holiday-tables/DECISIONS.md` —
   **W1-ASM-7** (line ~1389) and its `#98` follow-up (line ~1434) are the id scheme you are
   extending.
7. The fences, before you write anything they check: `tests/schedule_documentation/evidence_files.rs`,
   `tests/schedule_documentation/mod.rs` (`readme_states_the_holiday_coverage_count_from_the_ledger`,
   line ~691), `tests/holiday_tables.rs`, and the per-venue suites named in §4-§5.

The wave-5 tools survive in `tools/` (`wave5_rows.py`, `encode_wave5.py`, `check_wave5.py`,
`evidence_wave5.py`, `gen_wave5_tests.py`, `venue_intersection_wave5.py`, plus the generic
`encode_rows.py` and `venue_intersection.py`), with `tools/README.md` documenting each handoff and
the `WAVE5_RESEARCH`-style required variable. Copy and re-point them per PR; keep the
documentation in `tools/README.md` honest.

## 1. The task

**Stage 2.3: bring each served non-CME venue's holiday table back from 2026 to its floor, at T1
or T2, in the crate's trade-date key, and bring that venue's evidence files into the fixed
`### Documents` shape — which is issue #98.**

One PR per venue, in this order, smallest history first. Each rides the dormant sibling keys that
read the same operator documents.

| Order | Venue | Rides | Holiday floor | Window today |
|---|---|---|---|---|
| 1 | `coinbase_derivatives` | — | its first trading day, **2021-06-28** | `2026-01-01..2026-09-07` |
| 2 | `cfe` | `cfe_vix` | **2010-01-01** | `2026-01-01..2026-12-31` |
| 3 | `iceus` | `ice_us` + the six `ice_us_*` keys | see the `iceus` note in §5: the FANG+ launch for the venue, `2011-08-01` / `2011-02-07` for the keys | `2026-01-01..2028-01-03` |
| 4 | `eurex` | the `eurex` key + `eurex_fixed_income` | **2010-01-01** | `2026-01-01..2026-12-31` |

**The floors are the crate's own session horizons**, not invented dates: read each identity's
`horizon` cell in the ledger and its module's launch row before you fix a window's first day.
Where the operator's own documents do not reach the floor, the table starts where they do and the
evidence file names the gap (LAW-HOLIDAY-SCOPE). An identity whose operator published nothing
before a year says so rather than shipping a fabricated first row (LAW-NO-FABRICATED-DATES).

**#98 closes when all fourteen evidence files carry a `### Documents` table**, or it closes per
venue if the maintainer prefers; it is one issue spanning all four PRs, so say in each PR body
which of its files you converted and what remains, and cite the issue number
(LAW-FOLLOW-UPS-ARE-ISSUES). The plan notes a doc-only PR may close #98 ahead of stage 2.3; do
not do that as a drive-by, because each PR's rows need the table anyway.

## 2. What is already in the store — and what is not

The research store (`../exchange-hours-research`) holds exactly one non-CME holiday block:

- **`holidays/cfe-eurex-ice-cde-smfe-2026-2027.json`** — 2026 (and 2027 where published) for
  CFE, Eurex, ICE Futures U.S., Coinbase Derivatives and SMFE, with `.r0.json`, `.verify.json`
  and `.verify.r1.json` alongside it. It is **verified `matches: true`, zero discrepancies**.
- **`holidays/raw/cfe-eurex-ice-cde-smfe-2026-2027/`** (round 0) and
  **`holidays/raw/cfe-eurex-ice-cde-smfe-2026-2027-fix/`** (round 1) — the PDFs, HTML, CSV and
  text extractions, each with a URL, UTC retrieval time and sha256 in its `INDEX.md`. These are
  the bytes the 2026 rows rest on.

**Nothing below 2026 exists for any of these venues.** Stage 2.3 is therefore a *retrieval* stage:
there is no block to repair, and no `matches: false` verdict waiting for you. You will build a new
block per venue, have it verified adversarially, repair it if the verifier fails it, and only then
encode (the wave-4/wave-5 method, §7).

**Read `holidays/raw/cfe-eurex-ice-cde-smfe-2026-2027-fix/INDEX.md` in full before you plan
anything.** It is the single most useful document in the store for this stage, and its
`## CORRECTIONS` section supersedes five round-0 claims a careless agent would otherwise inherit.
Two matter immediately:

- **`web.archive.org` IS reachable, and the round-0 claim that it was unreachable is
  withdrawn.** See §3.
- **The CDE notice corpus has essentially no archival backstop.** A CDX query over
  `assets.ctfassets.net/k3n74unfin40*` for CDE notices returned exactly one row. The operator's
  own Market Notices listing is the only channel that has ever produced a CDE notice.

## 3. The retrieval channel: retest it, never inherit a down channel

This is the gating unknown of the whole stage, so settle it in your first hour and write the
result into `STATUS.md` before you plan around it.

**The round-0 index said web.archive.org was unreachable for the whole session and used that to
bound its gaps. That claim is false and was withdrawn.** The fix round re-tested it:
`https://web.archive.org/` returned **HTTP 200 in 1.04 s**, and the fixer enumerated 113 distinct
archived ICE Futures U.S. exchange-notice URLs by CDX query. The observed behaviour is worth
knowing: **the first connection in a burst can fail with `curl: (7) Failed to connect … port 443`
and the immediate retry succeeds.** Retry before concluding a channel is down; never let one
untested failure bound a gap you then record as evidence.

So for each venue, the channel order is:

1. **The operator's own live back-catalogue first** — an annual calendar PDF or a per-year page
   is T1 and is the only thing that can key a row without a capture date argument.
2. **Web-archive captures of the operator's own pages and PDFs** where the live catalogue does
   not reach back. These are T1 when the captured artifact is the operator's own document; the
   capture dates the *observation*, never the state (LAW-NO-FABRICATED-DATES).
3. **A CDX enumeration as negative evidence.** Query the CDX API for the operator's notice and
   calendar URL patterns over the years you need, and **save the JSON**. It is what lets you say
   "the operator published no such notice" instead of "I did not find one" — the distinction
   `holidays/raw/cfe-eurex-ice-cde-smfe-2026-2027-fix/INDEX.md` draws for the 2026 MLK notice.

Per-venue entry points, all already recorded in `docs/schedules/sources.md` (the `US-CFE`,
`US-COINBASE-DERIVATIVES`, `ICE-DERIVATIVES` and `EU-EUREX` sections) — use those, they are the
monitoring list and the archive URLs the crate has already verified:

- **CDE** — `coinbase.com/derivatives/market-notices` (client-rendered; direct `coinbase.com`
  returns 403 from this machine, so fetch through `r.jina.ai` with `-H "x-respond-with: html"`
  and grep the embedded Contentful JSON for `ctfassets` — the hrefs are `/`-escaped). One Notice
  per holiday; **no standing annual calendar page** (`coinbase.com/derivatives/holiday-calendar`
  is a 404, saved as proof). Check how far back the listing pages; if it does not reach 2021,
  that is a declared gap and the window's floor moves to where the listing does reach.
- **CFE** — `cboe.com/about/hours/us-futures` and its CSV export
  (`cboe.com/us/futures/holidays/csv/`) are current-year; historical years come from archived
  captures of that page and from CFE regulatory circulars (`RG-CFE-YYYY-NNN`, `IC-YYYY-NNN`) at
  `cdn.cboe.com/resources/regulation/circulars/`.
- **ICE Futures U.S.** — the annual `Trading Holiday Calendar` PDF at
  `ice.com/publicdocs/futures/IFUS_Trading_Hours_Holiday_Calendar.pdf` and, per year, the
  Exchange-Notice copy at
  `ice.com/publicdocs/futures_us/exchange_notices/ICE_Futures_US_Exchange_Notice_<YEAR>_Holiday_Calendar_<YYYYMMDD>.pdf`,
  plus per-holiday Exchange Notices in the same directory. **The existing CDX query
  (`from=2025&to=2027`) already found holiday notices for 2016, 2017, 2021, 2022, 2023 and 2025
  — but sparsely, several years appearing as a single notice rather than a season. Re-run it with
  `from=2010`; that enumeration is the backbone of your retrieval plan, not a substitute for
  reading the annual calendars.** Note that ICE serves the 2026 calendar at two paths with
  *different bytes*: prefer the `exchange_notices` copy, which is the later-generated one.
- **Eurex** — `eurex.com/ex-en/trade/trading-calendar/holiday-regulations` renders **only the
  current year**, and the `Trading Calendar <YEAR>` PDFs live at
  `eurex.com/resource/blob/<id>/<hash>/data/tradingcalendar_<year>_en.pdf` with a different blob
  id per year. Earlier years come from archived captures of the holiday-regulations page and the
  per-year calendar PDFs. Eurex's 2027-2036 calendars are explicitly preliminary and stay out
  (LAW-NO-FABRICATED-DATES).

**Keep the retrieval honest and bounded.** One block per venue, one artifact per document id,
every artifact's bytes in the store with its `INDEX.md` row (URL, UTC retrieval time, sha256).
`LAW-BOUNDED-WORK` applies: what you cannot source is a gap with what would close it, not an
archaeology project. If a channel refuses twice, record that and move on.

## 4. PR 1 — `coinbase_derivatives` (do this one first)

The smallest history and the thinnest evidence base, so it is the right one to learn the stage's
shape on.

**Target.** `src/calendar/schedules/holidays/coinbase_derivatives.rs`, the single `TABLE` static,
which serves `Exchange::CoinbaseDerivatives` and no key (`routing.rs` line 92). Extend its
`coverage:` from the 2026 window back to the venue's first trading day.

**The floor is 2021-06-28, and you must read it out of the session module, not take it from this
brief.** `src/calendar/schedules/futures/us/coinbase_derivatives.rs` states the launch: FairX
opened Monday 2021-06-28 at 09:00 ET (**13:00 UTC, 08:00 CDT**) with no Sunday-evening session
before it, and carries `FIRST_FULL_DAY = effective_date(2021, 6, 29)`. The venue is closed before
the launch instant, so a holiday row before the first trade date would be meaningless — the
window opens at the first trade date on which a closure is possible.

**What to retrieve.** Every CDE Market Notice for a holiday from that first trade date through
the existing window's end, 2026-09-07: 2021 H2, 2022, 2023, 2024, 2025, and 2026 up to 2026-09-07
(already held — do not re-retrieve what is saved). The 2026 Thanksgiving and Christmas notices
are **stage 2.4's** job by the plan's own words; leave them, and record in the evidence file
whether they had issued as of your retrieval date.

**Rows.** One per date-and-status that moves an answer, keyed to the crate's venue-local
**trade date** in `America/Chicago`. CDE's notices are already keyed by trade date and print the
neighbouring trade dates' open, close and roll instants beside each one, so the conversion is the
identity and the operator's own table corroborates it row for row — that is what makes this venue
tractable. Verify that claim per notice rather than assuming it across five years of changing
template. **The 23x5 tier only**: the 24x7 tier claims no key, Gold and Silver moved into it on
2026-06-15, and the notices' tier labels drift across the window ("Energy & Metal" and "Equity",
then "Energy, Metal & Equity", then "23x5 Products …" beside "24x5 Products …"). A label change
is not a schedule change; what matters is whether the tier the venue profile models is closed.

**The CDE archival constraint.** The operator's own listing is the channel, and a CDX query found
essentially nothing. So check the listing's own depth early: if it reaches 2021, the whole window
is T1; if it starts in, say, 2023, the table starts there and the evidence file names
2021-06-28..<that year> as **unretrieved**, with the closing condition (the operator's listing
gaining those years, or an archive capture of the listing). Do not infer a 2022 closure from a
2023 notice — `holidays/raw/cfe-eurex-ice-cde-smfe-2026-2027-fix/INDEX.md` records exactly that
error being caught for 2026 MLK: *"the 2025 notices must not be read across years."*

**Tests.** `tests/venue_sessions/holidays_coinbase_derivatives.rs` (177 lines) currently ships the
eight-notice fence, a whole-trading-day-removal test, and a coverage-stop fence. Extend it with
one test per audited era plus the handwritten `(date, kind, tier)` era table compared **in
order** — a count sweep alone cannot see a row moved to another audited date with the same kind.
Mutation-check one shipped row (change a date, then a kind) and confirm a test fails.

**Records.** `docs/schedules/verification.md`: the `coinbase_derivatives` row's `Holidays` cell
becomes the module's new window list, and `Reviewed on` becomes your PR's **UTC** date
(LAW-UTC-DATES). Its cadence is already `monthly`, which LAW-WATCH requires for a holiday-bearing
served identity — do not change it. `CHANGELOG.md` under `[Unreleased] / Added` with the years
covered. The module's `//!` header must name the new window: the fence
`every_holiday_module_header_names_its_declared_windows` parses the `coverage:` clause and fails
if a window's years are missing from the header. The evidence file gains the year subsections, the
`**Coverage:**` line, the era prose and the `### Documents` table (§6).

## 5. The shape every one of the four PRs ships

Copy wave 4's and wave 5's shape file for file. Per PR:

**1. The block, in the store.** `holidays/<venue>-<firstyear>-<lastyear>.json` built from the
retrieved bytes, then an adversarial verification round writing `<block>.verify.json`, then a
repair round only if the verifier fails it (write `<block>.r2.json` + `<block>.repair.json` and
keep round 1 byte-identical, exactly as wave 5 did). Read `holidays/DESIGN-holiday-tables.md`
§3.2 for the block schema the wave-5 verifier expects, and reuse the shape of
`cfe-eurex-ice-cde-smfe-2026-2027.json`.

**2. Rows.** One per date-and-status that moves an answer, keyed to the crate's venue-local trade
date. `normal` ships nothing; a printed instant equal to the family's ordinary instant **for that
era** ships nothing; `closed` → `Closed`; an early final close → `EarlyClose`; a reopen later
than the ordinary first open → `LateOpen` on the trade date it belongs to; both →
`LateOpenAndEarlyClose`. **The era's ordinary grid is not the current profile's**: read the
venue's dated `revisions!` timeline before deciding whether a holiday row moves anything. This is
the trap the wave-5 prompt names for CME and it applies verbatim here. CFE's January-2010 floor
is RTH 08:30-15:15 CT with **no** extended leg at all (`CFE_PROFILE_AT_2010_FLOOR`); a
07:20-08:30 extended session arrives 2010-12-10, its start moves to 07:00 on 2011-09-26, and the
two-phase 2013 expansion lands 2013-10-28 and 2013-11-04 — with RTH still 08:30-15:15 throughout.
So both a holiday rule *and* the "ordinary" value it has to be compared against must be read for
the era; a 2026 page gives you neither.

**3. One window per audited era.** `coverage:` lists one inclusive window per era the documents
actually audit. **A year you did not retrieve breaks the window** — do not span it, because
inside a contiguous window a date with no row is the positive claim that it was audited normal
(LAW-HOLIDAY-SCOPE). `HolidayCoverage::contains` answers per date, and the macro fails the build
on overlapping or non-ascending windows.

**4. The evidence files**, for every file the module's `// Evidence:` line declares — and note
that a shared table declares **several**. `cfe.rs` declares `cfe.md` *and* `cfe_vix.md`;
`eurex.rs` declares `eurex.md`, `eurex_key.md` and `eurex_fixed_income.md`. Each must carry the
same `**Coverage:**` line and the same documents table, because
`every_holiday_table_states_its_coverage_window` reads every declared file and
`every_document_id_resolves_to_one_artifact_repository_wide` requires the same id to resolve to
the same `Window` **and** sha in every file it appears in. Copying the table between siblings is
correct and required. Under `## Holidays`: `**Coverage:**` as exactly the module's window list in
order, comma-separated; a `### <year>` subsection per audited year; a `### Documents` table;
and a `### Gaps and residual risks, <era>` block. Recompute every count and interval you write.

**5. Tests.** Per identity, the baseline cases AGENTS.md step 8 lists — the published open and the
instant before it, the phase classification, every lunch or maintenance gap, the end-exclusive
close, the weekend boundary, at least one holiday and one early close, and the serde form — plus
the handwritten `(date, kind, tier)` era table compared in order, and the venue-level fence for
`iceus`'s intersection. Mutation-check one row per identity. Existing suites to extend:
`tests/futures_family_boundaries/holidays_cfe_vix.rs` (258 lines),
`holidays_eurex.rs` (227), `holidays_ice_us.rs` (358), and
`tests/venue_sessions/holidays_coinbase_derivatives.rs` (177).

**6. The ledger.** `docs/schedules/verification.md` — the `Holidays` cell of every identity that
moves, the `Reviewed on` date, and the cadence checked against LAW-WATCH. A shared table moves
every row that points at it: `cfe` + `cfe_vix`; `eurex` + the `eurex` key + `eurex_fixed_income`;
`iceus` + the seven `ice_us*` keys.

**7. The README count must NOT move.** `readme_states_the_holiday_coverage_count_from_the_ledger`
counts ledger rows whose `Holidays` cell is not `—`. All four venues already ship a table, so
`26 of the 132 ledger rows` stays exactly as it is. Do not hand-edit it; if your change makes the
fence fail, you have changed something else.

**8. `tools/`.** Copy the wave-5 scripts for your venue, re-point them, and update
`tools/README.md` with the required variable and the handoff order.

**9. `../exchange-hours-research/STATUS.md`.** A section per PR: what was retrieved and from
where, the channel result, the block's verdict, the review rounds and their findings. This is
where the next agent learns whether the archive channel held.

### `iceus` is the one with real derivation work

The other three venues' tables are their own single table: `Exchange::Cfe` and
`MarketHoursKey::CfeVix` both point at `cfe::TABLE`; `Exchange::Eurex` and the two Eurex keys
both point at `eurex::TABLE`; `Exchange::CoinbaseDerivatives` points at its own `TABLE`, and no
key routes to it. For those, the "venue intersection" of design memo D17 is the identity, and
the evidence file says so in one sentence.

`iceus` is different. `routing.rs` (lines 96, 165-171) routes `Exchange::Iceus` to
`ice_us::VENUE` and **seven** keys to five family tables:

- `IceUs` → `FANG`
- `IceUsSugar`, `IceUsCoffee`, `IceUsCocoa` → `SUGAR_COFFEE_COCOA`
- `IceUsCotton` → `COTTON`
- `IceUsOrangeJuice` → `ORANGE_JUICE`
- `IceUsDollarIndex` → `DOLLAR_INDEX`

So `VENUE` is the **intersection**: a row ships only where every routed family states the same
one, and a date on which they disagree carries `Unsourced` rather than a scheduling row. That is
why the 2026 `VENUE` table has 24 rows of which only four are `Closed` — on every other special
date the softs close while the index families trade shortened hours. Build the six tables, then
**derive** `VENUE` with the copied `venue_intersection` tool (`--check`, `--emit-evidence`,
`--insert`), never by hand, and recompute the intersection independently in your checker.

Each identity's window is its own. The `ice_us` key and `FANG` begin at the FANG+ launch the
20170926 notice states: ICE's launch-eve 19:30 Pre-Open and 20:00 matching start on
**2017-11-07**, which is the evening leg of the venue's first trade date — **read that trade date
out of the session module and the notice; do not take it from this brief.** The softs keys'
session horizon is **2011-08-01** and the USDX key's is **2011-02-07**, because ICE's own master
hours table survives only from AUGUST 2011 (January 2010 to August 2011 is bounded by document
availability, not by an unfinished search — `docs/schedules/sources.md` records that the operator
sets these hours administratively, not by rule, so no filing can close it). A `VENUE` window
that opens at the launch is not a narrowing: the venue profile is closed before it anyway.

## 6. #98 — the fixed `### Documents` shape

`tests/schedule_documentation/evidence_files.rs` already implements everything: `DOCUMENT_TABLE_HEADER`
(line ~689) is the **one shape a documents table may take**:

```
| Document | Window | Replay or service URL | Capture or retrieval, UTC | Tier | sha256 |
```

**Use the code's six-column constant, not the abbreviated five-column header in issue #98's own
body.** The issue text reads `| Document | Window | Capture or retrieval, UTC | Tier | sha256 |`
and is stale; a table written to it fails `a documents row reads <header>`.

Four constraints the fences enforce, all of which bite the moment a file gains a table:

- `every_document_id_resolves_to_one_artifact_repository_wide` — an id resolves to one
  `(Window, sha256)` pair repository-wide, so the same id in all eight ICE files must carry the
  same `Window` cell. The `Window` cell is the **artifact's** coverage, not the row's trade date.
- `every_artifact_carries_one_document_id` — one sha256 carries one id. Two byte-different copies
  of the same calendar (ICE serves the 2026 calendar at two paths) are two artifacts and need two
  ids. **Do not renumber ids already in use for tidiness**: a shipped row cites its id, so changing
  it is a data change that has to be sourced.
- `every_cited_document_id_is_resolved_exactly_once` — currently scoped to files that already
  carry a table (it `continue`s when `rows.is_empty()`). **#98's second ask is to drop that
  scoping caveat** so it covers every family, and to note the change beside W1-ASM-7 in
  `holiday-tables/DECISIONS.md`. Only drop it once every holiday-bearing evidence file has a
  table; until then leave it and say so.
- A blank line between the header row and the `|---|` separator makes the parser read **zero
  rows** and silently skip the file — the table finder ends at the first `\n\n`. Write the header,
  the separator and the rows as one unbroken block.

A left cell may name several ids separated by `, ` when one artifact carries more than one, and
every URL must start with `https://`. No file exercises the multi-id form today, so prefer one id
per row unless an artifact genuinely serves two.

**The fourteen files #98 covers** are exactly the evidence files the CFE, Eurex, ICE and CDE
holiday modules declare, and no others: `coinbase_derivatives.md`; `cfe.md` and `cfe_vix.md`;
`eurex.md`, `eurex_key.md` and `eurex_fixed_income.md`; and the eight ICE files — `iceus.md`,
`ice_us.md`, `ice_us_sugar.md`, `ice_us_coffee.md`, `ice_us_cocoa.md`, `ice_us_cotton.md`,
`ice_us_orange_juice.md`, `ice_us_dollar_index.md`. The eight CME family files and the four CME
venue files (`cme.md`, `cbot.md`, `comex.md`, `nymex.md`) already carry the shape and are not part
of this stage.

## 7. Method that worked (waves 3, 4, 5, and #108)

1. **Retrieve first, and test the channel before you let it bound anything** (§3). Save the bytes,
   write the `INDEX.md` row, and treat a CDX enumeration as evidence in its own right.
2. **Verify the block adversarially before encoding.** A row may not ship from a block a verifier
   failed. Prefer the verifier's own `fix` wording where it gives one, and re-derive from the
   saved bytes — never by extending a string from context.
3. **Delegate bulk conversions to subagents; keep the decisions yourself** — what ships, what is a
   gap, the trade-date conversions, the tier assignment, the final gate. Give each subagent a
   self-contained brief naming the inputs, the outputs, the rules and the questions it must ask
   rather than guess.
4. **Cross-check every derived artifact with an independent implementation.** Wave 4's
   `check_wave4.py` (13,528 re-derivations) and wave 5's `check_wave5.py` (32,105 over checks
   1-12) were written by agents that never read the generator, and they found real disagreements.
   Copy the structure; require it to pass before you push. Any `VENUE` intersection you derive
   gets recomputed from the family tables, not read back from your own constants.
5. **Mutate a shipped row** (a kind, an instant, a date) and confirm a test fails.
6. **Verify before pushing**, as one script, in this order:
   `cargo fmt --all --check && cargo clippy --all-targets -- -D warnings && cargo nextest run --all-targets && cargo test --doc && RUSTDOCFLAGS="-D warnings" cargo doc --no-deps && cargo deny check`,
   then `cargo +1.95 check --all-targets`. Two environment notes from wave 5: `cargo deny check`
   cannot write its advisory lock under the sandbox's read-only `~/.cargo` — copy that database
   to a writable `CARGO_HOME`; and the suite is dominated by
   `the_coverage_gate_is_sound_for_every_shipped_row` at **21-27 minutes**, growing with every
   wave. Budget for it in each of the four PRs rather than killing it; CI's `quality` job took
   32m53s on wave 5's head.

## 8. Branch, PR and review

- Branch from `origin/main` and work in a `git worktree` inside the repo
  (`git worktree add .worktrees/venue-cde origin/main`) — writes outside the workspace root may be
  denied, so a sibling path will not work. Remove the worktree and delete the branch when the PR
  merges, as the wave-5 cleanup did.
- One PR per venue, titled after the venue and era, whose body summarizes the retrieval, states
  the row counts per identity and the shape, the venue/intersection counts where they apply, the
  gaps, and the verification results — and **opens with `LAW-AGENT-ATTRIBUTION`**: *"Posted by an
  AI agent running as `<model>` (`<harness>`), using the maintainer's GitHub login."* Every
  comment, reply and commit trailer carries the same attribution.
- **Expect two to four rounds.** Wave 4 drew seventeen items (fifteen inline threads plus two
  review-body comments); #108 drew six across two rounds; wave 5 needed three review rounds before
  its merge. Reviewers are CodeRabbit *and* the maintainer's reviewer agent, whose report is a
  top-level PR comment. Reproduce every finding against the modules before changing anything.
  **Both reviewers recompute every number you write** — a past review found three wrong counts in
  prose while clearing every committed file. When a suggested fix is wrong, answer with the
  measurement or the code that shows it; when it is right, say so and fix it. Reply to every
  thread, resolve every thread, and do not merge with one open.
- Do not call a PR done until CI is green on the pushed head and it is `MERGEABLE` with zero
  unresolved threads. Record the outcome in the research store's `STATUS.md` and leave the merge
  to the maintainer.

## 9. Boundaries

- **Four PRs, one per venue, in the order above.** If a venue does not fit a working day, split it
  by year range — but then each window declares only what actually shipped
  (LAW-BOUNDED-WORK, LAW-HOLIDAY-SCOPE).
- Do not touch the CME families' blocks or modules, `docs/plans/` history below your own note, or
  any other venue's evidence files. #98 spans exactly the fourteen files listed in §6 — CDE's one,
  CFE's two, Eurex's three, ICE's eight — and nothing else.
- A follow-up you name anywhere (PR body, commit message, review reply, evidence file) is either
  done in that change or opened as an issue **in the same change**, with the number cited where
  the follow-up is named (LAW-FOLLOW-UPS-ARE-ISSUES). For a dormant identity the same obligation
  is met by a gap with a closing condition in its evidence file.
- If a discovered defect is outside your files, report it, open an issue, and leave the code alone
  rather than widening the PR.
- **Do not cut a release.** Release cutting follows `RELEASING.md` and is the maintainer's call;
  the latest tag is `v0.2.2` (2026-08-21) and every schedule change reaches the consumer only
  through a tagged release (LAW-WATCH).

## 10. What comes after 2.3

- **2.4 — refresh the published future**, within the month of the release: CME 2028 once the
  trading-hours service carries it, Coinbase Derivatives' Thanksgiving and Christmas 2026 notices,
  ICE's and Cboe's next calendars, Eurex 2027 once it is final. **This is why PR 1 leaves the CDE
  2026 H2 notices alone.**
- **Stage 3 — served-key hygiene.** `globex_cryptocurrency`'s 2026-09-19 Saturday row is a
  forward-dated `Scheduled` row that must be confirmed or corrected after that day. Open:
  **#79** (the undated 2012 Sunday Pre-Open move), **#77** (seasonal-key doc sentences), **#86**
  (the evidence-day fence needs a second source of dated boundaries — it names `coinbase_derivatives`
  and `eurex`, so it is adjacent to your work and may become cheaper once your tables are literal).
- **Stage 4** — dormant holidays to the floor; **stage 5** — consumer-side tasks in
  SharurPlatform; **stage 6** — the release (`RELEASING.md`, followed exactly).
- Open follow-ups to keep sight of: **#98** (this stage), **#110** (`globex_livestock` 2016-2018),
  **#107** (the gate's per-instant cost), **#105** (a consumer that maps `dairy`/`lumber`),
  **#101** (`globex_interest_rates`' 2010-2012 holiday-Monday grid), **#89**, **#85**.

## 11. Traps that cost waves 4 and 5 real time

- **A channel declared down is not down.** The 2026-09-12 round-0 index bounded its gaps with
  "web.archive.org was unreachable for the whole session"; the fix round measured HTTP 200 in
  1.04 s and withdrew it. The first connection in a burst can fail with `curl: (7)`; retry before
  concluding anything, and never let an untested failure become a recorded gap.
- **Do not read a document across years.** The 2026 MLK notice does not exist in ICE's archive
  and the CDE one had to come from the operator's live listing. A 2023 grid is not a statement
  about 2022, and `holidays/raw/cfe-eurex-ice-cde-smfe-2026-2027-fix/INDEX.md` records that
  mistake being caught.
- **Pin a sweep's population independently of the thing it sweeps.** `swept == derived.len()`
  reads the same source twice and holds even when an identity disappears from both; #108's final
  guard is a literal count plus an occurrence floor. Do the same for any era sweep you write.
- **A single-line scan misses rows written across several lines.** A past bench memo listed nine
  of a module's twelve rows because three were formatted one field per line, and a reviewer
  recomputed it. Parse the module or the block with a real reader; never grep a regex over lines
  for a count you are going to publish.
- **Every number you write in prose is data.** Recompute it from the module, the block or the
  artifact, and prefer a number a reader can re-derive.
- **The research store is outside the workspace.** Writes there may be denied; if a write is
  denied, retry that exact command with `danger_full_access` and a one-sentence justification, and
  never leave a retrieved artifact un-installed.
- **One artifact, one document id; one id, one artifact.** Both fences bite, and a shared
  calendar serves several identities under one id that must resolve identically in every file.
- **A blank line between a documents table's header and its separator silently disables the id
  fences** by making the parser read zero rows (§6).
- **Nested backticks and literal `|` inside an evidence cell** break the table. Wave 4's review
  caught both.
- **The `holidays!` macro fails the build** on a non-ascending row, a row outside every window,
  an empty document id, or a tier below T1 — read the error, it names the rule.
- **The era's ordinary grid is not the current profile's.** A holiday row ships only when it
  moves an answer *for that era*; read the dated `revisions!` timeline before deciding.
- **If you touch production code**, measure it. An unmeasured "obvious" change to a hot path is a
  guess.
