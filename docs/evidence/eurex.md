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

**Coverage:** 2010-01-01..2026-12-31 (inclusive trade dates). Tier: T1 throughout.

**The identity is `incomplete` across that whole window, and the refusal is deliberate.** The
operator's undated German closure scope (below) is declared for this identity, so `coverage_on`
reports `OutsideCoveredRange` for **every** date in the window — 2026 included, which the
2026-only table this change replaced reported `Covered`. The sessions themselves are answered:
`is_open` returns `Ok(true)` on an ordinary 2025 or 2026 weekday and `Ok(false)` on a closure,
because the declaration states a completeness fact and withholds no phase. A consumer that
reads `coverage_on` to decide whether to walk a range must therefore handle the error on dates
the crate can in fact answer; that is the honest reading of an operator who declares closures
and dates none of them, and it is the same shape as `globex_cryptocurrency`'s `#93` declaration.

One table serves `Exchange::Eurex`, the `eurex` key and the `eurex_fixed_income` key. The operator states the closure for “all derivatives”, which covers FESX, FDAX and FDXM behind the index rows and FGBL, FGBM, FGBS and FGBX behind the fixed-income rows alike, so the venue intersection is the same table.

The 2025 rows are the same page's **§ 2025, day by day**, read in its third and controlling 2025 state (`EUREX-HOLREG-2025`, Wayback capture 2025-09-13), corroborated by the operator's **Eurex trading calendar 2025** PDF (`EUREX-TC-2025`). The live page carries the 2026 section only, so the 2025 section is re-verifiable through the archived captures resolved in `### Documents` below.

**Documents.**

- `EUREX-HOLREG-2026` — Eurex “Holiday regulations”, § 2026, day by day. <https://www.eurex.com/ex-en/trade/trading-calendar/holiday-regulations> (raw bytes retrieved 2026-09-12 04:19 UTC, sha256 `7b28acd2d2fb126c01461ef5a4ae11fe93e8b6f318001c6304bbce0fc78f3821`) — **T1**. Corroborated by the Trading Calendar 2026 PDF, p.2 “Overview of holidays by countries” <https://www.eurex.com/resource/blob/4873184/0ca7669a8cb9a2f917d99a801fb3f2de/data/tradingcalendar_2026_en.pdf> (retrieved 2026-09-12 04:18 UTC, sha256 `b0796b42819b38c0757d727d9b789360ba84cd0d45cea215544f86342158ac65`, PDF CreationDate 2026-07-02).
- `EUREX-HOLREG-2025` — Eurex “Holiday regulations”, § 2025, day by day, read at the Wayback `id_` capture 2025-09-13T04:14:07Z. <https://web.archive.org/web/20250913041407id_/https://www.eurex.com/ex-en/trade/trading-calendar/holiday-regulations> (raw bytes retrieved 2026-09-26T07:15:50Z, sha256 `878e46c01a166f107607c19da8842e270f97915cfc1675b8928d2167ca804ea8`) — **T1**. Corroborated by the **Eurex trading calendar 2025** PDF, p.2 “Overview of holidays by countries” <https://www.eurex.com/resource/blob/4242284/1c8da6dc2702d508ee2a4740654ea77d/data/tradingcalendar_2025_en.pdf> (retrieved 2026-09-26T07:14:29Z, sha256 `d51053a39e786022db3fa2f12e00d9646145db10aa308ea1bd4446ed0cd16614`), which a Wayback `id_` capture dated 2025-01-16 replays byte-identically.

All 2026 bytes, with each artifact's URL, UTC retrieval time and sha256, are in the research store under `holidays/raw/cfe-eurex-ice-cde-smfe-2026-2027/INDEX.md` and `holidays/raw/cfe-eurex-ice-cde-smfe-2026-2027-fix/INDEX.md`; the normalised result is `holidays/cfe-eurex-ice-cde-smfe-2026-2027.json`, verified `matches: true` with zero discrepancies in its round-2 adversarial verdict. The 2025 and 2027 artifacts are in `holidays/raw/eurex-2025-2027/INDEX.md`, whose table carries each file's exact URL, retrieval instant, sha256 and byte count, and whose working note `eurex-2025-holidays.md` transcribes § 2025 row by row.

