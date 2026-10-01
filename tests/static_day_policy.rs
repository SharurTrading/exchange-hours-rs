// SPDX-License-Identifier: MIT-0

//! Public contracts for caller-owned, static trade-date overrides.

#![expect(
    clippy::expect_used,
    reason = "fixture literals and validated static records must fail the test if malformed"
)]

use chrono::{DateTime, NaiveDate, TimeZone, Utc};
use chrono_tz::US;
use exchange_hours::{
    DayOverride, DayPolicy, MarketHoursKey, NoPolicy, StaticDayPolicy, StaticDayPolicyError,
    calendar_for_market_hours_key,
};

const MONDAY: NaiveDate = NaiveDate::from_ymd_opt(2026, 4, 20).expect("valid fixture date");
const TUESDAY: NaiveDate = NaiveDate::from_ymd_opt(2026, 4, 21).expect("valid fixture date");
const WEDNESDAY: NaiveDate = NaiveDate::from_ymd_opt(2026, 4, 22).expect("valid fixture date");
const THURSDAY: NaiveDate = NaiveDate::from_ymd_opt(2026, 4, 23).expect("valid fixture date");

static OVERRIDES: [DayOverride; 4] = [
    DayOverride::closed(MONDAY),
    DayOverride::early_close(TUESDAY, 12 * 3_600 + 15 * 60),
    DayOverride::late_open(WEDNESDAY, 9 * 3_600 + 30 * 60),
    DayOverride::late_open_and_early_close(THURSDAY, 17 * 3_600 + 30 * 60, 12 * 3_600 + 15 * 60),
];

const POLICY_RESULT: Result<StaticDayPolicy<'static>, StaticDayPolicyError> =
    StaticDayPolicy::new(&OVERRIDES);

fn ct(date: (i32, u32, u32), time: (u32, u32, u32)) -> DateTime<Utc> {
    US::Central
        .with_ymd_and_hms(date.0, date.1, date.2, time.0, time.1, time.2)
        .single()
        .expect("fixture must be a valid CT instant")
        .with_timezone(&Utc)
}

fn policy() -> StaticDayPolicy<'static> {
    POLICY_RESULT.expect("the static records must be valid")
}

fn assert_copy_send_sync_static<T: Copy + Send + Sync + 'static>() {}

#[test]
fn constructors_are_const_and_expose_only_their_selected_state() {
    let policy = policy();

    assert_copy_send_sync_static::<DayOverride>();
    assert_copy_send_sync_static::<StaticDayPolicy<'static>>();
    assert_eq!(policy.overrides(), &OVERRIDES);

    let closed = policy.override_on(MONDAY).expect("Monday has a record");
    assert_eq!(closed.trade_date(), MONDAY);
    assert!(closed.is_closed());
    assert_eq!(closed.early_close_ssm(), None);
    assert_eq!(closed.late_open_ssm(), None);

    let early = policy.override_on(TUESDAY).expect("Tuesday has a record");
    assert!(!early.is_closed());
    assert_eq!(early.early_close_ssm(), Some(12 * 3_600 + 15 * 60));
    assert_eq!(early.late_open_ssm(), None);

    let late = policy
        .override_on(WEDNESDAY)
        .expect("Wednesday has a record");
    assert_eq!(late.early_close_ssm(), None);
    assert_eq!(late.late_open_ssm(), Some(9 * 3_600 + 30 * 60));

    let combined = policy.override_on(THURSDAY).expect("Thursday has a record");
    assert_eq!(combined.late_open_ssm(), Some(17 * 3_600 + 30 * 60));
    assert_eq!(combined.early_close_ssm(), Some(12 * 3_600 + 15 * 60));
}

#[test]
fn binary_search_is_exact_and_missing_dates_keep_normal_values() {
    let policy = policy();
    let friday = NaiveDate::from_ymd_opt(2026, 4, 24).expect("valid fixture date");

    assert_eq!(policy.override_on(friday), None);
    assert!(!policy.is_closed(friday));
    assert_eq!(policy.early_close_ssm(friday), None);
    assert_eq!(policy.late_open_ssm(friday), None);

    assert!(policy.is_closed(MONDAY));
    assert_eq!(policy.early_close_ssm(TUESDAY), Some(12 * 3_600 + 15 * 60));
    assert_eq!(policy.late_open_ssm(WEDNESDAY), Some(9 * 3_600 + 30 * 60));
}

#[test]
fn empty_policy_is_valid_and_total_at_chrono_date_extremes() {
    let empty = StaticDayPolicy::new(&[]).expect("an empty policy is valid");
    assert_eq!(empty.override_on(MONDAY), None);

    let records = [
        DayOverride::closed(NaiveDate::MIN),
        DayOverride::closed(NaiveDate::MAX),
    ];
    let policy = StaticDayPolicy::new(&records).expect("extreme dates remain ordered");

    for date in [NaiveDate::MIN, NaiveDate::MAX] {
        assert_eq!(policy.override_on(date), Some(DayOverride::closed(date)));
        assert!(policy.is_closed(date));
        assert_eq!(policy.early_close_ssm(date), None);
        assert_eq!(policy.late_open_ssm(date), None);
    }
}

