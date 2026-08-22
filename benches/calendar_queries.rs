// SPDX-License-Identifier: MIT-0

//! Query-cost baseline for the Globex equity-index family calendar.

use std::hint::black_box;

use chrono::{TimeZone, Utc};
use criterion::{Criterion, criterion_group, criterion_main};
use exchange_hours::{CalendarResolution, MarketHoursKey, calendar_for_market_hours_key};

fn calendar_queries(criterion: &mut Criterion) {
    let Some(instant) = Utc.with_ymd_and_hms(2026, 4, 20, 15, 0, 0).single() else {
        return;
    };
    let Some(closed_instant) = Utc.with_ymd_and_hms(2026, 4, 20, 21, 30, 0).single() else {
        return;
    };
    let calendar = calendar_for_market_hours_key(MarketHoursKey::GlobexEquityIndex);
    let mut group = criterion.benchmark_group("globex_equity_index");

    group.bench_function("is_open", |bencher| {
        bencher.iter(|| calendar.is_open(black_box(instant)));
    });
    group.bench_function("session_bounds", |bencher| {
        bencher.iter(|| calendar.session_bounds(black_box(instant)));
    });
    group.bench_function("candle_end_daily", |bencher| {
        bencher
            .iter(|| calendar.candle_end(black_box(instant), black_box(CalendarResolution::Daily)));
    });
    group.bench_function("trade_date", |bencher| {
        bencher.iter(|| calendar.trade_date(black_box(instant)));
    });
    group.bench_function("session_state_closed", |bencher| {
        bencher.iter(|| calendar.session_state(black_box(closed_instant)));
    });

    group.finish();
}

criterion_group!(benches, calendar_queries);
criterion_main!(benches);