The 2010-2024 rows are the operator's own **Trading Calendar** editions — `Eurex Trading Calendar 2010` through `Eurex trading calendar 2024`, one edition per year — whose `Overview of holidays by countries` panel opens with the all-derivatives closures in session language. Each edition states one or two lists: `closed for trading and clearing (exercise, settlement and cash) in all derivatives: …`, and where the year also closes days for trading only, a second `Eurex is closed for trading in all derivatives: …` list beside it. Every date on those two lists ships as a `Closed` row — the trading-only dates are full trading closures with clearing open, exactly the 24/31 December shape the 2025-2026 rows carry. The German-scope notes the same editions print beside those lists (Unity Day and the `tba` line) are product-scoped, are the #157 withholding recorded below, and key no row. The 2012-2024 editions were retrieved live from the operator's Trading Calendar archive on 2026-09-29 UTC; the 2010 and 2011 editions survive only on the predecessor `eurexchange.com` site and are read as Wayback `id_` replays of their era-original captures (2010-02-16 and 2011-01-14), so the window reaches the 2010-01-01 support floor with **no unaudited span**. All fifteen editions are resolved in `### Documents` below, and their bytes are saved under `holidays/raw/eurex-2010-2024/` with sha256s in that directory.

**One 2015 peculiarity, stated because it is load-bearing:** the 2015 edition is the only one that also closes Whit Monday — `Eurex is closed for trading in all derivatives: 25 May, 24 December, 31 December` — and the closure is trading-only (clearing stays open). No other edition 2010-2024 lists a Whit Monday or Ascension closure in all derivatives, and the later Holiday regulations page confirms the practice stopped, so the row ships for 2015 alone rather than being generalised into a recurring rule (LAW-HOLIDAY-SCOPE: a date-bounded arrangement is date-exception data, never a normal-week revision).

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

**Capture revision, 2025:** the page was revised during 2025 and three captures of its 2025 section survive: 2025-03-19 (58 rows, 41 of them closures), 2025-06-23 (54 rows, 37 closures) and 2025-09-13 (54 rows, 37 closures), the last being the controlling state transcribed here. The June and September captures agree row for row, and the whole delta is confined to one product scope, `Daily Futures on KOSPI 200 Derivatives and Daily USD/KRW Futures`: 03 June was added, and the 15 August KOSPI clause together with the 03, 06, 07, 08 and 09 October KOSPI closures were withdrawn. Every one of the eight rows above — and every `… in all derivatives` row — is byte-identical in all three captures. Neither product is in this identity's documented scope, which the ledger's basis note states as FESX/FDAX/FDXM benchmark-index futures, so the revision moves no row of this table and no other product of this identity's families is touched by it.

### 2027 is published twice, with conflicting labels, and is not encoded

The **Holiday regulations** page states 2027 with no caveat at all. Its table headed `Non-trading days at Eurex 2026 - 2030` prints the 2027 column as, verbatim, New Year's Day `Friday 01 Jan 2027`, Good Friday `Friday 26 Mar 2027`, Easter Monday `Monday 29 Mar 2027`, Labour Day `Saturday 01 May 2027`, Christmas Eve\* `Friday 24 Dec 2027`, Christmas Day `Saturday 25 Dec 2027`, Boxing Day `Sunday 26 Dec 2027` and New Year's Eve\* `Friday 31 Dec 2027`, under the same `* No trading; clearing and settlement are open if the holiday is not on a Saturday or Sunday.` footnote as 2025 and 2026. The words `preliminar` and `subject to change` appear nowhere on the live page or on any of the three 2025 captures, and the live page's ten `indicativ` matches are all navigation links to the Indicative Trading Calendars page rather than text in or beside the table, so the 2027 column itself carries no caveat.

Eurex's parallel **Indicative Trading Calendars** page labels the same years the other way, verbatim:

> The following trading calendars are provided on a preliminary and indicative basis for the years 2027 to 2036 to support forward planning by our customers and are subject to change.

and, under its sub-heading `Indicative trading holidays by calendar for the period 2027-2036`:

> The trading calendars for the years 2027 to 2036 are provided on an indicative and preliminary basis. All dates are subject to change and may be updated to reflect official announcements, holiday schedules, or other adjustments from the home exchanges, including any ad-hoc modifications. Please note that while Christmas Eve and New Year's Eve may not be recognized as trading holidays by the home exchanges, they are marked as holidays in the trading calendars of Eurex Deutschland.