#[test]
fn validation_rejects_the_first_ordering_violation() {
    let duplicate = [
        DayOverride::closed(MONDAY),
        DayOverride::early_close(MONDAY, 12 * 3_600),
        DayOverride::late_open(TUESDAY, 86_400),
    ];
    assert_eq!(
        StaticDayPolicy::new(&duplicate),
        Err(StaticDayPolicyError::DatesNotStrictlyIncreasing { index: 1 })
    );

    let descending = [DayOverride::closed(TUESDAY), DayOverride::closed(MONDAY)];
    assert_eq!(
        StaticDayPolicy::new(&descending),
        Err(StaticDayPolicyError::DatesNotStrictlyIncreasing { index: 1 })
    );
}

#[test]
fn validation_enforces_each_ssm_domain_without_comparing_wrapped_boundaries() {
    let valid_edges = [
        DayOverride::early_close(MONDAY, 86_400),
        DayOverride::late_open(TUESDAY, 86_399),
        DayOverride::late_open_and_early_close(WEDNESDAY, 17 * 3_600, 12 * 3_600 + 15 * 60),
    ];
    assert!(StaticDayPolicy::new(&valid_edges).is_ok());

    let invalid_early = [DayOverride::early_close(MONDAY, 86_401)];
    assert_eq!(
        StaticDayPolicy::new(&invalid_early),
        Err(StaticDayPolicyError::EarlyCloseOutOfRange {
            index: 0,
            early_close_ssm: 86_401,
        })
    );

    let invalid_late = [DayOverride::late_open(MONDAY, 86_400)];
    assert_eq!(
        StaticDayPolicy::new(&invalid_late),
        Err(StaticDayPolicyError::LateOpenOutOfRange {
            index: 0,
            late_open_ssm: 86_400,
        })
    );
}

#[test]
fn combined_override_clips_a_wrapped_globex_trade_date() {
    let records = [DayOverride::late_open_and_early_close(
        MONDAY,
        17 * 3_600 + 30 * 60,
        12 * 3_600 + 15 * 60,
    )];
    let policy = StaticDayPolicy::new(&records).expect("wrapped boundaries are valid");
    let calendar =
        calendar_for_market_hours_key(MarketHoursKey::GlobexEquityIndex).with_day_policy(&policy);

    assert!(
        !calendar
            .is_open(ct((2026, 4, 19), (17, 29, 59)))
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        calendar
            .is_open(ct((2026, 4, 19), (17, 30, 0)))
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        calendar
            .is_open(ct((2026, 4, 20), (12, 14, 59)))
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        !calendar
            .is_open(ct((2026, 4, 20), (12, 15, 0)))
            .expect("the coverage contract must answer a covered date")
    );
}

#[test]
fn closed_crypto_monday_uses_the_identity_specific_rollover() {
    let monday = NaiveDate::from_ymd_opt(2026, 6, 8).expect("valid fixture date");
    let tuesday = NaiveDate::from_ymd_opt(2026, 6, 9).expect("valid fixture date");
    let records = [DayOverride::closed(monday)];
    let policy = StaticDayPolicy::new(&records).expect("the closed date is valid");
    let calendar = calendar_for_market_hours_key(MarketHoursKey::GlobexCryptocurrency)
        .with_day_policy(&policy);

    assert!(
        calendar
            .is_open(ct((2026, 6, 7), (12, 0, 0)))
            .expect("the coverage contract must answer a covered date")
    );
    assert_eq!(
        calendar
            .trade_date(ct((2026, 6, 7), (12, 0, 0)))
            .expect("the coverage contract must answer a covered date"),
        Some(tuesday)
    );
    assert!(
        calendar
            .is_open(ct((2026, 6, 8), (10, 0, 0)))
            .expect("the coverage contract must answer a covered date")
    );
    assert_eq!(
        calendar
            .trade_date(ct((2026, 6, 8), (10, 0, 0)))
            .expect("the coverage contract must answer a covered date"),
        Some(tuesday)
    );
}

