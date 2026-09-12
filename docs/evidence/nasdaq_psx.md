<!-- SPDX-License-Identifier: MIT-0 -->

# `nasdaq_psx` — evidence

- **Kind:** `Exchange`
- **Owner module:** [`equities.rs`](../../src/calendar/schedules/equities/us/equities.rs)<br>[`history.rs`](../../src/calendar/schedules/equities/us/history.rs)
- **Source sets:** [`US-NASDAQ-EQUITIES`](../schedules/sources.md#us-nasdaq-equities)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

Closed before the sourced 2010-10-08 launch; 09:00–17:00 at launch and the 2010-12-13 move to the current 08:00–17:00 grid are primary-sourced. **Systems in scope (2026-09-02):** the PSX equities matching system (08:00–17:00) is the envelope; the other Nasdaq PHLX LLC facility, PHLX Options, is `nasdaq_phlx`. No discrepancy.

## Revision rows

- 2010-10-08 — T1 — Nasdaq Equity Trader Alert 2010-56 — PSX production launch on 09:00–17:00 ET system hours.
- 2010-12-13 — T1 — SEC SR-Phlx-2010-172 — the 08:00 ET opening, the implementation date the filing names.

## Sources

Retrieval dates: these sources were last opened on the row's reviewed-on date
(2026-08-22, UTC); per-source retrieval dates were not recorded before the
2026-09-12 migration and are added as each source is re-verified.

- <https://listingcenter.nasdaq.com/rulebook/phlx/rules/phlx-psx-legacy-3000> — the PSX legacy 3000-series rules behind the current 08:00–17:00 system hours.
- <https://www.nasdaqtrader.com/TraderNews.aspx?id=ETA2010-56> — Nasdaq Equity Trader Alert 2010-56, the 2010-10-08 launch.
- <https://www.sec.gov/files/rules/sro/phlx/2010/34-63492.pdf> — SR-Phlx-2010-172, which explicitly identifies 2010-12-13 as the implementation date for the 08:00 opening.

## Gaps and residual risks

- **None open.** The profile is `CLOSED` before the operator-dated launch, so
  nothing is carried below it and the horizon is `—`.
- **System coverage (2026-09-02).** No discrepancy. The other Nasdaq PHLX LLC
  facility, PHLX Options, is `nasdaq_phlx`.

> Shared module. The narrative for
> [`equities.rs`](../../src/calendar/schedules/equities/us/equities.rs) and [`history.rs`](../../src/calendar/schedules/equities/us/history.rs)
> lives in [`nasdaq`](nasdaq.md#module-narrative-moved-from-srccalendarschedulesequitiesusequitiesrs-on-2026-09-12-utc) and [`nasdaq`](nasdaq.md#module-narrative-moved-from-srccalendarschedulesequitiesushistoryrs-on-2026-09-12-utc).
> Sibling identities: [`nasdaq_bx`](nasdaq_bx.md), [`memx_eq`](memx_eq.md), [`miax_pearl_eq`](miax_pearl_eq.md).
