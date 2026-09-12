<!-- SPDX-License-Identifier: MIT-0 -->

# `nyse` — evidence

- **Kind:** `Exchange`
- **Owner module:** [`nyse.rs`](../../src/calendar/schedules/equities/us/nyse.rs)
- **Source sets:** [`US-NYSE-EQUITIES`](../schedules/sources.md#us-nyse-equities)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

**Gap: executable** — reclassified 2026-09-02, having been recorded as order-entry. The order-entry half is now **closed**: the 06:30 acceptance edge is NYSE Rule 7.34(a)(1) rulebook text (30 minutes before the 07:00 Early Trading Session), and NYSE's own filings date its production day unconditionally — "On April 9, 2018, the Exchange began trading UTP Securities on the Exchange on the Pillar trading platform" (83 FR 23313, restated in 84 FR 37702) — so 2018-04-09 is now a dated revision with both-sides tests. What remains is **executable and newly identified**: NYSE ran Crossing Session II, the surviving leg of its Off-Hours Trading Facility, from before the audit floor until it decommissioned the facility effective 18:30 on 2024-01-31 (89 FR 14909, and the exchange's Trader Updates of 2023-06-30, 2023-08-03 and 2024-01-05). Crossing Session I was eliminated in 2009, below the floor. The crate models no post-16:00 phase for `nyse`, so it errs toward closed; whether member-organization aggregate-priced basket crosses belong in this row's cash-equity envelope at all is an open scope decision, and the interval's amendment chain is unestablished. The pre-2018 Tape A order-acceptance edge is also still unmodelled. **Systems in scope (2026-09-02):** the Pillar equities matching system is the envelope, but two further New York Stock Exchange LLC systems sit in neither place — NYSE Bonds (04:00–20:00) and the Off-Hours Trading Facility/Crossing Session II (16:00–18:30, decommissioned 2024-01-31). Both are on the system-coverage discrepancy list; the modeled envelope is conservative, never over-served.

## Revision rows

- 2018-04-09 — T1 — SEC 34-83230 (NYSE UTP Pillar production) — UTP securities begin trading on Pillar, adding the 06:30–07:00 acceptance queue and the 07:00–09:30 Early Trading Session to the venue envelope.

## Sources

No per-URL retrieval date is recorded in this repository; every link below
was read at or before the row's `Reviewed on` date in the ledger.

- <https://www.nyse.com/trade/hours-calendars?os=.> — NYSE hours and calendars, the current phase table.
- <https://www.nyse.com/markets/hours-calendars> — the companion hours page, which also publishes the NYSE Bonds phase table.
- <https://www.nyse.com/regulation/rules> — the NYSE market rule books behind Rule 7.34(a)(1).
- <https://www.federalregister.gov/documents/2017/08/09/2017-16742/self-regulatory-organizations-new-york-stock-exchange-llc-notice-of-filing-of-proposed-rule-change> — SR-NYSE-2017-36, the UTP Pillar filing adopting Rule 7.34(a)(1).
- <https://www.federalregister.gov/documents/2018/03/29/2018-06339/self-regulatory-organizations-new-york-stock-exchange-llc-notice-of-filing-of-amendment-no-1-and> — the 2018 amendment filing.
- <https://www.federalregister.gov/documents/2018/05/18/2018-10606/self-regulatory-organizations-new-york-stock-exchange-llc-notice-of-filing-and-immediate> — 83 FR 23313, "On April 9, 2018, the Exchange began trading UTP Securities on the Exchange on the Pillar trading platform."
- <https://www.federalregister.gov/documents/2019/08/01/2019-16365/self-regulatory-organizations-new-york-stock-exchange-llc-notice-of-filing-and-immediate> — 84 FR 37702, which restates the same production day.
- <https://www.federalregister.gov/documents/2024/02/29/2024-04168/self-regulatory-organizations-new-york-stock-exchange-llc-notice-of-filing-and-immediate> — 89 FR 14909, the Crossing Session II decommissioning.
- <https://www.federalregister.gov/documents/full_text/text/2022/08/18/2022-17749.txt> — SR-NYSE-2022-37 (87 FR 50906), "Crossing Session II ... operates between 4:00 p.m. and 6:30 p.m."
- <https://www.federalregister.gov/documents/full_text/text/2024/02/21/2024-03449.txt> — SR-NYSE-2024-06 (89 FR 13132), which deletes Rule 7.39 and states the facility was decommissioned effective 2024-01-31.
- <https://www.federalregister.gov/documents/full_text/text/2025/03/04/2025-03432.txt> — the SEC's 2025 Section 36 order identifying NYSE Bonds as a facility of the Exchange (90 FR 11194).

