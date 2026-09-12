<!-- SPDX-License-Identifier: MIT-0 -->

# `sse` — evidence

- **Kind:** `Exchange`
- **Owner module:** [`sse.rs`](../../src/calendar/schedules/equities/apac/sse.rs)
- **Source sets:** [`APAC-CHINA-CASH`](../schedules/sources.md#apac-china-cash)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

Venue union includes the 15:00–15:30 block-order/trading phase already operative at the January-2010 floor. Later STAR eligibility does not create a new outer-envelope cutover.

## Revision rows

- 2018-08-20 — T1 — SSE news release 4947833 — the 14:57–15:00 closing call auction is added, so continuous trading ends 14:57.

## Sources

- <https://www.sse.com.cn/lawandrules/sselawsrules2025/stocks/exchange/c/c_20260424_10816482.shtml> — current SSE trading rule, including the 15:00–15:30 block and fixed-price phase.
- <https://www.sse.com.cn/lawandrules/sselawsrules2025/repeal/rules/c/c_20120918_10785158.shtml> — historical SSE block-trading rule, establishing that declarations have been accepted and confirmed through 15:30 since before the January-2010 audit floor.
- <https://english.sse.com.cn/news/newsrelease/c/4947833.shtml> — SSE news release 4947833, adding the closing call auction on 2018-08-20.

## Gaps and residual risks

- Block and fixed-price phases are `extended` by convention; not every security is eligible for them.
- The 2026-07-06 generic fixed-price expansion is deliberately not a revision row: it changed eligibility inside the existing venue envelope, not the exchange-level close.
- The pre-2018 profile's 15:00–15:30 block window is carried from the historical block-trading rule; the ledger records the venue union as January-2010-on for that reason.
