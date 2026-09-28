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

Row review: 2026-08-22 (UTC) is the date the ledger row was last reviewed as a
whole. Per-source retrieval dates were not recorded before the 2026-09-12
migration; where a later targeted review, capture or document date is recorded
beside a source below, that date governs for that source, and dates are added
as each source is re-verified.

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
- <https://www.nyse.com/trade/hours-calendars> — the operator's holiday calendar and phase table (live, states 2026-2028); the 2014-2025 era lives at `nyse.com/markets/hours-calendars` and the 2007-2013 page at `nyse.com/about/newsevents/1176373643795.html` (Wayback replays, digests in the `### Documents` tables above).
- <https://ir.theice.com/press/news-details/2024/The-New-York-Stock-Exchange-Will-Close-Markets-on-January-9-to-Honor-the-Passing-of-Former-President-Jimmy-Carter-on-National-Day-of-Mourning/default.aspx> — the National Day of Mourning release naming the New York Stock Exchange (2024-12-30).
- NYSE Euronext press releases `nyse.com/press/1351243418010.html` (2012-10-28) and `nyse.com/press/1351243421978.html` (2012-10-29) — the Hurricane Sandy closure statements, read through the web archive.

## Holidays

**Coverage:** 2010-01-01..2027-12-31 (inclusive trade dates). Tier: T1 throughout.

**Service tier.** The identity is served: SharurPlatform routes NYSE cash equities (LAW-SERVICE-TIERS admission by consumer reach). The cadence is monthly per LAW-WATCH — holiday-bearing and the operator republishes the calendar yearly.

**Corpus and retrieval story (2026-09-27 UTC).** Every artifact is the operator's own page or release, saved under `holidays/raw/nyse-nasdaq/nyse/` in the research store with its sha256 (`manifest.json` beside each directory). The operator's holiday page has had three homes: `nyse.com/about/newsevents/1176373643795.html` (captured 2010-02-08, 2010-12-03 and 2011-07-02; the 2011 capture states the 2011, 2012 **and** 2013 tables and footnotes), the JS-rendered `nyse.com/markets/hours-calendars` from 2014 (the 2014-11-24 capture carries the 2014/2015 tables server-side; monthly 200 captures run 2015-2026), and the current `nyse.com/trade/hours-calendars` (live retrieval, stating 2026, 2027 and 2028). The 2010-11-26 early close is printed only on the late-2010 capture (`NYSE-HOL-2010LATE`), whose footnote reads in full: "Each market will close at 1:00 p.m. on Friday, November 26, 2010 (the day after Thanksgiving). Crossing Session orders will be accepted beginning at 1:00 p.m. for continuous executions until 1:30 p.m. on this date." No channel refused for this venue: every year 2010-2027 is printed in full by at least one operator artifact, so the table ships zero `Unsourced` dates.

**Early-close convention (set here for the equities programme).** The sheet's own wording decides. For every early-close date 2010-2027 the operator prints exactly one fact about this identity — "Each market will close early at 1:00 p.m. (1:15 p.m. for eligible options) on <date>" — and prints nothing after it: no continuing session, no extended hours. The row is therefore `EarlyClose{13:00}`: the trade date's final close moves to 13:00, which is everything the sheet states. Two clauses in the same footnotes are deliberately **not** encoded, and recording them is part of this convention:

- "Crossing Session orders will be accepted beginning at 1:00 p.m. for continuous executions until 1:30 p.m. on this date" — the Off-Hours Trading Facility's Crossing Session II, which the `nyse` envelope does not model at all (the row's recorded executable gap below, modelled nowhere, so the answer errs toward closed). Encoding it would widen the executable envelope from two endpoints of a facility whose complete amendment chain is unestablished.
- "NYSE American Equities, NYSE Arca Equities, NYSE Chicago, NYSE National [and NYSE Texas] late trading sessions will close at 5:00 p.m." (worded without Chicago/Texas before 2020, at 4:00 p.m. on the 2018 page) — those venues are separate identities; nothing in the clause concerns the New York Stock Exchange's own book, and no `nyse` row moves for it.

No `ReplacementBlocks` rows ship for this identity: the operator never prints a post-13:00 session for the exchange itself on any early-close date, so there is no topology to restate and inventing one would fabricate instants.

**Unscheduled closures.** Two events interrupt the annual grids, each keyed from the operator's own release:

- **Hurricane Sandy, 2012-10-29/30.** The 2012-10-28 release states "NYSE Euronext (NYX) will close its markets on Monday, Oct. 29, 2012 and pending confirmation on Tuesday, Oct. 30, 2012"; the 2012-10-29 release states the Tuesday closure unconditionally — "will close its markets in coordination with all U.S. equities, bonds, options and derivatives markets on Tuesday, Oct. 30, 2012. This follows the closure of U.S. markets on Monday, Oct. 29, 2012." The Monday row keys on the first release, the Tuesday row on the second: each row's own document states its date unconditionally, and the conditional first-release wording for 10-30 is superseded, never relied on.
- **National Day of Mourning for President Carter, 2025-01-09.** The ICE release of 2024-12-30 states The New York Stock Exchange "will close all NYSE Group equity and options markets on Thursday, January 9, 2025, in observance of the National Day of Mourning", naming the New York Stock Exchange among the closing markets. The closure never appears on the operator's holiday grid (the grids list scheduled holidays only), which is why the row keys on the release.

**Observation rules.** Saturday holidays are observed Friday (Sunday holidays Monday) per the rulebook text the sheets print; the observed day, not the anniversary, carries the `Closed` row (2010-12-24, 2015-07-03, 2021-12-24, 2022-06-20, 2023-01-02, 2027-06-18, 2027-07-05 and siblings). The one sheet note that a Saturday holiday is *not* observed when the Friday ends a monthly or yearly accounting period (New Year 2010/2011 wording) never fired inside the window: 2010-01-01 and 2011 were handled by the operator as printed, and the rows follow the printed dates.

