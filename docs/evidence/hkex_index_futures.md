<!-- SPDX-License-Identifier: MIT-0 -->

# `hkex_index_futures`

HKEX index futures — Hang Seng Index (HSI) futures and the H-shares (HHI)
suite, `Asia/Hong_Kong`.

- **Kind:** `MarketHoursKey`
- **Admission:** served — the consumer's OANDA activation maps HK33 and the
  H-share CFDs to this family (SharurPlatform), so a consumer instrument
  reaches the key.

## Ledger basis (moved from docs/schedules/verification.md on 2026-10-14 UTC)

The verification row's basis, horizon and cadence live in
[the ledger](../schedules/verification.md); this file holds the quotations,
interpretive steps and residual risks behind the row. The dated timeline has
three operator-dated cutovers and three bracketed-undated ones; the holiday
table is the operator's own holiday pages and trading calendars 2010–2027,
converted to the crate's trade dates.

### Normal week

- Floor (to 2011-03-06): "Trading Hours 9:45 am - 12:30 pm & 2:30 pm - 4:15
  pm" (HSI futures contract summary, capture 2010-05).
- Phase 1 (2011-03-07): "Pre-Market Opening Period 8:45 am - 9:15 am & 1:00 pm
  - 1:30 pm … Trading Hours 9:15 am - 12:00 noon & 1:30 pm - 4:15 pm", on the
  contract page the operator stamped "Updated: 07/03/2011"; the same page
  prints Phase 2 as "(Effective 5 March 2012)". The page's own update stamp is
  the operator's machine channel dating the Phase 1 grid; the two 2011
  circulars behind it (DMD/006/11 of 24 January 2011, DMD/023/11 of 24
  February 2011, named by DMD/004/12) are not captured in the store.
- Phase 2 (2012-03-05): DMD/004/12 (9 January 2012): "Phase 2 extension of
  trading hours of the Stock Futures, Index Futures & Options markets will be
  implemented on 5 March 2012", with the index-futures table "Pre-market
  Opening Period … 12:30-13:00 … Afternoon Session … 13:00-16:15".
- T+1 launch (2013-04-08): DMD/022/13 (14 February 2013): "the launch date for
  AHFT will be on Monday, 8 April 2013", HSI and HHI futures first. The
  information paper states the shape: "the T+1 Session will open at 5:00 p.m.
  and end at 11:00 p.m." and — the trade-date rule this key serves on —
  "Trades executed during the T+1 Session (T+1 Trades) will be registered as
  the following day's trades." The holiday rule is the circular's own:
  "If it is a Hong Kong public holiday or there is half-day trading in the
  HKEx securities market or it is a bank holiday in both the UK and United
  States, there would be no T+1 Session on that day."
- The three bracketed-undated cutovers, each serving the sourced intersection
  across its uncaptured span and keyed to the local opening day of the first
  session its later state governs:
  - T+1 close 11:00pm → 11:45pm: the 23:00 grid is the launch shape (witnessed
    by the page updated 10/11/2013, captured 2014-09); the 11:45pm grid prints
    on the page stamped "Updated: 03/11/2014" (captured 2015-07). Changeover
    day uncaptured. The 2014 witness still prints the 5:00pm T+1 start, so the
    crate holds the close at the witnessed 23:00 until that page's date and
    serves 5:15pm-11:45pm from it — the window that holds under both that
    state and the 2016 one (see the next cutover).
  - Day close 4:15pm → 4:30pm and T+1 start 5:00pm → 5:15pm: the
    "Updated: 22/07/2016" page (captured 2016-09) prints "9:15 am - 12:00
    noon, 1:00 pm - 4:30 pm & 5:15 pm - 11:45 pm". Uncaptured circular. The
    day close holds at its narrowest sourced value 4:15pm until the 2016
    witness, and the T+1 start holds at 5:15pm from the 2014 witness — across
    the whole undated span only the intersection (day to 4:15pm, T+1
    5:15pm-11:45pm) holds under both flanking states.
  - T+1 5:15pm-11:45pm → 5:00pm-3:00am: the live derivatives trading-hours
    page (retrieved 2026-10-11; "Updated 20 Jul 2026") and the HSI product
    page print "5:00 p.m. - 03:00 a.m. the next day". The real changeover
    (2017) is uncaptured; the row is keyed to 2026-10-12 — the local opening
    day of the first session the later state governs, the Monday-evening leg —
    so the prior Friday's leg answers entirely under the state in force when
    it opened, and the whole span between the 2016 and 2026 witnesses serves
    the sourced intersection (5:15pm-11:45pm), with the remainder disclosed
    under Gaps.

### Half-day and holiday conventions

- The current trading-hours page states the half-day close for this family:
  "Closing time of Stock Index Futures and Options, Dividend Futures and HSI
  Volatility Index Futures for half-day trading will be 12:30p.m."
- The holiday pages state the eve rule per year: "There will be no afternoon
  trading session and after-hours trading session on the following business
  day[s]:" followed by the eve dates (Lunar New Year eve, Christmas Eve, New
  Year's Eve) — the listed date is the half day.
- The holiday pages 2017–2025 state the UK/US rule with its dates: "On the
  following trading day, there will be no after-hours trading session for
  Hang Seng Index Futures and Options …: <date> – Bank holiday in both the
  United Kingdom and the United States" — the listed date is the suspended
  AHT evening. The current (2026/2027) page's HSI note drops the UK/US list,
  so no suspension rows ship from 2026.
- Trade dates: the T+1 session's trades are "registered as the following day's
  trades" (the AHFT information paper; the circular states it as "registered
  as the following business day"), which is the crate's final-close
  convention. Through the
  same-day 11:00pm/11:45pm eras every session's trade date is its own civil
  day, so a holiday's `Closed` row removes that day's AHT with the day itself;
  in the wrapped 03:00 era (served from 2026-10-12) a holiday evening's leg
  belongs to the following trade date and is removed by a late-open row (or by
  a Saturday closure, whose trade date holds only that leg).

## Revision rows

- 2011-03-07 — T2 — HKEX-HSIF-PAGE-2011-06 — Phase 1: 9:15am-12:00 noon and
  1:30-4:15pm with 8:45-9:15am and 1:00-1:30pm pre-opening periods; keyed to
  the operator's own "Updated: 07/03/2011" page stamp.
- 2012-03-05 — T1 — HKEX-DMD-004-12 — Phase 2: afternoon 1:00-4:15pm with the
  pre-opening period moved to 12:30-1:00pm.
- 2013-04-08 — T1 — HKEX-DMD-022-13 — the T+1 session launches 5:00-11:00pm
  for HSI and HHI futures.
- 2014-11-03 — T2 — HKEX-HSIF-PAGE-2015-07 — T+1 5:15pm-11:45pm, the sourced
  intersection of the 2014 and 2016 witnesses: the page's own grid prints the
  11:45pm close with the 5:00pm start, and the start is held at the narrowest
  sourced value 5:15pm across the undated span (a knowledge boundary never
  granted early); keyed to the snapshot's "Updated: 03/11/2014" stamp, onset
  undated.
- 2016-07-22 — T2 — HKEX-HSIF-PAGE-2016-09 — day close 4:30pm against the
  unchanged T+1 5:15pm-11:45pm; keyed to the snapshot's "Updated: 22/07/2016"
  stamp, onset undated.
- 2026-10-12 — T2 — HKEX-HOURS-DERIV-CURRENT — the overnight T+1 shape
  5:00pm-3:00am; keyed to the local opening day of the first session the
  later state governs (the Monday-evening leg after the page's 2026-10-11
  retrieval), onset undated.

## Sources

- The live derivatives trading-hours page and HSI product page (retrieved
  2026-10-11), the derivatives Trading Calendar and Holiday Schedule pages
  (captures 2017–2025 plus the current 2026+2027 state), the HSI futures
  contract summaries 2010–2016 (each carrying the operator's own "Updated"
  field), the circulars DMD/004/12 and DMD/022/13, the AHFT information paper
  and FAQ, and the `tradcal` wall-calendar PDFs 2010–2017 (ids shared with the cash-market evidence) — all in
  `normal-weeks/hkex-futures-2026-10-11/` and `holidays/raw/hkex-futures-2010-2027/`
  of the research store (`SharurTrading/exchange-hours-evidence`, store PR #16,
  digests in `SHA256SUMS.txt`).

