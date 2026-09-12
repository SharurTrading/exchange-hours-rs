<!-- SPDX-License-Identifier: MIT-0 -->

# `iceus` — evidence

- **Kind:** `Exchange`
- **Owner module:** [`ice_us.rs`](../../src/calendar/schedules/futures/us/ice_us.rs)
- **Source sets:** [`ICE-DERIVATIVES`](../schedules/sources.md#ice-derivatives)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

NYSE FANG+ Index Futures only: closed before the sourced 2017-11-07 launch-eve 19:30 Pre-Open / 20:00 matching start. Current Pre-Open is Sunday 17:30–18:00 and Monday–Thursday 19:30–20:00; regular matching then runs through 18:00 the next day.

## Revision rows

- 2017-11-07 — T1 — ICE FANG+ launch notice 20170926 — the launch-eve profile: a Tuesday 19:30–20:00 ET Pre-Open and the 20:00 matching start, and nothing earlier that day.
- 2017-11-08 — T1 — ICE FANG+ launch notice 20170926 — the full grid for trade date 2017-11-08: Sunday 18:00 open, Monday–Thursday 20:00 opens, matching through 18:00 the next day, with the 30-minute Pre-Open queues.

Everything below the first row is `CLOSED_NEW_YORK`, a sourced closure, so the
row's horizon is `—` and nothing is carried back to the January-2010 floor.

## Sources

Retrieval dates: these sources were last opened on the row's reviewed-on date
(2026-08-22, UTC); per-source retrieval dates were not recorded before the
2026-09-12 migration and are added as each source is re-verified.

The documents below stand behind the row and behind the narrative moved below.

- <https://www.ice.com/publicdocs/futures_us/exchange_notices/ICE_Futures_US_FANG%2BFuture_20170926.pdf> — the ICE Futures U.S. FANG+ launch notice of 2017-09-26, which states trading begins at the start of trade date 2017-11-08 with 20:00–18:00 ET hours, the exceptional Sunday 18:00 open, and a Pre-Open 30 minutes before each executable session — T1.
- <https://www.ice.com/products/66380320/NYSE-FANG-Index-Future> — the current NYSE FANG+ Index Future product page, which retains the grid and separately publishes the 17:30 Sunday and 19:30 weekday queue starts — T1.
- <https://www.ice.com/publicdocs/futures_us/ICE_Futures_US_Regular_Trading_Hours.pdf> — the ICE Futures U.S. *Regular Trading Hours* master table, June-2026 edition, which retains the same grid — T1.
- <https://www.ice.com/trading-hours> — ICE trading hours, the current-schedule monitoring entry point — T1.
- <https://www.ice.com/products> — the ICE product directory — T1.
- <https://www.ice.com/holiday-hours> — ICE holiday hours, the watch channel — T1.

## Gaps and residual risks

- **No dated-history gap.** The pre-launch era is a sourced closure and both
  revision rows rest on the operator's own launch notice, so the row is
  `Primary` with no carried interval.
- **Scope.** The `iceus` default is the NYSE FANG+ Index Futures family, not a
  venue-wide ICE Futures U.S. clock. ICE has no venue-wide clock at all: the six
  ICE Futures U.S. soft-commodity and index keys are separate identities with
  their own modules, their own dated revisions and their own January-2010 to
  August-2011 baseline gap, which this row does not share because FANG+ did not
  exist before 2017.
- **Queue classification.** The 17:30 Sunday and 19:30 weekday phases are the
  Pre-Open queues the launch notice and product page describe: the platform
  accepts, amends and cancels orders and nothing matches until the 18:00 or
  20:00 open, so they are `order_entry` rather than tradeable extended sessions.
  FANG+ publishes no tradeable phase outside its executable session, so the
  `extended` slice is deliberately empty.
- **The exceptional Sunday session is encoded as a full local-day span.** Equal
  `SessionRule` endpoints encode one complete local-day span, so the Sunday
  session remains continuous through Monday 18:00 rather than closing at
  midnight. A reader changing that rule must preserve the continuity.

## Module narrative (moved from src/calendar/schedules/futures/us/ice_us.rs on 2026-09-12 UTC)

The `iceus` default is the NYSE FANG+ Index futures family, not a venue-wide
clock. ICE launched it for trade date 2017-11-08 with 20:00-18:00 ET hours
and an exceptional Sunday 18:00 open; the current product page and ICE's
June-2026 master table retain that grid. The launch notice starts Pre-Open
30 minutes before each executable session, and the current product page
separately publishes the 17:30 Sunday and 19:30 weekday queue starts.

Equal `SessionRule` endpoints encode one complete local-day span, so the
exceptional Sunday session remains continuous through Monday 18:00.
<https://www.ice.com/publicdocs/futures_us/exchange_notices/ICE_Futures_US_FANG%2BFuture_20170926.pdf>
<https://www.ice.com/products/66380320/NYSE-FANG-Index-Future>
<https://www.ice.com/publicdocs/futures_us/ICE_Futures_US_Regular_Trading_Hours.pdf>

ORDER ENTRY, NOT TRADING. The 17:30 Sunday and 19:30 weekday phases are the
Pre-Open queues the launch notice and product page describe: the platform
accepts, amends and cancels orders for the coming session, and nothing
matches until the 18:00 / 20:00 open. They are therefore classified as
order-entry phases rather than tradeable extended sessions. FANG publishes no
tradeable phase outside its executable session, so the extended slice is
empty.

The launch notice says trading began at the start of trade date 2017-11-08;
its 20:00 prior-day trading rule and 30-minute Pre-Open therefore pin the
first order-entry phase to Tuesday 2017-11-07 at 19:30 ET. This one-evening
profile avoids pretending the product accepted orders earlier that day.

Same Pre-Open queue as the current profile, so the same classification: this
is the launch evening's order-entry phase, not a tradeable session.
