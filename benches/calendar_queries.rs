// SPDX-License-Identifier: MIT-0

//! Query-cost baseline for the Globex equity-index family calendar.
//!
//! Three groups, in the order the design memo's performance plan asks for them.
//!
//! `globex_equity_index` is the bare date-aware calendar across the six
//! instant classes — regular, overnight, maintenance, closed weekend, and the
//! two the design memo's §6.2 asks a shipped table to be split by, a trade date
//! **adjacent** to a built-in row and one **on** it — plus the daily
//! trading-day derivation. The family ships a table, so "bare" here means the
//! built-in layer and nothing else: it is the cost a consumer actually pays.
//!
//! `overlay_layers` is the matrix that decides whether a built-in table can sit
//! on the consumer's hot path. It measures the bare calendar, the
//! `without_holidays` control, and a caller's two layers in the two states that
//! matter — a coverage window the queried date falls outside, which the gate
//! answers with one binary search, and a window it falls inside, which pays for
//! the full trading-day derivation.
//!
//! `cold_axis` reproduces the consumer's cold chart frame: 4,999 `is_open`
//! probes plus one daily close over a 5,000-point window. That is the figure
//! that decides whether a frame drops.

use std::hint::black_box;

use chrono::{DateTime, NaiveDate, TimeDelta, TimeZone, Utc};
use chrono_tz::US;
use criterion::{Criterion, criterion_group, criterion_main};
use exchange_hours::{
    CalendarResolution, CalendarSource, DayOverride, ExchangeCalendar, MarketHoursKey, SessionKind,
    StaticDayPolicy, StaticSessionExceptions, calendar_for_market_hours_key,
};

/// A 5,000-point chart frame, the shape the consumer's time axis builds.
const FRAME_POINTS: i64 = 5_000;

fn ct(date: (i32, u32, u32), time: (u32, u32)) -> Option<DateTime<Utc>> {
    Some(
        US::Central
            .with_ymd_and_hms(date.0, date.1, date.2, time.0, time.1, 0)
            .single()?
            .with_timezone(&Utc),
    )
}

/// The six instant classes, so the numbers line up with the recorded baseline.
///
/// `adjacent` and `holiday` are both **regular-session** minutes, chosen so the
/// only variable against `regular` is the date class the design memo's §6.2
/// asks for. A holiday minute after the day's clipped close would instead
/// measure the closed-instant scan, which is a different question.
struct Instants {
    regular: DateTime<Utc>,
    overnight: DateTime<Utc>,
    maintenance: DateTime<Utc>,
    closed: DateTime<Utc>,
    adjacent: DateTime<Utc>,
    holiday: DateTime<Utc>,
}

fn instants() -> Option<Instants> {
    Some(Instants {
        // Monday inside the 08:30-15:15 CT regular session.
        regular: ct((2026, 4, 20), (10, 0))?,
        // Monday evening, inside the leg that wraps to Tuesday 08:30 CT.
        overnight: ct((2026, 4, 20), (20, 0))?,
        // The 16:00-16:45 CT inter-trade-date break.
        maintenance: ct((2026, 4, 20), (16, 30))?,
        // Saturday: the whole weekend shutdown.
        closed: ct((2026, 4, 18), (12, 0))?,
        // Wednesday 2026-11-25, the trade date immediately before Thanksgiving:
        // the gate opens on the neighbouring row and the search misses.
        adjacent: ct((2026, 11, 25), (10, 0))?,
        // Friday 2026-11-27, which the built-in table clips to a 12:15 CT final
        // close: the gate opens, the search hits, and the clip applies, all
        // while the minute is still inside the regular session.
        holiday: ct((2026, 11, 27), (10, 0))?,
    })
}

/// Runs the five queries of the performance matrix over one calendar.
fn bench_queries(
    group: &mut criterion::BenchmarkGroup<'_, criterion::measurement::WallTime>,
    label: &str,
    run: impl Fn(DateTime<Utc>) -> bool + Copy,
    probes: &Instants,
) {
    for (class, instant) in [
        ("regular", probes.regular),
        ("overnight", probes.overnight),
        ("maintenance", probes.maintenance),
        ("closed_weekend", probes.closed),
        ("adjacent", probes.adjacent),
        ("holiday", probes.holiday),
    ] {
        group.bench_function(format!("{label}/is_open/{class}"), |bencher| {
            bencher.iter(|| run(black_box(instant)));
        });
    }
}

