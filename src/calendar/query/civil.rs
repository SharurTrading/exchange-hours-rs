// SPDX-License-Identifier: MIT-0

//! Civil-time calculations for the query engine.

use jiff::civil::Date;

/// Two dates belong to the same ISO week only if their week years also agree.
pub(super) fn iso_week(date: Date) -> (i16, i8) {
    let week = date.iso_week_date();
    (week.year(), week.week())
}
