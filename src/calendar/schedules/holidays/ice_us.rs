// SPDX-License-Identifier: MIT-0

//! ICE Futures U.S. holiday rows, 2026 through the 2027 calendar's last entry.
//!
//! Keyed by the crate's own venue-local trade date in `America/New_York`
//! (design memo D1). ICE prints its holiday calendar and its per-holiday
//! Exchange Notices by civil date, and for every family the crate models that
//! civil date **is** the trade date: the softs run inside one local day, and
//! Cotton, the NYSE FANG+ index family and the US Dollar Index family open on
//! the evening of `D-1` for trade date `D`, which is the date ICE names.
//!
//! The whole block is **T1**: the `June 9, 2025 - 2026 Trading Holiday
//! Calendar`, the `June 4, 2026 - 2027 Trading Holiday Calendar` issued as an
//! Exchange Notice, and the six 2026 per-holiday Exchange Notices that give the
//! exact late-open and early-close instants in New York time.
//!
//! ICE's calendars mark a date on which a product group trades non-regular
//! hours `open1`, footnoted "Trading Hours for these contracts will be
//! announced in advance of the respective holiday via Exchange Notices". Where
//! that notice exists the row carries its instants; where it does not, the
//! operator has published that the date is **not** normal without publishing
//! what it is, which is exactly what [`Unsourced`](super::HolidayKind::Unsourced)
//! records. Those dates, the missing 2026 MLK and Presidents' Day notices, and
//! the late-2026 notices that had not issued at retrieval are gaps in
//! [`docs/evidence/iceus.md`](../../../../../docs/evidence/iceus.md).
//!
//! Coverage runs to 2028-01-03, the last trade date the 2027 calendar names.
//! Its last two dates are audited normal: 2027-12-31 is not on ICE's calendar,
//! and on 2028-01-03 (New Year's Day observed) every group a crate identity
//! routes to prints `open`; the evidence files record both.

use super::fences::{early_close, late_open, late_open_and_early_close};
use super::{
    EvidenceTier::T1,
    HolidayKind::{Closed, Unsourced},
    HolidayTable, holidays,
};

