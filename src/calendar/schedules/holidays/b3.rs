// SPDX-License-Identifier: MIT-0

//! B3 cash-equity holiday rows, 2025-2026.
//!
//! Keyed by the venue-local trade date in `America/Sao_Paulo`. B3 cash
//! equities trade Monday-Friday, so every listed holiday lands on its own
//! trade date; the 2025 holidays that fell on weekends (07 September,
//! 12 October, 02 November, 15 November) print no session change and ship no
//! row.
//!
//! The whole block is **T1**: the operator's own yearly calendar articles and
//! the Ofício Circular that details them. The 2025 article
//! (`Calendário de feriados 2025`, 2024-12-04) and its OC 149/2024-PRE state
//! that no equities session runs on the listed dates, including
//! `Véspera de Natal` and 31 December, and OC 149's Anexo II states the
//! 2025-03-05 (Quarta-feira de Cinzas) special hours: pre-opening 12:45-13:00
//! and continuous equities trading 13:00-17:55. On that date the United
//! States is still on standard time, so the long grid is in force and its
//! normal 10:00 open and 17:55 close are unchanged; only the open moves, so
//! the row is a **late open** at 13:00. The 12:45-13:00 order-collection
//! window the circular states is not an executable phase and is not
//! representable by the scalar vocabulary; it is recorded as a residual in
//! the evidence file. The 2026 article
//! (`Calendário de negociação da B3 … em 2026`, 2026-01-09) states the same
//! shapes for 2026, with 2026-02-18's special hours printed as
//! `pré-abertura às 12h45 e as negociações começando às 13h`.
//!
//! B3 has published no 2027 calendar — the operator has released each year's
//! calendar in the December before or January of the year, and nothing for
//! 2027 existed at the 2026-09-28 retrieval — so coverage stops at
//! 2026-12-31; that and the per-row derivation are recorded in
//! [`docs/evidence/b3.md`](../../../../../docs/evidence/b3.md).

use super::EvidenceTier::T1;
use super::HolidayKind::Closed;
use super::fences::late_open;
use super::{HolidayTable, holidays};

