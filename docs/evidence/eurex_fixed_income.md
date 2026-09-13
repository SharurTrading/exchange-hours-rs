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

## Holidays

**Coverage:** 2026-01-01..2026-12-31 (inclusive trade dates). Tier: T1 throughout.

One table serves `Exchange::Eurex`, the `eurex` key and the `eurex_fixed_income` key. The operator states the closure for “all derivatives”, which covers FESX, FDAX and FDXM behind the index rows and FGBL, FGBM, FGBS and FGBX behind the fixed-income rows alike, so the venue intersection is the same table.

**Documents.**

- `EUREX-HOLREG-2026` — Eurex “Holiday regulations”, § 2026, day by day. <https://www.eurex.com/ex-en/trade/trading-calendar/holiday-regulations> (raw bytes retrieved 2026-09-12 04:19 UTC, sha256 `7b28acd2d2fb126c01461ef5a4ae11fe93e8b6f318001c6304bbce0fc78f3821`) — **T1**. Corroborated by the Trading Calendar 2026 PDF, p.2 “Overview of holidays by countries” <https://www.eurex.com/resource/blob/4873184/0ca7669a8cb9a2f917d99a801fb3f2de/data/tradingcalendar_2026_en.pdf> (retrieved 2026-09-12 04:18 UTC, sha256 `b0796b42819b38c0757d727d9b789360ba84cd0d45cea215544f86342158ac65`, PDF CreationDate 2026-07-02).

All bytes, with each artifact's URL, UTC retrieval time and sha256, are in the research store under `holidays/raw/cfe-eurex-ice-cde-smfe-2026-2027/INDEX.md` and `holidays/raw/cfe-eurex-ice-cde-smfe-2026-2027-fix/INDEX.md`; the normalised result is `holidays/cfe-eurex-ice-cde-smfe-2026-2027.json`, verified `matches: true` with zero discrepancies in its round-2 adversarial verdict.

### 2026

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2026-01-01 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives.` | `EUREX-HOLREG-2026` | T1 | Eurex event date 2026-01-01 (New Year's Day); one Berlin civil day per trade date, so the conversion is the identity |
| 2026-04-03 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives.` | `EUREX-HOLREG-2026` | T1 | Eurex event date 2026-04-03 (Good Friday) |
| 2026-04-06 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives.` | `EUREX-HOLREG-2026` | T1 | Eurex event date 2026-04-06 (Easter Monday) |
| 2026-05-01 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives.` | `EUREX-HOLREG-2026` | T1 | Eurex event date 2026-05-01 (Labour Day) |
| 2026-12-24 | closed | `Eurex is closed for trading in all derivatives: 24 December, 31 December` — a full trading closure; clearing stays open | `EUREX-HOLREG-2026` | T1 | Eurex event date 2026-12-24 (Christmas Eve) |
| 2026-12-25 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives.` | `EUREX-HOLREG-2026` | T1 | Eurex event date 2026-12-25 (Christmas Day) |
| 2026-12-31 | closed | `Eurex is closed for trading in all derivatives: 24 December, 31 December` — a full trading closure; clearing stays open | `EUREX-HOLREG-2026` | T1 | Eurex event date 2026-12-31 (New Year's Eve) |

**Gaps, 2026:** **Eurex 2027 does not ship.** Eurex's 2027-2036 trading calendars are published “on a preliminary and indicative basis … and are subject to change”, which is not the unconditional, day-level future LAW-NO-FABRICATED-DATES requires, so the five 2027 closures the indicative CSV carries (2027-01-01, 2027-03-26, 2027-03-29, 2027-12-24, 2027-12-31) are recorded here and encoded nowhere. Closed by the Eurex “Trading Calendar 2027” PDF and the 2027 section of the Holiday regulations page. **Additional German closures are unresolved.** The Trading Calendar 2026 PDF states that Eurex “is closed for trading and exercise in German equity and equity index derivatives as well as ETF and ETC derivatives which are based on Xetra® listings: to be announced”. DAX and Mini-DAX futures are German equity index derivatives, so that line could add closures beyond the seven above for FDAX and FDXM; the day-by-day Holiday regulations page lists no German-specific 2026 closure and German Unity Day 2026-10-03 falls on a Saturday, so the practical exposure is probably nil, but the operator has not said so. Closed by a Eurex announcement resolving that line.

**Interpretive steps, 2026:** None. Eurex states its closures as whole days for “all derivatives”, every phase the crate models runs inside one Berlin civil day, and Eurex publishes no early close — 24 and 31 December are full trading closures with clearing open, not half days. So each event date is its own trade date and each row is `Closed`.

## Sources

Row review: 2026-08-23 (UTC) is the date the ledger row was last reviewed as a
whole. Per-source retrieval dates were not recorded before the 2026-09-12
migration; where a later targeted review, capture or document date is recorded
beside a source below, that date governs for that source, and dates are added
as each source is re-verified.

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
