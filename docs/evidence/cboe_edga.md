<!-- SPDX-License-Identifier: MIT-0 -->

# `cboe_edga` — evidence

- **Kind:** `Exchange`
- **Owner module:** [`cboe.rs`](../../src/calendar/schedules/equities/us/cboe.rs)
- **Source sets:** [`US-CBOE-EQUITIES`](../schedules/sources.md#us-cboe-equities)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

**Gap: order-entry** — the trading session is sourced; what is undated is a queue phase in which no trade can print. Current 06:00–20:00 envelope and the 2010-07-02 first-symbol launch are primary-supported. The 2026-09-02 research pass closed the search this row used to describe. Rule 11.1(a)(1)'s 06:00 entry start was adopted by SR-EDGA-2014-20, approved by SEC Release 34-73592 on 2014-11-13, so the 06:00 queue is now dated to that day and the 2026-08-22 knowledge-bound row is gone. Direct Edge's own FIX and High Performance API specifications supply the earlier queue: "Begin Order Acceptance" reads 7:00 AM in the editions dated 2009-09-22, 2009-10-08 and 2010-10-01 and 6:00 AM from the 2011-02-03 editions onward, so the 07:00–08:00 window is carried from launch day. What stays undated is the 06:00 move itself — after 2010-10-01 and no later than 2011-02-03 — which no specification changelog, operator notice or SEC filing records. The 06:00–07:00 hour is therefore withheld until the 2014 approval, which under-reports order acceptance rather than over-reporting it. This is now a **knowledge-bound gap with a four-month bracket**, not an unfinished search. **Systems in scope (2026-09-02):** one equities matching system; EDGA Rule 11.1 states order entry from 06:00 until 20:00, matching the modeled envelope. This SRO operates no options facility. No discrepancy.

## Revision rows

- 2010-07-02 — T1 — SEC 34-62431 — first-symbol production launch, 08:00–20:00 matching with a 07:00–08:00 acceptance queue.
- 2014-11-13 — T1 — SEC 34-73592 — SR-EDGA-2014-20 approved, writing the 06:00 entry start into Rule 11.1(a)(1).
- 2016-05-24 — T1 — Bats release note 2016 7am matching — matching and routing start at 07:00, so the queue narrows to 06:00–07:00.

## Sources

No per-URL retrieval date is recorded in this repository; every link below
was read at or before the row's `Reviewed on` date in the ledger.

- <https://www.sec.gov/file/34-62431> — the SEC phase-in record for the 2010-07-02 first production symbol.
- <https://www.nasdaqtrader.com/TraderNews.aspx?id=uva2010-007> — the EDGA/EDGX first-production-symbol alert.
- <https://www.globenewswire.com/news-release/2010/07/21/425534/9381/en/Direct-Edge-Launches-Exchange-Operations.html> — the all-symbol completion release of 2010-07-21.
- <https://www.sec.gov/files/rules/sro/edga/2014/34-73592.pdf> — Release 34-73592 of 2014-11-13 approving SR-EDGA-2014-20.
- <https://www.sec.gov/files/rules/sro/edga/2014/34-72812.pdf> — the companion notice 34-72812.
- <https://www.federalregister.gov/d/2014-19415> — 79 FR publication of the notice.
- <https://www.federalregister.gov/d/2014-27312> — 79 FR 68937, the approval order.
- <https://www.federalregister.gov/documents/2015/01/15/2015-00531/> — SR-EDGA-2015-03, which quotes the same provision onto the BATS platform.
- <https://web.archive.org/web/20140923171131id_/http://www.directedge.com/Portals/0/docs/Rules_EDGA.pdf> — an archived Direct Edge EDGA rule book.
- <https://web.archive.org/web/20140924162806id_/http://www.directedge.com/Portals/0/05Regulation/Exchange%20Rules/EDGA%20Rules.pdf> — the EDGA rule book updated 2014-07-29.
- <https://cdn.cboe.com/resources/release_notes/2016/Update-Bats-to-Begin-Equity-Order-Matching-and-Routing-at-7-am-ET.pdf> — the 2016 release note.
- <https://web.archive.org/web/20091117080129id_/http://www.directedge.com/Portals/0/docs/20090924DirectEdgeNextGenFIXManualV1.pdf> — Direct Edge FIX Specifications v1.0 of 2009-09-22, "Begin Order Acceptance 7:00 AM".
- <https://web.archive.org/web/20091117080134id_/http://www.directedge.com/Portals/0/docs/20090924DirectEdgeNextGenHighPerfAPIManualV1.pdf> — High Performance API Specifications v1.1 of 2009-10-08, 7:00 AM.
- <https://web.archive.org/web/20101231125614id_/http://www.directedge.com/Portals/0/docs/Direct%20Edge%20Next%20Gen%20High%20Perf%20API%20Manual.pdf> — High Performance API Specifications v1.9 of 2010-10-01, 7:00 AM.
- <https://web.archive.org/web/20140419161939id_/http://www.directedge.com/Portals/0/docs/Direct%20Edge%20Next%20Gen%20FIX%20Manual.pdf> — FIX Specifications v1.13 of 2011-02-03, 6:00 AM.
- <https://web.archive.org/web/20140716171112id_/http://www.directedge.com/Portals/0/docs/Direct%20Edge%20Next%20Gen%20High%20Perf%20API%20Manual.pdf> — High Performance API Specifications v1.13 of 2011-02-03, 6:00 AM.
- <https://web.archive.org/web/20140528170202id_/http://www.directedge.com/Portals/0/docs/Connect/Direct%20Edge%20FIX%20Manual%20V%201.23.pdf> — FIX Specifications v1.23 of 2012-01-30, 6:00 AM.
- <https://web.archive.org/web/20120907224639id_/http://www.directedge.com/Portals/0/docs/Specs/Direct%20Edge%20FIX%20Manual.pdf> — FIX Specifications v1.29 of 2012-09-04, 6:00 AM.

## Gaps and residual risks

- **order-entry** — the day the acceptance start moved from 07:00 to 06:00 is
  undated. Direct Edge's own specifications place it after the edition dated
  2010-10-01 and no later than the 2011-02-03 editions, and a document's version
  date is a publication date, never a cutover (LAW-NO-FABRICATED-DATES). The
  crate therefore serves the 07:00–08:00 window, which holds under every sourced
  state, and withholds the 06:00–07:00 hour until the 2014 approval day. This
  under-reports order acceptance and never over-reports it, and nothing matches
  in the queue either way. This is a knowledge-bound gap with a four-month
  bracket, not an unfinished search: no specification changelog, operator notice
  or SEC filing records the move. Closing condition: a Direct Edge or SEC
  artifact that states the 06:00 start on a day-level effective date. Dormant
  identity, so the gap is recorded here rather than opened as an issue
  (LAW-FOLLOW-UPS-ARE-ISSUES).
- **None below the launch.** The profile is `CLOSED` before 2010-07-02, so the
  horizon is `—`.
- **System coverage (2026-09-02).** No discrepancy. One equities matching
  system; EDGA Rule 11.1 states order entry from 06:00 until 20:00, matching the
  modeled envelope. This SRO operates no options facility.

> Shared module. The narrative for
> [`cboe.rs`](../../src/calendar/schedules/equities/us/cboe.rs)
> lives in [`cboe_bzx`](cboe_bzx.md#module-narrative-moved-from-srccalendarschedulesequitiesuscboers-on-2026-09-12-utc).
> Sibling identities: [`cboe_byx`](cboe_byx.md), [`cboe_edgx`](cboe_edgx.md).
