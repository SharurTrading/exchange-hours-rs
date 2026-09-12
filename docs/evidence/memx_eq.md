<!-- SPDX-License-Identifier: MIT-0 -->

# `memx_eq` — evidence

- **Kind:** `Exchange`
- **Owner module:** [`equities.rs`](../../src/calendar/schedules/equities/us/equities.rs)<br>[`history.rs`](../../src/calendar/schedules/equities/us/history.rs)
- **Source sets:** [`US-MEMX`](../schedules/sources.md#us-memx)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

Closed before the sourced 2020-09-21 live launch; the 2020-10-05 post-market shortening, 2023-02-01 restoration, and 2025-05-19 early-open change are date-aware and primary-sourced. **Systems in scope (2026-09-02):** the MEMX equities matching system (04:00–20:00 on the operator hours table) is the envelope; MEMX Options is `memx_options`. No discrepancy.

## Revision rows

- 2020-09-21 — T1 — MEMX Day 1 retrospective — live launch on 07:00–20:00 ET around the core session.
- 2020-10-05 — T1 — MEMX trader alert 20-06 — Post-Market Session shortened from 20:00 to 17:00 ET.
- 2023-02-01 — T1 — MEMX trader alert 23-04 — the 20:00 ET post-market close restored.
- 2025-05-19 — T1 — MEMX retrospective 2025-06-06 — the 04:00 ET pre-market open begins.

## Sources

No per-URL retrieval date is recorded in this repository; every link below
was read at or before the row's `Reviewed on` date in the ledger.

- <https://info.memxtrading.com/market-hours-and-holiday-schedule/> — MEMX market hours and holiday schedule, the current 04:00–20:00 envelope.
- <https://info.memxtrading.com/regulation/memx-rules/> — the current MEMX rulebook.
- <https://memx.com/insights/day-1> — the operator's day-one record of the 2020-09-21 live launch.
- <https://info.memxtrading.com/trader-alert-20-06-memx-market-hours-change/> — trader alert 20-06, the 2020-10-05 move to a 17:00 post-market close.
- <https://info.memxtrading.com/trader-alert-23-04-memx-trading-hours-change/> — trader alert 23-04, the 2023-02-01 restoration of the 20:00 close.
- <https://www.sec.gov/files/rules/sro/memx/2023/34-96773.pdf> — the SEC filing behind the 2023 restoration.
- <https://memx.com/insights/pre-market-share-gains-and-new-options-active-risk-feature> — the 2025-06-06 retrospective, which identifies 2025-05-19 as the actual launch of the 04:00 pre-market.

## Gaps and residual risks

- **None open.** The profile is `CLOSED` before the operator-dated launch and
  every later move is dated by the operator, so nothing is carried and the
  horizon is `—`.
- **Source-conflict note.** The earlier rule filing proposed a March 2025 date
  for the 04:00 pre-market; the exchange's own stated production launch,
  2025-05-19, is the operative boundary, and the filing's proposed day is not
  encoded.
- **System coverage (2026-09-02).** No discrepancy. MEMX Options is
  `memx_options`.

> Shared module. The narrative for
> [`equities.rs`](../../src/calendar/schedules/equities/us/equities.rs) and [`history.rs`](../../src/calendar/schedules/equities/us/history.rs)
> lives in [`nasdaq`](nasdaq.md#module-narrative-moved-from-srccalendarschedulesequitiesusequitiesrs-on-2026-09-12-utc) and [`nasdaq`](nasdaq.md#module-narrative-moved-from-srccalendarschedulesequitiesushistoryrs-on-2026-09-12-utc).
> Sibling identities: [`nasdaq_bx`](nasdaq_bx.md), [`nasdaq_psx`](nasdaq_psx.md), [`miax_pearl_eq`](miax_pearl_eq.md).
