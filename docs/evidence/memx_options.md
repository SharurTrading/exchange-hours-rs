<!-- SPDX-License-Identifier: MIT-0 -->

# `memx_options` — evidence

- **Kind:** `Exchange`
- **Owner module:** [`options.rs`](../../src/calendar/schedules/equities/us/options.rs), [`history.rs`](../../src/calendar/schedules/equities/us/options/history.rs)
- **Source sets:** [`US-MEMX`](../schedules/sources.md#us-memx)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

Same ordinary individual-stock-options scope; closed before the sourced 2023-09-27 launch, then 09:30–16:00.

## Revision rows

- 2023-09-27 — T1 — MEMX trader alert 23-42 — MEMX Options launch on the 09:30-16:00 ET grid with no pre-open order-acceptance window.

## Sources

Row review: 2026-08-22 (UTC) is the date the ledger row was last reviewed as a
whole. Per-source retrieval dates were not recorded before the 2026-09-12
migration; where a later targeted review, capture or document date is recorded
beside a source below, that date governs for that source, and dates are added
as each source is re-verified.

- <https://info.memxtrading.com/trader-alert-23-42-memx-options-exchange-schedule-update/> — MEMX trader alert 23-42; MEMX Options launched 2023-09-27 with stock classes SBUX and IMGN plus GLD.
- <https://info.memxtrading.com/market-hours-and-holiday-schedule/> — MEMX market hours and holiday schedule; 09:30-16:00 ET with no pre-open order-acceptance window.
- <https://info.memxtrading.com/wp-content/uploads/2023/05/MEMX-Options-User-Manual.pdf> — MEMX Options user manual; MEMX rejects orders before 09:30 ET.

The queue carry-back was decided on 2026-09-01, after the 2026-08-22 source-set review;
that decision date is not a review date. The retrieved bytes live in the
research store beside the repository.

## Gaps and residual risks

- No modeled-history gap. MEMX Options publishes no pre-open order-acceptance window and its user manual states that orders are rejected before 09:30 ET, so the queue carry-back assumption that qualifies the other seventeen US options rows is not made here and nothing is carried. Residual risk: MEMX could introduce an acceptance window without a dated notice, and the on-demand review a dormant identity gets is what would catch it.
- **below the floor** — the 2006 coordinated SRO rule changes moved individual-stock options from a 16:02 to a 16:00 ET close before the January-2010 history floor, and that change is out of scope by design (LAW-NO-FABRICATED-DATES records amendment history back to January 2010). The 2006 filings are cited here because they are what sources the 09:30-16:00 ET grid through the floor.
- **scope** — ETF, ETN, index, FLEX, floor-only and venue-designated extended-hours option classes are separate product families because their executable sessions vary, and this row does not cover them.

> Shared module. `options.rs` and `options/history.rs` carry all eighteen US
> listed-equity-options identities, so their narrative is written once, in the
> anchor identity's file:
> [`options.rs`](cboe_options_c1.md#module-narrative-moved-from-srccalendarschedulesequitiesusoptionsrs-on-2026-09-12-utc)
> and
> [`options/history.rs`](cboe_options_c1.md#module-narrative-moved-from-srccalendarschedulesequitiesusoptionshistoryrs-on-2026-09-12-utc).
>
> Sibling identities: [`cboe_options_c1`](cboe_options_c1.md), [`cboe_c2_options`](cboe_c2_options.md), [`cboe_bzx_options`](cboe_bzx_options.md), [`cboe_edgx_options`](cboe_edgx_options.md), [`nyse_arca_options`](nyse_arca_options.md), [`nyse_american_options`](nyse_american_options.md), [`nasdaq_phlx`](nasdaq_phlx.md), [`nasdaq_ise`](nasdaq_ise.md), [`nasdaq_nom`](nasdaq_nom.md), [`nasdaq_mrx`](nasdaq_mrx.md), [`nasdaq_gemx`](nasdaq_gemx.md), [`nasdaq_bx_options`](nasdaq_bx_options.md), [`miax_options`](miax_options.md), [`miax_emerald_options`](miax_emerald_options.md), [`miax_pearl_options`](miax_pearl_options.md), [`miax_sapphire_options`](miax_sapphire_options.md), [`box_options`](box_options.md).
