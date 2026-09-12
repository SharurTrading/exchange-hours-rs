<!-- SPDX-License-Identifier: MIT-0 -->

# `miax_options` — evidence

- **Kind:** `Exchange`
- **Owner module:** [`options.rs`](../../src/calendar/schedules/equities/us/options.rs), [`history.rs`](../../src/calendar/schedules/equities/us/options/history.rs)
- **Source sets:** [`US-MIAX`](../schedules/sources.md#us-miax)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

**Gap: order-entry** — the trading session is sourced; what is undated is a queue or post-close phase in which no trade can print. Closed before the sourced 2012-12-07 launch. **This row makes no carry-back assumption** — it is the counterexample the other sixteen are measured against, sourced on both sides: the launch-era queue-free row is positively sourced and the queue is served from 2013-05-07, the first capture showing the window affecting the live book, not from launch. The official hours page captured 2012-12-09 states that pre-Live-Quote-Window activity "WILL NOT affect the live quote state", while the next capture (2013-05-07) states it WILL affect the live book. The order-acceptance onset therefore falls in 2012-12-09..2013-05-07 and no archived operator alert states the day. The start is an operator system setting on a mutable hours/system-settings page, not a rulebook boundary carrying a filed operative date, and no operator channel publishes a dated change notice for it. Unlike the sixteen carried rows, this row does **not** carry the queue across history: `MIAX_REVISIONS` in `options/history.rs` routes 2012-12-07 to the queue-free `LISTED_EQUITY_OPTIONS_HISTORICAL` profile and only the 2013-05-07 row to `MIAX_OPTIONS_PROFILE`, so the queue starts at 2013-05-07 and the disputed 2012-12-09..2013-05-07 remainder is withheld. The bracket affects order acceptance only, since nothing matches in a queue and the 09:30–16:00 execution history is sourced independently.

## Revision rows

- 2012-12-07 — T1 — MIAX launch alert 2012-12-06 — MIAX Options launch on a queue-free 09:30-16:00 ET grid; the 07:30 window existed but was connectivity verification only.
- 2013-05-07 — T1 — first capture showing the window affecting the live book — the 07:30 ET window becomes an order-acceptance queue that affects the live book.

## Sources

Row review: 2026-08-22 (UTC) is the date the ledger row was last reviewed as a
whole. Per-source retrieval dates were not recorded before the 2026-09-12
migration; where a later targeted review, capture or document date is recorded
beside a source below, that date governs for that source, and dates are added
as each source is re-verified.

- <https://www.miaxglobal.com/alert/2012/12/06/miax-options-will-commence-trading-friday-december-7-2012> — MIAX launch alert; MIAX Options launched 2012-12-07 with stock class CLF.
- <https://www.miaxglobal.com/markets/us-options/miax-options/trade-hours-calendar> — MIAX Options trade hours calendar; the current 07:30 ET order-acceptance queue and 09:30-16:00 ET session.
- <https://www.miaxglobal.com/markets/us-options/all-options-exchanges/trade-hours-calendar> — MIAX all-options-exchanges trade hours calendar; the operator system setting behind the queue.

The queue carry-back was decided on 2026-09-01, after the 2026-08-22 source-set review;
that decision date is not a review date. The retrieved bytes live in the
research store beside the repository.

## Gaps and residual risks

- **Raised in review of the ledger-reshape PR (#87), 2026-09-12 — a first-capture date is used as a revision boundary.** The 2013-05-07 row's citation literal is "first capture showing the window affecting the live book", and `MIAX_REVISIONS` in `src/calendar/schedules/equities/us/options/history.rs` routes the queue-carrying profile from that day. A capture date is the date of an *observation*, not an operator-stated effective day; LAW-NO-FABRICATED-DATES admits a cutover only on an unconditional day-level date at the required tier. The row is defensible as the conservative edge of the sourced 2012-12-09..2013-05-07 bracket — it withholds the disputed remainder and under-reports order acceptance rather than over-reporting it — but it is a knowledge boundary wearing a revision row's clothes, and the same objection applies to `miax_emerald_options`' 2019-03-01 launch-row queue. The reshape PR moved this text out of the owner module and changed no schedule rule, revision row, profile or routing. Closing condition: an operator-dated onset for the queue, or a later MIAX schedule change that restates the queue and lets this row be re-keyed as an explicit knowledge-bound row with the `"<date> review: verified current, onset undated"` citation form `updating.md` defines. Dormant identity, so recorded here rather than opened as an issue (LAW-FOLLOW-UPS-ARE-ISSUES).
- **order-entry** — the day the 07:30 ET window became a live order-acceptance queue is undated. It is bracketed, not carried: the official hours page captured 2012-12-09 states that activity before the Live Quote Window "WILL NOT affect the live quote state", and the next capture, 2013-05-07, states that it WILL affect the live book, so the true onset lies in 2012-12-09..2013-05-07. The crate serves the queue from the later capture, withholding the disputed remainder rather than granting it early. Closing condition: an archived MIAX alert or hours page stating the change in session language on a day-level effective date. Dormant identity, so the gap is recorded here rather than opened as an issue (LAW-FOLLOW-UPS-ARE-ISSUES).
- **below the floor** — the 2006 coordinated SRO rule changes moved individual-stock options from a 16:02 to a 16:00 ET close before the January-2010 history floor, and that change is out of scope by design (LAW-NO-FABRICATED-DATES records amendment history back to January 2010). The 2006 filings are cited here because they are what sources the 09:30-16:00 ET grid through the floor.
- **scope** — ETF, ETN, index, FLEX, floor-only and venue-designated extended-hours option classes are separate product families because their executable sessions vary, and this row does not cover them.

> Shared module. `options.rs` and `options/history.rs` carry all eighteen US
> listed-equity-options identities, so their narrative is written once, in the
> anchor identity's file:
> [`options.rs`](cboe_options_c1.md#module-narrative-moved-from-srccalendarschedulesequitiesusoptionsrs-on-2026-09-12-utc)
> and
> [`options/history.rs`](cboe_options_c1.md#module-narrative-moved-from-srccalendarschedulesequitiesusoptionshistoryrs-on-2026-09-12-utc).
>
> Sibling identities: [`cboe_options_c1`](cboe_options_c1.md), [`cboe_c2_options`](cboe_c2_options.md), [`cboe_bzx_options`](cboe_bzx_options.md), [`cboe_edgx_options`](cboe_edgx_options.md), [`nyse_arca_options`](nyse_arca_options.md), [`nyse_american_options`](nyse_american_options.md), [`nasdaq_phlx`](nasdaq_phlx.md), [`nasdaq_ise`](nasdaq_ise.md), [`nasdaq_nom`](nasdaq_nom.md), [`nasdaq_mrx`](nasdaq_mrx.md), [`nasdaq_gemx`](nasdaq_gemx.md), [`nasdaq_bx_options`](nasdaq_bx_options.md), [`miax_emerald_options`](miax_emerald_options.md), [`miax_pearl_options`](miax_pearl_options.md), [`miax_sapphire_options`](miax_sapphire_options.md), [`box_options`](box_options.md), [`memx_options`](memx_options.md).