Lineage cannot settle the conflict: both are live pages of the same operator and neither reprints the other. So the 2027 dates above are **recorded here and encoded nowhere** (LAW-NO-FABRICATED-DATES), and coverage stops at 2026-12-31. Re-checked 2026-09-29 UTC with fresh live reads of all three pages (artifacts under `holidays/raw/eurex/forward-2027/`): the regulations page still prints the same 2027 column uncaveated, the indicative page still opens `The following trading calendars are provided on a preliminary and indicative basis for the years 2027 to 2036 … and are subject to change`, and the archive page still contains the string `2027` zero times — the conflict stands and nothing past 2026-12-31 is claimed. Re-checked again 2026-10-02 UTC with fresh live reads of all three pages (artifacts `eurex_holiday_regulations.live-20261002T235200Z.html`, `eurex_indicative_trading_calendars.live-20261002T235200Z.html` and `eurex_trading_calendar_archive.live-20261002T235200Z.html` under `holidays/raw/eurex/forward-2027/`, `INDEX-recheck-2026-10-02.md` beside them): the regulations page's bytes still contain no `preliminar` caveat anywhere, the indicative page still opens with the same preliminary-and-indicative sentence, and the archive page still contains `2027` zero times with `tradingcalendar_2026` its newest edition linkage. Closing condition: the operator drops the preliminary label from its 2027-2036 calendars, or publishes a Trading Calendar 2027 edition — the Trading Calendar archive page lists editions 2012 through 2026 and contains the string `2027` zero times. An HTTP 200 from a guessed `tradingcalendar_2027_en.pdf` URL is not evidence: Eurex's blob route is keyed on the blob id and hash, not the trailing filename, and returns the 2026 PDF for a filename it does not have.

### Documents

