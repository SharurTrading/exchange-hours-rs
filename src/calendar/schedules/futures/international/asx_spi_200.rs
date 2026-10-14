// SPDX-License-Identifier: MIT-0

//! ASX SPI 200 index futures — the Sydney 24-hour grid.
//!
//! The family is AP SPI 200 on ASX 24 (formerly SFE/SYCOM). The operator's
//! own trading-hours table carries the one seasonal selector modelled here
//! verbatim: two columns, "For period from second Sunday in March to first
//! Sunday in November" (US daylight time — night session 5.10pm to 7.00am)
//! and "first Sunday in November to second Sunday in March" (night session
//! 5.10pm to 8.00am), with the day session 9.50am to 4.30pm in both. Every
//! captured state of that table from 2010 to 2026 prints the same two-column
//! grid, so the normal week sources to the floor with no dated change; the
//! quotations are in `docs/evidence/asx_spi_200.md`.
//!
//! The weekly shape the calendars imply: the day session opens Monday to
//! Friday, the night session opens Sunday to Friday evenings (the ANZAC
//! Saturdays the calendars mark closed close the Friday-evening leg, and the
//! 2015 calendar's "normal trading hours apply" on ANZAC Day Saturday keeps
//! it), and the week ends with that leg on the Saturday morning. The
//! pre-open footnote on the SYCOM hours page — "Pre-open starts 10 minutes
//! prior to trading for all contracts" — supplies both order-entry windows.
//!
//! The selector computes the New York local date of the night session's open
//! instant and reads the operator's own two columns off it; it never applies a
//! fixed day delta, which would misclassify the transition Sundays the night
//! session opens on.

use chrono::{Datelike, Duration, NaiveDate, Utc};
use chrono_tz::{America, Australia};

use super::super::StaticHoursProfile;
use crate::calendar::SessionRule;
use crate::calendar::local_time::mk_local_open;
use crate::calendar::rule::MON_FRI;
use crate::calendar::schedules::timeline::local_date;

/// The night session opens 17:10 Sydney.
const NIGHT_OPEN_SSM: u32 = 17 * 3_600 + 10 * 60;

/// Sunday through Friday: every evening the night session opens (the
/// seven slots are Monday-first).
const NIGHT_DAYS: [bool; 7] = [true, true, true, true, true, false, true];

// Day session 09:50-16:30 Sydney, identical in both seasonal columns.
pub(crate) static ASX_SPI_200_REGULAR: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 9 * 3_600 + 50 * 60,
    close_ssm: 16 * 3_600 + 30 * 60,
}];

// US daylight column: night session 5.10pm to 7.00am.
pub(crate) static ASX_SPI_200_EXTENDED_DST: &[SessionRule] = &[SessionRule {
    days: NIGHT_DAYS,
    open_ssm: NIGHT_OPEN_SSM,
    close_ssm: 7 * 3_600,
}];

// US standard column: night session 5.10pm to 8.00am.
pub(crate) static ASX_SPI_200_EXTENDED_STANDARD: &[SessionRule] = &[SessionRule {
    days: NIGHT_DAYS,
    open_ssm: NIGHT_OPEN_SSM,
    close_ssm: 8 * 3_600,
}];

// Both pre-opens: the SYCOM page's footnote starts each one ten minutes
// before its open, so the day pre-open is 09:40-09:50 and the night
// pre-open 17:00-17:10. Both only collect orders.
pub(crate) static ASX_SPI_200_ORDER_ENTRY: &[SessionRule] = &[
    SessionRule {
        days: MON_FRI,
        open_ssm: 9 * 3_600 + 40 * 60,
        close_ssm: 9 * 3_600 + 50 * 60,
    },
    SessionRule {
        days: NIGHT_DAYS,
        open_ssm: 17 * 3_600,
        close_ssm: NIGHT_OPEN_SSM,
    },
];

/// The US daylight column: night close 07:00 Sydney.
pub(crate) static ASX_SPI_200_US_DST: StaticHoursProfile = StaticHoursProfile {
    tz: Australia::Sydney,
    regular: ASX_SPI_200_REGULAR,
    extended: ASX_SPI_200_EXTENDED_DST,
    order_entry: ASX_SPI_200_ORDER_ENTRY,
    has_daily_close: true,
    has_weekend_close: true,
};

/// The US standard column: night close 08:00 Sydney.
pub(crate) static ASX_SPI_200_US_STANDARD: StaticHoursProfile = StaticHoursProfile {
    tz: Australia::Sydney,
    regular: ASX_SPI_200_REGULAR,
    extended: ASX_SPI_200_EXTENDED_STANDARD,
    order_entry: ASX_SPI_200_ORDER_ENTRY,
    has_daily_close: true,
    has_weekend_close: true,
};

/// The `n`-th Sunday of `month` in `year` — the operator's own column
/// boundaries ("second Sunday in March", "first Sunday in November").
fn nth_sunday(year: i32, month: u32, n: u32) -> Option<NaiveDate> {
    let first = NaiveDate::from_ymd_opt(year, month, 1)?;
    let weekday = first.weekday().num_days_from_sunday();
    let step = (7 - weekday) % 7 + 7 * (n - 1);
    first.checked_add_signed(Duration::days(i64::from(step)))
}

/// Whether the New York civil date sits in the operator's US daylight column
/// (from the second Sunday in March up to the first Sunday in November, the
/// table's own headings).
fn in_us_daylight_column(ny_date: NaiveDate) -> bool {
    let year = ny_date.year();
    match (nth_sunday(year, 3, 2), nth_sunday(year, 11, 1)) {
        (Some(start), Some(end)) => ny_date >= start && ny_date < end,
        // Unrepresentable outside chrono's year range; the standard column is
        // the profile both headings name outside the daylight period.
        _ => false,
    }
}

/// Selects the ASX SPI 200 profile in force for the session opening on
/// `as_of`'s Sydney day: the New York local date of that night session's open
/// instant decides the column (LAW-DETERMINISM — the caller's instant, never a
/// clock).
pub(crate) fn asx_spi_200_profile_at(as_of: chrono::DateTime<Utc>) -> &'static StaticHoursProfile {
    let day = local_date(as_of, Australia::Sydney);
    let night_open = mk_local_open(Australia::Sydney, day, NIGHT_OPEN_SSM).with_timezone(&Utc);
    let ny_date = local_date(night_open, America::New_York);
    if in_us_daylight_column(ny_date) {
        &ASX_SPI_200_US_DST
    } else {
        &ASX_SPI_200_US_STANDARD
    }
}
