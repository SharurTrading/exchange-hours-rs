<!-- SPDX-License-Identifier: MIT-0 -->

# `ice_us_cocoa` — evidence

- **Kind:** `MarketHoursKey`
- **Owner module:** [`ice_cocoa.rs`](../../src/calendar/schedules/futures/us/ice_cocoa.rs)
- **Source sets:** [`ICE-DERIVATIVES`](../schedules/sources.md#ice-derivatives)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

**Gap: executable** — the uncertainty touches a window where trades print, so this row's history is served conservatively and the basis below says how. Cocoa (`CC`) only. Current 04:45–13:30 NY grid sourced, with dated 2014-02-03 and 2018-10-08 revisions. Partial for the same reason as Coffee, and improved the same way: both dated master-table editions print "Cocoa   4:00 - 14:00", leaving only the January 2010 to August 2011 interval unsourced. The 2026-09-01 review established why that interval cannot be sourced: **ICE Futures U.S. sets these hours administratively, not by rule.** Its product rulebook chapters — Sugar No. 11, Cotton No. 2, Coffee, Cocoa, FCOJ and USDX, all captured December 2011 — contain no hours provision at all, and chapter 4 is trade-practice rules, so no SEC or CFTC filing fixes an ICE Futures U.S. trading hour and the master hours table is the only source. Its earliest surviving edition is August 2011. The gap is therefore bounded by document availability rather than by an unfinished search, and the August-2011 carry-back is the terminal answer unless an earlier edition surfaces.

## Revision rows

- 2014-02-03 — T1 — ICE ExNot 012714 hours — grid becomes 04:45-13:30 NY.
- 2018-10-08 — T1 — ICE PCPO notice 20180920 — post-close pre-open added, 14:00-18:00 NY.

## Sources

Retrieval dates: these sources were last opened on the row's reviewed-on date
(2026-08-23, UTC); per-source retrieval dates were not recorded before the
2026-09-12 migration and are added as each source is re-verified.

- <https://www.ice.com/publicdocs/futures_us/ICE_Futures_US_Regular_Trading_Hours.pdf> — ICE Futures U.S. Regular Trading Hours master table, live edition — the current grid.
- <https://web.archive.org/web/20111212140120id_/https://www.theice.com/publicdocs/futures_us/ICE_Futures_US_Regular_Trading_Hours.pdf> — ICE Regular Trading Hours master table, AUGUST 2011 edition — captured 2011-12-12, the earliest surviving edition.
- <https://web.archive.org/web/20130122132629id_/https://www.theice.com/publicdocs/futures_us/ICE_Futures_US_Regular_Trading_Hours.pdf> — ICE Regular Trading Hours master table, JANUARY 2, 2013 edition — captured 2013-01-22.
- <https://www.ice.com/publicdocs/futures_us/exchange_notices/ICE_Futures_US_PCPO_Session_20180920.pdf> — ICE Futures U.S. notice, 20 September 2018, extension of the pre-open order entry session (PCPO).
- <https://www.ice.com/publicdocs/futures_us/exchange_notices/ExNot012714Hours.pdf> — ICE Futures U.S. Exchange Notice, Changes to Daily Trading Hours, effective trade date 2014-02-03.
- <https://www.ice.com/publicdocs/rulebooks/futures_us/4_Trading.pdf> — ICE Futures U.S. Rulebook chapter 4 (Trading) — Rule 4.22(a) confines the Pre-Trading Session to Limit order entry.
- <https://web.archive.org/web/20111213011033id_/https://www.theice.com/publicdocs/rulebooks/futures_us/11_Sugar_11.pdf> — ICE Futures U.S. Rulebook chapter 11 (Sugar No. 11), captured December 2011 — no hours provision.
- <https://web.archive.org/web/20111213011055id_/https://www.theice.com/publicdocs/rulebooks/futures_us/8_Coffee.pdf> — ICE Futures U.S. Rulebook chapter 8 (Coffee), captured December 2011 — no hours provision.
- <https://web.archive.org/web/20111213011442id_/https://www.theice.com/publicdocs/rulebooks/futures_us/9_Cocoa.pdf> — ICE Futures U.S. Rulebook chapter 9 (Cocoa), captured December 2011 — no hours provision.
- <https://www.ice.com/products/7/Cocoa-Futures> — ICE Cocoa product page — the current grid and the pre-open footnote.

## Gaps and residual risks

- **executable** — January 2010 to August 2011 is unsourced. ICE Futures U.S. sets these hours administratively, not by rule: the product rulebook chapters captured December 2011 carry no hours provision at all and chapter 4 is trade-practice rules, so no SEC or CFTC filing fixes an ICE Futures U.S. trading hour and the master hours table is the only source. Its earliest surviving edition is AUGUST 2011 (captured 2011-12-12), enumerated exhaustively in the 2026-08-31 review. The gap is bounded by document availability rather than by an unfinished search, so the August-2011 carry-back is the terminal answer. Closing condition: an earlier edition of ICE's master hours table surfacing in a public archive. Dormant identity, so the gap is recorded here rather than opened as an issue (LAW-SERVICE-TIERS).
- **horizon precision** — the AUGUST 2011 edition of the master table is month-dated by ICE and carries no day, so the ledger horizon is written as `2011-08-01`, the first day of the edition's own month. No ICE document states a day-level date for that edition.
- **order-entry** — the Friday-evening leg of the 20:00 pre-open is omitted, for the reason the sugar row records.

## Module narrative (moved from src/calendar/schedules/futures/us/ice_cocoa.rs on 2026-09-12 UTC)

Cocoa runs one same-day executable session; the ICE master hours table carries
no footnote marker on its row, so nothing commences on the previous calendar
evening. Order entry is a separate, non-matching phase.

The 20:00 pre-open is modelled Monday-Thursday. ICE runs it "on the prior
Exchange business day", so a Monday trade date is fed by the preceding Friday
evening. That leg is not expressible here: a wrapping SessionRule always
wraps into the NEXT local day, so a Friday rule would assert order entry on
Saturday morning rather than carrying through to Monday. The Friday PCPO is
unaffected and is modelled MON_FRI, because it opens and closes inside one
local day. The omission is a limit of the normal-week rule model, not a claim
that ICE closes order entry over the weekend.

https://www.ice.com/publicdocs/futures_us/ICE_Futures_US_Regular_Trading_Hours.pdf
https://www.ice.com/products/7/Cocoa-Futures

---

Two order-entry-only phases: the post-close pre-open ("PCPO") beginning 30
minutes after the 13:30 close, and the regular pre-open from 20:00 running to
the next morning's open.

Both are classified order_entry, not extended: nothing matches in either. The
2018 notice creating the PCPO calls it an extension of the "pre-open order
entry session" and kills Day orders entered in it at its end, and the
pre-open only accepts orders ahead of the Opening Match at the open itself.
Cocoa publishes no tradeable phase outside its executable session, so the
extended slice is empty.

https://www.ice.com/publicdocs/futures_us/exchange_notices/ICE_Futures_US_PCPO_Session_20180920.pdf

---

Baseline before 2014-02-03: open 04:00 NY, close 14:00 NY.

Sourcing caveat, stated plainly: the January 2014 notice prints only the NEW
grid, marking in bold which of those figures moved. It never prints the times
it replaced.


2026-09-01: WHY THE 2010-2011 INTERVAL CANNOT BE SOURCED. ICE Futures U.S.
sets these hours administratively, not by rule. Its product rulebook chapters
- Sugar No. 11, Cotton No. 2, Coffee, Cocoa, FCOJ and USDX, all captured
December 2011 - contain no hours provision at all, and chapter 4 is
trade-practice rules. No SEC or CFTC filing therefore fixes an ICE Futures
U.S. trading hour, and the master hours table is the only source; its earliest
surviving edition is August 2011. This interval is bounded by document
availability, not by an unfinished search, so the carry-back above is the
terminal answer unless an earlier edition surfaces.
https://web.archive.org/web/20111213011033id_/https://www.theice.com/publicdocs/rulebooks/futures_us/11_Sugar_11.pdf
https://web.archive.org/web/20111213011055id_/https://www.theice.com/publicdocs/rulebooks/futures_us/8_Coffee.pdf
https://web.archive.org/web/20111213011442id_/https://www.theice.com/publicdocs/rulebooks/futures_us/9_Cocoa.pdf
SOURCING CAVEAT SUPERSEDED 2026-08-31. The pre-2014 grid is no longer merely
corroborated: two dated editions of ICE's own "ICE Futures U.S. Regular
Trading Hours" master table print it outright. The AUGUST 2011 edition and
the JANUARY 2, 2013 edition both read "Cocoa   4:00 - 14:00", so the
04:00 open and 14:00 close are stated by primary ICE documents at two dated
points spanning up to the 2014-02-03 change. Because no primary document
dates a cutover earlier than 2014-02-03 inside the modelled window, this grid
is still carried back as the baseline rather than inventing an earlier
revision; the residual gap is January 2010 to August 2011, for which no
edition of the master table survives in the archive.
Dated editions of ICE's own master table, official origin
https://www.ice.com/publicdocs/futures_us/ICE_Futures_US_Regular_Trading_Hours.pdf
delivered via
https://web.archive.org/web/20111212140120id_/https://www.theice.com/publicdocs/futures_us/ICE_Futures_US_Regular_Trading_Hours.pdf
https://web.archive.org/web/20130122132629id_/https://www.theice.com/publicdocs/futures_us/ICE_Futures_US_Regular_Trading_Hours.pdf

The pre-open likewise runs from 20:00 on the prior Exchange business day; only
its end differs, tracking the 04:00 open of this era. It is order entry, not
trading, so it sits in the order_entry slice.

https://www.ice.com/publicdocs/futures_us/exchange_notices/ExNot012714Hours.pdf

---

2014-02-03: "Effective with the start of trading for trade date Monday,
  February 3, 2014, the Exchange will implement changes to daily trading hours
  for Sugar No. 11, Coffee "C", Cocoa, Cotton No. 2 and Sugar No. 16 futures
  and option contracts. ... Cocoa 4:45 13:30"
  https://www.ice.com/publicdocs/futures_us/exchange_notices/ExNot012714Hours.pdf
2018-10-08: "Commencing for trade date Monday, October 8, 2018, the pre-open
  order entry session for Coffee "C", Cotton No. 2, Cocoa, FCOJ, Sugar No. 11
  and Sugar No. 16 futures contracts will be enhanced by the addition of a new
  post-close pre-open ("PCPO") session that will start at 30 minutes after the
  end of trading for the contract and end at 6:00 pm on the Exchange business
  day prior to each trading day."
  https://www.ice.com/publicdocs/futures_us/exchange_notices/ICE_Futures_US_PCPO_Session_20180920.pdf
Evidence: docs/evidence/ice_us_cocoa.md
