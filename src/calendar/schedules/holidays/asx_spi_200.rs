// SPDX-License-Identifier: MIT-0

//! ASX SPI 200 futures holiday and early-close data, audited 2010-01-01
//! through 2027-12-31 plus the 2028-01-01 trade date of the final New Year
//! evening leg. Evidence: `docs/evidence/asx_spi_200.md`.

use super::fences::early_close;
use crate::calendar::ExceptionBlock;

use super::EvidenceTier::T1;
use super::HolidayKind::{Closed, ReplacementBlocks};
use super::{HolidayTable, holidays};

// A "Night Session to cease trading at midnight" leg restates the night's own
// shape shortened at midnight: the ten-minute pre-open (the SYCOM footnote)
// from 17:00, the operator's 5:10pm open, and no morning leg after 00:00.
#[rustfmt::skip]
static BLOCKS_01: [ExceptionBlock; 4] = [ExceptionBlock::order_entry(-1, 61_200, 61_800), ExceptionBlock::regular(-1, 61_800, 0), ExceptionBlock::order_entry(0, 34_800, 35_400), ExceptionBlock::regular(0, 35_400, 59_400)];

// The same midnight-cease leg alone, for a trade date whose own day session
// does not run (the July-4 Saturdays; the Good Friday trade dates below).
#[rustfmt::skip]
static BLOCKS_02: [ExceptionBlock; 2] = [ExceptionBlock::order_entry(-1, 61_200, 61_800), ExceptionBlock::regular(-1, 61_800, 0)];

// The two 2016 rows print the night opening 5:00pm: pre-open 16:50-17:00, the
// leg from 17:00 to the column's normal morning close.
#[rustfmt::skip]
static BLOCKS_03: [ExceptionBlock; 4] = [ExceptionBlock::order_entry(-1, 60_600, 61_200), ExceptionBlock::regular(-1, 61_200, 28_800), ExceptionBlock::order_entry(0, 34_800, 35_400), ExceptionBlock::regular(0, 35_400, 59_400)];

#[rustfmt::skip]
static BLOCKS_04: [ExceptionBlock; 4] = [ExceptionBlock::order_entry(-1, 60_600, 61_200), ExceptionBlock::regular(-1, 61_200, 25_200), ExceptionBlock::order_entry(0, 34_800, 35_400), ExceptionBlock::regular(0, 35_400, 59_400)];

// The closed holiday evening's own night session is removed while the
// following trade date's day session and its pre-open run: the day-only
// replacement restages that trade date without a wrapped leg.
#[rustfmt::skip]
static BLOCKS_05: [ExceptionBlock; 2] = [ExceptionBlock::order_entry(0, 34_800, 35_400), ExceptionBlock::regular(0, 35_400, 59_400)];

