<!-- SPDX-License-Identifier: MIT-0 -->

### Holidays, 2022-2024

Tier: **T1** for the 23 rows read from CME's own published holiday
schedules, **T2** for the 16 read from its `trading-hours-by-product`
service.

**Coverage:** 2022-01-01..2024-12-31 (inclusive venue-local trade
dates). Inside the window a date with no row is audited normal; the
family's ordinary week over this wave is 19:00 CT the previous evening into 07:45 CT, then the 08:30-13:20 CT day session whose close is the trading day's final closing phase, then the 14:30-16:00 CT post-close period, read from `grains.rs DATED_CURRENT (2015-07-05, CME SER-7395R)`.
Rows are keyed by the crate's venue-local trade date, never by CME's
event date; the conversion is stated per row in `derived from`.

### 2022

| trade date | kind | instant as printed | document | tier | derived from |
|---|---|---|---|---|---|
| 2022-01-17 | closed | `closed` | `2022-mlk-day-holiday-schedule.xls @2022-01-17T21:22:30Z` | T1 | CME prints the closure for this date, so the session whose final close would have fallen here — the one that opened `19:00 CT` the previous evening — is removed with it, and the crate's trade date is the operator's event date. |
| 2022-02-21 | closed | `closed` | `2022-presidents-day-holiday-schedule.xls @2022-07-04T07:38:10Z` | T1 | CME prints the closure for this date, so the session whose final close would have fallen here — the one that opened `19:00 CT` the previous evening — is removed with it, and the crate's trade date is the operator's event date. |
| 2022-04-15 | closed | `Globex Closed` | `2022-good-friday-holiday-schedule.xls @2022-07-04T06:55:01Z` | T1 | CME prints the closure for this date, so the session whose final close would have fallen here — the one that opened `19:00 CT` the previous evening — is removed with it, and the crate's trade date is the operator's event date. |
| 2022-05-30 | closed | `closed` | `2022-memorial-day-holiday-schedule.xls @2022-07-04T06:54:38Z` | T1 | CME prints the closure for this date, so the session whose final close would have fallen here — the one that opened `19:00 CT` the previous evening — is removed with it, and the crate's trade date is the operator's event date. |
| 2022-06-20 | closed | `closed` | `2022-juneteenth-holiday-schedule.xls @2022-06-20T20:02:10Z` | T1 | CME prints the closure for this date, so the session whose final close would have fallen here — the one that opened `19:00 CT` the previous evening — is removed with it, and the crate's trade date is the operator's event date. |
| 2022-07-04 | closed | `closed` | `2022-independence-day-holiday-schedule.xls @2022-07-04T06:54:50Z` | T1 | CME prints the closure for this date, so the session whose final close would have fallen here — the one that opened `19:00 CT` the previous evening — is removed with it, and the crate's trade date is the operator's event date. |
| 2022-07-05 | late open | `08:30 CT` | `2022-independence-day-holiday-schedule.xls @2022-07-04T06:54:50Z` | T1 | The prior-evening leg of trade date 2022-07-05 did not run: CME's next open is the day session's own `08:30 CT` on that date, and `08:30` is earlier than this family's ordinary `19:00 CT` first open, so the cutoff lands on the trade date itself rather than the preceding local date. |
| 2022-09-05 | closed | `closed` | `2022-labor-day-holiday-schedule.xls @2022-07-04T06:54:41Z` | T1 | CME prints the closure for this date, so the session whose final close would have fallen here — the one that opened `19:00 CT` the previous evening — is removed with it, and the crate's trade date is the operator's event date. |
| 2022-11-24 | closed | `closed` | `2022-thanksgiving-holiday-schedule.FINAL-20221122.xls @2022-11-22T06:08:01Z` | T1 | CME prints the closure for this date, so the session whose final close would have fallen here — the one that opened `19:00 CT` the previous evening — is removed with it, and the crate's trade date is the operator's event date. |
| 2022-11-25 | late open and early close | `08:30 CT` / `12:05 CT` | `2022-thanksgiving-holiday-schedule.FINAL-20221122.xls @2022-11-22T06:08:01Z` | T1 | CME withdrew the prior-evening leg and printed the day session's own `08:30 CT` open beside the `12:05 CT` final close on trade date 2022-11-25, so both boundaries move; `08:30` is earlier than this family's ordinary `19:00 CT` first open, so the cutoff lands on the trade date itself rather than the preceding local date. |
| 2022-12-26 | closed | `Globex Closed` | `2022-christmas-holiday-schedule.xls @2022-07-04T06:54:30Z` | T1 | CME prints the closure for this date, so the session whose final close would have fallen here — the one that opened `19:00 CT` the previous evening — is removed with it, and the crate's trade date is the operator's event date. |

