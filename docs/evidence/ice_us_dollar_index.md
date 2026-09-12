<!-- SPDX-License-Identifier: MIT-0 -->

# `ice_us_dollar_index` — evidence

- **Kind:** `MarketHoursKey`
- **Owner module:** [`ice_usdx.rs`](../../src/calendar/schedules/futures/us/ice_usdx.rs)
- **Source sets:** [`ICE-DERIVATIVES`](../schedules/sources.md#ice-derivatives)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

**Gap: executable** — the uncertainty touches a window where trades print, so this row's history is served conservatively and the basis below says how. U.S. Dollar Index (`DX`) only. Current 20:00→17:00 NY grid with the Sunday 18:00 open confirmed by three independent primary ICE sources; dated 2011-02-14 close revision cited from an archived ICE notice. Partial because no primary source dates the pre-2011 grid. Confirmed 2026-08-31: ICE's 2007 currencies release states only the 20:00–18:00 ET currency-futures grid and never prints a USDX grid, and the earliest surviving master-table edition (August 2011) postdates the 2011-02-14 revision. The 2026-09-01 review established why that interval cannot be sourced: **ICE Futures U.S. sets these hours administratively, not by rule.** Its product rulebook chapters — Sugar No. 11, Cotton No. 2, Coffee, Cocoa, FCOJ and USDX, all captured December 2011 — contain no hours provision at all, and chapter 4 is trade-practice rules, so no SEC or CFTC filing fixes an ICE Futures U.S. trading hour and the master hours table is the only source. Its earliest surviving edition is August 2011. The gap is therefore bounded by document availability rather than by an unfinished search, and the August-2011 carry-back is the terminal answer unless an earlier edition surfaces.

## Revision rows

- 2011-02-14 — T1 — ICE ExNot 020311 DX hours — the electronic trading day ends at 17:00 NY; the notice states the opens are unchanged.

## Sources

Retrieval dates: these sources were last opened on the row's reviewed-on date
(2026-08-23, UTC); per-source retrieval dates were not recorded before the
2026-09-12 migration and are added as each source is re-verified.

- <https://www.ice.com/publicdocs/futures_us/ICE_Futures_US_Regular_Trading_Hours.pdf> — ICE Futures U.S. Regular Trading Hours master table, live edition — the current grid.
- <https://web.archive.org/web/20111212140120id_/https://www.theice.com/publicdocs/futures_us/ICE_Futures_US_Regular_Trading_Hours.pdf> — ICE Regular Trading Hours master table, AUGUST 2011 edition — captured 2011-12-12, the earliest surviving edition.
- <https://web.archive.org/web/20130122132629id_/https://www.theice.com/publicdocs/futures_us/ICE_Futures_US_Regular_Trading_Hours.pdf> — ICE Regular Trading Hours master table, JANUARY 2, 2013 edition — captured 2013-01-22.
- <https://www.ice.com/products/194/US-Dollar-Index-USDX-Futures> — ICE USDX product page 194 — "Market opens at 6:00 pm on Sunday for Monday's trade date. Pre-open at 5:30 pm Sunday, 7:30 pm Monday through Thursday.".
- <https://www.ice.com/publicdocs/futures_us/ICE_Dollar_Index_FAQ.pdf> — ICE US Dollar Index FAQ — the Friday 17:00 wrap and the 30-minute order-entry window.
- <https://web.archive.org/web/20110222135802/https://www.theice.com/publicdocs/futures_us/exchange_notices/ExNot020311DXhours.pdf> — ICE Futures U.S. Exchange Notice, "Change To Electronic Trading Hours For IFUS Currency Contracts", 7 February 2011 — retired from ice.com, cited from the archive capture.
- <https://web.archive.org/web/20101224164411id_/https://www.theice.com/publicdocs/futures_us/exchange_notices/Currencies_Electronic_Trade.pdf> — ICE 2007 currencies release — states the currency-futures grid and never prints a USDX grid.
- <https://web.archive.org/web/20111213003348id_/https://www.theice.com/publicdocs/rulebooks/futures_us/15_USDX.pdf> — ICE Futures U.S. Rulebook chapter 15 (USDX), captured December 2011 — no hours provision.

## Gaps and residual risks

- **executable** — no primary source dates the pre-2011 USDX grid. The baseline's 18:00 NY close and its unchanged opens are known only because ICE's 7 February 2011 notice states the state it replaced; ICE's 2007 currencies release mentions the Dollar Index solely as having "began trading electronically in June" and never prints a USDX grid, and the earliest surviving master-table edition (AUGUST 2011) postdates the 2011-02-14 revision. Closing condition: an ICE document dating the pre-2011 grid. Dormant identity, so recorded here rather than opened as an issue.
- **executable** — January 2010 to August 2011 is unsourced. ICE Futures U.S. sets these hours administratively, not by rule: the product rulebook chapters captured December 2011 carry no hours provision at all and chapter 4 is trade-practice rules, so no SEC or CFTC filing fixes an ICE Futures U.S. trading hour and the master hours table is the only source. Its earliest surviving edition is AUGUST 2011 (captured 2011-12-12), enumerated exhaustively in the 2026-08-31 review. The gap is bounded by document availability rather than by an unfinished search, so the August-2011 carry-back is the terminal answer. Closing condition: an earlier edition of ICE's master hours table surfacing in a public archive. Dormant identity, so the gap is recorded here rather than opened as an issue (LAW-SERVICE-TIERS).
- **wording conflict, resolved** — the 2011 notice's parenthetical lists the unchanged weekday opens as "8:00 pm NY time Tuesday through Thursday evenings", which would leave Tuesday's trade date with no open at all. The master-table footnote applies the previous-business-day rule to every trade date, so the weekday evening mask stays Monday through Thursday; no primary source dates any change to which evenings carry an open, so none is encoded.
- **order-entry** — the 30-minute pre-open is carried back unchanged. ICE states it as a standing platform property rather than a dated one: the April 2011 edition of the USDX FAQ already reads "The ICE trading platform is available for order entry thirty minutes before the opening of trading", and the current product page repeats it.

## Module narrative (moved from src/calendar/schedules/futures/us/ice_usdx.rs on 2026-09-12 UTC)

USDX is a near-24-hour contract whose session commences on the evening before
its trade date, with a Sunday evening that opens two hours earlier than the
weekday evenings. ICE's June 2026 master hours table gives the row as
"USDX(R)   20:00** - 17:00", where footnote ** reads "Trading commences on
previous business day and on Sunday evenings only trading commences at
18:00".

The 18:00 Sunday open is stated by three independent primary ICE documents,
so it is modelled as a distinct rule rather than folded into the weekday
evening grid:
  1. the master hours table footnote ** quoted above;
  2. the ICE US Dollar Index FAQ - "Trading ends at 5:00 p.m. ET on Friday
     afternoon. On Sunday evening, trading in the contracts begins at 6:00
     p.m. ET; the trading session that begins on Sunday evening ends at 5:00
     p.m. ET on the following Monday";
  3. product page 194 - "Market opens at 6:00 pm on Sunday for Monday's trade
     date. Pre-open at 5:30 pm Sunday, 7:30 pm Monday through Thursday."

The weekday evening rule stops at Thursday: a Friday 20:00 open would belong
to a Saturday trade date, which does not exist, and the FAQ fixes the weekly
wrap at "5:00 p.m. ET on Friday afternoon".

https://www.ice.com/publicdocs/futures_us/ICE_Futures_US_Regular_Trading_Hours.pdf
https://www.ice.com/products/194/US-Dollar-Index-USDX-Futures
https://www.ice.com/publicdocs/futures_us/ICE_Dollar_Index_FAQ.pdf

---

Order entry only, 30 minutes ahead of each open, and nothing else: USDX has
no post-close pre-open ("PCPO") window. The 2018 PCPO notice that added one
to the ICE softs names only Coffee "C", Cotton No. 2, Cocoa, FCOJ, Sugar
No. 11 and Sugar No. 16, and neither the USDX product page nor the FAQ
publishes a post-close order-entry phase; the product page's only extended
phase is the Pre-Open column.

Verbatim from the product page: "NEW YORK  8:00 PM - 5:00 PM*  20:00 - 17:00
7:30 PM  19:30" and "Pre-open at 5:30 pm Sunday, 7:30 pm Monday through
Thursday."; the specification block adds "*The trading platform is available
30 minutes before the opening for order entry. Open on Sunday night is 6:00
PM ET; Pre-Open at 5:30 PM ET".

ICE's own wording settles the classification: the platform "is available 30
minutes before the opening for order entry" and nothing matches until the
open, so the Pre-Open goes in order_entry. USDX publishes no tradeable phase
outside its near-24-hour executable session, so the extended slice is empty.

https://www.ice.com/products/194/US-Dollar-Index-USDX-Futures

---

Baseline before 2011-02-14: the same evening opens, closing an hour later at
18:00 NY. The February 2011 notice changed only the close and said so
explicitly - "This change does not affect the start of the electronic trading
day for these products" - so the Sunday 18:00 and weekday-evening 20:00 opens
are carried back rather than reconstructed.

The Sunday rule has equal endpoints here. A SessionRule with `open_ssm ==
close_ssm` encodes exactly one complete local-day span, so Sunday 18:00 runs
continuously to Monday 18:00, which is what the pre-2011 grid described.


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
https://web.archive.org/web/20111213003348id_/https://www.theice.com/publicdocs/rulebooks/futures_us/15_USDX.pdf
2026-08-31: the pre-2011 dating question was re-worked and stays negative.
ICE's 2007 currencies release states only that "Electronic trading hours for
currency futures are from 8:00 p.m. ET through 6:00 p.m. ET each trading
day", and mentions the Dollar Index solely as having "began trading
electronically in June" - it never prints a USDX grid. The earliest surviving
edition of the master table is AUGUST 2011, after the 2011-02-14 close
revision. So no primary source dates the pre-2011 USDX grid, and the gap is
document availability rather than an unfinished search.
https://web.archive.org/web/20101224164411id_/https://www.theice.com/publicdocs/futures_us/exchange_notices/Currencies_Electronic_Trade.pdf

The notice's parenthetical lists the unchanged weekday opens as "8:00 pm NY
time Tuesday through Thursday evenings". That wording would leave Tuesday's
trade date with no open at all, and the master hours table footnote applies
the previous-business-day rule to every trade date, so the weekday evening
mask stays Monday through Thursday. No primary source dates any change to
which evenings carry an open, so none is encoded.

---

The 30-minute pre-open is carried back unchanged. ICE states it as a standing
platform property rather than a dated one - the April 2011 edition of the US
Dollar Index FAQ already reads "The ICE trading platform is available for
order entry thirty minutes before the opening of trading", and the current
product page repeats it. No primary source dates its introduction, so no
cutover is encoded for the order-entry phases; only the close moves in 2011.
The FAQ's wording - "available for order entry thirty minutes before the
opening of trading" - is also why the phase is order_entry in this era, not a
tradeable extended session.

https://www.ice.com/publicdocs/futures_us/ICE_Dollar_Index_FAQ.pdf

---

2011-02-14: "Effective with the start of trading for trade date Monday,
  February 14, 2011, the Exchange is implementing a change to electronic
  trading hours for the USDX futures and options contracts, the Euro Index
  futures contract and for all currency pair futures contracts on the ICE
  electronic trading platform: the trading day for these contracts will now
  end at 5:00 pm NY time. This change does not affect the start of the
  electronic trading day for these products (which remains as 6:00 pm NY time
  on Sunday evenings, 8:00 pm NY time Tuesday through Thursday evenings) or
  the settlement window for these products (which remains 2:59 to 3:00 pm)."
  ICE Futures U.S. Exchange Notice, "Change To Electronic Trading Hours For
  IFUS Currency Contracts", February 7, 2011. The notice has been retired
  from ice.com; per the crate's established practice for superseded operator
  documents, the archive capture is cited as-is.
  https://web.archive.org/web/20110222135802/https://www.theice.com/publicdocs/futures_us/exchange_notices/ExNot020311DXhours.pdf
Evidence: docs/evidence/ice_us_dollar_index.md