| Document | Window | Replay or service URL | Capture or retrieval, UTC | Tier | sha256 |
|---|---|---|---|---|---|
| `EUREX-CAL-2010` | 2010-01-01 .. 2010-12-31 | <https://web.archive.org/web/20100216003515id_/http://www.eurexchange.com/download/trading/tradingcalendar_2010_en.pdf> | Wayback `id_` replay of capture `20100216003515`, retrieved 2026-09-29 UTC | T1 | `9caaee91446b115fc1b069fec44653290cf1e57cefc26f86859fe5bee0e98f59` |
| `EUREX-CAL-2011` | 2011-01-01 .. 2011-12-31 | <https://web.archive.org/web/20110114201916id_/http://www.eurexchange.com/download/trading/tradingcalendar_2011_en.pdf> | Wayback `id_` replay of capture `20110114201916`, retrieved 2026-09-29 UTC | T1 | `329fe76877537c873b433ab0b7fe2093b58fd72d0162444479d53a90ef704b2c` |
| `EUREX-CAL-2012` | 2012-01-01 .. 2012-12-31 | <https://www.eurex.com/resource/blob/284994/e988f1cef4437150c0a74a5a8f285d64/data/tradingcalendar_2012_en.pdf> | retrieved 2026-09-29 UTC | T1 | `3045def26f7af8ab5228b5714fb3d75043d14287c2ea8e476c43f3b0ac8d5739` |
| `EUREX-CAL-2013` | 2013-01-01 .. 2013-12-31 | <https://www.eurex.com/resource/blob/244836/3ad19654e330995e7d07e4da214ccecd/data/tradingcalendar_2013_en.pdf> | retrieved 2026-09-29 UTC | T1 | `c22be70750ba8bac19e45e62bf638deb0c0efd0321a193b4bc633c3a6f21fd40` |
| `EUREX-CAL-2014` | 2014-01-01 .. 2014-12-31 | <https://www.eurex.com/resource/blob/249120/6fadad3fe12a3cc3fa392442532d4414/data/tradingcalendar_2014_en.pdf> | retrieved 2026-09-29 UTC | T1 | `684fe66cb662a827ef9f6b44a41d566ec122b98fab5781b81af0ec28e22a1805` |
| `EUREX-CAL-2015` | 2015-01-01 .. 2015-12-31 | <https://www.eurex.com/resource/blob/243714/8c6cea18bbe820e719a578e3bd94976c/data/tradingcalendar_2015_en.pdf> | retrieved 2026-09-29 UTC | T1 | `45b0dc151bca7bbe80f7f659b725afc3016ec8201f43a66880ba7b8782f61ef7` |
| `EUREX-CAL-2016` | 2016-01-01 .. 2016-12-31 | <https://www.eurex.com/resource/blob/1260/f42d8f907b73ce30a66148cc287f2fe8/data/tradingcalendar_2016_en.pdf> | retrieved 2026-09-29 UTC | T1 | `3ef1bb9885fe0d94f903269ff75dda52731752647ad93772e65e80aeaa4a32f7` |
| `EUREX-CAL-2017` | 2017-01-01 .. 2017-12-31 | <https://www.eurex.com/resource/blob/241960/7dc968da8ed8d80fa004bd695ac9b648/data/tradingcalendar_2017_en.pdf> | retrieved 2026-09-29 UTC | T1 | `86ce6888b3d98ad82da84eb25a36bc9e5d928f1eb0ee4fdc194c77e38cdfe2de` |
| `EUREX-CAL-2018` | 2018-01-01 .. 2018-12-31 | <https://www.eurex.com/resource/blob/2806/32828d43772ac5ee3dd3d77a79743b47/data/tradingcalendar_2018_en.pdf> | retrieved 2026-09-29 UTC | T1 | `e90fb8d6fbe90945678b8646f7f22ba30f6b2e7c3b42f2d6876b254888e8fa81` |
| `EUREX-CAL-2019` | 2019-01-01 .. 2019-12-31 | <https://www.eurex.com/resource/blob/1396378/2915487ad1e2f7d56508e1bd2f056995/data/tradingcalendar_2019_en.pdf> | retrieved 2026-09-29 UTC | T1 | `f0a745897a1f3153a61a0ae8d3eda5245e79ef1376e8abcbb629fa81910504c9` |
| `EUREX-CAL-2020` | 2020-01-01 .. 2020-12-31 | <https://www.eurex.com/resource/blob/1690338/91db2a4864f0634f3ff1695ff0dddde5/data/tradingcalendar_2020_en.pdf> | retrieved 2026-09-29 UTC | T1 | `5359f2632aa397a686615d2308fe145977ea42799c0819988d1c83ee8ca929be` |
| `EUREX-CAL-2021` | 2021-01-01 .. 2021-12-31 | <https://www.eurex.com/resource/blob/2348622/f4f9a1b370f2e74462532319f2b15ddb/data/tradingcalendar_2021_en.pdf> | retrieved 2026-09-29 UTC | T1 | `18a537935fbbf705a89485b428d525f3931467056d028a68a3d081e061b07cfd` |
| `EUREX-CAL-2022` | 2022-01-01 .. 2022-12-31 | <https://www.eurex.com/resource/blob/2886140/a13cf78f8e10f826df3bf79278dd518a/data/tradingcalendar_2022_en.pdf> | retrieved 2026-09-29 UTC | T1 | `395ab4adacb3a6596ba087f389365432cff1d63a7aadba5186a659aaf7f3b77f` |
| `EUREX-CAL-2023` | 2023-01-01 .. 2023-12-31 | <https://www.eurex.com/resource/blob/3378814/910cf372738890f691bc1bfbccfd3aef/data/tradingcalendar_2023_en.pdf> | retrieved 2026-09-29 UTC | T1 | `c1d936b4201a69d8dce24e12d97ad2d73f17f04f9e9f075511e1a74f8002892f` |
| `EUREX-CAL-2024` | 2024-01-01 .. 2024-12-31 | <https://www.eurex.com/resource/blob/3816864/92a1dacec4476fdef2819750d932968a/data/tradingcalendar_2024_en.pdf> | retrieved 2026-09-29 UTC | T1 | `9028606ab0b01bcf3d8dbae28ecd9b76984769f14eb2d6d0a6c136e2c227b7e2` |
| `EUREX-HOLREG-2025` | 2025-01-01 .. 2025-12-31 | <https://web.archive.org/web/20250913041407id_/https://www.eurex.com/ex-en/trade/trading-calendar/holiday-regulations> | archive capture 2025-09-13T04:14:07Z (retrieved 2026-09-26T07:15:50Z) | T1 | `878e46c01a166f107607c19da8842e270f97915cfc1675b8928d2167ca804ea8` |
| `EUREX-TC-2025` | 2025-01-01 .. 2025-12-31 | <https://www.eurex.com/resource/blob/4242284/1c8da6dc2702d508ee2a4740654ea77d/data/tradingcalendar_2025_en.pdf> | live retrieval 2026-09-26T07:14:29Z | T1 | `d51053a39e786022db3fa2f12e00d9646145db10aa308ea1bd4446ed0cd16614` |
| `EUREX-HOLREG-2026` | 2026-01-01 .. 2026-12-31 | <https://www.eurex.com/ex-en/trade/trading-calendar/holiday-regulations> | live retrieval 2026-09-12T04:19Z | T1 | `7b28acd2d2fb126c01461ef5a4ae11fe93e8b6f318001c6304bbce0fc78f3821` |
| `EUREX-TC-2026` | 2026-01-01 .. 2026-12-31 | <https://www.eurex.com/resource/blob/4873184/0ca7669a8cb9a2f917d99a801fb3f2de/data/tradingcalendar_2026_en.pdf> | live retrieval 2026-09-12T04:18Z | T1 | `b0796b42819b38c0757d727d9b789360ba84cd0d45cea215544f86342158ac65` |

