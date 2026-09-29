// SPDX-License-Identifier: MIT-0

//! New York Stock Exchange holiday rows, 2010-2028.
//!
//! Keyed by the venue-local trade date in `America/New_York`; every session
//! the exchange publishes inside one civil day, so an event date is its own
//! trade date and no conversion applies.
//!
//! The whole block is **T1**: the operator's own holiday calendar pages, read
//! live (2026-2028) or as Wayback `id_` replays of the operator's own pages
//! (2010-2025), plus the operator's own press releases for the two Hurricane
//! Sandy closures and the ICE release naming the New York Stock Exchange for
//! the 2025 National Day of Mourning. Every artifact is saved under
//! `holidays/raw/nyse-nasdaq/` in the research store with its sha256, and the
//! per-row derivation is recorded in
//! [`docs/evidence/nyse.md`](../../../../../docs/evidence/nyse.md).
//!
//! Every scalar row is either `Closed` or `early_close(13:00)`: the operator's
//! sheet states, per early-close date, that "each market will close early at
//! 1:00 p.m.", and states nothing else for those dates, so the whole envelope
//! clips. The same footnotes add that Crossing Session orders are accepted
//! 1:00-1:30 p.m. and that the NYSE American/Arca/Chicago/National *late
//! trading sessions* close at 5:00 p.m.; neither concerns this identity — the
//! crossing facility is the `nyse` row's recorded executable gap (modelled
//! nowhere, so it errs toward closed), and the other venues are separate
//! identities — so neither clause moves a row.
//!
//! Coverage is 2010-01-01..2028-12-31 with no `Unsourced` dates: each year's
//! holidays and early closes are printed in full by at least one operator
//! artifact, and the live page's 2028 column ships from the same artifact as
//! the 2026-2027 columns.

use super::EvidenceTier::T1;
use super::fences::early_close;
use super::{HolidayKind::Closed, HolidayTable, holidays};

