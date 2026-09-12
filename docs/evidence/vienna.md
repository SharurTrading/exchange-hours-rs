<!-- SPDX-License-Identifier: MIT-0 -->

# `vienna` — evidence

- **Kind:** `Exchange`
- **Owner module:** [`vienna.rs`](../../src/calendar/schedules/equities/europe/vienna.rs)
- **Source sets:** [`EU-VIENNA`](../schedules/sources.md#eu-vienna), [`EU-FESE-SECONDARY`](../schedules/sources.md#eu-fese-secondary)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

ATX January-2010 phases, recurring third-Friday settlement grid, 2017 T7 migration, 2019-05-02 closing extension, and 2020-12-01 Trade-at-Close launch are primary-sourced.

## Revision rows

Vienna's eras are expressed as dated `NaiveDate` constants in `vienna.rs` rather than a `revisions!` table, because each era has a separate ordinary-day and derivatives-settlement-day profile and the selector chooses between them per date. The dated era boundaries are:

- 2017-07-31 — T1 — Wiener Börse Xetra T7 detailed specification — ATX equities migrate to Xetra T7; the legacy market-balancing phases are removed, ordinary-day continuous trading starts at the latest random edges 09:00:30 and 12:03:30, settlement days resume at 12:05:30.
- 2019-05-02 — T1 — Wiener Börse quarterly news release — the ATX closing call is extended by two minutes, pushing post-trading back to 17:35:30.
- 2020-12-01 — T1 — Wiener Börse "Vienna Stock Exchange extends trading hours" — Trade-at-Close launches, making the official closing price executable through 17:45 and moving post-trading to 17:45–17:50.

## Sources

- <https://web.archive.org/web/20090219151827id_/http://en.wienerborse.at/static/cms/sites/wbag/media/en/pdf/marketplace_products/feinspez_xetra_marktmodell.pdf> — Wiener Börse detailed Xetra 9.1 specification, effective 2009-01-02: the January-2010 ATX baseline. On ordinary days the auction market-balancing phases delayed continuous trading until 09:01 and 12:04 and the closing auction ended at 17:34; on derivatives-settlement days the corresponding latest boundaries were 09:02:30, 12:07:30 and 17:35:30.
- <https://web.archive.org/web/20150529063952id_/http://en.wienerborse.at/static/cms/sites/wbag/media/en/pdf/marketplace_products/feinspez_xetra_marktmodell.pdf> — archived Wiener Börse specification capture: the same tables remain in the 2012, 2014 and 2015 editions.
- <https://www.wienerborse.at/uploads/u/cms/files/trading/xetra-detailed-specifications-market-models.pdf> — Wiener Börse Xetra detailed specifications and market models, the T7 migration edition.
- <https://web.archive.org/web/20180214144727id_/https://www.wienerborse.at/en/trading/trading-information/trading-hours/> — archived 2018 Wiener Börse hours page, recording the pre-extension three-minute closing call; its maximum 30-second random period makes 17:33:30 the deterministic boundary.
- <https://www.wienerborse.at/en/news/vienna-stock-exchange-news/5-new-austrian-listings-in-q1-equity-turnover-reclining-throughout-europe-due-to-brexit/> — Wiener Börse news item recording the two-minute closing-call extension effective 2019-05-02.
- <https://web.archive.org/web/20200610172528id_/https://www.wienerborse.at/uploads/u/cms/files/trading/xetra-t7-detailed-specifications-market-models.pdf> — archived Wiener Börse detailed 2020 specification: the exact five-minute call, maximum 30-second random end, and post-trading through 17:45.
- <https://www.wienerborse.at/en/news/vienna-stock-exchange-news/vienna-stock-exchange-extends-trading-hours/> — Wiener Börse, "Vienna Stock Exchange extends trading hours": Trade-at-Close launch on 2020-12-01.
- <https://web.archive.org/web/20210127203612id_/https://www.wienerborse.at/en/trading/trading-information/trading-hours/> — archived 2021 Wiener Börse hours page, confirming the post-Trade-at-Close phases.
- <https://www.wienerborse.at/uploads/u/cms/files/trading/xetra-t7-detailed-specifications-market-models.pdf> — current Wiener Börse Xetra T7 detailed specifications and market models. Of the product state Pre-Trading it says it "is typically a time where traders may maintain their orders prior to the start of trading. No matching occurs in this phase", and of Post-Trading that traders "can maintain their orders in preparation of the next trading day. No matching occurs in this phase".
- <https://www.wienerborse.at/en/trading/trading-information/trading-hours/> — Wiener Börse trading hours, the source set's current entry point.
- <https://www.wienerborse.at/handel/handelsinformationen/handelszeiten/> — the current German hours table.
- <https://www.wienerborse.at/en/trading/trading-information/trading-system/> — Wiener Börse trading-system and model hub, the monitoring entry point.
- <https://www.fese.eu/app/uploads/2024/07/trading-hours-2025-1.pdf> — FESE 2025 trading-hours table, `EU-FESE-SECONDARY`: corroboration only.

## Gaps and residual risks

- **Recurring settlement grid.** Derivatives-settlement days — the Friday falling on the 15th to the 21st of a month — carry a longer midday call in every era. This is a recurring grid rule, not a holiday, and it is evaluated inside `profile_at` rather than as a revision row.
- **Timeline shape.** Because each era has two profiles, the module uses `effective_date` constants and an ordered `if` chain instead of `revisions!`. Any fence that derives revision days from `revisions![` blocks will find none in `vienna.rs`; the three dated days above are the days it must check.
- **Interpretive step, order-entry classification.** Pre-trading and post-trading are `order_entry` on the detailed specification's own "no matching occurs in this phase" language, with instruments in the closed instrument state Book. Auction windows cover a call phase and its price determination, which prints, so they stay `extended`.
- **Interpretive step, randomized uncrosses.** Auctions include their full maximum 30-second random period, so continuous trading begins at the latest sourced edge in each era.
- **Dormant identity.** Reviewed on demand; gaps are recorded here rather than as issues.
