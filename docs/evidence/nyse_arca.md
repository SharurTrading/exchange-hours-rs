<!-- SPDX-License-Identifier: MIT-0 -->

# `nyse_arca` — evidence

- **Kind:** `Exchange`
- **Owner module:** [`nyse.rs`](../../src/calendar/schedules/equities/us/nyse.rs)
- **Source sets:** [`US-NYSE-EQUITIES`](../schedules/sources.md#us-nyse-equities)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

Gap closed 2026-09-02. The acceptance edge is rulebook text throughout: pre-Pillar Rule 7.35(a)(1) began accepting orders 30 minutes before the 04:00 Opening Session, and the Pillar I filing carried that into Rule 7.34-E(a)(1) "without any substantive differences" (80 FR 28721), so 03:30 is carried back to the January-2010 floor with no cutover asserted. SR-NYSEArca-2021-71 (86 FR 46296) moved it to 90 minutes and deferred production to a Trader Update; that Trader Update, issued 2021-08-11 and repeated 2021-09-09, names the day without condition — "Beginning on Monday, September 13, 2021, NYSE Arca will change the time for order entry to 2:30 a.m. ET" — and archived captures of the exchange's hours page show 3:30 on 2021-09-12 and 2:30 on 2021-09-27. Current accepted-order envelope is 02:30–20:00. Watch item, not a gap: NYSE Arca's announced 23-hour Overnight Session from Sunday 2026-12-06 remains unencoded. **Systems in scope (2026-09-02):** the Arca equities Pillar system (02:30–20:00) is the envelope and NYSE Arca Options is `nyse_arca_options`; no discrepancy. The conditional 23/5 Overnight Trading Session under temporary Rule 7.34-E(T) (SR-NYSEARCA-2026-53: 21:00 Sunday to 20:00 Friday, order acceptance from 20:59) is monitored and unencoded pending Equity Data Plan readiness.

## Revision rows

- 2021-09-13 — T1 — NYSE Trader Update 2021-08-11 — order entry moves from 03:30 to 02:30 ET, 90 minutes before the 04:00 Opening Session.

## Sources

Retrieval dates: these sources were last opened on the row's reviewed-on date
(2026-08-22, UTC); per-source retrieval dates were not recorded before the
2026-09-12 migration and are added as each source is re-verified.

- <https://www.nyse.com/trade/hours-calendars?os=.> — NYSE Arca hours and calendars, the current 02:30–20:00 accepted-order envelope.
- <https://www.sec.gov/files/rules/sro/nysearca/2008/34-57505.pdf> — the SEC's pre-2010 Arca evidence for Rule 7.35(a)(1)'s 30-minute acceptance edge.
- <https://www.federalregister.gov/documents/2015/05/19/2015-12028/self-regulatory-organizations-nyse-arca-inc-notice-of-filing-of-proposed-rule-change-adopting-new> — the Pillar I filing, which carried that text into Rule 7.34-E(a)(1) "without any substantive differences" (80 FR 28721).
- <https://www.federalregister.gov/documents/2021/08/18/2021-17673/self-regulatory-organizations-nysearca-inc-notice-of-filing-and-immediate-effectiveness-of-proposed> — SR-NYSEArca-2021-71 (86 FR 46296), the 30-to-90-minute amendment that deferred production to a Trader Update.
- <https://www.nyse.com/publicdocs/nyse/markets/nyse-arca/rule-filings/filings/2021/SR-NYSEArca-2021-71.pdf> — the filing as published by the exchange.
- <https://www.nyse.com/trader-update/history#110000372318> — the Trader Update of 2021-08-11: "Beginning on Monday, September 13, 2021, NYSE Arca will change the time for order entry to 2:30 a.m. ET."
- <https://www.nyse.com/publicdocs/nyse/data/ArcaBook_Client_Specification.pdf> — the ArcaBook client specification.

## Gaps and residual risks

- **None open.** The acceptance edge is rulebook text on both sides of the
  January-2010 floor, so 03:30 is carried back to the floor with no cutover
  asserted, and the 2021 move is dated by the exchange's own Trader Update.
  Archived captures of the hours page corroborate it, showing 3:30 on
  2021-09-12 and 2:30 on 2021-09-27.
- **Watch item, not a gap.** The conditional 23/5 Overnight Trading Session
  under temporary Rule 7.34-E(T) — SR-NYSEARCA-2026-53 (91 FR 31509,
  2026-05-27): 21:00 Sunday to 20:00 Friday with a one-hour technical pause from
  20:00 Monday through Thursday and order acceptance from 20:59 — is monitored
  and unencoded pending Equity Data Plan readiness. It was added to the row by
  the 2026-09-02 audit as an enumeration omission, not a discrepancy.
- **System coverage (2026-09-02).** No discrepancy. NYSE Arca Options is
  `nyse_arca_options`.

> Shared module. The narrative for
> [`nyse.rs`](../../src/calendar/schedules/equities/us/nyse.rs)
> lives in [`nyse`](nyse.md#module-narrative-moved-from-srccalendarschedulesequitiesusnysers-on-2026-09-12-utc).
> Sibling identities: [`nyse_american`](nyse_american.md), [`nyse_national`](nyse_national.md), [`nyse_texas`](nyse_texas.md).
