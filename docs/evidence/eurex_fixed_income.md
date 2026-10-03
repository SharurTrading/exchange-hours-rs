<!-- SPDX-License-Identifier: MIT-0 -->

# `eurex_fixed_income` — evidence

- **Kind:** `MarketHoursKey`
- **Owner module:** [`eurex_fixed_income.rs`](../../src/calendar/schedules/futures/international/eurex_fixed_income.rs)
- **Source sets:** [`EU-EUREX`](../schedules/sources.md#eu-eurex)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

FGBL/FGBM/FGBS/FGBX fixed-income futures. Current continuous-trading and pre/post-trading phases sourced, with dated 2018-12-10 and 2019-02-25 revisions; the revision list was independently verified complete for 2010-01 through 2026-08. On 2026-09-30 UTC the baseline became sourced through the January-2010 floor: Eurex's own archived Contract Specifications amendments print the same FGBL/FGBM/FGBS/FGBX trading-hours row from 2009-09-14 through 2017-08-28, so the horizon moved from the 2018-11-15 circular date to the floor and the pre-floor carry notes below were discharged.

## Revision rows

- 2018-12-10 — T1 — Eurex Circular 088/18 — Pre-Trading moves from 07:30-08:00 CET to 01:00-01:10 CET / 02:00-02:10 CEST and continuous trading becomes 01:10-22:00 CET / 02:10-22:00 CEST (both seasonal timelines carry this day).
- 2019-02-25 — T1 — Eurex CS amendment 2019-02-25 — Post-Trading Period Until shortened from 22:30 to 22:10 for FGBL, FGBM, FGBS and FGBX; continuous trading untouched (both seasonal timelines carry this day).

## The baseline era, 2010-01-01 .. 2018-12-09

The baseline grid (Pre-Trading 07:30-08:00 CET, Continuous Trading 08:00-22:00,
Post-Trading until 22:30) is stated by the operator's own Contract Specifications
amendments, each printing the `Annex C — Trading Hours Futures Contracts — Fixed
Income Futures Contracts` table row by row:

- **2009-09-14** — "Euro-BTP Futures: Introduction of Futures Contracts on
  long-term Italian Government Bonds" (`As of September 14, 2009`), page 3:
  `Euro-Schatz Futures FGBS 07:30-08:00 | 08:00-22:00 | 22:00-22:30*`,
  `Euro-Bobl Futures FGBM 07:30-08:00 | 08:00-22:00 | 22:00-22:30`,
  `Euro-Bund Futures FGBL 07:30-08:00 | 08:00-22:00 | 22:00-22:30`,
  `Euro-Buxl® Futures FGBX 07:30-08:00 | 08:00-22:00 | 22:00-22:30` (the asterisk
  marks the last-trading-day footnote; `All times in CET`). The amendment
  predates the January-2010 floor, so the grid is sourced through the floor
  rather than carried back to it — the same doctrine the sibling `eurex` row
  records for its 2009 index-futures edition.
- **2011-09-19** — "Mid-Term Euro-BTP Futures: Introduction" (`As of 19.09.2011`),
  Annex C: FGBL/FGBM/FGBS/FGBX rows unchanged.
- **2013-03-11** — "Mid-Term Euro-OAT-Futures: Introduction" (`As of 11.03.2013`),
  Annex C: FGBL/FGBM/FGBS/FGBX rows unchanged.
- **2017-08-28** — "Fixed Income Futures: Harmonisation of Post-Trading period on
  regular trading days" (`As of 28.08.2017`), Annex C: FGBL/FGBM/FGBX
  `07:30-08:00 | 08:00-22:00 | 22:30*` and FGBS `07:30-08:00 | 08:00-22:00 |
  22:30*` — the same exchange phases under the amendment's new column layout;
  the harmonisation redrew the off-book (TES) columns, not the Pre-Trading,
  Continuous Trading or Post-Trading Period Until values this family models.
  The two 2018-12-10 cutover rows follow.

No amendment between 2009-09-14 and the 2018-12-10 circular touches the four
products' exchange trading hours: the contract-specification index's amendment
archive lists, for subpart 1.2 (fixed-income futures) plus Annex C in that
window, only the product-introduction amendments quoted above and the 2017-08-28
harmonisation; the other fixed-income entries amend options (subpart 2.3) or
delivery baskets (Annex A). The `Extension of trading time` amendment of
2010-07-05 (`cs_history_05072010`) carries index-futures rows only (MSCI Russia,
OMXH25, SLI, SMI) and no fixed-income table.

**Documents.**

| Document | Window | URL | Capture or retrieval, UTC | Tier | sha256 |
|---|---|---|---|---|---|
| `EUREX-CS-2009-09-14` | baseline era | <https://www.eurex.com/resource/blob/328400/e12f28dfcc8ba1f7b94ea4da40b69c41/data/cs_history_14092009_en.pdf.pdf> | retrieved 2026-09-30T04:35Z | T1 | `c94ac7911b1e2b2835833f0c04d0989cdcd5e1a20f8f4cf9890d2b60c0a19f96` |
| `EUREX-CS-2011-09-19` | baseline era | <https://www.eurex.com/resource/blob/326452/9aa55ddb4fd3b0fb7fb15567a734ab5d/data/cs_history_19092011_en.pdf.pdf> | retrieved 2026-09-30T04:45Z | T1 | `eb52f6af0ccfbf36b145601d90d28e9c5d92b1ffdcf4d19e8fd3cdccd1b33162` |
| `EUREX-CS-2013-03-11` | baseline era | <https://www.eurex.com/resource/blob/334064/488cfe99b1526c777b789ade2950e4e5/data/2013_03_11_cs_history_1_en.pdf.pdf> | retrieved 2026-09-30T05:00Z | T1 | `f927a78d8cc975ebe0207b32e64a7762bb53fd0c5c9fd9c37fe7b3b3e0a1560c` |
| `EUREX-CS-2017-08-28` | baseline era | <https://www.eurex.com/resource/blob/295608/36bcde7eecdd7167995bf1143432e5c9/data/2017_08_28_cs_2_history_en.pdf> | retrieved 2026-09-30T04:40Z | T1 | `a7a42a3936b182476cfbe4d0218ef518826171044236089bf34baef3a6b5cbac` |

All bytes are in the research store under
`normal-weeks/wave-c1/eurex/` with this page as their index
(`normal-weeks/wave-c1/INDEX.md`).

## Holidays

**Coverage:** 2010-01-01..2026-12-31 (inclusive trade dates). Tier: T1 throughout.

The fixed-income key ships its own table (`FIXED_INCOME` in the `eurex` holiday module): the all-derivatives closures alone. The operator's German-scope closure notes — the lines that close German equity and equity-index derivatives and the Xetra-based ETF/ETC derivatives, dated in the 2014, 2016, 2017 and 2018 editions and `tba` since 2025 — never name fixed income: the panel's grammar prints fixed income explicitly when a closure reaches it, as the recurring Swiss line (`Eurex is closed for trading and clearing (exercise and settlement) in Swiss fixed income as well as equity and equity index derivatives`) does. So FGBL, FGBM, FGBS and FGBX keep trading on every German-scope date — 2014-10-03, 2016-05-16, 2016-10-03, 2017-06-05, 2017-10-03, 2017-10-31, 2018-05-21, 2018-10-03 answer as ordinary fixed-income sessions, and each eve's 22:00-22:30 CET post-trading leg still carries the following German-scope date as its trade date — and the `tba` era withholds nothing from this family. The benchmark identities' German-scope rows live in the sibling table (`TABLE`) that serves `Exchange::Eurex` and the `eurex` key; see [eurex](eurex.md).

**Documents.**

- `EUREX-HOLREG-2026` — Eurex “Holiday regulations”, § 2026, day by day. <https://www.eurex.com/ex-en/trade/trading-calendar/holiday-regulations> (raw bytes retrieved 2026-09-12 04:19 UTC, sha256 `7b28acd2d2fb126c01461ef5a4ae11fe93e8b6f318001c6304bbce0fc78f3821`) — **T1**. Corroborated by the Trading Calendar 2026 PDF, p.2 “Overview of holidays by countries” <https://www.eurex.com/resource/blob/4873184/0ca7669a8cb9a2f917d99a801fb3f2de/data/tradingcalendar_2026_en.pdf> (retrieved 2026-09-12 04:18 UTC, sha256 `b0796b42819b38c0757d727d9b789360ba84cd0d45cea215544f86342158ac65`, PDF CreationDate 2026-07-02).

All bytes, with each artifact's URL, UTC retrieval time and sha256, are in the research store under `holidays/raw/cfe-eurex-ice-cde-smfe-2026-2027/INDEX.md` and `holidays/raw/cfe-eurex-ice-cde-smfe-2026-2027-fix/INDEX.md`; the normalised result is `holidays/cfe-eurex-ice-cde-smfe-2026-2027.json`, verified `matches: true` with zero discrepancies in its round-2 adversarial verdict.

### 2010

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2010-01-01 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: January 1, April 2, April 5` | `EUREX-CAL-2010` | T1 | Eurex event date 2010-01-01 (New Year's Day); one Berlin civil day per trade date, so the conversion is the identity |
| 2010-04-02 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: January 1, April 2, April 5` | `EUREX-CAL-2010` | T1 | Eurex event date 2010-04-02 (Good Friday); one Berlin civil day per trade date, so the conversion is the identity |
| 2010-04-05 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: January 1, April 2, April 5` | `EUREX-CAL-2010` | T1 | Eurex event date 2010-04-05 (Easter Monday); one Berlin civil day per trade date, so the conversion is the identity |
| 2010-12-24 | closed | `Eurex is closed for trading in all derivatives: December 24, December 31` — a full trading closure; clearing stays open | `EUREX-CAL-2010` | T1 | Eurex event date 2010-12-24 (Christmas Eve); one Berlin civil day per trade date, so the conversion is the identity |
| 2010-12-31 | closed | `Eurex is closed for trading in all derivatives: December 24, December 31` — a full trading closure; clearing stays open | `EUREX-CAL-2010` | T1 | Eurex event date 2010-12-31 (New Year's Eve); one Berlin civil day per trade date, so the conversion is the identity |

### 2011

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2011-04-22 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: April 22, April 25, December 26` | `EUREX-CAL-2011` | T1 | Eurex event date 2011-04-22 (Good Friday); one Berlin civil day per trade date, so the conversion is the identity |
| 2011-04-25 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: April 22, April 25, December 26` | `EUREX-CAL-2011` | T1 | Eurex event date 2011-04-25 (Easter Monday); one Berlin civil day per trade date, so the conversion is the identity |
| 2011-12-26 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: April 22, April 25, December 26` | `EUREX-CAL-2011` | T1 | Eurex event date 2011-12-26 (Boxing Day); one Berlin civil day per trade date, so the conversion is the identity |

### 2012

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2012-04-06 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: April 6, April 9, May 1, December 25, December 26` | `EUREX-CAL-2012` | T1 | Eurex event date 2012-04-06 (Good Friday); one Berlin civil day per trade date, so the conversion is the identity |
| 2012-04-09 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: April 6, April 9, May 1, December 25, December 26` | `EUREX-CAL-2012` | T1 | Eurex event date 2012-04-09 (Easter Monday); one Berlin civil day per trade date, so the conversion is the identity |
| 2012-05-01 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: April 6, April 9, May 1, December 25, December 26` | `EUREX-CAL-2012` | T1 | Eurex event date 2012-05-01 (Labour Day); one Berlin civil day per trade date, so the conversion is the identity |
| 2012-12-24 | closed | `Eurex is closed for trading in all derivatives: December 24, December 31` — a full trading closure; clearing stays open | `EUREX-CAL-2012` | T1 | Eurex event date 2012-12-24 (Christmas Eve); one Berlin civil day per trade date, so the conversion is the identity |
| 2012-12-25 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: April 6, April 9, May 1, December 25, December 26` | `EUREX-CAL-2012` | T1 | Eurex event date 2012-12-25 (Christmas Day); one Berlin civil day per trade date, so the conversion is the identity |
| 2012-12-26 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: April 6, April 9, May 1, December 25, December 26` | `EUREX-CAL-2012` | T1 | Eurex event date 2012-12-26 (Boxing Day); one Berlin civil day per trade date, so the conversion is the identity |
| 2012-12-31 | closed | `Eurex is closed for trading in all derivatives: December 24, December 31` — a full trading closure; clearing stays open | `EUREX-CAL-2012` | T1 | Eurex event date 2012-12-31 (New Year's Eve); one Berlin civil day per trade date, so the conversion is the identity |

### 2013

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2013-01-01 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: January 1, March 29, April 1, May 1, December 25, December 26` | `EUREX-CAL-2013` | T1 | Eurex event date 2013-01-01 (New Year's Day); one Berlin civil day per trade date, so the conversion is the identity |
| 2013-03-29 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: January 1, March 29, April 1, May 1, December 25, December 26` | `EUREX-CAL-2013` | T1 | Eurex event date 2013-03-29 (Good Friday); one Berlin civil day per trade date, so the conversion is the identity |
| 2013-04-01 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: January 1, March 29, April 1, May 1, December 25, December 26` | `EUREX-CAL-2013` | T1 | Eurex event date 2013-04-01 (Easter Monday); one Berlin civil day per trade date, so the conversion is the identity |
| 2013-05-01 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: January 1, March 29, April 1, May 1, December 25, December 26` | `EUREX-CAL-2013` | T1 | Eurex event date 2013-05-01 (Labour Day); one Berlin civil day per trade date, so the conversion is the identity |
| 2013-12-24 | closed | `Eurex is closed for trading in all derivatives: December 24, December 31` — a full trading closure; clearing stays open | `EUREX-CAL-2013` | T1 | Eurex event date 2013-12-24 (Christmas Eve); one Berlin civil day per trade date, so the conversion is the identity |
| 2013-12-25 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: January 1, March 29, April 1, May 1, December 25, December 26` | `EUREX-CAL-2013` | T1 | Eurex event date 2013-12-25 (Christmas Day); one Berlin civil day per trade date, so the conversion is the identity |
| 2013-12-26 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: January 1, March 29, April 1, May 1, December 25, December 26` | `EUREX-CAL-2013` | T1 | Eurex event date 2013-12-26 (Boxing Day); one Berlin civil day per trade date, so the conversion is the identity |
| 2013-12-31 | closed | `Eurex is closed for trading in all derivatives: December 24, December 31` — a full trading closure; clearing stays open | `EUREX-CAL-2013` | T1 | Eurex event date 2013-12-31 (New Year's Eve); one Berlin civil day per trade date, so the conversion is the identity |

### 2014

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2014-01-01 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: 1 January, 18 April, 21 April, 1 May, 25 December, 26 December` | `EUREX-CAL-2014` | T1 | Eurex event date 2014-01-01 (New Year's Day); one Berlin civil day per trade date, so the conversion is the identity |
| 2014-04-18 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: 1 January, 18 April, 21 April, 1 May, 25 December, 26 December` | `EUREX-CAL-2014` | T1 | Eurex event date 2014-04-18 (Good Friday); one Berlin civil day per trade date, so the conversion is the identity |
| 2014-04-21 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: 1 January, 18 April, 21 April, 1 May, 25 December, 26 December` | `EUREX-CAL-2014` | T1 | Eurex event date 2014-04-21 (Easter Monday); one Berlin civil day per trade date, so the conversion is the identity |
| 2014-05-01 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: 1 January, 18 April, 21 April, 1 May, 25 December, 26 December` | `EUREX-CAL-2014` | T1 | Eurex event date 2014-05-01 (Labour Day); one Berlin civil day per trade date, so the conversion is the identity |
| 2014-12-24 | closed | `Eurex is closed for trading in all derivatives: 24 December, 31 December` — a full trading closure; clearing stays open | `EUREX-CAL-2014` | T1 | Eurex event date 2014-12-24 (Christmas Eve); one Berlin civil day per trade date, so the conversion is the identity |
| 2014-12-25 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: 1 January, 18 April, 21 April, 1 May, 25 December, 26 December` | `EUREX-CAL-2014` | T1 | Eurex event date 2014-12-25 (Christmas Day); one Berlin civil day per trade date, so the conversion is the identity |
| 2014-12-26 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: 1 January, 18 April, 21 April, 1 May, 25 December, 26 December` | `EUREX-CAL-2014` | T1 | Eurex event date 2014-12-26 (Boxing Day); one Berlin civil day per trade date, so the conversion is the identity |
| 2014-12-31 | closed | `Eurex is closed for trading in all derivatives: 24 December, 31 December` — a full trading closure; clearing stays open | `EUREX-CAL-2014` | T1 | Eurex event date 2014-12-31 (New Year's Eve); one Berlin civil day per trade date, so the conversion is the identity |

### 2015

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2015-01-01 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: 1 January, 3 April, 6 April, 1 May, 25 December, 26 December` | `EUREX-CAL-2015` | T1 | Eurex event date 2015-01-01 (New Year's Day); one Berlin civil day per trade date, so the conversion is the identity |
| 2015-04-03 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: 1 January, 3 April, 6 April, 1 May, 25 December, 26 December` | `EUREX-CAL-2015` | T1 | Eurex event date 2015-04-03 (Good Friday); one Berlin civil day per trade date, so the conversion is the identity |
| 2015-04-06 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: 1 January, 3 April, 6 April, 1 May, 25 December, 26 December` | `EUREX-CAL-2015` | T1 | Eurex event date 2015-04-06 (Easter Monday); one Berlin civil day per trade date, so the conversion is the identity |
| 2015-05-01 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: 1 January, 3 April, 6 April, 1 May, 25 December, 26 December` | `EUREX-CAL-2015` | T1 | Eurex event date 2015-05-01 (Labour Day); one Berlin civil day per trade date, so the conversion is the identity |
| 2015-05-25 | closed | `Eurex is closed for trading in all derivatives: 25 May, 24 December, 31 December` — a full trading closure; clearing stays open | `EUREX-CAL-2015` | T1 | Eurex event date 2015-05-25 (Whit Monday); one Berlin civil day per trade date, so the conversion is the identity |
| 2015-12-24 | closed | `Eurex is closed for trading in all derivatives: 25 May, 24 December, 31 December` — a full trading closure; clearing stays open | `EUREX-CAL-2015` | T1 | Eurex event date 2015-12-24 (Christmas Eve); one Berlin civil day per trade date, so the conversion is the identity |
| 2015-12-25 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: 1 January, 3 April, 6 April, 1 May, 25 December, 26 December` | `EUREX-CAL-2015` | T1 | Eurex event date 2015-12-25 (Christmas Day); one Berlin civil day per trade date, so the conversion is the identity |
| 2015-12-26 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: 1 January, 3 April, 6 April, 1 May, 25 December, 26 December` | `EUREX-CAL-2015` | T1 | Eurex event date 2015-12-26 (Boxing Day); one Berlin civil day per trade date, so the conversion is the identity |
| 2015-12-31 | closed | `Eurex is closed for trading in all derivatives: 25 May, 24 December, 31 December` — a full trading closure; clearing stays open | `EUREX-CAL-2015` | T1 | Eurex event date 2015-12-31 (New Year's Eve); one Berlin civil day per trade date, so the conversion is the identity |

### 2016

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2016-01-01 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: 1 January, 25 March, 28 March, 26 December` | `EUREX-CAL-2016` | T1 | Eurex event date 2016-01-01 (New Year's Day); one Berlin civil day per trade date, so the conversion is the identity |
| 2016-03-25 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: 1 January, 25 March, 28 March, 26 December` | `EUREX-CAL-2016` | T1 | Eurex event date 2016-03-25 (Good Friday); one Berlin civil day per trade date, so the conversion is the identity |
| 2016-03-28 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: 1 January, 25 March, 28 March, 26 December` | `EUREX-CAL-2016` | T1 | Eurex event date 2016-03-28 (Easter Monday); one Berlin civil day per trade date, so the conversion is the identity |
| 2016-12-26 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: 1 January, 25 March, 28 March, 26 December` | `EUREX-CAL-2016` | T1 | Eurex event date 2016-12-26 (Boxing Day); one Berlin civil day per trade date, so the conversion is the identity |

### 2017

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2017-04-14 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: 14 April, 17 April, 1 May, 25 December, 26 December` | `EUREX-CAL-2017` | T1 | Eurex event date 2017-04-14 (Good Friday); one Berlin civil day per trade date, so the conversion is the identity |
| 2017-04-17 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: 14 April, 17 April, 1 May, 25 December, 26 December` | `EUREX-CAL-2017` | T1 | Eurex event date 2017-04-17 (Easter Monday); one Berlin civil day per trade date, so the conversion is the identity |
| 2017-05-01 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: 14 April, 17 April, 1 May, 25 December, 26 December` | `EUREX-CAL-2017` | T1 | Eurex event date 2017-05-01 (Labour Day); one Berlin civil day per trade date, so the conversion is the identity |
| 2017-12-25 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: 14 April, 17 April, 1 May, 25 December, 26 December` | `EUREX-CAL-2017` | T1 | Eurex event date 2017-12-25 (Christmas Day); one Berlin civil day per trade date, so the conversion is the identity |
| 2017-12-26 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: 14 April, 17 April, 1 May, 25 December, 26 December` | `EUREX-CAL-2017` | T1 | Eurex event date 2017-12-26 (Boxing Day); one Berlin civil day per trade date, so the conversion is the identity |

### 2018

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2018-01-01 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: 1 January, 30 March, 2 April, 1 May, 25 December, 26 December` | `EUREX-CAL-2018` | T1 | Eurex event date 2018-01-01 (New Year's Day); one Berlin civil day per trade date, so the conversion is the identity |
| 2018-03-30 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: 1 January, 30 March, 2 April, 1 May, 25 December, 26 December` | `EUREX-CAL-2018` | T1 | Eurex event date 2018-03-30 (Good Friday); one Berlin civil day per trade date, so the conversion is the identity |
| 2018-04-02 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: 1 January, 30 March, 2 April, 1 May, 25 December, 26 December` | `EUREX-CAL-2018` | T1 | Eurex event date 2018-04-02 (Easter Monday); one Berlin civil day per trade date, so the conversion is the identity |
| 2018-05-01 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: 1 January, 30 March, 2 April, 1 May, 25 December, 26 December` | `EUREX-CAL-2018` | T1 | Eurex event date 2018-05-01 (Labour Day); one Berlin civil day per trade date, so the conversion is the identity |
| 2018-12-24 | closed | `Eurex is closed for trading in all derivatives: 24 December, 31 December` — a full trading closure; clearing stays open | `EUREX-CAL-2018` | T1 | Eurex event date 2018-12-24 (Christmas Eve); one Berlin civil day per trade date, so the conversion is the identity |
| 2018-12-25 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: 1 January, 30 March, 2 April, 1 May, 25 December, 26 December` | `EUREX-CAL-2018` | T1 | Eurex event date 2018-12-25 (Christmas Day); one Berlin civil day per trade date, so the conversion is the identity |
| 2018-12-26 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: 1 January, 30 March, 2 April, 1 May, 25 December, 26 December` | `EUREX-CAL-2018` | T1 | Eurex event date 2018-12-26 (Boxing Day); one Berlin civil day per trade date, so the conversion is the identity |
| 2018-12-31 | closed | `Eurex is closed for trading in all derivatives: 24 December, 31 December` — a full trading closure; clearing stays open | `EUREX-CAL-2018` | T1 | Eurex event date 2018-12-31 (New Year's Eve); one Berlin civil day per trade date, so the conversion is the identity |

### 2019

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2019-01-01 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: 1 January, 19 April, 22 April, 1 May, 25 December, 26 December` | `EUREX-CAL-2019` | T1 | Eurex event date 2019-01-01 (New Year's Day); one Berlin civil day per trade date, so the conversion is the identity |
| 2019-04-19 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: 1 January, 19 April, 22 April, 1 May, 25 December, 26 December` | `EUREX-CAL-2019` | T1 | Eurex event date 2019-04-19 (Good Friday); one Berlin civil day per trade date, so the conversion is the identity |
| 2019-04-22 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: 1 January, 19 April, 22 April, 1 May, 25 December, 26 December` | `EUREX-CAL-2019` | T1 | Eurex event date 2019-04-22 (Easter Monday); one Berlin civil day per trade date, so the conversion is the identity |
| 2019-05-01 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: 1 January, 19 April, 22 April, 1 May, 25 December, 26 December` | `EUREX-CAL-2019` | T1 | Eurex event date 2019-05-01 (Labour Day); one Berlin civil day per trade date, so the conversion is the identity |
| 2019-12-24 | closed | `Eurex is closed for trading in all derivatives: 24 December, 31 December` — a full trading closure; clearing stays open | `EUREX-CAL-2019` | T1 | Eurex event date 2019-12-24 (Christmas Eve); one Berlin civil day per trade date, so the conversion is the identity |
| 2019-12-25 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: 1 January, 19 April, 22 April, 1 May, 25 December, 26 December` | `EUREX-CAL-2019` | T1 | Eurex event date 2019-12-25 (Christmas Day); one Berlin civil day per trade date, so the conversion is the identity |
| 2019-12-26 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: 1 January, 19 April, 22 April, 1 May, 25 December, 26 December` | `EUREX-CAL-2019` | T1 | Eurex event date 2019-12-26 (Boxing Day); one Berlin civil day per trade date, so the conversion is the identity |
| 2019-12-31 | closed | `Eurex is closed for trading in all derivatives: 24 December, 31 December` — a full trading closure; clearing stays open | `EUREX-CAL-2019` | T1 | Eurex event date 2019-12-31 (New Year's Eve); one Berlin civil day per trade date, so the conversion is the identity |

### 2020

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2020-01-01 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: 1 January, 10 April, 13 April, 1 May, 25 December` | `EUREX-CAL-2020` | T1 | Eurex event date 2020-01-01 (New Year's Day); one Berlin civil day per trade date, so the conversion is the identity |
| 2020-04-10 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: 1 January, 10 April, 13 April, 1 May, 25 December` | `EUREX-CAL-2020` | T1 | Eurex event date 2020-04-10 (Good Friday); one Berlin civil day per trade date, so the conversion is the identity |
| 2020-04-13 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: 1 January, 10 April, 13 April, 1 May, 25 December` | `EUREX-CAL-2020` | T1 | Eurex event date 2020-04-13 (Easter Monday); one Berlin civil day per trade date, so the conversion is the identity |
| 2020-05-01 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: 1 January, 10 April, 13 April, 1 May, 25 December` | `EUREX-CAL-2020` | T1 | Eurex event date 2020-05-01 (Labour Day); one Berlin civil day per trade date, so the conversion is the identity |
| 2020-12-24 | closed | `Eurex is closed for trading in all derivatives: 24 December, 31 December` — a full trading closure; clearing stays open | `EUREX-CAL-2020` | T1 | Eurex event date 2020-12-24 (Christmas Eve); one Berlin civil day per trade date, so the conversion is the identity |
| 2020-12-25 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: 1 January, 10 April, 13 April, 1 May, 25 December` | `EUREX-CAL-2020` | T1 | Eurex event date 2020-12-25 (Christmas Day); one Berlin civil day per trade date, so the conversion is the identity |
| 2020-12-31 | closed | `Eurex is closed for trading in all derivatives: 24 December, 31 December` — a full trading closure; clearing stays open | `EUREX-CAL-2020` | T1 | Eurex event date 2020-12-31 (New Year's Eve); one Berlin civil day per trade date, so the conversion is the identity |

### 2021

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2021-01-01 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: 1 January, 2 April, 5 April` | `EUREX-CAL-2021` | T1 | Eurex event date 2021-01-01 (New Year's Day); one Berlin civil day per trade date, so the conversion is the identity |
| 2021-04-02 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: 1 January, 2 April, 5 April` | `EUREX-CAL-2021` | T1 | Eurex event date 2021-04-02 (Good Friday); one Berlin civil day per trade date, so the conversion is the identity |
| 2021-04-05 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: 1 January, 2 April, 5 April` | `EUREX-CAL-2021` | T1 | Eurex event date 2021-04-05 (Easter Monday); one Berlin civil day per trade date, so the conversion is the identity |
| 2021-12-24 | closed | `Eurex is closed for trading in all derivatives: 24 December, 31 December` — a full trading closure; clearing stays open | `EUREX-CAL-2021` | T1 | Eurex event date 2021-12-24 (Christmas Eve); one Berlin civil day per trade date, so the conversion is the identity |
| 2021-12-31 | closed | `Eurex is closed for trading in all derivatives: 24 December, 31 December` — a full trading closure; clearing stays open | `EUREX-CAL-2021` | T1 | Eurex event date 2021-12-31 (New Year's Eve); one Berlin civil day per trade date, so the conversion is the identity |

### 2022

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2022-04-15 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: 15 April, 18 April, 26 December` | `EUREX-CAL-2022` | T1 | Eurex event date 2022-04-15 (Good Friday); one Berlin civil day per trade date, so the conversion is the identity |
| 2022-04-18 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: 15 April, 18 April, 26 December` | `EUREX-CAL-2022` | T1 | Eurex event date 2022-04-18 (Easter Monday); one Berlin civil day per trade date, so the conversion is the identity |
| 2022-12-26 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: 15 April, 18 April, 26 December` | `EUREX-CAL-2022` | T1 | Eurex event date 2022-12-26 (Boxing Day); one Berlin civil day per trade date, so the conversion is the identity |

### 2023

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2023-04-07 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: 7 April, 10 April, 1 May, 25 December, 26 December` | `EUREX-CAL-2023` | T1 | Eurex event date 2023-04-07 (Good Friday); one Berlin civil day per trade date, so the conversion is the identity |
| 2023-04-10 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: 7 April, 10 April, 1 May, 25 December, 26 December` | `EUREX-CAL-2023` | T1 | Eurex event date 2023-04-10 (Easter Monday); one Berlin civil day per trade date, so the conversion is the identity |
| 2023-05-01 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: 7 April, 10 April, 1 May, 25 December, 26 December` | `EUREX-CAL-2023` | T1 | Eurex event date 2023-05-01 (Labour Day); one Berlin civil day per trade date, so the conversion is the identity |
| 2023-12-25 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: 7 April, 10 April, 1 May, 25 December, 26 December` | `EUREX-CAL-2023` | T1 | Eurex event date 2023-12-25 (Christmas Day); one Berlin civil day per trade date, so the conversion is the identity |
| 2023-12-26 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: 7 April, 10 April, 1 May, 25 December, 26 December` | `EUREX-CAL-2023` | T1 | Eurex event date 2023-12-26 (Boxing Day); one Berlin civil day per trade date, so the conversion is the identity |

### 2024

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2024-01-01 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: 1 January, 29 March, 1 April, 1 May, 25 December, 26 December` | `EUREX-CAL-2024` | T1 | Eurex event date 2024-01-01 (New Year's Day); one Berlin civil day per trade date, so the conversion is the identity |
| 2024-03-29 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: 1 January, 29 March, 1 April, 1 May, 25 December, 26 December` | `EUREX-CAL-2024` | T1 | Eurex event date 2024-03-29 (Good Friday); one Berlin civil day per trade date, so the conversion is the identity |
| 2024-04-01 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: 1 January, 29 March, 1 April, 1 May, 25 December, 26 December` | `EUREX-CAL-2024` | T1 | Eurex event date 2024-04-01 (Easter Monday); one Berlin civil day per trade date, so the conversion is the identity |
| 2024-05-01 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: 1 January, 29 March, 1 April, 1 May, 25 December, 26 December` | `EUREX-CAL-2024` | T1 | Eurex event date 2024-05-01 (Labour Day); one Berlin civil day per trade date, so the conversion is the identity |
| 2024-12-24 | closed | `Eurex is closed for trading in all derivatives: 24 December, 31 December` — a full trading closure; clearing stays open | `EUREX-CAL-2024` | T1 | Eurex event date 2024-12-24 (Christmas Eve); one Berlin civil day per trade date, so the conversion is the identity |
| 2024-12-25 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: 1 January, 29 March, 1 April, 1 May, 25 December, 26 December` | `EUREX-CAL-2024` | T1 | Eurex event date 2024-12-25 (Christmas Day); one Berlin civil day per trade date, so the conversion is the identity |
| 2024-12-26 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives: 1 January, 29 March, 1 April, 1 May, 25 December, 26 December` | `EUREX-CAL-2024` | T1 | Eurex event date 2024-12-26 (Boxing Day); one Berlin civil day per trade date, so the conversion is the identity |
| 2024-12-31 | closed | `Eurex is closed for trading in all derivatives: 24 December, 31 December` — a full trading closure; clearing stays open | `EUREX-CAL-2024` | T1 | Eurex event date 2024-12-31 (New Year's Eve); one Berlin civil day per trade date, so the conversion is the identity |

### 2025

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2025-01-01 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives.` | `EUREX-HOLREG-2025` | T1 | Eurex event date 2025-01-01 (New Year's Day); one Berlin civil day per trade date, so the conversion is the identity |
| 2025-04-18 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives.` | `EUREX-HOLREG-2025` | T1 | Eurex event date 2025-04-18 (Good Friday) |
| 2025-04-21 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives.` | `EUREX-HOLREG-2025` | T1 | Eurex event date 2025-04-21 (Easter Monday) |
| 2025-05-01 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives.` | `EUREX-HOLREG-2025` | T1 | Eurex event date 2025-05-01 (Labour Day) |
| 2025-12-24 | closed | `Eurex is closed for trading in all derivatives.` — a full trading closure; clearing stays open | `EUREX-HOLREG-2025` | T1 | Eurex event date 2025-12-24 (Christmas Eve) |
| 2025-12-25 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives.` | `EUREX-HOLREG-2025` | T1 | Eurex event date 2025-12-25 (Christmas Day) |
| 2025-12-26 | closed | `Eurex is closed for trading and clearing (exercise, settlement and cash) in all derivatives.` | `EUREX-HOLREG-2025` | T1 | Eurex event date 2025-12-26 (Boxing Day) |
| 2025-12-31 | closed | `Eurex is closed for trading in all derivatives.` — a full trading closure; clearing stays open | `EUREX-HOLREG-2025` | T1 | Eurex event date 2025-12-31 (New Year's Eve) |

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

**Gaps, 2026:** **Eurex 2027 does not ship.** Eurex's 2027-2036 trading calendars are published “on a preliminary and indicative basis … and are subject to change”, which is not the unconditional, day-level future LAW-NO-FABRICATED-DATES requires, so the five 2027 closures the indicative CSV carries (2027-01-01, 2027-03-26, 2027-03-29, 2027-12-24, 2027-12-31) are recorded here and encoded nowhere (re-checked 2026-09-29 UTC: fresh live reads of the regulations, indicative and archive pages leave the conflict standing — the regulations page still states the same 2027 column uncaveated, the indicative page still labels 2027-2036 preliminary and subject to change, and the archive lists no 2027 edition; see eurex.md and `holidays/raw/eurex/forward-2027/`). Closed by the Eurex “Trading Calendar 2027” PDF and the 2027 section of the Holiday regulations page. **Additional German closures are unresolved.** The Trading Calendar 2026 PDF states that Eurex “is closed for trading and exercise in German equity and equity index derivatives as well as ETF and ETC derivatives which are based on Xetra® listings: to be announced”. DAX and Mini-DAX futures are German equity index derivatives, so that line could add closures beyond the seven above for FDAX and FDXM; the day-by-day Holiday regulations page lists no German-specific 2026 closure and German Unity Day 2026-10-03 falls on a Saturday, so the practical exposure is probably nil, but the operator has not said so. Closed by a Eurex announcement resolving that line.

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
- <https://www.eurex.com/resource/blob/328400/e12f28dfcc8ba1f7b94ea4da40b69c41/data/cs_history_14092009_en.pdf.pdf> — Eurex CS amendment of 2009-09-14 (Euro-BTP introduction), whose Annex C prints the FGBL/FGBM/FGBS/FGBX baseline grid — read 2026-09-30 (UTC).
- <https://www.eurex.com/resource/blob/326452/9aa55ddb4fd3b0fb7fb15567a734ab5d/data/cs_history_19092011_en.pdf.pdf> — Eurex CS amendment of 2011-09-19 (Mid-Term Euro-BTP introduction), same rows — read 2026-09-30 (UTC).
- <https://www.eurex.com/resource/blob/334064/488cfe99b1526c777b789ade2950e4e5/data/2013_03_11_cs_history_1_en.pdf.pdf> — Eurex CS amendment of 2013-03-11 (Mid-Term Euro-OAT introduction), same rows — read 2026-09-30 (UTC).
- <https://www.eurex.com/resource/blob/295608/36bcde7eecdd7167995bf1143432e5c9/data/2017_08_28_cs_2_history_en.pdf> — Eurex CS amendment of 2017-08-28 (Post-Trading harmonisation), FGBL/FGBM/FGBS/FGBX exchange phases unchanged — read 2026-09-30 (UTC).
- <https://www.eurex.com/ex-en/rules-regs/eurex-rules-regulations/03.-Contract-Specifications-4347288> — Eurex Contract Specifications index.

## Gaps and residual risks

- **horizon** — discharged on 2026-09-30 UTC. The pre-2018-12-10 baseline was
  previously known only from Circular 088/18's own statement of the phase it
  replaced, sourcing it from the circular's date, 2018-11-15, and carrying it
  below that to the floor. Eurex's own archived Contract Specifications
  amendments (2009-09-14, 2011-09-19, 2013-03-11, 2017-08-28 — see *The
  baseline era* above) now print the same grid from before the January-2010
  floor, so the horizon is the floor and nothing below the 2018-12-10 cutover
  is carried.
- **order-entry** — discharged on 2026-09-30 UTC. The 22:30 post-trading end
  was previously carried back into the baseline from the February 2019
  amendment that records the change away from it; the 2009-09-14 amendment
  already prints `Post-Trading Full-Period 22:00-22:30` for FGBL, FGBM, FGBS
  and FGBX, so the value is sourced across the whole baseline era and the
  2019-02-25 amendment dates its one change.
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

The same grid is stated directly by the operator's archived Contract
Specifications amendments from 2009-09-14 onward (see *The baseline era*
above), including the 22:30 post-trading end the February 2019 amendment later
shortened. The baseline is therefore sourced across the whole era from the
January-2010 floor, and the 22:30 value is a sourced state rather than a
carry.

https://www.eurex.com/resource/blob/1412768/e61a2c41d65ad165af7909002223b943/data/er18088e.pdf
https://www.eurex.com/ex-en/rules-regs/eurex-rules-regulations/03.-Contract-Specifications-4347288
https://www.eurex.com/resource/blob/328400/e12f28dfcc8ba1f7b94ea4da40b69c41/data/cs_history_14092009_en.pdf.pdf

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
