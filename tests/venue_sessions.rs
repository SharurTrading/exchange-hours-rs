// SPDX-License-Identifier: MIT-0

//! Normal-week baseline tests for futures venues: open/closed flags, session
//! bounds, weekend/holiday-window checks, and serde wire-format stability.
//!
//! Deliberately not covered here (and out of scope for the normal-week model):
//!
//!   - product-level hour variations (e.g. CME wheat vs equity index)
//!   - holiday calendars (the `is_holiday` stub always returns false)
//!   - early-close / half-day schedules
//!   - DST-transition edge cases (covered separately by the bias-aware helpers)
//!   - Binance quarterly-expiry pauses

#![expect(
    clippy::expect_used,
    reason = "fixture constructors assert their own literals; a bad literal must fail the test"
)]

use chrono::{DateTime, Datelike, TimeZone, Utc};
use chrono_tz::{America, Asia, Europe, US};
use exchange_hours::*;

fn utc(date: (i32, u32, u32), time: (u32, u32, u32)) -> DateTime<Utc> {
    Utc.with_ymd_and_hms(date.0, date.1, date.2, time.0, time.1, time.2)
        .single()
        .expect("valid UTC instant")
}

fn ct(date: (i32, u32, u32), time: (u32, u32, u32)) -> DateTime<Utc> {
    US::Central
        .with_ymd_and_hms(date.0, date.1, date.2, time.0, time.1, time.2)
        .single()
        .expect("valid CT instant")
        .with_timezone(&Utc)
}

fn et(date: (i32, u32, u32), time: (u32, u32, u32)) -> DateTime<Utc> {
    America::New_York
        .with_ymd_and_hms(date.0, date.1, date.2, time.0, time.1, time.2)
        .single()
        .expect("valid ET instant")
        .with_timezone(&Utc)
}

fn cet(date: (i32, u32, u32), time: (u32, u32, u32)) -> DateTime<Utc> {
    Europe::Berlin
        .with_ymd_and_hms(date.0, date.1, date.2, time.0, time.1, time.2)
        .single()
        .expect("valid CET instant")
        .with_timezone(&Utc)
}

fn lon(date: (i32, u32, u32), time: (u32, u32, u32)) -> DateTime<Utc> {
    Europe::London
        .with_ymd_and_hms(date.0, date.1, date.2, time.0, time.1, time.2)
        .single()
        .expect("valid London instant")
        .with_timezone(&Utc)
}

fn sgt(date: (i32, u32, u32), time: (u32, u32, u32)) -> DateTime<Utc> {
    Asia::Singapore
        .with_ymd_and_hms(date.0, date.1, date.2, time.0, time.1, time.2)
        .single()
        .expect("valid SGT instant")
        .with_timezone(&Utc)
}

// ---------------------------------------------------------------------------
// Named futures session profiles.
// ---------------------------------------------------------------------------

#[test]
fn all_market_hours_keys_return_profiles() {
    let keys = [
        MarketHoursKey::GlobexEquityIndex,
        MarketHoursKey::GlobexEnergy,
        MarketHoursKey::GlobexGrains,
        MarketHoursKey::GlobexFx,
        MarketHoursKey::CfeVix,
        MarketHoursKey::Eurex,
        MarketHoursKey::IceUs,
        MarketHoursKey::Sgx,
        MarketHoursKey::AlwaysOpen,
    ];

    for key in keys {
        let profile = session_profile(key);
        let total_rules = profile.regular.len() + profile.extended.len();
        assert!(
            total_rules > 0,
            "{key:?} profile must have at least one session rule"
        );
    }
}

#[test]
fn futures_session_profile_globex_equity_index_respects_daily_break() {
    let profile = session_profile(MarketHoursKey::GlobexEquityIndex);

    assert!(
        profile.is_open(ct((2026, 4, 20), (15, 45, 0))),
        "Globex equity-index profile is open during the 15:30-16:00 CT window"
    );
    assert!(
        !profile.is_open(ct((2026, 4, 20), (16, 30, 0))),
        "Globex equity-index profile is closed during the 16:00-17:00 CT maintenance break"
    );
    assert!(
        profile.is_open(ct((2026, 4, 20), (17, 0, 0))),
        "Globex equity-index profile reopens at 17:00 CT"
    );
}

#[test]
fn futures_session_profile_globex_energy_uses_wrap_session() {
    let profile = session_profile(MarketHoursKey::GlobexEnergy);

    assert!(
        profile.is_open(ct((2026, 4, 20), (15, 0, 0))),
        "Globex energy profile is open before the 16:00 CT daily close"
    );
    assert!(
        !profile.is_open(ct((2026, 4, 20), (16, 30, 0))),
        "Globex energy profile is closed during the 16:00-17:00 CT maintenance break"
    );
}

#[test]
fn futures_session_profile_always_open_has_one_all_days_rule() {
    let profile = session_profile(MarketHoursKey::AlwaysOpen);
    let total_rules = profile.regular.len() + profile.extended.len();

    assert_eq!(total_rules, 1, "AlwaysOpen should have exactly one rule");
    assert!(
        profile.extended.is_empty(),
        "AlwaysOpen has no extended rules"
    );
    assert!(
        !profile.has_daily_close,
        "AlwaysOpen must not have a daily close"
    );
    assert!(
        !profile.has_weekend_close,
        "AlwaysOpen must not have a weekend close"
    );

    let rule = &profile.regular[0];
    assert_eq!(
        rule.days, [true; 7],
        "AlwaysOpen rule must activate all days"
    );
    assert_eq!(rule.open_ssm, 0);
    assert_eq!(rule.close_ssm, 24 * 3600);
}

// ---------------------------------------------------------------------------
// CME (Equity Index) — Globex
//   Sun 17:00 – Fri 16:00 CT, daily maintenance 16:00–17:00 CT
//   RTH 08:30–15:15 CT, short window 15:30–16:00 CT
//   No Fri overnight session.
// ---------------------------------------------------------------------------

#[test]
fn cme_sunday_globex_open() {
    let h = hours_for_exchange(Exchange::Cme);
    let t = ct((2026, 4, 19), (17, 0, 0));
    assert!(h.is_open(t), "CME Globex opens Sun 17:00 CT");
    assert!(h.is_open_extended(t), "Sunday open is extended (Globex)");
}

#[test]
fn cme_sunday_before_open_closed() {
    let h = hours_for_exchange(Exchange::Cme);
    let t = ct((2026, 4, 19), (16, 59, 0));
    assert!(!h.is_open(t), "CME closed before Sun 17:00 CT");
}

#[test]
fn cme_monday_overnight_wrap() {
    let h = hours_for_exchange(Exchange::Cme);
    let t = ct((2026, 4, 20), (2, 0, 0));
    assert!(h.is_open(t), "CME Globex overnight Mon 02:00 CT");
    assert!(h.is_open_extended(t), "Overnight is extended");
}

#[test]
fn cme_monday_rth_open() {
    let h = hours_for_exchange(Exchange::Cme);
    let t = ct((2026, 4, 20), (8, 30, 0));
    assert!(h.is_open(t), "CME RTH Mon 08:30 CT");
    assert!(h.is_open_regular(t), "08:30 is RTH");
}

#[test]
fn cme_monday_rth_close() {
    let h = hours_for_exchange(Exchange::Cme);
    let t = ct((2026, 4, 20), (15, 15, 0));
    assert!(
        !h.is_open_regular(t),
        "CME RTH ends at 15:15 CT (end-exclusive)"
    );
}

#[test]
fn cme_monday_short_window_open() {
    let h = hours_for_exchange(Exchange::Cme);
    let t = ct((2026, 4, 20), (15, 30, 0));
    assert!(h.is_open(t), "CME short window 15:30–16:00 CT");
    assert!(h.is_open_extended(t), "short window is extended");
}

#[test]
fn cme_daily_maintenance_gap() {
    let h = hours_for_exchange(Exchange::Cme);
    let t = ct((2026, 4, 20), (16, 30, 0));
    assert!(!h.is_open(t), "CME maintenance gap 16:30 CT");
    assert!(
        h.is_maintenance(ct((2026, 4, 20), (16, 30, 0))),
        "16:30 is near-next-session"
    );
}

#[test]
fn cme_monday_globex_reopen() {
    let h = hours_for_exchange(Exchange::Cme);
    let t = ct((2026, 4, 20), (17, 0, 0));
    assert!(h.is_open(t), "CME Globex reopens Mon 17:00 CT");
}

#[test]
fn cme_friday_no_overnight() {
    let h = hours_for_exchange(Exchange::Cme);
    let t_short_end = ct((2026, 4, 24), (16, 0, 0));
    assert!(
        !h.is_open(t_short_end),
        "CME short window ends Fri 16:00 CT"
    );

    let t_fri_eve = ct((2026, 4, 24), (17, 0, 0));
    assert!(
        !h.is_open(t_fri_eve),
        "CME closed Fri 17:00 CT (no Fri overnight)"
    );
}

#[test]
fn cme_saturday_closed() {
    let h = hours_for_exchange(Exchange::Cme);
    let t = ct((2026, 4, 25), (10, 0, 0));
    assert!(!h.is_open(t), "CME closed Saturday");
}