## Gaps and residual risks

- **executable** — NYSE ran Crossing Session II, the surviving leg of its
  Off-Hours Trading Facility, from before the January-2010 floor until it
  decommissioned the facility effective 18:30 on 2024-01-31. The crate models no
  post-16:00 phase for `nyse`, so it errs toward closed rather than open.
  Crossing Session I was eliminated in 2009, below the floor. Two things block a
  fix: whether member-organization aggregate-priced basket crosses belong in this
  row's cash-equity envelope at all is an open scope decision, and the interval's
  amendment chain is unestablished. Widening an executable envelope from two
  endpoints is exactly the inference this crate refuses. Closing condition: the
  scope decision, plus the complete amendment chain for the 2010-01 to
  2024-01-31 interval. Dormant identity, so it is recorded here rather than
  opened as an issue (LAW-FOLLOW-UPS-ARE-ISSUES).
- **order-entry** — the pre-2018 Tape A order-acceptance edge is unmodelled. The
  post-2018 edge is closed: Rule 7.34(a)(1) is rulebook text and the production
  day is stated unconditionally by NYSE's own filings.
- **Horizon carried below the first dated row.** The baseline below 2018-04-09 is the 09:30–16:00
  core session, carried rather than sourced at a named day: no reviewed artifact
  in this row's material states the core session's hours on a floor-era date, so
  the ledger horizon is 2018-04-09, the first day at which this row's state is
  sourced, and everything below it is carried. Closing condition: a floor-era
  NYSE rulebook edition or hours publication that states the core session, which
  would move the horizon down to the January-2010 floor.
- **System coverage (2026-09-02), discrepancy #1.** NYSE Bonds (Early
  04:00–08:00, Core 08:00–17:00, Late 17:00–20:00 ET, with an Opening Bond
  Auction at 04:00 and a Core Bond Auction at 08:00) is a facility of New York
  Stock Exchange LLC and sits in neither the envelope nor a modelled identity.
  Decision required: a new `nyse_bonds` identity, or an explicit scope exclusion.
  It is not an envelope amendment — folding it in would widen the `nyse`
  cash-equity envelope from 06:30–16:00 to 04:00–20:00 for NMS stocks that
  cannot trade there.
- **System coverage (2026-09-02), discrepancy #2.** The Off-Hours Trading
  Facility entry above is the highest-severity item on the list, because trades
  printed in it.

## Module narrative (moved from src/calendar/schedules/equities/us/nyse.rs on 2026-09-12 UTC)

Pillar order-entry edges. NYSE's hours table lists "Order Entry" starting at
06:30 (02:30 on Arca) with the first executable phase — the Early Trading
Session — beginning only at 07:00 (04:00 on Arca). Nothing can print inside
these windows: they exist so orders can be entered, amended and cancelled
ahead of the first matching session, so they are `order_entry`, not
`extended`.
https://www.nyse.com/markets/hours-calendars

NYSE accepts orders from 06:30 ET. Tape A queues them for the core opening;
Tapes B/C also enter an active early session at 07:00. The 06:30–07:00 leg is
therefore acceptance only — no trade can print before the Early Trading
Session opens — and is classified `order_entry`; 07:00–09:30 stays Extended
because Tape B/C trades execute there.

