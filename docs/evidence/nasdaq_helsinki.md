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
- <https://www.nasdaq.com/docs/2026/08/25/Nasdaq-Nordic-Market-Model-2026-05.pdf> — Nasdaq Nordic Market Model 2026:05 (valid from 2026-09-21), retrieved 2026-11-03 UTC: section 3.1 confirms each principal-share continuous and closing phase unchanged from 2026:03 (#390).
- <https://www.nasdaq.com/european-market-activity/trading-hours> — Nasdaq European trading hours, the source set's current entry point.
- <https://www.nasdaq.com/market-regulation/nordic/member-rules> — Nasdaq Nordic member rules, the monitoring entry point.
- <https://www.fese.eu/app/uploads/2024/07/trading-hours-2025-1.pdf> — FESE 2025 trading-hours table, `EU-FESE-SECONDARY`: corroboration only.

## Gaps and residual risks

- **Scope.** The profile represents Nasdaq Helsinki principal shares, published one civil hour later locally: pre-open from 09:00, continuous trading to 18:25, closing call to 18:30 and post-trading through 19:00. The three Nordic books synchronise on CET, but Helsinki publishes one-hour-later local values and Copenhagen has a shorter continuous session, so each has its own profile.
- **Interpretive step, empty order-entry slice.** `order_entry` is empty: the opening call window is modelled `extended` because its uncross prints, and post-trading covers cancellation, limited order updates and manual trades, which print.
- **Horizon.** The baseline grid rests on Market Model 1.1 dated 21 January 2010 together with the migration notices' statement that the February-2010 INET migration did not materially change Nordic trading hours. The first three weeks of January 2010 are therefore covered by the operator's own assertion of continuity rather than by an artifact dated inside them; the horizon is recorded as the January-2010 floor on that basis.
- **Served identity.** Reviewed monthly after current-scope activation on
  2026-10-12 UTC; historical gaps remain tracked in #332 under the dated
  current OANDA scope exception in AGENTS.md.