#[test]
fn cme_weekend_boundary() {
    let h = hours_for_exchange(Exchange::Cme);
    assert!(
        !h.is_open(ct((2026, 4, 25), (23, 59, 0))),
        "CME closed Sat night"
    );
    assert!(
        !h.is_open(ct((2026, 4, 26), (10, 0, 0))),
        "CME closed Sun morning"
    );
    assert!(
        h.is_open(ct((2026, 4, 26), (17, 0, 0))),
        "CME opens Sun 17:00 CT"
    );
}

#[test]
fn cme_full_normal_week_open_flags() {
    let h = hours_for_exchange(Exchange::Cme);
    assert!(h.has_daily_close, "CME has a daily close (maintenance gap)");
    assert!(h.has_weekend_close, "CME has a weekend close");
}

// ---------------------------------------------------------------------------
// CBOT (Grains / Oilseeds)
//   Overnight: Sun + Mon–Thu 19:00 → next day 07:45 CT (wrap)
//   Day: Mon–Fri 08:30–13:20 CT
//   No Fri overnight session.
// ---------------------------------------------------------------------------

#[test]
fn cbot_sunday_overnight_open() {
    let h = hours_for_exchange(Exchange::Cbot);
    let t = ct((2026, 4, 19), (19, 0, 0));
    assert!(h.is_open(t), "CBOT overnight opens Sun 19:00 CT");
}

#[test]
fn cbot_overnight_wrap_into_monday() {
    let h = hours_for_exchange(Exchange::Cbot);
    let t = ct((2026, 4, 20), (2, 0, 0));
    assert!(h.is_open(t), "CBOT overnight wrap Mon 02:00 CT");
}

#[test]
fn cbot_overnight_close_before_day() {
    let h = hours_for_exchange(Exchange::Cbot);
    let t = ct((2026, 4, 20), (7, 45, 0));
    assert!(
        !h.is_open(t),
        "CBOT overnight ends 07:45 CT (end-exclusive)"
    );
}

#[test]
fn cbot_gap_between_overnight_and_day() {
    let h = hours_for_exchange(Exchange::Cbot);
    let t = ct((2026, 4, 20), (8, 0, 0));
    assert!(!h.is_open(t), "CBOT gap between 07:45 and 08:30 CT");
}

#[test]
fn cbot_day_session_open() {
    let h = hours_for_exchange(Exchange::Cbot);
    let t = ct((2026, 4, 20), (8, 30, 0));
    assert!(h.is_open(t), "CBOT day session Mon 08:30 CT");
    assert!(h.is_open_regular(t), "08:30 is regular (day)");
}

#[test]
fn cbot_day_session_close() {
    let h = hours_for_exchange(Exchange::Cbot);
    let t = ct((2026, 4, 20), (13, 20, 0));
    assert!(!h.is_open(t), "CBOT day ends 13:20 CT (end-exclusive)");
}

#[test]
fn cbot_monday_overnight_reopen() {
    let h = hours_for_exchange(Exchange::Cbot);
    let t = ct((2026, 4, 20), (19, 0, 0));
    assert!(h.is_open(t), "CBOT overnight reopens Mon 19:00 CT");
}

#[test]
fn cbot_friday_no_overnight() {
    let h = hours_for_exchange(Exchange::Cbot);
    let t = ct((2026, 4, 24), (19, 0, 0));
    assert!(!h.is_open(t), "CBOT closed Fri 19:00 CT (no Fri overnight)");
}

#[test]
fn cbot_saturday_closed() {
    let h = hours_for_exchange(Exchange::Cbot);
    assert!(
        !h.is_open(ct((2026, 4, 25), (10, 0, 0))),
        "CBOT closed Saturday"
    );
}

#[test]
fn cbot_weekend_boundary() {
    let h = hours_for_exchange(Exchange::Cbot);
    assert!(
        !h.is_open(ct((2026, 4, 26), (10, 0, 0))),
        "CBOT closed Sun morning"
    );
    assert!(
        h.is_open(ct((2026, 4, 26), (19, 0, 0))),
        "CBOT opens Sun 19:00 CT"
    );
}

// ---------------------------------------------------------------------------
// COMEX (Metals)
//   17:00–16:00 CT wrap (Sun + Mon–Thu), maintenance 16:00–17:00 daily
//   No Fri overnight.
// ---------------------------------------------------------------------------

#[test]
fn comex_sunday_open() {
    let h = hours_for_exchange(Exchange::Comex);
    let t = ct((2026, 4, 19), (17, 0, 0));
    assert!(h.is_open(t), "COMEX opens Sun 17:00 CT");
}

#[test]
fn comex_monday_trading() {
    let h = hours_for_exchange(Exchange::Comex);
    assert!(
        h.is_open(ct((2026, 4, 20), (10, 0, 0))),
        "COMEX Mon 10:00 CT"
    );
}

#[test]
fn comex_daily_maintenance() {
    let h = hours_for_exchange(Exchange::Comex);
    let t = ct((2026, 4, 20), (16, 30, 0));
    assert!(!h.is_open(t), "COMEX maintenance gap 16:30 CT");
    assert!(
        h.is_maintenance(t),
        "16:30 CT is near-next-session maintenance"
    );
}

#[test]
fn comex_monday_reopen() {
    let h = hours_for_exchange(Exchange::Comex);
    let t = ct((2026, 4, 20), (17, 0, 0));
    assert!(h.is_open(t), "COMEX reopens Mon 17:00 CT");
}

#[test]
fn comex_friday_close() {
    let h = hours_for_exchange(Exchange::Comex);
    let t = ct((2026, 4, 24), (16, 0, 0));
    assert!(!h.is_open(t), "COMEX closes Fri 16:00 CT (end-exclusive)");
    assert!(
        !h.is_open(ct((2026, 4, 24), (17, 0, 0))),
        "COMEX closed Fri 17:00 CT"
    );
}

#[test]
fn comex_saturday_closed() {
    let h = hours_for_exchange(Exchange::Comex);
    assert!(
        !h.is_open(ct((2026, 4, 25), (10, 0, 0))),
        "COMEX closed Saturday"
    );
}

#[test]
fn comex_weekend_boundary() {
    let h = hours_for_exchange(Exchange::Comex);
    assert!(
        !h.is_open(ct((2026, 4, 26), (16, 59, 0))),
        "COMEX closed before Sun 17:00 CT"
    );
    assert!(
        h.is_open(ct((2026, 4, 26), (17, 0, 0))),
        "COMEX opens Sun 17:00 CT"
    );
}

// ---------------------------------------------------------------------------
// NYMEX (Energy) — same profile as COMEX
// ---------------------------------------------------------------------------

#[test]
fn nymex_sunday_open() {
    let h = hours_for_exchange(Exchange::Nymex);
    assert!(
        h.is_open(ct((2026, 4, 19), (17, 0, 0))),
        "NYMEX opens Sun 17:00 CT"
    );
}

#[test]
fn nymex_daily_maintenance() {
    let h = hours_for_exchange(Exchange::Nymex);
    assert!(
        !h.is_open(ct((2026, 4, 20), (16, 30, 0))),
        "NYMEX maintenance 16:30 CT"
    );
}

#[test]
fn nymex_friday_close_and_weekend() {
    let h = hours_for_exchange(Exchange::Nymex);
    assert!(
        !h.is_open(ct((2026, 4, 24), (16, 0, 0))),
        "NYMEX closes Fri 16:00 CT"
    );
    assert!(
        !h.is_open(ct((2026, 4, 25), (10, 0, 0))),
        "NYMEX closed Saturday"
    );
}

// ---------------------------------------------------------------------------
// EUREX (Index / IR)
//   Regular: Mon–Fri 08:00–22:00 CET/CEST
//   Asian (post-2018): Mon–Fri 01:00–08:00 CET/CEST
//   No weekend trading.
// ---------------------------------------------------------------------------

#[test]
fn eurex_asian_hours() {
    let h = hours_for_exchange_as_of(Exchange::Eurex, utc((2026, 4, 20), (5, 0, 0)));
    let t = cet((2026, 4, 20), (5, 0, 0));
    assert!(h.is_open(t), "EUREX Asian hours Mon 05:00 CET");
    assert!(h.is_open_extended(t), "Asian is extended");
}

#[test]
fn eurex_regular_hours() {
    let h = hours_for_exchange_as_of(Exchange::Eurex, utc((2026, 4, 20), (10, 0, 0)));
    let t = cet((2026, 4, 20), (10, 0, 0));
    assert!(h.is_open(t), "EUREX regular Mon 10:00 CET");
    assert!(h.is_open_regular(t), "10:00 is regular");
}

#[test]
fn eurex_regular_close() {
    let h = hours_for_exchange_as_of(Exchange::Eurex, utc((2026, 4, 20), (22, 0, 0)));
    let t = cet((2026, 4, 20), (22, 0, 0));
    assert!(
        !h.is_open(t),
        "EUREX regular ends 22:00 CET (end-exclusive)"
    );
}

#[test]
fn eurex_friday_close() {
    let h = hours_for_exchange_as_of(Exchange::Eurex, utc((2026, 4, 24), (22, 0, 0)));
    let t = cet((2026, 4, 24), (22, 0, 0));
    assert!(!h.is_open(t), "EUREX closes Fri 22:00 CET");
}

#[test]
fn eurex_weekend_closed() {
    let h = hours_for_exchange_as_of(Exchange::Eurex, utc((2026, 4, 25), (10, 0, 0)));
    assert!(
        !h.is_open(cet((2026, 4, 25), (10, 0, 0))),
        "EUREX closed Saturday"
    );
    assert!(
        !h.is_open(cet((2026, 4, 26), (10, 0, 0))),
        "EUREX closed Sunday"
    );
}