The acceptance edge is a **rulebook** provision, not an operator system
setting: NYSE Rule 7.34(a)(1), adopted with the UTP Pillar filing
(SR-NYSE-2017-36), sets the Early Trading Session at 07:00 and provides that
"the Exchange would begin accepting orders 30 minutes before the Early
Trading Session begins, which means order entry acceptance would begin at
6:30 a.m. Eastern Time". Two later NYSE filings state the production day
unconditionally — "On April 9, 2018, the Exchange began trading UTP
Securities on the Exchange on the Pillar trading platform" — and the
exchange's own Trader Update of 2018-03-27 announced the same day. That is
the day the venue envelope first carried both phases, so it is the onset row
here; NYSE-listed (Tape A) symbols migrated to Pillar in tranches from
2019-08-05, which extends the same phases to more symbols without moving the
venue-level onset (see the envelope convention in `AGENTS.md`).

Before 2018-04-09 the modelled grid stays the sourced 09:30–16:00 core
session. Two pre-Pillar phases remain unmodelled and are recorded as the
row's residual gap in `docs/schedules/verification.md`: the pre-Pillar Tape A
order-acceptance edge, and Crossing Session II (16:00–18:30), the surviving
leg of the Off-Hours Trading Facility, which ran from before the audit floor
until the exchange decommissioned it effective 18:30 on 2024-01-31. Crossing
Session I was eliminated in 2009, below the floor. Modelling the crossing
leg would widen the executable envelope, so it waits on its full amendment
chain rather than being added from the endpoints.
https://www.federalregister.gov/documents/2017/08/09/2017-16742/self-regulatory-organizations-new-york-stock-exchange-llc-notice-of-filing-of-proposed-rule-change
https://www.federalregister.gov/documents/2018/03/29/2018-06339/self-regulatory-organizations-new-york-stock-exchange-llc-notice-of-filing-of-amendment-no-1-and
https://www.federalregister.gov/documents/2018/05/18/2018-10606/self-regulatory-organizations-new-york-stock-exchange-llc-notice-of-filing-and-immediate
https://www.federalregister.gov/documents/2019/08/01/2019-16365/self-regulatory-organizations-new-york-stock-exchange-llc-notice-of-filing-and-immediate
https://www.federalregister.gov/documents/2024/02/29/2024-04168/self-regulatory-organizations-new-york-stock-exchange-llc-notice-of-filing-and-immediate
https://www.nyse.com/trade/hours-calendars?os=.
https://www.nyse.com/markets/hours-calendars

Row evidence:
  2018-04-09 "SEC 34-83230 (NYSE UTP Pillar production)"
    83 FR 23313: "On April 9, 2018, the Exchange began trading UTP
    Securities on the Exchange on the Pillar trading platform."
    Restated in 84 FR 37702 and announced in the NYSE Trader Update of
    2018-03-27, "NYSE will begin trading Tape B and C securities on
    April 9, 2018".

NYSE Arca accepts and queues orders at 02:30 before its 04:00 active early
session. That queue matches nothing, so it is `order_entry`; the 04:00–20:00
execution grid stays Extended and predates the audit floor.

The acceptance edge is a **rulebook** provision throughout. Pre-Pillar Rule
7.35(a)(1) had the Corporation begin accepting orders 30 minutes before the
04:00 Opening Session, i.e. at 03:30; the Pillar I filing carried that text
into Rule 7.34-E(a)(1) "without any substantive differences". No reviewed
primary source names a change to that 30-minute edge between the January-2010
floor and the 2015 Pillar filing, so 03:30 is carried back to the floor and
no revision row is asserted for it. SR-NYSEArca-2021-71 then amended Rule
7.34-E(a)(1) from 30 to 90 minutes (02:30); the filing itself deferred
production to a Trader Update, and that Trader Update — issued 2021-08-11 and
repeated as a reminder on 2021-09-09 — states the day unconditionally:
"Beginning on Monday, September 13, 2021, NYSE Arca will change the time for
order entry to 2:30 a.m. ET. Currently, NYSE Arca opens for order entry at
3:30 a.m. ET." Archived captures of the exchange's own hours page agree,
showing 3:30 on 2021-09-12 and 2:30 on 2021-09-27.

