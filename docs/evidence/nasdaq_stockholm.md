<!-- SPDX-License-Identifier: MIT-0 -->

# `nasdaq_stockholm` — evidence

- **Kind:** `Exchange`
- **Owner module:** [`nasdaq_nordics.rs`](../../src/calendar/schedules/equities/europe/nasdaq_nordics.rs)
- **Source sets:** [`EU-NASDAQ-NORDIC`](../schedules/sources.md#eu-nasdaq-nordic), [`EU-FESE-SECONDARY`](../schedules/sources.md#eu-fese-secondary)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

The operator's 2010 INET model supplies the baseline; the 2015-11-16 five-second randomized auction edge and current post-trading phase are date-aware.

## Revision rows

- 2015-11-16 — T1 — Nasdaq INET notice 61/15 — five-second randomization added to the opening uncross, moving the earliest continuous-trading edge to 09:00:05.

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

- **Scope.** The profile represents Nasdaq Stockholm principal shares. The three Nordic books synchronise on CET, but Helsinki publishes one-hour-later local values and Copenhagen has a shorter continuous session, so each has its own profile.
- **Interpretive step, empty order-entry slice.** `order_entry` is empty: the opening call window is modelled `extended` because its uncross prints, and post-trading covers cancellation, limited order updates and manual trades, which print.
- **Horizon.** The baseline grid rests on Market Model 1.1 dated 21 January 2010 together with the migration notices' statement that the February-2010 INET migration did not materially change Nordic trading hours. The first three weeks of January 2010 are therefore covered by the operator's own assertion of continuity rather than by an artifact dated inside them; the horizon is recorded as the January-2010 floor on that basis.
- **Dormant identity.** Reviewed on demand; gaps are recorded here rather than as issues.

## Module narrative (moved from src/calendar/schedules/equities/europe/nasdaq_nordics.rs on 2026-09-12 UTC)

`nasdaq_nordics.rs` carries all three Nordic principal-share identities —
`nasdaq_stockholm`, `nasdaq_helsinki` and `nasdaq_copenhagen`. The books
synchronise on CET, but Helsinki publishes one-hour-later local values and
Copenhagen has a shorter continuous session, so each has its own profile.

Nasdaq's Jan-2010 INET notices made the migration effective 2010-02-08 and
explicitly state that it did not materially change Nordic trading hours.
They also identify the existing randomized close. The operator's 2015 INET
notice says the opening uncross had previously occurred exactly at 09:00 CET
and introduced its five-second randomization effective 2015-11-16.
<https://www.globenewswire.com/news-release/2010/01/25/151379/0/en/INET-Nordic-NASDAQ-OMX-Market-Model.html>
<https://www.globenewswire.com/news-release/2010/02/05/153049/0/en/INET-Activities-re-migration-starts-today-5-February-at-17-30-CET.html>

The operator-authored Market Model 1.1 attached to the migration notice gives
every exact Jan-2010 phase, including post-trading through 18:00 CET
(17:20 CET for Copenhagen).
<https://www.globenewswire.com/en/Attachment/DownloadAttachment?articleid=153059&fileId=93908&filename=market+model+version+1_1+januar+21+2010.pdf&filetype=3&islogo=0>
<https://www.globenewswire.com/news-release/2015/11/16/787323/0/en/IT-INET-REMINDER-Introduction-of-functional-changes-to-INET-auctions-61-15.html>

The current Market Model confirms the resulting five-second opening edge and
each principal-share continuous and closing phase the module encodes.
<https://www.nasdaq.com/docs/2026/06/17/Nasdaq_Nordic_Market_Model_2026_03_Clean.pdf>

Nasdaq Copenhagen launched executable Trading@Closing Price for its main
market on 2019-05-01, adding the executable 17:00-17:10 CET phase before
post-trading resumes through 17:20. The current Nordic Market Model confirms
all three principal-share grids and the five-second randomized opening edge.
<https://view.news.eu.nasdaq.com/view?id=b6276fe1aed34c7412a4d454976025d2d&lang=da>
<https://www.nasdaq.com/docs/2026/06/17/Nasdaq_Nordic_Market_Model_2026_03_Clean.pdf>
Evidence: docs/evidence/nasdaq_copenhagen.md

> Shared module. [`nasdaq_nordics.rs`](../../src/calendar/schedules/equities/europe/nasdaq_nordics.rs) also carries
> [`nasdaq_helsinki`](nasdaq_helsinki.md) and [`nasdaq_copenhagen`](nasdaq_copenhagen.md).
> Each of the three identities has its own evidence file and its own per-row
> citation lines and `// Evidence:` declaration beside its `revisions!` block in the
> module. `nasdaq_stockholm` is the anchor, and the module's narrative moved into
> this file on 2026-09-12, so the migration LAW-EVIDENCE-FILES requires is complete
> for this module: `nasdaq_nordics.rs` now carries rule data, citation lines and
> evidence links only.

## Holidays

**Coverage:** 2026-01-01..2027-12-31

Reviewed 2026-10-12 UTC. `NASDAQ-NORDIC-CASH-2026-2027` is the operator's current European Markets Trading Hours page, specifically the **Equity/Equity derivatives** column for Stockholm. The Fixed Income column is excluded. The audited holiday window is 2026-01-01..2027-12-31; earlier holidays have no new claim from this capture. The normal-week profiles and existing phase classification are unchanged.

The half-day equity row states 09:00–13:00 and post-trading 13:00–13:30 CET. The page's equity footnote reserves the last five minutes for the closing call, with final uncross in its last thirty seconds. Replacement blocks therefore preserve the existing 08:00 opening-call phase and five-second 09:00:05 boundary, end continuous regular trading at 12:55, retain the closing call to 13:00, and end the existing extended post-trading classification at 13:30. These are venue envelopes, not exact security uncross times. No order-entry phase is invented.

### 2026

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2026-01-01 | closed | Closed | `NASDAQ-NORDIC-CASH-2026-2027` | T1 | Stockholm Equity/Equity derivatives column; same civil trade date |
| 2026-01-05 | replacement blocks | Equity close 13:00; post-trade 13:00–13:30 CET | `NASDAQ-NORDIC-CASH-2026-2027` | T1 | Stockholm Equity/Equity derivatives column; same civil trade date; normal opening phases retained; final-five-minute closing call, existing extended post-trading classification |
| 2026-01-06 | closed | Closed | `NASDAQ-NORDIC-CASH-2026-2027` | T1 | Stockholm Equity/Equity derivatives column; same civil trade date |
| 2026-04-02 | replacement blocks | Equity close 13:00; post-trade 13:00–13:30 CET | `NASDAQ-NORDIC-CASH-2026-2027` | T1 | Stockholm Equity/Equity derivatives column; same civil trade date; normal opening phases retained; final-five-minute closing call, existing extended post-trading classification |
| 2026-04-03 | closed | Closed | `NASDAQ-NORDIC-CASH-2026-2027` | T1 | Stockholm Equity/Equity derivatives column; same civil trade date |
| 2026-04-06 | closed | Closed | `NASDAQ-NORDIC-CASH-2026-2027` | T1 | Stockholm Equity/Equity derivatives column; same civil trade date |
| 2026-04-30 | replacement blocks | Equity close 13:00; post-trade 13:00–13:30 CET | `NASDAQ-NORDIC-CASH-2026-2027` | T1 | Stockholm Equity/Equity derivatives column; same civil trade date; normal opening phases retained; final-five-minute closing call, existing extended post-trading classification |
| 2026-05-01 | closed | Closed | `NASDAQ-NORDIC-CASH-2026-2027` | T1 | Stockholm Equity/Equity derivatives column; same civil trade date |
| 2026-05-13 | replacement blocks | Equity close 13:00; post-trade 13:00–13:30 CET | `NASDAQ-NORDIC-CASH-2026-2027` | T1 | Stockholm Equity/Equity derivatives column; same civil trade date; normal opening phases retained; final-five-minute closing call, existing extended post-trading classification |
| 2026-05-14 | closed | Closed | `NASDAQ-NORDIC-CASH-2026-2027` | T1 | Stockholm Equity/Equity derivatives column; same civil trade date |
| 2026-06-19 | closed | Closed | `NASDAQ-NORDIC-CASH-2026-2027` | T1 | Stockholm Equity/Equity derivatives column; same civil trade date |
| 2026-10-30 | replacement blocks | Equity close 13:00; post-trade 13:00–13:30 CET | `NASDAQ-NORDIC-CASH-2026-2027` | T1 | Stockholm Equity/Equity derivatives column; same civil trade date; normal opening phases retained; final-five-minute closing call, existing extended post-trading classification |
| 2026-12-24 | closed | Closed | `NASDAQ-NORDIC-CASH-2026-2027` | T1 | Stockholm Equity/Equity derivatives column; same civil trade date |
| 2026-12-25 | closed | Closed | `NASDAQ-NORDIC-CASH-2026-2027` | T1 | Stockholm Equity/Equity derivatives column; same civil trade date |
| 2026-12-31 | closed | Closed | `NASDAQ-NORDIC-CASH-2026-2027` | T1 | Stockholm Equity/Equity derivatives column; same civil trade date |

### 2027

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2027-01-01 | closed | Closed | `NASDAQ-NORDIC-CASH-2026-2027` | T1 | Stockholm Equity/Equity derivatives column; same civil trade date |
| 2027-01-05 | replacement blocks | Equity close 13:00; post-trade 13:00–13:30 CET | `NASDAQ-NORDIC-CASH-2026-2027` | T1 | Stockholm Equity/Equity derivatives column; same civil trade date; normal opening phases retained; final-five-minute closing call, existing extended post-trading classification |
| 2027-01-06 | closed | Closed | `NASDAQ-NORDIC-CASH-2026-2027` | T1 | Stockholm Equity/Equity derivatives column; same civil trade date |
| 2027-03-25 | replacement blocks | Equity close 13:00; post-trade 13:00–13:30 CET | `NASDAQ-NORDIC-CASH-2026-2027` | T1 | Stockholm Equity/Equity derivatives column; same civil trade date; normal opening phases retained; final-five-minute closing call, existing extended post-trading classification |
| 2027-03-26 | closed | Closed | `NASDAQ-NORDIC-CASH-2026-2027` | T1 | Stockholm Equity/Equity derivatives column; same civil trade date |
| 2027-03-29 | closed | Closed | `NASDAQ-NORDIC-CASH-2026-2027` | T1 | Stockholm Equity/Equity derivatives column; same civil trade date |
| 2027-04-30 | replacement blocks | Equity close 13:00; post-trade 13:00–13:30 CET | `NASDAQ-NORDIC-CASH-2026-2027` | T1 | Stockholm Equity/Equity derivatives column; same civil trade date; normal opening phases retained; final-five-minute closing call, existing extended post-trading classification |
| 2027-05-05 | replacement blocks | Equity close 13:00; post-trade 13:00–13:30 CET | `NASDAQ-NORDIC-CASH-2026-2027` | T1 | Stockholm Equity/Equity derivatives column; same civil trade date; normal opening phases retained; final-five-minute closing call, existing extended post-trading classification |
| 2027-05-06 | closed | Closed | `NASDAQ-NORDIC-CASH-2026-2027` | T1 | Stockholm Equity/Equity derivatives column; same civil trade date |
| 2027-06-25 | closed | Closed | `NASDAQ-NORDIC-CASH-2026-2027` | T1 | Stockholm Equity/Equity derivatives column; same civil trade date |
| 2027-11-05 | replacement blocks | Equity close 13:00; post-trade 13:00–13:30 CET | `NASDAQ-NORDIC-CASH-2026-2027` | T1 | Stockholm Equity/Equity derivatives column; same civil trade date; normal opening phases retained; final-five-minute closing call, existing extended post-trading classification |
| 2027-12-24 | closed | Closed | `NASDAQ-NORDIC-CASH-2026-2027` | T1 | Stockholm Equity/Equity derivatives column; same civil trade date |
| 2027-12-31 | closed | Closed | `NASDAQ-NORDIC-CASH-2026-2027` | T1 | Stockholm Equity/Equity derivatives column; same civil trade date |

### Documents

The saved artifact at `captures/nasdaq-current-2026-10-12/nasdaq-trading-hours.web-extract.txt` is explicitly the web retrieval tool's extracted operator representation, **not original HTML or PDF bytes**. Direct HTTP retrieval failed with transport errors/timeouts; no original-byte hash is claimed. The saved representation retains the operator's named columns, annual headings, date lists and half-day clock language. `parse_nordic.py` selects the first (equity) column using those printed lines; its parsed output records all rows. The page has no stated issue date; 2026-10-12 is the UTC retrieval/review date, not a schedule effective date.

| Document | Window | Replay or service URL | Capture or retrieval, UTC | Tier | sha256 |
|---|---|---|---|---|---|
| `NASDAQ-NORDIC-CASH-2026-2027` | 2026-01-01..2027-12-31 | <https://www.nasdaq.com/european-market-activity/trading-hours> | 2026-10-12 00:49:43 UTC | T1 | `9f8fe49d65db294d48e7bd94e7b4b6b098dac44f102ad74dd71c89b5aa9bff4c` |

Source: <https://www.nasdaq.com/european-market-activity/trading-hours>. Calendar omissions inside these fully published annual columns are audited ordinary weekdays; no substitutes are invented for weekend holidays. Above 2027 the publication horizon remains unannounced. The identity remains dormant and reviewed on demand; the pre-2026 holiday gap remains disclosed before activation.
