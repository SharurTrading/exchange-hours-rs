<!-- SPDX-License-Identifier: MIT-0 -->

# `ltse` — evidence

- **Kind:** `Exchange`
- **Owner module:** [`independent.rs`](../../src/calendar/schedules/equities/us/independent.rs)
- **Source sets:** [`US-LTSE`](../schedules/sources.md#us-ltse)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

Closed before first production symbols on 2020-08-28, then 08:00–17:00 ET with 09:30–16:00 regular. **Systems in scope (2026-09-02):** one equities matching system covering all three published sessions. No discrepancy.

## Revision rows

- 2020-08-28 — T1 — SEC 34-89766 — LTSE commences operations on 08:00–17:00 ET around a 09:30–16:00 Regular Market Session.

## Sources

Retrieval dates: these sources were last opened on the row's reviewed-on date
(2026-08-22, UTC); per-source retrieval dates were not recorded before the
2026-09-12 migration and are added as each source is re-verified.

- <https://ltse.com/trading/trading-schedule> — LTSE's trading schedule, which publishes the 08:00–17:00 ET System Hours and their three phases.
- <https://www.sec.gov/rules/sro/ltse/2020/34-88515.pdf> — the SEC's operative phase-rule approval.
- <https://www.sec.gov/rules/sro/ltse/2020/34-89766.pdf> — the production-launch order recording 2020-08-28 as the first production-symbol day.

## Gaps and residual risks

- **None open.** The profile is `CLOSED` before the sourced first
  production-symbol day and the operator's envelope has not changed since, so
  nothing is carried and the horizon is `—`.
- **System coverage (2026-09-02).** No discrepancy. One equities matching system
  covering all three published sessions.

## Module narrative (moved from src/calendar/schedules/equities/us/independent.rs on 2026-09-12 UTC)

LTSE's trading schedule defines an 08:00–09:30 Early Trading Session,
09:30–16:00 Regular Market Session, and 16:00–17:00 Late Trading Session.
The SEC order records that LTSE commenced operations on 2020-08-28.
https://ltse.com/trading/trading-schedule
https://www.sec.gov/rules/sro/ltse/2020/34-89766.pdf
https://www.sec.gov/rules/sro/ltse/2020/34-88515.pdf

The SEC's 2026 order confirms that 24X commenced trading on 2025-10-14 and
that its current sessions are 04:00–09:30, 09:30–16:00, and 16:00–20:00 ET.
The condition-dependent 21:00–04:00 overnight session is not operative and
therefore has no runtime revision here.
https://www.sec.gov/files/rules/exorders/2026/34-106061.pdf
https://equities.24exchange.com/api/media/download/68e43b4830a49c75a17a8134

TXSE's production alerts distinguish its July 6–9 test-symbol activity from
the first NMS-stock production rollout on 2026-07-10. The current exchange
schedule accepts orders from 08:00 through its 17:00 late-session close. The
operator names 08:00–09:30 a Pre-Market session rather than an order-entry
phase and does not document a separate unmatchable acceptance window, so both
off-core legs stay Extended.
https://www.txse.com/alerts/6a5e8e60-8753-4eac-906d-ecbbf8682df9
https://www.txse.com/alerts/txse-production-launch-and-market-activation
