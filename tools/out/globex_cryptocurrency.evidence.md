<!-- SPDX-License-Identifier: MIT-0 -->

### Holidays, 2022-2024

Tier: **T1** for the 7 rows read from CME's own published holiday
schedules, **T2** for the 7 read from its `trading-hours-by-product`
service.

**Coverage:** 2022-01-01..2024-12-31 (inclusive venue-local trade
dates). Inside the window a date with no row is audited normal; the
family's ordinary week over this wave is one wrapped leg per trade date, 17:00 CT the previous evening into a 16:00 CT close on the trade date, read from `cryptocurrency.rs FIVE_DAY (2017-12-17, CME SER-8051R)`.
Rows are keyed by the crate's venue-local trade date, never by CME's
event date; the conversion is stated per row in `derived from`.

### 2022

| trade date | kind | instant as printed | document | tier | derived from |
|---|---|---|---|---|---|
| 2022-04-15 | closed | `Globex Closed` | `2022-good-friday-holiday-schedule.xls @2022-07-04T06:55:01Z` | T1 | CME prints the closure for this date, so the session whose final close would have fallen here — the one that opened `17:00 CT` the previous evening — is removed with it, and the crate's trade date is the operator's event date. |
| 2022-11-25 | early close | `12:45 CT` | `2022-thanksgiving-holiday-schedule.FINAL-20221122.xls @2022-11-22T06:08:01Z` | T1 | CME prints `12:45 CT` as this date's own final close; the session that opened `17:00 CT` the previous evening is clipped there, and the crate's trade date is the local date that close falls on — the operator's event date. |
| 2022-12-26 | closed | `Globex Closed` | `2022-christmas-holiday-schedule.xls @2022-07-04T06:54:30Z` | T1 | CME prints the closure for this date, so the session whose final close would have fallen here — the one that opened `17:00 CT` the previous evening — is removed with it, and the crate's trade date is the operator's event date. |

### 2023

| trade date | kind | instant as printed | document | tier | derived from |
|---|---|---|---|---|---|
| 2023-01-02 | closed | `Globex Closed` | `2023-new-years-holiday-schedule.xls @2022-07-04T06:55:01Z` | T1 | CME prints the closure for this date, so the session whose final close would have fallen here — the one that opened `17:00 CT` the previous evening — is removed with it, and the crate's trade date is the operator's event date. |
| 2023-01-16 | unsourced | `unknown` | `CME-SVC-2023-01-15` | T2 | The operator's documents do not cover 2023-01-16 for this family (2023 Dr. Martin Luther King, Jr. Day (Monday 16 January 2023)), and inside the audited window a date with no row reads as audited normal, so the row is `Unsourced`, which clips nothing. |
| 2023-02-20 | unsourced | `unknown` | `CME-SVC-2023-02-19` | T2 | The operator's documents do not cover 2023-02-20 for this family (2023 Presidents Day (Monday 20 February 2023)), and inside the audited window a date with no row reads as audited normal, so the row is `Unsourced`, which clips nothing. |
| 2023-04-07 | unsourced | `unknown` | `CME-SVC-2023-04-06` | T2 | The operator's documents do not cover 2023-04-07 for this family (2023 Good Friday (Friday 7 April 2023)), and inside the audited window a date with no row reads as audited normal, so the row is `Unsourced`, which clips nothing. |
| 2023-11-24 | early close | `12:45 CT` | `thanksgiving-day-2023.pdf @2023-12-03T20:59:29Z` | T1 | CME prints `12:45 CT` as this date's own final close; the session that opened `17:00 CT` the previous evening is clipped there, and the crate's trade date is the local date that close falls on — the operator's event date. |
| 2023-12-25 | closed | `closed` | `christmas-day-2023.pdf @2026-07-19T09:52:48Z` | T1 | CME prints the closure for this date, so the session whose final close would have fallen here — the one that opened `17:00 CT` the previous evening — is removed with it, and the crate's trade date is the operator's event date. |

### 2024

