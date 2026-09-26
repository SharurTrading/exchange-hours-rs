// SPDX-License-Identifier: MIT-0

//! Public contracts for caller-owned replacement trading days.
//!
//! Everything in this file is a **caller-owned fixture**. The crate ships no
//! exception data, and nothing here is loaded by any library code path; the
//! records exist to exercise the engine over shapes that real operators publish
//! and that the scalar `DayPolicy` vocabulary provably cannot express.
//!
//! Where a fixture restates a published arrangement, the source and what it
//! actually states are recorded beside it, including where a cited notice could
//! not be retrieved.
//!
//! **Coverage (Stage 2B, LAW-COVERAGE).** The two historical fixtures here —
//! CME's 2015 Thanksgiving week and Nasdaq's 2011 Thanksgiving week — are dated
//! before the permanent 2025 support floor, and the 2026 fixtures sit inside it.
//! An identity-backed date-aware query refuses a date it cannot source, so on
//! the pre-floor dates the engine's answer is no longer observable through the
//! public surface at all: a refusal is not a closure, and a caller-owned record
//! cannot license a date the identity does not answer
//! (`tests/coverage_query_errors.rs` states that contract directly). Each such
//! test below therefore asserts the refusal for the same probes it used to
//! assert answers for, keeps every claim the caller-owned table surface still
//! states, and says in a comment which half of its original claim is gone. The
//! engine's runtime semantics remain exercised on covered dates by the 2026
//! fixtures in this file and by `tests/coverage_query_errors.rs`.

#![expect(
    clippy::expect_used,
    reason = "fixture literals and validated static records must fail the test if malformed"
)]

use chrono::{DateTime, Datelike, NaiveDate, TimeZone, Utc};
use chrono_tz::US;
use exchange_hours::{
    CalendarQueryError, CalendarResolution, CalendarSource, DateCoverage, DateException,
    DayOverride, ExceptionBlock, ExceptionBlockKind, ExceptionCoverage, Exchange, Holiday,
    HolidayKind, MarketHoursKey, SessionExceptionRecord, SessionExceptionSource, SessionKind,
    SessionState, StaticDayPolicy, StaticSessionExceptions, StaticSessionExceptionsError,
    calendar_for_exchange, calendar_for_market_hours_key, hours_for_exchange,
};

const fn day(year: i32, month: u32, date: u32) -> NaiveDate {
    NaiveDate::from_ymd_opt(year, month, date).expect("valid fixture date")
}

fn ct(date: (i32, u32, u32), time: (u32, u32, u32)) -> DateTime<Utc> {
    US::Central
        .with_ymd_and_hms(date.0, date.1, date.2, time.0, time.1, time.2)
        .single()
        .expect("fixture must be a valid CT instant")
        .with_timezone(&Utc)
}

fn et(date: (i32, u32, u32), time: (u32, u32, u32)) -> DateTime<Utc> {
    US::Eastern
        .with_ymd_and_hms(date.0, date.1, date.2, time.0, time.1, time.2)
        .single()
        .expect("fixture must be a valid ET instant")
        .with_timezone(&Utc)
}

fn utc(date: (i32, u32, u32), time: (u32, u32, u32)) -> DateTime<Utc> {
    Utc.with_ymd_and_hms(date.0, date.1, date.2, time.0, time.1, time.2)
        .single()
        .expect("fixture must be a valid UTC instant")
}

fn assert_copy_send_sync_static<T: Copy + Send + Sync + 'static>() {}

/// Reads a table through the object-safe trait the engine consumes.
fn provider_coverage(provider: &dyn SessionExceptionSource) -> Option<ExceptionCoverage> {
    provider.coverage()
}

/// Asserts an identity-backed query returns exactly `expected`, the coverage
/// error the shipped data declares. The variant, identity and date are all part
/// of the contract: a refusal that named the wrong day would be as wrong as an
/// answer.
fn assert_refusal<T: std::fmt::Debug>(
    answer: Result<T, CalendarQueryError>,
    expected: CalendarQueryError,
    label: &str,
) {
    let error = answer.expect_err(&format!(
        "{label}: expected the coverage refusal {expected:?}"
    ));
    assert_eq!(
        error, expected,
        "{label}: the query must state the refusal its identity declares"
    );
}

/// Asserts a query refuses `date` because the venue-local day precedes the
/// permanent 2025 support floor (LAW-COVERAGE).
fn assert_before_floor<T: std::fmt::Debug>(
    answer: Result<T, CalendarQueryError>,
    source: CalendarSource,
    date: NaiveDate,
    label: &str,
) {
    assert_refusal(
        answer,
        CalendarQueryError::BeforeSupportFloor { source, date },
        label,
    );
}

/// Asserts an identity-backed query refuses `date` because the identity has no
/// sourced answer for it at or above the floor, and that the identity's own
/// published coverage says exactly that.
///
/// Driving the expectation from [`exchange_hours::CalendarCoverage::coverage_on`]
/// keeps the assertion honest for a many-date sweep: the test states the same
/// verdict the shipped data declares rather than a hand-copied list.
fn assert_declared_refusal<T: std::fmt::Debug>(
    answer: Result<T, CalendarQueryError>,
    calendar: exchange_hours::ExchangeCalendar,
    date: NaiveDate,
    label: &str,
) {
    let error = answer.expect_err(&format!("{label}: expected a coverage refusal"));
    let verdict = calendar.coverage().coverage_on(date);
    assert!(
        matches!(
            verdict,
            DateCoverage::OutsideCoveredRange
                | DateCoverage::UnresolvedGap
                | DateCoverage::BeforeSupportFloor
        ),
        "{label}: {date} is {verdict:?}, so a refusal is not the declared answer"
    );
    assert_eq!(
        Some(error),
        declared_error(calendar, date),
        "{label}: the query must state the coverage verdict its identity publishes"
    );
}

/// The refusal `calendar` publishes for `date`, or `None` where it declares the
/// date complete.
fn declared_error(
    calendar: exchange_hours::ExchangeCalendar,
    date: NaiveDate,
) -> Option<CalendarQueryError> {
    match calendar.coverage().coverage_on(date) {
        DateCoverage::OutsideCoveredRange => Some(CalendarQueryError::OutsideCoveredRange {
            source: calendar.source(),
            date,
        }),
        DateCoverage::UnresolvedGap => Some(CalendarQueryError::UnresolvedGap {
            source: calendar.source(),
            date,
        }),
        DateCoverage::BeforeSupportFloor => Some(CalendarQueryError::BeforeSupportFloor {
            source: calendar.source(),
            date,
        }),
        DateCoverage::Covered | DateCoverage::NormalWeekOnly | _ => None,
    }
}

// ---------------------------------------------------------------------------
// Fence 1 — CME's 2015 Thanksgiving pause and reopen.
//
// Primary source: CME Group, "Globex Thanksgiving Holiday Schedule", last
// updated 10/27/2015, Equity Products section. Retrieved through the public web
// archive because cmegroup.com serves an anti-scraping block:
// https://web.archive.org/web/2016id_/https://www.cmegroup.com/tools-information/holiday-calendar/files/2015-thanksgiving-holiday-schedule.pdf
// (origin https://www.cmegroup.com/tools-information/holiday-calendar/files/2015-thanksgiving-holiday-schedule.pdf).
//
// What it states for Equity Products:
//   Wednesday, Nov 25 — 1600 CT regular close; 1700 CT "Regular open for trade
//                       date Friday, Nov 27", with the footnote "Session orders
//                       entered after 1645 CT ... on Wednesday are for trade
//                       date Friday, Nov 27".
//   Thursday,  Nov 26 — 1200 CT "Trading halt (pre-open)"; 1700 CT "Products
//                       resume trading".
//   Friday,    Nov 27 — 1215 CT "Early close".
//
// So Thanksgiving Thursday carries no trade date at all, and trade date Friday
// Nov 27 is two disjoint tradeable blocks spanning three civil dates with an
// order-entry-only pause between them. No `early_close_ssm`/`late_open_ssm`
// pair can express that. The 0830 CT open of the Friday regular block is the
// profile's own normal RTH open, which the notice does not move; the notice
// moves only its close.
// ---------------------------------------------------------------------------

const CME_THURSDAY: NaiveDate = day(2015, 11, 26);
const CME_FRIDAY: NaiveDate = day(2015, 11, 27);

static CME_FRIDAY_BLOCKS: [ExceptionBlock; 5] = [
    ExceptionBlock::order_entry(-2, 16 * 3_600 + 45 * 60, 17 * 3_600),
    ExceptionBlock::extended(-2, 17 * 3_600, 12 * 3_600),
    ExceptionBlock::order_entry(-1, 12 * 3_600, 17 * 3_600),
    ExceptionBlock::extended(-1, 17 * 3_600, 12 * 3_600 + 15 * 60),
    ExceptionBlock::regular(0, 8 * 3_600 + 30 * 60, 12 * 3_600 + 15 * 60),
];

static CME_RECORDS: [SessionExceptionRecord<'static>; 6] = [
    SessionExceptionRecord::known_normal(day(2015, 11, 23)),
    SessionExceptionRecord::known_normal(day(2015, 11, 24)),
    SessionExceptionRecord::known_normal(day(2015, 11, 25)),
    SessionExceptionRecord::closed(CME_THURSDAY),
    SessionExceptionRecord::replace_sessions(CME_FRIDAY, &CME_FRIDAY_BLOCKS),
    SessionExceptionRecord::known_normal(day(2015, 11, 30)),
];

const CME_TABLE: Result<StaticSessionExceptions<'static>, StaticSessionExceptionsError> =
    StaticSessionExceptions::new(
        CalendarSource::Exchange(Exchange::Cme),
        day(2015, 11, 23),
        day(2015, 11, 30),
        &CME_RECORDS,
    );

fn cme_table() -> StaticSessionExceptions<'static> {
    CME_TABLE.expect("the CME fixture records must be valid")
}

#[test]
fn cme_thanksgiving_2015_pauses_and_reopens_inside_one_trade_date() {
    let table = cme_table();
    let calendar = calendar_for_exchange(Exchange::Cme)
        .with_session_exceptions(&table)
        .expect("the fixture is scoped to this calendar");

    // The caller's own record still states the shape this fence is about: trade
    // date Friday 2015-11-27 is five blocks spanning Wednesday's queue through
    // Friday's early close, with the 12:00-17:00 CT order-entry-only pause on
    // Thanksgiving Thursday between the two executable blocks.
    assert_eq!(
        table.exception_on(CME_FRIDAY),
        DateException::ReplaceSessions(&CME_FRIDAY_BLOCKS)
    );
    assert_eq!(CME_FRIDAY_BLOCKS.len(), 5);
    assert_eq!(CME_FRIDAY_BLOCKS[2].kind(), ExceptionBlockKind::OrderEntry);

    // Every probe below resolves a venue-local 2015 date, and `Exchange::Cme`'s
    // coverage begins at the permanent 2025 floor, so the date-aware surface
    // refuses each of them (LAW-COVERAGE): the engine's answer at these dates is
    // no longer observable through any public surface, and a coverage refusal is
    // not a closure. The identity is named once; every refusal here is Cme's.
    let cme = CalendarSource::Exchange(Exchange::Cme);

    // First block: Wednesday 17:00 CT through Thursday 12:00 CT.
    assert_before_floor(
        calendar.session_bounds(ct((2015, 11, 26), (10, 0, 0))),
        cme,
        CME_THURSDAY,
        "the first block's bounds",
    );
    assert_before_floor(
        calendar.is_open(ct((2015, 11, 26), (11, 59, 59))),
        cme,
        CME_THURSDAY,
        "one second before the pause",
    );

    // The pause is an order-entry-only phase, not a session and not a closure.
    assert_before_floor(
        calendar.is_open(ct((2015, 11, 26), (12, 0, 0))),
        cme,
        CME_THURSDAY,
        "the start of the pause",
    );
    assert_before_floor(
        calendar.is_order_entry_only(ct((2015, 11, 26), (13, 0, 0))),
        cme,
        CME_THURSDAY,
        "inside the pause",
    );
    assert_before_floor(
        calendar.is_accepting_orders(ct((2015, 11, 26), (13, 0, 0))),
        cme,
        CME_THURSDAY,
        "inside the pause",
    );
    assert_before_floor(
        calendar.session_state(ct((2015, 11, 26), (16, 59, 59))),
        cme,
        CME_THURSDAY,
        "the last second of the pause",
    );

    // Second block: Thursday 17:00 CT through the 12:15 CT Friday early close.
    assert_before_floor(
        calendar.session_bounds(ct((2015, 11, 26), (18, 0, 0))),
        cme,
        CME_THURSDAY,
        "the second block's bounds",
    );
    assert_before_floor(
        calendar.is_open_regular(ct((2015, 11, 27), (9, 0, 0))),
        cme,
        CME_FRIDAY,
        "Friday's regular block",
    );
    assert_before_floor(
        calendar.is_open(ct((2015, 11, 27), (12, 14, 59))),
        cme,
        CME_FRIDAY,
        "one second before the early close",
    );
    assert_before_floor(
        calendar.is_open(ct((2015, 11, 27), (12, 15, 0))),
        cme,
        CME_FRIDAY,
        "the early close itself",
    );

    // Both blocks belong to one trade date, and the whole trading day is one
    // daily bar running from Wednesday's 17:00 open to Friday's early close.
    // Each refusal below names **Friday 2015-11-27** — the trade date the
    // caller's record reassigns these instants to — which is the one part of the
    // reassignment the surface still shows: the query resolves the trade date it
    // needs, then refuses to state a date below the floor.
    for instant in [
        ct((2015, 11, 25), (18, 0, 0)),
        ct((2015, 11, 26), (10, 0, 0)),
        ct((2015, 11, 26), (18, 0, 0)),
        ct((2015, 11, 27), (9, 0, 0)),
    ] {
        assert_before_floor(
            calendar.trade_date(instant),
            cme,
            CME_FRIDAY,
            "the reassigned trade date",
        );
    }
    // A pre-floor probe is refused for the **instant's own** venue-local day, not
    // for the day the derived bar would open on: the floor is a fact about the
    // query the caller addressed, and the bar it would have produced is never
    // resolved (LAW-COVERAGE, plan section 6). Both candle adapters on a
    // `PolicyCalendar` now apply that gate exactly as `ExchangeCalendar`'s do.
    assert_before_floor(
        calendar.candle_start(ct((2015, 11, 26), (10, 0, 0)), CalendarResolution::Daily),
        cme,
        day(2015, 11, 26),
        "the daily bar's start",
    );
    // The bar's end is refused with the rest of the query: a pre-floor probe is
    // never partially answered, so the notice's 12:15 CT early close is not
    // observable through this surface on this date.
    assert_before_floor(
        calendar.candle_end(ct((2015, 11, 26), (10, 0, 0)), CalendarResolution::Daily),
        cme,
        CME_THURSDAY,
        "the daily bar's end",
    );

    // What survives of the original fence: the fixed snapshot still states the
    // crate's sourced CME week, in which Thursday 2015-11-26 carries a full
    // regular session. The notice's pause and reopen is caller data and is not
    // in that snapshot, so "the exception removes the normal Thursday session
    // and reassigns Wednesday's queue to Friday" is no longer claimable through
    // any public surface — only the caller's own record below still says it.
    let thursday_afternoon = ct((2015, 11, 26), (13, 0, 0));
    assert!(
        hours_for_exchange(Exchange::Cme, thursday_afternoon).is_open_regular(thursday_afternoon),
        "the fixed snapshot keeps the ordinary Thursday RTH session"
    );
    assert_eq!(
        table.exception_on(CME_THURSDAY),
        DateException::Closed,
        "the caller's record still closes Thursday's trade date"
    );
}

