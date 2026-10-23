// SPDX-License-Identifier: MIT-0

//! The `holidays!` table builder, whose constant evaluation fails the build on
//! any row or window invariant violation.

/// Builds a `&'static HolidayTable` whose invariants hold by construction.
///
/// The invocation is
///
/// ```text
/// holidays! {
///     coverage: [(2025, 1, 1) ..= (2027, 12, 31)],
///     rows: [
///         (2025, 1, 20, early_close(12 * 3_600), T2, "CME-SVC-2025-01-20"),
///         (2025, 12, 25, HolidayKind::Closed, T2, "CME-HOL-2025-CHRISTMAS"),
///     ],
/// }
/// ```
///
/// where each row is `(year, month, day, kind, tier, document id)` and
/// `coverage` lists one or more inclusive trade-date windows. **A table ships
/// one window per era it was audited over.** The built-in holiday tables are
/// built one operator-era wave at a time, and each wave's documents audit that
/// wave's years alone; a decade between two waves is audited by neither, so a
/// single span across it would make every date in the gap read as audited
/// normal. Listing the eras separately keeps that claim true, and
/// [`HolidayCoverage::contains`](crate::calendar::schedules::holidays::HolidayCoverage::contains)
/// answers per date.
///
/// Constant evaluation fails the build unless:
///
/// 1. every window endpoint is a real calendar date — each window is built
///    through `HolidayWindow::new`, which resolves both endpoints the same
///    way a row's trade date is resolved;
/// 2. the windows are non-empty, ascending and non-overlapping;
/// 3. trade dates are strictly ascending, so `HolidayTable::may_affect`'s
///    partition-point search and `HolidayTable::holiday_on`'s binary search
///    see a total order and no row is shadowed by a duplicate;
/// 4. every row lies inside one of the coverage windows, the rule
///    [`StaticSessionExceptions`](crate::StaticSessionExceptions) enforces for
///    a caller's records;
/// 5. every row carries a non-empty document id, so a holiday can never exist
///    without a named artifact; and
/// 6. every row's tier is
///    [`EvidenceTier::T1`](crate::calendar::schedules::holidays::EvidenceTier::T1)
///    or
///    [`EvidenceTier::T2`](crate::calendar::schedules::holidays::EvidenceTier::T2),
///    and
///    every instant is inside the `DayPolicy` ranges — a close in `0..=86_400`
///    and an open in `0..86_400`; and
/// 7. every replacement block set is non-empty, is ordered by opening day and
///    then open time, keeps each offset inside the block day range and each
///    instant inside the ranges above, and never opens on its own trade date
///    while wrapping past it — the same rules
///    [`StaticSessionExceptions`](crate::StaticSessionExceptions) applies to a
///    caller's records, so a built-in row and a caller's record cannot state
///    differently shaped topologies.
///
/// Because the windows are stored as the date pairs this construction
/// validated, the runtime coverage answers
/// ([`HolidayCoverage::windows`](crate::calendar::schedules::holidays::HolidayCoverage::windows)
/// and
/// [`HolidayCoverage::contains`](crate::calendar::schedules::holidays::HolidayCoverage::contains))
/// carry no endpoint conversion and no fallback branch.
///
/// The quotations, URLs, capture times and interpretive steps stay in
/// `docs/evidence/<owner>.md` (LAW-EVIDENCE-FILES); the row carries only its
/// tier and document id, and a fence checks that both appear there.
macro_rules! holidays {
    (
        coverage: [$(($first_year:expr, $first_month:expr, $first_day:expr)
            ..= ($last_year:expr, $last_month:expr, $last_day:expr)),* $(,)?],
        rows: [
            $( ($year:expr, $month:expr, $day:expr, $kind:expr, $tier:expr, $document:literal) ),*
            $(,)?
        ] $(,)?
    ) => {{
        const ROWS: &[$crate::calendar::schedules::holidays::HolidayRow] = &[
            $(
                $crate::calendar::schedules::holidays::HolidayRow {
                    trade_date: $crate::calendar::schedules::holidays::fences::holiday_date(
                        $year, $month, $day,
                    ),
                    kind: $kind,
                    tier: $tier,
                    document: $crate::calendar::schedules::timeline::SourceRef::new($document),
                }
            ),*
        ];
        const WINDOWS: &[$crate::calendar::schedules::holidays::HolidayWindow] = &[
            $(
                $crate::calendar::schedules::holidays::HolidayWindow::new(
                    ($first_year, $first_month, $first_day),
                    ($last_year, $last_month, $last_day),
                )
            ),*
        ];
        const TABLE: &$crate::calendar::schedules::holidays::HolidayTable =
            &$crate::calendar::schedules::holidays::HolidayTable {
                windows: WINDOWS,
                rows: ROWS,
                has_blocks: $crate::calendar::schedules::holidays::fences::any_blocks(ROWS),
            };
        const _: () = {
            const DATES: &[(i32, u32, u32)] = &[$(($year, $month, $day)),*];
            $crate::calendar::schedules::holidays::fences::assert_table(WINDOWS, DATES, ROWS);
        };
        TABLE
    }};
}

pub(crate) use holidays;
