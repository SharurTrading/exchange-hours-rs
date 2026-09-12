<!-- SPDX-License-Identifier: MIT-0 -->

# `eex` — evidence

- **Kind:** `Exchange`
- **Owner module:** [`europe.rs`](../../src/calendar/schedules/futures/international/europe.rs)
- **Source sets:** [`EU-EEX`](../schedules/sources.md#eu-eex)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

Nordic Zonal Power Futures only: closed before the sourced 2024-03-25 launch, then 08:00–18:00 CE(S)T.

## Revision rows

None. `europe.rs` carries no `revisions!` block for this identity; its 2024-03-25 Nordic Zonal Power Futures launch is encoded as a date comparison inside `eex_profile_at` instead.

The 2024-03-25 launch is in `eex_profile_at` rather than in a `revisions!`
block: before that venue-local day the profile is a sourced closure, on and
after it the 08:00–18:00 CE(S)T table applies. EEX's own customer information
for Nordic Zonal Power Futures states both the launch day and the trading
table, so the boundary is T1 and unconditional even though it is not carried as
a tuple. Nothing is carried below the launch, so the row's horizon is `—`.

## Sources

The row was last reviewed on 2026-08-22; these are the documents behind it. The
module narrative that cites them lives in the anchor file for `europe.rs`.

- <https://www.eex.com/fileadmin/Global/News/EEX/EEX_Customer_Information/2024/20240109_EEX_Customer_Information_Nordic_Zonal_Futures.pdf> — the EEX customer information for Nordic Zonal Power Futures, which gives the 2024-03-25 launch and the 08:00–18:00 CE(S)T trading table — T1.
- <https://www.eex.com/fileadmin/EEX/Downloads/Trading/Trading_Hours/20250701_Trading_Hours_on_EEX_Derivatives_Markets_.pdf> — the current EEX derivatives timetable, which retains that grid — T1.
- <https://www.eex.com/fileadmin/EEX/Downloads/Rules/Trading_Conditions/20260513_EEX_Trading_Conditions_0073a_E_FINAL.pdf> — the EEX Trading Conditions, which make product hours controlling and define exchange days as Monday to Friday — T1.
- <https://www.eex.com/en/trading-resources/trading-information/rules-and-regulations> — the current EEX rules and regulations — T1.
- <https://www.eex.com/en/trading-resources/trading-information/trading-forms-and-documentation> — the EEX trading-hours documents index — T1.
- <https://www.eex.com/en/newsroom> — the official EEX newsroom, the watch channel — T1.
- <https://www.eex.com/en/downloads> — the EEX downloads index, the second watch channel — T1.

## Gaps and residual risks

- **No dated-history gap.** The pre-launch era is a sourced closure and the
  launch day is stated by the operator, so nothing is carried back to the
  January-2010 floor.
- **Scope.** EEX has no venue-wide grid. This default is specifically Nordic
  Zonal Power Futures; every other EEX product family needs its own profile, and
  the Trading Conditions themselves make product hours controlling. A consumer
  that routes another EEX product through this identity would be using the wrong
  clock.
- **Dormant identity (LAW-SERVICE-TIERS).** No SharurPlatform adapter admits an
  EEX venue namespace and no root maps to it, so the row is reviewed on demand
  and its history obligation is best-effort. Any future gap is recorded here
  rather than opened as an issue.

> Shared module. The narrative for
> [`europe.rs`](../../src/calendar/schedules/futures/international/europe.rs)
> lives in [`eurex`](eurex.md#module-narrative-moved-from-srccalendarschedulesfuturesinternationaleuropers-on-2026-09-12-utc).
> Sibling identity: the [`eurex` key](eurex_key.md).
