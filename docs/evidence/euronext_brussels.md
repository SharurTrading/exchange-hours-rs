<!-- SPDX-License-Identifier: MIT-0 -->

# `euronext_brussels` — evidence

- **Kind:** `Exchange`
- **Owner module:** [`euronext.rs`](../../src/calendar/schedules/equities/europe/euronext.rs)
- **Source sets:** [`EU-EURONEXT`](../schedules/sources.md#eu-euronext), [`EU-FESE-SECONDARY`](../schedules/sources.md#eu-fese-secondary)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

Core shares at the published nominal exchange boundaries: legacy 07:15 pre-open, 09:00–17:30 continuous trading, 17:30–17:35 closing auction, and Trading-at-Last through 17:40; the 2023-03-20 move to a 07:30 pre-open is date-aware. Per-security 0–30-second auction uncross timing is outside scope and does not change venue availability.

## Revision rows

- 2023-03-20 — T1 — Euronext Go-Live Weekend Guidelines — legacy-market pre-opening moves from 07:15 to 07:30 CET.

## Sources

Row review: 2026-08-22 (UTC) is the date the ledger row was last reviewed as a
whole. Per-source retrieval dates were not recorded before the 2026-09-12
migration; where a later targeted review, capture or document date is recorded
beside a source below, that date governs for that source, and dates are added
as each source is re-verified.

- <https://www.euronext.com/sites/default/files/european_cash_markets_trading_hours_for_24th_and_31st_december_2010.pdf> — Euronext, "European cash markets trading hours for 24th and 31st December 2010": the operator's 2010 special-day appendix, showing the legacy 07:15 CET pre-opening and the principal-share opening at 09:00.
- <https://connect.euronext.com/nl/listview/notice-download?attachmentId=201416&id=581906&type=PDF> — Euronext 2014 normal-hours trading appendix, repeating the same legacy grid.
- <https://live.euronext.com/en/listview/notice-download?id=598779&type=PDF&attachmentId=218289> — Euronext notice PAR_20150924_07448_EUR, 2015 cash-market auction-randomization notice: zero-to-30-second randomized uncrosses for Belgian, Dutch, French and Portuguese trading groups.
- <https://live.euronext.com/en/listview/notice-download?id=598933&type=PDF&attachmentId=218443> — companion Euronext cash-market notice for the same randomization change.
- <https://connect.euronext.com/sites/default/files/it-documentation/Go-Live%20Weekend%20Guidelines%20-%20Borsa%20Italiana%20Optiq%20Migration.pdf> — Euronext Go-Live Weekend Guidelines: the phase-one timetable makes the legacy pre-opening change effective 2023-03-20 and separately gives 2023-03-27 for the Italian migration.
- <https://connect.euronext.com/sites/default/files/it-documentation/Guide%20to%20Trading%20System%20-%20Borsa%20Italiana%20Migration%20to%20Optiq%20-%20Functional%20Changes%20v.2.0.pdf> — Euronext guide to the trading system, Borsa Italiana migration to Optiq, functional changes v2.0.
- <https://www.euronext.com/sites/default/files/2026-07/appendix%20to%20Euronext%20Instructions%204-01%204-03%20Trading%20Manuals_0.xlsx> — current appendix to Euronext Instructions 4-01/4-03 Trading Manuals: pre-opening 07:30 CET, nominal continuous trading 09:00–17:30, closing auction to 17:35, Trading-at-Last through 17:40 for principal shares, with randomized zero-to-30-second uncross timing per security.
- <https://www.euronext.com/en/trading/trading-hours-holidays> — Euronext trading hours and holidays, the source set's current entry point.
- <https://www.euronext.com/en/regulation/euronext-regulated-markets> — Euronext regulated-market manual hub, the monitoring entry point.
- <https://www.euronext.com/en/products-services/cash-market-notices> — Euronext cash-market notices, the monitoring entry point.
- <https://www.fese.eu/app/uploads/2024/07/trading-hours-2025-1.pdf> — FESE 2025 trading-hours table, `EU-FESE-SECONDARY`: corroboration only.

## Gaps and residual risks

- **Scope.** The profile represents the principal continuous-trading share segment, not every Euronext Brussels instrument or segment.
- **Interpretive step, order-entry classification.** The operator's trading appendix describes the pre-opening as a Call phase — the French and Dutch columns render it "phase d'accumulation" / "accumulatiefase" — and its liquidity-provider clause speaks of "the order-accumulation periods preceding pre-scheduled or other Uncrossings during a Trading Day". The first uncrossing of the day is the 09:00 opening uncrossing, so no central-order-book trade can match before continuous trading starts; the pre-opening window is therefore `order_entry` and the closing uncrossing and Trading-at-Last stay `extended`.
- **Interpretive step, randomized uncrosses.** The 2015 notice's instrument-level 0–30-second micro-events do not define one exchange-wide transition instant, so this exchange-level profile retains the published nominal boundaries. The notice's defective effective year is deliberately not used as a cutover.
- **Horizon carried below the first dated artifact.** The earliest artifact cited for the legacy grid is the operator's special-day appendix for 24 and 31 December 2010, so the ledger horizon is 2010-12-24, its own first attested day, and the January-2010 to December-2010 interval is carried rather than sourced. The source set's "January-2010-or-launch" status is not an artifact dated inside that interval and does not source it. Closing condition: a Euronext trading appendix or notice dated in or before January 2010 that prints the legacy 07:15/09:00/17:30/17:40 grid would move the horizon down to the January-2010 floor.
- **Served identity.** Reviewed monthly after current-scope activation on
  2026-10-12 UTC; historical gaps remain tracked in #332 under the dated
  current OANDA scope exception in AGENTS.md.

> Shared module. [`euronext.rs`](../../src/calendar/schedules/equities/europe/euronext.rs) also carries
> [`euronext_paris`](euronext_paris.md), [`euronext_amsterdam`](euronext_amsterdam.md), [`euronext_lisbon`](euronext_lisbon.md), [`euronext_milan`](euronext_milan.md).
> The anchor identity for this module is [`euronext_paris`](euronext_paris.md), which receives the
> module narrative when LAW-EVIDENCE-FILES moves it; the module is not yet migrated.

## Holidays

**Coverage:** 2026-01-01..2026-12-31 (inclusive Brussels cash-market trade dates; T1).

### 2026

The operator’s current trading-hours page states: “The Euronext Cash Markets and the
Euronext Derivatives Markets will be open Monday to Friday throughout 2026 except
on the following days”. Its cash table names Amsterdam, Brussels, Dublin, Lisbon,
Milan, Oslo and Paris separately. This identity uses only the **Brussels** column,
not a same-timezone inference or another market’s dates.

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2026-01-01 | closed | Brussels `Closed` | `EURONEXT-CASH-CALENDAR-CURRENT-2026-10-12` | T1 | own named market column; no clock conversion |
| 2026-04-03 | closed | Brussels `Closed` | `EURONEXT-CASH-CALENDAR-CURRENT-2026-10-12` | T1 | own named market column; no clock conversion |
| 2026-04-06 | closed | Brussels `Closed` | `EURONEXT-CASH-CALENDAR-CURRENT-2026-10-12` | T1 | own named market column; no clock conversion |
| 2026-05-01 | closed | Brussels `Closed` | `EURONEXT-CASH-CALENDAR-CURRENT-2026-10-12` | T1 | own named market column; no clock conversion |
| 2026-12-24 | unsourced | Brussels `**Half Trading Day` | `EURONEXT-CASH-CALENDAR-CURRENT-2026-10-12` | T1 | announced half day; hours withheld pending appendix |
| 2026-12-25 | closed | Brussels `Closed` | `EURONEXT-CASH-CALENDAR-CURRENT-2026-10-12` | T1 | own named market column; no clock conversion |
| 2026-12-31 | unsourced | Brussels `**Half Trading Day` | `EURONEXT-CASH-CALENDAR-CURRENT-2026-10-12` | T1 | announced half day; hours withheld pending appendix |

The page says the exact half-day hours will be announced in an end-of-year
appendix to the trading manuals. Neither December 24 nor December 31 therefore
certifies an ordinary session or a guessed early close. This audited window is
2026-01-01 through 2026-12-31, with seven rows: five `Closed` and two `Unsourced`.
Dates without a Brussels exception in this window retain the existing named
normal profile. No historical holiday window, normal-week cutover, 2027 calendar
or minute-level half-day claim is introduced. The normal-profile horizon and the
earlier normal-week baseline gap above remain unchanged; older holiday calendars and the
unpublished half-day appendix would close the corresponding residual gaps.

The linked annual cash-calendar PDF (`EURONEXT-IF-2026`), independently
retrieved again on 2026-10-12 00:44 UTC, prints “Date: 07 November 2026”, which is
later than this review’s UTC date. Its closure and half-day cells agree with the
current webpage, but that anomalous printed date is not treated as an effective
date or evidence that a future publication has already happened. Runtime rows
use the separately retrieved current operator webpage at T1. The PDF is retained
only to disclose this lineage conflict.

### Documents

| Document | Window | Replay or service URL | Capture or retrieval, UTC | Tier | sha256 |
|---|---|---|---|---|---|
| `EURONEXT-CASH-CALENDAR-CURRENT-2026-10-12` | 2026 cash calendar; named market columns | <https://www.euronext.com/en/trading/trading-hours-holidays> | retrieved 2026-10-12 00:42 UTC | T1 | `f4f3d2ee85e88557db2aa6a29bf9ec23d1d2d82eaaa1a8cca2d91175c94fc831` |
| `EURONEXT-IF-2026` | 2026-01-01 .. 2026-12-31 | <https://connect2.euronext.com/sites/default/files/2025-11/IF251107CADE%202026%20Holiday%20Calendar%20for%20Euronexts%20Cash%20and%20Derivatives%20markets_1.pdf?VersionId=3ehe2.c9cMxv1K36.i6o4tz.5CAJmuw2> | retrieved 2026-09-28 01:09 | T1 | `617bc559510ef3a3d86d4a8b804422d0ba4c4dae43782a0a728d57a23fc4b1c5` |
