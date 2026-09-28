// SPDX-License-Identifier: MIT-0

//! Borsa İstanbul Equity Market holiday rows, 2025-2026.
//!
//! Keyed by the venue-local trade date in `Europe/Istanbul`. Borsa İstanbul's
//! Equity Market trades Monday-Friday, so every civil-date row in the
//! operator's tables lands on its own trade date and no event-date conversion
//! is needed; a holiday falling on a Saturday or Sunday (Zafer Bayramı
//! 2025-08-30 and 2026-08-30, Ramazan Bayramı Arefesi 2025-03-29) changes no
//! trade date and ships no row.
//!
//! The whole block is **T1**: the operator's own `Resmi Tatil Günleri` page,
//! which prints each year's table with the half-day entries marked
//! `Yarım Gün Tatil / Saat 13:00'e kadar` (half-day holiday until 13:00), plus
//! the per-market annex `EK-3 — Borsa İstanbul A.Ş. Pay Piyasası <year> Yılı
//! Tatil Tablosu`, which states per holiday whether a session takes place
//! (`seans yapılmayacaktır`) or a half-day session is held (`yarım gün seans
//! yapılacaktır`). The annex carries the closure rows and the half-day
//! session's existence; the corporate page carries the 13:00 instant an
//! early-close row states. Borsa İstanbul has published no 2027 holiday
//! tables — the page's year list stops at 2026 — so coverage stops at
//! 2026-12-31; that and the per-row derivation are recorded in
//! [`docs/evidence/borsa_istanbul.md`](../../../../../docs/evidence/borsa_istanbul.md).

use super::EvidenceTier::T1;
use super::HolidayKind::Closed;
use super::fences::early_close;
use super::{HolidayTable, holidays};

