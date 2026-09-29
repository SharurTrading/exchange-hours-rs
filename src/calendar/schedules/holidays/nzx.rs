// SPDX-License-Identifier: MIT-0

//! NZX Main Board holiday rows, 2010-2024 backfilled alongside the operator's
//! published rolling horizon (2025 through 2027-01-04).
//!
//! Keyed by the crate's venue-local trade date in `Pacific/Auckland` (design
//! memo D1). NZX sessions do not wrap past local midnight, so every holiday
//! row lands on its own civil date. NZX prints no consolidated year sheets:
//! the operator's `Market Holidays & Trading Hours` statements roll with the
//! page (the 2010-2017 era pages list upcoming closures, the 2018+ pages print
//! the current year's full table), so the rows are read from successive
//! Wayback `id_` replays of the operator's own pages, per era:
//! `nzx.com/markets/key-dates/trading-hours` (2010-2011),
//! `nzx.com/markets/NZSX/trading_hours` (2011-2017),
//! `nzx.com/investing/nzx-trading-hours` (2018-2019),
//! `nzx.com/services/nzx-trading/hours-boards` (2020-2024) and the current
//! `nzx.com/investing/nzx-trading-hours` page (2025-2027). The wayback replays
//! are verbatim captures of the operator's own pages, so every row is T1. The
//! per-row derivation is recorded in
//! [`docs/evidence/nzx.md`](../../../../../docs/evidence/nzx.md).
//!
//! The audited windows are `2010-01-01..2016-04-25` and
//! `2017-10-23..2027-01-04`. The span between them — 2016-04-26 through
//! 2017-10-22 — is a recorded gap: no capture of any operator trading-hours
//! page survives in the Wayback index for those eighteen months, so no
//! operator artifact prints those dates and the table claims nothing there.
//! Queries inside the gap refuse rather than answer; the gap and its closing
//! condition are recorded in the evidence file.
//!
//! The abbreviated-trading days ship as replacement block sets restating the
//! operator's own abbreviated grid for their era, **not** scalar early closes,
//! for the same reason the 2025-2026 module records: the operator's abbreviated
//! grid keeps a tradeable closing auction after the shortened Normal Trading
//! window, which a scalar clip cannot state. Three grids are in force across
//! the window, each printed by the era's own page:
//!
//! - **2010-2012** (`ERA_2010_ABBREVIATED_DAY_BLOCKS`): Pre-open 9:00-10:00,
//!   Normal Trading 10:00-15:45, Pre-Close 15:45-16:00, Adjust 16:00-16:30.
//! - **2013-2020** (`ERA_2013_ABBREVIATED_DAY_BLOCKS`): Pre-open 9:00-10:00,
//!   Normal Trading 10:00-12:45, Pre-Close 12:45-13:00, Adjust 13:00-13:30.
//!   The 2020 abbreviated days (24 and 31 December) hold the 9:00 Pre-open at
//!   its narrowest sourced value: the operator's 2020-06-08 page still printed
//!   the 9:00 Pre-open while the 2021-01-12 page printed 8:30, and no capture
//!   survives between them, so the disputed hour stays out.
//! - **2021 onward** (`ABBREVIATED_DAY_BLOCKS`): Pre-open 8:30-10:00 (the
//!   sourced 2020-04-06 pre-open move), Normal Trading 10:00-12:45, Pre-Close
//!   12:45-13:00, Adjust 13:00-13:30.
//!
//! In each grid the Pre-Close slice stops 30 seconds before its end so the
//! randomised closing uncross stays out of the order-entry block, and the
//! closing-uncross envelope (±30 seconds around the Pre-Close end) ships as
//! the tradeable `extended` block, exactly as the current era's blocks do.
//! Enquiry and Adjust accept no matched orders and state no blocks.

use super::EvidenceTier::T1;
use super::HolidayKind::{Closed, ReplacementBlocks};
use super::{HolidayTable, holidays};
use crate::calendar::exceptions::ExceptionBlock;

/// The operator's 2010-2012 abbreviated-trading day, relative to that trade
/// date.
///
/// Read from the abbreviated column of the operator's own page as captured
/// 2010-01-05 and 2011-12-19: Pre-open 09:00-10:00 (tradeable — off-market
/// reports print), Normal Trading 10:00-15:45, Pre-Close 15:45-16:00 (order
/// entry; the slice stops at 15:59:30 so the randomised uncross stays out of
/// the order-entry block), and the closing uncross envelope 15:59:30-16:00:30,
/// using the same ±30 second randomisation envelope the normal-week profile
/// carries around the Pre-Close end. Enquiry and Adjust accept no matched
/// orders and state no blocks.
///
/// Evidence: `docs/evidence/nzx.md`.
#[rustfmt::skip]
static ERA_2010_ABBREVIATED_DAY_BLOCKS: [ExceptionBlock; 4] = [
    ExceptionBlock::extended(0, 9 * 3_600, 10 * 3_600),
    ExceptionBlock::regular(0, 10 * 3_600, 15 * 3_600 + 45 * 60),
    ExceptionBlock::order_entry(0, 15 * 3_600 + 45 * 60, 15 * 3_600 + 59 * 60 + 30),
    ExceptionBlock::extended(0, 15 * 3_600 + 59 * 60 + 30, 16 * 3_600 + 30),
];

/// The operator's 2013-2020 abbreviated-trading day, relative to that trade
/// date.
///
/// Read from the abbreviated column of the operator's own pages as captured
/// 2013-01-16 through 2021-01-12: Pre-open 09:00-10:00 (tradeable — off-market
/// reports print), Normal Trading 10:00-12:45, Pre-Close 12:45-13:00 (order
/// entry; the slice stops at 12:59:30 so the randomised uncross stays out of
/// the order-entry block), and the closing uncross envelope 12:59:30-13:00:30.
/// The 2020 abbreviated days hold the 09:00 Pre-open at its narrowest sourced
/// value across the undated 2020-06-08..2021-01-12 span (see the module
/// header). Enquiry and Adjust accept no matched orders and state no blocks.
///
/// Evidence: `docs/evidence/nzx.md`.
#[rustfmt::skip]
static ERA_2013_ABBREVIATED_DAY_BLOCKS: [ExceptionBlock; 4] = [
    ExceptionBlock::extended(0, 9 * 3_600, 10 * 3_600),
    ExceptionBlock::regular(0, 10 * 3_600, 12 * 3_600 + 45 * 60),
    ExceptionBlock::order_entry(0, 12 * 3_600 + 45 * 60, 12 * 3_600 + 59 * 60 + 30),
    ExceptionBlock::extended(0, 12 * 3_600 + 59 * 60 + 30, 13 * 3_600 + 30),
];

