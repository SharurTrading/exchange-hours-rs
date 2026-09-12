<!-- SPDX-License-Identifier: MIT-0 -->

# `bme` — evidence

- **Kind:** `Exchange`
- **Owner module:** [`bme.rs`](../../src/calendar/schedules/equities/europe/bme.rs)
- **Source sets:** [`EU-BME`](../schedules/sources.md#eu-bme), [`EU-FESE-SECONDARY`](../schedules/sources.md#eu-fese-secondary)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

BME principal-share January-2010 phases use the sourced conservative 30-second auction edge; the 2023-12-04 Trading-At-Last launch is date-aware.

## Revision rows

- 2023-12-04 — T1 — BME Operating Instruction 47/2023 — ten-minute Trading-at-Last phase added for general trading, extending the executable envelope to 17:45.

## Sources

Retrieval dates: these sources were last opened on the row's reviewed-on date
(2026-08-22, UTC); per-source retrieval dates were not recorded before the
2026-09-12 migration and are added as each source is re-verified.

- <https://www.bolsasymercados.es/en/bme-exchange/trading-hours.html> — BME trading hours: opening auction 08:30–09:00, continuous trading 09:00–17:30, closing auction 17:30–17:35, Trading-at-Last to 17:45.
- <https://www.bolsasymercados.es/dam/descargas/regulacion/renta-variable/sociedad-de-bolsas/circular/2001/c20011uk.pdf> — Sociedad de Bolsas Circular 1/2001: the same opening, continuous and closing-auction grid, established before the January-2010 history floor.
- <https://www.bolsasymercados.es/es/sala-de-comunicacion/noticias/2023/las-subastas-en-la-bolsa-parte-2.html> — BME, "Las subastas en la Bolsa": SIBE auctions end in a random period of at most 30 seconds.
- <https://www.bolsasymercados.es/dam/descargas/regulacion/renta-variable/sociedad-de-bolsas/circular/2023/circular-1-23-english.pdf> — Sociedad de Bolsas Circular 1/2023, introducing the ten-minute Trading-at-Last phase for general trading.
- <https://www.bolsasymercados.es/dam/descargas/regulacion/renta-variable/sociedad-de-bolsas/instrucciones-operativas/2023/oi-47-2023-application-of-tal-phase-for-fixing-instruments.pdf> — BME Operating Instruction 47/2023: records the day-level 2023-12-04 entry into force and expressly excludes only the separate Fixing system.
- <https://www.bolsasymercados.es/bme-exchange/en/Regulation/Regulation-Explorer> — BME Regulation Explorer, the monitoring entry point.
- <https://www.fese.eu/app/uploads/2024/07/trading-hours-2025-1.pdf> — FESE 2025 trading-hours table, `EU-FESE-SECONDARY`: corroboration only.

## Gaps and residual risks

- **Scope.** The profile represents the general trading segment for principal shares. The separate Fixing system, which Operating Instruction 47/2023 excludes from the Trading-at-Last phase, is not modelled.
- **Interpretive step, empty order-entry slice.** Every non-regular phase modelled here is tradeable, so `order_entry` stays empty: the opening and closing windows are the SIBE auctions themselves, each ending in an allocation that prints trades at the auction price, and Trading-at-Last executes at the closing price. The operator publishes no separate pre-open or post-close order-entry phase for the general trading segment.
- **Interpretive step, randomized auction edge.** The deterministic profile uses the latest possible opening edge, 09:00:30, so it never reports continuous trading while the opening auction can still run; the closing auction likewise runs through 17:35:30.
- **Dormant identity.** Reviewed on demand; gaps are recorded here rather than as issues.
