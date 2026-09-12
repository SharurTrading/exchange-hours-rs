<!-- SPDX-License-Identifier: MIT-0 -->

# `bse_india` — evidence

- **Kind:** `Exchange`
- **Owner module:** [`bse.rs`](../../src/calendar/schedules/equities/apac/bse.rs)
- **Source sets:** [`APAC-INDIA-CASH`](../schedules/sources.md#apac-india-cash)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

Venue envelope includes 2026 CAS-eligible and non-CAS states.

## Revision rows

- 2010-01-04 — T1 — BSE notice 20091217-15 — continuous open moves 09:55 → 09:00.
- 2010-10-18 — T1 — BSE notice 20101014-8 — call-auction pre-open 09:00–09:15 introduced; continuous trading starts 09:15.
- 2026-08-03 — T1 — SEBI circular 99122 — Closing Auction Session: CAS-eligible stocks enter CAS at 15:15 and it ends 15:35, transition runs to 15:50, post-close ends 16:00.

## Sources

Row review: 2026-08-22 (UTC) is the date the ledger row was last reviewed as a
whole. Per-source retrieval dates were not recorded before the 2026-09-12
migration; where a later targeted review, capture or document date is recorded
beside a source below, that date governs for that source, and dates are added
as each source is re-verified.

- <https://www.bseindia.com/markets/MarketInfo/DispNewNoticesCirculars?page=20101014-8> — BSE operating notice for the 2010-10-18 call-auction launch: Order Entry Period 9:00am–9:07/08am with "Random stoppage between 7th and 8th minute" and "No trades are executed"; Order Matching & Confirmation Period 9:08am–9:12am; Buffer Period 9:12am–9:15am. The same notice records that the pre-open and continuous sessions "will not run concurrently" and that pre-open-ineligible stocks only trade from 9:15am.
- <https://www.nseindia.com/static/products-services/equity-market-pre-open> — NSE pre-open page, documenting the identical structure and still current after the 2026-08-03 CAS cutover.
- <https://www.nseindia.com/static/products-services/closing-auction-session> — NSE Closing Auction Session page.
- <https://www.sebi.gov.in/legal/circulars/jan-2026/introduction-of-closing-auction-session-cas-in-the-equity-cash-segment-and-certain-modifications-in-the-pre-open-auction-session_99122.html> — SEBI circular 99122, introducing CAS effective 2026-08-03.
- <https://www.bseindia.com/markets/MarketInfo/DispNewNoticesCirculars?page=20260801-1> — BSE notice for the CAS cutover.
- <https://www.bseindia.com/downloads/UploadDocs/Notices/20260610-41/20260610-41.pdf> — BSE detailed operating guidelines, stating that securities not eligible for CAS "shall continue to be available for continuous trading till 3:30pm".
- <https://www.bseindia.com/markets/MarketInfo/DispNewNoticesCirculars?page=20091217-15> — BSE notice 20091217-15, the December-2009 announcement of the 09:55 → 09:00 move.
- <https://nsearchives.nseindia.com/content/press/17122009.htm> — NSE press release of 2009-12-17, the same joint move.
- <https://api.bseindia.com/BseIndiaAPI/api/GetNoticesDownload_ng/w?Notice_no=20031205-4> — BSE notice 20031205-4, establishing the 15:40 post-close start before the audit floor.

## Gaps and residual risks

- The 2026-08-03 CAS row is keyed to a SEBI circular, the regulator's own binding instrument, rather than to a BSE notice; the BSE notices restate it. Recorded here for the same reason as on the NSE row.
- BSE has no 2011 post-close cutover: its 15:40 start predates the audit floor, so the NSE-only 2011-10-03 row is deliberately absent from this timeline (LAW-NO-FABRICATED-DATES).
- The pre-open order-entry boundary is set at 09:07, the conservative edge of the pre-open envelope: BSE notice 20101014-8 ends the Order Entry Period with a "Random stoppage between 7th and 8th minute" in which "[n]o trades are executed", and the first execution comes in the Order Matching & Confirmation Period that the same notice starts at 09:08. The crate serves 09:07 so the envelope never opens later than the operator's earliest stated stoppage second; 09:07–09:15 stays `extended`.
- The 09:55 grid below 2010-01-04 is sourced by the December-2009 artifacts; nothing below the January-2010 floor is reviewed.
