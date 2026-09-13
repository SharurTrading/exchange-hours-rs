<!-- SPDX-License-Identifier: MIT-0 -->

# `eurex` — evidence

- **Kind:** `Exchange`
- **Owner module:** [`europe.rs`](../../src/calendar/schedules/futures/international/europe.rs)
- **Source sets:** [`EU-EUREX`](../schedules/sources.md#eu-eurex)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

FESX/FDAX/FDXM benchmark-index futures: the January-2010 baseline has 07:30–07:50 pre-trading and 07:50–22:00 continuous trading. From the sourced 2018-12-10 cutover, pre-trading/opening auction runs 01:00–01:15 CET or 02:00–02:15 CEST before continuous trading to 22:00.

## Revision rows

None. `europe.rs` carries no `revisions!` block for this identity; its 2018-12-10 Asian-hours cutover and the seasonal CET/CEST choice are encoded as comparisons inside `eurex_profile_at` instead.

The 2018-12-10 Asian-hours cutover and the seasonal CET/CEST selection are both
in `eurex_profile_at` rather than in a `revisions!` block: the cutover is a
date comparison against `EUREX_ASIAN_HOURS`, and the seasonal choice is a
UTC-offset comparison, because the Asian-hours open is a fixed 00:00 UTC instant
rather than a fixed local time. Eurex circular 088/2018 states the cutover day
unconditionally at T1 and its redline preserves the predecessor grid, so the
date is sourced even though it is not carried as a tuple.

## Dated selectors

Day-level boundaries this identity's `profile_at` selects on directly, outside
any `revisions!` block. They are invisible to the module-declaration fences, so
they are recorded here in revision-row grammar and checked against
`HISTORICAL_CUTOVERS` / `HISTORICAL_INSTANT_CUTOVERS` in
`tests/contract/session_invariants/historical_expectations.rs`.

- 2018-12-10 — T1 — Eurex circular 088/2018 (`EUREX_ASIAN_HOURS`) — the Asian-hours open, selected by a date comparison in `eurex_profile_at`; the seasonal CET/CEST table is a UTC-offset comparison beside it and asserts no day.

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

Row review: 2026-08-22 (UTC) is the date the ledger row was last reviewed as a
whole. Per-source retrieval dates were not recorded before the 2026-09-12
migration; where a later targeted review, capture or document date is recorded
beside a source below, that date governs for that source, and dates are added
as each source is re-verified.

The documents below stand behind the row and behind the narrative moved below.

- <https://www.eurex.com/resource/blob/296888/978b08fe3a240a0b4a8fb62a2647a197/data/cs_history_26102009_en.pdf.pdf> — the official 2009 archived contract specifications, the edition that pins the grid in force at the January-2010 floor — T1.
- <https://www.eurex.com/resource/blob/337416/2598be26dcb9b5521549169d5e8b9e8e/data/2013_09_25_cs_history_en.pdf.pdf> — the official 2013 archived contract specifications — T1.
- <https://www.eurex.com/resource/blob/298554/d1fb9a8ac7a259104ea157830860e080/data/2015_10_28_cs_1_history_en.pdf> — the official 2015 archived contract specifications — T1.
- <https://www.eurex.com/resource/blob/317412/ff50dcdf5143258c382b4f682cbaf37b/data/2017_08_01_cs_1_history_en.pdf> — the official 2017 archived contract specifications — T1.
- <https://www.eurex.com/resource/blob/1412768/e61a2c41d65ad165af7909002223b943/data/er18088e.pdf> — Eurex circular 088/2018 and its Annex C, which make the extension effective 2018-12-10 and whose redline preserves the predecessor grid — T1.
- <https://www.eurex.com/resource/blob/1448250/29a4179e4d28742af5d0ee85f9af89f8/data/Eurex%20Asian%20Trading%20Hours_Nov%202018.pdf> — the Asian-hours launch phase diagram, distinguishing 10 minutes of pre-trading from the five-minute opening auction — T1.
- <https://www.eurex.com/resource/blob/2824010/3b94b95cdf5f31cc635294659a5e9786/data/2026_05_04_eurex_d_kontraktspezifikationen_annexe_en.pdf> — the current contract-specification Annex C, which retains the same product grid — T1.
- <https://www.eurex.com/ex-en/trade/trading-hours> — Eurex trading hours, the current-schedule monitoring entry point — T1.
- <https://www.eurex.com/ex-en/trade/trading-hours/trading-phases> — Eurex's trading-phases page, the source for the pre-trading classification — T1.
- <https://www.eurex.com/ex-en/find/circulars> — Eurex circulars, the watch channel — T1.

## Gaps and residual risks

- **No dated-history gap.** The 2009 archived specification predates the
  January-2010 floor, so the baseline grid is sourced through the floor rather
  than carried back to it, and the horizon is the floor itself.
- **Scope.** The `eurex` venue default is specifically the FESX/FDAX/FDXM
  benchmark-index futures, not a venue-wide clock. Eurex fixed-income futures
  are a separate identity with their own module and their own dated revisions;
  any other Eurex product family needs its own review.
- **Seasonal selection, not a revision.** After 2018-12-10 the profile is chosen
  by the venue's UTC offset, so a caller must supply an instant. A detached
  fixed snapshot carries whichever of the winter and summer tables it was built
  from and no seasonal behaviour of its own.
- **The pre-trading and opening-auction split is an interpretation, recorded
  here.** Eurex's trading-phases page defines pre-trading as the phase in which
  orders and quotes are entered, modified and deleted while the order book is
  not executable, so the first 10 minutes are `order_entry`; the opening auction
  that follows matches and prints at the auction price, so those five minutes
  stay `extended`. The pre-2018 grid names its whole 07:30–07:50 window
  pre-trading with the start of trading at 07:50, so all of it is `order_entry`.

## Module narrative (moved from src/calendar/schedules/futures/international/europe.rs on 2026-09-12 UTC)

The Eurex default is FESX/FDAX/FDXM benchmark index futures, not a
venue-wide clock. Official 2009, 2013, 2015, and 2017 archived specifications
pin the January-2010-to-cutover grid: 07:30-07:50 pre-trading followed by
07:50-22:00 continuous trading. Circular 088/2018 makes the extension
effective 2018-12-10 and its redline preserves that predecessor grid.
<https://www.eurex.com/resource/blob/296888/978b08fe3a240a0b4a8fb62a2647a197/data/cs_history_26102009_en.pdf.pdf>
<https://www.eurex.com/resource/blob/337416/2598be26dcb9b5521549169d5e8b9e8e/data/2013_09_25_cs_history_en.pdf.pdf>
<https://www.eurex.com/resource/blob/298554/d1fb9a8ac7a259104ea157830860e080/data/2015_10_28_cs_1_history_en.pdf>
<https://www.eurex.com/resource/blob/317412/ff50dcdf5143258c382b4f682cbaf37b/data/2017_08_01_cs_1_history_en.pdf>
<https://www.eurex.com/resource/blob/1412768/e61a2c41d65ad165af7909002223b943/data/er18088e.pdf>

The launch phase diagram distinguishes 10 minutes of pre-trading and a
five-minute opening auction before continuous trading starts at 01:15 CET /
02:15 CEST. The following continuous interval is regular. The seasonal open
remains a fixed 00:00 UTC instant. Current Annex C retains the same product
grid.
<https://www.eurex.com/resource/blob/1448250/29a4179e4d28742af5d0ee85f9af89f8/data/Eurex%20Asian%20Trading%20Hours_Nov%202018.pdf>
<https://www.eurex.com/resource/blob/2824010/3b94b95cdf5f31cc635294659a5e9786/data/2026_05_04_eurex_d_kontraktspezifikationen_annexe_en.pdf>
<https://www.eurex.com/ex-en/trade/trading-hours>
<https://www.eurex.com/ex-en/trade/trading-hours/trading-phases>

Classification note: the two non-continuous phases are not the same kind of
window. Pre-trading is order entry only - Eurex's trading-phases page defines
it as the phase in which orders and quotes are entered, modified and deleted
while the order book is not executable - so the first 10 minutes are
`order_entry`. The opening auction that follows it matches and prints at the
auction price, so those five minutes stay `extended`. The pre-2018 grid names
its whole 07:30-07:50 window pre-trading, with the start of trading at 07:50,
so all of it is `order_entry`.

07:30-07:50 pre-trading: order entry only, no matching.

01:00-01:10 CET pre-trading: order entry only.

01:10-01:15 CET opening auction: a trade prints at the auction price, so this
window is tradeable and stays `extended`.

02:00-02:10 CEST pre-trading: order entry only.

02:10-02:15 CEST opening auction: a trade prints at the auction price, so this
window is tradeable and stays `extended`.

EEX does not have one venue-wide grid. This default is Nordic Zonal Power
Futures. The official customer information gives both their 2024-03-25
launch and the 08:00-18:00 CE(S)T trading table. The current derivatives
timetable retains that grid, while the Trading Conditions make product
hours controlling and define exchange days as Monday-Friday.
<https://www.eex.com/fileadmin/Global/News/EEX/EEX_Customer_Information/2024/20240109_EEX_Customer_Information_Nordic_Zonal_Futures.pdf>
<https://www.eex.com/fileadmin/EEX/Downloads/Trading/Trading_Hours/20250701_Trading_Hours_on_EEX_Derivatives_Markets_.pdf>
<https://www.eex.com/fileadmin/EEX/Downloads/Rules/Trading_Conditions/20260513_EEX_Trading_Conditions_0073a_E_FINAL.pdf>
