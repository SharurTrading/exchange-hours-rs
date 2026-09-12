<!-- SPDX-License-Identifier: MIT-0 -->

# `pse` — evidence

- **Kind:** `Exchange`
- **Owner module:** [`pse.rs`](../../src/calendar/schedules/equities/apac/pse.rs)
- **Source sets:** [`APAC-PSE`](../schedules/sources.md#apac-pse)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

Cash equities including temporary pandemic regimes and 2024 VWAP.

## Revision rows

- 2011-10-01 — T1 — PSE circular CN-2011-0013 — extended trading phase one: close moves to 12:47 with the run-off to 13:00.
- 2012-01-02 — T1 — PSE announcement New Trading Hours — full morning/afternoon schedule: 09:30–12:00 and 13:30–15:17, run-off to 15:30.
- 2013-11-04 — T1 — PSE advisory TPA 2013-0185 — the pre-close moves two minutes earlier, so the afternoon session ends 15:15.
- 2020-03-16 — T1 — PSE circular CN-2020-0017 — shortened pandemic session 09:30–12:45 with run-off to 13:00.
- 2020-03-17 — T1 — PSE circular CN-2020-0021 — all trading suspended; 2020-03-17 and 2020-03-18 are full no-session dates.
- 2020-03-19 — T1 — PSE circular CN-2020-0025 — trading resumes on the shortened session.
- 2021-12-06 — T1 — PSE circular CN-2021-0059 — the full-day session is restored: 09:30–12:00 and 13:00–14:45, run-off to 15:00.
- 2022-01-14 — T1 — PSE circular CN-2022-0004 — the shortened session returns during the Omicron wave.
- 2022-02-02 — T1 — PSE circular CN-2022-0007 — the full-day session is restored again.
- 2024-03-01 — T1 — PSE circular CN-2024-0012 — VWAP extends the close-side envelope to 15:15.

## Sources

Retrieval dates: these sources were last opened on the row's reviewed-on date
(2026-08-22, UTC); per-source retrieval dates were not recorded before the
2026-09-12 migration and are added as each source is re-verified.

- <https://www.pse.com.ph/investing-at-pse/> — PSE investing/trading overview, the current phase table.
- <https://documents.pse.com.ph/CircularOPSPDF/CN-2024-0012.pdf> — PSE circular CN-2024-0012, the 2024-03-01 VWAP extension.
- <https://documents.pse.com.ph/AnnouncementOPSPDF/Proposed%20amendment%20of%20Trading%20Rules%20in%20relation%20to%20Extended%20Trading.pdf> — PSE proposed amendment of trading rules in relation to extended trading, which states the baseline grid before the 2011 extension.
- <https://documents.pse.com.ph/CircularOPSPDF/CN-2011-0013.pdf> — PSE circular CN-2011-0013, extended trading phase one.
- <https://documents.pse.com.ph/AnnouncementOPSPDF/PSE%20New%20Trading%20Hours.pdf> — PSE announcement of the full schedule effective 2012-01-02.
- <https://documents.pse.com.ph/wp-content/uploads/sites/15/2024/08/4_Extended-Pre-Close_TPA_2013-0185.pdf> — PSE advisory TPA 2013-0185, the 2013-11-04 pre-close move.
- <https://documents.pse.com.ph/CircularOPSPDF/CN-2020-0017.pdf> — PSE circular CN-2020-0017, the shortened pandemic session.
- <https://documents.pse.com.ph/CircularOPSPDF/CN-2020-0021.pdf> — PSE circular CN-2020-0021, the 2020-03-17 suspension.
- <https://documents.pse.com.ph/CircularOPSPDF/CN-2020-0025.pdf> — PSE circular CN-2020-0025, the 2020-03-19 resumption.
- <https://documents.pse.com.ph/CircularOPSPDF/CN-2021-0059.pdf> — PSE circular CN-2021-0059, the 2021-12-06 full-day restoration.
- <https://documents.pse.com.ph/wp-content/uploads/sites/15/2022/01/CN_2022-0004.pdf> — PSE circular CN-2022-0004, the 2022-01-14 Omicron shortening.
- <https://documents.pse.com.ph/wp-content/uploads/sites/15/2022/01/CN_2022-0007.pdf> — PSE circular CN-2022-0007, the 2022-02-02 restoration.

## Gaps and residual risks

- **Raised in review of the ledger-reshape PR (#87), 2026-09-12 — the 2020-03-17..19 closure is a holiday encoded as a schedule.** LAW-HOLIDAY-SCOPE is explicit: a change confined to a bounded run of dates — including a full calendar-day closure — is a holiday, and "never bends a normal-week template, adds a revision row, or deletes a valid phase". The 2020-03-17 (CN-2020-0021) and 2020-03-19 (CN-2020-0025) rows do exactly that: they add two revision rows to encode a two-day closure and a resumption. The bullet above records the consequence; this one records the rule it stands against. The reshape PR moved this text out of the owner module and changed no schedule rule, revision row, profile or routing; both rows are served exactly as before, and they report the two dates correctly. Closing condition: the PSE family holiday table shipping under LAW-HOLIDAY-SCOPE, at which point 2020-03-17 and 2020-03-18 move into it as closed dates and the two revision rows are deleted, leaving the 2020-03-16 shortened session running unbroken to 2021-12-06. Dormant identity, so recorded here rather than opened as an issue (LAW-FOLLOW-UPS-ARE-ISSUES).
- **horizon carried below the first dated row** — the baseline profile below 2011-10-01 (09:30–11:57 with run-off to 12:10) rests on the PSE proposed-amendment document, whose publication day is not recorded in the repository. The ledger horizon is therefore 2011-10-01, the first day at which this row's state is sourced, with everything below it carried. Closing condition: read that document's own date, or find a dated pre-2011 PSE circular stating the morning-only grid; either would move the horizon earlier.
- The 2020-03-17 and 2020-03-18 full-day suspension is modelled as a two-day closed era rather than as holiday data, because the crate carries no holiday table yet (LAW-HOLIDAY-SCOPE). When the PSE holiday table ships, this pair should be reviewed for re-homing.
- PSE profiles carry no `order_entry` window: the 09:00–09:30 pre-open and every run-off window are modelled as tradeable `extended`, and no source was worked up that would justify splitting them.