// Evidence: docs/evidence/asx_spi_200.md
pub(crate) static TABLE: &HolidayTable = holidays! {
    coverage: [(2010, 1, 1) ..= (2027, 12, 31), (2028, 1, 1) ..= (2028, 1, 1)],
    rows: [
        // 2010-01-01 - T1 - ASX24-CAL-2010 - operator Closed/Non Trading Day (day and night sessions)
        (2010, 1, 1, Closed, T1, "ASX24-CAL-2010"),
        // 2010-01-02 - T1 - ASX24-CAL-2010 - the operator closed the Friday evening's night session, whose trade date is the Saturday
        (2010, 1, 2, Closed, T1, "ASX24-CAL-2010"),
        // 2010-01-19 - T1 - ASX24-CAL-2010 - Night session to cease trading at midnight (US policy): the night session opening on the US holiday ceased at midnight Sydney, so its trade date (the following day) runs that shortened leg and then its own day
        (2010, 1, 19, ReplacementBlocks(&BLOCKS_01), T1, "ASX24-CAL-2010"),
        // 2010-01-26 - T1 - ASX24-CAL-2010 - Day Session Closed / Night Session to open at 5:10pm: the day session is removed and the morning night leg ends at its normal 08:00 close
        (2010, 1, 26, early_close(8 * 3_600 ), T1, "ASX24-CAL-2010"),
        // 2010-02-06 - T1 - ASX24-CAL-2010 - operator Closed/Non Trading Day (day and night sessions)
        (2010, 2, 6, Closed, T1, "ASX24-CAL-2010"),
        // 2010-02-16 - T1 - ASX24-CAL-2010 - Night session to cease trading at midnight (US policy): the night session opening on the US holiday ceased at midnight Sydney, so its trade date (the following day) runs that shortened leg and then its own day
        (2010, 2, 16, ReplacementBlocks(&BLOCKS_01), T1, "ASX24-CAL-2010"),
        // 2010-04-02 - T1 - ASX24-CAL-2010 - Day Before Good Friday: the night session opening on the eve ceased at midnight (operator row) and Good Friday itself is Closed/Non Trading Day, so the trade date runs that shortened evening leg and no day session
        (2010, 4, 2, ReplacementBlocks(&BLOCKS_02), T1, "ASX24-CAL-2010"),
        // 2010-04-03 - T1 - ASX24-CAL-2010 - the operator closed the Friday evening's night session, whose trade date is the Saturday
        (2010, 4, 3, Closed, T1, "ASX24-CAL-2010"),
        // 2010-04-05 - T1 - ASX24-CAL-2010 - operator Closed/Non Trading Day (day and night sessions)
        (2010, 4, 5, Closed, T1, "ASX24-CAL-2010"),
        // 2010-04-06 - T1 - ASX24-CAL-2010 - the operator closed the holiday evening's night session; the late open removes the wrapped leg and keeps the following day's own sessions
        (2010, 4, 6, ReplacementBlocks(&BLOCKS_05), T1, "ASX24-CAL-2010"),
        // 2010-04-26 - T1 - ASX24-CAL-2010 - Day Session Closed / Night Session to open at 5:10pm: the day session is removed and the morning night leg ends at its normal 07:00 close
        (2010, 4, 26, early_close(7 * 3_600 ), T1, "ASX24-CAL-2010"),
        // 2010-06-01 - T1 - ASX24-CAL-2010 - Night session to cease trading at midnight (US policy): the night session opening on the US holiday ceased at midnight Sydney, so its trade date (the following day) runs that shortened leg and then its own day
        (2010, 6, 1, ReplacementBlocks(&BLOCKS_01), T1, "ASX24-CAL-2010"),
        // 2010-06-14 - T1 - ASX24-CAL-2010 - Day Session Closed / Night Session to open at 5:10pm: the day session is removed and the morning night leg ends at its normal 07:00 close
        (2010, 6, 14, early_close(7 * 3_600 ), T1, "ASX24-CAL-2010"),
        // 2010-07-06 - T1 - ASX24-CAL-2010 - Night session to cease trading at midnight (US policy): the night session opening on the US holiday ceased at midnight Sydney, so its trade date (the following day) runs that shortened leg and then its own day
        (2010, 7, 6, ReplacementBlocks(&BLOCKS_01), T1, "ASX24-CAL-2010"),
        // 2010-09-07 - T1 - ASX24-CAL-2010 - Night session to cease trading at midnight (US policy): the night session opening on the US holiday ceased at midnight Sydney, so its trade date (the following day) runs that shortened leg and then its own day
        (2010, 9, 7, ReplacementBlocks(&BLOCKS_01), T1, "ASX24-CAL-2010"),
        // 2010-11-26 - T1 - ASX24-CAL-2010 - Night session to cease trading at midnight (US policy): the night session opening on the US holiday ceased at midnight Sydney, so its trade date (the following day) runs that shortened leg and then its own day
        (2010, 11, 26, ReplacementBlocks(&BLOCKS_01), T1, "ASX24-CAL-2010"),
        // 2010-12-24 - T1 - ASX24-CAL-2010 - Equity Products to close at 2:30pm and Night Session Closed: the day session closes 14:30 and the evening night session does not open
        (2010, 12, 24, early_close(14 * 3_600 + 30 * 60), T1, "ASX24-CAL-2010"),
        // 2010-12-25 - T1 - ASX24-CAL-2010 - operator Closed/Non Trading Day (day and night sessions)
        (2010, 12, 25, Closed, T1, "ASX24-CAL-2010"),
        // 2010-12-26 - T1 - ASX24-CAL-2010 - operator Closed/Non Trading Day (day and night sessions)
        (2010, 12, 26, Closed, T1, "ASX24-CAL-2010"),
        // 2010-12-27 - T1 - ASX24-CAL-2010 - operator Closed/Non Trading Day (day and night sessions)
        (2010, 12, 27, Closed, T1, "ASX24-CAL-2010"),
        // 2010-12-28 - T1 - ASX24-CAL-2010 - the operator closed the holiday evening's night session; the late open removes the wrapped leg and keeps the following day's own sessions
        (2010, 12, 28, ReplacementBlocks(&BLOCKS_05), T1, "ASX24-CAL-2010"),
        // 2010-12-31 - T1 - ASX24-CAL-2010 - Equity Products to close at 2:30pm and Night Session Closed: the day session closes 14:30 and the evening night session does not open
        (2010, 12, 31, early_close(14 * 3_600 + 30 * 60), T1, "ASX24-CAL-2010"),
        // 2011-01-01 - T1 - ASX24-CAL-2010 - the closed Friday night session's trade date is the Saturday
        (2011, 1, 1, Closed, T1, "ASX24-CAL-2010"),
        // 2011-01-03 - T1 - ASX24-CAL-2011 - operator Closed/Non Trading Day (day and night sessions)
        (2011, 1, 3, Closed, T1, "ASX24-CAL-2011"),
        // 2011-01-04 - T1 - ASX24-CAL-2011 - the operator closed the holiday evening's night session; the late open removes the wrapped leg and keeps the following day's own sessions
        (2011, 1, 4, ReplacementBlocks(&BLOCKS_05), T1, "ASX24-CAL-2011"),
        // 2011-01-18 - T1 - ASX24-CAL-2011 - Night session to cease trading at midnight (US policy): the night session opening on the US holiday ceased at midnight Sydney, so its trade date (the following day) runs that shortened leg and then its own day
        (2011, 1, 18, ReplacementBlocks(&BLOCKS_01), T1, "ASX24-CAL-2011"),
        // 2011-01-26 - T1 - ASX24-CAL-2011 - Day Session Closed / Night Session to open at 5:10pm: the day session is removed and the morning night leg ends at its normal 08:00 close
        (2011, 1, 26, early_close(8 * 3_600 ), T1, "ASX24-CAL-2011"),
        // 2011-02-06 - T1 - ASX24-CAL-2011 - operator Closed/Non Trading Day (day and night sessions)
        (2011, 2, 6, Closed, T1, "ASX24-CAL-2011"),
        // 2011-02-07 - T1 - ASX24-CAL-2011 - the operator closed the Sunday evening's night session; the late open removes the wrapped leg and keeps Monday's own sessions
        (2011, 2, 7, ReplacementBlocks(&BLOCKS_05), T1, "ASX24-CAL-2011"),
        // 2011-02-22 - T1 - ASX24-CAL-2011 - Night session to cease trading at midnight (US policy): the night session opening on the US holiday ceased at midnight Sydney, so its trade date (the following day) runs that shortened leg and then its own day
        (2011, 2, 22, ReplacementBlocks(&BLOCKS_01), T1, "ASX24-CAL-2011"),
        // 2011-04-22 - T1 - ASX24-CAL-2011 - Day Before Good Friday: the night session opening on the eve ceased at midnight (operator row) and Good Friday itself is Closed/Non Trading Day, so the trade date runs that shortened evening leg and no day session
        (2011, 4, 22, ReplacementBlocks(&BLOCKS_02), T1, "ASX24-CAL-2011"),
        // 2011-04-23 - T1 - ASX24-CAL-2011 - the operator closed the Friday evening's night session, whose trade date is the Saturday
        (2011, 4, 23, Closed, T1, "ASX24-CAL-2011"),
        // 2011-04-25 - T1 - ASX24-CAL-2011 - operator Closed/Non Trading Day (day and night sessions)
        (2011, 4, 25, Closed, T1, "ASX24-CAL-2011"),
        // 2011-04-26 - T1 - ASX24-CAL-2011 - the operator closed the day session and the prior evening's night session (the previous day was itself closed, so no morning leg ran); the holiday evening's own night session is the next trade date's opening leg and ran
        (2011, 4, 26, Closed, T1, "ASX24-CAL-2011"),
        // 2011-05-31 - T1 - ASX24-CAL-2011 - Night session to cease trading at midnight (US policy): the night session opening on the US holiday ceased at midnight Sydney, so its trade date (the following day) runs that shortened leg and then its own day
        (2011, 5, 31, ReplacementBlocks(&BLOCKS_01), T1, "ASX24-CAL-2011"),
        // 2011-06-13 - T1 - ASX24-CAL-2011 - Day Session Closed / Night Session to open at 5:10pm: the day session is removed and the morning night leg ends at its normal 07:00 close
        (2011, 6, 13, early_close(7 * 3_600 ), T1, "ASX24-CAL-2011"),
        // 2011-07-05 - T1 - ASX24-CAL-2011 - Night session to cease trading at midnight (US policy): the night session opening on the US holiday ceased at midnight Sydney, so its trade date (the following day) runs that shortened leg and then its own day
        (2011, 7, 5, ReplacementBlocks(&BLOCKS_01), T1, "ASX24-CAL-2011"),
        // 2011-09-06 - T1 - ASX24-CAL-2011 - Night session to cease trading at midnight (US policy): the night session opening on the US holiday ceased at midnight Sydney, so its trade date (the following day) runs that shortened leg and then its own day
        (2011, 9, 6, ReplacementBlocks(&BLOCKS_01), T1, "ASX24-CAL-2011"),
        // 2011-11-25 - T1 - ASX24-CAL-2011 - Night session to cease trading at midnight (US policy): the night session opening on the US holiday ceased at midnight Sydney, so its trade date (the following day) runs that shortened leg and then its own day
        (2011, 11, 25, ReplacementBlocks(&BLOCKS_01), T1, "ASX24-CAL-2011"),
        // 2011-12-23 - T1 - ASX24-CAL-2011 - Equity Products to close at 2:30pm and Night Session Closed: the day session closes 14:30 and the evening night session does not open
        (2011, 12, 23, early_close(14 * 3_600 + 30 * 60), T1, "ASX24-CAL-2011"),
        // 2011-12-24 - T1 - ASX24-CAL-2011 - the closed Friday night session's trade date is the Saturday
        (2011, 12, 24, Closed, T1, "ASX24-CAL-2011"),
        // 2011-12-25 - T1 - ASX24-CAL-2011 - operator Non Trading Day (the Sunday keeps its operator-stated row; no session opens into it)
        (2011, 12, 25, Closed, T1, "ASX24-CAL-2011"),
        // 2011-12-26 - T1 - ASX24-CAL-2011 - operator Closed/Non Trading Day (day and night sessions)
        (2011, 12, 26, Closed, T1, "ASX24-CAL-2011"),
        // 2011-12-27 - T1 - ASX24-CAL-2011 - operator Closed/Non Trading Day (day and night sessions)
        (2011, 12, 27, Closed, T1, "ASX24-CAL-2011"),
        // 2011-12-28 - T1 - ASX24-CAL-2011 - the operator closed the holiday evening's night session; the late open removes the wrapped leg and keeps the following day's own sessions
        (2011, 12, 28, ReplacementBlocks(&BLOCKS_05), T1, "ASX24-CAL-2011"),
        // 2011-12-30 - T1 - ASX24-CAL-2011 - Equity Products to close at 2:30pm and Night Session Closed: the day session closes 14:30 and the evening night session does not open
        (2011, 12, 30, early_close(14 * 3_600 + 30 * 60), T1, "ASX24-CAL-2011"),
        // 2011-12-31 - T1 - ASX24-CAL-2011 - the closed Friday night session's trade date is the Saturday
        (2011, 12, 31, Closed, T1, "ASX24-CAL-2011"),
        // 2012-01-02 - T1 - ASX24-CAL-2012 - operator Closed/Non Trading Day (day and night sessions)
        (2012, 1, 2, Closed, T1, "ASX24-CAL-2012"),
        // 2012-01-03 - T1 - ASX24-CAL-2012 - the operator closed the holiday evening's night session; the late open removes the wrapped leg and keeps the following day's own sessions
        (2012, 1, 3, ReplacementBlocks(&BLOCKS_05), T1, "ASX24-CAL-2012"),
        // 2012-01-17 - T1 - ASX24-CAL-2012 - Night session to cease trading at midnight (US policy): the night session opening on the US holiday ceased at midnight Sydney, so its trade date (the following day) runs that shortened leg and then its own day
        (2012, 1, 17, ReplacementBlocks(&BLOCKS_01), T1, "ASX24-CAL-2012"),
        // 2012-01-26 - T1 - ASX24-CAL-2012 - Day Session Closed / Night Session to open at 5:10pm: the day session is removed and the morning night leg ends at its normal 08:00 close
        (2012, 1, 26, early_close(8 * 3_600 ), T1, "ASX24-CAL-2012"),
        // 2012-02-21 - T1 - ASX24-CAL-2012 - Night session to cease trading at midnight (US policy): the night session opening on the US holiday ceased at midnight Sydney, so its trade date (the following day) runs that shortened leg and then its own day
        (2012, 2, 21, ReplacementBlocks(&BLOCKS_01), T1, "ASX24-CAL-2012"),
        // 2012-04-06 - T1 - ASX24-CAL-2012 - Day Before Good Friday: the night session opening on the eve ceased at midnight (operator row) and Good Friday itself is Closed/Non Trading Day, so the trade date runs that shortened evening leg and no day session
        (2012, 4, 6, ReplacementBlocks(&BLOCKS_02), T1, "ASX24-CAL-2012"),
        // 2012-04-07 - T1 - ASX24-CAL-2012 - the operator closed the Friday evening's night session, whose trade date is the Saturday
        (2012, 4, 7, Closed, T1, "ASX24-CAL-2012"),
        // 2012-04-09 - T1 - ASX24-CAL-2012 - operator Closed/Non Trading Day (day and night sessions)
        (2012, 4, 9, Closed, T1, "ASX24-CAL-2012"),
        // 2012-04-10 - T1 - ASX24-CAL-2012 - the operator closed the holiday evening's night session; the late open removes the wrapped leg and keeps the following day's own sessions
        (2012, 4, 10, ReplacementBlocks(&BLOCKS_05), T1, "ASX24-CAL-2012"),
        // 2012-04-25 - T1 - ASX24-CAL-2012 - Day Session Closed / Night Session to open at 5:10pm: the day session is removed and the morning night leg ends at its normal 07:00 close
        (2012, 4, 25, early_close(7 * 3_600 ), T1, "ASX24-CAL-2012"),
        // 2012-05-29 - T1 - ASX24-CAL-2012 - Night session to cease trading at midnight (US policy): the night session opening on the US holiday ceased at midnight Sydney, so its trade date (the following day) runs that shortened leg and then its own day
        (2012, 5, 29, ReplacementBlocks(&BLOCKS_01), T1, "ASX24-CAL-2012"),
        // 2012-06-11 - T1 - ASX24-CAL-2012 - Day Session Closed / Night Session to open at 5:10pm: the day session is removed and the morning night leg ends at its normal 07:00 close
        (2012, 6, 11, early_close(7 * 3_600 ), T1, "ASX24-CAL-2012"),
        // 2012-07-05 - T1 - ASX24-CAL-2012 - Night session to cease trading at midnight (US policy): the night session opening on the US holiday ceased at midnight Sydney, so its trade date (the following day) runs that shortened leg and then its own day
        (2012, 7, 5, ReplacementBlocks(&BLOCKS_01), T1, "ASX24-CAL-2012"),
        // 2012-09-04 - T1 - ASX24-CAL-2012 - Night session to cease trading at midnight (US policy): the night session opening on the US holiday ceased at midnight Sydney, so its trade date (the following day) runs that shortened leg and then its own day
        (2012, 9, 4, ReplacementBlocks(&BLOCKS_01), T1, "ASX24-CAL-2012"),
        // 2012-11-23 - T1 - ASX24-CAL-2012 - Night session to cease trading at midnight (US policy): the night session opening on the US holiday ceased at midnight Sydney, so its trade date (the following day) runs that shortened leg and then its own day
        (2012, 11, 23, ReplacementBlocks(&BLOCKS_01), T1, "ASX24-CAL-2012"),
        // 2012-12-24 - T1 - ASX24-CAL-2012 - Equity Products to close at 2:30pm and Night Session Closed: the day session closes 14:30 and the evening night session does not open
        (2012, 12, 24, early_close(14 * 3_600 + 30 * 60), T1, "ASX24-CAL-2012"),
        // 2012-12-25 - T1 - ASX24-CAL-2012 - operator Closed/Non Trading Day (day and night sessions)
        (2012, 12, 25, Closed, T1, "ASX24-CAL-2012"),
        // 2012-12-26 - T1 - ASX24-CAL-2012 - operator Closed/Non Trading Day (day and night sessions)
        (2012, 12, 26, Closed, T1, "ASX24-CAL-2012"),
        // 2012-12-27 - T1 - ASX24-CAL-2012 - the operator closed the holiday evening's night session; the late open removes the wrapped leg and keeps the following day's own sessions
        (2012, 12, 27, ReplacementBlocks(&BLOCKS_05), T1, "ASX24-CAL-2012"),
        // 2012-12-31 - T1 - ASX24-CAL-2012 - Equity Products to close at 2:30pm and Night Session Closed: the day session closes 14:30 and the evening night session does not open
        (2012, 12, 31, early_close(14 * 3_600 + 30 * 60), T1, "ASX24-CAL-2012"),
        // 2013-01-01 - T1 - ASX24-CAL-2013 - operator Closed/Non Trading Day (day and night sessions)
        (2013, 1, 1, Closed, T1, "ASX24-CAL-2013"),
        // 2013-01-02 - T1 - ASX24-CAL-2013 - the operator closed the holiday evening's night session; the late open removes the wrapped leg and keeps the following day's own sessions
        (2013, 1, 2, ReplacementBlocks(&BLOCKS_05), T1, "ASX24-CAL-2013"),
        // 2013-01-22 - T1 - ASX24-CAL-2013 - Night session to cease trading at midnight (US policy): the night session opening on the US holiday ceased at midnight Sydney, so its trade date (the following day) runs that shortened leg and then its own day
        (2013, 1, 22, ReplacementBlocks(&BLOCKS_01), T1, "ASX24-CAL-2013"),
        // 2013-01-28 - T1 - ASX24-CAL-2013 - Day Session Closed / Night Session to open at 5:10pm: the day session is removed and the morning night leg ends at its normal 08:00 close
        (2013, 1, 28, early_close(8 * 3_600 ), T1, "ASX24-CAL-2013"),
        // 2013-02-19 - T1 - ASX24-CAL-2013 - Night session to cease trading at midnight (US policy): the night session opening on the US holiday ceased at midnight Sydney, so its trade date (the following day) runs that shortened leg and then its own day
        (2013, 2, 19, ReplacementBlocks(&BLOCKS_01), T1, "ASX24-CAL-2013"),
        // 2013-03-29 - T1 - ASX24-CAL-2013 - Day Before Good Friday: the night session opening on the eve ceased at midnight (operator row) and Good Friday itself is Closed/Non Trading Day, so the trade date runs that shortened evening leg and no day session
        (2013, 3, 29, ReplacementBlocks(&BLOCKS_02), T1, "ASX24-CAL-2013"),
        // 2013-03-30 - T1 - ASX24-CAL-2013 - the operator closed the Friday evening's night session, whose trade date is the Saturday
        (2013, 3, 30, Closed, T1, "ASX24-CAL-2013"),
        // 2013-04-01 - T1 - ASX24-CAL-2013 - operator Closed/Non Trading Day (day and night sessions)
        (2013, 4, 1, Closed, T1, "ASX24-CAL-2013"),
        // 2013-04-02 - T1 - ASX24-CAL-2013 - the operator closed the holiday evening's night session; the late open removes the wrapped leg and keeps the following day's own sessions
        (2013, 4, 2, ReplacementBlocks(&BLOCKS_05), T1, "ASX24-CAL-2013"),
        // 2013-04-25 - T1 - ASX24-CAL-2013 - Day Session Closed / Night Session to open at 5:10pm: the day session is removed and the morning night leg ends at its normal 07:00 close
        (2013, 4, 25, early_close(7 * 3_600 ), T1, "ASX24-CAL-2013"),
        // 2013-05-28 - T1 - ASX24-CAL-2013 - Night session to cease trading at midnight (US policy): the night session opening on the US holiday ceased at midnight Sydney, so its trade date (the following day) runs that shortened leg and then its own day
        (2013, 5, 28, ReplacementBlocks(&BLOCKS_01), T1, "ASX24-CAL-2013"),
        // 2013-06-10 - T1 - ASX24-CAL-2013 - Day Session Closed / Night Session to open at 5:10pm: the day session is removed and the morning night leg ends at its normal 07:00 close
        (2013, 6, 10, early_close(7 * 3_600 ), T1, "ASX24-CAL-2013"),
        // 2013-07-05 - T1 - ASX24-CAL-2013 - Night session to cease trading at midnight (US policy): the night session opening on the US holiday ceased at midnight Sydney, so its trade date (the following day) runs that shortened leg and then its own day
        (2013, 7, 5, ReplacementBlocks(&BLOCKS_01), T1, "ASX24-CAL-2013"),
        // 2013-09-03 - T1 - ASX24-CAL-2013 - Night session to cease trading at midnight (US policy): the night session opening on the US holiday ceased at midnight Sydney, so its trade date (the following day) runs that shortened leg and then its own day
        (2013, 9, 3, ReplacementBlocks(&BLOCKS_01), T1, "ASX24-CAL-2013"),
        // 2013-11-29 - T1 - ASX24-CAL-2013 - Night session to cease trading at midnight (US policy): the night session opening on the US holiday ceased at midnight Sydney, so its trade date (the following day) runs that shortened leg and then its own day
        (2013, 11, 29, ReplacementBlocks(&BLOCKS_01), T1, "ASX24-CAL-2013"),
        // 2013-12-24 - T1 - ASX24-CAL-2013 - Equity Products to close at 2:30pm and Night Session Closed: the day session closes 14:30 and the evening night session does not open
        (2013, 12, 24, early_close(14 * 3_600 + 30 * 60), T1, "ASX24-CAL-2013"),
        // 2013-12-25 - T1 - ASX24-CAL-2013 - operator Closed/Non Trading Day (day and night sessions)
        (2013, 12, 25, Closed, T1, "ASX24-CAL-2013"),
        // 2013-12-26 - T1 - ASX24-CAL-2013 - operator Closed/Non Trading Day (day and night sessions)
        (2013, 12, 26, Closed, T1, "ASX24-CAL-2013"),
        // 2013-12-27 - T1 - ASX24-CAL-2013 - the operator closed the holiday evening's night session; the late open removes the wrapped leg and keeps the following day's own sessions
        (2013, 12, 27, ReplacementBlocks(&BLOCKS_05), T1, "ASX24-CAL-2013"),
        // 2013-12-31 - T1 - ASX24-CAL-2013 - Equity Products to close at 2:30pm and Night Session Closed: the day session closes 14:30 and the evening night session does not open
        (2013, 12, 31, early_close(14 * 3_600 + 30 * 60), T1, "ASX24-CAL-2013"),
        // 2014-01-01 - T1 - ASX24-CAL-2014 - operator Closed/Non Trading Day (day and night sessions)
        (2014, 1, 1, Closed, T1, "ASX24-CAL-2014"),
        // 2014-01-02 - T1 - ASX24-CAL-2014 - the operator closed the holiday evening's night session; the late open removes the wrapped leg and keeps the following day's own sessions
        (2014, 1, 2, ReplacementBlocks(&BLOCKS_05), T1, "ASX24-CAL-2014"),
        // 2014-01-21 - T1 - ASX24-CAL-2014 - Night session to cease trading at midnight (US policy): the night session opening on the US holiday ceased at midnight Sydney, so its trade date (the following day) runs that shortened leg and then its own day
        (2014, 1, 21, ReplacementBlocks(&BLOCKS_01), T1, "ASX24-CAL-2014"),
        // 2014-01-27 - T1 - ASX24-CAL-2014 - Day Session Closed / Night Session to open at 5:10pm: the day session is removed and the morning night leg ends at its normal 08:00 close
        (2014, 1, 27, early_close(8 * 3_600 ), T1, "ASX24-CAL-2014"),
        // 2014-02-18 - T1 - ASX24-CAL-2014 - Night session to cease trading at midnight (US policy): the night session opening on the US holiday ceased at midnight Sydney, so its trade date (the following day) runs that shortened leg and then its own day
        (2014, 2, 18, ReplacementBlocks(&BLOCKS_01), T1, "ASX24-CAL-2014"),
        // 2014-04-18 - T1 - ASX24-CAL-2014 - Day Before Good Friday: the night session opening on the eve ceased at midnight (operator row) and Good Friday itself is Closed/Non Trading Day, so the trade date runs that shortened evening leg and no day session
        (2014, 4, 18, ReplacementBlocks(&BLOCKS_02), T1, "ASX24-CAL-2014"),
        // 2014-04-19 - T1 - ASX24-CAL-2014 - the operator closed the Friday evening's night session, whose trade date is the Saturday
        (2014, 4, 19, Closed, T1, "ASX24-CAL-2014"),
        // 2014-04-21 - T1 - ASX24-CAL-2014 - operator Closed/Non Trading Day (day and night sessions)
        (2014, 4, 21, Closed, T1, "ASX24-CAL-2014"),
        // 2014-04-22 - T1 - ASX24-CAL-2014 - the operator closed the holiday evening's night session; the late open removes the wrapped leg and keeps the following day's own sessions
        (2014, 4, 22, ReplacementBlocks(&BLOCKS_05), T1, "ASX24-CAL-2014"),
        // 2014-04-25 - T1 - ASX24-CAL-2014 - Day Session Closed / Night Session to open at 5:10pm: the day session is removed and the morning night leg ends at its normal 07:00 close
        (2014, 4, 25, early_close(7 * 3_600 ), T1, "ASX24-CAL-2014"),
        // 2014-05-27 - T1 - ASX24-CAL-2014 - Night session to cease trading at midnight (US policy): the night session opening on the US holiday ceased at midnight Sydney, so its trade date (the following day) runs that shortened leg and then its own day
        (2014, 5, 27, ReplacementBlocks(&BLOCKS_01), T1, "ASX24-CAL-2014"),
        // 2014-06-09 - T1 - ASX24-CAL-2014 - Day Session Closed / Night Session to open at 5:10pm: the day session is removed and the morning night leg ends at its normal 07:00 close
        (2014, 6, 9, early_close(7 * 3_600 ), T1, "ASX24-CAL-2014"),
        // 2014-07-05 - T1 - ASX24-CAL-2014 - Night session to cease trading at midnight (US policy): the night session opening on the US holiday ceased at midnight Sydney, so its trade date (the following day) runs that shortened leg and then its own day alone (a Saturday)
        (2014, 7, 5, ReplacementBlocks(&BLOCKS_02), T1, "ASX24-CAL-2014"),
        // 2014-09-02 - T1 - ASX24-CAL-2014 - Night session to cease trading at midnight (US policy): the night session opening on the US holiday ceased at midnight Sydney, so its trade date (the following day) runs that shortened leg and then its own day
        (2014, 9, 2, ReplacementBlocks(&BLOCKS_01), T1, "ASX24-CAL-2014"),
        // 2014-11-28 - T1 - ASX24-CAL-2014 - Night session to cease trading at midnight (US policy): the night session opening on the US holiday ceased at midnight Sydney, so its trade date (the following day) runs that shortened leg and then its own day
        (2014, 11, 28, ReplacementBlocks(&BLOCKS_01), T1, "ASX24-CAL-2014"),
        // 2014-12-24 - T1 - ASX24-CAL-2014 - Equity Products to close at 2:30pm and Night Session Closed: the day session closes 14:30 and the evening night session does not open
        (2014, 12, 24, early_close(14 * 3_600 + 30 * 60), T1, "ASX24-CAL-2014"),
        // 2014-12-25 - T1 - ASX24-CAL-2014 - operator Closed/Non Trading Day (day and night sessions)
        (2014, 12, 25, Closed, T1, "ASX24-CAL-2014"),
        // 2014-12-26 - T1 - ASX24-CAL-2014 - operator Closed/Non Trading Day (day and night sessions)
        (2014, 12, 26, Closed, T1, "ASX24-CAL-2014"),
        // 2014-12-27 - T1 - ASX24-CAL-2014 - the operator closed the Friday evening's night session, whose trade date is the Saturday
        (2014, 12, 27, Closed, T1, "ASX24-CAL-2014"),
        // 2014-12-31 - T1 - ASX24-CAL-2014 - Equity Products to close at 2:30pm and Night Session Closed: the day session closes 14:30 and the evening night session does not open
        (2014, 12, 31, early_close(14 * 3_600 + 30 * 60), T1, "ASX24-CAL-2014"),
        // 2015-01-01 - T1 - ASX24-CAL-2015 - operator Closed/Non Trading Day (day and night sessions)
        (2015, 1, 1, Closed, T1, "ASX24-CAL-2015"),
        // 2015-01-02 - T1 - ASX24-CAL-2015 - the operator closed the holiday evening's night session; the late open removes the wrapped leg and keeps the following day's own sessions
        (2015, 1, 2, ReplacementBlocks(&BLOCKS_05), T1, "ASX24-CAL-2015"),
        // 2015-01-20 - T1 - ASX24-CAL-2015 - Night session to cease trading at midnight (US policy): the night session opening on the US holiday ceased at midnight Sydney, so its trade date (the following day) runs that shortened leg and then its own day
        (2015, 1, 20, ReplacementBlocks(&BLOCKS_01), T1, "ASX24-CAL-2015"),
        // 2015-01-26 - T1 - ASX24-CAL-2015 - Day Session Closed / Night Session to open at 5:10pm: the day session is removed and the morning night leg ends at its normal 08:00 close
        (2015, 1, 26, early_close(8 * 3_600 ), T1, "ASX24-CAL-2015"),
        // 2015-02-17 - T1 - ASX24-CAL-2015 - Night session to cease trading at midnight (US policy): the night session opening on the US holiday ceased at midnight Sydney, so its trade date (the following day) runs that shortened leg and then its own day
        (2015, 2, 17, ReplacementBlocks(&BLOCKS_01), T1, "ASX24-CAL-2015"),
        // 2015-04-03 - T1 - ASX24-CAL-2015 - Day Before Good Friday: the night session opening on the eve ceased at midnight (operator row) and Good Friday itself is Closed/Non Trading Day, so the trade date runs that shortened evening leg and no day session
        (2015, 4, 3, ReplacementBlocks(&BLOCKS_02), T1, "ASX24-CAL-2015"),
        // 2015-04-04 - T1 - ASX24-CAL-2015 - the operator closed the Friday evening's night session, whose trade date is the Saturday
        (2015, 4, 4, Closed, T1, "ASX24-CAL-2015"),
        // 2015-04-06 - T1 - ASX24-CAL-2015 - operator Closed/Non Trading Day (day and night sessions)
        (2015, 4, 6, Closed, T1, "ASX24-CAL-2015"),
        // 2015-04-07 - T1 - ASX24-CAL-2015 - the operator closed the holiday evening's night session; the late open removes the wrapped leg and keeps the following day's own sessions
        (2015, 4, 7, ReplacementBlocks(&BLOCKS_05), T1, "ASX24-CAL-2015"),
        // 2015-05-26 - T1 - ASX24-CAL-2015 - Night session to cease trading at midnight (US policy): the night session opening on the US holiday ceased at midnight Sydney, so its trade date (the following day) runs that shortened leg and then its own day
        (2015, 5, 26, ReplacementBlocks(&BLOCKS_01), T1, "ASX24-CAL-2015"),
        // 2015-06-08 - T1 - ASX24-CAL-2015 - Day Session Closed / Night Session to open at 5:10pm: the day session is removed and the morning night leg ends at its normal 07:00 close
        (2015, 6, 8, early_close(7 * 3_600 ), T1, "ASX24-CAL-2015"),
        // 2015-07-04 - T1 - ASX24-CAL-2015 - Night session to cease trading at midnight (US policy): the night session opening on the US holiday ceased at midnight Sydney, so its trade date (the following day) runs that shortened leg and then its own day alone (a Saturday)
        (2015, 7, 4, ReplacementBlocks(&BLOCKS_02), T1, "ASX24-CAL-2015"),
        // 2015-09-08 - T1 - ASX24-CAL-2015 - Night session to cease trading at midnight (US policy): the night session opening on the US holiday ceased at midnight Sydney, so its trade date (the following day) runs that shortened leg and then its own day
        (2015, 9, 8, ReplacementBlocks(&BLOCKS_01), T1, "ASX24-CAL-2015"),
        // 2015-11-27 - T1 - ASX24-CAL-2015 - Night session to cease trading at midnight (US policy): the night session opening on the US holiday ceased at midnight Sydney, so its trade date (the following day) runs that shortened leg and then its own day
        (2015, 11, 27, ReplacementBlocks(&BLOCKS_01), T1, "ASX24-CAL-2015"),
        // 2015-12-24 - T1 - ASX24-CAL-2015 - Equity Products to close at 2:30pm and Night Session Closed: the day session closes 14:30 and the evening night session does not open
        (2015, 12, 24, early_close(14 * 3_600 + 30 * 60), T1, "ASX24-CAL-2015"),
        // 2015-12-25 - T1 - ASX24-CAL-2015 - operator Closed/Non Trading Day (day and night sessions)
        (2015, 12, 25, Closed, T1, "ASX24-CAL-2015"),
        // 2015-12-26 - T1 - ASX24-CAL-2015 - the operator closed the Friday evening's night session, whose trade date is the Saturday
        (2015, 12, 26, Closed, T1, "ASX24-CAL-2015"),
        // 2015-12-28 - T1 - ASX24-CAL-2015 - operator Closed/Non Trading Day (day and night sessions)
        (2015, 12, 28, Closed, T1, "ASX24-CAL-2015"),
        // 2015-12-29 - T1 - ASX24-CAL-2015 - the operator closed the holiday evening's night session; the late open removes the wrapped leg and keeps the following day's own sessions
        (2015, 12, 29, ReplacementBlocks(&BLOCKS_05), T1, "ASX24-CAL-2015"),
        // 2015-12-31 - T1 - ASX24-CAL-2015 - Equity Products to close at 2:30pm and Night Session Closed: the day session closes 14:30 and the evening night session does not open
        (2015, 12, 31, early_close(14 * 3_600 + 30 * 60), T1, "ASX24-CAL-2015"),
        // 2016-01-01 - T1 - ASX24-CAL-2016 - operator Closed/Non Trading Day (day and night sessions)
        (2016, 1, 1, Closed, T1, "ASX24-CAL-2016"),
        // 2016-01-02 - T1 - ASX24-CAL-2016 - the operator closed the Friday evening's night session, whose trade date is the Saturday
        (2016, 1, 2, Closed, T1, "ASX24-CAL-2016"),
        // 2016-01-26 - T1 - ASX24-CAL-2016 - Day Session Closed / Night Session to open at 5:00pm: the day session is removed and the morning night leg ends at its normal 08:00 close
        (2016, 1, 26, early_close(8 * 3_600 ), T1, "ASX24-CAL-2016"),
        // 2016-01-27 - T1 - ASX24-CAL-2016 - the capture prints the night session opening 5:00pm on the holiday, so the next trade date's opening leg opened at 17:00 and its own day ran normally
        (2016, 1, 27, ReplacementBlocks(&BLOCKS_03), T1, "ASX24-CAL-2016"),
        // 2016-03-25 - T1 - ASX24-CAL-2016 - Day Before Good Friday: the night session opening on the eve ceased at midnight (operator row) and Good Friday itself is Closed/Non Trading Day, so the trade date runs that shortened evening leg and no day session
        (2016, 3, 25, ReplacementBlocks(&BLOCKS_02), T1, "ASX24-CAL-2016"),
        // 2016-03-26 - T1 - ASX24-CAL-2016 - the operator closed the Friday evening's night session, whose trade date is the Saturday
        (2016, 3, 26, Closed, T1, "ASX24-CAL-2016"),
        // 2016-03-28 - T1 - ASX24-CAL-2016 - operator Closed/Non Trading Day (day and night sessions)
        (2016, 3, 28, Closed, T1, "ASX24-CAL-2016"),
        // 2016-03-29 - T1 - ASX24-CAL-2016 - the operator closed the holiday evening's night session; the late open removes the wrapped leg and keeps the following day's own sessions
        (2016, 3, 29, ReplacementBlocks(&BLOCKS_05), T1, "ASX24-CAL-2016"),
        // 2016-04-25 - T1 - ASX24-CAL-2016 - Day Session Closed / Night Session to open at 5:00pm: the day session is removed and the morning night leg ends at its normal 07:00 close
        (2016, 4, 25, early_close(7 * 3_600 ), T1, "ASX24-CAL-2016"),
        // 2016-04-26 - T1 - ASX24-CAL-2016 - the capture prints the night session opening 5:00pm on the holiday, so the next trade date's opening leg opened at 17:00 (US daylight time, a 07:00 close) and its own day ran normally
        (2016, 4, 26, ReplacementBlocks(&BLOCKS_04), T1, "ASX24-CAL-2016"),
        // 2016-06-13 - T1 - ASX24-CAL-2016 - Day Session Closed / Night Session to open at 5:10pm: the day session is removed and the morning night leg ends at its normal 07:00 close
        (2016, 6, 13, early_close(7 * 3_600 ), T1, "ASX24-CAL-2016"),
        // 2016-12-23 - T1 - ASX24-CAL-2016 - Equity Products to close at 2:30pm and Night Session Closed: the day session closes 14:30 and the evening night session does not open
        (2016, 12, 23, early_close(14 * 3_600 + 30 * 60), T1, "ASX24-CAL-2016"),
        // 2016-12-24 - T1 - ASX24-CAL-2016 - the closed Friday night session's trade date is the Saturday
        (2016, 12, 24, Closed, T1, "ASX24-CAL-2016"),
        // 2016-12-26 - T1 - ASX24-CAL-2016 - operator Closed/Non Trading Day (day and night sessions)
        (2016, 12, 26, Closed, T1, "ASX24-CAL-2016"),
        // 2016-12-27 - T1 - ASX24-CAL-2016 - operator Closed/Non Trading Day (day and night sessions)
        (2016, 12, 27, Closed, T1, "ASX24-CAL-2016"),
        // 2016-12-28 - T1 - ASX24-CAL-2016 - the operator closed the holiday evening's night session; the late open removes the wrapped leg and keeps the following day's own sessions
        (2016, 12, 28, ReplacementBlocks(&BLOCKS_05), T1, "ASX24-CAL-2016"),
        // 2016-12-30 - T1 - ASX24-CAL-2016 - Equity Products to close at 2:30pm and Night Session Closed: the day session closes 14:30 and the evening night session does not open
        (2016, 12, 30, early_close(14 * 3_600 + 30 * 60), T1, "ASX24-CAL-2016"),
        // 2016-12-31 - T1 - ASX24-CAL-2016 - the closed Friday night session's trade date is the Saturday
        (2016, 12, 31, Closed, T1, "ASX24-CAL-2016"),
        // 2017-01-02 - T1 - ASX24-CAL-2017 - operator Closed/Non Trading Day (day and night sessions)
        (2017, 1, 2, Closed, T1, "ASX24-CAL-2017"),
        // 2017-01-03 - T1 - ASX24-CAL-2017 - the operator closed the holiday evening's night session; the late open removes the wrapped leg and keeps the following day's own sessions
        (2017, 1, 3, ReplacementBlocks(&BLOCKS_05), T1, "ASX24-CAL-2017"),
        // 2017-01-26 - T1 - ASX24-CAL-2017 - Day Session Closed / Night Session to open at 5:10pm: the day session is removed and the morning night leg ends at its normal 08:00 close
        (2017, 1, 26, early_close(8 * 3_600 ), T1, "ASX24-CAL-2017"),
        // 2017-04-14 - T1 - ASX24-CAL-2017 - Day Before Good Friday: the night session opening on the eve ceased at midnight (operator row) and Good Friday itself is Closed/Non Trading Day, so the trade date runs that shortened evening leg and no day session
        (2017, 4, 14, ReplacementBlocks(&BLOCKS_02), T1, "ASX24-CAL-2017"),
        // 2017-04-15 - T1 - ASX24-CAL-2017 - the operator closed the Friday evening's night session, whose trade date is the Saturday
        (2017, 4, 15, Closed, T1, "ASX24-CAL-2017"),
        // 2017-04-17 - T1 - ASX24-CAL-2017 - operator Closed/Non Trading Day (day and night sessions)
        (2017, 4, 17, Closed, T1, "ASX24-CAL-2017"),
        // 2017-04-18 - T1 - ASX24-CAL-2017 - the operator closed the holiday evening's night session; the late open removes the wrapped leg and keeps the following day's own sessions
        (2017, 4, 18, ReplacementBlocks(&BLOCKS_05), T1, "ASX24-CAL-2017"),
        // 2017-04-25 - T1 - ASX24-CAL-2017 - Day Session Closed / Night Session to open at 5:10pm: the day session is removed and the morning night leg ends at its normal 07:00 close
        (2017, 4, 25, early_close(7 * 3_600 ), T1, "ASX24-CAL-2017"),
        // 2017-06-12 - T1 - ASX24-CAL-2017 - Day Session Closed / Night Session to open at 5:10pm: the day session is removed and the morning night leg ends at its normal 07:00 close
        (2017, 6, 12, early_close(7 * 3_600 ), T1, "ASX24-CAL-2017"),
        // 2017-12-23 - T1 - ASX24-CAL-2017 - Day Session Normal / Night Session Closed: the closed Friday night session's trade date is the Saturday
        (2017, 12, 23, Closed, T1, "ASX24-CAL-2017"),
        // 2017-12-25 - T1 - ASX24-CAL-2017 - operator Closed/Non Trading Day (day and night sessions)
        (2017, 12, 25, Closed, T1, "ASX24-CAL-2017"),
        // 2017-12-26 - T1 - ASX24-CAL-2017 - operator Closed/Non Trading Day (day and night sessions)
        (2017, 12, 26, Closed, T1, "ASX24-CAL-2017"),
        // 2017-12-27 - T1 - ASX24-CAL-2017 - the operator closed the holiday evening's night session; the late open removes the wrapped leg and keeps the following day's own sessions
        (2017, 12, 27, ReplacementBlocks(&BLOCKS_05), T1, "ASX24-CAL-2017"),
        // 2017-12-30 - T1 - ASX24-CAL-2017 - Day Session Normal / Night Session Closed: the closed Friday night session's trade date is the Saturday
        (2017, 12, 30, Closed, T1, "ASX24-CAL-2017"),
        // 2018-01-01 - T1 - ASX24-CAL-2018 - operator Closed/Non Trading Day (day and night sessions)
        (2018, 1, 1, Closed, T1, "ASX24-CAL-2018"),
        // 2018-01-02 - T1 - ASX24-CAL-2018 - the operator closed the holiday evening's night session; the late open removes the wrapped leg and keeps the following day's own sessions
        (2018, 1, 2, ReplacementBlocks(&BLOCKS_05), T1, "ASX24-CAL-2018"),
        // 2018-01-26 - T1 - ASX24-CAL-2018 - Day Session Closed / Night Session to open at 5:10pm: the day session is removed and the morning night leg ends at its normal 08:00 close
        (2018, 1, 26, early_close(8 * 3_600 ), T1, "ASX24-CAL-2018"),
        // 2018-03-30 - T1 - ASX24-CAL-2018 - Day Before Good Friday: the night session opening on the eve ceased at midnight (operator row) and Good Friday itself is Closed/Non Trading Day, so the trade date runs that shortened evening leg and no day session
        (2018, 3, 30, ReplacementBlocks(&BLOCKS_02), T1, "ASX24-CAL-2018"),
        // 2018-03-31 - T1 - ASX24-CAL-2018 - the operator closed the Friday evening's night session, whose trade date is the Saturday
        (2018, 3, 31, Closed, T1, "ASX24-CAL-2018"),
        // 2018-04-02 - T1 - ASX24-CAL-2018 - operator Closed/Non Trading Day (day and night sessions)
        (2018, 4, 2, Closed, T1, "ASX24-CAL-2018"),
        // 2018-04-03 - T1 - ASX24-CAL-2018 - the operator closed the holiday evening's night session; the late open removes the wrapped leg and keeps the following day's own sessions
        (2018, 4, 3, ReplacementBlocks(&BLOCKS_05), T1, "ASX24-CAL-2018"),
        // 2018-04-25 - T1 - ASX24-CAL-2018 - Day Session Closed / Night Session to open at 5:10pm: the day session is removed and the morning night leg ends at its normal 07:00 close
        (2018, 4, 25, early_close(7 * 3_600 ), T1, "ASX24-CAL-2018"),
        // 2018-06-11 - T1 - ASX24-CAL-2018 - Day Session Closed / Night Session to open at 5:10pm: the day session is removed and the morning night leg ends at its normal 07:00 close
        (2018, 6, 11, early_close(7 * 3_600 ), T1, "ASX24-CAL-2018"),
        // 2018-12-24 - T1 - ASX24-CAL-2018 - Equity Products to close at 2:30pm and Night Session Closed: the day session closes 14:30 and the evening night session does not open
        (2018, 12, 24, early_close(14 * 3_600 + 30 * 60), T1, "ASX24-CAL-2018"),
        // 2018-12-25 - T1 - ASX24-CAL-2018 - operator Closed/Non Trading Day (day and night sessions)
        (2018, 12, 25, Closed, T1, "ASX24-CAL-2018"),
        // 2018-12-26 - T1 - ASX24-CAL-2018 - operator Closed/Non Trading Day (day and night sessions)
        (2018, 12, 26, Closed, T1, "ASX24-CAL-2018"),
        // 2018-12-27 - T1 - ASX24-CAL-2018 - the operator closed the holiday evening's night session; the late open removes the wrapped leg and keeps the following day's own sessions
        (2018, 12, 27, ReplacementBlocks(&BLOCKS_05), T1, "ASX24-CAL-2018"),
        // 2018-12-31 - T1 - ASX24-CAL-2018 - Equity Products to close at 2:30pm and Night Session Closed: the day session closes 14:30 and the evening night session does not open
        (2018, 12, 31, early_close(14 * 3_600 + 30 * 60), T1, "ASX24-CAL-2018"),
        // 2019-01-01 - T1 - ASX24-CAL-2019 - operator Closed/Non Trading Day (day and night sessions)
        (2019, 1, 1, Closed, T1, "ASX24-CAL-2019"),
        // 2019-01-02 - T1 - ASX24-CAL-2019 - the operator closed the holiday evening's night session; the late open removes the wrapped leg and keeps the following day's own sessions
        (2019, 1, 2, ReplacementBlocks(&BLOCKS_05), T1, "ASX24-CAL-2019"),
        // 2019-01-28 - T1 - ASX24-CAL-2019 - Day Session Closed / Night Session to open at 5:10pm: the day session is removed and the morning night leg ends at its normal 08:00 close
        (2019, 1, 28, early_close(8 * 3_600 ), T1, "ASX24-CAL-2019"),
        // 2019-04-19 - T1 - ASX24-CAL-2019 - Day Before Good Friday: the night session opening on the eve ceased at midnight (operator row) and Good Friday itself is Closed/Non Trading Day, so the trade date runs that shortened evening leg and no day session
        (2019, 4, 19, ReplacementBlocks(&BLOCKS_02), T1, "ASX24-CAL-2019"),
        // 2019-04-20 - T1 - ASX24-CAL-2019 - the operator closed the Friday evening's night session, whose trade date is the Saturday
        (2019, 4, 20, Closed, T1, "ASX24-CAL-2019"),
        // 2019-04-22 - T1 - ASX24-CAL-2019 - operator Closed/Non Trading Day (day and night sessions)
        (2019, 4, 22, Closed, T1, "ASX24-CAL-2019"),
        // 2019-04-23 - T1 - ASX24-CAL-2019 - the operator closed the holiday evening's night session; the late open removes the wrapped leg and keeps the following day's own sessions
        (2019, 4, 23, ReplacementBlocks(&BLOCKS_05), T1, "ASX24-CAL-2019"),
        // 2019-04-25 - T1 - ASX24-CAL-2019 - Day Session Closed / Night Session to open at 5:10pm: the day session is removed and the morning night leg ends at its normal 07:00 close
        (2019, 4, 25, early_close(7 * 3_600 ), T1, "ASX24-CAL-2019"),
        // 2019-06-10 - T1 - ASX24-CAL-2019 - Day Session Closed / Night Session to open at 5:10pm: the day session is removed and the morning night leg ends at its normal 07:00 close
        (2019, 6, 10, early_close(7 * 3_600 ), T1, "ASX24-CAL-2019"),
        // 2019-12-24 - T1 - ASX24-CAL-2019 - Equity Products to close at 2:30pm and Night Session Closed: the day session closes 14:30 and the evening night session does not open
        (2019, 12, 24, early_close(14 * 3_600 + 30 * 60), T1, "ASX24-CAL-2019"),
        // 2019-12-25 - T1 - ASX24-CAL-2019 - operator Closed/Non Trading Day (day and night sessions)
        (2019, 12, 25, Closed, T1, "ASX24-CAL-2019"),
        // 2019-12-26 - T1 - ASX24-CAL-2019 - operator Closed/Non Trading Day (day and night sessions)
        (2019, 12, 26, Closed, T1, "ASX24-CAL-2019"),
        // 2019-12-27 - T1 - ASX24-CAL-2019 - the operator closed the holiday evening's night session; the late open removes the wrapped leg and keeps the following day's own sessions
        (2019, 12, 27, ReplacementBlocks(&BLOCKS_05), T1, "ASX24-CAL-2019"),
        // 2019-12-31 - T1 - ASX24-CAL-2019 - Equity Products to close at 2:30pm and Night Session Closed: the day session closes 14:30 and the evening night session does not open
        (2019, 12, 31, early_close(14 * 3_600 + 30 * 60), T1, "ASX24-CAL-2019"),
        // 2020-01-01 - T1 - ASX24-CAL-2020 - operator Closed/Non Trading Day (day and night sessions)
        (2020, 1, 1, Closed, T1, "ASX24-CAL-2020"),
        // 2020-01-02 - T1 - ASX24-CAL-2020 - the operator closed the holiday evening's night session; the late open removes the wrapped leg and keeps the following day's own sessions
        (2020, 1, 2, ReplacementBlocks(&BLOCKS_05), T1, "ASX24-CAL-2020"),
        // 2020-01-27 - T1 - ASX24-CAL-2020 - Day Session Closed / Night Session to open at 5:10pm: the day session is removed and the morning night leg ends at its normal 08:00 close
        (2020, 1, 27, early_close(8 * 3_600 ), T1, "ASX24-CAL-2020"),
        // 2020-04-10 - T1 - ASX24-CAL-2020 - Day Before Good Friday: the night session opening on the eve ceased at midnight (operator row) and Good Friday itself is Closed/Non Trading Day, so the trade date runs that shortened evening leg and no day session
        (2020, 4, 10, ReplacementBlocks(&BLOCKS_02), T1, "ASX24-CAL-2020"),
        // 2020-04-11 - T1 - ASX24-CAL-2020 - the operator closed the Friday evening's night session, whose trade date is the Saturday
        (2020, 4, 11, Closed, T1, "ASX24-CAL-2020"),
        // 2020-04-13 - T1 - ASX24-CAL-2020 - operator Closed/Non Trading Day (day and night sessions)
        (2020, 4, 13, Closed, T1, "ASX24-CAL-2020"),
        // 2020-04-14 - T1 - ASX24-CAL-2020 - the operator closed the holiday evening's night session; the late open removes the wrapped leg and keeps the following day's own sessions
        (2020, 4, 14, ReplacementBlocks(&BLOCKS_05), T1, "ASX24-CAL-2020"),
        // 2020-04-25 - T1 - ASX24-CAL-2020 - operator Closed/Non Trading Day (day and night sessions)
        (2020, 4, 25, Closed, T1, "ASX24-CAL-2020"),
        // 2020-06-08 - T1 - ASX24-CAL-2020 - Day Session Closed / Night Session to open at 5:10pm: the day session is removed and the morning night leg ends at its normal 07:00 close
        (2020, 6, 8, early_close(7 * 3_600 ), T1, "ASX24-CAL-2020"),
        // 2020-12-24 - T1 - ASX24-CAL-2020 - Equity Products to close at 2:30pm and Night Session Closed: the day session closes 14:30 and the evening night session does not open
        (2020, 12, 24, early_close(14 * 3_600 + 30 * 60), T1, "ASX24-CAL-2020"),
        // 2020-12-25 - T1 - ASX24-CAL-2020 - operator Closed/Non Trading Day (day and night sessions)
        (2020, 12, 25, Closed, T1, "ASX24-CAL-2020"),
        // 2020-12-26 - T1 - ASX24-CAL-2020 - the operator closed the Friday evening's night session, whose trade date is the Saturday
        (2020, 12, 26, Closed, T1, "ASX24-CAL-2020"),
        // 2020-12-28 - T1 - ASX24-CAL-2020 - operator Closed/Non Trading Day (day and night sessions)
        (2020, 12, 28, Closed, T1, "ASX24-CAL-2020"),
        // 2020-12-29 - T1 - ASX24-CAL-2020 - the operator closed the holiday evening's night session; the late open removes the wrapped leg and keeps the following day's own sessions
        (2020, 12, 29, ReplacementBlocks(&BLOCKS_05), T1, "ASX24-CAL-2020"),
        // 2020-12-31 - T1 - ASX24-CAL-2020 - Equity Products to close at 2:30pm and Night Session Closed: the day session closes 14:30 and the evening night session does not open
        (2020, 12, 31, early_close(14 * 3_600 + 30 * 60), T1, "ASX24-CAL-2020"),
        // 2021-01-01 - T1 - ASX24-LIVE-2021-01 - operator Closed/Non Trading Day (day and night sessions)
        (2021, 1, 1, Closed, T1, "ASX24-LIVE-2021-01"),
        // 2021-01-02 - T1 - ASX24-LIVE-2021-01 - the operator closed the Friday evening's night session, whose trade date is the Saturday
        (2021, 1, 2, Closed, T1, "ASX24-LIVE-2021-01"),
        // 2021-01-26 - T1 - ASX24-LIVE-2021-01 - Day Session Closed / Night Session to open at 5:10pm: the day session is removed and the morning night leg ends at its normal 08:00 close
        (2021, 1, 26, early_close(8 * 3_600 ), T1, "ASX24-LIVE-2021-01"),
        // 2021-04-02 - T1 - ASX24-LIVE-2021-01 - Day Before Good Friday: the night session opening on the eve ceased at midnight (operator row) and Good Friday itself is Closed/Non Trading Day, so the trade date runs that shortened evening leg and no day session
        (2021, 4, 2, ReplacementBlocks(&BLOCKS_02), T1, "ASX24-LIVE-2021-01"),
        // 2021-04-03 - T1 - ASX24-LIVE-2021-01 - the operator closed the Friday evening's night session, whose trade date is the Saturday
        (2021, 4, 3, Closed, T1, "ASX24-LIVE-2021-01"),
        // 2021-04-05 - T1 - ASX24-LIVE-2021-01 - operator Closed/Non Trading Day (day and night sessions)
        (2021, 4, 5, Closed, T1, "ASX24-LIVE-2021-01"),
        // 2021-04-06 - T1 - ASX24-LIVE-2021-01 - the operator closed the holiday evening's night session; the late open removes the wrapped leg and keeps the following day's own sessions
        (2021, 4, 6, ReplacementBlocks(&BLOCKS_05), T1, "ASX24-LIVE-2021-01"),
        // 2021-04-25 - T1 - ASX24-LIVE-2021-01 - operator Closed/Non Trading Day (day and night sessions)
        (2021, 4, 25, Closed, T1, "ASX24-LIVE-2021-01"),
        // 2021-04-26 - T1 - ASX24-LIVE-2021-01 - the operator closed the Sunday evening's night session; the late open removes the wrapped leg and keeps Monday's own sessions
        (2021, 4, 26, ReplacementBlocks(&BLOCKS_05), T1, "ASX24-LIVE-2021-01"),
        // 2021-06-14 - T1 - ASX24-LIVE-2021-01 - Day Session Closed / Night Session to open at 5:10pm: the day session is removed and the morning night leg ends at its normal 07:00 close
        (2021, 6, 14, early_close(7 * 3_600 ), T1, "ASX24-LIVE-2021-01"),
        // 2021-12-24 - T1 - ASX24-LIVE-2021-01 - Equity Products to close at 2:30pm and Night Session Closed: the day session closes 14:30 and the evening night session does not open
        (2021, 12, 24, early_close(14 * 3_600 + 30 * 60), T1, "ASX24-LIVE-2021-01"),
        // 2021-12-25 - T1 - ASX24-LIVE-2021-01 - the closed Friday night session's trade date is the Saturday
        (2021, 12, 25, Closed, T1, "ASX24-LIVE-2021-01"),
        // 2021-12-27 - T1 - ASX24-LIVE-2021-01 - operator Closed/Non Trading Day (day and night sessions)
        (2021, 12, 27, Closed, T1, "ASX24-LIVE-2021-01"),
        // 2021-12-28 - T1 - ASX24-LIVE-2021-01 - operator Closed/Non Trading Day (day and night sessions)
        (2021, 12, 28, Closed, T1, "ASX24-LIVE-2021-01"),
        // 2021-12-29 - T1 - ASX24-LIVE-2021-01 - the operator closed the holiday evening's night session; the late open removes the wrapped leg and keeps the following day's own sessions
        (2021, 12, 29, ReplacementBlocks(&BLOCKS_05), T1, "ASX24-LIVE-2021-01"),
        // 2021-12-31 - T1 - ASX24-LIVE-2021-01 - Equity Products to close at 2:30pm and Night Session Closed: the day session closes 14:30 and the evening night session does not open
        (2021, 12, 31, early_close(14 * 3_600 + 30 * 60), T1, "ASX24-LIVE-2021-01"),
        // 2022-01-01 - T1 - ASX24-LIVE-2021-01 - the closed Friday night session's trade date is the Saturday
        (2022, 1, 1, Closed, T1, "ASX24-LIVE-2021-01"),
        // 2022-01-03 - T1 - ASX24-LIVE-2022-12 - operator Closed/Non Trading Day (day and night sessions)
        (2022, 1, 3, Closed, T1, "ASX24-LIVE-2022-12"),
        // 2022-01-04 - T1 - ASX24-LIVE-2022-12 - the operator closed the holiday evening's night session; the late open removes the wrapped leg and keeps the following day's own sessions
        (2022, 1, 4, ReplacementBlocks(&BLOCKS_05), T1, "ASX24-LIVE-2022-12"),
        // 2022-01-26 - T1 - ASX24-LIVE-2022-12 - Day Session Closed / Night Session to open at 5:10pm: the day session is removed and the morning night leg ends at its normal 08:00 close
        (2022, 1, 26, early_close(8 * 3_600 ), T1, "ASX24-LIVE-2022-12"),
        // 2022-04-15 - T1 - ASX24-LIVE-2022-12 - Day Before Good Friday: the night session opening on the eve ceased at midnight (operator row) and Good Friday itself is Closed/Non Trading Day, so the trade date runs that shortened evening leg and no day session
        (2022, 4, 15, ReplacementBlocks(&BLOCKS_02), T1, "ASX24-LIVE-2022-12"),
        // 2022-04-16 - T1 - ASX24-LIVE-2022-12 - the operator closed the Friday evening's night session, whose trade date is the Saturday
        (2022, 4, 16, Closed, T1, "ASX24-LIVE-2022-12"),
        // 2022-04-18 - T1 - ASX24-LIVE-2022-12 - operator Closed/Non Trading Day (day and night sessions)
        (2022, 4, 18, Closed, T1, "ASX24-LIVE-2022-12"),
        // 2022-04-19 - T1 - ASX24-LIVE-2022-12 - the operator closed the holiday evening's night session; the late open removes the wrapped leg and keeps the following day's own sessions
        (2022, 4, 19, ReplacementBlocks(&BLOCKS_05), T1, "ASX24-LIVE-2022-12"),
        // 2022-04-25 - T1 - ASX24-LIVE-2022-12 - Day Session Closed / Night Session to open at 5:10pm: the day session is removed and the morning night leg ends at its normal 07:00 close
        (2022, 4, 25, early_close(7 * 3_600 ), T1, "ASX24-LIVE-2022-12"),
        // 2022-06-13 - T1 - ASX24-LIVE-2022-12 - Day Session Closed / Night Session to open at 5:10pm: the day session is removed and the morning night leg ends at its normal 07:00 close
        (2022, 6, 13, early_close(7 * 3_600 ), T1, "ASX24-LIVE-2022-12"),
        // 2022-09-22 - T1 - ASX24-LIVE-2022-12 - Day Session Closed / Night Session to open at 5:10pm: the day session is removed and the morning night leg ends at its normal 07:00 close
        (2022, 9, 22, early_close(7 * 3_600 ), T1, "ASX24-LIVE-2022-12"),
        // 2022-12-24 - T1 - ASX24-LIVE-2022-12 - Day Session Normal / Night Session Closed: the closed Friday night session's trade date is the Saturday
        (2022, 12, 24, Closed, T1, "ASX24-LIVE-2022-12"),
        // 2022-12-26 - T1 - ASX24-LIVE-2022-12 - operator Closed/Non Trading Day (day and night sessions)
        (2022, 12, 26, Closed, T1, "ASX24-LIVE-2022-12"),
        // 2022-12-27 - T1 - ASX24-LIVE-2022-12 - operator Closed/Non Trading Day (day and night sessions)
        (2022, 12, 27, Closed, T1, "ASX24-LIVE-2022-12"),
        // 2022-12-28 - T1 - ASX24-LIVE-2022-12 - the operator closed the holiday evening's night session; the late open removes the wrapped leg and keeps the following day's own sessions
        (2022, 12, 28, ReplacementBlocks(&BLOCKS_05), T1, "ASX24-LIVE-2022-12"),
        // 2022-12-31 - T1 - ASX24-LIVE-2022-12 - Day Session Normal / Night Session Closed: the closed Friday night session's trade date is the Saturday
        (2022, 12, 31, Closed, T1, "ASX24-LIVE-2022-12"),
        // 2023-01-02 - T1 - ASX24-LIVE-2022-12 - operator Closed/Non Trading Day (day and night sessions)
        (2023, 1, 2, Closed, T1, "ASX24-LIVE-2022-12"),
        // 2023-01-03 - T1 - ASX24-LIVE-2022-12 - the operator closed the holiday evening's night session; the late open removes the wrapped leg and keeps the following day's own sessions
        (2023, 1, 3, ReplacementBlocks(&BLOCKS_05), T1, "ASX24-LIVE-2022-12"),
        // 2023-01-26 - T1 - ASX24-LIVE-2022-12 - Day Session Closed / Night Session to open at 5:10pm: the day session is removed and the morning night leg ends at its normal 08:00 close
        (2023, 1, 26, early_close(8 * 3_600 ), T1, "ASX24-LIVE-2022-12"),
        // 2023-04-07 - T1 - ASX24-LIVE-2022-12 - Day Before Good Friday: the night session opening on the eve ceased at midnight (operator row) and Good Friday itself is Closed/Non Trading Day, so the trade date runs that shortened evening leg and no day session
        (2023, 4, 7, ReplacementBlocks(&BLOCKS_02), T1, "ASX24-LIVE-2022-12"),
        // 2023-04-08 - T1 - ASX24-LIVE-2022-12 - the operator closed the Friday evening's night session, whose trade date is the Saturday
        (2023, 4, 8, Closed, T1, "ASX24-LIVE-2022-12"),
        // 2023-04-10 - T1 - ASX24-LIVE-2022-12 - operator Closed/Non Trading Day (day and night sessions)
        (2023, 4, 10, Closed, T1, "ASX24-LIVE-2022-12"),
        // 2023-04-11 - T1 - ASX24-LIVE-2022-12 - the operator closed the holiday evening's night session; the late open removes the wrapped leg and keeps the following day's own sessions
        (2023, 4, 11, ReplacementBlocks(&BLOCKS_05), T1, "ASX24-LIVE-2022-12"),
        // 2023-04-25 - T1 - ASX24-LIVE-2022-12 - Day Session Closed / Night Session to open at 5:10pm: the day session is removed and the morning night leg ends at its normal 07:00 close
        (2023, 4, 25, early_close(7 * 3_600 ), T1, "ASX24-LIVE-2022-12"),
        // 2023-06-12 - T1 - ASX24-LIVE-2022-12 - Day Session Closed / Night Session to open at 5:10pm: the day session is removed and the morning night leg ends at its normal 07:00 close
        (2023, 6, 12, early_close(7 * 3_600 ), T1, "ASX24-LIVE-2022-12"),
        // 2023-12-23 - T1 - ASX24-LIVE-2022-12 - Day Session Normal / Night Session Closed: the closed Friday night session's trade date is the Saturday
        (2023, 12, 23, Closed, T1, "ASX24-LIVE-2022-12"),
        // 2023-12-25 - T1 - ASX24-LIVE-2022-12 - operator Closed/Non Trading Day (day and night sessions)
        (2023, 12, 25, Closed, T1, "ASX24-LIVE-2022-12"),
        // 2023-12-26 - T1 - ASX24-LIVE-2022-12 - operator Closed/Non Trading Day (day and night sessions)
        (2023, 12, 26, Closed, T1, "ASX24-LIVE-2022-12"),
        // 2023-12-27 - T1 - ASX24-LIVE-2022-12 - the operator closed the holiday evening's night session; the late open removes the wrapped leg and keeps the following day's own sessions
        (2023, 12, 27, ReplacementBlocks(&BLOCKS_05), T1, "ASX24-LIVE-2022-12"),
        // 2023-12-30 - T1 - ASX24-LIVE-2022-12 - Day Session Normal / Night Session Closed: the closed Friday night session's trade date is the Saturday
        (2023, 12, 30, Closed, T1, "ASX24-LIVE-2022-12"),
        // 2024-01-01 - T1 - ASX24-LIVE-2023-12 - operator Closed/Non Trading Day (day and night sessions)
        (2024, 1, 1, Closed, T1, "ASX24-LIVE-2023-12"),
        // 2024-01-02 - T1 - ASX24-LIVE-2023-12 - the operator closed the holiday evening's night session; the late open removes the wrapped leg and keeps the following day's own sessions
        (2024, 1, 2, ReplacementBlocks(&BLOCKS_05), T1, "ASX24-LIVE-2023-12"),
        // 2024-01-26 - T1 - ASX24-LIVE-2023-12 - Day Session Closed / Night Session to open at 5:10pm: the day session is removed and the morning night leg ends at its normal 08:00 close
        (2024, 1, 26, early_close(8 * 3_600 ), T1, "ASX24-LIVE-2023-12"),
        // 2024-03-29 - T1 - ASX24-LIVE-2023-12 - Day Before Good Friday: the night session opening on the eve ceased at midnight (operator row) and Good Friday itself is Closed/Non Trading Day, so the trade date runs that shortened evening leg and no day session
        (2024, 3, 29, ReplacementBlocks(&BLOCKS_02), T1, "ASX24-LIVE-2023-12"),
        // 2024-03-30 - T1 - ASX24-LIVE-2023-12 - the operator closed the Friday evening's night session, whose trade date is the Saturday
        (2024, 3, 30, Closed, T1, "ASX24-LIVE-2023-12"),
        // 2024-04-01 - T1 - ASX24-LIVE-2023-12 - operator Closed/Non Trading Day (day and night sessions)
        (2024, 4, 1, Closed, T1, "ASX24-LIVE-2023-12"),
        // 2024-04-02 - T1 - ASX24-LIVE-2023-12 - the operator closed the holiday evening's night session; the late open removes the wrapped leg and keeps the following day's own sessions
        (2024, 4, 2, ReplacementBlocks(&BLOCKS_05), T1, "ASX24-LIVE-2023-12"),
        // 2024-04-25 - T1 - ASX24-LIVE-2023-12 - Day Session Closed / Night Session to open at 5:10pm: the day session is removed and the morning night leg ends at its normal 07:00 close
        (2024, 4, 25, early_close(7 * 3_600 ), T1, "ASX24-LIVE-2023-12"),
        // 2024-06-10 - T1 - ASX24-LIVE-2023-12 - Day Session Closed / Night Session to open at 5:10pm: the day session is removed and the morning night leg ends at its normal 07:00 close
        (2024, 6, 10, early_close(7 * 3_600 ), T1, "ASX24-LIVE-2023-12"),
        // 2024-12-25 - T1 - ASX24-LIVE-2023-12 - operator Closed/Non Trading Day (day and night sessions)
        (2024, 12, 25, Closed, T1, "ASX24-LIVE-2023-12"),
        // 2024-12-26 - T1 - ASX24-LIVE-2023-12 - operator Closed/Non Trading Day (day and night sessions)
        (2024, 12, 26, Closed, T1, "ASX24-LIVE-2023-12"),
        // 2024-12-27 - T1 - ASX24-LIVE-2023-12 - the operator closed the holiday evening's night session; the late open removes the wrapped leg and keeps the following day's own sessions
        (2024, 12, 27, ReplacementBlocks(&BLOCKS_05), T1, "ASX24-LIVE-2023-12"),
        // 2025-01-01 - T1 - ASX24-LIVE-2025-03 - operator Closed/Non Trading Day (day and night sessions)
        (2025, 1, 1, Closed, T1, "ASX24-LIVE-2025-03"),
        // 2025-01-02 - T1 - ASX24-LIVE-2025-03 - the operator closed the holiday evening's night session; the late open removes the wrapped leg and keeps the following day's own sessions
        (2025, 1, 2, ReplacementBlocks(&BLOCKS_05), T1, "ASX24-LIVE-2025-03"),
        // 2025-01-27 - T1 - ASX24-LIVE-2025-03 - Day Session Closed / Night Session to open at 5:10pm: the day session is removed and the morning night leg ends at its normal 08:00 close
        (2025, 1, 27, early_close(8 * 3_600 ), T1, "ASX24-LIVE-2025-03"),
        // 2025-04-18 - T1 - ASX24-LIVE-2025-03 - Day Before Good Friday: the night session opening on the eve ceased at midnight (operator row) and Good Friday itself is Closed/Non Trading Day, so the trade date runs that shortened evening leg and no day session
        (2025, 4, 18, ReplacementBlocks(&BLOCKS_02), T1, "ASX24-LIVE-2025-03"),
        // 2025-04-19 - T1 - ASX24-LIVE-2025-03 - the operator closed the Friday evening's night session, whose trade date is the Saturday
        (2025, 4, 19, Closed, T1, "ASX24-LIVE-2025-03"),
        // 2025-04-21 - T1 - ASX24-LIVE-2025-03 - operator Closed/Non Trading Day (day and night sessions)
        (2025, 4, 21, Closed, T1, "ASX24-LIVE-2025-03"),
        // 2025-04-22 - T1 - ASX24-LIVE-2025-03 - the operator closed the holiday evening's night session; the late open removes the wrapped leg and keeps the following day's own sessions
        (2025, 4, 22, ReplacementBlocks(&BLOCKS_05), T1, "ASX24-LIVE-2025-03"),
        // 2025-04-25 - T1 - ASX24-LIVE-2025-03 - Day Session Closed / Night Session to open at 5:10pm: the day session is removed and the morning night leg ends at its normal 07:00 close
        (2025, 4, 25, early_close(7 * 3_600 ), T1, "ASX24-LIVE-2025-03"),
        // 2025-06-09 - T1 - ASX24-LIVE-2025-03 - Day Session Closed / Night Session to open at 5:10pm: the day session is removed and the morning night leg ends at its normal 07:00 close
        (2025, 6, 9, early_close(7 * 3_600 ), T1, "ASX24-LIVE-2025-03"),
        // 2025-12-24 - T1 - ASX24-LIVE-2025-03 - Equity Products to close at 2:30pm and Night Session Closed: the day session closes 14:30 and the evening night session does not open
        (2025, 12, 24, early_close(14 * 3_600 + 30 * 60), T1, "ASX24-LIVE-2025-03"),
        // 2025-12-25 - T1 - ASX24-LIVE-2025-03 - operator Closed/Non Trading Day (day and night sessions)
        (2025, 12, 25, Closed, T1, "ASX24-LIVE-2025-03"),
        // 2025-12-26 - T1 - ASX24-LIVE-2025-03 - operator Closed/Non Trading Day (day and night sessions)
        (2025, 12, 26, Closed, T1, "ASX24-LIVE-2025-03"),
        // 2025-12-27 - T1 - ASX24-LIVE-2025-03 - the operator closed the Friday evening's night session, whose trade date is the Saturday
        (2025, 12, 27, Closed, T1, "ASX24-LIVE-2025-03"),
        // 2025-12-31 - T1 - ASX24-LIVE-2025-03 - Equity Products to close at 2:30pm and Night Session Closed: the day session closes 14:30 and the evening night session does not open
        (2025, 12, 31, early_close(14 * 3_600 + 30 * 60), T1, "ASX24-LIVE-2025-03"),
        // 2026-01-01 - T1 - ASX24-LIVE-2026-10 - operator Closed/Non Trading Day (day and night sessions)
        (2026, 1, 1, Closed, T1, "ASX24-LIVE-2026-10"),
        // 2026-01-02 - T1 - ASX24-LIVE-2026-10 - the operator closed the holiday evening's night session; the late open removes the wrapped leg and keeps the following day's own sessions
        (2026, 1, 2, ReplacementBlocks(&BLOCKS_05), T1, "ASX24-LIVE-2026-10"),
        // 2026-01-26 - T1 - ASX24-LIVE-2026-10 - Day Session Closed / Night Session to open at 5:10pm: the day session is removed and the morning night leg ends at its normal 08:00 close
        (2026, 1, 26, early_close(8 * 3_600 ), T1, "ASX24-LIVE-2026-10"),
        // 2026-04-03 - T1 - ASX24-LIVE-2026-10 - Day Before Good Friday: the night session opening on the eve ceased at midnight (operator row) and Good Friday itself is Closed/Non Trading Day, so the trade date runs that shortened evening leg and no day session
        (2026, 4, 3, ReplacementBlocks(&BLOCKS_02), T1, "ASX24-LIVE-2026-10"),
        // 2026-04-04 - T1 - ASX24-LIVE-2026-10 - the operator closed the Friday evening's night session, whose trade date is the Saturday
        (2026, 4, 4, Closed, T1, "ASX24-LIVE-2026-10"),
        // 2026-04-06 - T1 - ASX24-LIVE-2026-10 - operator Closed/Non Trading Day (day and night sessions)
        (2026, 4, 6, Closed, T1, "ASX24-LIVE-2026-10"),
        // 2026-04-07 - T1 - ASX24-LIVE-2026-10 - the operator closed the holiday evening's night session; the late open removes the wrapped leg and keeps the following day's own sessions
        (2026, 4, 7, ReplacementBlocks(&BLOCKS_05), T1, "ASX24-LIVE-2026-10"),
        // 2026-06-08 - T1 - ASX24-LIVE-2026-10 - Day Session Closed / Night Session to open at 5:10pm: the day session is removed and the morning night leg ends at its normal 07:00 close
        (2026, 6, 8, early_close(7 * 3_600 ), T1, "ASX24-LIVE-2026-10"),
        // 2026-12-24 - T1 - ASX24-LIVE-2026-10 - Equity Products to close at 2:30pm and Night Session Closed: the day session closes 14:30 and the evening night session does not open
        (2026, 12, 24, early_close(14 * 3_600 + 30 * 60), T1, "ASX24-LIVE-2026-10"),
        // 2026-12-25 - T1 - ASX24-LIVE-2026-10 - operator Closed/Non Trading Day (day and night sessions)
        (2026, 12, 25, Closed, T1, "ASX24-LIVE-2026-10"),
        // 2026-12-26 - T1 - ASX24-LIVE-2026-10 - the operator closed the Friday evening's night session, whose trade date is the Saturday
        (2026, 12, 26, Closed, T1, "ASX24-LIVE-2026-10"),
        // 2026-12-28 - T1 - ASX24-LIVE-2026-10 - operator Closed/Non Trading Day (day and night sessions)
        (2026, 12, 28, Closed, T1, "ASX24-LIVE-2026-10"),
        // 2026-12-29 - T1 - ASX24-LIVE-2026-10 - the operator closed the holiday evening's night session; the late open removes the wrapped leg and keeps the following day's own sessions
        (2026, 12, 29, ReplacementBlocks(&BLOCKS_05), T1, "ASX24-LIVE-2026-10"),
        // 2026-12-31 - T1 - ASX24-LIVE-2026-10 - Equity Products to close at 2:30pm and Night Session Closed: the day session closes 14:30 and the evening night session does not open
        (2026, 12, 31, early_close(14 * 3_600 + 30 * 60), T1, "ASX24-LIVE-2026-10"),
        // 2027-01-01 - T1 - ASX24-LIVE-2026-10 - operator Closed/Non Trading Day (day and night sessions)
        (2027, 1, 1, Closed, T1, "ASX24-LIVE-2026-10"),
        // 2027-01-02 - T1 - ASX24-LIVE-2026-10 - the operator closed the Friday evening's night session, whose trade date is the Saturday
        (2027, 1, 2, Closed, T1, "ASX24-LIVE-2026-10"),
        // 2027-01-26 - T1 - ASX24-LIVE-2026-10 - Day Session Closed / Night Session to open at 5:10pm: the day session is removed and the morning night leg ends at its normal 08:00 close
        (2027, 1, 26, early_close(8 * 3_600 ), T1, "ASX24-LIVE-2026-10"),
        // 2027-03-26 - T1 - ASX24-LIVE-2026-10 - Day Before Good Friday: the night session opening on the eve ceased at midnight (operator row) and Good Friday itself is Closed/Non Trading Day, so the trade date runs that shortened evening leg and no day session
        (2027, 3, 26, ReplacementBlocks(&BLOCKS_02), T1, "ASX24-LIVE-2026-10"),
        // 2027-03-27 - T1 - ASX24-LIVE-2026-10 - the operator closed the Friday evening's night session, whose trade date is the Saturday
        (2027, 3, 27, Closed, T1, "ASX24-LIVE-2026-10"),
        // 2027-03-29 - T1 - ASX24-LIVE-2026-10 - operator Closed/Non Trading Day (day and night sessions)
        (2027, 3, 29, Closed, T1, "ASX24-LIVE-2026-10"),
        // 2027-03-30 - T1 - ASX24-LIVE-2026-10 - the operator closed the holiday evening's night session; the late open removes the wrapped leg and keeps the following day's own sessions
        (2027, 3, 30, ReplacementBlocks(&BLOCKS_05), T1, "ASX24-LIVE-2026-10"),
        // 2027-06-14 - T1 - ASX24-LIVE-2026-10 - Day Session Closed / Night Session to open at 5:10pm: the day session is removed and the morning night leg ends at its normal 07:00 close
        (2027, 6, 14, early_close(7 * 3_600 ), T1, "ASX24-LIVE-2026-10"),
        // 2027-12-24 - T1 - ASX24-LIVE-2026-10 - Equity Products to close at 2:30pm and Night Session Closed: the day session closes 14:30 and the evening night session does not open
        (2027, 12, 24, early_close(14 * 3_600 + 30 * 60), T1, "ASX24-LIVE-2026-10"),
        // 2027-12-25 - T1 - ASX24-LIVE-2026-10 - the closed Friday night session's trade date is the Saturday
        (2027, 12, 25, Closed, T1, "ASX24-LIVE-2026-10"),
        // 2027-12-27 - T1 - ASX24-LIVE-2026-10 - operator Closed/Non Trading Day (day and night sessions)
        (2027, 12, 27, Closed, T1, "ASX24-LIVE-2026-10"),
        // 2027-12-28 - T1 - ASX24-LIVE-2026-10 - operator Closed/Non Trading Day (day and night sessions)
        (2027, 12, 28, Closed, T1, "ASX24-LIVE-2026-10"),
        // 2027-12-29 - T1 - ASX24-LIVE-2026-10 - the operator closed the holiday evening's night session; the late open removes the wrapped leg and keeps the following day's own sessions
        (2027, 12, 29, ReplacementBlocks(&BLOCKS_05), T1, "ASX24-LIVE-2026-10"),
        // 2027-12-31 - T1 - ASX24-LIVE-2026-10 - Equity Products to close at 2:30pm and Night Session Closed: the day session closes 14:30 and the evening night session does not open
        (2027, 12, 31, early_close(14 * 3_600 + 30 * 60), T1, "ASX24-LIVE-2026-10"),
        // 2028-01-01 - T1 - ASX24-LIVE-2026-10 - the operator closed the 31 December 2027 evening night session, whose trade date is 1 January 2028 (audited from the 2027 calendar)
        (2028, 1, 1, Closed, T1, "ASX24-LIVE-2026-10"),
    ],
};
