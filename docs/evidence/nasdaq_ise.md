<!-- SPDX-License-Identifier: MIT-0 -->

# `nasdaq_ise` — evidence

- **Kind:** `Exchange`
- **Owner module:** [`options.rs`](../../src/calendar/schedules/equities/us/options.rs), [`history.rs`](../../src/calendar/schedules/equities/us/options/history.rs)
- **Source sets:** [`US-NASDAQ-OPTIONS`](../schedules/sources.md#us-nasdaq-options)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

**Gap: order-entry** — the trading session is sourced; what is undated is a queue or post-close phase in which no trade can print. Current generic envelope includes the 06:00 queue and 09:30–16:00 regular session, published in the Nasdaq ISE System Settings as "System begins accepting orders" and already at 06:00 in the 2019-10-17 "Nasdaq ISE INET System Settings" capture. The exact queue-onset day is unavailable. The start is an operator system setting on a mutable hours/system-settings page, not a rulebook boundary carrying a filed operative date, and no operator channel publishes a dated change notice for it. As of 2026-09-01 the queue is therefore **carried across history** rather than withheld — from the January-2010 floor where the venue predates it, or from its sourced launch day — which asserts continuity no document states. That assumption is deliberate and is recorded beside the profile and in `options/history.rs`; it affects order acceptance only, since nothing matches in a queue and the 09:30–16:00 execution history is sourced independently.

## Revision rows

None. options/history.rs holds a single static profile with no dated revision row.

## Sources

- <https://www.sec.gov/rules/sro/ise/2006/34-53248.pdf> — ISE 2006 rule change to the 16:00 ET close; the pre-floor baseline for the ISE execution grid.
- <https://listingcenter.nasdaq.com/rulebook/ise/rules/ISE%20Options%203> — ISE Options 3; retains 09:30-16:00 ET RTH for this product family.
- <https://www.nasdaq.com/docs/ISESystemSettings> — Nasdaq ISE System Settings; "System begins accepting orders" at 06:00 ET, and the 2019-10-17 "Nasdaq ISE INET System Settings" capture already shows 06:00.

The owner modules record no per-URL retrieval date. This row's source set was last
reviewed on 2026-08-22 and the queue carry-back was decided on 2026-09-01; the retrieved
bytes live in the research store beside the repository.

## Gaps and residual risks

- **order-entry** — the day the 06:00 ET order-acceptance queue began is undated. No primary source states it: the queue is an operator *system setting* published on a mutable hours or system-settings page, not a rulebook boundary with a filed operative date. As of 2026-09-01 the queue is carried from the January-2010 floor rather than withheld, which asserts continuity no document states. Nothing matches in the queue, so the 09:30-16:00 ET execution history is unaffected. Closing condition: an operator artifact stating the queue's start in session language on a day-level effective date. Dormant identity, so the gap is recorded here rather than opened as an issue (LAW-FOLLOW-UPS-ARE-ISSUES).
- **below the floor** — the 2006 coordinated SRO rule changes moved individual-stock options from a 16:02 to a 16:00 ET close before the January-2010 history floor, and that change is out of scope by design (LAW-NO-FABRICATED-DATES records amendment history back to January 2010). The 2006 filings are cited here because they are what sources the 09:30-16:00 ET grid through the floor.
- **scope** — ETF, ETN, index, FLEX, floor-only and venue-designated extended-hours option classes are separate product families because their executable sessions vary, and this row does not cover them.
- **timeline** — this venue's 09:30-16:00 ET execution history predates the January-2010 floor, so its timeline static in `options/history.rs` is empty and `select_revision` returns the baseline profile — the current grid together with the carried 06:00 ET order-acceptance queue — at every date the crate answers for. That is why `## Revision rows` above carries no bullet.

> Shared module. `options.rs` and `options/history.rs` carry all eighteen US
> listed-equity-options identities, so their narrative is written once, in the
> anchor identity's file:
> [`options.rs`](cboe_options_c1.md#module-narrative-moved-from-srccalendarschedulesequitiesusoptionsrs-on-2026-09-12-utc)
> and
> [`options/history.rs`](cboe_options_c1.md#module-narrative-moved-from-srccalendarschedulesequitiesusoptionshistoryrs-on-2026-09-12-utc).
>
> Sibling identities: [`cboe_options_c1`](cboe_options_c1.md), [`cboe_c2_options`](cboe_c2_options.md), [`cboe_bzx_options`](cboe_bzx_options.md), [`cboe_edgx_options`](cboe_edgx_options.md), [`nyse_arca_options`](nyse_arca_options.md), [`nyse_american_options`](nyse_american_options.md), [`nasdaq_phlx`](nasdaq_phlx.md), [`nasdaq_nom`](nasdaq_nom.md), [`nasdaq_mrx`](nasdaq_mrx.md), [`nasdaq_gemx`](nasdaq_gemx.md), [`nasdaq_bx_options`](nasdaq_bx_options.md), [`miax_options`](miax_options.md), [`miax_emerald_options`](miax_emerald_options.md), [`miax_pearl_options`](miax_pearl_options.md), [`miax_sapphire_options`](miax_sapphire_options.md), [`box_options`](box_options.md), [`memx_options`](memx_options.md).
