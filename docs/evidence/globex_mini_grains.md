<!-- SPDX-License-Identifier: MIT-0 -->

# `globex_mini_grains` — evidence

- **Kind:** `MarketHoursKey`
- **Owner module:** [`mini_grains.rs`](../../src/calendar/schedules/futures/us/mini_grains.rs)
- **Source sets:** [`US-CME-GROUP`](../schedules/sources.md#us-cme-group)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

**Gap: order-entry** — every window in which a trade can print is dated and sourced; what is undated is a queue phase. CBOT mini-sized grain futures on one grid: Mini-Sized Corn `XC`, Mini-Sized Soybean `XK`, Mini-Sized Wheat `XW`, and Mini-Sized KC HRW Wheat `MKC`, whose 2014-03-23 listing is member catalog data, not a grid revision — SER-7044 launched it with the mini contracts' hours, and same-day September-2015 spec captures show MKC closing 13:45 while KC wheat closes 13:20. The family is not foldable into `globex_grains`: CME built the minis with a permanent 30-minute-later day close (13-092 footnote 1), the grids coincide only 2012-05-20..2012-09-15 and from 2022-10-02, and the minis skipped SER-7395R entirely. Every matching era is dated: the floor 09:30-13:45 with the 18:00-07:15 overnight (Globex notice 20100405's mini-specific block and the August-2010 mini specs, against February-April 2010 table cells reading 13:15, which are judged copy errors of the standard row — a page whose pit column says 13:45 cannot close electronic trading thirty minutes earlier — and both cells read 13:45 by 2010-08-24), the 2012-05-20 expansion to 17:00-14:00, the 2012-09-16 mini-only extension to 17:00-14:30 (advisory 20120904), the 2013-04-07 reduction to 19:00-07:45 plus 08:30-13:45 (GCC notice and 13-092), and the 2022-10-02 convergence to 08:30-13:20 (SER-9049 with Globex notice 20220905, one table row per product). **Partial for the order-entry record.** The floor queues come from notice 20100315's category line for "CBOT, KCBT and MGEX Grain Futures", which does not enumerate the minis — the first product-specific enumeration is CME's trading-hours table of 2011-09-27, with no onset day for the standalone 16:45 weekday slot it also shows, so that slot is omitted as on the standard grid. The 2012-05-20 era serves no queues because the queue switch is only bracketed. The 2012-09-16 era models only the PCP its advisory dates (14:40, to the 16:00 end of the same-day trading-hours page); the era's other queue slots have no stated onset day. From 2013-04-07 the queue set is the standard grains', dated by the same 2013-03-22 notice that names XC/XW/XK, and SER-9049 lists all three windows as "(unchanged)" for all four contracts in 2022. Every omission under-reports queueing and touches no matching window.

## Revision rows

- 2010-04-19 — T1 — CME Globex notice 20100405, mini-sized block — post-close Pre-Open re-anchors to 13:45:30 CT, 30 seconds after the mini day close.
- 2011-12-27 — T1 — CFTC filing rul120711cbot001 — Globex morning Pre-Open start moves from 07:15 to 08:00 CT.
- 2012-05-20 — T1 — CME market-data advisory 20120518 — 21-hour continuous session, 17:00-14:00 CT; no queue is served in this era.
- 2012-09-16 — T1 — CME market-data advisory 20120904 — mini-only extension to 14:30 CT with the 14:40-16:00 CT post-close Pre-Open.
- 2013-04-07 — T1 — CME GCC notice 2013-03-22 and CBOT Submission 13-092 — reduction to 19:00-07:45 CT plus 08:30-13:45 CT with the standard grains' queue set.
- 2013-08-18 — T1 — CME market-data advisory 20130812 — morning Pre-Open widened to 08:00-08:30 CT.
- 2022-10-02 — T1 — CME SER-9049 and Globex notice 20220905 — convergence on the standard grain grid, 08:30-13:20 CT, queues unchanged.

## Sources