#[test]
fn eurex_no_maintenance_gap() {
    let h = hours_for_exchange_as_of(Exchange::Eurex, utc((2026, 4, 20), (7, 55, 0)));
    let t = cet((2026, 4, 20), (7, 55, 0));
    assert!(h.is_open(t), "EUREX seamless handoff Asian→Regular");
}

#[test]
fn eurex_before_asian_hours() {
    let h = hours_for_exchange_as_of(Exchange::Eurex, utc((2026, 4, 20), (0, 30, 0)));
    let t = cet((2026, 4, 20), (0, 30, 0));
    assert!(!h.is_open(t), "EUREX closed before 01:00 CET");
}

#[test]
fn eurex_had_no_asian_session_before_2018_12_10() {
    // The 01:00-08:00 CET Asian window was added 2018-12-10; before that the
    // venue opened at 08:00. Pins both sides of the cutover so the arm in
    // `hours_for_exchange_as_of` cannot be inverted unnoticed.
    let before = hours_for_exchange_as_of(Exchange::Eurex, cet((2018, 12, 7), (12, 0, 0)));
    assert!(
        !before.is_open(cet((2018, 12, 7), (5, 0, 0))),
        "no Asian session on a Friday before the cutover"
    );
    assert!(
        before.is_open_regular(cet((2018, 12, 7), (10, 0, 0))),
        "the regular session is unaffected by the cutover"
    );

    let after = hours_for_exchange_as_of(Exchange::Eurex, cet((2018, 12, 10), (12, 0, 0)));
    assert!(
        after.is_open_extended(cet((2018, 12, 10), (5, 0, 0))),
        "the Asian session is live from the cutover date"
    );
}

// ---------------------------------------------------------------------------
// ICE Futures U.S. (ICEUS)
//   20:00–18:00 ET wrap (Sun + Mon–Thu), daily break 18:00–20:00 ET
//   No Fri overnight.
// ---------------------------------------------------------------------------

#[test]
fn iceus_sunday_open() {
    let h = hours_for_exchange(Exchange::Iceus);
    let t = et((2026, 4, 19), (20, 0, 0));
    assert!(h.is_open(t), "ICEUS opens Sun 20:00 ET");
}

#[test]
fn iceus_overnight_wrap() {
    let h = hours_for_exchange(Exchange::Iceus);
    let t = et((2026, 4, 20), (10, 0, 0));
    assert!(h.is_open(t), "ICEUS Mon 10:00 ET (in wrap)");
}

#[test]
fn iceus_daily_close() {
    let h = hours_for_exchange(Exchange::Iceus);
    let t = et((2026, 4, 20), (18, 0, 0));
    assert!(!h.is_open(t), "ICEUS closes Mon 18:00 ET (end-exclusive)");
}

#[test]
fn iceus_daily_break() {
    let h = hours_for_exchange(Exchange::Iceus);
    let t = et((2026, 4, 20), (19, 0, 0));
    assert!(!h.is_open(t), "ICEUS in 18:00–20:00 ET break");
    assert!(
        h.is_maintenance(t),
        "19:00 ET is near-next-session maintenance"
    );
}

#[test]
fn iceus_friday_close() {
    let h = hours_for_exchange(Exchange::Iceus);
    let t = et((2026, 4, 24), (18, 0, 0));
    assert!(!h.is_open(t), "ICEUS closes Fri 18:00 ET");
    assert!(
        !h.is_open(et((2026, 4, 24), (20, 0, 0))),
        "ICEUS no Fri overnight"
    );
}

#[test]
fn iceus_weekend_closed() {
    let h = hours_for_exchange(Exchange::Iceus);
    assert!(
        !h.is_open(et((2026, 4, 25), (10, 0, 0))),
        "ICEUS closed Saturday"
    );
}

// ---------------------------------------------------------------------------
// ICE Futures Europe (ICEEU)
//   01:00–23:00 London, Mon–Fri
// ---------------------------------------------------------------------------

#[test]
fn iceeu_monday_open() {
    let h = hours_for_exchange(Exchange::Iceeu);
    let t = lon((2026, 4, 20), (1, 0, 0));
    assert!(h.is_open(t), "ICEEU opens Mon 01:00 London");
}

#[test]
fn iceeu_trading_hours() {
    let h = hours_for_exchange(Exchange::Iceeu);
    assert!(
        h.is_open(lon((2026, 4, 20), (12, 0, 0))),
        "ICEEU Mon 12:00 London"
    );
}

#[test]
fn iceeu_close() {
    let h = hours_for_exchange(Exchange::Iceeu);
    let t = lon((2026, 4, 20), (23, 0, 0));
    assert!(!h.is_open(t), "ICEEU closes 23:00 London (end-exclusive)");
}

#[test]
fn iceeu_before_open() {
    let h = hours_for_exchange(Exchange::Iceeu);
    assert!(
        !h.is_open(lon((2026, 4, 20), (0, 30, 0))),
        "ICEEU closed before 01:00 London"
    );
}

#[test]
fn iceeu_weekend_closed() {
    let h = hours_for_exchange(Exchange::Iceeu);
    assert!(
        !h.is_open(lon((2026, 4, 25), (12, 0, 0))),
        "ICEEU closed Saturday"
    );
    assert!(
        !h.is_open(lon((2026, 4, 26), (12, 0, 0))),
        "ICEEU closed Sunday"
    );
}

// ---------------------------------------------------------------------------
// SGX Derivatives
//   Day: Mon–Fri 07:10–20:00 SGT
//   T+1: Mon–Fri 20:00–05:15 SGT (wrap)
// ---------------------------------------------------------------------------

#[test]
fn sgx_day_session() {
    let h = hours_for_exchange(Exchange::Sgx);
    let t = sgt((2026, 4, 20), (10, 0, 0));
    assert!(h.is_open(t), "SGX Mon 10:00 SGT");
    assert!(h.is_open_regular(t), "10:00 is regular (day)");
}

#[test]
fn sgx_day_open() {
    let h = hours_for_exchange(Exchange::Sgx);
    let t = sgt((2026, 4, 20), (7, 10, 0));
    assert!(h.is_open(t), "SGX opens Mon 07:10 SGT");
}

#[test]
fn sgx_day_close() {
    let h = hours_for_exchange(Exchange::Sgx);
    let t = sgt((2026, 4, 20), (20, 0, 0));
    assert!(
        !h.is_open_regular(t),
        "SGX day closes 20:00 SGT (end-exclusive)"
    );
}

#[test]
fn sgx_t1_wrap() {
    let h = hours_for_exchange(Exchange::Sgx);
    let t = sgt((2026, 4, 20), (22, 0, 0));
    assert!(h.is_open(t), "SGX T+1 wrap Mon 22:00 SGT");
    assert!(h.is_open_extended(t), "T+1 is extended");
}

#[test]
fn sgx_t1_wrap_into_tuesday() {
    let h = hours_for_exchange(Exchange::Sgx);
    let t = sgt((2026, 4, 21), (3, 0, 0));
    assert!(h.is_open(t), "SGX T+1 wrap Tue 03:00 SGT");
}

#[test]
fn sgx_t1_wrap_close() {
    let h = hours_for_exchange(Exchange::Sgx);
    let t = sgt((2026, 4, 21), (5, 15, 0));
    assert!(!h.is_open(t), "SGX T+1 closes 05:15 SGT (end-exclusive)");
}

#[test]
fn sgx_gap_before_day() {
    let h = hours_for_exchange(Exchange::Sgx);
    let t = sgt((2026, 4, 21), (6, 0, 0));
    assert!(!h.is_open(t), "SGX gap between 05:15 and 07:10 SGT");
}

#[test]
fn sgx_friday_t1_wrap_into_saturday() {
    let h = hours_for_exchange(Exchange::Sgx);
    let t = sgt((2026, 4, 25), (3, 0, 0));
    assert!(
        h.is_open(t),
        "SGX Fri T+1 wrap Sat 03:00 SGT (Friday's wrap extends into Sat)"
    );
}

#[test]
fn sgx_saturday_after_t1_close() {
    let h = hours_for_exchange(Exchange::Sgx);
    let t = sgt((2026, 4, 25), (6, 0, 0));
    assert!(!h.is_open(t), "SGX closed Sat after 05:15 SGT");
}

#[test]
fn sgx_sunday_closed() {
    let h = hours_for_exchange(Exchange::Sgx);
    assert!(
        !h.is_open(sgt((2026, 4, 26), (10, 0, 0))),
        "SGX closed Sunday"
    );
}

// ---------------------------------------------------------------------------
// CFE (Cboe Futures — VIX)
//   RTH 08:30–15:00 CT, post-settlement ETH 15:00–16:00 CT (Mon–Fri),
//   overnight Sun+Mon–Thu 17:00→08:30. Effective 2021-12-06; see the
//   venue-data section at the end of this file for the sources.
// ---------------------------------------------------------------------------

#[test]
fn cfe_sunday_overnight() {
    let h = hours_for_exchange(Exchange::Cfe);
    let t = ct((2026, 4, 19), (17, 0, 0));
    assert!(h.is_open(t), "CFE opens Sun 17:00 CT");
}

#[test]
fn cfe_rth() {
    let h = hours_for_exchange(Exchange::Cfe);
    let t = ct((2026, 4, 20), (10, 0, 0));
    assert!(h.is_open(t), "CFE RTH Mon 10:00 CT");
    assert!(h.is_open_regular(t), "10:00 is RTH");
}

