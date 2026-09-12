<!-- SPDX-License-Identifier: MIT-0 -->

# CME trade-type and standalone-product keys — implementation plan

Plan date: 2026-09-12 (UTC) · Planned against `main` @ `687e562` · Issue #58.

This authors [the trade-type handoff](2026-09-05-cme-trade-type-handoff.md),
which was work item 5 of
[the coverage plan](2026-09-05-cme-globex-family-coverage.md). The handoff said,
per root, which `MarketHoursKey` should own it and what still needed sourcing.
Eight research gates and their nine adversarial verdicts have since run against
it.

**Scope: 13 PRs, 32 new `MarketHoursKey` rows, 6 keys deliberately blocked,
3 rejections recorded on evidence.** One fence PR lands first, because every
later PR touches the same ledger and the same narrative counts.

## Where the full plan and the evidence live

The 1,842-line plan, the eight gate result files and their nine verifier files
are in the **local-only research store**, which is deliberately not in this
repository and is never committed (the coverage plan's "Where the research
already lives" section states the reasoning and the convention). The store is
at `$EXCHANGE_HOURS_RESEARCH`, defaulting to `../exchange-hours-research`
beside this checkout; the files for this plan are under
`cme-globex/trade-types/`:

| File | What it holds |
|---|---|
| `PLAN-58-trade-type-keys.md` | the full per-PR specification, every citation, every probe instant |
| `U1-btic-equity-conflict.*` | the EST/IPT/RVT ContractSpecs contradiction |
| `U4-regular-vs-extended.*` | the channels behind `regular: &[]` |
| `U6-energy-tas-mran-chain.*` | the MRAN chain and the "(ET)" adjudication |
| `U8-cross-zone-design.*` | the cross-zone representation memo and its two verifier passes |
| `U16-U17-taco.*` | TACO's two eras and the Saturday census |
| `ag-crypto-tas-history.*` | grain, livestock and cryptocurrency TAS |
| `metals-tas-history.*` | the five metals TAS families |
| `btic-non-equity-groups.*` | the ten non-equity BTIC shapes |
| `standalone-products-gates.*` | the standalone cash-settled families |

Nothing in the store is a second source of truth. Every conclusion is
reconstructible from the CME documents cited in the module comments, the
ledger, and `docs/schedules/sources.md`; the store is a cache of that
retrieval.

## What the gates changed since the handoff was written

Read this before opening any PR. Four of the handoff's own blockers are gone
and two of its dates are wrong.

| The handoff said | The gates found |
|---|---|
| U-4 (regular vs extended) blocks every shape | **Resolved.** `regular: &[]` for every BTIC, TAS, TAM, TACO and TMAC key in every era, on three affirmative channels plus one contrastive. Every Pre-Open is `order_entry` |
| U-8 (cross-zone) blocks eight shapes | **Premise falsified.** A key is not limited to one profile; four genuine cross-zone selectors already ship, and `MarketHoursKey::EurexFixedIncome` is the key-side precedent. Twelve shapes are affected, not eight |
| U-1 (EST/IPT/RVT contradiction) unadjudicated | **Resolved.** The 17:00 ET cells are the generic ClearPort envelope with its venue label dropped; seven venue-labelled live records and eleven dated archived states say 16:00 ET |
| TACO era 2 has "No notice found" | **Wrong.** Clearing Advisory 21-234 tabulates both grids and names the product codes; the weekly Globex notice carried the item in 13 consecutive issues |
| Five TAM launch rows keyed to trade dates | **All five are one day late.** These grids open 17:00 CT the previous evening, so LAW-NO-FABRICATED-DATES keys them to the local opening day |
| Gasoil's "current close 10:30 CT is undated" | **Wrong.** 11:30 a.m. ET *is* 10:30 CT; the 2016-12-19 notice is the cutover |
| RA1005-4's "(ET)" must be adjudicated | **Resolved as a typographic error.** The energy-TAS effective day is 2010-11-01, not 2010-10-25 — and no 2010 revision row should be opened at all |

## The 13-PR sequence

Each PR is independently mergeable and independently revertible.

| # | PR | Keys | Ready? |
|---|---|---|---|
| 1 | Fences before the families | — | **yes** |
| 2 | Metals TAS | `globex_gold_tas`, `globex_silver_tas`, `globex_copper_tas`, `globex_platinum_tas`, `globex_palladium_tas` | **yes** |
| 3 | Grain and livestock TAS | `globex_grains_tas`, `globex_livestock_tas` | **yes** |
| 4 | Cryptocurrency TAS | `globex_cryptocurrency_tas` | **yes**, after D-8's single retrieval |
| 5 | Energy and gasoil TAS | `globex_energy_tas`, `globex_gasoil_tas` | yes (D-5, D-6 taken) |
| 6 | Commodity-index cash families | `globex_bloomberg_commodity_index`, `globex_ftse_crb_index`, `globex_housing_index` | yes (D-7c taken) |
| 7 | Dairy, lumber, southern yellow pine | `globex_dairy`, `globex_lumber`, `globex_southern_yellow_pine` | yes |
| 8 | Santos soybeans, urea, Black Sea wheat | `globex_santos_soybeans`, `globex_urea_10_ton`, `globex_black_sea_wheat_cvb` | yes (D-7a, D-7b taken) |
| 9 | Equity, commodity-index and credit BTIC | `globex_equity_index_btic`, `globex_commodity_index_btic`, `globex_credit_index_btic` | yes |
| 10 | TACO and TMAC | `globex_equity_index_taco`, `globex_equity_index_tmac` | **no** — waits on #71 |
| 11 | Energy TAM, and the cross-zone pattern | `globex_energy_tam_london`, `globex_energy_tam_singapore`, `globex_energy_tam_shanghai` | **no** — waits on #77 |
| 12 | Metals TAM | `globex_gold_tam`, `globex_copper_tam` | **no** — waits on #72 |
| 13 | TOPIX BTIC, Nikkei BTIC, FX BTIC | `globex_topix_btic`, `globex_nikkei_btic`, `globex_fx_btic_euro` | **no** — waits on #71 |

PRs 9, 10 and 13 must each cite a resolution of the unexplained Saturday feed
block, [#76](https://github.com/SharurTrading/exchange-hours-rs/issues/76).
PRs 11–13 are the first seasonal keys, so the doc defect in
[#77](https://github.com/SharurTrading/exchange-hours-rs/issues/77) must be
fixed before PR 11 rather than with it.

## Decisions

Taken **2026-09-12 UTC** under the maintainer's standing delegation of
correctness and architectural calls:

- **D-1 — cross-zone representation: adopted.** Keep `tz: America::Chicago`,
  ship two `StaticHoursProfile` values per key, select with
  `reference_delta_seconds`, exactly as `ice_abu_dhabi.rs` and
  `eurex_fixed_income.rs` already do. Zero API change, zero serde change, no
  migration, no existing key's golden rows move. **Scoped to the ten shapes
  carrying CME's Friday-16:00 / Sunday-17:00 CT weekly close**; it is *not*
  exact for the two 24/7 crypto BTIC shapes, and it must not be spent on
  `America/New_York`-anchored shapes, which are exactly representable in
  Chicago local seconds.
- **D-2 — granularity: adopted.** Ten TAS keys and five TAM keys, decided on
  family semantics and history rather than on the envelope. Rows 10 and 22
  (CLT/HOT/RBT/BZT/BBT against NGT/NNT) stay **one** key: they differ only by a
  Pre-Open second the crate does not encode, never by an executable boundary.
- **D-5 — energy TAS: two eras**, on RA1104-4, which is an unconditional dated
  primary in session language. Do not split the key either way.
- **D-6 — map `MCT`** on published-group identity, sourced on `BZT`, with the
  reasoning written down. Documentation text, not a grid.
- **D-7 (a), (b), (c) — adopted as recommended**: CVB era 1 takes the day leg
  as `regular` and the overnight leg as `extended`, recorded as a convention
  rather than a source; SAS and MFV need no trade-date branch, only a test that
  asserts the default names the right day; BCOM and CCI serve the 16:45 → 08:15
  queue and record the narrowing, with the Friday-evening queue in the withheld
  list.

Referred to the maintainer, because both are **law edits to `AGENTS.md`** and
the repository's instruction is to keep to its law even where a case argues for
changing it:

- **D-3** — does "pending all relevant CFTC regulatory review periods" defeat
  an effective day?
  [#71](https://github.com/SharurTrading/exchange-hours-rs/issues/71). Gates
  PRs 10 and 13.
- **D-4** — does a labelled `closed@` feed event promote a marker instant to a
  session close?
  [#72](https://github.com/SharurTrading/exchange-hours-rs/issues/72). Gates
  PR 12; if it is decided **no**, PR 12 is dropped and GCD/HGF are recorded in
  `unsupported-families.md` naming their two `closed@` values.

**D-8** — whether the client-systems wiki's effective-day sentence scopes its
BTIC tables as well as its TAS table — is one retrieval, runs alongside PR 1,
and can cut either way: it either unblocks the crypto BTIC keys or forces PR 4
to be re-examined before it merges.

## Blocked keys

Six names are blocked, not rejected: the family exists and the grid is partly
or wholly sourced, and what is missing is an unconditional day or a
representation decision. All six are named in
`docs/schedules/unsupported-families.md` under "Blocked, not rejected", with
what blocks each.

| Key | Blocked on | Issue |
|---|---|---|
| `globex_cryptocurrency_btic_new_york` | U-15: the 24/7 cutover day is undated | [#73](https://github.com/SharurTrading/exchange-hours-rs/issues/73) |
| `globex_cryptocurrency_btic_london` | U-15, plus U-8 verdict N1 (the weekend block spans the DST transition), plus a FAQ-versus-feed conflict over the daily stop width | [#73](https://github.com/SharurTrading/exchange-hours-rs/issues/73) |
| `globex_cryptocurrency_btic_apac` | As London, measured on ABB across the US fall-back | [#73](https://github.com/SharurTrading/exchange-hours-rs/issues/73) |
| `globex_europe_index_btic` | No gate covered it; no launch certification; E3G's 68/EU → 68/EQ route change; the zone anchor is not fixed by the retrieved evidence | [#74](https://github.com/SharurTrading/exchange-hours-rs/issues/74) |
| `globex_ftse_china_50_btic` | No gate covered it; no launch certification. The zone is re-derived first-hand; FTC's own page's flat CT statement is wrong and is a conflict to record | [#74](https://github.com/SharurTrading/exchange-hours-rs/issues/74) |
| `globex_equity_index_btic_plus_taco_plus` | U-19: two CME documents give two different conditional launch days, which is a conflict rather than a condition. Rule 524 is also not a channel for this key | [#75](https://github.com/SharurTrading/exchange-hours-rs/issues/75) |

## Rejections recorded on evidence

Three groups have a launch, a root list and a measurable feed boundary, and no
CME document stating hours in session language. They are recorded in
`docs/schedules/unsupported-families.md` with the artifact that would close
each: **Treasury TAS** (`TNT`, `UBT`, `ZBT`, `ZFT`, `ZNS`, `ZTT`), **Dutch TTF
TAS** (`TAS`, `TTS`), and **commodity-index BTIC** (`AWT`, `BAT`, `BET`, `BGT`,
`BLT`, `BMT`, `BPT`, `BST`, `CCT`). `MCT` is a membership question, not a key —
see D-6.

## Conventions every PR follows

- **The registration surface**, all by hand, because `AGENTS.md` forbids
  generating the fences from `MarketHoursKey::ALL`: the enum row in the
  `market_hours_keys!` table; the `hours_for_market_hours_key` arm; the
  `session_profile` arm and its `FuturesSessionProfile` static; the module's
  `pub(crate) use` re-exports; `EXPECTED_MARKET_HOURS_KEY_NAMES` (bump the
  array length); `EXPECTED_MARKET_HOURS_KEYS`; `SUPPORTED_FAMILY_NAMES`; the
  ledger row; `CHANGELOG.md`; the README counts and prose; the regenerated
  golden file; and tests in a **new submodule** of
  `tests/futures_family_boundaries/`, never fattening the root.
- **The ledger row** carries a Basis, a `Reviewed on` date at or after the
  repository cutoff, and **no `|` anywhere inside a cell** — `row_cells` splits
  on `|` and the row must have exactly six cells.
- **`regular: &[]`** for every trade-type key, with each module comment naming
  **the channels its own empty `regular` rests on**. They differ per shape, and
  `globex_equity_index_btic_plus_taco_plus` is the one shape Rule 524 does not
  reach.
- **Date keying.** A grid that wraps from a 17:00 or 19:00 CT evening open is
  keyed to the **local opening day** — for a Monday trade date, the preceding
  Sunday. A non-wrapping day session is keyed to the day the grid first
  governs. This is where the U-8 gate got five rows wrong.
- **Every probe instant is stated in `America/Chicago` wall clock and
  converted**, so a DST slip in either direction fails rather than passing on a
  coincidence.
- **LAW-UTC-DATES.** Every date a PR records about the repository's own work —
  a ledger `Reviewed on`, a knowledge-bound row's date label, an amendment
  note, a CHANGELOG date, this plan's own date — is the UTC calendar date of
  that work, taken from `date -u`. Exchange effective days stay venue-local
  civil dates keyed to the opening day.
- **LAW-FOLLOW-UPS-ARE-ISSUES.** Every follow-up a PR names has a GitHub issue
  open before that PR merges, with the number cited where the follow-up is
  named.
- **Mutation-check every cutover fence** — move the revision one day, confirm
  the failure, restore, confirm green, and say in the PR that you did it.
- **Re-derive every tally from the merged ledger.** Every PR here touches the
  ledger, the README, `sources.md` and the changelog, so each merge breaks the
  next one's mergeability. That is expected: land one, rebase the rest, and
  never resolve a count conflict by picking a side — it happened with #51 and
  #49, and both branches were internally right. PR 1's fences turn that from a
  review habit into a red test.
- **Production files stay at or below 500 lines**, functions at or below 100.

## Residual risks that span the sequence

- **Feed-sourced phases are carried back to launch** for equity BTIC's
  Pre-Opens, the TAM onsets and the crypto BTIC grids. That is the carry-back
  convention working as designed, but the residual belongs beside **each**
  table, not stated once here.
- **The golden file renders only the aligned state of a seasonal key** — its
  instant, 2026-08-22 12:00 UTC, is an aligned day — so a wrong misaligned
  `close_ssm` is invisible there. The handwritten misaligned-state assertions
  are the only thing that catches it, and must not later be dropped as
  redundant.
- **Channel fragility.** `cmegroup.com` returns 403 at IP level to this
  machine, so much of the evidence was read as extracted text through a public
  reader in front of the cited URLs. `sources.md` already records that caveat
  for the event-contract keys; every PR here extends it, each ledger row should
  say which of its citations came through that channel, and a review date must
  never be advanced on the weaker channel alone.
