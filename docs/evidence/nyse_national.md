<!-- SPDX-License-Identifier: MIT-0 -->

# `nyse_national` — evidence

- **Kind:** `Exchange`
- **Owner module:** [`nyse.rs`](../../src/calendar/schedules/equities/us/nyse.rs)
- **Source sets:** [`US-NYSE-EQUITIES`](../schedules/sources.md#us-nyse-equities)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

**Gap: order-entry**, and **source-limited** rather than an unfinished search — recategorised 2026-09-02. The trading session is sourced; what is unestablished is whether legacy NSX ever had an order-acceptance phase distinct from its three trading sessions. NSX hours were rulebook text — Rule 11.1 ("Hours of Trading") — and the floor-era table is stated outright by the exchange's own 2010 filing: "the Exchange's Regular Trading Hours … are from 9:30 a.m. until 4 p.m. Eastern Time. The pre-Regular Trading Hours trading session is from 8 a.m. until 9:30 a.m. ET, and the post-Regular Trading Hours trading session is from 4 p.m. until 6:30 p.m. ET" (75 FR 47049). That is exactly the modelled baseline, and every later NSX change is dated. No reviewed primary source describes an NSX acceptance-only window, but Rule 11.1(a) also let the Board set business hours by Regulatory Circular, and NSX's circular archive is unrecoverable — nsx.com survives in the web archive only as index pages, with the circular PDFs themselves gone — so such a window could not be dated even if one existed. Because the crate models no NSX queue, the residual risk is under-reporting, never over-reporting. Current accepted-order envelope is 06:30–20:00; legacy grids, dormant intervals, and the 2018-05-21 Pillar relaunch are date-aware. **Systems in scope (2026-09-02):** one equities matching system; this SRO operates no options or bond facility. No discrepancy.

## Revision rows

- 2010-08-02 — T1 — SEC 34-62643 — the post-RTH close extends from 18:30 to 20:00 ET.
- 2014-05-16 — T1 — SEC 34-72215 — the post-RTH close shortens to 17:00 ET.
- 2014-05-31 — T1 — SEC 34-72107 — trading ceases after the 2014-05-30 close.
- 2015-12-22 — T1 — NSX SEC Form 1 relaunch filing — the phased relaunch on the rules then in effect.
- 2017-02-01 — T1 — SEC 34-80018 — trading ceases again before the open.
- 2018-05-21 — T1 — SR-NYSENat-2020-05 — the NYSE National Pillar launch, 06:30–20:00 with a 06:30–07:00 acceptance queue.

## Sources

Row review: 2026-08-22 (UTC) is the date the ledger row was last reviewed as a
whole. Per-source retrieval dates were not recorded before the 2026-09-12
migration; where a later targeted review, capture or document date is recorded
beside a source below, that date governs for that source, and dates are added
as each source is re-verified.

- <https://www.nyse.com/trade/hours-calendars?os=.> — NYSE National hours and calendars, the current 06:30–20:00 envelope.
- <https://www.federalregister.gov/documents/2010/08/04/2010-19225/self-regulatory-organizations-national-stock-exchange-inc-notice-of-filing-and-immediate> — the operative 2010 filing dating the 18:30 to 20:00 extension.
- <https://www.sec.gov/files/rules/sro/nsx/2010/34-62643.pdf> — the same filing as published by the SEC.
- <https://www.federalregister.gov/documents/2010/08/10/2010-19652/self-regulatory-organizations-national-stock-exchange-inc-notice-of-filing-and-immediate> — 75 FR 47049, which states the floor-era table outright.
- <https://www.federalregister.gov/documents/2014/05/28/2014-12229/self-regulatory-organizations-national-stock-exchange-inc-notice-of-filing-and-immediate> — the immediately operative 2014 filing shortening the close to 17:00.
- <https://www.sec.gov/files/rules/sro/nsx/2014/34-72215.pdf> — the same 2014 filing.
- <https://www.sec.gov/files/rules/sro/nsx/2014/34-72107.pdf> — the cessation filing for 2014-05-30.
- <https://www.sec.gov/files/rules/sro/nsx/2015/34-76640.pdf> — the SEC's 2015 approval of the resumed marketplace on the rules then in effect.
- <https://www.sec.gov/Archives/edgar/vprr/1601/16019238.pdf> — NSX's SEC-filed Form 1, which dates the phased relaunch to 2015-12-22.
- <https://www.sec.gov/files/rules/sro/nsx/2017/34-80018.pdf> — the second cessation, before the 2017-02-01 open.
- <https://www.federalregister.gov/documents/2018/03/13/2018-04962/self-regulatory-organizations-nyse-national-inc-notice-of-filing-of-proposed-rule-change-to-support> — the NYSE National Pillar filing.
- <https://www.nyse.com/publicdocs/nyse/markets/nyse-national/rule-filings/filings/2020/SR-NYSENat-2020-05.pdf> — SR-NYSENat-2020-05.

## Gaps and residual risks

- **order-entry, and source-limited rather than unsearched** (recategorised
  2026-09-02). The trading session is sourced; what is unestablished is whether
  legacy NSX ever had an order-acceptance phase distinct from its three trading
  sessions. NSX hours were rulebook text — Rule 11.1, "Hours of Trading" — and
  no reviewed primary source describes an NSX acceptance-only window. Rule
  11.1(a) also let the Board set business hours by Regulatory Circular, and NSX's
  circular archive is unrecoverable: nsx.com survives in the web archive only as
  index pages, with the circular PDFs themselves gone. Such a window therefore
  could not be dated even if one existed. Because the crate models no NSX queue,
  the residual risk is under-reporting, never over-reporting. Closing condition:
  a recovered NSX Regulatory Circular, or another operator artifact, stating an
  acceptance-only window on a day-level date. Dormant identity, so it is recorded
  here rather than opened as an issue (LAW-FOLLOW-UPS-ARE-ISSUES).
- **Horizon (carried interval).** The floor-era 08:00–18:30 table is quoted
  outright by the exchange's own 2010 filing — "the Exchange's Regular Trading
  Hours ... are from 9:30 a.m. until 4 p.m. Eastern Time. The pre-Regular Trading
  Hours trading session is from 8 a.m. until 9:30 a.m. ET, and the post-Regular
  Trading Hours trading session is from 4 p.m. until 6:30 p.m. ET" (75 FR 47049)
  — and is carried back from that August-2010 artifact to the January-2010 floor.
  The horizon is therefore 2010-08-02, the first sourced day above the carried
  interval, not the floor.
- **System coverage (2026-09-02).** No discrepancy. One equities matching
  system; this SRO operates no options or bond facility.

> Shared module. The narrative for
> [`nyse.rs`](../../src/calendar/schedules/equities/us/nyse.rs)
> lives in [`nyse`](nyse.md#module-narrative-moved-from-srccalendarschedulesequitiesusnysers-on-2026-09-12-utc).
> Sibling identities: [`nyse_arca`](nyse_arca.md), [`nyse_american`](nyse_american.md), [`nyse_texas`](nyse_texas.md).