**Gaps and residual risks.** The normal-week executable gap (Crossing Session II until 2024-01-31, NYSE Bonds, the pre-2018 Tape A acceptance edge) is unchanged by this table and is recorded in the ledger basis above; it is the reason the coverage verdict reads incomplete even though the holiday layer itself is complete across the window. The 2028 column the live page prints (including a 2028-07-03 early close) is deliberately outside the audited window and carries no rows.

### 2010

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2010-01-01 | closed | `Closed` | `NYSE-HOL-2010` | T1 | event date 2010-01-01; the sheet prints no session for it |
| 2010-01-18 | closed | `Closed` | `NYSE-HOL-2010` | T1 | event date 2010-01-18; the sheet prints no session for it |
| 2010-02-15 | closed | `Closed` | `NYSE-HOL-2010` | T1 | event date 2010-02-15; the sheet prints no session for it |
| 2010-04-02 | closed | `Closed` | `NYSE-HOL-2010` | T1 | event date 2010-04-02; the sheet prints no session for it |
| 2010-05-31 | closed | `Closed` | `NYSE-HOL-2010` | T1 | event date 2010-05-31; the sheet prints no session for it |
| 2010-07-05 | closed | `Closed` | `NYSE-HOL-2010` | T1 | event date 2010-07-05; the sheet prints no session for it |
| 2010-09-06 | closed | `Closed` | `NYSE-HOL-2010` | T1 | event date 2010-09-06; the sheet prints no session for it |
| 2010-11-25 | closed | `Closed` | `NYSE-HOL-2010` | T1 | event date 2010-11-25; the sheet prints no session for it |
| 2010-11-26 | early close | `close early at 1:00 p.m.` | `NYSE-HOL-2010LATE` | T1 | event date 2010-11-26; the year footnote prints the 1:00 p.m. close and nothing after it |
| 2010-12-24 | closed | `Closed` | `NYSE-HOL-2010LATE` | T1 | event date 2010-12-24; the sheet prints no session for it |

### 2011

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2011-01-17 | closed | `Closed` | `NYSE-HOL-2011` | T1 | event date 2011-01-17; the sheet prints no session for it |
| 2011-02-21 | closed | `Closed` | `NYSE-HOL-2011` | T1 | event date 2011-02-21; the sheet prints no session for it |
| 2011-04-22 | closed | `Closed` | `NYSE-HOL-2011` | T1 | event date 2011-04-22; the sheet prints no session for it |
| 2011-05-30 | closed | `Closed` | `NYSE-HOL-2011` | T1 | event date 2011-05-30; the sheet prints no session for it |
| 2011-07-04 | closed | `Closed` | `NYSE-HOL-2011` | T1 | event date 2011-07-04; the sheet prints no session for it |
| 2011-09-05 | closed | `Closed` | `NYSE-HOL-2011` | T1 | event date 2011-09-05; the sheet prints no session for it |
| 2011-11-24 | closed | `Closed` | `NYSE-HOL-2011` | T1 | event date 2011-11-24; the sheet prints no session for it |
| 2011-11-25 | early close | `close early at 1:00 p.m.` | `NYSE-HOL-2011` | T1 | event date 2011-11-25; the year footnote prints the 1:00 p.m. close and nothing after it |
| 2011-12-26 | closed | `Closed` | `NYSE-HOL-2011` | T1 | event date 2011-12-26; the sheet prints no session for it |

### 2012

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2012-01-02 | closed | `Closed` | `NYSE-HOL-2011` | T1 | event date 2012-01-02; the sheet prints no session for it |
| 2012-01-16 | closed | `Closed` | `NYSE-HOL-2011` | T1 | event date 2012-01-16; the sheet prints no session for it |
| 2012-02-20 | closed | `Closed` | `NYSE-HOL-2011` | T1 | event date 2012-02-20; the sheet prints no session for it |
| 2012-04-06 | closed | `Closed` | `NYSE-HOL-2011` | T1 | event date 2012-04-06; the sheet prints no session for it |
| 2012-05-28 | closed | `Closed` | `NYSE-HOL-2011` | T1 | event date 2012-05-28; the sheet prints no session for it |
| 2012-07-03 | early close | `close early at 1:00 p.m.` | `NYSE-HOL-2011` | T1 | event date 2012-07-03; the year footnote prints the 1:00 p.m. close and nothing after it |
| 2012-07-04 | closed | `Closed` | `NYSE-HOL-2011` | T1 | event date 2012-07-04; the sheet prints no session for it |
| 2012-09-03 | closed | `Closed` | `NYSE-HOL-2011` | T1 | event date 2012-09-03; the sheet prints no session for it |
| 2012-10-29 | closed | `Closed` | `NYSE-SANDY-2012A` | T1 | event date 2012-10-29; the sheet prints no session for it |
| 2012-10-30 | closed | `Closed` | `NYSE-SANDY-2012B` | T1 | event date 2012-10-30; the sheet prints no session for it |
| 2012-11-22 | closed | `Closed` | `NYSE-HOL-2011` | T1 | event date 2012-11-22; the sheet prints no session for it |
| 2012-11-23 | early close | `close early at 1:00 p.m.` | `NYSE-HOL-2011` | T1 | event date 2012-11-23; the year footnote prints the 1:00 p.m. close and nothing after it |
| 2012-12-24 | early close | `close early at 1:00 p.m.` | `NYSE-HOL-2011` | T1 | event date 2012-12-24; the year footnote prints the 1:00 p.m. close and nothing after it |
| 2012-12-25 | closed | `Closed` | `NYSE-HOL-2011` | T1 | event date 2012-12-25; the sheet prints no session for it |

