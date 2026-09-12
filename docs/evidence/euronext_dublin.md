<!-- SPDX-License-Identifier: MIT-0 -->

# `euronext_dublin` — evidence

- **Kind:** `Exchange`
- **Owner module:** [`dublin.rs`](../../src/calendar/schedules/equities/europe/euronext/dublin.rs)
- **Source sets:** [`EU-EURONEXT`](../schedules/sources.md#eu-euronext), [`EU-FESE-SECONDARY`](../schedules/sources.md#eu-fese-secondary)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

The operator's archived pre-floor timetable, successive ISE order-book models, and 2018 trading calendar establish the complete 06:30–17:15 legacy grid through the 2019 Optiq migration; the 2019 and 2023-03-20 revisions are date-aware. Randomized uncrosses use the documented conservative latest-edge envelope.

## Revision rows

- 2019-02-04 — T1 — Euronext Dublin Optiq migration press release — Dublin equities move to Optiq: 06:15 pre-opening, open after the latest 08:00:30 uncross, continuous to 16:28, closing-auction and Trading-at-Last envelope through 16:40.
- 2023-03-20 — T1 — Euronext Go-Live Weekend Guidelines — legacy-market pre-opening moves from 07:15 to 07:30 CET, that is 06:15 to 06:30 Dublin local time.

## Sources

Row review: 2026-08-22 (UTC) is the date the ledger row was last reviewed as a
whole. Per-source retrieval dates were not recorded before the 2026-09-12
migration; where a later targeted review, capture or document date is recorded
beside a source below, that date governs for that source, and dates are added
as each source is re-verified.

- <https://web.archive.org/web/20090930042026id_/http://www.ise.ie/index.asp?locID=311&docID=-1> — ISE's own archived trading-hours page, retrieved from the 2009-09-30 capture: the complete legacy grid before the January-2010 audit floor — pre-trading 06:30–07:50, opening auction to 08:00, continuous trading to 16:28, closing auction to 16:30, post-trading through 17:15.
- <https://web.archive.org/web/20121004024422id_/http://www.ise.ie/Membership-and-Trading/Market-Infrastructure/Trading/ISE-Xetra%C2%AE/Release-11-1/ISE_Xetra_Rel_11_1_Market_Model_090511.pdf> — ISE Xetra Release 11.1 market model.
- <https://web.archive.org/web/20120907001910id_/http://www.ise.ie/Membership-and-Trading/Market-Infrastructure/Trading/ISE-Xetra%C2%AE/Release-12/ISE_Xetra_Rel_12_Market_Model.pdf> — ISE Xetra Release 12 market model.
- <https://web.archive.org/web/20130517042005id_/http://ise.ie/Membership-and-Trading/Market-Infrastructure/Trading/ISE-Xetra%C2%AE/Release_13/ISE_Xetra_Market_Model_-_Release_13_0.pdf> — ISE Xetra Release 13 market model.
- <https://web.archive.org/web/20140718094950id_/http://ise.ie/Membership-and-Trading/Market-Infrastructure/Trading/ISE-Xetra%C2%AE/Release_14/ISE_Xetra_Rel_14_0_Market_Model.pdf> — ISE Xetra Release 14 market model.
- <https://web.archive.org/web/20150315070155id_/http://ise.ie/Products-Services/Trading-Members/Equity-Trading-Membership/ISE-Xetra-Release-15-Market-Model.pdf> — ISE Xetra Release 15 market model.
- <https://web.archive.org/web/20170301204221id_/http://www.ise.ie/Products-Services/Trading-Members/Equity-Trading-Membership/ISE%20Xetra%20Release%2016%20Market%20Model.pdf> — ISE Xetra Release 16 market model.
- <https://web.archive.org/web/20171029062447id_/http://www.ise.ie/Products-Services/Trading-Members/Equity-Trading-Membership/ISE-T7-Market-Model-Rel-5-0.pdf> — ISE T7 Release 5.0 market model.
- <https://web.archive.org/web/20171108073227id_/http://www.ise.ie/Products-Services/Trading-Members/Equity-Trading-Membership/T7%20Rel%206%20Market%20Model.pdf> — ISE T7 Release 6 market model. Each successive model retains the exact 06:30–17:15 grid and states that "the order book is only open for trading during auctions and continuous trading in the main trading phase".
- <https://web.archive.org/web/20181215004420id_/http://www.ise.ie/Products-Services/Trading-Members/Equity-Trading-Calendar-2018.pdf> — ISE official 2018 trading calendar, repeating the grid through the last full pre-Optiq year.
- <https://web.archive.org/web/20181215004420id_/http://www.ise.ie/Products-Services/Trading-Members/Equity-Trading-Membership/> — archived December-2018 operator page: identifies Release 7.0 as current, links its market model, and states that ISE T7 would remain live through 2019-02-01 before the Optiq migration on 2019-02-04.
- <https://www.centralbank.ie/docs/default-source/tns/about---tns/peer-reviews-and-reports/tns-1-11-imf-report-on-observance-of-standards-and-codes-on-securities-regulation.pdf?sfvrsn=2> — Central Bank of Ireland assessment, independently recording the same 07:50–16:30 order-book envelope.
- <https://www.euronext.com/sites/default/files/190204optiq_migration_dublin_press_release.pdf> — Euronext Dublin Optiq migration press release, 2019-02-04.
- <https://www.eurex.com/ex-en/find/circulars/Discontinuation-of-clearing-services-for-Irish-Stock-Exchange-Amendments-to-the-Clearing-Conditions-and-to-the-Price-List-of-Eurex-Clearing-AG-1391874> — Eurex Clearing circular, discontinuation of clearing services for the Irish Stock Exchange, corroborating the migration date.
- <https://live.euronext.com/sites/default/files/2021-07/Market%20Notice%20-%20Datalex%20Plc%20-%20Admission.pdf> — Euronext market notice, corroborating the post-migration Dublin grid.
- <https://web.archive.org/web/20191018025213id_/https://www.euronext.com/sites/default/files/2019-09/52118_Euronext-FAQ-2019_v07_0.pdf> — post-Optiq Euronext FAQ 2019, establishing the 06:15 pre-opening.
- <https://connect.euronext.com/sites/default/files/it-documentation/Go-Live%20Weekend%20Guidelines%20-%20Borsa%20Italiana%20Optiq%20Migration.pdf> — Euronext Go-Live Weekend Guidelines: the phase-one timetable shifts legacy-market pre-opening effective 2023-03-20 and gives 2023-03-27 for the Italian migration.
- <https://www.euronext.com/sites/default/files/2026-07/appendix%20to%20Euronext%20Instructions%204-01%204-03%20Trading%20Manuals_0.xlsx> — current appendix to Euronext Instructions 4-01/4-03 Trading Manuals, confirming the resulting principal-share grid and randomizing the opening uncrossing over the 30 seconds from 08:00:00 Dublin local time.
- <https://www.fese.eu/app/uploads/2024/07/trading-hours-2025-1.pdf> — FESE 2025 trading-hours table, `EU-FESE-SECONDARY`: corroboration only.

## Gaps and residual risks

- **Interpretive step, pre-Optiq tradeability.** Neither pre-trading nor post-trading matched an order-book trade under the ISE market models, but the same sourced record shows off-book reports were accepted in each phase, and prints occur wherever off-book reports are accepted. Both legs therefore stay in `extended`: demoting either to `order_entry` on this record would claim a window that can print cannot. The legacy profile consequently carries an empty `order_entry` slice.
- **Interpretive step, post-Optiq order entry.** Optiq pre-opening is a Call (order-accumulation) phase; the first order-book print of the day is the opening uncrossing, so only the accumulation leg is `order_entry`. The uncross, the closing uncrossing and Trading-at-Last all print and stay `extended`.
- **Interpretive step, randomized uncrosses.** Opening and closing uncrossings are randomized over 30 seconds; the profile uses the documented conservative latest edge (08:00:30 and 16:30:30).
- **Time zone.** Dublin retains its own IANA zone, `Europe::Dublin`, while Euronext's Central-European clock is translated one civil hour earlier locally.
- **Dormant identity.** Reviewed on demand; gaps are recorded here rather than as issues.