#[test]
fn cme_thanksgiving_thursday_carries_no_trade_date() {
    let table = cme_table();
    let calendar = calendar_for_exchange(Exchange::Cme)
        .with_session_exceptions(&table)
        .expect("the fixture is scoped to this calendar");
    let plain = calendar_for_exchange(Exchange::Cme);

    // The normal week has a full Thursday RTH session; the exception removes it.
    // The fixed snapshot still states the normal week, so that half of the claim
    // survives; the removal itself is caller data on a date the identity refuses,
    // so it is only stated by the record vocabulary below.
    let thursday_afternoon = ct((2015, 11, 26), (13, 0, 0));
    assert!(
        hours_for_exchange(Exchange::Cme, thursday_afternoon).is_open_regular(thursday_afternoon),
        "the fixed snapshot keeps the ordinary Thursday RTH session"
    );
    assert_eq!(
        table.exception_on(CME_THURSDAY),
        DateException::Closed,
        "the caller's record removes that trade date"
    );

    // Every date-aware probe in this test resolves a venue-local 2015 date, and
    // `Exchange::Cme`'s coverage begins at the permanent 2025 floor: each one
    // refuses with the same error, on both the excepted calendar and the plain
    // one, because the caller's layer cannot change a coverage verdict
    // (LAW-COVERAGE). "Thanksgiving Thursday carries no trade date" is no longer
    // a claim any public surface can state.
    let cme = CalendarSource::Exchange(Exchange::Cme);
    assert_before_floor(
        calendar.is_open(thursday_afternoon),
        cme,
        CME_THURSDAY,
        "the excepted calendar's Thursday session",
    );
    assert_before_floor(
        calendar.is_closed_trade_date(CME_THURSDAY, SessionKind::Both),
        cme,
        CME_THURSDAY,
        "the excepted calendar's trade date",
    );
    assert_before_floor(
        plain.is_closed_trade_date(CME_THURSDAY, SessionKind::Both),
        cme,
        CME_THURSDAY,
        "the plain calendar's trade date",
    );

    // Wednesday's own trade date is audited normal and is left alone.
    assert_before_floor(
        calendar.is_open_regular(ct((2015, 11, 25), (10, 0, 0))),
        cme,
        day(2015, 11, 25),
        "Wednesday's regular session",
    );
    assert_before_floor(
        calendar.trade_date(ct((2015, 11, 25), (10, 0, 0))),
        cme,
        day(2015, 11, 25),
        "Wednesday's trade date",
    );
    assert_before_floor(
        calendar.session_bounds(ct((2015, 11, 25), (15, 45, 0))),
        cme,
        day(2015, 11, 25),
        "Wednesday's closing session bounds",
    );

    // The Wednesday-evening queue is reassigned to Friday's trade date, which
    // is exactly what the notice's 1645 CT footnote states.
    assert_before_floor(
        calendar.is_order_entry_only(ct((2015, 11, 25), (16, 50, 0))),
        cme,
        day(2015, 11, 25),
        "Wednesday's evening queue",
    );
    // The queue's trade date is resolved by probing the order-entry phase, and
    // the Cme identity declares a phase-level gap on the withheld Sunday
    // quarter-hour (#79). The floor is decided before the phase, so a pre-floor
    // probe of either kind refuses as `BeforeSupportFloor` on the instant's own
    // venue-local day (2015-11-25), and the reassignment to Friday's trade date
    // is not observable here either.
    assert_before_floor(
        calendar.trade_date(ct((2015, 11, 25), (16, 50, 0))),
        cme,
        day(2015, 11, 25),
        "the queue's reassigned trade date",
    );

    // The following Monday is outside the exceptional run: the refusal is the
    // same on both calendars, which is all "unchanged" can mean once neither
    // surface answers the date.
    assert_before_floor(
        calendar.is_open_regular(ct((2015, 11, 30), (10, 0, 0))),
        cme,
        day(2015, 11, 30),
        "the following Monday",
    );
    assert_eq!(
        calendar.session_bounds(ct((2015, 11, 30), (10, 0, 0))),
        plain.session_bounds(ct((2015, 11, 30), (10, 0, 0))),
        "the recordless Monday refuses identically with and without the layer"
    );
}

// ---------------------------------------------------------------------------
// Fence 2 — a regular-only early close with extended trading continuing.
//
// Sourced facts. The NASDAQ OMX holiday calendar captured 2011-11-10 lists,
// for the NASDAQ OMX U.S. Equity Markets (NASDAQ, BX and PSX), "November 24 —
// Thanksgiving Day — Closed" and "November 25 — Early Market Close*", with the
// footnote that NASDAQ OMX "will continue to send alerts to notify customers of
// days when the Market will close early. Please refer to those alerts for full
// information, including system operating times."
// https://web.archive.org/web/20111110145837id_/http://www.nasdaqtrader.com/trader.aspx?id=calendar
//
// NOT sourced. `docs/schedules/date-exceptions.md` cites Equity Trader Alert
// ETA2011-54 for the system operating times behind that early close. That alert
// is not in the public web archive (checked via the CDX index for
// nasdaqtrader.com/TraderNews.aspx: 2011-51, -52, -53, -58 and -60 are held,
// -54 is not), and nasdaqtrader.com itself serves a bot interstitial. The 13:00
// and 17:00 ET wall clocks below are therefore fixture values chosen to
// exercise the shape the alert is cited for — a regular close with extended
// trading continuing past it — and are not asserted as sourced times. The
// 07:00 ET pre-market open is the crate's own sourced Nasdaq grid for 2011.
// ---------------------------------------------------------------------------

const NASDAQ_THANKSGIVING: NaiveDate = day(2011, 11, 24);
const NASDAQ_HALF_DAY: NaiveDate = day(2011, 11, 25);

static NASDAQ_HALF_DAY_BLOCKS: [ExceptionBlock; 3] = [
    ExceptionBlock::extended(0, 7 * 3_600, 9 * 3_600 + 30 * 60),
    ExceptionBlock::regular(0, 9 * 3_600 + 30 * 60, 13 * 3_600),
    ExceptionBlock::extended(0, 13 * 3_600, 17 * 3_600),
];

static NASDAQ_RECORDS: [SessionExceptionRecord<'static>; 2] = [
    SessionExceptionRecord::closed(NASDAQ_THANKSGIVING),
    SessionExceptionRecord::replace_sessions(NASDAQ_HALF_DAY, &NASDAQ_HALF_DAY_BLOCKS),
];

const NASDAQ_TABLE: Result<StaticSessionExceptions<'static>, StaticSessionExceptionsError> =
    StaticSessionExceptions::new(
        CalendarSource::Exchange(Exchange::Nasdaq),
        day(2011, 11, 21),
        day(2011, 11, 28),
        &NASDAQ_RECORDS,
    );

fn nasdaq_table() -> StaticSessionExceptions<'static> {
    NASDAQ_TABLE.expect("the Nasdaq fixture records must be valid")
}

#[test]
fn nasdaq_regular_only_early_close_keeps_extended_trading_open() {
    let table = nasdaq_table();
    let calendar = calendar_for_exchange(Exchange::Nasdaq)
        .with_session_exceptions(&table)
        .expect("the fixture is scoped to this calendar");

    // The caller's replacement still states the shape this fence is about: a
    // pre-market extended block, a regular block closing at 13:00 ET, and an
    // afternoon extended block that keeps trading to 17:00 ET.
    assert_eq!(
        table.exception_on(NASDAQ_HALF_DAY),
        DateException::ReplaceSessions(&NASDAQ_HALF_DAY_BLOCKS)
    );
    assert_eq!(
        NASDAQ_HALF_DAY_BLOCKS[0].kind(),
        ExceptionBlockKind::Extended
    );
    assert_eq!(
        NASDAQ_HALF_DAY_BLOCKS[1].kind(),
        ExceptionBlockKind::Regular
    );
    assert_eq!(NASDAQ_HALF_DAY_BLOCKS[1].close_ssm(), 13 * 3_600);
    assert_eq!(
        NASDAQ_HALF_DAY_BLOCKS[2].kind(),
        ExceptionBlockKind::Extended
    );
    assert_eq!(NASDAQ_HALF_DAY_BLOCKS[2].close_ssm(), 17 * 3_600);

    // Every probe below resolves the venue-local 2011-11-25, which precedes the
    // permanent 2025 support floor, so the date-aware surface refuses each of
    // them (LAW-COVERAGE). Whether the afternoon extended block survives the
    // 13:00 ET regular close is therefore no longer observable through this
    // surface — and, as above, the caller's own data is what still states it.
    let nasdaq = CalendarSource::Exchange(Exchange::Nasdaq);
    assert_before_floor(
        calendar.is_open_extended(et((2011, 11, 25), (8, 0, 0))),
        nasdaq,
        NASDAQ_HALF_DAY,
        "the pre-market block",
    );
    assert_before_floor(
        calendar.is_open_regular(et((2011, 11, 25), (12, 59, 59))),
        nasdaq,
        NASDAQ_HALF_DAY,
        "one second before the regular close",
    );
    assert_before_floor(
        calendar.is_open_regular(et((2011, 11, 25), (13, 0, 0))),
        nasdaq,
        NASDAQ_HALF_DAY,
        "the regular close itself",
    );
    assert_before_floor(
        calendar.is_open_extended(et((2011, 11, 25), (13, 0, 0))),
        nasdaq,
        NASDAQ_HALF_DAY,
        "the afternoon block's open",
    );
    assert_before_floor(
        calendar.is_open(et((2011, 11, 25), (16, 59, 59))),
        nasdaq,
        NASDAQ_HALF_DAY,
        "one second before the extended close",
    );
    assert_before_floor(
        calendar.is_open(et((2011, 11, 25), (17, 0, 0))),
        nasdaq,
        NASDAQ_HALF_DAY,
        "the extended close itself",
    );

    assert_before_floor(
        calendar.session_bounds_with(et((2011, 11, 25), (10, 0, 0)), SessionKind::Regular),
        nasdaq,
        NASDAQ_HALF_DAY,
        "the regular block's bounds",
    );
    assert_before_floor(
        calendar.session_bounds_with(et((2011, 11, 25), (14, 0, 0)), SessionKind::Extended),
        nasdaq,
        NASDAQ_HALF_DAY,
        "the afternoon block's bounds",
    );

    // The regular and full trading days end at different instants, which is
    // precisely what one scalar `early_close_ssm` cannot say. Both candle
    // adapters apply the same pre-floor gate as the rest of the surface — they
    // were the one seam where an overlay could still answer a pre-floor probe,
    // and it is closed — so the separation between the two closes is stated by
    // the caller's own blocks above and is not observable on this date.
    assert_before_floor(
        calendar.candle_end_with(
            et((2011, 11, 25), (10, 0, 0)),
            CalendarResolution::Daily,
            SessionKind::Regular,
        ),
        nasdaq,
        NASDAQ_HALF_DAY,
        "the overlay's regular daily close",
    );
    assert_before_floor(
        calendar.candle_end(et((2011, 11, 25), (10, 0, 0)), CalendarResolution::Daily),
        nasdaq,
        NASDAQ_HALF_DAY,
        "the overlay's full daily close",
    );
    assert_before_floor(
        calendar.calendar().candle_end_with(
            et((2011, 11, 25), (10, 0, 0)),
            CalendarResolution::Daily,
            SessionKind::Regular,
        ),
        nasdaq,
        NASDAQ_HALF_DAY,
        "the same identity's regular daily close without the overlay",
    );
    assert_before_floor(
        calendar.trade_date(et((2011, 11, 25), (14, 0, 0))),
        nasdaq,
        NASDAQ_HALF_DAY,
        "the half day's trade date",
    );

    // What the fixed snapshot still states: nothing in the crate's own Nasdaq
    // data makes 2011-11-25 an early close, so it remains an ordinary full
    // Friday there. The early close is caller data.
    let half_day = et((2011, 11, 25), (14, 0, 0));
    assert!(
        hours_for_exchange(Exchange::Nasdaq, half_day).is_open(half_day),
        "the fixed snapshot has no early close on this date"
    );
}

