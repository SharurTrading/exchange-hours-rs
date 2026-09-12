<!-- SPDX-License-Identifier: MIT-0 -->

# `always_open` — evidence

- **Kind:** `MarketHoursKey`
- **Owner module:** [`profiles.rs`](../../src/calendar/futures_profile/profiles.rs)
- **Source sets:** [`SYNTHETIC-24X7`](../schedules/sources.md#synthetic-24x7)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

Library-defined UTC 24×7 profile. It has no final daily close, so its key calendar returns `None` for `trade_date`.

## Revision rows

None. `profiles.rs` holds a single static profile with no dated revision row.

## Sources

None. The profile is deterministic library policy rather than a venue schedule, so it rests on no operator document.

## Gaps and residual risks

- **not a venue** — this key is a deliberate fallback and must never be used as a stand-in for an unsourced venue schedule. A caller that reaches it is asking for library policy, not for an exchange's hours.

## Module narrative (moved from src/calendar/futures_profile/profiles.rs on 2026-09-12 UTC)

`profiles.rs` is the shared fixed-current profile table behind `session_profile` for
every `MarketHoursKey`, so the block below is about the five metals Trading at
Settlement profiles it builds rather than about `always_open` itself; the same
substance is filed under each TAS key's own evidence file.

The five metals Trading at Settlement books. `regular` is empty on all of
them: the TAS shape is Globex-only in CME's own Rule 524 route enumeration,
its ProductSlate records carry no Floor venue component, and the
ContractSpecs "Open Outcry:" label CME does publish is never attached to a
TAS hours line. Each opens 17:00 CT Sunday through Thursday and wraps to its
own close, with no Friday-evening reopen, so every inter-trade-date gap
exceeds four hours and is `Closed` rather than `Maintenance`.

COMEX gold, silver and copper share the queue `metals_tas.rs` serves from
the sourced intersection — Sunday 16:15-17:00 CT with 16:00-16:15 withheld,
Monday-Thursday 16:45-17:00 — and that module holds their launches and the
two dated queue revisions. `pgm_tas.rs` holds NYMEX platinum and palladium,
which launched after the undated 2012 Sunday move and therefore serve the
full 16:00-17:00 CT Sunday queue with nothing withheld.
