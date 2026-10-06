// SPDX-License-Identifier: MIT-0

//! National Stock Exchange of India holiday rows, 2010-2026.
//!
//! Keyed by the crate's own venue-local trade date in `Asia/Kolkata` (design
//! memo D1). NSE runs no overnight wrap and no weekend sessions, so a closed
//! trade date is one whose daytime sessions are absent outright.
//!
//! The block is the operator's own published trading-holiday material at T1
//! or T2: the annual circulars ("Trading holidays for the calendar year ...",
//! 2010 and 2013), the annual trading-holiday pages on `nseindia.com`
//! (2011, 2014-2017, 2019-2022), and the operator's own
//! `holiday-master?type=trading` machine channel (2023 and 2024; T2) — all
//! read from the Wayback `id_` replays the live site's bot wall makes
//! necessary. Each source's CM/equities list prints one row per holiday and
//! separately lists the holidays falling on Saturday/Sunday; only the weekday
//! legs ship rows, exactly as with the SSE range legs, and a Muhurat-Trading
//! footnote keyed to a specific date withholds that date below. The
//! derivation and per-row quotations are recorded in
//! [`docs/evidence/nse_india.md`](../../../../../docs/evidence/nse_india.md).
//!
//! **2012 and 2018 sit outside every audited window.** No operator artifact
//! stating either year's CM holiday list survives in the archive (the
//! annual-page URL has no capture in either year, the old pages the archive
//! does hold from those eras are query forms or other segments, and the
//! annual circulars are uncaptured for those years), so the coverage windows
//! stop at 2011-12-31, 2017-12-31 and resume 2019-01-01; a date in 2012 or
//! 2018 refuses rather than answers (see the evidence file's gaps).
//!
//! Nine rows are **`Unsourced` rather than closures** (2010-11-05, 2011-10-26,
//! 2013-11-03, 2014-10-23, 2015-11-11, 2016-10-30, 2017-10-19, 2019-10-27 and
//! 2026-11-08): each source that prints a Muhurat-Trading footnote ("*Muhurat
//! Trading will be conducted", timings "shall be notified subsequently") keys
//! it to a specific date the row set would otherwise misstate — a printed
//! holiday a special session interrupts, or a weekend the operator announces a
//! session on. **Five further Muhurat dates ship `ReplacementBlocks`** (2020-
//! 11-14, 2022-10-24, 2023-11-12, 2024-11-01, 2025-10-21): the operator's own
//! capital-market circulars recovered 2026-10-06 UTC from the Wayback archive
//! state each day's complete special-session schedule, and the rows restate
//! those printed schedules. 2021 is the exception in the other direction: its
//! list prints the Muhurat footnote against no date at all, so no day is
//! withheld and the announced-but-undated session stays a recorded residual
//! risk. NSE publishes the next year's list each December; no 2027 list exists
//! as of the 2026-09-29 retrieval, so the window ends 2026-12-31.

use crate::calendar::exceptions::ExceptionBlock;

use super::EvidenceTier::{T1, T2};
use super::HolidayKind::{Closed, ReplacementBlocks, Unsourced};
use super::{HolidayTable, holidays};

/// The operator's printed Diwali Muhurat evening session of 2020, 2022 and
/// 2023, relative to that trade date.
///
/// Read from the operator's own capital-market circulars (`NSE-CIRC-2020-98`,
/// `NSE-CIRC-2022-124` and `NSE-CIRC-2023-139`, whose schedules are
/// identical): Block deal 17:45-18:00 (a trade prints), Pre Open 18:00-18:08
/// with the random closure "in last one minute" of that order-entry period,
/// Normal Market 18:15-19:15, and a Closing Session 19:25-19:35. The
/// order-entry leg stops at 18:07, the earliest second the auction's random
/// closure can fire, exactly as the normal-week profile splits its 09:00
/// pre-open at 09:07; the leg from the closure to the 18:15 Normal Market
/// open is tradeable and ships `extended`. The circular's Trade Modification
/// cut-off (18:15-19:45) is a modification window, not a session, and the
/// Call Auction Illiquid sub-window (18:20-19:05) runs inside the Normal
/// Market, so neither states an envelope edge.
///
/// Evidence: `docs/evidence/nse_india.md`.
#[rustfmt::skip]
static MUHURAT_EVENING_BLOCKS: [ExceptionBlock; 5] = [
    ExceptionBlock::extended(0, 17 * 3_600 + 45 * 60, 18 * 3_600),
    ExceptionBlock::order_entry(0, 18 * 3_600, 18 * 3_600 + 7 * 60),
    ExceptionBlock::extended(0, 18 * 3_600 + 7 * 60, 18 * 3_600 + 15 * 60),
    ExceptionBlock::regular(0, 18 * 3_600 + 15 * 60, 19 * 3_600 + 15 * 60),
    ExceptionBlock::extended(0, 19 * 3_600 + 25 * 60, 19 * 3_600 + 35 * 60),
];

/// The operator's printed Diwali Muhurat session of 2024-11-01, relative to
/// that trade date.
///
/// Read from the operator's own capital-market circular `NSE-CIRC-2024-147`:
/// Block deal 17:30-17:45, Pre Open 17:45-18:00 — its footnote bounds the
/// order-entry period at "17:45 to 17:53 hours" with the random closure "in
/// last one minute" of it, so the order-entry leg stops at 17:52, the
/// earliest second the closure can fire — Normal Market 18:00-19:00, and a
/// Closing Session 19:10-19:20. The Special Pre-open Session for IPO and
/// relisted scrips (17:45-18:30, opening 18:45-19:00) and the Call Auction
/// Illiquid sub-window (18:05-18:50) are sub-segment phases inside that
/// envelope and state no edge the envelope lacks.
///
/// Evidence: `docs/evidence/nse_india.md`.
#[rustfmt::skip]
static MUHURAT_2024_BLOCKS: [ExceptionBlock; 5] = [
    ExceptionBlock::extended(0, 17 * 3_600 + 30 * 60, 17 * 3_600 + 45 * 60),
    ExceptionBlock::order_entry(0, 17 * 3_600 + 45 * 60, 17 * 3_600 + 52 * 60),
    ExceptionBlock::extended(0, 17 * 3_600 + 52 * 60, 18 * 3_600),
    ExceptionBlock::regular(0, 18 * 3_600, 19 * 3_600),
    ExceptionBlock::extended(0, 19 * 3_600 + 10 * 60, 19 * 3_600 + 20 * 60),
];

