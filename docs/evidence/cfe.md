<!-- SPDX-License-Identifier: MIT-0 -->

# `cfe` — evidence

- **Kind:** `Exchange`
- **Owner module:** [`cfe.rs`](../../src/calendar/schedules/futures/us/cfe.rs)
- **Source sets:** [`US-CFE`](../schedules/sources.md#us-cfe)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

VIX futures normal-week history is complete from January 2010. Old-system pre-opens have exact sourced onsets; randomized new-system queue starts use conservative latest edges of 16:00:03/16:45:03 from 2018-02-25 and 16:00:06/16:45:06 from 2018-08-12.

## Revision rows

- 2010-12-10 — T1 — Cboe SR-CFE-2010-013 — a 07:20–08:30 CT extended session is added ahead of the unchanged 08:30–15:15 RTH.
- 2011-09-26 — T1 — Cboe SR-CFE-2011-019 — the extended session start moves from 07:20 to 07:00 CT.
- 2013-10-28 — T1 — Cboe IC13-041 — the Monday–Thursday 15:30–16:15 CT session opens, with its 15:29–15:30 pre-open queue.
- 2013-11-04 — T1 — Cboe IC13-041 — the morning open moves from 07:00 to 02:00 CT.
- 2014-06-22 — T1 — Cboe RG-CFE-2014-020 — near-24-hour VX trading begins, adding the Sunday 16:15–17:00 CT pre-open.
- 2018-02-25 — T1 — Cboe RG-CFE-2018-005 — system migration: a 15:15–15:30 CT queue, 15:30–16:00 ETH, and the 16:00:03 / 16:45:03 CT queue edges.
- 2018-08-12 — T1 — Cboe C2018071603 — TAS queue commencement widens to six seconds, so the conservative edges become 16:00:06 and 16:45:06 CT.
- 2021-12-06 — T1 — Cboe C2021102603 — the current grid: RTH 08:30–15:00 CT, ETH 15:00–16:00 and 17:00–08:30, queues at 16:00:06 and 16:45:06 CT.

## Holidays

**Coverage:** 2026-01-01..2026-12-31 (inclusive trade dates). Tier: T1 throughout.

One table serves the `cfe` venue and the `cfe_vix` key: Cboe publishes one holiday schedule for all CFE futures, and VIX futures are the only family the crate routes to the venue, so the venue intersection is that one family's own table.

**Documents.**

- `CBOE-HOURS-USFUT-2026` — Cboe “Hours & Holidays → U.S. Futures”, heading “2026 Futures Holiday Schedule”. <https://www.cboe.com/about/hours/us-futures> (raw bytes retrieved 2026-09-12 04:55 UTC, sha256 `b1b38ba66a870c06b9169eed58329407cc0ea23903a44d96e5284f7bf97e7759`), with Cboe's own CSV export <https://www.cboe.com/us/futures/holidays/csv/> (retrieved 2026-09-12 04:17 UTC, sha256 `cc98e21e97b8b830368a6ea462b5c262704432d22b163c67cb240311b3561c89`, header `# Generated: 2026:09:11 16:16:09`) — **T1**. The holiday *set* is corroborated by CFE Regulatory Circular 26-004 of 2026-03-31 (sha256 `7291896896d349360f8ecaf3d1a67cf7e074af70ec427eb12ab9c5bc19177a21`), which states no session instants and keys nothing.

All bytes, with each artifact's URL, UTC retrieval time and sha256, are in the research store under `holidays/raw/cfe-eurex-ice-cde-smfe-2026-2027/INDEX.md` and `holidays/raw/cfe-eurex-ice-cde-smfe-2026-2027-fix/INDEX.md`; the normalised result is `holidays/cfe-eurex-ice-cde-smfe-2026-2027.json`, verified `matches: true` with zero discrepancies in its round-2 adversarial verdict.

### 2026

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2026-01-01 | closed | `New Year's Day,2026-01-01,None,5:00 PM (Thu) to 8:30 AM (Fri)` | `CBOE-HOURS-USFUT-2026` | T1 | Cboe event date 2026-01-01; the only block printed is the Thursday-evening leg, which belongs to trade date 2026-01-02 |
| 2026-01-19 | early close | `Martin Luther King Jr. Day,2026-01-19,None,5:00 PM (Sun) to 10:30 AM (Mon) and 5:00 PM (Mon) to 8:30 AM (Tue)` — 10:30 CT | `CBOE-HOURS-USFUT-2026` | T1 | Cboe event date 2026-01-19; the Sunday-evening leg is trade date 2026-01-19's own and stops at 10:30, and the Monday-evening leg is trade date 2026-01-20's |
| 2026-02-16 | early close | `Presidents' Day,2026-02-16,None,5:00 PM (Sun) to 10:30 AM (Mon) and 5:00 PM (Mon) to 8:30 AM (Tue)` — 10:30 CT | `CBOE-HOURS-USFUT-2026` | T1 | Cboe event date 2026-02-16; same two-leg shape as MLK Day |
| 2026-04-03 | early close | `Good Friday,2026-04-03,None,5:00 PM (Thu) to 8:30 AM (Fri)` — 08:30 CT | `CBOE-HOURS-USFUT-2026` | T1 | Cboe event date 2026-04-03; the Thursday-evening leg is trade date 2026-04-03's own and stops at its normal regular open |
| 2026-05-25 | early close | `Memorial Day,2026-05-25,None,5:00 PM (Sun) to 10:30 AM (Mon) and 5:00 PM (Mon) to 8:30 AM (Tue)` — 10:30 CT | `CBOE-HOURS-USFUT-2026` | T1 | Cboe event date 2026-05-25 |
| 2026-06-19 | early close | `Juneteenth Holiday,2026-06-19,None,5:00 PM (Thu) to 10:30 AM (Fri)` — 10:30 CT | `CBOE-HOURS-USFUT-2026` | T1 | Cboe event date 2026-06-19; a Friday, so there is no evening leg to reassign |
| 2026-07-03 | early close | `Independence Day Observed,2026-07-03,None,5:00 PM (Thu) to 10:30 AM (Fri)` — 10:30 CT | `CBOE-HOURS-USFUT-2026` | T1 | Cboe event date 2026-07-03 |
| 2026-09-07 | early close | `Labor Day,2026-09-07,None,5:00 PM (Sun) to 10:30 AM (Mon) and 5:00 PM (Mon) to 8:30 AM (Tue)` — 10:30 CT | `CBOE-HOURS-USFUT-2026` | T1 | Cboe event date 2026-09-07 |
| 2026-11-26 | early close | `Thanksgiving Day,2026-11-26,None,5:00 PM (Wed) to 10:30 AM (Thu) and 5:00 PM (Thu) to 8:30 AM (Fri)` — 10:30 CT | `CBOE-HOURS-USFUT-2026` | T1 | Cboe event date 2026-11-26; the Thursday-evening leg belongs to trade date 2026-11-27 |
| 2026-11-27 | early close | `Thanksgiving Early Close,2026-11-27,08:30:00 - 12:15:00` — 12:15 CT | `CBOE-HOURS-USFUT-2026` | T1 | Cboe event date 2026-11-27; the first open is the normal Thursday 17:00 CT, so only the close moves |
| 2026-12-24 | early close | `Christmas Early Close,2026-12-24,08:30:00 - 12:15:00,5:00 PM (Wed) to 8:30 AM (Thu)` — 12:15 CT | `CBOE-HOURS-USFUT-2026` | T1 | Cboe event date 2026-12-24; no Thursday-evening leg is printed because trade date 2026-12-25 is closed |
| 2026-12-25 | closed | `Christmas Day,2026-12-25,None,None` | `CBOE-HOURS-USFUT-2026` | T1 | Cboe event date 2026-12-25; both columns empty |

**Gaps, 2026:** Cboe has published no 2027 CFE holiday schedule; the page carries only the “2026 Futures Holiday Schedule”, the CSV endpoint ignores a `?year=` parameter, and the 2026 regulatory-circular index lists no 2027 holiday circular. Coverage therefore ends 2026-12-31, and the crate answers the pure normal week for 2027. Closed by the Cboe 2027 futures holiday schedule when Cboe publishes it. No row in this window changes intraday phase topology, and no row is a late open.

**Interpretive steps, 2026:** Cboe prints two cells per holiday, `Regular Trading Hours` and `Extended Trading Hours`. On nine of these dates the regular cell reads `None` while the extended cell still names a block. On the crate's trade-date key that is an **early close** of the leg which opened at 17:00 CT the previous evening, not a closure: the block printed as “5:00 PM (Sun) to 10:30 AM (Mon)” *is* trade date Monday's own session, and the second block “5:00 PM (Mon) to 8:30 AM (Tue)” belongs to the next trade date and needs no row. Two dates are different. 2026-01-01 is `closed` because the only block printed is the Thursday-evening leg of trade date 2026-01-02, so no session at all belongs to 2026-01-01; and 2026-12-25 prints `None,None`. This reading answers the retrieval's own advisory on 2026-01-01 and is fenced in `tests/futures_family_boundaries/holidays_cfe_vix.rs`.

## Sources

Row review: 2026-08-22 (UTC) is the date the ledger row was last reviewed as a
whole. Per-source retrieval dates were not recorded before the 2026-09-12
migration; where a later targeted review, capture or document date is recorded
beside a source below, that date governs for that source, and dates are added
as each source is re-verified.

The documents below stand behind the row and behind the narrative moved below. All times are Chicago time.

- <https://cdn.cboe.com/resources/regulation/rule_filings/approved/2010/SR-CFE-2010-013.pdf> — Cboe SR-CFE-2010-013, the 2010-12-10 extended-session filing — T1.
- <https://cdn.cboe.com/resources/regulation/rule_filings/approved/2011/SR-CFE-2011-019.pdf> — Cboe SR-CFE-2011-019, the 2011-09-26 07:00 start — T1.
- <https://ir.cboe.com/news/news-details/2013/CBOE-Futures-Exchange-Announces-Launch-Dates-For-VIX-Futures-Extended-Trading-Hours-09-30-2013/default.aspx> — the 2013 phased-hours announcement, describing the predecessor 07:00–15:15 trading day and the exact phase shapes — T1.
- <https://ir.cboe.com/news/news-details/2014/2013-Trading-Volume-Reaches-New-All-Time-High-At-CBOE-Futures-Exchange-01-02-2014/default.aspx> — Cboe's year-end retrospective, recording the actual 2013-10-28 and 2013-11-04 launches — T1.
- <https://cdn.cboe.com/resources/regulation/circulars/general/CFE-IC-2013-041.pdf> — Cboe IC13-041, pinning both 2013 phases and publishing the 15:29–15:30 weekday pre-open queue — T1.
- <https://cdn.cboe.com/resources/regulation/rule_filings/approved/2013/SR-CFE-2013-032.pdf> — Cboe SR-CFE-2013-032, the 2013 extended-hours filing — T1.
- <https://cdn.cboe.com/resources/regulation/rule_filings/approved/2013/SR-CFE-2013-034.pdf> — Cboe SR-CFE-2013-034, the 2013 extended-hours filing — T1.
- <https://cdn.cboe.com/resources/regulation/rule_filings/approved/2014/SR-CFE-2014-010.pdf> — Cboe SR-CFE-2014-010, introducing nearly-24-hour VX trading — T1.
- <https://ir.cboe.com/news/news-details/2014/CBOE-Futures-Exchange-Set-For-June-22-Launch-Of-24-Hour-VIX-Futures-Trading-06-09-2014/default.aspx> — the 2014 round-the-clock launch announcement — T1.
- <https://cdn.cboe.com/resources/regulation/circulars/general/CFE-IC-2014-036.pdf> — Cboe IC14-036, publishing the Sunday 16:15–17:00 pre-open and the retained 15:29–15:30 weekday pre-open — T1.
- <https://cdn.cboe.com/resources/regulation/circulars/regulatory/RG-CFE-2014-020.pdf> — Cboe RG-CFE-2014-020, pinning the launch to Sunday 2014-06-22 — T1.
- <https://cdn.cboe.com/resources/regulation/rule_filings/approved/2017/SR-CFE-2017-017.pdf> — Cboe SR-CFE-2017-017, the migration-era phase enumeration — T1.
- <https://cdn.cboe.com/resources/regulation/circulars/regulatory/RG-CFE-2018-005.pdf> — Cboe RG-CFE-2018-005, confirming the migration completed Sunday 2018-02-25 for business date Monday 2018-02-26 — T1.
- <https://cdn.cboe.com/resources/release_notes/2018/Change-to-CFE-Pre-Open-Time-for-TAS-Contracts-and-Order-Submission-Commencement-Times.pdf> — Cboe C2018071603, the 2018-08-12 TAS queue-commencement change — T1.
- <https://cdn.cboe.com/resources/regulation/rule_filings/pending/2021/21-028-VX-VXM-and-AMERIBOR-Trading-Hours.pdf> — rule certification CFE-2021-028 behind Cboe notice C2021102603, the current queues and trading phases — T1.
- <https://www.cboe.com/about/hours/us-futures> — CFE trading hours, the current-schedule monitoring entry point — T1.
- <https://www.cboe.com/tradable-products/vix/vix-futures/specifications> — VIX futures specifications — T1.
- <https://www.cboe.com/markets/us/futures/regulation/circulars/cfe/regulatory/> — CFE regulatory circulars, the watch channel — T1.

## Gaps and residual risks

- **No dated-history gap.** The normal-week history is primary-supported from
  the January-2010 floor, so the row is `Primary` and its horizon is the floor
  itself rather than a carried-back date.
- **Randomized queue starts are modelled conservatively, not exactly.** From
  2018-02-25 CFE begins each opening queue at a randomized instant within three
  seconds of the nominal boundary, and from 2018-08-12 within six seconds for
  TAS contracts. The profile publishes the latest acceptance edge
  (16:00:03 / 16:45:03, then 16:00:06 / 16:45:06 CT) because each queue follows a
  closed or suspended period, so a caller never sees an open queue that CFE had
  not yet started. No source states a per-contract instant, and none is claimed.
- **Scope.** The `cfe` venue default is specifically VIX futures. Any other CFE
  contract family needs its own review and its own profile; a listing venue is
  not a venue-wide clock.

## Module narrative (moved from src/calendar/schedules/futures/us/cfe.rs on 2026-09-12 UTC)

CFE (VIX) — current schedule, effective 2021-12-06.

RTH is 08:30–15:00 CT. ETH runs 15:00–16:00 and, from Sunday plus
Monday–Thursday, 17:00–08:30. Order-entry queues run Sunday 16:00–17:00 and
Monday–Thursday 16:45–17:00, with starts randomized through six seconds after
the nominal boundary. Because each queue follows a closed/suspended period,
the profile uses the conservative latest 16:00:06 and 16:45:06 edges. The
change removed the former 15:15–15:30 queue and 15:00–15:15 RTH segment.

Sources: Cboe notice C2021102603, effective 2021-12-06; rule certification
CFE-2021-028 (all times in Chicago time).
<https://cdn.cboe.com/resources/regulation/rule_filings/pending/2021/21-028-VX-VXM-and-AMERIBOR-Trading-Hours.pdf>

ORDER-ENTRY CLASSIFICATION. The notice quoted above calls the Sunday
16:00-17:00 and Monday-Thursday 16:45-17:00 windows "order-entry queues":
CFE accepts non-market orders that cannot execute until trading resumes at
17:00, so no trade can print inside them. They are `order_entry`. The
15:00-16:00 and 17:00-08:30 ETH windows match and stay in `extended`.

At the January-2010 audit floor, VX traded 08:30-15:15 CT. CFE then
introduced a 07:20-08:30 extended session effective 2010-12-10 and moved
that start to 07:00 effective 2011-09-26. The filings state both day-level
effective dates and preserve the 08:30-15:15 regular session.
<https://cdn.cboe.com/resources/regulation/rule_filings/approved/2010/SR-CFE-2010-013.pdf>
<https://cdn.cboe.com/resources/regulation/rule_filings/approved/2011/SR-CFE-2011-019.pdf>

CFE expanded VX hours in two phases during 2013. Its announcement describes
the predecessor 07:00–15:15 trading day and the exact phase shapes; Cboe's
year-end retrospective records the actual launches as 2013-10-28 for the
Monday–Thursday 15:30–16:15 session and 2013-11-04 for the move from a 07:00
to a 02:00 morning open. IC13-041 pins those phases to 2013-10-28 and
2013-11-04 and publishes the 15:29–15:30 Monday–Thursday pre-open queue in
both. RTH remained 08:30–15:15 throughout.
<https://ir.cboe.com/news/news-details/2013/CBOE-Futures-Exchange-Announces-Launch-Dates-For-VIX-Futures-Extended-Trading-Hours-09-30-2013/default.aspx>
<https://ir.cboe.com/news/news-details/2014/2013-Trading-Volume-Reaches-New-All-Time-High-At-CBOE-Futures-Exchange-01-02-2014/default.aspx>
<https://cdn.cboe.com/resources/regulation/circulars/general/CFE-IC-2013-041.pdf>
<https://cdn.cboe.com/resources/regulation/rule_filings/approved/2013/SR-CFE-2013-032.pdf>
<https://cdn.cboe.com/resources/regulation/rule_filings/approved/2013/SR-CFE-2013-034.pdf>

IC13-041 publishes 15:29-15:30 as a Monday-Thursday "pre-open queue" ahead of
the 15:30 session, so it accepts orders without matching and is `order_entry`.

CFE-2014-010 introduced nearly-24-hour VX trading on Sunday 2014-06-22.
IC14-036 publishes the resulting 16:15–17:00 Sunday pre-open and retained
15:29–15:30 weekday pre-open; RG-CFE-2014-020 pins the launch to that Sunday.
<https://cdn.cboe.com/resources/regulation/rule_filings/approved/2014/SR-CFE-2014-010.pdf>
<https://ir.cboe.com/news/news-details/2014/CBOE-Futures-Exchange-Set-For-June-22-Launch-Of-24-Hour-VIX-Futures-Trading-06-09-2014/default.aspx>
<https://cdn.cboe.com/resources/regulation/circulars/general/CFE-IC-2014-036.pdf>
<https://cdn.cboe.com/resources/regulation/circulars/regulatory/RG-CFE-2014-020.pdf>

IC14-036 names both the Sunday 16:15-17:00 phase and the retained 15:29-15:30
weekday phase as pre-opens: orders queue, nothing matches.

SR-CFE-2017-017 tied a revised VX schedule to CFE's system migration:
08:30–15:15 RTH, a 15:15–15:30 order-entry-only queue, 15:30–16:00 ETH, a
16:00–16:45 weekday suspension, a 16:45–17:00 queue, then 17:00–08:30 ETH.
Sunday has a 16:00–17:00 opening queue. The queues accept non-market orders
that cannot execute until trading resumes, so the crate classifies them as
`order_entry` under the order-entry-phase convention — `cfe.rs` puts all three
(the 15:15–15:30 weekday queue and the Sunday 16:00 and weekday 16:45 opening
queues) in `CFE_ORDER_ENTRY_2018_02_25`, never in `extended`. The new-system opening
queues begin at randomized instants through three seconds after the nominal
boundary, so their conservative edges are 16:00:03 and 16:45:03. RG18-005
confirms that the migration completed Sunday 2018-02-25, for business date
Monday 2018-02-26.
<https://cdn.cboe.com/resources/regulation/rule_filings/approved/2017/SR-CFE-2017-017.pdf>
<https://cdn.cboe.com/resources/regulation/circulars/regulatory/RG-CFE-2018-005.pdf>
The filing above enumerates the migration-era phases separately: RTH ends at
15:15, 15:15-15:30 is an "order-entry-only queue", 15:30-16:00 is ETH, and
the 16:00/16:45 opening queues run to the 17:00 ETH open. Only the two ETH
windows match, so the 15:15-15:30 queue is split out of the former merged
15:15-16:00 rule and joins the evening queues in `order_entry`.

C2018071603 changed TAS queue commencement to a randomized instant three to
six seconds after the nominal Sunday 16:00 and weekday 16:45 boundaries,
effective with the Sunday 2018-08-12 opening. Non-TAS queues remained within
zero to three seconds. The all-contract profile therefore advances its
conservative latest edge from three to six seconds on that opening day.
<https://cdn.cboe.com/resources/release_notes/2018/Change-to-CFE-Pre-Open-Time-for-TAS-Contracts-and-Order-Submission-Commencement-Times.pdf>
Only the queue-commencement seconds change here, so the matching grid is the
one `CFE_EXT_2018_02_25` already carries.

Row evidence — each revision's day-level effective date and the primary
source that states it (full quotations sit in the blocks above):

```text
  2010-12-10 "Cboe SR-CFE-2010-013"
    https://cdn.cboe.com/resources/regulation/rule_filings/approved/2010/SR-CFE-2010-013.pdf
  2011-09-26 "Cboe SR-CFE-2011-019"
    https://cdn.cboe.com/resources/regulation/rule_filings/approved/2011/SR-CFE-2011-019.pdf
  2013-10-28 "Cboe IC13-041" and 2013-11-04 "Cboe IC13-041"
    https://cdn.cboe.com/resources/regulation/circulars/general/CFE-IC-2013-041.pdf
  2014-06-22 "Cboe RG-CFE-2014-020"
    https://cdn.cboe.com/resources/regulation/circulars/regulatory/RG-CFE-2014-020.pdf
  2018-02-25 "Cboe RG-CFE-2018-005"
    https://cdn.cboe.com/resources/regulation/circulars/regulatory/RG-CFE-2018-005.pdf
  2018-08-12 "Cboe C2018071603"
    https://cdn.cboe.com/resources/release_notes/2018/Change-to-CFE-Pre-Open-Time-for-TAS-Contracts-and-Order-Submission-Commencement-Times.pdf
  2021-12-06 "Cboe C2021102603"
    https://cdn.cboe.com/resources/regulation/rule_filings/pending/2021/21-028-VX-VXM-and-AMERIBOR-Trading-Hours.pdf
```

The 2018-02-25 tuple is keyed to the Sunday implementation; the revised
weekday hours first occur on CFE's Monday 2018-02-26 business date.