/// B3's built-in holiday rows and the window they were audited over.
///
/// Every row is one line of the operator's 2025 calendar article or its
/// OC 149/2024-PRE, or of the operator's 2026 calendar article. A date inside
/// the window with no row is audited normal.
// Evidence: docs/evidence/b3.md
pub(crate) static TABLE: &HolidayTable = holidays! {
    coverage: [(2025, 1, 1) ..= (2026, 12, 31)],
    rows: [
        // 2025-01-01 - T1 - B3-OC-149-2024-PRE - Confraternização Universal:
        // Anexo I lists 1 January and §1.1 states no equities trading.
        (2025, 1, 1, Closed, T1, "B3-OC-149-2024-PRE"),
        // 2025-03-03 - T1 - B3-OC-149-2024-PRE - Carnaval (Monday).
        (2025, 3, 3, Closed, T1, "B3-OC-149-2024-PRE"),
        // 2025-03-04 - T1 - B3-OC-149-2024-PRE - Carnaval (Tuesday).
        (2025, 3, 4, Closed, T1, "B3-OC-149-2024-PRE"),
        // 2025-03-05 - T1 - B3-OC-149-2024-PRE - Quarta-feira de Cinzas:
        // Anexo II states continuous equities trading 13:00-17:55 with a
        // 12:45-13:00 pre-opening, so the long grid's close is unchanged and
        // only the open moves.
        (2025, 3, 5, late_open(13 * 3_600), T1, "B3-OC-149-2024-PRE"),
        // 2025-04-18 - T1 - B3-OC-149-2024-PRE - Sexta-feira Santa.
        (2025, 4, 18, Closed, T1, "B3-OC-149-2024-PRE"),
        // 2025-04-21 - T1 - B3-OC-149-2024-PRE - Tiradentes.
        (2025, 4, 21, Closed, T1, "B3-OC-149-2024-PRE"),
        // 2025-05-01 - T1 - B3-OC-149-2024-PRE - Dia do Trabalho.
        (2025, 5, 1, Closed, T1, "B3-OC-149-2024-PRE"),
        // 2025-06-19 - T1 - B3-OC-149-2024-PRE - Corpus Christi.
        (2025, 6, 19, Closed, T1, "B3-OC-149-2024-PRE"),
        // 2025-11-20 - T1 - B3-OC-149-2024-PRE - Dia Nacional de Zumbi e
        // Consciência Negra.
        (2025, 11, 20, Closed, T1, "B3-OC-149-2024-PRE"),
        // 2025-12-24 - T1 - B3-OC-149-2024-PRE - Véspera de Natal: the
        // circular states no trading session; only off-exchange booking runs.
        (2025, 12, 24, Closed, T1, "B3-OC-149-2024-PRE"),
        // 2025-12-25 - T1 - B3-OC-149-2024-PRE - Natal.
        (2025, 12, 25, Closed, T1, "B3-OC-149-2024-PRE"),
        // 2025-12-31 - T1 - B3-OC-149-2024-PRE - Véspera de Ano Novo: the
        // circular states no trading session.
        (2025, 12, 31, Closed, T1, "B3-OC-149-2024-PRE"),
        // 2026-01-01 - T1 - B3-NEWS-CAL-2026 - Confraternização Universal.
        (2026, 1, 1, Closed, T1, "B3-NEWS-CAL-2026"),
        // 2026-02-16 - T1 - B3-NEWS-CAL-2026 - Carnaval (Monday).
        (2026, 2, 16, Closed, T1, "B3-NEWS-CAL-2026"),
        // 2026-02-17 - T1 - B3-NEWS-CAL-2026 - Carnaval (Tuesday).
        (2026, 2, 17, Closed, T1, "B3-NEWS-CAL-2026"),
        // 2026-02-18 - T1 - B3-NEWS-CAL-2026 - Quarta-feira de Cinzas: the
        // article states pre-opening at 12:45 and trading beginning at 13:00,
        // on the long grid, so only the open moves.
        (2026, 2, 18, late_open(13 * 3_600), T1, "B3-NEWS-CAL-2026"),
        // 2026-04-03 - T1 - B3-NEWS-CAL-2026 - Sexta-feira Santa.
        (2026, 4, 3, Closed, T1, "B3-NEWS-CAL-2026"),
        // 2026-04-21 - T1 - B3-NEWS-CAL-2026 - Tiradentes.
        (2026, 4, 21, Closed, T1, "B3-NEWS-CAL-2026"),
        // 2026-05-01 - T1 - B3-NEWS-CAL-2026 - Dia do Trabalho.
        (2026, 5, 1, Closed, T1, "B3-NEWS-CAL-2026"),
        // 2026-06-04 - T1 - B3-NEWS-CAL-2026 - Corpus Christi.
        (2026, 6, 4, Closed, T1, "B3-NEWS-CAL-2026"),
        // 2026-09-07 - T1 - B3-NEWS-CAL-2026 - Independência do Brasil.
        (2026, 9, 7, Closed, T1, "B3-NEWS-CAL-2026"),
        // 2026-10-12 - T1 - B3-NEWS-CAL-2026 - Nossa Senhora Aparecida.
        (2026, 10, 12, Closed, T1, "B3-NEWS-CAL-2026"),
        // 2026-11-02 - T1 - B3-NEWS-CAL-2026 - Finados.
        (2026, 11, 2, Closed, T1, "B3-NEWS-CAL-2026"),
        // 2026-11-20 - T1 - B3-NEWS-CAL-2026 - Dia Nacional de Zumbi e
        // Consciência Negra.
        (2026, 11, 20, Closed, T1, "B3-NEWS-CAL-2026"),
        // 2026-12-24 - T1 - B3-NEWS-CAL-2026 - Véspera de Natal: the article
        // states `Não haverá sessão de negociação`.
        (2026, 12, 24, Closed, T1, "B3-NEWS-CAL-2026"),
        // 2026-12-25 - T1 - B3-NEWS-CAL-2026 - Natal.
        (2026, 12, 25, Closed, T1, "B3-NEWS-CAL-2026"),
        // 2026-12-31 - T1 - B3-NEWS-CAL-2026 - Véspera de Ano Novo: the
        // article states no trading session on B3.
        (2026, 12, 31, Closed, T1, "B3-NEWS-CAL-2026"),
    ],
};