| trade date | kind | instant as printed | document | tier | derived from |
|---|---|---|---|---|---|
| 2024-01-01 | closed | `closed` | `new-years-day-2024.pdf @2026-08-11T16:57:16Z` | T1 | CME prints the closure for this date, so the session whose final close would have fallen here — the one that opened `17:00 CT` the previous evening — is removed with it, and the crate's trade date is the operator's event date. |
| 2024-03-29 | closed | `events []` | `CME-SVC-2024-03-28` | T2 | CME prints the closure for this date, so the session whose final close would have fallen here — the one that opened `17:00 CT` the previous evening — is removed with it, and the crate's trade date is the operator's event date. |
| 2024-11-29 | early close | `12:45 CT` | `CME-SVC-2024-11-27` | T2 | CME prints `12:45 CT` as this date's own final close; the session that opened `17:00 CT` the previous evening is clipped there, and the crate's trade date is the local date that close falls on — the operator's event date. |
| 2024-12-24 | early close | `12:45 CT` | `CME-SVC-2024-12-24` | T2 | CME prints `12:45 CT` as this date's own final close; the session that opened `17:00 CT` the previous evening is clipped there, and the crate's trade date is the local date that close falls on — the operator's event date. |
| 2024-12-25 | closed | `closed` | `CME-SVC-2024-12-24` | T2 | CME prints the closure for this date, so the session whose final close would have fallen here — the one that opened `17:00 CT` the previous evening — is removed with it, and the crate's trade date is the operator's event date. |

**`Unsourced` dates.** 2023-01-16, 2023-02-20, 2023-04-07 — inside the audited window, so
they ship `Unsourced` rather than reading as audited normal.

**Interpretive steps and open questions.** Every rule applied
to this family's rows is stated in `tools/out/DECISIONS.md`;
these bear on this family and need the maintainer's ruling:

- **Q11.** 14 closure entries print no reopen at all. Default: ship nothing for 2024-04-01 and record the gap
- **Q12.** The 3 uncovered 2023 dates cite the operator's own negative control as their document id. Default: cite the negative control, keep one 2022-2024 window
- **Q13.** `globex_nikkei_225_dollar` ships 16 `Unsourced` rows, 13 of them the whole 2024 window. Default: keep the contiguous window with `Unsourced` rows
- **Q14.** 3 document ids take their tier from the INDEX's channel statement, not from a per-row tier token. Default: accept the file-level tier statement
- **Q15.** The wave's dates 2022-01-01 carry `normal` for all ten product groups, so no family ships any row for them. Default: ship nothing

**No row, and why.** A status that changes no answer ships
nothing. For this family the block's entries were read as:

- open: open cell carries no clock token — 2022-11-25, 2023-11-24, 2024-11-29, 2024-12-24
- open: printed open 17:00 CT is the family's ordinary first open — 2022-01-17, 2022-02-21, 2022-05-30, 2022-06-20, 2022-07-04, 2022-09-05, 2022-11-24, 2023-05-29, 2023-06-19, 2023-07-04, 2023-09-04, 2023-11-23, 2024-01-15, 2024-02-19, 2024-05-27, 2024-06-19, 2024-07-04, 2024-09-02, 2024-11-28
- printed close is stated to be the family's ordinary final close — 2022-01-17, 2022-02-21, 2022-05-30, 2022-06-20, 2022-07-04, 2022-09-05, 2022-11-24, 2023-05-29, 2023-06-19, 2023-07-04, 2023-09-04, 2023-11-23, 2024-01-15, 2024-02-19, 2024-05-27, 2024-06-19, 2024-07-04, 2024-09-02, 2024-11-28
- reopen: open cell carries no clock token — 2024-03-29
- reopen: printed open 17:00 CT is the family's ordinary first open — 2022-04-15, 2022-12-26, 2023-01-02, 2023-12-25, 2024-01-01, 2024-12-25
- status normal — 2022-01-01, 2023-07-03, 2024-07-03, 2024-12-31

