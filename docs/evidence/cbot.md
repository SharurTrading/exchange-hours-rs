<!-- SPDX-License-Identifier: MIT-0 -->

# `cbot` — evidence

- **Kind:** `Exchange`
- **Owner module:** [`grains.rs`](../../src/calendar/schedules/futures/us/grains.rs)
- **Source sets:** [`US-CME-GROUP`](../schedules/sources.md#us-cme-group)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

**Gap: order-entry** — the trading session is sourced; what is undated is a queue or post-close phase in which no trade can print. Compatibility default for standard-size CBOT grain/oilseed futures. Current matching, morning Pre-Open, and PCP envelopes are primary-supported, and every exact matching/RTH revision is dated. The 2013-03-22 operator notice dates the Sunday 16:00-19:00 and Monday-Thursday 16:45-19:00 queues and the 14:30-16:00 PCP to 2013-04-07; only the 21-hour 2012-05-20..2013-04-06 regime's queue states remain undocumented. Mini grains are excluded.

## Revision rows

- 2010-04-19 — T1 — CME Globex notice 20100405 — the afternoon PCP expands to 13:15:30–16:00 CT.
- 2011-12-27 — T1 — CFTC filing rul120711cbot001 — the weekday morning queue moves to 08:00 CT.
- 2012-05-20 — T1 — CME market-data advisory 20120518 — matching expands to 17:00–14:00 CT.
- 2013-04-07 — T1 — CME SER-6617 and GCC notice 2013-03-22 — 19:00–07:45 CT electronic session around an 08:30–13:15 CT day session, with the full queue set.
- 2013-08-18 — T1 — CME market-data advisory 20130812 — the morning Pre-Open widens from 08:15 to 08:00 CT.
- 2015-07-05 — T1 — CME SER-7395R — the day-session close moves to 13:20 CT.

## Sources

Row review: 2026-08-29 (UTC) is the date the ledger row was last reviewed as a
whole. Per-source retrieval dates were not recorded before the 2026-09-12
migration; where a later targeted review, capture or document date is recorded
beside a source below, that date governs for that source, and dates are added
as each source is re-verified.

- <https://www.cmegroup.com/media-room/press-releases/2009/6/05/cme_group_announcesadditionalagricultureethanolelectronictrading.html> — CME press release of 2009-06-05, the pre-floor grain electronic-hours expansion.
- <https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20100315.html> — CME Globex notice 20100315, the March-2010 market-state table that supplies the then-live audit-floor queue and PCP phases.
- <https://www.cmegroup.com/tools-information/lookups/advisories/market-data/Q2010-62.html> — CME market-data advisory Q2010-62.
- <https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20100405.html> — CME Globex notice 20100405, the 2010-04-19 revision's source.
- <https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20101025.html> — CME Globex notice 20101025, the generic afternoon-queue notice that does not enumerate this family.
- <https://www.cftc.gov/stellent/groups/public/%40rulesandproducts/documents/ifdocs/rul120711cbot001.pdf> — CBOT rule filing `rul120711cbot001` as published by the CFTC, the 2011-12-27 revision's source.
- <https://www.cmegroup.com/tools-information/lookups/advisories/market-data/20120518.html> — CME market-data advisory 20120518, the 2012-05-20 matching expansion.
- <https://www.cmegroup.com/tools-information/lookups/advisories/market-data/20120904.html> — CME market-data advisory 20120904, the mini-grain divergence.
- <https://www.cmegroup.com/rulebook/files/ser_6617_cbot_grain_oilseed_hours_2013_final.pdf> — CME SER-6617, the 2013-04-07 revision's source.
- <https://web.archive.org/web/20130423023212/http://www.cmegroup.com/globex/files/cmegroup_reduced_grain_and_oilseed_hours.pdf> — CME Global Command Center client notice of 2013-03-22, which states every current queue's onset — capture 2013-04-23.
- <https://www.cmegroup.com/tools-information/lookups/advisories/market-data/20130812.html> — CME market-data advisory 20130812, the 2013-08-18 revision's source.
- <https://www.cmegroup.com/tools-information/lookups/advisories/ser/SER-7395R.html> — CME SER-7395R, the 2015-07-05 revision's source.
- <https://www.cmegroup.com/notices/ser/2022/02/SER-8921.pdf> — CME SER-8921, current-grid corroboration.
- <https://web.archive.org/web/20120511163357id_/http://www.cmegroup.com/trading_hours/index.html?show=Commodities> — CME trading-hours index — capture 2012-05-11, the pre-expansion grain grid with a 16:15 Sunday Pre-Open.
- <https://web.archive.org/web/20120528102754id_/http://www.cmegroup.com/trading_hours/index.html> — CME trading-hours index — capture 2012-05-28, the expanded 17:00–14:00 grid with a 16:00 Sunday Pre-Open.
- <https://web.archive.org/web/20120607015831id_/http://www.cmegroup.com/trading_hours/> — CME trading-hours index — capture 2012-06-07.

Official origin of the trading-hours captures: <http://www.cmegroup.com/trading_hours/>.

## Gaps and residual risks

- **order-entry** — the queue and PCP states of the 21-hour 2012-05-20..2013-04-06 regime have no operator-stated onset day. The 2026-08-31 review sourced the states themselves from CME's own trading-hours captures of 2012-05-28 and 2012-06-07 against the pre-expansion 2012-05-11 capture, which brackets the switch to 2012-05-11..2012-05-28; CME market-data advisory 20120518 states only the new matching hours and never the queue times, so no queue revision is keyed to 2012-05-20. Closing condition: a CME document that states those queue times in session language on a day-level effective date. Served identity, so tracked as an issue (LAW-FOLLOW-UPS-ARE-ISSUES).
- **residual risk** — the baseline queue and PCP phases rest on the operator's March-2010 market-state table, which states them as then-live rather than dating them, so they are carried back from 2010-03-15 to the January-2010 floor.
- **scope** — mini-sized Corn, Soybean, Wheat and KC HRW Wheat diverged on 2012-09-16 and are owned by `mini_grains.rs`; Rough Rice borrows the pre-2018 eras but owns its own timeline in `rough_rice.rs`.
- **holidays, no table** — this venue calendar ships no built-in holiday table in the change that landed the served families' tables, and `holiday_coverage()` answers `None` for it. A venue calendar's table is the **intersection** of the families that route to it (design memo D17): full closures agree across those families, most early closes do not, and a date on which two families disagree can ship no venue row at all. Building that intersection, and naming every date it has to drop as a gap, is its own bounded change (memo §5.2, Wave 7). Until it lands the caller's `DayPolicy` overlay is the only holiday layer for this venue, and a consumer that wants CME holiday data today should route through the product-family key rather than the venue. Closing condition: the venue-intersection wave, plus the cross-wave agreement audit that memo §7 follow-up 10 names (#95).

## Module narrative (moved from src/calendar/schedules/futures/us/grains.rs on 2026-09-12 UTC)

This profile is deliberately limited to standard-size CBOT grain and oilseed
futures. Mini-sized Corn, Soybean, Wheat, and KC HRW Wheat diverged on
2012-09-16 and are not represented by this key; `mini_grains.rs` owns their
sourced timeline.

The `CBOT_*` rule tables below are `pub(crate)` because Rough Rice
(`rough_rice.rs`) ran on this same grid until its 2018-01-21 divergence and
borrows the pre-divergence eras rather than copying them. That key owns its
own `StaticHoursProfile` values and its own timeline, so a future Rough
Rice-specific finding repoints one of its eras instead of editing anything
here. Nothing in this file may be changed on Rough Rice evidence.

At the January-2010 audit floor, matching ran 18:00-07:15 around the
09:30-13:15 RTH. The operator's March-2010 market-state table supplies the
then-live 16:15-18:00 Sunday, 07:15-09:30 weekday, and 14:30-16:00 PCP
phases. On 2010-04-19 PCP expanded to 13:15:30-16:00. The CFTC filing makes
the weekday morning queue's move to 08:00 effective Tuesday 2011-12-27.
A later generic Globex notice broadly names CBOT in an afternoon queue
change, but it does not enumerate this family and conflicts with the complete
family-specific state table. No separate evening queue is inferred from it.
https://www.cmegroup.com/media-room/press-releases/2009/6/05/cme_group_announcesadditionalagricultureethanolelectronictrading.html
https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20100315.html
https://www.cmegroup.com/tools-information/lookups/advisories/market-data/Q2010-62.html
https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20100405.html
https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20101025.html
https://www.cftc.gov/stellent/groups/public/%40rulesandproducts/documents/ifdocs/rul120711cbot001.pdf

Matching expanded to 17:00-14:00 on 2012-05-20. SER-6617 then established
19:00-07:45 and 08:30-13:15 effective Sunday 2013-04-07. CME expanded the
exact morning Pre-Open to 08:00-08:30 on 2013-08-18, and SER-7395R moved the
RTH close to 13:20 on 2015-07-05.
https://www.cmegroup.com/tools-information/lookups/advisories/market-data/20120518.html
https://www.cmegroup.com/tools-information/lookups/advisories/market-data/20120904.html
https://www.cmegroup.com/rulebook/files/ser_6617_cbot_grain_oilseed_hours_2013_final.pdf
https://www.cmegroup.com/tools-information/lookups/advisories/market-data/20130812.html
https://www.cmegroup.com/tools-information/lookups/advisories/ser/SER-7395R.html

The 22 March 2013 Global Command Center client notice that carried the
SER-6617 change also states every current queue's unconditional onset:
"Effective Sunday, April 7, 2013 (trade date Monday, April 8) ... Pre-Opens
(including MGEX): Sunday night: 16:00-19:00 CT / Monday-Thursday night:
16:45-19:00 CT / Monday-Friday morning: 08:15-08:30 CT. Post Close Pre-Open:
Monday-Friday: 14:30-16:00 CT", with a 07:45-08:15 cancellation-only slice
inside the break that no order-entry rule models. Only the 21-hour
2012-05-20..2013-04-06 regime's queue and PCP states remain undocumented, so
2026-08-31 grains-regime review — the states are now sourced, only their
switch-on day is not. CME's own trading-hours pages inside the 21-hour
regime publish, for Corn/Wheat/Soybean/Soybean Oil futures and options,
Sunday Pre-Open 16:00, weekday Pre-Open "14:30-16:00, 16:45-17:00" (the PCP
plus the evening queue) and ETH 17:00-14:00. The pre-expansion capture of
2012-05-11 shows the other side: Sunday Pre-Open 16:15, weekday
"14:30-16:00 16:45 08:00" and ETH 18:00-07:15, 09:30-13:15. The switch is
therefore bracketed to 2012-05-11..2012-05-28, which contains the sourced
2012-05-20 expansion — but Advisory #20120518 states only the new matching
hours, never the queue times, so no queue revision is keyed to that day.
Official origin http://www.cmegroup.com/trading_hours/ delivered via:
https://web.archive.org/web/20120511163357id_/http://www.cmegroup.com/trading_hours/index.html?show=Commodities
https://web.archive.org/web/20120528102754id_/http://www.cmegroup.com/trading_hours/index.html
https://web.archive.org/web/20120607015831id_/http://www.cmegroup.com/trading_hours/
that dated profile conservatively omits them instead of inventing onsets.
https://web.archive.org/web/20130423023212/http://www.cmegroup.com/globex/files/cmegroup_reduced_grain_and_oilseed_hours.pdf
https://www.cmegroup.com/notices/ser/2022/02/SER-8921.pdf

ORDER-ENTRY CLASSIFICATION. The comment above distinguishes the matching
windows from the market-state phases the operator's tables publish around
them. Only the matching windows can print a trade, so the Sunday evening
queue, the weekday morning queue (07:15, later 08:00, briefly 08:15 from
2013-04-07, back to 08:00 from 2013-08-18, up to the day-session open), and
the afternoon PCP are `order_entry`; the electronic session and the
post-2012 afternoon matching slice stay `extended`.

Revision evidence — each row's day-level effective date and the primary
source that states it (full quotations sit in the blocks above):
  2010-04-19 "CME Globex notice 20100405"
    https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20100405.html
  2011-12-27 "CFTC filing rul120711cbot001"
    https://www.cftc.gov/stellent/groups/public/%40rulesandproducts/documents/ifdocs/rul120711cbot001.pdf
  2012-05-20 "CME market-data advisory 20120518"
    https://www.cmegroup.com/tools-information/lookups/advisories/market-data/20120518.html
  2013-04-07 "CME SER-6617 and GCC notice 2013-03-22"
    https://www.cmegroup.com/rulebook/files/ser_6617_cbot_grain_oilseed_hours_2013_final.pdf
    https://web.archive.org/web/20130423023212/http://www.cmegroup.com/globex/files/cmegroup_reduced_grain_and_oilseed_hours.pdf
  2013-08-18 "CME market-data advisory 20130812"
    https://www.cmegroup.com/tools-information/lookups/advisories/market-data/20130812.html
  2015-07-05 "CME SER-7395R"
    https://www.cmegroup.com/tools-information/lookups/advisories/ser/SER-7395R.html
