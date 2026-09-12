<!-- SPDX-License-Identifier: MIT-0 -->

# `euronext_paris` — evidence

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

- **Scope.** The profile represents the principal continuous-trading share segment, not every Euronext Paris instrument or segment.
- **Interpretive step, order-entry classification.** The operator's trading appendix describes the pre-opening as a Call phase — the French and Dutch columns render it "phase d'accumulation" / "accumulatiefase" — and its liquidity-provider clause speaks of "the order-accumulation periods preceding pre-scheduled or other Uncrossings during a Trading Day". The first uncrossing of the day is the 09:00 opening uncrossing, so no central-order-book trade can match before continuous trading starts; the pre-opening window is therefore `order_entry` and the closing uncrossing and Trading-at-Last stay `extended`.
- **Interpretive step, randomized uncrosses.** The 2015 notice's instrument-level 0–30-second micro-events do not define one exchange-wide transition instant, so this exchange-level profile retains the published nominal boundaries. The notice's defective effective year is deliberately not used as a cutover.
- **Horizon carried below the first dated artifact.** The earliest artifact cited for the legacy grid is the operator's special-day appendix for 24 and 31 December 2010, so the ledger horizon is 2010-12-24, its own first attested day, and the January-2010 to December-2010 interval is carried rather than sourced. The source set's "January-2010-or-launch" status is not an artifact dated inside that interval and does not source it. Closing condition: a Euronext trading appendix or notice dated in or before January 2010 that prints the legacy 07:15/09:00/17:30/17:40 grid would move the horizon down to the January-2010 floor.
- **Dormant identity.** Reviewed on demand; gaps are recorded here rather than as issues.

> Shared module. [`euronext.rs`](../../src/calendar/schedules/equities/europe/euronext.rs) also carries
> [`euronext_amsterdam`](euronext_amsterdam.md), [`euronext_brussels`](euronext_brussels.md),
> [`euronext_lisbon`](euronext_lisbon.md) and [`euronext_milan`](euronext_milan.md).
> `euronext_paris` is the anchor identity, so the module's narrative belongs in this file
> when LAW-EVIDENCE-FILES moves it; the module is not yet migrated.