#[test]
fn a_scalar_early_close_cannot_express_the_nasdaq_half_day() {
    let overrides = [DayOverride::early_close(NASDAQ_HALF_DAY, 13 * 3_600)];
    let policy = StaticDayPolicy::new(&overrides).expect("valid override");
    let clipped = calendar_for_exchange(Exchange::Nasdaq).with_day_policy(&policy);
    let table = nasdaq_table();
    let replaced = calendar_for_exchange(Exchange::Nasdaq)
        .with_session_exceptions(&table)
        .expect("the fixture is scoped to this calendar");

    // The distinction this fence names is a fact about the two caller-owned
    // shapes, and it survives intact: the policy supplies one scalar clip, and
    // the replacement supplies a third block whose extended leg runs on to
    // 17:00 ET after the regular block closes at 13:00 ET.
    assert_eq!(overrides.len(), 1);
    assert_eq!(NASDAQ_HALF_DAY_BLOCKS.len(), 3);
    assert_eq!(NASDAQ_HALF_DAY_BLOCKS[1].close_ssm(), 13 * 3_600);
    assert_eq!(NASDAQ_HALF_DAY_BLOCKS[2].close_ssm(), 17 * 3_600);
    assert!(NASDAQ_HALF_DAY_BLOCKS[2].open_ssm() < NASDAQ_HALF_DAY_BLOCKS[2].close_ssm());

    // Both calendars refuse the venue-local 2011-11-25, which precedes the
    // permanent 2025 support floor (LAW-COVERAGE). What the two surfaces do to
    // that date — one clipping the whole day at 13:00 ET, the other keeping the
    // afternoon — is therefore no longer observable, and a coverage refusal is
    // not a closure.
    let nasdaq = CalendarSource::Exchange(Exchange::Nasdaq);
    assert_before_floor(
        clipped.is_open_regular(et((2011, 11, 25), (13, 0, 0))),
        nasdaq,
        NASDAQ_HALF_DAY,
        "the clipped calendar's regular close",
    );
    assert_before_floor(
        replaced.is_open_regular(et((2011, 11, 25), (13, 0, 0))),
        nasdaq,
        NASDAQ_HALF_DAY,
        "the replaced calendar's regular close",
    );
    assert_before_floor(
        clipped.is_open(et((2011, 11, 25), (14, 0, 0))),
        nasdaq,
        NASDAQ_HALF_DAY,
        "the clipped calendar's afternoon",
    );
    assert_before_floor(
        replaced.is_open_extended(et((2011, 11, 25), (14, 0, 0))),
        nasdaq,
        NASDAQ_HALF_DAY,
        "the replaced calendar's afternoon",
    );
}

#[test]
fn a_closed_record_removes_the_whole_trading_day() {
    let table = nasdaq_table();
    let calendar = calendar_for_exchange(Exchange::Nasdaq)
        .with_session_exceptions(&table)
        .expect("the fixture is scoped to this calendar");

    // The caller's record still closes the trade date, and that vocabulary is
    // the only surface left on which the closure is stated.
    assert_eq!(
        table.exception_on(NASDAQ_THANKSGIVING),
        DateException::Closed
    );

    // Every probe below resolves a venue-local 2011 date, before the permanent
    // 2025 support floor, so the date-aware surface refuses each of them
    // (LAW-COVERAGE). "The record removes the whole trading day including its
    // after-midnight tail" is no longer observable: the next open it used to
    // produce cannot be reached through this surface.
    let nasdaq = CalendarSource::Exchange(Exchange::Nasdaq);
    assert_before_floor(
        calendar.is_open(et((2011, 11, 24), (10, 0, 0))),
        nasdaq,
        NASDAQ_THANKSGIVING,
        "Thanksgiving morning",
    );
    assert_before_floor(
        calendar.is_open(et((2011, 11, 24), (18, 0, 0))),
        nasdaq,
        NASDAQ_THANKSGIVING,
        "Thanksgiving evening",
    );
    assert_before_floor(
        calendar.is_closed_trade_date(NASDAQ_THANKSGIVING, SessionKind::Both),
        nasdaq,
        NASDAQ_THANKSGIVING,
        "the closed trade date",
    );
    assert_before_floor(
        calendar.is_closed_all_day_on(NASDAQ_THANKSGIVING, SessionKind::Both),
        nasdaq,
        NASDAQ_THANKSGIVING,
        "the closed all-day window",
    );
    assert_before_floor(
        calendar.next_session_open_after(et((2011, 11, 23), (20, 0, 0))),
        nasdaq,
        day(2011, 11, 23),
        "the forward scan to the next open",
    );

    // The fixed snapshot still states the ordinary Wednesday session the record
    // removes, which is the other half of the original claim.
    let wednesday = et((2011, 11, 24), (10, 0, 0));
    assert!(
        hours_for_exchange(Exchange::Nasdaq, wednesday).is_open_regular(wednesday),
        "the fixed snapshot has no closure on this date"
    );
}

// ---------------------------------------------------------------------------
// Precedence: the caller's `DayPolicy` overlays the exception layer exactly as
// it overlays a normal week. Two replacement layers never compose.
// ---------------------------------------------------------------------------

#[test]
fn a_day_policy_clips_a_replaced_trading_day() {
    let table = nasdaq_table();
    let overrides = [DayOverride::early_close(NASDAQ_HALF_DAY, 11 * 3_600)];
    let policy = StaticDayPolicy::new(&overrides).expect("valid override");
    let calendar = calendar_for_exchange(Exchange::Nasdaq)
        .with_session_exceptions(&table)
        .expect("the fixture is scoped to this calendar")
        .with_day_policy(&policy);

    // Both caller layers are attached, and the policy still carries its single
    // 11:00 ET clip; that much is a fact about the fixture.
    assert!(calendar.has_session_exceptions());
    assert!(calendar.has_day_policy());
    assert_eq!(overrides.len(), 1);

    // The date is venue-local 2011-11-25, before the permanent 2025 support
    // floor, so the composed surface refuses every probe (LAW-COVERAGE).
    // Whether the 11:00 ET clip composes over the replacement blocks is no
    // longer observable through this surface, and a coverage refusal is not a
    // clip.
    let nasdaq = CalendarSource::Exchange(Exchange::Nasdaq);
    assert_before_floor(
        calendar.is_open_regular(et((2011, 11, 25), (10, 59, 59))),
        nasdaq,
        NASDAQ_HALF_DAY,
        "one second before the policy's clip",
    );
    assert_before_floor(
        calendar.is_open(et((2011, 11, 25), (11, 0, 0))),
        nasdaq,
        NASDAQ_HALF_DAY,
        "the policy's clip itself",
    );
    assert_before_floor(
        calendar.is_open(et((2011, 11, 25), (14, 0, 0))),
        nasdaq,
        NASDAQ_HALF_DAY,
        "the afternoon the clip would remove",
    );
    // The candle adapter applies the same pre-floor gate as the rest of the
    // surface, so the composed bar close — the 11:00 ET clip over the
    // replacement — is not observable below the floor either.
    assert_before_floor(
        calendar.candle_end(et((2011, 11, 25), (10, 0, 0)), CalendarResolution::Daily),
        nasdaq,
        NASDAQ_HALF_DAY,
        "the overlay's clipped daily close",
    );

    // The fixed snapshot still states the ordinary Friday it would have clipped.
    let afternoon = et((2011, 11, 25), (14, 0, 0));
    assert!(
        hours_for_exchange(Exchange::Nasdaq, afternoon).is_open(afternoon),
        "the fixed snapshot trades through 14:00 ET on an ordinary Friday"
    );
}

#[test]
fn a_day_policy_late_open_delays_a_replaced_trading_day() {
    let table = nasdaq_table();
    let overrides = [DayOverride::late_open(NASDAQ_HALF_DAY, 11 * 3_600)];
    let policy = StaticDayPolicy::new(&overrides).expect("valid override");
    let calendar = calendar_for_exchange(Exchange::Nasdaq)
        .with_session_exceptions(&table)
        .expect("the fixture is scoped to this calendar")
        .with_day_policy(&policy);

    // The caller's late-open layer is attached and carries its 11:00 ET onset.
    assert!(calendar.has_day_policy());
    assert!(calendar.has_session_exceptions());
    assert_eq!(overrides.len(), 1);

    // Venue-local 2011-11-25 is before the support floor, so the composed
    // surface refuses all three probes (LAW-COVERAGE); the delayed onset the
    // policy produces is no longer observable.
    let nasdaq = CalendarSource::Exchange(Exchange::Nasdaq);
    assert_before_floor(
        calendar.is_open(et((2011, 11, 25), (10, 59, 59))),
        nasdaq,
        NASDAQ_HALF_DAY,
        "one second before the policy's late open",
    );
    assert_before_floor(
        calendar.is_open_regular(et((2011, 11, 25), (11, 0, 0))),
        nasdaq,
        NASDAQ_HALF_DAY,
        "the policy's late open itself",
    );
    assert_before_floor(
        calendar.is_open_extended(et((2011, 11, 25), (14, 0, 0))),
        nasdaq,
        NASDAQ_HALF_DAY,
        "the afternoon behind the delayed open",
    );

    // The fixed snapshot still states the ordinary Friday morning the late open
    // would have delayed.
    let morning = et((2011, 11, 25), (10, 59, 59));
    assert!(
        hours_for_exchange(Exchange::Nasdaq, morning).is_open(morning),
        "the fixed snapshot trades through 10:59:59 ET on an ordinary Friday"
    );
}

#[test]
fn a_day_policy_closure_beats_a_replacement_record() {
    let table = nasdaq_table();
    let overrides = [DayOverride::closed(NASDAQ_HALF_DAY)];
    let policy = StaticDayPolicy::new(&overrides).expect("valid override");
    let calendar = calendar_for_exchange(Exchange::Nasdaq)
        .with_session_exceptions(&table)
        .expect("the fixture is scoped to this calendar")
        .with_day_policy(&policy);

    // The closure reaches the exception layer as a record-level fact: the table
    // still carries the caller's replacement, and the policy is attached over
    // it. Which of the two wins is decided by the date-aware engine, and that
    // answer is no longer observable at this date.
    assert_eq!(
        table.exception_on(NASDAQ_HALF_DAY),
        DateException::ReplaceSessions(&NASDAQ_HALF_DAY_BLOCKS)
    );
    assert!(calendar.has_day_policy());
    assert!(calendar.has_session_exceptions());

    // Venue-local 2011-11-25 is before the permanent 2025 support floor, so all
    // three probes refuse (LAW-COVERAGE): "the closure wins" cannot be stated
    // through a surface that refuses the date.
    let nasdaq = CalendarSource::Exchange(Exchange::Nasdaq);
    assert_before_floor(
        calendar.is_open(et((2011, 11, 25), (10, 0, 0))),
        nasdaq,
        NASDAQ_HALF_DAY,
        "the morning the closure removes",
    );
    assert_before_floor(
        calendar.is_open(et((2011, 11, 25), (14, 0, 0))),
        nasdaq,
        NASDAQ_HALF_DAY,
        "the afternoon the closure removes",
    );
    assert_before_floor(
        calendar.is_closed_trade_date(NASDAQ_HALF_DAY, SessionKind::Both),
        nasdaq,
        NASDAQ_HALF_DAY,
        "the closed trade date",
    );
}

#[test]
fn attaching_a_second_provider_replaces_the_first() {
    let closing = [SessionExceptionRecord::closed(NASDAQ_HALF_DAY)];
    let closing_table = StaticSessionExceptions::new(
        CalendarSource::Exchange(Exchange::Nasdaq),
        day(2011, 11, 21),
        day(2011, 11, 28),
        &closing,
    )
    .expect("valid records");
    let replacing = nasdaq_table();

    let calendar = calendar_for_exchange(Exchange::Nasdaq)
        .with_session_exceptions(&closing_table)
        .expect("the fixture is scoped to this calendar")
        .with_session_exceptions(&replacing)
        .expect("the fixture is scoped to this calendar");

    // The second provider's replacement stands; the first provider's closure
    // did not survive to compose with it. That is a fact about the calendar's
    // own layer, and it is still readable here.
    assert_eq!(
        calendar.session_exception_on(NASDAQ_HALF_DAY),
        Some(DateException::ReplaceSessions(&NASDAQ_HALF_DAY_BLOCKS))
    );

    // What the surviving layer does to the date is no longer observable:
    // venue-local 2011-11-25 precedes the permanent 2025 support floor, so the
    // query refuses rather than evaluating either provider (LAW-COVERAGE).
    assert_before_floor(
        calendar.is_open_regular(et((2011, 11, 25), (10, 0, 0))),
        CalendarSource::Exchange(Exchange::Nasdaq),
        NASDAQ_HALF_DAY,
        "the surviving replacement's regular block",
    );
}

// ---------------------------------------------------------------------------
// Identity conventions the replacement layer must keep working.
// ---------------------------------------------------------------------------