fn calendar_queries(criterion: &mut Criterion) {
    let Some(probes) = instants() else {
        return;
    };
    let calendar = calendar_for_market_hours_key(MarketHoursKey::GlobexEquityIndex);
    let mut group = criterion.benchmark_group("globex_equity_index");

    group.bench_function("is_open", |bencher| {
        bencher.iter(|| calendar.is_open(black_box(probes.regular)));
    });
    group.bench_function("session_bounds", |bencher| {
        bencher.iter(|| calendar.session_bounds(black_box(probes.regular)));
    });
    group.bench_function("candle_end_daily", |bencher| {
        bencher.iter(|| {
            calendar.candle_end(
                black_box(probes.regular),
                black_box(CalendarResolution::Daily),
            )
        });
    });
    group.bench_function("trade_date", |bencher| {
        bencher.iter(|| calendar.trade_date(black_box(probes.regular)));
    });
    group.bench_function("session_state_closed", |bencher| {
        bencher.iter(|| calendar.session_state(black_box(probes.maintenance)));
    });

    group.bench_function("is_open/overnight", |bencher| {
        bencher.iter(|| calendar.is_open(black_box(probes.overnight)));
    });
    group.bench_function("is_open/maintenance", |bencher| {
        bencher.iter(|| calendar.is_open(black_box(probes.maintenance)));
    });
    group.bench_function("is_open/closed_weekend", |bencher| {
        bencher.iter(|| calendar.is_open(black_box(probes.closed)));
    });
    group.bench_function("is_open/adjacent", |bencher| {
        bencher.iter(|| calendar.is_open(black_box(probes.adjacent)));
    });
    group.bench_function("is_open/holiday", |bencher| {
        bencher.iter(|| calendar.is_open(black_box(probes.holiday)));
    });
    group.bench_function("session_state/regular", |bencher| {
        bencher.iter(|| calendar.session_state(black_box(probes.regular)));
    });
    group.bench_function("session_state/closed_weekend", |bencher| {
        bencher.iter(|| calendar.session_state(black_box(probes.closed)));
    });
    // The daily trading-day derivation: both ends of the window, which is what
    // an overlay pays for per rule when the coverage gate opens.
    group.bench_function("daily_window", |bencher| {
        bencher.iter(|| daily_window(calendar, black_box(probes.regular)));
    });

    group.finish();
}

fn overlay_layers(criterion: &mut Criterion) {
    let Some(probes) = instants() else {
        return;
    };
    let (Some(covered), Some(remote_first), Some(remote_last)) = (
        NaiveDate::from_ymd_opt(2026, 4, 20),
        NaiveDate::from_ymd_opt(2019, 1, 1),
        NaiveDate::from_ymd_opt(2019, 12, 31),
    ) else {
        return;
    };
    let calendar = calendar_for_market_hours_key(MarketHoursKey::GlobexEquityIndex);
    let source = CalendarSource::MarketHoursKey(MarketHoursKey::GlobexEquityIndex);

    // A caller record on the queried trade date: the full derivation runs and
    // the clip applies.
    let on_date = [DayOverride::early_close(covered, 12 * 3_600)];
    let Ok(on_date_policy) = StaticDayPolicy::new(&on_date) else {
        return;
    };
    // A caller record far from the queried date. The policy trait publishes no
    // coverage window, so this is the state the gate cannot yet help with.
    let elsewhere = [DayOverride::early_close(remote_first, 12 * 3_600)];
    let Ok(elsewhere_policy) = StaticDayPolicy::new(&elsewhere) else {
        return;
    };
    // An exception provider whose window the queried date falls outside: the
    // gate exits on one binary search.
    let Ok(remote_table) = StaticSessionExceptions::new(source, remote_first, remote_last, &[])
    else {
        return;
    };
    // ... and one whose window it falls inside, with no record in it.
    let Ok(covering_table) = StaticSessionExceptions::new(source, remote_first, covered, &[])
    else {
        return;
    };

    let Ok(gated) = calendar.with_session_exceptions(&remote_table) else {
        return;
    };
    let Ok(ungated) = calendar.with_session_exceptions(&covering_table) else {
        return;
    };
    let detached = calendar.without_holidays();
    let policy_on_date = calendar.with_day_policy(&on_date_policy);
    let policy_elsewhere = calendar.with_day_policy(&elsewhere_policy);

    let mut group = criterion.benchmark_group("overlay_layers");
    bench_queries(
        &mut group,
        "none",
        |instant| calendar.is_open(instant),
        &probes,
    );
    bench_queries(
        &mut group,
        "without_holidays",
        |instant| detached.is_open(instant),
        &probes,
    );
    bench_queries(
        &mut group,
        "exceptions_gated",
        |instant| gated.is_open(instant),
        &probes,
    );
    bench_queries(
        &mut group,
        "exceptions_in_coverage",
        |instant| ungated.is_open(instant),
        &probes,
    );
    bench_queries(
        &mut group,
        "day_policy_elsewhere",
        |instant| policy_elsewhere.is_open(instant),
        &probes,
    );
    bench_queries(
        &mut group,
        "day_policy_on_date",
        |instant| policy_on_date.is_open(instant),
        &probes,
    );

    // The trading-day derivation an overlay pays for per rule once the gate
    // opens, against the same derivation with no layer attached.
    let windows: [(&str, &dyn Fn() -> bool); 3] = [
        ("none", &|| daily_window(calendar, probes.regular)),
        ("exceptions_gated", &|| {
            gated
                .candle_start(probes.regular, CalendarResolution::Daily)
                .is_some()
                && gated
                    .candle_end(probes.regular, CalendarResolution::Daily)
                    .is_some()
        }),
        ("day_policy_elsewhere", &|| {
            policy_elsewhere
                .candle_start(probes.regular, CalendarResolution::Daily)
                .is_some()
                && policy_elsewhere
                    .candle_end(probes.regular, CalendarResolution::Daily)
                    .is_some()
        }),
    ];
    for (label, run) in windows {
        group.bench_function(format!("{label}/daily_window"), |bencher| {
            bencher.iter(|| black_box(run()));
        });
    }

    group.finish();
}

