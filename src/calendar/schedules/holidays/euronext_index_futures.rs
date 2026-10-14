// SPDX-License-Identifier: MIT-0

//! Euronext index-futures holiday rows, 2010 through 2026.
//!
//! Keyed by the crate's own venue-local trade date in `Europe/Paris` (the
//! family's profile zone; Amsterdam shares it). Neither market runs an
//! overnight session, so an event date and its trade date are one civil day
//! and the conversion is the identity.
//!
//! The whole block is **T1**: the operator's own holiday calendar, read across
//! the same four generations as the sibling Paris cash table — the NYSE
//! Euronext press releases of 2010 and 2011, the per-year pages of 2014-2021,
//! the per-market tables from 2022 and the 2026 INFO-FLASH — because the
//! operator publishes **one** holiday calendar per year covering its cash and
//! derivatives markets together. Every row is therefore the shared calendar's
//! Paris cell (or, before the per-market columns, the group-wide list), and
//! the family reads it for its CAC 40 and AEX index futures alike: on every
//! date the banked calendars state, the Amsterdam and Paris columns agree.
//!
//! **The December half days ship `Unsourced`, not early closes.** The
//! generations from 2014 state the derivatives eves only as a group —
//! `Contracts on the Derivatives Markets will close at various times,
//! commencing at 13:55 CET` — and from 2022 the per-market tables print
//! Paris `**Half Trading Day` while deferring the hours to an end-of-year
//! appendix to the Euronext Instructions 4-01/4-03 Trading Manuals. The
//! appendices the store holds state the **cash** manual's Paris equity
//! segments (the 4-01 sheet); the 4-03 sheet carries the fund/NAV facility,
//! and no banked artifact states the index futures' own half-day instants.
//! 2011 is the one year the derivatives eves are stated for this family — the
//! press release's own list opens `Continental and U.S. Individual Equity
//! Futures and Options contracts and Index Products … will be open for a full
//! day` — and 2016 and 2017 state the eves closing `at the usual times` for
//! `the Cash and Derivatives markets`, so those three years ship no eve rows.
//! The 2010 release's `commence the closing of its derivatives contracts from
//! midday GMT` is the same unstated-instant shape and ships `Unsourced` eves.
//!
//! **2012 and 2013 ship `Unsourced` rows, not shared-calendar rows.** The
//! banked 2012 and 2013 documents are the cash-markets notice and Info-Flash
//! (their own scope sentences name the cash markets), and the 2013 restatement
//! on the 2014 calendar page repeats that scope, so no banked artifact states
//! the derivatives markets' 2012-2013 arrangements. The rows mark the dates
//! the cash edition closes as candidates whose status is withheld; a date
//! beside them claims normal only on the operator's one-calendar-per-year
//! practice, which the banked 2010, 2011, 2014-2026 editions witness, and the
//! residual is recorded in
//! [`docs/evidence/euronext_index_futures.md`](../../../../../docs/evidence/euronext_index_futures.md).

use super::EvidenceTier::T1;
use super::HolidayKind::{Closed, Unsourced};
use super::{HolidayTable, holidays};

