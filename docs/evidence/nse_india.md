<!-- SPDX-License-Identifier: MIT-0 -->

# `nse_india` — evidence

- **Kind:** `Exchange`
- **Owner module:** [`nse.rs`](../../src/calendar/schedules/equities/apac/nse.rs)
- **Source sets:** [`APAC-INDIA-CASH`](../schedules/sources.md#apac-india-cash)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

Venue envelope includes 2026 CAS-eligible and non-CAS states.

## Revision rows

- 2010-01-04 — T1 — NSE press release 17122009 — continuous open moves 09:55 → 09:00.
- 2010-10-18 — T1 — NSE circular NSE/CMTR/15981 — pre-open call auction 09:00–09:15 introduced; continuous trading starts 09:15.
- 2011-10-03 — T1 — NSE circular NSE/CMTR/19013 — post-close start moves 15:50 → 15:40.
- 2026-08-03 — T1 — SEBI circular 99122 — Closing Auction Session: CAS-eligible stocks enter CAS at 15:15 and it ends 15:35, transition runs to 15:50, post-close ends 16:00.

## Sources

Row review: 2026-08-22 (UTC) is the date the ledger row was last reviewed as a
whole. Per-source retrieval dates were not recorded before the 2026-09-12
migration; where a later targeted review, capture or document date is recorded
beside a source below, that date governs for that source, and dates are added
as each source is re-verified.

- <https://www.nseindia.com/static/products-services/equity-market-pre-open> — NSE pre-open page. The session "is comprised of Order collection period and order matching period"; "the order collection period of 8* minutes shall be provided for order entry, modification and cancellation (* - System driven random closure between 7th and 8th minute)"; "order matching period starts immediately after completion of order collection period".
- <https://www.bseindia.com/markets/MarketInfo/DispNewNoticesCirculars?page=20101014-8> — BSE's operating notice for the same 2010-10-18 launch, printing the grid outright: Order Entry Period 9:00am–9:07/08am with "No trades are executed", Order Matching & Confirmation Period 9:08am–9:12am, Buffer Period 9:12am–9:15am.
- <https://www.nseindia.com/static/products-services/closing-auction-session> — NSE Closing Auction Session page.
- <https://www.sebi.gov.in/legal/circulars/jan-2026/introduction-of-closing-auction-session-cas-in-the-equity-cash-segment-and-certain-modifications-in-the-pre-open-auction-session_99122.html> — SEBI circular 99122, introducing CAS effective 2026-08-03.
- <https://www.bseindia.com/markets/MarketInfo/DispNewNoticesCirculars?page=20260801-1> — BSE notice for the CAS cutover.
- <https://www.bseindia.com/downloads/UploadDocs/Notices/20260610-41/20260610-41.pdf> — BSE detailed operating guidelines, stating that securities not eligible for CAS "shall continue to be available for continuous trading till 3:30pm", which is why the whole 15:15–15:35 window stays tradeable venue-wide.
- <https://nsearchives.nseindia.com/global/content/about_us/NSEIL_Annual_Report_2011.pdf> — NSE annual report 2011, covering circular NSE/CMTR/15981.
- <https://nsearchives.nseindia.com/content/circulars/cmtr19013.pdf> — NSE circular NSE/CMTR/19013, the 2011-10-03 post-close change.
- <https://www.bseindia.com/markets/MarketInfo/DispNewNoticesCirculars?page=20091217-15> — BSE notice 20091217-15, the joint December-2009 announcement of the 09:55 → 09:00 move.
- <https://nsearchives.nseindia.com/content/press/17122009.htm> — NSE press release of 2009-12-17, the same move.

## Gaps and residual risks

- **Raised in review of the ledger-reshape PR (#87), 2026-09-12 — the NSE-wide 15:15–15:35 classification rests on a BSE statement.** The bullet above keeps the venue-wide 15:15–15:35 CAS window `extended` on the ground that non-CAS-eligible stocks keep trading continuously through it. The artifact that states that is BSE's, not NSE's: BSE's detailed operating guidelines of 2026-06-10 say securities not eligible for CAS "shall continue to be available for continuous trading till 3:30pm". SEBI circular 99122 introduces CAS for the cash segment but the reviewed set holds no NSE or SEBI text stating, for **NSE** securities outside CAS, that continuous trading runs through 15:15–15:35. The classification is therefore read across from a sibling venue's notice. The reshape PR moved this text out of the owner module and changed no schedule rule, revision row, profile or routing; the NSE window is served exactly as before. Closing condition: an NSE circular or SEBI text covering non-CAS-eligible **NSE** securities in that window, which would either confirm the `extended` classification or turn the window into an `order_entry` phase for this venue. Dormant identity, so recorded here rather than opened as an issue (LAW-FOLLOW-UPS-ARE-ISSUES).
- The 2026-08-03 CAS row is keyed to a SEBI circular, the regulator's own binding instrument, rather than to an NSE circular. The NSE Closing Auction Session page restates it. Recorded here because the tier of a regulator instrument is not the venue's own statement in the strict reading of LAW-PRIMARY-SOURCES.
- The pre-open order-entry boundary is set at 09:07, the earliest second a trade could print under the random closure between the 7th and 8th minute, never later; 09:07–09:15 stays `extended` so the auction match, its trade confirmations and the transition buffer remain tradeable.
- CAS 15:15–15:35 contains order-entry-only sub-phases for CAS-eligible stocks, but the venue-wide window stays `extended` because non-eligible stocks continue continuous trading through it.
- The 09:55 grid below 2010-01-04 is sourced by the December-2009 artifacts; nothing below the January-2010 floor is reviewed.