/// Both ends of one daily trading-day window on a bare calendar.
fn daily_window(calendar: ExchangeCalendar, instant: DateTime<Utc>) -> bool {
    calendar
        .candle_start(instant, CalendarResolution::Daily)
        .is_some()
        && calendar
            .candle_end(instant, CalendarResolution::Daily)
            .is_some()
}

/// One cold chart frame: `FRAME_POINTS - 1` open probes plus one daily close.
fn frame(calendar: ExchangeCalendar, start: DateTime<Utc>) -> usize {
    let mut open = 0_usize;
    for step in 0..FRAME_POINTS.saturating_sub(1) {
        let Some(instant) = start.checked_add_signed(TimeDelta::minutes(step)) else {
            break;
        };
        if calendar.is_open(instant) {
            open = open.saturating_add(1);
        }
    }
    if calendar
        .candle_end(start, CalendarResolution::Daily)
        .is_some()
    {
        open = open.saturating_add(1);
    }
    open
}

fn cold_axis(criterion: &mut Criterion) {
    let (Some(regular), Some(closed)) = (ct((2026, 4, 20), (0, 0)), ct((2026, 4, 18), (0, 0)))
    else {
        return;
    };
    let calendar = calendar_for_market_hours_key(MarketHoursKey::GlobexEquityIndex);
    let detached = calendar.without_holidays();

    let mut group = criterion.benchmark_group("cold_axis");
    group.sample_size(20);
    group.bench_function("none/regular", |bencher| {
        bencher.iter(|| frame(calendar, black_box(regular)));
    });
    group.bench_function("none/closed_weekend", |bencher| {
        bencher.iter(|| frame(calendar, black_box(closed)));
    });
    group.bench_function("without_holidays/regular", |bencher| {
        bencher.iter(|| frame(detached, black_box(regular)));
    });
    group.finish();
}

/// The five-query surface the design memo asks for, on the bare calendar.
///
/// `is_open` is the hot path and gets its own instant classes above; these are
/// the remaining four queries at one instant each, so a regression in any of
/// them is visible.
fn query_surface(criterion: &mut Criterion) {
    let Some(probes) = instants() else {
        return;
    };
    let calendar = calendar_for_market_hours_key(MarketHoursKey::GlobexEquityIndex);
    let mut group = criterion.benchmark_group("query_surface");

    group.bench_function("session_bounds/overnight", |bencher| {
        bencher.iter(|| calendar.session_bounds(black_box(probes.overnight)));
    });
    group.bench_function("trade_date/overnight", |bencher| {
        bencher.iter(|| calendar.trade_date(black_box(probes.overnight)));
    });
    group.bench_function("is_closed_trade_date", |bencher| {
        let Some(day) = NaiveDate::from_ymd_opt(2026, 4, 20) else {
            return;
        };
        bencher.iter(|| calendar.is_closed_trade_date(black_box(day), SessionKind::Both));
    });
    group.bench_function("holiday_on", |bencher| {
        let Some(day) = NaiveDate::from_ymd_opt(2026, 4, 20) else {
            return;
        };
        bencher.iter(|| calendar.holiday_on(black_box(day)));
    });

    group.finish();
}

criterion_group!(
    benches,
    calendar_queries,
    overlay_layers,
    cold_axis,
    query_surface
);
criterion_main!(benches);