- <https://web.archive.org/web/20190718051357id_/https://www.cmegroup.com/tools-information/lookups/advisories/ser/SER-7044.html> — CME SER-7044, Mini-Sized KC HRW Wheat launch with the mini contracts' hours.
- <https://web.archive.org/web/20150905115535id_/http://www.cmegroup.com/trading/agricultural/grain-and-oilseed/mini-sized-kc-hrw-wheat_contract_specifications.html> — CME Mini-Sized KC HRW Wheat contract specification — captured 2015-09-05, 08:30-13:45 CT.
- <https://web.archive.org/web/20150905192450id_/http://www.cmegroup.com/trading/agricultural/grain-and-oilseed/kc-wheat_contract_specifications.html> — CME KC HRW Wheat contract specification — captured the same day, 08:30-13:20 CT.
- <https://www.cftc.gov/stellent/groups/public/%40rulesandproducts/documents/ifdocs/rul031313cbot001.pdf> — CBOT Submission 13-092, footnote 1 on the permanent mini day-close premium.
- <https://web.archive.org/web/20190718173802id_/https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20100405.html> — CME Globex notice 20100405, mini-sized block — the floor 13:45 CT close.
- <https://web.archive.org/web/20100329154209id_/http://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20100315.html> — CME Globex notice 20100315, category line for CBOT, KCBT and MGEX Grain Futures queues.
- <https://web.archive.org/web/20100824064602id_/http://www.cmegroup.com/trading/agricultural/grain-and-oilseed/mini-sized-corn_contract_specifications.html> — CME Mini-Sized Corn contract specification — captured 2010-08-24, 09:30-13:45 CT.
- <https://web.archive.org/web/20110927011113id_/http://www.cmegroup.com/trading_hours/commodities-hours.html> — CME commodities trading-hours table — captured 2011-09-27, the first product-specific mini queue enumeration.
- <https://www.cftc.gov/stellent/groups/public/%40rulesandproducts/documents/ifdocs/rul120711cbot001.pdf> — CFTC filing rul120711cbot001 — the 2011-12-27 morning queue move.
- <https://web.archive.org/web/20120125164824id_/http://www.cmegroup.com/trading_hours/commodities-hours.html> — CME commodities trading-hours table — captured 2012-01-25.
- <https://web.archive.org/web/20190716055756id_/https://www.cmegroup.com/tools-information/lookups/advisories/market-data/20120518.html> — CME market-data advisory 20120518 — the 2012-05-20 expansion.
- <https://web.archive.org/web/20120616192606id_/http://www.cmegroup.com/trading/agricultural/grain-and-oilseed/mini-sized-corn_contract_specifications.html> — CME Mini-Sized Corn contract specification — captured 2012-06-16.
- <https://web.archive.org/web/20190716055549id_/https://www.cmegroup.com/tools-information/lookups/advisories/market-data/20120904.html> — CME market-data advisory 20120904 — the 2012-09-16 mini-only extension.
- <https://web.archive.org/web/20120915064618id_/http://www.cmegroup.com/trading_hours/commodities-hours.html> — CME commodities trading-hours table — captured 2012-09-15, the day before go-live.
- <https://web.archive.org/web/20130423023212id_/http://www.cmegroup.com/globex/files/cmegroup_reduced_grain_and_oilseed_hours.pdf> — CME Global Command Center notice of 22 March 2013, naming XC, XW and XK.
- <https://web.archive.org/web/20190822053114id_/https://www.cmegroup.com/tools-information/lookups/advisories/market-data/20130812.html> — CME market-data advisory 20130812 — the 2013-08-18 morning Pre-Open widening.
- <https://www.cmegroup.com/notices/ser/2022/09/SER-9049.pdf> — CME SER-9049 (2022-09-01) — the 2022-10-02 convergence, one table row per product.
- <https://web.archive.org/web/20260212002829id_/https://www.cmegroup.com/notices/electronic-trading/2022/09/20220905.html> — CME Globex notice 20220905 — the 2022-10-02 amendment.
- <https://web.archive.org/web/20221022130435id_/https://www.cmegroup.com/notices/electronic-trading/2022/09/20220912.html> — CME Globex notice 20220912 — restatement of the same amendment.
- <https://web.archive.org/web/20260411073852id_/https://www.cmegroup.com/articles/faqs/faq-micro-agriculture-futures.html> — CME Micro Agricultural futures FAQ — captured 2026-04-11, corroborating the current grid.
- <https://web.archive.org/web/20250328101242id_/https://www.cmegroup.com/trading/agricultural/files/grain-and-oilseed-futures-options-fact-card.pdf> — CME grain and oilseed fact card — captured 2025-03-28, stale 1:45 p.m. note, superseded.

## Gaps and residual risks

