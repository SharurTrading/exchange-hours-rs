<!-- SPDX-License-Identifier: MIT-0 -->

# `sgx` — evidence

- **Kind:** `MarketHoursKey`
- **Owner module:** [`sgx.rs`](../../src/calendar/schedules/futures/international/sgx.rs)
- **Source sets:** [`APAC-SGX-DERIVATIVES`](../schedules/sources.md#apac-sgx-derivatives)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

Three-Month SORA Futures, including closed-before-launch history through the dated key API.

## Revision rows

- 2024-07-29 — T1 — SGX launch announcement 2024-07-29 — Three-Month SORA Futures launch: T session 07:25–17:55 with a 17:55–18:00 closing routine, T+1 session 18:15–05:15, and the 07:10 and 18:05 Pre-Opening/Non-Cancel routines.

## Sources

Retrieval dates: these sources were last opened on the row's reviewed-on date
(2026-08-22, UTC); per-source retrieval dates were not recorded before the
2026-09-12 migration and are added as each source is re-verified.

- <https://links.sgx.com/1.0.0/corporate-announcements/LG3YO2RZCGZ92J0B/359e83de092b9d70d54305133c92a82e16f676fc43ef4aa06a6976d8bc771fdf> — SGX launch announcement giving the day-level 2024-07-29 launch.
- <https://www.sgx.com/derivatives/products/stir-products?cc=SORA> — live product specification publishing the complete T and T+1 opening, non-cancel, continuous, pre-close and overnight routine.
- <https://api2.sgx.com/sites/default/files/2026-01/SGX%20Calendar%202026_2.pdf> — SGX derivatives calendar 2026, independently retaining the same continuous windows.
- <https://rulebook.sgx.com/rulebook/415-trading-hours-opening-and-closing-routines-and-closing-range> — SGX Rule 4.1.5, trading hours, opening and closing routines and closing range.

## Gaps and residual risks

- The key names a sourced schedule family, not a venue-wide derivatives clock. A venue-keyed default is not permission to use this clock for every SGX product; the consumer's map decides (AGENTS.md, *Product-neutral family selection*).
- The key shares one timeline with the `sgx` exchange row, so any correction to either must be made once, in `futures/international/sgx.rs`, and reviewed against both rows.

## Shared module

> Shared module. [`sgx.rs`](../../src/calendar/schedules/futures/international/sgx.rs) also carries [`sgx`](sgx.md). The single copy of this module's narrative belongs in [`sgx`](sgx.md); the module still holds that narrative in source, and it moves in the migration that empties the fence's narrative-debt list.