#[test]
fn cfe_rth_close() {
    let h = hours_for_exchange(Exchange::Cfe);
    assert!(
        !h.is_open_regular(ct((2026, 4, 20), (15, 0, 0))),
        "CFE RTH ends 15:00 CT (end-exclusive) since 2021-12-06"
    );
    assert!(
        !h.is_open_regular(ct((2026, 4, 20), (15, 15, 0))),
        "15:15 CT is past the RTH close"
    );
}

#[test]
fn cfe_post_settlement_window() {
    let h = hours_for_exchange(Exchange::Cfe);
    let t = ct((2026, 4, 20), (15, 45, 0));
    assert!(h.is_open(t), "CFE post-settlement ETH 15:00–16:00 CT");
    assert!(h.is_open_extended(t), "post-settlement is extended");
}

#[test]
fn cfe_daily_maintenance() {
    let h = hours_for_exchange(Exchange::Cfe);
    let t = ct((2026, 4, 20), (16, 30, 0));
    assert!(!h.is_open(t), "CFE maintenance 16:00–17:00 CT");
}

#[test]
fn cfe_friday_close() {
    let h = hours_for_exchange(Exchange::Cfe);
    assert!(
        !h.is_open(ct((2026, 4, 24), (16, 0, 0))),
        "CFE closes Fri 16:00 CT"
    );
    assert!(
        !h.is_open(ct((2026, 4, 24), (17, 0, 0))),
        "CFE no Fri overnight"
    );
}

// ---------------------------------------------------------------------------
// Always-open venues (crypto perpetuals)
//
// These must be continuously open every day of the week with no maintenance
// gaps and no weekend boundaries.  The contract is explicit and separate
// from futures-session venues.
// ---------------------------------------------------------------------------

#[test]
fn binance_futures_always_open() {
    let h = hours_for_exchange(Exchange::BinanceFutures);
    assert!(
        h.is_open(utc((2026, 4, 20), (0, 0, 0))),
        "Binance Mon midnight UTC"
    );
    assert!(
        h.is_open(utc((2026, 4, 25), (12, 0, 0))),
        "Binance Sat 12:00 UTC"
    );
    assert!(
        h.is_open(utc((2026, 4, 26), (3, 0, 0))),
        "Binance Sun 03:00 UTC"
    );
}

#[test]
fn always_open_no_maintenance() {
    let h = hours_for_exchange(Exchange::BinanceFutures);
    for day in 19..26 {
        let t = utc((2026, 4, day), (12, 0, 0));
        assert!(
            !h.is_maintenance(t),
            "Always-open venue never in maintenance"
        );
    }
}

#[test]
fn always_open_no_daily_or_weekend_close() {
    let h = hours_for_exchange(Exchange::BinanceFutures);
    assert!(!h.has_daily_close, "Always-open venue has no daily close");
    assert!(
        !h.has_weekend_close,
        "Always-open venue has no weekend close"
    );
}

#[test]
fn always_open_distinct_from_futures() {
    let always_open = hours_for_exchange(Exchange::BinanceFutures);
    let cme = hours_for_exchange(Exchange::Cme);

    assert!(
        !always_open.has_daily_close,
        "Always-open venue no daily close"
    );
    assert!(cme.has_daily_close, "CME has daily close");
    assert!(
        !always_open.has_weekend_close,
        "Always-open venue no weekend close"
    );
    assert!(cme.has_weekend_close, "CME has weekend close");

    assert!(
        always_open.regular.len() == 1 && always_open.extended.is_empty(),
        "Always-open has single regular rule, no extended"
    );
    assert!(
        !cme.regular.is_empty() && !cme.extended.is_empty(),
        "CME has multi-rule session pattern"
    );
}

// ---------------------------------------------------------------------------
// Cross-venue: is_closed_all_day_on for weekend days
// ---------------------------------------------------------------------------

#[test]
fn futures_venues_closed_saturday() {
    let saturday = chrono::NaiveDate::from_ymd_opt(2026, 4, 25).unwrap();
    for &exch in &[
        Exchange::Cme,
        Exchange::Cbot,
        Exchange::Comex,
        Exchange::Nymex,
        Exchange::Eurex,
        Exchange::Iceus,
        Exchange::Iceeu,
        Exchange::Cfe,
    ] {
        let h = hours_for_exchange(exch);
        assert!(
            h.is_closed_all_day_on(saturday, SessionKind::Both),
            "{exch:?} should be closed all Saturday"
        );
    }
}

#[test]
fn sgx_saturday_partial_trading_from_friday_wrap() {
    let h = hours_for_exchange(Exchange::Sgx);
    let saturday = chrono::NaiveDate::from_ymd_opt(2026, 4, 25).unwrap();
    assert!(
        !h.is_closed_all_day_on(saturday, SessionKind::Both),
        "SGX Friday T+1 wrap extends into Saturday morning"
    );
    assert!(
        h.is_open(sgt((2026, 4, 25), (3, 0, 0))),
        "SGX Sat 03:00 SGT still in Friday's T+1 wrap"
    );
    assert!(
        !h.is_open(sgt((2026, 4, 25), (6, 0, 0))),
        "SGX closed Sat after 05:15 SGT"
    );
}

#[test]
fn futures_venues_sunday_morning_closed() {
    let sunday = chrono::NaiveDate::from_ymd_opt(2026, 4, 26).unwrap();
    for &exch in &[Exchange::Eurex, Exchange::Iceeu, Exchange::Sgx] {
        let h = hours_for_exchange(exch);
        assert!(
            h.is_closed_all_day_on(sunday, SessionKind::Both),
            "{exch:?} should be closed all Sunday"
        );
    }
}

#[test]
fn futures_venues_sunday_has_sessions() {
    let sunday = chrono::NaiveDate::from_ymd_opt(2026, 4, 26).unwrap();
    for &exch in &[
        Exchange::Cme,
        Exchange::Comex,
        Exchange::Nymex,
        Exchange::Iceus,
        Exchange::Cfe,
    ] {
        let h = hours_for_exchange(exch);
        assert!(
            !h.is_closed_all_day_on(sunday, SessionKind::Both),
            "{exch:?} has Sunday evening sessions"
        );
    }
}

// ---------------------------------------------------------------------------
// Session bounds sanity checks
// ---------------------------------------------------------------------------

#[test]
fn cme_session_bounds_monday_rth() {
    let h = hours_for_exchange(Exchange::Cme);
    let t = ct((2026, 4, 20), (10, 0, 0));
    let (open, close) = session_bounds(&h, t).expect("a session contains or follows t");
    assert_eq!(
        open,
        ct((2026, 4, 20), (8, 30, 0)),
        "Session opens at 08:30 CT"
    );
    assert_eq!(
        close,
        ct((2026, 4, 20), (15, 15, 0)),
        "Session closes at 15:15 CT"
    );
}

#[test]
fn cme_next_session_after_friday_close() {
    let h = hours_for_exchange(Exchange::Cme);
    let t = ct((2026, 4, 24), (16, 30, 0));
    let (open, _close) = next_session_after(&h, t).expect("the next session exists");
    assert_eq!(
        open,
        ct((2026, 4, 26), (17, 0, 0)),
        "Next session after Fri close is Sun 17:00 CT"
    );
}

#[test]
fn cbot_session_bounds_day_session() {
    let h = hours_for_exchange(Exchange::Cbot);
    let t = ct((2026, 4, 20), (10, 0, 0));
    let (open, close) = session_bounds(&h, t).expect("a session contains or follows t");
    assert_eq!(open, ct((2026, 4, 20), (8, 30, 0)));
    assert_eq!(close, ct((2026, 4, 20), (13, 20, 0)));
}

// Serde string compatibility: CamelCase variants must serialize to/from the same
// snake_case strings that existed before the rename, ensuring no wire-format break.
#[test]
fn exchange_serde_snake_case_nasdaq_bx() {
    let json = serde_json::to_string(&Exchange::NasdaqBx).unwrap();
    assert_eq!(json, "\"nasdaq_bx\"");
    let rt: Exchange = serde_json::from_str(&json).unwrap();
    assert_eq!(rt, Exchange::NasdaqBx);
}

#[test]
fn exchange_serde_snake_case_cboe_options_c1() {
    let json = serde_json::to_string(&Exchange::CboeOptionsC1).unwrap();
    assert_eq!(json, "\"cboe_options_c1\"");
    let rt: Exchange = serde_json::from_str(&json).unwrap();
    assert_eq!(rt, Exchange::CboeOptionsC1);
}

#[test]
fn exchange_serde_snake_case_ice_europe_commodities() {
    let json = serde_json::to_string(&Exchange::IceEuropeCommodities).unwrap();
    assert_eq!(json, "\"ice_europe_commodities\"");
    let rt: Exchange = serde_json::from_str(&json).unwrap();
    assert_eq!(rt, Exchange::IceEuropeCommodities);
}

#[test]
fn exchange_serde_snake_case_binance_futures() {
    let json = serde_json::to_string(&Exchange::BinanceFutures).unwrap();
    assert_eq!(json, "\"binance_futures\"");
    let rt: Exchange = serde_json::from_str(&json).unwrap();
    assert_eq!(rt, Exchange::BinanceFutures);
}