/// The operator's abbreviated-trading day, relative to that trade date.
///
/// Read from the abbreviated column of the operator's own Main Board table:
/// Pre-open 08:30-10:00 (tradeable — off-market reports print), Normal Trading
/// 10:00-12:45, Pre-Close 12:45-13:00 (order entry; the slice stops at
/// 12:59:30 so the randomised uncross stays out of the order-entry block),
/// and the closing uncross envelope 12:59:30-13:00:30, using the same ±30
/// second randomisation envelope the normal-week profile carries around 17:00
/// from the operator's anatomy-of-a-trading-day statement. Enquiry and Adjust
/// accept no matched orders and state no blocks.
///
/// Evidence: `docs/evidence/nzx.md`.
#[rustfmt::skip]
static ABBREVIATED_DAY_BLOCKS: [ExceptionBlock; 4] = [
    ExceptionBlock::extended(0, 8 * 3_600 + 30 * 60, 10 * 3_600),
    ExceptionBlock::regular(0, 10 * 3_600, 12 * 3_600 + 45 * 60),
    ExceptionBlock::order_entry(0, 12 * 3_600 + 45 * 60, 12 * 3_600 + 59 * 60 + 30),
    ExceptionBlock::extended(0, 12 * 3_600 + 59 * 60 + 30, 13 * 3_600 + 30),
];