### 2013

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2013-01-01 | closed | `Closed` | `NYSE-HOL-2011` | T1 | event date 2013-01-01; the sheet prints no session for it |
| 2013-01-21 | closed | `Closed` | `NYSE-HOL-2011` | T1 | event date 2013-01-21; the sheet prints no session for it |
| 2013-02-18 | closed | `Closed` | `NYSE-HOL-2011` | T1 | event date 2013-02-18; the sheet prints no session for it |
| 2013-03-29 | closed | `Closed` | `NYSE-HOL-2011` | T1 | event date 2013-03-29; the sheet prints no session for it |
| 2013-05-27 | closed | `Closed` | `NYSE-HOL-2011` | T1 | event date 2013-05-27; the sheet prints no session for it |
| 2013-07-03 | early close | `close early at 1:00 p.m.` | `NYSE-HOL-2011` | T1 | event date 2013-07-03; the year footnote prints the 1:00 p.m. close and nothing after it |
| 2013-07-04 | closed | `Closed` | `NYSE-HOL-2011` | T1 | event date 2013-07-04; the sheet prints no session for it |
| 2013-09-02 | closed | `Closed` | `NYSE-HOL-2011` | T1 | event date 2013-09-02; the sheet prints no session for it |
| 2013-11-28 | closed | `Closed` | `NYSE-HOL-2011` | T1 | event date 2013-11-28; the sheet prints no session for it |
| 2013-11-29 | early close | `close early at 1:00 p.m.` | `NYSE-HOL-2011` | T1 | event date 2013-11-29; the year footnote prints the 1:00 p.m. close and nothing after it |
| 2013-12-24 | early close | `close early at 1:00 p.m.` | `NYSE-HOL-2011` | T1 | event date 2013-12-24; the year footnote prints the 1:00 p.m. close and nothing after it |
| 2013-12-25 | closed | `Closed` | `NYSE-HOL-2011` | T1 | event date 2013-12-25; the sheet prints no session for it |

### 2014

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2014-01-01 | closed | `Closed` | `NYSE-HOL-2014` | T1 | event date 2014-01-01; the sheet prints no session for it |
| 2014-01-20 | closed | `Closed` | `NYSE-HOL-2014` | T1 | event date 2014-01-20; the sheet prints no session for it |
| 2014-02-17 | closed | `Closed` | `NYSE-HOL-2014` | T1 | event date 2014-02-17; the sheet prints no session for it |
| 2014-04-18 | closed | `Closed` | `NYSE-HOL-2014` | T1 | event date 2014-04-18; the sheet prints no session for it |
| 2014-05-26 | closed | `Closed` | `NYSE-HOL-2014` | T1 | event date 2014-05-26; the sheet prints no session for it |
| 2014-07-03 | early close | `close early at 1:00 p.m.` | `NYSE-HOL-2014` | T1 | event date 2014-07-03; the year footnote prints the 1:00 p.m. close and nothing after it |
| 2014-07-04 | closed | `Closed` | `NYSE-HOL-2014` | T1 | event date 2014-07-04; the sheet prints no session for it |
| 2014-09-01 | closed | `Closed` | `NYSE-HOL-2014` | T1 | event date 2014-09-01; the sheet prints no session for it |
| 2014-11-27 | closed | `Closed` | `NYSE-HOL-2014` | T1 | event date 2014-11-27; the sheet prints no session for it |
| 2014-11-28 | early close | `close early at 1:00 p.m.` | `NYSE-HOL-2014` | T1 | event date 2014-11-28; the year footnote prints the 1:00 p.m. close and nothing after it |
| 2014-12-24 | early close | `close early at 1:00 p.m.` | `NYSE-HOL-2014` | T1 | event date 2014-12-24; the year footnote prints the 1:00 p.m. close and nothing after it |
| 2014-12-25 | closed | `Closed` | `NYSE-HOL-2014` | T1 | event date 2014-12-25; the sheet prints no session for it |

### 2015

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2015-01-01 | closed | `Closed` | `NYSE-HOL-2015` | T1 | event date 2015-01-01; the sheet prints no session for it |
| 2015-01-19 | closed | `Closed` | `NYSE-HOL-2015` | T1 | event date 2015-01-19; the sheet prints no session for it |
| 2015-02-16 | closed | `Closed` | `NYSE-HOL-2015` | T1 | event date 2015-02-16; the sheet prints no session for it |
| 2015-04-03 | closed | `Closed` | `NYSE-HOL-2015` | T1 | event date 2015-04-03; the sheet prints no session for it |
| 2015-05-25 | closed | `Closed` | `NYSE-HOL-2015` | T1 | event date 2015-05-25; the sheet prints no session for it |
| 2015-07-03 | closed | `Closed` | `NYSE-HOL-2015` | T1 | event date 2015-07-03; the sheet prints no session for it |
| 2015-09-07 | closed | `Closed` | `NYSE-HOL-2015` | T1 | event date 2015-09-07; the sheet prints no session for it |
| 2015-11-26 | closed | `Closed` | `NYSE-HOL-2015` | T1 | event date 2015-11-26; the sheet prints no session for it |
| 2015-11-27 | early close | `close early at 1:00 p.m.` | `NYSE-HOL-2015` | T1 | event date 2015-11-27; the year footnote prints the 1:00 p.m. close and nothing after it |
| 2015-12-24 | early close | `close early at 1:00 p.m.` | `NYSE-HOL-2015` | T1 | event date 2015-12-24; the year footnote prints the 1:00 p.m. close and nothing after it |
| 2015-12-25 | closed | `Closed` | `NYSE-HOL-2015` | T1 | event date 2015-12-25; the sheet prints no session for it |

### 2016

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2016-01-01 | closed | `Closed` | `NYSE-HOL-2016` | T1 | event date 2016-01-01; the sheet prints no session for it |
| 2016-01-18 | closed | `Closed` | `NYSE-HOL-2016` | T1 | event date 2016-01-18; the sheet prints no session for it |
| 2016-02-15 | closed | `Closed` | `NYSE-HOL-2016` | T1 | event date 2016-02-15; the sheet prints no session for it |
| 2016-03-25 | closed | `Closed` | `NYSE-HOL-2016` | T1 | event date 2016-03-25; the sheet prints no session for it |
| 2016-05-30 | closed | `Closed` | `NYSE-HOL-2016` | T1 | event date 2016-05-30; the sheet prints no session for it |
| 2016-07-04 | closed | `Closed` | `NYSE-HOL-2016` | T1 | event date 2016-07-04; the sheet prints no session for it |
| 2016-09-05 | closed | `Closed` | `NYSE-HOL-2016` | T1 | event date 2016-09-05; the sheet prints no session for it |
| 2016-11-24 | closed | `Closed` | `NYSE-HOL-2016` | T1 | event date 2016-11-24; the sheet prints no session for it |
| 2016-11-25 | early close | `close early at 1:00 p.m.` | `NYSE-HOL-2016` | T1 | event date 2016-11-25; the year footnote prints the 1:00 p.m. close and nothing after it |
| 2016-12-26 | closed | `Closed` | `NYSE-HOL-2016` | T1 | event date 2016-12-26; the sheet prints no session for it |

