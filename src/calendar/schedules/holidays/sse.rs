// SPDX-License-Identifier: MIT-0

//! Shanghai Stock Exchange holiday rows, 2025-2026.
//!
//! Keyed by the crate's own venue-local trade date in `Asia/Shanghai` (design
//! memo D1). SSE runs no overnight wrap and no weekend sessions, so a closed
//! trade date is one whose daytime sessions are absent outright.
//!
//! The whole block is **T1**: the operator's own annual closure-arrangement
//! notices on `sse.com.cn` — 上证公告〔2024〕38号 for 2025 and 上证公告〔2025〕45号
//! for 2026 — each stating every holiday closure of its year, with 上证公告
//! 〔2025〕36号 restating the 2025 October block verbatim. The notices are
//! printed as event-date ranges ("10月1日（星期三）至10月8日（星期三）休市，10月9日
//! （星期四）起照常开市"), and a row ships for each **weekday** the range
//! covers: the range endpoints are inclusive closures, and the days inside a
//! range that fall on a Saturday or Sunday are already closed by the normal
//! week. The notices' "另外，X为周末休市" clauses are the national
//! working-weekend swaps (调休); the stock market keeps those days as ordinary
//! weekend closures and no session exists to encode, so none is invented.
//! The derivation and per-row quotations are recorded in
//! [`docs/evidence/sse.md`](../../../../../docs/evidence/sse.md).
//!
//! SSE publishes the next year's arrangement each December; no 2027 notice
//! exists as of the 2026-09-28 retrieval, so the window ends 2026-12-31.

use super::EvidenceTier::T1;
use super::HolidayKind::Closed;
use super::{HolidayTable, holidays};