- **order-entry** — the January-2010 floor queue set comes from Globex notice 20100315's category line for "CBOT, KCBT and MGEX Grain Futures", which does not enumerate the minis; the first product-specific enumeration is CME's trading-hours table of 2011-09-27, with the same values. Closing condition: a CME artifact stating the mini queues in session language on a day-level effective date before 2011-09-27.
- **order-entry** — the standalone 16:45 CT weekday Pre-Open slot the mini rows show through the 2012-01-25 capture has no stated onset day and is omitted, matching the standard grains' treatment of the same slot.
- **order-entry** — the 2012-05-20 era serves no queues at all: the advisory states matching hours only, and CME's trading-hours pages bracket the queue switch to 2012-05-05..2012-06-15 without dating it.
- **order-entry** — the 2012-09-16 era models only the post-close Pre-Open its advisory dates (14:40 CT, to the 16:00 end the same-day trading-hours page shows); the era's other queue slots have no stated onset day.
- **horizon** — the modelled floor grid rests on CME Globex notice 20100405 (2010-04-05), the earliest mini-specific artifact stating the 13:45 CT close. CME's February-April 2010 trading-hours tables print 13:15 in the mini rows' Globex column against a 13:45 open-outcry column; those cells are judged copy errors of the standard-grain row, and both cells read 13:45 by 2010-08-24. No 13:15/13:45 transition is dated and none is asserted.
- **catalog, not schedule** — `MKC` joined on Sunday 2014-03-23 (trade date Monday 2014-03-24) on the then-current mini grid. SER-7044 launched it with "the same contract specifications ... trading hours ... as the existing CBOT Mini-Sized Corn, Soybean, and Chicago SRW Wheat futures", so it is a member listing, not a grid change, and creates no revision row.

## Module narrative (moved from src/calendar/schedules/futures/us/mini_grains.rs on 2026-09-12 UTC)

CBOT mini-sized grain and oilseed futures in America/Chicago: Mini-Sized
Corn (CME Globex `XC`, CBOT Rulebook chapter 10B), Mini-Sized Soybean
(`XK`, chapter 11B), Mini-Sized Wheat (`XW`, chapter 14B), and Mini-Sized
KC HRW Wheat (`MKC`, chapter 14N as of 2022). ONE family on ONE grid: the
2012 expansion notice names Mini-Sized Corn/Soybeans/Wheat together, the
22 March 2013 Global Command Center notice lists "XC Mini-Sized Corn
Futures", "XW Mini-Sized Wheat Futures" and "XK Mini-Sized Soybean
Futures" with a single mini-specific day close, SER-9049's amendment table
gives all four contracts identical current and amended hours, and no
retrieved document between 2010 and 2026 gives any two of them different
hours. Excluded: the standard-size ZC/ZS/ZW/KE grain and oilseed contracts
(`grains.rs`), Rough Rice (`rough_rice.rs`), and the Micro Ag futures
MZC/MZS/MZW, which launched February 2022 on the standard grid.

WHY THIS IS NOT FOLDABLE INTO `globex_grains`. CME built the minis with a
deliberate, permanent 30-minute-later day close so positions could be
offset after the standard settlement — CFTC filing 13-092 footnote 1,
"To maintain the traditional mini- to full-sized trading hours, both
electronic and floor hours for CBOT Mini-Sized Corn, Soybean, and Wheat
futures will close daily at 1:45 p.m. CT", and SER-7044, "because
mini-sized products have slightly longer trading hours and because mini-
and standard-sized products allow for offsets, the mini-sized products
continue to be a popular tool for adjusting positions at the end of each
trading day". The envelopes coincide only 2012-05-20..2012-09-15 and from
2022-10-02; between those windows the standard grain key serves a day
close 25 or 30 minutes too early for the minis. The minis skipped
SER-7395R's 2015-07-05 move to 13:20 entirely — the 2022 notice's CURRENT
column still reads 1:45 p.m. — so their revision list is NOT a copy of the
grains list: they have a 2012-09-16 revision the standard grains do not,
no 2015-07-05 revision, and a 2022-10-02 revision the standard grains do
not.

