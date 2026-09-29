// SPDX-License-Identifier: MIT-0

//! Borsa İstanbul Equity Market holiday rows, 2012-2026.
//!
//! Keyed by the venue-local trade date in `Europe/Istanbul`. Borsa İstanbul's
//! Equity Market trades Monday-Friday, so every civil-date row in the
//! operator's tables lands on its own trade date and no event-date conversion
//! is needed; a holiday falling on a Saturday or Sunday (Zafer Bayramı
//! 2025-08-30 and 2026-08-30, Ramazan Bayramı Arefesi 2025-03-29, and the
//! weekend legs of the backfilled years) changes no trade date and ships no
//! row.
//!
//! The whole block is **T1**: the operator's own `Resmi Tatil Günleri` page,
//! whose server-rendered year tabs print every year 2012-2026 in one artifact,
//! each table stating the holiday and its status — `Kapalı` (closed) or
//! `Yarım Gün Tatil / Saat 13:00'e kadar` (half-day holiday until 13:00). The
//! 2012-2024 rows key to that single artifact; the 2025-2026 rows also key to
//! the per-market annex `EK-3 — Borsa İstanbul A.Ş. Pay Piyasası <year> Yılı
//! Tatil Tablosu`, which states per holiday whether a session takes place
//! (`seans yapılmayacaktır`) or a half-day session is held (`yarım gün seans
//! yapılacaktır`). The annex carries the 2025-2026 closure rows and the
//! half-day session's existence; the corporate page carries the 13:00 instant
//! an early-close row states. The coverage window opens at 2012-03-02, the
//! operator's own first sourced normal-week day; Borsa İstanbul has published
//! no 2027 holiday tables — the page's year list stops at 2026 — so coverage
//! stops at 2026-12-31; that and the per-row derivation are recorded in
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
    coverage: [(2012, 3, 2) ..= (2026, 12, 31)],
    rows: [
        // 2012-04-23 - T1 - BIST-RESMI-TATIL-GUNLERI - Ulusal Egemenlik ve
        // Çocuk Bayramı: `23 Nisan 2012, Pazartesi … Kapalı`.
        (2012, 4, 23, Closed, T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2012-05-01 - T1 - BIST-RESMI-TATIL-GUNLERI - Emek ve Dayanışma
        // Günü, Tuesday.
        (2012, 5, 1, Closed, T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2012-08-20 - T1 - BIST-RESMI-TATIL-GUNLERI - Ramazan Bayramı:
        // `19-21 Ağustos 2012, Pazar-Salı … Kapalı`, Monday leg.
        (2012, 8, 20, Closed, T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2012-08-21 - T1 - BIST-RESMI-TATIL-GUNLERI - Ramazan Bayramı,
        // Tuesday leg.
        (2012, 8, 21, Closed, T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2012-08-30 - T1 - BIST-RESMI-TATIL-GUNLERI - Zafer Bayramı,
        // Thursday.
        (2012, 8, 30, Closed, T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2012-10-24 - T1 - BIST-RESMI-TATIL-GUNLERI - Kurban Bayramı
        // Arefesi: `Yarım Gün Tatil / Saat 13:00'e kadar`, Wednesday.
        (2012, 10, 24, early_close(13 * 3_600), T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2012-10-25 - T1 - BIST-RESMI-TATIL-GUNLERI - Kurban Bayramı:
        // `25-28 Ekim 2012, Perşembe-Pazar … Kapalı`, Thursday leg.
        (2012, 10, 25, Closed, T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2012-10-26 - T1 - BIST-RESMI-TATIL-GUNLERI - Kurban Bayramı, Friday
        // leg; the 27th-28th are the weekend.
        (2012, 10, 26, Closed, T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2012-10-29 - T1 - BIST-RESMI-TATIL-GUNLERI - Cumhuriyet Bayramı,
        // Monday. The 28 Ekim leg is a Sunday.
        (2012, 10, 29, Closed, T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2013-01-01 - T1 - BIST-RESMI-TATIL-GUNLERI - Yılbaşı, Tuesday.
        (2013, 1, 1, Closed, T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2013-04-23 - T1 - BIST-RESMI-TATIL-GUNLERI - Ulusal Egemenlik ve
        // Çocuk Bayramı, Tuesday.
        (2013, 4, 23, Closed, T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2013-05-01 - T1 - BIST-RESMI-TATIL-GUNLERI - Emek ve Dayanışma
        // Günü, Wednesday.
        (2013, 5, 1, Closed, T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2013-08-07 - T1 - BIST-RESMI-TATIL-GUNLERI - Ramazan Bayramı
        // Arefesi: `Yarım Gün Tatil / Saat 13:00'e kadar`, Wednesday.
        (2013, 8, 7, early_close(13 * 3_600), T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2013-08-08 - T1 - BIST-RESMI-TATIL-GUNLERI - Ramazan Bayramı:
        // `8-10 Ağustos 2013, Perşembe-Cumartesi … Kapalı`, Thursday leg.
        (2013, 8, 8, Closed, T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2013-08-09 - T1 - BIST-RESMI-TATIL-GUNLERI - Ramazan Bayramı,
        // Friday leg; the 10th is a Saturday.
        (2013, 8, 9, Closed, T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2013-08-30 - T1 - BIST-RESMI-TATIL-GUNLERI - Zafer Bayramı, Friday.
        (2013, 8, 30, Closed, T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2013-10-14 - T1 - BIST-RESMI-TATIL-GUNLERI - Kurban Bayramı
        // Arefesi: `Yarım Gün Tatil / Saat 13:00'e kadar`, Monday.
        (2013, 10, 14, early_close(13 * 3_600), T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2013-10-15 - T1 - BIST-RESMI-TATIL-GUNLERI - Kurban Bayramı:
        // `15-18 Ekim 2013, Salı-Cuma … Kapalı`, Tuesday leg.
        (2013, 10, 15, Closed, T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2013-10-16 - T1 - BIST-RESMI-TATIL-GUNLERI - Kurban Bayramı,
        // Wednesday leg.
        (2013, 10, 16, Closed, T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2013-10-17 - T1 - BIST-RESMI-TATIL-GUNLERI - Kurban Bayramı,
        // Thursday leg.
        (2013, 10, 17, Closed, T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2013-10-18 - T1 - BIST-RESMI-TATIL-GUNLERI - Kurban Bayramı, Friday
        // leg.
        (2013, 10, 18, Closed, T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2013-10-28 - T1 - BIST-RESMI-TATIL-GUNLERI - Cumhuriyet Bayramı
        // Arefesi: `Yarım Gün Tatil / Saat 13:00'e kadar`, Monday.
        (2013, 10, 28, early_close(13 * 3_600), T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2013-10-29 - T1 - BIST-RESMI-TATIL-GUNLERI - Cumhuriyet Bayramı,
        // Tuesday.
        (2013, 10, 29, Closed, T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2014-01-01 - T1 - BIST-RESMI-TATIL-GUNLERI - Yılbaşı, Wednesday.
        (2014, 1, 1, Closed, T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2014-04-23 - T1 - BIST-RESMI-TATIL-GUNLERI - Ulusal Egemenlik ve
        // Çocuk Bayramı, Wednesday.
        (2014, 4, 23, Closed, T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2014-05-01 - T1 - BIST-RESMI-TATIL-GUNLERI - Emek ve Dayanışma
        // Günü, Thursday.
        (2014, 5, 1, Closed, T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2014-05-19 - T1 - BIST-RESMI-TATIL-GUNLERI - Atatürk'ü Anma,
        // Gençlik ve Spor Bayramı, Monday.
        (2014, 5, 19, Closed, T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2014-07-28 - T1 - BIST-RESMI-TATIL-GUNLERI - Ramazan Bayramı:
        // `28-29-30 Temmuz 2014, Pazartesi-Salı-Çarşamba … Kapalı`, Monday
        // leg (the 27 Ekim arefe leg is a Sunday).
        (2014, 7, 28, Closed, T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2014-07-29 - T1 - BIST-RESMI-TATIL-GUNLERI - Ramazan Bayramı,
        // Tuesday leg.
        (2014, 7, 29, Closed, T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2014-07-30 - T1 - BIST-RESMI-TATIL-GUNLERI - Ramazan Bayramı,
        // Wednesday leg.
        (2014, 7, 30, Closed, T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2014-10-03 - T1 - BIST-RESMI-TATIL-GUNLERI - Kurban Bayramı
        // Arifesi: `Yarım Gün Tatil / Saat 13:00'e kadar`, Friday.
        (2014, 10, 3, early_close(13 * 3_600), T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2014-10-06 - T1 - BIST-RESMI-TATIL-GUNLERI - Kurban Bayramı:
        // `4-5-6-7 Ekim 2014, Cumartesi-Pazar-Pazartesi-Salı … Kapalı`,
        // Monday leg.
        (2014, 10, 6, Closed, T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2014-10-07 - T1 - BIST-RESMI-TATIL-GUNLERI - Kurban Bayramı,
        // Tuesday leg.
        (2014, 10, 7, Closed, T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2014-10-28 - T1 - BIST-RESMI-TATIL-GUNLERI - Cumhuriyet Bayramı
        // Arefesi: `Yarım Gün Tatil / Saat 13:00'e kadar`, Tuesday.
        (2014, 10, 28, early_close(13 * 3_600), T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2014-10-29 - T1 - BIST-RESMI-TATIL-GUNLERI - Cumhuriyet Bayramı,
        // Wednesday.
        (2014, 10, 29, Closed, T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2015-01-01 - T1 - BIST-RESMI-TATIL-GUNLERI - Yılbaşı, Thursday.
        (2015, 1, 1, Closed, T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2015-04-23 - T1 - BIST-RESMI-TATIL-GUNLERI - Ulusal Egemenlik ve
        // Çocuk Bayramı, Thursday.
        (2015, 4, 23, Closed, T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2015-05-01 - T1 - BIST-RESMI-TATIL-GUNLERI - Emek ve Dayanışma
        // Günü, Friday.
        (2015, 5, 1, Closed, T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2015-05-19 - T1 - BIST-RESMI-TATIL-GUNLERI - Atatürk'ü Anma,
        // Gençlik ve Spor Bayramı, Tuesday.
        (2015, 5, 19, Closed, T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2015-07-16 - T1 - BIST-RESMI-TATIL-GUNLERI - Ramazan Bayramı
        // Arefesi: `Yarım Gün Tatil / Saat 13:00'e kadar`, Thursday.
        (2015, 7, 16, early_close(13 * 3_600), T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2015-07-17 - T1 - BIST-RESMI-TATIL-GUNLERI - Ramazan Bayramı:
        // `17-18-19 Temmuz, Cuma-Cumartesi-Pazar … Kapalı`, Friday leg.
        (2015, 7, 17, Closed, T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2015-09-23 - T1 - BIST-RESMI-TATIL-GUNLERI - Kurban Bayramı
        // Arefesi: `Yarım Gün Tatil / Saat 13:00'e kadar`, Wednesday.
        (2015, 9, 23, early_close(13 * 3_600), T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2015-09-24 - T1 - BIST-RESMI-TATIL-GUNLERI - Kurban Bayramı:
        // `24-25-26-27 Eylül, Perşembe-Cuma-Cumartesi-Pazar … Kapalı`,
        // Thursday leg.
        (2015, 9, 24, Closed, T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2015-09-25 - T1 - BIST-RESMI-TATIL-GUNLERI - Kurban Bayramı,
        // Friday leg; the 26th-27th are the weekend.
        (2015, 9, 25, Closed, T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2015-10-28 - T1 - BIST-RESMI-TATIL-GUNLERI - Cumhuriyet Bayramı
        // Arefesi: `Yarım Gün Tatil / Saat 13:00'e kadar`, Wednesday.
        (2015, 10, 28, early_close(13 * 3_600), T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2015-10-29 - T1 - BIST-RESMI-TATIL-GUNLERI - Cumhuriyet Bayramı,
        // Thursday.
        (2015, 10, 29, Closed, T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2016-01-01 - T1 - BIST-RESMI-TATIL-GUNLERI - Yılbaşı, Friday.
        (2016, 1, 1, Closed, T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2016-05-19 - T1 - BIST-RESMI-TATIL-GUNLERI - Atatürk'ü Anma,
        // Gençlik ve Spor Bayramı, Thursday (23 Nisan and 1 Mayıs 2016 are
        // weekend legs and ship no row).
        (2016, 5, 19, Closed, T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2016-07-04 - T1 - BIST-RESMI-TATIL-GUNLERI - Ramazan Bayramı
        // Arefesi: `Yarım Gün Tatil / Saat 13:00'e kadar`, Monday.
        (2016, 7, 4, early_close(13 * 3_600), T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2016-07-05 - T1 - BIST-RESMI-TATIL-GUNLERI - Ramazan Bayramı:
        // `5-6-7 Temmuz 2016, Salı-Çarşamba-Perşembe … Kapalı`, Tuesday leg.
        (2016, 7, 5, Closed, T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2016-07-06 - T1 - BIST-RESMI-TATIL-GUNLERI - Ramazan Bayramı,
        // Wednesday leg.
        (2016, 7, 6, Closed, T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2016-07-07 - T1 - BIST-RESMI-TATIL-GUNLERI - Ramazan Bayramı,
        // Thursday leg.
        (2016, 7, 7, Closed, T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2016-08-30 - T1 - BIST-RESMI-TATIL-GUNLERI - Zafer Bayramı,
        // Tuesday.
        (2016, 8, 30, Closed, T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2016-09-12 - T1 - BIST-RESMI-TATIL-GUNLERI - Kurban Bayramı:
        // `12-13-14-15 Eylül 2016, Pazartesi-Salı-Çarşamba-Perşembe …
        // Kapalı`, Monday leg (the 11 Eylül arefe leg is a Sunday).
        (2016, 9, 12, Closed, T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2016-09-13 - T1 - BIST-RESMI-TATIL-GUNLERI - Kurban Bayramı,
        // Tuesday leg.
        (2016, 9, 13, Closed, T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2016-09-14 - T1 - BIST-RESMI-TATIL-GUNLERI - Kurban Bayramı,
        // Wednesday leg.
        (2016, 9, 14, Closed, T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2016-09-15 - T1 - BIST-RESMI-TATIL-GUNLERI - Kurban Bayramı,
        // Thursday leg.
        (2016, 9, 15, Closed, T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2016-10-28 - T1 - BIST-RESMI-TATIL-GUNLERI - Cumhuriyet Bayramı
        // Arefesi: `Yarım Gün Tatil / Saat 13:00'e kadar`, Friday (29 Ekim
        // 2016 is a Saturday).
        (2016, 10, 28, early_close(13 * 3_600), T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2017-05-01 - T1 - BIST-RESMI-TATIL-GUNLERI - Emek ve Dayanışma
        // Günü, Monday (1 Ocak, 23 Nisan and 15 Temmuz 2017 are weekend legs).
        (2017, 5, 1, Closed, T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2017-05-19 - T1 - BIST-RESMI-TATIL-GUNLERI - Atatürk'ü Anma,
        // Gençlik ve Spor Bayramı, Friday.
        (2017, 5, 19, Closed, T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2017-06-26 - T1 - BIST-RESMI-TATIL-GUNLERI - Ramazan Bayramı:
        // `25-26-27 Haziran 2017, Pazar-Pazartesi-Salı … Kapalı`, Monday leg
        // (the 24 Haziran arefe leg is a Saturday).
        (2017, 6, 26, Closed, T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2017-06-27 - T1 - BIST-RESMI-TATIL-GUNLERI - Ramazan Bayramı,
        // Tuesday leg.
        (2017, 6, 27, Closed, T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2017-08-30 - T1 - BIST-RESMI-TATIL-GUNLERI - Zafer Bayramı,
        // Wednesday.
        (2017, 8, 30, Closed, T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2017-08-31 - T1 - BIST-RESMI-TATIL-GUNLERI - Kurban Bayramı
        // Arefesi: `Yarım Gün Tatil / Saat 13:00'e kadar`, Thursday.
        (2017, 8, 31, early_close(13 * 3_600), T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2017-09-01 - T1 - BIST-RESMI-TATIL-GUNLERI - Kurban Bayramı:
        // `1-2-3-4 Eylül 2017, Cuma-Cumartesi-Pazar-Pazartesi … Kapalı`,
        // Friday leg.
        (2017, 9, 1, Closed, T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2017-09-04 - T1 - BIST-RESMI-TATIL-GUNLERI - Kurban Bayramı,
        // Monday leg; the 2nd-3rd are the weekend (28-29 Ekim 2017 are
        // weekend legs).
        (2017, 9, 4, Closed, T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2018-01-01 - T1 - BIST-RESMI-TATIL-GUNLERI - Yılbaşı, Monday.
        (2018, 1, 1, Closed, T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2018-04-23 - T1 - BIST-RESMI-TATIL-GUNLERI - Ulusal Egemenlik ve
        // Çocuk Bayramı, Monday.
        (2018, 4, 23, Closed, T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2018-05-01 - T1 - BIST-RESMI-TATIL-GUNLERI - Emek ve Dayanışma
        // Günü, Tuesday.
        (2018, 5, 1, Closed, T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2018-06-14 - T1 - BIST-RESMI-TATIL-GUNLERI - Ramazan Bayramı
        // Arefesi: `Yarım Gün Tatil / Saat 13:00'e kadar`, Thursday.
        (2018, 6, 14, early_close(13 * 3_600), T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2018-06-15 - T1 - BIST-RESMI-TATIL-GUNLERI - Ramazan Bayramı:
        // `15-16-17 Haziran 2018, Cuma-Cumartesi-Pazar … Kapalı`, Friday leg.
        (2018, 6, 15, Closed, T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2018-08-20 - T1 - BIST-RESMI-TATIL-GUNLERI - Kurban Bayramı
        // Arefesi: `Yarım Gün Tatil / Saat 13:00'e kadar`, Monday.
        (2018, 8, 20, early_close(13 * 3_600), T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2018-08-21 - T1 - BIST-RESMI-TATIL-GUNLERI - Kurban Bayramı:
        // `21-22-23-24 Ağustos 2018, Salı-Çarşamba-Perşembe-Cuma … Kapalı`,
        // Tuesday leg.
        (2018, 8, 21, Closed, T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2018-08-22 - T1 - BIST-RESMI-TATIL-GUNLERI - Kurban Bayramı,
        // Wednesday leg.
        (2018, 8, 22, Closed, T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2018-08-23 - T1 - BIST-RESMI-TATIL-GUNLERI - Kurban Bayramı,
        // Thursday leg.
        (2018, 8, 23, Closed, T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2018-08-24 - T1 - BIST-RESMI-TATIL-GUNLERI - Kurban Bayramı,
        // Friday leg.
        (2018, 8, 24, Closed, T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2018-08-30 - T1 - BIST-RESMI-TATIL-GUNLERI - Zafer Bayramı,
        // Thursday (19 Mayıs and 15 Temmuz 2018 are weekend legs).
        (2018, 8, 30, Closed, T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2018-10-29 - T1 - BIST-RESMI-TATIL-GUNLERI - Cumhuriyet Bayramı,
        // Monday (the 28 Ekim leg is a Sunday).
        (2018, 10, 29, Closed, T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2019-01-01 - T1 - BIST-RESMI-TATIL-GUNLERI - Yılbaşı, Tuesday.
        (2019, 1, 1, Closed, T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2019-04-23 - T1 - BIST-RESMI-TATIL-GUNLERI - Ulusal Egemenlik ve
        // Çocuk Bayramı, Tuesday.
        (2019, 4, 23, Closed, T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2019-05-01 - T1 - BIST-RESMI-TATIL-GUNLERI - Emek ve Dayanışma
        // Günü, Wednesday.
        (2019, 5, 1, Closed, T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2019-06-03 - T1 - BIST-RESMI-TATIL-GUNLERI - Ramazan Bayramı
        // Arefesi: `Yarım Gün Tatil / Saat 13:00'e kadar`, Monday.
        (2019, 6, 3, early_close(13 * 3_600), T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2019-06-04 - T1 - BIST-RESMI-TATIL-GUNLERI - Ramazan Bayramı:
        // `4-5-6 Haziran 2019, Salı-Çarşamba-Perşembe … Kapalı`, Tuesday leg.
        (2019, 6, 4, Closed, T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2019-06-05 - T1 - BIST-RESMI-TATIL-GUNLERI - Ramazan Bayramı,
        // Wednesday leg.
        (2019, 6, 5, Closed, T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2019-06-06 - T1 - BIST-RESMI-TATIL-GUNLERI - Ramazan Bayramı,
        // Thursday leg.
        (2019, 6, 6, Closed, T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2019-07-15 - T1 - BIST-RESMI-TATIL-GUNLERI - Demokrasi ve Milli
        // Birlik Günü, Monday.
        (2019, 7, 15, Closed, T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2019-08-12 - T1 - BIST-RESMI-TATIL-GUNLERI - Kurban Bayramı:
        // `11-12-13-14 Ağustos 2019, Pazar-Pazartesi-Salı-Çarşamba … Kapalı`,
        // Monday leg (the 10 Ağustos arefe leg is a Saturday).
        (2019, 8, 12, Closed, T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2019-08-13 - T1 - BIST-RESMI-TATIL-GUNLERI - Kurban Bayramı,
        // Tuesday leg.
        (2019, 8, 13, Closed, T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2019-08-14 - T1 - BIST-RESMI-TATIL-GUNLERI - Kurban Bayramı,
        // Wednesday leg.
        (2019, 8, 14, Closed, T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2019-08-30 - T1 - BIST-RESMI-TATIL-GUNLERI - Zafer Bayramı, Friday
        // (19 Mayıs 2019 is a weekend leg).
        (2019, 8, 30, Closed, T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2019-10-28 - T1 - BIST-RESMI-TATIL-GUNLERI - Cumhuriyet Bayramı
        // Arefesi: `Yarım Gün Tatil / Saat 13:00'e kadar`, Monday.
        (2019, 10, 28, early_close(13 * 3_600), T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2019-10-29 - T1 - BIST-RESMI-TATIL-GUNLERI - Cumhuriyet Bayramı,
        // Tuesday.
        (2019, 10, 29, Closed, T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2020-01-01 - T1 - BIST-RESMI-TATIL-GUNLERI - Yılbaşı, Wednesday.
        (2020, 1, 1, Closed, T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2020-04-23 - T1 - BIST-RESMI-TATIL-GUNLERI - Ulusal Egemenlik ve
        // Çocuk Bayramı, Thursday.
        (2020, 4, 23, Closed, T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2020-05-01 - T1 - BIST-RESMI-TATIL-GUNLERI - Emek ve Dayanışma
        // Günü, Friday.
        (2020, 5, 1, Closed, T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2020-05-19 - T1 - BIST-RESMI-TATIL-GUNLERI - Atatürk'ü Anma,
        // Gençlik ve Spor Bayramı, Tuesday.
        (2020, 5, 19, Closed, T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2020-05-25 - T1 - BIST-RESMI-TATIL-GUNLERI - Ramazan Bayramı:
        // `24-25-26 Mayıs 2020, Pazar-Pazartesi-Salı … Kapalı`, Monday leg
        // (the 23 Mayıs arefe leg is a Saturday).
        (2020, 5, 25, Closed, T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2020-05-26 - T1 - BIST-RESMI-TATIL-GUNLERI - Ramazan Bayramı,
        // Tuesday leg.
        (2020, 5, 26, Closed, T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2020-07-15 - T1 - BIST-RESMI-TATIL-GUNLERI - Demokrasi ve Milli
        // Birlik Günü, Wednesday.
        (2020, 7, 15, Closed, T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2020-07-30 - T1 - BIST-RESMI-TATIL-GUNLERI - Kurban Bayramı
        // Arefesi: `Yarım Gün Tatil / Saat 13:00'e kadar`, Thursday.
        (2020, 7, 30, early_close(13 * 3_600), T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2020-07-31 - T1 - BIST-RESMI-TATIL-GUNLERI - Kurban Bayramı:
        // `31 Temmuz, 1-2-3 Ağustos 2020, Cuma-Cumartesi-Pazar-Pazartesi …
        // Kapalı`, Friday leg.
        (2020, 7, 31, Closed, T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2020-08-03 - T1 - BIST-RESMI-TATIL-GUNLERI - Kurban Bayramı,
        // Monday leg; the 1st-2nd August are the weekend.
        (2020, 8, 3, Closed, T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2020-10-28 - T1 - BIST-RESMI-TATIL-GUNLERI - Cumhuriyet Bayramı
        // Arefesi: `Yarım Gün Tatil / Saat 13:00'e kadar`, Wednesday (30
        // Ağustos 2020 is a weekend leg).
        (2020, 10, 28, early_close(13 * 3_600), T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2020-10-29 - T1 - BIST-RESMI-TATIL-GUNLERI - Cumhuriyet Bayramı,
        // Thursday.
        (2020, 10, 29, Closed, T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2021-01-01 - T1 - BIST-RESMI-TATIL-GUNLERI - Yılbaşı, Friday.
        (2021, 1, 1, Closed, T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2021-04-23 - T1 - BIST-RESMI-TATIL-GUNLERI - Ulusal Egemenlik ve
        // Çocuk Bayramı, Friday (1 Mayıs 2021 is a weekend leg).
        (2021, 4, 23, Closed, T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2021-05-12 - T1 - BIST-RESMI-TATIL-GUNLERI - Ramazan Bayramı
        // Arefesi: `Yarım Gün Tatil / Saat 13:00'e kadar`, Wednesday.
        (2021, 5, 12, early_close(13 * 3_600), T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2021-05-13 - T1 - BIST-RESMI-TATIL-GUNLERI - Ramazan Bayramı:
        // `13-14-15 Mayıs 2021, Perşembe-Cuma-Cumartesi … Kapalı`, Thursday
        // leg.
        (2021, 5, 13, Closed, T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2021-05-14 - T1 - BIST-RESMI-TATIL-GUNLERI - Ramazan Bayramı,
        // Friday leg.
        (2021, 5, 14, Closed, T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2021-05-19 - T1 - BIST-RESMI-TATIL-GUNLERI - Atatürk'ü Anma,
        // Gençlik ve Spor Bayramı, Wednesday.
        (2021, 5, 19, Closed, T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2021-07-15 - T1 - BIST-RESMI-TATIL-GUNLERI - Demokrasi ve Milli
        // Birlik Günü, Thursday.
        (2021, 7, 15, Closed, T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2021-07-19 - T1 - BIST-RESMI-TATIL-GUNLERI - Kurban Bayramı
        // Arefesi: `Yarım Gün Tatil / Saat 13:00'e kadar`, Monday.
        (2021, 7, 19, early_close(13 * 3_600), T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2021-07-20 - T1 - BIST-RESMI-TATIL-GUNLERI - Kurban Bayramı:
        // `20-21-22-23 Temmuz 2021, Salı-Çarşamba-Perşembe-Cuma … Kapalı`,
        // Tuesday leg.
        (2021, 7, 20, Closed, T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2021-07-21 - T1 - BIST-RESMI-TATIL-GUNLERI - Kurban Bayramı,
        // Wednesday leg.
        (2021, 7, 21, Closed, T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2021-07-22 - T1 - BIST-RESMI-TATIL-GUNLERI - Kurban Bayramı,
        // Thursday leg.
        (2021, 7, 22, Closed, T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2021-07-23 - T1 - BIST-RESMI-TATIL-GUNLERI - Kurban Bayramı,
        // Friday leg.
        (2021, 7, 23, Closed, T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2021-08-30 - T1 - BIST-RESMI-TATIL-GUNLERI - Zafer Bayramı,
        // Monday.
        (2021, 8, 30, Closed, T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2021-10-28 - T1 - BIST-RESMI-TATIL-GUNLERI - Cumhuriyet Bayramı
        // Arefesi: `Yarım Gün Tatil / Saat 13:00'e kadar`, Thursday.
        (2021, 10, 28, early_close(13 * 3_600), T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2021-10-29 - T1 - BIST-RESMI-TATIL-GUNLERI - Cumhuriyet Bayramı,
        // Friday.
        (2021, 10, 29, Closed, T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2022-05-02 - T1 - BIST-RESMI-TATIL-GUNLERI - Ramazan Bayramı:
        // `2-3-4 Mayıs 2022, Pazartesi-Salı-Çarşamba … Kapalı`, Monday leg
        // (1 Ocak, 23 Nisan and both 1 Mayıs 2022 legs are weekend legs).
        (2022, 5, 2, Closed, T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2022-05-03 - T1 - BIST-RESMI-TATIL-GUNLERI - Ramazan Bayramı,
        // Tuesday leg.
        (2022, 5, 3, Closed, T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2022-05-04 - T1 - BIST-RESMI-TATIL-GUNLERI - Ramazan Bayramı,
        // Wednesday leg.
        (2022, 5, 4, Closed, T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2022-05-19 - T1 - BIST-RESMI-TATIL-GUNLERI - Atatürk'ü Anma,
        // Gençlik ve Spor Bayramı, Thursday.
        (2022, 5, 19, Closed, T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2022-07-08 - T1 - BIST-RESMI-TATIL-GUNLERI - Kurban Bayramı
        // Arefesi: `Yarım Gün Tatil / Saat 13:00'e kadar`, Friday.
        (2022, 7, 8, early_close(13 * 3_600), T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2022-07-11 - T1 - BIST-RESMI-TATIL-GUNLERI - Kurban Bayramı:
        // `9-10-11-12 Temmuz 2022, Cumartesi-Pazar-Pazartesi-Salı … Kapalı`,
        // Monday leg.
        (2022, 7, 11, Closed, T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2022-07-12 - T1 - BIST-RESMI-TATIL-GUNLERI - Kurban Bayramı,
        // Tuesday leg.
        (2022, 7, 12, Closed, T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2022-07-15 - T1 - BIST-RESMI-TATIL-GUNLERI - Demokrasi ve Milli
        // Birlik Günü, Friday.
        (2022, 7, 15, Closed, T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2022-08-30 - T1 - BIST-RESMI-TATIL-GUNLERI - Zafer Bayramı,
        // Tuesday.
        (2022, 8, 30, Closed, T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2022-10-28 - T1 - BIST-RESMI-TATIL-GUNLERI - Cumhuriyet Bayramı
        // Arefesi: `Yarım Gün Tatil / Saat 13:00'e kadar`, Friday (29 Ekim
        // 2022 is a Saturday).
        (2022, 10, 28, early_close(13 * 3_600), T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2023-04-20 - T1 - BIST-RESMI-TATIL-GUNLERI - Ramazan Bayramı
        // Arefesi: `Yarım Gün Tatil / Saat 13:00'e kadar`, Thursday.
        (2023, 4, 20, early_close(13 * 3_600), T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2023-04-21 - T1 - BIST-RESMI-TATIL-GUNLERI - Ramazan Bayramı:
        // `21-22-23 Nisan 2023, Cuma-Cumartesi-Pazar … Kapalı`, Friday leg
        // (1 Ocak, 23 Nisan, 15 Temmuz, 28 Ekim and 29 Ekim 2023 are weekend
        // legs).
        (2023, 4, 21, Closed, T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2023-05-01 - T1 - BIST-RESMI-TATIL-GUNLERI - Emek ve Dayanışma
        // Günü, Monday.
        (2023, 5, 1, Closed, T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2023-05-19 - T1 - BIST-RESMI-TATIL-GUNLERI - Atatürk'ü Anma,
        // Gençlik ve Spor Bayramı, Friday.
        (2023, 5, 19, Closed, T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2023-06-27 - T1 - BIST-RESMI-TATIL-GUNLERI - Kurban Bayramı
        // Arefesi: `Yarım Gün Tatil / Saat 13:00'e kadar`, Tuesday.
        (2023, 6, 27, early_close(13 * 3_600), T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2023-06-28 - T1 - BIST-RESMI-TATIL-GUNLERI - Kurban Bayramı:
        // `28-29-30 Haziran ve 1 Temmuz 2023, Çarşamba-Perşembe-Cuma-Cumartesi
        // … Kapalı`, Wednesday leg.
        (2023, 6, 28, Closed, T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2023-06-29 - T1 - BIST-RESMI-TATIL-GUNLERI - Kurban Bayramı,
        // Thursday leg.
        (2023, 6, 29, Closed, T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2023-06-30 - T1 - BIST-RESMI-TATIL-GUNLERI - Kurban Bayramı,
        // Friday leg; the 1st July is a Saturday.
        (2023, 6, 30, Closed, T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2023-08-30 - T1 - BIST-RESMI-TATIL-GUNLERI - Zafer Bayramı,
        // Wednesday.
        (2023, 8, 30, Closed, T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2024-01-01 - T1 - BIST-RESMI-TATIL-GUNLERI - Yılbaşı, Monday.
        (2024, 1, 1, Closed, T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2024-04-09 - T1 - BIST-RESMI-TATIL-GUNLERI - Ramazan Bayramı
        // Arefesi: `Yarım Gün Tatil / Saat 13:00'e kadar`, Tuesday.
        (2024, 4, 9, early_close(13 * 3_600), T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2024-04-10 - T1 - BIST-RESMI-TATIL-GUNLERI - Ramazan Bayramı:
        // `10-11-12 Nisan 2024, Çarşamba-Perşembe-Cuma … Kapalı`, Wednesday
        // leg.
        (2024, 4, 10, Closed, T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2024-04-11 - T1 - BIST-RESMI-TATIL-GUNLERI - Ramazan Bayramı,
        // Thursday leg.
        (2024, 4, 11, Closed, T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2024-04-12 - T1 - BIST-RESMI-TATIL-GUNLERI - Ramazan Bayramı,
        // Friday leg.
        (2024, 4, 12, Closed, T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2024-04-23 - T1 - BIST-RESMI-TATIL-GUNLERI - Ulusal Egemenlik ve
        // Çocuk Bayramı, Tuesday.
        (2024, 4, 23, Closed, T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2024-05-01 - T1 - BIST-RESMI-TATIL-GUNLERI - Emek ve Dayanışma
        // Günü, Wednesday (19 Mayıs 2024 is a weekend leg).
        (2024, 5, 1, Closed, T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2024-06-17 - T1 - BIST-RESMI-TATIL-GUNLERI - Kurban Bayramı:
        // `16-17-18-19 Haziran 2024, Pazar-Pazartesi-Salı-Çarşamba … Kapalı`,
        // Monday leg (the 15 Haziran arefe leg is a Saturday).
        (2024, 6, 17, Closed, T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2024-06-18 - T1 - BIST-RESMI-TATIL-GUNLERI - Kurban Bayramı,
        // Tuesday leg.
        (2024, 6, 18, Closed, T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2024-06-19 - T1 - BIST-RESMI-TATIL-GUNLERI - Kurban Bayramı,
        // Wednesday leg.
        (2024, 6, 19, Closed, T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2024-07-15 - T1 - BIST-RESMI-TATIL-GUNLERI - Demokrasi ve Milli
        // Birlik Günü, Monday.
        (2024, 7, 15, Closed, T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2024-08-30 - T1 - BIST-RESMI-TATIL-GUNLERI - Zafer Bayramı, Friday.
        (2024, 8, 30, Closed, T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2024-10-28 - T1 - BIST-RESMI-TATIL-GUNLERI - Cumhuriyet Bayramı
        // Arefesi: `Yarım Gün Tatil / Saat 13:00'e kadar`, Monday.
        (2024, 10, 28, early_close(13 * 3_600), T1, "BIST-RESMI-TATIL-GUNLERI"),
        // 2024-10-29 - T1 - BIST-RESMI-TATIL-GUNLERI - Cumhuriyet Bayramı,
        // Tuesday.
        (2024, 10, 29, Closed, T1, "BIST-RESMI-TATIL-GUNLERI"),
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