#[test]
fn exchange_serde_snake_case_intelligentcross_iqx() {
    let json = serde_json::to_string(&Exchange::IntelligentcrossIqx).unwrap();
    assert_eq!(json, "\"intelligentcross_iqx\"");
    let rt: Exchange = serde_json::from_str(&json).unwrap();
    assert_eq!(rt, Exchange::IntelligentcrossIqx);
}

// ---------------------------------------------------------------------------
// Monthly candle boundaries + MarketHoursKey -> MarketHours resolver.
// ---------------------------------------------------------------------------

#[test]
fn candle_end_monthly_returns_last_close_of_month() {
    // A mid-January instant resolves to January's final daily close: the
    // returned instant is in January (exchange-local) and the very next daily
    // close after it falls in February.
    let h = hours_for_exchange(Exchange::Cme);
    let mid_jan = ct((2026, 1, 15), (12, 0, 0));

    let close =
        candle_end(&h, mid_jan, CalendarResolution::Monthly).expect("January has daily closes");
    assert!(
        close > mid_jan,
        "monthly close must be after the input instant"
    );
    assert_eq!(
        close.with_timezone(&US::Central).month(),
        1,
        "monthly close stays in January (exchange-local)"
    );

    let next =
        candle_end(&h, close, CalendarResolution::Daily).expect("trading continues in February");
    assert_eq!(
        next.with_timezone(&US::Central).month(),
        2,
        "the close after the monthly boundary is in February"
    );
}

#[test]
fn candle_end_monthly_on_final_trading_day_returns_that_close() {
    // On a month's last trading day, the monthly boundary coincides with that
    // day's daily close. Friday 2026-01-30 is January's last trading day
    // (2026-01-31 is a Saturday).
    let h = hours_for_exchange(Exchange::Cme);
    let on_last_day = ct((2026, 1, 30), (9, 0, 0));
    assert_eq!(
        candle_end(&h, on_last_day, CalendarResolution::Monthly),
        candle_end(&h, on_last_day, CalendarResolution::Daily),
        "monthly close equals the daily close of the month's final trading day"
    );
}

#[test]
fn calendar_resolution_monthly_serde_round_trip() {
    let json = serde_json::to_string(&CalendarResolution::Monthly).unwrap();
    let rt: CalendarResolution = serde_json::from_str(&json).unwrap();
    assert_eq!(rt, CalendarResolution::Monthly);
}

#[test]
fn candle_end_monthly_year_boundary() {
    // A late-December instant resolves to December's final close, not January:
    // the boundary stays in December 2026 and the next close rolls to 2027.
    let h = hours_for_exchange(Exchange::Cme);
    let late_dec = ct((2026, 12, 20), (12, 0, 0));

    let close =
        candle_end(&h, late_dec, CalendarResolution::Monthly).expect("December has daily closes");
    let local = close.with_timezone(&US::Central);
    assert_eq!(local.month(), 12, "boundary stays in December");
    assert_eq!(local.year(), 2026, "boundary stays in 2026");

    let next = candle_end(&h, close, CalendarResolution::Daily)
        .expect("trading continues in January 2027");
    let next_local = next.with_timezone(&US::Central);
    assert_eq!(next_local.month(), 1, "next close rolls into January");
    assert_eq!(next_local.year(), 2027, "next close rolls into 2027");
}

#[test]
fn candle_end_monthly_mid_month_is_idempotent_within_month() {
    // Two instants in the same month resolve to the same monthly boundary.
    let h = hours_for_exchange(Exchange::Cme);
    let early = ct((2026, 3, 3), (9, 0, 0));
    let later = ct((2026, 3, 25), (14, 0, 0));
    assert_eq!(
        candle_end(&h, early, CalendarResolution::Monthly),
        candle_end(&h, later, CalendarResolution::Monthly),
        "any two instants in March share one monthly boundary"
    );
}

#[test]
fn candle_start_daily_uses_the_overnight_session_open() {
    let hours = hours_for_market_hours_key(MarketHoursKey::GlobexEquityIndex);
    let first_trade = ct((2026, 1, 29), (18, 0, 0));

    assert_eq!(
        candle_start(&hours, first_trade, CalendarResolution::Daily),
        Some(ct((2026, 1, 29), (17, 0, 0))),
        "the trading-day candle starts at the catalog session open, not its first trade"
    );
    assert_eq!(
        candle_end(&hours, first_trade, CalendarResolution::Daily),
        Some(ct((2026, 1, 30), (16, 0, 0))),
        "the paired daily close remains the following civil day"
    );
}

#[test]
fn candle_start_resolves_the_post_dst_globex_open() {
    let hours = hours_for_market_hours_key(MarketHoursKey::GlobexEquityIndex);
    let post_spring_forward_trade = ct((2026, 3, 8), (18, 0, 0));
    let start = candle_start(&hours, post_spring_forward_trade, CalendarResolution::Daily)
        .expect("the Globex week is open");

    assert_eq!(start, ct((2026, 3, 8), (17, 0, 0)));
    assert_eq!(
        start,
        utc((2026, 3, 8), (22, 0, 0)),
        "17:00 CT is 22:00 UTC after the spring DST transition"
    );
}

#[test]
fn candle_start_monthly_can_open_in_the_preceding_civil_month() {
    let hours = hours_for_market_hours_key(MarketHoursKey::GlobexEquityIndex);
    let first_june_session_trade = ct((2026, 5, 31), (18, 0, 0));

    assert_eq!(
        candle_start(
            &hours,
            first_june_session_trade,
            CalendarResolution::Monthly,
        ),
        Some(ct((2026, 5, 31), (17, 0, 0))),
        "June's first trading session opens on the final civil day of May"
    );
    assert_eq!(
        candle_start(
            &hours,
            ct((2026, 6, 30), (12, 0, 0)),
            CalendarResolution::Monthly,
        ),
        Some(ct((2026, 5, 31), (17, 0, 0))),
        "every instant in the same trading month shares one canonical start"
    );
}

#[test]
fn hours_for_market_hours_key_matches_session_profile() {
    // The resolver reproduces the session profile's open/closed decisions; it is
    // a field copy of the same static table, never a second source of truth.
    for key in [
        MarketHoursKey::GlobexEquityIndex,
        MarketHoursKey::GlobexEnergy,
        MarketHoursKey::Eurex,
    ] {
        let hours = hours_for_market_hours_key(key);
        let profile = session_profile(key);
        for t in [
            utc((2026, 1, 5), (10, 0, 0)),
            utc((2026, 1, 5), (23, 30, 0)),
            utc((2026, 1, 4), (12, 0, 0)),
            utc((2026, 1, 3), (12, 0, 0)),
        ] {
            assert_eq!(
                hours.is_open(t),
                profile.is_open(t),
                "key={key:?} t={t} open mismatch"
            );
        }
    }
}

#[test]
fn hours_for_market_hours_key_drives_calendar_boundaries() {
    // The resolver exists for the calendar query surface (`candle_end`,
    // `session_bounds`, weekly/monthly consolidation), so validate the boundary
    // math on its hours — not only `is_open`. A field-copy bug that preserved
    // `is_open` while shifting boundaries would otherwise pass (SH-VAL-01).
    let hours = hours_for_market_hours_key(MarketHoursKey::GlobexEquityIndex);

    // Daily: a weekday instant resolves to a strictly-later close.
    let wed = ct((2026, 1, 7), (9, 0, 0));
    assert!(
        candle_end(&hours, wed, CalendarResolution::Daily).expect("mid-week close exists") > wed,
        "daily close must be after the input instant"
    );

    // Weekly: idempotent within an ISO week, distinct across weeks.
    let wed_weekly = candle_end(&hours, wed, CalendarResolution::Weekly);
    assert!(wed_weekly.is_some(), "the trading week has a weekly close");
    let thu = ct((2026, 1, 8), (9, 0, 0));
    assert_eq!(
        candle_end(&hours, thu, CalendarResolution::Weekly),
        wed_weekly,
        "Wed and Thu of the same week share one weekly boundary"
    );
    let next_wed = ct((2026, 1, 14), (9, 0, 0));
    assert_ne!(
        candle_end(&hours, next_wed, CalendarResolution::Weekly),
        wed_weekly,
        "the following week has a distinct weekly boundary"
    );

    // Monthly: a mid-January instant resolves to a January close whose next daily
    // close rolls into February.
    let mid_jan = ct((2026, 1, 15), (12, 0, 0));
    let monthly =
        candle_end(&hours, mid_jan, CalendarResolution::Monthly).expect("January has daily closes");
    assert_eq!(
        monthly.with_timezone(&US::Central).month(),
        1,
        "monthly boundary stays in January (exchange-local)"
    );
    assert_eq!(
        candle_end(&hours, monthly, CalendarResolution::Daily)
            .expect("trading continues in February")
            .with_timezone(&US::Central)
            .month(),
        2,
        "the next daily close after the monthly boundary is in February"
    );
}

#[test]
fn normal_week_open_seconds_unions_overlapping_session_rules() {
    let hours = hours_for_market_hours_key(MarketHoursKey::GlobexEquityIndex);

    assert_eq!(
        hours.normal_week_open_seconds(),
        409_500,
        "five equity-index sessions exclude both the daily halt and the 15-minute pause"
    );

    let always_open = hours_for_market_hours_key(MarketHoursKey::AlwaysOpen);
    assert_eq!(
        always_open.normal_week_open_seconds(),
        7 * 86_400,
        "overlapping coverage must never count more than one complete week"
    );
}