/// Sugar No. 11, Coffee "C" and Cocoa: one calendar group, one notice group,
/// and one shared Easter Monday late open, so one table serves all three.
// Evidence: docs/evidence/ice_us_sugar.md, docs/evidence/ice_us_coffee.md,
// docs/evidence/ice_us_cocoa.md
pub(crate) static SUGAR_COFFEE_COCOA: &HolidayTable = holidays! {
    coverage: [(2026, 1, 1) ..= (2028, 1, 3)],
    rows: [
        // 2026-01-01 - T1 - IFUS-CAL-2026 - New Year's Day.
        (2026, 1, 1, Closed, T1, "IFUS-CAL-2026"),
        // 2026-01-19 - T1 - IFUS-CAL-2026 - Martin Luther King Day.
        (2026, 1, 19, Closed, T1, "IFUS-CAL-2026"),
        // 2026-02-16 - T1 - IFUS-CAL-2026 - Presidents Day.
        (2026, 2, 16, Closed, T1, "IFUS-CAL-2026"),
        // 2026-04-03 - T1 - IFUS-NOTICE-2026-GOODFRIDAY - Good Friday.
        (2026, 4, 3, Closed, T1, "IFUS-NOTICE-2026-GOODFRIDAY"),
        // 2026-04-06 - T1 - IFUS-NOTICE-2026-GOODFRIDAY - Easter Monday late
        // open at 07:30 NY; regular hours for every other contract.
        (2026, 4, 6, late_open(7 * 3_600 + 30 * 60), T1, "IFUS-NOTICE-2026-GOODFRIDAY"),
        // 2026-05-25 - T1 - IFUS-NOTICE-2026-MEMORIAL - Memorial Day.
        (2026, 5, 25, Closed, T1, "IFUS-NOTICE-2026-MEMORIAL"),
        // 2026-06-19 - T1 - IFUS-NOTICE-2026-JUNETEENTH - Juneteenth.
        (2026, 6, 19, Closed, T1, "IFUS-NOTICE-2026-JUNETEENTH"),
        // 2026-07-03 - T1 - IFUS-NOTICE-2026-INDEPENDENCE - Independence Day observed.
        (2026, 7, 3, Closed, T1, "IFUS-NOTICE-2026-INDEPENDENCE"),
        // 2026-09-07 - T1 - IFUS-NOTICE-2026-LABORDAY - Labor Day.
        (2026, 9, 7, Closed, T1, "IFUS-NOTICE-2026-LABORDAY"),
        // 2026-11-26 - T1 - IFUS-CAL-2026 - Thanksgiving Day.
        (2026, 11, 26, Closed, T1, "IFUS-CAL-2026"),
        // 2026-12-25 - T1 - IFUS-CAL-2026 - Christmas Day.
        (2026, 12, 25, Closed, T1, "IFUS-CAL-2026"),
        // 2027-01-01 - T1 - IFUS-CAL-2027 - New Year's Day.
        (2027, 1, 1, Closed, T1, "IFUS-CAL-2027"),
        // 2027-01-18 - T1 - IFUS-CAL-2027 - Martin Luther King Day.
        (2027, 1, 18, Closed, T1, "IFUS-CAL-2027"),
        // 2027-02-15 - T1 - IFUS-CAL-2027 - Presidents Day.
        (2027, 2, 15, Closed, T1, "IFUS-CAL-2027"),
        // 2027-03-26 - T1 - IFUS-CAL-2027 - Good Friday.
        (2027, 3, 26, Closed, T1, "IFUS-CAL-2027"),
        // 2027-05-31 - T1 - IFUS-CAL-2027 - Memorial Day.
        (2027, 5, 31, Closed, T1, "IFUS-CAL-2027"),
        // 2027-06-18 - T1 - IFUS-CAL-2027 - Juneteenth.
        (2027, 6, 18, Closed, T1, "IFUS-CAL-2027"),
        // 2027-07-05 - T1 - IFUS-CAL-2027 - Independence Day observed.
        (2027, 7, 5, Closed, T1, "IFUS-CAL-2027"),
        // 2027-09-06 - T1 - IFUS-CAL-2027 - Labor Day.
        (2027, 9, 6, Closed, T1, "IFUS-CAL-2027"),
        // 2027-11-25 - T1 - IFUS-CAL-2027 - Thanksgiving Day.
        (2027, 11, 25, Closed, T1, "IFUS-CAL-2027"),
        // 2027-12-24 - T1 - IFUS-CAL-2027 - Christmas Day observed.
        (2027, 12, 24, Closed, T1, "IFUS-CAL-2027"),
    ],
};