## Gaps and residual risks

- **The bracketed AHT cutovers (tracked in the crate's #332).** The changeover
  days of the 11:00pm→11:45pm close, the 4:15→4:30pm day close with the
  5:00→5:15pm T+1 start, and the 5:15pm-11:45pm→5:00pm-3:00am overnight shape
  are uncaptured: every hunt closed negative on the operator's circular
  archive. Each undated span therefore serves the sourced intersection — the
  window that holds under both flanking states — and the residuals are
  per-span: below the 2014 witness the T+1 runs 5:00-11:00pm wherever the real
  11:45pm changeover preceded the witness; across the 2014-2016 span the
  served 5:15pm start under-reports the 5:00-5:15pm slice wherever the real
  2016 changeover preceded the 22/07/2016 page (and the 5:15pm hold
  under-reports it early in the span where the 2014 page's 5:00pm start may
  still have been in force); and across the 2016-2026 span the served
  5:15pm-11:45pm window under-reports the overnight session wherever the real
  changeover preceded the witness. Closers: the operator's dated circular for
  each change, or contract-page captures inside the spans.
- **2013–2016 UK/US AHT suspensions (tracked in the crate's #332).** The
  circular's holiday rule (no T+1 on a bank holiday in both the UK and the
  United States) is sourced, but the operator's per-year notes listing the
  affected dates are captured only from 2017; the 2013–2016 spring
  bank-holiday/Memorial-Day evenings ship no row and answer from the normal
  week, and the era is declared in `schedules/sourcing.rs` as unpublished
  closure dates (#332). Closer: the 2013–2016 holiday pages.
- **Half-day close instant before the current page.** The 12:30 p.m. half-day
  close is stated by the current trading-hours page; the 2010–2016 artifacts
  state "half-day trading day" (tradcal notes) without an instant. In the eras
  whose morning session ended at 12:00 noon the scalar clip serves the
  morning's own 12:00 close and discloses the possible 12:30 close; the 2010
  floor grid's morning already ended 12:30, so those rows are exact.
- **2017/2018 table provenance.** The two archived 2017/2018 holiday pages in
  the store are byte-distinct captures of the same table state; the 2017 rows
  cite the `tradcal-2017` wall calendar, the 2018 rows the archived page.
- Lunar New Year days trade from 2026 (the current page lists them "Trading
  Day (with trading)"); no rows ship for them, and the 2026/2027 LNY eves
  carry the half-day arrangement.

## Holidays

**Coverage:** 2010-01-01..2027-12-31, 2028-01-01..2028-01-01 (derivatives
Trading Calendar wall PDFs 2010–2017 and the Trading Calendar and Holiday
Schedule pages 2017–2027; every date inside the window was audited from its
year's list, and a date without a row is audited normal. The one-day 2028
window covers the final New Year evening leg, whose trade date the 2027
table's eve row removes.)

### Documents

| Document | Window | Replay or service URL | Capture or retrieval, UTC | Tier | sha256 |
|---|---|---|---|---|---|
| `HKEX-DMD-004-12` | circular DMD/004/12, 9 Jan 2012 (Phase 2 on 5 March 2012) | <https://www.hkex.com.hk/Services/Rules-and-Forms-and-Fees/Circulars-and-Notices> | retrieved 2026-10-11 UTC | T1 | `57822efb01ad7f010dad42bd58d223251e2a915ca1727f7a7c96b9e850928dd0` |
| `HKEX-DMD-022-13` | circular DMD/022/13, 14 Feb 2013 (AHFT launch 8 April 2013) | <https://www.hkex.com.hk/Services/Rules-and-Forms-and-Fees/Circulars-and-Notices> | retrieved 2026-10-11 UTC | T1 | `88982ee895a7303221f5ad1a91f174440dd313e66b581bf96aff91cf7b29945b` |
| `HKEX-HSIF-PAGE-2010-05` | HSI futures contract summary (floor grid) | <https://www.hkex.com.hk/eng/prod/drprod/hsif/Contract_Summary.htm> | retrieved 2026-10-11 UTC | T2 | `e4ad85abbee96e4693029baa2a716105da3025ae319839e2474c97c041f93817` |
| `HKEX-HSIF-PAGE-2011-06` | HSI futures contract summary, Updated 07/03/2011 (Phase 1 + Phase 2 effective day) | <https://www.hkex.com.hk/eng/prod/drprod/hsif/Contract_Summary.htm> | retrieved 2026-10-11 UTC | T2 | `1ce7b1d10bcbc3c96fca8cb73f71aff6c993b35ea2408fe671f6bf187049afbf` |
| `HKEX-HSIF-PAGE-2015-07` | HSI futures contract summary, Updated 03/11/2014 (T+1 11:45pm) | <https://www.hkex.com.hk/eng/prod/drprod/hsif/Contract_Summary.htm> | retrieved 2026-10-11 UTC | T2 | `14e792c207caf907301d4d9ae48c3e61dbde90f59b6252d05eac430943a4971c` |
| `HKEX-HSIF-PAGE-2016-09` | HSI futures contract summary, Updated 22/07/2016 (4:30pm + T+1 5:15-11:45pm) | <https://www.hkex.com.hk/eng/prod/drprod/hsif/Contract_Summary.htm> | retrieved 2026-10-11 UTC | T2 | `283bc30a3281f376c6b30fa96aee85470e783ed415f24b7815df166cae9ca7e2` |
| `HKEX-HOURS-DERIV-CURRENT` | derivatives trading-hours page, Updated 20/31 Jul 2026 (current grids, 12:30pm half-day close) | <https://www.hkex.com.hk/Services/Trading-hours-and-Severe-Weather-Arrangements/Trading-Hours/Derivatives-Market> | retrieved 2026-10-11 UTC | T2 | `6be2690a910de4fdc179245f120c9ce4df8a1c2336757390748f74e7282d7706` |
| `HKEX-TC-2010` | 2010-01-01 .. 2010-12-31 | <https://web.archive.org/web/20100331081510id_/http://www.hkex.com.hk/eng/market/sec_tradinfo/tradcal/documents/2010cal.pdf> (capture `20100331081510`) | Wayback `id_` replay of capture `20100331081510`, retrieved 2026-09-29 02:44:32 UTC | T1 | `84cb12d8c3343f5070fa53297a4ee10773993d6e06fd9e09b09b67f977a2f005` |
| `HKEX-TC-2011` | 2011-01-01 .. 2011-12-31 | <https://web.archive.org/web/20101202113436id_/http://www.hkex.com.hk/eng/market/sec_tradinfo/tradcal/documents/2011cal.pdf> (capture `20101202113436`) | Wayback `id_` replay of capture `20101202113436`, retrieved 2026-09-29 02:44:34 UTC | T1 | `fec9dc8da4926f0fd52e3c4fdee85e78da74619176c3ccf3e01ed7e6989ccf5c` |
| `HKEX-TC-2012` | 2012-01-01 .. 2012-12-31 | <https://web.archive.org/web/20111111202741id_/http://www.hkex.com.hk/eng/market/sec_tradinfo/tradcal/Documents/2012cal.pdf> (capture `20111111202741`) | Wayback `id_` replay of capture `20111111202741`, retrieved 2026-09-29 02:44:35 UTC | T1 | `ebd2e24c8d3eca71f7648fb2f8bd838500db777a6ce2a28c52008a3aec85bdef` |
| `HKEX-TC-2013` | 2013-01-01 .. 2013-12-31 | <https://web.archive.org/web/20121118194251id_/http://www.hkex.com.hk/eng/market/sec_tradinfo/tradcal/Documents/2013cal.pdf> (capture `20121118194251`) | Wayback `id_` replay of capture `20121118194251`, retrieved 2026-09-29 02:44:36 UTC | T1 | `c451bbf254a0cf51c5b6a252591003305b7129e9212b1e4a3d1eec8c4b5dda40` |
| `HKEX-TC-2014` | 2014-01-01 .. 2014-12-31 | <https://web.archive.org/web/20140124041841id_/http://www.hkex.com.hk/eng/market/sec_tradinfo/tradcal/Documents/2014cal.pdf> (capture `20140124041841`) | Wayback `id_` replay of capture `20140124041841`, retrieved 2026-09-29 02:44:39 UTC | T1 | `9339d812849a5fe3392f6beb4fc33a577c268d57cfef37a74934e2c9beaf2929` |
| `HKEX-TC-2015` | 2015-01-01 .. 2015-12-31 | <https://web.archive.org/web/20141222040839id_/http://www.hkex.com.hk/eng/market/sec_tradinfo/tradcal/Documents/2015cal.pdf> (capture `20141222040839`) | Wayback `id_` replay of capture `20141222040839`, retrieved 2026-09-29 02:44:41 UTC | T1 | `98a08cc02838c74531787ee563dd841e7c6ea01f0dc7b1b5c797c7103efe2887` |
| `HKEX-TC-2016` | 2016-01-01 .. 2016-12-31 | <https://web.archive.org/web/20160122222039id_/http://www.hkex.com.hk/eng/market/sec_tradinfo/tradcal/Documents/2016cal.pdf> (capture `20160122222039`) | Wayback `id_` replay of capture `20160122222039`, retrieved 2026-09-29 02:44:44 UTC | T1 | `53bc04b1464f780a435ff3a60c35de88f4f247ab78cbfe98a6e35a94903fad23` |
| `HKEX-TC-2017` | 2017-01-01 .. 2017-12-31 | <https://web.archive.org/web/20161213062357id_/http://www.hkex.com.hk/eng/market/sec_tradinfo/tradcal/Documents/2017calendar.pdf> (capture `20161213062357`) | Wayback `id_` replay of capture `20161213062357`, retrieved 2026-09-29 02:44:45 UTC | T1 | `5b2f374af743d39940a454e81a40d4641918859fe8a4b482a9b7e6bde8154110` |
| `HKEX-HOL-2018` | Trading Calendar and Holiday Schedule page (2017+2018 tables) | <https://www.hkex.com.hk/Services/Trading/Derivatives/Overview/Trading-Calendar-and-Holiday-Schedule?sc_lang=en> | retrieved 2026-10-11 UTC | T1 | `c910b4e007103985eab361607b7c31d817f378028396a8e91e7b14d3ebeca855` |
| `HKEX-TC-PAGE-2019` | 2019-01-01 .. 2020-12-31 | <https://web.archive.org/web/20190921174735id_/https://www.hkex.com.hk/Services/Trading/Derivatives/Overview/Trading-Calendar-and-Holiday-Schedule?sc_lang=en> (capture `20190921174735`, page footer "Updated 11 Jun 2019") | Wayback `id_` replay of capture `20190921174735`, retrieved 2026-09-29 UTC | T1 | `6c7c2403f059218d458fa554413ed18701da09d84677d604ab3e633e522ecfdf` |
| `HKEX-HOL-2020` | Trading Calendar and Holiday Schedule page (2019+2020 tables, Sep 2020 update) | <https://www.hkex.com.hk/Services/Trading/Derivatives/Overview/Trading-Calendar-and-Holiday-Schedule?sc_lang=en> | retrieved 2026-10-11 UTC | T1 | `d1cc92aed11396af27274743c3ece977f5b6134129a0c53e31fda7c25da2a33d` |
| `HKEX-TC-PAGE-2020` | 2020-01-01 .. 2021-12-31 | <https://web.archive.org/web/20210123020124id_/https://www.hkex.com.hk/Services/Trading/Derivatives/Overview/Trading-Calendar-and-Holiday-Schedule?sc_lang=en> (capture `20210123020124`, page footer "Updated 15 Jan 2021") | Wayback `id_` replay of capture `20210123020124`, retrieved 2026-09-29 UTC | T1 | `08a3609cab6cea4ce63d70b3a8ffb808933d3b327094ca90527161fa8cbf98ab` |
| `HKEX-HOL-2022` | Trading Calendar and Holiday Schedule page (2021+2022 tables) | <https://www.hkex.com.hk/Services/Trading/Derivatives/Overview/Trading-Calendar-and-Holiday-Schedule?sc_lang=en> | retrieved 2026-10-11 UTC | T1 | `eeb5d75de16521c4ad3185672d0ba72fca6a297b332f3433685f0e932aef30a7` |
| `HKEX-HOL-2023` | Trading Calendar and Holiday Schedule page (2022+2023 tables) | <https://www.hkex.com.hk/Services/Trading/Derivatives/Overview/Trading-Calendar-and-Holiday-Schedule?sc_lang=en> | retrieved 2026-10-11 UTC | T1 | `59eaeafaeefed60a683c7112da97198da5a16c47227d0259b00ee1cdc4d5ced1` |
| `HKEX-HOL-2024` | Trading Calendar and Holiday Schedule page (2023+2024 tables) | <https://www.hkex.com.hk/Services/Trading/Derivatives/Overview/Trading-Calendar-and-Holiday-Schedule?sc_lang=en> | retrieved 2026-10-11 UTC | T1 | `26a76e697797dd368cf8c85e05e1977d1883983934a2c58987bce7f342532039` |
| `HKEX-HOL-2025` | Trading Calendar and Holiday Schedule page (2024+2025 tables) | <https://www.hkex.com.hk/Services/Trading/Derivatives/Overview/Trading-Calendar-and-Holiday-Schedule?sc_lang=en> | retrieved 2026-10-11 UTC | T1 | `3366855bdcd0053a796ef77113867f046ab8f050ff323752eb61ae8ba49308d4` |
| `HKEX-HOL-CURRENT` | Trading Calendar and Holiday Schedule page (2026+2027 tables, Updated 31 Jul 2026) | <https://www.hkex.com.hk/Services/Trading/Derivatives/Overview/Trading-Calendar-and-Holiday-Schedule?sc_lang=en> | retrieved 2026-10-11 UTC | T1 | `ca151ca34820823c4d4087cedb329cc52dad0d9e09b09ba80dfcdeca201d4e40` |

The holiday-page and contract-page artifacts are archived captures of the
operator pages (each page's own "Updated" field is quoted in the prose where
it keys a reading); the store holds the saved bytes and their digests, and
every cell above records the retrieval that saved them.

### 2010

| 2010-01-01 | closed | — | `HKEX-TC-2010` | T1 | operator Holiday (no trading) |
| 2010-02-15 | closed | — | `HKEX-TC-2010` | T1 | operator Holiday (no trading) |
| 2010-02-16 | closed | — | `HKEX-TC-2010` | T1 | operator Holiday (no trading) |
| 2010-04-02 | closed | — | `HKEX-TC-2010` | T1 | operator Holiday (no trading) |
| 2010-04-05 | closed | — | `HKEX-TC-2010` | T1 | operator Holiday (no trading) |
| 2010-04-06 | closed | — | `HKEX-TC-2010` | T1 | operator Holiday (no trading) |
| 2010-05-21 | closed | — | `HKEX-TC-2010` | T1 | operator Holiday (no trading) |
| 2010-06-16 | closed | — | `HKEX-TC-2010` | T1 | operator Holiday (no trading) |
| 2010-07-01 | closed | — | `HKEX-TC-2010` | T1 | operator Holiday (no trading) |
| 2010-09-23 | closed | — | `HKEX-TC-2010` | T1 | operator Holiday (no trading) |
| 2010-10-01 | closed | — | `HKEX-TC-2010` | T1 | operator Holiday (no trading) |
| 2010-12-24 | early close | 12:30 | `HKEX-TC-2010` | T1 | the half-day eve: no afternoon session (and, from 2013, no AHT); the scalar clip keeps the morning and removes everything after 12:30 p.m. |
| 2010-12-27 | closed | — | `HKEX-TC-2010` | T1 | operator Holiday (no trading) |
| 2010-12-31 | early close | 12:30 | `HKEX-TC-2010` | T1 | the half-day eve: no afternoon session (and, from 2013, no AHT); the scalar clip keeps the morning and removes everything after 12:30 p.m. |

### 2011

| 2011-01-03 | closed | — | `HKEX-TC-2011` | T1 | operator Holiday (no trading) |
| 2011-02-02 | early close | 12:30 | `HKEX-TC-2011` | T1 | the half-day eve: no afternoon session (and, from 2013, no AHT); the scalar clip keeps the morning and removes everything after 12:30 p.m. |
| 2011-02-03 | closed | — | `HKEX-TC-2011` | T1 | operator Holiday (no trading) |
| 2011-02-04 | closed | — | `HKEX-TC-2011` | T1 | operator Holiday (no trading) |
| 2011-04-05 | closed | — | `HKEX-TC-2011` | T1 | operator Holiday (no trading) |
| 2011-04-22 | closed | — | `HKEX-TC-2011` | T1 | operator Holiday (no trading) |
| 2011-04-25 | closed | — | `HKEX-TC-2011` | T1 | operator Holiday (no trading) |
| 2011-05-02 | closed | — | `HKEX-TC-2011` | T1 | operator Holiday (no trading) |
| 2011-05-10 | closed | — | `HKEX-TC-2011` | T1 | operator Holiday (no trading) |
| 2011-06-06 | closed | — | `HKEX-TC-2011` | T1 | operator Holiday (no trading) |
| 2011-07-01 | closed | — | `HKEX-TC-2011` | T1 | operator Holiday (no trading) |
| 2011-09-13 | closed | — | `HKEX-TC-2011` | T1 | operator Holiday (no trading) |
| 2011-10-05 | closed | — | `HKEX-TC-2011` | T1 | operator Holiday (no trading) |
| 2011-12-26 | closed | — | `HKEX-TC-2011` | T1 | operator Holiday (no trading) |
| 2011-12-27 | closed | — | `HKEX-TC-2011` | T1 | operator Holiday (no trading) |

### 2012

| 2012-01-02 | closed | — | `HKEX-TC-2012` | T1 | operator Holiday (no trading) |
| 2012-01-23 | closed | — | `HKEX-TC-2012` | T1 | operator Holiday (no trading) |
| 2012-01-24 | closed | — | `HKEX-TC-2012` | T1 | operator Holiday (no trading) |
| 2012-01-25 | closed | — | `HKEX-TC-2012` | T1 | operator Holiday (no trading) |
| 2012-04-04 | closed | — | `HKEX-TC-2012` | T1 | operator Holiday (no trading) |
| 2012-04-06 | closed | — | `HKEX-TC-2012` | T1 | operator Holiday (no trading) |
| 2012-04-09 | closed | — | `HKEX-TC-2012` | T1 | operator Holiday (no trading) |
| 2012-05-01 | closed | — | `HKEX-TC-2012` | T1 | operator Holiday (no trading) |
| 2012-07-02 | closed | — | `HKEX-TC-2012` | T1 | operator Holiday (no trading) |
| 2012-10-01 | closed | — | `HKEX-TC-2012` | T1 | operator Holiday (no trading) |
| 2012-10-02 | closed | — | `HKEX-TC-2012` | T1 | operator Holiday (no trading) |
| 2012-10-23 | closed | — | `HKEX-TC-2012` | T1 | operator Holiday (no trading) |
| 2012-12-24 | early close | 12:30 | `HKEX-TC-2012` | T1 | the half-day eve: no afternoon session (and, from 2013, no AHT); the scalar clip keeps the morning and removes everything after 12:30 p.m. |
| 2012-12-25 | closed | — | `HKEX-TC-2012` | T1 | operator Holiday (no trading) |
| 2012-12-26 | closed | — | `HKEX-TC-2012` | T1 | operator Holiday (no trading) |
| 2012-12-31 | early close | 12:30 | `HKEX-TC-2012` | T1 | the half-day eve: no afternoon session (and, from 2013, no AHT); the scalar clip keeps the morning and removes everything after 12:30 p.m. |

### 2013

| 2013-01-01 | closed | — | `HKEX-TC-2013` | T1 | operator Holiday (no trading) |
| 2013-02-11 | closed | — | `HKEX-TC-2013` | T1 | operator Holiday (no trading) |
| 2013-02-12 | closed | — | `HKEX-TC-2013` | T1 | operator Holiday (no trading) |
| 2013-02-13 | closed | — | `HKEX-TC-2013` | T1 | operator Holiday (no trading) |
| 2013-03-29 | closed | — | `HKEX-TC-2013` | T1 | operator Holiday (no trading) |
| 2013-04-01 | closed | — | `HKEX-TC-2013` | T1 | operator Holiday (no trading) |
| 2013-04-04 | closed | — | `HKEX-TC-2013` | T1 | operator Holiday (no trading) |
| 2013-05-01 | closed | — | `HKEX-TC-2013` | T1 | operator Holiday (no trading) |
| 2013-05-17 | closed | — | `HKEX-TC-2013` | T1 | operator Holiday (no trading) |
| 2013-06-12 | closed | — | `HKEX-TC-2013` | T1 | operator Holiday (no trading) |
| 2013-07-01 | closed | — | `HKEX-TC-2013` | T1 | operator Holiday (no trading) |
| 2013-09-20 | closed | — | `HKEX-TC-2013` | T1 | operator Holiday (no trading) |
| 2013-10-14 | closed | — | `HKEX-TC-2013` | T1 | operator Holiday (no trading) |
| 2013-12-24 | early close | 12:30 | `HKEX-TC-2013` | T1 | the half-day eve: no afternoon session (and, from 2013, no AHT); the scalar clip keeps the morning and removes everything after 12:30 p.m. |
| 2013-12-25 | closed | — | `HKEX-TC-2013` | T1 | operator Holiday (no trading) |
| 2013-12-26 | closed | — | `HKEX-TC-2013` | T1 | operator Holiday (no trading) |
| 2013-12-31 | early close | 12:30 | `HKEX-TC-2013` | T1 | the half-day eve: no afternoon session (and, from 2013, no AHT); the scalar clip keeps the morning and removes everything after 12:30 p.m. |

### 2014

| 2014-01-01 | closed | — | `HKEX-TC-2014` | T1 | operator Holiday (no trading) |
| 2014-01-30 | early close | 12:30 | `HKEX-TC-2014` | T1 | the half-day eve: no afternoon session (and, from 2013, no AHT); the scalar clip keeps the morning and removes everything after 12:30 p.m. |
| 2014-01-31 | closed | — | `HKEX-TC-2014` | T1 | operator Holiday (no trading) |
| 2014-02-03 | closed | — | `HKEX-TC-2014` | T1 | operator Holiday (no trading) |
| 2014-04-18 | closed | — | `HKEX-TC-2014` | T1 | operator Holiday (no trading) |
| 2014-04-21 | closed | — | `HKEX-TC-2014` | T1 | operator Holiday (no trading) |
| 2014-05-01 | closed | — | `HKEX-TC-2014` | T1 | operator Holiday (no trading) |
| 2014-05-06 | closed | — | `HKEX-TC-2014` | T1 | operator Holiday (no trading) |
| 2014-06-02 | closed | — | `HKEX-TC-2014` | T1 | operator Holiday (no trading) |
| 2014-07-01 | closed | — | `HKEX-TC-2014` | T1 | operator Holiday (no trading) |
| 2014-09-09 | closed | — | `HKEX-TC-2014` | T1 | operator Holiday (no trading) |
| 2014-10-01 | closed | — | `HKEX-TC-2014` | T1 | operator Holiday (no trading) |
| 2014-10-02 | closed | — | `HKEX-TC-2014` | T1 | operator Holiday (no trading) |
| 2014-12-24 | early close | 12:30 | `HKEX-TC-2014` | T1 | the half-day eve: no afternoon session (and, from 2013, no AHT); the scalar clip keeps the morning and removes everything after 12:30 p.m. |
| 2014-12-25 | closed | — | `HKEX-TC-2014` | T1 | operator Holiday (no trading) |
| 2014-12-26 | closed | — | `HKEX-TC-2014` | T1 | operator Holiday (no trading) |
| 2014-12-31 | early close | 12:30 | `HKEX-TC-2014` | T1 | the half-day eve: no afternoon session (and, from 2013, no AHT); the scalar clip keeps the morning and removes everything after 12:30 p.m. |

### 2015

| 2015-01-01 | closed | — | `HKEX-TC-2015` | T1 | operator Holiday (no trading) |
| 2015-02-18 | early close | 12:30 | `HKEX-TC-2015` | T1 | the half-day eve: no afternoon session (and, from 2013, no AHT); the scalar clip keeps the morning and removes everything after 12:30 p.m. |
| 2015-02-19 | closed | — | `HKEX-TC-2015` | T1 | operator Holiday (no trading) |
| 2015-02-20 | closed | — | `HKEX-TC-2015` | T1 | operator Holiday (no trading) |
| 2015-04-03 | closed | — | `HKEX-TC-2015` | T1 | operator Holiday (no trading) |
| 2015-04-06 | closed | — | `HKEX-TC-2015` | T1 | operator Holiday (no trading) |
| 2015-04-07 | closed | — | `HKEX-TC-2015` | T1 | operator Holiday (no trading) |
| 2015-05-01 | closed | — | `HKEX-TC-2015` | T1 | operator Holiday (no trading) |
| 2015-05-25 | closed | — | `HKEX-TC-2015` | T1 | operator Holiday (no trading) |
| 2015-07-01 | closed | — | `HKEX-TC-2015` | T1 | operator Holiday (no trading) |
| 2015-09-28 | closed | — | `HKEX-TC-2015` | T1 | operator Holiday (no trading) |
| 2015-10-21 | closed | — | `HKEX-TC-2015` | T1 | operator Holiday (no trading) |
| 2015-12-24 | early close | 12:30 | `HKEX-TC-2015` | T1 | the half-day eve: no afternoon session (and, from 2013, no AHT); the scalar clip keeps the morning and removes everything after 12:30 p.m. |
| 2015-12-25 | closed | — | `HKEX-TC-2015` | T1 | operator Holiday (no trading) |
| 2015-12-31 | early close | 12:30 | `HKEX-TC-2015` | T1 | the half-day eve: no afternoon session (and, from 2013, no AHT); the scalar clip keeps the morning and removes everything after 12:30 p.m. |

### 2016

| 2016-01-01 | closed | — | `HKEX-TC-2016` | T1 | operator Holiday (no trading) |
| 2016-02-08 | closed | — | `HKEX-TC-2016` | T1 | operator Holiday (no trading) |
| 2016-02-09 | closed | — | `HKEX-TC-2016` | T1 | operator Holiday (no trading) |
| 2016-02-10 | closed | — | `HKEX-TC-2016` | T1 | operator Holiday (no trading) |
| 2016-03-25 | closed | — | `HKEX-TC-2016` | T1 | operator Holiday (no trading) |
| 2016-03-28 | closed | — | `HKEX-TC-2016` | T1 | operator Holiday (no trading) |
| 2016-04-04 | closed | — | `HKEX-TC-2016` | T1 | operator Holiday (no trading) |
| 2016-05-02 | closed | — | `HKEX-TC-2016` | T1 | operator Holiday (no trading) |
| 2016-06-09 | closed | — | `HKEX-TC-2016` | T1 | operator Holiday (no trading) |
| 2016-07-01 | closed | — | `HKEX-TC-2016` | T1 | operator Holiday (no trading) |
| 2016-09-16 | closed | — | `HKEX-TC-2016` | T1 | operator Holiday (no trading) |
| 2016-10-10 | closed | — | `HKEX-TC-2016` | T1 | operator Holiday (no trading) |
| 2016-12-26 | closed | — | `HKEX-TC-2016` | T1 | operator Holiday (no trading) |
| 2016-12-27 | closed | — | `HKEX-TC-2016` | T1 | operator Holiday (no trading) |

### 2017

| 2017-01-02 | closed | — | `HKEX-TC-2017` | T1 | operator Holiday (no trading) |
| 2017-01-27 | early close | 12:30 | `HKEX-TC-2017` | T1 | the half-day eve: no afternoon session (and, from 2013, no AHT); the scalar clip keeps the morning and removes everything after 12:30 p.m. |
| 2017-01-30 | closed | — | `HKEX-TC-2017` | T1 | operator Holiday (no trading) |
| 2017-01-31 | closed | — | `HKEX-TC-2017` | T1 | operator Holiday (no trading) |
| 2017-04-04 | closed | — | `HKEX-TC-2017` | T1 | operator Holiday (no trading) |
| 2017-04-14 | closed | — | `HKEX-TC-2017` | T1 | operator Holiday (no trading) |
| 2017-04-17 | closed | — | `HKEX-TC-2017` | T1 | operator Holiday (no trading) |
| 2017-05-01 | closed | — | `HKEX-TC-2017` | T1 | operator Holiday (no trading) |
| 2017-05-03 | closed | — | `HKEX-TC-2017` | T1 | operator Holiday (no trading) |
| 2017-05-29 | replacement blocks | blocks orderentry(0, 31500, 33300); regular(0, 33300, 43200); orderentry(0, 45000, 46800); regular(0, 46800, 59400) | `HKEX-TC-2017` | T1 | bank holiday in both the UK and the United States: the page's note suspends that day's AHT session while the day sessions run |
| 2017-05-30 | closed | — | `HKEX-TC-2017` | T1 | operator Holiday (no trading) |
| 2017-10-02 | closed | — | `HKEX-TC-2017` | T1 | operator Holiday (no trading) |
| 2017-10-05 | closed | — | `HKEX-TC-2017` | T1 | operator Holiday (no trading) |
| 2017-12-25 | closed | — | `HKEX-TC-2017` | T1 | operator Holiday (no trading) |
| 2017-12-26 | closed | — | `HKEX-TC-2017` | T1 | operator Holiday (no trading) |

### 2018

| 2018-01-01 | closed | — | `HKEX-HOL-2018` | T1 | operator Holiday (no trading) |
| 2018-02-15 | early close | 12:30 | `HKEX-HOL-2018` | T1 | the half-day eve: no afternoon session (and, from 2013, no AHT); the scalar clip keeps the morning and removes everything after 12:30 p.m. |
| 2018-02-16 | closed | — | `HKEX-HOL-2018` | T1 | operator Holiday (no trading) |
| 2018-02-19 | closed | — | `HKEX-HOL-2018` | T1 | operator Holiday (no trading) |
| 2018-03-30 | closed | — | `HKEX-HOL-2018` | T1 | operator Holiday (no trading) |
| 2018-04-02 | closed | — | `HKEX-HOL-2018` | T1 | operator Holiday (no trading) |
| 2018-04-05 | closed | — | `HKEX-HOL-2018` | T1 | operator Holiday (no trading) |
| 2018-05-01 | closed | — | `HKEX-HOL-2018` | T1 | operator Holiday (no trading) |
| 2018-05-22 | closed | — | `HKEX-HOL-2018` | T1 | operator Holiday (no trading) |
| 2018-05-28 | replacement blocks | blocks orderentry(0, 31500, 33300); regular(0, 33300, 43200); orderentry(0, 45000, 46800); regular(0, 46800, 59400) | `HKEX-HOL-2018` | T1 | bank holiday in both the UK and the United States: the page's note suspends that day's AHT session while the day sessions run |
| 2018-06-18 | closed | — | `HKEX-HOL-2018` | T1 | operator Holiday (no trading) |
| 2018-09-25 | closed | — | `HKEX-HOL-2018` | T1 | operator Holiday (no trading) |
| 2018-10-01 | closed | — | `HKEX-HOL-2018` | T1 | operator Holiday (no trading) |
| 2018-10-17 | closed | — | `HKEX-HOL-2018` | T1 | operator Holiday (no trading) |
| 2018-12-24 | early close | 12:30 | `HKEX-HOL-2018` | T1 | the half-day eve: no afternoon session (and, from 2013, no AHT); the scalar clip keeps the morning and removes everything after 12:30 p.m. |
| 2018-12-25 | closed | — | `HKEX-HOL-2018` | T1 | operator Holiday (no trading) |
| 2018-12-26 | closed | — | `HKEX-HOL-2018` | T1 | operator Holiday (no trading) |
| 2018-12-31 | early close | 12:30 | `HKEX-HOL-2018` | T1 | the half-day eve: no afternoon session (and, from 2013, no AHT); the scalar clip keeps the morning and removes everything after 12:30 p.m. |

### 2019

| 2019-01-01 | closed | — | `HKEX-TC-PAGE-2019` | T1 | operator Holiday (no trading) |
| 2019-02-04 | early close | 12:30 | `HKEX-TC-PAGE-2019` | T1 | the half-day eve: no afternoon session (and, from 2013, no AHT); the scalar clip keeps the morning and removes everything after 12:30 p.m. |
| 2019-02-05 | closed | — | `HKEX-TC-PAGE-2019` | T1 | operator Holiday (no trading) |
| 2019-02-06 | closed | — | `HKEX-TC-PAGE-2019` | T1 | operator Holiday (no trading) |
| 2019-02-07 | closed | — | `HKEX-TC-PAGE-2019` | T1 | operator Holiday (no trading) |
| 2019-04-05 | closed | — | `HKEX-TC-PAGE-2019` | T1 | operator Holiday (no trading) |
| 2019-04-19 | closed | — | `HKEX-TC-PAGE-2019` | T1 | operator Holiday (no trading) |
| 2019-04-22 | closed | — | `HKEX-TC-PAGE-2019` | T1 | operator Holiday (no trading) |
| 2019-05-01 | closed | — | `HKEX-TC-PAGE-2019` | T1 | operator Holiday (no trading) |
| 2019-05-13 | closed | — | `HKEX-TC-PAGE-2019` | T1 | operator Holiday (no trading) |
| 2019-05-27 | replacement blocks | blocks orderentry(0, 31500, 33300); regular(0, 33300, 43200); orderentry(0, 45000, 46800); regular(0, 46800, 59400) | `HKEX-TC-PAGE-2019` | T1 | bank holiday in both the UK and the United States: the page's note suspends that day's AHT session while the day sessions run |
| 2019-06-07 | closed | — | `HKEX-TC-PAGE-2019` | T1 | operator Holiday (no trading) |
| 2019-07-01 | closed | — | `HKEX-TC-PAGE-2019` | T1 | operator Holiday (no trading) |
| 2019-10-01 | closed | — | `HKEX-TC-PAGE-2019` | T1 | operator Holiday (no trading) |
| 2019-10-07 | closed | — | `HKEX-TC-PAGE-2019` | T1 | operator Holiday (no trading) |
| 2019-12-24 | early close | 12:30 | `HKEX-TC-PAGE-2019` | T1 | the half-day eve: no afternoon session (and, from 2013, no AHT); the scalar clip keeps the morning and removes everything after 12:30 p.m. |
| 2019-12-25 | closed | — | `HKEX-TC-PAGE-2019` | T1 | operator Holiday (no trading) |
| 2019-12-26 | closed | — | `HKEX-TC-PAGE-2019` | T1 | operator Holiday (no trading) |
| 2019-12-31 | early close | 12:30 | `HKEX-TC-PAGE-2019` | T1 | the half-day eve: no afternoon session (and, from 2013, no AHT); the scalar clip keeps the morning and removes everything after 12:30 p.m. |

### 2020

| 2020-01-01 | closed | — | `HKEX-HOL-2020` | T1 | operator Holiday (no trading) |
| 2020-01-24 | early close | 12:30 | `HKEX-HOL-2020` | T1 | the half-day eve: no afternoon session (and, from 2013, no AHT); the scalar clip keeps the morning and removes everything after 12:30 p.m. |
| 2020-01-27 | closed | — | `HKEX-HOL-2020` | T1 | operator Holiday (no trading) |
| 2020-01-28 | closed | — | `HKEX-HOL-2020` | T1 | operator Holiday (no trading) |
| 2020-04-10 | closed | — | `HKEX-HOL-2020` | T1 | operator Holiday (no trading) |
| 2020-04-13 | closed | — | `HKEX-HOL-2020` | T1 | operator Holiday (no trading) |
| 2020-04-30 | closed | — | `HKEX-HOL-2020` | T1 | operator Holiday (no trading) |
| 2020-05-01 | closed | — | `HKEX-HOL-2020` | T1 | operator Holiday (no trading) |
| 2020-05-25 | replacement blocks | blocks orderentry(0, 31500, 33300); regular(0, 33300, 43200); orderentry(0, 45000, 46800); regular(0, 46800, 59400) | `HKEX-HOL-2020` | T1 | bank holiday in both the UK and the United States: the page's note suspends that day's AHT session while the day sessions run |
| 2020-06-25 | closed | — | `HKEX-HOL-2020` | T1 | operator Holiday (no trading) |
| 2020-07-01 | closed | — | `HKEX-HOL-2020` | T1 | operator Holiday (no trading) |
| 2020-10-01 | closed | — | `HKEX-HOL-2020` | T1 | operator Holiday (no trading) |
| 2020-10-02 | closed | — | `HKEX-HOL-2020` | T1 | operator Holiday (no trading) |
| 2020-10-26 | closed | — | `HKEX-HOL-2020` | T1 | operator Holiday (no trading) |
| 2020-12-24 | early close | 12:30 | `HKEX-HOL-2020` | T1 | the half-day eve: no afternoon session (and, from 2013, no AHT); the scalar clip keeps the morning and removes everything after 12:30 p.m. |
| 2020-12-25 | closed | — | `HKEX-HOL-2020` | T1 | operator Holiday (no trading) |
| 2020-12-31 | early close | 12:30 | `HKEX-HOL-2020` | T1 | the half-day eve: no afternoon session (and, from 2013, no AHT); the scalar clip keeps the morning and removes everything after 12:30 p.m. |

### 2021

| 2021-01-01 | closed | — | `HKEX-TC-PAGE-2020` | T1 | operator Holiday (no trading) |
| 2021-02-11 | early close | 12:30 | `HKEX-TC-PAGE-2020` | T1 | the half-day eve: no afternoon session (and, from 2013, no AHT); the scalar clip keeps the morning and removes everything after 12:30 p.m. |
| 2021-02-12 | closed | — | `HKEX-TC-PAGE-2020` | T1 | operator Holiday (no trading) |
| 2021-02-15 | closed | — | `HKEX-TC-PAGE-2020` | T1 | operator Holiday (no trading) |
| 2021-04-02 | closed | — | `HKEX-TC-PAGE-2020` | T1 | operator Holiday (no trading) |
| 2021-04-05 | closed | — | `HKEX-TC-PAGE-2020` | T1 | operator Holiday (no trading) |
| 2021-04-06 | closed | — | `HKEX-TC-PAGE-2020` | T1 | operator Holiday (no trading) |
| 2021-05-19 | closed | — | `HKEX-TC-PAGE-2020` | T1 | operator Holiday (no trading) |
| 2021-05-31 | replacement blocks | blocks orderentry(0, 31500, 33300); regular(0, 33300, 43200); orderentry(0, 45000, 46800); regular(0, 46800, 59400) | `HKEX-TC-PAGE-2020` | T1 | bank holiday in both the UK and the United States: the page's note suspends that day's AHT session while the day sessions run |
| 2021-06-14 | closed | — | `HKEX-TC-PAGE-2020` | T1 | operator Holiday (no trading) |
| 2021-07-01 | closed | — | `HKEX-TC-PAGE-2020` | T1 | operator Holiday (no trading) |
| 2021-09-22 | closed | — | `HKEX-TC-PAGE-2020` | T1 | operator Holiday (no trading) |
| 2021-10-01 | closed | — | `HKEX-TC-PAGE-2020` | T1 | operator Holiday (no trading) |
| 2021-10-14 | closed | — | `HKEX-TC-PAGE-2020` | T1 | operator Holiday (no trading) |
| 2021-12-24 | early close | 12:30 | `HKEX-TC-PAGE-2020` | T1 | the half-day eve: no afternoon session (and, from 2013, no AHT); the scalar clip keeps the morning and removes everything after 12:30 p.m. |
| 2021-12-27 | closed | — | `HKEX-TC-PAGE-2020` | T1 | operator Holiday (no trading) |
| 2021-12-31 | early close | 12:30 | `HKEX-TC-PAGE-2020` | T1 | the half-day eve: no afternoon session (and, from 2013, no AHT); the scalar clip keeps the morning and removes everything after 12:30 p.m. |

### 2022

| 2022-01-31 | early close | 12:30 | `HKEX-HOL-2022` | T1 | the half-day eve: no afternoon session (and, from 2013, no AHT); the scalar clip keeps the morning and removes everything after 12:30 p.m. |
| 2022-02-01 | closed | — | `HKEX-HOL-2022` | T1 | operator Holiday (no trading) |
| 2022-02-02 | closed | — | `HKEX-HOL-2022` | T1 | operator Holiday (no trading) |
| 2022-02-03 | closed | — | `HKEX-HOL-2022` | T1 | operator Holiday (no trading) |
| 2022-04-05 | closed | — | `HKEX-HOL-2022` | T1 | operator Holiday (no trading) |
| 2022-04-15 | closed | — | `HKEX-HOL-2022` | T1 | operator Holiday (no trading) |
| 2022-04-18 | closed | — | `HKEX-HOL-2022` | T1 | operator Holiday (no trading) |
| 2022-05-02 | closed | — | `HKEX-HOL-2022` | T1 | operator Holiday (no trading) |
| 2022-05-09 | closed | — | `HKEX-HOL-2022` | T1 | operator Holiday (no trading) |
| 2022-06-03 | closed | — | `HKEX-HOL-2022` | T1 | operator Holiday (no trading) |
| 2022-07-01 | closed | — | `HKEX-HOL-2022` | T1 | operator Holiday (no trading) |
| 2022-09-12 | closed | — | `HKEX-HOL-2022` | T1 | operator Holiday (no trading) |
| 2022-10-04 | closed | — | `HKEX-HOL-2022` | T1 | operator Holiday (no trading) |
| 2022-12-26 | closed | — | `HKEX-HOL-2022` | T1 | operator Holiday (no trading) |
| 2022-12-27 | closed | — | `HKEX-HOL-2022` | T1 | operator Holiday (no trading) |

### 2023

| 2023-01-02 | closed | — | `HKEX-HOL-2023` | T1 | operator Holiday (no trading) |
| 2023-01-23 | closed | — | `HKEX-HOL-2023` | T1 | operator Holiday (no trading) |
| 2023-01-24 | closed | — | `HKEX-HOL-2023` | T1 | operator Holiday (no trading) |
| 2023-01-25 | closed | — | `HKEX-HOL-2023` | T1 | operator Holiday (no trading) |
| 2023-04-05 | closed | — | `HKEX-HOL-2023` | T1 | operator Holiday (no trading) |
| 2023-04-07 | closed | — | `HKEX-HOL-2023` | T1 | operator Holiday (no trading) |
| 2023-04-10 | closed | — | `HKEX-HOL-2023` | T1 | operator Holiday (no trading) |
| 2023-05-01 | closed | — | `HKEX-HOL-2023` | T1 | operator Holiday (no trading) |
| 2023-05-26 | closed | — | `HKEX-HOL-2023` | T1 | operator Holiday (no trading) |
| 2023-05-29 | replacement blocks | blocks orderentry(0, 31500, 33300); regular(0, 33300, 43200); orderentry(0, 45000, 46800); regular(0, 46800, 59400) | `HKEX-HOL-2023` | T1 | bank holiday in both the UK and the United States: the page's note suspends that day's AHT session while the day sessions run |
| 2023-06-22 | closed | — | `HKEX-HOL-2023` | T1 | operator Holiday (no trading) |
| 2023-10-02 | closed | — | `HKEX-HOL-2023` | T1 | operator Holiday (no trading) |
| 2023-10-23 | closed | — | `HKEX-HOL-2023` | T1 | operator Holiday (no trading) |
| 2023-12-25 | closed | — | `HKEX-HOL-2023` | T1 | operator Holiday (no trading) |
| 2023-12-26 | closed | — | `HKEX-HOL-2023` | T1 | operator Holiday (no trading) |

### 2024

| 2024-01-01 | closed | — | `HKEX-HOL-2024` | T1 | operator Holiday (no trading) |
| 2024-02-09 | early close | 12:30 | `HKEX-HOL-2024` | T1 | the half-day eve: no afternoon session (and, from 2013, no AHT); the scalar clip keeps the morning and removes everything after 12:30 p.m. |
| 2024-02-12 | closed | — | `HKEX-HOL-2024` | T1 | operator Holiday (no trading) |
| 2024-02-13 | closed | — | `HKEX-HOL-2024` | T1 | operator Holiday (no trading) |
| 2024-03-29 | closed | — | `HKEX-HOL-2024` | T1 | operator Holiday (no trading) |
| 2024-04-01 | closed | — | `HKEX-HOL-2024` | T1 | operator Holiday (no trading) |
| 2024-04-04 | closed | — | `HKEX-HOL-2024` | T1 | operator Holiday (no trading) |
| 2024-05-01 | closed | — | `HKEX-HOL-2024` | T1 | operator Holiday (no trading) |
| 2024-05-15 | closed | — | `HKEX-HOL-2024` | T1 | operator Holiday (no trading) |
| 2024-05-27 | replacement blocks | blocks orderentry(0, 31500, 33300); regular(0, 33300, 43200); orderentry(0, 45000, 46800); regular(0, 46800, 59400) | `HKEX-HOL-2024` | T1 | bank holiday in both the UK and the United States: the page's note suspends that day's AHT session while the day sessions run |
| 2024-06-10 | closed | — | `HKEX-HOL-2024` | T1 | operator Holiday (no trading) |
| 2024-07-01 | closed | — | `HKEX-HOL-2024` | T1 | operator Holiday (no trading) |
| 2024-09-18 | closed | — | `HKEX-HOL-2024` | T1 | operator Holiday (no trading) |
| 2024-10-01 | closed | — | `HKEX-HOL-2024` | T1 | operator Holiday (no trading) |
| 2024-10-11 | closed | — | `HKEX-HOL-2024` | T1 | operator Holiday (no trading) |
| 2024-12-24 | early close | 12:30 | `HKEX-HOL-2024` | T1 | the half-day eve: no afternoon session (and, from 2013, no AHT); the scalar clip keeps the morning and removes everything after 12:30 p.m. |
| 2024-12-25 | closed | — | `HKEX-HOL-2024` | T1 | operator Holiday (no trading) |
| 2024-12-26 | closed | — | `HKEX-HOL-2024` | T1 | operator Holiday (no trading) |
| 2024-12-31 | early close | 12:30 | `HKEX-HOL-2024` | T1 | the half-day eve: no afternoon session (and, from 2013, no AHT); the scalar clip keeps the morning and removes everything after 12:30 p.m. |

### 2025

| 2025-01-01 | closed | — | `HKEX-HOL-2025` | T1 | operator Holiday (no trading) |
| 2025-01-28 | early close | 12:30 | `HKEX-HOL-2025` | T1 | the half-day eve: no afternoon session (and, from 2013, no AHT); the scalar clip keeps the morning and removes everything after 12:30 p.m. |
| 2025-01-29 | closed | — | `HKEX-HOL-2025` | T1 | operator Holiday (no trading) |
| 2025-01-30 | closed | — | `HKEX-HOL-2025` | T1 | operator Holiday (no trading) |
| 2025-01-31 | closed | — | `HKEX-HOL-2025` | T1 | operator Holiday (no trading) |
| 2025-04-04 | closed | — | `HKEX-HOL-2025` | T1 | operator Holiday (no trading) |
| 2025-04-18 | closed | — | `HKEX-HOL-2025` | T1 | operator Holiday (no trading) |
| 2025-04-21 | closed | — | `HKEX-HOL-2025` | T1 | operator Holiday (no trading) |
| 2025-05-01 | closed | — | `HKEX-HOL-2025` | T1 | operator Holiday (no trading) |
| 2025-05-05 | closed | — | `HKEX-HOL-2025` | T1 | operator Holiday (no trading) |
| 2025-05-26 | replacement blocks | blocks orderentry(0, 31500, 33300); regular(0, 33300, 43200); orderentry(0, 45000, 46800); regular(0, 46800, 59400) | `HKEX-HOL-2025` | T1 | bank holiday in both the UK and the United States: the page's note suspends that day's AHT session while the day sessions run |
| 2025-07-01 | closed | — | `HKEX-HOL-2025` | T1 | operator Holiday (no trading) |
| 2025-10-01 | closed | — | `HKEX-HOL-2025` | T1 | operator Holiday (no trading) |
| 2025-10-07 | closed | — | `HKEX-HOL-2025` | T1 | operator Holiday (no trading) |
| 2025-10-29 | closed | — | `HKEX-HOL-2025` | T1 | operator Holiday (no trading) |
| 2025-12-24 | early close | 12:30 | `HKEX-HOL-2025` | T1 | the half-day eve: no afternoon session (and, from 2013, no AHT); the scalar clip keeps the morning and removes everything after 12:30 p.m. |
| 2025-12-25 | closed | — | `HKEX-HOL-2025` | T1 | operator Holiday (no trading) |
| 2025-12-26 | closed | — | `HKEX-HOL-2025` | T1 | operator Holiday (no trading) |
| 2025-12-31 | early close | 12:30 | `HKEX-HOL-2025` | T1 | the half-day eve: no afternoon session (and, from 2013, no AHT); the scalar clip keeps the morning and removes everything after 12:30 p.m. |

### 2026

| 2026-01-01 | closed | — | `HKEX-HOL-CURRENT` | T1 | operator Holiday (no trading) |
| 2026-02-16 | early close | 12:30 | `HKEX-HOL-CURRENT` | T1 | the half-day eve: no afternoon session (and, from 2013, no AHT); the scalar clip keeps the morning and removes everything after 12:30 p.m. |
| 2026-04-03 | closed | — | `HKEX-HOL-CURRENT` | T1 | operator Holiday (no trading) |
| 2026-04-06 | closed | — | `HKEX-HOL-CURRENT` | T1 | operator Holiday (no trading) |
| 2026-04-07 | closed | — | `HKEX-HOL-CURRENT` | T1 | operator Holiday (no trading) |
| 2026-05-01 | closed | — | `HKEX-HOL-CURRENT` | T1 | operator Holiday (no trading) |
| 2026-05-25 | closed | — | `HKEX-HOL-CURRENT` | T1 | operator Holiday (no trading) |
| 2026-06-19 | closed | — | `HKEX-HOL-CURRENT` | T1 | operator Holiday (no trading) |
| 2026-07-01 | closed | — | `HKEX-HOL-CURRENT` | T1 | operator Holiday (no trading) |
| 2026-10-01 | closed | — | `HKEX-HOL-CURRENT` | T1 | operator Holiday (no trading) |
| 2026-10-19 | closed | — | `HKEX-HOL-CURRENT` | T1 | operator Holiday (no trading) |
| 2026-10-20 | late open | 08:45 | `HKEX-HOL-CURRENT` | T1 | the holiday evening ran no AHT session; the late open removes the wrapped leg and keeps the following day's own sessions |
| 2026-12-24 | replacement blocks | blocks regular(-1, 61200, 10800); orderentry(0, 31500, 33300); regular(0, 33300, 45000) | `HKEX-HOL-CURRENT` | T1 | the half-day eve closes at 12:30 p.m. with no afternoon session and no evening AHT; the prior evening's AHT leg runs to 03:00 and the day's own morning runs to the half-day close |
| 2026-12-25 | closed | — | `HKEX-HOL-CURRENT` | T1 | operator Holiday (no trading) |
| 2026-12-26 | closed | — | `HKEX-HOL-CURRENT` | T1 | the holiday evening ran no AHT session; the wrapped leg's trade date is this Saturday and no session belongs to it |
| 2026-12-31 | replacement blocks | blocks regular(-1, 61200, 10800); orderentry(0, 31500, 33300); regular(0, 33300, 45000) | `HKEX-HOL-CURRENT` | T1 | the half-day eve closes at 12:30 p.m. with no afternoon session and no evening AHT; the prior evening's AHT leg runs to 03:00 and the day's own morning runs to the half-day close |

### 2027

| 2027-01-01 | closed | — | `HKEX-HOL-CURRENT` | T1 | operator Holiday (no trading) |
| 2027-01-02 | closed | — | `HKEX-HOL-CURRENT` | T1 | the holiday evening ran no AHT session; the wrapped leg's trade date is this Saturday and no session belongs to it |
| 2027-02-05 | replacement blocks | blocks regular(-1, 61200, 10800); orderentry(0, 31500, 33300); regular(0, 33300, 45000) | `HKEX-HOL-CURRENT` | T1 | the half-day eve closes at 12:30 p.m. with no afternoon session and no evening AHT; the prior evening's AHT leg runs to 03:00 and the day's own morning runs to the half-day close |
| 2027-02-06 | closed | — | `HKEX-HOL-CURRENT` | T1 | the eve ran no AHT session; the wrapped leg's trade date is this Saturday and no session belongs to it |
| 2027-02-08 | closed | — | `HKEX-HOL-CURRENT` | T1 | operator Holiday (no trading) |
| 2027-02-09 | closed | — | `HKEX-HOL-CURRENT` | T1 | operator Holiday (no trading) |
| 2027-02-10 | late open | 08:45 | `HKEX-HOL-CURRENT` | T1 | the holiday evening ran no AHT session; the late open removes the wrapped leg and keeps the following day's own sessions |
| 2027-03-26 | closed | — | `HKEX-HOL-CURRENT` | T1 | operator Holiday (no trading) |
| 2027-03-27 | closed | — | `HKEX-HOL-CURRENT` | T1 | the holiday evening ran no AHT session; the wrapped leg's trade date is this Saturday and no session belongs to it |
| 2027-03-29 | closed | — | `HKEX-HOL-CURRENT` | T1 | operator Holiday (no trading) |
| 2027-03-30 | late open | 08:45 | `HKEX-HOL-CURRENT` | T1 | the holiday evening ran no AHT session; the late open removes the wrapped leg and keeps the following day's own sessions |
| 2027-04-05 | closed | — | `HKEX-HOL-CURRENT` | T1 | operator Holiday (no trading) |
| 2027-04-06 | late open | 08:45 | `HKEX-HOL-CURRENT` | T1 | the holiday evening ran no AHT session; the late open removes the wrapped leg and keeps the following day's own sessions |
| 2027-05-13 | closed | — | `HKEX-HOL-CURRENT` | T1 | operator Holiday (no trading) |
| 2027-05-14 | late open | 08:45 | `HKEX-HOL-CURRENT` | T1 | the holiday evening ran no AHT session; the late open removes the wrapped leg and keeps the following day's own sessions |
| 2027-06-09 | closed | — | `HKEX-HOL-CURRENT` | T1 | operator Holiday (no trading) |
| 2027-06-10 | late open | 08:45 | `HKEX-HOL-CURRENT` | T1 | the holiday evening ran no AHT session; the late open removes the wrapped leg and keeps the following day's own sessions |
| 2027-07-01 | closed | — | `HKEX-HOL-CURRENT` | T1 | operator Holiday (no trading) |
| 2027-07-02 | late open | 08:45 | `HKEX-HOL-CURRENT` | T1 | the holiday evening ran no AHT session; the late open removes the wrapped leg and keeps the following day's own sessions |
| 2027-09-16 | closed | — | `HKEX-HOL-CURRENT` | T1 | operator Holiday (no trading) |
| 2027-09-17 | late open | 08:45 | `HKEX-HOL-CURRENT` | T1 | the holiday evening ran no AHT session; the late open removes the wrapped leg and keeps the following day's own sessions |
| 2027-10-01 | closed | — | `HKEX-HOL-CURRENT` | T1 | operator Holiday (no trading) |
| 2027-10-02 | closed | — | `HKEX-HOL-CURRENT` | T1 | the holiday evening ran no AHT session; the wrapped leg's trade date is this Saturday and no session belongs to it |
| 2027-10-08 | closed | — | `HKEX-HOL-CURRENT` | T1 | operator Holiday (no trading) |
| 2027-10-09 | closed | — | `HKEX-HOL-CURRENT` | T1 | the holiday evening ran no AHT session; the wrapped leg's trade date is this Saturday and no session belongs to it |
| 2027-12-24 | replacement blocks | blocks regular(-1, 61200, 10800); orderentry(0, 31500, 33300); regular(0, 33300, 45000) | `HKEX-HOL-CURRENT` | T1 | the half-day eve closes at 12:30 p.m. with no afternoon session and no evening AHT; the prior evening's AHT leg runs to 03:00 and the day's own morning runs to the half-day close |
| 2027-12-25 | closed | — | `HKEX-HOL-CURRENT` | T1 | the eve ran no AHT session; the wrapped leg's trade date is this Saturday and no session belongs to it |
| 2027-12-27 | closed | — | `HKEX-HOL-CURRENT` | T1 | operator Holiday (no trading) |
| 2027-12-28 | late open | 08:45 | `HKEX-HOL-CURRENT` | T1 | the holiday evening ran no AHT session; the late open removes the wrapped leg and keeps the following day's own sessions |
| 2027-12-31 | replacement blocks | blocks regular(-1, 61200, 10800); orderentry(0, 31500, 33300); regular(0, 33300, 45000) | `HKEX-HOL-CURRENT` | T1 | the half-day eve closes at 12:30 p.m. with no afternoon session and no evening AHT; the prior evening's AHT leg runs to 03:00 and the day's own morning runs to the half-day close |

### 2028

| 2028-01-01 | closed | — | `HKEX-HOL-CURRENT` | T1 | the operator closed the 31 December 2027 evening AHT session, whose wrapped leg's trade date is 1 January 2028 (audited from the 2027 table) |
