<!-- SPDX-License-Identifier: MIT-0 -->

# `txse` — evidence

- **Kind:** `Exchange`
- **Owner module:** [`independent.rs`](../../src/calendar/schedules/equities/us/independent.rs)
- **Source sets:** [`US-TXSE`](../schedules/sources.md#us-txse)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

Closed through the July 6–9 non-clearing test-symbol period; live NMS production begins 2026-07-10 with an 08:00–17:00 ET envelope. **Systems in scope (2026-09-02):** one equities matching system; the separately published UAT environment (08:00–17:00) is a test facility and is excluded. No discrepancy.

## Revision rows

- 2026-07-10 — T1 — TXSE production launch alert — first live NMS production trading on an 08:00–17:00 ET envelope.

## Sources

Row review: 2026-08-22 (UTC) is the date the ledger row was last reviewed as a
whole. Per-source retrieval dates were not recorded before the 2026-09-12
migration; where a later targeted review, capture or document date is recorded
beside a source below, that date governs for that source, and dates are added
as each source is re-verified.

- <https://www.txse.com/regulations> — the operator's regulation center and current regulatory alerts.
- <https://www.txse.com/alerts/6a5e8e60-8753-4eac-906d-ecbbf8682df9> — the production launch schedule, which distinguishes the July 6–9 non-clearing test symbols from first live NMS trading on 2026-07-10.
- <https://www.txse.com/alerts/txse-production-launch-and-market-activation> — the market-activation notice, which records the same phase table.

## Gaps and residual risks

- **None open.** The profile is `CLOSED` through the July 2026 non-clearing
  test-symbol period and before the sourced first live NMS production day, so
  nothing is carried and the horizon is `—`.
- **Classification note.** The operator names 08:00–09:30 a Pre-Market session
  rather than an order-entry phase and documents no separate unmatchable
  acceptance window, so both off-core legs stay `extended`.
- **System coverage (2026-09-02).** No discrepancy. One equities matching
  system; the separately published UAT environment (08:00–17:00) is a test
  facility and is excluded.

> Shared module. The narrative for
> [`independent.rs`](../../src/calendar/schedules/equities/us/independent.rs)
> lives in [`ltse`](ltse.md#module-narrative-moved-from-srccalendarschedulesequitiesusindependentrs-on-2026-09-12-utc).
> Sibling identities: [`24x`](24x.md).
