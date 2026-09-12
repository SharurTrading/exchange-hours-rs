<!-- SPDX-License-Identifier: MIT-0 -->

# `cboe_c2_options` — evidence

- **Kind:** `Exchange`
- **Owner module:** [`options.rs`](../../src/calendar/schedules/equities/us/options.rs), [`history.rs`](../../src/calendar/schedules/equities/us/options/history.rs)
- **Source sets:** [`US-CBOE-OPTIONS`](../schedules/sources.md#us-cboe-options)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

**Gap: order-entry** — the trading session is sourced; what is undated is a queue or post-close phase in which no trade can print. Closed before the sourced 2010-10-29 launch; current envelope includes the 07:30 queue. SR-C2-2019-009 (84 FR 20673, 2019-05-10) records the queuing period beginning at 07:30 as "the same time at which the System begins accepting orders and quotes today", so the queue is sourced at least that far back; its exact onset remains undated. The start is an operator system setting on a mutable hours/system-settings page, not a rulebook boundary carrying a filed operative date, and no operator channel publishes a dated change notice for it. As of 2026-09-01 the queue is therefore **carried across history** rather than withheld — from the January-2010 floor where the venue predates it, or from its sourced launch day — which asserts continuity no document states. That assumption is deliberate and is recorded beside the profile and in `options/history.rs`; it affects order acceptance only, since nothing matches in a queue and the 09:30–16:00 execution history is sourced independently.

## Revision rows

- 2010-10-29 — T1 — Cboe circular IC-CBOE-2010-168 — C2 Options launch; the queue-carrying 07:30 and 09:30-16:00 ET profile takes effect over the pre-launch closure.

## Sources

Retrieval dates: these sources were last opened on the row's reviewed-on date
(2026-08-22, UTC); per-source retrieval dates were not recorded before the
2026-09-12 migration and are added as each source is re-verified.

- <https://cdn.cboe.com/resources/regulation/circulars/general/IC-CBOE-2010-168.pdf> — Cboe circular IC-CBOE-2010-168; C2 launched 2010-10-29 with Ford as its first class.
- <https://cdn.cboe.com/resources/regulation/rule_book/C2_Exchange_Rule_Book.pdf> — Cboe C2 Exchange Rule Book; retains 09:30-16:00 ET RTH for this product family.
- <https://www.cboe.com/about/hours/us-options> — Cboe US options hours page; the operator system setting behind the 07:30 ET order-acceptance queue.
- SR-C2-2019-009 (84 FR 20673, 2019-05-10) — records the queuing period beginning at 07:30 as "the same time at which the System begins accepting orders and quotes today" while declining to change it; the owner module carries no URL for it.

The queue carry-back was decided on 2026-09-01, after the 2026-08-22 source-set review;
that decision date is not a review date. The retrieved bytes live in the
research store beside the repository.

## Gaps and residual risks

- **order-entry** — the day the 07:30 ET order-acceptance queue began is undated. No primary source states it: the queue is an operator *system setting* published on a mutable hours or system-settings page, not a rulebook boundary with a filed operative date. As of 2026-09-01 the queue is carried from this venue's sourced launch row rather than withheld, which asserts continuity no document states. Nothing matches in the queue, so the 09:30-16:00 ET execution history is unaffected. Closing condition: an operator artifact stating the queue's start in session language on a day-level effective date. Dormant identity, so the gap is recorded here rather than opened as an issue (LAW-FOLLOW-UPS-ARE-ISSUES).
- **below the floor** — the 2006 coordinated SRO rule changes moved individual-stock options from a 16:02 to a 16:00 ET close before the January-2010 history floor, and that change is out of scope by design (LAW-NO-FABRICATED-DATES records amendment history back to January 2010). The 2006 filings are cited here because they are what sources the 09:30-16:00 ET grid through the floor.
- **scope** — ETF, ETN, index, FLEX, floor-only and venue-designated extended-hours option classes are separate product families because their executable sessions vary, and this row does not cover them.

> Shared module. `options.rs` and `options/history.rs` carry all eighteen US
> listed-equity-options identities, so their narrative is written once, in the
> anchor identity's file:
> [`options.rs`](cboe_options_c1.md#module-narrative-moved-from-srccalendarschedulesequitiesusoptionsrs-on-2026-09-12-utc)
> and
> [`options/history.rs`](cboe_options_c1.md#module-narrative-moved-from-srccalendarschedulesequitiesusoptionshistoryrs-on-2026-09-12-utc).
>
> Sibling identities: [`cboe_options_c1`](cboe_options_c1.md), [`cboe_bzx_options`](cboe_bzx_options.md), [`cboe_edgx_options`](cboe_edgx_options.md), [`nyse_arca_options`](nyse_arca_options.md), [`nyse_american_options`](nyse_american_options.md), [`nasdaq_phlx`](nasdaq_phlx.md), [`nasdaq_ise`](nasdaq_ise.md), [`nasdaq_nom`](nasdaq_nom.md), [`nasdaq_mrx`](nasdaq_mrx.md), [`nasdaq_gemx`](nasdaq_gemx.md), [`nasdaq_bx_options`](nasdaq_bx_options.md), [`miax_options`](miax_options.md), [`miax_emerald_options`](miax_emerald_options.md), [`miax_pearl_options`](miax_pearl_options.md), [`miax_sapphire_options`](miax_sapphire_options.md), [`box_options`](box_options.md), [`memx_options`](memx_options.md).
