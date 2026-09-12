<!-- SPDX-License-Identifier: MIT-0 -->

# `finra_trf_chicago` — evidence

- **Kind:** `Exchange`
- **Owner module:** [`trfs.rs`](../../src/calendar/schedules/equities/us/trfs.rs)
- **Source sets:** [`US-FINRA-TRF`](../schedules/sources.md#us-finra-trf)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

Closed before the sourced 2018-09-10 facility launch (test securities only through 2018-09-21); the 2026-03-30 revision is sourced. The SIP-conditional overnight expansion remains unencoded.

## Revision rows

- 2018-09-10 — T1 — FINRA/Nasdaq TRF Chicago technical notice — the facility commences operation on 08:00–20:00 ET.
- 2026-03-30 — T1 — FINRA Notice 25-15 — system hours move from 08:00–20:00 to 04:00–20:00 ET.

## Sources

No per-URL retrieval date is recorded in this repository; every link below
was read at or before the row's `Reviewed on` date in the ledger.

- <https://www.finra.org/filing-reporting/trade-reporting-facility-trf> — the FINRA TRF hub.
- <https://www.finra.org/rules-guidance/rulebooks/finra-rules/6380a> — Rule 6380A, which requires the outside-RTH modifier.
- <https://www.finra.org/rules-guidance/rulebooks/finra-rules/6380b> — Rule 6380B, the same requirement for the other facility.
- <https://www.finra.org/rules-guidance/notices/25-15> — Regulatory Notice 25-15, which identifies all three active TRFs, states that system hours changed from 08:00–20:00 to 04:00–20:00 ET on 2026-03-30, and classifies 09:30–16:00 as Regular Trading Hours.
- <https://www.finra.org/sites/default/files/2026-07/SR-FINRA-2026-015.pdf> — SR-FINRA-2026-015, the Sunday-through-Friday expansion whose implementation follows the SIP rollout.
- <https://www.finra.org/filing-reporting/trf/technical-notices/reminder-finranasdaq-trf-chicago> — FINRA's Chicago launch notice, which dates commencement to 2018-09-10 and records that test securities only were accepted through 2018-09-21.

## Gaps and residual risks

- **None below the launch.** The profile is `CLOSED` before the sourced
  2018-09-10 facility launch, so nothing is carried and the horizon is `—`. All
  NMS stocks became reportable on 2018-09-24; the test-security interval is
  inside the launched facility's own window and is not modelled separately.
- **Watch item, not a gap.** The SIP-conditional Sunday-through-Friday expansion
  remains unencoded (LAW-NO-FABRICATED-DATES).
- **Classification note.** A TRF is a reporting facility, not a matching engine
  and not an order book: it never accepts orders, so `order_entry` is empty
  because the concept does not apply. Every modelled window is one in which an
  executed trade is reported and disseminated.

> Shared module. The narrative for
> [`trfs.rs`](../../src/calendar/schedules/equities/us/trfs.rs)
> lives in [`finra_trf_carteret`](finra_trf_carteret.md#module-narrative-moved-from-srccalendarschedulesequitiesustrfsrs-on-2026-09-12-utc).
> Sibling identities: [`finra_trf_nyse`](finra_trf_nyse.md).
