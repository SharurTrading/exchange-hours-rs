<!-- SPDX-License-Identifier: MIT-0 -->

# `bmv` — evidence

- **Kind:** `Exchange`
- **Owner module:** [`bmv.rs`](../../src/calendar/schedules/equities/americas/bmv.rs)
- **Source sets:** [`AMER-BMV`](../schedules/sources.md#amer-bmv)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

Date-aware normal/early grids retain the sourced HD/ID post-close tails: `:06` through 2016-09-04, `:10` from 2016-09-05, early `:20` from 2023-05-29, and normal `:20` from 2023-11-06. Operative manuals and the regulator annual record control over stale one-off DST notices.

## Revision rows

- 2010-03-16 — T1 — BMV DST notice 20100218 — the bounded early grid begins, 07:00–14:06 including the opening-auction stage and the HD/ID tail.
- 2010-04-01 — T1 — BMV DST notice 20100218 — the normal grid resumes, 08:00–15:06.

## Dated selectors

Day-level boundaries this identity's `profile_at` selects on directly, outside
any `revisions!` block. They are invisible to the module-declaration fences, so
they are recorded here in revision-row grammar and checked against
`HISTORICAL_CUTOVERS` / `HISTORICAL_INSTANT_CUTOVERS` in
`tests/contract/session_invariants/historical_expectations.rs`.

- 2010-11-01 — T1 — BMV notice 20101013 (`REFERENCE_GRID`) — the explicit 2010 bounded-exception rows end and the recurring New-York-offset selector begins.
- 2016-09-05 — T1 — Grupo BMV annual report 2016 (`HD_EXTENSION_2016`) — the HD/ID post-close tail moves from `:06` to `:10`.
- 2023-05-29 — T1 — BMV Manual v1.88 (`EARLY_HD_EXTENSION_2023`) — the early grid's tail extends to `:20` while the normal close stays at 15:10.
- 2023-11-06 — T1 — BMV Manual v1.90 (`NORMAL_HD_EXTENSION_2023`) — the normal grid's tail extends to `:20`.

## Sources

Row review: 2026-08-22 (UTC) is the date the ledger row was last reviewed as a
whole. Per-source retrieval dates were not recorded before the 2026-09-12
migration; where a later targeted review, capture or document date is recorded
beside a source below, that date governs for that source, and dates are added
as each source is re-verified.

- <https://www.bmv.com.mx/docs-pub/MARCO_NORMATIVO/CTEN_MNBM/MANUAL_OPERATIVO.PDF> — BMV Manual Operativo, the living operator entry point.
- <https://www.bmv.com.mx/docs-pub/MARCO_NORMATIVO/CTEN_MERMO/20260723_V1.97_Clarif_Dto.Aranceles_Ambiente_Pruerbas.pdf> — BMV Manual v1.97, printing the normal and US-daylight-time grids and the HD/ID post-close stages.
- <https://web.archive.org/web/20130908220405id_/http://www.bmv.com.mx/wb3/wb/BMV/BMV_repositorio/_vtp/BMV/BMV_8aa_sistema_electronico_de_negocios/_rid/223/_mto/3/20100218_DST_Cambio_de_horario.pdf> — BMV daylight-time notice of 2010-02-18: the exact bounded early grid for 2010-03-16 to 2010-03-31 and the normal grid that resumes on 2010-04-01.
- <https://web.archive.org/web/20101122224905id_/http://www.bmv.com.mx:80/wb3/wb/BMV/BMV_repositorio/_vtp/BMV/BMV_8aa_sistema_electronico_de_negocios/_rid/223/_mto/3/20101013_Aviso_Cambio_de_Horario.pdf> — BMV notice of 2010-10-13: another exact table and, crucially, the statement that BMV will align its hours with New York whenever the two countries' clock-change dates differ in the future. The recurring selector therefore starts with the notice's first affected session on 2010-11-01.
- <https://web.archive.org/web/20150510152627id_/http://www.bmv.com.mx/wb3/wb/BMV/BMV_repositorio/_vtp/BMV/BMV_1139_bmv_informa/_rid/223/_mto/3/Aviso_Importante_Horario_Operacion_2014.pdf> — BMV hours notice, 2014.
- <https://web.archive.org/web/20180413023907id_/http://www.bmv.com.mx:80/docs-pub/SALA_PRENSA/CTEN_NOTI/Aviso_Importante_Horario_Operaci%C3%B3n_ING.pdf> — BMV hours notice, English, 2018.
- <https://web.archive.org/web/20210310164243id_/https://www.bmv.com.mx/docs-pub/SALA_PRENSA/CTEN_NOTI/Aviso_Importante_Horario_Operaci%C3%B3n%20marzo%202021.pdf> — BMV hours notice, March 2021.
- <https://www.bmv.com.mx/docs-pub/informeAnual/INFORME%20ANUAL%20GRUPO%20BMV%202016.pdf> — Grupo BMV annual report 2016, the regulator-filed record dating the first HD extension to 2016-09-05.
- <https://www.bmv.com.mx/docs-pub/MARCO_NORMATIVO/CTEN_MERMO/20170522_V1%2053_Filtros%20valores%20menores%20a%20un%20peso_%28Esp%29.pdf> — BMV Manual v1.53.
- <https://bmv.com.mx/docs-pub/MARCO_NORMATIVO/CTEN_MERMO/20230529_V1.88%20Manual%20Operativo%20BMV%20%28ESP%29.pdf> — BMV Manual v1.88: extends the early grid to 14:20 from 2023-05-29 while retaining the 15:10 normal close.
- <https://www.bmv.com.mx/docs-pub/MARCO_NORMATIVO/CTEN_MERMO/20230728_V1.89%20Manual%20Operativo%20BMV%20%28ESP%29.pdf> — BMV Manual v1.89, effective 2023-08-01, preserving that table.
- <https://www.bmv.com.mx/docs-pub/MARCO_NORMATIVO/CTEN_MERMO/20231106_V1.90_Horario%20DC_Filtros%201E_%20Avisos%20en%20Susp_ReagruapTarif%20Anexo12%28ESP%29.pdf> — BMV Manual v1.90, effective 2023-11-06: extends the normal grid to 15:20 and prints both current grids with the New York alignment rule.

## Gaps and residual risks

- **Recurring regime.** From 2010-11-01 the profile is a recurring selector, not a revision row: the early grid applies whenever Mexico City sits two hours behind New York, per the operator's prospective alignment statement. The era constants `REFERENCE_GRID` (2010-11-01), `HD_EXTENSION_2016` (2016-09-05), `EARLY_HD_EXTENSION_2023` (2023-05-29) and `NORMAL_HD_EXTENSION_2023` (2023-11-06) live in `bmv.rs` and are not `revisions!` tuples; only the two 2010 bounded-exception rows are.
- **Horizon.** The earliest artifact indexed for this identity is the daylight-time notice of 2010-02-18, which prints the normal grid alongside the bounded early grid. The January-2010 to mid-February-2010 interval is therefore carried from that notice rather than sourced inside it, and the horizon is 2010-02-18. Closing condition: a BMV notice or manual edition dated at or before the January-2010 floor that prints the normal 08:00–15:06 grid.
- **Source conflict, disclosed.** A few narrow 2017 and 2018 daylight-time notices copied the older `:06` HD/ID row after the 2016-09-05 extension. The operative manuals and the regulator-filed implementation report govern this timeline; the discrepancy is retained here rather than silently ignored, and the ledger row states the same.
- **Interpretive step, empty order-entry slice.** No BMV phase modelled here is order-entry-only. The window before the open is the venue's opening-auction stage — the cancellation-only setup that precedes it is already excluded from the model — and the HD/ID tail is executable, so both print.
- **Dormant identity.** Reviewed on demand; gaps are recorded here rather than as issues.
