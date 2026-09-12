<!-- SPDX-License-Identifier: MIT-0 -->

# `ice_canada` — evidence

- **Kind:** `Exchange`
- **Owner module:** [`ice_canada.rs`](../../src/calendar/schedules/futures/international/ice_canada.rs)
- **Source sets:** [`ICE-CANADA-LEGACY`](../schedules/sources.md#ice-canada-legacy)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

Legacy Winnipeg Canola: sourced January-2010, 2011 open, 2012/2013 close, and 2016 close-extension eras use their actual session-opening days. Closed from the Sunday 2018-07-29 opening for the 2018-07-30 IFUS transfer trade date.

## Revision rows

- 2012-06-24 — T1 — ICE Canada June 13 2012 trading hours change — close moves 13:15 → 14:00 CT for trade date 2012-06-25, whose session opened Sunday 2012-06-24.
- 2013-04-07 — T1 — ICE Canada April 8 2013 reminder — close restored to 13:15 CT for trade date 2013-04-08, whose session opened Sunday 2013-04-07.
- 2016-01-24 — T1 — ICE Canada Jan 18 2016 reminder — close extended 13:15 → 13:20 CT beginning trade date 2016-01-25.
- 2018-07-29 — T1 — ICE Futures US notice Canola 20180501 — Canola leaves ICE Futures Canada at the start of trading for trade date 2018-07-30; the identity is closed from that Sunday opening.

## Dated selectors

Day-level boundaries this identity's `profile_at` selects on directly, outside
any `revisions!` block. They are invisible to the module-declaration fences, so
they are recorded here in revision-row grammar and checked against
`HISTORICAL_CUTOVERS` / `HISTORICAL_INSTANT_CUTOVERS` in
`tests/contract/session_invariants/historical_expectations.rs`.

- 2011-02-28 — T1 — ICE Canada notice of 1 February 2011, revised (`REVISED_HOURS_2011_UNIX_SECONDS`) — an exact-instant boundary: the pre-open/open move to 18:30/19:00 CT on Monday 2011-02-28 for trade date 2011-03-01, which is 2011-03-01 00:30:00 UTC. Local midnight of 2011-02-28 falls inside the running Sunday session, so the boundary is the pre-open instant and never a day-level row; `HISTORICAL_INSTANT_CUTOVERS` records the UTC instant.

## Sources

Row review: 2026-08-22 (UTC) is the date the ledger row was last reviewed as a
whole. Per-source retrieval dates were not recorded before the 2026-09-12
migration; where a later targeted review, capture or document date is recorded
beside a source below, that date governs for that source, and dates are added
as each source is re-verified.

- <https://www.ice.com/publicdocs/futures_canada/member_notices/Trading_Calendar_2009.pdf> — the official 2009 trading calendar, pinning the January-2010 baseline: pre-open 19:00, continuous trading 20:00–13:15 CT.
- <https://www.ice.com/publicdocs/futures_canada/member_notices/Feb1_2011_revised_trading_hours.pdf> — 2011 notice moving the pre-open/open to 18:30/19:00 on Monday 2011-02-28 for trade date 2011-03-01.
- <https://www.ice.com/publicdocs/futures_canada/member_notices/June_13_2012_ICE_Futures_Canada_notice-Trading_Hours_and_Settlement_Time_Change.pdf> — 2012 notice moving the close to 14:00 for trade date 2012-06-25.
- <https://www.ice.com/publicdocs/futures_canada/member_notices/April_8_2013_Reminder_Closing_time_and_Settlement_time_changes_today.pdf> — 2013 reminder restoring the close to 13:15 for trade date 2013-04-08.
- <https://www.ice.com/publicdocs/futures_canada/member_notices/2016_01_18_Reminder_Canola_Trade_At_Settlement.pdf> — 2016 notice pinning the final legacy close extension from 13:15 to 13:20 beginning trade date 2016-01-25.
- <https://www.ice.com/publicdocs/futures_canada/member_notices/2017_11_27_Christmas_2017_and_New_Years_2018_Schedules.pdf> — 2017 holiday notice corroborating the final 19:00–13:20 CT grid.
- <https://www.ice.com/publicdocs/futures_us/exchange_notices/ICE_Futures_US-Notice-Canola-20180501.pdf> — 2018 transfer notice removing the product from IFCA at the start of trading for trade date 2018-07-30.
- <https://www.ice.com/historical-volumes-ifus-futures> — ICE historical volumes, confirming the product continued on ICE Futures U.S.

## Gaps and residual risks

- The 2011 change is deliberately absent from the day-level timeline. Local midnight of 2011-02-28 falls inside the still-running Sunday session (20:00 CT open, 13:15 CT next-day close), so a day-level row would split a running session (LAW-NO-FABRICATED-DATES). It is encoded as the exact UTC instant of the first new-schedule phase — the 18:30 CT pre-open, 2011-03-01 00:30:00 UTC — as the constant `REVISED_HOURS_2011_UNIX_SECONDS`.
- Contract specifications were otherwise unchanged across the transfer; the closed state after 2018-07-29 asserts only that this venue identity stopped trading it.
