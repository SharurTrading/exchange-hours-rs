<!-- SPDX-License-Identifier: MIT-0 -->

# `nse_india` — evidence

- **Kind:** `Exchange`
- **Owner module:** [`nse.rs`](../../src/calendar/schedules/equities/apac/nse.rs)
- **Source sets:** [`APAC-INDIA-CASH`](../schedules/sources.md#apac-india-cash)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

Venue envelope includes 2026 CAS-eligible and non-CAS states.

## Revision rows

- 2010-01-04 — T1 — NSE press release 17122009 — continuous open moves 09:55 → 09:00.
- 2010-10-18 — T1 — NSE circular NSE/CMTR/15981 — pre-open call auction 09:00–09:15 introduced; continuous trading starts 09:15.
- 2011-10-03 — T1 — NSE circular NSE/CMTR/19013 — post-close start moves 15:50 → 15:40.
- 2026-08-03 — T1 — SEBI circular 99122 — Closing Auction Session: CAS-eligible stocks enter CAS at 15:15 and it ends 15:35, transition runs to 15:50, post-close ends 16:00.

## Holidays

**Coverage:** 2010-01-01..2017-12-31, 2019-01-01..2026-12-31 (inclusive trade dates; T1 except the holiday-master feed years, which are T2)

Fourteen sources key the 2010-2024 rows and two more corroborate them, all the operator's own published holiday material, plus the two current banners: the annual circulars (`NSE-CIRC-2010-61` from the operator's `HOLIDAYS.zip` press bundle, `NSE-CIRC-2011-66` dated 2011-12-09 for the 2012 list, `NSE-CIRC-2012-79` dated 2012-12-17), the annual trading-holiday pages (`NSE-HOL-PAGE-2011`, `NSE-HOL-PAGE-2014`..`NSE-HOL-PAGE-2017`, `NSE-HOL-PAGE-2019`..`NSE-HOL-PAGE-2022`), and the operator's own `holiday-master?type=trading` machine channel (`NSE-HOLMASTER-2023` and `NSE-HOLMASTER-2024` for their years; T2) — the 2023-2024 rows are the block's only T2 rows. `NSE-CIRC-2011-126` corroborates the 2011 page date-for-date and `NSE-HOLMASTER-2022` the 2022 page date-for-date. The 2025-2026 rows key from the "NSE Holiday List 2025" banner (`NSE-HOL-2025`) and the "Trading Holiday List for Equity & Equity Derivatives, Calendar Year 2026" banner (`NSE-HOL-2026`). Every artifact was read from Wayback `id_` replays because the live site refuses the session: on 2026-09-28 and 2026-09-29 the `nseindia.com` web channel returned HTTP 403 and its `holiday-master` API empty bodies (two refusals each), while the archive holds verbatim captures of every document cited here. The `holiday-master?type=CM` capture the archive lists (`20260909065333`) stores an empty payload and is not evidence of anything.

The lists are the capital-market trading holidays — the envelope the `nse_india` identity models. Each source prints its year's holiday rows (`DATE / DAY / DESCRIPTION` on the pages and in the 2012 circular, `tradingDate/weekDay/description` in the feed) and separately lists the holidays falling on Saturday/Sunday; only the weekday legs ship rows, and each printed date's weekday as printed matches the civil calendar, so every row below is the printed date itself. No source states an early close, late open or shortened session for any listed date, and every closure below is a full closure. **The windows run 2010-01-01..2017-12-31 and 2019-01-01..2026-12-31 because 2018 is unrecovered** (see Gaps): a window across it would make every date of that year read as audited normal, so the coverage stops at each gap's edge. 2012 sat in that state until 2026-10-06 UTC, when the 2012 list's own annual circular (`NSE-CIRC-2011-66`, dated 2011-12-09) surfaced in the archive and the first window merged across the year; the three years 2010-2011 and 2013-2017 on either side of it were already audited, so the merged window is one span.

**Seven dates remain `Unsourced`, not closed — six Muhurat Trading dates and the 2026 banner date** (2011-10-26, 2012-11-13, 2013-11-03, 2014-10-23, 2017-10-19, 2019-10-27 and 2026-11-08). Each source that keys a Muhurat footnote to a date footnotes a special session whose instants the operator had not published in the captured artifact. The pre-2025 sources print the shorter footnote `Muhurat Trading will be conducted. Timings of Muhurat Trading shall be notified subsequently` against an asterisked date of their lists — the 2012 circular's own footnote reads `*Muhurat Trading will be conducted. Timings of Muhurat Trading shall be notified subsequently.` against `13-Nov-12 Tuesday Diwali – Laxmi Puja*`, so recovering the 2012 list recovered its withheld date with it (the 2013 circular and the 2023-2024 feeds name the day or carry the asterisk; the 2013, 2016 and 2019 Muhurat dates are weekend days, so there the announcement contradicts the audited-normal weekend the normal week would otherwise claim). The 2026 banner: `Muhurat Trading will be conducted on Sunday, November 08, 2026. Timings of Muhurat Trading shall be notified subsequently`, and silence on 2026-11-08 would claim the audited-normal weekend closure the operator's announcement contradicts, so the date ships `Unsourced`. **Closing condition:** the operator's Muhurat Trading circular stating the session instants, at which point 2026-11-08 becomes a stated special session. Press coverage keys no row (LAW-PRIMARY-SOURCES). The 2026-10-09 (UTC) Common Crawl sweep closed the second archive for the circular channel: no queried collection (2012, 2014-10, 2017-43, 2019-35, on all three hosts) holds a capture of the `content/cmtr` directory, so the Wayback absence is not a Wayback artifact (store `holidays/raw/cc-sweeps-2026-10-09.txt`; one 2013 collection query remains 504-unresolved).