/// FCOJ: the softs calendar group without the Easter Monday late open, which
/// the Good Friday notice gives to Sugar, Coffee and Cocoa only.
// Evidence: docs/evidence/ice_us_orange_juice.md
pub(crate) static ORANGE_JUICE: &HolidayTable = holidays! {
    coverage: [(2026, 1, 1) ..= (2028, 1, 3)],
    rows: [
        // 2026-01-01 - T1 - IFUS-CAL-2026 - New Year's Day.
        (2026, 1, 1, Closed, T1, "IFUS-CAL-2026"),
        // 2026-01-19 - T1 - IFUS-CAL-2026 - Martin Luther King Day.
        (2026, 1, 19, Closed, T1, "IFUS-CAL-2026"),
        // 2026-02-16 - T1 - IFUS-CAL-2026 - Presidents Day.
        (2026, 2, 16, Closed, T1, "IFUS-CAL-2026"),
        // 2026-04-03 - T1 - IFUS-NOTICE-2026-GOODFRIDAY - Good Friday.
        (2026, 4, 3, Closed, T1, "IFUS-NOTICE-2026-GOODFRIDAY"),
        // 2026-05-25 - T1 - IFUS-NOTICE-2026-MEMORIAL - Memorial Day.
        (2026, 5, 25, Closed, T1, "IFUS-NOTICE-2026-MEMORIAL"),
        // 2026-06-19 - T1 - IFUS-NOTICE-2026-JUNETEENTH - Juneteenth.
        (2026, 6, 19, Closed, T1, "IFUS-NOTICE-2026-JUNETEENTH"),
        // 2026-07-03 - T1 - IFUS-NOTICE-2026-INDEPENDENCE - Independence Day observed.
        (2026, 7, 3, Closed, T1, "IFUS-NOTICE-2026-INDEPENDENCE"),
        // 2026-09-07 - T1 - IFUS-NOTICE-2026-LABORDAY - Labor Day.
        (2026, 9, 7, Closed, T1, "IFUS-NOTICE-2026-LABORDAY"),
        // 2026-11-26 - T1 - IFUS-CAL-2026 - Thanksgiving Day.
        (2026, 11, 26, Closed, T1, "IFUS-CAL-2026"),
        // 2026-12-25 - T1 - IFUS-CAL-2026 - Christmas Day.
        (2026, 12, 25, Closed, T1, "IFUS-CAL-2026"),
        // 2027-01-01 - T1 - IFUS-CAL-2027 - New Year's Day.
        (2027, 1, 1, Closed, T1, "IFUS-CAL-2027"),
        // 2027-01-18 - T1 - IFUS-CAL-2027 - Martin Luther King Day.
        (2027, 1, 18, Closed, T1, "IFUS-CAL-2027"),
        // 2027-02-15 - T1 - IFUS-CAL-2027 - Presidents Day.
        (2027, 2, 15, Closed, T1, "IFUS-CAL-2027"),
        // 2027-03-26 - T1 - IFUS-CAL-2027 - Good Friday.
        (2027, 3, 26, Closed, T1, "IFUS-CAL-2027"),
        // 2027-05-31 - T1 - IFUS-CAL-2027 - Memorial Day.
        (2027, 5, 31, Closed, T1, "IFUS-CAL-2027"),
        // 2027-06-18 - T1 - IFUS-CAL-2027 - Juneteenth.
        (2027, 6, 18, Closed, T1, "IFUS-CAL-2027"),
        // 2027-07-05 - T1 - IFUS-CAL-2027 - Independence Day observed.
        (2027, 7, 5, Closed, T1, "IFUS-CAL-2027"),
        // 2027-09-06 - T1 - IFUS-CAL-2027 - Labor Day.
        (2027, 9, 6, Closed, T1, "IFUS-CAL-2027"),
        // 2027-11-25 - T1 - IFUS-CAL-2027 - Thanksgiving Day.
        (2027, 11, 25, Closed, T1, "IFUS-CAL-2027"),
        // 2027-12-24 - T1 - IFUS-CAL-2027 - Christmas Day observed.
        (2027, 12, 24, Closed, T1, "IFUS-CAL-2027"),
    ],
};

