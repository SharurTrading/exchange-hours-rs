<!-- SPDX-License-Identifier: MIT-0 -->

# `ice_us_orange_juice` — evidence

- **Kind:** `MarketHoursKey`
- **Owner module:** [`ice_fcoj.rs`](../../src/calendar/schedules/futures/us/ice_fcoj.rs)
- **Source sets:** [`ICE-DERIVATIVES`](../schedules/sources.md#ice-derivatives)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

**Gap: executable** — the uncertainty touches a window where trades print, so this row's history is served conservatively and the basis below says how. FCOJ-A (`OJ`) only. Current 08:00–14:00 NY grid and both order-entry windows sourced, with the dated 2018-10-08 PCPO revision. Partial because no primary source dates any earlier change inside the modelled window; the 2026-08-31 review made that positive rather than inferential by finding "FCOJ-A   8:00 - 14:00" unchanged in both the AUGUST 2011 and JANUARY 2, 2013 master-table editions. January 2010 to August 2011 stays unsourced. The 2026-09-01 review established why that interval cannot be sourced: **ICE Futures U.S. sets these hours administratively, not by rule.** Its product rulebook chapters — Sugar No. 11, Cotton No. 2, Coffee, Cocoa, FCOJ and USDX, all captured December 2011 — contain no hours provision at all, and chapter 4 is trade-practice rules, so no SEC or CFTC filing fixes an ICE Futures U.S. trading hour and the master hours table is the only source. Its earliest surviving edition is August 2011. The gap is therefore bounded by document availability rather than by an unfinished search, and the August-2011 carry-back is the terminal answer unless an earlier edition surfaces.

## Revision rows

- 2018-10-08 — T1 — ICE PCPO notice 20180920 — post-close pre-open added, 14:30-18:00 NY.

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
- <https://www.ice.com/products/30/FCOJ-A-Futures> — ICE FCOJ-A product page — "NEW YORK  8:00 AM - 2:00 PM" and the PCPO footnote.
- <https://web.archive.org/web/20111213010113id_/https://www.theice.com/publicdocs/rulebooks/futures_us/13_FCOJ.pdf> — ICE Futures U.S. Rulebook chapter 13 (FCOJ), captured December 2011 — no hours provision.

## Gaps and residual risks

- **executable** — January 2010 to August 2011 is unsourced. ICE Futures U.S. sets these hours administratively, not by rule: the product rulebook chapters captured December 2011 carry no hours provision at all and chapter 4 is trade-practice rules, so no SEC or CFTC filing fixes an ICE Futures U.S. trading hour and the master hours table is the only source. Its earliest surviving edition is AUGUST 2011 (captured 2011-12-12), enumerated exhaustively in the 2026-08-31 review. The gap is bounded by document availability rather than by an unfinished search, so the August-2011 carry-back is the terminal answer. Closing condition: an earlier edition of ICE's master hours table surfacing in a public archive. Dormant identity, so the gap is recorded here rather than opened as an issue (LAW-SERVICE-TIERS).
- **horizon precision** — the AUGUST 2011 edition of the master table is month-dated by ICE and carries no day, so the ledger horizon is written as `2011-08-01`, the first day of the edition's own month. No ICE document states a day-level date for that edition.
- **order-entry** — the Friday-evening leg of the 20:00 pre-open is omitted, for the reason the sugar row records. The pre-open start itself is only ever stated in undated product-page form ("Pre-Open ... 8:00 PM 20:00"); ICE publishes no notice establishing or moving it, so it is carried in the baseline rather than given a cutover.
- **corroboration** — ICE's 2014 softs hours notice re-tabulated Sugar No. 11, Coffee "C", Cocoa, Cotton No. 2 and Sugar No. 16 and then stated "Daily trading hours for all other products are unchanged", which excludes FCOJ-A from that change.

## Module narrative (moved from src/calendar/schedules/futures/us/ice_fcoj.rs on 2026-09-12 UTC)

FCOJ-A runs one same-day executable session. The ICE master hours table
carries no footnote marker on the FCOJ-A row - neither the single asterisk
("Trading commences on previous business day") nor the double asterisk that
additionally moves the Sunday open - so nothing commences on the previous
calendar evening and there is no Sunday session. The executable week runs
Monday 08:00 through Friday 14:00 New York.

Master table, verbatim: "FCOJ-A   8:00 - 14:00". Corroborated by the ICE
product page: "NEW YORK  8:00 AM - 2:00 PM  08:00 - 14:00".


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
https://web.archive.org/web/20111213010113id_/https://www.theice.com/publicdocs/rulebooks/futures_us/13_FCOJ.pdf
2026-08-31: the same "FCOJ-A   8:00 - 14:00" row appears in the AUGUST 2011
and JANUARY 2, 2013 editions of the master table, so "no earlier change
inside the modelled window" is now supported by two dated ICE documents
rather than by absence of evidence. The residual gap is January 2010 to
August 2011, for which no edition survives in the archive.
Dated editions of ICE's own master table, official origin
https://www.ice.com/publicdocs/futures_us/ICE_Futures_US_Regular_Trading_Hours.pdf
delivered via
https://web.archive.org/web/20111212140120id_/https://www.theice.com/publicdocs/futures_us/ICE_Futures_US_Regular_Trading_Hours.pdf
https://web.archive.org/web/20130122132629id_/https://www.theice.com/publicdocs/futures_us/ICE_Futures_US_Regular_Trading_Hours.pdf

https://www.ice.com/publicdocs/futures_us/ICE_Futures_US_Regular_Trading_Hours.pdf
https://www.ice.com/products/30/FCOJ-A-Futures

---

Two order-entry-only phases, neither of which matches: the post-close pre-open
("PCPO") beginning 30 minutes after the 14:00 close and ending at 18:00, and
the regular pre-open from 20:00 running to the next morning's 08:00 open. Both
attach to the Exchange business day prior to the trade date, so the windows
feeding a Tuesday session are Monday 14:30-18:00 and Monday 20:00 onward.

The 20:00 pre-open start is modelled Monday-Thursday only. ICE places it "on
the prior Exchange business day", which for a Monday trade date is the
preceding Friday - a 60-hour order-entry window spanning the weekend. No ICE
document states that order entry stays live across the weekend, so the
Friday-evening leg is deliberately omitted rather than asserted. The PCPO is
modelled Monday-Friday because Friday 14:30-18:00 is an ordinary same-week
window that ICE's own worked example confirms (the PCPO for trade date Monday
8 October 2018 fell on Friday 5 October 2018).