/// The operator's printed Diwali Muhurat session of 2025-10-21, relative to
/// that trade date.
///
/// Read from the operator's own capital-market circular `NSE-CIRC-2025-124`:
/// Block deal 13:15-13:30, Pre Open 13:30-13:45 — its footnote states the
/// random closure "any time between last one minute i.e. 13:37 to 13:38
/// hours", so the order-entry leg stops at 13:37 — Normal Market 13:45-14:45,
/// and a Closing Session 14:55-15:05. The Special Pre-open Session for IPO
/// and relisted scrips (13:30-14:15, opening 14:30-14:45) and the Call
/// Auction Illiquid sub-window (13:50-14:35) are sub-segment phases inside
/// that envelope and state no edge the envelope lacks.
///
/// Evidence: `docs/evidence/nse_india.md`.
#[rustfmt::skip]
static MUHURAT_2025_BLOCKS: [ExceptionBlock; 5] = [
    ExceptionBlock::extended(0, 13 * 3_600 + 15 * 60, 13 * 3_600 + 30 * 60),
    ExceptionBlock::order_entry(0, 13 * 3_600 + 30 * 60, 13 * 3_600 + 37 * 60),
    ExceptionBlock::extended(0, 13 * 3_600 + 37 * 60, 13 * 3_600 + 45 * 60),
    ExceptionBlock::regular(0, 13 * 3_600 + 45 * 60, 14 * 3_600 + 45 * 60),
    ExceptionBlock::extended(0, 14 * 3_600 + 55 * 60, 15 * 3_600 + 5 * 60),
];