/// Cotton No. 2: the softs calendar group, plus its own 2026-07-06 late open.
// Evidence: docs/evidence/ice_us_cotton.md
pub(crate) static COTTON: &HolidayTable = holidays! {
    coverage: [(2026, 1, 1) ..= (2028, 1, 3)],
    rows: [
        // 2026-01-01 - T1 - IFUS-CAL-2026 - New Year's Day.
        (2026, 1, 1, Closed, T1, "IFUS-CAL-2026"),
        // 2026-01-19 - T1 - IFUS-CAL-2026 - Martin Luther King Day.
        (2026, 1, 19, Closed, T1, "IFUS-CAL-2026"),
        // 2026-02-16 - T1 - IFUS-CAL-2026 - Presidents Day.
        (2026, 2, 16, Closed, T1, "IFUS-CAL-2026"),
        // 2026-04-03 - T1 - IFUS-NOTICE-2026-GOODFRIDAY - Good Friday.
        (2026, 4, 3, Closed, T1, "IFUS-NOTICE-2026-GOODFRIDAY"),
        // 2026-05-25 - T1 - IFUS-NOTICE-2026-MEMORIAL - Memorial Day.
        (2026, 5, 25, Closed, T1, "IFUS-NOTICE-2026-MEMORIAL"),
        // 2026-06-19 - T1 - IFUS-NOTICE-2026-JUNETEENTH - Juneteenth.
        (2026, 6, 19, Closed, T1, "IFUS-NOTICE-2026-JUNETEENTH"),
        // 2026-07-03 - T1 - IFUS-NOTICE-2026-INDEPENDENCE - Independence Day observed.
        (2026, 7, 3, Closed, T1, "IFUS-NOTICE-2026-INDEPENDENCE"),
        // 2026-07-06 - T1 - IFUS-NOTICE-2026-INDEPENDENCE - late open at 08:00
        // NY for Cotton only; regular open times for every other contract.
        (2026, 7, 6, late_open(8 * 3_600), T1, "IFUS-NOTICE-2026-INDEPENDENCE"),
        // 2026-09-07 - T1 - IFUS-NOTICE-2026-LABORDAY - Labor Day.
        (2026, 9, 7, Closed, T1, "IFUS-NOTICE-2026-LABORDAY"),
        // 2026-11-26 - T1 - IFUS-CAL-2026 - Thanksgiving Day.
        (2026, 11, 26, Closed, T1, "IFUS-CAL-2026"),
        // 2026-12-25 - T1 - IFUS-CAL-2026 - Christmas Day.
        (2026, 12, 25, Closed, T1, "IFUS-CAL-2026"),
        // 2027-01-01 - T1 - IFUS-CAL-2027 - New Year's Day.
        (2027, 1, 1, Closed, T1, "IFUS-CAL-2027"),
        // 2027-01-18 - T1 - IFUS-CAL-2027 - Martin Luther King Day.
        (2027, 1, 18, Closed, T1, "IFUS-CAL-2027"),
        // 2027-02-15 - T1 - IFUS-CAL-2027 - Presidents Day.
        (2027, 2, 15, Closed, T1, "IFUS-CAL-2027"),
        // 2027-03-26 - T1 - IFUS-CAL-2027 - Good Friday.
        (2027, 3, 26, Closed, T1, "IFUS-CAL-2027"),
        // 2027-05-31 - T1 - IFUS-CAL-2027 - Memorial Day.
        (2027, 5, 31, Closed, T1, "IFUS-CAL-2027"),
        // 2027-06-18 - T1 - IFUS-CAL-2027 - Juneteenth.
        (2027, 6, 18, Closed, T1, "IFUS-CAL-2027"),
        // 2027-07-05 - T1 - IFUS-CAL-2027 - Independence Day observed.
        (2027, 7, 5, Closed, T1, "IFUS-CAL-2027"),
        // 2027-09-06 - T1 - IFUS-CAL-2027 - Labor Day.
        (2027, 9, 6, Closed, T1, "IFUS-CAL-2027"),
        // 2027-11-25 - T1 - IFUS-CAL-2027 - Thanksgiving Day.
        (2027, 11, 25, Closed, T1, "IFUS-CAL-2027"),
        // 2027-12-24 - T1 - IFUS-CAL-2027 - Christmas Day observed.
        (2027, 12, 24, Closed, T1, "IFUS-CAL-2027"),
    ],
};