> Shared module. [`nasdaq_nordics.rs`](../../src/calendar/schedules/equities/europe/nasdaq_nordics.rs) also carries
> [`nasdaq_stockholm`](nasdaq_stockholm.md) and [`nasdaq_copenhagen`](nasdaq_copenhagen.md).
> Each of the three identities has its own evidence file and its own per-row citation
> lines and `// Evidence:` declaration beside its `revisions!` block in the module.
> The anchor is [`nasdaq_stockholm`](nasdaq_stockholm.md), which received the module
> narrative on 2026-09-12, so the migration LAW-EVIDENCE-FILES requires is complete
> for this module:
> [module narrative](nasdaq_stockholm.md#module-narrative-moved-from-srccalendarschedulesequitieseuropenasdaq_nordicsrs-on-2026-09-12-utc).

## Holidays

**Coverage:** 2026-01-01..2027-12-31

Reviewed 2026-10-12 UTC. `NASDAQ-NORDIC-CASH-2026-2027` is the operator's current European Markets Trading Hours page, specifically the **Equity/Equity derivatives** column for Helsinki. The Fixed Income column is excluded. The audited holiday window is 2026-01-01..2027-12-31; earlier holidays have no new claim from this capture. The normal-week profiles and existing phase classification are unchanged.

### 2026

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2026-01-01 | closed | Closed | `NASDAQ-NORDIC-CASH-2026-2027` | T1 | Helsinki Equity/Equity derivatives column; same civil trade date |
| 2026-01-06 | closed | Closed | `NASDAQ-NORDIC-CASH-2026-2027` | T1 | Helsinki Equity/Equity derivatives column; same civil trade date |
| 2026-04-03 | closed | Closed | `NASDAQ-NORDIC-CASH-2026-2027` | T1 | Helsinki Equity/Equity derivatives column; same civil trade date |
| 2026-04-06 | closed | Closed | `NASDAQ-NORDIC-CASH-2026-2027` | T1 | Helsinki Equity/Equity derivatives column; same civil trade date |
| 2026-05-01 | closed | Closed | `NASDAQ-NORDIC-CASH-2026-2027` | T1 | Helsinki Equity/Equity derivatives column; same civil trade date |
| 2026-05-14 | closed | Closed | `NASDAQ-NORDIC-CASH-2026-2027` | T1 | Helsinki Equity/Equity derivatives column; same civil trade date |
| 2026-06-19 | closed | Closed | `NASDAQ-NORDIC-CASH-2026-2027` | T1 | Helsinki Equity/Equity derivatives column; same civil trade date |
| 2026-12-24 | closed | Closed | `NASDAQ-NORDIC-CASH-2026-2027` | T1 | Helsinki Equity/Equity derivatives column; same civil trade date |
| 2026-12-25 | closed | Closed | `NASDAQ-NORDIC-CASH-2026-2027` | T1 | Helsinki Equity/Equity derivatives column; same civil trade date |
| 2026-12-31 | closed | Closed | `NASDAQ-NORDIC-CASH-2026-2027` | T1 | Helsinki Equity/Equity derivatives column; same civil trade date |

### 2027

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2027-01-01 | closed | Closed | `NASDAQ-NORDIC-CASH-2026-2027` | T1 | Helsinki Equity/Equity derivatives column; same civil trade date |
| 2027-01-06 | closed | Closed | `NASDAQ-NORDIC-CASH-2026-2027` | T1 | Helsinki Equity/Equity derivatives column; same civil trade date |
| 2027-03-26 | closed | Closed | `NASDAQ-NORDIC-CASH-2026-2027` | T1 | Helsinki Equity/Equity derivatives column; same civil trade date |
| 2027-03-29 | closed | Closed | `NASDAQ-NORDIC-CASH-2026-2027` | T1 | Helsinki Equity/Equity derivatives column; same civil trade date |
| 2027-05-06 | closed | Closed | `NASDAQ-NORDIC-CASH-2026-2027` | T1 | Helsinki Equity/Equity derivatives column; same civil trade date |
| 2027-06-25 | closed | Closed | `NASDAQ-NORDIC-CASH-2026-2027` | T1 | Helsinki Equity/Equity derivatives column; same civil trade date |
| 2027-12-06 | closed | Closed | `NASDAQ-NORDIC-CASH-2026-2027` | T1 | Helsinki Equity/Equity derivatives column; same civil trade date |
| 2027-12-24 | closed | Closed | `NASDAQ-NORDIC-CASH-2026-2027` | T1 | Helsinki Equity/Equity derivatives column; same civil trade date |
| 2027-12-31 | closed | Closed | `NASDAQ-NORDIC-CASH-2026-2027` | T1 | Helsinki Equity/Equity derivatives column; same civil trade date |

### Documents

The saved artifact at `captures/nasdaq-current-2026-10-12/nasdaq-trading-hours.web-extract.txt` is explicitly the web retrieval tool's extracted operator representation, **not original HTML or PDF bytes**. Direct HTTP retrieval failed with transport errors/timeouts; no original-byte hash is claimed. The saved representation retains the operator's named columns, annual headings, date lists and half-day clock language. `parse_nordic.py` selects the first (equity) column using those printed lines; its parsed output records all rows. The page has no stated issue date; 2026-10-12 is the UTC retrieval/review date, not a schedule effective date.

| Document | Window | Replay or service URL | Capture or retrieval, UTC | Tier | sha256 |
|---|---|---|---|---|---|
| `NASDAQ-NORDIC-CASH-2026-2027` | 2026-01-01..2027-12-31 | <https://www.nasdaq.com/european-market-activity/trading-hours> | 2026-10-12 00:49:43 UTC | T1 | `9f8fe49d65db294d48e7bd94e7b4b6b098dac44f102ad74dd71c89b5aa9bff4c` |
| `NASDAQ-NORDIC-MARKET-MODEL-2026-05` | valid from 2026-09-21 | <https://www.nasdaq.com/docs/2026/08/25/Nasdaq-Nordic-Market-Model-2026-05.pdf> | retrieved 2026-11-03 UTC | T1 | `68f09275c0ed004a1a352633ec5e1b83c55efe6565ddad8fc4a5d694508cf3ad` |

Source: <https://www.nasdaq.com/european-market-activity/trading-hours>. Calendar omissions inside these fully published annual columns are audited ordinary weekdays; no substitutes are invented for weekend holidays. Above 2027 the publication horizon remains unannounced. The identity is served and reviewed monthly after the 2026-10-12 current-scope activation; its pre-2026 holiday gap remains disclosed in #332 under the dated AGENTS.md exception.

On 2026-11-03 UTC the Nasdaq Nordic Market Model 2026:05 (valid from
2026-09-21; `NASDAQ-NORDIC-MARKET-MODEL-2026-05`) was reviewed section by
section against 2026:04 (valid from 2026-08-31, saved beside 2026:05 in the
research store) and the encoded 2026:03 grids — sections 3.1, 3.1.1, 3.2,
3.2.1, 3.3, 3.4–3.6, 4.1–4.6, Appendices AA, AB and AC, and the Revision
History. The section 3.1 principal-share rows and the section 3.3/3.6
half-day arrangements match the encodings for Stockholm, Helsinki and
Copenhagen, section 3.6 prints no holiday dates, and the only
2026:04-to-2026:05 changes are the Auction Volume Discovery order type
(Appendix AC, active only inside existing auction calls) and new First North
warrants/certificates segments — neither touches the principal-share
session, phase or half-day grids, so the encodings are unchanged (#390).