#[test]
fn market_hours_derives_eq() {
    // Two profiles built for the same exchange compare equal (the derive Task 3
    // depends on); distinct venues do not.
    let a = hours_for_exchange(Exchange::Cme);
    let b = hours_for_exchange(Exchange::Cme);
    assert_eq!(a, b, "same exchange yields equal hours");
    assert_ne!(
        a,
        hours_for_exchange(Exchange::Eurex),
        "distinct exchanges yield unequal hours"
    );
}

// ---------------------------------------------------------------------------
// Overnight-wrap open-side regression
//
// A wrap rule (`open_ssm > close_ssm`) that is enabled on a given weekday
// contributes only its OPEN side on that day. Its close belongs to the previous
// day's instance of the same rule, so a venue whose previous local day ran no
// session must read closed right up to today's open.
//
// COMEX/NYMEX (17:00→16:00 CT) and ICE US (20:00→18:00 ET) are the venues that
// expose this: their wrap close falls late in the day, so every Sunday instant
// before that close previously reported open even though Saturday never opened
// a session. CME is pinned alongside them because its wrap closes at 08:30,
// which makes it insensitive to the same defect — its behaviour must not move.
// ---------------------------------------------------------------------------

#[test]
fn comex_sunday_morning_closed_before_globex_open() {
    let h = hours_for_exchange(Exchange::Comex);
    let t = ct((2026, 4, 19), (10, 0, 0));
    assert!(
        !h.is_open(t),
        "COMEX closed Sun 10:00 CT: Saturday opened no session to wrap into Sunday"
    );
}

#[test]
fn nymex_sunday_morning_closed_before_globex_open() {
    let h = hours_for_exchange(Exchange::Nymex);
    let t = ct((2026, 4, 19), (10, 0, 0));
    assert!(
        !h.is_open(t),
        "NYMEX closed Sun 10:00 CT: Saturday opened no session to wrap into Sunday"
    );
}

#[test]
fn iceus_sunday_morning_closed_before_open() {
    let h = hours_for_exchange(Exchange::Iceus);
    let t = et((2026, 4, 19), (10, 0, 0));
    assert!(
        !h.is_open(t),
        "ICE US closed Sun 10:00 ET: Saturday opened no session to wrap into Sunday"
    );
}

#[test]
fn cme_sunday_morning_closed_before_globex_open() {
    let h = hours_for_exchange(Exchange::Cme);
    let t = ct((2026, 4, 19), (10, 0, 0));
    assert!(!h.is_open(t), "CME closed Sun 10:00 CT");
}

#[test]
fn sunday_morning_is_open_agrees_with_session_bounds() {
    // The defect showed up as two public queries disagreeing: `is_open` said
    // open while `session_bounds` returned a session starting hours later.
    for (exchange, instant) in [
        (Exchange::Comex, ct((2026, 4, 19), (10, 0, 0))),
        (Exchange::Nymex, ct((2026, 4, 19), (10, 0, 0))),
        (Exchange::Iceus, et((2026, 4, 19), (10, 0, 0))),
        (Exchange::Cme, ct((2026, 4, 19), (10, 0, 0))),
    ] {
        let h = hours_for_exchange(exchange);
        let (open, close) = session_bounds(&h, instant).expect("a session follows the weekend");
        let contained = open <= instant && instant < close;
        assert_eq!(
            h.is_open(instant),
            contained,
            "{exchange:?}: is_open disagrees with session_bounds at {instant} \
             (bounds {open}..{close})"
        );
    }
}

#[test]
fn wrap_venues_reopen_at_their_published_sunday_open() {
    // The fix must not close a venue that genuinely is open: each of these is
    // the first instant of the Sunday-evening session.
    for (exchange, instant) in [
        (Exchange::Comex, ct((2026, 4, 19), (17, 0, 0))),
        (Exchange::Nymex, ct((2026, 4, 19), (17, 0, 0))),
        (Exchange::Iceus, et((2026, 4, 19), (20, 0, 0))),
        (Exchange::Cme, ct((2026, 4, 19), (17, 0, 0))),
    ] {
        let h = hours_for_exchange(exchange);
        assert!(h.is_open(instant), "{exchange:?} open at its Sunday open");
    }
}

// ---------------------------------------------------------------------------
// Venue-data corrections (primary-source verified)
// ---------------------------------------------------------------------------

// IEX runs narrower extended hours than the Reg NMS default: "System Hours" are
// 08:00–17:00 ET, with pre-market 08:00–09:30 and post-market 16:00–17:00.
// Source: IEX Exchange, "Trading Hours & Holidays"
// (https://www.iex.io/resources/trading/trading-hours-holidays); Investors
// Exchange Rule Book Rule 1.160(z)/(aa)/(gg).
#[test]
fn iex_premarket_opens_at_0800_not_0400() {
    let h = hours_for_exchange(Exchange::Iex);
    assert!(
        !h.is_open(et((2026, 4, 20), (4, 0, 0))),
        "IEX closed 04:00 ET; System Hours start at 08:00"
    );
    assert!(
        !h.is_open(et((2026, 4, 20), (7, 59, 0))),
        "IEX closed 07:59 ET"
    );
    assert!(
        h.is_open_extended(et((2026, 4, 20), (8, 0, 0))),
        "IEX pre-market opens 08:00 ET"
    );
}

#[test]
fn iex_postmarket_closes_at_1700_not_2000() {
    let h = hours_for_exchange(Exchange::Iex);
    assert!(
        h.is_open_extended(et((2026, 4, 20), (16, 30, 0))),
        "IEX post-market runs 16:00–17:00 ET"
    );
    assert!(
        !h.is_open(et((2026, 4, 20), (17, 0, 0))),
        "IEX closed at 17:00 ET (end-exclusive)"
    );
    assert!(
        !h.is_open(et((2026, 4, 20), (19, 0, 0))),
        "IEX closed 19:00 ET; it runs no 20:00 post-market"
    );
}

#[test]
fn iex_had_no_extended_sessions_before_2015_08_21() {
    // IEX Trading Alert #2015-015: "Effective Friday, August 21, 2015, IEX will
    // begin to rollout extended hours trading with Pre-Market and Post-Market
    // Sessions." Pins both sides of the cutover so the arm in
    // `hours_for_exchange_as_of` cannot be inverted unnoticed.
    let before = hours_for_exchange_as_of(Exchange::Iex, et((2015, 8, 20), (12, 0, 0)));
    assert!(
        !before.is_open(et((2015, 8, 20), (8, 30, 0))),
        "no pre-market before the rollout"
    );
    assert!(
        !before.is_open(et((2015, 8, 20), (16, 30, 0))),
        "no post-market before the rollout"
    );
    assert!(
        before.is_open_regular(et((2015, 8, 20), (10, 0, 0))),
        "the regular session is unaffected by the cutover"
    );

    let after = hours_for_exchange_as_of(Exchange::Iex, et((2015, 8, 21), (12, 0, 0)));
    assert!(
        after.is_open_extended(et((2015, 8, 21), (8, 30, 0))),
        "pre-market is live from the rollout date"
    );
}

#[test]
fn iex_extended_hours_differ_from_the_generic_us_equities_profile() {
    let iex = hours_for_exchange(Exchange::Iex);
    let generic = hours_for_exchange(Exchange::Nasdaq);
    assert_eq!(
        iex.regular, generic.regular,
        "IEX shares the Reg NMS regular session"
    );
    assert_ne!(
        iex.extended, generic.extended,
        "IEX must not inherit the 04:00/20:00 generic extended windows"
    );
    assert!(
        generic.is_open(et((2026, 4, 20), (4, 0, 0))),
        "the generic profile does trade at 04:00 ET, so the contrast is real"
    );
}

// Blue Ocean ATS reached production on 2021-10-05; the June 2021 beta has no
// day-level primary source and is deliberately excluded.
// Source: Blue Ocean Technologies, "Announcing Launch of Blue Ocean ATS
// Afterhours Trading" (2021-10-05).
#[test]
fn blue_ocean_has_no_sessions_before_its_production_launch() {
    let before = hours_for_exchange_as_of(Exchange::BlueOceanAts, et((2021, 10, 4), (22, 0, 0)));
    assert!(
        before.regular.is_empty() && before.extended.is_empty(),
        "Blue Ocean ATS had not launched on 2021-10-04"
    );

    let after = hours_for_exchange_as_of(Exchange::BlueOceanAts, et((2021, 10, 5), (22, 0, 0)));
    assert!(
        !after.extended.is_empty(),
        "Blue Ocean ATS trades from its 2021-10-05 production launch"
    );
    assert!(
        after.is_open(et((2021, 10, 5), (22, 0, 0))),
        "Blue Ocean session runs 20:00–04:00 ET"
    );
}

#[test]
fn blue_ocean_runs_sunday_through_thursday_only() {
    // No Friday-evening session: the NYSE TRF cannot report the trades on
    // Saturday. Source: Blue Ocean SEC Form ATS-N, "Hours of Operations".
    let h = hours_for_exchange(Exchange::BlueOceanAts);
    assert!(
        h.is_open(et((2026, 4, 19), (22, 0, 0))),
        "Sunday evening session trades"
    );
    assert!(
        !h.is_open(et((2026, 4, 24), (22, 0, 0))),
        "no Friday-evening session"
    );
    assert!(
        !h.is_open(et((2026, 4, 25), (2, 0, 0))),
        "nothing wraps into Saturday morning"
    );
}