/// NZX's built-in holiday rows and the windows they were audited over.
///
/// Every date inside a window with no row is audited normal; the span between
/// the two windows (2016-04-26..2017-10-22) is the recorded capture gap and
/// carries no answer. 2010 rows cite the operator's key-dates page as replayed
/// 2010-01-05; 2011 rows the 2010-12-29 replay of the same page; 2012-2016
/// rows the operator's NZSX Main Board trading-hours page replays; 2017-2019
/// rows the `investing/nzx-trading-hours` page replays; 2020-2024 rows the
/// operator's `services/nzx-trading/hours-boards` page replays; 2025 rows cite
/// the operator's page as replayed 2025-01-23 (New Year's Day and the day
/// after, which that replay had already rotated out, cite the 2024-12-16
/// replay that still prints them), and Waitangi Day 2026 onward cite the
/// 2026-02-03 replay.
// Evidence: docs/evidence/nzx.md
pub(crate) static TABLE: &HolidayTable = holidays! {
    coverage: [(2010, 1, 1) ..= (2016, 4, 25), (2017, 10, 23) ..= (2027, 1, 4)],
    rows: [
        // 2010-01-01 - T1 - NZX-KD-2010-01-05 - New Year's Day, `Closed`.
        (2010, 1, 1, Closed, T1, "NZX-KD-2010-01-05"),
        // 2010-01-04 - T1 - NZX-KD-2010-01-05 - New Year's Day Holiday
        // (Monday, the sheet's own date for the Saturday 2 January holiday),
        // `Closed`.
        (2010, 1, 4, Closed, T1, "NZX-KD-2010-01-05"),
        // 2010-02-06 - T1 - NZX-KD-2010-01-05 - Waitangi Day (Saturday, as
        // printed; no mondayisation), `Closed`.
        (2010, 2, 6, Closed, T1, "NZX-KD-2010-01-05"),
        // 2010-04-01 - T1 - NZX-KD-2010-01-05 - Abbreviated Trading, the
        // business day prior to Good Friday: the operator's own abbreviated
        // grid as one replacement day.
        (2010, 4, 1, ReplacementBlocks(&ERA_2010_ABBREVIATED_DAY_BLOCKS), T1, "NZX-KD-2010-01-05"),
        // 2010-04-02 - T1 - NZX-KD-2010-01-05 - Good Friday, `Closed`.
        (2010, 4, 2, Closed, T1, "NZX-KD-2010-01-05"),
        // 2010-04-05 - T1 - NZX-KD-2010-01-05 - Easter Monday, `Closed`.
        (2010, 4, 5, Closed, T1, "NZX-KD-2010-01-05"),
        // 2010-04-25 - T1 - NZX-KD-2010-01-05 - ANZAC Day (Sunday, as
        // printed), `Closed`.
        (2010, 4, 25, Closed, T1, "NZX-KD-2010-01-05"),
        // 2010-06-07 - T1 - NZX-KD-2010-01-05 - Queen's Birthday, `Closed`.
        (2010, 6, 7, Closed, T1, "NZX-KD-2010-01-05"),
        // 2010-10-25 - T1 - NZX-KD-2010-01-05 - Labour Day, `Closed`.
        (2010, 10, 25, Closed, T1, "NZX-KD-2010-01-05"),
        // 2010-12-24 - T1 - NZX-KD-2010-01-05 - Abbreviated Trading: the
        // operator's own abbreviated grid as one replacement day.
        (2010, 12, 24, ReplacementBlocks(&ERA_2010_ABBREVIATED_DAY_BLOCKS), T1, "NZX-KD-2010-01-05"),
        // 2010-12-27 - T1 - NZX-KD-2010-01-05 - Christmas Day (Monday, the
        // sheet's own date for the Saturday 25 December holiday), `Closed`.
        (2010, 12, 27, Closed, T1, "NZX-KD-2010-01-05"),
        // 2010-12-28 - T1 - NZX-KD-2010-01-05 - Boxing Day (Tuesday), `Closed`.
        (2010, 12, 28, Closed, T1, "NZX-KD-2010-01-05"),
        // 2010-12-31 - T1 - NZX-KD-2010-01-05 - Abbreviated Trading.
        (2010, 12, 31, ReplacementBlocks(&ERA_2010_ABBREVIATED_DAY_BLOCKS), T1, "NZX-KD-2010-01-05"),
        // 2011-01-03 - T1 - NZX-KD-2010-12-29 - New Year's Day (Monday, the
        // sheet's own date for the Saturday 1 January holiday), `Closed`.
        (2011, 1, 3, Closed, T1, "NZX-KD-2010-12-29"),
        // 2011-01-04 - T1 - NZX-KD-2010-12-29 - New Year's Day Holiday
        // (Tuesday), `Closed`.
        (2011, 1, 4, Closed, T1, "NZX-KD-2010-12-29"),
        // 2011-02-06 - T1 - NZX-KD-2010-12-29 - Waitangi Day (Sunday, as
        // printed), `Closed`.
        (2011, 2, 6, Closed, T1, "NZX-KD-2010-12-29"),
        // 2011-04-21 - T1 - NZX-KD-2010-12-29 - Abbreviated Trading, the
        // business day prior to Good Friday.
        (2011, 4, 21, ReplacementBlocks(&ERA_2010_ABBREVIATED_DAY_BLOCKS), T1, "NZX-KD-2010-12-29"),
        // 2011-04-22 - T1 - NZX-KD-2010-12-29 - Good Friday, `Closed`.
        (2011, 4, 22, Closed, T1, "NZX-KD-2010-12-29"),
        // 2011-04-25 - T1 - NZX-KD-2010-12-29 - Easter Monday and ANZAC Day,
        // printed as one date, `Closed`.
        (2011, 4, 25, Closed, T1, "NZX-KD-2010-12-29"),
        // 2011-06-06 - T1 - NZX-KD-2010-12-29 - Queen's Birthday, `Closed`.
        (2011, 6, 6, Closed, T1, "NZX-KD-2010-12-29"),
        // 2011-10-24 - T1 - NZX-KD-2010-12-29 - Labour Day, `Closed`.
        (2011, 10, 24, Closed, T1, "NZX-KD-2010-12-29"),
        // 2011-12-23 - T1 - NZX-KD-2010-12-29 - Christmas Eve, Abbreviated
        // Trading.
        (2011, 12, 23, ReplacementBlocks(&ERA_2010_ABBREVIATED_DAY_BLOCKS), T1, "NZX-KD-2010-12-29"),
        // 2011-12-26 - T1 - NZX-KD-2010-12-29 - Boxing Day (Monday), `Closed`.
        (2011, 12, 26, Closed, T1, "NZX-KD-2010-12-29"),
        // 2011-12-27 - T1 - NZX-KD-2010-12-29 - Christmas Day (Tuesday, the
        // sheet's own date for the Sunday 25 December holiday), `Closed`.
        (2011, 12, 27, Closed, T1, "NZX-KD-2010-12-29"),
        // 2011-12-30 - T1 - NZX-KD-2010-12-29 - Abbreviated Trading.
        (2011, 12, 30, ReplacementBlocks(&ERA_2010_ABBREVIATED_DAY_BLOCKS), T1, "NZX-KD-2010-12-29"),
        // 2012-01-02 - T1 - NZX-SX-2011-12-19 - New Year (Monday), `Closed`.
        (2012, 1, 2, Closed, T1, "NZX-SX-2011-12-19"),
        // 2012-01-03 - T1 - NZX-SX-2011-12-19 - New Year (Tuesday), `Closed`.
        (2012, 1, 3, Closed, T1, "NZX-SX-2011-12-19"),
        // 2012-02-06 - T1 - NZX-SX-2011-12-19 - Waitangi Day (Monday),
        // `Closed`.
        (2012, 2, 6, Closed, T1, "NZX-SX-2011-12-19"),
        // 2012-04-05 - T1 - NZX-SX-2011-12-19 - Abbreviated Trading, the
        // business day prior to Good Friday.
        (2012, 4, 5, ReplacementBlocks(&ERA_2010_ABBREVIATED_DAY_BLOCKS), T1, "NZX-SX-2011-12-19"),
        // 2012-04-06 - T1 - NZX-SX-2011-12-19 - Good Friday, `Closed`.
        (2012, 4, 6, Closed, T1, "NZX-SX-2011-12-19"),
        // 2012-04-09 - T1 - NZX-SX-2011-12-19 - Easter Monday, `Closed`.
        (2012, 4, 9, Closed, T1, "NZX-SX-2011-12-19"),
        // 2012-04-25 - T1 - NZX-SX-2011-12-19 - Anzac Day, `Closed`.
        (2012, 4, 25, Closed, T1, "NZX-SX-2011-12-19"),
        // 2012-06-04 - T1 - NZX-SX-2012-05-04 - Queen's Birthday, `Closed`
        // (the 2011-12-19 replay prints the same row).
        (2012, 6, 4, Closed, T1, "NZX-SX-2012-05-04"),
        // 2012-10-22 - T1 - NZX-SX-2012-05-04 - Labour Day, `Closed` (the
        // 2011-12-19 replay prints the same row).
        (2012, 10, 22, Closed, T1, "NZX-SX-2012-05-04"),
        // 2012-12-24 - T1 - NZX-SX-2012-05-04 - Abbreviated Trading.
        (2012, 12, 24, ReplacementBlocks(&ERA_2010_ABBREVIATED_DAY_BLOCKS), T1, "NZX-SX-2012-05-04"),
        // 2012-12-25 - T1 - NZX-SX-2012-05-04 - Christmas Day, `Closed`.
        (2012, 12, 25, Closed, T1, "NZX-SX-2012-05-04"),
        // 2012-12-26 - T1 - NZX-SX-2012-05-04 - Boxing Day, `Closed`.
        (2012, 12, 26, Closed, T1, "NZX-SX-2012-05-04"),
        // 2012-12-31 - T1 - NZX-SX-2012-05-04 - Abbreviated Trading.
        (2012, 12, 31, ReplacementBlocks(&ERA_2010_ABBREVIATED_DAY_BLOCKS), T1, "NZX-SX-2012-05-04"),
        // 2013-01-01 - T1 - NZX-SX-2012-05-04 - New Year, `Closed`.
        (2013, 1, 1, Closed, T1, "NZX-SX-2012-05-04"),
        // 2013-01-02 - T1 - NZX-SX-2012-05-04 - New Year, `Closed`.
        (2013, 1, 2, Closed, T1, "NZX-SX-2012-05-04"),
        // 2013-02-06 - T1 - NZX-SX-2013-01-16 - Waitangi Day, `Closed`.
        (2013, 2, 6, Closed, T1, "NZX-SX-2013-01-16"),
        // 2013-03-29 - T1 - NZX-SX-2013-01-16 - Good Friday, `Closed`.
        (2013, 3, 29, Closed, T1, "NZX-SX-2013-01-16"),
        // 2013-04-01 - T1 - NZX-SX-2013-01-16 - Easter Monday, `Closed`.
        (2013, 4, 1, Closed, T1, "NZX-SX-2013-01-16"),
        // 2013-04-25 - T1 - NZX-SX-2013-01-16 - Anzac Day, `Closed`.
        (2013, 4, 25, Closed, T1, "NZX-SX-2013-01-16"),
        // 2013-06-03 - T1 - NZX-SX-2013-01-16 - Queen's Birthday, `Closed`
        // (the 2013-05-16 replay prints the same row).
        (2013, 6, 3, Closed, T1, "NZX-SX-2013-01-16"),
        // 2013-10-28 - T1 - NZX-SX-2013-01-16 - Labour Day, `Closed` (the
        // 2013-05-16 replay prints the same row).
        (2013, 10, 28, Closed, T1, "NZX-SX-2013-01-16"),
        // 2013-12-24 - T1 - NZX-SX-2013-05-16 - Christmas Eve, Abbreviated
        // Trading (the 12:45 abbreviated grid).
        (2013, 12, 24, ReplacementBlocks(&ERA_2013_ABBREVIATED_DAY_BLOCKS), T1, "NZX-SX-2013-05-16"),
        // 2013-12-25 - T1 - NZX-SX-2013-05-16 - Christmas Day, `Closed`.
        (2013, 12, 25, Closed, T1, "NZX-SX-2013-05-16"),
        // 2013-12-26 - T1 - NZX-SX-2013-05-16 - Boxing Day, `Closed`.
        (2013, 12, 26, Closed, T1, "NZX-SX-2013-05-16"),
        // 2013-12-31 - T1 - NZX-SX-2013-05-16 - Abbreviated Trading (the
        // 12:45 abbreviated grid).
        (2013, 12, 31, ReplacementBlocks(&ERA_2013_ABBREVIATED_DAY_BLOCKS), T1, "NZX-SX-2013-05-16"),
        // 2014-01-01 - T1 - NZX-SX-2013-05-16 - New Year, `Closed` (the
        // 2014-01-27 replay prints the same row).
        (2014, 1, 1, Closed, T1, "NZX-SX-2013-05-16"),
        // 2014-01-02 - T1 - NZX-SX-2013-05-16 - New Year, `Closed` (the
        // 2014-01-27 replay prints the same row).
        (2014, 1, 2, Closed, T1, "NZX-SX-2013-05-16"),
        // 2014-02-06 - T1 - NZX-SX-2014-01-27 - Waitangi Day, `Closed`.
        (2014, 2, 6, Closed, T1, "NZX-SX-2014-01-27"),
        // 2014-04-18 - T1 - NZX-SX-2014-01-27 - Good Friday, `Closed`.
        (2014, 4, 18, Closed, T1, "NZX-SX-2014-01-27"),
        // 2014-04-21 - T1 - NZX-SX-2014-01-27 - Easter Monday, `Closed`.
        (2014, 4, 21, Closed, T1, "NZX-SX-2014-01-27"),
        // 2014-04-25 - T1 - NZX-SX-2014-01-27 - Anzac Day, `Closed`.
        (2014, 4, 25, Closed, T1, "NZX-SX-2014-01-27"),
        // 2014-06-02 - T1 - NZX-SX-2014-01-27 - Queen's Birthday, `Closed`.
        (2014, 6, 2, Closed, T1, "NZX-SX-2014-01-27"),
        // 2014-10-27 - T1 - NZX-SX-2014-01-27 - Labour Day, `Closed` (the
        // 2014-10-20 replay prints the same row).
        (2014, 10, 27, Closed, T1, "NZX-SX-2014-01-27"),
        // 2014-12-24 - T1 - NZX-SX-2014-10-20 - Christmas Eve, Abbreviated
        // Trading (the 12:45 abbreviated grid).
        (2014, 12, 24, ReplacementBlocks(&ERA_2013_ABBREVIATED_DAY_BLOCKS), T1, "NZX-SX-2014-10-20"),
        // 2014-12-25 - T1 - NZX-SX-2014-10-20 - Christmas Day, `Closed`.
        (2014, 12, 25, Closed, T1, "NZX-SX-2014-10-20"),
        // 2014-12-26 - T1 - NZX-SX-2014-10-20 - Boxing Day, `Closed`.
        (2014, 12, 26, Closed, T1, "NZX-SX-2014-10-20"),
        // 2014-12-31 - T1 - NZX-SX-2014-10-20 - New Years Eve, Abbreviated
        // Trading.
        (2014, 12, 31, ReplacementBlocks(&ERA_2013_ABBREVIATED_DAY_BLOCKS), T1, "NZX-SX-2014-10-20"),
        // 2015-01-01 - T1 - NZX-SX-2014-10-20 - New Year, `Closed`.
        (2015, 1, 1, Closed, T1, "NZX-SX-2014-10-20"),
        // 2015-01-02 - T1 - NZX-SX-2014-10-20 - New Year, `Closed`.
        (2015, 1, 2, Closed, T1, "NZX-SX-2014-10-20"),
        // 2015-02-06 - T1 - NZX-SX-2015-01-13 - Waitangi Day, `Closed`.
        (2015, 2, 6, Closed, T1, "NZX-SX-2015-01-13"),
        // 2015-04-03 - T1 - NZX-SX-2015-01-13 - Good Friday, `Closed`.
        (2015, 4, 3, Closed, T1, "NZX-SX-2015-01-13"),
        // 2015-04-06 - T1 - NZX-SX-2015-01-13 - Easter Monday, `Closed`.
        (2015, 4, 6, Closed, T1, "NZX-SX-2015-01-13"),
        // 2015-04-25 - T1 - NZX-SX-2015-01-13 - Anzac Day (Saturday, as
        // printed), `Closed`.
        (2015, 4, 25, Closed, T1, "NZX-SX-2015-01-13"),
        // 2015-04-27 - T1 - NZX-SX-2015-01-13 - Anzac Observance (Monday,
        // the sheet's own mondayised date), `Closed`.
        (2015, 4, 27, Closed, T1, "NZX-SX-2015-01-13"),
        // 2015-06-01 - T1 - NZX-SX-2015-01-13 - Queen's Birthday, `Closed`
        // (the 2015-04-27 replay prints the same row).
        (2015, 6, 1, Closed, T1, "NZX-SX-2015-01-13"),
        // 2015-10-26 - T1 - NZX-SX-2015-01-13 - Labour Day, `Closed` (the
        // 2015-04-27 replay prints the same row).
        (2015, 10, 26, Closed, T1, "NZX-SX-2015-01-13"),
        // 2015-12-24 - T1 - NZX-SX-2015-01-13 - Christmas Eve, Abbreviated
        // Trading (the 12:45 abbreviated grid).
        (2015, 12, 24, ReplacementBlocks(&ERA_2013_ABBREVIATED_DAY_BLOCKS), T1, "NZX-SX-2015-01-13"),
        // 2015-12-25 - T1 - NZX-SX-2015-01-13 - Christmas Day, `Closed`.
        (2015, 12, 25, Closed, T1, "NZX-SX-2015-01-13"),
        // 2015-12-28 - T1 - NZX-SX-2015-01-13 - Boxing Day (Monday, the
        // sheet's own date for the Saturday 26 December holiday), `Closed`.
        (2015, 12, 28, Closed, T1, "NZX-SX-2015-01-13"),
        // 2015-12-31 - T1 - NZX-SX-2015-01-13 - New Years Eve, Abbreviated
        // Trading.
        (2015, 12, 31, ReplacementBlocks(&ERA_2013_ABBREVIATED_DAY_BLOCKS), T1, "NZX-SX-2015-01-13"),
        // 2016-01-01 - T1 - NZX-SX-2015-04-27 - New Year, `Closed`.
        (2016, 1, 1, Closed, T1, "NZX-SX-2015-04-27"),
        // 2016-01-04 - T1 - NZX-SX-2015-04-27 - New Year (Monday, the
        // sheet's own date for the Saturday 2 January holiday), `Closed`.
        (2016, 1, 4, Closed, T1, "NZX-SX-2015-04-27"),
        // 2016-02-06 - T1 - NZX-SX-2015-04-27 - Waitangi Day (Saturday, as
        // printed), `Closed`.
        (2016, 2, 6, Closed, T1, "NZX-SX-2015-04-27"),
        // 2016-02-08 - T1 - NZX-SX-2015-04-27 - Waitangi Observance (Monday,
        // the sheet's own mondayised date), `Closed`.
        (2016, 2, 8, Closed, T1, "NZX-SX-2015-04-27"),
        // 2016-03-25 - T1 - NZX-SX-2015-04-27 - Good Friday, `Closed`.
        (2016, 3, 25, Closed, T1, "NZX-SX-2015-04-27"),
        // 2016-03-28 - T1 - NZX-SX-2015-04-27 - Easter Monday, `Closed`.
        (2016, 3, 28, Closed, T1, "NZX-SX-2015-04-27"),
        // 2016-04-25 - T1 - NZX-SX-2015-04-27 - Anzac Day, `Closed`, the
        // last date any surviving operator artifact prints before the
        // 2016-2017 capture gap.
        (2016, 4, 25, Closed, T1, "NZX-SX-2015-04-27"),
        // 2017-10-23 - T1 - NZX-SX-2017-06-23 - Labour Day, `Closed`, the
        // first date the next surviving operator artifact prints after the
        // capture gap.
        (2017, 10, 23, Closed, T1, "NZX-SX-2017-06-23"),
        // 2017-12-22 - T1 - NZX-SX-2017-06-23 - Christmas Eve, Abbreviated
        // Trading (the 12:45 abbreviated grid).
        (2017, 12, 22, ReplacementBlocks(&ERA_2013_ABBREVIATED_DAY_BLOCKS), T1, "NZX-SX-2017-06-23"),
        // 2017-12-25 - T1 - NZX-SX-2017-06-23 - Christmas Day, `Closed`.
        (2017, 12, 25, Closed, T1, "NZX-SX-2017-06-23"),
        // 2017-12-26 - T1 - NZX-SX-2017-06-23 - Boxing Day, `Closed`.
        (2017, 12, 26, Closed, T1, "NZX-SX-2017-06-23"),
        // 2017-12-29 - T1 - NZX-SX-2017-06-23 - New Years Eve, Abbreviated
        // Trading.
        (2017, 12, 29, ReplacementBlocks(&ERA_2013_ABBREVIATED_DAY_BLOCKS), T1, "NZX-SX-2017-06-23"),
        // 2018-01-01 - T1 - NZX-TH-2018-08-24 - New Years Day, `Closed`.
        (2018, 1, 1, Closed, T1, "NZX-TH-2018-08-24"),
        // 2018-01-02 - T1 - NZX-TH-2018-08-24 - Day after New Year's Day,
        // `Closed`.
        (2018, 1, 2, Closed, T1, "NZX-TH-2018-08-24"),
        // 2018-02-06 - T1 - NZX-TH-2018-08-24 - Waitangi Day, `Closed`.
        (2018, 2, 6, Closed, T1, "NZX-TH-2018-08-24"),
        // 2018-03-30 - T1 - NZX-TH-2018-08-24 - Good Friday, `Closed`.
        (2018, 3, 30, Closed, T1, "NZX-TH-2018-08-24"),
        // 2018-04-02 - T1 - NZX-TH-2018-08-24 - Easter Monday, `Closed`.
        (2018, 4, 2, Closed, T1, "NZX-TH-2018-08-24"),
        // 2018-04-25 - T1 - NZX-TH-2018-08-24 - Anzac Day, `Closed`.
        (2018, 4, 25, Closed, T1, "NZX-TH-2018-08-24"),
        // 2018-06-04 - T1 - NZX-TH-2018-08-24 - Queen's Birthday, `Closed`.
        (2018, 6, 4, Closed, T1, "NZX-TH-2018-08-24"),
        // 2018-10-22 - T1 - NZX-TH-2018-08-24 - Labour Day, `Closed`.
        (2018, 10, 22, Closed, T1, "NZX-TH-2018-08-24"),
        // 2018-12-24 - T1 - NZX-TH-2018-08-24 - Business Day prior to
        // Christmas Day, Abbreviated Trading (the 12:45 abbreviated grid).
        (2018, 12, 24, ReplacementBlocks(&ERA_2013_ABBREVIATED_DAY_BLOCKS), T1, "NZX-TH-2018-08-24"),
        // 2018-12-25 - T1 - NZX-TH-2018-08-24 - Christmas Day, `Closed`.
        (2018, 12, 25, Closed, T1, "NZX-TH-2018-08-24"),
        // 2018-12-26 - T1 - NZX-TH-2018-08-24 - Boxing Day, `Closed`.
        (2018, 12, 26, Closed, T1, "NZX-TH-2018-08-24"),
        // 2018-12-31 - T1 - NZX-TH-2018-08-24 - Business Day prior to New
        // Year's Day, Abbreviated Trading.
        (2018, 12, 31, ReplacementBlocks(&ERA_2013_ABBREVIATED_DAY_BLOCKS), T1, "NZX-TH-2018-08-24"),
        // 2019-01-01 - T1 - NZX-TH-2019-03-25 - New Years Day, `Closed`.
        (2019, 1, 1, Closed, T1, "NZX-TH-2019-03-25"),
        // 2019-01-02 - T1 - NZX-TH-2019-03-25 - Day after New Year's Day,
        // `Closed`.
        (2019, 1, 2, Closed, T1, "NZX-TH-2019-03-25"),
        // 2019-02-06 - T1 - NZX-TH-2019-03-25 - Waitangi Day, `Closed`.
        (2019, 2, 6, Closed, T1, "NZX-TH-2019-03-25"),
        // 2019-04-19 - T1 - NZX-TH-2019-03-25 - Good Friday, `Closed`.
        (2019, 4, 19, Closed, T1, "NZX-TH-2019-03-25"),
        // 2019-04-22 - T1 - NZX-TH-2019-03-25 - Easter Monday, `Closed`.
        (2019, 4, 22, Closed, T1, "NZX-TH-2019-03-25"),
        // 2019-04-25 - T1 - NZX-TH-2019-03-25 - ANZAC Day, `Closed`.
        (2019, 4, 25, Closed, T1, "NZX-TH-2019-03-25"),
        // 2019-06-03 - T1 - NZX-TH-2019-03-25 - Queen's Birthday, `Closed`.
        (2019, 6, 3, Closed, T1, "NZX-TH-2019-03-25"),
        // 2019-10-28 - T1 - NZX-TH-2019-03-25 - Labour Day, `Closed`.
        (2019, 10, 28, Closed, T1, "NZX-TH-2019-03-25"),
        // 2019-12-24 - T1 - NZX-TH-2019-03-25 - Business Day prior to
        // Christmas Day, Abbreviated Trading (the 12:45 abbreviated grid).
        (2019, 12, 24, ReplacementBlocks(&ERA_2013_ABBREVIATED_DAY_BLOCKS), T1, "NZX-TH-2019-03-25"),
        // 2019-12-25 - T1 - NZX-TH-2019-03-25 - Christmas Day, `Closed`.
        (2019, 12, 25, Closed, T1, "NZX-TH-2019-03-25"),
        // 2019-12-26 - T1 - NZX-TH-2019-03-25 - Boxing Day, `Closed`.
        (2019, 12, 26, Closed, T1, "NZX-TH-2019-03-25"),
        // 2019-12-31 - T1 - NZX-TH-2019-03-25 - Business Day prior to New
        // Year's Day, Abbreviated Trading.
        (2019, 12, 31, ReplacementBlocks(&ERA_2013_ABBREVIATED_DAY_BLOCKS), T1, "NZX-TH-2019-03-25"),
        // 2020-01-01 - T1 - NZX-HB-2020-06-08 - New Years Day, `Closed`.
        (2020, 1, 1, Closed, T1, "NZX-HB-2020-06-08"),
        // 2020-01-02 - T1 - NZX-HB-2020-06-08 - Day after New Year's Day,
        // `Closed`.
        (2020, 1, 2, Closed, T1, "NZX-HB-2020-06-08"),
        // 2020-02-06 - T1 - NZX-HB-2020-06-08 - Waitangi Day, `Closed`.
        (2020, 2, 6, Closed, T1, "NZX-HB-2020-06-08"),
        // 2020-04-10 - T1 - NZX-HB-2020-06-08 - Good Friday, `Closed`.
        (2020, 4, 10, Closed, T1, "NZX-HB-2020-06-08"),
        // 2020-04-13 - T1 - NZX-HB-2020-06-08 - Easter Monday, `Closed`.
        (2020, 4, 13, Closed, T1, "NZX-HB-2020-06-08"),
        // 2020-04-27 - T1 - NZX-HB-2020-06-08 - ANZAC Day observed (Monday,
        // the sheet's own date for the Saturday 25 April holiday; the sheet
        // prints no Saturday row), `Closed`.
        (2020, 4, 27, Closed, T1, "NZX-HB-2020-06-08"),
        // 2020-06-01 - T1 - NZX-HB-2020-06-08 - Queen's Birthday, `Closed`.
        (2020, 6, 1, Closed, T1, "NZX-HB-2020-06-08"),
        // 2020-10-26 - T1 - NZX-HB-2020-06-08 - Labour Day, `Closed`.
        (2020, 10, 26, Closed, T1, "NZX-HB-2020-06-08"),
        // 2020-12-24 - T1 - NZX-HB-2020-06-08 - Business Day prior to
        // Christmas Day, Abbreviated Trading (the 2013-2020 grid; the 09:00
        // Pre-open is the narrowest sourced value across the 2020-06-08 and
        // 2021-01-12 replays).
        (2020, 12, 24, ReplacementBlocks(&ERA_2013_ABBREVIATED_DAY_BLOCKS), T1, "NZX-HB-2020-06-08"),
        // 2020-12-25 - T1 - NZX-HB-2020-06-08 - Christmas Day, `Closed`.
        (2020, 12, 25, Closed, T1, "NZX-HB-2020-06-08"),
        // 2020-12-28 - T1 - NZX-HB-2020-06-08 - Boxing Day (Monday, the
        // sheet's own date for the Saturday 26 December holiday), `Closed`.
        (2020, 12, 28, Closed, T1, "NZX-HB-2020-06-08"),
        // 2020-12-31 - T1 - NZX-HB-2021-01-12 - Business Day Prior to New
        // Year's Day, Abbreviated Trading (the 2013-2020 grid, same Pre-open
        // reading as 2020-12-24).
        (2020, 12, 31, ReplacementBlocks(&ERA_2013_ABBREVIATED_DAY_BLOCKS), T1, "NZX-HB-2021-01-12"),
        // 2021-01-01 - T1 - NZX-HB-2021-01-12 - New Years Day, `Closed`.
        (2021, 1, 1, Closed, T1, "NZX-HB-2021-01-12"),
        // 2021-01-04 - T1 - NZX-HB-2021-01-12 - Day after New Year's Day
        // (Monday, the sheet's own date for the Saturday 2 January holiday),
        // `Closed`.
        (2021, 1, 4, Closed, T1, "NZX-HB-2021-01-12"),
        // 2021-02-08 - T1 - NZX-HB-2021-01-12 - Waitangi Day (Monday, the
        // sheet's own date for the Saturday 6 February holiday; the sheet
        // prints no Saturday row), `Closed`.
        (2021, 2, 8, Closed, T1, "NZX-HB-2021-01-12"),
        // 2021-04-02 - T1 - NZX-HB-2021-01-12 - Good Friday, `Closed`.
        (2021, 4, 2, Closed, T1, "NZX-HB-2021-01-12"),
        // 2021-04-05 - T1 - NZX-HB-2021-01-12 - Easter Monday, `Closed`.
        (2021, 4, 5, Closed, T1, "NZX-HB-2021-01-12"),
        // 2021-04-26 - T1 - NZX-HB-2021-01-12 - ANZAC Day observed (Monday,
        // the sheet's own date for the Sunday 25 April holiday), `Closed`.
        (2021, 4, 26, Closed, T1, "NZX-HB-2021-01-12"),
        // 2021-06-07 - T1 - NZX-HB-2021-01-12 - Queen's Birthday, `Closed`.
        (2021, 6, 7, Closed, T1, "NZX-HB-2021-01-12"),
        // 2021-10-25 - T1 - NZX-HB-2021-01-12 - Labour Day, `Closed`.
        (2021, 10, 25, Closed, T1, "NZX-HB-2021-01-12"),
        // 2021-12-24 - T1 - NZX-HB-2021-01-12 - Business Day prior to
        // Christmas Day, Abbreviated Trading (the 08:30-pre-open grid that
        // page's own Main Board table prints).
        (2021, 12, 24, ReplacementBlocks(&ABBREVIATED_DAY_BLOCKS), T1, "NZX-HB-2021-01-12"),
        // 2021-12-27 - T1 - NZX-HB-2021-01-12 - Christmas Day (Monday, the
        // sheet's own date for the Saturday 25 December holiday), `Closed`.
        (2021, 12, 27, Closed, T1, "NZX-HB-2021-01-12"),
        // 2021-12-28 - T1 - NZX-HB-2021-01-12 - Boxing Day (Tuesday),
        // `Closed`.
        (2021, 12, 28, Closed, T1, "NZX-HB-2021-01-12"),
        // 2021-12-31 - T1 - NZX-HB-2021-01-12 - Business Day Prior to New
        // Year's Day, Abbreviated Trading.
        (2021, 12, 31, ReplacementBlocks(&ABBREVIATED_DAY_BLOCKS), T1, "NZX-HB-2021-01-12"),
        // 2022-01-03 - T1 - NZX-HB-2022-02-01 - New Years Day (Monday, the
        // sheet's own date for the Saturday 1 January holiday), `Closed`.
        (2022, 1, 3, Closed, T1, "NZX-HB-2022-02-01"),
        // 2022-01-04 - T1 - NZX-HB-2022-02-01 - Day after New Year's Day
        // (Tuesday), `Closed`.
        (2022, 1, 4, Closed, T1, "NZX-HB-2022-02-01"),
        // 2022-02-07 - T1 - NZX-HB-2022-02-01 - Waitangi Day (Monday, the
        // sheet's own date for the Sunday 6 February holiday), `Closed`.
        (2022, 2, 7, Closed, T1, "NZX-HB-2022-02-01"),
        // 2022-04-15 - T1 - NZX-HB-2022-02-01 - Good Friday, `Closed`.
        (2022, 4, 15, Closed, T1, "NZX-HB-2022-02-01"),
        // 2022-04-18 - T1 - NZX-HB-2022-02-01 - Easter Monday, `Closed`.
        (2022, 4, 18, Closed, T1, "NZX-HB-2022-02-01"),
        // 2022-04-25 - T1 - NZX-HB-2022-02-01 - ANZAC Day (Monday),
        // `Closed`.
        (2022, 4, 25, Closed, T1, "NZX-HB-2022-02-01"),
        // 2022-06-06 - T1 - NZX-HB-2022-02-01 - Queen's Birthday, `Closed`.
        (2022, 6, 6, Closed, T1, "NZX-HB-2022-02-01"),
        // 2022-06-24 - T1 - NZX-HB-2022-02-01 - Matariki, `Closed`, the
        // holiday's first observance.
        (2022, 6, 24, Closed, T1, "NZX-HB-2022-02-01"),
        // 2022-09-26 - T1 - NZX-HB-2022-11-15 - Queen Elizabeth II Memorial
        // Day, `Closed`: the sheet's own footnote states the exchange will be
        // closed on 26 September, a national public holiday to honour the
        // passing of Queen Elizabeth II.
        (2022, 9, 26, Closed, T1, "NZX-HB-2022-11-15"),
        // 2022-10-24 - T1 - NZX-HB-2022-11-15 - Labour Day, `Closed`.
        (2022, 10, 24, Closed, T1, "NZX-HB-2022-11-15"),
        // 2022-12-23 - T1 - NZX-HB-2022-11-15 - Business Day Prior to
        // Christmas Day, Abbreviated Trading (the 08:30-pre-open grid).
        (2022, 12, 23, ReplacementBlocks(&ABBREVIATED_DAY_BLOCKS), T1, "NZX-HB-2022-11-15"),
        // 2022-12-26 - T1 - NZX-HB-2022-11-15 - Boxing Day (Monday),
        // `Closed`.
        (2022, 12, 26, Closed, T1, "NZX-HB-2022-11-15"),
        // 2022-12-27 - T1 - NZX-HB-2022-11-15 - Christmas Day (Tuesday, the
        // sheet's own date for the Sunday 25 December holiday), `Closed`.
        (2022, 12, 27, Closed, T1, "NZX-HB-2022-11-15"),
        // 2022-12-30 - T1 - NZX-HB-2022-11-15 - Business Day Prior to New
        // Year's Day, Abbreviated Trading.
        (2022, 12, 30, ReplacementBlocks(&ABBREVIATED_DAY_BLOCKS), T1, "NZX-HB-2022-11-15"),
        // 2023-01-02 - T1 - NZX-HB-2023-11-06 - New Year's Day (Monday, the
        // sheet's own date for the Sunday 1 January holiday), `Closed`.
        (2023, 1, 2, Closed, T1, "NZX-HB-2023-11-06"),
        // 2023-01-03 - T1 - NZX-HB-2023-11-06 - Day after New Year's Day
        // (Tuesday), `Closed`.
        (2023, 1, 3, Closed, T1, "NZX-HB-2023-11-06"),
        // 2023-02-06 - T1 - NZX-HB-2023-11-06 - Waitangi Day (Monday),
        // `Closed`.
        (2023, 2, 6, Closed, T1, "NZX-HB-2023-11-06"),
        // 2023-04-07 - T1 - NZX-HB-2023-11-06 - Good Friday, `Closed`.
        (2023, 4, 7, Closed, T1, "NZX-HB-2023-11-06"),
        // 2023-04-10 - T1 - NZX-HB-2023-11-06 - Easter Monday, `Closed`.
        (2023, 4, 10, Closed, T1, "NZX-HB-2023-11-06"),
        // 2023-04-25 - T1 - NZX-HB-2023-11-06 - ANZAC Day (Tuesday),
        // `Closed`.
        (2023, 4, 25, Closed, T1, "NZX-HB-2023-11-06"),
        // 2023-06-05 - T1 - NZX-HB-2023-11-06 - King's Birthday, `Closed`.
        (2023, 6, 5, Closed, T1, "NZX-HB-2023-11-06"),
        // 2023-07-14 - T1 - NZX-HB-2023-11-06 - Matariki, `Closed`.
        (2023, 7, 14, Closed, T1, "NZX-HB-2023-11-06"),
        // 2023-10-23 - T1 - NZX-HB-2023-11-06 - Labour Day, `Closed`.
        (2023, 10, 23, Closed, T1, "NZX-HB-2023-11-06"),
        // 2023-12-22 - T1 - NZX-HB-2023-11-06 - Business Day Prior to
        // Christmas Day, Abbreviated Trading (the 08:30-pre-open grid).
        (2023, 12, 22, ReplacementBlocks(&ABBREVIATED_DAY_BLOCKS), T1, "NZX-HB-2023-11-06"),
        // 2023-12-25 - T1 - NZX-HB-2023-11-06 - Christmas Day, `Closed`.
        (2023, 12, 25, Closed, T1, "NZX-HB-2023-11-06"),
        // 2023-12-26 - T1 - NZX-HB-2023-11-06 - Boxing Day, `Closed`.
        (2023, 12, 26, Closed, T1, "NZX-HB-2023-11-06"),
        // 2023-12-29 - T1 - NZX-HB-2023-11-06 - Business Day Prior to New
        // Year's Day, Abbreviated Trading.
        (2023, 12, 29, ReplacementBlocks(&ABBREVIATED_DAY_BLOCKS), T1, "NZX-HB-2023-11-06"),
        // 2024-01-01 - T1 - NZX-HB-2024-02-19 - New Year's Day, `Closed`.
        (2024, 1, 1, Closed, T1, "NZX-HB-2024-02-19"),
        // 2024-01-02 - T1 - NZX-HB-2024-02-19 - Day after New Year's Day,
        // `Closed`.
        (2024, 1, 2, Closed, T1, "NZX-HB-2024-02-19"),
        // 2024-02-06 - T1 - NZX-HB-2024-02-19 - Waitangi Day, `Closed`.
        (2024, 2, 6, Closed, T1, "NZX-HB-2024-02-19"),
        // 2024-03-29 - T1 - NZX-HB-2024-02-19 - Good Friday, `Closed`.
        (2024, 3, 29, Closed, T1, "NZX-HB-2024-02-19"),
        // 2024-04-01 - T1 - NZX-HB-2024-02-19 - Easter Monday, `Closed`.
        (2024, 4, 1, Closed, T1, "NZX-HB-2024-02-19"),
        // 2024-04-25 - T1 - NZX-HB-2024-02-19 - ANZAC Day (Thursday),
        // `Closed`.
        (2024, 4, 25, Closed, T1, "NZX-HB-2024-02-19"),
        // 2024-06-03 - T1 - NZX-HB-2024-02-19 - King's Birthday, `Closed`.
        (2024, 6, 3, Closed, T1, "NZX-HB-2024-02-19"),
        // 2024-06-28 - T1 - NZX-HB-2024-02-19 - Matariki, `Closed`.
        (2024, 6, 28, Closed, T1, "NZX-HB-2024-02-19"),
        // 2024-10-28 - T1 - NZX-HB-2024-02-19 - Labour Day, `Closed`.
        (2024, 10, 28, Closed, T1, "NZX-HB-2024-02-19"),
        // 2024-12-24 - T1 - NZX-HB-2024-02-19 - Business Day Prior to
        // Christmas Day, Abbreviated Trading (the 08:30-pre-open grid).
        (2024, 12, 24, ReplacementBlocks(&ABBREVIATED_DAY_BLOCKS), T1, "NZX-HB-2024-02-19"),
        // 2024-12-25 - T1 - NZX-HB-2024-02-19 - Christmas Day, `Closed`.
        (2024, 12, 25, Closed, T1, "NZX-HB-2024-02-19"),
        // 2024-12-26 - T1 - NZX-HB-2024-02-19 - Boxing Day, `Closed`.
        (2024, 12, 26, Closed, T1, "NZX-HB-2024-02-19"),
        // 2024-12-31 - T1 - NZX-HB-2024-02-19 - Business Day Prior to New
        // Year's Day, Abbreviated Trading.
        (2024, 12, 31, ReplacementBlocks(&ABBREVIATED_DAY_BLOCKS), T1, "NZX-HB-2024-02-19"),
        // 2025-01-01 - T1 - NZX-TH-2024-12-16 - New Year's Day, `Closed`.
        (2025, 1, 1, Closed, T1, "NZX-TH-2024-12-16"),
        // 2025-01-02 - T1 - NZX-TH-2024-12-16 - Day after New Year's Day, `Closed`.
        (2025, 1, 2, Closed, T1, "NZX-TH-2024-12-16"),
        // 2025-02-06 - T1 - NZX-TH-2025-01-23 - Waitangi Day, `Closed`.
        (2025, 2, 6, Closed, T1, "NZX-TH-2025-01-23"),
        // 2025-04-18 - T1 - NZX-TH-2025-01-23 - Good Friday, `Closed`.
        (2025, 4, 18, Closed, T1, "NZX-TH-2025-01-23"),
        // 2025-04-21 - T1 - NZX-TH-2025-01-23 - Easter Monday, `Closed`.
        (2025, 4, 21, Closed, T1, "NZX-TH-2025-01-23"),
        // 2025-04-25 - T1 - NZX-TH-2025-01-23 - ANZAC Day, `Closed`.
        (2025, 4, 25, Closed, T1, "NZX-TH-2025-01-23"),
        // 2025-06-02 - T1 - NZX-TH-2025-01-23 - King's Birthday, `Closed`.
        (2025, 6, 2, Closed, T1, "NZX-TH-2025-01-23"),
        // 2025-06-20 - T1 - NZX-TH-2025-01-23 - Matariki, `Closed`.
        (2025, 6, 20, Closed, T1, "NZX-TH-2025-01-23"),
        // 2025-10-27 - T1 - NZX-TH-2025-01-23 - Labour Day, `Closed`.
        (2025, 10, 27, Closed, T1, "NZX-TH-2025-01-23"),
        // 2025-12-24 - T1 - NZX-TH-2025-01-23 - Business Day Prior to Christmas
        // Day, `Abbreviated Trading`: the operator's own abbreviated grid as
        // one replacement day.
        (2025, 12, 24, ReplacementBlocks(&ABBREVIATED_DAY_BLOCKS), T1, "NZX-TH-2025-01-23"),
        // 2025-12-25 - T1 - NZX-TH-2025-01-23 - Christmas Day, `Closed`.
        (2025, 12, 25, Closed, T1, "NZX-TH-2025-01-23"),
        // 2025-12-26 - T1 - NZX-TH-2025-01-23 - Boxing Day, `Closed`.
        (2025, 12, 26, Closed, T1, "NZX-TH-2025-01-23"),
        // 2025-12-31 - T1 - NZX-TH-2025-01-23 - Business Day Prior to New
        // Year's Day, `Abbreviated Trading`.
        (2025, 12, 31, ReplacementBlocks(&ABBREVIATED_DAY_BLOCKS), T1, "NZX-TH-2025-01-23"),
        // 2026-01-01 - T1 - NZX-TH-2025-01-23 - New Year's Day, `Closed`.
        (2026, 1, 1, Closed, T1, "NZX-TH-2025-01-23"),
        // 2026-01-02 - T1 - NZX-TH-2025-01-23 - Day after New Year's Day,
        // `Closed`.
        (2026, 1, 2, Closed, T1, "NZX-TH-2025-01-23"),
        // 2026-02-06 - T1 - NZX-TH-2026-02-03 - Waitangi Day, `Closed`.
        (2026, 2, 6, Closed, T1, "NZX-TH-2026-02-03"),
        // 2026-04-03 - T1 - NZX-TH-2026-02-03 - Good Friday, `Closed`.
        (2026, 4, 3, Closed, T1, "NZX-TH-2026-02-03"),
        // 2026-04-06 - T1 - NZX-TH-2026-02-03 - Easter Monday, `Closed`.
        (2026, 4, 6, Closed, T1, "NZX-TH-2026-02-03"),
        // 2026-04-27 - T1 - NZX-TH-2026-02-03 - ANZAC Day, `Closed`: the sheet
        // mondayises the Saturday to Monday 27 April.
        (2026, 4, 27, Closed, T1, "NZX-TH-2026-02-03"),
        // 2026-06-01 - T1 - NZX-TH-2026-02-03 - King's Birthday, `Closed`.
        (2026, 6, 1, Closed, T1, "NZX-TH-2026-02-03"),
        // 2026-07-10 - T1 - NZX-TH-2026-02-03 - Matariki, `Closed`.
        (2026, 7, 10, Closed, T1, "NZX-TH-2026-02-03"),
        // 2026-10-26 - T1 - NZX-TH-2026-02-03 - Labour Day, `Closed`.
        (2026, 10, 26, Closed, T1, "NZX-TH-2026-02-03"),
        // 2026-12-24 - T1 - NZX-TH-2026-02-03 - Business Day Prior to Christmas
        // Day, `Abbreviated Trading`.
        (2026, 12, 24, ReplacementBlocks(&ABBREVIATED_DAY_BLOCKS), T1, "NZX-TH-2026-02-03"),
        // 2026-12-25 - T1 - NZX-TH-2026-02-03 - Christmas Day, `Closed`.
        (2026, 12, 25, Closed, T1, "NZX-TH-2026-02-03"),
        // 2026-12-28 - T1 - NZX-TH-2026-02-03 - Boxing Day, `Closed`: the
        // sheet mondayises the Saturday to Monday 28 December.
        (2026, 12, 28, Closed, T1, "NZX-TH-2026-02-03"),
        // 2026-12-31 - T1 - NZX-TH-2026-02-03 - Business Day Prior to New
        // Year's Day, `Abbreviated Trading`.
        (2026, 12, 31, ReplacementBlocks(&ABBREVIATED_DAY_BLOCKS), T1, "NZX-TH-2026-02-03"),
        // 2027-01-01 - T1 - NZX-TH-2026-02-03 - New Year's Day, `Closed`, the
        // last year the operator's table reaches.
        (2027, 1, 1, Closed, T1, "NZX-TH-2026-02-03"),
        // 2027-01-04 - T1 - NZX-TH-2026-02-03 - Day after New Year's Day,
        // `Closed`, the operator's published horizon.
        (2027, 1, 4, Closed, T1, "NZX-TH-2026-02-03"),
    ],
};