### 2023

| trade date | kind | instant as printed | document | tier | derived from |
|---|---|---|---|---|---|
| 2023-01-02 | closed | `Globex Closed` | `2023-new-years-holiday-schedule.xls @2022-07-04T06:55:01Z` | T1 | CME prints the closure for this date, so the session whose final close would have fallen here — the one that opened `19:00 CT` the previous evening — is removed with it, and the crate's trade date is the operator's event date. |
| 2023-01-16 | unsourced | `unknown` | `CME-SVC-2023-01-15` | T2 | The operator's documents do not cover 2023-01-16 for this family (2023 Dr. Martin Luther King, Jr. Day (Monday 16 January 2023)), and inside the audited window a date with no row reads as audited normal, so the row is `Unsourced`, which clips nothing. |
| 2023-02-20 | unsourced | `unknown` | `CME-SVC-2023-02-19` | T2 | The operator's documents do not cover 2023-02-20 for this family (2023 Presidents Day (Monday 20 February 2023)), and inside the audited window a date with no row reads as audited normal, so the row is `Unsourced`, which clips nothing. |
| 2023-04-07 | unsourced | `unknown` | `CME-SVC-2023-04-06` | T2 | The operator's documents do not cover 2023-04-07 for this family (2023 Good Friday (Friday 7 April 2023)), and inside the audited window a date with no row reads as audited normal, so the row is `Unsourced`, which clips nothing. |
| 2023-05-29 | closed | `closed` | `memorial-day-2023.pdf @2023-04-20T22:40:18Z` | T1 | CME prints the closure for this date, so the session whose final close would have fallen here — the one that opened `19:00 CT` the previous evening — is removed with it, and the crate's trade date is the operator's event date. |
| 2023-06-19 | closed | `closed` | `juneteenth-2023.pdf @2023-06-13T18:59:49Z` | T1 | CME prints the closure for this date, so the session whose final close would have fallen here — the one that opened `19:00 CT` the previous evening — is removed with it, and the crate's trade date is the operator's event date. |
| 2023-07-04 | closed | `no entries` | `4th-of-july-2023.pdf @2023-06-27T12:50:57Z` | T1 | CME prints the closure for this date, so the session whose final close would have fallen here — the one that opened `19:00 CT` the previous evening — is removed with it, and the crate's trade date is the operator's event date. |
| 2023-07-05 | late open | `08:30 CT` | `4th-of-july-2023.pdf @2023-06-27T12:50:57Z` | T1 | The prior-evening leg of trade date 2023-07-05 did not run: CME's next open is the day session's own `08:30 CT` on that date, and `08:30` is earlier than this family's ordinary `19:00 CT` first open, so the cutoff lands on the trade date itself rather than the preceding local date. |
| 2023-09-04 | closed | `closed` | `labor-day-2023.pdf @2023-08-02T19:24:46Z` | T1 | CME prints the closure for this date, so the session whose final close would have fallen here — the one that opened `19:00 CT` the previous evening — is removed with it, and the crate's trade date is the operator's event date. |
| 2023-11-23 | closed | `no time events` | `thanksgiving-day-2023.pdf @2023-12-03T20:59:29Z` | T1 | CME prints the closure for this date, so the session whose final close would have fallen here — the one that opened `19:00 CT` the previous evening — is removed with it, and the crate's trade date is the operator's event date. |
| 2023-11-24 | late open and early close | `08:30 CT` / `12:05 CT` | `thanksgiving-day-2023.pdf @2023-12-03T20:59:29Z` | T1 | CME withdrew the prior-evening leg and printed the day session's own `08:30 CT` open beside the `12:05 CT` final close on trade date 2023-11-24, so both boundaries move; `08:30` is earlier than this family's ordinary `19:00 CT` first open, so the cutoff lands on the trade date itself rather than the preceding local date. |
| 2023-12-25 | closed | `no time events` | `christmas-day-2023.pdf @2026-07-19T09:52:48Z` | T1 | CME prints the closure for this date, so the session whose final close would have fallen here — the one that opened `19:00 CT` the previous evening — is removed with it, and the crate's trade date is the operator's event date. |
| 2023-12-26 | late open | `08:30 CT` | `christmas-day-2023.pdf @2026-07-19T09:52:48Z` | T1 | The prior-evening leg of trade date 2023-12-26 did not run: CME's next open is the day session's own `08:30 CT` on that date, and `08:30` is earlier than this family's ordinary `19:00 CT` first open, so the cutoff lands on the trade date itself rather than the preceding local date. |