**Eight Muhurat dates ship `ReplacementBlocks` — the circulars were recovered from the Wayback archive** (2010-11-05, 2015-11-11 and 2016-10-30 on the 2026-10-06 UTC second pass; 2020-11-14, 2022-10-24, 2023-11-12, 2024-11-01 and 2025-10-21 in the same day's first pass). The operator's own capital-market Muhurat circulars state each day's complete special-session schedule in session language, and each row restates its printed schedule as one replacement day: Block deal (a trade prints, `extended`), Pre Open split at the earliest second the footnote's random closure can fire (order entry to it, the tradeable tail to the Normal Market open `extended`, exactly as the normal-week profile splits its 09:00 pre-open at 09:07), Normal Market (`regular`), and the Closing Session (`extended`, the fixed-price post-close the family already models). The Trade Modification cut-off is a modification window, not a session; the Special Pre-open Session for IPO and relisted scrips and the Call Auction Illiquid window are sub-segment phases inside the stated envelope, and neither states an edge the envelope lacks. The printed schedules: 2010 — pre-open order entry 18:00-18:08 (`with random closure in last one minute`), Normal / RDM / Odd Lot Market 18:15-19:00, Closing Session 19:20-19:30, no block-deal session in 2010; 2015 and 2016 identical in shape — block deal 17:45-18:20 (2015) / 18:30-19:05 (2016) inside the Normal Market window, Pre Open 17:30-17:38 (2015) / 18:15-18:23 (2016) with the random closure `* Random closure in last one minute`, Normal Market/LPM 17:45-18:45 (2015) / 18:30-19:30 (2016), Call Auction Illiquid 17:50-18:35 (2015) / 18:35-19:20 (2016), Closing Session 18:55-19:05 (2015) / 19:40-19:50 (2016); 2020, 2022 and 2023 identical — block deal 17:45-18:00, Pre Open 18:00-18:08 (`* Random closure in last one minute`), Normal Market 18:15-19:15, Closing 19:25-19:35; 2024 — block deal 17:30-17:45, Pre Open 17:45-18:00 with the order-entry period bounded `17:45 to 17:53 hours` (random closure in its last minute), Normal Market 18:00-19:00, Closing 19:10-19:20; 2025 — block deal 13:15-13:30, Pre Open 13:30-13:45 with the random closure `i.e. 13:37 to 13:38 hours`, Normal Market 13:45-14:45, Closing 14:55-15:05. The 2020 rows' instants are corroborated date-for-time by the operator's own `Muhurat Trading Session` page as served 2020-11-20 (`NSE-MUHURAT-PAGE-2020`, printed `Updated on: 12/11/2020`, which prints the same five CM windows and names the four segment circulars of the 46230-46233 block).

### 2010

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2010-01-01 | closed | `01-Jan-10 Friday New Year` | `NSE-CIRC-2010-61` | T1 | Printed date 2010-01-01, a Friday |
| 2010-01-26 | closed | `26-Jan-10 Tuesday Republic Day` | `NSE-CIRC-2010-61` | T1 | Printed date 2010-01-26, a Tuesday |
| 2010-02-12 | closed | `12-Feb-10 Friday Mahashivratri` | `NSE-CIRC-2010-61` | T1 | Printed date 2010-02-12, a Friday |
| 2010-03-01 | closed | `01-Mar-10 Monday Holi` | `NSE-CIRC-2010-61` | T1 | Printed date 2010-03-01, a Monday |
| 2010-03-24 | closed | `24-Mar-10 Wednesday Ram Navmi` | `NSE-CIRC-2010-61` | T1 | Printed date 2010-03-24, a Wednesday |
| 2010-04-02 | closed | `02-Apr-10 Friday Good Friday` | `NSE-CIRC-2010-61` | T1 | Printed date 2010-04-02, a Friday |
| 2010-04-14 | closed | `14-Apr-10 Wednesday Dr. Ambedkar Jayanti` | `NSE-CIRC-2010-61` | T1 | Printed date 2010-04-14, a Wednesday |
| 2010-09-10 | closed | `10-Sep-10 Friday Ramzan ID` | `NSE-CIRC-2010-61` | T1 | Printed date 2010-09-10, a Friday |
| 2010-11-05 | replacement blocks | `Pre-open order entry 18:00-18:07` order entry with its random-closure tail to the 18:15 open (the circular prints the pre-open row 1800-1808 `with random closure in last one minute`); `Normal / RDM / Odd Lot Market 18:15-19:00`; `Closing Session 19:20-19:30`; no block-deal session exists in 2010 | `NSE-CIRC-2010-121` | T1 | circular 121/2010 (2010-10-20) `notifies the special trading session for muhurat trading on account of Diwali on Friday, November 05, 2010`; the row restates the printed schedule as one replacement day, the order-entry leg stopping at 18:07, the earliest second the closure can fire |
| 2010-11-17 | closed | `17-Nov-10 Wednesday Bakri Id` | `NSE-CIRC-2010-61` | T1 | Printed date 2010-11-17, a Wednesday |
| 2010-12-17 | closed | `17-Dec-10 Friday Moharum` | `NSE-CIRC-2010-61` | T1 | Printed date 2010-12-17, a Friday |

### 2011

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2011-01-26 | closed | `26-Jan-2011 Wednesday Republic Day` | `NSE-HOL-PAGE-2011` | T1 | Printed date 2011-01-26, a Wednesday |
| 2011-03-02 | closed | `02-Mar-2011 Wednesday Mahashivratri` | `NSE-HOL-PAGE-2011` | T1 | Printed date 2011-03-02, a Wednesday |
| 2011-04-12 | closed | `12-Apr-2011 Tuesday Ram Navmi` | `NSE-HOL-PAGE-2011` | T1 | Printed date 2011-04-12, a Tuesday |
| 2011-04-14 | closed | `14-Apr-2011 Thursday Dr. Ambedkar Jayanti` | `NSE-HOL-PAGE-2011` | T1 | Printed date 2011-04-14, a Thursday |
| 2011-04-22 | closed | `22-Apr-2011 Friday Good Friday` | `NSE-HOL-PAGE-2011` | T1 | Printed date 2011-04-22, a Friday |
| 2011-08-15 | closed | `15-Aug-2011 Monday Independence Day` | `NSE-HOL-PAGE-2011` | T1 | Printed date 2011-08-15, a Monday |
| 2011-08-31 | closed | `31-Aug-2011 Wednesday Ramzan ID` | `NSE-HOL-PAGE-2011` | T1 | Printed date 2011-08-31, a Wednesday |
| 2011-09-01 | closed | `01-Sep-2011 Thursday Ganesh Chaturthi` | `NSE-HOL-PAGE-2011` | T1 | Printed date 2011-09-01, a Thursday |
| 2011-10-06 | closed | `06-Oct-2011 Thursday Dasara` | `NSE-HOL-PAGE-2011` | T1 | Printed date 2011-10-06, a Thursday |
| 2011-10-26 | unsourced | `26-Oct-2011 Wednesday Laxmi Puja*` with the page's footnote `Muhurat Trading will be conducted. Timings of Muhurat Trading shall be notified subsequently.` | `NSE-HOL-PAGE-2011` | T1 | Printed date 2011-10-26, a Wednesday; the day's special session has no published instants, so the date is withheld rather than closed |
| 2011-10-27 | closed | `27-Oct-2011 Thursday Diwali - Balipratipada` | `NSE-HOL-PAGE-2011` | T1 | Printed date 2011-10-27, a Thursday |
| 2011-11-07 | closed | `07-Nov-2011 Monday Bakri Id` | `NSE-HOL-PAGE-2011` | T1 | Printed date 2011-11-07, a Monday |
| 2011-11-10 | closed | `10-Nov-2011 Thursday Gurunanak Jayanti` | `NSE-HOL-PAGE-2011` | T1 | Printed date 2011-11-10, a Thursday |
| 2011-12-06 | closed | `06-Dec-2011 Tuesday Moharum` | `NSE-HOL-PAGE-2011` | T1 | Printed date 2011-12-06, a Tuesday |

### 2012

The 2012 list is the operator's own annual circular 66/2011, recovered 2026-10-06 UTC as a Wayback `id_` replay (`NSE-CIRC-2011-66`, download `NSE/CMTR/19539`, dated December 09, 2011): `the Exchange hereby notifies trading holidays for the calendar year 2012 as below:` — fifteen weekday rows, `26-Jan-12 Thursday Republic Day` through `25-Dec-12 Tuesday Christmas`, plus five Saturday/Sunday holidays that key no rows (`01-Jan-12 Sunday New Year`, `01-Apr-12 Sunday Ram Navami`, `14-Apr-12 Saturday Ambedkar Jayanti`, `27-Oct-12 Saturday Bakri Id`, `25-Nov-12 Sunday Moharram`). The circular had been uncaptured through the 2026-09-29 hunt; the second pass the same UTC day found it by sweeping the archive's whole captured `CMTR` circular series against the issuance-number ranges the recovered circulars themselves anchor (the store's `muhurat-recovery-2026-10-06/INDEX.md` records the method and the negatives it closed).

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2012-01-26 | closed | `26-Jan-12 Thursday Republic Day` | `NSE-CIRC-2011-66` | T1 | Printed date 2012-01-26, a Thursday |
| 2012-02-20 | closed | `20-Feb-12 Monday Mahashivratri` | `NSE-CIRC-2011-66` | T1 | Printed date 2012-02-20, a Monday |
| 2012-03-08 | closed | `08-Mar-12 Thursday Holi` | `NSE-CIRC-2011-66` | T1 | Printed date 2012-03-08, a Thursday |
| 2012-04-05 | closed | `05-Apr-12 Thursday Mahavir Jayanti` | `NSE-CIRC-2011-66` | T1 | Printed date 2012-04-05, a Thursday |
| 2012-04-06 | closed | `06-Apr-12 Friday Good Friday` | `NSE-CIRC-2011-66` | T1 | Printed date 2012-04-06, a Friday |
| 2012-05-01 | closed | `01-May-12 Tuesday May Day` | `NSE-CIRC-2011-66` | T1 | Printed date 2012-05-01, a Tuesday |
| 2012-08-15 | closed | `15-Aug-12 Wednesday Independence Day` | `NSE-CIRC-2011-66` | T1 | Printed date 2012-08-15, a Wednesday |
| 2012-08-20 | closed | `20-Aug-12 Monday Ramzan ID` | `NSE-CIRC-2011-66` | T1 | Printed date 2012-08-20, a Monday |
| 2012-09-19 | closed | `19-Sep-12 Wednesday Ganesh Chaturthi` | `NSE-CIRC-2011-66` | T1 | Printed date 2012-09-19, a Wednesday |
| 2012-10-02 | closed | `02-Oct-12 Tuesday Gandhi Jayanti` | `NSE-CIRC-2011-66` | T1 | Printed date 2012-10-02, a Tuesday |
| 2012-10-24 | closed | `24-Oct-12 Wednesday Dasera` | `NSE-CIRC-2011-66` | T1 | Printed date 2012-10-24, a Wednesday |
| 2012-11-13 | unsourced | `13-Nov-12 Tuesday Diwali – Laxmi Puja*` with the circular's footnote `*Muhurat Trading will be conducted. Timings of Muhurat Trading shall be notified subsequently.` | `NSE-CIRC-2011-66` | T1 | Printed date 2012-11-13, a Tuesday; the day's special session has no published instants, so the date is withheld rather than closed — recovering the list recovered its withheld Muhurat date with it |
| 2012-11-14 | closed | `14-Nov-12 Wednesday Diwali - Balipratipada` | `NSE-CIRC-2011-66` | T1 | Printed date 2012-11-14, a Wednesday |
| 2012-11-28 | closed | `28-Nov-12 Wednesday Gurunanak Jayanti` | `NSE-CIRC-2011-66` | T1 | Printed date 2012-11-28, a Wednesday |
| 2012-12-25 | closed | `25-Dec-12 Tuesday Christmas` | `NSE-CIRC-2011-66` | T1 | Printed date 2012-12-25, a Tuesday |

**Gaps, 2012:** none. Every printed weekday holiday ships its row, the printed Muhurat date is withheld with its sibling years, and the five printed Saturday/Sunday holidays key no rows by the block's own convention — so the merged window audits 2012 completely at the list's own depth.

### 2013

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2013-03-27 | closed | `27-Mar-13 Wednesday Holi` | `NSE-CIRC-2012-79` | T1 | Printed date 2013-03-27, a Wednesday |
| 2013-03-29 | closed | `29-Mar-13 Friday Good Friday` | `NSE-CIRC-2012-79` | T1 | Printed date 2013-03-29, a Friday |
| 2013-04-19 | closed | `19-Apr-13 Friday Ram Navmi` | `NSE-CIRC-2012-79` | T1 | Printed date 2013-04-19, a Friday |
| 2013-04-24 | closed | `24-Apr-13 Wednesday Mahavir Jayanti` | `NSE-CIRC-2012-79` | T1 | Printed date 2013-04-24, a Wednesday |
| 2013-05-01 | closed | `01-May-13 Wednesday May Day` | `NSE-CIRC-2012-79` | T1 | Printed date 2013-05-01, a Wednesday |
| 2013-08-09 | closed | `09-Aug-13 Friday Ramzan ID` | `NSE-CIRC-2012-79` | T1 | Printed date 2013-08-09, a Friday |
| 2013-08-15 | closed | `15-Aug-13 Thursday Independence Day` | `NSE-CIRC-2012-79` | T1 | Printed date 2013-08-15, a Thursday |
| 2013-09-09 | closed | `09-Sep-13 Monday Ganesh Chaturthi` | `NSE-CIRC-2012-79` | T1 | Printed date 2013-09-09, a Monday |
| 2013-10-02 | closed | `02-Oct-13 Wednesday Gandhi Jayanti` | `NSE-CIRC-2012-79` | T1 | Printed date 2013-10-02, a Wednesday |
| 2013-10-16 | closed | `16-Oct-13 Wednesday Bakri ID` | `NSE-CIRC-2012-79` | T1 | Printed date 2013-10-16, a Wednesday |
| 2013-11-03 | unsourced | the Saturday/Sunday list's `03-Nov-13 Sunday Diwali-Laxmi Puja*` with the circular's footnote `Muhurat Trading will be conducted on Sunday, November 03, 2013.` | `NSE-CIRC-2012-79` | T1 | Printed Muhurat date 2013-11-03, a Sunday; the announced session has no published instants, so the day is not the audited-normal weekend closure the normal week alone would claim |
| 2013-11-04 | closed | `04-Nov-13 Monday Diwali-Balipratipada` | `NSE-CIRC-2012-79` | T1 | Printed date 2013-11-04, a Monday |
| 2013-11-14 | closed | `14-Nov-13 Thursday Moharram` | `NSE-CIRC-2012-79` | T1 | Printed date 2013-11-14, a Thursday |
| 2013-12-25 | closed | `25-Dec-13 Wednesday Christmas` | `NSE-CIRC-2012-79` | T1 | Printed date 2013-12-25, a Wednesday |

### 2014

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2014-02-27 | closed | `27-Feb-2014 Thursday Mahashivratri` | `NSE-HOL-PAGE-2014` | T1 | Printed date 2014-02-27, a Thursday |
| 2014-03-17 | closed | `17-Mar-2014 Monday Holi` | `NSE-HOL-PAGE-2014` | T1 | Printed date 2014-03-17, a Monday |
| 2014-04-08 | closed | `08-Apr-2014 Tuesday Ram Navmi` | `NSE-HOL-PAGE-2014` | T1 | Printed date 2014-04-08, a Tuesday |
| 2014-04-14 | closed | `14-Apr-2014 Monday Dr. Babasaheb Ambedkar Jayanti` | `NSE-HOL-PAGE-2014` | T1 | Printed date 2014-04-14, a Monday |
| 2014-04-18 | closed | `18-Apr-2014 Friday Good Friday` | `NSE-HOL-PAGE-2014` | T1 | Printed date 2014-04-18, a Friday |
| 2014-05-01 | closed | `01-May-2014 Thursday May Day` | `NSE-HOL-PAGE-2014` | T1 | Printed date 2014-05-01, a Thursday |
| 2014-07-29 | closed | `29-Jul-2014 Tuesday Ramzan ID` | `NSE-HOL-PAGE-2014` | T1 | Printed date 2014-07-29, a Tuesday |
| 2014-08-15 | closed | `15-Aug-2014 Friday Independence Day` | `NSE-HOL-PAGE-2014` | T1 | Printed date 2014-08-15, a Friday |
| 2014-08-29 | closed | `29-Aug-2014 Friday Ganesh Chaturthi` | `NSE-HOL-PAGE-2014` | T1 | Printed date 2014-08-29, a Friday |
| 2014-10-02 | closed | `02-Oct-2014 Thursday Mahatma Gandhi Jayanti` | `NSE-HOL-PAGE-2014` | T1 | Printed date 2014-10-02, a Thursday |
| 2014-10-03 | closed | `03-Oct-2014 Friday Dasera` | `NSE-HOL-PAGE-2014` | T1 | Printed date 2014-10-03, a Friday |
| 2014-10-06 | closed | `06-Oct-2014 Monday Bakri ID` | `NSE-HOL-PAGE-2014` | T1 | Printed date 2014-10-06, a Monday |
| 2014-10-23 | unsourced | `23-Oct-2014 Thursday Diwali-Laxmi Pujan*` with the page's footnote `Muhurat Trading will be conducted. Timings of Muhurat Trading shall be notified subsequently.` | `NSE-HOL-PAGE-2014` | T1 | Printed date 2014-10-23, a Thursday; the day's special session has no published instants, so the date is withheld rather than closed |
| 2014-10-24 | closed | `24-Oct-2014 Friday Diwali-Balipratipada` | `NSE-HOL-PAGE-2014` | T1 | Printed date 2014-10-24, a Friday |
| 2014-11-04 | closed | `04-Nov-2014 Tuesday Moharram` | `NSE-HOL-PAGE-2014` | T1 | Printed date 2014-11-04, a Tuesday |
| 2014-11-06 | closed | `06-Nov-2014 Thursday Gurunank Jayanti` | `NSE-HOL-PAGE-2014` | T1 | Printed date 2014-11-06, a Thursday |
| 2014-12-25 | closed | `25-Dec-2014 Thursday Christmas` | `NSE-HOL-PAGE-2014` | T1 | Printed date 2014-12-25, a Thursday |

### 2015

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2015-01-26 | closed | `26-Jan-2015 Monday Republic Day` | `NSE-HOL-PAGE-2015` | T1 | Printed date 2015-01-26, a Monday |
| 2015-02-17 | closed | `17-Feb-2015 Tuesday Mahashivratri` | `NSE-HOL-PAGE-2015` | T1 | Printed date 2015-02-17, a Tuesday |
| 2015-03-06 | closed | `06-Mar-2015 Friday Holi` | `NSE-HOL-PAGE-2015` | T1 | Printed date 2015-03-06, a Friday |
| 2015-04-02 | closed | `02-Apr-2015 Thursday Mahavir Jayanti` | `NSE-HOL-PAGE-2015` | T1 | Printed date 2015-04-02, a Thursday |
| 2015-04-03 | closed | `03-Apr-2015 Friday Good Friday` | `NSE-HOL-PAGE-2015` | T1 | Printed date 2015-04-03, a Friday |
| 2015-04-14 | closed | `14-Apr-2015 Tuesday Dr. Baba Saheb Ambedkar Jayanti` | `NSE-HOL-PAGE-2015` | T1 | Printed date 2015-04-14, a Tuesday |
| 2015-05-01 | closed | `01-May-2015 Friday Maharashtra Day` | `NSE-HOL-PAGE-2015` | T1 | Printed date 2015-05-01, a Friday |
| 2015-09-17 | closed | `17-Sep-2015 Thursday Ganesh Chaturthi` | `NSE-HOL-PAGE-2015` | T1 | Printed date 2015-09-17, a Thursday |
| 2015-09-25 | closed | `25-Sep-2015 Friday Bakri ID` | `NSE-HOL-PAGE-2015` | T1 | Printed date 2015-09-25, a Friday |
| 2015-10-02 | closed | `02-Oct-2015 Friday Mahatma Gandhi Jayanti` | `NSE-HOL-PAGE-2015` | T1 | Printed date 2015-10-02, a Friday |
| 2015-10-22 | closed | `22-Oct-2015 Thursday Dussehra` | `NSE-HOL-PAGE-2015` | T1 | Printed date 2015-10-22, a Thursday |
| 2015-11-11 | replacement blocks | `Pre Open* 17:30-17:37` order entry with its random-closure tail to the 17:45 open (the circular prints the pre-open row 1730-1738 with `* Random closure in last one minute`); `Normal Market/LPM 17:45-18:45`; `Closing Session 18:55-19:05` | `NSE-CIRC-2015-65` | T1 | circular 65/2015 (2015-10-30) states `a special live trading session shall be held on Wednesday, November 11, 2015 on account of Muhurat trading on Diwali as per the following schedule`; the row restates the printed schedule as one replacement day, the order-entry leg stopping at 17:37, the earliest second the closure can fire; the circular's `Block Deal Session 1745-1820` and `Call Auction Illiquid session 1750-1835` run inside the Normal Market hours and state no edge the envelope lacks |
| 2015-11-12 | closed | `12-Nov-2015 Thursday Diwali-Balipratipada` | `NSE-HOL-PAGE-2015` | T1 | Printed date 2015-11-12, a Thursday |
| 2015-11-25 | closed | `25-Nov-2015 Wednesday Gurunanak Jayanti` | `NSE-HOL-PAGE-2015` | T1 | Printed date 2015-11-25, a Wednesday |
| 2015-12-25 | closed | `25-Dec-2015 Friday Christmas` | `NSE-HOL-PAGE-2015` | T1 | Printed date 2015-12-25, a Friday |

### 2016

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2016-01-26 | closed | `26-Jan-2016 Tuesday Republic Day` | `NSE-HOL-PAGE-2016` | T1 | Printed date 2016-01-26, a Tuesday |
| 2016-03-07 | closed | `07-Mar-2016 Monday Mahashivratri` | `NSE-HOL-PAGE-2016` | T1 | Printed date 2016-03-07, a Monday |
| 2016-03-24 | closed | `24-Mar-2016 Thursday Holi` | `NSE-HOL-PAGE-2016` | T1 | Printed date 2016-03-24, a Thursday |
| 2016-03-25 | closed | `25-Mar-2016 Friday Good Friday` | `NSE-HOL-PAGE-2016` | T1 | Printed date 2016-03-25, a Friday |
| 2016-04-14 | closed | `14-Apr-2016 Thursday Dr. Baba Saheb Ambedkar Jayanti` | `NSE-HOL-PAGE-2016` | T1 | Printed date 2016-04-14, a Thursday |
| 2016-04-15 | closed | `15-Apr-2016 Friday Ram Navami` | `NSE-HOL-PAGE-2016` | T1 | Printed date 2016-04-15, a Friday |
| 2016-04-19 | closed | `19-Apr-2016 Tuesday Mahavir Jayanti` | `NSE-HOL-PAGE-2016` | T1 | Printed date 2016-04-19, a Tuesday |
| 2016-07-06 | closed | `06-Jul-2016 Wednesday Id-uI-Fitar (Ramzan ID)` | `NSE-HOL-PAGE-2016` | T1 | Printed date 2016-07-06, a Wednesday |
| 2016-08-15 | closed | `15-Aug-2016 Monday Independence Day` | `NSE-HOL-PAGE-2016` | T1 | Printed date 2016-08-15, a Monday |
| 2016-09-05 | closed | `05-Sep-2016 Monday Ganesh Chaturthi` | `NSE-HOL-PAGE-2016` | T1 | Printed date 2016-09-05, a Monday |
| 2016-09-13 | closed | `13-Sep-2016 Tuesday Bakri ID` | `NSE-HOL-PAGE-2016` | T1 | Printed date 2016-09-13, a Tuesday |
| 2016-10-11 | closed | `11-Oct-2016 Tuesday Dasera` | `NSE-HOL-PAGE-2016` | T1 | Printed date 2016-10-11, a Tuesday |
| 2016-10-12 | closed | `12-Oct-2016 Wednesday Moharram` | `NSE-HOL-PAGE-2016` | T1 | Printed date 2016-10-12, a Wednesday |
| 2016-10-30 | replacement blocks | `Pre Open* 18:15-18:22` order entry with its random-closure tail to the 18:30 open (the circular prints the pre-open row 1815-1823 with `* Random closure in last one minute`); `Normal Market/LPM 18:30-19:30`; `Closing Session 19:40-19:50` | `NSE-CIRC-2016-56` | T1 | circular 56/2016 (2016-10-17) states `a special live trading session shall be held on Sunday, October 30, 2016 on account of Muhurat trading on Diwali as per the following schedule`; the row restates the printed schedule as one replacement day, the order-entry leg stopping at 18:22, the earliest second the closure can fire; the circular's `Block Deal Session 1830-1905` and `Call Auction Illiquid session 1835-1920` run inside the Normal Market hours and state no edge the envelope lacks |
| 2016-10-31 | closed | `31-Oct-2016 Monday Diwali-Balipratipada` | `NSE-HOL-PAGE-2016` | T1 | Printed date 2016-10-31, a Monday |
| 2016-11-14 | closed | `14-Nov-2016 Monday Gurunanak Jayanti` | `NSE-HOL-PAGE-2016` | T1 | Printed date 2016-11-14, a Monday |

### 2017

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2017-01-26 | closed | `26-Jan-2017 Thursday Republic Day` | `NSE-HOL-PAGE-2017` | T1 | Printed date 2017-01-26, a Thursday |
| 2017-02-24 | closed | `24-Feb-2017 Friday Mahashivratri` | `NSE-HOL-PAGE-2017` | T1 | Printed date 2017-02-24, a Friday |
| 2017-03-13 | closed | `13-Mar-2017 Monday Holi` | `NSE-HOL-PAGE-2017` | T1 | Printed date 2017-03-13, a Monday |
| 2017-04-04 | closed | `04-Apr-2017 Tuesday Ram Navami` | `NSE-HOL-PAGE-2017` | T1 | Printed date 2017-04-04, a Tuesday |
| 2017-04-14 | closed | `14-Apr-2017 Friday Dr.Baba Saheb Ambedkar Jayanti/ Good Friday` | `NSE-HOL-PAGE-2017` | T1 | Printed date 2017-04-14, a Friday |
| 2017-05-01 | closed | `01-May-2017 Monday Maharashtra Day` | `NSE-HOL-PAGE-2017` | T1 | Printed date 2017-05-01, a Monday |
| 2017-06-26 | closed | `26-Jun-2017 Monday Id-Ul-Fitr (Ramzan ID)` | `NSE-HOL-PAGE-2017` | T1 | Printed date 2017-06-26, a Monday |
| 2017-08-15 | closed | `15-Aug-2017 Tuesday Independence Day` | `NSE-HOL-PAGE-2017` | T1 | Printed date 2017-08-15, a Tuesday |
| 2017-08-25 | closed | `25-Aug-2017 Friday Ganesh Chaturthi` | `NSE-HOL-PAGE-2017` | T1 | Printed date 2017-08-25, a Friday |
| 2017-10-02 | closed | `02-Oct-2017 Monday Mahatama Gandhi Jayanti` | `NSE-HOL-PAGE-2017` | T1 | Printed date 2017-10-02, a Monday |
| 2017-10-19 | unsourced | `19-Oct-2017 Thursday Diwali-Laxmi Pujan*` with the page's footnote `Muhurat Trading will be conducted. Timings of Muhurat Trading shall be notified subsequently.` | `NSE-HOL-PAGE-2017` | T1 | Printed date 2017-10-19, a Thursday; the day's special session has no published instants, so the date is withheld rather than closed |
| 2017-10-20 | closed | `20-Oct-2017 Friday Diwali-Balipratipada` | `NSE-HOL-PAGE-2017` | T1 | Printed date 2017-10-20, a Friday |
| 2017-12-25 | closed | `25-Dec-2017 Monday Christmas` | `NSE-HOL-PAGE-2017` | T1 | Printed date 2017-12-25, a Monday |

### 2019

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2019-03-04 | closed | `04-Mar-2019 Monday Mahashivratri` | `NSE-HOL-PAGE-2019` | T1 | Printed date 2019-03-04, a Monday |
| 2019-03-21 | closed | `21-Mar-2019 Thursday Holi` | `NSE-HOL-PAGE-2019` | T1 | Printed date 2019-03-21, a Thursday |
| 2019-04-17 | closed | `17-Apr-2019 Wednesday Mahavir Jayanti` | `NSE-HOL-PAGE-2019` | T1 | Printed date 2019-04-17, a Wednesday |
| 2019-04-19 | closed | `19-Apr-2019 Friday Good Friday` | `NSE-HOL-PAGE-2019` | T1 | Printed date 2019-04-19, a Friday |
| 2019-04-29 | closed | `29-Apr-2019 Monday Parliamentary Elections` | `NSE-HOL-PAGE-2019` | T1 | Printed date 2019-04-29, a Monday |
| 2019-05-01 | closed | `01-May-2019 Wednesday Maharashtra Day` | `NSE-HOL-PAGE-2019` | T1 | Printed date 2019-05-01, a Wednesday |
| 2019-06-05 | closed | `05-Jun-2019 Wednesday Id-Ul-Fitr (Ramzan ID)` | `NSE-HOL-PAGE-2019` | T1 | Printed date 2019-06-05, a Wednesday |
| 2019-08-12 | closed | `12-Aug-2019 Monday Bakri Id` | `NSE-HOL-PAGE-2019` | T1 | Printed date 2019-08-12, a Monday |
| 2019-08-15 | closed | `15-Aug-2019 Thursday Independence Day` | `NSE-HOL-PAGE-2019` | T1 | Printed date 2019-08-15, a Thursday |
| 2019-09-02 | closed | `02-Sep-2019 Monday Ganesh Chaturthi` | `NSE-HOL-PAGE-2019` | T1 | Printed date 2019-09-02, a Monday |
| 2019-09-10 | closed | `10-Sep-2019 Tuesday Moharram` | `NSE-HOL-PAGE-2019` | T1 | Printed date 2019-09-10, a Tuesday |
| 2019-10-02 | closed | `02-Oct-2019 Wednesday Mahatma Gandhi Jayanti` | `NSE-HOL-PAGE-2019` | T1 | Printed date 2019-10-02, a Wednesday |
| 2019-10-08 | closed | `08-Oct-2019 Tuesday Dasera` | `NSE-HOL-PAGE-2019` | T1 | Printed date 2019-10-08, a Tuesday |
| 2019-10-27 | unsourced | the weekend list's `27-Oct-2019 Sunday Diwali-Laxmi Pujan*` with the page's footnote `Muhurat Trading will be conducted. Timings of Muhurat Trading shall be notified subsequently.` | `NSE-HOL-PAGE-2019` | T1 | Printed Muhurat date 2019-10-27, a Sunday; the announced session has no published instants, so the day is not the audited-normal weekend closure the normal week alone would claim |
| 2019-10-28 | closed | `28-Oct-2019 Monday Diwali-Balipratipada` | `NSE-HOL-PAGE-2019` | T1 | Printed date 2019-10-28, a Monday |
| 2019-11-12 | closed | `12-Nov-2019 Tuesday Gurunanak Jayanti` | `NSE-HOL-PAGE-2019` | T1 | Printed date 2019-11-12, a Tuesday |
| 2019-12-25 | closed | `25-Dec-2019 Wednesday Christmas` | `NSE-HOL-PAGE-2019` | T1 | Printed date 2019-12-25, a Wednesday |

### 2020

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2020-02-21 | closed | `21-Feb-2020 Friday Mahashivratri` | `NSE-HOL-PAGE-2020` | T1 | Printed date 2020-02-21, a Friday |
| 2020-03-10 | closed | `10-Mar-2020 Tuesday Holi` | `NSE-HOL-PAGE-2020` | T1 | Printed date 2020-03-10, a Tuesday |
| 2020-04-02 | closed | `02-Apr-2020 Thursday Ram Navami` | `NSE-HOL-PAGE-2020` | T1 | Printed date 2020-04-02, a Thursday |
| 2020-04-06 | closed | `06-Apr-2020 Monday Mahavir Jayanti` | `NSE-HOL-PAGE-2020` | T1 | Printed date 2020-04-06, a Monday |
| 2020-04-10 | closed | `10-Apr-2020 Friday Good Friday` | `NSE-HOL-PAGE-2020` | T1 | Printed date 2020-04-10, a Friday |
| 2020-04-14 | closed | `14-Apr-2020 Tuesday Dr.Baba Saheb Ambedkar Jayanti` | `NSE-HOL-PAGE-2020` | T1 | Printed date 2020-04-14, a Tuesday |
| 2020-05-01 | closed | `01-May-2020 Friday Maharashtra Day` | `NSE-HOL-PAGE-2020` | T1 | Printed date 2020-05-01, a Friday |
| 2020-05-25 | closed | `25-May-2020 Monday Id-Ul-Fitr (Ramzan ID)` | `NSE-HOL-PAGE-2020` | T1 | Printed date 2020-05-25, a Monday |
| 2020-10-02 | closed | `02-Oct-2020 Friday Mahatma Gandhi Jayanti` | `NSE-HOL-PAGE-2020` | T1 | Printed date 2020-10-02, a Friday |
| 2020-11-14 | replacement blocks | `Block deal session 17:45-18:00`; `Pre Open 18:00-18:07` order entry with its random-closure tail to the 18:15 open (the circular prints the pre-open row 1800-1808 with `* Random closure in last one minute`); `Normal Market 18:15-19:15`; `Closing Session 19:25-19:35`; corroborated window-for-window by `NSE-MUHURAT-PAGE-2020` | `NSE-CIRC-2020-98` | T1 | circular 98/2020 (2020-11-02) names "Saturday, November 14, 2020"; the row restates the printed schedule as one replacement day, the order-entry leg stopping at 18:07, the earliest second the closure can fire |
| 2020-11-16 | closed | `16-Nov-2020 Monday Diwali-Balipratipada` | `NSE-HOL-PAGE-2020` | T1 | Printed date 2020-11-16, a Monday |
| 2020-11-30 | closed | `30-Nov-2020 Monday Gurunanak Jayanti` | `NSE-HOL-PAGE-2020` | T1 | Printed date 2020-11-30, a Monday |
| 2020-12-25 | closed | `25-Dec-2020 Friday Christmas` | `NSE-HOL-PAGE-2020` | T1 | Printed date 2020-12-25, a Friday |

### 2021

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2021-01-26 | closed | `26-Jan-2021 Tuesday Republic Day` | `NSE-HOL-PAGE-2021` | T1 | Printed date 2021-01-26, a Tuesday |
| 2021-03-11 | closed | `11-Mar-2021 Thursday Mahashivratri` | `NSE-HOL-PAGE-2021` | T1 | Printed date 2021-03-11, a Thursday |
| 2021-03-29 | closed | `29-Mar-2021 Monday Holi` | `NSE-HOL-PAGE-2021` | T1 | Printed date 2021-03-29, a Monday |
| 2021-04-02 | closed | `02-Apr-2021 Friday Good Friday` | `NSE-HOL-PAGE-2021` | T1 | Printed date 2021-04-02, a Friday |
| 2021-04-14 | closed | `14-Apr-2021 Wednesday Dr.Baba Saheb Ambedkar Jayanti` | `NSE-HOL-PAGE-2021` | T1 | Printed date 2021-04-14, a Wednesday |
| 2021-04-21 | closed | `21-Apr-2021 Wednesday Ram Navami` | `NSE-HOL-PAGE-2021` | T1 | Printed date 2021-04-21, a Wednesday |
| 2021-05-13 | closed | `13-May-2021 Thursday Id-Ul-Fitr (Ramzan ID)` | `NSE-HOL-PAGE-2021` | T1 | Printed date 2021-05-13, a Thursday |
| 2021-07-21 | closed | `21-Jul-2021 Wednesday Bakri Id` | `NSE-HOL-PAGE-2021` | T1 | Printed date 2021-07-21, a Wednesday |
| 2021-08-19 | closed | `19-Aug-2021 Thursday Moharram` | `NSE-HOL-PAGE-2021` | T1 | Printed date 2021-08-19, a Thursday |
| 2021-09-10 | closed | `10-Sep-2021 Friday Ganesh Chaturthi` | `NSE-HOL-PAGE-2021` | T1 | Printed date 2021-09-10, a Friday |
| 2021-10-15 | closed | `15-Oct-2021 Friday Dussehra` | `NSE-HOL-PAGE-2021` | T1 | Printed date 2021-10-15, a Friday |
| 2021-11-05 | closed | `05-Nov-2021 Friday Diwali-Balipratipada` | `NSE-HOL-PAGE-2021` | T1 | Printed date 2021-11-05, a Friday |
| 2021-11-19 | closed | `19-Nov-2021 Friday Gurunanak Jayanti` | `NSE-HOL-PAGE-2021` | T1 | Printed date 2021-11-19, a Friday |

### 2022

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2022-01-26 | closed | `26-Jan-2022 Wednesday Republic Day` | `NSE-HOL-PAGE-2022` | T1 | Printed date 2022-01-26, a Wednesday; corroborated by `NSE-HOLMASTER-2022` |
| 2022-03-01 | closed | `01-Mar-2022 Tuesday Mahashivratri` | `NSE-HOL-PAGE-2022` | T1 | Printed date 2022-03-01, a Tuesday; corroborated by `NSE-HOLMASTER-2022` |
| 2022-03-18 | closed | `18-Mar-2022 Friday Holi` | `NSE-HOL-PAGE-2022` | T1 | Printed date 2022-03-18, a Friday; corroborated by `NSE-HOLMASTER-2022` |
| 2022-04-14 | closed | `14-Apr-2022 Thursday Dr.Baba Saheb Ambedkar Jayanti/Mahavir Jayan` | `NSE-HOL-PAGE-2022` | T1 | Printed date 2022-04-14, a Thursday; corroborated by `NSE-HOLMASTER-2022` |
| 2022-04-15 | closed | `15-Apr-2022 Friday Good Friday` | `NSE-HOL-PAGE-2022` | T1 | Printed date 2022-04-15, a Friday; corroborated by `NSE-HOLMASTER-2022` |
| 2022-05-03 | closed | `03-May-2022 Tuesday Id-Ul-Fitr (Ramzan ID)` | `NSE-HOL-PAGE-2022` | T1 | Printed date 2022-05-03, a Tuesday; corroborated by `NSE-HOLMASTER-2022` |
| 2022-08-09 | closed | `09-Aug-2022 Tuesday Moharram` | `NSE-HOL-PAGE-2022` | T1 | Printed date 2022-08-09, a Tuesday; corroborated by `NSE-HOLMASTER-2022` |
| 2022-08-15 | closed | `15-Aug-2022 Monday Independence Day` | `NSE-HOL-PAGE-2022` | T1 | Printed date 2022-08-15, a Monday; corroborated by `NSE-HOLMASTER-2022` |
| 2022-08-31 | closed | `31-Aug-2022 Wednesday Ganesh Chaturthi` | `NSE-HOL-PAGE-2022` | T1 | Printed date 2022-08-31, a Wednesday; corroborated by `NSE-HOLMASTER-2022` |
| 2022-10-05 | closed | `05-Oct-2022 Wednesday Dussehra` | `NSE-HOL-PAGE-2022` | T1 | Printed date 2022-10-05, a Wednesday; corroborated by `NSE-HOLMASTER-2022` |
| 2022-10-24 | replacement blocks | `Block deal session 17:45-18:00`; `Pre Open* 18:00-18:07` order entry with its random-closure tail to the 18:15 open (the circular prints the pre-open row 1800-1808); `Normal Market 18:15-19:15`; `Closing Session 19:25-19:35` (the 2020 schedule reprinted) | `NSE-CIRC-2022-124` | T1 | circular 124/2022 (2022-10-11) names "Monday, October 24, 2022"; the row restates the printed schedule as one replacement day, the order-entry leg stopping at 18:07 |
| 2022-10-26 | closed | `26-Oct-2022 Wednesday Diwali-Balipratipada` | `NSE-HOL-PAGE-2022` | T1 | Printed date 2022-10-26, a Wednesday; corroborated by `NSE-HOLMASTER-2022` |
| 2022-11-08 | closed | `08-Nov-2022 Tuesday Gurunanak Jayanti` | `NSE-HOL-PAGE-2022` | T1 | Printed date 2022-11-08, a Tuesday; corroborated by `NSE-HOLMASTER-2022` |

### 2023

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2023-01-26 | closed | `{"tradingDate": "26-Jan-2023", "weekDay": "Thursday", "description": "Republic Day"}` | `NSE-HOLMASTER-2023` | T2 | Printed date 2023-01-26, a Thursday |
| 2023-03-07 | closed | `07-Mar-2023 Tuesday Holi` | `NSE-HOLMASTER-2023` | T2 | Printed date 2023-03-07, a Tuesday |
| 2023-03-30 | closed | `30-Mar-2023 Thursday Ram Navami` | `NSE-HOLMASTER-2023` | T2 | Printed date 2023-03-30, a Thursday |
| 2023-04-04 | closed | `04-Apr-2023 Tuesday Mahavir Jayanti` | `NSE-HOLMASTER-2023` | T2 | Printed date 2023-04-04, a Tuesday |
| 2023-04-07 | closed | `07-Apr-2023 Friday Good Friday` | `NSE-HOLMASTER-2023` | T2 | Printed date 2023-04-07, a Friday |
| 2023-04-14 | closed | `14-Apr-2023 Friday Dr. Baba Saheb Ambedkar Jayanti` | `NSE-HOLMASTER-2023` | T2 | Printed date 2023-04-14, a Friday |
| 2023-05-01 | closed | `01-May-2023 Monday Maharashtra Day` | `NSE-HOLMASTER-2023` | T2 | Printed date 2023-05-01, a Monday |
| 2023-06-28 | closed | `28-Jun-2023 Wednesday Bakri Id` | `NSE-HOLMASTER-2023` | T2 | Printed date 2023-06-28, a Wednesday |
| 2023-08-15 | closed | `15-Aug-2023 Tuesday Independence Day` | `NSE-HOLMASTER-2023` | T2 | Printed date 2023-08-15, a Tuesday |
| 2023-09-19 | closed | `19-Sep-2023 Tuesday Ganesh Chaturthi` | `NSE-HOLMASTER-2023` | T2 | Printed date 2023-09-19, a Tuesday |
| 2023-10-02 | closed | `02-Oct-2023 Monday Mahatma Gandhi Jayanti` | `NSE-HOLMASTER-2023` | T2 | Printed date 2023-10-02, a Monday |
| 2023-10-24 | closed | `24-Oct-2023 Tuesday Dussehra` | `NSE-HOLMASTER-2023` | T2 | Printed date 2023-10-24, a Tuesday |
| 2023-11-12 | replacement blocks | `Block deal session 17:45-18:00`; `Pre Open* 18:00-18:07` order entry with its random-closure tail to the 18:15 open (the circular prints the pre-open row 1800-1808); `Normal Market 18:15-19:15`; `Closing Session 19:25-19:35` (the 2020 schedule reprinted) | `NSE-CIRC-2023-139` | T1 | circular 139/2023 (2023-10-27) names "Sunday, November 12, 2023"; the row restates the printed schedule as one replacement day, the order-entry leg stopping at 18:07 |
| 2023-11-14 | closed | `14-Nov-2023 Tuesday Diwali-Balipratipada` | `NSE-HOLMASTER-2023` | T2 | Printed date 2023-11-14, a Tuesday |
| 2023-11-27 | closed | `27-Nov-2023 Monday Gurunanak Jayanti` | `NSE-HOLMASTER-2023` | T2 | Printed date 2023-11-27, a Monday |
| 2023-12-25 | closed | `25-Dec-2023 Monday Christmas` | `NSE-HOLMASTER-2023` | T2 | Printed date 2023-12-25, a Monday |

### 2024

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2024-01-26 | closed | `26-Jan-2024 Friday Republic Day` | `NSE-HOLMASTER-2024` | T2 | Printed date 2024-01-26, a Friday |
| 2024-03-08 | closed | `08-Mar-2024 Friday Mahashivratri` | `NSE-HOLMASTER-2024` | T2 | Printed date 2024-03-08, a Friday |
| 2024-03-25 | closed | `25-Mar-2024 Monday Holi` | `NSE-HOLMASTER-2024` | T2 | Printed date 2024-03-25, a Monday |
| 2024-03-29 | closed | `29-Mar-2024 Friday Good Friday` | `NSE-HOLMASTER-2024` | T2 | Printed date 2024-03-29, a Friday |
| 2024-04-11 | closed | `11-Apr-2024 Thursday Id-Ul-Fitr (Ramadan Eid)` | `NSE-HOLMASTER-2024` | T2 | Printed date 2024-04-11, a Thursday |
| 2024-04-17 | closed | `17-Apr-2024 Wednesday Shri Ram Navmi` | `NSE-HOLMASTER-2024` | T2 | Printed date 2024-04-17, a Wednesday |
| 2024-05-01 | closed | `01-May-2024 Wednesday Maharashtra Day` | `NSE-HOLMASTER-2024` | T2 | Printed date 2024-05-01, a Wednesday |
| 2024-06-17 | closed | `17-Jun-2024 Monday Bakri Id` | `NSE-HOLMASTER-2024` | T2 | Printed date 2024-06-17, a Monday |
| 2024-07-17 | closed | `17-Jul-2024 Wednesday Moharram` | `NSE-HOLMASTER-2024` | T2 | Printed date 2024-07-17, a Wednesday |
| 2024-08-15 | closed | `15-Aug-2024 Thursday Independence Day` | `NSE-HOLMASTER-2024` | T2 | Printed date 2024-08-15, a Thursday |
| 2024-10-02 | closed | `02-Oct-2024 Wednesday Mahatma Gandhi Jayanti` | `NSE-HOLMASTER-2024` | T2 | Printed date 2024-10-02, a Wednesday |
| 2024-11-01 | replacement blocks | `Block deal session 17:30-17:45`; `Pre Open* 17:45-17:52` order entry with its random-closure tail to the 18:00 open (the circular prints the pre-open row 1745-1800 and bounds its order-entry period at 1745 to 1753 hours with the random closure in its last minute); `Normal Market 18:00-19:00`; `Closing Session 19:10-19:20` | `NSE-CIRC-2024-147` | T1 | circular 147/2024 (2024-10-19) names "Friday, November 01, 2024"; the row restates the printed schedule as one replacement day, the order-entry leg stopping at 17:52, the earliest second the closure can fire |
| 2024-11-15 | closed | `15-Nov-2024 Friday Gurunanak Jayanti` | `NSE-HOLMASTER-2024` | T2 | Printed date 2024-11-15, a Friday |
| 2024-12-25 | closed | `25-Dec-2024 Wednesday Christmas` | `NSE-HOLMASTER-2024` | T2 | Printed date 2024-12-25, a Wednesday |

### 2025

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2025-02-26 | closed | `26 Feb Wednesday Mahashivratri` | `NSE-HOL-2025` | T1 | Printed date 2025-02-26, a Wednesday |
| 2025-03-14 | closed | `14 Mar Friday Holi` | `NSE-HOL-2025` | T1 | Printed date 2025-03-14, a Friday |
| 2025-03-31 | closed | `31 Mar Monday Eid-ul-Fitr (Ramadan Eid)` | `NSE-HOL-2025` | T1 | Printed date 2025-03-31, a Monday |
| 2025-04-10 | closed | `10 Apr Thursday Shri Mahavir Jayanti` | `NSE-HOL-2025` | T1 | Printed date 2025-04-10, a Thursday |
| 2025-04-14 | closed | `14 Apr Monday Dr. Babasaheb Ambedkar Jayanti` | `NSE-HOL-2025` | T1 | Printed date 2025-04-14, a Monday |
| 2025-04-18 | closed | `18 Apr Friday Good Friday` | `NSE-HOL-2025` | T1 | Printed date 2025-04-18, a Friday |
| 2025-05-01 | closed | `01 May Thursday Maharashtra Day` | `NSE-HOL-2025` | T1 | Printed date 2025-05-01, a Thursday |
| 2025-08-15 | closed | `15 Aug Friday Independence Day` | `NSE-HOL-2025` | T1 | Printed date 2025-08-15, a Friday |
| 2025-08-27 | closed | `27 Aug Wednesday Ganesh Chaturthi` | `NSE-HOL-2025` | T1 | Printed date 2025-08-27, a Wednesday |
| 2025-10-02 | closed | `02 Oct Thursday Mahatma Gandhi Jayanti / Dussehra` | `NSE-HOL-2025` | T1 | Printed date 2025-10-02, a Thursday |
| 2025-10-21 | replacement blocks | `Block deal session 13:15-13:30`; `Pre Open* 13:30-13:37` order entry with its random-closure tail to the 13:45 open (the circular prints the pre-open row 1330-1345 and bounds the random closure in its last minute, i.e. 1337 to 1338 hours); `Normal Market 13:45-14:45`; `Closing Session 14:55-15:05` | `NSE-CIRC-2025-124` | T1 | circular 124/2025 (2025-09-22) names "Tuesday, October 21, 2025"; the row restates the printed schedule as one replacement day, the order-entry leg stopping at 13:37 as the circular's own footnote bounds it |
| 2025-10-22 | closed | `22 Oct Wednesday Diwali Balipratipada` | `NSE-HOL-2025` | T1 | Printed date 2025-10-22, a Wednesday |
| 2025-11-05 | closed | `05 Nov Wednesday Prakash Gurpurab Sri Guru Nanak Dev` | `NSE-HOL-2025` | T1 | Printed date 2025-11-05, a Wednesday |
| 2025-12-25 | closed | `25 Dec Thursday Christmas` | `NSE-HOL-2025` | T1 | Printed date 2025-12-25, a Thursday |

### 2026

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2026-01-26 | closed | `26th JAN MONDAY Republic Day` | `NSE-HOL-2026` | T1 | Printed date 2026-01-26, a Monday |
| 2026-03-03 | closed | `03rd MAR TUESDAY Holi` | `NSE-HOL-2026` | T1 | Printed date 2026-03-03, a Tuesday |
| 2026-03-26 | closed | `26th MAR THURSDAY Shri Ram Navami` | `NSE-HOL-2026` | T1 | Printed date 2026-03-26, a Thursday |
| 2026-03-31 | closed | `31st MAR TUESDAY Shri Mahavir Jayanti` | `NSE-HOL-2026` | T1 | Printed date 2026-03-31, a Tuesday |
| 2026-04-03 | closed | `03rd APR FRIDAY Good Friday` | `NSE-HOL-2026` | T1 | Printed date 2026-04-03, a Friday |
| 2026-04-14 | closed | `14th APR TUESDAY Dr. Baba Saheb Ambedkar Jayanti` | `NSE-HOL-2026` | T1 | Printed date 2026-04-14, a Tuesday |
| 2026-05-01 | closed | `01st MAY FRIDAY Maharashtra Day` | `NSE-HOL-2026` | T1 | Printed date 2026-05-01, a Friday |
| 2026-05-28 | closed | `28th MAY THURSDAY Bakri Id` | `NSE-HOL-2026` | T1 | Printed date 2026-05-28, a Thursday |
| 2026-06-26 | closed | `26th JUN FRIDAY Muharram` | `NSE-HOL-2026` | T1 | Printed date 2026-06-26, a Friday |
| 2026-09-14 | closed | `14th SEP MONDAY Ganesh Chaturthi` | `NSE-HOL-2026` | T1 | Printed date 2026-09-14, a Monday |
| 2026-10-02 | closed | `02nd OCT FRIDAY Mahatma Gandhi Jayanti` | `NSE-HOL-2026` | T1 | Printed date 2026-10-02, a Friday |
| 2026-10-20 | closed | `20th OCT TUESDAY Dussehra` | `NSE-HOL-2026` | T1 | Printed date 2026-10-20, a Tuesday |
| 2026-11-08 | unsourced | the footnote `Muhurat Trading will be conducted on Sunday, November 08, 2026. Timings of Muhurat Trading shall be notified subsequently` | `NSE-HOL-2026` | T1 | Printed Muhurat date 2026-11-08, a Sunday; the announced session has no published instants, so the day is not the audited-normal weekend closure the normal week alone would claim |
| 2026-11-10 | closed | `10th NOV TUESDAY Diwali-Balipratipada` | `NSE-HOL-2026` | T1 | Printed date 2026-11-10, a Tuesday |
| 2026-11-24 | closed | `24th NOV TUESDAY Prakash Gurpurab Sri Guru Nanak Dev` | `NSE-HOL-2026` | T1 | Printed date 2026-11-24, a Tuesday |
| 2026-12-25 | closed | `25th DEC FRIDAY Christmas` | `NSE-HOL-2026` | T1 | Printed date 2026-12-25, a Friday |

### Documents

| Document | Window | Replay or service URL | Capture or retrieval, UTC | Tier | sha256 |
|---|---|---|---|---|---|
| `NSE-CIRC-2010-61` | 2010-01-01..2011-12-31 | <https://web.archive.org/web/20100102003811id_/http://nseindia.com/content/press/HOLIDAYS.zip> (inner `HOLIDAYS/eq_holidays.pdf`, circular 061) | Wayback `id_` replay of capture `20100102003811`, retrieved 2026-09-29 UTC | T1 | `04b0f872a2de4ddec0091aada244abb68e45abd569f312f55eba05f34a9c8f87` |
| `NSE-CIRC-2010-121` | 2010-11-05 | <https://web.archive.org/web/20101122005959id_/http://nseindia.com/content/circulars/cmtr16062.pdf> (circular 121/2010 dated 2010-10-20) | Wayback `id_` replay of capture `20101122005959`, retrieved 2026-10-06 UTC | T1 | `0e39bfe8a687d62666b3811e2854d98d996df05374307339969db6c35fbbc255` |
| `NSE-HOL-PAGE-2011` | 2010-01-01..2011-12-31 | <https://web.archive.org/web/20110708222938id_/http://nseindia.com/products/content/equities/equities/mrkt_timing_holidays.htm> | Wayback `id_` replay of capture `20110708222938`, retrieved 2026-09-29 UTC | T1 | `f679f40ed4f347764fcbf813774d38a1351524c25c7bb6a37c34cfb67e75132f` |
| `NSE-CIRC-2011-126` | 2010-01-01..2011-12-31 | <https://web.archive.org/web/20131021135725id_/http://www.nseindia.com/content/equities/eq_holidays.pdf> (circular 126; corroborates `NSE-HOL-PAGE-2011` date-for-date) | Wayback `id_` replay of capture `20131021135725`, retrieved 2026-09-29 UTC | T1 | `d52836223ceba71a6fd1f1ab73fade75df122ce57f7d98a63a3918ad66908ed7` |
| `NSE-CIRC-2011-66` | 2010-01-01..2017-12-31 | <https://web.archive.org/web/20131001001639id_/http://www.nseindia.com/content/circulars/CMTR19539.pdf> (circular 66/2011 dated 2011-12-09, the 2012 list) | Wayback `id_` replay of capture `20131001001639`, retrieved 2026-10-06 UTC | T1 | `4f798db91049897c90cf381ac36c7c8a0315b7144dcf90dc1f6eb3d854e1055d` |
| `NSE-CIRC-2012-79` | 2013-01-01..2017-12-31 | <https://web.archive.org/web/20160121003215id_/http://www.nseindia.com/content/press/consolidated_holidays_2013.pdf> (circular 79/2012 dated 2012-12-17) | Wayback `id_` replay of capture `20160121003215`, retrieved 2026-09-29 UTC | T1 | `a028c949c258e40fbaed853a4e64d5178c8b3ce7e1a80e45bcd3e709534730d9` |
| `NSE-HOL-PAGE-2014` | 2013-01-01..2017-12-31 | <https://web.archive.org/web/20140122082410id_/http://nseindia.com/products/content/equities/equities/mrkt_timing_holidays.htm> | Wayback `id_` replay of capture `20140122082410`, retrieved 2026-09-29 UTC | T1 | `17a29786b83c29383570bbebb0260789745ef6ae3525c35fa0ba0f79c36fdead` |
| `NSE-HOL-PAGE-2015` | 2013-01-01..2017-12-31 | <https://web.archive.org/web/20150102110045id_/http://nseindia.com/products/content/equities/equities/mrkt_timing_holidays.htm> | Wayback `id_` replay of capture `20150102110045`, retrieved 2026-09-29 UTC | T1 | `92604a7f407489808163fafe96d8cad55a9e269b5a8fe6d12ed81879f3d89884` |
| `NSE-CIRC-2015-65` | 2015-11-11 | <https://web.archive.org/web/20160216033210id_/http://www.nseindia.com/content/circulars/CMTR31047.pdf> (circular 65/2015 dated 2015-10-30) | Wayback `id_` replay of capture `20160216033210`, retrieved 2026-10-06 UTC | T1 | `060afe66d01e7d03098254bbd6fe772f485184684c3453e411dceb3339bd169d` |
| `NSE-HOL-PAGE-2016` | 2013-01-01..2017-12-31 | <https://web.archive.org/web/20160407194718id_/http://www.nseindia.com/products/content/equities/equities/mrkt_timing_holidays.htm> | Wayback `id_` replay of capture `20160407194718`, retrieved 2026-09-29 UTC | T1 | `117bce43b0be8609086f81516d8703afcdad960e6117b9fb8a48a77621d1b5fc` |
| `NSE-CIRC-2016-56` | 2016-10-30 | <https://web.archive.org/web/20170224214712id_/https://www.nseindia.com/content/circulars/CMTR33424.pdf> (circular 56/2016 dated 2016-10-17) | Wayback `id_` replay of capture `20170224214712`, retrieved 2026-10-06 UTC | T1 | `9e11622e7acd164c68ef56d0eae60014617b47747c9c4de7545399fd98ad4dfc` |
| `NSE-HOL-PAGE-2017` | 2013-01-01..2017-12-31 | <https://web.archive.org/web/20170505185052id_/https://www1.nseindia.com/products/content/equities/equities/mrkt_timing_holidays.htm> | Wayback `id_` replay of capture `20170505185052`, retrieved 2026-09-29 UTC | T1 | `622bd1c06c2a8c11f4751178ddb6692ab83d3e6cfdc9520cf804ff4e7c61df90` |
| `NSE-HOL-PAGE-2019` | 2019-01-01..2026-12-31 | <https://web.archive.org/web/20190614034618id_/https://www.nseindia.com/products/content/equities/equities/mrkt_timing_holidays.htm> | Wayback `id_` replay of capture `20190614034618`, retrieved 2026-09-29 UTC | T1 | `15dee1ef59a086149271a6d5f52c49cf35ebd6f4b91d9f74f1f29194b34e13a6` |
| `NSE-HOL-PAGE-2020` | 2019-01-01..2026-12-31 | <https://web.archive.org/web/20200113084255id_/https://www.nseindia.com/products-services/equity-market-timings-holidays> ("Holidays for the calendar year 2020 : Equities") | Wayback `id_` replay of capture `20200113084255`, retrieved 2026-09-29 UTC | T1 | `d1ffc25c371896d010d486008e9f314fce51728928ebae0ecc99bcf6e16396d2` |
| `NSE-HOL-PAGE-2021` | 2019-01-01..2026-12-31 | <https://web.archive.org/web/20210126030414id_/https://www1.nseindia.com/products/content/equities/equities/mrkt_timing_holidays.htm> | Wayback `id_` replay of capture `20210126030414`, retrieved 2026-09-29 UTC | T1 | `55f1e8458140161348dff1f80399eaaae300ca786d03a8fe6f472cc24d900c11` |
| `NSE-HOL-PAGE-2022` | 2019-01-01..2026-12-31 | <https://web.archive.org/web/20220813022100id_/https://www1.nseindia.com/products/content/equities/equities/mrkt_timing_holidays.htm> | Wayback `id_` replay of capture `20220813022100`, retrieved 2026-09-29 UTC | T1 | `f7fada7605861c924792bfe9474f5d32fcce799c2b4d3c6752daa253c1cdd5d7` |
| `NSE-HOLMASTER-2022` | 2019-01-01..2026-12-31 | <https://web.archive.org/web/20220110123106id_/https://www.nseindia.com/api/holiday-master?type=trading> (CM segment; corroborates `NSE-HOL-PAGE-2022`) | Wayback `id_` replay of capture `20220110123106`, retrieved 2026-09-29 UTC | T2 | `8bfa55ac1b4628b6ff7dfaf61ceb0dae47d54d0a6afd83a8ecd3fc7b68cac0cf` |
| `NSE-HOLMASTER-2023` | 2019-01-01..2026-12-31 | <https://web.archive.org/web/20230130033052id_/https://www.nseindia.com/api/holiday-master?type=trading> (CM segment) | Wayback `id_` replay of capture `20230130033052`, retrieved 2026-09-29 UTC | T2 | `ae9cb7d82bea1786351edbe18645d1dbc1acbe3476e7f582bcbc348ff450b6d2` |
| `NSE-HOLMASTER-2024` | 2019-01-01..2026-12-31 | <https://web.archive.org/web/20240118085508id_/https://www.nseindia.com/api/holiday-master?type=trading> (CM segment) | Wayback `id_` replay of capture `20240118085508`, retrieved 2026-09-29 UTC | T2 | `befd3029bd5e13fcc6cfc4f5024750f19df3f3790c4da17c25c76f4f098abb9f` |
| `NSE-HOL-2025` | 2025-01-01..2026-12-31 | <https://web.archive.org/web/20241223224845id_/https://nsearchives.nseindia.com/web/sites/default/files/2024-12/Holiday%20List%20Web%20Banner.jpg> | Wayback `id_` replay of capture `20241223224845`, retrieved 2026-09-28 UTC | T1 | `22e29e5a236591c9dd64e9bf1788703da77d9d4934dbbc617668618668e3a115` |
| `NSE-HOL-2026` | 2025-01-01..2026-12-31 | <https://web.archive.org/web/20260108142249id_/https://nsearchives.nseindia.com/web/banner/2025-12/TradingHolidayList_636x555__final_20251218152820.jpg?w=1200> | Wayback `id_` replay of capture `20260108142249`, retrieved 2026-09-28 UTC (banner file dated 2025-12-18) | T1 | `4920fdb026e210d0badc135577871cd6383559fe82d93d164c830b9ffd04a89e` |
| `NSE-TIMINGS-2025` | 2025-01-01..2026-12-31 | <https://web.archive.org/web/20250821072044id_/https://www.nseindia.com/api/cmsNote?url=exchange-communication-holidays-equities> | Wayback `id_` replay of capture `20250821072044`, retrieved 2026-09-28 UTC | T2 | `cb5f2beb9e685a58b4ba9cc1ad177912983ef88057c4aaac205c8eb256888a66` |
| `NSE-TIMINGS-2026` | 2025-01-01..2026-12-31 | <https://web.archive.org/web/20260916045525id_/https://www.nseindia.com/api/getNotes20?url=/resources/exchange-communication-holidays-equities> | Wayback `id_` replay of capture `20260916045525`, retrieved 2026-09-28 UTC | T2 | `20c69a1ca6fd37a969c3bb54e665bacb9b4844ffad4b1b7236feae7dbbd13f49` |
| `NSE-CIRC-2020-98` | 2020-11-14 | <https://web.archive.org/web/20201126150646id_/https://archives.nseindia.com/content/circulars/CMTR46230.pdf> (circular 98/2020 dated 2020-11-02) | Wayback `id_` replay of capture `20201126150646`, retrieved 2026-10-06 UTC | T1 | `dc545416272a0da2d7465e66b879f884fa6c49bdead5b9e0cc5ae15d1e5672be` |
| `NSE-MUHURAT-PAGE-2020` | 2020-11-14 (corroboration only; keys no row) | <https://web.archive.org/web/20201120015423id_/https://www.nseindia.com/muhurat-trading-session> (`Diwali Muhurat Trading Session on Saturday, November 14, 2020`; page line `Updated on: 12/11/2020`) | Wayback `id_` replay of capture `20201120015423`, retrieved 2026-10-06 UTC | T1 | `ddfe2eeb58d229ab0c5a4106ab9a18a98735d6887d7296268b03f2dc8e1dd953` |
| `NSE-CIRC-2022-124` | 2022-10-24 | <https://web.archive.org/web/20221129084650id_/https://www1.nseindia.com/content/circulars/CMTR54023.pdf> (circular 124/2022 dated 2022-10-11) | Wayback `id_` replay of capture `20221129084650`, retrieved 2026-10-06 UTC | T1 | `01e579eac8e2dd4a870a9ebc9f08d60bc26fae28d0d35cf58891bf2a2964e0c7` |
| `NSE-CIRC-2023-139` | 2023-11-12 | <https://web.archive.org/web/20231113171510id_/https://nsearchives.nseindia.com/content/circulars/CMTR59124.pdf> (circular 139/2023 dated 2023-10-27) | Wayback `id_` replay of capture `20231113171510`, retrieved 2026-10-06 UTC | T1 | `09f80430f3ec95aeaeeb98938124273892abccc38a7aaa440b38a6426b428c21` |
| `NSE-CIRC-2024-147` | 2024-11-01 | <https://web.archive.org/web/20241031135620id_/https://nsearchives.nseindia.com/content/circulars/CMTR64628.pdf> (circular 147/2024 dated 2024-10-19) | Wayback `id_` replay of capture `20241031135620`, retrieved 2026-10-06 UTC | T1 | `dd1b8bc34a4b3c2900002341c41e4b8f165af862a74fd2a5bfb1355b0afec4a8` |
| `NSE-CIRC-2025-124` | 2025-10-21 | <https://web.archive.org/web/20251118042606id_/https://nsearchives.nseindia.com/content/circulars/CMTR70319.pdf> (circular 124/2025 dated 2025-09-22) | Wayback `id_` replay of capture `20251118042606`, retrieved 2026-10-06 UTC | T1 | `19b7d88003cb05eb8c6abdc8d1bfad5df821a17e4f9ae36f75e327a68a48d258` |

The 2010-2024 rows were keyed on 2026-09-29 UTC from the fifteen artifacts in `holidays/raw/equities/nse_india/2010-2024/` (one Wayback `id_` replay per year, plus one corroborating circular), each directory with an `INDEX.md` carrying the same digests; all nineteen artifacts of the block live in the research store under the `2010-2024/` and `2025-2027/` directories. The circular PDFs' text layer spaces every glyph, so quotations from them below are extracted with the spacing normalized; the HTML pages quote directly. The four current artifacts are saved under `holidays/raw/equities/nse_india/2025-2027/` with an `INDEX.md` carrying the same digests. The two `NSE-TIMINGS` captures are the operator's own machine channel stating the market-timings grid, not the holiday dates; they corroborate that the session grid inside this window is the one the normal-week profile already models (pre-open 09:00, continuous 09:15–15:30, post-close to 16:00, with the CAS phases from the 2026 capture) and key no holiday row. Both are stored gzip-compressed exactly as replayed. The 2020-2025 Muhurat circulars and the corroboration page of 2026-10-06 UTC are saved under `holidays/raw/equities/nse_india/domain-lineage-2026-10-06/`, whose `INDEX.md` and `SHA256SUMS.txt` carry the same digests; the 2022, 2023, 2024 and 2025 PDFs are byte-identical to the working copies a 2026-10-05/06 session had already gathered under the store's `nse-muhurat/` directory, and their `id_`-replay digests above are the citable provenance. The 2010, 2015 and 2016 Muhurat circulars and the 2012 annual list — the second pass the same UTC day — are saved under `holidays/raw/equities/nse_india/muhurat-recovery-2026-10-06/` (`pdf/` subdirectory), whose `INDEX.md` and `SHA256SUMS.txt` carry the same digests, re-verified against the saved bytes on 2026-10-06 UTC; that `INDEX.md` also records the sweep method and the issuance-window negatives it closed.

**Why the window stops at 2026-12-31 is the operator's horizon, not a withholding.** NSE publishes the next year's list each December (the 2026 banner's file name is dated 2025-12-18); no "Trading Holiday List — Calendar Year 2027" exists as of the 2026-09-28 retrieval, and web search found none. Re-checked 2026-09-29 UTC on the operator's own pages: the live `Market Timings & Holidays` page (`holidays/raw/equities/nse_india/forward-2027/`, sha256 `f100e16e…`) carries no 2027 date, and the `holiday-master` API again answered an empty body, the standing refusal the rows' capture-based sourcing records below. Web search for the 2027 list returns only third-party projections, which key no row (LAW-PRIMARY-SOURCES). Re-checked again 2026-10-02 UTC (artifacts `nse_holidays_page.live-20261002T235833Z.html` and `nse_holiday_master.live-20261002T235833Z.json` and `INDEX-recheck-2026-10-03.md` under `holidays/raw/equities/nse_india/forward-2027/`): the live holidays page again carries zero `2027` mentions and the `holiday-master?type=CM` API again answered an empty body — the standing refusal. **Closing condition:** NSE's 2027 list publication, which extends the window to 2027-12-31. Re-checked monthly per LAW-WATCH.

## Sources

Row review: 2026-08-22 (UTC) is the date the ledger row was last reviewed as a
whole. Per-source retrieval dates were not recorded before the 2026-09-12
migration; where a later targeted review, capture or document date is recorded
beside a source below, that date governs for that source, and dates are added
as each source is re-verified.

- <https://www.nseindia.com/static/products-services/equity-market-pre-open> — NSE pre-open page. The session "is comprised of Order collection period and order matching period"; "the order collection period of 8* minutes shall be provided for order entry, modification and cancellation (* - System driven random closure between 7th and 8th minute)"; "order matching period starts immediately after completion of order collection period".
- <https://www.bseindia.com/markets/MarketInfo/DispNewNoticesCirculars?page=20101014-8> — BSE's operating notice for the same 2010-10-18 launch, printing the grid outright: Order Entry Period 9:00am–9:07/08am with "No trades are executed", Order Matching & Confirmation Period 9:08am–9:12am, Buffer Period 9:12am–9:15am.
- <https://www.nseindia.com/static/products-services/closing-auction-session> — NSE Closing Auction Session page.
- <https://www.sebi.gov.in/legal/circulars/jan-2026/introduction-of-closing-auction-session-cas-in-the-equity-cash-segment-and-certain-modifications-in-the-pre-open-auction-session_99122.html> — SEBI circular 99122, introducing CAS effective 2026-08-03.
- <https://www.bseindia.com/markets/MarketInfo/DispNewNoticesCirculars?page=20260801-1> — BSE notice for the CAS cutover.
- <https://www.bseindia.com/downloads/UploadDocs/Notices/20260610-41/20260610-41.pdf> — BSE detailed operating guidelines, stating that securities not eligible for CAS "shall continue to be available for continuous trading till 3:30pm", which is why the whole 15:15–15:35 window stays tradeable venue-wide.
- <https://nsearchives.nseindia.com/global/content/about_us/NSEIL_Annual_Report_2011.pdf> — NSE annual report 2011, covering circular NSE/CMTR/15981.
- <https://nsearchives.nseindia.com/content/circulars/cmtr19013.pdf> — NSE circular NSE/CMTR/19013, the 2011-10-03 post-close change.
- <https://www.bseindia.com/markets/MarketInfo/DispNewNoticesCirculars?page=20091217-15> — BSE notice 20091217-15, the joint December-2009 announcement of the 09:55 → 09:00 move.
- <https://nsearchives.nseindia.com/content/press/17122009.htm> — NSE press release of 2009-12-17, the same move.

## Gaps and residual risks

- **Seven Muhurat dates are withheld `Unsourced` rows** (2011-10-26, 2012-11-13, 2013-11-03, 2014-10-23, 2017-10-19, 2019-10-27 and the 2026-11-08 banner date), the only withheld dates in the audited windows; the 2026-10-09 UTC re-check closed negative on every remaining retrieval angle (recorded below). What is missing is each special session's instants, which the captured holiday lists state are "notified/subsequently". Closing condition: the operator's Muhurat circular for the year, which turns a withheld holiday into a replacement-blocks day and a withheld weekend into a stated special session (the block engine of #93 encodes it — the eight recovered circulars of 2026-10-06 UTC are the worked example). Queries inside the windows on those dates refuse rather than answer, which is the honest failure mode (LAW-HOLIDAY-SCOPE).
- **The Muhurat circular hunt, recorded (2026-10-06 UTC; artifacts under `holidays/raw/equities/nse_india/domain-lineage-2026-10-06/` and `muhurat-recovery-2026-10-06/` in the research store).** The first pass recovered the 2020, 2022, 2023, 2024 and 2025 circulars through the operator's own index pages and the archive: the 2020 one through the operator's own `muhurat-trading-session` page (capture 2020-11-20), which names `archives.nseindia.com/content/circulars/CMTR46230.pdf`; the 2022, 2023 and 2024 ones by their `nsearchives`/`www1` filenames against the Wayback CDX; the 2025 one the same way. The second pass the same UTC day swept the whole captured `CMTR`/`cmtr` circular series on all four hosts (510 unique captured PDFs, `cdx_*_cmtr*.txt`), cross-referenced against the download-number range of every Muhurat issuance window anchored by the dated circulars the recovered bytes themselves state, and recovered the 2010 (CMTR16062, capture 20101122005959), 2015 (CMTR31047, capture 20160216033210) and 2016 (CMTR33424, capture 20170224214712) circulars plus the 2012 annual list (CMTR19539, capture 20131001001639). The remaining years closed negative on the same channel family, two angles each:
  - **2011.** The operator's own `circ_latest.htm` capture 2011-10-21 names the CM circular (`NSE/CMTR/19187`, "Muhurat Trading session on account of Diwali", 2011-10-20) — but the PDF itself has no Wayback capture at any host form (the second pass's full-prefix sweeps on all four hosts, upper- and lower-case paths, included it; its issuance number sits between 19075 of 2011-10-05 and 19431 of 2011-11-24, neither neighbour being it). The F&O twin `FAOP19189.pdf` IS captured (replay of 2017-08-08, saved beside the artifacts) and states `Normal Market open 16:45 hrs / Normal Market close 18:00 hrs` for the **F&O segment** — a different segment's schedule, which keys no CM row and is recorded here so a future pass does not mistake it for the CM one.
  - **2013.** The department year-page `circ_cmt2013.htm` has zero captures; `circ_latest.htm`'s captures around the window are 2013-10-14 (before issuance) and 2014-01-23 (after the rolling page had moved on). The second pass's issuance window (≈ 24200-24700, between 23588/24121 of 2013-09 and 25324 of 2013-12-28) holds no captures.
  - **2014.** The second pass's issuance window (≈ 27000-27500, between 26919 of 2014-07 and 28337 of 2014-12-12) holds no captures; the year-list pages of the era are uncaptured, 404 or 302 in every Diwali window (the 2015 redesign moved the site to `www1` and then the new portal). The 2015-era `FAOP25325.pdf` capture probed and read (2015-11-26 replay, saved) is the 2014-holiday-list circular of 2013-12-20, not a Muhurat notice.
  - **2017.** The second pass's issuance window (≈ 33770-34100, after 33746 of ~2017-06/07) holds only 302 wrappers and 404s; no PDF bytes.
  - **2019.** The new portal's circulars listing renders client-side; the `/api/circulars` JSON channel's first captures are 2026-02-10 and 2026-05-21, neither naming a 2019 artifact; the `muhurat-trading-session` page's earliest capture is 2020-11-20. The archive's September-2019 bulk capture ends at CMTR42070 (2019-09-23) and the next capture is 42877 (2019-12-11); the Muhurat issuance window (Diwali 2019-10-27) falls in between: no captures.
  - **2026.** The 2026 circular had not issued (or had not been captured) at the sweep: the archive's latest `nsearchives` circular captures run to 2026-10-02 and name no Muhurat circular, and the 2025 edition issued 2025-09-22 for a 2025-10-21 Diwali, so the 2026 edition's issuance window (Diwali 2026-11-08) was opening as the sweep ran. Publication-season watch, not a gap. Closing condition unchanged: the operator's circular.
- **The Muhurat re-check of 2026-10-09 UTC closed negative on every angle and re-dated the 2026 watch** (artifacts under `holidays/raw/equities/nse_india/muhurat-recheck-2026-10-09/` in the research store). It also corrected the 2026-10-06 record: that pass's two apex-host sweep files (`cdx_nseindia_cmtr.txt`, `cdx_nseindia_cmtr_lc.txt`) are an archive "Temporarily Offline" page and a 504 error page, not CDX rows, so the apex-host (`nseindia.com`, no `www`) prefix sweep had never completed. This pass completed it — the CDX API canonicalises the apex and `www` hosts onto one key set, so the completed sweep covers the URL space the successful `www`/`www1` sweeps already read — and the per-year windows closed negative again on it: 19xxx holds no 19187 (only 19013, 19075, 19431, 19539, 19860, 19982 and three 2003-era `.wri` files), the 2xxxx windows hold only the September-2013 zip run and the July-2014 stragglers the first pass knew, the 2017 window tops out at 33746, the 2019 window at 42070, and the `/content/cmtr/` path family has zero captures at all. The Wayback availability API answered `429` to every exact-URL probe (seven `19187` host/path variants); the completed prefix sweeps are the pass's evidence. The 2026 angle moved from watch to dated re-check: the operator's own live circulars channel (`/api/circulars?index=CM`, answered through a browser user-agent and cookie warm-up; the HTML pages still answer 403) lists 414 CM circulars from 2026-09-22 through 2026-10-09 and not one names Muhurat or Diwali — the 2025 edition issued 29 days before Diwali and this retrieval sits 30 days before Diwali 2026-11-08, so the issuance window was opening as the query ran. Closing conditions unchanged.
  - The two candidate PDFs a 2026-10-05/06 session had gathered beside the four circulars under the store's `nse-muhurat/` directory are **not** Muhurat circulars and key nothing: `CMTR45837` is a September-2020 DR-site mock-bidding session notice, and `CMTR64626` is a dynamic-price-band extension notice. `COM70362` is the 2025 **Commodity Derivatives** segment's Muhurat circular (same day and Normal Market window as `NSE-CIRC-2025-124`); it corroborates the CM row's Normal Market block and keys no separate row.
- **The 2018 holiday list is unrecovered; 2012 was recovered 2026-10-06 UTC.** No operator artifact stating the 2018 CM trading-holiday list survives in the Internet Archive: the annual page URL has no capture in that calendar year (searched 2026-09-29 UTC; the 2012-era `marketinfo/holiday_master` pages the archive does hold are a query form, not a list; the old page 302-redirects from 2017-07 and the new pages begin 2020-01; the `holiday-master` API begins 2021-03), and the annual circular PDF is uncaptured — the second pass's issuance-window sweep (issued 2017-12, numbers ≈ 33950-34200) also closed negative. 2018 therefore sits outside every audited window. Closing condition: an operator-hosted copy of the year's annual circular or holiday page, live or archived, worked up the same way. **Bridged residual (2026-10-06 UTC, #296):** the span sits **between two audited windows**, so under the charter's bridged-residual convention (AGENTS.md, "Modeling conventions") it no longer refuses whole dates: the session layer answers 2018-01-01..2018-12-31 (365 days) from the floor-sourced normal week, and the holiday layer stays honestly absent — no closure is asserted that no operator statement witnesses, the metadata reports the span as `HolidayWindowsBridged` rather than complete, and this bullet is the residual's disclosure. What is served: the sourced normal week's sessions on every span date — the profile timeline's dated revisions (2010-01-04, 2010-10-18, 2011-10-03, 2026-08-03) land on their own days, so one continuous grid governs the gap and the flanking windows' ordinary days alike. The disputed remainder: each span date's holiday arrangement (which of the operator's recurring closures fell there), which no flank states. The named closer is the closing condition above. 2012 sat in the same state until the same UTC day, when the annual circular `NSE-CIRC-2011-66` (dated 2011-12-09) surfaced in the archive and the first window merged across the year — fifteen weekday rows shipped, its printed Muhurat date 2012-11-13 one of the withheld.
- **The Muhurat instants for the eight published years are the circulars' own schedules.** The 2010, 2015, 2016, 2020, 2022, 2023, 2024 and 2025 sources that keyed the old withholding still stand for their closure dates, but the special-session instants now come from the operator's circulars (`NSE-CIRC-2010-121`, `NSE-CIRC-2015-65`, `NSE-CIRC-2016-56`, `NSE-CIRC-2020-98`, `NSE-CIRC-2022-124`, `NSE-CIRC-2023-139`, `NSE-CIRC-2024-147`, `NSE-CIRC-2025-124`), each a Wayback `id_` replay at T1 (LAW-PRIMARY-SOURCES). The remaining seven withheld dates wait on the same closing condition as above.
- **The 2021 list announces Muhurat against no date.** The 2021 page prints `Muhurat Trading will be conducted. Timings of Muhurat Trading shall be notified subsequently.` after a Trading Holidays tab in which no row carries the asterisk, so no trade date is named and nothing is withheld; the day session of 2021-11-04 (the Laxmi Pujan evening the special session ran on, per the clearing tab's `Diwali- Laxmi Pujan` entry) is audited normal per the operator's own list, and the special session itself stays outside what the holiday table states. Same closing condition as above.
- **Retrieval channels.** The live `nseindia.com` channel refused both its HTML pages (HTTP 403) and the `holiday-master` API (empty bodies) twice on 2026-09-28; the citations above are Wayback `id_` replays of the operator's own artifacts, and the `archives.nseindia.com` host no longer serves the legacy holiday pages (404). A future review with browser-grade retrieval should prefer the live pages and re-verify the digests there.
- **The 2025 banner's scope sentence differs from the 2026 one.** The 2026 banner is headed "Trading Holiday List for Equity & Equity Derivatives"; the 2025 banner is headed "NSE Holiday List 2025" without the segment line. Both are the exchange's single published trading-holiday list for the year and no date in either is stated to differ between the cash and derivatives segments; if NSE resumes publishing separate per-segment lists, the cash list replaces this table's basis.
- **Raised in review of the ledger-reshape PR (#87), 2026-09-12 — the NSE-wide 15:15–15:35 classification rests on a BSE statement.** The bullet above keeps the venue-wide 15:15–15:35 CAS window `extended` on the ground that non-CAS-eligible stocks keep trading continuously through it. The artifact that states that is BSE's, not NSE's: BSE's detailed operating guidelines of 2026-06-10 say securities not eligible for CAS "shall continue to be available for continuous trading till 3:30pm". SEBI circular 99122 introduces CAS for the cash segment but the reviewed set holds no NSE or SEBI text stating, for **NSE** securities outside CAS, that continuous trading runs through 15:15–15:35. The classification is therefore read across from a sibling venue's notice. The reshape PR moved this text out of the owner module and changed no schedule rule, revision row, profile or routing; the NSE window is served exactly as before. Closing condition: an NSE circular or SEBI text covering non-CAS-eligible **NSE** securities in that window, which would either confirm the `extended` classification or turn the window into an `order_entry` phase for this venue. Dormant identity, so recorded here rather than opened as an issue (LAW-FOLLOW-UPS-ARE-ISSUES).
- The 2026-08-03 CAS row is keyed to a SEBI circular, the regulator's own binding instrument, rather than to an NSE circular. The NSE Closing Auction Session page restates it. Recorded here because the tier of a regulator instrument is not the venue's own statement in the strict reading of LAW-PRIMARY-SOURCES.
- The pre-open order-entry boundary is set at 09:07, the earliest second a trade could print under the random closure between the 7th and 8th minute, never later; 09:07–09:15 stays `extended` so the auction match, its trade confirmations and the transition buffer remain tradeable.
- CAS 15:15–15:35 contains order-entry-only sub-phases for CAS-eligible stocks, but the venue-wide window stays `extended` because non-eligible stocks continue continuous trading through it.
- The 09:55 grid below 2010-01-04 is sourced by the December-2009 artifacts; nothing below the January-2010 floor is reviewed.

## Module narrative (moved from src/calendar/schedules/equities/apac/nse.rs on 2026-10-02 UTC)

NSE's own pre-open page
states the session "is comprised of Order collection period and order
matching period", that "the order collection period of 8* minutes shall be
provided for order entry, modification and cancellation (* - System driven
random closure between 7th and 8th minute)", and that "order matching period
starts immediately after completion of order collection period". BSE's
operating notice for the same 2010-10-18 launch prints the grid outright:
Order Entry Period 9:00am–9:07/08am with "No trades are executed", Order
Matching & Confirmation Period 9:08am–9:12am, Buffer Period 9:12am–9:15am.

Only the collection phase is `order_entry`. Because the collection phase is
cut by a random stoppage anywhere in the 7th–8th minute, the boundary is set
at the earliest second a trade could print (09:07), never later; 09:07–09:15
stays `extended` so the auction match, its trade confirmations, and the
transition buffer remain tradeable. The same grid is still current after the
2026-08-03 CAS cutover, so every post-2010-10-18 profile shares it.
https://www.nseindia.com/static/products-services/equity-market-pre-open
https://www.bseindia.com/markets/MarketInfo/DispNewNoticesCirculars?page=20101014-8

---

Effective 2026-08-03, derivative-eligible
cash stocks enter CAS at 15:15 while non-CAS stocks continue normally to
15:30; the overlapping regular/extended rules preserve both venue-wide
states. CAS ends 15:35, transition runs to 15:50, and post-close ends 16:00.
Sources:
https://www.nseindia.com/static/products-services/closing-auction-session
https://www.sebi.gov.in/legal/circulars/jan-2026/introduction-of-closing-auction-session-cas-in-the-equity-cash-segment-and-certain-modifications-in-the-pre-open-auction-session_99122.html
https://www.bseindia.com/markets/MarketInfo/DispNewNoticesCirculars?page=20260801-1

Both close-side windows stay `extended`. CAS 15:15–15:35 contains
order-entry-only sub-phases for CAS-eligible stocks, but BSE's detailed
operating guidelines also state that securities not eligible for CAS "shall
continue to be available for continuous trading till 3:30pm", so trades print
venue-wide throughout; the CAS itself then matches 15:30–15:35. The
15:50–16:00 post-close is a fixed-price session in which trades execute.
https://www.bseindia.com/downloads/UploadDocs/Notices/20260610-41/20260610-41.pdf

---

BSE's notice is 20091217-15; NSE's official release follows.
https://www.bseindia.com/markets/MarketInfo/DispNewNoticesCirculars?page=20091217-15
https://nsearchives.nseindia.com/content/press/17122009.htm

No pre-open existed before 2010-10-18, so these two profiles have nothing to
classify as order entry: their only extended window is the 15:50–16:00
post-close, a fixed-price session in which trades execute.
