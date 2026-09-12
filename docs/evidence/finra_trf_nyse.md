<!-- SPDX-License-Identifier: MIT-0 -->

# `finra_trf_nyse` — evidence

- **Kind:** `Exchange`
- **Owner module:** [`trfs.rs`](../../src/calendar/schedules/equities/us/trfs.rs)
- **Source sets:** [`US-FINRA-TRF`](../schedules/sources.md#us-finra-trf)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

Reporting facility; the 2026-03-30 04:00–20:00 revision is sourced. The announced overnight expansion remains unencoded while its date depends on the SIP rollout.

## Revision rows

- 2026-03-30 — T1 — FINRA Notice 25-15 — system hours move from 08:00–20:00 to 04:00–20:00 ET.

## Sources

Row review: 2026-08-22 (UTC) is the date the ledger row was last reviewed as a
whole. Per-source retrieval dates were not recorded before the 2026-09-12
migration; where a later targeted review, capture or document date is recorded
beside a source below, that date governs for that source, and dates are added
as each source is re-verified.

- <https://www.finra.org/filing-reporting/trade-reporting-facility-trf> — the FINRA TRF hub.
- <https://www.finra.org/rules-guidance/rulebooks/finra-rules/6380a> — Rule 6380A, which requires the outside-RTH modifier.
- <https://www.finra.org/rules-guidance/rulebooks/finra-rules/6380b> — Rule 6380B, the same requirement for the other facility.
- <https://www.finra.org/rules-guidance/notices/25-15> — Regulatory Notice 25-15, which identifies all three active TRFs, states that system hours changed from 08:00–20:00 to 04:00–20:00 ET on 2026-03-30, and classifies 09:30–16:00 as Regular Trading Hours.
- <https://www.finra.org/sites/default/files/2026-07/SR-FINRA-2026-015.pdf> — SR-FINRA-2026-015, the Sunday-through-Friday expansion whose implementation follows the SIP rollout.

## Gaps and residual risks

- **Horizon (carried interval).** The 08:00–20:00 grid below 2026-03-30 is
  carried, not separately sourced: Regulatory Notice 25-15 states the outgoing
  value when it dates the change, and no earlier FINRA artifact in the reviewed
  set states the facility's system hours at a day level. The notice's own issue
  date is not recorded here, so the horizon is keyed to the effective day it
  states, 2026-03-30. Closing condition: a FINRA notice or rule filing that
  states the 08:00 system-hours open on a floor-era day.
- **Watch item, not a gap.** The announced Sunday-through-Friday expansion stays
  unencoded while its anticipated implementation day is conditional on the SIP
  rollout (LAW-NO-FABRICATED-DATES).
- **Classification note.** A TRF is a reporting facility, not a matching engine
  and not an order book: it never accepts orders, so `order_entry` is empty
  because the concept does not apply. Every modelled window is one in which an
  executed trade is reported and disseminated, so a print occurs inside it and
  all of them stay in `extended`.

> Shared module. The narrative for
> [`trfs.rs`](../../src/calendar/schedules/equities/us/trfs.rs)
> lives in [`finra_trf_carteret`](finra_trf_carteret.md#module-narrative-moved-from-srccalendarschedulesequitiesustrfsrs-on-2026-09-12-utc).
> Sibling identities: [`finra_trf_chicago`](finra_trf_chicago.md).