### 2024

| trade date | kind | instant as printed | document | tier | derived from |
|---|---|---|---|---|---|
| 2024-01-01 | closed | `no time events` | `new-years-day-2024.pdf @2026-08-11T16:57:16Z` | T1 | CME prints the closure for this date, so the session whose final close would have fallen here — the one that opened `19:00 CT` the previous evening — is removed with it, and the crate's trade date is the operator's event date. |
| 2024-01-02 | late open | `08:30 CT` | `new-years-day-2024.pdf @2026-08-11T16:57:16Z` | T1 | The prior-evening leg of trade date 2024-01-02 did not run: CME's next open is the day session's own `08:30 CT` on that date, and `08:30` is earlier than this family's ordinary `19:00 CT` first open, so the cutoff lands on the trade date itself rather than the preceding local date. |
| 2024-01-15 | closed | `closed` | `CME-SVC-2024-01-14` | T2 | CME prints the closure for this date, so the session whose final close would have fallen here — the one that opened `19:00 CT` the previous evening — is removed with it, and the crate's trade date is the operator's event date. |
| 2024-02-19 | closed | `closed` | `CME-SVC-2024-02-18` | T2 | CME prints the closure for this date, so the session whose final close would have fallen here — the one that opened `19:00 CT` the previous evening — is removed with it, and the crate's trade date is the operator's event date. |
| 2024-03-29 | closed | `events []` | `CME-SVC-2024-03-28` | T2 | CME prints the closure for this date, so the session whose final close would have fallen here — the one that opened `19:00 CT` the previous evening — is removed with it, and the crate's trade date is the operator's event date. |
| 2024-05-27 | closed | `closed` | `CME-SVC-2024-05-26` | T2 | CME prints the closure for this date, so the session whose final close would have fallen here — the one that opened `19:00 CT` the previous evening — is removed with it, and the crate's trade date is the operator's event date. |
| 2024-06-19 | closed | `closed` | `CME-SVC-2024-06-18` | T2 | CME prints the closure for this date, so the session whose final close would have fallen here — the one that opened `19:00 CT` the previous evening — is removed with it, and the crate's trade date is the operator's event date. |
| 2024-07-04 | closed | `events []` | `CME-SVC-2024-07-03` | T2 | CME prints the closure for this date, so the session whose final close would have fallen here — the one that opened `19:00 CT` the previous evening — is removed with it, and the crate's trade date is the operator's event date. |
| 2024-07-05 | late open | `08:30 CT` | `CME-SVC-2024-07-03` | T2 | The prior-evening leg of trade date 2024-07-05 did not run: CME's next open is the day session's own `08:30 CT` on that date, and `08:30` is earlier than this family's ordinary `19:00 CT` first open, so the cutoff lands on the trade date itself rather than the preceding local date. |
| 2024-09-02 | closed | `closed` | `CME-SVC-2024-09-01` | T2 | CME prints the closure for this date, so the session whose final close would have fallen here — the one that opened `19:00 CT` the previous evening — is removed with it, and the crate's trade date is the operator's event date. |
| 2024-11-28 | closed | `events []` | `CME-SVC-2024-11-27` | T2 | CME prints the closure for this date, so the session whose final close would have fallen here — the one that opened `19:00 CT` the previous evening — is removed with it, and the crate's trade date is the operator's event date. |
| 2024-11-29 | late open and early close | `08:30 CT` / `12:05 CT` | `CME-SVC-2024-11-27` | T2 | CME withdrew the prior-evening leg and printed the day session's own `08:30 CT` open beside the `12:05 CT` final close on trade date 2024-11-29, so both boundaries move; `08:30` is earlier than this family's ordinary `19:00 CT` first open, so the cutoff lands on the trade date itself rather than the preceding local date. |
| 2024-12-24 | early close | `12:05 CT` | `CME-SVC-2024-12-24` | T2 | CME prints `12:05 CT` as this date's own final close; the session that opened `19:00 CT` the previous evening is clipped there, and the crate's trade date is the local date that close falls on — the operator's event date. |
| 2024-12-25 | closed | `events []` | `CME-SVC-2024-12-24` | T2 | CME prints the closure for this date, so the session whose final close would have fallen here — the one that opened `19:00 CT` the previous evening — is removed with it, and the crate's trade date is the operator's event date. |
| 2024-12-26 | late open | `08:30 CT` | `CME-SVC-2024-12-24` | T2 | The prior-evening leg of trade date 2024-12-26 did not run: CME's next open is the day session's own `08:30 CT` on that date, and `08:30` is earlier than this family's ordinary `19:00 CT` first open, so the cutoff lands on the trade date itself rather than the preceding local date. |