/// NYSE FANG+ Index futures, the family the `ice_us` key names.
///
/// ICE prints FANG+ inside the `NYSE ... Index` early-close bullet on the two
/// 2026 notices that name it at all, at the same instant as NYSE Stock Index;
/// the interpretive step that carries that reading to the three notices which
/// do not spell it out is recorded per year in the evidence file.
// Evidence: docs/evidence/ice_us.md
pub(crate) static FANG: &HolidayTable = holidays! {
    coverage: [(2026, 1, 1) ..= (2028, 1, 3)],
    rows: [
        // 2026-01-01 - T1 - IFUS-CAL-2026 - New Year's Day.
        (2026, 1, 1, Closed, T1, "IFUS-CAL-2026"),
        // 2026-01-19 - T1 - IFUS-CAL-2026 - MLK Day `open1`; no notice issued.
        (2026, 1, 19, Unsourced, T1, "IFUS-CAL-2026"),
        // 2026-02-16 - T1 - IFUS-CAL-2026 - Presidents Day `open1`; no notice issued.
        (2026, 2, 16, Unsourced, T1, "IFUS-CAL-2026"),
        // 2026-04-03 - T1 - IFUS-NOTICE-2026-GOODFRIDAY - late open 05:00 NY,
        // early close 09:15 NY, so the Thursday-evening leg does not run.
        (2026, 4, 3, late_open_and_early_close(5 * 3_600, 9 * 3_600 + 15 * 60), T1, "IFUS-NOTICE-2026-GOODFRIDAY"),
        // 2026-05-25 - T1 - IFUS-NOTICE-2026-MEMORIAL - early close 13:00 NY.
        (2026, 5, 25, early_close(13 * 3_600), T1, "IFUS-NOTICE-2026-MEMORIAL"),
        // 2026-06-19 - T1 - IFUS-NOTICE-2026-JUNETEENTH - early close 13:00 NY.
        (2026, 6, 19, early_close(13 * 3_600), T1, "IFUS-NOTICE-2026-JUNETEENTH"),
        // 2026-07-03 - T1 - IFUS-NOTICE-2026-INDEPENDENCE - early close 13:00 NY.
        (2026, 7, 3, early_close(13 * 3_600), T1, "IFUS-NOTICE-2026-INDEPENDENCE"),
        // 2026-09-07 - T1 - IFUS-NOTICE-2026-LABORDAY - early close 13:00 NY,
        // the one 2026 notice that names `NYSE FANG+` in that bullet.
        (2026, 9, 7, early_close(13 * 3_600), T1, "IFUS-NOTICE-2026-LABORDAY"),
        // 2026-11-26 - T1 - IFUS-CAL-2026 - Thanksgiving `open1`; notice not issued.
        (2026, 11, 26, Unsourced, T1, "IFUS-CAL-2026"),
        // 2026-12-25 - T1 - IFUS-CAL-2026 - Christmas Day.
        (2026, 12, 25, Closed, T1, "IFUS-CAL-2026"),
        // 2026-12-28 - T1 - IFUS-CAL-2026 - Boxing Day `open1`; notice not issued.
        (2026, 12, 28, Unsourced, T1, "IFUS-CAL-2026"),
        // 2027-01-01 - T1 - IFUS-CAL-2027 - New Year's Day.
        (2027, 1, 1, Closed, T1, "IFUS-CAL-2027"),
        // 2027-01-18 - T1 - IFUS-CAL-2027 - MLK Day `open1`.
        (2027, 1, 18, Unsourced, T1, "IFUS-CAL-2027"),
        // 2027-02-15 - T1 - IFUS-CAL-2027 - Presidents Day `open1`.
        (2027, 2, 15, Unsourced, T1, "IFUS-CAL-2027"),
        // 2027-03-26 - T1 - IFUS-CAL-2027 - Good Friday `open1`.
        (2027, 3, 26, Unsourced, T1, "IFUS-CAL-2027"),
        // 2027-05-31 - T1 - IFUS-CAL-2027 - Memorial Day `open1`.
        (2027, 5, 31, Unsourced, T1, "IFUS-CAL-2027"),
        // 2027-06-18 - T1 - IFUS-CAL-2027 - Juneteenth `open1`.
        (2027, 6, 18, Unsourced, T1, "IFUS-CAL-2027"),
        // 2027-07-05 - T1 - IFUS-CAL-2027 - Independence Day observed `open1`.
        (2027, 7, 5, Unsourced, T1, "IFUS-CAL-2027"),
        // 2027-09-06 - T1 - IFUS-CAL-2027 - Labor Day `open1`.
        (2027, 9, 6, Unsourced, T1, "IFUS-CAL-2027"),
        // 2027-11-25 - T1 - IFUS-CAL-2027 - Thanksgiving Day `open1`.
        (2027, 11, 25, Unsourced, T1, "IFUS-CAL-2027"),
        // 2027-12-24 - T1 - IFUS-CAL-2027 - Christmas Day observed.
        (2027, 12, 24, Closed, T1, "IFUS-CAL-2027"),
        // 2027-12-27 - T1 - IFUS-CAL-2027 - Boxing Day observed `open1`.
        (2027, 12, 27, Unsourced, T1, "IFUS-CAL-2027"),
    ],
};