### 2017

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2017-01-02 | closed | `Closed` | `NYSE-HOL-2017` | T1 | event date 2017-01-02; the sheet prints no session for it |
| 2017-01-16 | closed | `Closed` | `NYSE-HOL-2017` | T1 | event date 2017-01-16; the sheet prints no session for it |
| 2017-02-20 | closed | `Closed` | `NYSE-HOL-2017` | T1 | event date 2017-02-20; the sheet prints no session for it |
| 2017-04-14 | closed | `Closed` | `NYSE-HOL-2017` | T1 | event date 2017-04-14; the sheet prints no session for it |
| 2017-05-29 | closed | `Closed` | `NYSE-HOL-2017` | T1 | event date 2017-05-29; the sheet prints no session for it |
| 2017-07-03 | early close | `close early at 1:00 p.m.` | `NYSE-HOL-2017` | T1 | event date 2017-07-03; the year footnote prints the 1:00 p.m. close and nothing after it |
| 2017-07-04 | closed | `Closed` | `NYSE-HOL-2017` | T1 | event date 2017-07-04; the sheet prints no session for it |
| 2017-09-04 | closed | `Closed` | `NYSE-HOL-2017` | T1 | event date 2017-09-04; the sheet prints no session for it |
| 2017-11-23 | closed | `Closed` | `NYSE-HOL-2017` | T1 | event date 2017-11-23; the sheet prints no session for it |
| 2017-11-24 | early close | `close early at 1:00 p.m.` | `NYSE-HOL-2017` | T1 | event date 2017-11-24; the year footnote prints the 1:00 p.m. close and nothing after it |
| 2017-12-25 | closed | `Closed` | `NYSE-HOL-2017` | T1 | event date 2017-12-25; the sheet prints no session for it |

### 2018

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2018-01-01 | closed | `Closed` | `NYSE-HOL-2018` | T1 | event date 2018-01-01; the sheet prints no session for it |
| 2018-01-15 | closed | `Closed` | `NYSE-HOL-2018` | T1 | event date 2018-01-15; the sheet prints no session for it |
| 2018-02-19 | closed | `Closed` | `NYSE-HOL-2018` | T1 | event date 2018-02-19; the sheet prints no session for it |
| 2018-03-30 | closed | `Closed` | `NYSE-HOL-2018` | T1 | event date 2018-03-30; the sheet prints no session for it |
| 2018-05-28 | closed | `Closed` | `NYSE-HOL-2018` | T1 | event date 2018-05-28; the sheet prints no session for it |
| 2018-07-03 | early close | `close early at 1:00 p.m.` | `NYSE-HOL-2018` | T1 | event date 2018-07-03; the year footnote prints the 1:00 p.m. close and nothing after it |
| 2018-07-04 | closed | `Closed` | `NYSE-HOL-2018` | T1 | event date 2018-07-04; the sheet prints no session for it |
| 2018-09-03 | closed | `Closed` | `NYSE-HOL-2018` | T1 | event date 2018-09-03; the sheet prints no session for it |
| 2018-11-22 | closed | `Closed` | `NYSE-HOL-2018` | T1 | event date 2018-11-22; the sheet prints no session for it |
| 2018-11-23 | early close | `close early at 1:00 p.m.` | `NYSE-HOL-2018` | T1 | event date 2018-11-23; the year footnote prints the 1:00 p.m. close and nothing after it |
| 2018-12-24 | early close | `close early at 1:00 p.m.` | `NYSE-HOL-2018` | T1 | event date 2018-12-24; the year footnote prints the 1:00 p.m. close and nothing after it |
| 2018-12-25 | closed | `Closed` | `NYSE-HOL-2018` | T1 | event date 2018-12-25; the sheet prints no session for it |

### 2019

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2019-01-01 | closed | `Closed` | `NYSE-HOL-2019` | T1 | event date 2019-01-01; the sheet prints no session for it |
| 2019-01-21 | closed | `Closed` | `NYSE-HOL-2019` | T1 | event date 2019-01-21; the sheet prints no session for it |
| 2019-02-18 | closed | `Closed` | `NYSE-HOL-2019` | T1 | event date 2019-02-18; the sheet prints no session for it |
| 2019-04-19 | closed | `Closed` | `NYSE-HOL-2019` | T1 | event date 2019-04-19; the sheet prints no session for it |
| 2019-05-27 | closed | `Closed` | `NYSE-HOL-2019` | T1 | event date 2019-05-27; the sheet prints no session for it |
| 2019-07-03 | early close | `close early at 1:00 p.m.` | `NYSE-HOL-2019` | T1 | event date 2019-07-03; the year footnote prints the 1:00 p.m. close and nothing after it |
| 2019-07-04 | closed | `Closed` | `NYSE-HOL-2019` | T1 | event date 2019-07-04; the sheet prints no session for it |
| 2019-09-02 | closed | `Closed` | `NYSE-HOL-2019` | T1 | event date 2019-09-02; the sheet prints no session for it |
| 2019-11-28 | closed | `Closed` | `NYSE-HOL-2019` | T1 | event date 2019-11-28; the sheet prints no session for it |
| 2019-11-29 | early close | `close early at 1:00 p.m.` | `NYSE-HOL-2019` | T1 | event date 2019-11-29; the year footnote prints the 1:00 p.m. close and nothing after it |
| 2019-12-24 | early close | `close early at 1:00 p.m.` | `NYSE-HOL-2019` | T1 | event date 2019-12-24; the year footnote prints the 1:00 p.m. close and nothing after it |
| 2019-12-25 | closed | `Closed` | `NYSE-HOL-2019` | T1 | event date 2019-12-25; the sheet prints no session for it |