#[test]
fn a_closed_crypto_monday_still_rolls_the_weekend_to_the_next_business_date() {
    let monday = day(2026, 6, 8);
    let tuesday = day(2026, 6, 9);
    let records = [SessionExceptionRecord::closed(monday)];
    let table = StaticSessionExceptions::new(
        CalendarSource::MarketHoursKey(MarketHoursKey::GlobexCryptocurrency),
        day(2026, 6, 1),
        day(2026, 6, 14),
        &records,
    )
    .expect("valid records");
    let calendar = calendar_for_market_hours_key(MarketHoursKey::GlobexCryptocurrency)
        .with_session_exceptions(&table)
        .expect("the fixture is scoped to this calendar");

    for instant in [
        ct((2026, 6, 5), (17, 0, 0)),
        ct((2026, 6, 6), (1, 0, 0)),
        ct((2026, 6, 7), (12, 0, 0)),
        ct((2026, 6, 8), (10, 0, 0)),
    ] {
        assert!(
            calendar
                .is_open(instant)
                .expect("the coverage contract must answer a covered date"),
            "weekend trading vanished at {instant}"
        );
        assert_eq!(
            calendar
                .trade_date(instant)
                .expect("the coverage contract must answer a covered date"),
            Some(tuesday)
        );
    }
    assert!(
        calendar
            .is_closed_trade_date(monday, SessionKind::Both)
            .expect("the coverage contract must answer a covered date")
    );
    assert_eq!(
        calendar
            .candle_start(ct((2026, 6, 7), (12, 0, 0)), CalendarResolution::Daily)
            .expect("the coverage contract must answer a covered date"),
        Some(ct((2026, 6, 5), (16, 2, 0)))
    );
    assert_eq!(
        calendar
            .candle_end(ct((2026, 6, 7), (12, 0, 0)), CalendarResolution::Daily)
            .expect("the coverage contract must answer a covered date"),
        Some(ct((2026, 6, 9), (16, 0, 0)))
    );
}

static ALWAYS_OPEN_BLOCKS: [ExceptionBlock; 1] =
    [ExceptionBlock::regular(0, 9 * 3_600, 13 * 3_600)];

#[test]
fn a_profile_without_a_daily_close_ignores_the_replacement_layer() {
    // A trade-date-keyed overlay has nothing to attach to on a schedule with no
    // final daily close, exactly as `DayPolicy` has nothing to clip there.
    let trade_date = day(2026, 4, 20);
    let records = [SessionExceptionRecord::replace_sessions(
        trade_date,
        &ALWAYS_OPEN_BLOCKS,
    )];
    let table = StaticSessionExceptions::new(
        CalendarSource::Exchange(Exchange::BinanceFutures),
        day(2026, 4, 13),
        day(2026, 4, 27),
        &records,
    )
    .expect("valid records");
    let calendar = calendar_for_exchange(Exchange::BinanceFutures)
        .with_session_exceptions(&table)
        .expect("the fixture is scoped to this calendar");
    let plain = calendar_for_exchange(Exchange::BinanceFutures);

    for hour in [3_u32, 10, 14, 22] {
        let instant = Utc
            .with_ymd_and_hms(2026, 4, 20, hour, 0, 0)
            .single()
            .expect("valid UTC instant");
        assert!(
            calendar
                .is_open(instant)
                .expect("the coverage contract must answer a covered date"),
            "24x7 trading vanished at {hour}"
        );
        assert_eq!(
            calendar
                .session_bounds(instant)
                .expect("the coverage contract must answer a covered date"),
            plain
                .session_bounds(instant)
                .expect("the coverage contract must answer a covered date")
        );
    }
    // Unchanged from the no-overlay answer: an always-open profile has no trade
    // date to close or replace.
    assert_eq!(
        calendar
            .is_closed_trade_date(trade_date, SessionKind::Both)
            .expect("the coverage contract must answer a covered date"),
        plain
            .is_closed_trade_date(trade_date, SessionKind::Both)
            .expect("the coverage contract must answer a covered date")
    );

    // The same holds for a closed record. The layer declines to govern the
    // date at all rather than resolving no blocks and calling that no session,
    // so the refusal cannot come out as a closure the profile cannot express.
    let closed = [SessionExceptionRecord::closed(trade_date)];
    let closed_table = StaticSessionExceptions::new(
        CalendarSource::Exchange(Exchange::BinanceFutures),
        day(2026, 4, 13),
        day(2026, 4, 27),
        &closed,
    )
    .expect("valid records");
    let closed_calendar = calendar_for_exchange(Exchange::BinanceFutures)
        .with_session_exceptions(&closed_table)
        .expect("the fixture is scoped to this calendar");
    for hour in [3_u32, 10, 14, 22] {
        let instant = Utc
            .with_ymd_and_hms(2026, 4, 20, hour, 0, 0)
            .single()
            .expect("valid UTC instant");
        assert!(
            closed_calendar
                .is_open(instant)
                .expect("the coverage contract must answer a covered date"),
            "a closed record removed 24x7 trading at {hour}"
        );
        assert_eq!(
            closed_calendar
                .session_bounds(instant)
                .expect("the coverage contract must answer a covered date"),
            plain
                .session_bounds(instant)
                .expect("the coverage contract must answer a covered date")
        );
    }
    assert_eq!(
        closed_calendar
            .is_closed_trade_date(trade_date, SessionKind::Both)
            .expect("the coverage contract must answer a covered date"),
        plain
            .is_closed_trade_date(trade_date, SessionKind::Both)
            .expect("the coverage contract must answer a covered date")
    );
}

#[test]
fn hours_at_still_returns_the_unmodified_sourced_profile() {
    let table = nasdaq_table();
    let calendar = calendar_for_exchange(Exchange::Nasdaq)
        .with_session_exceptions(&table)
        .expect("the fixture is scoped to this calendar");
    let instant = et((2011, 11, 25), (10, 0, 0));

    assert_eq!(
        calendar.hours_at(instant),
        calendar_for_exchange(Exchange::Nasdaq).hours_at(instant)
    );
    assert_eq!(
        calendar.source(),
        CalendarSource::Exchange(Exchange::Nasdaq)
    );
    assert_eq!(calendar.exchange(), Some(Exchange::Nasdaq));
    assert_eq!(calendar.market_hours_key(), None);
}

// ---------------------------------------------------------------------------
// Coverage, scope, and record vocabulary.
// ---------------------------------------------------------------------------

#[test]
fn coverage_separates_an_audited_normal_date_from_an_unaudited_one() {
    let table = nasdaq_table();
    let coverage = provider_coverage(&table).expect("the fixture publishes coverage");

    assert_eq!(coverage.first(), day(2011, 11, 21));
    assert_eq!(coverage.last(), day(2011, 11, 28));
    assert!(coverage.contains(day(2011, 11, 23)));
    assert!(!coverage.contains(day(2011, 11, 29)));

    assert_eq!(
        table.exception_on(day(2011, 11, 23)),
        DateException::KnownNormal
    );
    assert_eq!(
        table.exception_on(day(2011, 11, 29)),
        DateException::OutOfCoverage
    );
    assert_eq!(
        table.exception_on(NASDAQ_THANKSGIVING),
        DateException::Closed
    );
    assert_eq!(
        table.exception_on(NASDAQ_HALF_DAY),
        DateException::ReplaceSessions(&NASDAQ_HALF_DAY_BLOCKS)
    );

    // Both KnownNormal and OutOfCoverage serve the normal week at runtime, so
    // the distinction is only reachable through the provider surface — and it is
    // now only reachable there. Venue-local 2011-11-23 and 2011-11-29 both
    // precede the permanent 2025 support floor, so the date-aware surface
    // refuses them for that reason alone and never reaches the provider's
    // verdict at all (LAW-COVERAGE). The runtime half of the original claim is
    // gone; the provider half above is untouched.
    let calendar = calendar_for_exchange(Exchange::Nasdaq)
        .with_session_exceptions(&table)
        .expect("the fixture is scoped to this calendar");
    let nasdaq = CalendarSource::Exchange(Exchange::Nasdaq);
    assert_before_floor(
        calendar.is_open_regular(et((2011, 11, 23), (14, 0, 0))),
        nasdaq,
        day(2011, 11, 23),
        "an audited-normal date",
    );
    assert_before_floor(
        calendar.is_open_regular(et((2011, 11, 29), (14, 0, 0))),
        nasdaq,
        day(2011, 11, 29),
        "an unaudited date",
    );
    assert_eq!(
        calendar.session_exception_on(day(2011, 11, 29)),
        Some(DateException::OutOfCoverage)
    );
    assert_eq!(
        calendar_for_exchange(Exchange::Nasdaq)
            .with_day_policy(&StaticDayPolicy::new(&[]).expect("empty is valid"))
            .session_exception_on(day(2011, 11, 29)),
        None
    );
}

#[test]
fn a_provider_scoped_to_another_identity_is_refused() {
    let table = cme_table();
    let error = calendar_for_exchange(Exchange::Nasdaq)
        .with_session_exceptions(&table)
        .expect_err("a CME table must not drive a Nasdaq calendar");

    assert_eq!(error.calendar, CalendarSource::Exchange(Exchange::Nasdaq));
    assert_eq!(error.provider, CalendarSource::Exchange(Exchange::Cme));
    assert!(error.to_string().contains("Nasdaq"));
    assert_eq!(
        calendar_for_market_hours_key(MarketHoursKey::GlobexEquityIndex)
            .with_session_exceptions(&table)
            .err()
            .map(|error| error.provider),
        Some(CalendarSource::Exchange(Exchange::Cme))
    );
}

#[test]
fn records_and_blocks_expose_only_their_selected_state() {
    assert_copy_send_sync_static::<ExceptionBlock>();
    assert_copy_send_sync_static::<SessionExceptionRecord<'static>>();
    assert_copy_send_sync_static::<StaticSessionExceptions<'static>>();
    assert_copy_send_sync_static::<ExceptionCoverage>();

    let table = cme_table();
    assert_eq!(table.source(), CalendarSource::Exchange(Exchange::Cme));
    assert_eq!(table.records(), &CME_RECORDS);
    let record = table.record_on(CME_FRIDAY).expect("Friday has a record");
    assert_eq!(record.trade_date(), CME_FRIDAY);
    assert_eq!(
        record.exception(),
        DateException::ReplaceSessions(&CME_FRIDAY_BLOCKS)
    );
    assert_eq!(table.record_on(day(2015, 11, 28)), None);

    let pause = CME_FRIDAY_BLOCKS[2];
    assert_eq!(pause.kind(), ExceptionBlockKind::OrderEntry);
    assert_eq!(pause.open_day_offset(), -1);
    assert_eq!(pause.open_ssm(), 12 * 3_600);
    assert_eq!(pause.close_ssm(), 17 * 3_600);
    assert!(!pause.wraps_to_next_day());
    assert!(CME_FRIDAY_BLOCKS[1].wraps_to_next_day());
    assert_eq!(CME_FRIDAY_BLOCKS[1].kind(), ExceptionBlockKind::Extended);
    assert_eq!(CME_FRIDAY_BLOCKS[4].kind(), ExceptionBlockKind::Regular);

    assert_eq!(ExceptionCoverage::new(CME_FRIDAY, CME_THURSDAY), None);
    assert!(ExceptionCoverage::new(CME_THURSDAY, CME_THURSDAY).is_some());
}

#[test]
fn an_attached_but_recordless_layer_changes_no_answer() {
    let table = StaticSessionExceptions::new(
        CalendarSource::Exchange(Exchange::Cme),
        day(2015, 11, 1),
        day(2015, 12, 31),
        &[],
    )
    .expect("an empty record slice is valid");
    let calendar = calendar_for_exchange(Exchange::Cme)
        .with_session_exceptions(&table)
        .expect("the fixture is scoped to this calendar");
    let plain = calendar_for_exchange(Exchange::Cme);

    assert!(calendar.has_session_exceptions());
    assert!(!calendar.has_day_policy());
    assert!(format!("{calendar:?}").contains("session_exceptions: true"));

    // The layer spans a 2015 window, so every probe below is a venue-local 2015
    // date and both calendars refuse it: the claim "the recordless layer changes
    // no answer" survives for the state, bounds and trade-date queries, because
    // the answer it must not change is now the coverage refusal (LAW-COVERAGE).
    // The refusal is stated once below, so the test cannot pass by comparing two
    // refusals it never named.
    //
    // The layer changes no answer of any kind, refusals included: the overlay's
    // `candle_end` applies the same pre-floor gate as the plain identity's and
    // refuses the same day, so the claim holds for every entry point this test
    // probes.
    let cme = CalendarSource::Exchange(Exchange::Cme);
    let mut instant = ct((2015, 11, 22), (0, 0, 0));
    let end = ct((2015, 11, 30), (0, 0, 0));
    while instant < end {
        assert_eq!(
            calendar.session_state(instant),
            plain.session_state(instant),
            "state diverged at {instant}"
        );
        assert_eq!(
            calendar.session_bounds(instant),
            plain.session_bounds(instant),
            "bounds diverged at {instant}"
        );
        assert_eq!(
            calendar.trade_date(instant),
            plain.trade_date(instant),
            "trade date diverged at {instant}"
        );
        assert_before_floor(
            plain.candle_end(instant, CalendarResolution::Daily),
            cme,
            instant.with_timezone(&US::Central).date_naive(),
            "the plain calendar's daily close",
        );
        assert_before_floor(
            calendar.candle_end(instant, CalendarResolution::Daily),
            cme,
            instant.with_timezone(&US::Central).date_naive(),
            "the overlay's daily close",
        );
        instant += chrono::TimeDelta::minutes(37);
    }
    assert_before_floor(
        calendar.session_state(ct((2015, 11, 22), (0, 0, 0))),
        cme,
        day(2015, 11, 22),
        "a probe inside the recordless window",
    );
    // The recordless layer changes no answer, and that now includes the candle
    // adapter: the overlay and the plain identity refuse the same probe for the
    // same day.
    assert_before_floor(
        calendar.candle_end(ct((2015, 11, 22), (0, 0, 0)), CalendarResolution::Daily),
        cme,
        day(2015, 11, 22),
        "the overlay's daily close inside the recordless window",
    );
}

