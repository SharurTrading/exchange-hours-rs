<!-- SPDX-License-Identifier: MIT-0 -->

# exchange-hours — architectural review and proposed charter

Written 2026-09-12 (UTC) by the reviewing model, on four fact reports gathered read-only
the same day (`A-crate-anatomy-and-cost.md`, `B-sharur-consumption.md`, `C-prior-art.md`,
`D-consequences.md`, this directory). Every number below is cited in those reports.
This review ignores the repository's current laws by instruction and proposes what the
goals and rules should be. Nothing here is decided until the maintainer says so.

## 1. What the crate is for

The crate began as "exchange hours good enough to find session breaks and candle
boundaries for backtesting", grew a product-family profile type when futures such as
cattle did not fit a venue envelope, and now faces a plan for 32 more keys covering CME
trade-type variants. The fact reports settle what it is actually for today:

- SharurPlatform is the only consumer. Two of its fourteen crates import it, through one
  adapter. It asks five things: is it open now; where does this trading day begin and end;
  which trade date does an instant belong to; when does the next session open; is this gap
  a closure to compress out of the chart.
- Seven of the eight production `is_open` calls ask about now. SharurPlatform ships no
  historical replay engine. Intraday history never exceeds about 75 days. The only deep
  query is the daily chart's backward walk, which is unclamped by accident, reaches 2007,
  and gets zero rows back.
- Being told "closed" when the venue is open destroys data (live prints dropped, daily
  bars folded into the next day). Being told "open" when the venue is closed corrupts
  labels and gates only. A calendar that can only widen is recoverable; one that narrows
  is not.

**Purpose, restated.** exchange-hours is SharurPlatform's live session calendar for the
instruments its adapters can route. It answers those five questions correctly for today
and for as far back as the platform's own data reaches. It is not an archive of exchange
history, and it is not a public reference work.

## 2. The findings that force the shape

| # | Finding | Consequence |
|---|---|---|
| F1 | 36 keys exist; 8 are reachable from a Sharur instrument; 23 have no reference anywhere in SharurPlatform. All five SGX equity-index keys, all five metals TAS keys, weather, spot-quoted, event contracts, mini grains, rough rice, six ICE softs and Eurex fixed income have no consumer. | The admission rule, not the model shape, is what is unbounded. |
| F2 | 279 revision rows; 35 reachable; 185 (66%) pre-2020. Of the 35 reachable rows exactly two fall in 2020–2025. | The January-2010 floor manufactures rows no query reaches on purpose. |
| F3 | The SGX pre-2020 programme: 6 issues, 4 PRs, 10 days, about 100 agent runs, 417 files, 101 MB, 14 revision rows, all still Partial, zero reachable. ECBTC: eight runs and 37 artifacts to adjudicate one hour on an unreachable key. | The evidence process is priced for a public reference work the crate is not. |
| F4 | Holidays are excluded by law; SharurPlatform built the overlay seam and never injects it. About 13 CME days a year, roughly 5% of a daily window, are served as open on every root, and the DOM ladder stays armed on Christmas Day. | The one recurring defect every root suffers every year is the one thing the law forbids fixing. |
| F5 | Rithmic admits NYBOT and EUREX, the crate has correct keys for ICE softs and Eurex fixed income, and Sharur's root table is Globex-only, so ICE Sugar silently gets NYSE FANG+ hours, wrong by about 12.5 hours a day. | A live, current-schedule error on a traded venue costs zero crate work to fix and is a catalog gap. |
| F6 | Prose outweighs Rust 2.35 to 1 by bytes; schedule modules are 60% comment; `futures_profile.rs` sits at 488 of a 500-line ceiling; four modules were split only to store citations. | The ceiling is binding on prose, not code. |
| F7 | Effort splits roughly 35% retrieval, 25% adjudication, 20% citation prose, 10% fence upkeep, 10% review cycles. cmegroup.com returns 403 at IP level; the operator's own machine feed returns 401 and is excluded by law. | The law forbids the cheapest correct channel and the process pays for its absence. |
| F8 | `regular` versus `extended` is inert on 78% of Sharur's roots and tracks whether CME printed the word "RTH"; `order_entry` is load-bearing (it keeps the Pre-Open out of `is_open`). The 4-hour maintenance heuristic that decides how every chart renders a gap has no citation at all. | Rigour is spent on the field that carries no information and none on the rule that decides rendering. |
| F9 | `globex_cryptocurrency`, on 21 Sharur roots, changed three times in five weeks and carries a forward-dated row for 2026-09-19. Nothing in the governance prioritises watching it over archaeology on an unreachable key. | Monitoring the served keys is the real correctness risk. |
| F10 | The 32-key plan projects about 10,000 source lines, 70 rows, 169 hand-written registration edits and twelve guaranteed ledger conflicts, for zero reachable roots. Neither Sharur adapter can produce a trade-type root. | The plan is correct research aimed at nobody. |

