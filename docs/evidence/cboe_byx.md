<!-- SPDX-License-Identifier: MIT-0 -->

# `cboe_byx` — evidence

- **Kind:** `Exchange`
- **Owner module:** [`cboe.rs`](../../src/calendar/schedules/equities/us/cboe.rs)
- **Source sets:** [`US-CBOE-EQUITIES`](../schedules/sources.md#us-cboe-equities)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

Closed before the 2010-10-15 launch; the exact 2014-12-01 06:00 queue onset, the 2016-05-23 matching start (the 06:00–08:00 queue narrowed to 06:00–07:00 as the 07:00 hour became tradeable), and 2018-08-27 20:00 close are date-aware. **Systems in scope (2026-09-02):** one equities matching system, envelope 06:00–20:00 on the operator hours table; this SRO operates no options facility. No discrepancy.

## Revision rows

- 2010-10-15 — T1 — SEC 34-63097 — BYX launch on 08:00–17:00 ET.
- 2014-12-01 — T1 — SEC 34-73744 — the 06:00 order-acceptance queue opens ahead of 08:00 matching.
- 2016-05-23 — T1 — Bats release note 2016 7am matching — matching and routing start at 07:00, so the queue narrows to 06:00–07:00.
- 2018-08-27 — T1 — Bats release note 2018 8pm post-market — the post-market close extends to 20:00.

## Sources

Row review: 2026-08-24 (UTC) is the date the ledger row was last reviewed as a
whole. Per-source retrieval dates were not recorded before the 2026-09-12
migration; where a later targeted review, capture or document date is recorded
beside a source below, that date governs for that source, and dates are added
as each source is re-verified.

- <https://www.sec.gov/files/rules/sro/byx/2010/34-63097.pdf> — the SEC-filed BYX launch record, 2010-10-15.
- <https://cdn.cboe.com/resources/fee_schedule/2010/BATS-Announces-BATS-Y-Exchange-BYX-Pricing-Effective-October-15-2010-and-New-B2B-TRIM-SLIM-and-One-Under-Routing-Strategies.pdf> — the operator's own launch notice for the same day.
- <https://www.sec.gov/rules/sro/byx/2014/34-73744.pdf> — SEC 34-73744, the BYX 06:00 queue onset of 2014-12-01.
- <https://cdn.cboe.com/resources/release_notes/2014/BATS-BYX-Exchange-and-BZX-Exchange-Feature-Release-Postponed-Until-December-2014.pdf> — the operator's final 2014 queue rollout notice.
- <https://cdn.cboe.com/resources/release_notes/2016/Update-Bats-to-Begin-Equity-Order-Matching-and-Routing-at-7-am-ET.pdf> — the 2016 release note.
- <https://cdn.cboe.com/resources/release_notes/2018/BZX-Exchange-and-BYX-Exchange-to-Extend-Post-Market-Session-Hours-to-8PM-ET.pdf> — the 2018 close-extension notice.

## Gaps and residual risks

- **None open.** The profile is `CLOSED` before the sourced launch and every
  later move is dated, so nothing is carried and the horizon is `—`.
- **System coverage (2026-09-02).** No discrepancy. One equities matching
  system, envelope 06:00–20:00 on the operator hours table; this SRO operates no
  options facility.

> Shared module. The narrative for
> [`cboe.rs`](../../src/calendar/schedules/equities/us/cboe.rs)
> lives in [`cboe_bzx`](cboe_bzx.md#module-narrative-moved-from-srccalendarschedulesequitiesuscboers-on-2026-09-12-utc).
> Sibling identities: [`cboe_edga`](cboe_edga.md), [`cboe_edgx`](cboe_edgx.md).