`MKC` FOLLOWS THE MINI GRID DESPITE ITS KC-WHEAT LINEAGE. On captures
taken the same day, 5 September 2015, the KC HRW Wheat spec reads
"Monday – Friday, 8:30 a.m. – 1:20 p.m. CT" while the Mini-Sized KC HRW
Wheat spec reads "Monday - Friday, 8:30 a.m. - 1:45 p.m. CT"; SER-7044
launched MKC with "the same contract specifications - ... trading hours
... - as the existing CBOT Mini-Sized Corn, Soybean, and Chicago SRW Wheat
futures". MKC joined on Sunday 2014-03-23 (trade date Monday 2014-03-24)
on the then-current mini grid; that is a member listing, not a grid
change, so it creates no revision row here — launch dates remain
caller-catalog data.
https://web.archive.org/web/20190718051357id_/https://www.cmegroup.com/tools-information/lookups/advisories/ser/SER-7044.html
https://web.archive.org/web/20150905115535id_/http://www.cmegroup.com/trading/agricultural/grain-and-oilseed/mini-sized-kc-hrw-wheat_contract_specifications.html
https://web.archive.org/web/20150905192450id_/http://www.cmegroup.com/trading/agricultural/grain-and-oilseed/kc-wheat_contract_specifications.html
https://www.cftc.gov/stellent/groups/public/%40rulesandproducts/documents/ifdocs/rul031313cbot001.pdf

