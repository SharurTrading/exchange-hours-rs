<!-- SPDX-License-Identifier: MIT-0 -->

# `cme` — evidence

- **Kind:** `Exchange`
- **Owner module:** [`cme_group.rs`](../../src/calendar/schedules/futures/us/cme_group.rs)
- **Source sets:** [`US-CME-GROUP`](../schedules/sources.md#us-cme-group)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

**Gap: order-entry** — the trading session is sourced; what is undated is a queue or post-close phase in which no trade can print. Compatibility default for the scoped CME/CBOT equity-index family. Current RTH/ETH and Pre-Open queues are primary-supported; matching and the exact 2010, 2012, 2015, and 2021 revisions are dated, but the Sunday queue's move from 16:15 to 16:00 lacks a source-stated day: the 2026-08-31 review narrowed it to 2012-05-28..2012-06-07 and found no operator notice in either CME channel across that window. Dated profiles now serve the sourced intersection — Sunday 16:15–17:00, carried from the January-2010 floor because the queue only ever widened — so only the disputed 16:00–16:15 quarter-hour waits on the undated move. Full-size `SP`, NKD, BTIC, and TACO products are excluded.

## Revision rows

- 2010-11-15 — T1 — CME Globex notice 20101025 — Monday–Thursday Pre-Open moves from 16:50 to 16:45 CT.
- 2012-11-18 — T1 — CME Globex notice 20121022 — new daily trading-hour schedule; post-halt slice becomes 15:30–16:15 CT including Fridays.
- 2015-09-20 — T1 — CME Globex notice 20150817 — CME Equity and CBOT Equity closes move 15 minutes earlier to 16:00 CT.
- 2021-06-27 — T1 — CME Globex notice 20210621 — the 15:15–15:30 CT halt is removed, producing the continuous 17:00–16:00 CT ETH envelope.
- 2026-08-22 — T1 — 2026-08-22 review: verified current, onset undated — knowledge-bound row widening the Sunday queue to the sourced current 16:00–17:00 CT Pre-Open.

## Sources

- <https://www.cmegroup.com/content/dam/cmegroup/education/modules/files/EQ240_EQ_for_AIT.pdf> — CME `EQ240` equity-index education module, the October-2009 product guide that supplies the complete audit-floor grid.
- <https://www.cmegroup.com/education/files/eq-trading-hours.pdf> — CME equity-index trading-hours sheet.
- <https://www.cmegroup.com/tools-information/lookups/advisories/clearing/Chadv12-423.html> — CME clearing advisory Chadv12-423, the 2012 trade-date boundary change.
- <https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20121022.html> — CME Globex notice 20121022, the 2012-11-18 revision's source.
- <https://www.cmegroup.com/tools-information/lookups/advisories/market-data/20121015.html> — CME market-data advisory 20121015, corroborating the 2012 change.
- <https://www.cmegroup.com/notices/clearing/2019/06/Chadv19-182.pdf> — CME clearing advisory Chadv19-182.
- <https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20101025.html> — CME Globex notice 20101025, the 2010-11-15 weekday Pre-Open move.
- <https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20150817.html> — CME Globex Notice #20150817 of 17 August 2015, the 2015-09-20 revision's source.
- <https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20150914.html> — CME Globex Notice #20150914, the "Effective this Monday" repeat of the same article.
- <https://www.cmegroup.com/notices/electronic-trading/2021/06/20210621.html> — CME Globex notice 20210621, the 2021-06-27 halt removal.
- <https://www.cmegroup.com/market-regulation/rule-filings/2021/6/21-244R_2.pdf> — CME rule filing 21-244R, corroborating the halt removal.
- <https://www.cmegroup.com/notices/ser/2022/02/SER-8921.pdf> — CME SER-8921, current-grid corroboration.
- <https://web.archive.org/web/20120503104328/http://www.cmegroup.com/trading_hours/equities-hours.html> — CME equities trading-hours page — capture 2012-05-03, Sunday Pre-Open still 16:15.
- <https://web.archive.org/web/20120616181609/http://www.cmegroup.com/trading_hours/equities-hours.html> — CME equities trading-hours page — capture 2012-06-16, Sunday Pre-Open already 16:00.
- <https://web.archive.org/web/20120511163357id_/http://www.cmegroup.com/trading_hours/index.html?show=Commodities> — CME trading-hours index — capture 2012-05-11.
- <https://web.archive.org/web/20120528102754id_/http://www.cmegroup.com/trading_hours/index.html> — CME trading-hours index — capture 2012-05-28, Sunday Pre-Open 16:15 platform-wide.
- <https://web.archive.org/web/20120607015831id_/http://www.cmegroup.com/trading_hours/> — CME trading-hours index — capture 2012-06-07, Sunday Pre-Open 16:00 platform-wide.
- <https://web.archive.org/web/20190820012118id_/https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20120521.html> — CME Globex Notice 2012-05-21 — read in full, silent on the Pre-Open.
- <https://web.archive.org/web/20190716070058id_/https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20120528.html> — CME Globex Notice 2012-05-28 — read in full, silent on the Pre-Open.
- <https://web.archive.org/web/20190720204402id_/https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20120604.html> — CME Globex Notice 2012-06-04 — read in full, silent on the Pre-Open.
- <https://web.archive.org/web/20120622070557id_/https://www.cmegroup.com/tools-information/lookups/advisories/market-data/20120528.html> — CME Market Data Notice 2012-05-28 — read in full, silent on the Pre-Open.

Official origin of the four trading-hours captures: <http://www.cmegroup.com/trading_hours/>.

## Gaps and residual risks

- **order-entry** — the Sunday Pre-Open's move from 16:15 to 16:00 CT has no operator-stated effective day. The 2026-08-31 review narrowed the bracket to 2012-05-28..2012-06-07 from CME's own trading-hours captures and read both CME dated notice channels in full across that window without finding an announcement, so the dated profiles serve the sourced 16:15–17:00 CT intersection and withhold only the 16:00–16:15 CT quarter-hour. Closing condition: a CME document that states the new Sunday Pre-Open in session language on a day-level effective date. Served identity, so tracked as an issue (LAW-FOLLOW-UPS-ARE-ISSUES). Horizon 2012-05-03: below that capture the Sunday 16:15–17:00 CT queue is carried, not sourced.
- **residual risk** — the only Sunday inside the narrowed bracket is 2012-06-03; that is an observation about the bracket, not a source-stated effective day, so LAW-NO-FABRICATED-DATES keeps it out of the tables.
- **scope** — full-size `SP`, NKD, BTIC and TACO products are excluded from this compatibility default; NKD has its own key and module.

## Module narrative (moved from src/calendar/schedules/futures/us/cme_group.rs on 2026-09-12 UTC)

U.S.-grid CME and CBOT equity-index futures, including CBOT YM/MYM but not
CME Nikkei 225 Dollar (NKD), whose historical grid differs. CME's
October-2009 product guide supplies the complete grid at the audit floor:
Sunday 17:00–Monday 15:15, then Monday–Thursday 17:00–15:15 and
15:30–16:30, with 16:30–17:00 maintenance. The 2012 notice changed the
trade-date boundary and the post-halt slice to 15:30–16:15, including
Fridays, effective Sunday 2012-11-18. CME Globex then moved that close
15 minutes earlier to 16:00 CT effective Sunday 2015-09-20 for trade date
Monday 2015-09-21. CME then removed the 15:15-15:30 halt for the scoped
contracts effective Sunday 2021-06-27, producing the current continuous
17:00-16:00 ETH envelope around the unchanged 08:30-15:15 RTH.

The exact Monday-Thursday Pre-Open changed from 16:50 to 16:45 on
2010-11-15. Current primary material also establishes Sunday 16:00-17:00,
but calls it a long-term practice without giving the day when the earlier
16:15 start moved: primary documents updated 2012-05-03 still publish
Sunday 16:15, trading-hours pages crawled 2012-06-15/16 already publish
16:00, and no notice in between states the day. The fixed-current table
includes that sourced current queue. Dated profiles carry the sourced
Sunday 16:15–17:00 intersection from the January-2010 floor and withhold
only the disputed 16:00–16:15 quarter-hour rather than inventing its
cutover; their executable trading and weekday queues remain exact. Revisions are keyed by the local
session-opening day.
https://www.cmegroup.com/content/dam/cmegroup/education/modules/files/EQ240_EQ_for_AIT.pdf
https://www.cmegroup.com/education/files/eq-trading-hours.pdf
https://www.cmegroup.com/tools-information/lookups/advisories/clearing/Chadv12-423.html
https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20121022.html
https://www.cmegroup.com/tools-information/lookups/advisories/market-data/20121015.html
https://www.cmegroup.com/notices/clearing/2019/06/Chadv19-182.pdf
https://web.archive.org/web/20120503104328/http://www.cmegroup.com/trading_hours/equities-hours.html

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
https://web.archive.org/web/20120616181609/http://www.cmegroup.com/trading_hours/equities-hours.html
The 2015-09-20 revision's original announcement, CME Globex Notice #20150817
of 17 August 2015: "Effective Monday, September 21, the daily CME Globex
maintenance period will begin 15 minutes earlier Monday through Thursday from
16:00 until 16:45 Central Time (CT). ... the closing times for the following
markets will now occur 15 minutes earlier Monday through Friday at 16:00 CT.
CME Equity / CBOT Equity / COMEX / NYMEX / DME. All other CME Globex markets
trading hours remain unchanged." The #20150914 repeat below carries the same
article with "Effective this Monday" wording.
https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20150817.html
https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20150914.html
https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20101025.html
https://www.cmegroup.com/notices/electronic-trading/2021/06/20210621.html
https://www.cmegroup.com/market-regulation/rule-filings/2021/6/21-244R_2.pdf
https://www.cmegroup.com/notices/ser/2022/02/SER-8921.pdf

ORDER-ENTRY CLASSIFICATION. The evening phases the citations above name as
the "Pre-Open" (Monday-Thursday 16:50, later 16:45, to the 17:00 Globex open)
and the Sunday 16:00-17:00 queue are Globex pre-open states: the book accepts,
amends, and cancels orders, but the matching engine is not running and no
trade can print until the 17:00 open. They are therefore `order_entry`, not
`extended`. Everything that remains in the extended slices below is a
matching phase: the post-halt afternoon slice and the 17:00 electronic
session both print trades.

Pre-Open queues. No trade can match in any of these windows.
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
