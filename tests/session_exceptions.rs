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

#![expect(
    clippy::expect_used,
    reason = "fixture literals and validated static records must fail the test if malformed"
)]

use chrono::{DateTime, NaiveDate, TimeZone, Utc};
use chrono_tz::US;
use exchange_hours::{
    CalendarResolution, CalendarSource, DateException, DayOverride, ExceptionBlock,
    ExceptionBlockKind, ExceptionCoverage, Exchange, MarketHoursKey, SessionExceptionRecord,
    SessionExceptionSource, SessionKind, SessionState, StaticDayPolicy, StaticSessionExceptions,
    StaticSessionExceptionsError, calendar_for_exchange, calendar_for_market_hours_key,
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

    // First block: Wednesday 17:00 CT through Thursday 12:00 CT.
    assert_eq!(
        calendar.session_bounds(ct((2015, 11, 26), (10, 0, 0))),
        Some((
            ct((2015, 11, 25), (17, 0, 0)),
            ct((2015, 11, 26), (12, 0, 0))
        ))
    );
    assert!(calendar.is_open(ct((2015, 11, 26), (11, 59, 59))));

    // The pause is an order-entry-only phase, not a session and not a closure.
    assert!(!calendar.is_open(ct((2015, 11, 26), (12, 0, 0))));
    assert!(calendar.is_order_entry_only(ct((2015, 11, 26), (13, 0, 0))));
    assert!(calendar.is_accepting_orders(ct((2015, 11, 26), (13, 0, 0))));
    assert_eq!(
        calendar.session_state(ct((2015, 11, 26), (16, 59, 59))),
        SessionState::OrderEntry
    );

    // Second block: Thursday 17:00 CT through the 12:15 CT Friday early close.
    assert_eq!(
        calendar.session_bounds(ct((2015, 11, 26), (18, 0, 0))),
        Some((
            ct((2015, 11, 26), (17, 0, 0)),
            ct((2015, 11, 27), (12, 15, 0))
        ))
    );
    assert!(calendar.is_open_regular(ct((2015, 11, 27), (9, 0, 0))));
    assert!(calendar.is_open(ct((2015, 11, 27), (12, 14, 59))));
    assert!(!calendar.is_open(ct((2015, 11, 27), (12, 15, 0))));

    // Both blocks belong to one trade date, and the whole trading day is one
    // daily bar running from Wednesday's 17:00 open to Friday's early close.
    for instant in [
        ct((2015, 11, 25), (18, 0, 0)),
        ct((2015, 11, 26), (10, 0, 0)),
        ct((2015, 11, 26), (18, 0, 0)),
        ct((2015, 11, 27), (9, 0, 0)),
    ] {
        assert_eq!(
            calendar.trade_date(instant),
            Some(CME_FRIDAY),
            "wrong trade date at {instant}"
        );
    }
    assert_eq!(
        calendar.candle_start(ct((2015, 11, 26), (10, 0, 0)), CalendarResolution::Daily),
        Some(ct((2015, 11, 25), (17, 0, 0)))
    );
    assert_eq!(
        calendar.candle_end(ct((2015, 11, 26), (10, 0, 0)), CalendarResolution::Daily),
        Some(ct((2015, 11, 27), (12, 15, 0)))
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
    assert!(plain.is_open_regular(ct((2015, 11, 26), (13, 0, 0))));
    assert!(!calendar.is_open(ct((2015, 11, 26), (13, 0, 0))));
    assert!(calendar.is_closed_trade_date(CME_THURSDAY, SessionKind::Both));
    assert!(!plain.is_closed_trade_date(CME_THURSDAY, SessionKind::Both));

    // Wednesday's own trade date is audited normal and is left alone.
    assert!(calendar.is_open_regular(ct((2015, 11, 25), (10, 0, 0))));
    assert_eq!(
        calendar.trade_date(ct((2015, 11, 25), (10, 0, 0))),
        Some(day(2015, 11, 25))
    );
    assert_eq!(
        calendar.session_bounds(ct((2015, 11, 25), (15, 45, 0))),
        Some((
            ct((2015, 11, 25), (15, 30, 0)),
            ct((2015, 11, 25), (16, 0, 0))
        ))
    );

    // The Wednesday-evening queue is reassigned to Friday's trade date, which
    // is exactly what the notice's 1645 CT footnote states.
    assert!(calendar.is_order_entry_only(ct((2015, 11, 25), (16, 50, 0))));
    assert_eq!(
        calendar.trade_date(ct((2015, 11, 25), (16, 50, 0))),
        Some(CME_FRIDAY)
    );

    // The following Monday is outside the exceptional run and unchanged.
    assert!(calendar.is_open_regular(ct((2015, 11, 30), (10, 0, 0))));
    assert_eq!(
        calendar.session_bounds(ct((2015, 11, 30), (10, 0, 0))),
        plain.session_bounds(ct((2015, 11, 30), (10, 0, 0)))
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

    assert!(calendar.is_open_extended(et((2011, 11, 25), (8, 0, 0))));
    assert!(calendar.is_open_regular(et((2011, 11, 25), (12, 59, 59))));
    assert!(!calendar.is_open_regular(et((2011, 11, 25), (13, 0, 0))));
    assert!(calendar.is_open_extended(et((2011, 11, 25), (13, 0, 0))));
    assert!(calendar.is_open(et((2011, 11, 25), (16, 59, 59))));
    assert!(!calendar.is_open(et((2011, 11, 25), (17, 0, 0))));

    assert_eq!(
        calendar.session_bounds_with(et((2011, 11, 25), (10, 0, 0)), SessionKind::Regular),
        Some((
            et((2011, 11, 25), (9, 30, 0)),
            et((2011, 11, 25), (13, 0, 0))
        ))
    );
    assert_eq!(
        calendar.session_bounds_with(et((2011, 11, 25), (14, 0, 0)), SessionKind::Extended),
        Some((
            et((2011, 11, 25), (13, 0, 0)),
            et((2011, 11, 25), (17, 0, 0))
        ))
    );

    // The regular and full trading days end at different instants, which is
    // precisely what one scalar `early_close_ssm` cannot say.
    assert_eq!(
        calendar.candle_end_with(
            et((2011, 11, 25), (10, 0, 0)),
            CalendarResolution::Daily,
            SessionKind::Regular
        ),
        Some(et((2011, 11, 25), (13, 0, 0)))
    );
    assert_eq!(
        calendar.candle_end(et((2011, 11, 25), (10, 0, 0)), CalendarResolution::Daily),
        Some(et((2011, 11, 25), (17, 0, 0)))
    );
    assert_eq!(
        calendar.trade_date(et((2011, 11, 25), (14, 0, 0))),
        Some(NASDAQ_HALF_DAY)
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

    // Both stop the regular session at 13:00 ET.
    assert!(!clipped.is_open_regular(et((2011, 11, 25), (13, 0, 0))));
    assert!(!replaced.is_open_regular(et((2011, 11, 25), (13, 0, 0))));

    // Only the replacement layer keeps the afternoon extended session.
    assert!(!clipped.is_open(et((2011, 11, 25), (14, 0, 0))));
    assert!(replaced.is_open_extended(et((2011, 11, 25), (14, 0, 0))));
}

#[test]
fn a_closed_record_removes_the_whole_trading_day() {
    let table = nasdaq_table();
    let calendar = calendar_for_exchange(Exchange::Nasdaq)
        .with_session_exceptions(&table)
        .expect("the fixture is scoped to this calendar");

    assert!(!calendar.is_open(et((2011, 11, 24), (10, 0, 0))));
    assert!(!calendar.is_open(et((2011, 11, 24), (18, 0, 0))));
    assert!(calendar.is_closed_trade_date(NASDAQ_THANKSGIVING, SessionKind::Both));
    assert!(calendar.is_closed_all_day_on(NASDAQ_THANKSGIVING, SessionKind::Both));
    assert_eq!(
        calendar.next_session_open_after(et((2011, 11, 23), (20, 0, 0))),
        Some(et((2011, 11, 25), (7, 0, 0)))
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

    assert!(calendar.is_open_regular(et((2011, 11, 25), (10, 59, 59))));
    assert!(!calendar.is_open(et((2011, 11, 25), (11, 0, 0))));
    assert!(!calendar.is_open(et((2011, 11, 25), (14, 0, 0))));
    assert_eq!(
        calendar.candle_end(et((2011, 11, 25), (10, 0, 0)), CalendarResolution::Daily),
        Some(et((2011, 11, 25), (11, 0, 0)))
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

    assert!(!calendar.is_open(et((2011, 11, 25), (10, 59, 59))));
    assert!(calendar.is_open_regular(et((2011, 11, 25), (11, 0, 0))));
    assert!(calendar.is_open_extended(et((2011, 11, 25), (14, 0, 0))));
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

    assert!(!calendar.is_open(et((2011, 11, 25), (10, 0, 0))));
    assert!(!calendar.is_open(et((2011, 11, 25), (14, 0, 0))));
    assert!(calendar.is_closed_trade_date(NASDAQ_HALF_DAY, SessionKind::Both));
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
    // did not survive to compose with it.
    assert!(calendar.is_open_regular(et((2011, 11, 25), (10, 0, 0))));
    assert_eq!(
        calendar.session_exception_on(NASDAQ_HALF_DAY),
        Some(DateException::ReplaceSessions(&NASDAQ_HALF_DAY_BLOCKS))
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
            calendar.is_open(instant),
            "weekend trading vanished at {instant}"
        );
        assert_eq!(calendar.trade_date(instant), Some(tuesday));
    }
    assert!(calendar.is_closed_trade_date(monday, SessionKind::Both));
    assert_eq!(
        calendar.candle_start(ct((2026, 6, 7), (12, 0, 0)), CalendarResolution::Daily),
        Some(ct((2026, 6, 5), (16, 1, 0)))
    );
    assert_eq!(
        calendar.candle_end(ct((2026, 6, 7), (12, 0, 0)), CalendarResolution::Daily),
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
        assert!(calendar.is_open(instant), "24x7 trading vanished at {hour}");
        assert_eq!(
            calendar.session_bounds(instant),
            plain.session_bounds(instant)
        );
    }
    // Unchanged from the no-overlay answer: an always-open profile has no trade
    // date to close or replace.
    assert_eq!(
        calendar.is_closed_trade_date(trade_date, SessionKind::Both),
        plain.is_closed_trade_date(trade_date, SessionKind::Both)
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
            closed_calendar.is_open(instant),
            "a closed record removed 24x7 trading at {hour}"
        );
        assert_eq!(
            closed_calendar.session_bounds(instant),
            plain.session_bounds(instant)
        );
    }
    assert_eq!(
        closed_calendar.is_closed_trade_date(trade_date, SessionKind::Both),
        plain.is_closed_trade_date(trade_date, SessionKind::Both)
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
    // the distinction is only reachable through the provider surface.
    let calendar = calendar_for_exchange(Exchange::Nasdaq)
        .with_session_exceptions(&table)
        .expect("the fixture is scoped to this calendar");
    assert!(calendar.is_open_regular(et((2011, 11, 23), (14, 0, 0))));
    assert!(calendar.is_open_regular(et((2011, 11, 29), (14, 0, 0))));
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
        assert_eq!(
            calendar.candle_end(instant, CalendarResolution::Daily),
            plain.candle_end(instant, CalendarResolution::Daily),
            "daily close diverged at {instant}"
        );
        instant += chrono::TimeDelta::minutes(37);
    }
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
    assert_eq!(
        calendar.session_bounds(instant),
        plain.session_bounds(instant)
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
            calendar.session_bounds(instant),
            Some(expected),
            "the fold block did not cover {instant}"
        );
        assert!(calendar.is_open(instant));
        assert_eq!(calendar.trade_date(instant), Some(trade_date));
    }
    assert!(!calendar.is_open(utc((2026, 11, 1), (6, 29, 59))));
    assert!(!calendar.is_open(utc((2026, 11, 1), (7, 45, 0))));
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
        calendar.session_bounds(utc((2026, 3, 8), (8, 30, 0))),
        Some((utc((2026, 3, 8), (8, 0, 0)), utc((2026, 3, 8), (9, 0, 0))))
    );
    assert!(calendar.is_open(utc((2026, 3, 8), (8, 0, 0))));
    assert!(!calendar.is_open(utc((2026, 3, 8), (7, 59, 59))));
    assert!(!calendar.is_open(utc((2026, 3, 8), (9, 0, 0))));
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
    assert!(!calendar.is_open(instant));
    assert_eq!(calendar.session_bounds(instant), None);
    assert_eq!(calendar.next_session_open_after(instant), None);
    assert_eq!(calendar.next_session_after(instant), None);
    for offset in 0..21 {
        let day_in_window = first
            .checked_add_signed(chrono::Duration::days(offset))
            .expect("in-range fixture date");
        assert!(calendar.is_closed_trade_date(day_in_window, SessionKind::Both));
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