/// SSE's built-in holiday rows and the window they were audited over.
///
/// Every row is one weekday inside an event-date range the operator's own
/// notice prints as a closure: `SSE-NOTICE-2024-38` for 2025 and
/// `SSE-NOTICE-2025-45` for 2026. A date inside the window with no row is
/// audited normal.
// Evidence: docs/evidence/sse.md
pub(crate) static TABLE: &HolidayTable = holidays! {
    coverage: [(2025, 1, 1) ..= (2026, 12, 31)],
    rows: [
        // 2025-01-01 - T1 - SSE-NOTICE-2024-38 - 元旦: "1月1日（星期三）休市".
        (2025, 1, 1, Closed, T1, "SSE-NOTICE-2024-38"),
        // 2025-01-28 - T1 - SSE-NOTICE-2024-38 - 春节: "1月28日（星期二）至2月4日
        // （星期二）休市"; weekday legs Jan 28-31.
        (2025, 1, 28, Closed, T1, "SSE-NOTICE-2024-38"),
        // 2025-01-29 - T1 - SSE-NOTICE-2024-38 - 春节 (weekday leg).
        (2025, 1, 29, Closed, T1, "SSE-NOTICE-2024-38"),
        // 2025-01-30 - T1 - SSE-NOTICE-2024-38 - 春节 (weekday leg).
        (2025, 1, 30, Closed, T1, "SSE-NOTICE-2024-38"),
        // 2025-01-31 - T1 - SSE-NOTICE-2024-38 - 春节 (weekday leg; Feb 1-2 are
        // the weekend inside the range).
        (2025, 1, 31, Closed, T1, "SSE-NOTICE-2024-38"),
        // 2025-02-03 - T1 - SSE-NOTICE-2024-38 - 春节 (weekday leg).
        (2025, 2, 3, Closed, T1, "SSE-NOTICE-2024-38"),
        // 2025-02-04 - T1 - SSE-NOTICE-2024-38 - 春节: the range's own last day.
        (2025, 2, 4, Closed, T1, "SSE-NOTICE-2024-38"),
        // 2025-04-04 - T1 - SSE-NOTICE-2024-38 - 清明节: "4月4日（星期五）至4月6日
        // （星期日）休市"; the Friday is the range's only weekday.
        (2025, 4, 4, Closed, T1, "SSE-NOTICE-2024-38"),
        // 2025-05-01 - T1 - SSE-NOTICE-2024-38 - 劳动节: "5月1日（星期四）至5月5日
        // （星期一）休市"; weekday legs May 1-2 and May 5.
        (2025, 5, 1, Closed, T1, "SSE-NOTICE-2024-38"),
        // 2025-05-02 - T1 - SSE-NOTICE-2024-38 - 劳动节 (weekday leg).
        (2025, 5, 2, Closed, T1, "SSE-NOTICE-2024-38"),
        // 2025-05-05 - T1 - SSE-NOTICE-2024-38 - 劳动节: the range's own last day.
        (2025, 5, 5, Closed, T1, "SSE-NOTICE-2024-38"),
        // 2025-06-02 - T1 - SSE-NOTICE-2024-38 - 端午节: "5月31日（星期六）至6月2日
        // （星期一）休市"; the Monday is the range's only weekday.
        (2025, 6, 2, Closed, T1, "SSE-NOTICE-2024-38"),
        // 2025-10-01 - T1 - SSE-NOTICE-2024-38 - 国庆节、中秋节: "10月1日（星期三）
        // 至10月8日（星期三）休市"; weekday legs Oct 1-3 and Oct 6-8. Restated
        // verbatim by SSE-NOTICE-2025-36.
        (2025, 10, 1, Closed, T1, "SSE-NOTICE-2024-38"),
        // 2025-10-02 - T1 - SSE-NOTICE-2024-38 - 国庆节、中秋节 (weekday leg).
        (2025, 10, 2, Closed, T1, "SSE-NOTICE-2024-38"),
        // 2025-10-03 - T1 - SSE-NOTICE-2024-38 - 国庆节、中秋节 (weekday leg;
        // Oct 4 and Oct 5 are the weekend inside the range).
        (2025, 10, 3, Closed, T1, "SSE-NOTICE-2024-38"),
        // 2025-10-06 - T1 - SSE-NOTICE-2024-38 - 国庆节、中秋节 (weekday leg).
        (2025, 10, 6, Closed, T1, "SSE-NOTICE-2024-38"),
        // 2025-10-07 - T1 - SSE-NOTICE-2024-38 - 国庆节、中秋节 (weekday leg).
        (2025, 10, 7, Closed, T1, "SSE-NOTICE-2024-38"),
        // 2025-10-08 - T1 - SSE-NOTICE-2024-38 - 国庆节、中秋节: the range's own
        // last day.
        (2025, 10, 8, Closed, T1, "SSE-NOTICE-2024-38"),
        // 2026-01-01 - T1 - SSE-NOTICE-2025-45 - 元旦: "1月1日（星期四）至1月3日
        // （星期六）休市"; weekday legs Jan 1-2.
        (2026, 1, 1, Closed, T1, "SSE-NOTICE-2025-45"),
        // 2026-01-02 - T1 - SSE-NOTICE-2025-45 - 元旦 (weekday leg).
        (2026, 1, 2, Closed, T1, "SSE-NOTICE-2025-45"),
        // 2026-02-16 - T1 - SSE-NOTICE-2025-45 - 春节: "2月15日（星期日）至2月23日
        // （星期一）休市"; weekday legs Feb 16-20 and Feb 23.
        (2026, 2, 16, Closed, T1, "SSE-NOTICE-2025-45"),
        // 2026-02-17 - T1 - SSE-NOTICE-2025-45 - 春节 (weekday leg).
        (2026, 2, 17, Closed, T1, "SSE-NOTICE-2025-45"),
        // 2026-02-18 - T1 - SSE-NOTICE-2025-45 - 春节 (weekday leg).
        (2026, 2, 18, Closed, T1, "SSE-NOTICE-2025-45"),
        // 2026-02-19 - T1 - SSE-NOTICE-2025-45 - 春节 (weekday leg).
        (2026, 2, 19, Closed, T1, "SSE-NOTICE-2025-45"),
        // 2026-02-20 - T1 - SSE-NOTICE-2025-45 - 春节 (weekday leg; Feb 21-22 are
        // the weekend inside the range).
        (2026, 2, 20, Closed, T1, "SSE-NOTICE-2025-45"),
        // 2026-02-23 - T1 - SSE-NOTICE-2025-45 - 春节: the range's own last day.
        (2026, 2, 23, Closed, T1, "SSE-NOTICE-2025-45"),
        // 2026-04-06 - T1 - SSE-NOTICE-2025-45 - 清明节: "4月4日（星期六）至4月6日
        // （星期一）休市"; the Monday is the range's only weekday.
        (2026, 4, 6, Closed, T1, "SSE-NOTICE-2025-45"),
        // 2026-05-01 - T1 - SSE-NOTICE-2025-45 - 劳动节: "5月1日（星期五）至5月5日
        // （星期二）休市"; weekday legs May 1 and May 4-5.
        (2026, 5, 1, Closed, T1, "SSE-NOTICE-2025-45"),
        // 2026-05-04 - T1 - SSE-NOTICE-2025-45 - 劳动节 (weekday leg).
        (2026, 5, 4, Closed, T1, "SSE-NOTICE-2025-45"),
        // 2026-05-05 - T1 - SSE-NOTICE-2025-45 - 劳动节: the range's own last day.
        (2026, 5, 5, Closed, T1, "SSE-NOTICE-2025-45"),
        // 2026-06-19 - T1 - SSE-NOTICE-2025-45 - 端午节: "6月19日（星期五）至6月21日
        // （星期日）休市"; the Friday is the range's only weekday.
        (2026, 6, 19, Closed, T1, "SSE-NOTICE-2025-45"),
        // 2026-09-25 - T1 - SSE-NOTICE-2025-45 - 中秋节: "9月25日（星期五）至9月27日
        // （星期日）休市"; the Friday is the range's only weekday.
        (2026, 9, 25, Closed, T1, "SSE-NOTICE-2025-45"),
        // 2026-10-01 - T1 - SSE-NOTICE-2025-45 - 国庆节: "10月1日（星期四）至10月7日
        // （星期三）休市"; weekday legs Oct 1-2 and Oct 5-7.
        (2026, 10, 1, Closed, T1, "SSE-NOTICE-2025-45"),
        // 2026-10-02 - T1 - SSE-NOTICE-2025-45 - 国庆节 (weekday leg).
        (2026, 10, 2, Closed, T1, "SSE-NOTICE-2025-45"),
        // 2026-10-05 - T1 - SSE-NOTICE-2025-45 - 国庆节 (weekday leg).
        (2026, 10, 5, Closed, T1, "SSE-NOTICE-2025-45"),
        // 2026-10-06 - T1 - SSE-NOTICE-2025-45 - 国庆节 (weekday leg).
        (2026, 10, 6, Closed, T1, "SSE-NOTICE-2025-45"),
        // 2026-10-07 - T1 - SSE-NOTICE-2025-45 - 国庆节: the range's own last day.
        (2026, 10, 7, Closed, T1, "SSE-NOTICE-2025-45"),
    ],
};
