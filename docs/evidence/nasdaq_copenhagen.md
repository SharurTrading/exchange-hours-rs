<!-- SPDX-License-Identifier: MIT-0 -->

# `nasdaq_copenhagen` — evidence

- **Kind:** `Exchange`
- **Owner module:** [`nasdaq_nordics.rs`](../../src/calendar/schedules/equities/europe/nasdaq_nordics.rs)
- **Source sets:** [`EU-NASDAQ-NORDIC`](../schedules/sources.md#eu-nasdaq-nordic), [`EU-FESE-SECONDARY`](../schedules/sources.md#eu-fese-secondary)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

The operator's 2010 INET model supplies the baseline; the 2015 randomized edge and 2019-05-01 Trading@Closing Price revision are date-aware.

## Revision rows

- 2015-11-16 — T1 — Nasdaq INET notice 61/15 — five-second randomization added to the opening uncross, moving the earliest continuous-trading edge to 09:00:05.
- 2019-05-01 — T1 — Nasdaq Copenhagen Trading@Closing Price announcement — executable Trading@Closing Price added for the main market, 17:00–17:10 CET, before post-trading resumes through 17:20.

## Sources

Retrieval dates: these sources were last opened on the row's reviewed-on date
(2026-08-22, UTC); per-source retrieval dates were not recorded before the
2026-09-12 migration and are added as each source is re-verified.

- <https://www.globenewswire.com/news-release/2010/01/25/151379/0/en/INET-Nordic-NASDAQ-OMX-Market-Model.html> — Nasdaq OMX INET Nordic market-model notice, 2010-01-25: the migration is effective 2010-02-08 and does not materially change Nordic trading hours; it also identifies the existing randomized close.
- <https://www.globenewswire.com/news-release/2010/02/05/153049/0/en/INET-Activities-re-migration-starts-today-5-February-at-17-30-CET.html> — Nasdaq OMX INET migration-activity notice, 2010-02-05.
- <https://www.globenewswire.com/en/Attachment/DownloadAttachment?articleid=153059&fileId=93908&filename=market+model+version+1_1+januar+21+2010.pdf&filetype=3&islogo=0> — Nasdaq OMX Nordic Market Model version 1.1, 21 January 2010, attached to the migration notice: every exact January-2010 phase, including post-trading through 18:00 CET.
- <https://www.globenewswire.com/news-release/2015/11/16/787323/0/en/IT-INET-REMINDER-Introduction-of-functional-changes-to-INET-auctions-61-15.html> — Nasdaq INET notice 61/15, 2015-11-16: the opening uncross had previously occurred exactly at 09:00 CET; five-second randomization is introduced effective 2015-11-16.
- <https://www.nasdaq.com/docs/2026/06/17/Nasdaq_Nordic_Market_Model_2026_03_Clean.pdf> — Nasdaq Nordic Market Model 2026:03, section 3.1: confirms the resulting five-second opening edge and each principal-share continuous and closing phase.
- <https://www.nasdaq.com/european-market-activity/trading-hours> — Nasdaq European trading hours, the source set's current entry point.
- <https://www.nasdaq.com/market-regulation/nordic/member-rules> — Nasdaq Nordic member rules, the monitoring entry point.
- <https://view.news.eu.nasdaq.com/view?id=b6276fe1aed34c7412a4d454976025d2d&lang=da> — Nasdaq Copenhagen Trading@Closing Price announcement, launching the executable 17:00–17:10 CET phase on 2019-05-01.
- <https://www.fese.eu/app/uploads/2024/07/trading-hours-2025-1.pdf> — FESE 2025 trading-hours table, `EU-FESE-SECONDARY`: corroboration only.

## Gaps and residual risks

- **Scope.** The profile represents Nasdaq Copenhagen principal shares, whose continuous session is shorter than Stockholm's: pre-open from 08:00, continuous trading to 16:55, closing call to 17:00, Trading@Closing Price to 17:10 and post-trading through 17:20. The three Nordic books synchronise on CET, but Helsinki publishes one-hour-later local values and Copenhagen has a shorter continuous session, so each has its own profile.
- **Interpretive step, empty order-entry slice.** `order_entry` is empty: the opening call window is modelled `extended` because its uncross prints, and post-trading covers cancellation, limited order updates and manual trades, which print.
- **Horizon.** The baseline grid rests on Market Model 1.1 dated 21 January 2010 together with the migration notices' statement that the February-2010 INET migration did not materially change Nordic trading hours. The first three weeks of January 2010 are therefore covered by the operator's own assertion of continuity rather than by an artifact dated inside them; the horizon is recorded as the January-2010 floor on that basis.
- **Dormant identity.** Reviewed on demand; gaps are recorded here rather than as issues.

> Shared module. [`nasdaq_nordics.rs`](../../src/calendar/schedules/equities/europe/nasdaq_nordics.rs) also carries
> [`nasdaq_stockholm`](nasdaq_stockholm.md) and [`nasdaq_helsinki`](nasdaq_helsinki.md).
> Each of the three identities has its own evidence file and its own `// Evidence:`
> declaration beside its `revisions!` block in the module, so the evidence-file
> migration LAW-EVIDENCE-FILES requires is complete for all three.
> [`nasdaq_stockholm`](nasdaq_stockholm.md) is the anchor: it receives the module's
> remaining prose when `nasdaq_nordics.rs` is drained under issue #85, which is the
> only part still outstanding.