### 2020

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2020-01-01 | closed | `Closed` | `NYSE-HOL-2020` | T1 | event date 2020-01-01; the sheet prints no session for it |
| 2020-01-20 | closed | `Closed` | `NYSE-HOL-2020` | T1 | event date 2020-01-20; the sheet prints no session for it |
| 2020-02-17 | closed | `Closed` | `NYSE-HOL-2020` | T1 | event date 2020-02-17; the sheet prints no session for it |
| 2020-04-10 | closed | `Closed` | `NYSE-HOL-2020` | T1 | event date 2020-04-10; the sheet prints no session for it |
| 2020-05-25 | closed | `Closed` | `NYSE-HOL-2020` | T1 | event date 2020-05-25; the sheet prints no session for it |
| 2020-07-03 | closed | `Closed` | `NYSE-HOL-2020` | T1 | event date 2020-07-03; the sheet prints no session for it |
| 2020-09-07 | closed | `Closed` | `NYSE-HOL-2020` | T1 | event date 2020-09-07; the sheet prints no session for it |
| 2020-11-26 | closed | `Closed` | `NYSE-HOL-2020` | T1 | event date 2020-11-26; the sheet prints no session for it |
| 2020-11-27 | early close | `close early at 1:00 p.m.` | `NYSE-HOL-2020` | T1 | event date 2020-11-27; the year footnote prints the 1:00 p.m. close and nothing after it |
| 2020-12-24 | early close | `close early at 1:00 p.m.` | `NYSE-HOL-2020` | T1 | event date 2020-12-24; the year footnote prints the 1:00 p.m. close and nothing after it |
| 2020-12-25 | closed | `Closed` | `NYSE-HOL-2020` | T1 | event date 2020-12-25; the sheet prints no session for it |

### 2021

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2021-01-01 | closed | `Closed` | `NYSE-HOL-2021` | T1 | event date 2021-01-01; the sheet prints no session for it |
| 2021-01-18 | closed | `Closed` | `NYSE-HOL-2021` | T1 | event date 2021-01-18; the sheet prints no session for it |
| 2021-02-15 | closed | `Closed` | `NYSE-HOL-2021` | T1 | event date 2021-02-15; the sheet prints no session for it |
| 2021-04-02 | closed | `Closed` | `NYSE-HOL-2021` | T1 | event date 2021-04-02; the sheet prints no session for it |
| 2021-05-31 | closed | `Closed` | `NYSE-HOL-2021` | T1 | event date 2021-05-31; the sheet prints no session for it |
| 2021-07-05 | closed | `Closed` | `NYSE-HOL-2021` | T1 | event date 2021-07-05; the sheet prints no session for it |
| 2021-09-06 | closed | `Closed` | `NYSE-HOL-2021` | T1 | event date 2021-09-06; the sheet prints no session for it |
| 2021-11-25 | closed | `Closed` | `NYSE-HOL-2021` | T1 | event date 2021-11-25; the sheet prints no session for it |
| 2021-11-26 | early close | `close early at 1:00 p.m.` | `NYSE-HOL-2021` | T1 | event date 2021-11-26; the year footnote prints the 1:00 p.m. close and nothing after it |
| 2021-12-24 | closed | `Closed` | `NYSE-HOL-2021` | T1 | event date 2021-12-24; the sheet prints no session for it |

### 2022

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2022-01-17 | closed | `Closed` | `NYSE-HOL-2022` | T1 | event date 2022-01-17; the sheet prints no session for it |
| 2022-02-21 | closed | `Closed` | `NYSE-HOL-2022` | T1 | event date 2022-02-21; the sheet prints no session for it |
| 2022-04-15 | closed | `Closed` | `NYSE-HOL-2022` | T1 | event date 2022-04-15; the sheet prints no session for it |
| 2022-05-30 | closed | `Closed` | `NYSE-HOL-2022` | T1 | event date 2022-05-30; the sheet prints no session for it |
| 2022-06-20 | closed | `Closed` | `NYSE-HOL-2022` | T1 | event date 2022-06-20; the sheet prints no session for it |
| 2022-07-04 | closed | `Closed` | `NYSE-HOL-2022` | T1 | event date 2022-07-04; the sheet prints no session for it |
| 2022-09-05 | closed | `Closed` | `NYSE-HOL-2022` | T1 | event date 2022-09-05; the sheet prints no session for it |
| 2022-11-24 | closed | `Closed` | `NYSE-HOL-2022` | T1 | event date 2022-11-24; the sheet prints no session for it |
| 2022-11-25 | early close | `close early at 1:00 p.m.` | `NYSE-HOL-2022` | T1 | event date 2022-11-25; the year footnote prints the 1:00 p.m. close and nothing after it |
| 2022-12-26 | closed | `Closed` | `NYSE-HOL-2022` | T1 | event date 2022-12-26; the sheet prints no session for it |

### 2023

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2023-01-02 | closed | `Closed` | `NYSE-HOL-2023` | T1 | event date 2023-01-02; the sheet prints no session for it |
| 2023-01-16 | closed | `Closed` | `NYSE-HOL-2023` | T1 | event date 2023-01-16; the sheet prints no session for it |
| 2023-02-20 | closed | `Closed` | `NYSE-HOL-2023` | T1 | event date 2023-02-20; the sheet prints no session for it |
| 2023-04-07 | closed | `Closed` | `NYSE-HOL-2023` | T1 | event date 2023-04-07; the sheet prints no session for it |
| 2023-05-29 | closed | `Closed` | `NYSE-HOL-2023` | T1 | event date 2023-05-29; the sheet prints no session for it |
| 2023-06-19 | closed | `Closed` | `NYSE-HOL-2023` | T1 | event date 2023-06-19; the sheet prints no session for it |
| 2023-07-03 | early close | `close early at 1:00 p.m.` | `NYSE-HOL-2023` | T1 | event date 2023-07-03; the year footnote prints the 1:00 p.m. close and nothing after it |
| 2023-07-04 | closed | `Closed` | `NYSE-HOL-2023` | T1 | event date 2023-07-04; the sheet prints no session for it |
| 2023-09-04 | closed | `Closed` | `NYSE-HOL-2023` | T1 | event date 2023-09-04; the sheet prints no session for it |
| 2023-11-23 | closed | `Closed` | `NYSE-HOL-2023` | T1 | event date 2023-11-23; the sheet prints no session for it |
| 2023-11-24 | early close | `close early at 1:00 p.m.` | `NYSE-HOL-2023` | T1 | event date 2023-11-24; the year footnote prints the 1:00 p.m. close and nothing after it |
| 2023-12-25 | closed | `Closed` | `NYSE-HOL-2023` | T1 | event date 2023-12-25; the sheet prints no session for it |

