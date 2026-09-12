<!-- SPDX-License-Identifier: MIT-0 -->

# `sgx` — evidence

- **Kind:** `Exchange`
- **Owner module:** [`sgx.rs`](../../src/calendar/schedules/futures/international/sgx.rs)
- **Source sets:** [`APAC-SGX-DERIVATIVES`](../schedules/sources.md#apac-sgx-derivatives)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

Three-Month SORA Futures only: closed before the sourced 2024-07-29 launch; current T/T+1 auctions, continuous sessions, and the 18:00–18:05 gap are primary-sourced.

## Revision rows

- 2024-07-29 — T1 — SGX launch announcement 2024-07-29 — Three-Month SORA Futures launch: T session 07:25–17:55 with a 17:55–18:00 closing routine, T+1 session 18:15–05:15, and the 07:10 and 18:05 Pre-Opening/Non-Cancel routines.

## Sources

Row review: 2026-08-22 (UTC) is the date the ledger row was last reviewed as a
whole. Per-source retrieval dates were not recorded before the 2026-09-12
migration; where a later targeted review, capture or document date is recorded
beside a source below, that date governs for that source, and dates are added
as each source is re-verified.

- <https://links.sgx.com/1.0.0/corporate-announcements/LG3YO2RZCGZ92J0B/359e83de092b9d70d54305133c92a82e16f676fc43ef4aa06a6976d8bc771fdf> — SGX launch announcement giving the day-level 2024-07-29 launch.
- <https://www.sgx.com/derivatives/products/stir-products?cc=SORA> — live product specification publishing the complete T and T+1 opening, non-cancel, continuous, pre-close and overnight routine.
- <https://api2.sgx.com/sites/default/files/2026-01/SGX%20Calendar%202026_2.pdf> — SGX derivatives calendar 2026, independently retaining the same continuous windows.
- <https://rulebook.sgx.com/rulebook/415-trading-hours-opening-and-closing-routines-and-closing-range> — SGX Rule 4.1.5, trading hours, opening and closing routines and closing range.

## Gaps and residual risks

- The row is scoped to Three-Month SORA Futures only. Other SGX derivatives remain contract-specific and must not borrow this clock; the consumer's family map decides (AGENTS.md, *Product-neutral family selection*).
- The two opening routines are Pre-Opening/Non-Cancel windows under SGX Rule 4.1.5: they collect orders and compute an indicative opening price without matching, so they are `order_entry`. The opening match itself falls on the session-open instant that already begins the `regular` window, so nothing tradeable is lost.
- The closing routine that follows the T session ends in a match at a single closing price, so it is `extended` rather than `order_entry`.
