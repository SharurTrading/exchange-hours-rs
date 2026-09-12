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

Retrieval dates: these sources were last opened on the row's reviewed-on date
(2026-08-22, UTC); per-source retrieval dates were not recorded before the
2026-09-12 migration and are added as each source is re-verified.

- <https://www.miaxglobal.com/company/markets/us-equities> — the official Pearl Equities market history, which records the 2020-09-29 live launch.
- <https://www.miaxglobal.com/sites/default/files/circular-files/MIAX_Pearl_Equities_RC_2025_02_0.pdf> — Regulatory Circular 2025-02, the 2025-02-20 early/late expansion.
- <https://www.miaxglobal.com/alert/2025/01/17/miax-pearl-equities-changes-expand-trading-hours-adopting-early-and-late> — the operator alert announcing the same expansion.
- <https://www.miaxglobal.com/markets/us-equities/pearl-equities/trade-hours-calendar> — the operator's *Trade Hours of Operation* table, which publishes the 03:30 Live Order Window.

## Gaps and residual risks

- **Raised in review of the ledger-reshape PR (#87), 2026-09-12 — the omitted 03:30–04:00 order-entry period needs its own PR.** The operator's trade-hours table opens a Live Order Window at 03:30 and the profile carries no `order_entry` phase, so the envelope under-reports order acceptance by thirty minutes; the bullet above records it, and it is discrepancy #5 on the system-coverage list. Closing it is a **schedule change**, not a documentation change: it adds an `order_entry` rule to the MIAX Pearl Equities profile, needs the operator artifact at T1 or T2, needs a knowledge-bound onset row because the window is published only on a mutable operator page, and needs its own public-surface tests and CHANGELOG entry. Under LAW-BOUNDED-WORK that is one pull request sized to a working day, and it is not this one: the reshape PR moved text out of the owner modules and changed no schedule rule, revision row, profile or routing. Closing condition: that separate PR. Dormant identity, so recorded here rather than opened as an issue (LAW-FOLLOW-UPS-ARE-ISSUES).
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
