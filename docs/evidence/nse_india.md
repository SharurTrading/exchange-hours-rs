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

## Holidays

**Coverage:** 2025-01-01..2026-12-31 (inclusive trade dates; T1 throughout)

Two artifacts key the block, both the operator's own published trading-holiday lists served from NSE's asset host: the "NSE Holiday List 2025" banner (`NSE-HOL-2025`) and the "Trading Holiday List for Equity & Equity Derivatives, Calendar Year 2026" banner (`NSE-HOL-2026`). Both were read from Wayback `id_` replays of the `nsearchives.nseindia.com` URLs because the live site refused the session: on 2026-09-28 the `nseindia.com` web channel returned HTTP 403 and its `holiday-master` API empty bodies (two refusals each), while the archive holds verbatim captures of the banners. The `holiday-master?type=CM` capture the archive lists (`20260909065333`) stores an empty payload and is not evidence of anything.

The lists are the capital-market trading holidays — the envelope the `nse_india` identity models. Each banner prints `DATE / DAY / OCCASION` rows; each printed date's weekday as printed matches the civil calendar, and every row below is the printed date itself. The banners state no early close, late open or shortened session for any listed date, and every closure below is a full closure.

**Two dates are `Unsourced`, not closed — the Muhurat Trading sessions.** Each banner footnotes a special session whose instants the operator had not published in the captured artifact. The 2025 banner: `*Muhurat Trading will be conducted on Tuesday, October 21, 2025. Timings of Muhurat Trading shall be notified in due course` — the same day it lists as `21 Oct Tue Diwali Laxmi Pujan*`. The 2026 banner: `Muhurat Trading will be conducted on Sunday, November 08, 2026. Timings of Muhurat Trading shall be notified subsequently`. A `Closed` row on 2025-10-21 would deny the session the operator states exists there, and silence on 2026-11-08 would claim the audited-normal weekend closure the operator's announcement contradicts, so both dates ship `Unsourced`: the table withholds the whole day rather than stating an incomplete one. **Closing condition:** the operator's Muhurat Trading circular stating the session instants, at which point 2025-10-21 can become a replacement-blocks day (closed regular session plus the stated special block) and 2026-11-08 a stated special session. Press coverage of the 2025 session exists but is T4 and keys no row (LAW-PRIMARY-SOURCES).

### 2025

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2025-02-26 | closed | `26 Feb Wednesday Mahashivratri` | `NSE-HOL-2025` | T1 | Printed date 2025-02-26, a Wednesday |
| 2025-03-14 | closed | `14 Mar Friday Holi` | `NSE-HOL-2025` | T1 | Printed date 2025-03-14, a Friday |
| 2025-03-31 | closed | `31 Mar Monday Eid-ul-Fitr (Ramadan Eid)` | `NSE-HOL-2025` | T1 | Printed date 2025-03-31, a Monday |
| 2025-04-10 | closed | `10 Apr Thursday Shri Mahavir Jayanti` | `NSE-HOL-2025` | T1 | Printed date 2025-04-10, a Thursday |
| 2025-04-14 | closed | `14 Apr Monday Dr. Babasaheb Ambedkar Jayanti` | `NSE-HOL-2025` | T1 | Printed date 2025-04-14, a Monday |
| 2025-04-18 | closed | `18 Apr Friday Good Friday` | `NSE-HOL-2025` | T1 | Printed date 2025-04-18, a Friday |
| 2025-05-01 | closed | `01 May Thursday Maharashtra Day` | `NSE-HOL-2025` | T1 | Printed date 2025-05-01, a Thursday |
| 2025-08-15 | closed | `15 Aug Friday Independence Day` | `NSE-HOL-2025` | T1 | Printed date 2025-08-15, a Friday |
| 2025-08-27 | closed | `27 Aug Wednesday Ganesh Chaturthi` | `NSE-HOL-2025` | T1 | Printed date 2025-08-27, a Wednesday |
| 2025-10-02 | closed | `02 Oct Thursday Mahatma Gandhi Jayanti / Dussehra` | `NSE-HOL-2025` | T1 | Printed date 2025-10-02, a Thursday |
| 2025-10-21 | unsourced | `21 Oct Tuesday Diwali Laxmi Pujan*` with the footnote `Muhurat Trading will be conducted on Tuesday, October 21, 2025. Timings of Muhurat Trading shall be notified in due course` | `NSE-HOL-2025` | T1 | Printed date 2025-10-21, a Tuesday; the day's special session has no published instants, so the date is withheld rather than closed |
| 2025-10-22 | closed | `22 Oct Wednesday Diwali Balipratipada` | `NSE-HOL-2025` | T1 | Printed date 2025-10-22, a Wednesday |
| 2025-11-05 | closed | `05 Nov Wednesday Prakash Gurpurab Sri Guru Nanak Dev` | `NSE-HOL-2025` | T1 | Printed date 2025-11-05, a Wednesday |
| 2025-12-25 | closed | `25 Dec Thursday Christmas` | `NSE-HOL-2025` | T1 | Printed date 2025-12-25, a Thursday |

