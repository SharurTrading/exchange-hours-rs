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

## Holidays

**Coverage:** 2025-01-01..2027-12-31 (inclusive venue-local trade dates in `Europe/Berlin`; tier T1 throughout).

The rows key on Deutsche Börse's own cash-market trading calendar: the `Non-trading days at Frankfurter Wertpapierbörse (FWB®) ... (Xetra and Börse Frankfurt)` table of the operator's `Trading calendar and trading hours` page and the per-year `Trading calendar` PDFs that restate each year's set in one sentence. The 2025-04-22 capture of the page (`DB-TC-PAGE-2025`) and the `Trading calendar 2025` PDF it links (`DB-TC-PDF-2025`) govern 2025; the live page (`DB-TC-PAGE`) and its `Trading calendar 2026` PDF (`DB-TC-PDF-2026`) govern 2026 and 2027.

Two readings are the operator's own, not the crate's. First, Christmas Eve and New Year's Eve are **closures**: the `**` footnote reads "No trading but settlement is open", which states that the market does not trade. Second, the live page names 2026's trading holidays — `Ascension Day (14 May 2026)`, `Whit Monday (25 May 2026)`, `Corpus Christi (4 June 2026)` — and states `Trading of shares and Exchange traded products on Frankfurt and Xetra ends on public holidays (Germany and State of Hesse) where trading takes place according to the FWB trading calendar at 20:00 CET`, so those three dates carry an early close at 20:00 with the printed instant. The 2025 capture words the same note over `Börse Frankfurt` only, so no 2025 Xetra early close is sourced and none ships; the 2025 trading holidays (Ascension 29 May, Whit Monday 9 June, Corpus Christi 19 June, German Unity Day 3 October) ship closures-free as ordinary days for this identity. The live page's conditional note `On December 30, 2026, deviating trading hours may apply` — echoed by the 2026 PDF's `*)` footnote `Trading hours may differ from normal trading days` — keys no row (LAW-NO-FABRICATED-DATES).

### 2025

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2025-01-01 | closed | `New Year's Day — Wednesday 01 Jan 2025` | `DB-TC-PDF-2025` | T1 | the PDF's sentence: trading in 2025 "with the exception of 1 January, 18 April, 21 April, 1 May, 24, 25, 26 and 31 December"; the page table corroborates |
| 2025-04-18 | closed | `Good Friday — Friday 18 Apr 2025` | `DB-TC-PDF-2025` | T1 | same sentence; the page table corroborates |
| 2025-04-21 | closed | `Easter Monday — Monday 21 Apr 2025` | `DB-TC-PDF-2025` | T1 | same sentence; the page table corroborates |
| 2025-05-01 | closed | `Labour Day — Thursday 01 May 2025` | `DB-TC-PDF-2025` | T1 | same sentence; the page table corroborates |
| 2025-12-24 | closed | `Christmas Eve** — Wednesday 24 Dec 2025` — `**` = "No trading but settlement is open" | `DB-TC-PDF-2025` | T1 | same sentence; the page's footnote states the day is not a trading day |
| 2025-12-25 | closed | `Christmas Day — Thursday 25 Dec 2025` | `DB-TC-PDF-2025` | T1 | same sentence; the page table corroborates |
| 2025-12-26 | closed | `Boxing Day — Friday 26 Dec 2025` | `DB-TC-PDF-2025` | T1 | same sentence; the page table corroborates |
| 2025-12-31 | closed | `New Year's Eve** — Wednesday 31 Dec 2025` — `**` = "No trading but settlement is open" | `DB-TC-PDF-2025` | T1 | same sentence; the page's footnote states the day is not a trading day |

### 2026

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2026-01-01 | closed | `New Year's Day — Thursday Jan 01, 2026`; the PDF: trading "with the exception of 1 January, 3 April, 6 April, 1 May, 24 December, 25 December and 31 December" | `DB-TC-PDF-2026` | T1 | the PDF's own closure sentence; the live page's 2026 column corroborates |
| 2026-04-03 | closed | `Good Friday — Friday Apr 03, 2026` | `DB-TC-PDF-2026` | T1 | the PDF's closure sentence |
| 2026-04-06 | closed | `Easter Monday — Monday Apr 06, 2026` | `DB-TC-PDF-2026` | T1 | the PDF's closure sentence |
| 2026-05-01 | closed | `Labor Day — Friday May 01, 2026` | `DB-TC-PDF-2026` | T1 | the PDF's closure sentence |
| 2026-05-14 | early close | `Ascension Day (14 May 2026)` trades, and `Trading of shares and Exchange traded products on Frankfurt and Xetra ends ... at 20:00 CET` | `DB-TC-PAGE` | T1 | the page names 2026-05-14 as a trading holiday and states the 20:00 shares close in the same note |
| 2026-05-25 | early close | `Whit Monday (25 May 2026)` trades; shares end at 20:00 CET | `DB-TC-PAGE` | T1 | same note, same rule |
| 2026-06-04 | early close | `Corpus Christi (4 June 2026)` trades; shares end at 20:00 CET | `DB-TC-PAGE` | T1 | same note, same rule |
| 2026-12-24 | closed | `Christmas Eve** — Thursday Dec 24, 2026` — `**` = "No trading but settlement is open" | `DB-TC-PDF-2026` | T1 | the PDF's closure sentence; the page's footnote states the day is not a trading day |
| 2026-12-25 | closed | `Christmas Day — Friday Dec 25, 2026` | `DB-TC-PDF-2026` | T1 | the PDF's closure sentence |
| 2026-12-31 | closed | `New Year's Eve** — Thursday Dec 31, 2026` — `**` = "No trading but settlement is open" | `DB-TC-PDF-2026` | T1 | the PDF's closure sentence; the page's footnote states the day is not a trading day |