## 3. Proposed charter — the rules going forward

Each rule names what it replaces. Kept laws are listed in R11.

**R1 — Purpose.** As stated in section 1. Every other rule is judged against it.

**R2 — Granularity: product family, admitted on demand.** A key exists when a root a
consumer's adapter can produce maps to it and the listing-exchange default would be wrong
for that root at a day boundary, on `is_open`, or on the trade date. Trade-type variants
(TAS, TAM, BTIC, TACO, TMAC) get no keys: the consumer maps a variant to its underlying
family with a disclosed variant flag and the known close offset, and refuses nothing; a
variant with more structure than its underlying (Nikkei BTIC's second window, 6EB's
London break, the crypto BTIC stops) stays on the flagged fallback. The 23 unreachable keys
are frozen, not deleted: they keep their wire identity, their ledger row says "frozen —
no consumer", and they receive no further research or review cycles until a consumer
appears. Replaces: unbounded admission; the 32-key plan.

**R3 — History: a per-key horizon, never a floor.** Each key declares the earliest date
its evidence covers. Below it the calendar serves the sessionless closure the crypto key
already uses, documented as "not modelled" rather than "closed", and the consumer's window
walk returns nothing rather than a fabricated day. Existing rows stay; nothing is carried
back below its first operator statement; the January-2010 floor and the carry-back
convention are retired. New keys start at the current schedule plus the dated changes the
consumer's data can reach. Replaces: the floor, carry-back, sourced intersections across
undated spans, the knowledge-boundary Monday rule.

**R4 — Evidence: tiered and recorded.** T1, the operator's own statement (rulebook,
notice, specification, circular), public or a verbatim member mirror. T2, the operator's
machine feed or service, public preferred, authenticated admissible when the artifact is
saved. T3, a member firm's or vendor's restatement. T4, press. A current schedule needs T1
or T2; a dated change needs an operator-stated day at T1 or T2; T3 may date a change only
when it mirrors an operator document verbatim; T4 never keys a row. Every row records its
tier. When two operator statements conflict, document lineage decides and both are
recorded, as ECBTC did. A close that coincides with a settlement range is still a close
when T2 observes it; the settlement instant itself is never the boundary. Replaces:
primary-only and public-only as absolutes.

**R5 — Holidays: in scope, per family, for served keys.** The crate carries its own
holiday and early-close tables per product family, from the operator's published holiday
calendar (T1, annual), for served families only, starting with the current and next
year. Lookup must be a sorted-date binary search so the overlay stops costing 100 times an
`is_open`; re-measure first. Replaces: LAW-HOLIDAY-SCOPE.

**R6 — Fields.** Session boundaries, trade date and `order_entry` are required.
`regular` is optional and declared with one sentence, not four channels. The
maintenance-versus-closed heuristic is written into the charter as crate policy with a
test, not sourced per key. Replaces: the four-channel proof of an empty `regular`.

**R7 — Evidence lives in `docs/evidence/<key>.md`.** A schedule module holds rule data,
one line per row (effective day, tier, document id) and a link; the narrative, quotations
and URLs live in the evidence file; a fence checks that every row's day appears there.
The 500-line ceiling applies to code. Ledger rows shrink to a fixed shape: key, owner,
served or frozen, tier, horizon, reviewed-on (UTC), gap kind, at most three sentences.
Replaces: citation beside the literal; the 6,000-character ledger cell.

**R8 — Admission is a bounded task.** A key PR is: current schedule from T1 or T2, its
horizon, tests for the boundaries, the trade date, the weekend and any dated change, the
evidence file, the ledger row and the registration list. One PR, one day, no adversarial
multi-agent programme, no archaeology. The research store stays local; only the evidence
file is committed.

**R9 — Watch, don't dig.** Served keys carry a review cadence (monthly for high-churn
families such as crypto and event contracts, quarterly otherwise); a forward-dated row
carries a confirm-by date; every schedule change ships as a tagged release with a
CHANGELOG entry so the consumer pins versions, not commits.

