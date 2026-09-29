// SPDX-License-Identifier: MIT-0

//! SGX-ST securities-market holiday rows, 2014-2020-01-01 backfilled
//! alongside the operator's published 2025-2026 sheets.
//!
//! Keyed by the crate's venue-local trade date in `Asia/Singapore` (design
//! memo D1). SGX sessions do not wrap past local midnight, so every holiday
//! row lands on its own civil date. The operator's own securities
//! `Trading Hours & Calendar` page (`sgx.com/wps/portal/sgxweb/home/trading/
//! securities/trading_hours_calendar`) prints one `Public Holidays <year>`
//! table per year with the operator's own substitution and half-day markers —
//! `*` names the following Monday and `#` names the preceding half day — so
//! the 2014-2019 rows (and 2020 New Year's Day) are read from that page's
//! Wayback `id_` replays, verbatim bytes of an operator statement and
//! therefore T1. The 2025-2026 rows continue to cite the operator's current
//! `Stock Exchange — Trading` page read as bytes from SGX's content API
//! (T2), which designates the MOM calendar and prints the current half-day
//! grid. The per-row derivation is recorded in
//! [`docs/evidence/sgx_securities.md`](../../../../../docs/evidence/sgx_securities.md).
//!
//! The audited windows are `2014-01-01..2019-12-31`, `2020-01-01` (the one
//! 2020 date the 2019 sheet prints) and `2025-01-01..2026-12-31`. The spans
//! between them — 2010-01-01 through 2013-12-31 and 2020-01-02 through
//! 2024-12-31 — are recorded gaps: no capture of any operator trading-hours
//! page survives in the Wayback index for those eras, so no operator
//! artifact prints those closures and the table claims nothing there.
//! Queries inside the gaps refuse rather than answer; the gaps and their
//! closing conditions are recorded in the evidence file and tracked as
//! issue #213.
//!
//! The pre-2025 half-day dates ship as replacement block sets restating the
//! half-day grid the operator's own page prints — **not** scalar early
//! closes, for the same reason the 2025-2026 module records: the half-day
//! sheet keeps tradeable phases after the trading close (the closing
//! Non-Cancel match and, today, Trade at Close). The 2017-2019 grid is the
//! one that page prints in both surviving replays: Pre-Open 08:30-08:58/59,
//! morning Non-Cancel to 09:00, Trading 09:00-12:30, Pre-Close 12:30-12:34/35
//! and the closing Non-Cancel to the printed `12:36` close. Whether the June
//! 2019 Trade-at-Close launch moved the late-2019 half-day close (as it
//! moved the full-day close to 17:16) is printed by no surviving artifact,
//! so the printed 12:36 grid is held and the question is disclosed. The
//! 2014-2016 sheets print no half-day markers or grid at all, so no half-day
//! row ships for those years and the eves' treatment is recorded as an
//! evidence-file gap rather than guessed.

use super::EvidenceTier::{T1, T2};
use super::HolidayKind::{Closed, ReplacementBlocks};
use super::{HolidayTable, holidays};
use crate::calendar::exceptions::ExceptionBlock;

/// The operator's 2017-2019 half-day trading day, relative to that trade
/// date.
///
/// Read from the `Half Day Trading` column of the operator's own
/// `Trading Hours & Calendar` page as captured 2017-09-27 and 2018-12-23
/// (both replays print the identical grid): Pre-Open 08:30-08:58/59 (order
/// entry; the slice stops at the earliest possible Non-Cancel start, exactly
/// as the normal-week and current half-day profiles do), morning Non-Cancel
/// 08:58/59-09:00, Trading 09:00-12:30, Pre-Close 12:30-12:34/35 (order
/// entry; the slice stops at 12:34), and the closing Non-Cancel to the
/// printed `12:36` close, which is tradeable and ships as one `extended`
/// block.
///
/// Evidence: `docs/evidence/sgx_securities.md`.
#[rustfmt::skip]
static ERA_2017_HALF_DAY_BLOCKS: [ExceptionBlock; 5] = [
    ExceptionBlock::order_entry(0, 8 * 3_600 + 30 * 60, 8 * 3_600 + 58 * 60),
    ExceptionBlock::extended(0, 8 * 3_600 + 58 * 60, 9 * 3_600),
    ExceptionBlock::regular(0, 9 * 3_600, 12 * 3_600 + 30 * 60),
    ExceptionBlock::order_entry(0, 12 * 3_600 + 30 * 60, 12 * 3_600 + 34 * 60),
    ExceptionBlock::extended(0, 12 * 3_600 + 34 * 60, 12 * 3_600 + 36 * 60),
];

