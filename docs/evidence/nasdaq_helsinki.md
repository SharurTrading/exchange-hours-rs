<!-- SPDX-License-Identifier: MIT-0 -->

# `nasdaq_helsinki` — evidence

- **Kind:** `Exchange`
- **Owner module:** [`nasdaq_nordics.rs`](../../src/calendar/schedules/equities/europe/nasdaq_nordics.rs)
- **Source sets:** [`EU-NASDAQ-NORDIC`](../schedules/sources.md#eu-nasdaq-nordic), [`EU-FESE-SECONDARY`](../schedules/sources.md#eu-fese-secondary)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

The operator's 2010 INET model supplies the baseline; the 2015-11-16 five-second randomized auction edge and current post-trading phase are date-aware.

## Revision rows

- 2015-11-16 — T1 — Nasdaq INET notice 61/15 — five-second randomization added to the opening uncross, moving the earliest continuous-trading edge to 10:00:05 Helsinki local time.

## Sources

Row review: 2026-08-22 (UTC) is the date the ledger row was last reviewed as a
whole. Per-source retrieval dates were not recorded before the 2026-09-12
migration; where a later targeted review, capture or document date is recorded
beside a source below, that date governs for that source, and dates are added
as each source is re-verified.

- <https://www.globenewswire.com/news-release/2010/01/25/151379/0/en/INET-Nordic-NASDAQ-OMX-Market-Model.html> — Nasdaq OMX INET Nordic market-model notice, 2010-01-25: the migration is effective 2010-02-08 and does not materially change Nordic trading hours; it also identifies the existing randomized close.
- <https://www.globenewswire.com/news-release/2010/02/05/153049/0/en/INET-Activities-re-migration-starts-today-5-February-at-17-30-CET.html> — Nasdaq OMX INET migration-activity notice, 2010-02-05.
- <https://www.globenewswire.com/en/Attachment/DownloadAttachment?articleid=153059&fileId=93908&filename=market+model+version+1_1+januar+21+2010.pdf&filetype=3&islogo=0> — Nasdaq OMX Nordic Market Model version 1.1, 21 January 2010, attached to the migration notice: every exact January-2010 phase, including post-trading through 18:00 CET.
- <https://www.globenewswire.com/news-release/2015/11/16/787323/0/en/IT-INET-REMINDER-Introduction-of-functional-changes-to-INET-auctions-61-15.html> — Nasdaq INET notice 61/15, 2015-11-16: the opening uncross had previously occurred exactly at 09:00 CET; five-second randomization is introduced effective 2015-11-16.
- <https://www.nasdaq.com/docs/2026/06/17/Nasdaq_Nordic_Market_Model_2026_03_Clean.pdf> — Nasdaq Nordic Market Model 2026:03, section 3.1: confirms the resulting five-second opening edge and each principal-share continuous and closing phase.
- <https://www.nasdaq.com/european-market-activity/trading-hours> — Nasdaq European trading hours, the source set's current entry point.
- <https://www.nasdaq.com/market-regulation/nordic/member-rules> — Nasdaq Nordic member rules, the monitoring entry point.
- <https://www.fese.eu/app/uploads/2024/07/trading-hours-2025-1.pdf> — FESE 2025 trading-hours table, `EU-FESE-SECONDARY`: corroboration only.

## Gaps and residual risks

- **Scope.** The profile represents Nasdaq Helsinki principal shares, published one civil hour later locally: pre-open from 09:00, continuous trading to 18:25, closing call to 18:30 and post-trading through 19:00. The three Nordic books synchronise on CET, but Helsinki publishes one-hour-later local values and Copenhagen has a shorter continuous session, so each has its own profile.
- **Interpretive step, empty order-entry slice.** `order_entry` is empty: the opening call window is modelled `extended` because its uncross prints, and post-trading covers cancellation, limited order updates and manual trades, which print.
- **Horizon.** The baseline grid rests on Market Model 1.1 dated 21 January 2010 together with the migration notices' statement that the February-2010 INET migration did not materially change Nordic trading hours. The first three weeks of January 2010 are therefore covered by the operator's own assertion of continuity rather than by an artifact dated inside them; the horizon is recorded as the January-2010 floor on that basis.
- **Dormant identity.** Reviewed on demand; gaps are recorded here rather than as issues.

> Shared module. [`nasdaq_nordics.rs`](../../src/calendar/schedules/equities/europe/nasdaq_nordics.rs) also carries
> [`nasdaq_stockholm`](nasdaq_stockholm.md) and [`nasdaq_copenhagen`](nasdaq_copenhagen.md).
> Each of the three identities has its own evidence file and its own per-row citation
> lines and `// Evidence:` declaration beside its `revisions!` block in the module.
> The anchor is [`nasdaq_stockholm`](nasdaq_stockholm.md), which received the module
> narrative on 2026-09-12, so the migration LAW-EVIDENCE-FILES requires is complete
> for this module:
> [module narrative](nasdaq_stockholm.md#module-narrative-moved-from-srccalendarschedulesequitieseuropenasdaq_nordicsrs-on-2026-09-12-utc).
