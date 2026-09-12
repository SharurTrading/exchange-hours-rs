<!-- SPDX-License-Identifier: MIT-0 -->

# `bursa_malaysia` — evidence

- **Kind:** `Exchange`
- **Owner module:** [`bursa.rs`](../../src/calendar/schedules/equities/apac/bursa.rs)
- **Source sets:** [`APAC-BURSA`](../schedules/sources.md#apac-bursa)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

The pre-audit-floor 2009 v2 manual and dated 2011, 2012, 2021, and 2024 manuals support the unchanged normal-lot grid through the 2010-on window.

## Revision rows

None. bursa.rs holds a single static profile with no dated revision row.

## Sources

Row review: 2026-08-22 (UTC) is the date the ledger row was last reviewed as a
whole. Per-source retrieval dates were not recorded before the 2026-09-12
migration; where a later targeted review, capture or document date is recorded
beside a source below, that date governs for that source, and dates are added
as each source is re-verified.

- <https://www.bursamalaysia.com/sites/5bb54be15f36ca0af339077a/assets/5bb55ac75f36ca0c3028d8e7/Amended_Participating_Organisations__Trading_Manual.pdf> — Participating Organisations' Trading Manual v2.0, dated 2009-10-26 and in force at the January-2010 audit floor.
- <https://www.bursamalaysia.com/sites/5bb54be15f36ca0af339077a/assets/5cda944139fba22dab508ab1/rules_bms_cir_rr2_110411.pdf> — 2011 rules circular (v3.0 era).
- <https://www.bursamalaysia.com/sites/5bb54be15f36ca0af339077a/assets/5bb55ab65f36ca0c3028d8c2/1._Amendments_to_the_Rules_of_Bursa_Malaysia_Securities_Berhad_in_relation_to_Market_Making_and_Margin_Financing.pdf> — 2012 amendments (v5.0 era).
- <https://www.bursamalaysia.com/sites/5d809dcf39fba22790cad230/assets/60b1b8e85b711a63ee7f1395/POs_Trading_Manual_v28n_29.pdf> — Trading Manual v29.0 (2021).
- <https://www.bursamalaysia.com/sites/5d809dcf39fba22790cad230/assets/65ead6cbe6414a1e16de8b8e/POs_Trading_Manual_v36_4_March_2024.pdf> — Trading Manual v36.0 (4 March 2024), the current table.

## Gaps and residual risks

- **order-entry classification withheld** — each of the three non-continuous windows (08:30–09:00, 14:00–14:30 and 16:45–17:00) is a combined order-entry/call phase whose call leg matches and prints. The crate has no reachable primary source for the sub-phase boundary at which the call begins, so splitting would be a guess and the whole window stays tradeable `extended`. Closing condition: a Bursa document stating the call-phase start inside each window. Dormant identity, so the gap is recorded here rather than opened as an issue.
- The true lunch closure is 12:30–14:00 and is modelled as a gap, not a rule (AGENTS.md, *Lunch breaks are gaps, not rules*).
- The intervening amendment register between the dated manuals contains no in-scope clock change; the module records no dated revision row for that reason, and a sourced revision later replaces the static profile with a real timeline row without any routing change.
