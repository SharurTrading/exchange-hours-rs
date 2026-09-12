<!-- SPDX-License-Identifier: MIT-0 -->

# `miax_pearl_eq` — evidence

- **Kind:** `Exchange`
- **Owner module:** [`equities.rs`](../../src/calendar/schedules/equities/us/equities.rs)<br>[`history.rs`](../../src/calendar/schedules/equities/us/history.rs)
- **Source sets:** [`US-MIAX`](../schedules/sources.md#us-miax)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

Closed before the sourced 2020-09-29 live launch; the 2025-02-20 early/late expansion is also sourced. **Systems in scope (2026-09-02):** MIAX Pearl Options is `miax_pearl_options`, but the operator's trade-hours table opens a Live Order Window at 03:30 — “Firms can send MEO and FIX orders” — half an hour before the 04:00 Early Trading Session, and this profile carries no `order_entry` phase. On the discrepancy list as an order-entry omission; the 01:30 market-data start and the 03:00 connectivity step stay excluded.

## Revision rows

- 2020-09-29 — T1 — MIAX Pearl Equities launch notice — live launch carrying Regular Trading Hours only.
- 2025-02-20 — T1 — MIAX Pearl Regulatory Circular 2025-02 — the 04:00–09:30 Early and 16:00–20:00 Late Trading Sessions become available.

## Sources

No per-URL retrieval date is recorded in this repository; every link below
was read at or before the row's `Reviewed on` date in the ledger.

- <https://www.miaxglobal.com/company/markets/us-equities> — the official Pearl Equities market history, which records the 2020-09-29 live launch.
- <https://www.miaxglobal.com/sites/default/files/circular-files/MIAX_Pearl_Equities_RC_2025_02_0.pdf> — Regulatory Circular 2025-02, the 2025-02-20 early/late expansion.
- <https://www.miaxglobal.com/alert/2025/01/17/miax-pearl-equities-changes-expand-trading-hours-adopting-early-and-late> — the operator alert announcing the same expansion.
- <https://www.miaxglobal.com/markets/us-equities/pearl-equities/trade-hours-calendar> — the operator's *Trade Hours of Operation* table, which publishes the 03:30 Live Order Window.

## Gaps and residual risks

- **Order-entry omission (discrepancy #5, 2026-09-02).** The operator's
  trade-hours table opens a Live Order Window at 03:30 — "Firms can send MEO and
  FIX orders" — thirty minutes before the 04:00 Early Trading Session, and this
  profile carries no `order_entry` phase. The envelope therefore under-reports
  order acceptance; it never over-reports it. Closing condition: an envelope
  amendment adding the `order_entry` leg. Its onset day is published only on a
  mutable operator page, so the amendment must expect a knowledge-bound onset,
  exactly like the US options queues.
- **Excluded steps.** The 01:30 start of System Hours is market-data
  dissemination and the 03:00 step is connectivity only ("Firms can connect");
  both stay outside the envelope.
- **None below the launch.** The profile is `CLOSED` before the sourced launch,
  so nothing is carried and the horizon is `—`.

> Shared module. The narrative for
> [`equities.rs`](../../src/calendar/schedules/equities/us/equities.rs) and [`history.rs`](../../src/calendar/schedules/equities/us/history.rs)
> lives in [`nasdaq`](nasdaq.md#module-narrative-moved-from-srccalendarschedulesequitiesusequitiesrs-on-2026-09-12-utc) and [`nasdaq`](nasdaq.md#module-narrative-moved-from-srccalendarschedulesequitiesushistoryrs-on-2026-09-12-utc).
> Sibling identities: [`nasdaq_bx`](nasdaq_bx.md), [`nasdaq_psx`](nasdaq_psx.md), [`memx_eq`](memx_eq.md).
