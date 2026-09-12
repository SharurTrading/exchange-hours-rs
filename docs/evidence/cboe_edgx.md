<!-- SPDX-License-Identifier: MIT-0 -->

# `cboe_edgx` — evidence

- **Kind:** `Exchange`
- **Owner module:** [`cboe.rs`](../../src/calendar/schedules/equities/us/cboe.rs)
- **Source sets:** [`US-CBOE-EQUITIES`](../schedules/sources.md#us-cboe-equities)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

**Gap: order-entry** — the trading session is sourced; what is undated is a queue phase in which no trade can print. Current 02:30–20:00 envelope, 2010-07-02 launch, and exact 2021-03-08/2021-09-07 queue changes are sourced. The 2026-09-02 research pass dated the 06:00 entry start to SR-EDGX-2014-18, approved by SEC Release 34-73468 on 2014-10-29, and dated the launch-era queue to Direct Edge's own specifications, whose "Begin Order Acceptance" row reads 7:00 AM through the edition dated 2010-10-01 and 6:00 AM from 2011-02-03 onward. The 07:00–08:00 window is carried from launch; the undated 06:00 move (after 2010-10-01, no later than 2011-02-03) leaves the 06:00–07:00 hour withheld until the 2014 approval day. That residual is a **knowledge-bound gap with a four-month bracket**, not an unfinished search. The conditional future overnight phase stays unencoded. **Systems in scope (2026-09-02):** the EDGX equities matching system (02:30–20:00) is the envelope; EDGX Options is `cboe_edgx_options`. No discrepancy.

## Revision rows

- 2010-07-02 — T1 — SEC 34-62431 — first-symbol production launch, 08:00–20:00 matching with a 07:00–08:00 acceptance queue.
- 2014-10-29 — T1 — SEC 34-73468 — SR-EDGX-2014-18 approved, writing the 06:00 entry start into Rule 11.1(a)(1).
- 2016-05-26 — T1 — Bats release note 2016 7am matching — matching and routing start at 07:00, so the queue narrows to 06:00–07:00.
- 2021-03-08 — T1 — Cboe press release 2021-02-08 — the 03:30 queue and 04:00 active session begin.
- 2021-09-07 — T1 — SEC 34-92914 — the queue moves to 02:30.

## Sources

Retrieval dates: these sources were last opened on the row's reviewed-on date
(2026-08-22, UTC); per-source retrieval dates were not recorded before the
2026-09-12 migration and are added as each source is re-verified.

- <https://www.sec.gov/file/34-62431> — the SEC phase-in record for the 2010-07-02 first production symbol.
- <https://www.nasdaqtrader.com/TraderNews.aspx?id=uva2010-007> — the EDGA/EDGX first-production-symbol alert.
- <https://www.globenewswire.com/news-release/2010/07/21/425534/9381/en/Direct-Edge-Launches-Exchange-Operations.html> — the all-symbol completion release of 2010-07-21.
- <https://www.sec.gov/rules/sro/edgx/2014/34-73468.pdf> — Release 34-73468 of 2014-10-29 approving SR-EDGX-2014-18.
- <https://www.sec.gov/rules/sro/edgx/2014/34-72676.pdf> — the companion notice 34-72676.
- <https://www.federalregister.gov/d/2014-17989> — 79 FR publication of the notice.
- <https://www.federalregister.gov/d/2014-26127> — 79 FR 65450, the approval order.
- <https://www.federalregister.gov/documents/2015/01/15/2015-00525/> — SR-EDGX-2015-03, which quotes the same provision onto the BATS platform.
- <https://web.archive.org/web/20140924155619id_/http://www.directedge.com/Portals/0/docs/Rules_EDGX.pdf> — an archived Direct Edge EDGX rule book.
- <https://web.archive.org/web/20140924164838id_/http://www.directedge.com/Portals/0/05Regulation/Exchange%20Rules/EDGX%20Rules.pdf> — the EDGX rule book updated 2014-07-29.
- <https://cdn.cboe.com/resources/release_notes/2016/Update-Bats-to-Begin-Equity-Order-Matching-and-Routing-at-7-am-ET.pdf> — the 2016 release note.
- <https://ir.cboe.com/news/news-details/2021/Cboe-EDGX-Equities-Exchange-To-Introduce-Early-Trading-Hours-Beginning-March-8-02-08-2021/default.aspx> — the 2021 early-hours announcement for 2021-03-08.
- <https://www.sec.gov/files/rules/sro/cboeedgx/2021/34-92914.pdf> — the 2021-09-07 SEC queue order.
- <https://www.cboe.com/document/tech-spec/document/technical-specifications/cboe-titanium-u.s.-equities-opening-process> — Cboe's current EDGX opening-process specification.
- <https://www.sec.gov/files/rules/sro/cboeedgx/2026/34-105587.pdf> — the SEC EDGX approval order for the monitored future overnight session.
- <https://web.archive.org/web/20091117080129id_/http://www.directedge.com/Portals/0/docs/20090924DirectEdgeNextGenFIXManualV1.pdf> — Direct Edge FIX Specifications v1.0 of 2009-09-22, "Begin Order Acceptance 7:00 AM".
- <https://web.archive.org/web/20091117080134id_/http://www.directedge.com/Portals/0/docs/20090924DirectEdgeNextGenHighPerfAPIManualV1.pdf> — High Performance API Specifications v1.1 of 2009-10-08, 7:00 AM.
- <https://web.archive.org/web/20101231125614id_/http://www.directedge.com/Portals/0/docs/Direct%20Edge%20Next%20Gen%20High%20Perf%20API%20Manual.pdf> — High Performance API Specifications v1.9 of 2010-10-01, 7:00 AM.
- <https://web.archive.org/web/20140419161939id_/http://www.directedge.com/Portals/0/docs/Direct%20Edge%20Next%20Gen%20FIX%20Manual.pdf> — FIX Specifications v1.13 of 2011-02-03, 6:00 AM.
- <https://web.archive.org/web/20140716171112id_/http://www.directedge.com/Portals/0/docs/Direct%20Edge%20Next%20Gen%20High%20Perf%20API%20Manual.pdf> — High Performance API Specifications v1.13 of 2011-02-03, 6:00 AM.
- <https://web.archive.org/web/20140528170202id_/http://www.directedge.com/Portals/0/docs/Connect/Direct%20Edge%20FIX%20Manual%20V%201.23.pdf> — FIX Specifications v1.23 of 2012-01-30, 6:00 AM.
- <https://web.archive.org/web/20120907224639id_/http://www.directedge.com/Portals/0/docs/Specs/Direct%20Edge%20FIX%20Manual.pdf> — FIX Specifications v1.29 of 2012-09-04, 6:00 AM.

## Gaps and residual risks

- **order-entry** — the day the acceptance start moved from 07:00 to 06:00 is
  undated, exactly as on EDGA. Direct Edge's "Begin Order Acceptance" row reads
  7:00 AM through the edition dated 2010-10-01 and 6:00 AM from 2011-02-03
  onward, and no source names the day between. The 07:00–08:00 window is carried
  from launch and the 06:00–07:00 hour is withheld until the 2014-10-29 approval
  day, which under-reports order acceptance rather than over-reporting it. This
  is a knowledge-bound gap with a four-month bracket, not an unfinished search.
  Closing condition: a Direct Edge or SEC artifact that states the 06:00 start on
  a day-level effective date. Dormant identity, so the gap is recorded here
  rather than opened as an issue (LAW-FOLLOW-UPS-ARE-ISSUES).
- **Watch item, not a gap.** The conditional future overnight phase stays
  unencoded until the Equity Data Plan and readiness conditions are satisfied.
- **None below the launch.** The profile is `CLOSED` before 2010-07-02, so the
  horizon is `—`.
- **System coverage (2026-09-02).** No discrepancy. EDGX Options is
  `cboe_edgx_options`.

> Shared module. The narrative for
> [`cboe.rs`](../../src/calendar/schedules/equities/us/cboe.rs)
> lives in [`cboe_bzx`](cboe_bzx.md#module-narrative-moved-from-srccalendarschedulesequitiesuscboers-on-2026-09-12-utc).
> Sibling identities: [`cboe_byx`](cboe_byx.md), [`cboe_edga`](cboe_edga.md).
