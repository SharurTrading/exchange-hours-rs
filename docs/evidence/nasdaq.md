<!-- SPDX-License-Identifier: MIT-0 -->

# `nasdaq` — evidence

- **Kind:** `Exchange`
- **Owner module:** [`equities.rs`](../../src/calendar/schedules/equities/us/equities.rs)<br>[`history.rs`](../../src/calendar/schedules/equities/us/history.rs)
- **Source sets:** [`US-NASDAQ-EQUITIES`](../schedules/sources.md#us-nasdaq-equities)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

Nasdaq Stock Market normal week; date-aware lookups retain the sourced 2013 07:00→04:00 early-open change. The announced Night Session is monitored but unencoded pending Equity Data Plan readiness and a later Nasdaq filing, so current and future snapshots remain 04:00–20:00. **Systems in scope (2026-09-02):** the Nasdaq equities matching system (System Hours 04:00–20:00) is the envelope; The Nasdaq Options Market is `nasdaq_nom` and the FINRA/Nasdaq TRF is `finra_trf_carteret`. ACT, Weblink ACT 2.0, ACES (08:00–18:30, interior), the Nasdaq Testing Facility and index dissemination are excluded classes; Nasdaq Fixed Income and Nasdaq Futures belong to neither this SRO nor cash equity. No discrepancy.

## Revision rows

- 2013-03-18 — T1 — Nasdaq Equity Trader Alert 2013-21 — pre-market open moves from 07:00 to 04:00 ET.

## Sources

No per-URL retrieval date is recorded in this repository; every link below
was read at or before the row's `Reviewed on` date in the ledger.

- <https://listingcenter.nasdaq.com/rulebook/nasdaq/rules/Nasdaq%20Equity%202> — Nasdaq Equity 2, the rulebook provision behind the 04:00–20:00 System Hours.
- <https://www.nasdaqtrader.com/content/technicalsupport/nasdaq_sys_hours.pdf> — *Nasdaq Systems — Hours of Operation*, the operator's system inventory and phase table (2020 edition; read through the web archive, see the ledger's channel notes).
- <https://www.nasdaqtrader.com/TraderNews.aspx?id=ETA2013-21> — Nasdaq Equity Trader Alert 2013-21, the 04:00 pre-market open effective Monday 2013-03-18.
- <https://www.nasdaqtrader.com/TraderNews.aspx?id=ETA2026-46> — Nasdaq Equity Trader Alert 2026-46, the announced Night Session.
- <https://listingcenter.nasdaq.com/assets/rulebook/nasdaq/filings/SR-NASDAQ-2025-109_Approval.pdf> — the SEC approval order for the Night Session rule.

## Gaps and residual risks

- **Horizon (carried interval).** The 07:00–20:00 grid below 2013-03-18 is
  carried, not separately sourced: Equity Trader Alert 2013-21 states the
  outgoing value when it dates the change, and no earlier Nasdaq artifact in the
  reviewed set states the pre-market open at a day level. The alert's own
  publication date is not recorded here, so the horizon is keyed to the
  effective day it states, 2013-03-18. Closing condition: a Nasdaq rulebook
  edition or trader alert that states the 07:00 System Hours open on a
  floor-era day.
- **Watch item, not a gap.** The announced Night Session is monitored and
  unencoded: Nasdaq Equity 1 conditions it on Equity Data Plan readiness and a
  later Nasdaq readiness filing, so it has no unconditional effective day
  (LAW-NO-FABRICATED-DATES). Current and future snapshots stay 04:00–20:00.
- **System coverage (2026-09-02).** No discrepancy. ACT, Weblink ACT 2.0, ACES,
  the Nasdaq Testing Facility and index dissemination are excluded classes;
  Nasdaq Fixed Income and Nasdaq Futures belong to neither this SRO nor cash
  equity. Dormant identity, so the residual items above are recorded here rather
  than opened as issues (LAW-FOLLOW-UPS-ARE-ISSUES).

## Module narrative (moved from src/calendar/schedules/equities/us/equities.rs on 2026-09-12 UTC)

Nasdaq, MEMX, and MIAX Pearl publish the 04:00–20:00 shape. Sources: Nasdaq
Equity Rules Equity 2 § 8; MEMX market-hours notice; MIAX Pearl Equities
alert 2024-11-13 and its trading-hours page.

Nasdaq operated 07:00–20:00 ET before moving its pre-market open to 04:00
effective 2013-03-18.
https://www.nasdaqtrader.com/TraderNews.aspx?id=ETA2013-21

Nasdaq BX, renamed Nasdaq Texas by the operator. This is not the unrelated
NYSE Texas venue, whose profile lives in `nyse.rs`. The stable public
identity here remains `nasdaq_bx`. The venue publishes 07:00–19:00 ET system
hours around the 09:30–16:00 core session.
An official 2009 circular proves an 08:00–19:00 January-2010 baseline, and
SR-BX-2011-016 proves the later 08:00→07:00 system-hours change, and Equity
Trader Alert 2011-20 makes its production date Monday 2011-04-18. A
March-2014 Nasdaq data notice independently confirms the 07:00 platform open.
https://www.nasdaqtrader.com/content/technicalsupport/nasdaq_sys_hours.pdf
https://www.nasdaqtrader.com/TraderNews.aspx?id=ETA2009-003
https://www.sec.gov/rules/sro/bx/2011/34-64105.pdf
https://www.nasdaqtrader.com/TraderNews.aspx?id=ETA2011-20
https://www.nasdaqtrader.com/TraderNews.aspx?id=dtn2014-08

Nasdaq PSX currently publishes 08:00–17:00 ET system hours. PSX launched
with a 09:00 ET start and kept the same 17:00 close before the 2010-12-13
expansion.
https://listingcenter.nasdaq.com/rulebook/phlx/rules/phlx-psx-legacy-3000
https://www.sec.gov/files/rules/sro/phlx/2010/34-63492.pdf

MEMX shortened its executable Post-Market Session from 20:00 to 17:00 ET
effective 2020-10-05, then restored the 20:00 close on 2023-02-01.
https://info.memxtrading.com/trader-alert-20-06-memx-market-hours-change/
https://info.memxtrading.com/trader-alert-23-04-memx-trading-hours-change/

## Module narrative (moved from src/calendar/schedules/equities/us/history.rs on 2026-09-12 UTC)

Nasdaq Equity Trader Alert 2013-21 moved the pre-market open from 07:00 to
04:00 ET effective Monday 2013-03-18. Future Night Session announcements are
monitored in the schedule update guide but are not selected until Nasdaq's
required readiness filing supplies an unconditional effective day.
https://www.nasdaqtrader.com/TraderNews.aspx?id=ETA2013-21

Nasdaq Equity Trader Alert 2011-20 states that BX began accepting and
executing orders at 07:00 ET on Monday 2011-04-18. The official launch alert
supplies the 08:00 ET predecessor open and unchanged 19:00 close.
https://www.nasdaqtrader.com/TraderNews.aspx?id=ETA2009-003
https://www.nasdaqtrader.com/TraderNews.aspx?id=ETA2011-20

Nasdaq's launch alert dates PSX production to 2010-10-08. The initial rules
operated 09:00–17:00 ET; SR-Phlx-2010-172 explicitly identifies 2010-12-13
as the implementation date for the 08:00 ET opening.
Row evidence:
  2010-10-08 "Nasdaq Equity Trader Alert 2010-56"
    https://www.nasdaqtrader.com/TraderNews.aspx?id=ETA2010-56
  2010-12-13 "SEC SR-Phlx-2010-172"
    https://www.sec.gov/files/rules/sro/phlx/2010/34-63492.pdf

MEMX began live trading on 2020-09-21. It shortened the Post-Market Session
from 20:00 to 17:00 ET on 2020-10-05 and restored the 20:00 close on
2023-02-01. Its own 2025-06-06 retrospective identifies 2025-05-19 as the
actual launch of its 04:00 ET pre-market. The earlier rule filing proposed a
March date, so the exchange's stated production launch is the operative
boundary.
https://memx.com/insights/day-1
https://info.memxtrading.com/trader-alert-20-06-memx-market-hours-change/
https://www.sec.gov/files/rules/sro/memx/2023/34-96773.pdf
https://info.memxtrading.com/trader-alert-23-04-memx-trading-hours-change/
https://memx.com/insights/pre-market-share-gains-and-new-options-active-risk-feature
Row evidence:
  2020-09-21 "MEMX Day 1 retrospective"
    https://memx.com/insights/day-1
  2020-10-05 "MEMX trader alert 20-06"
    https://info.memxtrading.com/trader-alert-20-06-memx-market-hours-change/
  2023-02-01 "MEMX trader alert 23-04"
    https://info.memxtrading.com/trader-alert-23-04-memx-trading-hours-change/
    https://www.sec.gov/files/rules/sro/memx/2023/34-96773.pdf
  2025-05-19 "MEMX retrospective 2025-06-06"
    https://memx.com/insights/pre-market-share-gains-and-new-options-active-risk-feature

MIAX Pearl Equities launched on 2020-09-29. Regulatory Circular 2025-02
later made the Early Trading Session (04:00–09:30 ET) and Late Trading
Session (16:00–20:00 ET) available beginning 2025-02-20. Before that
amendment the exchange-level profile contains Regular Trading Hours only.
https://www.miaxglobal.com/company/markets/us-equities
https://www.miaxglobal.com/sites/default/files/circular-files/MIAX_Pearl_Equities_RC_2025_02_0.pdf
Row evidence:
  2020-09-29 "MIAX Pearl Equities launch notice"
    https://www.miaxglobal.com/company/markets/us-equities
  2025-02-20 "MIAX Pearl Regulatory Circular 2025-02"
    https://www.miaxglobal.com/sites/default/files/circular-files/MIAX_Pearl_Equities_RC_2025_02_0.pdf
