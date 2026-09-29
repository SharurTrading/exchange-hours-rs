// SPDX-License-Identifier: MIT-0

//! Shanghai Stock Exchange holiday rows, 2011-2026.
//!
//! Keyed by the crate's own venue-local trade date in `Asia/Shanghai` (design
//! memo D1). SSE runs no overnight wrap and no weekend sessions, so a closed
//! trade date is one whose daytime sessions are absent outright.
//!
//! The whole block is **T1**: the operator's own annual closure-arrangement
//! notices — 关于2011年全年休市安排的通知 onward, each stating every holiday
//! closure of its year. The 2011-2013 notices were retrieved live from the
//! operator's own media-center reprints (`aboutus/mediacenter/hotandd/`),
//! where SSE republishes its pre-2015 notices verbatim; the 2014-2024 notices
//! from the same announcement channel's Wayback `id_` replays; 上证公告
//! 〔2024〕38号 for 2025 and 上证公告〔2025〕45号 for 2026 complete the window,
//! with 上证公告〔2025〕36号 restating the 2025 October block verbatim. The
//! notices are printed as event-date ranges ("10月1日（星期三）至10月8日
//! （星期三）休市，10月9日（星期四）起照常开市"), and a row ships for each
//! **weekday** the range covers: the range endpoints are inclusive closures,
//! and the days inside a range that fall on a Saturday or Sunday are already
//! closed by the normal week. The notices' "另外，X为周末休市" clauses are the
//! national working-weekend swaps (调休); the stock market keeps those days as
//! ordinary weekend closures and no session exists to encode, so none is
//! invented. A range may reach back into the prior December (the 2019, 2023
//! and 2024 notices' 元旦 legs): its weekday legs ship with the notice that
//! states them, so 2018-12-31 keys from 上证公告〔2018〕39号. The derivation and
//! per-row quotations are recorded in
//! [`docs/evidence/sse.md`](../../../../../docs/evidence/sse.md).
//!
//! SSE publishes the next year's arrangement each December; no 2027 notice
//! exists as of the 2026-09-29 retrieval, so the window ends 2026-12-31. The
//! 2010 arrangement (上证交字〔2009〕42号) is unrecovered — the operator's
//! media-center reprints begin with the 2011 notice and the pre-2011-era
//! pages are not in the archive — so 2010 sits outside the audited window as
//! a recorded gap, never a claim.

use super::EvidenceTier::T1;
use super::HolidayKind::Closed;
use super::{HolidayTable, holidays};