The announced 23-hour Overnight Session (from Sunday 2026-12-06) is a future
change tracked in `docs/schedules/updating.md`, not encoded here.
https://www.sec.gov/files/rules/sro/nysearca/2008/34-57505.pdf
https://www.federalregister.gov/documents/2015/05/19/2015-12028/self-regulatory-organizations-nyse-arca-inc-notice-of-filing-of-proposed-rule-change-adopting-new
https://www.federalregister.gov/documents/2021/08/18/2021-17673/self-regulatory-organizations-nysearca-inc-notice-of-filing-and-immediate-effectiveness-of-proposed
https://www.nyse.com/trader-update/history#110000372318
https://www.nyse.com/trade/hours-calendars?os=.
https://www.nyse.com/publicdocs/nyse/data/ArcaBook_Client_Specification.pdf
https://www.nyse.com/publicdocs/nyse/markets/nyse-arca/rule-filings/filings/2021/SR-NYSEArca-2021-71.pdf

Row evidence:
  2021-09-13 "NYSE Trader Update 2021-08-11"
    https://www.nyse.com/trader-update/history#110000372318
    Reminder of 2021-09-09: https://www.nyse.com/trader-update/history#110000381060

NYSE American's Pillar launch added its current 06:30 order-acceptance edge
around the 07:00–20:00 execution grid on 2017-07-24. The acceptance edge is
`order_entry` — 06:30–07:00 matches nothing — and the execution grid is
Extended. Before Pillar, the sourced continuous session was 09:30–16:00.

Like its NYSE and NYSE Arca siblings, the acceptance edge is a **rulebook**
provision: NYSE American Rule 7.34E(a)(1), adopted by SR-NYSEMKT-2017-01,
puts the Early Trading Session at 07:00 and provides that the exchange "would
begin accepting orders 30 minutes before the Early Trading Session begins,
which means order entry acceptance would begin at 6:30 a.m." The Commission
separately records the production day without condition: "NYSE American's
cash equities market transitioned to Pillar on July 24, 2017."

Legacy off-hours crosses are still not backfilled. That residue is an
**executable** gap, not an order-entry one: at the audit floor NYSE Amex
participated in the NYSE Off-Hours Trading Facility crossing sessions, and
the 2024 NYSE price-list filing records that Crossing Session I was
eliminated in 2009 while Crossing Session II ran until 18:30 on 2024-01-31.
The exact January-2010 NYSE Amex phase table and its complete amendment chain
have not been established, and widening the executable envelope from two
endpoints is exactly the inference this crate refuses; this history therefore
remains explicitly partial.
https://www.sec.gov/rules/sro/nyseamex/2010/34-61890.pdf
https://www.federalregister.gov/documents/2017/02/15/2017-02990/self-regulatory-organizations-nyse-mkt-llc-notice-of-filing-of-proposed-rule-change-to-adopt-new
https://www.federalregister.gov/documents/2018/03/29/2018-06339/self-regulatory-organizations-new-york-stock-exchange-llc-notice-of-filing-of-amendment-no-1-and
https://www.federalregister.gov/documents/2024/02/29/2024-04168/self-regulatory-organizations-new-york-stock-exchange-llc-notice-of-filing-and-immediate
https://www.nyse.com/publicdocs/nyse/markets/nyse-american/Pillar_Update_NYSE_American_March_2017.pdf
https://www.nyse.com/publicdocs/nyse/markets/nyse-american/Pillar_Update_NYSE_American_Weekend_Test_Update_July21_2017.pdf