// CFE removed the 15:15–15:30 queuing period and moved the RTH close to 15:00
// on 2021-12-06, so RTH now flows seamlessly into post-settlement ETH.
// Sources: Cboe notice C2021102603 ("Effective December 6, 2021 … CFE will
// eliminate the queuing period which occurs between 3:15 p.m. CT and 3:30 p.m.
// CT, Monday through Friday … and redefine regular trading hours as 8:30 a.m.
// CT to 3:00 p.m. CT"); CFE rule filing CFE-2021-028 (2021-11-04); CFE Rulebook
// Rule 1202, whose amendment history stamps "December 6, 2021 (21-028)".
#[test]
fn cfe_rth_transitions_seamlessly_into_post_settlement_at_1500() {
    let h = hours_for_exchange(Exchange::Cfe);
    let close = ct((2026, 4, 20), (15, 0, 0));
    assert!(
        !h.is_open_regular(close),
        "RTH is end-exclusive at 15:00 CT"
    );
    assert!(
        h.is_open_extended(close),
        "ETH takes over at 15:00 CT with no pause"
    );
    assert!(h.is_open(close), "the venue never closes at 15:00 CT");
}

#[test]
fn cfe_has_no_queuing_gap_between_1515_and_1530() {
    // The old model left 15:15–15:30 closed. That queuing period is gone.
    let h = hours_for_exchange(Exchange::Cfe);
    for minute in [15, 20, 29] {
        let t = ct((2026, 4, 20), (15, minute, 0));
        assert!(
            h.is_open(t),
            "CFE open at 15:{minute:02} CT since 2021-12-06"
        );
    }
}

#[test]
fn cfe_post_settlement_runs_on_friday_too() {
    // The session is Monday through Friday in both the pre- and post-2021
    // rulebook charts; the old model wrongly limited it to Mon–Thu.
    let h = hours_for_exchange(Exchange::Cfe);
    assert!(
        h.is_open(ct((2026, 4, 24), (15, 45, 0))),
        "CFE post-settlement trades on Friday"
    );
    assert!(
        !h.is_open(ct((2026, 4, 24), (17, 0, 0))),
        "but there is still no Friday overnight session"
    );
}

#[test]
fn cfe_before_the_2021_cutover_keeps_the_queuing_gap() {
    let before = hours_for_exchange_as_of(Exchange::Cfe, ct((2021, 12, 3), (12, 0, 0)));
    assert!(
        before.is_open_regular(ct((2021, 12, 3), (15, 10, 0))),
        "RTH ran to 15:15 CT before the cutover"
    );
    assert!(
        !before.is_open(ct((2021, 12, 3), (15, 20, 0))),
        "15:15–15:30 was a queuing period with no trading"
    );

    let after = hours_for_exchange_as_of(Exchange::Cfe, ct((2021, 12, 6), (12, 0, 0)));
    assert!(
        !after.is_open_regular(ct((2021, 12, 6), (15, 10, 0))),
        "RTH ends 15:00 CT from the cutover"
    );
    assert!(
        after.is_open(ct((2021, 12, 6), (15, 20, 0))),
        "the queuing period is replaced by extended trading"
    );
}

// SIX Swiss Exchange shares segments: continuous trading ends 17:20, closing
// auction 17:20–17:30, Trading-At-Last to 17:40. The 17:30–17:35 auction the
// model previously used belongs to the ETF/ETP segments, which have no TAL.
// Source: SIX Group "Trading hours" and the SIX Swiss Exchange Trading Guide
// ("Continuous Trading 09:00 - 17:20 CET / Closing Auction 17:20 - 17:30 CET /
// Trading-At-Last … End: 17:40 CET").
#[test]
fn six_continuous_trading_ends_at_1720() {
    let h = hours_for_exchange(Exchange::Six);
    assert!(
        h.is_open_regular(cet((2026, 4, 20), (17, 19, 0))),
        "SIX continuous trading runs to 17:20"
    );
    assert!(
        !h.is_open_regular(cet((2026, 4, 20), (17, 20, 0))),
        "17:20 is the end-exclusive continuous close"
    );
}

#[test]
fn six_closing_auction_and_trading_at_last_run_to_1740() {
    let h = hours_for_exchange(Exchange::Six);
    assert!(
        h.is_open_extended(cet((2026, 4, 20), (17, 25, 0))),
        "closing auction 17:20–17:30"
    );
    assert!(
        h.is_open_extended(cet((2026, 4, 20), (17, 35, 0))),
        "Trading-At-Last 17:30–17:40"
    );
    assert!(
        !h.is_open(cet((2026, 4, 20), (17, 40, 0))),
        "SIX is done at 17:40 (end-exclusive)"
    );
}

#[test]
fn six_does_not_share_the_xetra_schedule() {
    let six = hours_for_exchange(Exchange::Six);
    let xetra = hours_for_exchange(Exchange::Xetra);
    assert_ne!(
        six.regular, xetra.regular,
        "SIX closes continuous trading 10 minutes before Xetra"
    );
    assert!(
        xetra.is_open_regular(cet((2026, 4, 20), (17, 25, 0))),
        "Xetra still trades continuously at 17:25, so the contrast is real"
    );
}

// ---------------------------------------------------------------------------
// candle_start_with — kind-aware period starts
// ---------------------------------------------------------------------------

#[test]
fn candle_start_with_regular_kind_anchors_the_trading_day_at_rth() {
    let hours = hours_for_market_hours_key(MarketHoursKey::GlobexEquityIndex);
    let monday_mid_rth = ct((2026, 4, 20), (10, 0, 0));

    // Consulting only regular sessions, the Monday trading day runs
    // 08:30–15:15 CT; the overnight wrap belongs to the extended set.
    assert_eq!(
        candle_start_with(
            &hours,
            monday_mid_rth,
            CalendarResolution::Daily,
            SessionKind::Regular
        ),
        Some(ct((2026, 4, 20), (8, 30, 0))),
        "Regular-kind daily start is the RTH open, not the Globex overnight open"
    );
    assert_eq!(
        candle_end_with(
            &hours,
            monday_mid_rth,
            CalendarResolution::Daily,
            SessionKind::Regular
        ),
        Some(ct((2026, 4, 20), (15, 15, 0))),
        "Regular-kind daily end is the RTH close"
    );
}

#[test]
fn candle_start_with_extended_kind_anchors_the_trading_day_at_the_globex_open() {
    let hours = hours_for_market_hours_key(MarketHoursKey::GlobexEquityIndex);
    let monday_mid_rth = ct((2026, 4, 20), (10, 0, 0));

    // Consulting only extended sessions, the Monday trading day opened with
    // Sunday's 17:00 CT Globex wrap and its last close is the 15:30-16:00
    // post-settlement window.
    assert_eq!(
        candle_start_with(
            &hours,
            monday_mid_rth,
            CalendarResolution::Daily,
            SessionKind::Extended
        ),
        Some(ct((2026, 4, 19), (17, 0, 0))),
        "Extended-kind daily start is the Sunday Globex open"
    );
    assert_eq!(
        candle_end_with(
            &hours,
            monday_mid_rth,
            CalendarResolution::Daily,
            SessionKind::Extended
        ),
        Some(ct((2026, 4, 20), (16, 0, 0))),
        "Extended-kind daily end is the 16:00 CT extended close"
    );
}

#[test]
fn candle_start_with_both_matches_candle_start() {
    let hours = hours_for_market_hours_key(MarketHoursKey::GlobexEquityIndex);
    for (res, t) in [
        (CalendarResolution::Daily, ct((2026, 1, 29), (18, 0, 0))),
        (
            CalendarResolution::Minutes(5),
            ct((2026, 4, 20), (10, 0, 0)),
        ),
        (CalendarResolution::Weekly, ct((2026, 4, 20), (10, 0, 0))),
    ] {
        assert_eq!(
            candle_start_with(&hours, t, res, SessionKind::Both),
            candle_start(&hours, t, res),
            "candle_start must be exactly candle_start_with over Both ({res:?})"
        );
    }
}

// ---------------------------------------------------------------------------
// Venue data verified against primary sources (2026-08 pass): NYSE-family and
// Cboe early sessions, the Nasdaq Nordic closes, and the European post-close
// price-formation windows. Reference week: Monday 2026-04-20.
// ---------------------------------------------------------------------------

/// Builds a Helsinki (EET/EEST) local instant.
fn hel(date: (i32, u32, u32), time: (u32, u32, u32)) -> DateTime<Utc> {
    Europe::Helsinki
        .with_ymd_and_hms(date.0, date.1, date.2, time.0, time.1, time.2)
        .single()
        .expect("valid Helsinki instant")
        .with_timezone(&Utc)
}

#[test]
fn nyse_trades_the_core_session_only() {
    // nyse.com "Trading Hours": NYSE lists a core session 9:30-16:00 ET and
    // no early or late session; extended trading in NYSE Group happens on
    // Arca, American, National, and Texas.
    let hours = hours_for_exchange(Exchange::Nyse);
    assert!(
        hours.extended.is_empty(),
        "NYSE must have no extended rules"
    );
    assert!(!hours.is_open(et((2026, 4, 20), (8, 0, 0))));
    assert!(!hours.is_open(et((2026, 4, 20), (16, 30, 0))));
    assert!(hours.is_open_regular(et((2026, 4, 20), (10, 0, 0))));
}