### 2027

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2027-01-01 | closed | `New Year's Day — Friday Jan 01, 2027` | `DB-TC-PAGE` | T1 | the live page's 2027 column of the non-trading-days table |
| 2027-03-26 | closed | `Good Friday — Friday Mar 26, 2027` | `DB-TC-PAGE` | T1 | the live page's 2027 column |
| 2027-03-29 | closed | `Easter Monday — Monday Mar 29, 2027` | `DB-TC-PAGE` | T1 | the live page's 2027 column; Labour Day 2027 falls on Saturday and names no weekday closure |
| 2027-12-24 | closed | `Christmas Eve** — Friday Dec 24, 2027` — `**` = "No trading but settlement is open" | `DB-TC-PAGE` | T1 | the live page's 2027 column; Christmas Day (Saturday) and Boxing Day (Sunday) 2027 name no weekday closure |
| 2027-12-31 | closed | `New Year's Eve** — Friday Dec 31, 2027` — `**` = "No trading but settlement is open" | `DB-TC-PAGE` | T1 | the live page's 2027 column |

**Gaps.** The 2025-04-22 page capture words its 20:00 holiday-close note over `Börse Frankfurt` only, so the 2025 trading holidays carry no Xetra early close here; the 2027 trading holidays (Ascension, Whit Monday, Corpus Christi) are named by no retrieved artifact — the page's named list is scoped to `the year 2026` — so 2027-05-06, 2027-05-17 and 2027-05-27 ship as ordinary days and may in fact end at 20:00. **Closing condition for both:** the operator's next per-year page edition or `Trading calendar` PDF that names Xetra's own hours on those days; the live page is re-checked monthly per LAW-WATCH and its 2027 edition is the expected closer. Tracked as #200. Nothing else in 2025-2027 is unresolved: every closure the operator prints ships, and every other trade date in the window is audited normal.

### Documents

Every artifact was retrieved on 2026-09-28 UTC and saved under `holidays/raw/equities/xetra/2025-2027/` in the research store, whose `INDEX.md` carries the same digests.

| Document | Window | Replay or service URL | Capture or retrieval, UTC | Tier | sha256 |
|---|---|---|---|---|---|
| `DB-TC-PAGE-2025` | 2025-01-01 .. 2027-12-31 | <https://web.archive.org/web/20250422181518id_/https://www.xetra.com/xetra-en/trading/trading-calendar-and-trading-hours> (capture `20250422181518`) | retrieved 2026-09-28 UTC | T1 | `44b6b2783375a17ac736ebc1c0247e0bae011f1a64c9ae125c602fea9ab41918` |
| `DB-TC-PDF-2025` | 2025-01-01 .. 2025-12-31 | <https://www.xetra.com/resource/blob/4064968/4079a2d5a9fec324905942b807b398ed/data/xetra-trading-calendar-2025.pdf> (2025 archive replay of the PDF linked from `DB-TC-PAGE-2025`) | retrieved 2026-09-28 UTC | T1 | `84c71bed702dd753f4272939f9c65d87ebd9afdfc89ff7917d545b66c3f8a8e5` |
| `DB-TC-PDF-2026` | 2026-01-01 .. 2026-12-31 | <https://www.cashmarket.deutsche-boerse.com/resource/blob/4481276/1b643791fcb4d60bdd7f25efad3f4626/data/deutsche-boerse-trading-calendar-2026.pdf> ("Trading calendar 2026") | retrieved 2026-09-28 01:28 UTC | T1 | `1edfc7b737ae1f5fe93179bfa9af59223b3c9d11cc8deddd127b386fb87e44b6` |
| `DB-TC-PAGE` | 2025-01-01 .. 2027-12-31 | <https://www.cashmarket.deutsche-boerse.com/cash-en/trading/trading-calendar-and-trading-hours> | retrieved 2026-09-28 01:28 UTC | T1 | `d70a8f5d54cb529103bf674ea8382702a3510800dac8b48965a43aa08ae4bca4` |

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
- **Served identity, 2026-09-28 UTC.** The consumer's market clock routes its `XETRA` and `XETRA_CENTRE` sets to this venue, so the row is **served** and reviewed monthly per LAW-WATCH; follow-ups are tracked as issues (#200) (LAW-SERVICE-TIERS, LAW-FOLLOW-UPS-ARE-ISSUES).
