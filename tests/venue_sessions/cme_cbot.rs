// SPDX-License-Identifier: MIT-0

//! CME and CBOT normal-week session boundaries.

use super::prelude::*;

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
        "16:30 is inside the sub-six-hour 16:00→17:00 break"
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

// CME's October-2009 product guide records a Monday–Thursday 15:30–16:30 CT
// slice at the audit floor. Chadv12-423 moved the same-trade-date close to
// 16:15 and added the slice on Fridays effective Sunday 2012-11-18.
// https://www.cmegroup.com/content/dam/cmegroup/education/modules/files/EQ240_EQ_for_AIT.pdf
// https://www.cmegroup.com/education/files/eq-trading-hours.pdf
// https://www.cmegroup.com/tools-information/lookups/advisories/clearing/Chadv12-423.html
#[test]
fn cme_post_halt_session_changed_on_2012_11_18() {
    let cutover = ct((2012, 11, 18), (0, 0, 0));
    let before = hours_for_exchange_as_of(Exchange::Cme, cutover - chrono::Duration::seconds(1));
    let after = hours_for_exchange_as_of(Exchange::Cme, cutover);
    let monday = (2012, 11, 19);
    let friday = (2012, 11, 23);

    assert!(before.is_open_regular(ct(monday, (15, 14, 59))));
    assert!(!before.is_open(ct(monday, (15, 15, 0))));
    assert!(before.is_open_extended(ct(monday, (15, 30, 0))));
    assert!(before.is_open_extended(ct(monday, (16, 29, 59))));
    assert!(!before.is_open(ct(monday, (16, 30, 0))));
    assert!(!before.is_open(ct(friday, (15, 30, 0))));
    assert!(!after.is_open(ct(monday, (15, 15, 0))));
    assert!(after.is_open_extended(ct(monday, (15, 30, 0))));
    assert!(after.is_open_extended(ct(monday, (16, 14, 59))));
    assert!(!after.is_open(ct(monday, (16, 15, 0))));
    assert!(after.is_open_extended(ct(friday, (15, 30, 0))));
    assert!(!after.is_open(ct(friday, (16, 15, 0))));
    assert!(calendar_for_exchange(Exchange::Cme).is_open_extended(ct(monday, (15, 30, 0))));
}

// CME moved the CME Equity close from 16:15 to 16:00 CT effective Sunday,
// 2015-09-20, for trade date Monday 2015-09-21. All other modeled hours were
// unchanged.
// https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20150914.html
#[test]
fn cme_equity_close_moved_with_the_2015_09_20_sunday_session() {
    let cutover = ct((2015, 9, 20), (0, 0, 0));
    let before = hours_for_exchange_as_of(Exchange::Cme, cutover - chrono::Duration::seconds(1));
    let after = hours_for_exchange_as_of(Exchange::Cme, cutover);
    let sunday_open = ct((2015, 9, 20), (17, 0, 0));
    let monday = (2015, 9, 21);

    assert_eq!(
        candle_end(&before, sunday_open, CalendarResolution::Daily),
        Some(ct(monday, (16, 15, 0)))
    );
    assert_eq!(
        candle_end(&after, sunday_open, CalendarResolution::Daily),
        Some(ct(monday, (16, 0, 0)))
    );
    assert!(before.is_open_extended(ct(monday, (16, 0, 0))));
    assert!(before.is_open_extended(ct(monday, (16, 14, 59))));
    assert!(!before.is_open(ct(monday, (16, 15, 0))));
    assert!(after.is_open_extended(ct(monday, (15, 59, 59))));
    assert!(!after.is_open(ct(monday, (16, 0, 0))));

    let calendar = calendar_for_exchange(Exchange::Cme);
    assert!(!calendar.is_open(ct(monday, (16, 0, 0))));
    assert_eq!(
        calendar.candle_end(sunday_open, CalendarResolution::Daily),
        Some(ct(monday, (16, 0, 0)))
    );
}

// CME's 2009 announcement supplies the schedule in force at the January 2010
// audit floor: electronic trading 18:00–07:15 CT, with the unchanged weekday
// 09:30–13:15 day session.
// https://www.cmegroup.com/media-room/press-releases/2009/6/05/cme_group_announcesadditionalagricultureethanolelectronictrading.html
#[test]
fn cbot_2010_floor_uses_the_published_split_sessions() {
    let hours = hours_for_exchange_as_of(Exchange::Cbot, ct((2010, 1, 4), (12, 0, 0)));
    let sunday = (2010, 1, 3);
    let monday = (2010, 1, 4);

    assert!(!hours.is_open(ct(sunday, (17, 59, 59))));
    assert!(hours.is_open_extended(ct(sunday, (18, 0, 0))));
    assert!(hours.is_open_extended(ct(monday, (7, 14, 59))));
    assert!(!hours.is_open(ct(monday, (7, 15, 0))));
    assert!(!hours.is_open(ct(monday, (9, 29, 59))));
    assert!(hours.is_open_regular(ct(monday, (9, 30, 0))));
    assert!(!hours.is_open(ct(monday, (13, 15, 0))));
    assert!(hours.is_open_extended(ct(monday, (18, 0, 0))));
}

