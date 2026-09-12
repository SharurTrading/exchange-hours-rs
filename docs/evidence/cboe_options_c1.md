<!-- SPDX-License-Identifier: MIT-0 -->

# `cboe_options_c1` — evidence

- **Kind:** `Exchange`
- **Owner module:** [`options.rs`](../../src/calendar/schedules/equities/us/options.rs), [`history.rs`](../../src/calendar/schedules/equities/us/options/history.rs)
- **Source sets:** [`US-CBOE-OPTIONS`](../schedules/sources.md#us-cboe-options)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

**Gap: order-entry** — the trading session is sourced; what is undated is a queue or post-close phase in which no trade can print. Current ordinary-stock-options envelope includes the 07:30 order queue and 09:30–16:00 regular session. The RTH baseline predates 2010; the exact generic queue-onset day is unavailable. The start is an operator system setting on a mutable hours/system-settings page, not a rulebook boundary carrying a filed operative date, and no operator channel publishes a dated change notice for it. As of 2026-09-01 the queue is therefore **carried across history** rather than withheld — from the January-2010 floor where the venue predates it, or from its sourced launch day — which asserts continuity no document states. That assumption is deliberate and is recorded beside the profile and in `options/history.rs`; it affects order acceptance only, since nothing matches in a queue and the 09:30–16:00 execution history is sourced independently.

## Revision rows

None. `options/history.rs` holds a single static profile for this identity with no dated revision row; its `revisions!` blocks belong to the venues that launched after the floor.

## Sources

Retrieval dates: these sources were last opened on the row's reviewed-on date
(2026-08-22, UTC); per-source retrieval dates were not recorded before the
2026-09-12 migration and are added as each source is re-verified.

- <https://www.sec.gov/rules/sro/cboe/2006/34-53246.pdf> — CBOE 2006 rule change moving individual-stock options from a 16:02 to a 16:00 ET close; the pre-floor baseline for the C1 execution grid.
- <https://cdn.cboe.com/resources/regulation/rule_book/C1_Exchange_Rule_Book.pdf> — Cboe C1 Exchange Rule Book; retains 09:30-16:00 ET RTH for this product family.
- <https://www.cboe.com/about/hours/us-options> — Cboe US options hours page; the operator system setting behind the 07:30 ET order-acceptance queue.
- <https://cdn.cboe.com/resources/release_notes/2026/Schedule_Update_C1_Options_to_Offer_GTH_Sessions_for_Multi_List_Options_Series.html> — Cboe schedule update introducing C1 GTH sessions for venue-designated multi-list series; a separate product family, deliberately outside this profile's scope.

The queue carry-back was decided on 2026-09-01, after the 2026-08-22 source-set review;
that decision date is not a review date. The retrieved bytes live in the
research store beside the repository.

## Gaps and residual risks

- **order-entry** — the day the 07:30 ET order-acceptance queue began is undated. No primary source states it: the queue is an operator *system setting* published on a mutable hours or system-settings page, not a rulebook boundary with a filed operative date. As of 2026-09-01 the queue is carried from the January-2010 floor rather than withheld, which asserts continuity no document states. Nothing matches in the queue, so the 09:30-16:00 ET execution history is unaffected. Closing condition: an operator artifact stating the queue's start in session language on a day-level effective date. Dormant identity, so the gap is recorded here rather than opened as an issue (LAW-FOLLOW-UPS-ARE-ISSUES).
- **scope** — C1 offers an additional GTH session for venue-designated multi-list series. That is a separate product family with its own executable session and is deliberately outside this profile, which is scoped to ordinary options on individual US stocks.
- **below the floor** — the 2006 coordinated SRO rule changes moved individual-stock options from a 16:02 to a 16:00 ET close before the January-2010 history floor, and that change is out of scope by design (LAW-NO-FABRICATED-DATES records amendment history back to January 2010). The 2006 filings are cited here because they are what sources the 09:30-16:00 ET grid through the floor.
- **scope** — ETF, ETN, index, FLEX, floor-only and venue-designated extended-hours option classes are separate product families because their executable sessions vary, and this row does not cover them.
- **timeline** — this venue's 09:30-16:00 ET execution history predates the January-2010 floor, so its timeline static in `options/history.rs` is empty and `select_revision` returns the baseline profile — the current grid together with the carried 07:30 ET order-acceptance queue — at every date the crate answers for. That is why `## Revision rows` above carries no bullet.

## Module narrative (moved from src/calendar/schedules/equities/us/options.rs on 2026-09-12 UTC)

Every profile in this module is deliberately scoped to ordinary options on  
individual US stocks. Generic pre-open order acceptance is part of the  
exchange envelope, but execution in this product family begins at 09:30, so  
those windows are order entry rather than trading. ETF, ETN, index, FLEX,  
floor-only, and venue-designated extended-hours classes remain separate  
product families because their executable sessions vary.

The 2006 coordinated rule changes moved individual-stock options from a  
16:02 to a 16:00 ET close before this repository's January-2010 history  
floor. Each older venue has its own primary baseline (not merely a shared  
industry inference):  
C1: https://www.sec.gov/rules/sro/cboe/2006/34-53246.pdf  
Arca: https://www.sec.gov/rules/sro/pcx/34-53249.pdf  
American: https://www.sec.gov/rules/sro/amex/2006/34-53244.pdf  
PHLX: https://www.sec.gov/rules/sro/phlx/34-53247.pdf  
ISE: https://www.sec.gov/rules/sro/ise/2006/34-53248.pdf  
BOX: https://www.sec.gov/rules/sro/bse/2006/34-53245.pdf  
NOM's approved rules set 09:30–16:00 for this family, and its launch alert  
identifies AMAT (an individual stock) among the 2008-03-31 launch classes:  
https://www.sec.gov/rules/sro/nasdaq/2008/34-57478.pdf  
https://www.nasdaqtrader.com/MicroNews.aspx?id=OTA2008-001  
The current operator rules retain 09:30–16:00 RTH for this exact product  
family. C1 has an additional session for venue-designated classes. MRX's  
approved Options 3C session remains unencoded until the required trader alert  
makes it operative.  
https://cdn.cboe.com/resources/release_notes/2026/Schedule_Update_C1_Options_to_Offer_GTH_Sessions_for_Multi_List_Options_Series.html  
https://www.sec.gov/rules-regulations/self-regulatory-organization-rulemaking/sr-mrx-2026-11  
https://cdn.cboe.com/resources/regulation/rule_book/C1_Exchange_Rule_Book.pdf  
https://cdn.cboe.com/resources/regulation/rule_book/C2_Exchange_Rule_Book.pdf  
https://cdn.cboe.com/resources/regulation/rule_book/BZX_Exchange_Rulebook.pdf  
https://cdn.cboe.com/resources/regulation/rule_book/EDGX_Rulebook.pdf  
https://nysearcaguide.srorules.com/rules  
https://nyseamericanguide.srorules.com/rules  
https://listingcenter.nasdaq.com/RuleBook/Nasdaq/rules/nasdaq-options-3  
https://listingcenter.nasdaq.com/rulebook/phlx/rules/Phlx%20Options%203  
https://listingcenter.nasdaq.com/rulebook/ise/rules/ISE%20Options%203  
https://listingcenter.nasdaq.com/rulebook/gemx/rules/GEMX%20Options%203  
https://listingcenter.nasdaq.com/rulebook/mrx/rules/MRX%20Options%203  
https://listingcenter.nasdaq.com/rulebook/nasdaqtx/rules/NTX%20Options%203  
https://www.miaxglobal.com/markets/us-options/miax-options/trade-hours-calendar  
https://www.miaxglobal.com/markets/us-options/pearl-options/trade-hours-calendar  
https://www.miaxglobal.com/markets/us-options/emerald-options/trade-hours-calendar  
https://www.miaxglobal.com/markets/us-options/sapphire-options/trade-hours-calendar  
https://rules.boxexchange.com  
https://info.memxtrading.com/market-hours-and-holiday-schedule/

Order-entry-only pre-open queues. Each venue below opens its book to order  
entry, amendment, and cancellation at the stated time, but no contract in  
this product family can match until the opening process runs at 09:30 ET —  
the cited operator system-settings and hours pages describe these windows as  
order acceptance/queuing, and the first execution of the day is the 09:30  
opening. They are therefore `order_entry`, not tradeable extended sessions.

Current ordinary-stock-option order-acceptance edges. The reviewed primary  
sources supply no day-level amendment chain for these queues — they are  
operator system settings, not rulebook boundaries — so as of 2026-09-01 each  
venue carries its queue across history rather than only from a review-date  
row. The per-profile notes below say which basis each venue uses, and  
`options/history.rs` states the assumption in full. Because nothing matches in  
a queue, this changes `order_entry` coverage only: every venue's 09:30–16:00  
execution history is sourced independently and untouched.  
https://www.cboe.com/about/hours/us-options  
https://www.nyse.com/trade/hours-calendars?os=.  
https://www.nasdaq.com/docs/PHLXSystemSettings  
https://www.nasdaq.com/docs/NOMSystemSettings  
https://www.nasdaq.com/docs/ISESystemSettings  
https://www.nasdaq.com/docs/GEMXSystemSettings.pdf  
https://www.nasdaq.com/docs/MRXSystemSettings  
https://nasdaqtrader.com/Content/BXOptions/BXOptions_FAQs.pdf  
https://www.miaxglobal.com/markets/us-options/all-options-exchanges/trade-hours-calendar  
https://boxexchange.com/assets/BOX-Exchange-Quoting-Requirements-Summary_10.15.pdf  
https://info.memxtrading.com/wp-content/uploads/2023/05/MEMX-Options-User-Manual.pdf

This family has no tradeable session outside 09:30–16:00: every non-regular  
window a venue publishes here is a pre-open order-acceptance queue, so  
`extended` is empty and the queue lands in `order_entry`.

## Module narrative (moved from src/calendar/schedules/equities/us/options/history.rs on 2026-09-12 UTC)

Launch evidence for venues that began after the January-2010 audit floor.  
BZX launched 2010-02-26 with cash-equity underliers among its 18 classes:  
https://cdn.cboe.com/resources/press_releases/BATSOptionsGoesLive_FINAL.pdf  
C2 launched 2010-10-29 with Ford as its first class:  
https://cdn.cboe.com/resources/regulation/circulars/general/IC-CBOE-2010-168.pdf  
EDGX launched phase one on 2015-11-02; its schedule lists five stock classes:  
https://cdn.cboe.com/resources/release_notes/2015/BATS-EDGX-Options-Update-2015_11_10.pdf  
https://cdn.cboe.com/resources/edgx_options/EDGX_Options_Symbol_Rollout_Schedule.xlsx  
BX launched 2012-06-29 with five stock classes, including AA and INTC:  
https://www.nasdaqtrader.com/MicroNews.aspx?id=OTA2012-41  
GEMX's audited filing calls it an equity-and-index-options exchange and  
states that it formally commenced trading on 2013-08-05:  
https://www.sec.gov/Archives/edgar/vprr/1601/16019242.pdf  
MRX initiated trading on 2016-02-16; the operator reports an equity-and-ETF  
rollout, while the SEC filing independently fixes the exact launch day:  
https://www.deutsche-boerse.com/resource/blob/324026/912f25fc1b9e0cdb916acbd69d4013fb/data/Detailed_volume_statistics_are_found_in_the_following_document-1.pdf  
https://www.sec.gov/files/rules/sro/bats/2016/34-77256.pdf  
MIAX launched 2012-12-07 with stock class CLF:  
https://www.miaxglobal.com/alert/2012/12/06/miax-options-will-commence-trading-friday-december-7-2012  
MIAX Pearl launched 2017-02-06 with IBM:  
https://www.miaxglobal.com/alerts/2017/02/01/market-underlying-security-used-openings-miax-pearl-newly-listed-option-class  
MIAX Emerald launched 2019-03-01 with IBM:  
https://www.miaxglobal.com/news/miax-emerald-successfully-launches-trading-operations  
MIAX Sapphire launched 2024-08-12 with IBM:  
https://www.miaxglobal.com/sites/default/files/alert-files/MIAX_Press_Release_09102024.pdf  
MEMX launched 2023-09-27 with stock classes SBUX and IMGN (plus GLD):  
https://info.memxtrading.com/trader-alert-23-42-memx-options-exchange-schedule-update/

Row evidence — each table's launch day mapped to its primary source:  
  2010-02-26 "BATS Options launch press release" (BZX)  
    https://cdn.cboe.com/resources/press_releases/BATSOptionsGoesLive_FINAL.pdf  
  2010-10-29 "Cboe circular IC-CBOE-2010-168" (C2)  
    https://cdn.cboe.com/resources/regulation/circulars/general/IC-CBOE-2010-168.pdf  
  2015-11-02 "Bats EDGX options update 2015-11-10" (EDGX)  
    https://cdn.cboe.com/resources/release_notes/2015/BATS-EDGX-Options-Update-2015_11_10.pdf  
  2012-06-29 "Nasdaq OTA 2012-41" (BX)  
    https://www.nasdaqtrader.com/MicroNews.aspx?id=OTA2012-41  
  2013-08-05 "SEC filing 16019242" (GEMX)  
    https://www.sec.gov/Archives/edgar/vprr/1601/16019242.pdf  
  2016-02-16 "SEC 34-77256" (MRX)  
    https://www.sec.gov/files/rules/sro/bats/2016/34-77256.pdf  
  2012-12-07 "MIAX launch alert 2012-12-06"  
    https://www.miaxglobal.com/alert/2012/12/06/miax-options-will-commence-trading-friday-december-7-2012  
  2017-02-06 "MIAX Pearl launch alert 2017-02-01"  
    https://www.miaxglobal.com/alerts/2017/02/01/market-underlying-security-used-openings-miax-pearl-newly-listed-option-class  
  2019-03-01 "MIAX Emerald launch announcement"  
    https://www.miaxglobal.com/news/miax-emerald-successfully-launches-trading-operations  
  2024-08-12 "MIAX press release 2024-09-10" (Sapphire)  
    https://www.miaxglobal.com/sites/default/files/alert-files/MIAX_Press_Release_09102024.pdf  
  2023-09-27 "MEMX trader alert 23-42"  
    https://info.memxtrading.com/trader-alert-23-42-memx-options-exchange-schedule-update/  
QUEUE CARRY-BACK, DECIDED 2026-09-01 — READ THIS BEFORE TRUSTING A HISTORICAL  
QUEUE ANSWER. Each venue's current order-acceptance queue is now served across  
its whole modelled history rather than only from the repository review date.  
The queues are `order_entry`: nothing matches in them, and every venue's  
09:30-16:00 execution history is sourced independently and unaffected.

THE ASSUMPTION, STATED PLAINLY. No primary source says when any of these  
queues began. They are operator *system settings* published on mutable  
hours/system-settings pages, not rulebook boundaries with filed operative  
dates — SR-C2-2019-009 and SR-CboeBZX-2020-012 each write down 07:30 as "the  
same time at which the System begins accepting orders and quotes today" while  
declining to change it, and Cboe Options Rule 6.2(a) bounds the pre-opening  
period rather than fixing it. Carrying the queue back therefore asserts  
continuity that no document states. It is a deliberate, recorded choice, not  
a sourced fact: a venue almost certainly accepted orders before its open, and  
under-reporting order acceptance for sixteen years was judged the worse error.

WHERE THE ASSUMPTION IS NOT MADE. MIAX Options is the counterexample and is  
modelled from evidence instead. Its 07:30 window existed at the sourced  
2012-12-07 launch but was connectivity verification only — the official hours  
page captured 2012-12-09 says activity before the Live Quote Window "WILL NOT  
affect the live quote state" — and the next capture, 2013-05-07, says it WILL  
affect the live book. So MIAX carries a queue-free launch row and gains the  
queue at that second capture, not at launch.

The three evidence classes below:  
  * no launch inside the window (C1, NYSE Arca/American, PHLX, ISE, NOM, BOX)  
    — queue carried from the January-2010 audit floor, assumption applies;  
  * launch-dated (C2, BZX, EDGX, BX, GEMX, MRX, MIAX Pearl/Emerald/Sapphire)  
    — queue carried from the sourced launch day, assumption applies;  
  * MIAX Options — sourced on both sides, no assumption.  
MEMX Options is outside all three: it has no queue at all, rejecting orders  
before 09:30, so its sourced launch row is its only row.

Venues whose execution history predates the audit floor and whose only  
timeline row is the knowledge bound: the dated baseline is the queue-less  
09:30–16:00 grid, and the 2026-08-22 row applies the verified-current  
order-acceptance queue.