/// The operator's half-day trading day, relative to that trade date.
///
/// Read from the half-day column of the operator's own `Trading Schedules`
/// table: Pre-Open 08:30-08:58/08:59 (order entry; the slice stops at the
/// earliest possible Non-Cancel start, exactly as the normal-week profile
/// does), morning Non-Cancel to 09:00, Trading 09:00-12:00, Pre-Close
/// 12:00-12:04/12:05 (order entry), and the closing Non-Cancel plus Trade at
/// Close to the printed `Close: 12:16pm`. The two closing phases are both
/// tradeable and ship as one `extended` block.
///
/// Evidence: `docs/evidence/sgx_securities.md`.
#[rustfmt::skip]
static HALF_DAY_BLOCKS: [ExceptionBlock; 5] = [
    ExceptionBlock::order_entry(0, 8 * 3_600 + 30 * 60, 8 * 3_600 + 58 * 60),
    ExceptionBlock::extended(0, 8 * 3_600 + 58 * 60, 9 * 3_600),
    ExceptionBlock::regular(0, 9 * 3_600, 12 * 3_600),
    ExceptionBlock::order_entry(0, 12 * 3_600, 12 * 3_600 + 4 * 60),
    ExceptionBlock::extended(0, 12 * 3_600 + 4 * 60, 12 * 3_600 + 16 * 60),
];