/// The New York Stock Exchange's built-in holiday rows and the window they
/// were audited over.
///
/// Every row's date, kind and instant is one line of an NYSE holiday page or
/// closure press release; the document ids resolve through the evidence file's
/// `### Documents` table.
// Evidence: docs/evidence/nyse.md
pub(crate) static TABLE: &HolidayTable = holidays! {
    coverage: [(2010, 1, 1) ..= (2028, 12, 31)],
    rows: [
        // 2010 - NYSE-HOL-2010 / NYSE-HOL-2010LATE - operator holiday pages.
        (2010, 1, 1, Closed, T1, "NYSE-HOL-2010"),
        (2010, 1, 18, Closed, T1, "NYSE-HOL-2010"),
        (2010, 2, 15, Closed, T1, "NYSE-HOL-2010"),
        (2010, 4, 2, Closed, T1, "NYSE-HOL-2010"),
        (2010, 5, 31, Closed, T1, "NYSE-HOL-2010"),
        (2010, 7, 5, Closed, T1, "NYSE-HOL-2010"),
        (2010, 9, 6, Closed, T1, "NYSE-HOL-2010"),
        (2010, 11, 25, Closed, T1, "NYSE-HOL-2010"),
        // Early close as printed: "close at 1:00 p.m. on Friday, November
        // 26, 2010 (the day after Thanksgiving)".
        (2010, 11, 26, early_close(13 * 3_600), T1, "NYSE-HOL-2010LATE"),
        (2010, 12, 24, Closed, T1, "NYSE-HOL-2010LATE"),
        // 2011 - NYSE-HOL-2011.
        (2011, 1, 17, Closed, T1, "NYSE-HOL-2011"),
        (2011, 2, 21, Closed, T1, "NYSE-HOL-2011"),
        (2011, 4, 22, Closed, T1, "NYSE-HOL-2011"),
        (2011, 5, 30, Closed, T1, "NYSE-HOL-2011"),
        (2011, 7, 4, Closed, T1, "NYSE-HOL-2011"),
        (2011, 9, 5, Closed, T1, "NYSE-HOL-2011"),
        (2011, 11, 24, Closed, T1, "NYSE-HOL-2011"),
        (2011, 11, 25, early_close(13 * 3_600), T1, "NYSE-HOL-2011"),
        (2011, 12, 26, Closed, T1, "NYSE-HOL-2011"),
        // 2012 - NYSE-HOL-2011 (2012 table and footnotes) + the Sandy
        // press releases for the unscheduled closures.
        (2012, 1, 2, Closed, T1, "NYSE-HOL-2011"),
        (2012, 1, 16, Closed, T1, "NYSE-HOL-2011"),
        (2012, 2, 20, Closed, T1, "NYSE-HOL-2011"),
        (2012, 4, 6, Closed, T1, "NYSE-HOL-2011"),
        (2012, 5, 28, Closed, T1, "NYSE-HOL-2011"),
        (2012, 7, 3, early_close(13 * 3_600), T1, "NYSE-HOL-2011"),
        (2012, 7, 4, Closed, T1, "NYSE-HOL-2011"),
        (2012, 9, 3, Closed, T1, "NYSE-HOL-2011"),
        (2012, 10, 29, Closed, T1, "NYSE-SANDY-2012A"),
        (2012, 10, 30, Closed, T1, "NYSE-SANDY-2012B"),
        (2012, 11, 22, Closed, T1, "NYSE-HOL-2011"),
        (2012, 11, 23, early_close(13 * 3_600), T1, "NYSE-HOL-2011"),
        (2012, 12, 24, early_close(13 * 3_600), T1, "NYSE-HOL-2011"),
        (2012, 12, 25, Closed, T1, "NYSE-HOL-2011"),
        // 2013 - NYSE-HOL-2011 (2013 table and footnotes).
        (2013, 1, 1, Closed, T1, "NYSE-HOL-2011"),
        (2013, 1, 21, Closed, T1, "NYSE-HOL-2011"),
        (2013, 2, 18, Closed, T1, "NYSE-HOL-2011"),
        (2013, 3, 29, Closed, T1, "NYSE-HOL-2011"),
        (2013, 5, 27, Closed, T1, "NYSE-HOL-2011"),
        (2013, 7, 3, early_close(13 * 3_600), T1, "NYSE-HOL-2011"),
        (2013, 7, 4, Closed, T1, "NYSE-HOL-2011"),
        (2013, 9, 2, Closed, T1, "NYSE-HOL-2011"),
        (2013, 11, 28, Closed, T1, "NYSE-HOL-2011"),
        (2013, 11, 29, early_close(13 * 3_600), T1, "NYSE-HOL-2011"),
        (2013, 12, 24, early_close(13 * 3_600), T1, "NYSE-HOL-2011"),
        (2013, 12, 25, Closed, T1, "NYSE-HOL-2011"),
        // 2014 - NYSE-HOL-2014.
        (2014, 1, 1, Closed, T1, "NYSE-HOL-2014"),
        (2014, 1, 20, Closed, T1, "NYSE-HOL-2014"),
        (2014, 2, 17, Closed, T1, "NYSE-HOL-2014"),
        (2014, 4, 18, Closed, T1, "NYSE-HOL-2014"),
        (2014, 5, 26, Closed, T1, "NYSE-HOL-2014"),
        (2014, 7, 3, early_close(13 * 3_600), T1, "NYSE-HOL-2014"),
        (2014, 7, 4, Closed, T1, "NYSE-HOL-2014"),
        (2014, 9, 1, Closed, T1, "NYSE-HOL-2014"),
        (2014, 11, 27, Closed, T1, "NYSE-HOL-2014"),
        (2014, 11, 28, early_close(13 * 3_600), T1, "NYSE-HOL-2014"),
        (2014, 12, 24, early_close(13 * 3_600), T1, "NYSE-HOL-2014"),
        (2014, 12, 25, Closed, T1, "NYSE-HOL-2014"),
        // 2015 - NYSE-HOL-2015: Independence Day observed Friday July 3,
        // so the holiday itself deletes that Thursday-opened trade date.
        (2015, 1, 1, Closed, T1, "NYSE-HOL-2015"),
        (2015, 1, 19, Closed, T1, "NYSE-HOL-2015"),
        (2015, 2, 16, Closed, T1, "NYSE-HOL-2015"),
        (2015, 4, 3, Closed, T1, "NYSE-HOL-2015"),
        (2015, 5, 25, Closed, T1, "NYSE-HOL-2015"),
        (2015, 7, 3, Closed, T1, "NYSE-HOL-2015"),
        (2015, 9, 7, Closed, T1, "NYSE-HOL-2015"),
        (2015, 11, 26, Closed, T1, "NYSE-HOL-2015"),
        (2015, 11, 27, early_close(13 * 3_600), T1, "NYSE-HOL-2015"),
        (2015, 12, 24, early_close(13 * 3_600), T1, "NYSE-HOL-2015"),
        (2015, 12, 25, Closed, T1, "NYSE-HOL-2015"),
        // 2016 - NYSE-HOL-2016.
        (2016, 1, 1, Closed, T1, "NYSE-HOL-2016"),
        (2016, 1, 18, Closed, T1, "NYSE-HOL-2016"),
        (2016, 2, 15, Closed, T1, "NYSE-HOL-2016"),
        (2016, 3, 25, Closed, T1, "NYSE-HOL-2016"),
        (2016, 5, 30, Closed, T1, "NYSE-HOL-2016"),
        (2016, 7, 4, Closed, T1, "NYSE-HOL-2016"),
        (2016, 9, 5, Closed, T1, "NYSE-HOL-2016"),
        (2016, 11, 24, Closed, T1, "NYSE-HOL-2016"),
        (2016, 11, 25, early_close(13 * 3_600), T1, "NYSE-HOL-2016"),
        (2016, 12, 26, Closed, T1, "NYSE-HOL-2016"),
        // 2017 - NYSE-HOL-2017.
        (2017, 1, 2, Closed, T1, "NYSE-HOL-2017"),
        (2017, 1, 16, Closed, T1, "NYSE-HOL-2017"),
        (2017, 2, 20, Closed, T1, "NYSE-HOL-2017"),
        (2017, 4, 14, Closed, T1, "NYSE-HOL-2017"),
        (2017, 5, 29, Closed, T1, "NYSE-HOL-2017"),
        (2017, 7, 3, early_close(13 * 3_600), T1, "NYSE-HOL-2017"),
        (2017, 7, 4, Closed, T1, "NYSE-HOL-2017"),
        (2017, 9, 4, Closed, T1, "NYSE-HOL-2017"),
        (2017, 11, 23, Closed, T1, "NYSE-HOL-2017"),
        (2017, 11, 24, early_close(13 * 3_600), T1, "NYSE-HOL-2017"),
        (2017, 12, 25, Closed, T1, "NYSE-HOL-2017"),
        // 2018 - NYSE-HOL-2018.
        (2018, 1, 1, Closed, T1, "NYSE-HOL-2018"),
        (2018, 1, 15, Closed, T1, "NYSE-HOL-2018"),
        (2018, 2, 19, Closed, T1, "NYSE-HOL-2018"),
        (2018, 3, 30, Closed, T1, "NYSE-HOL-2018"),
        (2018, 5, 28, Closed, T1, "NYSE-HOL-2018"),
        (2018, 7, 3, early_close(13 * 3_600), T1, "NYSE-HOL-2018"),
        (2018, 7, 4, Closed, T1, "NYSE-HOL-2018"),
        (2018, 9, 3, Closed, T1, "NYSE-HOL-2018"),
        (2018, 11, 22, Closed, T1, "NYSE-HOL-2018"),
        (2018, 11, 23, early_close(13 * 3_600), T1, "NYSE-HOL-2018"),
        (2018, 12, 24, early_close(13 * 3_600), T1, "NYSE-HOL-2018"),
        (2018, 12, 25, Closed, T1, "NYSE-HOL-2018"),
        // 2019 - NYSE-HOL-2019.
        (2019, 1, 1, Closed, T1, "NYSE-HOL-2019"),
        (2019, 1, 21, Closed, T1, "NYSE-HOL-2019"),
        (2019, 2, 18, Closed, T1, "NYSE-HOL-2019"),
        (2019, 4, 19, Closed, T1, "NYSE-HOL-2019"),
        (2019, 5, 27, Closed, T1, "NYSE-HOL-2019"),
        (2019, 7, 3, early_close(13 * 3_600), T1, "NYSE-HOL-2019"),
        (2019, 7, 4, Closed, T1, "NYSE-HOL-2019"),
        (2019, 9, 2, Closed, T1, "NYSE-HOL-2019"),
        (2019, 11, 28, Closed, T1, "NYSE-HOL-2019"),
        (2019, 11, 29, early_close(13 * 3_600), T1, "NYSE-HOL-2019"),
        (2019, 12, 24, early_close(13 * 3_600), T1, "NYSE-HOL-2019"),
        (2019, 12, 25, Closed, T1, "NYSE-HOL-2019"),
        // 2020 - NYSE-HOL-2020.
        (2020, 1, 1, Closed, T1, "NYSE-HOL-2020"),
        (2020, 1, 20, Closed, T1, "NYSE-HOL-2020"),
        (2020, 2, 17, Closed, T1, "NYSE-HOL-2020"),
        (2020, 4, 10, Closed, T1, "NYSE-HOL-2020"),
        (2020, 5, 25, Closed, T1, "NYSE-HOL-2020"),
        (2020, 7, 3, Closed, T1, "NYSE-HOL-2020"),
        (2020, 9, 7, Closed, T1, "NYSE-HOL-2020"),
        (2020, 11, 26, Closed, T1, "NYSE-HOL-2020"),
        (2020, 11, 27, early_close(13 * 3_600), T1, "NYSE-HOL-2020"),
        (2020, 12, 24, early_close(13 * 3_600), T1, "NYSE-HOL-2020"),
        (2020, 12, 25, Closed, T1, "NYSE-HOL-2020"),
        // 2021 - NYSE-HOL-2021: Christmas observed Friday December 24.
        (2021, 1, 1, Closed, T1, "NYSE-HOL-2021"),
        (2021, 1, 18, Closed, T1, "NYSE-HOL-2021"),
        (2021, 2, 15, Closed, T1, "NYSE-HOL-2021"),
        (2021, 4, 2, Closed, T1, "NYSE-HOL-2021"),
        (2021, 5, 31, Closed, T1, "NYSE-HOL-2021"),
        (2021, 7, 5, Closed, T1, "NYSE-HOL-2021"),
        (2021, 9, 6, Closed, T1, "NYSE-HOL-2021"),
        (2021, 11, 25, Closed, T1, "NYSE-HOL-2021"),
        (2021, 11, 26, early_close(13 * 3_600), T1, "NYSE-HOL-2021"),
        (2021, 12, 24, Closed, T1, "NYSE-HOL-2021"),
        // 2022 - NYSE-HOL-2022: Juneteenth enters the grid, observed
        // Monday June 20.
        (2022, 1, 17, Closed, T1, "NYSE-HOL-2022"),
        (2022, 2, 21, Closed, T1, "NYSE-HOL-2022"),
        (2022, 4, 15, Closed, T1, "NYSE-HOL-2022"),
        (2022, 5, 30, Closed, T1, "NYSE-HOL-2022"),
        (2022, 6, 20, Closed, T1, "NYSE-HOL-2022"),
        (2022, 7, 4, Closed, T1, "NYSE-HOL-2022"),
        (2022, 9, 5, Closed, T1, "NYSE-HOL-2022"),
        (2022, 11, 24, Closed, T1, "NYSE-HOL-2022"),
        (2022, 11, 25, early_close(13 * 3_600), T1, "NYSE-HOL-2022"),
        (2022, 12, 26, Closed, T1, "NYSE-HOL-2022"),
        // 2023 - NYSE-HOL-2023.
        (2023, 1, 2, Closed, T1, "NYSE-HOL-2023"),
        (2023, 1, 16, Closed, T1, "NYSE-HOL-2023"),
        (2023, 2, 20, Closed, T1, "NYSE-HOL-2023"),
        (2023, 4, 7, Closed, T1, "NYSE-HOL-2023"),
        (2023, 5, 29, Closed, T1, "NYSE-HOL-2023"),
        (2023, 6, 19, Closed, T1, "NYSE-HOL-2023"),
        (2023, 7, 3, early_close(13 * 3_600), T1, "NYSE-HOL-2023"),
        (2023, 7, 4, Closed, T1, "NYSE-HOL-2023"),
        (2023, 9, 4, Closed, T1, "NYSE-HOL-2023"),
        (2023, 11, 23, Closed, T1, "NYSE-HOL-2023"),
        (2023, 11, 24, early_close(13 * 3_600), T1, "NYSE-HOL-2023"),
        (2023, 12, 25, Closed, T1, "NYSE-HOL-2023"),
        // 2024 - NYSE-HOL-2024.
        (2024, 1, 1, Closed, T1, "NYSE-HOL-2024"),
        (2024, 1, 15, Closed, T1, "NYSE-HOL-2024"),
        (2024, 2, 19, Closed, T1, "NYSE-HOL-2024"),
        (2024, 3, 29, Closed, T1, "NYSE-HOL-2024"),
        (2024, 5, 27, Closed, T1, "NYSE-HOL-2024"),
        (2024, 6, 19, Closed, T1, "NYSE-HOL-2024"),
        (2024, 7, 3, early_close(13 * 3_600), T1, "NYSE-HOL-2024"),
        (2024, 7, 4, Closed, T1, "NYSE-HOL-2024"),
        (2024, 9, 2, Closed, T1, "NYSE-HOL-2024"),
        (2024, 11, 28, Closed, T1, "NYSE-HOL-2024"),
        (2024, 11, 29, early_close(13 * 3_600), T1, "NYSE-HOL-2024"),
        (2024, 12, 24, early_close(13 * 3_600), T1, "NYSE-HOL-2024"),
        (2024, 12, 25, Closed, T1, "NYSE-HOL-2024"),
        // 2025 - NYSE-HOL-2025 + NYSE-CARTER-2025 for the mourning day.
        (2025, 1, 1, Closed, T1, "NYSE-HOL-2025"),
        (2025, 1, 9, Closed, T1, "NYSE-CARTER-2025"),
        (2025, 1, 20, Closed, T1, "NYSE-HOL-2025"),
        (2025, 2, 17, Closed, T1, "NYSE-HOL-2025"),
        (2025, 4, 18, Closed, T1, "NYSE-HOL-2025"),
        (2025, 5, 26, Closed, T1, "NYSE-HOL-2025"),
        (2025, 6, 19, Closed, T1, "NYSE-HOL-2025"),
        (2025, 7, 3, early_close(13 * 3_600), T1, "NYSE-HOL-2025"),
        (2025, 7, 4, Closed, T1, "NYSE-HOL-2025"),
        (2025, 9, 1, Closed, T1, "NYSE-HOL-2025"),
        (2025, 11, 27, Closed, T1, "NYSE-HOL-2025"),
        (2025, 11, 28, early_close(13 * 3_600), T1, "NYSE-HOL-2025"),
        (2025, 12, 24, early_close(13 * 3_600), T1, "NYSE-HOL-2025"),
        (2025, 12, 25, Closed, T1, "NYSE-HOL-2025"),
        // 2026 - NYSE-HOL-2026 (live operator page).
        (2026, 1, 1, Closed, T1, "NYSE-HOL-2026"),
        (2026, 1, 19, Closed, T1, "NYSE-HOL-2026"),
        (2026, 2, 16, Closed, T1, "NYSE-HOL-2026"),
        (2026, 4, 3, Closed, T1, "NYSE-HOL-2026"),
        (2026, 5, 25, Closed, T1, "NYSE-HOL-2026"),
        (2026, 6, 19, Closed, T1, "NYSE-HOL-2026"),
        (2026, 7, 3, Closed, T1, "NYSE-HOL-2026"),
        (2026, 9, 7, Closed, T1, "NYSE-HOL-2026"),
        (2026, 11, 26, Closed, T1, "NYSE-HOL-2026"),
        (2026, 11, 27, early_close(13 * 3_600), T1, "NYSE-HOL-2026"),
        (2026, 12, 24, early_close(13 * 3_600), T1, "NYSE-HOL-2026"),
        (2026, 12, 25, Closed, T1, "NYSE-HOL-2026"),
        // 2027 - NYSE-HOL-2026 (live operator page, 2027 column).
        (2027, 1, 1, Closed, T1, "NYSE-HOL-2026"),
        (2027, 1, 18, Closed, T1, "NYSE-HOL-2026"),
        (2027, 2, 15, Closed, T1, "NYSE-HOL-2026"),
        (2027, 3, 26, Closed, T1, "NYSE-HOL-2026"),
        (2027, 5, 31, Closed, T1, "NYSE-HOL-2026"),
        (2027, 6, 18, Closed, T1, "NYSE-HOL-2026"),
        (2027, 7, 5, Closed, T1, "NYSE-HOL-2026"),
        (2027, 9, 6, Closed, T1, "NYSE-HOL-2026"),
        (2027, 11, 25, Closed, T1, "NYSE-HOL-2026"),
        (2027, 11, 26, early_close(13 * 3_600), T1, "NYSE-HOL-2026"),
        (2027, 12, 24, Closed, T1, "NYSE-HOL-2026"),
        // 2028 - NYSE-HOL-2026 (live operator page, 2028 column). New
        // Year's Day falls on Saturday, January 1, 2028, and the sheet's
        // footnote states no New Year's Day holiday is observed, so Friday
        // 2027-12-31 carries no row and is audited normal.
        (2028, 1, 17, Closed, T1, "NYSE-HOL-2026"),
        (2028, 2, 21, Closed, T1, "NYSE-HOL-2026"),
        (2028, 4, 14, Closed, T1, "NYSE-HOL-2026"),
        (2028, 5, 29, Closed, T1, "NYSE-HOL-2026"),
        (2028, 6, 19, Closed, T1, "NYSE-HOL-2026"),
        // Early close as printed: "close early at 1:00 p.m. ... on Monday,
        // July 3, 2028".
        (2028, 7, 3, early_close(13 * 3_600), T1, "NYSE-HOL-2026"),
        (2028, 7, 4, Closed, T1, "NYSE-HOL-2026"),
        (2028, 9, 4, Closed, T1, "NYSE-HOL-2026"),
        (2028, 11, 23, Closed, T1, "NYSE-HOL-2026"),
        // Early close as printed: "close early at 1:00 p.m. ... on ...
        // Friday, November 24, 2028 (the day after Thanksgiving)".
        (2028, 11, 24, early_close(13 * 3_600), T1, "NYSE-HOL-2026"),
        (2028, 12, 25, Closed, T1, "NYSE-HOL-2026"),
    ],
};