### 2024

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2024-01-01 | closed | `Closed` | `NYSE-HOL-2024` | T1 | event date 2024-01-01; the sheet prints no session for it |
| 2024-01-15 | closed | `Closed` | `NYSE-HOL-2024` | T1 | event date 2024-01-15; the sheet prints no session for it |
| 2024-02-19 | closed | `Closed` | `NYSE-HOL-2024` | T1 | event date 2024-02-19; the sheet prints no session for it |
| 2024-03-29 | closed | `Closed` | `NYSE-HOL-2024` | T1 | event date 2024-03-29; the sheet prints no session for it |
| 2024-05-27 | closed | `Closed` | `NYSE-HOL-2024` | T1 | event date 2024-05-27; the sheet prints no session for it |
| 2024-06-19 | closed | `Closed` | `NYSE-HOL-2024` | T1 | event date 2024-06-19; the sheet prints no session for it |
| 2024-07-03 | early close | `close early at 1:00 p.m.` | `NYSE-HOL-2024` | T1 | event date 2024-07-03; the year footnote prints the 1:00 p.m. close and nothing after it |
| 2024-07-04 | closed | `Closed` | `NYSE-HOL-2024` | T1 | event date 2024-07-04; the sheet prints no session for it |
| 2024-09-02 | closed | `Closed` | `NYSE-HOL-2024` | T1 | event date 2024-09-02; the sheet prints no session for it |
| 2024-11-28 | closed | `Closed` | `NYSE-HOL-2024` | T1 | event date 2024-11-28; the sheet prints no session for it |
| 2024-11-29 | early close | `close early at 1:00 p.m.` | `NYSE-HOL-2024` | T1 | event date 2024-11-29; the year footnote prints the 1:00 p.m. close and nothing after it |
| 2024-12-24 | early close | `close early at 1:00 p.m.` | `NYSE-HOL-2024` | T1 | event date 2024-12-24; the year footnote prints the 1:00 p.m. close and nothing after it |
| 2024-12-25 | closed | `Closed` | `NYSE-HOL-2024` | T1 | event date 2024-12-25; the sheet prints no session for it |

### 2025

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2025-01-01 | closed | `Closed` | `NYSE-HOL-2025` | T1 | event date 2025-01-01; the sheet prints no session for it |
| 2025-01-09 | closed | `Closed` | `NYSE-CARTER-2025` | T1 | event date 2025-01-09; the sheet prints no session for it |
| 2025-01-20 | closed | `Closed` | `NYSE-HOL-2025` | T1 | event date 2025-01-20; the sheet prints no session for it |
| 2025-02-17 | closed | `Closed` | `NYSE-HOL-2025` | T1 | event date 2025-02-17; the sheet prints no session for it |
| 2025-04-18 | closed | `Closed` | `NYSE-HOL-2025` | T1 | event date 2025-04-18; the sheet prints no session for it |
| 2025-05-26 | closed | `Closed` | `NYSE-HOL-2025` | T1 | event date 2025-05-26; the sheet prints no session for it |
| 2025-06-19 | closed | `Closed` | `NYSE-HOL-2025` | T1 | event date 2025-06-19; the sheet prints no session for it |
| 2025-07-03 | early close | `1:00 p.m. ET` | `NYSE-HOL-2025` | T1 | the sheet's own footnote: "Each market will close early at 1:00 p.m. (1:15 p.m. for eligible options) on Thursday, July 3, 2025" — the day before Independence Day, per the July 3 pattern the same footnote family uses across the years |
| 2025-07-04 | closed | `Closed` | `NYSE-HOL-2025` | T1 | event date 2025-07-04; the sheet prints no session for it |
| 2025-09-01 | closed | `Closed` | `NYSE-HOL-2025` | T1 | event date 2025-09-01; the sheet prints no session for it |
| 2025-11-27 | closed | `Closed` | `NYSE-HOL-2025` | T1 | event date 2025-11-27; the sheet prints no session for it |
| 2025-11-28 | early close | `close early at 1:00 p.m.` | `NYSE-HOL-2025` | T1 | event date 2025-11-28; the year footnote prints the 1:00 p.m. close and nothing after it |
| 2025-12-24 | early close | `close early at 1:00 p.m.` | `NYSE-HOL-2025` | T1 | event date 2025-12-24; the year footnote prints the 1:00 p.m. close and nothing after it |
| 2025-12-25 | closed | `Closed` | `NYSE-HOL-2025` | T1 | event date 2025-12-25; the sheet prints no session for it |

