// SPDX-License-Identifier: MIT-0

//! The Nasdaq Stock Market holiday rows, 2010-2026.
//!
//! Keyed by the venue-local trade date in `America/New_York`; every session
//! the operator publishes inside one civil day, so an event date is its own
//! trade date and no conversion applies.
//!
//! The whole block is **T1**: the operator's own "U.S. Equity and Options
//! Markets Holiday Schedule" pages on `nasdaqtrader.com` (2010-2025, read as
//! Wayback `id_` replays of the operator's pages), the live operator pages and
//! holiday PDF (2026), and the operator's Equity Trader Alert `ETA2012-44`
//! for the Hurricane Sandy closure. Every artifact is saved under
//! `holidays/raw/nyse-nasdaq/` in the research store with its sha256, and the
//! per-row derivation is recorded in
//! [`docs/evidence/nasdaq.md`](../../../../../docs/evidence/nasdaq.md).
//!
//! Every scalar row is either `Closed` or `early_close(13:00)`: from 2012 on,
//! the operator's sheet prints each early close for the Nasdaq Stock Market
//! itself as "Early Close - U.S. 1:00 p.m." and states nothing else for those
//! dates, so the whole envelope clips. The same sheets add that "Nasdaq will
//! continue to send alerts to notify customers of days when the Market will
//! close early ... for full information, including system operating times";
//! no such alert was recoverable from the operator's own channels for any
//! historical early-close date (see the evidence file's interpretive steps),
//! so no row invents post-13:00 session topology the sheets do not print.
//! The sheet's options columns state option-product times; they are a
//! different market and never key a row here.
//!
//! Coverage is 2010-01-01..2026-12-31. The operator has published no 2027
//! holiday schedule (verified 2026-09-27 UTC against both live channels), so
//! nothing past 2026-12-31 is claimed. Four dates inside the window are
//! `Unsourced` — 2010-11-26 and 2011-11-25, whose early closes the sheets
//! print as `TBA` whose alert text is unrecoverable; 2012-10-30, whose Sandy
//! closure the operator's own alert calls "likely" with the confirmation
//! unrecovered; and 2025-01-09, whose mourning closure the operator announced
//! outside every channel this store could reach — and this table therefore
//! ships labelled incomplete, never complete-by-omission.

use super::EvidenceTier::T1;
use super::HolidayKind::{Closed, Unsourced};
use super::fences::early_close;
use super::{HolidayTable, holidays};