#[test]
fn an_empty_table_is_valid_and_asserts_an_audited_normal_window() {
    let table = StaticSessionExceptions::new(
        CalendarSource::Exchange(Exchange::Nasdaq),
        day(2011, 11, 21),
        day(2011, 11, 28),
        &[],
    )
    .expect("an empty record slice is valid");

    assert_eq!(table.records(), &[]);
    assert_eq!(
        table.exception_on(day(2011, 11, 25)),
        DateException::KnownNormal
    );

    let calendar = calendar_for_exchange(Exchange::Nasdaq)
        .with_session_exceptions(&table)
        .expect("the fixture is scoped to this calendar");
    let plain = calendar_for_exchange(Exchange::Nasdaq);
    let instant = et((2011, 11, 25), (14, 0, 0));
    // The audited-normal window is a 2011 one, so both calendars refuse the
    // date: an empty provider still changes nothing, and the thing it does not
    // change is the coverage refusal (LAW-COVERAGE).
    assert_eq!(
        calendar.session_bounds(instant),
        plain.session_bounds(instant),
        "the empty audited-normal window changed the answer"
    );
    assert_before_floor(
        calendar.session_bounds(instant),
        CalendarSource::Exchange(Exchange::Nasdaq),
        NASDAQ_HALF_DAY,
        "the audited-normal date",
    );
}

// ---------------------------------------------------------------------------
// Daylight-saving edges and scan bounds.
// ---------------------------------------------------------------------------

// US/Central falls back on Sunday 2026-11-01: 01:00-02:00 CT is served twice,
// first as CDT (UTC-5) and again as CST (UTC-6). A replacement block resolves
// its open like any session open (the earliest of the two valid instants) and
// its close like any session close (the latest), so a block stated wholly
// inside the repeated hour covers both passes rather than one.
static FOLD_BLOCKS: [ExceptionBlock; 1] = [ExceptionBlock::regular(
    -1,
    3_600 + 30 * 60,
    3_600 + 45 * 60,
)];

#[test]
fn a_replacement_block_covers_both_passes_of_a_dst_fold() {
    let trade_date = day(2026, 11, 2);
    let records = [SessionExceptionRecord::replace_sessions(
        trade_date,
        &FOLD_BLOCKS,
    )];
    let table = StaticSessionExceptions::new(
        CalendarSource::Exchange(Exchange::Cme),
        day(2026, 10, 30),
        day(2026, 11, 3),
        &records,
    )
    .expect("valid records");
    let calendar = calendar_for_exchange(Exchange::Cme)
        .with_session_exceptions(&table)
        .expect("the fixture is scoped to this calendar");

    // 01:30 CDT is 06:30 UTC and 01:45 CST is 07:45 UTC. A single bias on both
    // endpoints would yield a 15-minute session at one end of the fold or the
    // other; the split bias spans the whole repeated hour.
    let expected = (
        utc((2026, 11, 1), (6, 30, 0)),
        utc((2026, 11, 1), (7, 45, 0)),
    );
    for instant in [
        utc((2026, 11, 1), (6, 30, 0)),
        utc((2026, 11, 1), (7, 0, 0)),
        utc((2026, 11, 1), (7, 44, 59)),
    ] {
        assert_eq!(
            calendar
                .session_bounds(instant)
                .expect("the coverage contract must answer a covered date"),
            Some(expected),
            "the fold block did not cover {instant}"
        );
        assert!(
            calendar
                .is_open(instant)
                .expect("the coverage contract must answer a covered date")
        );
        assert_eq!(
            calendar
                .trade_date(instant)
                .expect("the coverage contract must answer a covered date"),
            Some(trade_date)
        );
    }
    assert!(
        !calendar
            .is_open(utc((2026, 11, 1), (6, 29, 59)))
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        !calendar
            .is_open(utc((2026, 11, 1), (7, 45, 0)))
            .expect("the coverage contract must answer a covered date")
    );
}

// US/Central springs forward on Sunday 2026-03-08: 02:00-03:00 CT does not
// exist. A block stated inside the gap resolves forward to the first instant
// the zone actually serves, 03:00 CDT, rather than being dropped.
static GAP_BLOCKS: [ExceptionBlock; 1] =
    [ExceptionBlock::regular(0, 2 * 3_600 + 30 * 60, 4 * 3_600)];

#[test]
fn a_replacement_block_that_opens_in_a_dst_gap_moves_to_the_first_real_instant() {
    let trade_date = day(2026, 3, 8);
    let records = [SessionExceptionRecord::replace_sessions(
        trade_date,
        &GAP_BLOCKS,
    )];
    let table = StaticSessionExceptions::new(
        CalendarSource::Exchange(Exchange::Cme),
        day(2026, 3, 6),
        day(2026, 3, 10),
        &records,
    )
    .expect("valid records");
    let calendar = calendar_for_exchange(Exchange::Cme)
        .with_session_exceptions(&table)
        .expect("the fixture is scoped to this calendar");

    // 02:30 CT is unrepresentable, so the open lands on 03:00 CDT = 08:00 UTC.
    // The 04:00 CDT close is 09:00 UTC.
    assert_eq!(
        calendar
            .session_bounds(utc((2026, 3, 8), (8, 30, 0)))
            .expect("the coverage contract must answer a covered date"),
        Some((utc((2026, 3, 8), (8, 0, 0)), utc((2026, 3, 8), (9, 0, 0))))
    );
    assert!(
        calendar
            .is_open(utc((2026, 3, 8), (8, 0, 0)))
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        !calendar
            .is_open(utc((2026, 3, 8), (7, 59, 59)))
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        !calendar
            .is_open(utc((2026, 3, 8), (9, 0, 0)))
            .expect("the coverage contract must answer a covered date")
    );
}

#[test]
fn an_all_closed_window_terminates_every_forward_scan() {
    // Longer than any of the engine's forward horizons, so a scan that failed
    // to terminate would hang instead of answering.
    let first = day(2026, 6, 1);
    let last = day(2026, 7, 15);
    let mut records = Vec::new();
    let mut date = first;
    while date <= last {
        records.push(SessionExceptionRecord::closed(date));
        date = date.succ_opt().expect("in-range fixture date");
    }
    let table = StaticSessionExceptions::new(
        CalendarSource::Exchange(Exchange::Nasdaq),
        first,
        last,
        &records,
    )
    .expect("valid records");
    let calendar = calendar_for_exchange(Exchange::Nasdaq)
        .with_session_exceptions(&table)
        .expect("the fixture is scoped to this calendar");

    let instant = utc((2026, 6, 22), (14, 30, 0));
    // `Exchange::Nasdaq` ships no holiday table, so its coverage is empty at and
    // above the floor: it refuses every date in this window rather than guessing
    // a holiday layer it does not have (LAW-COVERAGE). The scans below terminate
    // by returning that refusal — which is what the original fence about
    // termination now asserts, and the strongest statement the identity supports,
    // because "every date in this window is closed" is exactly the claim the
    // crate must *not* make without a sourced table.
    let nasdaq = calendar_for_exchange(Exchange::Nasdaq);
    assert_declared_refusal(
        calendar.is_open(instant),
        nasdaq,
        day(2026, 6, 22),
        "the all-closed window",
    );
    assert_declared_refusal(
        calendar.session_bounds(instant),
        nasdaq,
        day(2026, 6, 22),
        "the all-closed window's bounds",
    );
    assert_declared_refusal(
        calendar.next_session_open_after(instant),
        nasdaq,
        day(2026, 6, 22),
        "the forward open scan",
    );
    assert_declared_refusal(
        calendar.next_session_after(instant),
        nasdaq,
        day(2026, 6, 22),
        "the forward session scan",
    );
    for offset in 0..21 {
        let day_in_window = first
            .checked_add_signed(chrono::Duration::days(offset))
            .expect("in-range fixture date");
        // `is_closed_trade_date` asks the composed rules whether the trade date
        // has a close, so it is answered by the caller's record without needing
        // the holiday table the identity does not ship; only the floor gates it.
        // The claim therefore survives in full: every day in the all-closed
        // window is a closed trade date.
        assert!(
            calendar
                .is_closed_trade_date(day_in_window, SessionKind::Both)
                .expect("a post-floor trade-date closure answers from the caller's record"),
            "{day_in_window} was not reported closed"
        );
    }
}

#[test]
fn a_policy_calendar_refuses_a_provider_scoped_to_another_identity() {
    // `PolicyCalendar` carries its own scope check; the `ExchangeCalendar`
    // entry point does not stand in for it.
    let policy = StaticDayPolicy::new(&[]).expect("empty is valid");
    let error = calendar_for_exchange(Exchange::Nasdaq)
        .with_day_policy(&policy)
        .with_session_exceptions(&cme_table())
        .expect_err("a CME table must not drive a Nasdaq policy calendar");

    assert_eq!(error.calendar, CalendarSource::Exchange(Exchange::Nasdaq));
    assert_eq!(error.provider, CalendarSource::Exchange(Exchange::Cme));

    // The matching identity is still accepted through the same entry point.
    assert!(
        calendar_for_exchange(Exchange::Cme)
            .with_day_policy(&policy)
            .with_session_exceptions(&cme_table())
            .is_ok()
    );
}

// ---------------------------------------------------------------------------
// Validation.
// ---------------------------------------------------------------------------

fn nasdaq_records<'a>(
    records: &'a [SessionExceptionRecord<'a>],
) -> Result<StaticSessionExceptions<'a>, StaticSessionExceptionsError> {
    StaticSessionExceptions::new(
        CalendarSource::Exchange(Exchange::Nasdaq),
        day(2011, 11, 21),
        day(2011, 11, 28),
        records,
    )
}

#[test]
fn inverted_coverage_bounds_are_rejected() {
    assert_eq!(
        StaticSessionExceptions::new(
            CalendarSource::Exchange(Exchange::Nasdaq),
            day(2011, 11, 28),
            day(2011, 11, 21),
            &[],
        ),
        Err(StaticSessionExceptionsError::CoverageBoundsInverted)
    );
}

#[test]
fn out_of_order_and_duplicate_records_are_rejected() {
    let duplicate = [
        SessionExceptionRecord::closed(NASDAQ_HALF_DAY),
        SessionExceptionRecord::known_normal(NASDAQ_HALF_DAY),
    ];
    assert_eq!(
        nasdaq_records(&duplicate),
        Err(StaticSessionExceptionsError::DatesNotStrictlyIncreasing { index: 1 })
    );

    let reversed = [
        SessionExceptionRecord::closed(NASDAQ_HALF_DAY),
        SessionExceptionRecord::closed(NASDAQ_THANKSGIVING),
    ];
    assert_eq!(
        nasdaq_records(&reversed),
        Err(StaticSessionExceptionsError::DatesNotStrictlyIncreasing { index: 1 })
    );
}

#[test]
fn a_record_outside_the_coverage_window_is_rejected() {
    let outside = [SessionExceptionRecord::closed(day(2011, 12, 26))];
    assert_eq!(
        nasdaq_records(&outside),
        Err(StaticSessionExceptionsError::RecordOutsideCoverage { index: 0 })
    );

    let before = [SessionExceptionRecord::closed(day(2011, 11, 20))];
    assert_eq!(
        nasdaq_records(&before),
        Err(StaticSessionExceptionsError::RecordOutsideCoverage { index: 0 })
    );
}

#[test]
fn an_empty_replacement_must_be_recorded_as_closed_instead() {
    let empty = [SessionExceptionRecord::replace_sessions(
        NASDAQ_HALF_DAY,
        &[],
    )];
    assert_eq!(
        nasdaq_records(&empty),
        Err(StaticSessionExceptionsError::EmptyReplacement { index: 0 })
    );
}

#[test]
fn block_domain_violations_are_rejected() {
    let bad_offset = [ExceptionBlock::regular(1, 0, 3_600)];
    let records = [SessionExceptionRecord::replace_sessions(
        NASDAQ_HALF_DAY,
        &bad_offset,
    )];
    assert_eq!(
        nasdaq_records(&records),
        Err(StaticSessionExceptionsError::BlockOffsetOutOfRange {
            index: 0,
            block: 0,
            open_day_offset: 1,
        })
    );

    let too_early = [ExceptionBlock::regular(-8, 0, 3_600)];
    let records = [SessionExceptionRecord::replace_sessions(
        NASDAQ_HALF_DAY,
        &too_early,
    )];
    assert_eq!(
        nasdaq_records(&records),
        Err(StaticSessionExceptionsError::BlockOffsetOutOfRange {
            index: 0,
            block: 0,
            open_day_offset: -8,
        })
    );

    let bad_open = [ExceptionBlock::regular(0, 86_400, 3_600)];
    let records = [SessionExceptionRecord::replace_sessions(
        NASDAQ_HALF_DAY,
        &bad_open,
    )];
    assert_eq!(
        nasdaq_records(&records),
        Err(StaticSessionExceptionsError::BlockOpenOutOfRange {
            index: 0,
            block: 0,
            open_ssm: 86_400,
        })
    );

    let bad_close = [ExceptionBlock::extended(0, 0, 86_401)];
    let records = [SessionExceptionRecord::replace_sessions(
        NASDAQ_HALF_DAY,
        &bad_close,
    )];
    assert_eq!(
        nasdaq_records(&records),
        Err(StaticSessionExceptionsError::BlockCloseOutOfRange {
            index: 0,
            block: 0,
            close_ssm: 86_401,
        })
    );
}

#[test]
fn a_block_opening_on_its_trade_date_may_not_wrap_past_it() {
    // A trade date is named by the local date of its final close, so a block
    // at offset 0 may not run past midnight into the following date: the
    // record would still be keyed by the earlier date, handing back a trade
    // date and a candle boundary that disagree with the block itself.
    let wraps_past_the_trade_date = [ExceptionBlock::regular(0, 17 * 3_600, 16 * 3_600)];
    assert!(wraps_past_the_trade_date[0].wraps_to_next_day());
    let records = [SessionExceptionRecord::replace_sessions(
        NASDAQ_HALF_DAY,
        &wraps_past_the_trade_date,
    )];
    assert_eq!(
        nasdaq_records(&records),
        Err(StaticSessionExceptionsError::BlockClosesAfterTradeDate { index: 0, block: 0 })
    );

    // Equal endpoints wrap by the same rule, so they are refused too.
    let equal_endpoints = [ExceptionBlock::extended(0, 9 * 3_600, 9 * 3_600)];
    let records = [SessionExceptionRecord::replace_sessions(
        NASDAQ_HALF_DAY,
        &equal_endpoints,
    )];
    assert_eq!(
        nasdaq_records(&records),
        Err(StaticSessionExceptionsError::BlockClosesAfterTradeDate { index: 0, block: 0 })
    );

    // A block covering one whole local day has a non-wrapping spelling, so
    // nothing expressible is lost: `close_ssm` may state the closing midnight.
    let whole_local_day = [ExceptionBlock::regular(0, 0, 86_400)];
    assert!(!whole_local_day[0].wraps_to_next_day());
    let records = [SessionExceptionRecord::replace_sessions(
        NASDAQ_HALF_DAY,
        &whole_local_day,
    )];
    assert!(nasdaq_records(&records).is_ok());

    // A block that opens on an earlier date may still wrap.
    let wraps_into_the_trade_date = [ExceptionBlock::regular(-1, 17 * 3_600, 16 * 3_600)];
    let records = [SessionExceptionRecord::replace_sessions(
        NASDAQ_HALF_DAY,
        &wraps_into_the_trade_date,
    )];
    assert!(nasdaq_records(&records).is_ok());
}