`EUREX-TC-2025` is the corroborating artifact rather than a row's key: it states the same eight 2025 closures in the same session language, and it is the only artifact that carries the German-scope note below, but the day-by-day page is what each row cites, exactly as the 2026 rows cite `EUREX-HOLREG-2026` rather than the 2026 PDF.

**Gaps, 2025:** **The German equity and equity-index closure set is operator-declared `tba`.** The *Eurex trading calendar 2025* PDF prints, verbatim: `Kein Handel und keine Ausübung in deutschen Aktien- und Aktienindex-derivaten sowie in ETF- und ETC-Derivaten, die auf Xetra@-Börsen-notierungen basieren: tba.` — that is, no trading and no exercise in German equity and equity-index derivatives and in the ETF and ETC derivatives based on Xetra® listings, to be announced. FDAX and FDXM are German equity-index derivatives, so 2025 could carry German closures this table does not state, and the operator has named no date for any of them. The 2026 edition of the same PDF carries the note in English and still says `Eurex is closed for trading and exercise in German equity and equity index derivatives as well as ETF and ETC derivatives which are based on Xetra® listings: to be announced`, and the 2027 conflict recorded above leaves no later edition to read it from either. **This is an operator-declared withholding, not a retrieval failure**: the Holiday regulations page never carries the note at all, so it cannot be closed from that page, and it must not be closed from a T3 restatement (LAW-PRIMARY-SOURCES). Closing condition: an Eurex announcement or Trading Calendar edition that dates the German-scope closures. Tracked as issue #157.

**The same scope is dated, not withheld, in the pre-2025 editions — and stands unencoded there.** The 2014, 2016, 2017, 2018-2021 and 2022-2024 editions print the German line with dates (for example the 2016 edition closes German equity and equity-index derivatives on `16 May, 3 October`), while the 2010-2013 and 2015 editions state no German-scope line at all. Those dates are not on this table's lists — they close only the German product scope, and the one table this crate ships serves FESX alongside FDAX/FDXM — so the #157 record spans the whole audited window: dated-but-unencoded in 2010-2024, operator-withheld (`tba`) since 2025. Encoding the dated German-scope dates without splitting the table is the same follow-up #157 carries; a consumer that routes FDAX or FDXM must not read this table as complete on a German Unity Day.

**Interpretive steps, 2025:** None beyond the trading-only rows. Eurex states its closures as whole days and every phase the crate models runs inside one Berlin civil day, so each event date is its own trade date and each row is `Closed`. The two trading-only rows (24 and 31 December) carry the operator's own footnote, `* No trading; clearing and settlement are open if the holiday is not on a Saturday or Sunday.`; both dates fall on a Wednesday in 2025, so clearing and settlement are open on them. That does not make either a trading day: this crate answers when a *market* accepts and matches orders, and "no trading" removes every phase it models, so a closed-for-trading date is `Closed` whatever the clearing and settlement half does.

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