/// The coverage window the engine's gate consults (`DayPolicy::may_affect`,
/// issue #94): `true` exactly when a record is keyed inside the inclusive
/// window, one binary search wide, with `false` only where no record can be
/// consulted.
#[test]
fn may_affect_answers_whether_a_record_is_keyed_inside_the_window() {
    let policy = policy();
    let friday = NaiveDate::from_ymd_opt(2026, 4, 24).expect("valid fixture date");

    // Windows that hold no record, on both sides of the span.
    assert!(!policy.may_affect(friday, friday));
    assert!(!policy.may_affect(
        NaiveDate::from_ymd_opt(2026, 4, 10).expect("valid fixture date"),
        NaiveDate::from_ymd_opt(2026, 4, 17).expect("valid fixture date"),
    ));
    // Single-day windows on every record, and windows that touch their edges.
    assert!(policy.may_affect(MONDAY, MONDAY));
    assert!(policy.may_affect(TUESDAY, TUESDAY));
    assert!(policy.may_affect(WEDNESDAY, WEDNESDAY));
    assert!(policy.may_affect(THURSDAY, THURSDAY));
    assert!(policy.may_affect(MONDAY, friday));
    assert!(policy.may_affect(THURSDAY, NaiveDate::MAX));
    assert!(policy.may_affect(NaiveDate::MIN, MONDAY));
    // The whole representable calendar holds the records.
    assert!(policy.may_affect(NaiveDate::MIN, NaiveDate::MAX));

    // An empty policy holds nothing anywhere; the never-affecting NoPolicy
    // answers the same, so the engine's gate may always skip its derivation.
    let empty = StaticDayPolicy::new(&[]).expect("an empty policy is valid");
    assert!(!empty.may_affect(NaiveDate::MIN, NaiveDate::MAX));
    assert!(!NoPolicy.may_affect(NaiveDate::MIN, NaiveDate::MAX));
}

/// A policy whose only record is far from the queried days must answer exactly
/// as the bare calendar, refusal for refusal (issue #94's gate).
///
/// The gate now exits on [`StaticDayPolicy`]'s own `may_affect`, so the full
/// trading-day derivation is skipped for every instant probed here. The
/// relation asserted is therefore stronger than overlay-agreement: the
/// overlaid calendar is **equal**, not merely compatible, because a policy
/// with no record in an occurrence's trade-date window cannot change it. The
/// record itself must still apply — asserted at its own date, which is the
/// failure mode of a `may_affect` that answers `false` too often.
#[test]
fn a_policy_record_far_from_the_queried_days_leaves_every_answer_unchanged() {
    let calendar = calendar_for_market_hours_key(MarketHoursKey::GlobexEquityIndex);
    let far = NaiveDate::from_ymd_opt(2019, 7, 5).expect("valid fixture date");
    let records = [DayOverride::closed(far)];
    let policy = StaticDayPolicy::new(&records).expect("the far record is valid");
    let overlaid = calendar.with_day_policy(&policy);

    // A fortnight of trading around the probe date — regular minutes, the
    // maintenance gap, the overnight leg and a weekend — plus the day before
    // and on Thanksgiving, the row-adjacent and row-on classes the gate
    // derivation is most expensive on.
    let mut instant = ct((2026, 4, 13), (0, 0, 0));
    let end = ct((2026, 4, 27), (0, 0, 0));
    while instant < end {
        assert_eq!(
            overlaid.is_open(instant).ok(),
            calendar.is_open(instant).ok(),
            "is_open moved at {instant}"
        );
        assert_eq!(
            overlaid.trade_date(instant).ok().flatten(),
            calendar.trade_date(instant).ok().flatten(),
            "trade_date moved at {instant}"
        );
        assert_eq!(
            overlaid.session_bounds(instant).ok().flatten(),
            calendar.session_bounds(instant).ok().flatten(),
            "session_bounds moved at {instant}"
        );
        instant += chrono::TimeDelta::try_minutes(30).expect("30 minutes is representable");
    }
    for (label, probe) in [
        (
            "row-adjacent regular minute",
            ct((2026, 11, 25), (10, 0, 0)),
        ),
        ("row minute", ct((2026, 11, 27), (10, 0, 0))),
    ] {
        assert_eq!(
            overlaid.is_open(probe).ok(),
            calendar.is_open(probe).ok(),
            "{label} moved"
        );
        assert_eq!(
            overlaid.session_bounds(probe).ok().flatten(),
            calendar.session_bounds(probe).ok().flatten(),
            "{label} bounds moved"
        );
    }

    // The far record still applies: the caller closes an ordinary far Friday
    // that no built-in row names, deleting its own day session and the
    // wrapped leg that closes into it. This is the failure mode of a
    // `may_affect` that answers `false` too often — the record silently
    // stops mattering.
    assert!(
        calendar
            .is_open(ct((2019, 7, 5), (10, 0, 0)))
            .expect("the coverage contract must answer a covered date"),
        "the far fixture date must be an ordinary trading minute bare"
    );
    assert!(
        !overlaid
            .is_open(ct((2019, 7, 5), (10, 0, 0)))
            .expect("the coverage contract must answer a covered date"),
        "the far record must still close its own date"
    );
    assert!(
        calendar
            .is_open(ct((2019, 7, 4), (19, 0, 0)))
            .expect("the coverage contract must answer a covered date"),
        "the wrapped leg feeding the far date must trade bare"
    );
    assert!(
        !overlaid
            .is_open(ct((2019, 7, 4), (19, 0, 0)))
            .expect("the coverage contract must answer a covered date"),
        "the wrapped leg carrying the far closed trade date must be removed"
    );
}