/// US Dollar Index futures, the family the `ice_us_dollar_index` key names.
///
/// ICE names `U.S. Dollar Index` in its own bullet on every 2026 notice, so no
/// interpretive step carries these rows — including the Labor Day row that is
/// absent because ICE published regular hours for the family that day.
// Evidence: docs/evidence/ice_us_dollar_index.md
pub(crate) static DOLLAR_INDEX: &HolidayTable = holidays! {
    coverage: [(2026, 1, 1) ..= (2028, 1, 3)],
    rows: [
        // 2026-01-01 - T1 - IFUS-CAL-2026 - New Year's Day.
        (2026, 1, 1, Closed, T1, "IFUS-CAL-2026"),
        // 2026-01-19 - T1 - IFUS-CAL-2026 - MLK Day `open1`; no notice issued.
        (2026, 1, 19, Unsourced, T1, "IFUS-CAL-2026"),
        // 2026-02-16 - T1 - IFUS-CAL-2026 - Presidents Day `open1`; no notice issued.
        (2026, 2, 16, Unsourced, T1, "IFUS-CAL-2026"),
        // 2026-04-03 - T1 - IFUS-NOTICE-2026-GOODFRIDAY - late open 05:00 NY,
        // early close 11:15 NY.
        (2026, 4, 3, late_open_and_early_close(5 * 3_600, 11 * 3_600 + 15 * 60), T1, "IFUS-NOTICE-2026-GOODFRIDAY"),
        // 2026-05-25 - T1 - IFUS-NOTICE-2026-MEMORIAL - early close 14:30 NY.
        (2026, 5, 25, early_close(14 * 3_600 + 30 * 60), T1, "IFUS-NOTICE-2026-MEMORIAL"),
        // 2026-06-19 - T1 - IFUS-NOTICE-2026-JUNETEENTH - early close 14:30 NY.
        (2026, 6, 19, early_close(14 * 3_600 + 30 * 60), T1, "IFUS-NOTICE-2026-JUNETEENTH"),
        // 2026-07-03 - T1 - IFUS-NOTICE-2026-INDEPENDENCE - early close 14:30 NY.
        (2026, 7, 3, early_close(14 * 3_600 + 30 * 60), T1, "IFUS-NOTICE-2026-INDEPENDENCE"),
        // 2026-11-26 - T1 - IFUS-CAL-2026 - Thanksgiving `open1`; notice not issued.
        (2026, 11, 26, Unsourced, T1, "IFUS-CAL-2026"),
        // 2026-12-25 - T1 - IFUS-CAL-2026 - Christmas Day.
        (2026, 12, 25, Closed, T1, "IFUS-CAL-2026"),
        // 2026-12-28 - T1 - IFUS-CAL-2026 - Boxing Day `open1`; notice not issued.
        (2026, 12, 28, Unsourced, T1, "IFUS-CAL-2026"),
        // 2027-01-01 - T1 - IFUS-CAL-2027 - New Year's Day.
        (2027, 1, 1, Closed, T1, "IFUS-CAL-2027"),
        // 2027-01-18 - T1 - IFUS-CAL-2027 - MLK Day `open1`.
        (2027, 1, 18, Unsourced, T1, "IFUS-CAL-2027"),
        // 2027-02-15 - T1 - IFUS-CAL-2027 - Presidents Day `open1`.
        (2027, 2, 15, Unsourced, T1, "IFUS-CAL-2027"),
        // 2027-03-26 - T1 - IFUS-CAL-2027 - Good Friday `open1`.
        (2027, 3, 26, Unsourced, T1, "IFUS-CAL-2027"),
        // 2027-05-31 - T1 - IFUS-CAL-2027 - Memorial Day `open1`.
        (2027, 5, 31, Unsourced, T1, "IFUS-CAL-2027"),
        // 2027-06-18 - T1 - IFUS-CAL-2027 - Juneteenth `open1`.
        (2027, 6, 18, Unsourced, T1, "IFUS-CAL-2027"),
        // 2027-07-05 - T1 - IFUS-CAL-2027 - Independence Day observed `open1`.
        (2027, 7, 5, Unsourced, T1, "IFUS-CAL-2027"),
        // 2027-09-06 - T1 - IFUS-CAL-2027 - Labor Day `open1`.
        (2027, 9, 6, Unsourced, T1, "IFUS-CAL-2027"),
        // 2027-11-25 - T1 - IFUS-CAL-2027 - Thanksgiving Day `open1`.
        (2027, 11, 25, Unsourced, T1, "IFUS-CAL-2027"),
        // 2027-12-24 - T1 - IFUS-CAL-2027 - Christmas Day observed.
        (2027, 12, 24, Closed, T1, "IFUS-CAL-2027"),
        // 2027-12-27 - T1 - IFUS-CAL-2027 - Boxing Day observed `open1`.
        (2027, 12, 27, Unsourced, T1, "IFUS-CAL-2027"),
    ],
};

