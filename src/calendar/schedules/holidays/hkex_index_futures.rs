// SPDX-License-Identifier: MIT-0

//! HKEX index-futures holiday, half-day and AHT-suspension data, audited
//! 2010-01-01 through 2027-12-31 plus the 2028-01-01 trade date of the final
//! New Year evening leg. Evidence: `docs/evidence/hkex_index_futures.md`.

use super::fences::{early_close, late_open};
use crate::calendar::ExceptionBlock;

use super::EvidenceTier::T1;
use super::HolidayKind::{Closed, ReplacementBlocks};
use super::{HolidayTable, holidays};

#[rustfmt::skip]
static BLOCKS_01: [ExceptionBlock; 4] = [ExceptionBlock::order_entry(0, 31_500, 33_300), ExceptionBlock::regular(0, 33_300, 43_200), ExceptionBlock::order_entry(0, 45_000, 46_800), ExceptionBlock::regular(0, 46_800, 59_400)];

#[rustfmt::skip]
static BLOCKS_02: [ExceptionBlock; 3] = [ExceptionBlock::regular(-1, 61_200, 10_800), ExceptionBlock::order_entry(0, 31_500, 33_300), ExceptionBlock::regular(0, 33_300, 45_000)];

// Evidence: docs/evidence/hkex_index_futures.md
pub(crate) static TABLE: &HolidayTable = holidays! {
    coverage: [(2010, 1, 1) ..= (2027, 12, 31), (2028, 1, 1) ..= (2028, 1, 1)],
    rows: [
        // 2010-01-01 - T1 - TRADCAL-2010 - operator Holiday (no trading)
        (2010, 1, 1, Closed, T1, "HKEX-TC-2010"),
        // 2010-02-15 - T1 - TRADCAL-2010 - operator Holiday (no trading)
        (2010, 2, 15, Closed, T1, "HKEX-TC-2010"),
        // 2010-02-16 - T1 - TRADCAL-2010 - operator Holiday (no trading)
        (2010, 2, 16, Closed, T1, "HKEX-TC-2010"),
        // 2010-04-02 - T1 - TRADCAL-2010 - operator Holiday (no trading)
        (2010, 4, 2, Closed, T1, "HKEX-TC-2010"),
        // 2010-04-05 - T1 - TRADCAL-2010 - operator Holiday (no trading)
        (2010, 4, 5, Closed, T1, "HKEX-TC-2010"),
        // 2010-04-06 - T1 - TRADCAL-2010 - operator Holiday (no trading)
        (2010, 4, 6, Closed, T1, "HKEX-TC-2010"),
        // 2010-05-21 - T1 - TRADCAL-2010 - operator Holiday (no trading)
        (2010, 5, 21, Closed, T1, "HKEX-TC-2010"),
        // 2010-06-16 - T1 - TRADCAL-2010 - operator Holiday (no trading)
        (2010, 6, 16, Closed, T1, "HKEX-TC-2010"),
        // 2010-07-01 - T1 - TRADCAL-2010 - operator Holiday (no trading)
        (2010, 7, 1, Closed, T1, "HKEX-TC-2010"),
        // 2010-09-23 - T1 - TRADCAL-2010 - operator Holiday (no trading)
        (2010, 9, 23, Closed, T1, "HKEX-TC-2010"),
        // 2010-10-01 - T1 - TRADCAL-2010 - operator Holiday (no trading)
        (2010, 10, 1, Closed, T1, "HKEX-TC-2010"),
        // 2010-12-24 - T1 - TRADCAL-2010 - the half-day eve: no afternoon session (and, from 2013, no AHT); the scalar clip keeps the morning and removes everything after 12:30 p.m.
        (2010, 12, 24, early_close(12 * 3_600), T1, "HKEX-TC-2010"),
        // 2010-12-27 - T1 - TRADCAL-2010 - operator Holiday (no trading)
        (2010, 12, 27, Closed, T1, "HKEX-TC-2010"),
        // 2010-12-31 - T1 - TRADCAL-2010 - the half-day eve: no afternoon session (and, from 2013, no AHT); the scalar clip keeps the morning and removes everything after 12:30 p.m.
        (2010, 12, 31, early_close(12 * 3_600), T1, "HKEX-TC-2010"),
        // 2011-01-03 - T1 - TRADCAL-2011 - operator Holiday (no trading)
        (2011, 1, 3, Closed, T1, "HKEX-TC-2011"),
        // 2011-02-02 - T1 - TRADCAL-2011 - the half-day eve: no afternoon session (and, from 2013, no AHT); the scalar clip keeps the morning and removes everything after 12:30 p.m.
        (2011, 2, 2, early_close(12 * 3_600), T1, "HKEX-TC-2011"),
        // 2011-02-03 - T1 - TRADCAL-2011 - operator Holiday (no trading)
        (2011, 2, 3, Closed, T1, "HKEX-TC-2011"),
        // 2011-02-04 - T1 - TRADCAL-2011 - operator Holiday (no trading)
        (2011, 2, 4, Closed, T1, "HKEX-TC-2011"),
        // 2011-04-05 - T1 - TRADCAL-2011 - operator Holiday (no trading)
        (2011, 4, 5, Closed, T1, "HKEX-TC-2011"),
        // 2011-04-22 - T1 - TRADCAL-2011 - operator Holiday (no trading)
        (2011, 4, 22, Closed, T1, "HKEX-TC-2011"),
        // 2011-04-25 - T1 - TRADCAL-2011 - operator Holiday (no trading)
        (2011, 4, 25, Closed, T1, "HKEX-TC-2011"),
        // 2011-05-02 - T1 - TRADCAL-2011 - operator Holiday (no trading)
        (2011, 5, 2, Closed, T1, "HKEX-TC-2011"),
        // 2011-05-10 - T1 - TRADCAL-2011 - operator Holiday (no trading)
        (2011, 5, 10, Closed, T1, "HKEX-TC-2011"),
        // 2011-06-06 - T1 - TRADCAL-2011 - operator Holiday (no trading)
        (2011, 6, 6, Closed, T1, "HKEX-TC-2011"),
        // 2011-07-01 - T1 - TRADCAL-2011 - operator Holiday (no trading)
        (2011, 7, 1, Closed, T1, "HKEX-TC-2011"),
        // 2011-09-13 - T1 - TRADCAL-2011 - operator Holiday (no trading)
        (2011, 9, 13, Closed, T1, "HKEX-TC-2011"),
        // 2011-10-05 - T1 - TRADCAL-2011 - operator Holiday (no trading)
        (2011, 10, 5, Closed, T1, "HKEX-TC-2011"),
        // 2011-12-26 - T1 - TRADCAL-2011 - operator Holiday (no trading)
        (2011, 12, 26, Closed, T1, "HKEX-TC-2011"),
        // 2011-12-27 - T1 - TRADCAL-2011 - operator Holiday (no trading)
        (2011, 12, 27, Closed, T1, "HKEX-TC-2011"),
        // 2012-01-02 - T1 - TRADCAL-2012 - operator Holiday (no trading)
        (2012, 1, 2, Closed, T1, "HKEX-TC-2012"),
        // 2012-01-23 - T1 - TRADCAL-2012 - operator Holiday (no trading)
        (2012, 1, 23, Closed, T1, "HKEX-TC-2012"),
        // 2012-01-24 - T1 - TRADCAL-2012 - operator Holiday (no trading)
        (2012, 1, 24, Closed, T1, "HKEX-TC-2012"),
        // 2012-01-25 - T1 - TRADCAL-2012 - operator Holiday (no trading)
        (2012, 1, 25, Closed, T1, "HKEX-TC-2012"),
        // 2012-04-04 - T1 - TRADCAL-2012 - operator Holiday (no trading)
        (2012, 4, 4, Closed, T1, "HKEX-TC-2012"),
        // 2012-04-06 - T1 - TRADCAL-2012 - operator Holiday (no trading)
        (2012, 4, 6, Closed, T1, "HKEX-TC-2012"),
        // 2012-04-09 - T1 - TRADCAL-2012 - operator Holiday (no trading)
        (2012, 4, 9, Closed, T1, "HKEX-TC-2012"),
        // 2012-05-01 - T1 - TRADCAL-2012 - operator Holiday (no trading)
        (2012, 5, 1, Closed, T1, "HKEX-TC-2012"),
        // 2012-07-02 - T1 - TRADCAL-2012 - operator Holiday (no trading)
        (2012, 7, 2, Closed, T1, "HKEX-TC-2012"),
        // 2012-10-01 - T1 - TRADCAL-2012 - operator Holiday (no trading)
        (2012, 10, 1, Closed, T1, "HKEX-TC-2012"),
        // 2012-10-02 - T1 - TRADCAL-2012 - operator Holiday (no trading)
        (2012, 10, 2, Closed, T1, "HKEX-TC-2012"),
        // 2012-10-23 - T1 - TRADCAL-2012 - operator Holiday (no trading)
        (2012, 10, 23, Closed, T1, "HKEX-TC-2012"),
        // 2012-12-24 - T1 - TRADCAL-2012 - the half-day eve: no afternoon session (and, from 2013, no AHT); the scalar clip keeps the morning and removes everything after 12:30 p.m.
        (2012, 12, 24, early_close(12 * 3_600), T1, "HKEX-TC-2012"),
        // 2012-12-25 - T1 - TRADCAL-2012 - operator Holiday (no trading)
        (2012, 12, 25, Closed, T1, "HKEX-TC-2012"),
        // 2012-12-26 - T1 - TRADCAL-2012 - operator Holiday (no trading)
        (2012, 12, 26, Closed, T1, "HKEX-TC-2012"),
        // 2012-12-31 - T1 - TRADCAL-2012 - the half-day eve: no afternoon session (and, from 2013, no AHT); the scalar clip keeps the morning and removes everything after 12:30 p.m.
        (2012, 12, 31, early_close(12 * 3_600), T1, "HKEX-TC-2012"),
        // 2013-01-01 - T1 - TRADCAL-2013 - operator Holiday (no trading)
        (2013, 1, 1, Closed, T1, "HKEX-TC-2013"),
        // 2013-02-11 - T1 - TRADCAL-2013 - operator Holiday (no trading)
        (2013, 2, 11, Closed, T1, "HKEX-TC-2013"),
        // 2013-02-12 - T1 - TRADCAL-2013 - operator Holiday (no trading)
        (2013, 2, 12, Closed, T1, "HKEX-TC-2013"),
        // 2013-02-13 - T1 - TRADCAL-2013 - operator Holiday (no trading)
        (2013, 2, 13, Closed, T1, "HKEX-TC-2013"),
        // 2013-03-29 - T1 - TRADCAL-2013 - operator Holiday (no trading)
        (2013, 3, 29, Closed, T1, "HKEX-TC-2013"),
        // 2013-04-01 - T1 - TRADCAL-2013 - operator Holiday (no trading)
        (2013, 4, 1, Closed, T1, "HKEX-TC-2013"),
        // 2013-04-04 - T1 - TRADCAL-2013 - operator Holiday (no trading)
        (2013, 4, 4, Closed, T1, "HKEX-TC-2013"),
        // 2013-05-01 - T1 - TRADCAL-2013 - operator Holiday (no trading)
        (2013, 5, 1, Closed, T1, "HKEX-TC-2013"),
        // 2013-05-17 - T1 - TRADCAL-2013 - operator Holiday (no trading)
        (2013, 5, 17, Closed, T1, "HKEX-TC-2013"),
        // 2013-06-12 - T1 - TRADCAL-2013 - operator Holiday (no trading)
        (2013, 6, 12, Closed, T1, "HKEX-TC-2013"),
        // 2013-07-01 - T1 - TRADCAL-2013 - operator Holiday (no trading)
        (2013, 7, 1, Closed, T1, "HKEX-TC-2013"),
        // 2013-09-20 - T1 - TRADCAL-2013 - operator Holiday (no trading)
        (2013, 9, 20, Closed, T1, "HKEX-TC-2013"),
        // 2013-10-14 - T1 - TRADCAL-2013 - operator Holiday (no trading)
        (2013, 10, 14, Closed, T1, "HKEX-TC-2013"),
        // 2013-12-24 - T1 - TRADCAL-2013 - the half-day eve: no afternoon session (and, from 2013, no AHT); the scalar clip keeps the morning and removes everything after 12:30 p.m.
        (2013, 12, 24, early_close(12 * 3_600), T1, "HKEX-TC-2013"),
        // 2013-12-25 - T1 - TRADCAL-2013 - operator Holiday (no trading)
        (2013, 12, 25, Closed, T1, "HKEX-TC-2013"),
        // 2013-12-26 - T1 - TRADCAL-2013 - operator Holiday (no trading)
        (2013, 12, 26, Closed, T1, "HKEX-TC-2013"),
        // 2013-12-31 - T1 - TRADCAL-2013 - the half-day eve: no afternoon session (and, from 2013, no AHT); the scalar clip keeps the morning and removes everything after 12:30 p.m.
        (2013, 12, 31, early_close(12 * 3_600), T1, "HKEX-TC-2013"),
        // 2014-01-01 - T1 - TRADCAL-2014 - operator Holiday (no trading)
        (2014, 1, 1, Closed, T1, "HKEX-TC-2014"),
        // 2014-01-30 - T1 - TRADCAL-2014 - the half-day eve: no afternoon session (and, from 2013, no AHT); the scalar clip keeps the morning and removes everything after 12:30 p.m.
        (2014, 1, 30, early_close(12 * 3_600), T1, "HKEX-TC-2014"),
        // 2014-01-31 - T1 - TRADCAL-2014 - operator Holiday (no trading)
        (2014, 1, 31, Closed, T1, "HKEX-TC-2014"),
        // 2014-02-03 - T1 - TRADCAL-2014 - operator Holiday (no trading)
        (2014, 2, 3, Closed, T1, "HKEX-TC-2014"),
        // 2014-04-18 - T1 - TRADCAL-2014 - operator Holiday (no trading)
        (2014, 4, 18, Closed, T1, "HKEX-TC-2014"),
        // 2014-04-21 - T1 - TRADCAL-2014 - operator Holiday (no trading)
        (2014, 4, 21, Closed, T1, "HKEX-TC-2014"),
        // 2014-05-01 - T1 - TRADCAL-2014 - operator Holiday (no trading)
        (2014, 5, 1, Closed, T1, "HKEX-TC-2014"),
        // 2014-05-06 - T1 - TRADCAL-2014 - operator Holiday (no trading)
        (2014, 5, 6, Closed, T1, "HKEX-TC-2014"),
        // 2014-06-02 - T1 - TRADCAL-2014 - operator Holiday (no trading)
        (2014, 6, 2, Closed, T1, "HKEX-TC-2014"),
        // 2014-07-01 - T1 - TRADCAL-2014 - operator Holiday (no trading)
        (2014, 7, 1, Closed, T1, "HKEX-TC-2014"),
        // 2014-09-09 - T1 - TRADCAL-2014 - operator Holiday (no trading)
        (2014, 9, 9, Closed, T1, "HKEX-TC-2014"),
        // 2014-10-01 - T1 - TRADCAL-2014 - operator Holiday (no trading)
        (2014, 10, 1, Closed, T1, "HKEX-TC-2014"),
        // 2014-10-02 - T1 - TRADCAL-2014 - operator Holiday (no trading)
        (2014, 10, 2, Closed, T1, "HKEX-TC-2014"),
        // 2014-12-24 - T1 - TRADCAL-2014 - the half-day eve: no afternoon session (and, from 2013, no AHT); the scalar clip keeps the morning and removes everything after 12:30 p.m.
        (2014, 12, 24, early_close(12 * 3_600), T1, "HKEX-TC-2014"),
        // 2014-12-25 - T1 - TRADCAL-2014 - operator Holiday (no trading)
        (2014, 12, 25, Closed, T1, "HKEX-TC-2014"),
        // 2014-12-26 - T1 - TRADCAL-2014 - operator Holiday (no trading)
        (2014, 12, 26, Closed, T1, "HKEX-TC-2014"),
        // 2014-12-31 - T1 - TRADCAL-2014 - the half-day eve: no afternoon session (and, from 2013, no AHT); the scalar clip keeps the morning and removes everything after 12:30 p.m.
        (2014, 12, 31, early_close(12 * 3_600), T1, "HKEX-TC-2014"),
        // 2015-01-01 - T1 - TRADCAL-2015 - operator Holiday (no trading)
        (2015, 1, 1, Closed, T1, "HKEX-TC-2015"),
        // 2015-02-18 - T1 - TRADCAL-2015 - the half-day eve: no afternoon session (and, from 2013, no AHT); the scalar clip keeps the morning and removes everything after 12:30 p.m.
        (2015, 2, 18, early_close(12 * 3_600), T1, "HKEX-TC-2015"),
        // 2015-02-19 - T1 - TRADCAL-2015 - operator Holiday (no trading)
        (2015, 2, 19, Closed, T1, "HKEX-TC-2015"),
        // 2015-02-20 - T1 - TRADCAL-2015 - operator Holiday (no trading)
        (2015, 2, 20, Closed, T1, "HKEX-TC-2015"),
        // 2015-04-03 - T1 - TRADCAL-2015 - operator Holiday (no trading)
        (2015, 4, 3, Closed, T1, "HKEX-TC-2015"),
        // 2015-04-06 - T1 - TRADCAL-2015 - operator Holiday (no trading)
        (2015, 4, 6, Closed, T1, "HKEX-TC-2015"),
        // 2015-04-07 - T1 - TRADCAL-2015 - operator Holiday (no trading)
        (2015, 4, 7, Closed, T1, "HKEX-TC-2015"),
        // 2015-05-01 - T1 - TRADCAL-2015 - operator Holiday (no trading)
        (2015, 5, 1, Closed, T1, "HKEX-TC-2015"),
        // 2015-05-25 - T1 - TRADCAL-2015 - operator Holiday (no trading)
        (2015, 5, 25, Closed, T1, "HKEX-TC-2015"),
        // 2015-07-01 - T1 - TRADCAL-2015 - operator Holiday (no trading)
        (2015, 7, 1, Closed, T1, "HKEX-TC-2015"),
        // 2015-09-28 - T1 - TRADCAL-2015 - operator Holiday (no trading)
        (2015, 9, 28, Closed, T1, "HKEX-TC-2015"),
        // 2015-10-21 - T1 - TRADCAL-2015 - operator Holiday (no trading)
        (2015, 10, 21, Closed, T1, "HKEX-TC-2015"),
        // 2015-12-24 - T1 - TRADCAL-2015 - the half-day eve: no afternoon session (and, from 2013, no AHT); the scalar clip keeps the morning and removes everything after 12:30 p.m.
        (2015, 12, 24, early_close(12 * 3_600), T1, "HKEX-TC-2015"),
        // 2015-12-25 - T1 - TRADCAL-2015 - operator Holiday (no trading)
        (2015, 12, 25, Closed, T1, "HKEX-TC-2015"),
        // 2015-12-31 - T1 - TRADCAL-2015 - the half-day eve: no afternoon session (and, from 2013, no AHT); the scalar clip keeps the morning and removes everything after 12:30 p.m.
        (2015, 12, 31, early_close(12 * 3_600), T1, "HKEX-TC-2015"),
        // 2016-01-01 - T1 - TRADCAL-2016 - operator Holiday (no trading)
        (2016, 1, 1, Closed, T1, "HKEX-TC-2016"),
        // 2016-02-08 - T1 - TRADCAL-2016 - operator Holiday (no trading)
        (2016, 2, 8, Closed, T1, "HKEX-TC-2016"),
        // 2016-02-09 - T1 - TRADCAL-2016 - operator Holiday (no trading)
        (2016, 2, 9, Closed, T1, "HKEX-TC-2016"),
        // 2016-02-10 - T1 - TRADCAL-2016 - operator Holiday (no trading)
        (2016, 2, 10, Closed, T1, "HKEX-TC-2016"),
        // 2016-03-25 - T1 - TRADCAL-2016 - operator Holiday (no trading)
        (2016, 3, 25, Closed, T1, "HKEX-TC-2016"),
        // 2016-03-28 - T1 - TRADCAL-2016 - operator Holiday (no trading)
        (2016, 3, 28, Closed, T1, "HKEX-TC-2016"),
        // 2016-04-04 - T1 - TRADCAL-2016 - operator Holiday (no trading)
        (2016, 4, 4, Closed, T1, "HKEX-TC-2016"),
        // 2016-05-02 - T1 - TRADCAL-2016 - operator Holiday (no trading)
        (2016, 5, 2, Closed, T1, "HKEX-TC-2016"),
        // 2016-06-09 - T1 - TRADCAL-2016 - operator Holiday (no trading)
        (2016, 6, 9, Closed, T1, "HKEX-TC-2016"),
        // 2016-07-01 - T1 - TRADCAL-2016 - operator Holiday (no trading)
        (2016, 7, 1, Closed, T1, "HKEX-TC-2016"),
        // 2016-09-16 - T1 - TRADCAL-2016 - operator Holiday (no trading)
        (2016, 9, 16, Closed, T1, "HKEX-TC-2016"),
        // 2016-10-10 - T1 - TRADCAL-2016 - operator Holiday (no trading)
        (2016, 10, 10, Closed, T1, "HKEX-TC-2016"),
        // 2016-12-26 - T1 - TRADCAL-2016 - operator Holiday (no trading)
        (2016, 12, 26, Closed, T1, "HKEX-TC-2016"),
        // 2016-12-27 - T1 - TRADCAL-2016 - operator Holiday (no trading)
        (2016, 12, 27, Closed, T1, "HKEX-TC-2016"),
        // 2017-01-02 - T1 - TRADCAL-2017 - operator Holiday (no trading)
        (2017, 1, 2, Closed, T1, "HKEX-TC-2017"),
        // 2017-01-27 - T1 - TRADCAL-2017 - the half-day eve: no afternoon session (and, from 2013, no AHT); the scalar clip keeps the morning and removes everything after 12:30 p.m.
        (2017, 1, 27, early_close(12 * 3_600), T1, "HKEX-TC-2017"),
        // 2017-01-30 - T1 - TRADCAL-2017 - operator Holiday (no trading)
        (2017, 1, 30, Closed, T1, "HKEX-TC-2017"),
        // 2017-01-31 - T1 - TRADCAL-2017 - operator Holiday (no trading)
        (2017, 1, 31, Closed, T1, "HKEX-TC-2017"),
        // 2017-04-04 - T1 - TRADCAL-2017 - operator Holiday (no trading)
        (2017, 4, 4, Closed, T1, "HKEX-TC-2017"),
        // 2017-04-14 - T1 - TRADCAL-2017 - operator Holiday (no trading)
        (2017, 4, 14, Closed, T1, "HKEX-TC-2017"),
        // 2017-04-17 - T1 - TRADCAL-2017 - operator Holiday (no trading)
        (2017, 4, 17, Closed, T1, "HKEX-TC-2017"),
        // 2017-05-01 - T1 - TRADCAL-2017 - operator Holiday (no trading)
        (2017, 5, 1, Closed, T1, "HKEX-TC-2017"),
        // 2017-05-03 - T1 - TRADCAL-2017 - operator Holiday (no trading)
        (2017, 5, 3, Closed, T1, "HKEX-TC-2017"),
        // 2017-05-29 - T1 - TRADCAL-2017 - bank holiday in both the UK and the United States: the page's note suspends that day's AHT session while the day sessions run
        (2017, 5, 29, ReplacementBlocks(&BLOCKS_01), T1, "HKEX-TC-2017"),
        // 2017-05-30 - T1 - TRADCAL-2017 - operator Holiday (no trading)
        (2017, 5, 30, Closed, T1, "HKEX-TC-2017"),
        // 2017-10-02 - T1 - TRADCAL-2017 - operator Holiday (no trading)
        (2017, 10, 2, Closed, T1, "HKEX-TC-2017"),
        // 2017-10-05 - T1 - TRADCAL-2017 - operator Holiday (no trading)
        (2017, 10, 5, Closed, T1, "HKEX-TC-2017"),
        // 2017-12-25 - T1 - TRADCAL-2017 - operator Holiday (no trading)
        (2017, 12, 25, Closed, T1, "HKEX-TC-2017"),
        // 2017-12-26 - T1 - TRADCAL-2017 - operator Holiday (no trading)
        (2017, 12, 26, Closed, T1, "HKEX-TC-2017"),
        // 2018-01-01 - T1 - HKEX-HOL-2018 - operator Holiday (no trading)
        (2018, 1, 1, Closed, T1, "HKEX-HOL-2018"),
        // 2018-02-15 - T1 - HKEX-HOL-2018 - the half-day eve: no afternoon session (and, from 2013, no AHT); the scalar clip keeps the morning and removes everything after 12:30 p.m.
        (2018, 2, 15, early_close(12 * 3_600), T1, "HKEX-HOL-2018"),
        // 2018-02-16 - T1 - HKEX-HOL-2018 - operator Holiday (no trading)
        (2018, 2, 16, Closed, T1, "HKEX-HOL-2018"),
        // 2018-02-19 - T1 - HKEX-HOL-2018 - operator Holiday (no trading)
        (2018, 2, 19, Closed, T1, "HKEX-HOL-2018"),
        // 2018-03-30 - T1 - HKEX-HOL-2018 - operator Holiday (no trading)
        (2018, 3, 30, Closed, T1, "HKEX-HOL-2018"),
        // 2018-04-02 - T1 - HKEX-HOL-2018 - operator Holiday (no trading)
        (2018, 4, 2, Closed, T1, "HKEX-HOL-2018"),
        // 2018-04-05 - T1 - HKEX-HOL-2018 - operator Holiday (no trading)
        (2018, 4, 5, Closed, T1, "HKEX-HOL-2018"),
        // 2018-05-01 - T1 - HKEX-HOL-2018 - operator Holiday (no trading)
        (2018, 5, 1, Closed, T1, "HKEX-HOL-2018"),
        // 2018-05-22 - T1 - HKEX-HOL-2018 - operator Holiday (no trading)
        (2018, 5, 22, Closed, T1, "HKEX-HOL-2018"),
        // 2018-05-28 - T1 - HKEX-HOL-2018 - bank holiday in both the UK and the United States: the page's note suspends that day's AHT session while the day sessions run
        (2018, 5, 28, ReplacementBlocks(&BLOCKS_01), T1, "HKEX-HOL-2018"),
        // 2018-06-18 - T1 - HKEX-HOL-2018 - operator Holiday (no trading)
        (2018, 6, 18, Closed, T1, "HKEX-HOL-2018"),
        // 2018-09-25 - T1 - HKEX-HOL-2018 - operator Holiday (no trading)
        (2018, 9, 25, Closed, T1, "HKEX-HOL-2018"),
        // 2018-10-01 - T1 - HKEX-HOL-2018 - operator Holiday (no trading)
        (2018, 10, 1, Closed, T1, "HKEX-HOL-2018"),
        // 2018-10-17 - T1 - HKEX-HOL-2018 - operator Holiday (no trading)
        (2018, 10, 17, Closed, T1, "HKEX-HOL-2018"),
        // 2018-12-24 - T1 - HKEX-HOL-2018 - the half-day eve: no afternoon session (and, from 2013, no AHT); the scalar clip keeps the morning and removes everything after 12:30 p.m.
        (2018, 12, 24, early_close(12 * 3_600), T1, "HKEX-HOL-2018"),
        // 2018-12-25 - T1 - HKEX-HOL-2018 - operator Holiday (no trading)
        (2018, 12, 25, Closed, T1, "HKEX-HOL-2018"),
        // 2018-12-26 - T1 - HKEX-HOL-2018 - operator Holiday (no trading)
        (2018, 12, 26, Closed, T1, "HKEX-HOL-2018"),
        // 2018-12-31 - T1 - HKEX-HOL-2018 - the half-day eve: no afternoon session (and, from 2013, no AHT); the scalar clip keeps the morning and removes everything after 12:30 p.m.
        (2018, 12, 31, early_close(12 * 3_600), T1, "HKEX-HOL-2018"),
        // 2019-01-01 - T1 - HKEX-HOL-2019 - operator Holiday (no trading)
        (2019, 1, 1, Closed, T1, "HKEX-TC-PAGE-2019"),
        // 2019-02-04 - T1 - HKEX-HOL-2019 - the half-day eve: no afternoon session (and, from 2013, no AHT); the scalar clip keeps the morning and removes everything after 12:30 p.m.
        (2019, 2, 4, early_close(12 * 3_600), T1, "HKEX-TC-PAGE-2019"),
        // 2019-02-05 - T1 - HKEX-HOL-2019 - operator Holiday (no trading)
        (2019, 2, 5, Closed, T1, "HKEX-TC-PAGE-2019"),
        // 2019-02-06 - T1 - HKEX-HOL-2019 - operator Holiday (no trading)
        (2019, 2, 6, Closed, T1, "HKEX-TC-PAGE-2019"),
        // 2019-02-07 - T1 - HKEX-HOL-2019 - operator Holiday (no trading)
        (2019, 2, 7, Closed, T1, "HKEX-TC-PAGE-2019"),
        // 2019-04-05 - T1 - HKEX-HOL-2019 - operator Holiday (no trading)
        (2019, 4, 5, Closed, T1, "HKEX-TC-PAGE-2019"),
        // 2019-04-19 - T1 - HKEX-HOL-2019 - operator Holiday (no trading)
        (2019, 4, 19, Closed, T1, "HKEX-TC-PAGE-2019"),
        // 2019-04-22 - T1 - HKEX-HOL-2019 - operator Holiday (no trading)
        (2019, 4, 22, Closed, T1, "HKEX-TC-PAGE-2019"),
        // 2019-05-01 - T1 - HKEX-HOL-2019 - operator Holiday (no trading)
        (2019, 5, 1, Closed, T1, "HKEX-TC-PAGE-2019"),
        // 2019-05-13 - T1 - HKEX-HOL-2019 - operator Holiday (no trading)
        (2019, 5, 13, Closed, T1, "HKEX-TC-PAGE-2019"),
        // 2019-05-27 - T1 - HKEX-HOL-2019 - bank holiday in both the UK and the United States: the page's note suspends that day's AHT session while the day sessions run
        (2019, 5, 27, ReplacementBlocks(&BLOCKS_01), T1, "HKEX-TC-PAGE-2019"),
        // 2019-06-07 - T1 - HKEX-HOL-2019 - operator Holiday (no trading)
        (2019, 6, 7, Closed, T1, "HKEX-TC-PAGE-2019"),
        // 2019-07-01 - T1 - HKEX-HOL-2019 - operator Holiday (no trading)
        (2019, 7, 1, Closed, T1, "HKEX-TC-PAGE-2019"),
        // 2019-10-01 - T1 - HKEX-HOL-2019 - operator Holiday (no trading)
        (2019, 10, 1, Closed, T1, "HKEX-TC-PAGE-2019"),
        // 2019-10-07 - T1 - HKEX-HOL-2019 - operator Holiday (no trading)
        (2019, 10, 7, Closed, T1, "HKEX-TC-PAGE-2019"),
        // 2019-12-24 - T1 - HKEX-HOL-2019 - the half-day eve: no afternoon session (and, from 2013, no AHT); the scalar clip keeps the morning and removes everything after 12:30 p.m.
        (2019, 12, 24, early_close(12 * 3_600), T1, "HKEX-TC-PAGE-2019"),
        // 2019-12-25 - T1 - HKEX-HOL-2019 - operator Holiday (no trading)
        (2019, 12, 25, Closed, T1, "HKEX-TC-PAGE-2019"),
        // 2019-12-26 - T1 - HKEX-HOL-2019 - operator Holiday (no trading)
        (2019, 12, 26, Closed, T1, "HKEX-TC-PAGE-2019"),
        // 2019-12-31 - T1 - HKEX-HOL-2019 - the half-day eve: no afternoon session (and, from 2013, no AHT); the scalar clip keeps the morning and removes everything after 12:30 p.m.
        (2019, 12, 31, early_close(12 * 3_600), T1, "HKEX-TC-PAGE-2019"),
        // 2020-01-01 - T1 - HKEX-HOL-2020 - operator Holiday (no trading)
        (2020, 1, 1, Closed, T1, "HKEX-HOL-2020"),
        // 2020-01-24 - T1 - HKEX-HOL-2020 - the half-day eve: no afternoon session (and, from 2013, no AHT); the scalar clip keeps the morning and removes everything after 12:30 p.m.
        (2020, 1, 24, early_close(12 * 3_600), T1, "HKEX-HOL-2020"),
        // 2020-01-27 - T1 - HKEX-HOL-2020 - operator Holiday (no trading)
        (2020, 1, 27, Closed, T1, "HKEX-HOL-2020"),
        // 2020-01-28 - T1 - HKEX-HOL-2020 - operator Holiday (no trading)
        (2020, 1, 28, Closed, T1, "HKEX-HOL-2020"),
        // 2020-04-10 - T1 - HKEX-HOL-2020 - operator Holiday (no trading)
        (2020, 4, 10, Closed, T1, "HKEX-HOL-2020"),
        // 2020-04-13 - T1 - HKEX-HOL-2020 - operator Holiday (no trading)
        (2020, 4, 13, Closed, T1, "HKEX-HOL-2020"),
        // 2020-04-30 - T1 - HKEX-HOL-2020 - operator Holiday (no trading)
        (2020, 4, 30, Closed, T1, "HKEX-HOL-2020"),
        // 2020-05-01 - T1 - HKEX-HOL-2020 - operator Holiday (no trading)
        (2020, 5, 1, Closed, T1, "HKEX-HOL-2020"),
        // 2020-05-25 - T1 - HKEX-HOL-2020 - bank holiday in both the UK and the United States: the page's note suspends that day's AHT session while the day sessions run
        (2020, 5, 25, ReplacementBlocks(&BLOCKS_01), T1, "HKEX-HOL-2020"),
        // 2020-06-25 - T1 - HKEX-HOL-2020 - operator Holiday (no trading)
        (2020, 6, 25, Closed, T1, "HKEX-HOL-2020"),
        // 2020-07-01 - T1 - HKEX-HOL-2020 - operator Holiday (no trading)
        (2020, 7, 1, Closed, T1, "HKEX-HOL-2020"),
        // 2020-10-01 - T1 - HKEX-HOL-2020 - operator Holiday (no trading)
        (2020, 10, 1, Closed, T1, "HKEX-HOL-2020"),
        // 2020-10-02 - T1 - HKEX-HOL-2020 - operator Holiday (no trading)
        (2020, 10, 2, Closed, T1, "HKEX-HOL-2020"),
        // 2020-10-26 - T1 - HKEX-HOL-2020 - operator Holiday (no trading)
        (2020, 10, 26, Closed, T1, "HKEX-HOL-2020"),
        // 2020-12-24 - T1 - HKEX-HOL-2020 - the half-day eve: no afternoon session (and, from 2013, no AHT); the scalar clip keeps the morning and removes everything after 12:30 p.m.
        (2020, 12, 24, early_close(12 * 3_600), T1, "HKEX-HOL-2020"),
        // 2020-12-25 - T1 - HKEX-HOL-2020 - operator Holiday (no trading)
        (2020, 12, 25, Closed, T1, "HKEX-HOL-2020"),
        // 2020-12-31 - T1 - HKEX-HOL-2020 - the half-day eve: no afternoon session (and, from 2013, no AHT); the scalar clip keeps the morning and removes everything after 12:30 p.m.
        (2020, 12, 31, early_close(12 * 3_600), T1, "HKEX-HOL-2020"),
        // 2021-01-01 - T1 - HKEX-HOL-2021 - operator Holiday (no trading)
        (2021, 1, 1, Closed, T1, "HKEX-TC-PAGE-2020"),
        // 2021-02-11 - T1 - HKEX-HOL-2021 - the half-day eve: no afternoon session (and, from 2013, no AHT); the scalar clip keeps the morning and removes everything after 12:30 p.m.
        (2021, 2, 11, early_close(12 * 3_600), T1, "HKEX-TC-PAGE-2020"),
        // 2021-02-12 - T1 - HKEX-HOL-2021 - operator Holiday (no trading)
        (2021, 2, 12, Closed, T1, "HKEX-TC-PAGE-2020"),
        // 2021-02-15 - T1 - HKEX-HOL-2021 - operator Holiday (no trading)
        (2021, 2, 15, Closed, T1, "HKEX-TC-PAGE-2020"),
        // 2021-04-02 - T1 - HKEX-HOL-2021 - operator Holiday (no trading)
        (2021, 4, 2, Closed, T1, "HKEX-TC-PAGE-2020"),
        // 2021-04-05 - T1 - HKEX-HOL-2021 - operator Holiday (no trading)
        (2021, 4, 5, Closed, T1, "HKEX-TC-PAGE-2020"),
        // 2021-04-06 - T1 - HKEX-HOL-2021 - operator Holiday (no trading)
        (2021, 4, 6, Closed, T1, "HKEX-TC-PAGE-2020"),
        // 2021-05-19 - T1 - HKEX-HOL-2021 - operator Holiday (no trading)
        (2021, 5, 19, Closed, T1, "HKEX-TC-PAGE-2020"),
        // 2021-05-31 - T1 - HKEX-HOL-2021 - bank holiday in both the UK and the United States: the page's note suspends that day's AHT session while the day sessions run
        (2021, 5, 31, ReplacementBlocks(&BLOCKS_01), T1, "HKEX-TC-PAGE-2020"),
        // 2021-06-14 - T1 - HKEX-HOL-2021 - operator Holiday (no trading)
        (2021, 6, 14, Closed, T1, "HKEX-TC-PAGE-2020"),
        // 2021-07-01 - T1 - HKEX-HOL-2021 - operator Holiday (no trading)
        (2021, 7, 1, Closed, T1, "HKEX-TC-PAGE-2020"),
        // 2021-09-22 - T1 - HKEX-HOL-2021 - operator Holiday (no trading)
        (2021, 9, 22, Closed, T1, "HKEX-TC-PAGE-2020"),
        // 2021-10-01 - T1 - HKEX-HOL-2021 - operator Holiday (no trading)
        (2021, 10, 1, Closed, T1, "HKEX-TC-PAGE-2020"),
        // 2021-10-14 - T1 - HKEX-HOL-2021 - operator Holiday (no trading)
        (2021, 10, 14, Closed, T1, "HKEX-TC-PAGE-2020"),
        // 2021-12-24 - T1 - HKEX-HOL-2021 - the half-day eve: no afternoon session (and, from 2013, no AHT); the scalar clip keeps the morning and removes everything after 12:30 p.m.
        (2021, 12, 24, early_close(12 * 3_600), T1, "HKEX-TC-PAGE-2020"),
        // 2021-12-27 - T1 - HKEX-HOL-2021 - operator Holiday (no trading)
        (2021, 12, 27, Closed, T1, "HKEX-TC-PAGE-2020"),
        // 2021-12-31 - T1 - HKEX-HOL-2021 - the half-day eve: no afternoon session (and, from 2013, no AHT); the scalar clip keeps the morning and removes everything after 12:30 p.m.
        (2021, 12, 31, early_close(12 * 3_600), T1, "HKEX-TC-PAGE-2020"),
        // 2022-01-31 - T1 - HKEX-HOL-2022 - the half-day eve: no afternoon session (and, from 2013, no AHT); the scalar clip keeps the morning and removes everything after 12:30 p.m.
        (2022, 1, 31, early_close(12 * 3_600), T1, "HKEX-HOL-2022"),
        // 2022-02-01 - T1 - HKEX-HOL-2022 - operator Holiday (no trading)
        (2022, 2, 1, Closed, T1, "HKEX-HOL-2022"),
        // 2022-02-02 - T1 - HKEX-HOL-2022 - operator Holiday (no trading)
        (2022, 2, 2, Closed, T1, "HKEX-HOL-2022"),
        // 2022-02-03 - T1 - HKEX-HOL-2022 - operator Holiday (no trading)
        (2022, 2, 3, Closed, T1, "HKEX-HOL-2022"),
        // 2022-04-05 - T1 - HKEX-HOL-2022 - operator Holiday (no trading)
        (2022, 4, 5, Closed, T1, "HKEX-HOL-2022"),
        // 2022-04-15 - T1 - HKEX-HOL-2022 - operator Holiday (no trading)
        (2022, 4, 15, Closed, T1, "HKEX-HOL-2022"),
        // 2022-04-18 - T1 - HKEX-HOL-2022 - operator Holiday (no trading)
        (2022, 4, 18, Closed, T1, "HKEX-HOL-2022"),
        // 2022-05-02 - T1 - HKEX-HOL-2022 - operator Holiday (no trading)
        (2022, 5, 2, Closed, T1, "HKEX-HOL-2022"),
        // 2022-05-09 - T1 - HKEX-HOL-2022 - operator Holiday (no trading)
        (2022, 5, 9, Closed, T1, "HKEX-HOL-2022"),
        // 2022-06-03 - T1 - HKEX-HOL-2022 - operator Holiday (no trading)
        (2022, 6, 3, Closed, T1, "HKEX-HOL-2022"),
        // 2022-07-01 - T1 - HKEX-HOL-2022 - operator Holiday (no trading)
        (2022, 7, 1, Closed, T1, "HKEX-HOL-2022"),
        // 2022-09-12 - T1 - HKEX-HOL-2022 - operator Holiday (no trading)
        (2022, 9, 12, Closed, T1, "HKEX-HOL-2022"),
        // 2022-10-04 - T1 - HKEX-HOL-2022 - operator Holiday (no trading)
        (2022, 10, 4, Closed, T1, "HKEX-HOL-2022"),
        // 2022-12-26 - T1 - HKEX-HOL-2022 - operator Holiday (no trading)
        (2022, 12, 26, Closed, T1, "HKEX-HOL-2022"),
        // 2022-12-27 - T1 - HKEX-HOL-2022 - operator Holiday (no trading)
        (2022, 12, 27, Closed, T1, "HKEX-HOL-2022"),
        // 2023-01-02 - T1 - HKEX-HOL-2023 - operator Holiday (no trading)
        (2023, 1, 2, Closed, T1, "HKEX-HOL-2023"),
        // 2023-01-23 - T1 - HKEX-HOL-2023 - operator Holiday (no trading)
        (2023, 1, 23, Closed, T1, "HKEX-HOL-2023"),
        // 2023-01-24 - T1 - HKEX-HOL-2023 - operator Holiday (no trading)
        (2023, 1, 24, Closed, T1, "HKEX-HOL-2023"),
        // 2023-01-25 - T1 - HKEX-HOL-2023 - operator Holiday (no trading)
        (2023, 1, 25, Closed, T1, "HKEX-HOL-2023"),
        // 2023-04-05 - T1 - HKEX-HOL-2023 - operator Holiday (no trading)
        (2023, 4, 5, Closed, T1, "HKEX-HOL-2023"),
        // 2023-04-07 - T1 - HKEX-HOL-2023 - operator Holiday (no trading)
        (2023, 4, 7, Closed, T1, "HKEX-HOL-2023"),
        // 2023-04-10 - T1 - HKEX-HOL-2023 - operator Holiday (no trading)
        (2023, 4, 10, Closed, T1, "HKEX-HOL-2023"),
        // 2023-05-01 - T1 - HKEX-HOL-2023 - operator Holiday (no trading)
        (2023, 5, 1, Closed, T1, "HKEX-HOL-2023"),
        // 2023-05-26 - T1 - HKEX-HOL-2023 - operator Holiday (no trading)
        (2023, 5, 26, Closed, T1, "HKEX-HOL-2023"),
        // 2023-05-29 - T1 - HKEX-HOL-2023 - bank holiday in both the UK and the United States: the page's note suspends that day's AHT session while the day sessions run
        (2023, 5, 29, ReplacementBlocks(&BLOCKS_01), T1, "HKEX-HOL-2023"),
        // 2023-06-22 - T1 - HKEX-HOL-2023 - operator Holiday (no trading)
        (2023, 6, 22, Closed, T1, "HKEX-HOL-2023"),
        // 2023-10-02 - T1 - HKEX-HOL-2023 - operator Holiday (no trading)
        (2023, 10, 2, Closed, T1, "HKEX-HOL-2023"),
        // 2023-10-23 - T1 - HKEX-HOL-2023 - operator Holiday (no trading)
        (2023, 10, 23, Closed, T1, "HKEX-HOL-2023"),
        // 2023-12-25 - T1 - HKEX-HOL-2023 - operator Holiday (no trading)
        (2023, 12, 25, Closed, T1, "HKEX-HOL-2023"),
        // 2023-12-26 - T1 - HKEX-HOL-2023 - operator Holiday (no trading)
        (2023, 12, 26, Closed, T1, "HKEX-HOL-2023"),
        // 2024-01-01 - T1 - HKEX-HOL-2024 - operator Holiday (no trading)
        (2024, 1, 1, Closed, T1, "HKEX-HOL-2024"),
        // 2024-02-09 - T1 - HKEX-HOL-2024 - the half-day eve: no afternoon session (and, from 2013, no AHT); the scalar clip keeps the morning and removes everything after 12:30 p.m.
        (2024, 2, 9, early_close(12 * 3_600), T1, "HKEX-HOL-2024"),
        // 2024-02-12 - T1 - HKEX-HOL-2024 - operator Holiday (no trading)
        (2024, 2, 12, Closed, T1, "HKEX-HOL-2024"),
        // 2024-02-13 - T1 - HKEX-HOL-2024 - operator Holiday (no trading)
        (2024, 2, 13, Closed, T1, "HKEX-HOL-2024"),
        // 2024-03-29 - T1 - HKEX-HOL-2024 - operator Holiday (no trading)
        (2024, 3, 29, Closed, T1, "HKEX-HOL-2024"),
        // 2024-04-01 - T1 - HKEX-HOL-2024 - operator Holiday (no trading)
        (2024, 4, 1, Closed, T1, "HKEX-HOL-2024"),
        // 2024-04-04 - T1 - HKEX-HOL-2024 - operator Holiday (no trading)
        (2024, 4, 4, Closed, T1, "HKEX-HOL-2024"),
        // 2024-05-01 - T1 - HKEX-HOL-2024 - operator Holiday (no trading)
        (2024, 5, 1, Closed, T1, "HKEX-HOL-2024"),
        // 2024-05-15 - T1 - HKEX-HOL-2024 - operator Holiday (no trading)
        (2024, 5, 15, Closed, T1, "HKEX-HOL-2024"),
        // 2024-05-27 - T1 - HKEX-HOL-2024 - bank holiday in both the UK and the United States: the page's note suspends that day's AHT session while the day sessions run
        (2024, 5, 27, ReplacementBlocks(&BLOCKS_01), T1, "HKEX-HOL-2024"),
        // 2024-06-10 - T1 - HKEX-HOL-2024 - operator Holiday (no trading)
        (2024, 6, 10, Closed, T1, "HKEX-HOL-2024"),
        // 2024-07-01 - T1 - HKEX-HOL-2024 - operator Holiday (no trading)
        (2024, 7, 1, Closed, T1, "HKEX-HOL-2024"),
        // 2024-09-18 - T1 - HKEX-HOL-2024 - operator Holiday (no trading)
        (2024, 9, 18, Closed, T1, "HKEX-HOL-2024"),
        // 2024-10-01 - T1 - HKEX-HOL-2024 - operator Holiday (no trading)
        (2024, 10, 1, Closed, T1, "HKEX-HOL-2024"),
        // 2024-10-11 - T1 - HKEX-HOL-2024 - operator Holiday (no trading)
        (2024, 10, 11, Closed, T1, "HKEX-HOL-2024"),
        // 2024-12-24 - T1 - HKEX-HOL-2024 - the half-day eve: no afternoon session (and, from 2013, no AHT); the scalar clip keeps the morning and removes everything after 12:30 p.m.
        (2024, 12, 24, early_close(12 * 3_600), T1, "HKEX-HOL-2024"),
        // 2024-12-25 - T1 - HKEX-HOL-2024 - operator Holiday (no trading)
        (2024, 12, 25, Closed, T1, "HKEX-HOL-2024"),
        // 2024-12-26 - T1 - HKEX-HOL-2024 - operator Holiday (no trading)
        (2024, 12, 26, Closed, T1, "HKEX-HOL-2024"),
        // 2024-12-31 - T1 - HKEX-HOL-2024 - the half-day eve: no afternoon session (and, from 2013, no AHT); the scalar clip keeps the morning and removes everything after 12:30 p.m.
        (2024, 12, 31, early_close(12 * 3_600), T1, "HKEX-HOL-2024"),
        // 2025-01-01 - T1 - HKEX-HOL-2025 - operator Holiday (no trading)
        (2025, 1, 1, Closed, T1, "HKEX-HOL-2025"),
        // 2025-01-28 - T1 - HKEX-HOL-2025 - the half-day eve: no afternoon session (and, from 2013, no AHT); the scalar clip keeps the morning and removes everything after 12:30 p.m.
        (2025, 1, 28, early_close(12 * 3_600), T1, "HKEX-HOL-2025"),
        // 2025-01-29 - T1 - HKEX-HOL-2025 - operator Holiday (no trading)
        (2025, 1, 29, Closed, T1, "HKEX-HOL-2025"),
        // 2025-01-30 - T1 - HKEX-HOL-2025 - operator Holiday (no trading)
        (2025, 1, 30, Closed, T1, "HKEX-HOL-2025"),
        // 2025-01-31 - T1 - HKEX-HOL-2025 - operator Holiday (no trading)
        (2025, 1, 31, Closed, T1, "HKEX-HOL-2025"),
        // 2025-04-04 - T1 - HKEX-HOL-2025 - operator Holiday (no trading)
        (2025, 4, 4, Closed, T1, "HKEX-HOL-2025"),
        // 2025-04-18 - T1 - HKEX-HOL-2025 - operator Holiday (no trading)
        (2025, 4, 18, Closed, T1, "HKEX-HOL-2025"),
        // 2025-04-21 - T1 - HKEX-HOL-2025 - operator Holiday (no trading)
        (2025, 4, 21, Closed, T1, "HKEX-HOL-2025"),
        // 2025-05-01 - T1 - HKEX-HOL-2025 - operator Holiday (no trading)
        (2025, 5, 1, Closed, T1, "HKEX-HOL-2025"),
        // 2025-05-05 - T1 - HKEX-HOL-2025 - operator Holiday (no trading)
        (2025, 5, 5, Closed, T1, "HKEX-HOL-2025"),
        // 2025-05-26 - T1 - HKEX-HOL-2025 - bank holiday in both the UK and the United States: the page's note suspends that day's AHT session while the day sessions run
        (2025, 5, 26, ReplacementBlocks(&BLOCKS_01), T1, "HKEX-HOL-2025"),
        // 2025-07-01 - T1 - HKEX-HOL-2025 - operator Holiday (no trading)
        (2025, 7, 1, Closed, T1, "HKEX-HOL-2025"),
        // 2025-10-01 - T1 - HKEX-HOL-2025 - operator Holiday (no trading)
        (2025, 10, 1, Closed, T1, "HKEX-HOL-2025"),
        // 2025-10-07 - T1 - HKEX-HOL-2025 - operator Holiday (no trading)
        (2025, 10, 7, Closed, T1, "HKEX-HOL-2025"),
        // 2025-10-29 - T1 - HKEX-HOL-2025 - operator Holiday (no trading)
        (2025, 10, 29, Closed, T1, "HKEX-HOL-2025"),
        // 2025-12-24 - T1 - HKEX-HOL-2025 - the half-day eve: no afternoon session (and, from 2013, no AHT); the scalar clip keeps the morning and removes everything after 12:30 p.m.
        (2025, 12, 24, early_close(12 * 3_600), T1, "HKEX-HOL-2025"),
        // 2025-12-25 - T1 - HKEX-HOL-2025 - operator Holiday (no trading)
        (2025, 12, 25, Closed, T1, "HKEX-HOL-2025"),
        // 2025-12-26 - T1 - HKEX-HOL-2025 - operator Holiday (no trading)
        (2025, 12, 26, Closed, T1, "HKEX-HOL-2025"),
        // 2025-12-31 - T1 - HKEX-HOL-2025 - the half-day eve: no afternoon session (and, from 2013, no AHT); the scalar clip keeps the morning and removes everything after 12:30 p.m.
        (2025, 12, 31, early_close(12 * 3_600), T1, "HKEX-HOL-2025"),
        // 2026-01-01 - T1 - HKEX-HOL-CURRENT - operator Holiday (no trading)
        (2026, 1, 1, Closed, T1, "HKEX-HOL-CURRENT"),
        // 2026-02-16 - T1 - HKEX-HOL-CURRENT - the half-day eve: no afternoon session (and, from 2013, no AHT); the scalar clip keeps the morning and removes everything after 12:30 p.m.
        (2026, 2, 16, early_close(12 * 3_600), T1, "HKEX-HOL-CURRENT"),
        // 2026-04-03 - T1 - HKEX-HOL-CURRENT - operator Holiday (no trading)
        (2026, 4, 3, Closed, T1, "HKEX-HOL-CURRENT"),
        // 2026-04-06 - T1 - HKEX-HOL-CURRENT - operator Holiday (no trading)
        (2026, 4, 6, Closed, T1, "HKEX-HOL-CURRENT"),
        // 2026-04-07 - T1 - HKEX-HOL-CURRENT - operator Holiday (no trading)
        (2026, 4, 7, Closed, T1, "HKEX-HOL-CURRENT"),
        // 2026-05-01 - T1 - HKEX-HOL-CURRENT - operator Holiday (no trading)
        (2026, 5, 1, Closed, T1, "HKEX-HOL-CURRENT"),
        // 2026-05-25 - T1 - HKEX-HOL-CURRENT - operator Holiday (no trading)
        (2026, 5, 25, Closed, T1, "HKEX-HOL-CURRENT"),
        // 2026-06-19 - T1 - HKEX-HOL-CURRENT - operator Holiday (no trading)
        (2026, 6, 19, Closed, T1, "HKEX-HOL-CURRENT"),
        // 2026-07-01 - T1 - HKEX-HOL-CURRENT - operator Holiday (no trading)
        (2026, 7, 1, Closed, T1, "HKEX-HOL-CURRENT"),
        // 2026-10-01 - T1 - HKEX-HOL-CURRENT - operator Holiday (no trading)
        (2026, 10, 1, Closed, T1, "HKEX-HOL-CURRENT"),
        // 2026-10-19 - T1 - HKEX-HOL-CURRENT - operator Holiday (no trading)
        (2026, 10, 19, Closed, T1, "HKEX-HOL-CURRENT"),
        // 2026-10-20 - T1 - HKEX-HOL-CURRENT - the holiday evening ran no AHT session; the late open removes the wrapped leg and keeps the following day's own sessions
        (2026, 10, 20, late_open(8 * 3_600 + 45 * 60), T1, "HKEX-HOL-CURRENT"),
        // 2026-12-24 - T1 - HKEX-HOL-CURRENT - the half-day eve closes at 12:30 p.m. with no afternoon session and no evening AHT; the prior evening's AHT leg runs to 03:00 and the day's own morning runs to the half-day close
        (2026, 12, 24, ReplacementBlocks(&BLOCKS_02), T1, "HKEX-HOL-CURRENT"),
        // 2026-12-25 - T1 - HKEX-HOL-CURRENT - operator Holiday (no trading)
        (2026, 12, 25, Closed, T1, "HKEX-HOL-CURRENT"),
        // 2026-12-26 - T1 - HKEX-HOL-CURRENT - the holiday evening ran no AHT session; the wrapped leg's trade date is this Saturday and no session belongs to it
        (2026, 12, 26, Closed, T1, "HKEX-HOL-CURRENT"),
        // 2026-12-31 - T1 - HKEX-HOL-CURRENT - the half-day eve closes at 12:30 p.m. with no afternoon session and no evening AHT; the prior evening's AHT leg runs to 03:00 and the day's own morning runs to the half-day close
        (2026, 12, 31, ReplacementBlocks(&BLOCKS_02), T1, "HKEX-HOL-CURRENT"),
        // 2027-01-01 - T1 - HKEX-HOL-CURRENT - operator Holiday (no trading)
        (2027, 1, 1, Closed, T1, "HKEX-HOL-CURRENT"),
        // 2027-01-02 - T1 - HKEX-HOL-CURRENT - the holiday evening ran no AHT session; the wrapped leg's trade date is this Saturday and no session belongs to it
        (2027, 1, 2, Closed, T1, "HKEX-HOL-CURRENT"),
        // 2027-02-05 - T1 - HKEX-HOL-CURRENT - the half-day eve closes at 12:30 p.m. with no afternoon session and no evening AHT; the prior evening's AHT leg runs to 03:00 and the day's own morning runs to the half-day close
        (2027, 2, 5, ReplacementBlocks(&BLOCKS_02), T1, "HKEX-HOL-CURRENT"),
        // 2027-02-06 - T1 - HKEX-HOL-CURRENT - the eve ran no AHT session; the wrapped leg's trade date is this Saturday and no session belongs to it
        (2027, 2, 6, Closed, T1, "HKEX-HOL-CURRENT"),
        // 2027-02-08 - T1 - HKEX-HOL-CURRENT - operator Holiday (no trading)
        (2027, 2, 8, Closed, T1, "HKEX-HOL-CURRENT"),
        // 2027-02-09 - T1 - HKEX-HOL-CURRENT - operator Holiday (no trading)
        (2027, 2, 9, Closed, T1, "HKEX-HOL-CURRENT"),
        // 2027-02-10 - T1 - HKEX-HOL-CURRENT - the holiday evening ran no AHT session; the late open removes the wrapped leg and keeps the following day's own sessions
        (2027, 2, 10, late_open(8 * 3_600 + 45 * 60), T1, "HKEX-HOL-CURRENT"),
        // 2027-03-26 - T1 - HKEX-HOL-CURRENT - operator Holiday (no trading)
        (2027, 3, 26, Closed, T1, "HKEX-HOL-CURRENT"),
        // 2027-03-27 - T1 - HKEX-HOL-CURRENT - the holiday evening ran no AHT session; the wrapped leg's trade date is this Saturday and no session belongs to it
        (2027, 3, 27, Closed, T1, "HKEX-HOL-CURRENT"),
        // 2027-03-29 - T1 - HKEX-HOL-CURRENT - operator Holiday (no trading)
        (2027, 3, 29, Closed, T1, "HKEX-HOL-CURRENT"),
        // 2027-03-30 - T1 - HKEX-HOL-CURRENT - the holiday evening ran no AHT session; the late open removes the wrapped leg and keeps the following day's own sessions
        (2027, 3, 30, late_open(8 * 3_600 + 45 * 60), T1, "HKEX-HOL-CURRENT"),
        // 2027-04-05 - T1 - HKEX-HOL-CURRENT - operator Holiday (no trading)
        (2027, 4, 5, Closed, T1, "HKEX-HOL-CURRENT"),
        // 2027-04-06 - T1 - HKEX-HOL-CURRENT - the holiday evening ran no AHT session; the late open removes the wrapped leg and keeps the following day's own sessions
        (2027, 4, 6, late_open(8 * 3_600 + 45 * 60), T1, "HKEX-HOL-CURRENT"),
        // 2027-05-13 - T1 - HKEX-HOL-CURRENT - operator Holiday (no trading)
        (2027, 5, 13, Closed, T1, "HKEX-HOL-CURRENT"),
        // 2027-05-14 - T1 - HKEX-HOL-CURRENT - the holiday evening ran no AHT session; the late open removes the wrapped leg and keeps the following day's own sessions
        (2027, 5, 14, late_open(8 * 3_600 + 45 * 60), T1, "HKEX-HOL-CURRENT"),
        // 2027-06-09 - T1 - HKEX-HOL-CURRENT - operator Holiday (no trading)
        (2027, 6, 9, Closed, T1, "HKEX-HOL-CURRENT"),
        // 2027-06-10 - T1 - HKEX-HOL-CURRENT - the holiday evening ran no AHT session; the late open removes the wrapped leg and keeps the following day's own sessions
        (2027, 6, 10, late_open(8 * 3_600 + 45 * 60), T1, "HKEX-HOL-CURRENT"),
        // 2027-07-01 - T1 - HKEX-HOL-CURRENT - operator Holiday (no trading)
        (2027, 7, 1, Closed, T1, "HKEX-HOL-CURRENT"),
        // 2027-07-02 - T1 - HKEX-HOL-CURRENT - the holiday evening ran no AHT session; the late open removes the wrapped leg and keeps the following day's own sessions
        (2027, 7, 2, late_open(8 * 3_600 + 45 * 60), T1, "HKEX-HOL-CURRENT"),
        // 2027-09-16 - T1 - HKEX-HOL-CURRENT - operator Holiday (no trading)
        (2027, 9, 16, Closed, T1, "HKEX-HOL-CURRENT"),
        // 2027-09-17 - T1 - HKEX-HOL-CURRENT - the holiday evening ran no AHT session; the late open removes the wrapped leg and keeps the following day's own sessions
        (2027, 9, 17, late_open(8 * 3_600 + 45 * 60), T1, "HKEX-HOL-CURRENT"),
        // 2027-10-01 - T1 - HKEX-HOL-CURRENT - operator Holiday (no trading)
        (2027, 10, 1, Closed, T1, "HKEX-HOL-CURRENT"),
        // 2027-10-02 - T1 - HKEX-HOL-CURRENT - the holiday evening ran no AHT session; the wrapped leg's trade date is this Saturday and no session belongs to it
        (2027, 10, 2, Closed, T1, "HKEX-HOL-CURRENT"),
        // 2027-10-08 - T1 - HKEX-HOL-CURRENT - operator Holiday (no trading)
        (2027, 10, 8, Closed, T1, "HKEX-HOL-CURRENT"),
        // 2027-10-09 - T1 - HKEX-HOL-CURRENT - the holiday evening ran no AHT session; the wrapped leg's trade date is this Saturday and no session belongs to it
        (2027, 10, 9, Closed, T1, "HKEX-HOL-CURRENT"),
        // 2027-12-24 - T1 - HKEX-HOL-CURRENT - the half-day eve closes at 12:30 p.m. with no afternoon session and no evening AHT; the prior evening's AHT leg runs to 03:00 and the day's own morning runs to the half-day close
        (2027, 12, 24, ReplacementBlocks(&BLOCKS_02), T1, "HKEX-HOL-CURRENT"),
        // 2027-12-25 - T1 - HKEX-HOL-CURRENT - the eve ran no AHT session; the wrapped leg's trade date is this Saturday and no session belongs to it
        (2027, 12, 25, Closed, T1, "HKEX-HOL-CURRENT"),
        // 2027-12-27 - T1 - HKEX-HOL-CURRENT - operator Holiday (no trading)
        (2027, 12, 27, Closed, T1, "HKEX-HOL-CURRENT"),
        // 2027-12-28 - T1 - HKEX-HOL-CURRENT - the holiday evening ran no AHT session; the late open removes the wrapped leg and keeps the following day's own sessions
        (2027, 12, 28, late_open(8 * 3_600 + 45 * 60), T1, "HKEX-HOL-CURRENT"),
        // 2027-12-31 - T1 - HKEX-HOL-CURRENT - the half-day eve closes at 12:30 p.m. with no afternoon session and no evening AHT; the prior evening's AHT leg runs to 03:00 and the day's own morning runs to the half-day close
        (2027, 12, 31, ReplacementBlocks(&BLOCKS_02), T1, "HKEX-HOL-CURRENT"),
        // 2028-01-01 - T1 - HKEX-HOL-CURRENT - the operator closed the 31 December 2027 evening AHT session, whose wrapped leg's trade date is 1 January 2028 (audited from the 2027 table)
        (2028, 1, 1, Closed, T1, "HKEX-HOL-CURRENT"),
    ],
};
