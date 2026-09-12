<!-- SPDX-License-Identifier: MIT-0 -->

# `xetra` — evidence

- **Kind:** `Exchange`
- **Owner module:** [`xetra.rs`](../../src/calendar/schedules/equities/europe/xetra.rs)
- **Source sets:** [`EU-XETRA`](../schedules/sources.md#eu-xetra), [`EU-FESE-SECONDARY`](../schedules/sources.md#eu-fese-secondary)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

DAX constituent-share envelope with its January-2010 intraday auction, 2020-11-24 Trade-at-Close launch, and participant-restricted Extended Retail from 2025-12-01.

## Revision rows

- 2020-11-24 — T1 — Deutsche Börse Trade-at-Close press release — executable Trade-at-Close inserted after the DAX closing auction through 17:45, post-trading pushed back to 17:45.
- 2025-12-01 — T1 — Deutsche Börse Extended Xetra Retail circular — envelope opens at 07:00, Trade-at-Close ends 17:40, participant-restricted late retail through 22:00, post-trading to 22:05.

## Sources

Row review: 2026-08-22 (UTC) is the date the ledger row was last reviewed as a
whole. Per-source retrieval dates were not recorded before the 2026-09-12
migration; where a later targeted review, capture or document date is recorded
beside a source below, that date governs for that source, and dates are added
as each source is re-verified.

- <https://cashmarket.deutsche-boerse.com/resource/blob/197910/0890768f3f753299e4c268b80fe7944d/data/207_08e.pdf> — Deutsche Börse circular 207/08, the January-2010-era market model.
- <https://www.cashmarket.deutsche-boerse.com/resource/blob/1431340/a23cc3ff15d46a3b649bd23f1618b928/data/091_18e.pdf> — Deutsche Börse circular 091/18. With 207/08 it brackets the January-2010 baseline and confirms the DAX grid remained: pre-trading from 07:30, opening auction 08:50–09:00, intraday auction 13:00–13:02, continuous trading to 17:30, closing auction to 17:35, order-entry-only post-trading through 20:30.
- <https://www.cashmarket.deutsche-boerse.com/resource/blob/31802/6ab37d564c2934a20766824e4284d608/data/2026_07_07_fwb_boersenordnung_en.pdf> — FWB Exchange Rules: § 67 makes pre-trading and post-trading Trading Periods distinct from the periods in which prices are determined, and § 67(2) states that "[d]uring the pre-trading period, the order book shall remain closed"; § 123 confines trading to 08:30–17:30 plus the closing auction and the Trade-at-Close period; § 123(2b) permits Extended Xetra Retail Service trading from 08:00 to 09:00 and through 22:00.
- <https://www.cashmarket.deutsche-boerse.com/cash-en/Stay-Informed/circulars-newsletters/deutsche-boerse-circulars/Introduction-of-T7-Release-9.0-1978838> — Deutsche Börse circular, T7 Release 9.0 entered production 2020-11-23.
- <https://www.cashmarket.deutsche-boerse.com/cash-en/Stay-Informed/newsroom/press-releases/Xetra-Trade-at-Close-enables-trading-at-the-official-closing-price-2346762> — Deutsche Börse factsheet and release: Trade-at-Close itself launched 2020-11-24, one day after the release went to production.
- <https://www.cashmarket.deutsche-boerse.com/cash-en/Stay-Informed/circulars-newsletters/deutsche-boerse-circulars/Introduction-of-the-Extended-Xetra-Retail-Service-early-and-late-trading-Planned-changes-to-the-trading-process-valid-from-1-December-2025-4793480> — Deutsche Börse circular, Extended Xetra Retail Service effective 2025-12-01.
- <https://www.cashmarket.deutsche-boerse.com/resource/blob/250890/24d50260d22cd63e0f600ae2543ca529/data/trading-parameters-xetra.pdf> — Xetra trading-parameter sheet: marks pre-trading and post-trading "(Book)", quotes no price for them, and runs the Retail Pre-Call/Retail-Call from 08:00.
- <https://www.cashmarket.deutsche-boerse.com/cash-en/trading/trading-calendar-and-trading-hours> — Xetra calendar and hours, the source set's current entry point.
- <https://www.cashmarket.deutsche-boerse.com/cash-en/trading/Xetra/continuous-trading-with-auctions> — Xetra continuous trading with auctions.
- <https://www.cashmarket.deutsche-boerse.com/cash-en/Stay-Informed/rules-and-regulations-for-the-fwb> — FWB rules and regulations, the monitoring entry point.
- <https://www.fese.eu/app/uploads/2024/07/trading-hours-2025-1.pdf> — FESE 2025 trading-hours table, `EU-FESE-SECONDARY`: corroboration only.

## Gaps and residual risks

- **Scope.** The profile represents the liquid DAX constituent-share segment, not every Xetra instrument. Other segments have their own auction schedules and are out of scope.
- **Interpretive step, order-entry classification.** Pre-trading and post-trading are `order_entry` on § 67/§ 123 and on the parameter sheet's "(Book)" marking. Every auction call is `extended` whole because its price determination prints at the auction price.
- **Interpretive step, retail phases.** The 08:00–08:55 early retail and 17:40–22:00 late retail windows are participant-restricted, so they are `extended` rather than `regular`; only the unrestricted continuous phases stay `regular`. Each auction is modelled through its 30-second random end, so regular trading begins at the latest possible edge.
- **Dormant identity.** No consumer adapter admits a Xetra namespace, so this row is reviewed on demand and its follow-ups are recorded here rather than as issues (LAW-SERVICE-TIERS, LAW-FOLLOW-UPS-ARE-ISSUES).
