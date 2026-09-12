<!-- SPDX-License-Identifier: MIT-0 -->

# `twse` — evidence

- **Kind:** `Exchange`
- **Owner module:** [`twse.rs`](../../src/calendar/schedules/equities/apac/twse.rs)
- **Source sets:** [`APAC-TWSE`](../schedules/sources.md#apac-twse)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

Venue union is 08:00–17:00 across paired-block acceptance, regular pre-open/continuous trading, and block trading. Primary pre-scope evidence establishes the block tail at the January-2010 floor; the 2020 continuous-trading launch remains date-aware.

## Revision rows

- 2020-03-23 — T1 — TWSE company history page — continuous matching replaces the five-second call auctions in the central order book, so the 09:00–13:25 session becomes `regular`.

## Sources

Row review: 2026-08-22 (UTC) is the date the ledger row was last reviewed as a
whole. Per-source retrieval dates were not recorded before the 2026-09-12
migration; where a later targeted review, capture or document date is recorded
beside a source below, that date governs for that source, and dates are added
as each source is re-verified.

- <https://www.twse.com.tw/en/products/system/trading.html> — TWSE trading-system page, splitting the 08:00–17:00 envelope: paired block trading 08:00–08:30 executing on quoted terms, then non-paired and paired block trading 09:00–17:00, with 08:30–09:00 the regular session's order-placing window whose call-auction match lands at 09:00.
- <https://www.twse.com.tw/en/about/company/history.html> — TWSE company history, dating the continuous-trading launch to 2020-03-23.
- <https://www.twse.com.tw/en/about/company/guide.html> — TWSE investor guide, stating that block trading had already been expanded to the present 08:00/17:00 envelope at the beginning of 2009, before the January-2010 audit floor.

## Gaps and residual risks

- The investor guide states "the beginning of 2009" rather than a day-level effective date, so no pre-floor effective day is encoded (LAW-NO-FABRICATED-DATES). Nothing below the January-2010 floor is reviewed in any case.
- The 08:30–09:00 window is `order_entry`: it is the regular session's order-placing window, no block-trading window is open in it, and the first match is the 09:00 opening call.
- Before 2020-03-23 the central book's intraday matches were call auctions every five seconds, so that primary session is `extended` rather than mislabelled `regular`; the outer envelope is unchanged.
- Specialized block, odd-lot, auction and after-hours methods are classified `extended`; not every security is eligible for every phase.
