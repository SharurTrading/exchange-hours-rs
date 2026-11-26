// SPDX-License-Identifier: MIT-0

//! Constructors for valid and occasionally ambiguous test fixture dates and times.

use jiff::civil::{Date, DateTime};
use jiff::tz::{AmbiguousOffset, TimeZone};
use jiff::{SignedDuration, Timestamp};

pub(crate) const PROFILE_INSTANT: Timestamp = Timestamp::constant(1_787_400_000, 0);

pub(crate) fn date_opt(
    year: impl TryInto<i16>,
    month: impl TryInto<i8>,
    day: impl TryInto<i8>,
) -> Option<Date> {
    Date::new(
        year.try_into().ok()?,
        month.try_into().ok()?,
        day.try_into().ok()?,
    )
    .ok()
}

pub(crate) fn elapsed_days(days: i64) -> SignedDuration {
    SignedDuration::from_hours(24 * days)
}

pub(crate) fn local_datetime(
    date: (impl TryInto<i16>, impl TryInto<i8>, impl TryInto<i8>),
    time: (impl TryInto<i8>, impl TryInto<i8>, impl TryInto<i8>),
) -> Option<DateTime> {
    DateTime::new(
        date.0.try_into().ok()?,
        date.1.try_into().ok()?,
        date.2.try_into().ok()?,
        time.0.try_into().ok()?,
        time.1.try_into().ok()?,
        time.2.try_into().ok()?,
        0,
    )
    .ok()
}

pub(crate) fn local_opt(
    zone: &TimeZone,
    date: (impl TryInto<i16>, impl TryInto<i8>, impl TryInto<i8>),
    time: (impl TryInto<i8>, impl TryInto<i8>, impl TryInto<i8>),
) -> Option<Timestamp> {
    zone.to_ambiguous_timestamp(local_datetime(date, time)?)
        .unambiguous()
        .ok()
}

pub(crate) fn local_earliest(
    zone: &TimeZone,
    date: (impl TryInto<i16>, impl TryInto<i8>, impl TryInto<i8>),
    time: (impl TryInto<i8>, impl TryInto<i8>, impl TryInto<i8>),
) -> Option<Timestamp> {
    let ambiguous = zone.to_ambiguous_timestamp(local_datetime(date, time)?);
    match ambiguous.offset() {
        AmbiguousOffset::Gap { .. } => None,
        AmbiguousOffset::Unambiguous { .. } | AmbiguousOffset::Fold { .. } => {
            ambiguous.earlier().ok()
        }
    }
}

pub(crate) fn local(zone: &TimeZone, date: (i16, i8, i8), time: (u32, u32, u32)) -> Timestamp {
    local_opt(zone, date, time).expect("local fixture must be unambiguous and representable")
}

pub(crate) fn utc_datetime(local: DateTime) -> Timestamp {
    jiff::tz::Offset::UTC
        .to_timestamp(local)
        .expect("UTC fixture must be a representable timestamp")
}
