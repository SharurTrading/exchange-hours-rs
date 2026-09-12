<!-- SPDX-License-Identifier: MIT-0 -->

# `lse` — evidence

- **Kind:** `Exchange`
- **Owner module:** [`lse.rs`](../../src/calendar/schedules/equities/europe/lse.rs)
- **Source sets:** [`EU-LSE`](../schedules/sources.md#eu-lse), [`EU-FESE-SECONDARY`](../schedules/sources.md#eu-fese-secondary)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

SETS January-2010 baseline, the 2012-04-30 CPX launch, and the 2016-03-21 randomized midday auction are primary-sourced with conservative latest uncross edges.

## Revision rows

- 2012-04-30 — T1 — LSE MIT201 document history — Closing Price Crossing session added, closing envelope extended to 16:40.
- 2016-03-21 — T1 — LSE notice N01/16 — SETS intraday auction at 12:00 with its two-minute run and 30-second random end.

## Sources

Retrieval dates: these sources were last opened on the row's reviewed-on date
(2026-08-22, UTC); per-source retrieval dates were not recorded before the
2026-09-12 migration and are added as each source is re-verified.

- <https://docs.londonstockexchange.com/sites/default/files/documents/compliance_update_mar_09.pdf> — LSE compliance update, March 2009: the 2009 compliance parameters behind the January-2010 SETS baseline.
- <https://docs.londonstockexchange.com/sites/default/files/documents/live-001-300910-appendix-a.pdf> — LSE Millennium Exchange rehearsal timetable, September 2010, appendix A: pre-trading 07:00, opening call 07:50, randomized 08:00 uncross, closing call 16:30 to its latest 16:35:30 edge.
- <https://docs.londonstockexchange.com/sites/default/files/documents/mit201-guide-to-the-trading-system-15-6-20240429.pdf> — LSE MIT201, Guide to the Trading System: section 4.4 lists pre-trading as a scheduled session preceding the opening auction call; section 4.5 calls CPX "a short, modified regular trading session"; its operator-maintained document history records the 2012-04-30 production functional release.
- <https://docs.londonstockexchange.com/sites/default/files/documents/n1512_attach1.pdf> — LSE notice N15/12, official attachment: the closing-auction uncross starts at 16:35 and CPX is the up-to-five-minute executable session immediately following it.
- <https://docs.londonstockexchange.com/sites/default/files/documents/mit501.pdf> — LSE MIT501: CPX introduced in April 2012 with a default five-minute duration.
- <https://docs.londonstockexchange.com/sites/default/files/documents/servicetechnicaldescriptionintroductionofnewtradingcurrencies.pdf> — LSE service technical description: CPX scheduled grid 16:35:01–16:40:00.
- <https://docs.londonstockexchange.com/sites/default/files/documents/n0116.pdf> — LSE notice N01/16: SETS intraday auction effective 2016-03-21, starting at 12:00, running two minutes, with a random end of up to 30 seconds.
- <https://www.londonstockexchange.com/resources/equities-trading-resources?tab=technical-library> — LSE equities technical library: current technical parameters preserving the intraday-auction grid and CPX to 16:40.
- <https://www.londonstockexchange.com/equities-trading/asset-classes/shares-trading/sets> — LSE SETS product page, the source set's current entry point.
- <https://docs.londonstockexchange.com/sites/default/files/documents/international-order-book-introduction-sheet.pdf> — LSE SETS-aligned trading-day timetable.
- <https://www.fese.eu/app/uploads/2024/07/trading-hours-2025-1.pdf> — FESE 2025 trading-hours table, `EU-FESE-SECONDARY`: corroboration only, never an effective date.

## Gaps and residual risks

- **Interpretive step, order-entry classification.** Pre-trading 07:00–07:50 is modelled `order_entry` on MIT201 section 4.4, which lists it as a scheduled trading session distinct from the executable phases of the order-book day. No on-book execution can occur before the opening auction uncrosses. Closing condition: none needed; a later MIT201 edition that reclassifies the phase would move it.
- **Interpretive step, randomized uncrosses.** The opening uncross, the intraday auction uncross and the closing uncross are each randomized. The deterministic profile holds the auction classification through the latest possible edge (08:00:30, 12:02:30, 16:35:30), so the calendar never reports continuous trading while an auction can still run. This is the crate's conservative-envelope convention, not an operator statement about any individual security.
- **Dormant identity.** No SharurPlatform adapter admits an LSE namespace and no root maps to it, so this row is reviewed on demand (LAW-SERVICE-TIERS, LAW-WATCH). Gaps are recorded here rather than as issues.