/// Euronext index-futures rows: the shared calendar's Paris (and, where
/// printed, group-wide) cells that reach the derivatives markets, and the
/// window they were audited over.
///
/// Serves the `euronext_index_futures` key. The 2011 index-products full-day
/// statement and the 2016/2017 usual-times statements ship no eve rows — the
/// operator's own enumeration, not silence to infer from.
// Evidence: docs/evidence/euronext_index_futures.md
pub(crate) static TABLE: &HolidayTable = holidays! {
    coverage: [(2010, 1, 1) ..= (2026, 12, 31)],
    rows: [
        // 2010-01-01 - T1 - EURONEXT-PR-2010 - New Year's Day: the group-wide closure list reaches the derivatives markets.
        (2010, 1, 1, Closed, T1, "EURONEXT-PR-2010"),
        // 2010-04-02 - T1 - EURONEXT-PR-2010 - Good Friday.
        (2010, 4, 2, Closed, T1, "EURONEXT-PR-2010"),
        // 2010-04-05 - T1 - EURONEXT-PR-2010 - Easter Monday.
        (2010, 4, 5, Closed, T1, "EURONEXT-PR-2010"),
        // 2010-12-24 - T1 - EURONEXT-PR-2010 - the release states `NYSE Liffe will commence the closing of its derivatives contracts from midday GMT (1.00 pm CET)` — an arrangement whose per-product instants are unstated; no status is claimed for the index futures' hours.
        (2010, 12, 24, Unsourced, T1, "EURONEXT-PR-2010"),
        // 2010-12-31 - T1 - EURONEXT-PR-2010 - the same derivatives closing-commencement sentence.
        (2010, 12, 31, Unsourced, T1, "EURONEXT-PR-2010"),
        // 2011-04-22 - T1 - EURONEXT-PR-2011 - Good Friday.
        (2011, 4, 22, Closed, T1, "EURONEXT-PR-2011"),
        // 2011-04-25 - T1 - EURONEXT-PR-2011 - Easter Monday.
        (2011, 4, 25, Closed, T1, "EURONEXT-PR-2011"),
        // 2011-12-26 - T1 - EURONEXT-PR-2011 - the list's unnamed weekday closure (Boxing Day).
        (2011, 12, 26, Closed, T1, "EURONEXT-PR-2011"),
        // 2012-04-06 - T1 - EURONEXT-TC-2012 - the banked 2012 edition is the `Calendar of Cash Business Days` notice, whose scope is the cash markets; the derivatives arrangement for Good Friday is not stated in any banked artifact, so the candidate date's status is withheld.
        (2012, 4, 6, Unsourced, T1, "EURONEXT-TC-2012"),
        // 2012-04-09 - T1 - EURONEXT-TC-2012 - the banked 2012 edition is the `Calendar of Cash Business Days` notice, whose scope is the cash markets; the derivatives arrangement for Easter Monday is not stated in any banked artifact, so the candidate date's status is withheld.
        (2012, 4, 9, Unsourced, T1, "EURONEXT-TC-2012"),
        // 2012-05-01 - T1 - EURONEXT-TC-2012 - the banked 2012 edition is the `Calendar of Cash Business Days` notice, whose scope is the cash markets; the derivatives arrangement for Labour Day is not stated in any banked artifact, so the candidate date's status is withheld.
        (2012, 5, 1, Unsourced, T1, "EURONEXT-TC-2012"),
        // 2012-12-24 - T1 - EURONEXT-TC-2012 - the banked 2012 edition is the `Calendar of Cash Business Days` notice, whose scope is the cash markets; the derivatives arrangement for Christmas Eve half day in the cash scope is not stated in any banked artifact, so the candidate date's status is withheld.
        (2012, 12, 24, Unsourced, T1, "EURONEXT-TC-2012"),
        // 2012-12-25 - T1 - EURONEXT-TC-2012 - the banked 2012 edition is the `Calendar of Cash Business Days` notice, whose scope is the cash markets; the derivatives arrangement for Christmas Day is not stated in any banked artifact, so the candidate date's status is withheld.
        (2012, 12, 25, Unsourced, T1, "EURONEXT-TC-2012"),
        // 2012-12-26 - T1 - EURONEXT-TC-2012 - the banked 2012 edition is the `Calendar of Cash Business Days` notice, whose scope is the cash markets; the derivatives arrangement for Boxing Day is not stated in any banked artifact, so the candidate date's status is withheld.
        (2012, 12, 26, Unsourced, T1, "EURONEXT-TC-2012"),
        // 2012-12-31 - T1 - EURONEXT-TC-2012 - the banked 2012 edition is the `Calendar of Cash Business Days` notice, whose scope is the cash markets; the derivatives arrangement for New Year's Eve half day in the cash scope is not stated in any banked artifact, so the candidate date's status is withheld.
        (2012, 12, 31, Unsourced, T1, "EURONEXT-TC-2012"),
        // 2013-01-01 - T1 - EURONEXT-IF-2013 - the banked 2013 editions state the cash-markets scope; the derivatives arrangement for New Year's Day is not stated in any banked artifact, so the candidate date's status is withheld.
        (2013, 1, 1, Unsourced, T1, "EURONEXT-IF-2013"),
        // 2013-03-29 - T1 - EURONEXT-IF-2013 - the banked 2013 editions state the cash-markets scope; the derivatives arrangement for Good Friday is not stated in any banked artifact, so the candidate date's status is withheld.
        (2013, 3, 29, Unsourced, T1, "EURONEXT-IF-2013"),
        // 2013-04-01 - T1 - EURONEXT-IF-2013 - the banked 2013 editions state the cash-markets scope; the derivatives arrangement for Easter Monday is not stated in any banked artifact, so the candidate date's status is withheld.
        (2013, 4, 1, Unsourced, T1, "EURONEXT-IF-2013"),
        // 2013-05-01 - T1 - EURONEXT-IF-2013 - the banked 2013 editions state the cash-markets scope; the derivatives arrangement for Labour Day is not stated in any banked artifact, so the candidate date's status is withheld.
        (2013, 5, 1, Unsourced, T1, "EURONEXT-IF-2013"),
        // 2013-12-24 - T1 - EURONEXT-IF-2013 - the banked 2013 editions state the cash-markets scope; the derivatives arrangement for Christmas Eve half day in the cash scope is not stated in any banked artifact, so the candidate date's status is withheld.
        (2013, 12, 24, Unsourced, T1, "EURONEXT-IF-2013"),
        // 2013-12-25 - T1 - EURONEXT-IF-2013 - the banked 2013 editions state the cash-markets scope; the derivatives arrangement for Christmas Day is not stated in any banked artifact, so the candidate date's status is withheld.
        (2013, 12, 25, Unsourced, T1, "EURONEXT-IF-2013"),
        // 2013-12-26 - T1 - EURONEXT-IF-2013 - the banked 2013 editions state the cash-markets scope; the derivatives arrangement for Boxing Day is not stated in any banked artifact, so the candidate date's status is withheld.
        (2013, 12, 26, Unsourced, T1, "EURONEXT-IF-2013"),
        // 2013-12-31 - T1 - EURONEXT-IF-2013 - the banked 2013 editions state the cash-markets scope; the derivatives arrangement for New Year's Eve half day in the cash scope is not stated in any banked artifact, so the candidate date's status is withheld.
        (2013, 12, 31, Unsourced, T1, "EURONEXT-IF-2013"),
        // 2014-01-01 - T1 - EURONEXT-HH-2014-01-12 - New Year's Day.
        (2014, 1, 1, Closed, T1, "EURONEXT-HH-2014-01-12"),
        // 2014-04-18 - T1 - EURONEXT-HH-2014-01-12 - Good Friday.
        (2014, 4, 18, Closed, T1, "EURONEXT-HH-2014-01-12"),
        // 2014-04-21 - T1 - EURONEXT-HH-2014-01-12 - Easter Monday.
        (2014, 4, 21, Closed, T1, "EURONEXT-HH-2014-01-12"),
        // 2014-05-01 - T1 - EURONEXT-HH-2014-01-12 - Labour Day.
        (2014, 5, 1, Closed, T1, "EURONEXT-HH-2014-01-12"),
        // 2014-12-24 - T1 - EURONEXT-HH-2014-01-12 - `Contracts on the Derivatives markets will close at various times, commencing at midday` — no per-product instant.
        (2014, 12, 24, Unsourced, T1, "EURONEXT-HH-2014-01-12"),
        // 2014-12-25 - T1 - EURONEXT-HH-2014-01-12 - Christmas Day.
        (2014, 12, 25, Closed, T1, "EURONEXT-HH-2014-01-12"),
        // 2014-12-26 - T1 - EURONEXT-HH-2014-01-12 - Boxing Day.
        (2014, 12, 26, Closed, T1, "EURONEXT-HH-2014-01-12"),
        // 2014-12-31 - T1 - EURONEXT-HH-2014-01-12 - the same derivatives various-times sentence.
        (2014, 12, 31, Unsourced, T1, "EURONEXT-HH-2014-01-12"),
        // 2015-01-01 - T1 - EURONEXT-HH-2015-03-22 - New Year's Day.
        (2015, 1, 1, Closed, T1, "EURONEXT-HH-2015-03-22"),
        // 2015-04-03 - T1 - EURONEXT-HH-2015-03-22 - Good Friday.
        (2015, 4, 3, Closed, T1, "EURONEXT-HH-2015-03-22"),
        // 2015-04-06 - T1 - EURONEXT-HH-2015-03-22 - Easter Monday.
        (2015, 4, 6, Closed, T1, "EURONEXT-HH-2015-03-22"),
        // 2015-05-01 - T1 - EURONEXT-HH-2015-03-22 - Labour Day.
        (2015, 5, 1, Closed, T1, "EURONEXT-HH-2015-03-22"),
        // 2015-12-24 - T1 - EURONEXT-HH-2015-03-22 - `Contracts on the Derivatives markets will close at various times, commencing at 13:55 CET`.
        (2015, 12, 24, Unsourced, T1, "EURONEXT-HH-2015-03-22"),
        // 2015-12-25 - T1 - EURONEXT-HH-2015-03-22 - Christmas Day.
        (2015, 12, 25, Closed, T1, "EURONEXT-HH-2015-03-22"),
        // 2015-12-31 - T1 - EURONEXT-HH-2015-03-22 - New Year's Eve, in the calendar's own closure list.
        (2015, 12, 31, Closed, T1, "EURONEXT-HH-2015-03-22"),
        // 2016-01-01 - T1 - EURONEXT-HH-2016-03-04 - New Year's Day.
        (2016, 1, 1, Closed, T1, "EURONEXT-HH-2016-03-04"),
        // 2016-03-25 - T1 - EURONEXT-HH-2016-03-04 - Good Friday.
        (2016, 3, 25, Closed, T1, "EURONEXT-HH-2016-03-04"),
        // 2016-03-28 - T1 - EURONEXT-HH-2016-03-04 - Easter Monday.
        (2016, 3, 28, Closed, T1, "EURONEXT-HH-2016-03-04"),
        // 2016-12-26 - T1 - EURONEXT-HH-2016-03-04 - Boxing Day; 23 and 30 December close `at the usual times` for the Cash and Derivatives markets, so no eve rows.
        (2016, 12, 26, Closed, T1, "EURONEXT-HH-2016-03-04"),
        // 2017-04-14 - T1 - EURONEXT-HH-2017-08-05 - Good Friday.
        (2017, 4, 14, Closed, T1, "EURONEXT-HH-2017-08-05"),
        // 2017-04-17 - T1 - EURONEXT-HH-2017-08-05 - Easter Monday.
        (2017, 4, 17, Closed, T1, "EURONEXT-HH-2017-08-05"),
        // 2017-05-01 - T1 - EURONEXT-HH-2017-08-05 - Labour Day; 1 January fell on a Sunday and the calendar prints no substitute.
        (2017, 5, 1, Closed, T1, "EURONEXT-HH-2017-08-05"),
        // 2017-12-25 - T1 - EURONEXT-HH-2017-08-05 - Christmas Day.
        (2017, 12, 25, Closed, T1, "EURONEXT-HH-2017-08-05"),
        // 2017-12-26 - T1 - EURONEXT-HH-2017-08-05 - Boxing Day; 22 and 29 December close `at the usual times` for the Cash and Derivatives markets, so no eve rows.
        (2017, 12, 26, Closed, T1, "EURONEXT-HH-2017-08-05"),
        // 2018-01-01 - T1 - EURONEXT-HH-2018-08-26 - New Year's Day.
        (2018, 1, 1, Closed, T1, "EURONEXT-HH-2018-08-26"),
        // 2018-03-30 - T1 - EURONEXT-HH-2018-08-26 - Good Friday.
        (2018, 3, 30, Closed, T1, "EURONEXT-HH-2018-08-26"),
        // 2018-04-02 - T1 - EURONEXT-HH-2018-08-26 - Easter Monday.
        (2018, 4, 2, Closed, T1, "EURONEXT-HH-2018-08-26"),
        // 2018-05-01 - T1 - EURONEXT-HH-2018-08-26 - Labour Day.
        (2018, 5, 1, Closed, T1, "EURONEXT-HH-2018-08-26"),
        // 2018-12-24 - T1 - EURONEXT-HH-2018-08-26 - `Contracts on the Derivatives markets will close at various times` — no per-product instant.
        (2018, 12, 24, Unsourced, T1, "EURONEXT-HH-2018-08-26"),
        // 2018-12-25 - T1 - EURONEXT-HH-2018-08-26 - Christmas Day.
        (2018, 12, 25, Closed, T1, "EURONEXT-HH-2018-08-26"),
        // 2018-12-26 - T1 - EURONEXT-HH-2018-08-26 - Boxing Day.
        (2018, 12, 26, Closed, T1, "EURONEXT-HH-2018-08-26"),
        // 2018-12-31 - T1 - EURONEXT-HH-2018-08-26 - the same derivatives various-times sentence.
        (2018, 12, 31, Unsourced, T1, "EURONEXT-HH-2018-08-26"),
        // 2019-01-01 - T1 - EURONEXT-HH-2019-03-25 - New Year's Day.
        (2019, 1, 1, Closed, T1, "EURONEXT-HH-2019-03-25"),
        // 2019-04-19 - T1 - EURONEXT-HH-2019-03-25 - Good Friday.
        (2019, 4, 19, Closed, T1, "EURONEXT-HH-2019-03-25"),
        // 2019-04-22 - T1 - EURONEXT-HH-2019-03-25 - Easter Monday.
        (2019, 4, 22, Closed, T1, "EURONEXT-HH-2019-03-25"),
        // 2019-05-01 - T1 - EURONEXT-HH-2019-03-25 - Labour Day.
        (2019, 5, 1, Closed, T1, "EURONEXT-HH-2019-03-25"),
        // 2019-12-24 - T1 - EURONEXT-HH-2019-12-10 - `Contracts on the Derivatives Markets will close at various times, commencing at 13:55 CET`.
        (2019, 12, 24, Unsourced, T1, "EURONEXT-HH-2019-12-10"),
        // 2019-12-25 - T1 - EURONEXT-HH-2019-03-25 - Christmas Day.
        (2019, 12, 25, Closed, T1, "EURONEXT-HH-2019-03-25"),
        // 2019-12-26 - T1 - EURONEXT-HH-2019-03-25 - Boxing Day.
        (2019, 12, 26, Closed, T1, "EURONEXT-HH-2019-03-25"),
        // 2019-12-31 - T1 - EURONEXT-HH-2019-12-10 - the same derivatives various-times sentence.
        (2019, 12, 31, Unsourced, T1, "EURONEXT-HH-2019-12-10"),
        // 2020-01-01 - T1 - EURONEXT-HH-2019-12-10 - New Year's Day.
        (2020, 1, 1, Closed, T1, "EURONEXT-HH-2019-12-10"),
        // 2020-04-10 - T1 - EURONEXT-HH-2019-12-10 - Good Friday.
        (2020, 4, 10, Closed, T1, "EURONEXT-HH-2019-12-10"),
        // 2020-04-13 - T1 - EURONEXT-HH-2019-12-10 - Easter Monday.
        (2020, 4, 13, Closed, T1, "EURONEXT-HH-2019-12-10"),
        // 2020-05-01 - T1 - EURONEXT-HH-2019-12-10 - Labour Day.
        (2020, 5, 1, Closed, T1, "EURONEXT-HH-2019-12-10"),
        // 2020-12-24 - T1 - EURONEXT-HH-2019-12-10 - `Contracts on the Derivatives Markets will close at various times, commencing at 13:55 CET`; the detailed instants live in the operator's derivatives appendix, which is not banked.
        (2020, 12, 24, Unsourced, T1, "EURONEXT-HH-2019-12-10"),
        // 2020-12-25 - T1 - EURONEXT-HH-2019-12-10 - Christmas Day.
        (2020, 12, 25, Closed, T1, "EURONEXT-HH-2019-12-10"),
        // 2020-12-31 - T1 - EURONEXT-HH-2019-12-10 - the same derivatives various-times sentence.
        (2020, 12, 31, Unsourced, T1, "EURONEXT-HH-2019-12-10"),
        // 2021-01-01 - T1 - EURONEXT-HH-2020-12-05 - New Year's Day; the Dublin and Oslo rows print `Full Day Trading` for Paris.
        (2021, 1, 1, Closed, T1, "EURONEXT-HH-2020-12-05"),
        // 2021-04-02 - T1 - EURONEXT-HH-2020-12-05 - Good Friday.
        (2021, 4, 2, Closed, T1, "EURONEXT-HH-2020-12-05"),
        // 2021-04-05 - T1 - EURONEXT-HH-2020-12-05 - Easter Monday.
        (2021, 4, 5, Closed, T1, "EURONEXT-HH-2020-12-05"),
        // 2021-12-24 - T1 - EURONEXT-HH-2020-12-05 - `Contracts on the Derivatives Markets will close at various times, starting at 13:55 CET`.
        (2021, 12, 24, Unsourced, T1, "EURONEXT-HH-2020-12-05"),
        // 2021-12-31 - T1 - EURONEXT-HH-2020-12-05 - the same derivatives various-times sentence.
        (2021, 12, 31, Unsourced, T1, "EURONEXT-HH-2020-12-05"),
        // 2022-04-15 - T1 - EURONEXT-HH-2022-05-23 - Good Friday: the Paris cell of the per-market table.
        (2022, 4, 15, Closed, T1, "EURONEXT-HH-2022-05-23"),
        // 2022-04-18 - T1 - EURONEXT-HH-2022-05-23 - Easter Monday: the Paris cell.
        (2022, 4, 18, Closed, T1, "EURONEXT-HH-2022-05-23"),
        // 2022-12-26 - T1 - EURONEXT-HH-2022-05-23 - Stephen's Day / Boxing Day: the Paris cell; 12-23 and 12-30 print `Full Day Trading` for Paris (Dublin's substitutes).
        (2022, 12, 26, Closed, T1, "EURONEXT-HH-2022-05-23"),
        // 2023-04-07 - T1 - EURONEXT-HH-2023-11-27 - Good Friday: the Paris cell.
        (2023, 4, 7, Closed, T1, "EURONEXT-HH-2023-11-27"),
        // 2023-04-10 - T1 - EURONEXT-HH-2023-11-27 - Easter Monday: the Paris cell.
        (2023, 4, 10, Closed, T1, "EURONEXT-HH-2023-11-27"),
        // 2023-05-01 - T1 - EURONEXT-HH-2023-11-27 - May Day: the Paris cell.
        (2023, 5, 1, Closed, T1, "EURONEXT-HH-2023-11-27"),
        // 2023-12-25 - T1 - EURONEXT-HH-2023-11-27 - Christmas: the Paris cell.
        (2023, 12, 25, Closed, T1, "EURONEXT-HH-2023-11-27"),
        // 2023-12-26 - T1 - EURONEXT-HH-2023-11-27 - St Stephens Day / Boxing Day: the Paris cell; 12-22 and 12-29 print `Full Day Trading` for Paris.
        (2023, 12, 26, Closed, T1, "EURONEXT-HH-2023-11-27"),
        // 2024-01-01 - T1 - EURONEXT-HH-2023-11-27 - New Year's Day: the Paris cell.
        (2024, 1, 1, Closed, T1, "EURONEXT-HH-2023-11-27"),
        // 2024-03-29 - T1 - EURONEXT-HH-2023-11-27 - Good Friday: the Paris cell.
        (2024, 3, 29, Closed, T1, "EURONEXT-HH-2023-11-27"),
        // 2024-04-01 - T1 - EURONEXT-HH-2023-11-27 - Easter Monday: the Paris cell.
        (2024, 4, 1, Closed, T1, "EURONEXT-HH-2023-11-27"),
        // 2024-05-01 - T1 - EURONEXT-HH-2023-11-27 - Labour Day: the Paris cell.
        (2024, 5, 1, Closed, T1, "EURONEXT-HH-2023-11-27"),
        // 2024-12-24 - T1 - EURONEXT-HH-2023-11-27 - the Paris cell prints `**Half Trading Day`; the banked end-of-year appendix states the cash manual's Paris equity segments only, so no derivatives instant is claimed.
        (2024, 12, 24, Unsourced, T1, "EURONEXT-HH-2023-11-27"),
        // 2024-12-25 - T1 - EURONEXT-HH-2023-11-27 - Christmas: the Paris cell.
        (2024, 12, 25, Closed, T1, "EURONEXT-HH-2023-11-27"),
        // 2024-12-26 - T1 - EURONEXT-HH-2023-11-27 - St Stephens Day / Boxing Day: the Paris cell.
        (2024, 12, 26, Closed, T1, "EURONEXT-HH-2023-11-27"),
        // 2024-12-31 - T1 - EURONEXT-HH-2023-11-27 - the same `**Half Trading Day` Paris cell and unstated derivatives instants.
        (2024, 12, 31, Unsourced, T1, "EURONEXT-HH-2023-11-27"),
        // 2025-01-01 - T1 - EURONEXT-HH-2025-12-06 - New Year's Day: the Paris cell.
        (2025, 1, 1, Closed, T1, "EURONEXT-HH-2025-12-06"),
        // 2025-04-18 - T1 - EURONEXT-HH-2025-12-06 - Good Friday: the Paris cell.
        (2025, 4, 18, Closed, T1, "EURONEXT-HH-2025-12-06"),
        // 2025-04-21 - T1 - EURONEXT-HH-2025-12-06 - Easter Monday: the Paris cell.
        (2025, 4, 21, Closed, T1, "EURONEXT-HH-2025-12-06"),
        // 2025-05-01 - T1 - EURONEXT-HH-2025-12-06 - Labour Day: the Paris cell.
        (2025, 5, 1, Closed, T1, "EURONEXT-HH-2025-12-06"),
        // 2025-12-24 - T1 - EURONEXT-HH-2025-12-06 - the Paris cell prints `**Half Trading Day`; the banked appendix's 4-03 sheet states the fund/NAV facility only, so no derivatives instant is claimed.
        (2025, 12, 24, Unsourced, T1, "EURONEXT-HH-2025-12-06"),
        // 2025-12-25 - T1 - EURONEXT-HH-2025-12-06 - Christmas: the Paris cell.
        (2025, 12, 25, Closed, T1, "EURONEXT-HH-2025-12-06"),
        // 2025-12-26 - T1 - EURONEXT-HH-2025-12-06 - St Stephen's Day / Boxing Day: the Paris cell.
        (2025, 12, 26, Closed, T1, "EURONEXT-HH-2025-12-06"),
        // 2025-12-31 - T1 - EURONEXT-HH-2025-12-06 - the same `**Half Trading Day` Paris cell and unstated derivatives instants.
        (2025, 12, 31, Unsourced, T1, "EURONEXT-HH-2025-12-06"),
        // 2026-01-01 - T1 - EURONEXT-IF-2026 - New Year's Day: Paris `Closed`.
        (2026, 1, 1, Closed, T1, "EURONEXT-IF-2026"),
        // 2026-04-03 - T1 - EURONEXT-IF-2026 - Good Friday: Paris `Closed`.
        (2026, 4, 3, Closed, T1, "EURONEXT-IF-2026"),
        // 2026-04-06 - T1 - EURONEXT-IF-2026 - Easter Monday: Paris `Closed`.
        (2026, 4, 6, Closed, T1, "EURONEXT-IF-2026"),
        // 2026-05-01 - T1 - EURONEXT-IF-2026 - Labour Day: Paris `Closed`.
        (2026, 5, 1, Closed, T1, "EURONEXT-IF-2026"),
        // 2026-12-24 - T1 - EURONEXT-IF-2026 - Paris `**Half Trading Day` with the hours announced and unstated (`To be announced`); no instant is claimed.
        (2026, 12, 24, Unsourced, T1, "EURONEXT-IF-2026"),
        // 2026-12-25 - T1 - EURONEXT-IF-2026 - Christmas: Paris `Closed`.
        (2026, 12, 25, Closed, T1, "EURONEXT-IF-2026"),
        // 2026-12-31 - T1 - EURONEXT-IF-2026 - Paris `**Half Trading Day` with the hours announced and unstated; no instant is claimed.
        (2026, 12, 31, Unsourced, T1, "EURONEXT-IF-2026"),
    ],
};