### 2026

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2026-01-26 | closed | `26th JAN MONDAY Republic Day` | `NSE-HOL-2026` | T1 | Printed date 2026-01-26, a Monday |
| 2026-03-03 | closed | `03rd MAR TUESDAY Holi` | `NSE-HOL-2026` | T1 | Printed date 2026-03-03, a Tuesday |
| 2026-03-26 | closed | `26th MAR THURSDAY Shri Ram Navami` | `NSE-HOL-2026` | T1 | Printed date 2026-03-26, a Thursday |
| 2026-03-31 | closed | `31st MAR TUESDAY Shri Mahavir Jayanti` | `NSE-HOL-2026` | T1 | Printed date 2026-03-31, a Tuesday |
| 2026-04-03 | closed | `03rd APR FRIDAY Good Friday` | `NSE-HOL-2026` | T1 | Printed date 2026-04-03, a Friday |
| 2026-04-14 | closed | `14th APR TUESDAY Dr. Baba Saheb Ambedkar Jayanti` | `NSE-HOL-2026` | T1 | Printed date 2026-04-14, a Tuesday |
| 2026-05-01 | closed | `01st MAY FRIDAY Maharashtra Day` | `NSE-HOL-2026` | T1 | Printed date 2026-05-01, a Friday |
| 2026-05-28 | closed | `28th MAY THURSDAY Bakri Id` | `NSE-HOL-2026` | T1 | Printed date 2026-05-28, a Thursday |
| 2026-06-26 | closed | `26th JUN FRIDAY Muharram` | `NSE-HOL-2026` | T1 | Printed date 2026-06-26, a Friday |
| 2026-09-14 | closed | `14th SEP MONDAY Ganesh Chaturthi` | `NSE-HOL-2026` | T1 | Printed date 2026-09-14, a Monday |
| 2026-10-02 | closed | `02nd OCT FRIDAY Mahatma Gandhi Jayanti` | `NSE-HOL-2026` | T1 | Printed date 2026-10-02, a Friday |
| 2026-10-20 | closed | `20th OCT TUESDAY Dussehra` | `NSE-HOL-2026` | T1 | Printed date 2026-10-20, a Tuesday |
| 2026-11-08 | unsourced | the footnote `Muhurat Trading will be conducted on Sunday, November 08, 2026. Timings of Muhurat Trading shall be notified subsequently` | `NSE-HOL-2026` | T1 | Printed Muhurat date 2026-11-08, a Sunday; the announced session has no published instants, so the day is not the audited-normal weekend closure the normal week alone would claim |
| 2026-11-10 | closed | `10th NOV TUESDAY Diwali-Balipratipada` | `NSE-HOL-2026` | T1 | Printed date 2026-11-10, a Tuesday |
| 2026-11-24 | closed | `24th NOV TUESDAY Prakash Gurpurab Sri Guru Nanak Dev` | `NSE-HOL-2026` | T1 | Printed date 2026-11-24, a Tuesday |
| 2026-12-25 | closed | `25th DEC FRIDAY Christmas` | `NSE-HOL-2026` | T1 | Printed date 2026-12-25, a Friday |

### Documents

| Document | Window | Replay or service URL | Capture or retrieval, UTC | Tier | sha256 |
|---|---|---|---|---|---|
| `NSE-HOL-2025` | 2025-01-01..2026-12-31 | <https://web.archive.org/web/20241223224845id_/https://nsearchives.nseindia.com/web/sites/default/files/2024-12/Holiday%20List%20Web%20Banner.jpg> | retrieved 2026-09-28 UTC (Wayback capture 2024-12-23) | T1 | `22e29e5a236591c9dd64e9bf1788703da77d9d4934dbbc617668618668e3a115` |
| `NSE-HOL-2026` | 2025-01-01..2026-12-31 | <https://web.archive.org/web/20260108142249id_/https://nsearchives.nseindia.com/web/banner/2025-12/TradingHolidayList_636x555__final_20251218152820.jpg?w=1200> | retrieved 2026-09-28 UTC (Wayback capture 2026-01-08; banner file dated 2025-12-18) | T1 | `4920fdb026e210d0badc135577871cd6383559fe82d93d164c830b9ffd04a89e` |
| `NSE-TIMINGS-2025` | 2025-01-01..2026-12-31 | <https://web.archive.org/web/20250821072044id_/https://www.nseindia.com/api/cmsNote?url=exchange-communication-holidays-equities> | retrieved 2026-09-28 UTC (Wayback capture 2025-08-21) | T2 | `cb5f2beb9e685a58b4ba9cc1ad177912983ef88057c4aaac205c8eb256888a66` |
| `NSE-TIMINGS-2026` | 2025-01-01..2026-12-31 | <https://web.archive.org/web/20260916045525id_/https://www.nseindia.com/api/getNotes20?url=/resources/exchange-communication-holidays-equities> | retrieved 2026-09-28 UTC (Wayback capture 2026-09-16) | T2 | `20c69a1ca6fd37a969c3bb54e665bacb9b4844ffad4b1b7236feae7dbbd13f49` |

