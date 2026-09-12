<!-- SPDX-License-Identifier: MIT-0 -->

# `globex_livestock` — evidence

- **Kind:** `MarketHoursKey`
- **Owner module:** [`livestock.rs`](../../src/calendar/schedules/futures/us/livestock.rs)
- **Source sets:** [`US-CME-GROUP`](../schedules/sources.md#us-cme-group)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

**Gap: order-entry** — the trading session is sourced; what is undated is a queue or post-close phase in which no trade can print. CME Live Cattle, Feeder Cattle, and Lean Hog futures. Matching revisions in 2014/2016 and the 2020 08:00 Pre-Open start are exact. The 2016-05-30 Globex notice dates the 14:30-16:00 PCP onset to 2016-06-06; official trading-hours captures omit the PCP row between November 2016 and March 2020 with no removal notice, and the pre-2020 06:00 queue's own onset is unresolved, so those remain the open gaps. The 2026-08-31 review checked the contract-specification channel as a second route into that interval and found it silent as well: the Live Cattle specification captured 2016-09-25, 2016-12-04, 2017-09-23 and 2018-04-19 renders only ClearPort/Default 08:30-13:05 CT hours with no Globex Pre-Open or PCP row. The pre-2020 morning queue is no longer omitted, though: SER-8599R states the outgoing 06:00 value when it dates the move to 08:00, so 06:00–08:30 is carried across 2016-02-29..2020-05-31 — the span of the matching grid it runs into — and only its onset before that grid stays unresolved.

## Revision rows

- 2014-10-27 — T1 — CME SER-7194 — the evening sessions are removed from the around-the-clock grid.
- 2016-02-29 — T1 — CME SER-7591 — the current 08:30–13:05 CT weekday session is established for LE, GF and HE.
- 2016-06-06 — T1 — CME Globex notice 20160530 — a Monday–Friday 14:30–16:00 CT Post-Close order-entry period begins.
- 2020-05-31 — T1 — CME SER-8599R — the morning Pre-Open start moves from 06:00 to 08:00 CT for trade date Monday 2020-06-01.

## Sources

- <https://www.cmegroup.com/media-room/press-releases/2007/3/07/cme_to_offer_around-the-clocktradingofcommodityproductsoncmeglob.html> — CME press release of 2007-03-07, the around-the-clock launch that establishes the Monday 09:05 CT weekly open and the 16:00–17:00 CT daily halts.
- <https://www.cmegroup.com/tools-information/lookups/advisories/market-data/Q2008-215.html> — CME market-data advisory Q2008-215, the pre-floor move of the Friday close to 13:55 CT.
- <https://www.cmegroup.com/tools-information/lookups/advisories/ser/files/SER-7194.pdf> — CME SER-7194, the 2014-10-27 revision's source.
- <https://www.cmegroup.com/market-regulation/files/14-408.pdf> — CME rule filing 14-408, the 2014 reduction report confirming the complete old grid.
- <https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20101025.html> — CME Globex notice 20101025, the generic afternoon-queue notice that does not enumerate livestock.
- <https://www.cmegroup.com/tools-information/lookups/advisories/ser/files/SER-7591.pdf> — CME SER-7591, the 2016-02-29 revision's source.
- <https://www.cmegroup.com/notices/electronic-trading/2016/05/20160530.html> — CME Globex notice of 30 May 2016, the 2016-06-06 Post-Close onset.
- <https://web.archive.org/web/20160605123512/http://www.cmegroup.com:80/notices/electronic-trading/2016/05/20160530.html> — the same notice — capture 2016-06-05.
- <https://www.cmegroup.com/notices/electronic-trading/2020/05/20200511.html> — CME Globex notice 20200511, corroborating the 2020 Pre-Open move.
- <https://www.cmegroup.com/notices/ser/2020/05/SER-8599R.pdf> — CME SER-8599R, the 2020-05-31 revision's source and the statement of the outgoing 06:00 CT value.
- <https://www.cmegroup.com/market-regulation/rule-filings/2020/5/20-232.pdf> — CME rule filing 20-232.
- <https://www.cmegroup.com/trading-hours/files/memorial-day-2023.pdf> — CME Memorial Day 2023 trading-hours sheet, current-grid corroboration.
- <https://www.cmegroup.com/education/lessons/live-cattle-product-overview> — CME Live Cattle product overview.

## Gaps and residual risks

- **order-entry** — official CME trading-hours captures omit the 14:30–16:00 CT PCP row between November 2016 and March 2020 with no removal notice. The 2026-08-31 review checked the contract-specification channel as a second route into that interval and found it silent as well: the Live Cattle specification captured 2016-09-25, 2016-12-04, 2017-09-23 and 2018-04-19 renders only ClearPort and Default 08:30–13:05 CT hours with no Globex Pre-Open or PCP row. The omission is treated as a published-table gap rather than an operator-stated removal, so the sourced 2016-06-06 onset stands. Closing condition: a CME document that either removes or restates the PCP in session language on a day-level effective date. Served identity, so tracked as an issue (LAW-FOLLOW-UPS-ARE-ISSUES).
- **order-entry** — the pre-2020 06:00 CT morning queue's own onset is unresolved. SER-8599R states the outgoing 06:00 value when it dates the move to 08:00, so 06:00–08:30 is carried across 2016-02-29..2020-05-31 — the span of the matching grid it runs into — and is deliberately not carried further back, because before 2016-02-29 the family ran the old around-the-clock grid with no 08:30 open for a morning queue to precede.
- **residual risk** — a generic 2010 CME Globex queue notice does not enumerate livestock, so it is not used to invent a family-specific afternoon queue in the old around-the-clock grid.

## Module narrative (moved from src/calendar/schedules/futures/us/livestock.rs on 2026-09-12 UTC)

CME's 2007 launch announcement establishes the Monday 09:05 CT weekly open
and 16:00-17:00 daily halts; Q2008-215 moved the Friday close to 13:55 before
the audit floor, and the 2014 reduction report confirms that complete old
grid. SER-7194 removed the evening sessions effective Monday 2014-10-27.
SER-7591 then set the current 08:30-13:05 CT weekday session for LE, GF, and
HE effective Monday 2016-02-29. CME's 30 May 2016 Globex notice implemented
a Post-Close state — GTC/GTD order entry, modification, and cancellation for
the next trade date with "No matching ... during the Post-Close" — Monday
through Friday 14:30-16:00 CT for the same LE, GF, and HE families,
effective Monday 2016-06-06. Official trading-hours captures omit the PCP
row between November 2016 and March 2020 without any removal notice, so the
omission is treated as a published-table gap rather than an operator-stated
removal and the sourced onset stands. CME moved the Pre-Open start from
06:00 to 08:00 effective Sunday 2020-05-31 for trade date Monday 2020-06-01.
2026-08-31 review: the contract-specification channel was checked as a second
route into the 2016-11..2020-03 interval and is silent too — the Live Cattle
specification captured 2016-09-25, 2016-12-04, 2017-09-23 and 2018-04-19
renders only "CME ClearPort" and "Default" hours (Monday-Friday 08:30-13:05
CT) with no CME Globex Pre-Open or PCP row at all. Both the trading-hours and
the specification channels therefore fail to carry the PCP through that
interval, which corroborates the omission below rather than resolving it.
The pre-2020 06:00 queue is now carried across 2016-02-29..2020-05-31 (see the
note beside its rule set below); its onset before that grid is still
unresolved, so the older around-the-clock profiles keep no queue. A generic 2010 Globex
queue notice does not enumerate livestock, so it is not used to invent a
family-specific afternoon queue in the old around-the-clock grid.
https://www.cmegroup.com/media-room/press-releases/2007/3/07/cme_to_offer_around-the-clocktradingofcommodityproductsoncmeglob.html
https://www.cmegroup.com/tools-information/lookups/advisories/market-data/Q2008-215.html
https://www.cmegroup.com/tools-information/lookups/advisories/ser/files/SER-7194.pdf
https://www.cmegroup.com/market-regulation/files/14-408.pdf
https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20101025.html
https://www.cmegroup.com/tools-information/lookups/advisories/ser/files/SER-7591.pdf
https://www.cmegroup.com/notices/electronic-trading/2016/05/20160530.html
https://web.archive.org/web/20160605123512/http://www.cmegroup.com:80/notices/electronic-trading/2016/05/20160530.html
https://www.cmegroup.com/notices/electronic-trading/2020/05/20200511.html
https://www.cmegroup.com/notices/ser/2020/05/SER-8599R.pdf
https://www.cmegroup.com/market-regulation/rule-filings/2020/5/20-232.pdf
https://www.cmegroup.com/trading-hours/files/memorial-day-2023.pdf
https://www.cmegroup.com/education/lessons/live-cattle-product-overview

ORDER-ENTRY CLASSIFICATION. Both phases modelled after 2016-02-29 are
non-matching. The comment above names 08:00-08:30 as the "Pre-Open" (its
start moved from 06:00 on 2020-05-31) which queues orders until the 08:30
regular open, and 14:30-16:00 as PCP, the post-close order-entry period that
follows the 13:05 close. Neither can print a trade, so the family has no
tradeable extended session at all: `extended` is empty and both phases are
`order_entry`.

PRE-2020 MORNING QUEUE, CARRIED BACK TO THE MATCHING GRID IT BELONGS TO. The
2020 notice dates the move of the morning Pre-Open start "from 06:00 to
08:00" on 2020-05-31, so it states the outgoing 06:00 value the same way
SER-6465 states CME's outgoing equity-index close. No primary source names a
cutover between 2016-02-29 - when SER-7591 established the 08:30 open this
queue runs into - and 2020-05-31, so 06:00-08:30 is carried across that
interval rather than omitted. It is deliberately NOT carried further back:
before 2016-02-29 the family ran the old around-the-clock grid with no 08:30
open for a morning queue to precede, and the generic 2010 Globex queue notice
does not enumerate livestock.
