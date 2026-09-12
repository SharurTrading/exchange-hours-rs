<!-- SPDX-License-Identifier: MIT-0 -->

# `borsa_istanbul` — evidence

- **Kind:** `Exchange`
- **Owner module:** [`bist.rs`](../../src/calendar/schedules/equities/europe/bist.rs)
- **Source sets:** [`EU-BIST`](../schedules/sources.md#eu-bist)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

Equity Market; all modeled post-2010 changes have dated official evidence.

## Revision rows

- 2012-03-02 — T1 — Borsa Istanbul closing_session — closing auction added: the afternoon continuous session ends at 17:17 and a 17:17–17:30 closing envelope follows.
- 2012-07-16 — T1 — Borsa Istanbul Genelge gn2012394 — afternoon session extended: continuous to 17:30 with a 17:30–17:40 closing envelope.
- 2013-04-05 — T1 — Borsa Istanbul Genelge gn2013421 — morning opening session moves to 09:15–09:45.
- 2013-06-10 — T1 — Borsa Istanbul Genelge gn2013430 — morning opening session shortens to 09:15–09:35.
- 2015-11-30 — T1 — Borsa Istanbul announcement 13472 — midday single-price call introduced at 12:30–13:30 and the afternoon continuous session starts at 13:30.
- 2016-03-28 — T1 — Borsa Istanbul announcement 13446 — midday call moves to 13:00–14:00, morning continuous trading runs to 13:00 and the afternoon session to 17:30.
- 2016-11-14 — T1 — Borsa Istanbul announcement 13376 — extended day: order collection 09:40–09:55, opening print to 10:00, continuous 10:00–13:00 and 14:00–18:00, closing envelope 18:00–18:10.
- 2019-10-04 — T1 — Borsa Istanbul duyuru 2019/56 — the midday single-price section is removed, leaving one continuous 10:00–18:00 session.

## Sources

Row review: 2026-08-22 (UTC) is the date the ledger row was last reviewed as a
whole. Per-source retrieval dates were not recorded before the 2026-09-12
migration; where a later targeted review, capture or document date is recorded
beside a source below, that date governs for that source, and dates are added
as each source is re-verified.

- <https://www.borsaistanbul.com/files/equity-market-procedure.pdf> — Borsa İstanbul Equity Market Procedure, the current rulebook. Its call-auction rules state that "[n]o transactions are executed during the order collection period", and its session table splits the 09:40–10:00 opening auction into an Order Collection Process (09:40–09:55) and Determination of Opening Price (09:55 onward).
- <https://www.borsaistanbul.com/datum/closing_session.pdf> — Borsa İstanbul closing-session document, the 2012-03-02 revision's source.
- <https://www.borsaistanbul.com/data/Genelge/gn2012394.pdf> — Borsa İstanbul Genelge gn2012394, the 2012-07-16 afternoon extension.
- <https://www.borsaistanbul.com/data/Genelge/gn2013421.pdf> — Borsa İstanbul Genelge gn2013421, the 2013-04-05 opening change.
- <https://www.borsaistanbul.com/data/Genelge/gn2013430.pdf> — Borsa İstanbul Genelge gn2013430, the 2013-06-10 opening change.
- <https://www.borsaistanbul.com/en/announcement/13472/single-session-era-borsa-istanbul> — Borsa İstanbul announcement 13472, the 2015-11-30 midday call.
- <https://www.borsaistanbul.com/en/announcement/13446/new-arrangement-borsa-istanbul-equity-market-midday-session> — Borsa İstanbul announcement 13446, the 2016-03-28 midday-session rearrangement.
- <https://www.borsaistanbul.com/en/announcement/13376/borsa-istanbul-trading-session-hours-change> — Borsa İstanbul announcement 13376, the 2016-11-14 extended day. It states that "the trading session shall start at 09:40 with order collection" and that "[f]ollowing the end of the order collection phase at 09:55, continuous auction shall start at 10:00".
- <https://www.borsaistanbul.com/duyuru/11640/pay-piyasasi-seansinda-gun-ortasi-tek-fiyat-bolumu-hk-201956-sayili-duyuru> — Borsa İstanbul duyuru 2019/56, removing the midday single-price section on 2019-10-04.

## Gaps and residual risks

- **Horizon carried below the first dated row.** The baseline profile below 2012-03-02 — morning opening call 09:30–09:50 and continuous 09:50–12:30, afternoon call 14:00–14:20 and continuous 14:20–17:30 — cites no artifact of its own. Every source indexed here states a change and its replacement table; none is identified as the document that attests the pre-2012 grid, and no retrieval date is recorded for the closing-session document, so its own publication day cannot be read off the citation. The ledger horizon is therefore 2012-03-02, the first day at which this row's state is sourced, with everything below it carried. Closing condition: retrieve a Borsa İstanbul circular, procedure edition or announcement dated at or before the January-2010 floor that prints the pre-2012 grid, or record the closing-session document's publication date so the baseline can be carried from it; either would move the horizon earlier.
- **Interpretive step, order-entry classification.** From 2016-11-14 only the 09:40–09:55 Order Collection Process is `order_entry`; 09:55–10:00 carries the opening print and stays `extended`. The midday single-price call and the 18:00–18:10 closing envelope each bundle collection with a price-determination leg that prints, so both stay `extended` whole. Earlier eras carry no `order_entry` window because no source separates their collection legs.
- **Source set has no monitoring feed.** `EU-BIST` records that no stable consolidated announcements-feed URL is indexed; review means reopening the Equity Market Procedure and the individual circulars listed above.
- **Dormant identity.** Reviewed on demand; gaps are recorded here rather than as issues.