/// The `iceus` venue table: the intersection of the seven ICE Futures U.S.
/// families the crate routes to the venue (design memo D17).
///
/// A date ships a scheduling row only where all seven agree, which in this
/// window means the four full closures. On every other special date the
/// families disagree — the softs close while the index families trade shortened
/// hours — so the venue ships `Unsourced` rather than a row it cannot state or
/// the silence that would claim the date was audited normal.
// Evidence: docs/evidence/iceus.md
pub(crate) static VENUE: &HolidayTable = holidays! {
    coverage: [(2026, 1, 1) ..= (2028, 1, 3)],
    rows: [
        // 2026-01-01 - T1 - IFUS-CAL-2026 - New Year's Day, every family closed.
        (2026, 1, 1, Closed, T1, "IFUS-CAL-2026"),
        // 2026-01-19 - T1 - IFUS-CAL-2026 - softs closed, index families `open1`.
        (2026, 1, 19, Unsourced, T1, "IFUS-CAL-2026"),
        // 2026-02-16 - T1 - IFUS-CAL-2026 - softs closed, index families `open1`.
        (2026, 2, 16, Unsourced, T1, "IFUS-CAL-2026"),
        // 2026-04-03 - T1 - IFUS-NOTICE-2026-GOODFRIDAY - softs closed, index
        // families on a late open and an early close.
        (2026, 4, 3, Unsourced, T1, "IFUS-NOTICE-2026-GOODFRIDAY"),
        // 2026-04-06 - T1 - IFUS-NOTICE-2026-GOODFRIDAY - three softs late, the rest regular.
        (2026, 4, 6, Unsourced, T1, "IFUS-NOTICE-2026-GOODFRIDAY"),
        // 2026-05-25 - T1 - IFUS-NOTICE-2026-MEMORIAL - softs closed, index families early.
        (2026, 5, 25, Unsourced, T1, "IFUS-NOTICE-2026-MEMORIAL"),
        // 2026-06-19 - T1 - IFUS-NOTICE-2026-JUNETEENTH - softs closed, index families early.
        (2026, 6, 19, Unsourced, T1, "IFUS-NOTICE-2026-JUNETEENTH"),
        // 2026-07-03 - T1 - IFUS-NOTICE-2026-INDEPENDENCE - softs closed, index families early.
        (2026, 7, 3, Unsourced, T1, "IFUS-NOTICE-2026-INDEPENDENCE"),
        // 2026-07-06 - T1 - IFUS-NOTICE-2026-INDEPENDENCE - Cotton late, the rest regular.
        (2026, 7, 6, Unsourced, T1, "IFUS-NOTICE-2026-INDEPENDENCE"),
        // 2026-09-07 - T1 - IFUS-NOTICE-2026-LABORDAY - softs closed, FANG+ early,
        // the dollar index regular.
        (2026, 9, 7, Unsourced, T1, "IFUS-NOTICE-2026-LABORDAY"),
        // 2026-11-26 - T1 - IFUS-CAL-2026 - softs closed, index families `open1`.
        (2026, 11, 26, Unsourced, T1, "IFUS-CAL-2026"),
        // 2026-12-25 - T1 - IFUS-CAL-2026 - Christmas Day, every family closed.
        (2026, 12, 25, Closed, T1, "IFUS-CAL-2026"),
        // 2026-12-28 - T1 - IFUS-CAL-2026 - softs regular, index families `open1`.
        (2026, 12, 28, Unsourced, T1, "IFUS-CAL-2026"),
        // 2027-01-01 - T1 - IFUS-CAL-2027 - New Year's Day, every family closed.
        (2027, 1, 1, Closed, T1, "IFUS-CAL-2027"),
        // 2027-01-18 - T1 - IFUS-CAL-2027 - softs closed, index families `open1`.
        (2027, 1, 18, Unsourced, T1, "IFUS-CAL-2027"),
        // 2027-02-15 - T1 - IFUS-CAL-2027 - softs closed, index families `open1`.
        (2027, 2, 15, Unsourced, T1, "IFUS-CAL-2027"),
        // 2027-03-26 - T1 - IFUS-CAL-2027 - softs closed, index families `open1`.
        (2027, 3, 26, Unsourced, T1, "IFUS-CAL-2027"),
        // 2027-05-31 - T1 - IFUS-CAL-2027 - softs closed, index families `open1`.
        (2027, 5, 31, Unsourced, T1, "IFUS-CAL-2027"),
        // 2027-06-18 - T1 - IFUS-CAL-2027 - softs closed, index families `open1`.
        (2027, 6, 18, Unsourced, T1, "IFUS-CAL-2027"),
        // 2027-07-05 - T1 - IFUS-CAL-2027 - softs closed, index families `open1`.
        (2027, 7, 5, Unsourced, T1, "IFUS-CAL-2027"),
        // 2027-09-06 - T1 - IFUS-CAL-2027 - softs closed, index families `open1`.
        (2027, 9, 6, Unsourced, T1, "IFUS-CAL-2027"),
        // 2027-11-25 - T1 - IFUS-CAL-2027 - softs closed, index families `open1`.
        (2027, 11, 25, Unsourced, T1, "IFUS-CAL-2027"),
        // 2027-12-24 - T1 - IFUS-CAL-2027 - Christmas observed, every family closed.
        (2027, 12, 24, Closed, T1, "IFUS-CAL-2027"),
        // 2027-12-27 - T1 - IFUS-CAL-2027 - softs regular, index families `open1`.
        (2027, 12, 27, Unsourced, T1, "IFUS-CAL-2027"),
    ],
};
