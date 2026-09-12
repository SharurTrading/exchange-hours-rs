<!-- SPDX-License-Identifier: MIT-0 -->

# `eurex_fixed_income` — evidence

- **Kind:** `MarketHoursKey`
- **Owner module:** [`eurex_fixed_income.rs`](../../src/calendar/schedules/futures/international/eurex_fixed_income.rs)
- **Source sets:** [`EU-EUREX`](../schedules/sources.md#eu-eurex)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

FGBL/FGBM/FGBS/FGBX fixed-income futures. Current continuous-trading and pre/post-trading phases sourced, with dated 2018-12-10 and 2019-02-25 revisions; the revision list was independently verified complete for 2010-01 through 2026-08.

## Revision rows

- 2018-12-10 — T1 — Eurex Circular 088/18 — Pre-Trading moves from 07:30-08:00 CET to 01:00-01:10 CET / 02:00-02:10 CEST and continuous trading becomes 01:10-22:00 CET / 02:10-22:00 CEST (both seasonal timelines carry this day).
- 2019-02-25 — T1 — Eurex CS amendment 2019-02-25 — Post-Trading Period Until shortened from 22:30 to 22:10 for FGBL, FGBM, FGBS and FGBX; continuous trading untouched (both seasonal timelines carry this day).

## Sources

Retrieval dates: these sources were last opened on the row's reviewed-on date
(2026-08-23, UTC); per-source retrieval dates were not recorded before the
2026-09-12 migration and are added as each source is re-verified.

- <https://www.eurex.com/resource/blob/2824010/044ff047cefd531f61ffe58d55404ca3/data/2026_08_17_eurex_d_kontraktspezifikationen_annexe_en.pdf> — Eurex Contract Specifications, Annex C as of 17.08.2026 — "01:10-22:00 MEZ / CET" and "02:10-22:00 MESZ / CEST", Post-Trading Period Until 22:10.
- <https://www.eurex.com/resource/blob/4873184/0ca7669a8cb9a2f917d99a801fb3f2de/data/tradingcalendar_2026_en.pdf> — Eurex trading calendar 2026 — 01:10-22:00 with footnote 8 "02:10-22:00 CEST".
- <https://www.eurex.com/ex-en/markets/int/long-term-interest-rates/fix/government-bonds/Euro-Bund-Futures-137298> — Eurex Euro-Bund Futures product page.
- <https://www.eurex.com/ex-en/trade/trading-hours/trading-phases> — Eurex trading-phases page — defines Pre-Trading and Post-Trading as non-executable order-entry phases.
- <https://www.eurex.com/resource/blob/1412768/e61a2c41d65ad165af7909002223b943/data/er18088e.pdf> — Eurex Circular 088/18, 15 November 2018, "Extension of trading hours for selected benchmark futures and MSCI futures" — states the phase it replaced.
- <https://www.eurex.com/resource/blob/1493194/bee8965900f0124d7ff7c7993d5f969b/data/2019_02_25_cs_4_history.pdf> — Eurex Contract Specifications amendment of 2019-02-25, "Shortening of post-trading phase for products traded until 22:00 CET".
- <https://www.eurex.com/ex-en/rules-regs/eurex-rules-regulations/03.-Contract-Specifications-4347288> — Eurex Contract Specifications index.

## Gaps and residual risks

- **horizon** — the pre-2018-12-10 baseline (Pre-Trading 07:30-08:00 CET, continuous trading 08:00-22:00) is known from Circular 088/18's own statement of the phase it replaced, so it is sourced from the circular's date, 2018-11-15, and carried below that to the January-2010 floor. No Eurex document dates an earlier change inside the modelled window.
- **order-entry** — the 22:30 post-trading end is carried back into the baseline rather than dated: the earliest primary statement of that value inside the modelled window is the February 2019 amendment recording the change away from it, and no Eurex document dates an earlier change to it.
- **excluded by design** — the Eurex T7 Entry Service (off-book TES, 01:15-22:00 CET / 02:15-22:00 CEST) is bilateral block, EFP and vola business rather than the central order book; clearing hours are not a trading phase; and per-contract last-trading-day hours (continuous trading ending 12:30) are an exceptional-day matter outside this normal-week model.

## Module narrative (moved from src/calendar/schedules/futures/international/eurex_fixed_income.rs on 2026-09-12 UTC)

Eurex publishes a phase machine, not a single open/close pair: Pre-Trading,
Opening auction, Continuous Trading, Closing auction, Post-Trading. The
executable on-exchange order book is the Continuous Trading phase, so that
phase alone is `regular`; Pre-Trading and Post-Trading accept order entry and
maintenance without matching, so they are `order_entry`.

Classification note: no trade can print in Pre-Trading or Post-Trading. The
trading-phases page defines them as the phases in which orders and quotes may
be entered, modified and deleted while the order book is not executable, and
the two auctions that do print sit inside the Continuous Trading row modelled
as `regular` here. Both phases are therefore `order_entry`, not `extended`,
and the `extended` slice for this family is empty: this grid has no tradeable
phase outside continuous trading.

The morning phases are anchored to 08:00 Singapore time, not to a fixed
Berlin wall clock: Annex C states Continuous Trading as "01:10-22:00 MEZ /
CET" and "02:10-22:00 MESZ / CEST", and the 2026 trading calendar prints
01:10-22:00 with footnote 8 reading "02:10-22:00 CEST". The close stays at
22:00 local in both seasons. A single fixed-local-wall-clock profile would
therefore be wrong for roughly half of every year, so the seasons are two
profiles selected by the venue's own UTC offset, matching how the sibling
Eurex benchmark-index module in `europe.rs` models the same Asian-hours
slice. `_CURRENT` is the CEST grid there, and is the CEST grid here too.

Excluded deliberately: the Eurex T7 Entry Service (off-book TES, 01:15-22:00
CET / 02:15-22:00 CEST) is bilateral block/EFP/vola business rather than the
central order book, and clearing hours are not a trading phase at all.
Per-contract last-trading-day hours (continuous trading ending 12:30) are an
exceptional-day matter and are outside this normal-week model.

https://www.eurex.com/resource/blob/2824010/044ff047cefd531f61ffe58d55404ca3/data/2026_08_17_eurex_d_kontraktspezifikationen_annexe_en.pdf
https://www.eurex.com/resource/blob/4873184/0ca7669a8cb9a2f917d99a801fb3f2de/data/tradingcalendar_2026_en.pdf
https://www.eurex.com/ex-en/markets/int/long-term-interest-rates/fix/government-bonds/Euro-Bund-Futures-137298
https://www.eurex.com/ex-en/trade/trading-hours/trading-phases

---

2018-12-10 through 2019-02-24: the executable session is already the 2026-08-23 review's
Asian-hours grid, but the post-trading phase still ran to 22:30 rather than
22:10. The 22:30 value is the one the February 2019 Contract Specifications
amendment records as the state it replaced. That whole window sits inside
CET, so the CEST twin below is never selected in practice; it is kept so the
regime is described by its dates rather than by an accident of the calendar.
Both windows are Pre-Trading and Post-Trading, so both are order entry only.

---

Baseline before 2018-12-10. Circular 088/18 states the phase it replaced:
Pre-Trading ran 07:30-08:00 CET, so continuous trading began at 08:00 and ran
to the unchanged 22:00 close - one same-day grid with no seasonal split,
because nothing in it was anchored to an Asian clock.

The 22:30 post-trading end is carried back into this baseline rather than
dated: the earliest primary statement of that value inside the modelled
window is the February 2019 amendment recording the change away from it, and
no Eurex document dates an earlier change to it. Carrying the value back is
preferred here over inventing a cutover.

https://www.eurex.com/resource/blob/1412768/e61a2c41d65ad165af7909002223b943/data/er18088e.pdf
https://www.eurex.com/ex-en/rules-regs/eurex-rules-regulations/03.-Contract-Specifications-4347288

---

2018-12-10: Eurex Circular 088/18, dated 15 November 2018, "Extension of
  trading hours for selected benchmark futures and MSCI futures": "With the
  introduction of extended trading hours planned as of 10 December 2018, ..."
  Pre-Trading moved from 07:30-08:00 CET to 01:00-01:10 CET / 02:00-02:10
  CEST and continuous trading became 01:10-22:00 CET / 02:10-22:00 CEST. This
  is the only change to the executable session across the modelled window.
  https://www.eurex.com/resource/blob/1412768/e61a2c41d65ad165af7909002223b943/data/er18088e.pdf
2019-02-25: Contract Specifications amendment, indexed by Eurex under the
  title "Shortening of post-trading phase for products traded until 22:00
  CET" and headed "Contract Specifications for Futures Contracts and Options
  Contracts at Eurex Deutschland". For FGBL, FGBM, FGBS and FGBX the
  Post-Trading Period Until changed from 22:30 to 22:10; continuous trading
  was untouched.
  https://www.eurex.com/resource/blob/1493194/bee8965900f0124d7ff7c7993d5f969b/data/2019_02_25_cs_4_history.pdf

Revision evidence — both seasonal tables carry the same two rows, and each
row's day-level effective date is stated by the primary source quoted in
full above:
  2018-12-10 "Eurex Circular 088/18"
    https://www.eurex.com/resource/blob/1412768/e61a2c41d65ad165af7909002223b943/data/er18088e.pdf
  2019-02-25 "Eurex CS amendment 2019-02-25"
    https://www.eurex.com/resource/blob/1493194/bee8965900f0124d7ff7c7993d5f969b/data/2019_02_25_cs_4_history.pdf

This is the CEST (summer) timeline; `EUREX_FIXED_INCOME_WINTER_REVISIONS`
carries the identical dates against the CET grid.
Evidence: docs/evidence/eurex_fixed_income.md