/// Borsa İstanbul Equity Market's built-in holiday rows and the window they
/// were audited over.
///
/// Every row is one line of the operator's `Resmi Tatil Günleri` page or of
/// its `Pay Piyasası <year> Yılı Tatil Tablosu` annex (EK-3). A date inside
/// the window with no row is audited normal.
// Evidence: docs/evidence/borsa_istanbul.md
pub(crate) static TABLE: &HolidayTable = holidays! {
    coverage: [(2025, 1, 1) ..= (2026, 12, 31)],
    rows: [
        // 2025-01-01 - T1 - BIST-PP-TATIL-2025 - Yılbaşı: the annex prints no
        // session for Wednesday 1 January 2025.
        (2025, 1, 1, Closed, T1, "BIST-PP-TATIL-2025"),
        // 2025-03-31 - T1 - BIST-PP-TATIL-2025 - Ramazan Bayramı: no session
        // Monday 31 March; the printed 29 March (Saturday) and 30 March
        // (Sunday) legs are not trade dates.
        (2025, 3, 31, Closed, T1, "BIST-PP-TATIL-2025"),
        // 2025-04-01 - T1 - BIST-PP-TATIL-2025 - Ramazan Bayramı: no session
        // Tuesday 1 April.
        (2025, 4, 1, Closed, T1, "BIST-PP-TATIL-2025"),
        // 2025-04-23 - T1 - BIST-PP-TATIL-2025 - Ulusal Egemenlik ve Çocuk
        // Bayramı, no session Wednesday.
        (2025, 4, 23, Closed, T1, "BIST-PP-TATIL-2025"),
        // 2025-05-01 - T1 - BIST-PP-TATIL-2025 - Emek ve Dayanışma Günü, no
        // session Thursday.
        (2025, 5, 1, Closed, T1, "BIST-PP-TATIL-2025"),
        // 2025-05-19 - T1 - BIST-PP-TATIL-2025 - Atatürk'ü Anma, Gençlik ve
        // Spor Bayramı, no session Monday.
        (2025, 5, 19, Closed, T1, "BIST-PP-TATIL-2025"),
        // 2025-06-05 - T1 - BIST-RESMI-TATIL-GUNLERI - Kurban Bayramı Arefesi:
        // a half-day session until 13:00 on Thursday, per the corporate page's
        // `Yarım Gün Tatil / Saat 13:00'e kadar`; the annex states the
        // half-day session itself.
        (2025, 6, 5, early_close(13 * 3_600), T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2025-06-06 - T1 - BIST-PP-TATIL-2025 - Kurban Bayramı: no session
        // Friday; the printed 7 June (Saturday) and 8 June (Sunday) legs are
        // not trade dates.
        (2025, 6, 6, Closed, T1, "BIST-PP-TATIL-2025"),
        // 2025-06-09 - T1 - BIST-PP-TATIL-2025 - Kurban Bayramı: no session
        // Monday.
        (2025, 6, 9, Closed, T1, "BIST-PP-TATIL-2025"),
        // 2025-07-15 - T1 - BIST-PP-TATIL-2025 - Demokrasi ve Milli Birlik
        // Günü, no session Tuesday.
        (2025, 7, 15, Closed, T1, "BIST-PP-TATIL-2025"),
        // 2025-10-28 - T1 - BIST-RESMI-TATIL-GUNLERI - Cumhuriyet Bayramı
        // Arefesi: a half-day session until 13:00 on Tuesday.
        (2025, 10, 28, early_close(13 * 3_600), T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2025-10-29 - T1 - BIST-PP-TATIL-2025 - Cumhuriyet Bayramı, no
        // session Wednesday.
        (2025, 10, 29, Closed, T1, "BIST-PP-TATIL-2025"),
        // 2026-01-01 - T1 - BIST-PP-TATIL-2026 - Yılbaşı, no session Thursday.
        (2026, 1, 1, Closed, T1, "BIST-PP-TATIL-2026"),
        // 2026-03-19 - T1 - BIST-RESMI-TATIL-GUNLERI - Ramazan Bayramı
        // Arefesi: a half-day session until 13:00 on Thursday.
        (2026, 3, 19, early_close(13 * 3_600), T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2026-03-20 - T1 - BIST-PP-TATIL-2026 - Ramazan Bayramı: no session
        // Friday; the printed 21-22 March legs are a weekend.
        (2026, 3, 20, Closed, T1, "BIST-PP-TATIL-2026"),
        // 2026-04-23 - T1 - BIST-PP-TATIL-2026 - Ulusal Egemenlik ve Çocuk
        // Bayramı, no session Thursday.
        (2026, 4, 23, Closed, T1, "BIST-PP-TATIL-2026"),
        // 2026-05-01 - T1 - BIST-PP-TATIL-2026 - Emek ve Dayanışma Günü, no
        // session Friday.
        (2026, 5, 1, Closed, T1, "BIST-PP-TATIL-2026"),
        // 2026-05-19 - T1 - BIST-PP-TATIL-2026 - Atatürk'ü Anma, Gençlik ve
        // Spor Bayramı, no session Tuesday.
        (2026, 5, 19, Closed, T1, "BIST-PP-TATIL-2026"),
        // 2026-05-26 - T1 - BIST-RESMI-TATIL-GUNLERI - Kurban Bayramı Arefesi:
        // a half-day session until 13:00 on Tuesday.
        (2026, 5, 26, early_close(13 * 3_600), T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2026-05-27 - T1 - BIST-PP-TATIL-2026 - Kurban Bayramı: no session
        // Wednesday.
        (2026, 5, 27, Closed, T1, "BIST-PP-TATIL-2026"),
        // 2026-05-28 - T1 - BIST-PP-TATIL-2026 - Kurban Bayramı: no session
        // Thursday.
        (2026, 5, 28, Closed, T1, "BIST-PP-TATIL-2026"),
        // 2026-05-29 - T1 - BIST-PP-TATIL-2026 - Kurban Bayramı: no session
        // Friday; the printed 30 May leg is a Saturday.
        (2026, 5, 29, Closed, T1, "BIST-PP-TATIL-2026"),
        // 2026-07-15 - T1 - BIST-PP-TATIL-2026 - Demokrasi ve Milli Birlik
        // Günü, no session Wednesday.
        (2026, 7, 15, Closed, T1, "BIST-PP-TATIL-2026"),
        // 2026-10-28 - T1 - BIST-RESMI-TATIL-GUNLERI - Cumhuriyet Bayramı
        // Arefesi: a half-day session until 13:00 on Wednesday.
        (2026, 10, 28, early_close(13 * 3_600), T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2026-10-29 - T1 - BIST-PP-TATIL-2026 - Cumhuriyet Bayramı, no
        // session Thursday.
        (2026, 10, 29, Closed, T1, "BIST-PP-TATIL-2026"),
    ],
};