#[test]
fn nyse_american_and_cboe_byx_premarket_opens_at_0700_not_0400() {
    // nyse.com "Trading Hours": American/National early session 7:00-9:30 ET.
    // cboe.com "Hours & Holidays": BYX/EDGA early session begins 7:00 a.m.
    for exch in [
        Exchange::NyseAmerican,
        Exchange::NyseNational,
        Exchange::CboeByx,
        Exchange::CboeEdga,
    ] {
        let hours = hours_for_exchange(exch);
        assert!(
            !hours.is_open(et((2026, 4, 20), (5, 0, 0))),
            "{exch:?} must be closed at 05:00 ET"
        );
        assert!(
            !hours.is_open(et((2026, 4, 20), (6, 59, 59))),
            "{exch:?} must be closed just before 07:00 ET"
        );
        assert!(
            hours.is_open_extended(et((2026, 4, 20), (7, 0, 0))),
            "{exch:?} early session must open 07:00 ET"
        );
        assert!(
            hours.is_open_extended(et((2026, 4, 20), (19, 59, 59))),
            "{exch:?} late session must run to 20:00 ET"
        );
    }
}

#[test]
fn bzx_and_edgx_premarket_opens_at_0400_today() {
    // cboe.com "Hours & Holidays": BZX/EDGX early session begins 4:00 a.m. ET.
    for exch in [Exchange::CboeBzx, Exchange::CboeEdgx] {
        let hours = hours_for_exchange(exch);
        assert!(!hours.is_open(et((2026, 4, 20), (3, 59, 59))));
        assert!(
            hours.is_open_extended(et((2026, 4, 20), (4, 0, 0))),
            "{exch:?} early session must open 04:00 ET"
        );
    }
}

#[test]
fn edgx_early_session_started_0700_before_2021_03_08() {
    // Cboe press release (2021-02-08): early trading from 4:00 a.m. ET
    // effective Monday, March 8, 2021.
    let before = hours_for_exchange_as_of(Exchange::CboeEdgx, et((2021, 3, 5), (12, 0, 0)));
    assert!(!before.is_open(et((2021, 3, 5), (5, 0, 0))));
    assert!(before.is_open_extended(et((2021, 3, 5), (7, 30, 0))));

    let after = hours_for_exchange_as_of(Exchange::CboeEdgx, et((2021, 3, 8), (12, 0, 0)));
    assert!(after.is_open_extended(et((2021, 3, 8), (5, 0, 0))));
}

#[test]
fn bzx_early_session_started_0700_before_2025_05_01() {
    // Cboe release notice #54236: "Effective May 1, 2025 … will commence the
    // Early Trading Session at 4:00 a.m. ET. Currently, BZX … commences the
    // Early Trading Session at 7:00 a.m. ET."
    let before = hours_for_exchange_as_of(Exchange::CboeBzx, et((2025, 4, 30), (12, 0, 0)));
    assert!(!before.is_open(et((2025, 4, 30), (5, 0, 0))));
    assert!(before.is_open_extended(et((2025, 4, 30), (7, 30, 0))));

    let after = hours_for_exchange_as_of(Exchange::CboeBzx, et((2025, 5, 1), (12, 0, 0)));
    assert!(after.is_open_extended(et((2025, 5, 1), (5, 0, 0))));
}

#[test]
fn nasdaq_stockholm_continuous_ends_1725() {
    // FESE 2025 hours: Stockholm continuous 09:00-17:25, closing call to ~17:30.
    let hours = hours_for_exchange(Exchange::NasdaqStockholm);
    assert!(hours.is_open_regular(cet((2026, 4, 20), (17, 24, 59))));
    assert!(!hours.is_open_regular(cet((2026, 4, 20), (17, 25, 0))));
    assert!(hours.is_open_extended(cet((2026, 4, 20), (17, 27, 0))));
    assert!(!hours.is_open(cet((2026, 4, 20), (17, 30, 0))));
    // Pre-opening from 08:00.
    assert!(hours.is_open_extended(cet((2026, 4, 20), (8, 30, 0))));
}

#[test]
fn nasdaq_helsinki_trades_1000_to_1825_local_eet() {
    // FESE 2025 hours: Helsinki 10:00-18:25 EET — one hour ahead of the CET
    // books in local terms, simultaneous in absolute time.
    let hours = hours_for_exchange(Exchange::NasdaqHelsinki);
    assert!(!hours.is_open_regular(hel((2026, 4, 20), (9, 30, 0))));
    assert!(hours.is_open_extended(hel((2026, 4, 20), (9, 30, 0))));
    assert!(hours.is_open_regular(hel((2026, 4, 20), (10, 0, 0))));
    assert!(hours.is_open_regular(hel((2026, 4, 20), (18, 24, 59))));
    assert!(!hours.is_open_regular(hel((2026, 4, 20), (18, 25, 0))));
    assert!(!hours.is_open(hel((2026, 4, 20), (18, 30, 0))));
    // Helsinki's regular open coincides with Stockholm's in absolute time.
    assert_eq!(
        hel((2026, 4, 20), (10, 0, 0)),
        cet((2026, 4, 20), (9, 0, 0)),
        "the Nordic books align on CET"
    );
}

#[test]
fn nasdaq_copenhagen_closes_1655_with_optional_trade_at_close() {
    // FESE 2025 hours: Copenhagen continuous 09:00-16:55, closing call to
    // ~17:00, Trade-at-Close 17:00-17:10.
    let hours = hours_for_exchange(Exchange::NasdaqCopenhagen);
    assert!(hours.is_open_regular(cet((2026, 4, 20), (16, 54, 59))));
    assert!(!hours.is_open_regular(cet((2026, 4, 20), (16, 55, 0))));
    assert!(hours.is_open_extended(cet((2026, 4, 20), (17, 5, 0))));
    assert!(!hours.is_open(cet((2026, 4, 20), (17, 10, 0))));
}

#[test]
fn euronext_trading_at_last_runs_to_1740() {
    // FESE 2025 hours / Euronext cash documentation: closing auction uncross
    // ~17:35, Trading-at-Last until 17:40.
    for exch in [
        Exchange::EuronextParis,
        Exchange::EuronextAmsterdam,
        Exchange::EuronextBrussels,
        Exchange::EuronextMilan,
    ] {
        let hours = hours_for_exchange(exch);
        assert!(
            hours.is_open_extended(cet((2026, 4, 20), (17, 37, 0))),
            "{exch:?} must run Trading-at-Last 17:35-17:40"
        );
        assert!(
            !hours.is_open(cet((2026, 4, 20), (17, 40, 0))),
            "{exch:?} must be closed at 17:40 (end-exclusive)"
        );
    }
}

#[test]
fn euronext_lisbon_trading_at_last_runs_to_1740_local_time() {
    // Lisbon shares the Euronext schedule but keeps its own zone, one hour
    // behind the continental venues year-round, so the TAL window must be
    // probed in Europe/Lisbon local time — 17:37 on the continental clock is
    // 16:37 in Lisbon, outside the window.
    let hours = hours_for_exchange(Exchange::EuronextLisbon);
    let lis = |h: u32, m: u32| {
        Europe::Lisbon
            .with_ymd_and_hms(2026, 4, 20, h, m, 0)
            .single()
            .expect("valid Lisbon instant")
            .with_timezone(&Utc)
    };
    assert!(hours.is_open_extended(lis(17, 37)));
    assert!(
        !hours.is_open(lis(17, 40)),
        "closed at 17:40 (end-exclusive)"
    );
}

#[test]
fn euronext_dublin_continuous_ends_1728() {
    // FESE 2025 hours: Dublin continuous 09:00-17:28 (WET), closing auction
    // 17:30, Trading-at-Last 17:30-17:40.
    let hours = hours_for_exchange(Exchange::EuronextDublin);
    let dub = |h: u32, m: u32, s: u32| {
        Europe::Dublin
            .with_ymd_and_hms(2026, 4, 20, h, m, s)
            .single()
            .expect("valid Dublin instant")
            .with_timezone(&Utc)
    };
    assert!(hours.is_open_regular(dub(17, 27, 59)));
    assert!(!hours.is_open_regular(dub(17, 28, 0)));
    assert!(hours.is_open_extended(dub(17, 29, 0)));
    assert!(hours.is_open_extended(dub(17, 35, 0)));
    assert!(!hours.is_open(dub(17, 40, 0)));
}

#[test]
fn lse_closing_price_crossing_runs_to_1640() {
    // LSE Guide to the Trading System (MIT201): Closing Auction Call
    // 16:30-16:35, Closing Price Crossing Session to 16:40.
    let hours = hours_for_exchange(Exchange::Lse);
    assert!(hours.is_open_extended(lon((2026, 4, 20), (16, 37, 0))));
    assert!(!hours.is_open(lon((2026, 4, 20), (16, 40, 0))));
}

#[test]
fn xetra_bme_and_vienna_run_post_close_price_formation() {
    // FESE 2025 hours: Xetra Trade-at-Close 17:35-17:45; BME Trading-at-Last
    // 17:35-17:45; Vienna Trade-at-Close 17:35-17:45.
    for exch in [Exchange::Xetra, Exchange::Bme, Exchange::Vienna] {
        let hours = hours_for_exchange(exch);
        assert!(
            hours.is_open_extended(cet((2026, 4, 20), (17, 40, 0))),
            "{exch:?} must run post-close price formation 17:35-17:45"
        );
        assert!(
            !hours.is_open(cet((2026, 4, 20), (17, 45, 0))),
            "{exch:?} must be closed at 17:45 (end-exclusive)"
        );
    }
}
