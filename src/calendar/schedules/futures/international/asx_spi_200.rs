// SPDX-License-Identifier: MIT-0

//! ASX SPI 200 index futures — the Sydney 24-hour grid.
//!
//! Evidence: `docs/evidence/asx_spi_200.md`.

use crate::calendar::time_zones::{America, Australia, SelectionInstant};
use jiff::civil::Date;

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

/// Whether New York is between the second Sunday in March (inclusive) and
/// the first Sunday in November (exclusive).
fn in_us_daylight_column(ny_date: Date) -> bool {
    // A result below one means the most recent Sunday is in the prior month.
    let sunday_day = ny_date.day() - ny_date.weekday().to_sunday_zero_offset();
    match ny_date.month() {
        3 => sunday_day >= 8,
        4..=10 => true,
        11 => sunday_day < 1,
        _ => false,
    }
}

/// Selects the ASX SPI 200 profile in force for the session opening on
/// `as_of`'s Sydney day: the New York local date of that night session's open
/// instant decides the column (LAW-DETERMINISM — the caller's instant, never a
/// clock).
pub(crate) fn asx_spi_200_profile_at(as_of: SelectionInstant<'_>) -> &'static StaticHoursProfile {
    let day = local_date(as_of, Australia::Sydney);
    // The supported timestamp limits both fall in New York's standard column.
    // The caller's instant selects that column if Sydney's open is out of range.
    let night_open = mk_local_open(as_of.zones.resolve(Australia::Sydney), day, NIGHT_OPEN_SSM)
        .unwrap_or(as_of.instant);
    let ny_date = local_date(
        SelectionInstant {
            instant: night_open,
            ..as_of
        },
        America::New_York,
    );
    if in_us_daylight_column(ny_date) {
        &ASX_SPI_200_US_DST
    } else {
        &ASX_SPI_200_US_STANDARD
    }
}
