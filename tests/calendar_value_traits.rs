// SPDX-License-Identifier: MIT-0

//! Compile-time integration contracts for cheap calendar values.

use exchange_hours::{ExchangeCalendar, PolicyCalendar};

fn requires_calendar_value_traits<T: Copy + Send + Sync + 'static>() {}

#[test]
fn exchange_calendar_remains_a_copy_send_sync_static_value() {
    requires_calendar_value_traits::<ExchangeCalendar>();
}

fn requires_borrowed_calendar_value_traits<T: Copy + Send + Sync>() {}

#[test]
fn policy_calendar_remains_copy_send_and_sync() {
    requires_borrowed_calendar_value_traits::<PolicyCalendar<'static>>();
}