/// NSE's built-in holiday rows and the windows they were audited over.
///
/// Every `Closed` row is one printed date of the operator's own holiday
/// material: `NSE-CIRC-2010-61` for 2010, `NSE-HOL-PAGE-2011` for 2011,
/// `NSE-CIRC-2012-79` for 2013, `NSE-HOL-PAGE-2014`..`NSE-HOL-PAGE-2017` and
/// `NSE-HOL-PAGE-2019`..`NSE-HOL-PAGE-2022` for their years, the T2
/// `NSE-HOLMASTER-2023` and `NSE-HOLMASTER-2024` feeds for 2023-2024, then
/// `NSE-HOL-2025` for 2025 and `NSE-HOL-2026` for 2026. The five Muhurat
/// replacement days cite the operator's own circulars
/// `NSE-CIRC-2020-98`, `NSE-CIRC-2022-124`, `NSE-CIRC-2023-139`,
/// `NSE-CIRC-2024-147` and `NSE-CIRC-2025-124`. A date inside a window with
/// no row is audited normal; the nine `Unsourced` Muhurat dates are not, and
/// 2012 and 2018 are inside no window at all.
// Evidence: docs/evidence/nse_india.md
pub(crate) static TABLE: &HolidayTable = holidays! {
    coverage: [
        (2010, 1, 1) ..= (2011, 12, 31),
        (2013, 1, 1) ..= (2017, 12, 31),
        (2019, 1, 1) ..= (2026, 12, 31),
    ],
    rows: [
        // 2010, from the operator's circular 061 ("Trading holidays for the
        // calendar year 2010", HOLIDAYS.zip inner eq_holidays.pdf).
        // 2010-01-01 - T1 - NSE-CIRC-2010-61 - New Year, Friday.
        (2010, 1, 1, Closed, T1, "NSE-CIRC-2010-61"),
        // 2010-01-26 - T1 - NSE-CIRC-2010-61 - Republic Day, Tuesday.
        (2010, 1, 26, Closed, T1, "NSE-CIRC-2010-61"),
        // 2010-02-12 - T1 - NSE-CIRC-2010-61 - Mahashivratri, Friday.
        (2010, 2, 12, Closed, T1, "NSE-CIRC-2010-61"),
        // 2010-03-01 - T1 - NSE-CIRC-2010-61 - Holi, Monday.
        (2010, 3, 1, Closed, T1, "NSE-CIRC-2010-61"),
        // 2010-03-24 - T1 - NSE-CIRC-2010-61 - Ram Navmi, Wednesday.
        (2010, 3, 24, Closed, T1, "NSE-CIRC-2010-61"),
        // 2010-04-02 - T1 - NSE-CIRC-2010-61 - Good Friday.
        (2010, 4, 2, Closed, T1, "NSE-CIRC-2010-61"),
        // 2010-04-14 - T1 - NSE-CIRC-2010-61 - Dr. Ambedkar Jayanti, Wednesday.
        (2010, 4, 14, Closed, T1, "NSE-CIRC-2010-61"),
        // 2010-09-10 - T1 - NSE-CIRC-2010-61 - Ramzan ID, Friday.
        (2010, 9, 10, Closed, T1, "NSE-CIRC-2010-61"),
        // 2010-11-05 - T1 - NSE-CIRC-2010-61 - Laxmi Puja*: the circular footnotes
        // Muhurat Trading with timings "shall be notified subsequently", so no
        // complete statement of the day exists and the date is withheld rather
        // than closed.
        (2010, 11, 5, Unsourced, T1, "NSE-CIRC-2010-61"),
        // 2010-11-17 - T1 - NSE-CIRC-2010-61 - Bakri Id, Wednesday.
        (2010, 11, 17, Closed, T1, "NSE-CIRC-2010-61"),
        // 2010-12-17 - T1 - NSE-CIRC-2010-61 - Moharum, Friday.
        (2010, 12, 17, Closed, T1, "NSE-CIRC-2010-61"),
        // 2011, from the operator's 2011 trading-holiday page (corroborated
        // date-for-date by circular 126).
        // 2011-01-26 - T1 - NSE-HOL-PAGE-2011 - Republic Day, Wednesday.
        (2011, 1, 26, Closed, T1, "NSE-HOL-PAGE-2011"),
        // 2011-03-02 - T1 - NSE-HOL-PAGE-2011 - Mahashivratri, Wednesday.
        (2011, 3, 2, Closed, T1, "NSE-HOL-PAGE-2011"),
        // 2011-04-12 - T1 - NSE-HOL-PAGE-2011 - Ram Navmi, Tuesday.
        (2011, 4, 12, Closed, T1, "NSE-HOL-PAGE-2011"),
        // 2011-04-14 - T1 - NSE-HOL-PAGE-2011 - Dr. Ambedkar Jayanti, Thursday.
        (2011, 4, 14, Closed, T1, "NSE-HOL-PAGE-2011"),
        // 2011-04-22 - T1 - NSE-HOL-PAGE-2011 - Good Friday.
        (2011, 4, 22, Closed, T1, "NSE-HOL-PAGE-2011"),
        // 2011-08-15 - T1 - NSE-HOL-PAGE-2011 - Independence Day, Monday.
        (2011, 8, 15, Closed, T1, "NSE-HOL-PAGE-2011"),
        // 2011-08-31 - T1 - NSE-HOL-PAGE-2011 - Ramzan ID, Wednesday.
        (2011, 8, 31, Closed, T1, "NSE-HOL-PAGE-2011"),
        // 2011-09-01 - T1 - NSE-HOL-PAGE-2011 - Ganesh Chaturthi, Thursday.
        (2011, 9, 1, Closed, T1, "NSE-HOL-PAGE-2011"),
        // 2011-10-06 - T1 - NSE-HOL-PAGE-2011 - Dasara, Thursday.
        (2011, 10, 6, Closed, T1, "NSE-HOL-PAGE-2011"),
        // 2011-10-26 - T1 - NSE-HOL-PAGE-2011 - Laxmi Puja*: the list footnotes
        // Muhurat Trading with timings "shall be notified subsequently".
        (2011, 10, 26, Unsourced, T1, "NSE-HOL-PAGE-2011"),
        // 2011-10-27 - T1 - NSE-HOL-PAGE-2011 - Diwali-Balipratipada, Thursday.
        (2011, 10, 27, Closed, T1, "NSE-HOL-PAGE-2011"),
        // 2011-11-07 - T1 - NSE-HOL-PAGE-2011 - Bakri Id, Monday.
        (2011, 11, 7, Closed, T1, "NSE-HOL-PAGE-2011"),
        // 2011-11-10 - T1 - NSE-HOL-PAGE-2011 - Gurunanak Jayanti, Thursday.
        (2011, 11, 10, Closed, T1, "NSE-HOL-PAGE-2011"),
        // 2011-12-06 - T1 - NSE-HOL-PAGE-2011 - Moharum, Tuesday.
        (2011, 12, 6, Closed, T1, "NSE-HOL-PAGE-2011"),
        // 2013, from the operator's circular 79/2012 dated 2012-12-17.
        // 2013-03-27 - T1 - NSE-CIRC-2012-79 - Holi, Wednesday.
        (2013, 3, 27, Closed, T1, "NSE-CIRC-2012-79"),
        // 2013-03-29 - T1 - NSE-CIRC-2012-79 - Good Friday.
        (2013, 3, 29, Closed, T1, "NSE-CIRC-2012-79"),
        // 2013-04-19 - T1 - NSE-CIRC-2012-79 - Ram Navmi, Friday.
        (2013, 4, 19, Closed, T1, "NSE-CIRC-2012-79"),
        // 2013-04-24 - T1 - NSE-CIRC-2012-79 - Mahavir Jayanti, Wednesday.
        (2013, 4, 24, Closed, T1, "NSE-CIRC-2012-79"),
        // 2013-05-01 - T1 - NSE-CIRC-2012-79 - May Day, Wednesday.
        (2013, 5, 1, Closed, T1, "NSE-CIRC-2012-79"),
        // 2013-08-09 - T1 - NSE-CIRC-2012-79 - Ramzan ID, Friday.
        (2013, 8, 9, Closed, T1, "NSE-CIRC-2012-79"),
        // 2013-08-15 - T1 - NSE-CIRC-2012-79 - Independence Day, Thursday.
        (2013, 8, 15, Closed, T1, "NSE-CIRC-2012-79"),
        // 2013-09-09 - T1 - NSE-CIRC-2012-79 - Ganesh Chaturthi, Monday.
        (2013, 9, 9, Closed, T1, "NSE-CIRC-2012-79"),
        // 2013-10-02 - T1 - NSE-CIRC-2012-79 - Gandhi Jayanti, Wednesday.
        (2013, 10, 2, Closed, T1, "NSE-CIRC-2012-79"),
        // 2013-10-16 - T1 - NSE-CIRC-2012-79 - Bakri ID, Wednesday.
        (2013, 10, 16, Closed, T1, "NSE-CIRC-2012-79"),
        // 2013-11-03 - T1 - NSE-CIRC-2012-79 - Diwali-Laxmi Puja: a Sunday the
        // circular footnotes "Muhurat Trading will be conducted on Sunday,
        // November 03, 2013" without instants, so the normal week alone would
        // claim an audited-normal weekend the operator contradicts.
        (2013, 11, 3, Unsourced, T1, "NSE-CIRC-2012-79"),
        // 2013-11-04 - T1 - NSE-CIRC-2012-79 - Diwali-Balipratipada, Monday.
        (2013, 11, 4, Closed, T1, "NSE-CIRC-2012-79"),
        // 2013-11-14 - T1 - NSE-CIRC-2012-79 - Moharram, Thursday.
        (2013, 11, 14, Closed, T1, "NSE-CIRC-2012-79"),
        // 2013-12-25 - T1 - NSE-CIRC-2012-79 - Christmas, Wednesday.
        (2013, 12, 25, Closed, T1, "NSE-CIRC-2012-79"),
        // 2014, from the operator's 2014 trading-holiday page.
        // 2014-02-27 - T1 - NSE-HOL-PAGE-2014 - Mahashivratri, Thursday.
        (2014, 2, 27, Closed, T1, "NSE-HOL-PAGE-2014"),
        // 2014-03-17 - T1 - NSE-HOL-PAGE-2014 - Holi, Monday.
        (2014, 3, 17, Closed, T1, "NSE-HOL-PAGE-2014"),
        // 2014-04-08 - T1 - NSE-HOL-PAGE-2014 - Ram Navmi, Tuesday.
        (2014, 4, 8, Closed, T1, "NSE-HOL-PAGE-2014"),
        // 2014-04-14 - T1 - NSE-HOL-PAGE-2014 - Dr. Babasaheb Ambedkar Jayanti, Monday.
        (2014, 4, 14, Closed, T1, "NSE-HOL-PAGE-2014"),
        // 2014-04-18 - T1 - NSE-HOL-PAGE-2014 - Good Friday.
        (2014, 4, 18, Closed, T1, "NSE-HOL-PAGE-2014"),
        // 2014-05-01 - T1 - NSE-HOL-PAGE-2014 - May Day, Thursday.
        (2014, 5, 1, Closed, T1, "NSE-HOL-PAGE-2014"),
        // 2014-07-29 - T1 - NSE-HOL-PAGE-2014 - Ramzan ID, Tuesday.
        (2014, 7, 29, Closed, T1, "NSE-HOL-PAGE-2014"),
        // 2014-08-15 - T1 - NSE-HOL-PAGE-2014 - Independence Day, Friday.
        (2014, 8, 15, Closed, T1, "NSE-HOL-PAGE-2014"),
        // 2014-08-29 - T1 - NSE-HOL-PAGE-2014 - Ganesh Chaturthi, Friday.
        (2014, 8, 29, Closed, T1, "NSE-HOL-PAGE-2014"),
        // 2014-10-02 - T1 - NSE-HOL-PAGE-2014 - Mahatma Gandhi Jayanti, Thursday.
        (2014, 10, 2, Closed, T1, "NSE-HOL-PAGE-2014"),
        // 2014-10-03 - T1 - NSE-HOL-PAGE-2014 - Dasera, Friday.
        (2014, 10, 3, Closed, T1, "NSE-HOL-PAGE-2014"),
        // 2014-10-06 - T1 - NSE-HOL-PAGE-2014 - Bakri ID, Monday.
        (2014, 10, 6, Closed, T1, "NSE-HOL-PAGE-2014"),
        // 2014-10-23 - T1 - NSE-HOL-PAGE-2014 - Diwali-Laxmi Pujan*: Muhurat
        // timings "shall be notified subsequently".
        (2014, 10, 23, Unsourced, T1, "NSE-HOL-PAGE-2014"),
        // 2014-10-24 - T1 - NSE-HOL-PAGE-2014 - Diwali-Balipratipada, Friday.
        (2014, 10, 24, Closed, T1, "NSE-HOL-PAGE-2014"),
        // 2014-11-04 - T1 - NSE-HOL-PAGE-2014 - Moharram, Tuesday.
        (2014, 11, 4, Closed, T1, "NSE-HOL-PAGE-2014"),
        // 2014-11-06 - T1 - NSE-HOL-PAGE-2014 - Gurunank Jayanti, Thursday.
        (2014, 11, 6, Closed, T1, "NSE-HOL-PAGE-2014"),
        // 2014-12-25 - T1 - NSE-HOL-PAGE-2014 - Christmas, Thursday.
        (2014, 12, 25, Closed, T1, "NSE-HOL-PAGE-2014"),
        // 2015, from the operator's 2015 trading-holiday page.
        // 2015-01-26 - T1 - NSE-HOL-PAGE-2015 - Republic Day, Monday.
        (2015, 1, 26, Closed, T1, "NSE-HOL-PAGE-2015"),
        // 2015-02-17 - T1 - NSE-HOL-PAGE-2015 - Mahashivratri, Tuesday.
        (2015, 2, 17, Closed, T1, "NSE-HOL-PAGE-2015"),
        // 2015-03-06 - T1 - NSE-HOL-PAGE-2015 - Holi, Friday.
        (2015, 3, 6, Closed, T1, "NSE-HOL-PAGE-2015"),
        // 2015-04-02 - T1 - NSE-HOL-PAGE-2015 - Mahavir Jayanti, Thursday.
        (2015, 4, 2, Closed, T1, "NSE-HOL-PAGE-2015"),
        // 2015-04-03 - T1 - NSE-HOL-PAGE-2015 - Good Friday.
        (2015, 4, 3, Closed, T1, "NSE-HOL-PAGE-2015"),
        // 2015-04-14 - T1 - NSE-HOL-PAGE-2015 - Dr. Baba Saheb Ambedkar Jayanti, Tuesday.
        (2015, 4, 14, Closed, T1, "NSE-HOL-PAGE-2015"),
        // 2015-05-01 - T1 - NSE-HOL-PAGE-2015 - Maharashtra Day, Friday.
        (2015, 5, 1, Closed, T1, "NSE-HOL-PAGE-2015"),
        // 2015-09-17 - T1 - NSE-HOL-PAGE-2015 - Ganesh Chaturthi, Thursday.
        (2015, 9, 17, Closed, T1, "NSE-HOL-PAGE-2015"),
        // 2015-09-25 - T1 - NSE-HOL-PAGE-2015 - Bakri ID, Friday.
        (2015, 9, 25, Closed, T1, "NSE-HOL-PAGE-2015"),
        // 2015-10-02 - T1 - NSE-HOL-PAGE-2015 - Mahatma Gandhi Jayanti, Friday.
        (2015, 10, 2, Closed, T1, "NSE-HOL-PAGE-2015"),
        // 2015-10-22 - T1 - NSE-HOL-PAGE-2015 - Dussehra, Thursday.
        (2015, 10, 22, Closed, T1, "NSE-HOL-PAGE-2015"),
        // 2015-11-11 - T1 - NSE-HOL-PAGE-2015 - Diwali-Laxmi Pujan*: Muhurat
        // timings "shall be notified subsequently".
        (2015, 11, 11, Unsourced, T1, "NSE-HOL-PAGE-2015"),
        // 2015-11-12 - T1 - NSE-HOL-PAGE-2015 - Diwali-Balipratipada, Thursday.
        (2015, 11, 12, Closed, T1, "NSE-HOL-PAGE-2015"),
        // 2015-11-25 - T1 - NSE-HOL-PAGE-2015 - Gurunanak Jayanti, Wednesday.
        (2015, 11, 25, Closed, T1, "NSE-HOL-PAGE-2015"),
        // 2015-12-25 - T1 - NSE-HOL-PAGE-2015 - Christmas, Friday.
        (2015, 12, 25, Closed, T1, "NSE-HOL-PAGE-2015"),
        // 2016, from the operator's 2016 trading-holiday page.
        // 2016-01-26 - T1 - NSE-HOL-PAGE-2016 - Republic Day, Tuesday.
        (2016, 1, 26, Closed, T1, "NSE-HOL-PAGE-2016"),
        // 2016-03-07 - T1 - NSE-HOL-PAGE-2016 - Mahashivratri, Monday.
        (2016, 3, 7, Closed, T1, "NSE-HOL-PAGE-2016"),
        // 2016-03-24 - T1 - NSE-HOL-PAGE-2016 - Holi, Thursday.
        (2016, 3, 24, Closed, T1, "NSE-HOL-PAGE-2016"),
        // 2016-03-25 - T1 - NSE-HOL-PAGE-2016 - Good Friday.
        (2016, 3, 25, Closed, T1, "NSE-HOL-PAGE-2016"),
        // 2016-04-14 - T1 - NSE-HOL-PAGE-2016 - Dr. Baba Saheb Ambedkar Jayanti, Thursday.
        (2016, 4, 14, Closed, T1, "NSE-HOL-PAGE-2016"),
        // 2016-04-15 - T1 - NSE-HOL-PAGE-2016 - Ram Navami, Friday.
        (2016, 4, 15, Closed, T1, "NSE-HOL-PAGE-2016"),
        // 2016-04-19 - T1 - NSE-HOL-PAGE-2016 - Mahavir Jayanti, Tuesday.
        (2016, 4, 19, Closed, T1, "NSE-HOL-PAGE-2016"),
        // 2016-07-06 - T1 - NSE-HOL-PAGE-2016 - Id-uI-Fitar (Ramzan ID), Wednesday.
        (2016, 7, 6, Closed, T1, "NSE-HOL-PAGE-2016"),
        // 2016-08-15 - T1 - NSE-HOL-PAGE-2016 - Independence Day, Monday.
        (2016, 8, 15, Closed, T1, "NSE-HOL-PAGE-2016"),
        // 2016-09-05 - T1 - NSE-HOL-PAGE-2016 - Ganesh Chaturthi, Monday.
        (2016, 9, 5, Closed, T1, "NSE-HOL-PAGE-2016"),
        // 2016-09-13 - T1 - NSE-HOL-PAGE-2016 - Bakri ID, Tuesday.
        (2016, 9, 13, Closed, T1, "NSE-HOL-PAGE-2016"),
        // 2016-10-11 - T1 - NSE-HOL-PAGE-2016 - Dasera, Tuesday.
        (2016, 10, 11, Closed, T1, "NSE-HOL-PAGE-2016"),
        // 2016-10-12 - T1 - NSE-HOL-PAGE-2016 - Moharram, Wednesday.
        (2016, 10, 12, Closed, T1, "NSE-HOL-PAGE-2016"),
        // 2016-10-30 - T1 - NSE-HOL-PAGE-2016 - Diwali-Laxmi Pujan*: a Sunday the
        // list footnotes Muhurat Trading with timings "shall be notified
        // subsequently".
        (2016, 10, 30, Unsourced, T1, "NSE-HOL-PAGE-2016"),
        // 2016-10-31 - T1 - NSE-HOL-PAGE-2016 - Diwali-Balipratipada, Monday.
        (2016, 10, 31, Closed, T1, "NSE-HOL-PAGE-2016"),
        // 2016-11-14 - T1 - NSE-HOL-PAGE-2016 - Gurunanak Jayanti, Monday.
        (2016, 11, 14, Closed, T1, "NSE-HOL-PAGE-2016"),
        // 2017, from the operator's 2017 trading-holiday page.
        // 2017-01-26 - T1 - NSE-HOL-PAGE-2017 - Republic Day, Thursday.
        (2017, 1, 26, Closed, T1, "NSE-HOL-PAGE-2017"),
        // 2017-02-24 - T1 - NSE-HOL-PAGE-2017 - Mahashivratri, Friday.
        (2017, 2, 24, Closed, T1, "NSE-HOL-PAGE-2017"),
        // 2017-03-13 - T1 - NSE-HOL-PAGE-2017 - Holi, Monday.
        (2017, 3, 13, Closed, T1, "NSE-HOL-PAGE-2017"),
        // 2017-04-04 - T1 - NSE-HOL-PAGE-2017 - Ram Navami, Tuesday.
        (2017, 4, 4, Closed, T1, "NSE-HOL-PAGE-2017"),
        // 2017-04-14 - T1 - NSE-HOL-PAGE-2017 - Dr.Baba Saheb Ambedkar Jayanti /
        // Good Friday.
        (2017, 4, 14, Closed, T1, "NSE-HOL-PAGE-2017"),
        // 2017-05-01 - T1 - NSE-HOL-PAGE-2017 - Maharashtra Day, Monday.
        (2017, 5, 1, Closed, T1, "NSE-HOL-PAGE-2017"),
        // 2017-06-26 - T1 - NSE-HOL-PAGE-2017 - Id-Ul-Fitr (Ramzan ID), Monday.
        (2017, 6, 26, Closed, T1, "NSE-HOL-PAGE-2017"),
        // 2017-08-15 - T1 - NSE-HOL-PAGE-2017 - Independence Day, Tuesday.
        (2017, 8, 15, Closed, T1, "NSE-HOL-PAGE-2017"),
        // 2017-08-25 - T1 - NSE-HOL-PAGE-2017 - Ganesh Chaturthi, Friday.
        (2017, 8, 25, Closed, T1, "NSE-HOL-PAGE-2017"),
        // 2017-10-02 - T1 - NSE-HOL-PAGE-2017 - Mahatama Gandhi Jayanti, Monday.
        (2017, 10, 2, Closed, T1, "NSE-HOL-PAGE-2017"),
        // 2017-10-19 - T1 - NSE-HOL-PAGE-2017 - Diwali-Laxmi Pujan*: Muhurat
        // timings "shall be notified subsequently".
        (2017, 10, 19, Unsourced, T1, "NSE-HOL-PAGE-2017"),
        // 2017-10-20 - T1 - NSE-HOL-PAGE-2017 - Diwali-Balipratipada, Friday.
        (2017, 10, 20, Closed, T1, "NSE-HOL-PAGE-2017"),
        // 2017-12-25 - T1 - NSE-HOL-PAGE-2017 - Christmas, Monday.
        (2017, 12, 25, Closed, T1, "NSE-HOL-PAGE-2017"),
        // 2019, from the operator's 2019 trading-holiday page.
        // 2019-03-04 - T1 - NSE-HOL-PAGE-2019 - Mahashivratri, Monday.
        (2019, 3, 4, Closed, T1, "NSE-HOL-PAGE-2019"),
        // 2019-03-21 - T1 - NSE-HOL-PAGE-2019 - Holi, Thursday.
        (2019, 3, 21, Closed, T1, "NSE-HOL-PAGE-2019"),
        // 2019-04-17 - T1 - NSE-HOL-PAGE-2019 - Mahavir Jayanti, Wednesday.
        (2019, 4, 17, Closed, T1, "NSE-HOL-PAGE-2019"),
        // 2019-04-19 - T1 - NSE-HOL-PAGE-2019 - Good Friday.
        (2019, 4, 19, Closed, T1, "NSE-HOL-PAGE-2019"),
        // 2019-04-29 - T1 - NSE-HOL-PAGE-2019 - Parliamentary Elections, Monday.
        (2019, 4, 29, Closed, T1, "NSE-HOL-PAGE-2019"),
        // 2019-05-01 - T1 - NSE-HOL-PAGE-2019 - Maharashtra Day, Wednesday.
        (2019, 5, 1, Closed, T1, "NSE-HOL-PAGE-2019"),
        // 2019-06-05 - T1 - NSE-HOL-PAGE-2019 - Id-Ul-Fitr (Ramzan ID), Wednesday.
        (2019, 6, 5, Closed, T1, "NSE-HOL-PAGE-2019"),
        // 2019-08-12 - T1 - NSE-HOL-PAGE-2019 - Bakri Id, Monday.
        (2019, 8, 12, Closed, T1, "NSE-HOL-PAGE-2019"),
        // 2019-08-15 - T1 - NSE-HOL-PAGE-2019 - Independence Day, Thursday.
        (2019, 8, 15, Closed, T1, "NSE-HOL-PAGE-2019"),
        // 2019-09-02 - T1 - NSE-HOL-PAGE-2019 - Ganesh Chaturthi, Monday.
        (2019, 9, 2, Closed, T1, "NSE-HOL-PAGE-2019"),
        // 2019-09-10 - T1 - NSE-HOL-PAGE-2019 - Moharram, Tuesday.
        (2019, 9, 10, Closed, T1, "NSE-HOL-PAGE-2019"),
        // 2019-10-02 - T1 - NSE-HOL-PAGE-2019 - Mahatma Gandhi Jayanti, Wednesday.
        (2019, 10, 2, Closed, T1, "NSE-HOL-PAGE-2019"),
        // 2019-10-08 - T1 - NSE-HOL-PAGE-2019 - Dasera, Tuesday.
        (2019, 10, 8, Closed, T1, "NSE-HOL-PAGE-2019"),
        // 2019-10-27 - T1 - NSE-HOL-PAGE-2019 - Diwali-Laxmi Pujan*: a Sunday the
        // list footnotes Muhurat Trading with timings "shall be notified
        // subsequently".
        (2019, 10, 27, Unsourced, T1, "NSE-HOL-PAGE-2019"),
        // 2019-10-28 - T1 - NSE-HOL-PAGE-2019 - Diwali-Balipratipada, Monday.
        (2019, 10, 28, Closed, T1, "NSE-HOL-PAGE-2019"),
        // 2019-11-12 - T1 - NSE-HOL-PAGE-2019 - Gurunanak Jayanti, Tuesday.
        (2019, 11, 12, Closed, T1, "NSE-HOL-PAGE-2019"),
        // 2019-12-25 - T1 - NSE-HOL-PAGE-2019 - Christmas, Wednesday.
        (2019, 12, 25, Closed, T1, "NSE-HOL-PAGE-2019"),
        // 2020, from the operator's 2020 equities holiday page.
        // 2020-02-21 - T1 - NSE-HOL-PAGE-2020 - Mahashivratri, Friday.
        (2020, 2, 21, Closed, T1, "NSE-HOL-PAGE-2020"),
        // 2020-03-10 - T1 - NSE-HOL-PAGE-2020 - Holi, Tuesday.
        (2020, 3, 10, Closed, T1, "NSE-HOL-PAGE-2020"),
        // 2020-04-02 - T1 - NSE-HOL-PAGE-2020 - Ram Navami, Thursday.
        (2020, 4, 2, Closed, T1, "NSE-HOL-PAGE-2020"),
        // 2020-04-06 - T1 - NSE-HOL-PAGE-2020 - Mahavir Jayanti, Monday.
        (2020, 4, 6, Closed, T1, "NSE-HOL-PAGE-2020"),
        // 2020-04-10 - T1 - NSE-HOL-PAGE-2020 - Good Friday.
        (2020, 4, 10, Closed, T1, "NSE-HOL-PAGE-2020"),
        // 2020-04-14 - T1 - NSE-HOL-PAGE-2020 - Dr.Baba Saheb Ambedkar Jayanti, Tuesday.
        (2020, 4, 14, Closed, T1, "NSE-HOL-PAGE-2020"),
        // 2020-05-01 - T1 - NSE-HOL-PAGE-2020 - Maharashtra Day, Friday.
        (2020, 5, 1, Closed, T1, "NSE-HOL-PAGE-2020"),
        // 2020-05-25 - T1 - NSE-HOL-PAGE-2020 - Id-Ul-Fitr (Ramzan ID), Monday.
        (2020, 5, 25, Closed, T1, "NSE-HOL-PAGE-2020"),
        // 2020-10-02 - T1 - NSE-HOL-PAGE-2020 - Mahatma Gandhi Jayanti, Friday.
        (2020, 10, 2, Closed, T1, "NSE-HOL-PAGE-2020"),
        // 2020-11-14 - T1 - NSE-CIRC-2020-98 - Diwali Muhurat Trading: the
        // operator's circular 98/2020 (2020-11-02) states the Saturday
        // special session's complete schedule; the row restates it as one
        // replacement day.
        (2020, 11, 14, ReplacementBlocks(&MUHURAT_EVENING_BLOCKS), T1, "NSE-CIRC-2020-98"),
        // 2020-11-16 - T1 - NSE-HOL-PAGE-2020 - Diwali-Balipratipada, Monday.
        (2020, 11, 16, Closed, T1, "NSE-HOL-PAGE-2020"),
        // 2020-11-30 - T1 - NSE-HOL-PAGE-2020 - Gurunanak Jayanti, Monday.
        (2020, 11, 30, Closed, T1, "NSE-HOL-PAGE-2020"),
        // 2020-12-25 - T1 - NSE-HOL-PAGE-2020 - Christmas, Friday.
        (2020, 12, 25, Closed, T1, "NSE-HOL-PAGE-2020"),
        // 2021, from the operator's 2021 trading-holiday page; the page prints
        // its Muhurat footnote against no date, so nothing is withheld here
        // (the residual risk is recorded in the evidence file).
        // 2021-01-26 - T1 - NSE-HOL-PAGE-2021 - Republic Day, Tuesday.
        (2021, 1, 26, Closed, T1, "NSE-HOL-PAGE-2021"),
        // 2021-03-11 - T1 - NSE-HOL-PAGE-2021 - Mahashivratri, Thursday.
        (2021, 3, 11, Closed, T1, "NSE-HOL-PAGE-2021"),
        // 2021-03-29 - T1 - NSE-HOL-PAGE-2021 - Holi, Monday.
        (2021, 3, 29, Closed, T1, "NSE-HOL-PAGE-2021"),
        // 2021-04-02 - T1 - NSE-HOL-PAGE-2021 - Good Friday.
        (2021, 4, 2, Closed, T1, "NSE-HOL-PAGE-2021"),
        // 2021-04-14 - T1 - NSE-HOL-PAGE-2021 - Dr.Baba Saheb Ambedkar Jayanti, Wednesday.
        (2021, 4, 14, Closed, T1, "NSE-HOL-PAGE-2021"),
        // 2021-04-21 - T1 - NSE-HOL-PAGE-2021 - Ram Navami, Wednesday.
        (2021, 4, 21, Closed, T1, "NSE-HOL-PAGE-2021"),
        // 2021-05-13 - T1 - NSE-HOL-PAGE-2021 - Id-Ul-Fitr (Ramzan ID), Thursday.
        (2021, 5, 13, Closed, T1, "NSE-HOL-PAGE-2021"),
        // 2021-07-21 - T1 - NSE-HOL-PAGE-2021 - Bakri Id, Wednesday.
        (2021, 7, 21, Closed, T1, "NSE-HOL-PAGE-2021"),
        // 2021-08-19 - T1 - NSE-HOL-PAGE-2021 - Moharram, Thursday.
        (2021, 8, 19, Closed, T1, "NSE-HOL-PAGE-2021"),
        // 2021-09-10 - T1 - NSE-HOL-PAGE-2021 - Ganesh Chaturthi, Friday.
        (2021, 9, 10, Closed, T1, "NSE-HOL-PAGE-2021"),
        // 2021-10-15 - T1 - NSE-HOL-PAGE-2021 - Dussehra, Friday.
        (2021, 10, 15, Closed, T1, "NSE-HOL-PAGE-2021"),
        // 2021-11-05 - T1 - NSE-HOL-PAGE-2021 - Diwali-Balipratipada, Friday.
        (2021, 11, 5, Closed, T1, "NSE-HOL-PAGE-2021"),
        // 2021-11-19 - T1 - NSE-HOL-PAGE-2021 - Gurunanak Jayanti, Friday.
        (2021, 11, 19, Closed, T1, "NSE-HOL-PAGE-2021"),
        // 2022, from the operator's 2022 trading-holiday page (corroborated
        // date-for-date by the holiday-master T2 feed).
        // 2022-01-26 - T1 - NSE-HOL-PAGE-2022 - Republic Day, Wednesday.
        (2022, 1, 26, Closed, T1, "NSE-HOL-PAGE-2022"),
        // 2022-03-01 - T1 - NSE-HOL-PAGE-2022 - Mahashivratri, Tuesday.
        (2022, 3, 1, Closed, T1, "NSE-HOL-PAGE-2022"),
        // 2022-03-18 - T1 - NSE-HOL-PAGE-2022 - Holi, Friday.
        (2022, 3, 18, Closed, T1, "NSE-HOL-PAGE-2022"),
        // 2022-04-14 - T1 - NSE-HOL-PAGE-2022 - Dr.Baba Saheb Ambedkar Jayanti /
        // Mahavir Jayanti, Thursday.
        (2022, 4, 14, Closed, T1, "NSE-HOL-PAGE-2022"),
        // 2022-04-15 - T1 - NSE-HOL-PAGE-2022 - Good Friday.
        (2022, 4, 15, Closed, T1, "NSE-HOL-PAGE-2022"),
        // 2022-05-03 - T1 - NSE-HOL-PAGE-2022 - Id-Ul-Fitr (Ramzan ID), Tuesday.
        (2022, 5, 3, Closed, T1, "NSE-HOL-PAGE-2022"),
        // 2022-08-09 - T1 - NSE-HOL-PAGE-2022 - Moharram, Tuesday.
        (2022, 8, 9, Closed, T1, "NSE-HOL-PAGE-2022"),
        // 2022-08-15 - T1 - NSE-HOL-PAGE-2022 - Independence Day, Monday.
        (2022, 8, 15, Closed, T1, "NSE-HOL-PAGE-2022"),
        // 2022-08-31 - T1 - NSE-HOL-PAGE-2022 - Ganesh Chaturthi, Wednesday.
        (2022, 8, 31, Closed, T1, "NSE-HOL-PAGE-2022"),
        // 2022-10-05 - T1 - NSE-HOL-PAGE-2022 - Dussehra, Wednesday.
        (2022, 10, 5, Closed, T1, "NSE-HOL-PAGE-2022"),
        // 2022-10-24 - T1 - NSE-CIRC-2022-124 - Diwali * Laxmi Pujan: the
        // operator's circular 124/2022 (2022-10-11) states the Monday
        // Muhurat session's complete schedule (identical to 2020's); the row
        // restates it as one replacement day.
        (2022, 10, 24, ReplacementBlocks(&MUHURAT_EVENING_BLOCKS), T1, "NSE-CIRC-2022-124"),
        // 2022-10-26 - T1 - NSE-HOL-PAGE-2022 - Diwali-Balipratipada, Wednesday.
        (2022, 10, 26, Closed, T1, "NSE-HOL-PAGE-2022"),
        // 2022-11-08 - T1 - NSE-HOL-PAGE-2022 - Gurunanak Jayanti, Tuesday.
        (2022, 11, 8, Closed, T1, "NSE-HOL-PAGE-2022"),
        // 2023, from the operator's holiday-master T2 feed (CM segment).
        // 2023-01-26 - T2 - NSE-HOLMASTER-2023 - Republic Day, Thursday.
        (2023, 1, 26, Closed, T2, "NSE-HOLMASTER-2023"),
        // 2023-03-07 - T2 - NSE-HOLMASTER-2023 - Holi, Tuesday.
        (2023, 3, 7, Closed, T2, "NSE-HOLMASTER-2023"),
        // 2023-03-30 - T2 - NSE-HOLMASTER-2023 - Ram Navami, Thursday.
        (2023, 3, 30, Closed, T2, "NSE-HOLMASTER-2023"),
        // 2023-04-04 - T2 - NSE-HOLMASTER-2023 - Mahavir Jayanti, Tuesday.
        (2023, 4, 4, Closed, T2, "NSE-HOLMASTER-2023"),
        // 2023-04-07 - T2 - NSE-HOLMASTER-2023 - Good Friday.
        (2023, 4, 7, Closed, T2, "NSE-HOLMASTER-2023"),
        // 2023-04-14 - T2 - NSE-HOLMASTER-2023 - Dr. Baba Saheb Ambedkar Jayanti, Friday.
        (2023, 4, 14, Closed, T2, "NSE-HOLMASTER-2023"),
        // 2023-05-01 - T2 - NSE-HOLMASTER-2023 - Maharashtra Day, Monday.
        (2023, 5, 1, Closed, T2, "NSE-HOLMASTER-2023"),
        // 2023-06-28 - T2 - NSE-HOLMASTER-2023 - Bakri Id, Wednesday.
        (2023, 6, 28, Closed, T2, "NSE-HOLMASTER-2023"),
        // 2023-08-15 - T2 - NSE-HOLMASTER-2023 - Independence Day, Tuesday.
        (2023, 8, 15, Closed, T2, "NSE-HOLMASTER-2023"),
        // 2023-09-19 - T2 - NSE-HOLMASTER-2023 - Ganesh Chaturthi, Tuesday.
        (2023, 9, 19, Closed, T2, "NSE-HOLMASTER-2023"),
        // 2023-10-02 - T2 - NSE-HOLMASTER-2023 - Mahatma Gandhi Jayanti, Monday.
        (2023, 10, 2, Closed, T2, "NSE-HOLMASTER-2023"),
        // 2023-10-24 - T2 - NSE-HOLMASTER-2023 - Dussehra, Tuesday.
        (2023, 10, 24, Closed, T2, "NSE-HOLMASTER-2023"),
        // 2023-11-12 - T1 - NSE-CIRC-2023-139 - Diwali-Laxmi Pujan: the
        // operator's circular 139/2023 (2023-10-27) states the Sunday
        // Muhurat session's complete schedule (identical to 2020's); the row
        // restates it as one replacement day.
        (2023, 11, 12, ReplacementBlocks(&MUHURAT_EVENING_BLOCKS), T1, "NSE-CIRC-2023-139"),
        // 2023-11-14 - T2 - NSE-HOLMASTER-2023 - Diwali-Balipratipada, Tuesday.
        (2023, 11, 14, Closed, T2, "NSE-HOLMASTER-2023"),
        // 2023-11-27 - T2 - NSE-HOLMASTER-2023 - Gurunanak Jayanti, Monday.
        (2023, 11, 27, Closed, T2, "NSE-HOLMASTER-2023"),
        // 2023-12-25 - T2 - NSE-HOLMASTER-2023 - Christmas, Monday.
        (2023, 12, 25, Closed, T2, "NSE-HOLMASTER-2023"),
        // 2024, from the operator's holiday-master T2 feed (CM segment).
        // 2024-01-26 - T2 - NSE-HOLMASTER-2024 - Republic Day, Friday.
        (2024, 1, 26, Closed, T2, "NSE-HOLMASTER-2024"),
        // 2024-03-08 - T2 - NSE-HOLMASTER-2024 - Mahashivratri, Friday.
        (2024, 3, 8, Closed, T2, "NSE-HOLMASTER-2024"),
        // 2024-03-25 - T2 - NSE-HOLMASTER-2024 - Holi, Monday.
        (2024, 3, 25, Closed, T2, "NSE-HOLMASTER-2024"),
        // 2024-03-29 - T2 - NSE-HOLMASTER-2024 - Good Friday.
        (2024, 3, 29, Closed, T2, "NSE-HOLMASTER-2024"),
        // 2024-04-11 - T2 - NSE-HOLMASTER-2024 - Id-Ul-Fitr (Ramadan Eid), Thursday.
        (2024, 4, 11, Closed, T2, "NSE-HOLMASTER-2024"),
        // 2024-04-17 - T2 - NSE-HOLMASTER-2024 - Shri Ram Navmi, Wednesday.
        (2024, 4, 17, Closed, T2, "NSE-HOLMASTER-2024"),
        // 2024-05-01 - T2 - NSE-HOLMASTER-2024 - Maharashtra Day, Wednesday.
        (2024, 5, 1, Closed, T2, "NSE-HOLMASTER-2024"),
        // 2024-06-17 - T2 - NSE-HOLMASTER-2024 - Bakri Id, Monday.
        (2024, 6, 17, Closed, T2, "NSE-HOLMASTER-2024"),
        // 2024-07-17 - T2 - NSE-HOLMASTER-2024 - Moharram, Wednesday.
        (2024, 7, 17, Closed, T2, "NSE-HOLMASTER-2024"),
        // 2024-08-15 - T2 - NSE-HOLMASTER-2024 - Independence Day, Thursday.
        (2024, 8, 15, Closed, T2, "NSE-HOLMASTER-2024"),
        // 2024-10-02 - T2 - NSE-HOLMASTER-2024 - Mahatma Gandhi Jayanti, Wednesday.
        (2024, 10, 2, Closed, T2, "NSE-HOLMASTER-2024"),
        // 2024-11-01 - T1 - NSE-CIRC-2024-147 - Diwali Laxmi Pujan: the
        // operator's circular 147/2024 (2024-10-19) states the Friday
        // Muhurat session's complete schedule; the row restates it as one
        // replacement day.
        (2024, 11, 1, ReplacementBlocks(&MUHURAT_2024_BLOCKS), T1, "NSE-CIRC-2024-147"),
        // 2024-11-15 - T2 - NSE-HOLMASTER-2024 - Gurunanak Jayanti, Friday.
        (2024, 11, 15, Closed, T2, "NSE-HOLMASTER-2024"),
        // 2024-12-25 - T2 - NSE-HOLMASTER-2024 - Christmas, Wednesday.
        (2024, 12, 25, Closed, T2, "NSE-HOLMASTER-2024"),
        // 2025-02-26 - T1 - NSE-HOL-2025 - Mahashivratri, Wednesday.
        (2025, 2, 26, Closed, T1, "NSE-HOL-2025"),
        // 2025-03-14 - T1 - NSE-HOL-2025 - Holi, Friday.
        (2025, 3, 14, Closed, T1, "NSE-HOL-2025"),
        // 2025-03-31 - T1 - NSE-HOL-2025 - Eid-ul-Fitr (Ramadan Eid), Monday.
        (2025, 3, 31, Closed, T1, "NSE-HOL-2025"),
        // 2025-04-10 - T1 - NSE-HOL-2025 - Shri Mahavir Jayanti, Thursday.
        (2025, 4, 10, Closed, T1, "NSE-HOL-2025"),
        // 2025-04-14 - T1 - NSE-HOL-2025 - Dr. Babasaheb Ambedkar Jayanti, Monday.
        (2025, 4, 14, Closed, T1, "NSE-HOL-2025"),
        // 2025-04-18 - T1 - NSE-HOL-2025 - Good Friday.
        (2025, 4, 18, Closed, T1, "NSE-HOL-2025"),
        // 2025-05-01 - T1 - NSE-HOL-2025 - Maharashtra Day, Thursday.
        (2025, 5, 1, Closed, T1, "NSE-HOL-2025"),
        // 2025-08-15 - T1 - NSE-HOL-2025 - Independence Day, Friday.
        (2025, 8, 15, Closed, T1, "NSE-HOL-2025"),
        // 2025-08-27 - T1 - NSE-HOL-2025 - Ganesh Chaturthi, Wednesday.
        (2025, 8, 27, Closed, T1, "NSE-HOL-2025"),
        // 2025-10-02 - T1 - NSE-HOL-2025 - Mahatma Gandhi Jayanti / Dussehra,
        // Thursday.
        (2025, 10, 2, Closed, T1, "NSE-HOL-2025"),
        // 2025-10-21 - T1 - NSE-CIRC-2025-124 - Diwali Laxmi Pujan: the
        // operator's circular 124/2025 (2025-09-22) states the day's
        // complete Muhurat schedule; the row restates it as one replacement
        // day (the banner's list row had withheld it pending exactly this
        // circular).
        (2025, 10, 21, ReplacementBlocks(&MUHURAT_2025_BLOCKS), T1, "NSE-CIRC-2025-124"),
        // 2025-10-22 - T1 - NSE-HOL-2025 - Diwali Balipratipada, Wednesday.
        (2025, 10, 22, Closed, T1, "NSE-HOL-2025"),
        // 2025-11-05 - T1 - NSE-HOL-2025 - Prakash Gurpurab Sri Guru Nanak Dev,
        // Wednesday.
        (2025, 11, 5, Closed, T1, "NSE-HOL-2025"),
        // 2025-12-25 - T1 - NSE-HOL-2025 - Christmas, Thursday.
        (2025, 12, 25, Closed, T1, "NSE-HOL-2025"),
        // 2026-01-26 - T1 - NSE-HOL-2026 - Republic Day, Monday.
        (2026, 1, 26, Closed, T1, "NSE-HOL-2026"),
        // 2026-03-03 - T1 - NSE-HOL-2026 - Holi, Tuesday.
        (2026, 3, 3, Closed, T1, "NSE-HOL-2026"),
        // 2026-03-26 - T1 - NSE-HOL-2026 - Shri Ram Navami, Thursday.
        (2026, 3, 26, Closed, T1, "NSE-HOL-2026"),
        // 2026-03-31 - T1 - NSE-HOL-2026 - Shri Mahavir Jayanti, Tuesday.
        (2026, 3, 31, Closed, T1, "NSE-HOL-2026"),
        // 2026-04-03 - T1 - NSE-HOL-2026 - Good Friday.
        (2026, 4, 3, Closed, T1, "NSE-HOL-2026"),
        // 2026-04-14 - T1 - NSE-HOL-2026 - Dr. Baba Saheb Ambedkar Jayanti, Tuesday.
        (2026, 4, 14, Closed, T1, "NSE-HOL-2026"),
        // 2026-05-01 - T1 - NSE-HOL-2026 - Maharashtra Day, Friday.
        (2026, 5, 1, Closed, T1, "NSE-HOL-2026"),
        // 2026-05-28 - T1 - NSE-HOL-2026 - Bakri Id, Thursday.
        (2026, 5, 28, Closed, T1, "NSE-HOL-2026"),
        // 2026-06-26 - T1 - NSE-HOL-2026 - Muharram, Friday.
        (2026, 6, 26, Closed, T1, "NSE-HOL-2026"),
        // 2026-09-14 - T1 - NSE-HOL-2026 - Ganesh Chaturthi, Monday.
        (2026, 9, 14, Closed, T1, "NSE-HOL-2026"),
        // 2026-10-02 - T1 - NSE-HOL-2026 - Mahatma Gandhi Jayanti, Friday.
        (2026, 10, 2, Closed, T1, "NSE-HOL-2026"),
        // 2026-10-20 - T1 - NSE-HOL-2026 - Dussehra, Tuesday.
        (2026, 10, 20, Closed, T1, "NSE-HOL-2026"),
        // 2026-11-08 - T1 - NSE-HOL-2026 - Muhurat Trading: the list footnotes a
        // special session onto this Sunday with timings "shall be notified
        // subsequently", so the day is not the audited-normal weekend closure the
        // normal week alone would claim and is withheld.
        (2026, 11, 8, Unsourced, T1, "NSE-HOL-2026"),
        // 2026-11-10 - T1 - NSE-HOL-2026 - Diwali-Balipratipada, Tuesday.
        (2026, 11, 10, Closed, T1, "NSE-HOL-2026"),
        // 2026-11-24 - T1 - NSE-HOL-2026 - Prakash Gurpurab Sri Guru Nanak Dev,
        // Tuesday.
        (2026, 11, 24, Closed, T1, "NSE-HOL-2026"),
        // 2026-12-25 - T1 - NSE-HOL-2026 - Christmas, Friday.
        (2026, 12, 25, Closed, T1, "NSE-HOL-2026"),
    ],
};