#[test]
fn out_of_order_blocks_are_rejected() {
    let unordered = [
        ExceptionBlock::regular(0, 9 * 3_600, 13 * 3_600),
        ExceptionBlock::extended(-1, 17 * 3_600, 3_600),
    ];
    let records = [SessionExceptionRecord::replace_sessions(
        NASDAQ_HALF_DAY,
        &unordered,
    )];
    assert_eq!(
        nasdaq_records(&records),
        Err(StaticSessionExceptionsError::BlocksNotOrdered { index: 0, block: 1 })
    );

    let same_day_unordered = [
        ExceptionBlock::regular(0, 13 * 3_600, 16 * 3_600),
        ExceptionBlock::extended(0, 9 * 3_600, 13 * 3_600),
    ];
    let records = [SessionExceptionRecord::replace_sessions(
        NASDAQ_HALF_DAY,
        &same_day_unordered,
    )];
    assert_eq!(
        nasdaq_records(&records),
        Err(StaticSessionExceptionsError::BlocksNotOrdered { index: 0, block: 1 })
    );
}

#[test]
fn every_validation_error_renders_a_distinct_message() {
    let messages = [
        StaticSessionExceptionsError::CoverageBoundsInverted.to_string(),
        StaticSessionExceptionsError::DatesNotStrictlyIncreasing { index: 1 }.to_string(),
        StaticSessionExceptionsError::RecordOutsideCoverage { index: 2 }.to_string(),
        StaticSessionExceptionsError::EmptyReplacement { index: 3 }.to_string(),
        StaticSessionExceptionsError::BlockOffsetOutOfRange {
            index: 4,
            block: 0,
            open_day_offset: 9,
        }
        .to_string(),
        StaticSessionExceptionsError::BlockOpenOutOfRange {
            index: 5,
            block: 1,
            open_ssm: 90_000,
        }
        .to_string(),
        StaticSessionExceptionsError::BlockCloseOutOfRange {
            index: 6,
            block: 2,
            close_ssm: 90_001,
        }
        .to_string(),
        StaticSessionExceptionsError::BlockClosesAfterTradeDate { index: 7, block: 3 }.to_string(),
        StaticSessionExceptionsError::BlocksNotOrdered { index: 8, block: 4 }.to_string(),
    ];
    for (position, message) in messages.iter().enumerate() {
        assert!(!message.is_empty());
        assert!(
            !messages[position + 1..].contains(message),
            "duplicate message {message}"
        );
    }
}

// ---------------------------------------------------------------------------
// Stage 3 (#93): the topology shapes the fifth `HolidayKind` exists to state.
//
// The built-in kind itself cannot be exercised from here: the crate ships no
// row that uses it, and Stage 4 (#116) lands the operator rows with their
// evidence. What this section fences is the layer such a row is served by. A
// built-in replacement row reaches every query through the same
// `DateException::ReplaceSessions` arm a caller's record uses, so these
// fixtures drive that one resolver over the shapes plan section 7 names -- a
// session on a normally empty day, blocks spanning several civil dates, a
// reassigned trade date, and a restated order-entry topology -- and assert that
// every query family gives the same account of the day.
//
// Every fixture below is a caller-owned record. None of it is shipped data.
// ---------------------------------------------------------------------------

/// The shape #93 records for CME's published Saturday sessions: a block opening
/// on a normally empty Saturday that carries the following Monday's trade date.
///
/// A scalar row cannot state this at all. `LateOpen` can only push an existing
/// occurrence later, never create one, so before this vocabulary existed the
/// date had to ship as a declared gap.
static ADDED_SATURDAY_BLOCKS: [ExceptionBlock; 1] =
    [ExceptionBlock::regular(-2, 5 * 3_600, 17 * 3_600)];

#[test]
fn an_added_session_on_a_normally_empty_day_carries_its_own_trade_date() {
    let trade_date = day(2026, 8, 24);
    assert_eq!(trade_date.weekday(), chrono::Weekday::Mon);
    let saturday = day(2026, 8, 22);
    assert_eq!(saturday.weekday(), chrono::Weekday::Sat);

    let records = [SessionExceptionRecord::replace_sessions(
        trade_date,
        &ADDED_SATURDAY_BLOCKS,
    )];
    let table = StaticSessionExceptions::new(
        CalendarSource::MarketHoursKey(MarketHoursKey::GlobexEnergy),
        day(2026, 8, 1),
        day(2026, 8, 31),
        &records,
    )
    .expect("valid records");
    let calendar = calendar_for_market_hours_key(MarketHoursKey::GlobexEnergy)
        .with_session_exceptions(&table)
        .expect("the fixture is scoped to this calendar");

    let open = ct((2026, 8, 22), (5, 0, 0));
    let close = ct((2026, 8, 22), (17, 0, 0));
    let inside = ct((2026, 8, 22), (10, 0, 0));

    assert_eq!(
        calendar
            .session_bounds(inside)
            .expect("the coverage contract must answer a covered date"),
        Some((open, close)),
        "the added Saturday session did not resolve to its own block"
    );
    assert!(
        calendar
            .is_open(inside)
            .expect("the coverage contract must answer a covered date")
    );
    // The block states its own trade-date assignment, which is the whole point
    // of the shape: the session trades on a Saturday and belongs to the
    // following Monday.
    assert_eq!(
        calendar
            .trade_date(inside)
            .expect("the coverage contract must answer a covered date"),
        Some(trade_date)
    );
    // The trade date's own final close is the added block's close, because the
    // replacement covers the *complete* trade date rather than only the day the
    // block opens on.
    assert_eq!(
        calendar
            .candle_start(inside, CalendarResolution::Daily)
            .expect("the coverage contract must answer a covered date"),
        Some(open)
    );
    assert_eq!(
        calendar
            .candle_end(inside, CalendarResolution::Daily)
            .expect("the coverage contract must answer a covered date"),
        Some(close)
    );

    // End-exclusive close, and closed before the open.
    for instant in [ct((2026, 8, 22), (4, 59, 59)), close] {
        assert!(
            !calendar
                .is_open(instant)
                .expect("the coverage contract must answer a covered date"),
            "the added Saturday session disagrees at {instant}"
        );
    }

    // The normal week's own Monday trade date is gone with it: CME Globex would
    // otherwise be trading on Sunday evening, and a session there would belong
    // to the same trade date the block replaced.
    assert!(
        !calendar
            .is_open(ct((2026, 8, 23), (18, 0, 0)))
            .expect("the coverage contract must answer a covered date"),
        "the replaced trade date kept its normal-week Sunday evening session"
    );
}

/// Three blocks across three opening days: the shape a trade date takes when
/// its sessions begin three local days before it and the weekend between them
/// is normally empty. The gaps between the blocks are pauses inside one trade
/// date, not closures between trade dates.
///
/// The blocks sit on days the normal week does not trade — Friday after the
/// 16:00 CT close, Saturday, and the Monday itself — so each gap is genuinely
/// the replacement's own, not a span the normal week would fill with a session
/// belonging to some other trade date.
static MULTI_DAY_BLOCKS: [ExceptionBlock; 3] = [
    ExceptionBlock::regular(-3, 18 * 3_600, 20 * 3_600),
    ExceptionBlock::regular(-2, 5 * 3_600, 17 * 3_600),
    ExceptionBlock::regular(0, 7 * 3_600, 12 * 3_600),
];

#[test]
fn a_trade_date_may_span_three_opening_days_with_pauses_between_them() {
    let trade_date = day(2026, 8, 31);
    assert_eq!(trade_date.weekday(), chrono::Weekday::Mon);
    let records = [SessionExceptionRecord::replace_sessions(
        trade_date,
        &MULTI_DAY_BLOCKS,
    )];
    let table = StaticSessionExceptions::new(
        CalendarSource::MarketHoursKey(MarketHoursKey::GlobexEnergy),
        day(2026, 8, 1),
        day(2026, 8, 31),
        &records,
    )
    .expect("valid records");
    let calendar = calendar_for_market_hours_key(MarketHoursKey::GlobexEnergy)
        .with_session_exceptions(&table)
        .expect("the fixture is scoped to this calendar");

    // Friday 2026-08-28 18:00-20:00 CT, Saturday 2026-08-29 05:00-17:00 CT and
    // Monday 2026-08-31 07:00-12:00 CT all belong to the one trade date.
    for instant in [
        ct((2026, 8, 28), (19, 0, 0)),
        ct((2026, 8, 29), (10, 0, 0)),
        ct((2026, 8, 31), (8, 0, 0)),
    ] {
        assert!(
            calendar
                .is_open(instant)
                .expect("the coverage contract must answer a covered date"),
            "one of the three blocks is not open at {instant}"
        );
        assert_eq!(
            calendar
                .trade_date(instant)
                .expect("the coverage contract must answer a covered date"),
            Some(trade_date),
            "a block three opening days out lost its trade date at {instant}"
        );
    }

    // The gaps are inside one trade date, so they are pauses rather than the
    // closure that separates trade dates.
    for instant in [
        ct((2026, 8, 28), (21, 0, 0)),
        ct((2026, 8, 29), (18, 0, 0)),
        ct((2026, 8, 30), (12, 0, 0)),
    ] {
        assert!(
            !calendar
                .is_open(instant)
                .expect("the coverage contract must answer a covered date"),
            "a gap between two blocks of one trade date is reported open at {instant}"
        );
        assert_eq!(
            calendar
                .session_state(instant)
                .expect("the coverage contract must answer a covered date"),
            SessionState::Halt,
            "a same-trade-date gap must read as a pause at {instant}"
        );
    }
}

/// The 2025-01-01 shape #93 records: the pre-open queue starts at 16:00 CT
/// instead of the normal 16:45 CT, and the tradeable session is unchanged.
static RESTATED_ORDER_ENTRY_BLOCKS: [ExceptionBlock; 2] = [
    ExceptionBlock::order_entry(-1, 16 * 3_600, 16 * 3_600 + 45 * 60),
    ExceptionBlock::regular(0, 9 * 3_600, 17 * 3_600),
];

#[test]
fn a_replaced_day_restates_its_order_entry_topology() {
    let trade_date = day(2026, 8, 27);
    assert_eq!(trade_date.weekday(), chrono::Weekday::Thu);
    let records = [SessionExceptionRecord::replace_sessions(
        trade_date,
        &RESTATED_ORDER_ENTRY_BLOCKS,
    )];
    let table = StaticSessionExceptions::new(
        CalendarSource::MarketHoursKey(MarketHoursKey::GlobexEnergy),
        day(2026, 8, 1),
        day(2026, 8, 31),
        &records,
    )
    .expect("valid records");
    let calendar = calendar_for_market_hours_key(MarketHoursKey::GlobexEnergy)
        .with_session_exceptions(&table)
        .expect("the fixture is scoped to this calendar");

    // The queue opens on the preceding local day, 2026-08-26 at 16:00 CT.
    let queued = ct((2026, 8, 26), (16, 10, 0));
    assert!(
        calendar
            .is_order_entry_only(queued)
            .expect("the coverage contract must answer a covered date"),
        "the restated queue is not active at {queued}"
    );
    assert!(
        calendar
            .is_accepting_orders(queued)
            .expect("the coverage contract must answer a covered date")
    );
    assert_eq!(
        calendar
            .session_state(queued)
            .expect("the coverage contract must answer a covered date"),
        SessionState::OrderEntry
    );
    // An order-entry phase is not tradeable, and the normal week's own queue
    // does not survive the replacement.
    assert!(
        !calendar
            .is_open(queued)
            .expect("the coverage contract must answer a covered date")
    );

    // 16:45 CT ends the restated queue; the normal 16:45 CT start would still
    // be running here if the replacement had not replaced it.
    let after = ct((2026, 8, 26), (16, 50, 0));
    assert!(
        !calendar
            .is_order_entry_only(after)
            .expect("the coverage contract must answer a covered date"),
        "the restated queue outlived its own close at {after}"
    );
    assert!(
        !calendar
            .is_accepting_orders(after)
            .expect("the coverage contract must answer a covered date")
    );
}

/// A caller's replacement record suppresses the built-in holiday row for that
/// trade date rather than composing with it.
///
/// This is the precedence Stage 3's new kind has to obey, fenced here on the
/// half of it a caller can reach: `GlobexEnergy` ships a built-in replacement for
/// 2026-11-27 whose last block ends at 13:45 CT — the day after Thanksgiving, which
/// that family states as a merged trade date — and a caller's `ReplaceSessions`
/// record for the same date must win outright. If the two composed, the replaced
/// day's blocks would be clipped back to 13:45 and one row would have been applied
/// twice.
static SUPPRESSES_BUILTIN_BLOCKS: [ExceptionBlock; 1] =
    [ExceptionBlock::regular(0, 9 * 3_600, 16 * 3_600)];