NSX's operative 2010 filing dates the 18:30→20:00 close extension to
2010-08-02. Its immediately operative 2014 filing shortened that close to
17:00 on 2014-05-16. Trading ceased after the 2014-05-30 close. The SEC's
2015 approval says the resumed marketplace would use the rules then in
effect, and the exchange's SEC-filed Form 1 dates its phased relaunch to
2015-12-22. It ceased again before the 2017-02-01 open. NYSE National
launched on Pillar on 2018-05-21 with its current 06:30 order-acceptance edge
around the 07:00–20:00 execution grid; the acceptance edge is `order_entry`
because no trade prints before the 07:00 Early Trading Session.

The pre-2018 NSX grid is a **rulebook** provision too: NSX Rule 11.1 ("Hours
of Trading") fixed the sessions, and the companion 2010 filing states the
floor-era table outright — "the Exchange's Regular Trading Hours ... are from
9:30 a.m. until 4 p.m. Eastern Time. The pre-Regular Trading Hours trading
session is from 8 a.m. until 9:30 a.m. ET, and the post-Regular Trading Hours
trading session is from 4 p.m. until 6:30 p.m. ET." That is exactly the
baseline modelled below, so it is carried back to the January-2010 floor.
No reviewed primary source describes an NSX order-acceptance phase distinct
from those three trading sessions, so none is modelled; because Rule 11.1(a)
also let the Board set business hours by Regulatory Circular, and NSX's own
circular archive is unreachable (nsx.com serves only index pages through the
web archive, with the circular PDFs themselves gone), a Board-noticed
acceptance edge could not be recovered even if one existed. The residual risk
is therefore under-reporting an NSX queue, never over-reporting one.
https://www.federalregister.gov/documents/2010/08/04/2010-19225/self-regulatory-organizations-national-stock-exchange-inc-notice-of-filing-and-immediate
https://www.federalregister.gov/documents/2010/08/10/2010-19652/self-regulatory-organizations-national-stock-exchange-inc-notice-of-filing-and-immediate
https://www.federalregister.gov/documents/2014/05/28/2014-12229/self-regulatory-organizations-national-stock-exchange-inc-notice-of-filing-and-immediate
https://www.federalregister.gov/documents/2018/03/13/2018-04962/self-regulatory-organizations-nyse-national-inc-notice-of-filing-of-proposed-rule-change-to-support
https://www.sec.gov/files/rules/sro/nsx/2010/34-62643.pdf
https://www.sec.gov/files/rules/sro/nsx/2014/34-72215.pdf
https://www.sec.gov/files/rules/sro/nsx/2014/34-72107.pdf
https://www.sec.gov/files/rules/sro/nsx/2015/34-76640.pdf
https://www.sec.gov/Archives/edgar/vprr/1601/16019238.pdf
https://www.sec.gov/files/rules/sro/nsx/2017/34-80018.pdf
https://www.nyse.com/publicdocs/nyse/markets/nyse-national/rule-filings/filings/2020/SR-NYSENat-2020-05.pdf

NYSE Texas is the same registered exchange formerly called NYSE Chicago and
CHX; its 2025-03-28 conversion and rename were non-substantive. At the audit
floor, CHX accepted orders from 07:00 through 17:00 ET: early trading to
09:30, core trading to 16:00, the late session to 16:15, and cross-only late
crossing through 17:00. Its Pillar migration established the current 06:30
order-acceptance edge around the 07:00–20:00 three-session grid on 2019-11-04.
That 06:30–07:00 edge is `order_entry`; the pre-Pillar CHX slice stays wholly
Extended because its 07:00 early session, 16:00–16:15 late session and
16:15–17:00 cross-only late crossing all print trades.
https://www.sec.gov/rules/sro/chx/2009/34-60775.pdf
https://www.sec.gov/files/rules/sro/nysechx/2019/34-86709.pdf
https://www.nyse.com/publicdocs/nyse/markets/nyse-chicago/NYSE_Chicago_Migration.pdf
https://www.sec.gov/files/rules/sro/nysechx/2019/34-87264.pdf
https://www.sec.gov/files/rules/sro/nysechx/2025/34-102507.pdf
https://www.nyse.com/trade/hours-calendars?os=.