Product page footnote, verbatim: "**In addition to the Pre-Open start time
shown above, there will be a Post-Close Pre-Open order entry session from 2:30
pm to 6:00 pm NY time on the prior Exchange business day."

Both windows are order_entry, not extended: the product page names the PCPO
an "order entry session", and Rule 4.22(a) confines the Pre-Trading Session
to Limit order entry with the Opening Match at the open. FCOJ-A publishes no
tradeable phase outside its 08:00-14:00 executable session, so the extended
slice is empty.

https://www.ice.com/products/30/FCOJ-A-Futures
https://www.ice.com/publicdocs/rulebooks/futures_us/4_Trading.pdf

---

Baseline before 2018-10-08: the executable session is already the 2026-08-23 review's
08:00-14:00 grid, but the PCPO order-entry window does not exist yet, leaving
the 20:00 pre-open as the only non-executable phase - order entry, like the
current one, so this era carries no extended phase either.

The executable grid is carried back unchanged rather than dated to an earlier
cutover. ICE's 2014 softs hours notice re-tabulated Sugar No. 11, Coffee "C",
Cocoa, Cotton No. 2 and Sugar No. 16 and then stated "Daily trading hours for
all other products are unchanged", which excludes FCOJ-A; no primary ICE
document inside the modelled window states a different FCOJ-A open or close,
so no earlier revision is invented here.

https://www.ice.com/publicdocs/futures_us/exchange_notices/ExNot012714Hours.pdf

---

2018-10-08: "Commencing for trade date Monday, October 8, 2018, the pre-open
  order entry session for Coffee "C", Cotton No. 2, Cocoa, FCOJ, Sugar No. 11
  and Sugar No. 16 futures contracts will be enhanced by the addition of a new
  post-close pre-open ("PCPO") session that will start at 30 minutes after the
  end of trading for the contract and end at 6:00 pm on the Exchange business
  day prior to each trading day."
  https://www.ice.com/publicdocs/futures_us/exchange_notices/ICE_Futures_US_PCPO_Session_20180920.pdf

FCOJ is named in the notice by contract, so the 14:00 close puts the FCOJ-A
PCPO at 14:30-18:00, matching the 2:30 pm figure the product page footnote
carries at the 2026-08-23 review. The executable session is untouched by this notice.

No other FCOJ-A schedule change inside the modelled window is dated by a
primary ICE source. The pre-open start itself is only ever stated in undated
product-page form ("Pre-Open ... 8:00 PM 20:00"); ICE publishes no notice
establishing or moving it, so it is carried in the baseline rather than given
a cutover.
Evidence: docs/evidence/ice_us_orange_juice.md