/// SSE's built-in holiday rows and the window they were audited over.
///
/// Every row is one weekday inside an event-date range the operator's own
/// notice prints as a closure: `SSE-NOTICE-2011`..`SSE-NOTICE-2014` (the
/// unnumbered annual notices), `SSE-NOTICE-2014-15` .. `SSE-NOTICE-2023-47`
/// (the 上证公告-numbered notices), then `SSE-NOTICE-2024-38` for 2025 and
/// `SSE-NOTICE-2025-45` for 2026. A date inside the window with no row is
/// audited normal.
// Evidence: docs/evidence/sse.md
pub(crate) static TABLE: &HolidayTable = holidays! {
    coverage: [(2011, 1, 1) ..= (2026, 12, 31)],
    rows: [
        // 2011-01-03 - T1 - SSE-NOTICE-2011 - 元旦: "元旦：1月1日（星期六）至1月3日（星期一）为节假日休市"; the range's only weekday.
        (2011, 1, 3, Closed, T1, "SSE-NOTICE-2011"),
        // 2011-02-02 - T1 - SSE-NOTICE-2011 - 春节: "春节：2月2日（星期三）至2月8日（星期二）为节假日休市"; weekday legs.
        (2011, 2, 2, Closed, T1, "SSE-NOTICE-2011"),
        // 2011-02-03 - T1 - SSE-NOTICE-2011 - 春节 (weekday leg).
        (2011, 2, 3, Closed, T1, "SSE-NOTICE-2011"),
        // 2011-02-04 - T1 - SSE-NOTICE-2011 - 春节 (weekday leg).
        (2011, 2, 4, Closed, T1, "SSE-NOTICE-2011"),
        // 2011-02-07 - T1 - SSE-NOTICE-2011 - 春节 (weekday leg).
        (2011, 2, 7, Closed, T1, "SSE-NOTICE-2011"),
        // 2011-02-08 - T1 - SSE-NOTICE-2011 - 春节: the range's own last day.
        (2011, 2, 8, Closed, T1, "SSE-NOTICE-2011"),
        // 2011-04-04 - T1 - SSE-NOTICE-2011 - 清明节: "清明节：4月3日（星期日）至4月5日（星期二）为节假日休市"; weekday legs.
        (2011, 4, 4, Closed, T1, "SSE-NOTICE-2011"),
        // 2011-04-05 - T1 - SSE-NOTICE-2011 - 清明节: the range's own last day.
        (2011, 4, 5, Closed, T1, "SSE-NOTICE-2011"),
        // 2011-05-02 - T1 - SSE-NOTICE-2011 - 劳动节: "劳动节：4月30日（星期六）至5月2日（星期一）为节假日休市"; the range's only weekday.
        (2011, 5, 2, Closed, T1, "SSE-NOTICE-2011"),
        // 2011-06-06 - T1 - SSE-NOTICE-2011 - 端午节: "端午节：6月4日（星期六）至6月6日（星期一）为节假日休市"; the range's only weekday.
        (2011, 6, 6, Closed, T1, "SSE-NOTICE-2011"),
        // 2011-09-12 - T1 - SSE-NOTICE-2011 - 中秋节: "中秋节：9月10日（星期六）至9月12日（星期一）为节假日休市"; the range's only weekday.
        (2011, 9, 12, Closed, T1, "SSE-NOTICE-2011"),
        // 2011-10-03 - T1 - SSE-NOTICE-2011 - 国庆节: "国庆节：10月1日（星期六）至10月7日（星期五）为节假日休市"; weekday legs.
        (2011, 10, 3, Closed, T1, "SSE-NOTICE-2011"),
        // 2011-10-04 - T1 - SSE-NOTICE-2011 - 国庆节 (weekday leg).
        (2011, 10, 4, Closed, T1, "SSE-NOTICE-2011"),
        // 2011-10-05 - T1 - SSE-NOTICE-2011 - 国庆节 (weekday leg).
        (2011, 10, 5, Closed, T1, "SSE-NOTICE-2011"),
        // 2011-10-06 - T1 - SSE-NOTICE-2011 - 国庆节 (weekday leg).
        (2011, 10, 6, Closed, T1, "SSE-NOTICE-2011"),
        // 2011-10-07 - T1 - SSE-NOTICE-2011 - 国庆节: the range's own last day.
        (2011, 10, 7, Closed, T1, "SSE-NOTICE-2011"),
        // 2012-01-02 - T1 - SSE-NOTICE-2012 - 元旦: "元旦：1月1日（星期日）至1月3日（星期二）休市"; weekday legs.
        (2012, 1, 2, Closed, T1, "SSE-NOTICE-2012"),
        // 2012-01-03 - T1 - SSE-NOTICE-2012 - 元旦: the range's own last day.
        (2012, 1, 3, Closed, T1, "SSE-NOTICE-2012"),
        // 2012-01-23 - T1 - SSE-NOTICE-2012 - 春节: "春节：1月22日（星期日）至1月28日（星期六）休市"; weekday legs.
        (2012, 1, 23, Closed, T1, "SSE-NOTICE-2012"),
        // 2012-01-24 - T1 - SSE-NOTICE-2012 - 春节 (weekday leg).
        (2012, 1, 24, Closed, T1, "SSE-NOTICE-2012"),
        // 2012-01-25 - T1 - SSE-NOTICE-2012 - 春节 (weekday leg).
        (2012, 1, 25, Closed, T1, "SSE-NOTICE-2012"),
        // 2012-01-26 - T1 - SSE-NOTICE-2012 - 春节 (weekday leg).
        (2012, 1, 26, Closed, T1, "SSE-NOTICE-2012"),
        // 2012-01-27 - T1 - SSE-NOTICE-2012 - 春节 (weekday leg).
        (2012, 1, 27, Closed, T1, "SSE-NOTICE-2012"),
        // 2012-04-02 - T1 - SSE-NOTICE-2012 - 清明节: "清明节：4月2日（星期一）至4月4日（星期三）休市"; weekday legs.
        (2012, 4, 2, Closed, T1, "SSE-NOTICE-2012"),
        // 2012-04-03 - T1 - SSE-NOTICE-2012 - 清明节 (weekday leg).
        (2012, 4, 3, Closed, T1, "SSE-NOTICE-2012"),
        // 2012-04-04 - T1 - SSE-NOTICE-2012 - 清明节: the range's own last day.
        (2012, 4, 4, Closed, T1, "SSE-NOTICE-2012"),
        // 2012-04-30 - T1 - SSE-NOTICE-2012 - 劳动节: "劳动节：4月29日（星期日）至5月1日（星期二）休市"; weekday legs.
        (2012, 4, 30, Closed, T1, "SSE-NOTICE-2012"),
        // 2012-05-01 - T1 - SSE-NOTICE-2012 - 劳动节: the range's own last day.
        (2012, 5, 1, Closed, T1, "SSE-NOTICE-2012"),
        // 2012-06-22 - T1 - SSE-NOTICE-2012 - 端午节: "端午节：6月22日（星期五）至6月24日（星期日）休市"; the range's only weekday.
        (2012, 6, 22, Closed, T1, "SSE-NOTICE-2012"),
        // 2012-10-01 - T1 - SSE-NOTICE-2012 - 中秋节、国庆节: "中秋节、国庆节：9月30日（星期日）至10月7日（星期日）休市"; weekday legs.
        (2012, 10, 1, Closed, T1, "SSE-NOTICE-2012"),
        // 2012-10-02 - T1 - SSE-NOTICE-2012 - 中秋节、国庆节 (weekday leg).
        (2012, 10, 2, Closed, T1, "SSE-NOTICE-2012"),
        // 2012-10-03 - T1 - SSE-NOTICE-2012 - 中秋节、国庆节 (weekday leg).
        (2012, 10, 3, Closed, T1, "SSE-NOTICE-2012"),
        // 2012-10-04 - T1 - SSE-NOTICE-2012 - 中秋节、国庆节 (weekday leg).
        (2012, 10, 4, Closed, T1, "SSE-NOTICE-2012"),
        // 2012-10-05 - T1 - SSE-NOTICE-2012 - 中秋节、国庆节 (weekday leg).
        (2012, 10, 5, Closed, T1, "SSE-NOTICE-2012"),
        // 2013-01-01 - T1 - SSE-NOTICE-2013 - 元旦: "元旦：1月1日（星期二）至1月3日（星期四）休市"; weekday legs.
        (2013, 1, 1, Closed, T1, "SSE-NOTICE-2013"),
        // 2013-01-02 - T1 - SSE-NOTICE-2013 - 元旦 (weekday leg).
        (2013, 1, 2, Closed, T1, "SSE-NOTICE-2013"),
        // 2013-01-03 - T1 - SSE-NOTICE-2013 - 元旦: the range's own last day.
        (2013, 1, 3, Closed, T1, "SSE-NOTICE-2013"),
        // 2013-02-11 - T1 - SSE-NOTICE-2013 - 春节: "春节：2月9日（星期六）至2月15日（星期五）休市"; weekday legs.
        (2013, 2, 11, Closed, T1, "SSE-NOTICE-2013"),
        // 2013-02-12 - T1 - SSE-NOTICE-2013 - 春节 (weekday leg).
        (2013, 2, 12, Closed, T1, "SSE-NOTICE-2013"),
        // 2013-02-13 - T1 - SSE-NOTICE-2013 - 春节 (weekday leg).
        (2013, 2, 13, Closed, T1, "SSE-NOTICE-2013"),
        // 2013-02-14 - T1 - SSE-NOTICE-2013 - 春节 (weekday leg).
        (2013, 2, 14, Closed, T1, "SSE-NOTICE-2013"),
        // 2013-02-15 - T1 - SSE-NOTICE-2013 - 春节: the range's own last day.
        (2013, 2, 15, Closed, T1, "SSE-NOTICE-2013"),
        // 2013-04-04 - T1 - SSE-NOTICE-2013 - 清明节: "清明节：4月4日（星期四）至4月6日（星期六）休市"; weekday legs.
        (2013, 4, 4, Closed, T1, "SSE-NOTICE-2013"),
        // 2013-04-05 - T1 - SSE-NOTICE-2013 - 清明节 (weekday leg).
        (2013, 4, 5, Closed, T1, "SSE-NOTICE-2013"),
        // 2013-04-29 - T1 - SSE-NOTICE-2013 - 劳动节: "劳动节：4月29日（星期一）至5月1日（星期三）休市"; weekday legs.
        (2013, 4, 29, Closed, T1, "SSE-NOTICE-2013"),
        // 2013-04-30 - T1 - SSE-NOTICE-2013 - 劳动节 (weekday leg).
        (2013, 4, 30, Closed, T1, "SSE-NOTICE-2013"),
        // 2013-05-01 - T1 - SSE-NOTICE-2013 - 劳动节: the range's own last day.
        (2013, 5, 1, Closed, T1, "SSE-NOTICE-2013"),
        // 2013-06-10 - T1 - SSE-NOTICE-2013 - 端午节: "端午节：6月10日（星期一）至6月12日（星期三）休市"; weekday legs.
        (2013, 6, 10, Closed, T1, "SSE-NOTICE-2013"),
        // 2013-06-11 - T1 - SSE-NOTICE-2013 - 端午节 (weekday leg).
        (2013, 6, 11, Closed, T1, "SSE-NOTICE-2013"),
        // 2013-06-12 - T1 - SSE-NOTICE-2013 - 端午节: the range's own last day.
        (2013, 6, 12, Closed, T1, "SSE-NOTICE-2013"),
        // 2013-09-19 - T1 - SSE-NOTICE-2013 - 中秋节: "中秋节：9月19日（星期四）至9月21日（星期六）休市"; weekday legs.
        (2013, 9, 19, Closed, T1, "SSE-NOTICE-2013"),
        // 2013-09-20 - T1 - SSE-NOTICE-2013 - 中秋节 (weekday leg).
        (2013, 9, 20, Closed, T1, "SSE-NOTICE-2013"),
        // 2013-10-01 - T1 - SSE-NOTICE-2013 - 国庆节: "国庆节：10月1日（星期二）至10月7日（星期一）休市"; weekday legs.
        (2013, 10, 1, Closed, T1, "SSE-NOTICE-2013"),
        // 2013-10-02 - T1 - SSE-NOTICE-2013 - 国庆节 (weekday leg).
        (2013, 10, 2, Closed, T1, "SSE-NOTICE-2013"),
        // 2013-10-03 - T1 - SSE-NOTICE-2013 - 国庆节 (weekday leg).
        (2013, 10, 3, Closed, T1, "SSE-NOTICE-2013"),
        // 2013-10-04 - T1 - SSE-NOTICE-2013 - 国庆节 (weekday leg).
        (2013, 10, 4, Closed, T1, "SSE-NOTICE-2013"),
        // 2013-10-07 - T1 - SSE-NOTICE-2013 - 国庆节: the range's own last day.
        (2013, 10, 7, Closed, T1, "SSE-NOTICE-2013"),
        // 2014-01-01 - T1 - SSE-NOTICE-2014 - 元旦: "元旦：1月1日（星期三）休市"; the range's only weekday.
        (2014, 1, 1, Closed, T1, "SSE-NOTICE-2014"),
        // 2014-01-31 - T1 - SSE-NOTICE-2014 - 春节: "春节：1月31日（星期五）至2月6日（星期四）休市"; weekday legs.
        (2014, 1, 31, Closed, T1, "SSE-NOTICE-2014"),
        // 2014-02-03 - T1 - SSE-NOTICE-2014 - 春节 (weekday leg).
        (2014, 2, 3, Closed, T1, "SSE-NOTICE-2014"),
        // 2014-02-04 - T1 - SSE-NOTICE-2014 - 春节 (weekday leg).
        (2014, 2, 4, Closed, T1, "SSE-NOTICE-2014"),
        // 2014-02-05 - T1 - SSE-NOTICE-2014 - 春节 (weekday leg).
        (2014, 2, 5, Closed, T1, "SSE-NOTICE-2014"),
        // 2014-02-06 - T1 - SSE-NOTICE-2014 - 春节: the range's own last day.
        (2014, 2, 6, Closed, T1, "SSE-NOTICE-2014"),
        // 2014-04-07 - T1 - SSE-NOTICE-2014 - 清明节: "清明节：4月7日（星期一）休市"; the range's only weekday.
        (2014, 4, 7, Closed, T1, "SSE-NOTICE-2014"),
        // 2014-05-01 - T1 - SSE-NOTICE-2014 - 劳动节: "劳动节：5月1日（星期四）至5月3日（星期六）休市"; weekday legs.
        (2014, 5, 1, Closed, T1, "SSE-NOTICE-2014"),
        // 2014-05-02 - T1 - SSE-NOTICE-2014 - 劳动节 (weekday leg).
        (2014, 5, 2, Closed, T1, "SSE-NOTICE-2014"),
        // 2014-06-02 - T1 - SSE-NOTICE-2014 - 端午节: "端午节：6月2日（星期一）休市"; the range's only weekday.
        (2014, 6, 2, Closed, T1, "SSE-NOTICE-2014"),
        // 2014-09-08 - T1 - SSE-NOTICE-2014 - 中秋节: "中秋节：9月8日（星期一）休市"; the range's only weekday.
        (2014, 9, 8, Closed, T1, "SSE-NOTICE-2014"),
        // 2014-10-01 - T1 - SSE-NOTICE-2014 - 国庆节: "国庆节：10月1日（星期三）至10月7日（星期二）休市"; weekday legs.
        (2014, 10, 1, Closed, T1, "SSE-NOTICE-2014"),
        // 2014-10-02 - T1 - SSE-NOTICE-2014 - 国庆节 (weekday leg).
        (2014, 10, 2, Closed, T1, "SSE-NOTICE-2014"),
        // 2014-10-03 - T1 - SSE-NOTICE-2014 - 国庆节 (weekday leg).
        (2014, 10, 3, Closed, T1, "SSE-NOTICE-2014"),
        // 2014-10-06 - T1 - SSE-NOTICE-2014 - 国庆节 (weekday leg).
        (2014, 10, 6, Closed, T1, "SSE-NOTICE-2014"),
        // 2014-10-07 - T1 - SSE-NOTICE-2014 - 国庆节: the range's own last day.
        (2014, 10, 7, Closed, T1, "SSE-NOTICE-2014"),
        // 2015-01-01 - T1 - SSE-NOTICE-2014-15 - 元旦: "元旦：1月1日（星期四）至1月3日（星期六）休市"; weekday legs.
        (2015, 1, 1, Closed, T1, "SSE-NOTICE-2014-15"),
        // 2015-01-02 - T1 - SSE-NOTICE-2014-15 - 元旦 (weekday leg).
        (2015, 1, 2, Closed, T1, "SSE-NOTICE-2014-15"),
        // 2015-02-18 - T1 - SSE-NOTICE-2014-15 - 春节: "春节：2月18日（星期三）至2月24日（星期二）休市"; weekday legs.
        (2015, 2, 18, Closed, T1, "SSE-NOTICE-2014-15"),
        // 2015-02-19 - T1 - SSE-NOTICE-2014-15 - 春节 (weekday leg).
        (2015, 2, 19, Closed, T1, "SSE-NOTICE-2014-15"),
        // 2015-02-20 - T1 - SSE-NOTICE-2014-15 - 春节 (weekday leg).
        (2015, 2, 20, Closed, T1, "SSE-NOTICE-2014-15"),
        // 2015-02-23 - T1 - SSE-NOTICE-2014-15 - 春节 (weekday leg).
        (2015, 2, 23, Closed, T1, "SSE-NOTICE-2014-15"),
        // 2015-02-24 - T1 - SSE-NOTICE-2014-15 - 春节: the range's own last day.
        (2015, 2, 24, Closed, T1, "SSE-NOTICE-2014-15"),
        // 2015-04-06 - T1 - SSE-NOTICE-2014-15 - 清明节: "清明节：4月5日（星期日）至4月6日（星期一）休市"; the range's only weekday.
        (2015, 4, 6, Closed, T1, "SSE-NOTICE-2014-15"),
        // 2015-05-01 - T1 - SSE-NOTICE-2014-15 - 劳动节: "劳动节：5月1日（星期五）至5月3日（星期日）休市"; the range's only weekday.
        (2015, 5, 1, Closed, T1, "SSE-NOTICE-2014-15"),
        // 2015-06-22 - T1 - SSE-NOTICE-2014-15 - 端午节: "端午节：6月20日（星期六）至6月22日（星期一）休市"; the range's only weekday.
        (2015, 6, 22, Closed, T1, "SSE-NOTICE-2014-15"),
        // 2015-10-01 - T1 - SSE-NOTICE-2014-15 - 国庆节: "国庆节：10月1日（星期四）至10月7日（星期三）休市"; weekday legs.
        (2015, 10, 1, Closed, T1, "SSE-NOTICE-2014-15"),
        // 2015-10-02 - T1 - SSE-NOTICE-2014-15 - 国庆节 (weekday leg).
        (2015, 10, 2, Closed, T1, "SSE-NOTICE-2014-15"),
        // 2015-10-05 - T1 - SSE-NOTICE-2014-15 - 国庆节 (weekday leg).
        (2015, 10, 5, Closed, T1, "SSE-NOTICE-2014-15"),
        // 2015-10-06 - T1 - SSE-NOTICE-2014-15 - 国庆节 (weekday leg).
        (2015, 10, 6, Closed, T1, "SSE-NOTICE-2014-15"),
        // 2015-10-07 - T1 - SSE-NOTICE-2014-15 - 国庆节: the range's own last day.
        (2015, 10, 7, Closed, T1, "SSE-NOTICE-2014-15"),
        // 2016-01-01 - T1 - SSE-NOTICE-2015-36 - 元旦: "元旦：1月1日（星期五）至1月3日（星期日）休市"; the range's only weekday.
        (2016, 1, 1, Closed, T1, "SSE-NOTICE-2015-36"),
        // 2016-02-08 - T1 - SSE-NOTICE-2015-36 - 春节: "春节：2月7日（星期日）至2月13日（星期六）休市"; weekday legs.
        (2016, 2, 8, Closed, T1, "SSE-NOTICE-2015-36"),
        // 2016-02-09 - T1 - SSE-NOTICE-2015-36 - 春节 (weekday leg).
        (2016, 2, 9, Closed, T1, "SSE-NOTICE-2015-36"),
        // 2016-02-10 - T1 - SSE-NOTICE-2015-36 - 春节 (weekday leg).
        (2016, 2, 10, Closed, T1, "SSE-NOTICE-2015-36"),
        // 2016-02-11 - T1 - SSE-NOTICE-2015-36 - 春节 (weekday leg).
        (2016, 2, 11, Closed, T1, "SSE-NOTICE-2015-36"),
        // 2016-02-12 - T1 - SSE-NOTICE-2015-36 - 春节 (weekday leg).
        (2016, 2, 12, Closed, T1, "SSE-NOTICE-2015-36"),
        // 2016-04-04 - T1 - SSE-NOTICE-2015-36 - 清明节: "清明节：4月2日（星期六）至4月4日（星期一）休市"; the range's only weekday.
        (2016, 4, 4, Closed, T1, "SSE-NOTICE-2015-36"),
        // 2016-05-02 - T1 - SSE-NOTICE-2015-36 - 劳动节: "劳动节：4月30日（星期六）至5月2日（星期一）休市"; the range's only weekday.
        (2016, 5, 2, Closed, T1, "SSE-NOTICE-2015-36"),
        // 2016-06-09 - T1 - SSE-NOTICE-2015-36 - 端午节: "端午节：6月9日（星期四）至6月11日（星期六）休市"; weekday legs.
        (2016, 6, 9, Closed, T1, "SSE-NOTICE-2015-36"),
        // 2016-06-10 - T1 - SSE-NOTICE-2015-36 - 端午节 (weekday leg).
        (2016, 6, 10, Closed, T1, "SSE-NOTICE-2015-36"),
        // 2016-09-15 - T1 - SSE-NOTICE-2015-36 - 中秋节: "中秋节：9月15日（星期四）至9月17日（星期六）休市"; weekday legs.
        (2016, 9, 15, Closed, T1, "SSE-NOTICE-2015-36"),
        // 2016-09-16 - T1 - SSE-NOTICE-2015-36 - 中秋节 (weekday leg).
        (2016, 9, 16, Closed, T1, "SSE-NOTICE-2015-36"),
        // 2016-10-03 - T1 - SSE-NOTICE-2015-36 - 国庆节: "国庆节：10月1日（星期六）至10月7日（星期五）休市"; weekday legs.
        (2016, 10, 3, Closed, T1, "SSE-NOTICE-2015-36"),
        // 2016-10-04 - T1 - SSE-NOTICE-2015-36 - 国庆节 (weekday leg).
        (2016, 10, 4, Closed, T1, "SSE-NOTICE-2015-36"),
        // 2016-10-05 - T1 - SSE-NOTICE-2015-36 - 国庆节 (weekday leg).
        (2016, 10, 5, Closed, T1, "SSE-NOTICE-2015-36"),
        // 2016-10-06 - T1 - SSE-NOTICE-2015-36 - 国庆节 (weekday leg).
        (2016, 10, 6, Closed, T1, "SSE-NOTICE-2015-36"),
        // 2016-10-07 - T1 - SSE-NOTICE-2015-36 - 国庆节: the range's own last day.
        (2016, 10, 7, Closed, T1, "SSE-NOTICE-2015-36"),
        // 2017-01-02 - T1 - SSE-NOTICE-2016-25 - 元旦: "元旦：1月1日（星期日）至1月2日（星期一）休市"; the range's only weekday.
        (2017, 1, 2, Closed, T1, "SSE-NOTICE-2016-25"),
        // 2017-01-27 - T1 - SSE-NOTICE-2016-25 - 春节: "春节：1月27日（星期五）至2月2日（星期四）休市"; weekday legs.
        (2017, 1, 27, Closed, T1, "SSE-NOTICE-2016-25"),
        // 2017-01-30 - T1 - SSE-NOTICE-2016-25 - 春节 (weekday leg).
        (2017, 1, 30, Closed, T1, "SSE-NOTICE-2016-25"),
        // 2017-01-31 - T1 - SSE-NOTICE-2016-25 - 春节 (weekday leg).
        (2017, 1, 31, Closed, T1, "SSE-NOTICE-2016-25"),
        // 2017-02-01 - T1 - SSE-NOTICE-2016-25 - 春节 (weekday leg).
        (2017, 2, 1, Closed, T1, "SSE-NOTICE-2016-25"),
        // 2017-02-02 - T1 - SSE-NOTICE-2016-25 - 春节: the range's own last day.
        (2017, 2, 2, Closed, T1, "SSE-NOTICE-2016-25"),
        // 2017-04-03 - T1 - SSE-NOTICE-2016-25 - 清明节: "清明节：4月2日（星期日）至4月4日（星期二）休市"; weekday legs.
        (2017, 4, 3, Closed, T1, "SSE-NOTICE-2016-25"),
        // 2017-04-04 - T1 - SSE-NOTICE-2016-25 - 清明节: the range's own last day.
        (2017, 4, 4, Closed, T1, "SSE-NOTICE-2016-25"),
        // 2017-05-01 - T1 - SSE-NOTICE-2016-25 - 劳动节: "劳动节：4月29日（星期六）至5月1日（星期一）休市"; the range's only weekday.
        (2017, 5, 1, Closed, T1, "SSE-NOTICE-2016-25"),
        // 2017-05-29 - T1 - SSE-NOTICE-2016-25 - 端午节: "端午节：5月28日（星期日）至5月30日（星期二）休市"; weekday legs.
        (2017, 5, 29, Closed, T1, "SSE-NOTICE-2016-25"),
        // 2017-05-30 - T1 - SSE-NOTICE-2016-25 - 端午节: the range's own last day.
        (2017, 5, 30, Closed, T1, "SSE-NOTICE-2016-25"),
        // 2017-10-02 - T1 - SSE-NOTICE-2016-25 - 中秋节、国庆节: "中秋节、国庆节：10月1日（星期日）至10月8日（星期日）休市"; weekday legs.
        (2017, 10, 2, Closed, T1, "SSE-NOTICE-2016-25"),
        // 2017-10-03 - T1 - SSE-NOTICE-2016-25 - 中秋节、国庆节 (weekday leg).
        (2017, 10, 3, Closed, T1, "SSE-NOTICE-2016-25"),
        // 2017-10-04 - T1 - SSE-NOTICE-2016-25 - 中秋节、国庆节 (weekday leg).
        (2017, 10, 4, Closed, T1, "SSE-NOTICE-2016-25"),
        // 2017-10-05 - T1 - SSE-NOTICE-2016-25 - 中秋节、国庆节 (weekday leg).
        (2017, 10, 5, Closed, T1, "SSE-NOTICE-2016-25"),
        // 2017-10-06 - T1 - SSE-NOTICE-2016-25 - 中秋节、国庆节 (weekday leg).
        (2017, 10, 6, Closed, T1, "SSE-NOTICE-2016-25"),
        // 2018-01-01 - T1 - SSE-NOTICE-2017-26 - 元旦: "元旦：1月1日（星期一）休市"; the range's only weekday.
        (2018, 1, 1, Closed, T1, "SSE-NOTICE-2017-26"),
        // 2018-02-15 - T1 - SSE-NOTICE-2017-26 - 春节: "春节：2月15日（星期四）至2月21日（星期三）休市"; weekday legs.
        (2018, 2, 15, Closed, T1, "SSE-NOTICE-2017-26"),
        // 2018-02-16 - T1 - SSE-NOTICE-2017-26 - 春节 (weekday leg).
        (2018, 2, 16, Closed, T1, "SSE-NOTICE-2017-26"),
        // 2018-02-19 - T1 - SSE-NOTICE-2017-26 - 春节 (weekday leg).
        (2018, 2, 19, Closed, T1, "SSE-NOTICE-2017-26"),
        // 2018-02-20 - T1 - SSE-NOTICE-2017-26 - 春节 (weekday leg).
        (2018, 2, 20, Closed, T1, "SSE-NOTICE-2017-26"),
        // 2018-02-21 - T1 - SSE-NOTICE-2017-26 - 春节: the range's own last day.
        (2018, 2, 21, Closed, T1, "SSE-NOTICE-2017-26"),
        // 2018-04-05 - T1 - SSE-NOTICE-2017-26 - 清明节: "清明节：4月5日（星期四）至4月7日（星期六）休市"; weekday legs.
        (2018, 4, 5, Closed, T1, "SSE-NOTICE-2017-26"),
        // 2018-04-06 - T1 - SSE-NOTICE-2017-26 - 清明节 (weekday leg).
        (2018, 4, 6, Closed, T1, "SSE-NOTICE-2017-26"),
        // 2018-04-30 - T1 - SSE-NOTICE-2017-26 - 劳动节: "劳动节：4月29日（星期日）至5月1日（星期二）休市"; weekday legs.
        (2018, 4, 30, Closed, T1, "SSE-NOTICE-2017-26"),
        // 2018-05-01 - T1 - SSE-NOTICE-2017-26 - 劳动节: the range's own last day.
        (2018, 5, 1, Closed, T1, "SSE-NOTICE-2017-26"),
        // 2018-06-18 - T1 - SSE-NOTICE-2017-26 - 端午节: "端午节：6月16日（星期六）至6月18日（星期一）休市"; the range's only weekday.
        (2018, 6, 18, Closed, T1, "SSE-NOTICE-2017-26"),
        // 2018-09-24 - T1 - SSE-NOTICE-2017-26 - 中秋节: "中秋节：9月22日（星期六）至9月24日（星期一）休市"; the range's only weekday.
        (2018, 9, 24, Closed, T1, "SSE-NOTICE-2017-26"),
        // 2018-10-01 - T1 - SSE-NOTICE-2017-26 - 国庆节: "国庆节：10月1日（星期一）至10月7日（星期日）休市"; weekday legs.
        (2018, 10, 1, Closed, T1, "SSE-NOTICE-2017-26"),
        // 2018-10-02 - T1 - SSE-NOTICE-2017-26 - 国庆节 (weekday leg).
        (2018, 10, 2, Closed, T1, "SSE-NOTICE-2017-26"),
        // 2018-10-03 - T1 - SSE-NOTICE-2017-26 - 国庆节 (weekday leg).
        (2018, 10, 3, Closed, T1, "SSE-NOTICE-2017-26"),
        // 2018-10-04 - T1 - SSE-NOTICE-2017-26 - 国庆节 (weekday leg).
        (2018, 10, 4, Closed, T1, "SSE-NOTICE-2017-26"),
        // 2018-10-05 - T1 - SSE-NOTICE-2017-26 - 国庆节 (weekday leg).
        (2018, 10, 5, Closed, T1, "SSE-NOTICE-2017-26"),
        // 2018-12-31 - T1 - SSE-NOTICE-2018-39 - 元旦: "元旦：2018年12月30日（星期日）至2019年1月1日（星期二）休市"; weekday legs.
        (2018, 12, 31, Closed, T1, "SSE-NOTICE-2018-39"),
        // 2019-01-01 - T1 - SSE-NOTICE-2018-39 - 元旦: the range's own last day.
        (2019, 1, 1, Closed, T1, "SSE-NOTICE-2018-39"),
        // 2019-02-04 - T1 - SSE-NOTICE-2018-39 - 春节: "春节：2月4日（星期一）至2月10日（星期日）休市"; weekday legs.
        (2019, 2, 4, Closed, T1, "SSE-NOTICE-2018-39"),
        // 2019-02-05 - T1 - SSE-NOTICE-2018-39 - 春节 (weekday leg).
        (2019, 2, 5, Closed, T1, "SSE-NOTICE-2018-39"),
        // 2019-02-06 - T1 - SSE-NOTICE-2018-39 - 春节 (weekday leg).
        (2019, 2, 6, Closed, T1, "SSE-NOTICE-2018-39"),
        // 2019-02-07 - T1 - SSE-NOTICE-2018-39 - 春节 (weekday leg).
        (2019, 2, 7, Closed, T1, "SSE-NOTICE-2018-39"),
        // 2019-02-08 - T1 - SSE-NOTICE-2018-39 - 春节 (weekday leg).
        (2019, 2, 8, Closed, T1, "SSE-NOTICE-2018-39"),
        // 2019-04-05 - T1 - SSE-NOTICE-2018-39 - 清明节: "清明节：4月5日（星期五）至4月7日（星期日）休市"; the range's only weekday.
        (2019, 4, 5, Closed, T1, "SSE-NOTICE-2018-39"),
        // 2019-05-01 - T1 - SSE-NOTICE-2018-39 - 劳动节: "劳动节：5月1日（星期三）休市"; the range's only weekday.
        (2019, 5, 1, Closed, T1, "SSE-NOTICE-2018-39"),
        // 2019-06-07 - T1 - SSE-NOTICE-2018-39 - 端午节: "端午节：6月7日（星期五）至6月9日（星期日）休市"; the range's only weekday.
        (2019, 6, 7, Closed, T1, "SSE-NOTICE-2018-39"),
        // 2019-09-13 - T1 - SSE-NOTICE-2018-39 - 中秋节: "中秋节：9月13日（星期五）至9月15日（星期日）休市"; the range's only weekday.
        (2019, 9, 13, Closed, T1, "SSE-NOTICE-2018-39"),
        // 2019-10-01 - T1 - SSE-NOTICE-2018-39 - 国庆节: "国庆节：10月1日（星期二）至10月7日（星期一）休市"; weekday legs.
        (2019, 10, 1, Closed, T1, "SSE-NOTICE-2018-39"),
        // 2019-10-02 - T1 - SSE-NOTICE-2018-39 - 国庆节 (weekday leg).
        (2019, 10, 2, Closed, T1, "SSE-NOTICE-2018-39"),
        // 2019-10-03 - T1 - SSE-NOTICE-2018-39 - 国庆节 (weekday leg).
        (2019, 10, 3, Closed, T1, "SSE-NOTICE-2018-39"),
        // 2019-10-04 - T1 - SSE-NOTICE-2018-39 - 国庆节 (weekday leg).
        (2019, 10, 4, Closed, T1, "SSE-NOTICE-2018-39"),
        // 2019-10-07 - T1 - SSE-NOTICE-2018-39 - 国庆节: the range's own last day.
        (2019, 10, 7, Closed, T1, "SSE-NOTICE-2018-39"),
        // 2020-01-01 - T1 - SSE-NOTICE-2019-65 - 元旦: "元旦：1月1日（星期三）休市"; the range's only weekday.
        (2020, 1, 1, Closed, T1, "SSE-NOTICE-2019-65"),
        // 2020-01-24 - T1 - SSE-NOTICE-2019-65 - 春节: "春节：1月24日（星期五）至1月30日（星期四）休市"; weekday legs.
        (2020, 1, 24, Closed, T1, "SSE-NOTICE-2019-65"),
        // 2020-01-27 - T1 - SSE-NOTICE-2019-65 - 春节 (weekday leg).
        (2020, 1, 27, Closed, T1, "SSE-NOTICE-2019-65"),
        // 2020-01-28 - T1 - SSE-NOTICE-2019-65 - 春节 (weekday leg).
        (2020, 1, 28, Closed, T1, "SSE-NOTICE-2019-65"),
        // 2020-01-29 - T1 - SSE-NOTICE-2019-65 - 春节 (weekday leg).
        (2020, 1, 29, Closed, T1, "SSE-NOTICE-2019-65"),
        // 2020-01-30 - T1 - SSE-NOTICE-2019-65 - 春节: the range's own last day.
        (2020, 1, 30, Closed, T1, "SSE-NOTICE-2019-65"),
        // 2020-04-06 - T1 - SSE-NOTICE-2019-65 - 清明节: "清明节：4月4日（星期六）至4月6日（星期一）休市"; the range's only weekday.
        (2020, 4, 6, Closed, T1, "SSE-NOTICE-2019-65"),
        // 2020-05-01 - T1 - SSE-NOTICE-2019-65 - 劳动节: "劳动节：5月1日（星期五）至5月5日（星期二）休市"; weekday legs.
        (2020, 5, 1, Closed, T1, "SSE-NOTICE-2019-65"),
        // 2020-05-04 - T1 - SSE-NOTICE-2019-65 - 劳动节 (weekday leg).
        (2020, 5, 4, Closed, T1, "SSE-NOTICE-2019-65"),
        // 2020-05-05 - T1 - SSE-NOTICE-2019-65 - 劳动节: the range's own last day.
        (2020, 5, 5, Closed, T1, "SSE-NOTICE-2019-65"),
        // 2020-06-25 - T1 - SSE-NOTICE-2019-65 - 端午节: "端午节：6月25日（星期四）至6月27日（星期六）休市"; weekday legs.
        (2020, 6, 25, Closed, T1, "SSE-NOTICE-2019-65"),
        // 2020-06-26 - T1 - SSE-NOTICE-2019-65 - 端午节 (weekday leg).
        (2020, 6, 26, Closed, T1, "SSE-NOTICE-2019-65"),
        // 2020-10-01 - T1 - SSE-NOTICE-2019-65 - 国庆节、中秋节: "国庆节、中秋节：10月1日（星期四）至10月8日（星期四）休市"; weekday legs.
        (2020, 10, 1, Closed, T1, "SSE-NOTICE-2019-65"),
        // 2020-10-02 - T1 - SSE-NOTICE-2019-65 - 国庆节、中秋节 (weekday leg).
        (2020, 10, 2, Closed, T1, "SSE-NOTICE-2019-65"),
        // 2020-10-05 - T1 - SSE-NOTICE-2019-65 - 国庆节、中秋节 (weekday leg).
        (2020, 10, 5, Closed, T1, "SSE-NOTICE-2019-65"),
        // 2020-10-06 - T1 - SSE-NOTICE-2019-65 - 国庆节、中秋节 (weekday leg).
        (2020, 10, 6, Closed, T1, "SSE-NOTICE-2019-65"),
        // 2020-10-07 - T1 - SSE-NOTICE-2019-65 - 国庆节、中秋节 (weekday leg).
        (2020, 10, 7, Closed, T1, "SSE-NOTICE-2019-65"),
        // 2020-10-08 - T1 - SSE-NOTICE-2019-65 - 国庆节、中秋节: the range's own last day.
        (2020, 10, 8, Closed, T1, "SSE-NOTICE-2019-65"),
        // 2021-01-01 - T1 - SSE-NOTICE-2020-48 - 元旦: "元旦：1月1日（星期五）至1月3日（星期日）休市"; the range's only weekday.
        (2021, 1, 1, Closed, T1, "SSE-NOTICE-2020-48"),
        // 2021-02-11 - T1 - SSE-NOTICE-2020-48 - 春节: "春节：2月11日（星期四）至2月17日（星期三）休市"; weekday legs.
        (2021, 2, 11, Closed, T1, "SSE-NOTICE-2020-48"),
        // 2021-02-12 - T1 - SSE-NOTICE-2020-48 - 春节 (weekday leg).
        (2021, 2, 12, Closed, T1, "SSE-NOTICE-2020-48"),
        // 2021-02-15 - T1 - SSE-NOTICE-2020-48 - 春节 (weekday leg).
        (2021, 2, 15, Closed, T1, "SSE-NOTICE-2020-48"),
        // 2021-02-16 - T1 - SSE-NOTICE-2020-48 - 春节 (weekday leg).
        (2021, 2, 16, Closed, T1, "SSE-NOTICE-2020-48"),
        // 2021-02-17 - T1 - SSE-NOTICE-2020-48 - 春节: the range's own last day.
        (2021, 2, 17, Closed, T1, "SSE-NOTICE-2020-48"),
        // 2021-04-05 - T1 - SSE-NOTICE-2020-48 - 清明节: "清明节：4月3日（星期六）至4月5日（星期一）休市"; the range's only weekday.
        (2021, 4, 5, Closed, T1, "SSE-NOTICE-2020-48"),
        // 2021-05-03 - T1 - SSE-NOTICE-2020-48 - 劳动节: "劳动节：5月1日（星期六）至5月5日（星期三）休市"; weekday legs.
        (2021, 5, 3, Closed, T1, "SSE-NOTICE-2020-48"),
        // 2021-05-04 - T1 - SSE-NOTICE-2020-48 - 劳动节 (weekday leg).
        (2021, 5, 4, Closed, T1, "SSE-NOTICE-2020-48"),
        // 2021-05-05 - T1 - SSE-NOTICE-2020-48 - 劳动节: the range's own last day.
        (2021, 5, 5, Closed, T1, "SSE-NOTICE-2020-48"),
        // 2021-06-14 - T1 - SSE-NOTICE-2020-48 - 端午节: "端午节：6月12日（星期六）至6月14日（星期一）休市"; the range's only weekday.
        (2021, 6, 14, Closed, T1, "SSE-NOTICE-2020-48"),
        // 2021-09-20 - T1 - SSE-NOTICE-2020-48 - 中秋节: "中秋节：9月19日（星期日）至9月21日（星期二）休市"; weekday legs.
        (2021, 9, 20, Closed, T1, "SSE-NOTICE-2020-48"),
        // 2021-09-21 - T1 - SSE-NOTICE-2020-48 - 中秋节: the range's own last day.
        (2021, 9, 21, Closed, T1, "SSE-NOTICE-2020-48"),
        // 2021-10-01 - T1 - SSE-NOTICE-2020-48 - 国庆节: "国庆节：10月1日（星期五）至10月7日（星期四）休市"; weekday legs.
        (2021, 10, 1, Closed, T1, "SSE-NOTICE-2020-48"),
        // 2021-10-04 - T1 - SSE-NOTICE-2020-48 - 国庆节 (weekday leg).
        (2021, 10, 4, Closed, T1, "SSE-NOTICE-2020-48"),
        // 2021-10-05 - T1 - SSE-NOTICE-2020-48 - 国庆节 (weekday leg).
        (2021, 10, 5, Closed, T1, "SSE-NOTICE-2020-48"),
        // 2021-10-06 - T1 - SSE-NOTICE-2020-48 - 国庆节 (weekday leg).
        (2021, 10, 6, Closed, T1, "SSE-NOTICE-2020-48"),
        // 2021-10-07 - T1 - SSE-NOTICE-2020-48 - 国庆节: the range's own last day.
        (2021, 10, 7, Closed, T1, "SSE-NOTICE-2020-48"),
        // 2022-01-03 - T1 - SSE-NOTICE-2021-37 - 元旦: "元旦：1月1日（星期六）至1月3日（星期一）休市"; the range's only weekday.
        (2022, 1, 3, Closed, T1, "SSE-NOTICE-2021-37"),
        // 2022-01-31 - T1 - SSE-NOTICE-2021-37 - 春节: "春节：1月31日（星期一）至2月6日（星期日）休市"; weekday legs.
        (2022, 1, 31, Closed, T1, "SSE-NOTICE-2021-37"),
        // 2022-02-01 - T1 - SSE-NOTICE-2021-37 - 春节 (weekday leg).
        (2022, 2, 1, Closed, T1, "SSE-NOTICE-2021-37"),
        // 2022-02-02 - T1 - SSE-NOTICE-2021-37 - 春节 (weekday leg).
        (2022, 2, 2, Closed, T1, "SSE-NOTICE-2021-37"),
        // 2022-02-03 - T1 - SSE-NOTICE-2021-37 - 春节 (weekday leg).
        (2022, 2, 3, Closed, T1, "SSE-NOTICE-2021-37"),
        // 2022-02-04 - T1 - SSE-NOTICE-2021-37 - 春节 (weekday leg).
        (2022, 2, 4, Closed, T1, "SSE-NOTICE-2021-37"),
        // 2022-04-04 - T1 - SSE-NOTICE-2021-37 - 清明节: "清明节：4月3日（星期日）至4月5日（星期二）休市"; weekday legs.
        (2022, 4, 4, Closed, T1, "SSE-NOTICE-2021-37"),
        // 2022-04-05 - T1 - SSE-NOTICE-2021-37 - 清明节: the range's own last day.
        (2022, 4, 5, Closed, T1, "SSE-NOTICE-2021-37"),
        // 2022-05-02 - T1 - SSE-NOTICE-2021-37 - 劳动节: "劳动节：4月30日（星期六）至5月4日（星期三）休市"; weekday legs.
        (2022, 5, 2, Closed, T1, "SSE-NOTICE-2021-37"),
        // 2022-05-03 - T1 - SSE-NOTICE-2021-37 - 劳动节 (weekday leg).
        (2022, 5, 3, Closed, T1, "SSE-NOTICE-2021-37"),
        // 2022-05-04 - T1 - SSE-NOTICE-2021-37 - 劳动节: the range's own last day.
        (2022, 5, 4, Closed, T1, "SSE-NOTICE-2021-37"),
        // 2022-06-03 - T1 - SSE-NOTICE-2021-37 - 端午节: "端午节：6月3日（星期五）至6月5日（星期日）休市"; the range's only weekday.
        (2022, 6, 3, Closed, T1, "SSE-NOTICE-2021-37"),
        // 2022-09-12 - T1 - SSE-NOTICE-2021-37 - 中秋节: "中秋节：9月10日（星期六）至9月12日（星期一）休市"; the range's only weekday.
        (2022, 9, 12, Closed, T1, "SSE-NOTICE-2021-37"),
        // 2022-10-03 - T1 - SSE-NOTICE-2021-37 - 国庆节: "国庆节：10月1日（星期六）至10月7日（星期五）休市"; weekday legs.
        (2022, 10, 3, Closed, T1, "SSE-NOTICE-2021-37"),
        // 2022-10-04 - T1 - SSE-NOTICE-2021-37 - 国庆节 (weekday leg).
        (2022, 10, 4, Closed, T1, "SSE-NOTICE-2021-37"),
        // 2022-10-05 - T1 - SSE-NOTICE-2021-37 - 国庆节 (weekday leg).
        (2022, 10, 5, Closed, T1, "SSE-NOTICE-2021-37"),
        // 2022-10-06 - T1 - SSE-NOTICE-2021-37 - 国庆节 (weekday leg).
        (2022, 10, 6, Closed, T1, "SSE-NOTICE-2021-37"),
        // 2022-10-07 - T1 - SSE-NOTICE-2021-37 - 国庆节: the range's own last day.
        (2022, 10, 7, Closed, T1, "SSE-NOTICE-2021-37"),
        // 2023-01-02 - T1 - SSE-NOTICE-2022-51 - 元旦: "元旦：2022年12月31日（星期六）至2023年1月2日（星期一）休市"; the range's only weekday.
        (2023, 1, 2, Closed, T1, "SSE-NOTICE-2022-51"),
        // 2023-01-23 - T1 - SSE-NOTICE-2022-51 - 春节: "春节：1月21日（星期六）至1月27日（星期五）休市"; weekday legs.
        (2023, 1, 23, Closed, T1, "SSE-NOTICE-2022-51"),
        // 2023-01-24 - T1 - SSE-NOTICE-2022-51 - 春节 (weekday leg).
        (2023, 1, 24, Closed, T1, "SSE-NOTICE-2022-51"),
        // 2023-01-25 - T1 - SSE-NOTICE-2022-51 - 春节 (weekday leg).
        (2023, 1, 25, Closed, T1, "SSE-NOTICE-2022-51"),
        // 2023-01-26 - T1 - SSE-NOTICE-2022-51 - 春节 (weekday leg).
        (2023, 1, 26, Closed, T1, "SSE-NOTICE-2022-51"),
        // 2023-01-27 - T1 - SSE-NOTICE-2022-51 - 春节: the range's own last day.
        (2023, 1, 27, Closed, T1, "SSE-NOTICE-2022-51"),
        // 2023-04-05 - T1 - SSE-NOTICE-2022-51 - 清明节: "清明节：4月5日（星期三）休市"; the range's only weekday.
        (2023, 4, 5, Closed, T1, "SSE-NOTICE-2022-51"),
        // 2023-05-01 - T1 - SSE-NOTICE-2022-51 - 劳动节: "劳动节：4月29日（星期六）至5月3日（星期三）休市"; weekday legs.
        (2023, 5, 1, Closed, T1, "SSE-NOTICE-2022-51"),
        // 2023-05-02 - T1 - SSE-NOTICE-2022-51 - 劳动节 (weekday leg).
        (2023, 5, 2, Closed, T1, "SSE-NOTICE-2022-51"),
        // 2023-05-03 - T1 - SSE-NOTICE-2022-51 - 劳动节: the range's own last day.
        (2023, 5, 3, Closed, T1, "SSE-NOTICE-2022-51"),
        // 2023-06-22 - T1 - SSE-NOTICE-2022-51 - 端午节: "端午节：6月22日（星期四）至6月24日（星期六）休市"; weekday legs.
        (2023, 6, 22, Closed, T1, "SSE-NOTICE-2022-51"),
        // 2023-06-23 - T1 - SSE-NOTICE-2022-51 - 端午节 (weekday leg).
        (2023, 6, 23, Closed, T1, "SSE-NOTICE-2022-51"),
        // 2023-09-29 - T1 - SSE-NOTICE-2022-51 - 中秋节、国庆节: "中秋节、国庆节：9月29日（星期五）至10月6日（星期五）休市"; weekday legs.
        (2023, 9, 29, Closed, T1, "SSE-NOTICE-2022-51"),
        // 2023-10-02 - T1 - SSE-NOTICE-2022-51 - 中秋节、国庆节 (weekday leg).
        (2023, 10, 2, Closed, T1, "SSE-NOTICE-2022-51"),
        // 2023-10-03 - T1 - SSE-NOTICE-2022-51 - 中秋节、国庆节 (weekday leg).
        (2023, 10, 3, Closed, T1, "SSE-NOTICE-2022-51"),
        // 2023-10-04 - T1 - SSE-NOTICE-2022-51 - 中秋节、国庆节 (weekday leg).
        (2023, 10, 4, Closed, T1, "SSE-NOTICE-2022-51"),
        // 2023-10-05 - T1 - SSE-NOTICE-2022-51 - 中秋节、国庆节 (weekday leg).
        (2023, 10, 5, Closed, T1, "SSE-NOTICE-2022-51"),
        // 2023-10-06 - T1 - SSE-NOTICE-2022-51 - 中秋节、国庆节: the range's own last day.
        (2023, 10, 6, Closed, T1, "SSE-NOTICE-2022-51"),
        // 2024-01-01 - T1 - SSE-NOTICE-2023-47 - 元旦: "元旦：2023年12月30日（星期六）至2024年1月1日（星期一）休市"; the range's only weekday.
        (2024, 1, 1, Closed, T1, "SSE-NOTICE-2023-47"),
        // 2024-02-09 - T1 - SSE-NOTICE-2023-47 - 春节: "春节：2月9日（星期五）至2月17日（星期六）休市"; weekday legs.
        (2024, 2, 9, Closed, T1, "SSE-NOTICE-2023-47"),
        // 2024-02-12 - T1 - SSE-NOTICE-2023-47 - 春节 (weekday leg).
        (2024, 2, 12, Closed, T1, "SSE-NOTICE-2023-47"),
        // 2024-02-13 - T1 - SSE-NOTICE-2023-47 - 春节 (weekday leg).
        (2024, 2, 13, Closed, T1, "SSE-NOTICE-2023-47"),
        // 2024-02-14 - T1 - SSE-NOTICE-2023-47 - 春节 (weekday leg).
        (2024, 2, 14, Closed, T1, "SSE-NOTICE-2023-47"),
        // 2024-02-15 - T1 - SSE-NOTICE-2023-47 - 春节 (weekday leg).
        (2024, 2, 15, Closed, T1, "SSE-NOTICE-2023-47"),
        // 2024-02-16 - T1 - SSE-NOTICE-2023-47 - 春节 (weekday leg).
        (2024, 2, 16, Closed, T1, "SSE-NOTICE-2023-47"),
        // 2024-04-04 - T1 - SSE-NOTICE-2023-47 - 清明节: "清明节：4月4日（星期四）至4月6日（星期六）休市"; weekday legs.
        (2024, 4, 4, Closed, T1, "SSE-NOTICE-2023-47"),
        // 2024-04-05 - T1 - SSE-NOTICE-2023-47 - 清明节 (weekday leg).
        (2024, 4, 5, Closed, T1, "SSE-NOTICE-2023-47"),
        // 2024-05-01 - T1 - SSE-NOTICE-2023-47 - 劳动节: "劳动节：5月1日（星期三）至5月5日（星期日）休市"; weekday legs.
        (2024, 5, 1, Closed, T1, "SSE-NOTICE-2023-47"),
        // 2024-05-02 - T1 - SSE-NOTICE-2023-47 - 劳动节 (weekday leg).
        (2024, 5, 2, Closed, T1, "SSE-NOTICE-2023-47"),
        // 2024-05-03 - T1 - SSE-NOTICE-2023-47 - 劳动节 (weekday leg).
        (2024, 5, 3, Closed, T1, "SSE-NOTICE-2023-47"),
        // 2024-06-10 - T1 - SSE-NOTICE-2023-47 - 端午节: "端午节：6月10日（星期一）休市"; the range's only weekday.
        (2024, 6, 10, Closed, T1, "SSE-NOTICE-2023-47"),
        // 2024-09-16 - T1 - SSE-NOTICE-2023-47 - 中秋节: "中秋节：9月15日（星期日）至9月17日（星期二）休市"; weekday legs.
        (2024, 9, 16, Closed, T1, "SSE-NOTICE-2023-47"),
        // 2024-09-17 - T1 - SSE-NOTICE-2023-47 - 中秋节: the range's own last day.
        (2024, 9, 17, Closed, T1, "SSE-NOTICE-2023-47"),
        // 2024-10-01 - T1 - SSE-NOTICE-2023-47 - 国庆节: "国庆节：10月1日（星期二）至10月7日（星期一）休市"; weekday legs.
        (2024, 10, 1, Closed, T1, "SSE-NOTICE-2023-47"),
        // 2024-10-02 - T1 - SSE-NOTICE-2023-47 - 国庆节 (weekday leg).
        (2024, 10, 2, Closed, T1, "SSE-NOTICE-2023-47"),
        // 2024-10-03 - T1 - SSE-NOTICE-2023-47 - 国庆节 (weekday leg).
        (2024, 10, 3, Closed, T1, "SSE-NOTICE-2023-47"),
        // 2024-10-04 - T1 - SSE-NOTICE-2023-47 - 国庆节 (weekday leg).
        (2024, 10, 4, Closed, T1, "SSE-NOTICE-2023-47"),
        // 2024-10-07 - T1 - SSE-NOTICE-2023-47 - 国庆节: the range's own last day.
        (2024, 10, 7, Closed, T1, "SSE-NOTICE-2023-47"),
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