**`Unsourced` dates.** 2023-01-16, 2023-02-20, 2023-04-07 — inside the audited window, so
they ship `Unsourced` rather than reading as audited normal.

**Interpretive steps and open questions.** Every rule applied
to this family's rows is stated in `tools/out/DECISIONS.md`;
these bear on this family and need the maintainer's ruling:

- **Q2.** The `dairy` -> `globex_grains` fold joins two different clocks. Default: keep the fold for status only; the crate family's instants stand
- **Q4.** Fold disagreement on 2022-11-25: `dairy` says closed, `globex_grains` says late open 08:30 CT and early close 12:05 CT. Default: ship the crate family's row and record the disagreement
- **Q5.** Fold disagreement on 2023-11-24: `dairy` says closed, `globex_grains` says late open 08:30 CT and early close 12:05 CT. Default: ship the crate family's row and record the disagreement
- **Q6.** Fold disagreement on 2024-11-29: `dairy` says closed, `globex_grains` says late open 08:30 CT and early close 12:05 CT. Default: ship the crate family's row and record the disagreement
- **Q7.** Fold disagreement on 2024-12-24: `dairy` says early close 12:00 CT, `globex_grains` says early close 12:05 CT. Default: ship the crate family's row and record the disagreement
- **Q9.** The day-after-closure late opens on 2022-07-05, 2023-07-05, 2023-12-26, 2024-01-02, 2024-07-05, 2024-12-26 have no counterpart in the folded group. Default: ship the crate family's `LateOpen` rows
- **Q10.** 6 rows are keyed to trade dates the block has no entry for. Default: keep the created rows
- **Q11.** 14 closure entries print no reopen at all. Default: ship nothing for 2024-04-01 and record the gap
- **Q12.** The 3 uncovered 2023 dates cite the operator's own negative control as their document id. Default: cite the negative control, keep one 2022-2024 window
- **Q13.** `globex_nikkei_225_dollar` ships 16 `Unsourced` rows, 13 of them the whole 2024 window. Default: keep the contiguous window with `Unsourced` rows
- **Q14.** 3 document ids take their tier from the INDEX's channel statement, not from a per-row tier token. Default: accept the file-level tier statement
- **Q15.** The wave's dates 2022-01-01 carry `normal` for all ten product groups, so no family ships any row for them. Default: ship nothing

**No row, and why.** A status that changes no answer ships
nothing. For this family the block's entries were read as:

- open: open cell carries no clock token — 2024-12-31
- open: the trade date's own printed morning shows `07:45`, so the prior-evening leg ran and the `08:30 CT` open inside it moves no boundary — 2024-12-24
- printed close 16:00 equals the family's ordinary 16:00 CT final close — 2023-07-03, 2024-12-31
- printed close is stated to be the family's ordinary final close — 2024-07-03
- reopen: open cell carries no clock token — 2024-03-29
- reopen: printed open 19:00 CT is the family's ordinary first open — 2022-01-17, 2022-02-21, 2022-04-15, 2022-05-30, 2022-06-20, 2022-09-05, 2022-12-26, 2023-01-02, 2023-05-29, 2023-06-19, 2023-09-04, 2024-01-15, 2024-02-19, 2024-05-27, 2024-06-19, 2024-09-02
- status normal — 2022-01-01

