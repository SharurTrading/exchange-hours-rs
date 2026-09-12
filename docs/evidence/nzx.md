<!-- SPDX-License-Identifier: MIT-0 -->

# `nzx` — evidence

- **Kind:** `Exchange`
- **Owner module:** [`nzx.rs`](../../src/calendar/schedules/equities/apac/nzx.rs)
- **Source sets:** [`APAC-NZX`](../schedules/sources.md#apac-nzx)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

Main Board; deterministic edges for randomized auctions.

## Revision rows

- 2020-04-06 — T1 — NZX announcement 350919 — pre-open start moves 09:00 → 08:30.

## Sources

Retrieval dates: these sources were last opened on the row's reviewed-on date
(2026-08-22, UTC); per-source retrieval dates were not recorded before the
2026-09-12 migration and are added as each source is re-verified.

- <https://www.nzx.com/learning/help-reference/trading-hours> — NZX trading hours.
- <https://www.nzx.com/learning/issuer-participant-resources/nzx-trading/anatomy-of-a-trading-day> — NZX Anatomy of a Trading Day. Of Pre-Open it says: "Orders can be placed, amended, and deleted. No trades execute until the opening auction. Off-market trades may be reported." Off-market reports print, so Pre-Open is tradeable `extended`, not order-entry-only.
- <https://www.nzx.com/announcements/350919> — NZX announcement 350919, the 2020-04-06 pre-open move.
- <https://www.nzx.com/announcements/353837> — NZX announcement 353837, making the initially temporary change indefinite.

## Gaps and residual risks

- **horizon carried below the first dated row** — the pre-2020 baseline rests only on NZX announcement 350919, whose publication day is not recorded in the repository (an NZX announcement number is not a date). The ledger horizon is therefore 2020-04-06, the first day at which this row's state is sourced, with everything below it carried. Closing condition: read the announcement's own publication date, or find a dated pre-2020 NZX trading-hours page; either would move the horizon earlier.
- The closing uncross is randomised within 30 seconds either side of 17:00, so the tradeable window runs to 17:00:30; stopping at 17:00 would drop the half of the randomisation in which the official closing print most often occurs.
- The only order-entry-only phase is Pre-Close 16:45–16:59:30, which neither matches nor accepts reports. The slice stops 30 seconds short of the nominal 17:00 boundary so the randomized uncross stays inside the tradeable window.
- Enquiry and Adjust do not accept automatically matched orders and are excluded from the envelope (AGENTS.md, *Cash-equity venue envelope*).