/// The Nasdaq Stock Market's built-in holiday rows and the window they were
/// audited over.
///
/// Every row's date, kind and instant is one line of an operator holiday page
/// or trader alert; the document ids resolve through the evidence file's
/// `### Documents` table.
// Evidence: docs/evidence/nasdaq.md
pub(crate) static TABLE: &HolidayTable = holidays! {
    coverage: [(2010, 1, 1) ..= (2026, 12, 31)],
    rows: [
        // 2010 - NQ-HOL-2010; the 11-26 early close prints as TBA on the
        // sheet and the alert is unrecovered, so the date is withheld.
        (2010, 1, 1, Closed, T1, "NQ-HOL-2010"),
        (2010, 1, 18, Closed, T1, "NQ-HOL-2010"),
        (2010, 2, 15, Closed, T1, "NQ-HOL-2010"),
        (2010, 4, 2, Closed, T1, "NQ-HOL-2010"),
        (2010, 5, 31, Closed, T1, "NQ-HOL-2010"),
        (2010, 7, 5, Closed, T1, "NQ-HOL-2010"),
        (2010, 9, 6, Closed, T1, "NQ-HOL-2010"),
        (2010, 11, 25, Closed, T1, "NQ-HOL-2010"),
        (2010, 11, 26, Unsourced, T1, "NQ-HOL-2010"),
        (2010, 12, 24, Closed, T1, "NQ-HOL-2010"),
        // 2011 - NQ-HOL-2011; 11-25 early close prints as TBA.
        (2011, 1, 17, Closed, T1, "NQ-HOL-2011"),
        (2011, 2, 21, Closed, T1, "NQ-HOL-2011"),
        (2011, 4, 22, Closed, T1, "NQ-HOL-2011"),
        (2011, 5, 30, Closed, T1, "NQ-HOL-2011"),
        (2011, 7, 4, Closed, T1, "NQ-HOL-2011"),
        (2011, 9, 5, Closed, T1, "NQ-HOL-2011"),
        (2011, 11, 24, Closed, T1, "NQ-HOL-2011"),
        (2011, 11, 25, Unsourced, T1, "NQ-HOL-2011"),
        (2011, 12, 26, Closed, T1, "NQ-HOL-2011"),
        // 2012 - NQ-HOL-2011 (2012 table) + ETA2012-44 for the Sandy
        // closure; 10-30 stays withheld ("likely ... will confirm").
        (2012, 1, 2, Closed, T1, "NQ-HOL-2011"),
        (2012, 1, 16, Closed, T1, "NQ-HOL-2011"),
        (2012, 2, 20, Closed, T1, "NQ-HOL-2011"),
        (2012, 4, 6, Closed, T1, "NQ-HOL-2011"),
        (2012, 5, 28, Closed, T1, "NQ-HOL-2011"),
        (2012, 7, 3, early_close(13 * 3_600), T1, "NQ-HOL-2012"),
        (2012, 7, 4, Closed, T1, "NQ-HOL-2012"),
        (2012, 9, 3, Closed, T1, "NQ-HOL-2012"),
        (2012, 10, 29, Closed, T1, "NQ-SANDY-2012"),
        (2012, 10, 30, Unsourced, T1, "NQ-HOL-2012"),
        (2012, 11, 22, Closed, T1, "NQ-HOL-2012"),
        (2012, 11, 23, early_close(13 * 3_600), T1, "NQ-HOL-2012"),
        (2012, 12, 24, early_close(13 * 3_600), T1, "NQ-HOL-2012"),
        (2012, 12, 25, Closed, T1, "NQ-HOL-2012"),
        // 2013 - NQ-HOL-2013.
        (2013, 1, 1, Closed, T1, "NQ-HOL-2013"),
        (2013, 1, 21, Closed, T1, "NQ-HOL-2013"),
        (2013, 2, 18, Closed, T1, "NQ-HOL-2013"),
        (2013, 3, 29, Closed, T1, "NQ-HOL-2013"),
        (2013, 5, 27, Closed, T1, "NQ-HOL-2013"),
        (2013, 7, 3, early_close(13 * 3_600), T1, "NQ-HOL-2013"),
        (2013, 7, 4, Closed, T1, "NQ-HOL-2013"),
        (2013, 9, 2, Closed, T1, "NQ-HOL-2013"),
        (2013, 11, 28, Closed, T1, "NQ-HOL-2013"),
        (2013, 11, 29, early_close(13 * 3_600), T1, "NQ-HOL-2013"),
        (2013, 12, 24, early_close(13 * 3_600), T1, "NQ-HOL-2013"),
        (2013, 12, 25, Closed, T1, "NQ-HOL-2013"),
        // 2014 - NQ-HOL-2014.
        (2014, 1, 1, Closed, T1, "NQ-HOL-2014"),
        (2014, 1, 20, Closed, T1, "NQ-HOL-2014"),
        (2014, 2, 17, Closed, T1, "NQ-HOL-2014"),
        (2014, 4, 18, Closed, T1, "NQ-HOL-2014"),
        (2014, 5, 26, Closed, T1, "NQ-HOL-2014"),
        (2014, 7, 3, early_close(13 * 3_600), T1, "NQ-HOL-2014"),
        (2014, 7, 4, Closed, T1, "NQ-HOL-2014"),
        (2014, 9, 1, Closed, T1, "NQ-HOL-2014"),
        (2014, 11, 27, Closed, T1, "NQ-HOL-2014"),
        (2014, 11, 28, early_close(13 * 3_600), T1, "NQ-HOL-2014"),
        (2014, 12, 24, early_close(13 * 3_600), T1, "NQ-HOL-2014"),
        (2014, 12, 25, Closed, T1, "NQ-HOL-2014"),
        // 2015 - NQ-HOL-2015: Independence Day observed Friday July 3.
        (2015, 1, 1, Closed, T1, "NQ-HOL-2015"),
        (2015, 1, 19, Closed, T1, "NQ-HOL-2015"),
        (2015, 2, 16, Closed, T1, "NQ-HOL-2015"),
        (2015, 4, 3, Closed, T1, "NQ-HOL-2015"),
        (2015, 5, 25, Closed, T1, "NQ-HOL-2015"),
        (2015, 7, 3, Closed, T1, "NQ-HOL-2015"),
        (2015, 9, 7, Closed, T1, "NQ-HOL-2015"),
        (2015, 11, 26, Closed, T1, "NQ-HOL-2015"),
        (2015, 11, 27, early_close(13 * 3_600), T1, "NQ-HOL-2015"),
        (2015, 12, 24, early_close(13 * 3_600), T1, "NQ-HOL-2015"),
        (2015, 12, 25, Closed, T1, "NQ-HOL-2015"),
        // 2016 - NQ-HOL-2015 (2016 table).
        (2016, 1, 1, Closed, T1, "NQ-HOL-2015"),
        (2016, 1, 18, Closed, T1, "NQ-HOL-2015"),
        (2016, 2, 15, Closed, T1, "NQ-HOL-2015"),
        (2016, 3, 25, Closed, T1, "NQ-HOL-2015"),
        (2016, 5, 30, Closed, T1, "NQ-HOL-2015"),
        (2016, 7, 4, Closed, T1, "NQ-HOL-2015"),
        (2016, 9, 5, Closed, T1, "NQ-HOL-2015"),
        (2016, 11, 24, Closed, T1, "NQ-HOL-2015"),
        (2016, 11, 25, early_close(13 * 3_600), T1, "NQ-HOL-2015"),
        (2016, 12, 26, Closed, T1, "NQ-HOL-2015"),
        // 2017 - NQ-HOL-2017 (the December-2016 capture of the 2017 page).
        (2017, 1, 2, Closed, T1, "NQ-HOL-2017"),
        (2017, 1, 16, Closed, T1, "NQ-HOL-2017"),
        (2017, 2, 20, Closed, T1, "NQ-HOL-2017"),
        (2017, 4, 14, Closed, T1, "NQ-HOL-2017"),
        (2017, 5, 29, Closed, T1, "NQ-HOL-2017"),
        (2017, 7, 3, early_close(13 * 3_600), T1, "NQ-HOL-2017"),
        (2017, 7, 4, Closed, T1, "NQ-HOL-2017"),
        (2017, 9, 4, Closed, T1, "NQ-HOL-2017"),
        (2017, 11, 23, Closed, T1, "NQ-HOL-2017"),
        (2017, 11, 24, early_close(13 * 3_600), T1, "NQ-HOL-2017"),
        (2017, 12, 25, Closed, T1, "NQ-HOL-2017"),
        // 2018 - NQ-HOL-2018 (the December-2017 capture of the 2018 page).
        (2018, 1, 1, Closed, T1, "NQ-HOL-2018"),
        (2018, 1, 15, Closed, T1, "NQ-HOL-2018"),
        (2018, 2, 19, Closed, T1, "NQ-HOL-2018"),
        (2018, 3, 30, Closed, T1, "NQ-HOL-2018"),
        (2018, 5, 28, Closed, T1, "NQ-HOL-2018"),
        (2018, 7, 3, early_close(13 * 3_600), T1, "NQ-HOL-2018"),
        (2018, 7, 4, Closed, T1, "NQ-HOL-2018"),
        (2018, 9, 3, Closed, T1, "NQ-HOL-2018"),
        (2018, 11, 22, Closed, T1, "NQ-HOL-2018"),
        (2018, 11, 23, early_close(13 * 3_600), T1, "NQ-HOL-2018"),
        (2018, 12, 24, early_close(13 * 3_600), T1, "NQ-HOL-2018"),
        (2018, 12, 25, Closed, T1, "NQ-HOL-2018"),
        // 2019 - NQ-HOL-2019 (the December-2018 capture of the 2019 page).
        (2019, 1, 1, Closed, T1, "NQ-HOL-2019"),
        (2019, 1, 21, Closed, T1, "NQ-HOL-2019"),
        (2019, 2, 18, Closed, T1, "NQ-HOL-2019"),
        (2019, 4, 19, Closed, T1, "NQ-HOL-2019"),
        (2019, 5, 27, Closed, T1, "NQ-HOL-2019"),
        (2019, 7, 3, early_close(13 * 3_600), T1, "NQ-HOL-2019"),
        (2019, 7, 4, Closed, T1, "NQ-HOL-2019"),
        (2019, 9, 2, Closed, T1, "NQ-HOL-2019"),
        (2019, 11, 28, Closed, T1, "NQ-HOL-2019"),
        (2019, 11, 29, early_close(13 * 3_600), T1, "NQ-HOL-2019"),
        (2019, 12, 24, early_close(13 * 3_600), T1, "NQ-HOL-2019"),
        (2019, 12, 25, Closed, T1, "NQ-HOL-2019"),
        // 2020 - NQ-HOL-2020 (the July-2020 capture of the 2020 page).
        (2020, 1, 1, Closed, T1, "NQ-HOL-2020"),
        (2020, 1, 20, Closed, T1, "NQ-HOL-2020"),
        (2020, 2, 17, Closed, T1, "NQ-HOL-2020"),
        (2020, 4, 10, Closed, T1, "NQ-HOL-2020"),
        (2020, 5, 25, Closed, T1, "NQ-HOL-2020"),
        (2020, 7, 3, Closed, T1, "NQ-HOL-2020"),
        (2020, 9, 7, Closed, T1, "NQ-HOL-2020"),
        (2020, 11, 26, Closed, T1, "NQ-HOL-2020"),
        (2020, 11, 27, early_close(13 * 3_600), T1, "NQ-HOL-2020"),
        (2020, 12, 24, early_close(13 * 3_600), T1, "NQ-HOL-2020"),
        (2020, 12, 25, Closed, T1, "NQ-HOL-2020"),
        // 2021 - NQ-HOL-2021 (the December-2020 capture of the 2021 page).
        (2021, 1, 1, Closed, T1, "NQ-HOL-2021"),
        (2021, 1, 18, Closed, T1, "NQ-HOL-2021"),
        (2021, 2, 15, Closed, T1, "NQ-HOL-2021"),
        (2021, 4, 2, Closed, T1, "NQ-HOL-2021"),
        (2021, 5, 31, Closed, T1, "NQ-HOL-2021"),
        (2021, 7, 5, Closed, T1, "NQ-HOL-2021"),
        (2021, 9, 6, Closed, T1, "NQ-HOL-2021"),
        (2021, 11, 25, Closed, T1, "NQ-HOL-2021"),
        (2021, 11, 26, early_close(13 * 3_600), T1, "NQ-HOL-2021"),
        (2021, 12, 24, Closed, T1, "NQ-HOL-2021"),
        // 2022 - NQ-HOL-2022 (the December-2021 capture of the 2022 page);
        // Juneteenth enters the grid, observed Monday June 20.
        (2022, 1, 17, Closed, T1, "NQ-HOL-2022"),
        (2022, 2, 21, Closed, T1, "NQ-HOL-2022"),
        (2022, 4, 15, Closed, T1, "NQ-HOL-2022"),
        (2022, 5, 30, Closed, T1, "NQ-HOL-2022"),
        (2022, 6, 20, Closed, T1, "NQ-HOL-2022"),
        (2022, 7, 4, Closed, T1, "NQ-HOL-2022"),
        (2022, 9, 5, Closed, T1, "NQ-HOL-2022"),
        (2022, 11, 24, Closed, T1, "NQ-HOL-2022"),
        (2022, 11, 25, early_close(13 * 3_600), T1, "NQ-HOL-2022"),
        (2022, 12, 26, Closed, T1, "NQ-HOL-2022"),
        // 2023 - NQ-HOL-2023 (the December-2022 capture of the 2023 page).
        (2023, 1, 2, Closed, T1, "NQ-HOL-2023"),
        (2023, 1, 16, Closed, T1, "NQ-HOL-2023"),
        (2023, 2, 20, Closed, T1, "NQ-HOL-2023"),
        (2023, 4, 7, Closed, T1, "NQ-HOL-2023"),
        (2023, 5, 29, Closed, T1, "NQ-HOL-2023"),
        (2023, 6, 19, Closed, T1, "NQ-HOL-2023"),
        (2023, 7, 4, Closed, T1, "NQ-HOL-2023"),
        (2023, 9, 4, Closed, T1, "NQ-HOL-2023"),
        (2023, 11, 23, Closed, T1, "NQ-HOL-2023"),
        (2023, 11, 24, early_close(13 * 3_600), T1, "NQ-HOL-2023"),
        (2023, 12, 25, Closed, T1, "NQ-HOL-2023"),
        // 2024 - NQ-HOL-2024 (the December-2023 capture of the 2024 page).
        (2024, 1, 1, Closed, T1, "NQ-HOL-2024"),
        (2024, 1, 15, Closed, T1, "NQ-HOL-2024"),
        (2024, 2, 19, Closed, T1, "NQ-HOL-2024"),
        (2024, 3, 29, Closed, T1, "NQ-HOL-2024"),
        (2024, 5, 27, Closed, T1, "NQ-HOL-2024"),
        (2024, 6, 19, Closed, T1, "NQ-HOL-2024"),
        (2024, 7, 3, early_close(13 * 3_600), T1, "NQ-HOL-2024"),
        (2024, 7, 4, Closed, T1, "NQ-HOL-2024"),
        (2024, 9, 2, Closed, T1, "NQ-HOL-2024"),
        (2024, 11, 28, Closed, T1, "NQ-HOL-2024"),
        (2024, 11, 29, early_close(13 * 3_600), T1, "NQ-HOL-2024"),
        (2024, 12, 24, early_close(13 * 3_600), T1, "NQ-HOL-2024"),
        (2024, 12, 25, Closed, T1, "NQ-HOL-2024"),
        // 2025 - NQ-HOL-2025 (the December-2024 capture of the 2025 page).
        // The National Day of Mourning 2025-01-09 is not on the operator's
        // own sheet and no notice was recoverable, so the date is withheld.
        (2025, 1, 1, Closed, T1, "NQ-HOL-2025"),
        (2025, 1, 9, Unsourced, T1, "NQ-HOL-2025"),
        (2025, 1, 20, Closed, T1, "NQ-HOL-2025"),
        (2025, 2, 17, Closed, T1, "NQ-HOL-2025"),
        (2025, 4, 18, Closed, T1, "NQ-HOL-2025"),
        (2025, 5, 26, Closed, T1, "NQ-HOL-2025"),
        (2025, 6, 19, Closed, T1, "NQ-HOL-2025"),
        (2025, 7, 3, early_close(13 * 3_600), T1, "NQ-HOL-2025"),
        (2025, 7, 4, Closed, T1, "NQ-HOL-2025"),
        (2025, 9, 1, Closed, T1, "NQ-HOL-2025"),
        (2025, 11, 27, Closed, T1, "NQ-HOL-2025"),
        (2025, 11, 28, early_close(13 * 3_600), T1, "NQ-HOL-2025"),
        (2025, 12, 24, early_close(13 * 3_600), T1, "NQ-HOL-2025"),
        (2025, 12, 25, Closed, T1, "NQ-HOL-2025"),
        // 2026 - NQ-HOL-2026 (live operator calendar page).
        (2026, 1, 1, Closed, T1, "NQ-HOL-2026"),
        (2026, 1, 19, Closed, T1, "NQ-HOL-2026"),
        (2026, 2, 16, Closed, T1, "NQ-HOL-2026"),
        (2026, 4, 3, Closed, T1, "NQ-HOL-2026"),
        (2026, 5, 25, Closed, T1, "NQ-HOL-2026"),
        (2026, 6, 19, Closed, T1, "NQ-HOL-2026"),
        (2026, 7, 3, Closed, T1, "NQ-HOL-2026"),
        (2026, 9, 7, Closed, T1, "NQ-HOL-2026"),
        (2026, 11, 26, Closed, T1, "NQ-HOL-2026"),
        (2026, 11, 27, early_close(13 * 3_600), T1, "NQ-HOL-2026"),
        (2026, 12, 24, early_close(13 * 3_600), T1, "NQ-HOL-2026"),
        (2026, 12, 25, Closed, T1, "NQ-HOL-2026"),
    ],
};
