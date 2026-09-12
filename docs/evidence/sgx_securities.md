<!-- SPDX-License-Identifier: MIT-0 -->

# `sgx_securities` — evidence

- **Kind:** `Exchange`
- **Owner module:** [`sgx.rs`](../../src/calendar/schedules/equities/apac/sgx.rs)
- **Source sets:** [`APAC-SGX-SECURITIES`](../schedules/sources.md#apac-sgx-securities)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

SGX-ST normal week; sourced 2011/2017/2019 phases.

## Revision rows

- 2011-08-01 — T1 — SGX-ST Rules 2011-08-01 — continuous all-day trading 09:00–17:00; the midday break is removed.
- 2017-11-13 — T1 — SGX announcement 2017-07-18 — the midday break returns: regular 09:00–12:00 and 13:00–17:00 with a 12:00–13:00 routine.
- 2019-06-03 — T1 — SGX announcement 2019-05-14 — Trade at Close extends the closing tail to 17:16.

## Sources

Row review: 2026-08-22 (UTC) is the date the ledger row was last reviewed as a
whole. Per-source retrieval dates were not recorded before the 2026-09-12
migration; where a later targeted review, capture or document date is recorded
beside a source below, that date governs for that source, and dates are added
as each source is re-verified.

- <https://rulebook.sgx.com/rulebook/regulatory-notice-821-trading-hours-market-phases-application-market-phases-and-principles> — SGX-ST Regulatory Notice 8.2.1. Every routine has a Pre-Open/Pre-Close Phase that "allows order entry, order modification and withdrawal of orders but no matching of orders", and a Non-Cancel Phase in which "all existing orders that can be matched are matched at a single price".
- <https://rulebook.sgx.com/sites/default/files/net_file_store/SGX_ST_Rules_August_1_2011.pdf> — SGX-ST Rules as at 2011-08-01, introducing continuous all-day trading. Its Practice Note 8.2.1 carries the pre-2017 routine boundaries used by the two oldest profiles: Pre-Open 08:30–08:59 / Non-Cancel 08:59–09:00, lunch-break Adjust 12:30–13:59 with no matching and its 13:59–14:00 match, and Pre-Close 17:00–17:05 / Non-Cancel 17:05–17:06.
- <https://links.sgx.com/1.0.0/corporate-announcements/AYXNAX3DG8RCFZT7/20170718_SGX_to_adjust_equities_market_structure_after_supportive_feedback.pdf> — SGX announcement of 2017-07-18, restoring the midday break from 2017-11-13.
- <https://links.sgx.com/1.0.0/corporate-announcements/46OQY4VBYIHO4ARN/20190514_SGX_to_launch_securities_market_trade_at_close_session_on_3_June.pdf> — SGX announcement of 2019-05-14, launching Trade at Close on 2019-06-03.

## Gaps and residual risks

- **horizon carried below the first dated row** — the pre-2011-08-01 session bounds (09:00–12:30 and 14:00–17:00) are not attested by any artifact named in the repository; the 2011-08-01 rulebook supplies only the routine phase boundaries carried by the two oldest profiles. The ledger horizon is therefore 2011-08-01, the first day at which this row's state is sourced, with everything below it carried. Closing condition: a dated pre-2011 SGX-ST rulebook or practice-note edition stating the lunch-break session bounds, which would move the horizon earlier.
- Current routine ends are randomized: Pre-Open ends 08:58–08:59 and 12:58–12:59, Pre-Close ends 17:04–17:05. Each order-entry slice stops at the earliest possible end so no matching time is claimed as order entry.
- Trade at Close matches at the Equilibrium Price and is therefore tradeable throughout its window.
