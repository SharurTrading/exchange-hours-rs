<!-- SPDX-License-Identifier: MIT-0 -->

# `globex_interest_rates` — evidence

- **Kind:** `MarketHoursKey`
- **Owner module:** [`interest_rates.rs`](../../src/calendar/schedules/futures/us/interest_rates.rs)
- **Source sets:** [`US-CME-GROUP`](../schedules/sources.md#us-cme-group)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

**Gap: order-entry** — the trading session is sourced; what is undated is a queue or post-close phase in which no trade can print. CBOT Treasury/micro-Treasury, 30-Day Fed Funds, and CME SOFR standard grid. The January-2010 queues, 2010 weekday-queue revision, and 2011 matching-open revision are exact; the current Sunday queue is sourced but its 16:15→16:00 cutover day is unavailable — bracketed to 2012-05-28..2012-06-07 by CME's own trading-hours captures, with both CME notice channels read in full across that window and silent on it. Dated profiles now carry the sourced Sunday 16:15–17:00 intersection from the January-2010 floor, so only the 16:00–16:15 quarter-hour remains withheld — and is omitted after 2011 in dated routing.

## Revision rows

- 2010-11-15 — T1 — CME Globex notice 20101025 — Monday–Thursday Pre-Open moves from 16:50 to 16:45 CT.
- 2011-10-02 — T1 — CME Globex notice 20110926 — every legacy CBOT interest-rate open moves to 17:00 CT for trade date Monday 2011-10-03.
- 2026-08-22 — T1 — 2026-08-22 review: verified current, onset undated — knowledge-bound row widening the Sunday queue to the sourced current 16:00–17:00 CT Pre-Open.

## Sources

- <https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20080121.html> — CME Globex notice 20080121, the January-2008 CBOT migration notice establishing the 17:30–16:00 CT schedule.
- <https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20090326.html> — CME Globex notice 20090326, the 2009 table that pins the audit-floor queues at Sunday 16:15 and weekdays 16:50.
- <https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20101025.html> — CME Globex notice 20101025, the 2010-11-15 revision's source.
- <https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20110926.html> — CME Globex notice 20110926, the 2011-10-02 revision's source.
- <https://www.cmegroup.com/notices/electronic-trading/2018/04/20180409.html> — CME Globex notice 20180409, the SOFR launch.
- <https://www.cmegroup.com/trading/interest-rates/files/us-treasury-futures-delivery-process.pdf> — CME U.S. Treasury futures delivery-process guide.
- <https://www.cmegroup.com/markets/interest-rates/stirs/30-day-federal-fund.contractSpecs.html> — CME 30-Day Federal Funds contract specification, current grid.
- <https://www.cmegroup.com/education/articles-and-reports/understanding-sofr-futures> — CME SOFR futures explainer.
- <https://www.cmegroup.com/notices/ser/2022/02/SER-8921.pdf> — CME SER-8921, current-grid corroboration.
- <https://web.archive.org/web/20120505161543/http://www.cmegroup.com/tools-information/holiday-calendar/files/2012-memorial-day.xls> — CME Memorial Day 2012 holiday workbook — capture 2012-05-05, every complex's Sunday pre-opening still at 16:15.
- <https://web.archive.org/web/20120616195651/http://www.cmegroup.com:80/trading_hours/interest-rates-hours.html> — CME interest-rate trading-hours page — capture 2012-06-16, Sunday Pre-Open already 16:00.
- <https://web.archive.org/web/20120511163357id_/http://www.cmegroup.com/trading_hours/index.html?show=Commodities> — CME trading-hours index — capture 2012-05-11.
- <https://web.archive.org/web/20120528102754id_/http://www.cmegroup.com/trading_hours/index.html> — CME trading-hours index — capture 2012-05-28, Sunday Pre-Open 16:15 platform-wide.
- <https://web.archive.org/web/20120607015831id_/http://www.cmegroup.com/trading_hours/> — CME trading-hours index — capture 2012-06-07, Sunday Pre-Open 16:00 platform-wide.
- <https://web.archive.org/web/20190820012118id_/https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20120521.html> — CME Globex Notice 2012-05-21 — read in full, silent on the Pre-Open.
- <https://web.archive.org/web/20190716070058id_/https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20120528.html> — CME Globex Notice 2012-05-28 — read in full, silent on the Pre-Open.
- <https://web.archive.org/web/20190720204402id_/https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20120604.html> — CME Globex Notice 2012-06-04 — read in full, silent on the Pre-Open.
- <https://web.archive.org/web/20120622070557id_/https://www.cmegroup.com/tools-information/lookups/advisories/market-data/20120528.html> — CME Market Data Notice 2012-05-28 — read in full, silent on the Pre-Open.

Official origin of the trading-hours captures: <http://www.cmegroup.com/trading_hours/>.

## Gaps and residual risks

- **order-entry** — the Sunday Pre-Open's move from 16:15 to 16:00 CT has no operator-stated effective day. The 2026-08-31 review narrowed the bracket to 2012-05-28..2012-06-07 from CME's own trading-hours captures and read both CME dated notice channels in full across that window without finding an announcement, so the dated profiles serve the sourced 16:15–17:00 CT intersection and withhold only the 16:00–16:15 CT quarter-hour. Closing condition: a CME document that states the new Sunday Pre-Open in session language on a day-level effective date. Served identity, so tracked as an issue (LAW-FOLLOW-UPS-ARE-ISSUES).
- **residual risk** — the only Sunday inside the narrowed bracket is 2012-06-03; that is an observation about the bracket, not a source-stated effective day, so LAW-NO-FABRICATED-DATES keeps it out of the tables.
- **scope** — individual contract launch dates (SOFR joined the already-live family in May 2018) are catalog facts, not revisions of this product-neutral family clock.

## Module narrative (moved from src/calendar/schedules/futures/us/interest_rates.rs on 2026-09-12 UTC)

The January-2008 CBOT migration notice establishes the 17:30-16:00 CT
schedule inherited by the January-2010 audit-floor Treasury and 30-Day Fed
Funds family. CME moved every legacy CBOT interest-rate open to 17:00 CT
effective Sunday 2011-10-02 (trade date Monday 2011-10-03), aligning the
family with the current 17:00-16:00 grid. The 2009 table also pins the
audit-floor queues at Sunday 16:15 and weekdays 16:50; CME moved the weekday
queue to 16:45 on 2010-11-15. Current material publishes a Sunday 16:00
queue, but no primary source states the exact day on which 16:15 moved to
16:00: the holiday workbook updated 2012-05-03 still schedules every
complex's Sunday pre-opening at 16:15, the interest-rate hours page crawled
2012-06-16 already shows 16:00, and no notice in between states the day.
The fixed-current profile includes the exact current queue. The dated
selector retains the sourced audit-floor queue, then omits only that Sunday
phase after the exact 2011 matching-open revision rather than inventing a
queue cutover.

SOFR joined this already-live
family in May 2018; individual contract launch dates remain catalog facts,
not separate revisions of this product-neutral family clock.
https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20080121.html
https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20090326.html
https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20101025.html
https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20110926.html
https://www.cmegroup.com/notices/electronic-trading/2018/04/20180409.html
https://www.cmegroup.com/trading/interest-rates/files/us-treasury-futures-delivery-process.pdf
https://www.cmegroup.com/markets/interest-rates/stirs/30-day-federal-fund.contractSpecs.html
https://www.cmegroup.com/education/articles-and-reports/understanding-sofr-futures
https://www.cmegroup.com/notices/ser/2022/02/SER-8921.pdf
https://web.archive.org/web/20120505161543/http://www.cmegroup.com/tools-information/holiday-calendar/files/2012-memorial-day.xls
https://web.archive.org/web/20120616195651/http://www.cmegroup.com:80/trading_hours/interest-rates-hours.html

2026-08-31 Sunday-queue review — bracket narrowed, notice channels negative.
Three archived captures of CME's own trading-hours pages, unused by the
earlier review, move the bracket from 2012-05-03..2012-06-15 down to
2012-05-28..2012-06-07. The move was platform-wide and simultaneous: on the
2012-05-28 capture the Sunday Pre-Open is 16:15 for E-mini S&P 500,
Eurodollar, 30-Year Interest Rate Swap, Euroyen TIBOR and (as "17:15 ET
(16:15 CT)") Gold, Silver, Light Sweet Crude and Henry Hub; on the
2012-06-07 capture every one of them reads 16:00. Weekday Pre-Opens are
unchanged across both captures, so this is a Sunday-only change.
CBOT grains are NOT part of it: the 2012-05-11 capture still shows the
pre-expansion 18:00-07:15/09:30-13:15 grain grid with a 16:15 Sunday
Pre-Open, and the 2012-05-28 capture shows the expanded 17:00-14:00 grid
with 16:00 — so grains moved at the separately sourced 2012-05-20
expansion (CME Globex Advisory #20120518), which the grains module already
dates.
Both of CME's dated notice channels were then read in full across the
narrowed window and none announces the change: CME Globex Notices of
2012-05-21, 2012-05-28 and 2012-06-04, and Market Data Notices of
2012-05-28, contain no occurrence of "Pre-Open", "trading hours", "16:00"
or "16:15". The change was therefore made without a dated operator notice,
which is why no cutover is encoded. (The only Sunday inside the narrowed
bracket is 2012-06-03; that is an observation about the bracket, not a
source-stated effective day, so LAW-NO-FABRICATED-DATES keeps it out of the
tables.) Official origin http://www.cmegroup.com/trading_hours/ delivered
via:
https://web.archive.org/web/20120511163357id_/http://www.cmegroup.com/trading_hours/index.html?show=Commodities
https://web.archive.org/web/20120528102754id_/http://www.cmegroup.com/trading_hours/index.html
https://web.archive.org/web/20120607015831id_/http://www.cmegroup.com/trading_hours/
https://web.archive.org/web/20190820012118id_/https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20120521.html
https://web.archive.org/web/20190716070058id_/https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20120528.html
https://web.archive.org/web/20190720204402id_/https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20120604.html
https://web.archive.org/web/20120622070557id_/https://www.cmegroup.com/tools-information/lookups/advisories/market-data/20120528.html

SUNDAY QUEUE, CARRIED BACK AS THE SOURCED INTERSECTION. CME's Sunday Pre-Open
only ever widened inside the modelled window: the audit-floor material pins it
at 16:15 and the verified-current value is 16:00, with the undated 2012 move
(bracketed 2012-05-28..2012-06-07) the only change between them. The
16:15-17:00 window is therefore order-entry under *every* sourced state, so
carrying it from the January-2010 floor asserts no cutover at all - it is the
intersection of the two regimes, not a guess at either. The undated change
adds only the 16:00-16:15 quarter-hour, which the knowledge-bound row supplies
from the repository review date onward. Previously these dated profiles
omitted the Sunday queue entirely, which under-reported order acceptance for
the whole 16:00-17:00 hour rather than only the disputed quarter-hour.
The two profiles above already carry Sunday 16:15; this one dropped it purely
because the 16:15->16:00 day is undated. It now keeps the same intersection.
