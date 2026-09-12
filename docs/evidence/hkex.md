<!-- SPDX-License-Identifier: MIT-0 -->

# `hkex` — evidence

- **Kind:** `Exchange`
- **Owner module:** [`hkex.rs`](../../src/calendar/schedules/equities/apac/hkex.rs)
- **Source sets:** [`APAC-HKEX`](../schedules/sources.md#apac-hkex)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

Securities venue union: the executable Extended Morning Session is Regular under crate semantics and bridges the former lunch gap. The 2011 open change and 2016-07-25 CAS tail are observable cutovers; 2012 only rearranged internal phases.

## Revision rows

- 2011-03-07 — T1 — HKEX news release 110303news — Phase One: morning session 09:30–12:00, Extended Morning Session 12:00–13:30, afternoon session 13:30–16:00, with the Pre-opening Session moved to 09:00–09:30.
- 2016-07-25 — T1 — HKEX market communication 160725news — the Closing Auction Session adds the 16:00–16:10 tail for its first eligible securities.

## Sources

Row review: 2026-08-22 (UTC) is the date the ledger row was last reviewed as a
whole. Per-source retrieval dates were not recorded before the 2026-09-12
migration; where a later targeted review, capture or document date is recorded
beside a source below, that date governs for that source, and dates are added
as each source is re-verified.

- <https://www.hkex.com.hk/Services/Trading-hours-and-Severe-Weather-Arrangements/Trading-Hours/Securities-Market?sc_lang=en> — HKEX securities-market trading hours: POS 09:00–09:30, continuous trading 09:30–16:00, then CAS with a randomized 16:08–16:10 close.
- <https://www.hkex.com.hk/Global/Exchange/FAQ/Securities-Market/Trading/Pre_opening-Session?sc_lang=en> — HKEX Pre-opening Session FAQ. Orders "will be accumulated and updated but no matching will occur" during the order input and pre-order matching periods, so 09:00–09:20 is order entry.
- <https://www.hkex.com.hk/-/media/HKEX-Market/Services/Rules-and-Forms-and-Fees/Rules/SEHK/Securities/Rule-Update_Rules-of-the-Exchange/05-11-SEHK-StampDuty-TradingHour_e.pdf> — SEHK rule update. Rule 501G divides the 09:00–09:30 POS into four named periods: order input 09:00–09:15, pre-order matching (renamed no-cancellation in 2020) 09:15–09:20, order matching from 09:20, then a blocking period to 09:30.
- <https://www.hkex.com.hk/News/News-Release/2011/110303news?sc_lang=en> — HKEX news release of 2011-03-03, announcing Phase One effective 2011-03-07 and stating the pre-change 10:00 morning open.
- <https://www.hkex.com.hk/News/Regulatory-Announcements/2012/120301news?sc_lang=en> — HKEX announcement of the 2012-03-05 Phase Two, which moved the internal Extended Morning/afternoon handoff to 13:00 without changing the envelope.
- <https://www.hkex.com.hk/News/Market-Communications/2016/160725news?sc_lang=en> — HKEX market communication, the 2016-07-25 CAS launch.

## Gaps and residual risks

- **horizon 2011-03-03** — the pre-2011 profile (10:00 open, POS 09:30–10:00) is attested only by the 2011-03-03 news release and the SEHK rule update that accompanied Phase One, so below 2011-03-03 the grid is carried back to the January-2010 floor rather than independently sourced (AGENTS.md, *Carry the earliest sourced state back to the floor*). Closing condition: a dated pre-2011 SEHK rulebook edition or trading-hours page.
- No primary SEHK text for the pre-2011-03-07 POS period boundaries was located, so the whole 09:30–10:00 window is left `extended` rather than guessing where its matching period began.
- The 2012-03-05 Phase Two is deliberately not a revision row: it rearranged internal phases without changing the venue-level open or close (LAW-HOLIDAY-SCOPE's companion rule on topology, and the ledger's own statement that it is not an observable envelope cutover).
- Later CAS eligibility expansions do not create new exchange-level open/close cutovers; the static profile uses the maximum scheduled CAS edge and not every security is eligible for every phase.
- The CAS 16:00–16:10 window ends in a randomised uncrossing that prints the closing trades, so the whole auction stays `extended`.
