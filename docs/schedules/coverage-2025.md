<!-- SPDX-License-Identifier: MIT-0 -->

# Complete 2025-onward coverage inventory

Stage 1 of the [release plan](../plans/2026-09-12-path-to-release.md) (section 5), tracking issue
[#114](https://github.com/SharurTrading/exchange-hours-rs/issues/114). Inspected at `3353d13` on
2026-09-21 UTC. Seven rows' `Missing / disputed`, `Complete?` and `Closing issues` cells were
corrected on 2026-09-22 UTC when the phase-level declarations shipped — six whose scope withholds the
Sunday 16:00-16:15 CT quarter-hour, and `globex_cryptocurrency`, whose three cells moved with them —
together with §4's verdict count and §5's `#79` bullet. The inspection statement above still
describes the revision the values were first derived at; the counts, windows and horizons it covers
are otherwise unchanged. The `cme` and `cbot` rows' own prose counts, and §2's, were corrected on
2026-09-26 UTC to the `Unsrc 2025+ dates` cells the inventory fence derives, and
`globex_cryptocurrency`'s `2025+ dates` count, `Missing / disputed` and `Complete?` cells moved on
the same date, when its nine merged trade dates shipped as rows: the count is now 32 and the two
cells name what remains open rather than the merges. On the same date the `globex_grains` and
`globex_livestock` verdicts moved, to **incomplete** under #152: both serve a `14:30-16:00` CT
post-close queue whose trade date the crate derives from the session it feeds, so every covered
date carrying it answers a trade date other than the operator's own printing. That change also
lands the eighteen `globex_grains` replacement-block rows and moves the `cme`, `cbot` and
`globex_grains` date counts with them.

One row per served instrument scope, recording what the crate actually ships against the adopted
2025 floor. This is a **record, not a claim**: no runtime change accompanies it and it moves no
ledger row. Every value is derived from the shipped modules, the verification ledger, the consumer's
own routing tables and the owner evidence files. The artifacts cited in
[Artifact resolution](#artifact-resolution) were located in the research store and their digests
recomputed by this stage.

## What “complete” means here

LAW-COVERAGE fixes the support floor at **2025-01-01**: complete coverage means no unresolved
normal-week, required-phase, holiday or special-session gap in the claimed interval. Three
consequences drive every verdict below.

- **An audited window containing `Unsourced` dates is not complete.** A window endpoint is an outer
  bound, not a completeness claim, so a scope can reach 2027-12-31 and still be incomplete.
- **Coverage that stops before the inspection date leaves a forward gap**, unless the operator
  publishes nothing further. Each scope's horizon is recorded below.
- **A broad venue intersection is not a substitute for an instrument's exact calendar.**
  `cme`, `cbot`, `comex` and `nymex` are the design memo D17 **intersection** of the families routed
  to them, so a date the routed families dispute carries `Unsourced` rather than a row.

## How to read a row

| Column | Meaning |
|---|---|
| Identity | canonical wire name from `Exchange::as_str` / `MarketHoursKey::as_str` |
| Owner | shipped module carrying the normal-week timeline |
| Normal week | sourced `revisions!` timeline: earliest…latest effective day and row count |
| Horizon | ledger carried-below date; `—` when nothing is carried |
| Holidays | this scope's shipped windows from `holiday_coverage()` |
| `2025+ dates` | trade dates at or after 2025-01-01 that this scope answers with a holiday row; one row per date in every served table |
| `Unsrc` 2025+ dates | trade dates at or after 2025-01-01 this scope withholds as `Unsourced`; any one fails completeness |
| Missing / disputed | what the scope cannot answer inside 2025+, and its issue |
| Complete? | verdict over the 2025-onward interval |
| Closing issues | open issues to discharge for this scope |

Every count is taken over the identity's **own routed table** - the one `holidays/routing.rs`
selects for it - and not over the module that table lives in. The distinction is load-bearing:
`src/calendar/schedules/holidays/ice_us.rs` holds six tables, of which only `VENUE` backs the
served `iceus` identity, while `FANG`, `DOLLAR_INDEX`, `SUGAR_COFFEE_COCOA`, `COTTON` and
`ORANGE_JUICE` back dormant `MarketHoursKey`s. Those five tables are individually smaller than
`VENUE`, but together they hold 105 further rows, and a count over the module would have reported
129 rows where the served identity answers for 24 dates.

## Inventory

| Identity | Owner | Normal week | Horizon | Holidays | 2025+ dates | `Unsrc` 2025+ dates | Missing / disputed | Complete? | Closing issues |
|---|---|---|---|---|---|---|---|---|---|
| `cme` | [cme_group.rs](../../src/calendar/schedules/futures/us/cme_group.rs) | 2010-11-15 … 2026-08-22 (5 rows) | 2012-05-03 | 2010-01-01..2012-12-31, 2013-01-01..2015-12-31, 2016-01-01..2018-12-31, 2019-01-01..2021-12-31, 2022-01-01..2024-12-31, 2025-01-01..2027-12-31 | 70 | 61 | all of 2025-2027 on 61 disputed dates (2025-01-02 … 2027-12-23); the Sunday 16:00-16:15 CT quarter-hour, withheld (#79) | **incomplete**: 61 `Unsourced` dates in 2025+ and the Sunday 16:00-16:15 CT quarter-hour is withheld (#79) | #79, #116, #117 |
| `cbot` | [grains.rs](../../src/calendar/schedules/futures/us/grains.rs) | 2010-04-19 … 2015-07-05 (6 rows) | 2010-03-15 | 2010-01-01..2012-12-31, 2013-01-01..2015-12-31, 2016-01-01..2018-12-31, 2019-01-01..2021-12-31, 2022-01-01..2024-12-31, 2025-01-01..2027-12-31 | 70 | 61 | all of 2025-2027 on 61 disputed dates (2025-01-02 … 2027-12-23) | **incomplete**: 61 `Unsourced` dates in 2025+ | #116, #117 |
| `comex` | [energy_metals.rs](../../src/calendar/schedules/futures/us/energy_metals.rs) | 2015-09-20 … 2026-08-22 (2 rows) | 2012-05-11 | 2010-01-01..2012-12-31, 2013-01-01..2015-12-31, 2016-01-01..2018-12-31, 2019-01-01..2021-12-31, 2022-01-01..2024-12-31, 2025-01-01..2027-12-31 | 53 | — | the Sunday 16:00-16:15 CT quarter-hour, withheld (#79) | **incomplete**: the Sunday 16:00-16:15 CT quarter-hour is withheld (#79) | #79, #116, #117 |
| `nymex` | [energy_metals.rs](../../src/calendar/schedules/futures/us/energy_metals.rs) | 2015-09-20 … 2026-08-22 (2 rows) | 2012-05-11 | 2010-01-01..2012-12-31, 2013-01-01..2015-12-31, 2016-01-01..2018-12-31, 2019-01-01..2021-12-31, 2022-01-01..2024-12-31, 2025-01-01..2027-12-31 | 53 | — | the Sunday 16:00-16:15 CT quarter-hour, withheld (#79) | **incomplete**: the Sunday 16:00-16:15 CT quarter-hour is withheld (#79) | #79, #116, #117 |
| `cfe` | [cfe.rs](../../src/calendar/schedules/futures/us/cfe.rs) | 2010-12-10 … 2021-12-06 (8 rows) | 2010-01-01 | 2026-01-01..2026-12-31 | 12 | — | all of 2025; the window opens 2026-01-01 | **no 2025 coverage** | #98, #116 |
| `coinbase_derivatives` | [coinbase_derivatives.rs](../../src/calendar/schedules/futures/us/coinbase_derivatives.rs) | 2026-09-11 … 2026-09-11 (1 row) | — | 2021-06-28..2026-09-07 | 20 | — | 2026-09-08 onward (past the horizon) | complete to 2026-09-07; **horizon before inspection** | #86, #98, #116 |
| `eurex` | [europe.rs](../../src/calendar/schedules/futures/international/europe.rs) | seasonal selector, no `revisions!` timeline | 2010-01-01 | 2026-01-01..2026-12-31 | 7 | — | all of 2025; the window opens 2026-01-01 | **no 2025 coverage** | #77, #86, #98, #116 |
| `iceus` | [ice_us.rs](../../src/calendar/schedules/futures/us/ice_us.rs) | 2017-11-07 … 2017-11-08 (2 rows) | — | 2026-01-01..2028-01-03 | 24 | 20 | all of 2025, plus 20 `Unsourced` dates in 2026-2027 | **no 2025 coverage** | #98, #116 |
| `globex_equity_index` | [cme_group.rs](../../src/calendar/schedules/futures/us/cme_group.rs) | 2010-11-15 … 2026-08-22 (5 rows) | 2012-05-03 | 2010-01-01..2012-12-31, 2013-01-01..2015-12-31, 2016-01-01..2018-12-31, 2019-01-01..2021-12-31, 2022-01-01..2024-12-31, 2025-01-01..2027-12-31 | 54 | — | the 16:00-16:15 CT Sunday quarter-hour, withheld (#79) | **incomplete**: the Sunday 16:00-16:15 CT quarter-hour is withheld (#79) | #79, #116, #117 |
| `globex_energy` | [energy_metals.rs](../../src/calendar/schedules/futures/us/energy_metals.rs) | 2015-09-20 … 2026-08-22 (2 rows) | 2012-05-11 | 2010-01-01..2012-12-31, 2013-01-01..2015-12-31, 2016-01-01..2018-12-31, 2019-01-01..2021-12-31, 2022-01-01..2024-12-31, 2025-01-01..2027-12-31 | 53 | — | the Sunday 16:00-16:15 CT quarter-hour, withheld (#79) | **incomplete**: the Sunday 16:00-16:15 CT quarter-hour is withheld (#79) | #79, #116, #117 |
| `globex_grains` | [grains.rs](../../src/calendar/schedules/futures/us/grains.rs) | 2010-04-19 … 2015-07-05 (6 rows) | 2010-03-15 | 2010-01-01..2012-12-31, 2013-01-01..2015-12-31, 2016-01-01..2018-12-31, 2019-01-01..2021-12-31, 2022-01-01..2024-12-31, 2025-01-01..2027-12-31 | 54 | — | the post-close queue's trade-date label: the crate dates the 14:30-16:00 CT queue by the session it feeds, so all 746 covered dates that carry it answer a trade date other than the operator's own printed one (#152) | **incomplete**: the post-close queue's trade-date label differs on every one of the 746 dates that carry the queue (#152) | #116, #117, #152 |
| `globex_fx` | [fx.rs](../../src/calendar/schedules/futures/us/fx.rs) | 2010-11-15 … 2026-08-22 (2 rows) | 2012-05-03 | 2010-01-01..2012-12-31, 2013-01-01..2015-12-31, 2016-01-01..2018-12-31, 2019-01-01..2021-12-31, 2022-01-01..2024-12-31, 2025-01-01..2027-12-31 | 36 | — | the Sunday 16:00-16:15 CT quarter-hour, withheld (#79) | **incomplete**: the Sunday 16:00-16:15 CT quarter-hour is withheld (#79) | #79, #116, #117 |
| `globex_interest_rates` | [interest_rates.rs](../../src/calendar/schedules/futures/us/interest_rates.rs) | 2010-11-15 … 2026-08-22 (3 rows) | 2010-01-01 | 2010-01-01..2012-12-31, 2013-01-01..2015-12-31, 2016-01-01..2018-12-31, 2019-01-01..2021-12-31, 2022-01-01..2024-12-31, 2025-01-01..2027-12-31 | 53 | — | the Sunday 16:00-16:15 CT quarter-hour, withheld (#79) | **incomplete**: the Sunday 16:00-16:15 CT quarter-hour is withheld (#79) | #79, #116, #117 |
| `globex_livestock` | [livestock.rs](../../src/calendar/schedules/futures/us/livestock.rs) | 2014-10-27 … 2020-05-31 (4 rows) | 2010-01-01 | 2010-01-01..2012-12-31, 2013-01-01..2015-12-31, 2019-01-01..2021-12-31, 2022-01-01..2024-12-31, 2025-01-01..2027-12-31 | 36 | — | the post-close queue's trade-date label: the crate dates the 14:30-16:00 CT queue by the session it feeds, so every covered date that carries it answers a trade date other than the operator's own printed one (#152) | **incomplete**: the post-close queue's trade-date label differs on every date that carries the queue (#152) | #116, #117, #152 |
| `globex_cryptocurrency` | [cryptocurrency.rs](../../src/calendar/schedules/futures/us/cryptocurrency.rs) | 2017-12-17 … 2026-09-20 (9 rows) | — | 2019-01-01..2021-12-31, 2022-01-01..2024-12-31, 2025-01-01..2027-12-31 | 32 | — | the 24/7 era's Monday and Thursday holidays omit the 16:00 CT close, so the 16:00-16:01 CT minute is served closed (#93); the five-day era's Sunday and weekday Pre-Open onset is undated (#123) | **incomplete**: the 24/7-era 16:00-16:01 CT minute is served closed (#93) and the five-day era's Pre-Open onset is undated (#123) | #93, #116, #117, #123 |
| `globex_nikkei_225_dollar` | [cme_nikkei.rs](../../src/calendar/schedules/futures/us/cme_nikkei.rs) | 2011-01-12 … 2015-09-20 (4 rows) | — | 2016-01-01..2018-12-31, 2019-01-01..2021-12-31, 2022-01-01..2024-12-31, 2025-01-01..2027-12-31 | 49 | — | none in 2025+ | complete to 2027-12-31 | #116, #117 |

## Consumer routing

Reachability is what makes an identity **served** under LAW-SERVICE-TIERS, so each row above is tied
to a site in SharurPlatform. The consumer owns both maps; this crate maps no symbols.

**Venue identities.** The consumer's only venue-namespace → `Exchange` map is `NAMESPACES` in
`crates/adapters/rithmic/src/catalog.rs` (lines 61-174), consumed by
`translate::exchange_for_rithmic_code`, and all eight are reached there. The four CME-group
venues also hold many authored roots in `static PRODUCTS`, so their mapped instruments resolve as
`SessionHoursBasis::ProductFamily`; only `cfe`, `eurex`, `iceus` and `coinbase_derivatives` have no
authored root and so reach the venue calendar through `ExchangeFallback`.

| Identity | Namespace | Authored root rows | Other consumer sites |
|---|---|---|---|
| `cme` | "CME" | many in `static PRODUCTS` | market-clock overview, settings selector, platform default |
| `cbot` | "CBOT" | `static PRODUCTS` | settings selector |
| `comex` | "COMEX" | `static PRODUCTS` | settings selector |
| `nymex` | "NYMEX" | `static PRODUCTS` | settings selector |
| `cfe` | "CFE" | none (`VX` absent) | market clock uses the `CfeVix` family instead |
| `eurex` | "EUREX" | none | market clock uses the `Eurex` family instead |
| `iceus` | "NYBOT" | none | market clock uses the `IceUs` family instead |
| `coinbase_derivatives` | "CDE" | none | - |

**Product-family identities.** The eight `globex_*` keys are reached by instrument root through
`static PRODUCTS` in `crates/domain/src/globex_products.rs` (104 roots), so a mapped root returns
`SessionHoursBasis::ProductFamily(key)`. The curated UI lists reach seven of the eight:
`GLOBEX_MARKETS` in `crates/ui/src/market_clock/sets.rs` names the equity-index, energy, grains,
FX, interest-rate, livestock and crypto keys, while `FAMILIES` in
`crates/ui/src/platform_settings/window.rs` names six of those - the same list without crypto -
plus `GlobexMiniGrains`; `globex_nikkei_225_dollar` is reached only through the authored root
table. `GlobexMiniGrains` is one the ledger carries as **dormant** and which no authored root
reaches; that is a routing question for Stage 6 (#118), not a coverage gap here, and it is
recorded rather than resolved in this stage.

### No-holiday and synthetic scopes

Stated explicitly, as the stage requires. **No served scope is synthetic and no served scope is a
no-holiday scope**: all sixteen ship a holiday table, and the row above for each names its windows.
The crate's two synthetic identities are `Exchange::Unknown`, the UTC 24x7 fallback, and
`AlwaysOpen`; both are **dormant**, both answer `None` in `holidays/routing.rs`, and neither is a
venue fact. They therefore contribute no inventory row, and no consumer instrument in the maps
recorded under [Consumer routing](#consumer-routing) reaches either.

### Venue intersections are derived, not retrieved

`cme`, `cbot`, `comex` and `nymex` cite no holiday artifact of their own: each table is the D17
intersection of the family tables routed to that venue, so its document ids are the family modules'
own and the four venue evidence files carry the derivation and every dropped date rather than a
retrieval. `comex` and `nymex` route a single family and so reproduce it row for row; `cme` and
`cbot` route several and therefore withhold the disputed dates recorded above.

## Findings

### 1. Three served scopes ship no 2025 holiday coverage at all

`cfe` and `eurex` ship a single 2026 window and `iceus` a 2026-01-01..2028-01-03 window, so all three
answer no holiday question over any part of 2025. `iceus` ships only four scheduling rows: its
table is the D17 intersection of the seven ICE Futures U.S. families routed to the venue, so a row
ships only where all seven agree - the four full closures in its window - and the other **20** dates
are `Unsourced`, the shape of a date on which the softs close while the index families trade
shortened hours. These are the one-operator scopes whose instruments reach the venue calendar
directly through `ExchangeFallback` rather than through a family key, so the gap is on the path a
real instrument takes. Stage 4's PR order (section 8, items 3-5) already anticipates these three,
and they are the three served scopes whose evidence files still lack the fixed
`### Documents` shape that #98 tracks.

### 2. Two venue intersections carry `Unsourced` dates inside 2025+

`cme` withholds **61** dates and `cbot` **61** across 2025-2027, covering every US market holiday the
routed families dispute — fourteen of the additions are the 2025-2027 closure eves, on which
`globex_grains` states a complete replacement-blocks day and the other routed families audited the
date normal. This is the intersection behaving as designed rather than a defect: the
charter states that family disagreement is `Unsourced`, never silence, and that a broad intersection
remains explicitly partial and cannot stand in for a complete family calendar. It does not by itself
block release provided every reachable instrument resolves to a complete exact family or documented
scope.

### 3. One scope's publication horizon is already behind the inspection date

`coinbase_derivatives` covers 2021-06-28..2026-09-07 with no 2025+ `Unsourced`, but the operator's
covered future stops 14 days before this inspection (2026-09-21). The interval after 2026-09-07 has
no answer, and Stage 4 item 2 refreshes it.

### 4. One scope is complete to 2027-12-31

`globex_nikkei_225_dollar` reaches 2027-12-31 with no `Unsourced` row at or after 2025-01-01 and no
declared phase-level gap. It is the only one: `globex_grains` and `globex_livestock` reach that date
with no withheld date either, but both answer the post-close queue's trade date with the crate's own
convention rather than the operator's printed label on every covered date that carries the queue —
746 of them for `globex_grains` — so both now read **incomplete** under #152. The other nine scopes
that reach that date are **not** complete either: `cbot` withholds disputed dates in §2 and `cme`
those plus the Sunday 16:00-16:15 CT quarter-hour; `comex`, `nymex`, `globex_energy`,
`globex_interest_rates` and `globex_equity_index` withhold that same quarter-hour in §5, and so does
`globex_fx`; and `globex_cryptocurrency` loses the 24/7-era 16:00-16:01 CT minute to an
unstateable gap and carries an undated five-day-era Pre-Open onset. `comex` and `nymex` are intersections too, but each routes one
family's grid, so they match it row for row and their six `Unsourced` rows all fall in 2019-2023,
outside the new floor.

### 5. Issues checked for an effect that survives the new floor

- **#79 - still blocks, for seven scopes, but only in the dated era.** `cme`, `comex`, `nymex`,
  `globex_energy`, `globex_equity_index`, `globex_fx` and `globex_interest_rates` each end their
  timeline in a knowledge-bound row (2026-08-22) that *widens* the Sunday queue to 16:00-17:00 CT, and
  each module and ledger basis note records that only the disputed 16:00-16:15 CT quarter-hour depends
  on the undated 2012 move, so that phase is withheld rather than sourced and none of the seven is
  complete in the era before it. From 2026-08-22 on — a Saturday, so the first Sunday the bound
  governs is 2026-08-23 — each profile's Sunday queue serves the quarter-hour, and the metadata's
  `#79` declarations are bounded there, so those dates stop being withheld, and all seven then report
  `Covered`. `globex_fx` used to be the exception, on account of the second, whole-domain `#93` gap it
  carried; its merged trade dates now ship as rows (the last of them in Stage 5) and that declaration
  is gone, so only the quarter-hour it names here remains. The plan's condition for #79 is met for all
  seven, and the gap it names survives to the floor.
- **#105 - dormant, not blocking.** CME's `dairy` and `lumber` product groups have no
  `MarketHoursKey` and fold into `globex_grains` and `globex_livestock`. Neither group is reachable:
  no root for either appears in `static PRODUCTS` (which maps `LE` to livestock - live cattle, not
  lumber), and no lumber or dairy reference appears anywhere in the consumer's sources. The plan's
  condition is "yes if a served instrument uses the wrong family; otherwise dormant evidence", so it
  does not block and must be re-checked in Stage 6 if the consumer ever maps those groups.

### 6. Special-session needs (Stage 3, #93)

Dates that change internal phase topology are not representable by the scalar vocabulary, so an
affected scope declares them as a gap rather than approximating them. `globex_cryptocurrency` is the
one scope that still does, and it is the last of them: its nine five-day-era merged trade dates ship
as built-in replacement rows (2026-09-26 UTC), so what remains unstated for it is the 24/7 era's
16:00-16:01 CT minute, served closed, plus its own undated five-day-era Pre-Open onset (#123), and
those closing conditions stay in its evidence file. The other scopes this section used to name are
covered — `globex_energy`, `globex_equity_index`, `globex_fx`, `globex_interest_rates` and
`globex_nikkei_225_dollar` carry the three Saturday-session trade dates as built-in replacement-block
rows, as do `comex` and `nymex`, which route `globex_energy`'s table whole; and `globex_fx` now
carries its merged trade dates too, so every session CME publishes for it is stated and its `#93`
declaration is gone. `globex_grains` and `globex_livestock` state no row on those Saturdays.

## Artifact resolution

Two checks were run, and both are reported as they came out.

**Full sweep of the evidence corpus.** Every Markdown table whose header carries a `sha256` column
was parsed across the thirteen owner evidence files that carry such a table: **43 tables and 1,941
rows**, resolving **256 distinct document ids**, whose quoted digests are in bijection with 256
distinct digests. Those same bytes are saved more than once: a walk of the store found all 256 at
**400 loose paths**, with 111 of the digests present at more than one path and 95 under more than
one era directory, so a digest resolves to bytes that more than one location carries -
including loose copies of members of the store's 14 zip bundles. Every quoted digest was
recomputed from the bytes: **1,941 of 1,941 reproduce, with 0 mismatches, 0 unlocatable artifacts
and 0 unparseable rows.** A second independent pass re-hashed every distinct artifact from raw
bytes, re-extracting the zip members, and reproduced every one.

**Load-bearing endpoints.** For each scope, the artifact establishing the state in force at the 2025
floor (**baseline**) and the one establishing its **horizon** were resolved through the owner
evidence file's document tables - a `### Documents` section, or the file's `## Evidence
documents` section - to saved bytes, and their digests recomputed by this stage:

| Identity | Baseline document | Tier | Baseline artifact and digest | Horizon document | Tier | Horizon artifact and digest |
|---|---|---|---|---|---|---|
| `cme` | `CME-SVC-2024-12-31` | T2 | `holidays/raw/cme-2025-2027/arc/thbp_2024-12-31_2025-01-02_20241220155340.json` - sha256 reproduces | `CME-SVC-2027-12-22` | T2 | `holidays/raw/cme-2025-2027/live/thbp/thbp_2027-12-22_2027-12-25.json` - sha256 reproduces |
| `cbot` | `CME-SVC-2024-12-31` | T2 | `holidays/raw/cme-2025-2027/arc/thbp_2024-12-31_2025-01-02_20241220155340.json` - sha256 reproduces | `CME-SVC-2027-12-22` | T2 | `holidays/raw/cme-2025-2027/live/thbp/thbp_2027-12-22_2027-12-25.json` - sha256 reproduces |
| `comex` | `CME-SVC-2024-12-31` | T2 | `holidays/raw/cme-2025-2027/arc/thbp_2024-12-31_2025-01-02_20241220155340.json` - sha256 reproduces | `CME-SVC-2027-12-22` | T2 | `holidays/raw/cme-2025-2027/live/thbp/thbp_2027-12-22_2027-12-25.json` - sha256 reproduces |
| `nymex` | `CME-SVC-2024-12-31` | T2 | `holidays/raw/cme-2025-2027/arc/thbp_2024-12-31_2025-01-02_20241220155340.json` - sha256 reproduces | `CME-SVC-2027-12-22` | T2 | `holidays/raw/cme-2025-2027/live/thbp/thbp_2027-12-22_2027-12-25.json` - sha256 reproduces |
| `cfe` | `CBOE-HOURS-USFUT-2026` | - | no `### Documents` table | `CBOE-HOURS-USFUT-2026` | - | no `### Documents` table |
| `coinbase_derivatives` | `CDE-MN-24-25` | T1 | `holidays/raw/cde-2021-2025/pdf/24-25.pdf` - sha256 reproduces | `CDE-MN-26-36` | T1 | `holidays/raw/cde-2021-2025/pdf/26-36.pdf` - sha256 reproduces |
| `eurex` | `EUREX-HOLREG-2026` | - | no `### Documents` table | `EUREX-HOLREG-2026` | - | no `### Documents` table |
| `iceus` | `IFUS-CAL-2026` | - | no `### Documents` table | `IFUS-CAL-2027` | - | no `### Documents` table |
| `globex_equity_index` | `CME-SVC-2024-12-31` | T2 | `holidays/raw/cme-2025-2027/arc/thbp_2024-12-31_2025-01-02_20241220155340.json` - sha256 reproduces | `CME-SVC-2027-12-22` | T2 | `holidays/raw/cme-2025-2027/live/thbp/thbp_2027-12-22_2027-12-25.json` - sha256 reproduces |
| `globex_energy` | `CME-SVC-2024-12-31` | T2 | `holidays/raw/cme-2025-2027/arc/thbp_2024-12-31_2025-01-02_20241220155340.json` - sha256 reproduces | `CME-SVC-2027-12-22` | T2 | `holidays/raw/cme-2025-2027/live/thbp/thbp_2027-12-22_2027-12-25.json` - sha256 reproduces |
| `globex_grains` | `CME-SVC-2024-12-31` | T2 | `holidays/raw/cme-2025-2027/arc/thbp_2024-12-31_2025-01-02_20241220155340.json` - sha256 reproduces | `CME-SVC-2027-12-22` | T2 | `holidays/raw/cme-2025-2027/live/thbp/thbp_2027-12-22_2027-12-25.json` - sha256 reproduces |
| `globex_fx` | `CME-SVC-2024-12-31` | T2 | `holidays/raw/cme-2025-2027/arc/thbp_2024-12-31_2025-01-02_20241220155340.json` - sha256 reproduces | `CME-SVC-2027-12-22` | T2 | `holidays/raw/cme-2025-2027/live/thbp/thbp_2027-12-22_2027-12-25.json` - sha256 reproduces |
| `globex_interest_rates` | `CME-SVC-2024-12-31` | T2 | `holidays/raw/cme-2025-2027/arc/thbp_2024-12-31_2025-01-02_20241220155340.json` - sha256 reproduces | `CME-SVC-2027-12-22` | T2 | `holidays/raw/cme-2025-2027/live/thbp/thbp_2027-12-22_2027-12-25.json` - sha256 reproduces |
| `globex_livestock` | `CME-SVC-2024-12-31` | T2 | `holidays/raw/cme-2025-2027/arc/thbp_2024-12-31_2025-01-02_20241220155340.json` - sha256 reproduces | `CME-SVC-2027-12-22` | T2 | `holidays/raw/cme-2025-2027/live/thbp/thbp_2027-12-22_2027-12-25.json` - sha256 reproduces |
| `globex_cryptocurrency` | `CME-SVC-2024-12-31` | T2 | `holidays/raw/cme-2025-2027/arc/thbp_2024-12-31_2025-01-02_20241220155340.json` - sha256 reproduces | `CME-SVC-2027-12-22` | T2 | `holidays/raw/cme-2025-2027/live/thbp/thbp_2027-12-22_2027-12-25.json` - sha256 reproduces |
| `globex_nikkei_225_dollar` | `CME-SVC-2024-12-31` | T2 | `holidays/raw/cme-2025-2027/arc/thbp_2024-12-31_2025-01-02_20241220155340.json` - sha256 reproduces | `CME-SVC-B-2027-12-22` | T2 | `holidays/raw/cme-2025-2027/live/extra/extra_2027-12-22_2027-12-25.json` - sha256 reproduces |

All twenty-six endpoints of the thirteen scopes that carry a `### Documents` table reproduce.
The three that show no endpoint are a shape gap, not a missing source:

- `cfe`, `eurex` and `iceus` carry **no Markdown table with a `sha256` header at all**; their
  digests (`CBOE-HOURS-USFUT-2026`, `EUREX-HOLREG-2026`, `IFUS-CAL-2026`, `IFUS-CAL-2027` and the
  ICE notices) are quoted inline in prose bullets, so the fixed `### Documents` shape
  LAW-EVIDENCE-FILES describes has not reached these three files and they contribute no rows to
  the sweep. They are the three **served scopes** that still lack it - #98 itself names more, the
  dormant CFE and Eurex keys and the six ICE Futures U.S. family files among them -
  which are also the three scopes that ship no 2025 holiday coverage. Their digests are **not
  machine-verified by this stage**; closing that needs the sweep rule widened to prose-quoted
  digests, and it is recorded as unverified rather than verified.
- No artifact is recorded as missing: every document id this inventory cites resolves to bytes.

## What this stage does not do

- It changes **no runtime data**: no profile, holiday row, revision, ledger row, count, API or wire
  name, and no test expectation.
- It closes no issue. Plan section 3 reserves superseded closure for Stage 5 (#117), and
  #89/#101/#112 must keep their disclosures while the older rows still ship.
- It retrieves nothing: every input was already saved in the research store.