**R10 — Consumer contract.** The ledger marks each key served or frozen. SharurPlatform
owns the root-to-family map and must cover every venue namespace its adapters admit (ICE
softs and Eurex fixed income today), clamps the daily walk to the contract's activation,
and pins a tagged release rather than a non-ancestor commit.

**R11 — Laws kept and laws changed.** Kept as written: DETERMINISM, PANIC,
SESSION-NOT-EXPIRY, UTC-DATES, NO-FABRICATED-DATES (a dated row needs a stated day; the
horizon replaces the floor), FOLLOW-UPS-ARE-ISSUES (scoped to served keys). Changed:
PRIMARY-SOURCES becomes R4; PUBLIC-SOURCES becomes R4's tiering; HOLIDAY-SCOPE becomes R5;
citation-beside-literal becomes R7; the floor and carry-back become R3.

**R12 — What this gives up.** The crate stops being a public evidence ledger with
history to 2010; the SGX, ICE and metals-TAS research is preserved but frozen; a future
replay engine that needs deep history extends a key's horizon as a bounded R8 task, which
the evidence-file model makes cheap.

## 4. Migration, in order

1. The maintainer decides on this review (the questions in section 5).
2. One PR rewrites AGENTS.md to the charter. It is the only PR that touches law.
3. Close out the trade-type programme: #83 is not merged and its branch is kept as
   research; #79–#82, #71, #72, #73–#77 close as "frozen scope: trade types are not
   modelled"; #58 is re-scoped to the consumer-side variant flag; #66 closes as frozen.
   #78 is cut down to the two fences that survive the ledger reshape (the ledger array
   equals `MarketHoursKey::ALL`; the number-word capacity fix) and merged.
4. One PR reshapes the ledger and moves the eight served keys' evidence into
   `docs/evidence/`; one PR marks the remaining keys frozen and moves their evidence
   opportunistically.
5. Holidays: re-measure the overlay, implement per-family tables for the eight served
   families from CME Group's 2026 and 2027 holiday calendars with a binary-search lookup.
6. Tag 1.1.0. SharurPlatform pins it, clamps the daily walk to activation, and adds root
   tables for ICE softs and Eurex fixed income.
7. Confirm the crypto key's 2026-09-19 row before that Saturday; start the watch cadence.

## 5. Questions for the maintainer

1. Is SharurPlatform the crate's only intended consumer for the foreseeable future? R2's
   freezing of 23 keys and R3's horizon assume yes.
2. Freeze or delete the unreachable keys? The review recommends freeze: reversible, no
   wire-identity break, no code churn.
3. Holidays in the crate (R5) or in a separate crate? The review recommends in-crate for
   a small studio: one dependency, one release, the family granularity already fits.
4. Horizon equals the existing evidence horizon (keep every paid-for row), or a repo-wide
   cut at 2019 or 2020 (delete rows)? The review recommends the former.
5. Confirm closing #83 unmerged and cutting #78 down, as in step 3.

## 6. Decisions taken (maintainer, 2026-09-12 UTC)

1. **SharurPlatform is the only intended consumer** for the foreseeable future, and it
   will support more markets over time. The charter therefore keeps proactive coverage
   possible but priced: keys are **served** (a consumer root maps to them) or **dormant**
   (kept, correct as of their last review, re-reviewed on demand). Nothing is deleted.
   R2's "frozen" becomes "dormant".
2. **Holidays live in this crate**, per family, from the January-2010 floor to the
   operator's published future (exchanges publish one to two years ahead). R5 is adopted
   with that range; the tables are data, released annually.
3. **History runs from January 2010 to the future.** The floor and the carry-back
   convention stay. What changes is who pays for depth: a served key's history to the
   floor is an obligation; a dormant key's is best-effort and labelled. R3 is amended
   accordingly; the per-key horizon becomes the recorded date below which a key's rows
   are carried rather than sourced.
4. **Paid-for work is kept unless discarding it is justified for the long term.** #78
   merges as is; #83 (five metals TAS keys) merges as dormant after a proportionate
   review; the trade-type research is preserved and the remaining 27 keys are built only
   when a consumer maps a variant. The two law questions #71 (a past conditional day
   discharged by a later operator artifact) and #72 (a labelled feed close is T2 session
   language unless operator prose contradicts it) are folded into R4 and closed.
5. The charter PR rewrites AGENTS.md next; the ledger reshape, evidence files and the
   holiday tables follow as separate PRs.
