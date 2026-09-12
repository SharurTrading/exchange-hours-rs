<!-- SPDX-License-Identifier: MIT-0 -->

# `globex_nikkei_225_dollar` — evidence

- **Kind:** `MarketHoursKey`
- **Owner module:** [`cme_nikkei.rs`](../../src/calendar/schedules/futures/us/cme_nikkei.rs)
- **Source sets:** [`US-CME-GROUP`](../schedules/sources.md#us-cme-group)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

**Gap: executable** — the uncertainty touches a window where trades print, so this row's history is served conservatively and the basis below says how. Nikkei 225 Dollar (`NKD`) only. Current 17:00→16:00 CT grid sourced from the contract specification, with a dated timeline from 2012-11-18: SER-6465 (session-opening day 2012-11-18) extended the close to 16:15 CT with a 15:15–15:30 CT halt, SER-6554R (2013-03-03) removed that halt for International Equity Index futures naming NKD explicitly, and CME Globex Notice #20150817 (2015-09-20, trade date 2015-09-21) moved the CME Equity close to 16:00 CT. Partial because the pre-2011 interval is omitted rather than modelled. The 2026-09-01 review found why it must be: CME's own trading-hours pages captured 2010-03-10 and 2010-04-07 show a **materially different** NKD grid — CDT 03:00–15:15 reopening 15:30–16:30 and 17:00–18:00, CST 02:00–15:15 with **no Sunday hours** — so the 17:00–15:15 continuous grid cannot be carried across 2010 without reporting the contract open all night when it was closed. An earlier revision of this branch did exactly that; it is corrected. The changeover is undated (2010-04-07 still shows the old grid, 2011-01-12 already shows the new one, with no capture or located notice between), so dates before the first sourced appearance of the served grid are sessionless and the 2010 grid is left sourced-but-unmodelled — encoding it would need seasonal CDT/CST rules and a boundary that is still undated. CME's trading-hours pages captured 2011-01-12 onward state the served grid — Sunday Pre-Open 16:15, ETH (Sunday) 17:00-15:15, weekday Pre-Open "15:25, 16:45", ETH (Weekday) "15:30-16:30, 17:00-15:15", byte-identical to the E-mini S&P 500 row on the same page — so the pre-2012 evening open is 17:00 CT and is primary-sourced. What stays undated is when that grid began, since the 2010 change is attested only by a third-party aggregator; keying a revision to a capture date would fabricate a cutover. Under the carry-back convention the sourced pre-2012 grid (17:00–15:15 with the 15:30–16:30 post-halt segment) is now extended to the January-2010 floor instead of returning no session, since no primary source names a cutover inside that interval; the residual risk — a 2010 grid change attested only by a third-party aggregator — is recorded beside the table. [Stale when moved on 2026-09-12: `nkd_profile_at` returns `NKD_CLOSED` before 2011-01-12 and nothing is carried to the floor; see Gaps and residual risks.]

## Revision rows

- 2011-01-12 — T1 — first sourced CME trading-hours capture of this grid — knowledge boundary: 17:00–15:15 CT with the 15:30–16:30 CT post-halt segment.
- 2012-11-18 — T1 — CME SER-6465 — the close is extended to 16:15 CT with a 15:15–15:30 CT electronic halt.
- 2013-03-03 — T1 — CME SER-6554R — the 15:15–15:30 CT halt is removed for International Equity Index futures, naming NKD explicitly.
- 2015-09-20 — T1 — CME Globex notice 20150817 — the CME Equity close moves to 16:00 CT for trade date Monday 2015-09-21.

## Sources

Retrieval dates: these sources were last opened on the row's reviewed-on date
(2026-08-24, UTC); per-source retrieval dates were not recorded before the
2026-09-12 migration and are added as each source is re-verified.

- <https://www.cmegroup.com/markets/equities/international-indices/nikkei-225-dollar.contractSpecs.html> — CME Nikkei 225 Dollar contract specification, the current grid.
- <https://www.cmegroup.com/CmeWS/mvc/ContractSpecs/List/productId/168> — CME `ContractSpecs` service for product id 168, corroborating the current grid.
- <https://www.cmegroup.com/markets/equities/files/trade-japanese-equity-index-futures-fact-card.pdf> — CME Japanese equity index futures fact card.
- <https://www.cmegroup.com/trading-hours.html> — CME trading-hours page, which states that hours are U.S. Central unless otherwise noted.
- <https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20150817.html> — CME Globex Notice #20150817 of 17 August 2015, the 2015-09-20 revision's source.
- <https://www.cmegroup.com/rulebook/files/ser-6465_Equity_Index_Futures_Options_on_Equity_Index_Futures_20121114.pdf> — CME SER-6465, the 2012-11-18 revision's source.
- <https://www.cmegroup.com/rulebook/files/ser_6554R_-_CME_Modifies_Trading_Hours_for_International_Equity_Index_futures_on_20130304.pdf> — CME SER-6554R, the 2013-03-03 revision's source, naming Nikkei 225 Dollar Futures explicitly.
- <https://web.archive.org/web/20150905151851/http://www.cmegroup.com/trading/equity-index/international-index/nikkei-225-dollar_contract_specifications.html> — NKD contract specification — capture 2015-09-05, "5:00 p.m. previous day - 4:15 p.m.".
- <https://web.archive.org/web/20151127190940/http://www.cmegroup.com:80/trading/equity-index/international-index/nikkei-225-dollar_contract_specifications.html> — NKD contract specification — capture 2015-11-27, "5:00 p.m. - 4:00 p.m. Chicago Time/CT".
- <https://web.archive.org/web/20100310022002id_/http://www.cmegroup.com/trading_hours/> — CME trading-hours index — capture 2010-03-10, the materially different 2010 NKD grid.
- <https://web.archive.org/web/20100407094843id_/http://www.cmegroup.com/trading_hours/> — CME trading-hours index — capture 2010-04-07, still the 2010 grid.
- <https://web.archive.org/web/20110112032949id_/http://www.cmegroup.com/trading_hours/> — CME trading-hours index — capture 2011-01-12, the first sourced appearance of the served grid.
- <https://web.archive.org/web/20110811113223id_/http://www.cmegroup.com/trading_hours/> — CME trading-hours index — capture 2011-08-11, the served grid restated.

Official origin of the trading-hours captures: <http://www.cmegroup.com/trading_hours/>.

## Gaps and residual risks

- **executable** — the 2010 grid is sourced but structurally different and its changeover day is undated, so dates before 2011-01-12 resolve to a sessionless profile rather than carrying either grid. CME's 2010-03-10 and 2010-04-07 captures read a daytime-anchored, DST-dependent grid — CDT 03:00–15:15 reopening 15:30–16:30 and 17:00–18:00, CST 02:00–15:15 with no Sunday hours — and the 2011-01-12 capture already reads the served grid, with no capture and no located CME notice in between. Serving the continuous grid across 2010 would report the contract open all night when it was closed, which an earlier revision of the module did and which is corrected. Closing condition: a CME document that dates the changeover, or a capture inside the 2010-04-07..2011-01-12 window. Served identity, so tracked as an issue (LAW-FOLLOW-UPS-ARE-ISSUES).
- **residual risk** — encoding the 2010 grid itself would need seasonal CDT/CST rules and a boundary that is still undated, so it is left sourced-but-unmodelled.
- **residual risk** — the ledger note carried into this file ends with a sentence, written for an earlier revision of the module, saying the pre-2012 grid "is now extended to the January-2010 floor". The module does not do that and must not: `nkd_profile_at` returns `NKD_CLOSED` below 2011-01-12. The surviving correction earlier in the same note is the authoritative statement.
- **scope** — Nikkei 225 Dollar outrights only. BTIC (`NKT`) is separately scheduled on its own CME-published hours and takes its own key if a consumer maps one; the 16:00–17:00 CT daily break is a maintenance period, not an order-entry phase.

## Module narrative (moved from src/calendar/schedules/futures/us/cme_nikkei.rs on 2026-09-12 UTC)

NKD outrights run one continuous Globex envelope per trade date: the session
opens 17:00 CT on the previous calendar evening and closes 16:00 CT on the
trade date, with the 60-minute 16:00-17:00 CT break separating consecutive
trade dates. CME's own wording: "Sunday - Friday 6:00 p.m. - 5:00 p.m. ET
(5:00 p.m. - 4:00 p.m. CT) with a 60-minute break each day beginning at
5:00 p.m. ET (4:00 p.m. CT)". There is no intraday halt as of the 2026-08-24 review. Friday is
absent from the opening-day mask because a Friday-evening open would belong
to a Saturday trade date, which does not exist; that omission is what
produces the Friday 16:00 CT weekly wrap.

HOW THE NKD GRID DIFFERS FROM THE STANDARD CME EQUITY-INDEX GRID (the reason
`MarketHoursKey::GlobexEquityIndex` explicitly excludes NKD): the U.S.-grid
contracts carry a pit-anchored 08:30-15:15 CT regular session with the
electronic envelope modelled around it as extended hours, and they carried a
15:15-15:30 CT halt until 2021-06-27. NKD is a pure-Globex international
equity-index contract: it has no pit/RTH split, so its entire envelope is the
regular session, and its 15:15-15:30 CT halt was removed eight years earlier,
on 2013-03-04, by a notice scoped to International Equity Index futures only.
The two grids differed from the (undatable, see below) 2010 change through
2012-11-18, and again through the 2013-03-04 halt removal. Today the envelopes
coincide, but the regular/extended split does not, so the key stays separate.

https://www.cmegroup.com/markets/equities/international-indices/nikkei-225-dollar.contractSpecs.html
https://www.cmegroup.com/CmeWS/mvc/ContractSpecs/List/productId/168
https://www.cmegroup.com/markets/equities/files/trade-japanese-equity-index-futures-fact-card.pdf
https://www.cmegroup.com/trading-hours.html

CME publishes no normal-week pre-open or order-entry start time for NKD on the
contract specs page, the ContractSpecs service, or the Japanese equity index
fact card, so no extended phase is asserted. The 16:00-17:00 CT daily break is
a maintenance/closed period, not an order-entry phase, and BTIC ("Sunday -
Friday 6:00 p.m. ET - 3:30 p.m. Tokyo time ... and Monday - Friday Noon to
5:00 p.m. ET") is separately scheduled, on its own published hours, so it is
not a phase of this outright order book. Both are deliberately omitted rather
than modelled as extended sessions.

That is a statement about scope, not about tradability: the Nikkei BTIC
instruments are their own order book with their own CME-published hours, and
the quoted sentence above is itself the primary source for their second daily
window. Should they be authored, they take their own key rather than becoming
a phase here — see the trade-type handoff in `docs/plans/`, whose survey found
exactly one trade-type root out of roughly 180 that genuinely rides its
underlying's clock.

NKD now carries a dated timeline from 2012-11-18. The 16:15 -> 16:00 CT
close, which was previously undatable and forced this family to be modelled
current-only, is dated by CME Globex Notice #20150817 of 17 August 2015:

  "Effective Monday, September 21, the daily CME Globex maintenance period
   will begin 15 minutes earlier Monday through Thursday from 16:00 until
   16:45 Central Time (CT). ... With this change, the closing times for the
   following markets will now occur 15 minutes earlier Monday through Friday
   at 16:00 CT. CME Equity / CBOT Equity / COMEX / NYMEX / DME. All other CME
   Globex markets trading hours remain unchanged."

https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20150817.html

NKD sits in the named "CME Equity" Globex product group, and CME's own NKD
contract-specification captures bracket the change directly: 2015-09-05 reads
"MON - FRI: 5:00 p.m. previous day - 4:15 p.m.", and 2015-11-27 reads
"5:00 p.m. - 4:00 p.m. Chicago Time/CT".
https://web.archive.org/web/20150905151851/http://www.cmegroup.com/trading/equity-index/international-index/nikkei-225-dollar_contract_specifications.html
https://web.archive.org/web/20151127190940/http://www.cmegroup.com:80/trading/equity-index/international-index/nikkei-225-dollar_contract_specifications.html

Revisions are keyed by the local session-opening day, matching `cme_group`:
the first close at 16:00 CT is trade date Monday 2015-09-21, whose session
opened Sunday 2015-09-20.

THE 2011 GRID, AND WHY IT IS NOT CARRIED TO THE FLOOR. CME's trading-hours
pages publish this grid — Electronic Trading (Sunday) "17:00-15:15" and
(Weekday) "15:30-16:30, 17:00-15:15", byte-identical to the E-mini S&P 500 row
beside it — from the 2011-01-12 capture onward. SER-6465 corroborates the
outgoing 15:15 CT close by describing its own change as an extension of it.

It is NOT carried back to the January-2010 floor, because the 2010 grid was
materially different and is sourced. The 2010-03-10 and 2010-04-07 captures of
the same page read, for "Nikkei 225 (Dollar) Futures":

  Electronic (weekday)  CDT: 03:00-15:15 reopens 15:30-16:30; closes
                        16:30-17:00; reopens 17:00-18:00
                        CST: 02:00-15:15; reopens 15:30-16:30; closes 16:30
  Sunday                CDT: Opens 17:00-18:00    CST: No Sunday Hours

That is a daytime-anchored, DST-dependent grid whose evening segment ran only
17:00-18:00 and which had no Sunday session at all in CST. Serving the
17:00-15:15 continuous grid across it would report the contract open through
the whole overnight window when it was closed — a false open, in executable
hours. An earlier revision of this module did exactly that, on the reasoning
that no source named a cutover; a source does, and the carry-back convention
requires that none exists.

The transition is undated: 2010-04-07 still shows the old grid and 2011-01-12
already shows the new one, with no capture and no located CME notice in
between. Dates before the first sourced appearance of the served grid are
therefore modelled sessionless, the same knowledge boundary a launch day
provides. Encoding the 2010 grid itself would need seasonal CDT/CST rules and
a boundary that is still undated, so it is left as sourced-but-unmodelled and
recorded here.
Official origin http://www.cmegroup.com/trading_hours/ delivered via:
https://web.archive.org/web/20100310022002id_/http://www.cmegroup.com/trading_hours/
https://web.archive.org/web/20100407094843id_/http://www.cmegroup.com/trading_hours/
https://web.archive.org/web/20110112032949id_/http://www.cmegroup.com/trading_hours/
https://web.archive.org/web/20110811113223id_/http://www.cmegroup.com/trading_hours/

2012-11-18: "CME Group announces that the new daily trading hour schedule for
  CBOT and CME Equity Index futures and Options on Equity Index futures will
  begin on Sunday, November 18, 2012 for trade date Monday, November 19, 2012."
  https://www.cmegroup.com/rulebook/files/ser-6465_Equity_Index_Futures_Options_on_Equity_Index_Futures_20121114.pdf
2013-03-03: "The modified Globex trading hours will be effective Monday,
  March 4, 2013. The 15 minute trading halt between 3:15 p.m. and 3:30 p.m.,
  Central Time, Monday through Friday, will be eliminated for CME
  International Equity [Index futures] ..." - NKD is named explicitly as
  "Nikkei 225 Dollar Futures". Keyed to the Sunday session-opening day.
  https://www.cmegroup.com/rulebook/files/ser_6554R_-_CME_Modifies_Trading_Hours_for_International_Equity_Index_futures_on_20130304.pdf
2015-09-20: CME Globex Notice #20150817, quoted above; trade date Monday
  2015-09-21, session-opening day Sunday 2015-09-20.