All four artifacts are saved in the research store under `holidays/raw/equities/nse_india/2025-2027/` with an `INDEX.md` carrying the same digests. The two `NSE-TIMINGS` captures are the operator's own machine channel stating the market-timings grid, not the holiday dates; they corroborate that the session grid inside this window is the one the normal-week profile already models (pre-open 09:00, continuous 09:15–15:30, post-close to 16:00, with the CAS phases from the 2026 capture) and key no holiday row. Both are stored gzip-compressed exactly as replayed.

**Why the window stops at 2026-12-31 is the operator's horizon, not a withholding.** NSE publishes the next year's list each December (the 2026 banner's file name is dated 2025-12-18); no "Trading Holiday List — Calendar Year 2027" exists as of the 2026-09-28 retrieval, and web search found none. **Closing condition:** NSE's 2027 list publication, which extends the window to 2027-12-31. Re-checked monthly per LAW-WATCH.

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

- **The two Muhurat Trading dates are withheld `Unsourced` rows** (2025-10-21 and 2026-11-08), the only withheld dates in the audited window. The closure on the first is sourced; what is missing is the special session's instants, which the captured banners state are "notified/subsequently". Closing condition: the operator's Muhurat circular, which turns the first into a replacement-blocks day and the second into a stated special session. Queries inside the window on those two dates refuse rather than answer, which is the honest failure mode (LAW-HOLIDAY-SCOPE).
- **The 2010-2024 holiday era is unmodelled.** The holiday table above was built from the two current annual banners alone, per this wave's bounded scope, so dates before 2025-01-01 sit outside every audited window and the identity refuses them rather than claiming a holiday answer. What would close it: the archived annual lists (NSE holiday-list banners or circulars per year, the pre-2025 captures the same channels hold) worked up the same way.
- **Retrieval channels.** The live `nseindia.com` channel refused both its HTML pages (HTTP 403) and the `holiday-master` API (empty bodies) twice on 2026-09-28; the citations above are Wayback `id_` replays of the operator's own artifacts, and the `archives.nseindia.com` host no longer serves the legacy holiday pages (404). A future review with browser-grade retrieval should prefer the live pages and re-verify the digests there.
- **The 2025 banner's scope sentence differs from the 2026 one.** The 2026 banner is headed "Trading Holiday List for Equity & Equity Derivatives"; the 2025 banner is headed "NSE Holiday List 2025" without the segment line. Both are the exchange's single published trading-holiday list for the year and no date in either is stated to differ between the cash and derivatives segments; if NSE resumes publishing separate per-segment lists, the cash list replaces this table's basis.
- **Raised in review of the ledger-reshape PR (#87), 2026-09-12 — the NSE-wide 15:15–15:35 classification rests on a BSE statement.** The bullet above keeps the venue-wide 15:15–15:35 CAS window `extended` on the ground that non-CAS-eligible stocks keep trading continuously through it. The artifact that states that is BSE's, not NSE's: BSE's detailed operating guidelines of 2026-06-10 say securities not eligible for CAS "shall continue to be available for continuous trading till 3:30pm". SEBI circular 99122 introduces CAS for the cash segment but the reviewed set holds no NSE or SEBI text stating, for **NSE** securities outside CAS, that continuous trading runs through 15:15–15:35. The classification is therefore read across from a sibling venue's notice. The reshape PR moved this text out of the owner module and changed no schedule rule, revision row, profile or routing; the NSE window is served exactly as before. Closing condition: an NSE circular or SEBI text covering non-CAS-eligible **NSE** securities in that window, which would either confirm the `extended` classification or turn the window into an `order_entry` phase for this venue. Dormant identity, so recorded here rather than opened as an issue (LAW-FOLLOW-UPS-ARE-ISSUES).
- The 2026-08-03 CAS row is keyed to a SEBI circular, the regulator's own binding instrument, rather than to an NSE circular. The NSE Closing Auction Session page restates it. Recorded here because the tier of a regulator instrument is not the venue's own statement in the strict reading of LAW-PRIMARY-SOURCES.
- The pre-open order-entry boundary is set at 09:07, the earliest second a trade could print under the random closure between the 7th and 8th minute, never later; 09:07–09:15 stays `extended` so the auction match, its trade confirmations and the transition buffer remain tradeable.
- CAS 15:15–15:35 contains order-entry-only sub-phases for CAS-eligible stocks, but the venue-wide window stays `extended` because non-eligible stocks continue continuous trading through it.
- The 09:55 grid below 2010-01-04 is sourced by the December-2009 artifacts; nothing below the January-2010 floor is reviewed.