/// The built-in row those blocks replace, with the same 13:45 CT end.
static SUPPRESSED_BUILTIN_BLOCKS: [ExceptionBlock; 4] = [
    ExceptionBlock::order_entry(-2, 16 * 3_600 + 45 * 60, 17 * 3_600),
    ExceptionBlock::extended(-2, 17 * 3_600, 13 * 3_600 + 30 * 60),
    ExceptionBlock::order_entry(-1, 13 * 3_600 + 30 * 60, 17 * 3_600),
    ExceptionBlock::extended(-1, 17 * 3_600, 13 * 3_600 + 45 * 60),
];

#[test]
fn a_caller_replacement_suppresses_the_built_in_row_for_that_trade_date() {
    let trade_date = day(2026, 11, 27);
    assert_eq!(trade_date.weekday(), chrono::Weekday::Fri);

    let bare = calendar_for_market_hours_key(MarketHoursKey::GlobexEnergy);
    assert_eq!(
        bare.holiday_on(trade_date).map(Holiday::kind),
        Some(HolidayKind::ReplacementBlocks(&SUPPRESSED_BUILTIN_BLOCKS)),
        "the fixture depends on this identity shipping a built-in row that day, \
         whose session ends at the same 13:45 the check below probes past"
    );

    let records = [SessionExceptionRecord::replace_sessions(
        trade_date,
        &SUPPRESSES_BUILTIN_BLOCKS,
    )];
    let table = StaticSessionExceptions::new(
        CalendarSource::MarketHoursKey(MarketHoursKey::GlobexEnergy),
        day(2026, 11, 1),
        day(2026, 11, 30),
        &records,
    )
    .expect("valid records");
    let calendar = bare
        .with_session_exceptions(&table)
        .expect("the fixture is scoped to this calendar");

    // 14:30 CT is inside the caller's block and past the built-in early close.
    // A composed answer would report it closed, so the bare calendar states the
    // other half of the fence.
    let after_builtin_close = ct((2026, 11, 27), (14, 30, 0));
    assert!(
        !bare
            .is_open(after_builtin_close)
            .expect("the coverage contract must answer a covered date"),
        "the built-in early close must be what closes this instant"
    );
    assert_eq!(
        calendar
            .session_bounds(after_builtin_close)
            .expect("the coverage contract must answer a covered date"),
        Some((
            ct((2026, 11, 27), (9, 0, 0)),
            ct((2026, 11, 27), (16, 0, 0))
        )),
        "the built-in early close was applied on top of the caller's replacement"
    );
    assert!(
        calendar
            .is_open(after_builtin_close)
            .expect("the coverage contract must answer a covered date")
    );

    // And the normal week's own session for this trade date is gone rather than
    // shortened: the preceding evening would otherwise be trading.
    assert!(
        bare.is_open(ct((2026, 11, 26), (18, 0, 0)))
            .expect("the coverage contract must answer a covered date"),
        "the preceding evening must be open on the bare calendar for this to fence anything"
    );
    assert!(
        !calendar
            .is_open(ct((2026, 11, 26), (18, 0, 0)))
            .expect("the coverage contract must answer a covered date"),
        "the normal-week session belonging to the replaced trade date survived"
    );
}

/// A replaced trade date with extended trading but **no** regular session: the
/// plan's "regular-only closure", stated as blocks rather than as a scalar early
/// close.
///
/// One wrapped extended block opens the preceding evening and closes at 03:00 CT
/// on the trade date. The replacement suite's other post-floor fixtures state
/// only regular and order-entry blocks, and the pre-floor Nasdaq half-day
/// fixture that used to exercise this distinction now asserts a refusal, so this
/// is the case that holds `query::replacement::selects` to its mapping of block
/// kind to rule set.
///
/// Deliberately one block and not two: an evening block on the trade date would
/// overlap the **following** trade date's normal session, which opens the same
/// evening. No operator publishes two sessions over one instant, and a fixture
/// that did would be asking which of them containment should report rather than
/// testing the kind that is stated.
static EXTENDED_ONLY_BLOCKS: [ExceptionBlock; 1] =
    [ExceptionBlock::extended(-1, 17 * 3_600, 3 * 3_600)];

#[test]
fn a_replaced_day_with_no_regular_block_closes_regular_trading_only() {
    let trade_date = day(2026, 8, 27);
    assert_eq!(trade_date.weekday(), chrono::Weekday::Thu);
    let records = [SessionExceptionRecord::replace_sessions(
        trade_date,
        &EXTENDED_ONLY_BLOCKS,
    )];
    let table = StaticSessionExceptions::new(
        CalendarSource::MarketHoursKey(MarketHoursKey::GlobexEnergy),
        day(2026, 8, 1),
        day(2026, 8, 31),
        &records,
    )
    .expect("valid records");
    let calendar = calendar_for_market_hours_key(MarketHoursKey::GlobexEnergy)
        .with_session_exceptions(&table)
        .expect("the fixture is scoped to this calendar");

    // 02:00 CT is inside the wrapped extended block that opened Wednesday
    // evening. It is a real session, and it is not a regular one.
    let overnight = ct((2026, 8, 27), (2, 0, 0));
    assert!(
        calendar
            .is_open_extended(overnight)
            .expect("the coverage contract must answer a covered date"),
        "the extended block is not open at {overnight}"
    );
    assert!(
        calendar
            .is_open(overnight)
            .expect("the coverage contract must answer a covered date"),
        "a tradeable extended block must count as open at {overnight}"
    );
    assert!(
        !calendar
            .is_open_regular(overnight)
            .expect("the coverage contract must answer a covered date"),
        "an extended block was reported as a regular session at {overnight}"
    );
    assert_eq!(
        calendar
            .session_bounds_with(overnight, SessionKind::Extended)
            .expect("the coverage contract must answer a covered date"),
        Some((ct((2026, 8, 26), (17, 0, 0)), ct((2026, 8, 27), (3, 0, 0)))),
        "the extended kind did not resolve the wrapped block"
    );

    // 10:00 CT is the middle of the trade date's ordinary regular session, and
    // this replacement states no regular block at all. A consumer reading the
    // regular kind must see the closure, and one reading the session as a whole
    // must not be handed the ordinary week back.
    let regular_hours = ct((2026, 8, 27), (10, 0, 0));
    assert!(
        !calendar
            .is_open_regular(regular_hours)
            .expect("the coverage contract must answer a covered date"),
        "the replaced trade date kept a regular session it did not state"
    );
    assert!(
        !calendar
            .is_open(regular_hours)
            .expect("the coverage contract must answer a covered date"),
        "a regular-only closure was reported as open"
    );
    assert_eq!(
        calendar
            .session_bounds_with(regular_hours, SessionKind::Regular)
            .expect("the coverage contract must answer a covered date"),
        None,
        "the regular kind resolved a session from an extended block"
    );
}

/// The last link of the precedence chain, above the floor: the replacement layer
/// decides what the trading day is, and the caller's `DayPolicy` then clips that
/// result exactly as it clips a normal week.
///
/// The suite's three fixtures for this all state 2011 dates, so since Stage 2B
/// they assert `BeforeSupportFloor` and the composition is no longer observable
/// through them — their own comments say so. This is the post-floor case, and it
/// is differential: the same replacement is resolved twice, once with the policy
/// and once without, so a policy that silently failed to clip would fail here
/// rather than pass for the right-looking reason.
static POLICY_CLIP_BLOCKS: [ExceptionBlock; 1] =
    [ExceptionBlock::regular(0, 9 * 3_600, 17 * 3_600)];

#[test]
fn a_day_policy_clips_a_replaced_trading_day_above_the_floor() {
    let trade_date = day(2026, 8, 27);
    assert_eq!(trade_date.weekday(), chrono::Weekday::Thu);
    let records = [SessionExceptionRecord::replace_sessions(
        trade_date,
        &POLICY_CLIP_BLOCKS,
    )];
    let table = StaticSessionExceptions::new(
        CalendarSource::MarketHoursKey(MarketHoursKey::GlobexEnergy),
        day(2026, 8, 1),
        day(2026, 8, 31),
        &records,
    )
    .expect("valid records");
    let base = calendar_for_market_hours_key(MarketHoursKey::GlobexEnergy)
        .with_session_exceptions(&table)
        .expect("the fixture is scoped to this calendar");

    let overrides = [DayOverride::early_close(trade_date, 13 * 3_600)];
    let policy = StaticDayPolicy::new(&overrides).expect("valid override");
    let clipped = base.with_day_policy(&policy);
    assert!(clipped.has_session_exceptions());
    assert!(clipped.has_day_policy());

    let morning = ct((2026, 8, 27), (10, 0, 0));
    let afternoon = ct((2026, 8, 27), (14, 0, 0));

    // Unclipped, the replacement's own block stands: 09:00-17:00 CT.
    assert_eq!(
        base.session_bounds(morning)
            .expect("the coverage contract must answer a covered date"),
        Some((ct((2026, 8, 27), (9, 0, 0)), ct((2026, 8, 27), (17, 0, 0)))),
        "the fixture's replacement block is not the session it claims"
    );
    assert!(
        base.is_open(afternoon)
            .expect("the coverage contract must answer a covered date"),
        "the unclipped replacement must still be trading at {afternoon}"
    );

    // Clipped, the policy's 13:00 CT bound wins over the block's own close.
    assert_eq!(
        clipped
            .session_bounds(morning)
            .expect("the coverage contract must answer a covered date"),
        Some((ct((2026, 8, 27), (9, 0, 0)), ct((2026, 8, 27), (13, 0, 0)))),
        "the DayPolicy did not clip the replaced trading day"
    );
    assert!(
        !clipped
            .is_open(afternoon)
            .expect("the coverage contract must answer a covered date"),
        "the replaced day traded past the policy's clip"
    );
    // The clip is end-exclusive, like every other close.
    assert!(
        clipped
            .is_open(ct((2026, 8, 27), (12, 59, 59)))
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        !clipped
            .is_open(ct((2026, 8, 27), (13, 0, 0)))
            .expect("the coverage contract must answer a covered date")
    );
}

/// Two blocks may state the same opening instant when they state different
/// kinds: a regular session that ends at midday while extended trading runs on
/// from the same open.
///
/// The ordering rule is non-decreasing, not strictly increasing, so this set is
/// a valid arrangement rather than a duplicate. It is pinned here because the
/// rule is shared with the built-in table's constant-evaluation fence from
/// Stage 3 (#93), and a future reader tightening `<` to `<=` would reject a
/// shape operators do publish without any other test noticing.
///
/// The two kinds carry different closes on purpose, so both blocks are
/// observable from the one opening instant: the regular kind ends at 12:00 CT
/// and the extended kind at 17:00 CT. A pair whose windows coincided would be
/// accepted by validation but prove nothing about which block answered.
static EQUAL_OPENING_BLOCKS: [ExceptionBlock; 2] = [
    ExceptionBlock::regular(0, 9 * 3_600, 12 * 3_600),
    ExceptionBlock::extended(0, 9 * 3_600, 17 * 3_600),
];

#[test]
fn blocks_may_share_an_opening_instant_when_their_kinds_differ() {
    let trade_date = day(2026, 8, 27);
    let records = [SessionExceptionRecord::replace_sessions(
        trade_date,
        &EQUAL_OPENING_BLOCKS,
    )];
    // The point of the fixture: a set whose blocks agree on opening day and open
    // time is accepted rather than reported as out of order.
    let table = StaticSessionExceptions::new(
        CalendarSource::MarketHoursKey(MarketHoursKey::GlobexEnergy),
        day(2026, 8, 1),
        day(2026, 8, 31),
        &records,
    )
    .expect("two blocks of different kinds may share an opening instant");
    let calendar = calendar_for_market_hours_key(MarketHoursKey::GlobexEnergy)
        .with_session_exceptions(&table)
        .expect("the fixture is scoped to this calendar");

    // Both blocks are selected by their own kind from the one opening instant,
    // and each keeps its own close.
    let shared_open = ct((2026, 8, 27), (11, 0, 0));
    assert!(
        calendar
            .is_open_regular(shared_open)
            .expect("the coverage contract must answer a covered date"),
        "the regular block was not selected at the shared opening instant"
    );
    assert!(
        calendar
            .is_open_extended(shared_open)
            .expect("the coverage contract must answer a covered date"),
        "the extended block was not selected at the shared opening instant"
    );
    assert_eq!(
        calendar
            .session_bounds_with(shared_open, SessionKind::Regular)
            .expect("the coverage contract must answer a covered date"),
        Some((ct((2026, 8, 27), (9, 0, 0)), ct((2026, 8, 27), (12, 0, 0)))),
        "the regular kind did not resolve its own block"
    );
    assert_eq!(
        calendar
            .session_bounds_with(shared_open, SessionKind::Extended)
            .expect("the coverage contract must answer a covered date"),
        Some((ct((2026, 8, 27), (9, 0, 0)), ct((2026, 8, 27), (17, 0, 0)))),
        "the extended kind did not resolve its own block"
    );

    // After 12:00 CT the regular block is done while extended trading runs on,
    // which is the observable consequence of the two blocks being distinct and
    // not one block counted twice.
    let after_regular_close = ct((2026, 8, 27), (13, 0, 0));
    assert!(
        !calendar
            .is_open_regular(after_regular_close)
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        calendar
            .is_open_extended(after_regular_close)
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        calendar
            .is_open(after_regular_close)
            .expect("the coverage contract must answer a covered date")
    );
}