// CME expanded grain/oilseed electronic trading to 17:00–14:00 CT effective
// Sunday 2012-05-20, while the 09:30–13:15 day session remained the regular
// phase in this venue-level profile.
// https://www.cmegroup.com/media-room/press-releases/2012/5/18/cme_group_to_startexpandedcbotgrainandoilseedtradinghoursmay20.html
// https://www.cmegroup.com/tools-information/lookups/advisories/market-data/20120518.html
#[test]
fn cbot_continuous_1700_to_1400_session_started_on_2012_05_20() {
    let cutover = ct((2012, 5, 20), (0, 0, 0));
    let before = hours_for_exchange_as_of(Exchange::Cbot, cutover - chrono::Duration::seconds(1));
    let after = hours_for_exchange_as_of(Exchange::Cbot, cutover);
    let sunday = (2012, 5, 20);
    let monday = (2012, 5, 21);

    assert!(!before.is_open(ct(sunday, (17, 0, 0))));
    assert!(after.is_open_extended(ct(sunday, (17, 0, 0))));
    assert!(!before.is_open(ct(monday, (7, 15, 0))));
    assert!(after.is_open_extended(ct(monday, (7, 15, 0))));
    assert!(after.is_open_regular(ct(monday, (9, 30, 0))));
    assert!(after.is_open_extended(ct(monday, (13, 15, 0))));
    assert!(after.is_open_extended(ct(monday, (13, 59, 59))));
    assert!(!after.is_open(ct(monday, (14, 0, 0))));
    assert!(calendar_for_exchange(Exchange::Cbot).is_open_extended(ct(sunday, (17, 0, 0))));
}

// CME Group SER-6617 set 19:00–07:45 and 08:30–13:15 CT effective Sunday
// 2013-04-07 for trade date Monday 2013-04-08. SER-7395R moved the close to
// 13:20 effective Sunday 2015-07-05 for trade date Monday 2015-07-06.
// https://www.cmegroup.com/rulebook/files/ser_6617_cbot_grain_oilseed_hours_2013_final.pdf
// https://www.cmegroup.com/tools-information/lookups/advisories/ser/SER-7395R.html
#[test]
fn cbot_2013_profile_kept_the_1315_close() {
    let cutover = ct((2013, 4, 7), (0, 0, 0));
    let before = hours_for_exchange_as_of(Exchange::Cbot, cutover - chrono::Duration::seconds(1));
    let after = hours_for_exchange_as_of(Exchange::Cbot, cutover);

    assert!(before.is_open_extended(ct((2013, 4, 7), (17, 0, 0))));
    assert!(!after.is_open(ct((2013, 4, 7), (17, 0, 0))));
    assert!(after.is_open_extended(ct((2013, 4, 7), (19, 0, 0))));
    assert!(before.is_open_extended(ct((2013, 4, 8), (8, 0, 0))));
    assert!(!after.is_open(ct((2013, 4, 8), (8, 0, 0))));
    assert!(after.is_open_regular(ct((2013, 4, 8), (8, 30, 0))));
    assert!(after.is_open_regular(ct((2013, 4, 8), (13, 14, 59))));
    assert!(!after.is_open(ct((2013, 4, 8), (13, 15, 0))));
}

#[test]
fn cbot_1320_close_started_with_the_2015_07_06_trade_date() {
    // The profile boundary is CME's stated Sunday implementation date. The
    // only changed session is first observable on Monday's named trade date.
    let cutover = ct((2015, 7, 5), (0, 0, 0));
    let before = hours_for_exchange_as_of(Exchange::Cbot, cutover - chrono::Duration::seconds(1));
    let after = hours_for_exchange_as_of(Exchange::Cbot, cutover);
    let monday = (2015, 7, 6);

    assert!(!before.is_open(ct(monday, (13, 15, 0))));
    assert!(after.is_open_regular(ct(monday, (13, 15, 0))));
    assert!(after.is_open_regular(ct(monday, (13, 19, 59))));
    assert!(!after.is_open(ct(monday, (13, 20, 0))));

    let calendar = calendar_for_exchange(Exchange::Cbot);
    assert!(calendar.is_open_regular(ct(monday, (13, 15, 0))));
}