/// SGX-ST's built-in holiday rows and the windows they were audited over.
///
/// Every date inside a window with no row is audited normal; the spans
/// between the windows (2010-2013 and 2020-01-02..2024-12-31) are the
/// recorded capture gaps and carry no answer. The 2014-2016 rows cite the
/// operator's `Public Holidays` tables as replayed 2014-08-21, 2015-09-24
/// and 2016-01-08; the 2017 rows the 2017-09-27 replay; the 2018-2020 rows
/// the 2018-12-23 replay, whose sheet prints the 2018, 2019 and the one
/// 2020 table; the 2025-2026 rows the operator's current page read as bytes
/// (`SGX-ST-SCHED`).
// Evidence: docs/evidence/sgx_securities.md
pub(crate) static TABLE: &HolidayTable = holidays! {
    coverage: [
        (2014, 1, 1) ..= (2019, 12, 31),
        (2020, 1, 1) ..= (2020, 1, 1),
        (2025, 1, 1) ..= (2026, 12, 31),
    ],
    rows: [
        // 2014-01-01 - T1 - SGX-CAL-2014 - New Year's Day, `Closed`.
        (2014, 1, 1, Closed, T1, "SGX-CAL-2014"),
        // 2014-01-31 - T1 - SGX-CAL-2014 - Chinese New Year, `Closed`.
        (2014, 1, 31, Closed, T1, "SGX-CAL-2014"),
        // 2014-04-18 - T1 - SGX-CAL-2014 - Good Friday, `Closed`.
        (2014, 4, 18, Closed, T1, "SGX-CAL-2014"),
        // 2014-05-01 - T1 - SGX-CAL-2014 - Labour Day, `Closed`.
        (2014, 5, 1, Closed, T1, "SGX-CAL-2014"),
        // 2014-05-13 - T1 - SGX-CAL-2014 - Vesak Day, `Closed`.
        (2014, 5, 13, Closed, T1, "SGX-CAL-2014"),
        // 2014-07-28 - T1 - SGX-CAL-2014 - Hari Raya Puasa, `Closed`.
        (2014, 7, 28, Closed, T1, "SGX-CAL-2014"),
        // 2014-10-06 - T1 - SGX-CAL-2014 - Hari Raya Haji substitution: the
        // sheet's own footnote states `As Hari Raya Haji falls on Sunday 5
        // October 2014, the next day, Monday 6 October 2014, will be a
        // public holiday.`
        (2014, 10, 6, Closed, T1, "SGX-CAL-2014"),
        // 2014-10-22 - T1 - SGX-CAL-2014 - Deepavali, `Closed`: the sheet's
        // own footnote records the Hindu Advisory Board's confirmation of
        // the Wednesday date in place of the tentative Thursday.
        (2014, 10, 22, Closed, T1, "SGX-CAL-2014"),
        // 2014-12-25 - T1 - SGX-CAL-2014 - Christmas Day, `Closed`.
        (2014, 12, 25, Closed, T1, "SGX-CAL-2014"),
        // 2015-01-01 - T1 - SGX-CAL-2015 - New Year's Day, `Closed`.
        (2015, 1, 1, Closed, T1, "SGX-CAL-2015"),
        // 2015-02-19 - T1 - SGX-CAL-2015 - Chinese New Year, `Closed`.
        (2015, 2, 19, Closed, T1, "SGX-CAL-2015"),
        // 2015-02-20 - T1 - SGX-CAL-2015 - Chinese New Year, `Closed`.
        (2015, 2, 20, Closed, T1, "SGX-CAL-2015"),
        // 2015-04-03 - T1 - SGX-CAL-2015 - Good Friday, `Closed`.
        (2015, 4, 3, Closed, T1, "SGX-CAL-2015"),
        // 2015-05-01 - T1 - SGX-CAL-2015 - Labour Day, `Closed`.
        (2015, 5, 1, Closed, T1, "SGX-CAL-2015"),
        // 2015-06-01 - T1 - SGX-CAL-2015 - Vesak Day, `Closed`.
        (2015, 6, 1, Closed, T1, "SGX-CAL-2015"),
        // 2015-07-17 - T1 - SGX-CAL-2015 - Hari Raya Puasa, `Closed`.
        (2015, 7, 17, Closed, T1, "SGX-CAL-2015"),
        // 2015-08-07 - T1 - SGX-CAL-2015 - SG50 Public Holiday, `Closed`.
        (2015, 8, 7, Closed, T1, "SGX-CAL-2015"),
        // 2015-08-10 - T1 - SGX-CAL-2015 - National Day substitution: the
        // sheet marks Sunday 9 August `*` and its legend reads `The
        // following Monday will be a public holiday`.
        (2015, 8, 10, Closed, T1, "SGX-CAL-2015"),
        // 2015-09-11 - T1 - SGX-CAL-2015 - Polling Day, `Closed`.
        (2015, 9, 11, Closed, T1, "SGX-CAL-2015"),
        // 2015-09-24 - T1 - SGX-CAL-2015 - Hari Raya Haji, `Closed`.
        (2015, 9, 24, Closed, T1, "SGX-CAL-2015"),
        // 2015-11-10 - T1 - SGX-CAL-2015 - Deepavali, `Closed` (the sheet's
        // own footnote then marked the date subject to the Hindu Almanac's
        // reconfirmation; the printed date is the one served).
        (2015, 11, 10, Closed, T1, "SGX-CAL-2015"),
        // 2015-12-25 - T1 - SGX-CAL-2015 - Christmas Day, `Closed`.
        (2015, 12, 25, Closed, T1, "SGX-CAL-2015"),
        // 2016-01-01 - T1 - SGX-CAL-2016 - New Year's Day, `Closed`.
        (2016, 1, 1, Closed, T1, "SGX-CAL-2016"),
        // 2016-02-08 - T1 - SGX-CAL-2016 - Chinese New Year, `Closed`.
        (2016, 2, 8, Closed, T1, "SGX-CAL-2016"),
        // 2016-02-09 - T1 - SGX-CAL-2016 - Chinese New Year, `Closed`.
        (2016, 2, 9, Closed, T1, "SGX-CAL-2016"),
        // 2016-03-25 - T1 - SGX-CAL-2016 - Good Friday, `Closed`.
        (2016, 3, 25, Closed, T1, "SGX-CAL-2016"),
        // 2016-05-02 - T1 - SGX-CAL-2016 - Labour Day substitution: the
        // sheet marks Sunday 1 May `*` and its legend reads `The following
        // Monday will be a public holiday`.
        (2016, 5, 2, Closed, T1, "SGX-CAL-2016"),
        // 2016-07-06 - T1 - SGX-CAL-2016 - Hari Raya Puasa, `Closed`.
        (2016, 7, 6, Closed, T1, "SGX-CAL-2016"),
        // 2016-08-09 - T1 - SGX-CAL-2016 - National Day, `Closed`.
        (2016, 8, 9, Closed, T1, "SGX-CAL-2016"),
        // 2016-09-12 - T1 - SGX-CAL-2016 - Hari Raya Haji, `Closed`.
        (2016, 9, 12, Closed, T1, "SGX-CAL-2016"),
        // 2016-12-26 - T1 - SGX-CAL-2016 - Christmas Day substitution: the
        // sheet marks Sunday 25 December `*` and its legend reads `The
        // following Monday will be a public holiday`.
        (2016, 12, 26, Closed, T1, "SGX-CAL-2016"),
        // 2017-01-02 - T1 - SGX-CAL-2017 - New Year's Day substitution: the
        // sheet marks Sunday 1 January `*`, `The following Monday will be a
        // public holiday`.
        (2017, 1, 2, Closed, T1, "SGX-CAL-2017"),
        // 2017-01-27 - T1 - SGX-CAL-2017 - Eve of Chinese New Year: the
        // sheet marks Saturday 28 January `#` and its footnote reads `The
        // preceding Friday, 27-Jan-17, is a half-day trading day`; the row
        // restates the page's own printed half-day grid as one replacement
        // day.
        (2017, 1, 27, ReplacementBlocks(&ERA_2017_HALF_DAY_BLOCKS), T1, "SGX-CAL-2017"),
        // 2017-01-30 - T1 - SGX-CAL-2017 - Chinese New Year substitution:
        // the sheet marks Sunday 29 January `*`, `The following Monday will
        // be a public holiday`.
        (2017, 1, 30, Closed, T1, "SGX-CAL-2017"),
        // 2017-04-14 - T1 - SGX-CAL-2017 - Good Friday, `Closed`.
        (2017, 4, 14, Closed, T1, "SGX-CAL-2017"),
        // 2017-05-01 - T1 - SGX-CAL-2017 - Labour Day, `Closed`.
        (2017, 5, 1, Closed, T1, "SGX-CAL-2017"),
        // 2017-05-10 - T1 - SGX-CAL-2017 - Vesak Day, `Closed`.
        (2017, 5, 10, Closed, T1, "SGX-CAL-2017"),
        // 2017-06-26 - T1 - SGX-CAL-2017 - Hari Raya Puasa substitution:
        // the sheet marks Sunday 25 June `*`, `The following Monday will be
        // a public holiday`.
        (2017, 6, 26, Closed, T1, "SGX-CAL-2017"),
        // 2017-08-09 - T1 - SGX-CAL-2017 - National Day, `Closed`.
        (2017, 8, 9, Closed, T1, "SGX-CAL-2017"),
        // 2017-09-01 - T1 - SGX-CAL-2017 - Hari Raya Haji, `Closed`.
        (2017, 9, 1, Closed, T1, "SGX-CAL-2017"),
        // 2017-10-18 - T1 - SGX-CAL-2017 - Deepavali, `Closed`.
        (2017, 10, 18, Closed, T1, "SGX-CAL-2017"),
        // 2017-12-25 - T1 - SGX-CAL-2017 - Christmas Day, `Closed` (no `#`:
        // the eve was a Sunday, so no half-day row).
        (2017, 12, 25, Closed, T1, "SGX-CAL-2017"),
        // 2018-01-01 - T1 - SGX-CAL-2018-2019 - New Year's Day, `Closed`.
        (2018, 1, 1, Closed, T1, "SGX-CAL-2018-2019"),
        // 2018-02-15 - T1 - SGX-CAL-2018-2019 - Eve of Chinese New Year: the
        // sheet marks Friday 16 February `#`, `The preceding day is a
        // half-day trading day`; the row restates the page's own printed
        // half-day grid.
        (2018, 2, 15, ReplacementBlocks(&ERA_2017_HALF_DAY_BLOCKS), T1, "SGX-CAL-2018-2019"),
        // 2018-02-16 - T1 - SGX-CAL-2018-2019 - Chinese New Year, `Closed`.
        (2018, 2, 16, Closed, T1, "SGX-CAL-2018-2019"),
        // 2018-03-30 - T1 - SGX-CAL-2018-2019 - Good Friday, `Closed`.
        (2018, 3, 30, Closed, T1, "SGX-CAL-2018-2019"),
        // 2018-05-01 - T1 - SGX-CAL-2018-2019 - Labour Day, `Closed`.
        (2018, 5, 1, Closed, T1, "SGX-CAL-2018-2019"),
        // 2018-05-29 - T1 - SGX-CAL-2018-2019 - Vesak Day, `Closed`.
        (2018, 5, 29, Closed, T1, "SGX-CAL-2018-2019"),
        // 2018-06-15 - T1 - SGX-CAL-2018-2019 - Hari Raya Puasa, `Closed`.
        (2018, 6, 15, Closed, T1, "SGX-CAL-2018-2019"),
        // 2018-08-09 - T1 - SGX-CAL-2018-2019 - National Day, `Closed`.
        (2018, 8, 9, Closed, T1, "SGX-CAL-2018-2019"),
        // 2018-08-22 - T1 - SGX-CAL-2018-2019 - Hari Raya Haji, `Closed`.
        (2018, 8, 22, Closed, T1, "SGX-CAL-2018-2019"),
        // 2018-11-06 - T1 - SGX-CAL-2018-2019 - Deepavali, `Closed`.
        (2018, 11, 6, Closed, T1, "SGX-CAL-2018-2019"),
        // 2018-12-24 - T1 - SGX-CAL-2018-2019 - Eve of Christmas: the sheet
        // marks Tuesday 25 December `#`, `The preceding day is a half-day
        // trading day`; the row restates the page's own printed half-day
        // grid.
        (2018, 12, 24, ReplacementBlocks(&ERA_2017_HALF_DAY_BLOCKS), T1, "SGX-CAL-2018-2019"),
        // 2018-12-25 - T1 - SGX-CAL-2018-2019 - Christmas Day, `Closed`.
        (2018, 12, 25, Closed, T1, "SGX-CAL-2018-2019"),
        // 2018-12-31 - T1 - SGX-CAL-2018-2019 - Eve of New Year: the sheet's
        // 2019 table marks Tuesday 1 January 2019 `#`, so the preceding
        // Monday is a half-day trading day; the row restates the page's own
        // printed half-day grid.
        (2018, 12, 31, ReplacementBlocks(&ERA_2017_HALF_DAY_BLOCKS), T1, "SGX-CAL-2018-2019"),
        // 2019-01-01 - T1 - SGX-CAL-2018-2019 - New Year's Day, `Closed`.
        (2019, 1, 1, Closed, T1, "SGX-CAL-2018-2019"),
        // 2019-02-04 - T1 - SGX-CAL-2018-2019 - Eve of Chinese New Year: the
        // sheet marks Tuesday 5 February `#`, `The preceding day is a
        // half-day trading day`; the row restates the page's own printed
        // half-day grid.
        (2019, 2, 4, ReplacementBlocks(&ERA_2017_HALF_DAY_BLOCKS), T1, "SGX-CAL-2018-2019"),
        // 2019-02-05 - T1 - SGX-CAL-2018-2019 - Chinese New Year, `Closed`.
        (2019, 2, 5, Closed, T1, "SGX-CAL-2018-2019"),
        // 2019-02-06 - T1 - SGX-CAL-2018-2019 - Chinese New Year, `Closed`.
        (2019, 2, 6, Closed, T1, "SGX-CAL-2018-2019"),
        // 2019-04-19 - T1 - SGX-CAL-2018-2019 - Good Friday, `Closed`.
        (2019, 4, 19, Closed, T1, "SGX-CAL-2018-2019"),
        // 2019-05-01 - T1 - SGX-CAL-2018-2019 - Labour Day, `Closed`.
        (2019, 5, 1, Closed, T1, "SGX-CAL-2018-2019"),
        // 2019-05-20 - T1 - SGX-CAL-2018-2019 - Vesak Day substitution: the
        // sheet marks Sunday 19 May `*`, `The following Monday will be a
        // public holiday`.
        (2019, 5, 20, Closed, T1, "SGX-CAL-2018-2019"),
        // 2019-06-05 - T1 - SGX-CAL-2018-2019 - Hari Raya Puasa, `Closed`.
        (2019, 6, 5, Closed, T1, "SGX-CAL-2018-2019"),
        // 2019-08-09 - T1 - SGX-CAL-2018-2019 - National Day, `Closed`.
        (2019, 8, 9, Closed, T1, "SGX-CAL-2018-2019"),
        // 2019-08-12 - T1 - SGX-CAL-2018-2019 - Hari Raya Haji substitution:
        // the sheet marks Sunday 11 August `*`, `The following Monday will
        // be a public holiday`.
        (2019, 8, 12, Closed, T1, "SGX-CAL-2018-2019"),
        // 2019-10-28 - T1 - SGX-CAL-2018-2019 - Deepavali substitution: the
        // sheet marks Sunday 27 October `*`, `The following Monday will be a
        // public holiday`.
        (2019, 10, 28, Closed, T1, "SGX-CAL-2018-2019"),
        // 2019-12-24 - T1 - SGX-CAL-2018-2019 - Eve of Christmas: the sheet
        // marks Wednesday 25 December `#`, `The preceding day is a half-day
        // trading day`; the row restates the page's own printed half-day
        // grid (the Trade-at-Close launch's effect on half days is printed
        // by no surviving artifact — see the evidence file).
        (2019, 12, 24, ReplacementBlocks(&ERA_2017_HALF_DAY_BLOCKS), T1, "SGX-CAL-2018-2019"),
        // 2019-12-25 - T1 - SGX-CAL-2018-2019 - Christmas Day, `Closed`.
        (2019, 12, 25, Closed, T1, "SGX-CAL-2018-2019"),
        // 2019-12-31 - T1 - SGX-CAL-2018-2019 - Eve of New Year: the sheet's
        // 2020 table marks Wednesday 1 January 2020 `#`, so the preceding
        // Tuesday is a half-day trading day; the row restates the page's own
        // printed half-day grid (same Trade-at-Close disclosure).
        (2019, 12, 31, ReplacementBlocks(&ERA_2017_HALF_DAY_BLOCKS), T1, "SGX-CAL-2018-2019"),
        // 2020-01-01 - T1 - SGX-CAL-2018-2019 - New Year's Day, `Closed`,
        // the one 2020 date the sheet's 2020 table prints.
        (2020, 1, 1, Closed, T1, "SGX-CAL-2018-2019"),
        // 2025-01-01 - T2 - SGX-ST-SCHED - New Year's Day, gazetted weekday
        // holiday (MOM: Wednesday), SGX-ST closed.
        (2025, 1, 1, Closed, T2, "SGX-ST-SCHED"),
        // 2025-01-28 - T2 - SGX-ST-SCHED - Eve of Chinese New Year, half-day
        // trading, `Close: 12:16pm`.
        (2025, 1, 28, ReplacementBlocks(&HALF_DAY_BLOCKS), T2, "SGX-ST-SCHED"),
        // 2025-01-29 - T2 - SGX-ST-SCHED - Chinese New Year, gazetted weekday
        // holiday (MOM: Wednesday).
        (2025, 1, 29, Closed, T2, "SGX-ST-SCHED"),
        // 2025-01-30 - T2 - SGX-ST-SCHED - Chinese New Year, gazetted weekday
        // holiday (MOM: Thursday).
        (2025, 1, 30, Closed, T2, "SGX-ST-SCHED"),
        // 2025-03-31 - T2 - SGX-ST-SCHED - Hari Raya Puasa, gazetted weekday
        // holiday (MOM: Monday).
        (2025, 3, 31, Closed, T2, "SGX-ST-SCHED"),
        // 2025-04-18 - T2 - SGX-ST-SCHED - Good Friday, gazetted weekday
        // holiday (MOM: Friday).
        (2025, 4, 18, Closed, T2, "SGX-ST-SCHED"),
        // 2025-05-01 - T2 - SGX-ST-SCHED - Labour Day, gazetted weekday
        // holiday (MOM: Thursday).
        (2025, 5, 1, Closed, T2, "SGX-ST-SCHED"),
        // 2025-05-12 - T2 - SGX-ST-SCHED - Vesak Day, gazetted weekday
        // holiday (MOM: Monday).
        (2025, 5, 12, Closed, T2, "SGX-ST-SCHED"),
        // 2025-10-20 - T2 - SGX-ST-SCHED - Deepavali, gazetted weekday
        // holiday (MOM: Monday).
        (2025, 10, 20, Closed, T2, "SGX-ST-SCHED"),
        // 2025-12-24 - T2 - SGX-ST-SCHED - Eve of Christmas, half-day trading,
        // `Close: 12:16pm`.
        (2025, 12, 24, ReplacementBlocks(&HALF_DAY_BLOCKS), T2, "SGX-ST-SCHED"),
        // 2025-12-25 - T2 - SGX-ST-SCHED - Christmas Day, gazetted weekday
        // holiday (MOM: Thursday).
        (2025, 12, 25, Closed, T2, "SGX-ST-SCHED"),
        // 2025-12-31 - T2 - SGX-ST-SCHED - Eve of New Year, half-day trading,
        // `Close: 12:16pm`.
        (2025, 12, 31, ReplacementBlocks(&HALF_DAY_BLOCKS), T2, "SGX-ST-SCHED"),
        // 2026-01-01 - T2 - SGX-ST-SCHED - New Year's Day, gazetted weekday
        // holiday (MOM: Thursday).
        (2026, 1, 1, Closed, T2, "SGX-ST-SCHED"),
        // 2026-02-16 - T2 - SGX-ST-SCHED - Eve of Chinese New Year, half-day
        // trading, `Close: 12:16pm`.
        (2026, 2, 16, ReplacementBlocks(&HALF_DAY_BLOCKS), T2, "SGX-ST-SCHED"),
        // 2026-02-17 - T2 - SGX-ST-SCHED - Chinese New Year, gazetted weekday
        // holiday (MOM: Tuesday).
        (2026, 2, 17, Closed, T2, "SGX-ST-SCHED"),
        // 2026-02-18 - T2 - SGX-ST-SCHED - Chinese New Year, gazetted weekday
        // holiday (MOM: Wednesday).
        (2026, 2, 18, Closed, T2, "SGX-ST-SCHED"),
        // 2026-04-03 - T2 - SGX-ST-SCHED - Good Friday, gazetted weekday
        // holiday (MOM: Friday).
        (2026, 4, 3, Closed, T2, "SGX-ST-SCHED"),
        // 2026-05-01 - T2 - SGX-ST-SCHED - Labour Day, gazetted weekday
        // holiday (MOM: Friday).
        (2026, 5, 1, Closed, T2, "SGX-ST-SCHED"),
        // 2026-05-27 - T2 - SGX-ST-SCHED - Hari Raya Haji, gazetted weekday
        // holiday (MOM: Wednesday).
        (2026, 5, 27, Closed, T2, "SGX-ST-SCHED"),
        // 2026-06-01 - T2 - SGX-ST-SCHED - Vesak Day substitution: MOM prints
        // Sunday 31 May and states `Monday, 1 June 2026, will be a public
        // holiday`.
        (2026, 6, 1, Closed, T2, "SGX-ST-SCHED"),
        // 2026-08-10 - T2 - SGX-ST-SCHED - National Day substitution: MOM
        // prints Sunday 9 August and states `Monday, 10 August 2026, will be a
        // public holiday`.
        (2026, 8, 10, Closed, T2, "SGX-ST-SCHED"),
        // 2026-11-09 - T2 - SGX-ST-SCHED - Deepavali substitution: MOM prints
        // Sunday 8 November and states `Monday, 9 November 2026, will be a
        // public holiday`.
        (2026, 11, 9, Closed, T2, "SGX-ST-SCHED"),
        // 2026-12-24 - T2 - SGX-ST-SCHED - Eve of Christmas, half-day trading,
        // `Close: 12:16pm`.
        (2026, 12, 24, ReplacementBlocks(&HALF_DAY_BLOCKS), T2, "SGX-ST-SCHED"),
        // 2026-12-25 - T2 - SGX-ST-SCHED - Christmas Day, gazetted weekday
        // holiday (MOM: Friday).
        (2026, 12, 25, Closed, T2, "SGX-ST-SCHED"),
        // 2026-12-31 - T2 - SGX-ST-SCHED - Eve of New Year, half-day trading,
        // `Close: 12:16pm`.
        (2026, 12, 31, ReplacementBlocks(&HALF_DAY_BLOCKS), T2, "SGX-ST-SCHED"),
    ],
};