### 2026

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2026-01-01 | closed | `Closed` | `NYSE-HOL-2026` | T1 | event date 2026-01-01; the sheet prints no session for it |
| 2026-01-19 | closed | `Closed` | `NYSE-HOL-2026` | T1 | event date 2026-01-19; the sheet prints no session for it |
| 2026-02-16 | closed | `Closed` | `NYSE-HOL-2026` | T1 | event date 2026-02-16; the sheet prints no session for it |
| 2026-04-03 | closed | `Closed` | `NYSE-HOL-2026` | T1 | event date 2026-04-03; the sheet prints no session for it |
| 2026-05-25 | closed | `Closed` | `NYSE-HOL-2026` | T1 | event date 2026-05-25; the sheet prints no session for it |
| 2026-06-19 | closed | `Closed` | `NYSE-HOL-2026` | T1 | event date 2026-06-19; the sheet prints no session for it |
| 2026-07-03 | closed | `Closed` | `NYSE-HOL-2026` | T1 | event date 2026-07-03; the sheet prints no session for it |
| 2026-09-07 | closed | `Closed` | `NYSE-HOL-2026` | T1 | event date 2026-09-07; the sheet prints no session for it |
| 2026-11-26 | closed | `Closed` | `NYSE-HOL-2026` | T1 | event date 2026-11-26; the sheet prints no session for it |
| 2026-11-27 | early close | `close early at 1:00 p.m.` | `NYSE-HOL-2026` | T1 | event date 2026-11-27; the year footnote prints the 1:00 p.m. close and nothing after it |
| 2026-12-24 | early close | `close early at 1:00 p.m.` | `NYSE-HOL-2026` | T1 | event date 2026-12-24; the year footnote prints the 1:00 p.m. close and nothing after it |
| 2026-12-25 | closed | `Closed` | `NYSE-HOL-2026` | T1 | event date 2026-12-25; the sheet prints no session for it |

### 2027

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2027-01-01 | closed | `Closed` | `NYSE-HOL-2026` | T1 | event date 2027-01-01; the sheet prints no session for it |
| 2027-01-18 | closed | `Closed` | `NYSE-HOL-2026` | T1 | event date 2027-01-18; the sheet prints no session for it |
| 2027-02-15 | closed | `Closed` | `NYSE-HOL-2026` | T1 | event date 2027-02-15; the sheet prints no session for it |
| 2027-03-26 | closed | `Closed` | `NYSE-HOL-2026` | T1 | event date 2027-03-26; the sheet prints no session for it |
| 2027-05-31 | closed | `Closed` | `NYSE-HOL-2026` | T1 | event date 2027-05-31; the sheet prints no session for it |
| 2027-06-18 | closed | `Closed` | `NYSE-HOL-2026` | T1 | event date 2027-06-18; the sheet prints no session for it |
| 2027-07-05 | closed | `Closed` | `NYSE-HOL-2026` | T1 | event date 2027-07-05; the sheet prints no session for it |
| 2027-09-06 | closed | `Closed` | `NYSE-HOL-2026` | T1 | event date 2027-09-06; the sheet prints no session for it |
| 2027-11-25 | closed | `Closed` | `NYSE-HOL-2026` | T1 | event date 2027-11-25; the sheet prints no session for it |
| 2027-11-26 | early close | `close early at 1:00 p.m.` | `NYSE-HOL-2026` | T1 | event date 2027-11-26; the year footnote prints the 1:00 p.m. close and nothing after it |
| 2027-12-24 | closed | `Closed` | `NYSE-HOL-2026` | T1 | event date 2027-12-24; the sheet prints no session for it |
### Documents