/// Every query family reads the same replacement, which is the failure mode
/// Stage 2B's single gate makes possible to get wrong: a block visible to
/// `is_open` but not to the candle edges, or to the trade date but not to the
/// session bounds, would be a schedule that disagrees with itself.
#[test]
fn every_query_family_gives_the_same_account_of_a_replaced_day() {
    let trade_date = day(2026, 8, 24);
    let records = [SessionExceptionRecord::replace_sessions(
        trade_date,
        &ADDED_SATURDAY_BLOCKS,
    )];
    let table = StaticSessionExceptions::new(
        CalendarSource::MarketHoursKey(MarketHoursKey::GlobexEnergy),
        day(2026, 8, 1),
        day(2026, 8, 31),
        &records,
    )
    .expect("valid records");
    let calendar = calendar_for_market_hours_key(MarketHoursKey::GlobexEnergy)
        .with_session_exceptions(&table)
        .expect("the fixture is scoped to this calendar");

    let open = ct((2026, 8, 22), (5, 0, 0));
    let close = ct((2026, 8, 22), (17, 0, 0));
    // Inside the added block, the boundary, the status, the trade date and the
    // candle edges are five views of one resolved session.
    let inside = ct((2026, 8, 22), (10, 0, 0));
    let bounds = calendar
        .session_bounds(inside)
        .expect("the coverage contract must answer a covered date")
        .expect("the added block is a session");
    assert_eq!(bounds, (open, close));
    assert!(
        calendar
            .is_open(inside)
            .expect("the coverage contract must answer a covered date")
    );
    assert_eq!(
        calendar
            .session_state(inside)
            .expect("the coverage contract must answer a covered date"),
        SessionState::OpenRegular
    );
    assert_eq!(
        calendar
            .trade_date(inside)
            .expect("the coverage contract must answer a covered date"),
        Some(trade_date)
    );
    assert_eq!(
        calendar
            .candle_start(inside, CalendarResolution::Daily)
            .expect("the coverage contract must answer a covered date"),
        Some(open)
    );
    assert_eq!(
        calendar
            .candle_end(inside, CalendarResolution::Daily)
            .expect("the coverage contract must answer a covered date"),
        Some(close)
    );
    // The addition is visible as the trade date's own session, not as an
    // absence: the day the block opens on is the trade date's only session.
    assert!(
        !calendar
            .is_closed_trade_date(trade_date, SessionKind::Both)
            .expect("the coverage contract must answer a covered date"),
        "the trade date carrying the added session reads as closed"
    );
}

/// A block opening on an instant the **next** trade date's normal session also
/// opens on: issue #130's shape, stated as the one invariant that shape broke.
///
/// The block below opens Thursday 19:00 CT, which is also the normal-week
/// electronic open for *Friday* on this grid, so the two layers claim the same
/// instant. Before the fix, `session_bounds` answered from the normal scan
/// (Thursday 19:00 → Friday 07:45) while `trade_date` answered from the
/// replacement scan (Thursday), and a caller pairing them received a window and
/// a date naming two different sessions. The rule the fix states is that a
/// replacement block meeting a normal occurrence takes its place, so both
/// answers describe the replacement.
#[test]
fn a_block_meeting_the_next_trade_dates_open_replaces_it() {
    static BLOCKS: [ExceptionBlock; 1] = [ExceptionBlock::extended(0, 19 * 3_600, 21 * 3_600)];
    let trade_date = day(2026, 6, 11);
    let records = [SessionExceptionRecord::replace_sessions(
        trade_date, &BLOCKS,
    )];
    let table = StaticSessionExceptions::new(
        CalendarSource::MarketHoursKey(MarketHoursKey::GlobexGrains),
        day(2026, 6, 1),
        day(2026, 6, 30),
        &records,
    )
    .expect("valid records");
    let calendar = calendar_for_market_hours_key(MarketHoursKey::GlobexGrains)
        .with_session_exceptions(&table)
        .expect("the fixture is scoped to this calendar");
    let plain = calendar_for_market_hours_key(MarketHoursKey::GlobexGrains);

    let open = ct((2026, 6, 11), (19, 0, 0));
    let close = ct((2026, 6, 11), (21, 0, 0));
    for instant in [
        open,
        ct((2026, 6, 11), (20, 0, 0)),
        // The last instant inside the block, where the two candidate windows
        // differ most: the normal occurrence would still have nearly twelve
        // hours to run.
        close - chrono::Duration::nanoseconds(1),
    ] {
        assert!(
            calendar
                .is_open(instant)
                .expect("the coverage contract must answer a covered date"),
            "the replacing block is not open at {instant}"
        );
        assert_eq!(
            calendar
                .session_bounds(instant)
                .expect("the coverage contract must answer a covered date"),
            Some((open, close)),
            "session_bounds described the neighbouring trade date's session"
        );
        assert_eq!(
            calendar
                .trade_date(instant)
                .expect("the coverage contract must answer a covered date"),
            Some(trade_date),
            "trade_date and session_bounds must describe one session"
        );
    }

    // The cost of the rule is stated rather than hidden, and so is its reach.
    // The extended occurrence the block met opened Thursday 19:00 and would have
    // run to Friday 07:45; because the block takes that occurrence's place, the
    // rest of the occurrence is not reported either, and an instant after the
    // block reaches Friday's **regular** session instead. That session is a
    // separate occurrence of a separate rule set, so the kind-aware test leaves
    // it alone — the same rule that keeps a replaced date's queue when its
    // record states only tradeable blocks. A caller that wants the ordinary week
    // around a block states the whole day in blocks instead of a fragment.
    let after = close + chrono::Duration::hours(1);
    assert_eq!(
        calendar
            .session_bounds(after)
            .expect("the coverage contract must answer a covered date"),
        Some((
            ct((2026, 6, 12), (8, 30, 0)),
            ct((2026, 6, 12), (13, 20, 0))
        )),
        "the extended occurrence yielded, so the next session is Friday's regular one"
    );
    // `trade_date` at this instant is not asserted: this identity's declared
    // gaps (#79, #93) make the answer a coverage verdict rather than a date, and
    // that verdict is Stage 2B's contract, not this one's.
    // Away from the replaced day the two calendars agree outright.
    let elsewhere = ct((2026, 6, 17), (12, 0, 0));
    assert_eq!(
        calendar
            .session_bounds(elsewhere)
            .expect("the coverage contract must answer a covered date"),
        plain
            .session_bounds(elsewhere)
            .expect("the coverage contract must answer a covered date")
    );
}

/// The reach of the invariant, swept over every served identity.
///
/// A window `session_bounds` reports as containing the query is a session the
/// identity says is open there, so the two queries cannot disagree about whether
/// the instant has a trade date at all. This is the weak half of issue #130's
/// invariant — the strong half, that the two answers name the *same* session, is
/// fenced by the collision fixture above, which is where a shape exists to get it
/// wrong — and it is swept here because it costs one extra call per open instant
/// and would catch that disagreement for any identity.
#[test]
fn no_identity_is_open_where_it_reports_no_trade_date() {
    let mut worst = String::new();
    for &key in MarketHoursKey::ALL {
        let calendar = calendar_for_market_hours_key(key);
        for (year, month) in [(2025, 11_u32), (2026, 3), (2026, 8), (2027, 6)] {
            for date in 1..=28 {
                for hour in [1_u32, 7, 13, 19, 23] {
                    let Some(instant) =
                        Utc.with_ymd_and_hms(year, month, date, hour, 0, 0).single()
                    else {
                        continue;
                    };
                    let Ok(Some(bounds)) = calendar.session_bounds(instant) else {
                        continue;
                    };
                    if !(bounds.0 <= instant && instant < bounds.1) {
                        // `session_bounds` is "containing **or next**", so an
                        // instant in a gap legitimately receives the session
                        // that follows it.
                        continue;
                    }
                    let Ok(open) = calendar.is_open(instant) else {
                        continue;
                    };
                    let Ok(trade_date) = calendar.trade_date(instant) else {
                        continue;
                    };
                    if open && trade_date.is_none() {
                        worst = format!(
                            "{key:?} at {instant}: the reported session {bounds:?} contains \
                             this instant but the instant has no trade date"
                        );
                        break;
                    }
                }
            }
        }
        assert!(worst.is_empty(), "{worst}");
    }
}

/// The malformed block sets a built-in `holidays!` row would fail the build over.
///
/// `fences::assert_blocks` is reached from `assert_instants` for a
/// `HolidayKind::ReplacementBlocks` row, and it applies exactly the check a
/// caller's record meets here — one predicate, two verdicts. Stage 3 shipped
/// that path with no row to exercise it, so this pins the rules it enforces and
/// the order it reports them in; the built-in half is the same call in constant
/// evaluation, where the verdict is a build failure rather than an `Err`.
mod malformed_block_sets {
    use super::*;

    /// An empty set is a closure, not a replacement: `Closed` states it.
    static EMPTY: [ExceptionBlock; 0] = [];
    /// A block opening on the trade date may not wrap: the trade date is named
    /// by the local date of its final close.
    static WRAPS_AT_TRADE_DATE: [ExceptionBlock; 1] =
        [ExceptionBlock::extended(0, 19 * 3_600, 3 * 3_600)];
    /// Ordering is non-decreasing: a later block placed first is rejected.
    static OUT_OF_ORDER: [ExceptionBlock; 2] = [
        ExceptionBlock::extended(-1, 19 * 3_600, 21 * 3_600),
        ExceptionBlock::extended(-2, 19 * 3_600, 21 * 3_600),
    ];
    /// Equal openings are legal when the kinds differ, which is why the rule is
    /// non-decreasing rather than strictly increasing.
    static SHARED_OPEN: [ExceptionBlock; 2] = [
        ExceptionBlock::order_entry(-1, 16 * 3_600, 19 * 3_600),
        ExceptionBlock::extended(-1, 16 * 3_600, 19 * 3_600),
    ];
    /// The offset domain is bounded, so the per-day replacement scan is too.
    static OFFSET_TOO_EARLY: [ExceptionBlock; 1] =
        [ExceptionBlock::extended(-8, 19 * 3_600, 21 * 3_600)];
    /// Instants are held to the same ranges a caller's `DayPolicy` is.
    static OPEN_TOO_LATE: [ExceptionBlock; 1] = [ExceptionBlock::extended(-1, 86_400, 86_400)];
    /// A close one second past the end of a local day.
    static CLOSE_TOO_LATE: [ExceptionBlock; 1] = [ExceptionBlock::extended(-1, 19 * 3_600, 86_401)];

    #[test]
    fn are_rejected_with_a_named_violation() {
        let trade_date = day(2026, 6, 11);
        // Validates one replacement record over `blocks`, which is how a
        // built-in row is validated during constant evaluation. The table is not
        // returned: it borrows the record slice built here, so the caller learns
        // only what the one predicate `assert_blocks` also applies decided.
        let validate = |blocks: &'static [ExceptionBlock]| {
            let records = [SessionExceptionRecord::replace_sessions(trade_date, blocks)];
            StaticSessionExceptions::new(
                CalendarSource::MarketHoursKey(MarketHoursKey::GlobexGrains),
                day(2026, 6, 1),
                day(2026, 6, 30),
                &records,
            )
            .err()
        };

        assert_eq!(
            validate(&EMPTY),
            Some(StaticSessionExceptionsError::EmptyReplacement { index: 0 })
        );
        assert_eq!(
            validate(&WRAPS_AT_TRADE_DATE),
            Some(StaticSessionExceptionsError::BlockClosesAfterTradeDate { index: 0, block: 0 })
        );
        assert_eq!(
            validate(&OUT_OF_ORDER),
            Some(StaticSessionExceptionsError::BlocksNotOrdered { index: 0, block: 1 })
        );
        assert_eq!(
            validate(&SHARED_OPEN),
            None,
            "two kinds may state one opening instant"
        );
        assert_eq!(
            validate(&OFFSET_TOO_EARLY),
            Some(StaticSessionExceptionsError::BlockOffsetOutOfRange {
                index: 0,
                block: 0,
                open_day_offset: -8,
            })
        );
        assert_eq!(
            validate(&OPEN_TOO_LATE),
            Some(StaticSessionExceptionsError::BlockOpenOutOfRange {
                index: 0,
                block: 0,
                open_ssm: 86_400,
            })
        );
        assert_eq!(
            validate(&CLOSE_TOO_LATE),
            Some(StaticSessionExceptionsError::BlockCloseOutOfRange {
                index: 0,
                block: 0,
                close_ssm: 86_401,
            })
        );
    }
}

/// A block opening **two or more** local days before its trade date is reached.
///
/// Found by an independent review after the first cut of #130's fix. The guard's
/// trade-date walk was bounded by the window's last local day, but a block may
/// open up to `-7` days before its trade date, so a record keyed at offset `-2`
/// was never visited and the collision it was meant to settle survived: the
/// normal occurrence answered `session_bounds` while `trade_date` reported the
/// block's date. This fixture is the reviewer's reproducer.
#[test]
fn a_block_opening_two_days_early_still_governs_its_own_instant() {
    static BLOCKS: [ExceptionBlock; 1] = [ExceptionBlock::extended(-2, 19 * 3_600, 21 * 3_600)];
    let trade_date = day(2026, 6, 16);
    let records = [SessionExceptionRecord::replace_sessions(
        trade_date, &BLOCKS,
    )];
    let table = StaticSessionExceptions::new(
        CalendarSource::MarketHoursKey(MarketHoursKey::GlobexGrains),
        day(2026, 6, 1),
        day(2026, 6, 30),
        &records,
    )
    .expect("valid records");
    let calendar = calendar_for_market_hours_key(MarketHoursKey::GlobexGrains)
        .with_session_exceptions(&table)
        .expect("the fixture is scoped to this calendar");

    // 19:00-21:00 CT on 2026-06-14, the block's own opening day.
    let instant = ct((2026, 6, 14), (19, 30, 0));
    assert_eq!(
        calendar
            .session_bounds(instant)
            .expect("the coverage contract must answer a covered date"),
        Some((ct((2026, 6, 14), (19, 0, 0)), ct((2026, 6, 14), (21, 0, 0)))),
        "the offset -2 block must answer for its own window"
    );
    assert_eq!(
        calendar
            .trade_date(instant)
            .expect("the coverage contract must answer a covered date"),
        Some(trade_date),
        "and carry its own trade date"
    );
}