THE JANUARY-2010 FLOOR. Day matching 09:30-13:45 around the 18:00-07:15
overnight leg. The 13:45 close is stated by Globex notice 20100405's
mini-specific block — "CBOT Mini-sized grain futures / Trading ends at
1:45 p.m. Central time (CT)" — inside a notice that describes the
then-current state ("This will not affect the end of trading times, as
described below") with three product blocks side by side: CBOT and KCBT
grains end 1:15 p.m., CBOT Mini-sized grain futures end 1:45 p.m., MGEX
1:30 p.m. The same notice therefore contradicts CME's own trading-hours
tables of February-April 2010 directly rather than merely outweighing
them: those tables show 13:15 in the mini rows' Globex column against a
13:45 open-outcry column, cells judged copied from the standard-grain row
in error — a page whose pit column says 13:45 cannot have an electronic
column closing thirty minutes earlier for a product CME describes as
electronic-and-floor — and by 24 August 2010 both cells read 13:45. The
notice is also corroborated by the August-2010 contract specs for all
three minis ("9:30 am - 1:45 pm Central Time, Sunday - Friday") and
retrospectively by 13-092's "traditional mini- to full-sized trading
hours". No 13:15<->13:45 transition is dated and none is asserted. Floor
queues (Sunday 16:15-18:00, weekday 07:15-09:30, PCP
14:30-16:00) come from Globex notice 20100315's category line "Current
Pre-Open for CBOT, KCBT and MGEX Grain Futures", which does not enumerate
the minis; the first product-specific enumeration is CME's trading-hours
table of 27 September 2011, with the same values. The mini rows there also
list a standalone 16:45 weekday pre-open slot through the 25 January 2012
capture; no notice dates its onset, so it is not modeled, matching the
standard grains' treatment of the same slot.
https://web.archive.org/web/20190718173802id_/https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20100405.html
https://web.archive.org/web/20100329154209id_/http://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20100315.html
https://web.archive.org/web/20100824064602id_/http://www.cmegroup.com/trading/agricultural/grain-and-oilseed/mini-sized-corn_contract_specifications.html
https://web.archive.org/web/20110927011113id_/http://www.cmegroup.com/trading_hours/commodities-hours.html

2010-04-19 moves the afternoon post-close pre-open to begin 30 seconds
after the mini day close — "CBOT Mini-sized grain futures ... Post-close
pre-open begins at 1:45.30 CT", "GTC and GTD orders may be entered,
modified and cancelled 1:45.30 - 4:00 p.m. CT" — the same notice that
moved the standard grains' PCP to 13:15:30, carrying a separate
mini-specific block. 2011-12-27 moves the Globex morning pre-opening start
from 07:15 to 08:00 for CBOT agricultural futures (CFTC filing
rul120711cbot001); CME's trading-hours table of 25 January 2012 corroborates
the mini row with "Pre-Open Weekday 14:30-16:00 / 16:45 / 08:00". Both
change only order-entry boundaries; matching is unchanged.
https://www.cftc.gov/stellent/groups/public/%40rulesandproducts/documents/ifdocs/rul120711cbot001.pdf
https://web.archive.org/web/20120125164824id_/http://www.cmegroup.com/trading_hours/commodities-hours.html

2012-05-20 joins the standard grains' 21-hour continuous session:
"Effective this Sunday, May 20 (trade date Monday, May 21), the electronic
trading hours on CME Globex for all CBOT Commodity, KCBT, and MGEX Grain
and Oilseed futures and options will be expanded to ... Sunday to Friday:
17:00 CT to 14:00 Central Time (CT)". The regular slice stays the
open-outcry window, which CME's tables show at 09:30-13:45 for the minis
through this era. The advisory states only matching hours, never queue
times, and CME's trading-hours pages bracket the queue switch inside
2012-05-05..2012-06-15 without proving it, so this era conservatively
serves no order-entry phases — exactly the standard grains' treatment of
the same undated switch.
https://web.archive.org/web/20190716055756id_/https://www.cmegroup.com/tools-information/lookups/advisories/market-data/20120518.html
https://web.archive.org/web/20120616192606id_/http://www.cmegroup.com/trading/agricultural/grain-and-oilseed/mini-sized-corn_contract_specifications.html

2012-09-16 IS THE DIVERGENCE: the minis alone extend to 14:30 while the
standard grains stay at 14:00. "Expanded CBOT Mini-Sized Grain Futures
Trading Hour Change / Effective Sunday, September 16 (trade date Monday,
September 17) ... Sunday to Friday: 17:00 CT to 14:30 Central Time (CT) /
Pause State: 14:30 (CT) / Close State: 14:35 (CT) / Post Close/Pre Open
(PCP)14:40 (CT) / Please note: During PCP only GT orders will be allowed
for these products." The PCP end is not in the advisory; CME's
trading-hours page captured 15 September 2012 — the day before go-live —
shows the mini rows at "14:40- 16:00, 16:45 - 17:00". Only the PCP is
modeled from this row: the Sunday and evening queue slots the same page
shows have no stated onset day, and GT-only acceptance is still order
entry, the classification the crate already gives the standard grains'
GT-order PCP of 2010-04-19. The pit window's move to 09:30-14:30 appears
only in that table, not in the advisory, so the 13:45-14:30 afternoon
slice stays classified extended; the open/closed envelope is the same
either way inside the continuous electronic session.
https://web.archive.org/web/20190716055549id_/https://www.cmegroup.com/tools-information/lookups/advisories/market-data/20120904.html
https://web.archive.org/web/20120915064618id_/http://www.cmegroup.com/trading_hours/commodities-hours.html

2013-04-07 is the great reduction, and it restores the named mini premium:
"Sunday-Friday: 19:00 to 07:45 CT / Monday-Friday: Break from 07:45 to
08:30 CT / Monday-Friday: 08:30 to 13:15 CT, Mini-Sized Grains: 08:30 to
13:45 CT" (Global Command Center notice of 22 March 2013, whose
impacted-product list names XC/XW/XK; certified in parallel by CBOT
Submission 13-092, effective "Sunday, April 7, 2013 for the Monday, April
8, 2013 trading date"). The same notice establishes the queue set:
"Pre-Opens (including MGEX): Sunday night: 16:00-19:00 CT / Monday-Thursday
night: 16:45-19:00 CT / Monday-Friday morning: 08:15-08:30 CT. Post Close
Pre-Open: Monday-Friday: 14:30-16:00 CT", plus a 07:45-08:15
cancellation-only slice inside the break that no order-entry rule models —
identical values to the standard grains' queues, which `grains.rs` encodes
from the same notice.
https://web.archive.org/web/20130423023212id_/http://www.cmegroup.com/globex/files/cmegroup_reduced_grain_and_oilseed_hours.pdf
https://www.cftc.gov/stellent/groups/public/%40rulesandproducts/documents/ifdocs/rul031313cbot001.pdf

2013-08-18 widens the morning Pre-Open to 08:00-08:30: "Effective Sunday,
August 18, 2013 (trade date Monday, August 19), the Pre-Open market hours
will be expanded to: Monday – Friday, 08:00 Central Time (CT) to 08:30 CT
for the following products: CBOT Grain and Oilseed futures and options
...", with "The PCP state will remain unchanged Monday – Friday, 14:30 CT
to 16:00 CT". SER-9049's Pre-Open column later lists the same three
product-specific pre-open windows for all four minis as "(unchanged)".
https://web.archive.org/web/20190822053114id_/https://www.cmegroup.com/tools-information/lookups/advisories/market-data/20130812.html

2022-10-02 IS THE CONVERGENCE, and it ends the premium: "Effective Sunday,
October 2 (trade date Monday, October 3), the trading hours for the
following CBOT mini-sized agriculture futures will be amended" from
"Sunday - Friday, 7:00 p.m. - 7:45 a.m. CT and Monday - Friday, 8:30 a.m. -
1:45 p.m. CT" to "Sunday - Friday, 7:00 p.m. - 7:45 a.m. CT and Monday -
Friday, 8:30 a.m. - 1:20 p.m. CT", one table row per product for XC, MKC,
XK and XW. SER-9049 (2022-09-01) is the dated CME report behind it and
states the intent outright: "The Amendments to the CME Globex hours of the
Contracts will align the trading hours of the Contracts with the
corresponding standard sized agricultural futures contracts", with
ClearPort and Pre-Open hours unchanged. From this day the mini grid equals
the standard grain grid in every phase, so the current tables below are
the standard grains' tables under mini names. Corroborated by CME's Micro
Agricultural futures FAQ, captured 11 April 2026: "On CME Globex, trades
may be entered on: Sunday – Friday: 7:00 p.m. – 7:45 a.m. CT and Monday –
Friday: 8:30 a.m. – 1:20 p.m. CT". A stale CME fact card still carrying
the pre-2022 "Mini-sized contracts close at 1:45 p.m. CT" note (capture of
28 March 2025) is superseded by both dated documents.
https://www.cmegroup.com/notices/ser/2022/09/SER-9049.pdf
https://web.archive.org/web/20260212002829id_/https://www.cmegroup.com/notices/electronic-trading/2022/09/20220905.html
https://web.archive.org/web/20221022130435id_/https://www.cmegroup.com/notices/electronic-trading/2022/09/20220912.html
https://web.archive.org/web/20260411073852id_/https://www.cmegroup.com/articles/faqs/faq-micro-agriculture-futures.html
https://web.archive.org/web/20250328101242id_/https://www.cmegroup.com/trading/agricultural/files/grain-and-oilseed-futures-options-fact-card.pdf

RTH/ETH CLASSIFICATION. No CME document states a literal
regular-versus-extended label for these products in any era. The split
follows the operator's own published columns: before 2013 the pages
separate "Open Outcry" from "Electronic Trading", and the pit window
(09:30-13:45, then 08:30-13:45 per 13-092 footnote 1's "both electronic
and floor hours") is regular; from the 2013 reduction the overnight leg is
extended and the 08:30 day session regular, exactly the standard grains'
reviewed encoding. The current tables are shared with `grains.rs` because
the 2022 convergence makes the grids genuinely identical, not because one
envelope was assumed to transfer — every pre-2022 era here is keyed to the
minis' own dated sources.

---

Revision evidence — each row's day-level effective date and the primary
source that states it (full quotations sit in the blocks above):
  2010-04-19 "CME Globex notice 20100405, mini-sized block"
    https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20100405.html
  2011-12-27 "CFTC filing rul120711cbot001"
    https://www.cftc.gov/stellent/groups/public/%40rulesandproducts/documents/ifdocs/rul120711cbot001.pdf
  2012-05-20 "CME market-data advisory 20120518"
    https://www.cmegroup.com/tools-information/lookups/advisories/market-data/20120518.html
  2012-09-16 "CME market-data advisory 20120904"
    https://www.cmegroup.com/tools-information/lookups/advisories/market-data/20120904.html
  2013-04-07 "CME GCC notice 2013-03-22 and CBOT Submission 13-092"
    https://www.cmegroup.com/globex/files/cmegroup_reduced_grain_and_oilseed_hours.pdf
    https://www.cftc.gov/stellent/groups/public/%40rulesandproducts/documents/ifdocs/rul031313cbot001.pdf
  2013-08-18 "CME market-data advisory 20130812"
    https://www.cmegroup.com/tools-information/lookups/advisories/market-data/20130812.html
  2022-10-02 "CME SER-9049 and Globex notice 20220905"
    https://www.cmegroup.com/notices/ser/2022/09/SER-9049.pdf
    https://www.cmegroup.com/notices/electronic-trading/2022/09/20220905.html
Evidence: docs/evidence/globex_mini_grains.md