| Document | Window | Replay or service URL | Capture or retrieval, UTC | Tier | sha256 |
|---|---|---|---|---|---|
| `NYSE-HOL-2010` | 2010-01-01 .. 2011-12-31 | <https://web.archive.org/web/20100208104805id_/https://www.nyse.com/about/newsevents/1176373643795.html> | captured 2010-02-08, retrieved 2026-09-27 | T1 | `63cd364803122479ed9dec3abdfef7b1ebe9c19d6c30ab6ce69db51c50b83607` |
| `NYSE-HOL-2010LATE` | 2010-11-26 .. 2011-12-31 | <https://web.archive.org/web/20101203013357id_/https://www.nyse.com/about/newsevents/1176373643795.html> | captured 2010-12-03, retrieved 2026-09-27 | T1 | `abab3022188dbf14665db5bc94a9c8cd8a9782b44c9bb933f6153e1457751d87` |
| `NYSE-HOL-2011` | 2011-01-01 .. 2013-12-31 | <https://web.archive.org/web/20110702021005id_/https://www.nyse.com/about/newsevents/1176373643795.html> | captured 2011-07-02, retrieved 2026-09-27 | T1 | `4a54a54923e01ce867992cb8328a50f23b3af75437aa7724b3b84c3d932ea5d6` |
| `NYSE-HOL-2014` | 2014-01-01 .. 2015-12-31 | <https://web.archive.org/web/20141124052340id_/https://www.nyse.com/markets/hours-calendars> | captured 2014-11-24, retrieved 2026-09-27 | T1 | `df0829142b81b746d41c084a8b361c4abf239d878b9933537f57aee8671adfc0` |
| `NYSE-HOL-2015` | 2015-01-01 .. 2017-12-31 | <https://web.archive.org/web/20151225081701id_/https://www.nyse.com/markets/hours-calendars> | captured 2015-12-25, retrieved 2026-09-27 | T1 | `0ac69e6e3d4e42ff4c2bae49223b294a7e49d1992dcb2477fb0d8cedc260662f` |
| `NYSE-HOL-2016` | 2016-01-01 .. 2018-12-31 | <https://web.archive.org/web/20160909175058id_/https://www.nyse.com/markets/hours-calendars> | captured 2016-09-09, retrieved 2026-09-27 | T1 | `3d75bd0fd16fb9e54ffc5de2c029d18857fd7df07b75f42801f05aa678f0f0b1` |
| `NYSE-HOL-2017` | 2017-01-01 .. 2019-12-31 | <https://web.archive.org/web/20170902160103id_/https://www.nyse.com/markets/hours-calendars> | captured 2017-09-02, retrieved 2026-09-27 | T1 | `a2b7b8eb6c57408d0cb780c3f48b6b7997c3253726ca349c3b0fb91da90b5f44` |
| `NYSE-HOL-2018` | 2018-01-01 .. 2020-12-31 | <https://web.archive.org/web/20180901004308id_/https://www.nyse.com/markets/hours-calendars> | captured 2018-09-01, retrieved 2026-09-27 | T1 | `dfd6ef772a5e3e816b4e394249b347c2e2c1c833a4d2fbcf5471399c7739a8ed` |
| `NYSE-HOL-2019` | 2019-01-01 .. 2021-12-31 | <https://web.archive.org/web/20190902131814id_/https://www.nyse.com/markets/hours-calendars> | captured 2019-09-02, retrieved 2026-09-27 | T1 | `fffbd78bec84d0cf962acffc8adb205aa2730edd4a70552cd5327783c5501ca1` |
| `NYSE-HOL-2020` | 2020-01-01 .. 2022-12-31 | <https://web.archive.org/web/20200904174658id_/https://www.nyse.com/markets/hours-calendars> | captured 2020-09-04, retrieved 2026-09-27 | T1 | `9ffde1b57bd7fee83b95adfbe4bad77d40f715e39463d3416f5e13a3530dc48a` |
| `NYSE-HOL-2021` | 2021-01-01 .. 2023-12-31 | <https://web.archive.org/web/20210905140537id_/https://www.nyse.com/markets/hours-calendars> | captured 2021-09-05, retrieved 2026-09-27 | T1 | `a59ce59af7259b81582b1627aa8a6d8ced18355fa410e3cb758b0154693e84b3` |
| `NYSE-HOL-2022` | 2022-01-01 .. 2024-12-31 | <https://web.archive.org/web/20220901141553id_/https://www.nyse.com/markets/hours-calendars> | captured 2022-09-01, retrieved 2026-09-27 | T1 | `b766c50b82e21a0f9d70946dcc95f74a7da424ef23b15f7c7ce44a750cc71b0d` |
| `NYSE-HOL-2023` | 2023-01-01 .. 2025-12-31 | <https://web.archive.org/web/20230903115422id_/https://www.nyse.com/markets/hours-calendars> | captured 2023-09-03, retrieved 2026-09-27 | T1 | `cb79b8a8127778306ffb13d167f6662a153d5444a3e27e1e33aeca3335a01035` |
| `NYSE-HOL-2024` | 2024-01-01 .. 2026-12-31 | <https://web.archive.org/web/20240901130644id_/https://www.nyse.com/markets/hours-calendars> | captured 2024-09-01, retrieved 2026-09-27 | T1 | `408f99c7809dd412e00903a98ae5029cb9b74607b9d1773c6a94b1314f22ceea` |
| `NYSE-HOL-2025` | 2025-01-01 .. 2027-12-31 | <https://web.archive.org/web/20250901035035id_/https://www.nyse.com/markets/hours-calendars> | captured 2025-09-01, retrieved 2026-09-27 | T1 | `f5d72690309f62db4ae40a04048dbf5d69ac63095490a9a24efcf3cf7264a7ff` |
| `NYSE-HOL-2026` | 2026-01-01 .. 2028-12-31 | <https://www.nyse.com/trade/hours-calendars> | retrieved 2026-09-27 | T1 | `b7c8c2fa1923de735a0bc29a4697b94957a6cc6ad7e219739282fea871d2de6f` |
| `NYSE-SANDY-2012A` | 2012-10-29 | <https://www.nyse.com/press/1351243418010.html> | captured 2012-11-01, retrieved 2026-09-27 | T1 | `01e6ac79aaa6feaf26273ff2e5b31826b01907243de18aed4d4f6cc67d215751` |
| `NYSE-SANDY-2012B` | 2012-10-30 | <https://www.nyse.com/press/1351243421978.html> | captured 2012-11-02, retrieved 2026-09-27 | T1 | `33e019250494c1935c0ac9a0f900fdadd6c09c7f76d7f55163c763e1205073f4` |
| `NYSE-CARTER-2025` | 2025-01-09 | <https://ir.theice.com/press/news-details/2024/The-New-York-Stock-Exchange-Will-Close-Markets-on-January-9-to-Honor-the-Passing-of-Former-President-Jimmy-Carter-on-National-Day-of-Mourning/default.aspx> | captured 2024-12-30, retrieved 2026-09-27 | T1 | `ea4d74e47babed59481977b0ff5950a189015a099901702cadd72582d667501e` |

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

NYSE accepts orders from 06:30 ET. Tape A queues them for the core opening;
Tapes B/C also enter an active early session at 07:00. The 06:30–07:00 leg is
therefore acceptance only — no trade can print before the Early Trading
Session opens — and is classified `order_entry`; 07:00–09:30 stays Extended
because Tape B/C trades execute there.

The acceptance edge is a **rulebook** provision: NYSE Rule 7.34(a)(1), adopted
with the UTP Pillar filing (SR-NYSE-2017-36). Two later NYSE filings state the
production day unconditionally — "On April 9, 2018, the Exchange began trading
UTP Securities on the Exchange on the Pillar trading platform". Before
2018-04-09 the modelled grid stays the sourced 09:30–16:00 core session; the
pre-Pillar phases (Crossing Session II, the pre-2018 Tape A acceptance edge)
remain unmodelled and are recorded as the row's residual gaps in
`docs/schedules/verification.md`.

The announced 23-hour Overnight Session (from Sunday 2026-12-06) is a future
change tracked in `docs/schedules/updating.md`, not encoded here. The
normal-week sources below are unchanged by the holiday table; the holiday
corpus lives under `## Holidays` above.
- <https://www.nyse.com/trade/hours-calendars> — the operator's holiday calendar and phase table (live, states 2026-2028); the 2014-2025 era lives at `nyse.com/markets/hours-calendars` and the 2007-2013 page at `nyse.com/about/newsevents/1176373643795.html` (Wayback replays, digests in the `### Documents` tables).
- <https://ir.theice.com/press/news-details/2024/The-New-York-Stock-Exchange-Will-Close-Markets-on-January-9-to-Honor-the-Passing-of-Former-President-Jimmy-Carter-on-National-Day-of-Mourning/default.aspx> — the National Day of Mourning release naming the New York Stock Exchange (2024-12-30).
- NYSE Euronext press releases 1351243418010 (2012-10-28) and 1351243421978 (2012-10-29) — the Hurricane Sandy closure statements, read through the web archive.

