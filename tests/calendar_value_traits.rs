// SPDX-License-Identifier: MIT-0

//! Compile-time integration contracts for cheap calendar values.

use exchange_hours::{
    CalendarCoverage, CalendarQueryError, CoverageGap, CoverageGapReason, DateCoverage, DateRange,
    ExchangeCalendar, HolidayContract, PolicyCalendar,
};

fn requires_calendar_value_traits<T: Copy + Send + Sync + 'static>() {}

#[test]
fn exchange_calendar_remains_a_copy_send_sync_static_value() {
    requires_calendar_value_traits::<ExchangeCalendar>();
}

#[test]
fn coverage_metadata_stays_a_copy_send_sync_static_value() {
    // LAW-COVERAGE's metadata and error vocabulary cross the public boundary as
    // values: no allocation, no `String`, no borrow tied to the calendar.
    requires_calendar_value_traits::<CalendarCoverage>();
    requires_calendar_value_traits::<CalendarQueryError>();
    requires_calendar_value_traits::<DateRange>();
    requires_calendar_value_traits::<DateCoverage>();
    requires_calendar_value_traits::<CoverageGap>();
    requires_calendar_value_traits::<CoverageGapReason>();
    requires_calendar_value_traits::<HolidayContract>();
}

fn requires_borrowed_calendar_value_traits<T: Copy + Send + Sync>() {}

#[test]
fn policy_calendar_remains_copy_send_and_sync() {
    requires_borrowed_calendar_value_traits::<PolicyCalendar<'static>>();
}
