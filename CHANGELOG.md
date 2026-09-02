<!-- SPDX-License-Identifier: MIT-0 -->

# Changelog

All notable changes to `exchange-hours` are documented in this file, starting
from the first tagged release. The format is based on
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and the crate adheres
to [Semantic Versioning](https://semver.org/spec/v2.0.0.html). Session-data
corrections (a venue's hours fixed against a primary source) go under
**Fixed**; new venues and new API surface under **Unreleased**/**Added**.

## [Unreleased]

### Changed

- **US options queues are carried across history instead of withheld
  (behaviour change).** The seventeen `Partial` US options rows served no
  order-acceptance queue before the 2026-08-22 review row, which under-reported
  order acceptance for up to sixteen years per venue. They now carry it — from
  the January-2010 floor where the venue predates the window, or from its sourced
  launch day. A 07:45 ET instant in 2011 answers `OrderEntry` on C1 and C2 where
  it previously answered `Closed`.

  **The assumption is stated, not hidden.** No primary source says when any of
  these queues began: they are operator system settings on mutable pages, not
  rulebook boundaries — SR-C2-2019-009 and SR-CboeBZX-2020-012 each write down
  07:30 as "the same time at which the System begins accepting orders and quotes
  today" while declining to change it. Carrying the queue back asserts continuity
  no document states. It is recorded in `options/history.rs`, in a `///` note on
  every affected profile, and in all seventeen ledger rows.

  **MIAX Options is excluded and modelled from evidence instead.** Its 07:30
  window existed at the sourced 2012-12-07 launch but was connectivity
  verification only — the official hours page captured 2012-12-09 says activity
  before the Live Quote Window "WILL NOT affect the live quote state", and the
  2013-05-07 capture says it WILL affect the live book. MIAX therefore keeps a
  queue-free launch row and gains the queue at that second capture. MEMX Options
  is outside the change entirely: it has no queue, rejecting orders before 09:30.

  Nothing matches in a queue, so this changes `order_entry` coverage only — every
  venue's 09:30–16:00 execution history is sourced independently and untouched.
  `HISTORICAL_CUTOVERS` drops the seventeen 2026-08-22 option rows, which no
  longer exist, and gains MIAX Options' sourced 2013-05-07 cutover.

- **Recorded decision: the timeline is not bounded at the January-2010 floor.**
  `select_revision` returns a venue's baseline for any date before its first
  revision, so a pre-floor instant resolves to the January-2010 grid rather than
  to an absence — `globex_equity_index` answers open at 18:30 CT in 2005, and
  always has. That is now a deliberate, recorded convention rather than an
  undocumented side effect: the profile returned is the earliest state the crate
  has sourced, and the caller asked for a date the crate never undertook to
  model. Adding a lower bound would touch every venue with a non-empty baseline
  and trade one unreviewed answer for another, since neither `Closed` nor the
  2010 grid is sourced below the floor. Recorded in `AGENTS.md`, and the README
  now tells callers that a pre-2010 answer is the oldest profile on record rather
  than a reviewed one. No profile, selector, or schedule data changed.

- **Every `Partial` row now states its gap kind.** `AGENTS.md` requires it — a
  missing trading session and an undated queue start carry very different weight
  and the basis label cannot distinguish them — but only the US futures families
  had been classified. All 46 rows are now tagged: **34 order-entry**, where the
  trading session is sourced and what is undated is a queue or post-close phase
  in which no trade can print, and **12 executable**, where the uncertainty
  touches a window where trades print. The executable twelve are the six ICE
  Futures U.S. keys, `globex_nikkei_225_dollar` and the five SGX equity-index
  keys; none serves hours it cannot support, and each errs toward `Closed`. The
  ledger's classification-status note is replaced with the completed result and
  the README carries the split. No profile, selector, or schedule data changed.

### Fixed

- **CME Nikkei 225 Dollar: the 2011 grid is no longer carried across 2010, where
  it was wrong (behaviour change).** An earlier commit on this branch carried the
  17:00–15:15 CT grid back to the January-2010 floor, on the reasoning that no
  primary source named a cutover inside the interval. A source does. CME's own
  trading-hours pages captured **2010-03-10** and **2010-04-07** publish a
  materially different grid for `Nikkei 225 (Dollar) Futures`: CDT 03:00–15:15
  reopening 15:30–16:30 and 17:00–18:00, CST 02:00–15:15, and **no Sunday hours
  at all in CST**. Serving the continuous grid across that period reported the
  contract open through the whole overnight window when it was closed — a false
  open, in executable hours.

  The changeover day is undated: 2010-04-07 still shows the old grid, 2011-01-12
  already shows the new one, and no capture or CME notice between them was
  located. So the served grid now applies from **2011-01-12**, its first sourced
  appearance, and earlier dates are sessionless. The 2010 grid is recorded beside
  the table as sourced-but-unmodelled — encoding it would need seasonal CDT/CST
  rules and a boundary that is still undated.

  `nkd_close_tracks_its_three_sourced_revisions` now pins 2010 as sessionless at
  three points and the grid as served from its first sourced capture.

### Fixed

- **ICE Futures U.S.: the January 2010 – August 2011 gap is document-bound, not
  an unfinished search.** The six `ice_us_*` keys carry the August 2011 master
  hours table back to the January-2010 floor, which is executable-hours coverage
  resting on a carry-back rather than direct sourcing — one of only two such
  places in the crate. The 2026-09-01 review established why it cannot be closed
  by a filing: **ICE Futures U.S. sets these hours administratively, not by
  rule.** Its product rulebook chapters — Sugar No. 11, Cotton No. 2, Coffee,
  Cocoa, FCOJ and USDX, all captured December 2011 — contain no hours provision
  at all, and chapter 4 is trade-practice rules. So no SEC or CFTC filing fixes
  an ICE Futures U.S. trading hour, the master hours table is the only source,
  and its earliest surviving edition is August 2011. The rows keep `Partial` and
  the carry-back stands as the terminal answer unless an earlier edition
  surfaces. **No profile, selector, or schedule data changed** — the hours the
  crate serves are identical before and after; only the characterisation of the
  gap changed, recorded in all six ledger rows, beside each table, and in the
  source registry.

### Changed

- **Sourcing policy recorded: public sources only.** The project uses only
  publicly available operator and regulator material and encodes the schedule
  facts those documents state — opening and closing times, phase boundaries,
  effective days — citing each back to its source. Public availability of the
  source is what gates the work. The policy sets the standard for new and
  revised data; it makes no claim that every literal already in the crate is
  cited, and the documented `Missing/uncited` gaps in the ledger stand. Recorded as **LAW-PUBLIC-SOURCES** in
  `AGENTS.md`, as the sourcing policy in `docs/schedules/date-exceptions.md`,
  and in the Phase 5 gate of the coverage plan, which is now a public-source
  check. Material behind a member portal or
  an authenticated feed is out of scope as a data source, though its existence
  and publication date may still be cited as evidence that a change occurred, as
  the SGX Titan newsletters are. A venue whose holiday calendar exists only
  behind authentication stays `OutOfCoverage` rather than being filled from a
  non-public source.

### Fixed

- **Cboe EDGA/EDGX early-queue gap reclassified: unfinished search, not a
  knowledge-bound row.** SR-EDGX-2015-03 (80 FR 2163, filed 2015-01-08) and its
  EDGA twin SR-EDGA-2015-03 (80 FR 2125) quote Rule 11.1(a)(1) as *already*
  providing order entry "from 6:00 a.m. until 8:00 p.m. Eastern Time" while
  carrying it onto the BATS platform. That puts the 06:00 start no later than
  2015-01-08 and — the material point — locates it in the **rulebook** rather
  than in an operator system setting, so some dated SEC filing established it.
  These two rows are therefore expected to close, and must not be written up
  with the permanent-gap language the US options rows carry, where the start is
  a mutable system setting no filing ever fixed. The 2010 registration order
  (75 FR 13151) does not state the hours; the filing that first set Rule
  11.1(a)(1) to 06:00 remains the target.

- **SGX equity-index history rebuilt from six calendar editions: three sourced
  eras, and a correction to the fix itself (behaviour change).** The five
  `sgx_equity_index_*` keys carried today's grid back to the January-2010 floor
  across transitions the module itself recorded as real, which made them the only
  rows in the crate that could **over**-report. SGX's Derivatives Trading
  Calendar — static, readable PDFs under `api2.sgx.com/sites/default/files/` —
  supplies the dated grids, and reading six editions rather than two shows there
  were **two** changes:

  | edition | Japan T / T+1 | China T+1 | SiMSCI T+1 | Taiwan T+1 | NTR T+1 |
  |---|---|---|---|---|---|
  | 2020, 2021-07, 2024 | 07:30–14:25 / 14:55 | 17:00 | 17:50 | 14:15 | 19:00 |
  | 2025-01 | 07:30–**14:55** / **15:25** | 17:00 | 17:50 | 14:15 | 19:00 |
  | 2025-11, 2026-01 | 07:30–14:55 / **15:10** | **16:45** | **17:35** | **14:00** | **18:45** |

  Japan's T session lengthened at the 2024/2025 boundary while its T+1 moved to
  15:25; only later in 2025 did Japan's T+1 settle at 15:10 and the other four
  families pull their T+1 opens fifteen minutes earlier. **An intersection
  computed from the 2021 and 2026 editions alone — which this crate briefly
  shipped — puts Japan's T+1 at 15:10 and so reports the market open between
  15:10 and 15:25 through 2025, when it was not.** The dated surface therefore serves **one window** from the first sourced
  edition — the intersection of all six, the bounds `regular` in every one of
  them (Japan T 07:30–14:25 and T+1 15:25–05:15; China T+1 17:00; SiMSCI
  17:50; Taiwan 14:15; NTR 19:00) — and the verified-current grid from the
  2026 edition. No revision is keyed to either undated transition.
  Neither transition day is stated anywhere reachable, and an annual edition's
  year is a publication scope rather than an effective date, so no revision is
  keyed to one: the intersection carries the uncertainty instead. The two
  boundaries that remain are not inferred cutovers — 2020-01-01 separates
  sourced from unsourced, and 2026-01-01 is the scope of the edition titled
  "SGX Calendar 2026". Routines are dropped from the historical eras deliberately — the calendar
  states session bounds only, and each Pre-Opening/Non-Cancel and closing routine
  moved with the session it brackets. Dates before the 2020 edition remain
  sessionless. `sgx_equity_index_history_has_three_sourced_eras` pins all three
  eras, including the 15:10–15:25 probe that the earlier intersection failed.

- **Historical queue and session gaps are now served instead of withheld
  (behaviour change).** Two conventions were made explicit in `AGENTS.md` and
  applied: carry the earliest sourced state back to the January-2010 audit
  floor when no primary source names a cutover inside the interval, and prefer
  the sourced *intersection* to omission when only a changeover day is undated.
  - **CME Sunday Pre-Open.** The four Globex families (`globex_equity_index`,
    `globex_energy`, `globex_fx`, `globex_interest_rates`) and the four venue
    rows that reuse them previously answered `Closed` for the whole Sunday
    16:00–17:00 hour on every dated instant, because the 16:15→16:00 move is
    undated. CME's queue only ever widened — 16:15 at the audit floor, 16:00
    verified current — so Sunday **16:15–17:00 is order-entry under every
    sourced state** and is now carried from the January-2010 floor with no
    cutover asserted — for `globex_equity_index`, `globex_energy`, `globex_fx` and
    `globex_interest_rates`, and the `cme`/`cbot`/`comex`/`nymex` rows that
    reuse them. Only the disputed 16:00–16:15 quarter-hour still waits on the
    undated 2012 move, and that quarter-hour alone — not the whole hour — is
    what the 2026-08-22 review row adds. A Sunday 16:30 CT instant in 2011, 2015 or 2025 now
    answers `OrderEntry` rather than `Closed`. `globex_energy` also regains its
    Monday–Thursday 16:45–17:00 queue, which the dated profiles dropped
    entirely.
  - **CME Nikkei 225 Dollar.** `globex_nikkei_225_dollar` previously returned
    **no session at all** before 2012-11-18, modelling a contract that was
    demonstrably trading as closed for nearly three years. CME's own
    trading-hours pages captured 2012-05-11 and 2012-05-28 publish the grid
    directly — Electronic Trading (Sunday) "17:00-15:15" and (Weekday)
    "15:30-16:30, 17:00-15:15", byte-identical to the E-mini S&P 500 row beside
    it — so that grid is carried back to the January-2010 floor. The residual
    risk is stated beside the table: a 2010 grid change is attested by a
    third-party aggregator and no primary source, so the profile may be wrong
    for part of 2010, which is a smaller error than reporting the whole
    interval closed.
  - **CME livestock morning queue.** `globex_livestock` dropped its 06:00–08:30
    CT morning Pre-Open for the whole 2016-02-29..2020-05-31 span because the
    queue's original onset is undated. SER-8599R states the outgoing 06:00
    value when it dates the move to 08:00, and no source names a cutover
    between SER-7591's 2016-02-29 grid and that move, so the queue is now
    carried across that interval. It is deliberately not carried further back:
    the older around-the-clock grid has no 08:30 open for a morning queue to
    precede.
  - New fences pin these behaviours on both sides:
    `sunday_pre_open_carries_back_at_its_narrowest_sourced_edge` asserts the
    queue is served at 16:30 CT and withheld at 16:00 CT across four families
    and four decades of Sundays, the NKD boundary test now probes the
    pre-2012 16:30 CT close against the 16:15 CT close SER-6465 introduced, and
    `livestock_morning_queue_spans_its_sourced_matching_grid` pins the 06:00 CT
    edge on both sides of the 2020 move and its non-extension past 2016.


- **ICE Futures U.S. softs and USDX: the pre-2014 grids are now stated, not
  inferred, and Cotton's Sunday omission rests on contrast rather than
  silence.** Two dated editions of ICE's own *ICE Futures U.S. Regular Trading
  Hours* master table — **August 2011** and **January 2, 2013** — were recovered
  and read. They print the pre-2014 grids outright: Coffee "C" 3:30–14:00,
  Cocoa 4:00–14:00 and FCOJ-A 8:00–14:00 in both editions, which **supersedes
  the previous record** that the pre-2014 coffee and cocoa grids were
  "corroborated by the surrounding primary record rather than stated". For
  Cotton the two editions repeat the same footnote contrast — Cotton is marked
  `*` ("Trading commences on previous business day") while the Grains, Russell,
  USDX and currency rows carry `**` ("…and on Sunday evenings only trading
  commences at 18:00") and energy `***` — so ICE drew the Sunday distinction
  explicitly at two independent dated points and did not extend it to Cotton;
  the modelled omission of Cotton's Sunday-evening open is now positively
  evidenced. Two gaps were re-worked and confirmed negative: every archived
  edition of the master table was enumerated and August 2011 is the earliest in
  existence, so Sugar's January-2010 close is limited by document availability;
  and ICE's 2007 currencies release states only the 20:00–18:00 ET
  currency-futures grid without ever printing a USDX grid, so the pre-2011 USDX
  grid stays undated. All six keys retain `Partial`, now with a single shared,
  precisely named residual gap: January 2010 to August 2011. No profile or
  revision row changed.

- **CME grains, NKD, cryptocurrency and livestock: the remaining Batch B gaps
  are now sourced states or confirmed source-level silences.** Grains — CME's
  own trading-hours pages inside the 21-hour regime publish Sunday Pre-Open
  16:00, weekday "14:30-16:00, 16:45-17:00" and ETH 17:00-14:00 (2012-05-28 and
  2012-06-07), against Sunday 16:15 and the 18:00-07:15/09:30-13:15 grid on
  2012-05-11, bracketing the switch to 2012-05-11..2012-05-28 around the sourced
  2012-05-20 expansion; Advisory #20120518 states only matching hours, so no
  queue revision is keyed to it. NKD — the same pages state the pre-2012 grid
  (Sunday Pre-Open 16:15, ETH 17:00-15:15, weekday "15:25, 16:45" and
  "15:30-16:30, 17:00-15:15"), byte-identical to the E-mini S&P 500 row beside
  it, which **supersedes the previous record that no primary source states the
  pre-2012 evening open**; only the grid's onset stays undated, so the
  sessionless pre-2012 treatment is unchanged. Cryptocurrency — the bitcoin
  contract specification captured 2017-12-14, carrying its own launch statement
  for trade date 2017-12-18, publishes the Globex matching grid and no Pre-Open,
  confirming the five-day era's queue onset is undated at the source. Livestock
  — the specification channel was checked as a second route into the
  2016-11..2020-03 interval and is silent too, rendering only ClearPort/Default
  08:30-13:05 CT hours with no Globex Pre-Open or PCP row. No profile or
  revision row changed.

- **CME Globex Sunday pre-open: bracket narrowed to ten days, and both notice
  channels shown silent.** Three archived captures of CME's own trading-hours
  pages, unused by the earlier review, cut the 16:15→16:00 CT Sunday Pre-Open
  bracket from 2012-05-03..2012-06-15 down to **2012-05-28..2012-06-07**. The
  move was platform-wide and simultaneous — E-mini S&P 500, Eurodollar, 30-Year
  Interest Rate Swap, Euroyen TIBOR, Gold, Silver, Light Sweet Crude and Henry
  Hub all read 16:15 on the 2012-05-28 capture and 16:00 on the 2012-06-07
  capture — and Sunday-only, since weekday Pre-Opens are identical across both.
  CBOT grains are separated out of it: the 2012-05-11 capture still shows the
  pre-expansion grain grid at 16:15 and the 2012-05-28 capture the expanded
  17:00–14:00 grid at 16:00, confirming grains moved at the already-dated
  2012-05-20 expansion rather than with the platform. Both CME notice channels
  were then read in full across the narrowed window — Globex Notices of
  2012-05-21, 2012-05-28 and 2012-06-04, and Market Data Notices of 2012-05-28
  — and none contains any occurrence of "Pre-Open", "trading hours", "16:00" or
  "16:15", so the change was made without a dated operator notice. The
  `globex_equity_index`, `globex_energy`, `globex_fx` and
  `globex_interest_rates` keys and the `cme`/`cbot`/`comex`/`nymex` venue rows
  keep their `Partial` basis and their knowledge-bound Sunday-queue treatment;
  no cutover is encoded, because the only Sunday inside the bracket
  (2012-06-03) is an inference from the bracket rather than a source-stated
  effective day.

- **US options queue onsets: the gap is knowledge-bound, and MIAX's is now
  bracketed.** A review of all seventeen `Partial` US listed-equity-options
  rows established why no onset day exists: on every venue the generic
  order-acceptance start is an operator *system setting* published on a mutable
  hours or system-settings page, not a rulebook boundary carrying a filed
  operative date. The two filings that codified the Cboe queuing periods say so
  outright — SR-C2-2019-009 (84 FR 20673) and SR-CboeBZX-2020-012 (85 FR
  6246) each write down 07:30 as "the same time at which the System begins
  accepting orders and quotes today", and record that Cboe Options Rule 6.2(a)
  bounds the pre-opening period rather than fixing it. Nasdaq states each start
  in a per-venue System Settings document, NYSE on its hours page, and MIAX on
  its trade-hours calendar; none of those channels publishes a dated change
  notice. The rows keep their `Partial` basis, but each now names a
  knowledge-bound gap rather than an unfinished search, and three carry sourced
  lower bounds (C2 no later than 2019-05-10, BZX Options 2020-02-04, ISE's
  06:00 start 2019-10-17). MIAX Options gains a real bracket: its official
  hours page captured 2012-12-09 — two days after the sourced launch — states
  that pre-Live-Quote-Window activity "WILL NOT affect the live quote state",
  while the 2013-05-07 capture states it WILL affect the live book, so the
  launch-era queue-free row is now positively sourced and the order-acceptance
  onset falls in 2012-12-09..2013-05-07. No profile, revision row, or selector
  changed: no primary source states a day, so none was invented.

- **CME grains: dated queues and PCP now begin at their sourced 2013 onset.**
  CME's 22 March 2013 Global Command Center notice, the state-level companion
  to SER-6617, states the unconditional onset of every current CBOT
  grain/oilseed queue: Sunday 16:00–19:00, Monday–Thursday 16:45–19:00,
  Monday–Friday morning 08:15–08:30 (widened to 08:00 on 2013-08-18), and the
  14:30–16:00 PCP, effective Sunday 2013-04-07. Dated `globex_grains`/`cbot`
  selectors now carry those phases from that day (previously omitted to the
  2026-08-22 knowledge-bound row); the dated and fixed-current surfaces now
  agree, and the remaining gap shrinks to the 21-hour 2012-05-20..2013-04-06
  regime's undocumented queue states.
- **CME livestock: the 14:30–16:00 PCP onset is sourced to 2016-06-06.** The
  30 May 2016 CME Globex notice implements the Post-Close state for Live
  Cattle, Feeder Cattle, and Lean Hog futures effective Monday 2016-06-06;
  dated `globex_livestock` selectors now carry the PCP from that day.
  Official trading-hours captures omit the row between November 2016 and
  March 2020 with no removal notice, so that interval and the pre-2020 06:00
  queue's onset remain documented gaps.
- **CME Globex Sunday pre-open 16:15→16:00: bracket documented, day still
  unsourced.** The move was real and platform-wide: primary documents updated
  2012-05-03 (holiday workbook and equities/FX/metals hours pages) still
  publish Sunday 16:15 CT, and pages crawled 2012-06-15/16 already publish
  16:00, but no notice states the day. The equity-index, FX, energy/metals,
  and interest-rate families keep their knowledge-bound Sunday-queue
  treatment, now with the bracket recorded beside each table and in the
  verification ledger.

### Changed

- **README assurance statements rewritten in plain language.** The three
  headline counts kept their values and their machine-checked format, but
  "Non-synthetic profiles requiring reconciliation" was actively misleading:
  the ledger reserves *reconcile* for **Known issue** rows, of which there are
  none, so the label implied 27 broken venues when it counted 27 rows whose
  present-day hours are verified and whose history has one named missing day.
  The three lines are now "Today's hours check out against the exchange",
  "Full dated history back to January 2010" and "History complete except for
  one named gap", each followed by a plain-terms explanation of what it does
  and does not claim. The dated audit report carries the same relabelling; no
  count, method or exclusion in it changed.

- **Single-path, instant-driven profile selection (breaking, pre-1.0).**
  Every schedule-selection entry point now requires the caller's instant:
  `hours_for_exchange(exchange, as_of)` and
  `hours_for_market_hours_key(key, as_of)` (formerly the `_as_of` variants —
  the clock-less twins are removed, and the plain names moved to the
  instant-driven signatures), and every `bulk` builder takes a trailing
  `as_of`. The crate still never reads a clock (LAW-DETERMINISM): a live
  caller passes their own `Utc::now()` at the application edge, and a
  backtest passes its historical instant — identical code either way. A
  fully sourced, unconditional future revision can now be encoded ahead of
  its effective day and rolls over with no release in between.
  `presets/current.rs` is gone; the one exhaustive routing match lives in
  `presets/historical.rs` with no catch-all arm, and every venue module owns
  a `profile_at` selector. The `ExchangeCalendar` opening-day anchor moved
  from venue-local noon to the end of the local opening day, so a sourced
  intraday cutover in an afternoon gap (as ICE Canada's 2011 18:30 CT
  pre-open) governs that day's later sessions. `session_profile` remains the
  static current-table accessor and equals the timelines' selection at any
  instant on or after the 2026-08-22 knowledge-bound rows.
- **Verified-current phases are carried by knowledge-bound timeline rows.**
  Twenty-four venues and the Globex families whose current grids include
  an order-acceptance or early phase with no sourced onset day (for example
  the US options pre-open queues and CME's Sunday 16:00 CT Pre-Open) now
  record that phase as the final timeline row dated at the 2026-08-22
  repository review, labeled "verified current, onset undated". Instants
  before the row keep the conservative dated grid; instants on or after it
  resolve to the verified-current grid, so the previous clock-less snapshot
  answers are preserved exactly. Finding a sourced onset day replaces each
  row — the grains and livestock rows above are the first two replaced.

### Added

- **Thirteen product-family keys, closing the deferred-family gap.** Every
  futures family previously listed as unsupported now ships with a
  primary-sourced profile: ICE Futures U.S. Sugar No. 11 (`ice_us_sugar`),
  Coffee "C" (`ice_us_coffee`), Cocoa (`ice_us_cocoa`), Cotton No. 2
  (`ice_us_cotton`), FCOJ-A (`ice_us_orange_juice`) and U.S. Dollar Index
  (`ice_us_dollar_index`); CME Nikkei 225 Dollar
  (`globex_nikkei_225_dollar`); and Eurex fixed income
  (`eurex_fixed_income`). `MarketHoursKey` goes from 12 variants to 25.
- **SGX equity index ships as five keys, not one.** Sourcing the schedules
  showed SGX equity-index products do not share a grid: Japan 07:30–14:55,
  China 09:00–16:30, Singapore 08:30–17:20, Taiwan 08:45–13:45, and the
  NTR (USD) global grid 07:25–18:30, all `Asia/Singapore`. They ship as
  `sgx_equity_index_japan`, `_china`, `_singapore`, `_taiwan` and
  `_ntr_usd`. The ambiguous name `sgx_equity_index` stays rejected: a single
  key would answer a Taiwan contract with Singapore's close, which is the
  substitution the key API exists to prevent.

### Fixed

- **NKD's 2012–2013 post-halt segment now covers its closing days.** The
  15:30–16:15 CT continuation during the sourced halt regime carried the
  opening-day mask (Sunday–Thursday), which fabricated a Sunday-afternoon
  session and dropped Friday's post-halt trading entirely. The rule now runs
  Monday–Friday — the closing days of the wrapped sessions — with new
  Friday and Sunday probes in the key boundary test.
- **Tadawul's 2016–2018 era no longer carries the current pre-open queue.**
  The 09:30–10:00 order-entry window is evidenced only by the operator's
  current trading-cycle page; applying it to the 2016-04-03 through
  2018-05-27 regime asserted an undated phase against a dated one. The era
  now carries no order-entry schedule until a dated primary source
  restores it.
- **ICE Futures Canada's 2011 hours change now flips at its sourced intraday
  instant, not local midnight.** The February 2011 notice moves the Canola
  pre-open/open to 18:30/19:00 CT on Monday 2011-02-28, and local midnight of
  that day falls inside the still-running Sunday session. The previous
  day-level row split that running session: a 2011-02-28 morning query
  reported the Sunday session as opening at the not-yet-in-force 19:00 CT.
  The change is now an exact-instant cutover at 2011-03-01 00:30:00 UTC, and
  the date-aware engine's opening-day anchor moved from local noon to the end
  of the local opening day so an afternoon intraday cutover governs that
  day's later sessions. A new contract fence (`day_level_cutovers_never_
  split_a_running_session`) now enforces the no-split invariant for every
  recorded cutover.
- **BZX and BYX no longer report trading before their 2016 matching start.**
  Bats' 2016 release note moved equity order matching and routing from 08:00
  to 07:00 ET on staggered days (BYX 2016-05-23, BZX 2016-05-25). The dated
  timelines previously kept 07:00–08:00 tradeable across that boundary, so a
  2015 instant answered "open" during a window in which orders were accepted
  but nothing could match. Both exchanges now carry their sourced 2016
  revision: the 06:00–08:00 queue narrows to 06:00–07:00 and the 07:00 hour
  becomes tradeable exactly on the operator's day.
- **Unsourced historical order-entry phases are no longer asserted.** B3's
  2010–2012 11:00-open grids and Tadawul's pre-2016 11:00-open grids carried
  inferred pre-opening order windows (10:45–11:00 and 10:00–11:00
  respectively) with no dated primary source behind them; both now read
  closed until evidence surfaces. TSE's historical 08:00 order acceptance,
  by contrast, gained dated primary evidence and stays: JPX's November 2020
  Investigation Report into the October 1, 2020 system failure states "Order
  acceptance began as normal at 08:00", covering the post-2011 era alongside
  Working Paper No.3's 2010-01-04 data for the earlier one.
- **ICE Cotton's Sunday evening open is no longer asserted.** The Sunday
  21:00 NY opening (and its 19:30 pre-open) was a sourced inference: three
  primary-source strands pointed at it, but no ICE document names Sunday for
  Cotton No. 2. Under the crate's primary-source law an unasserted phase is
  omitted, so the tradeable week now runs Monday 21:00 through Friday 14:20
  and Sunday evenings read closed until ICE states otherwise. The sourced
  Friday 14:50–18:00 post-close pre-open remains the week's final
  order-entry window, feeding Monday's session.

### Notes

- Every revision timeline row now carries its primary-source citation in the
  type (`SourceRef`), and a `revisions!` macro fails the build unless a
  timeline's effective dates are strictly ascending and every row names its
  source. A shadowed duplicate date or an uncited revision can no longer
  compile.
- Fifteen dated cutovers were encoded from operator notices, each carrying its
  verbatim quote and source URL — ICE Sugar 2012-01-30, 2012-11-05,
  2014-02-03 and 2018-10-08; Coffee, Cocoa and Cotton 2014-02-03 and
  2018-10-08; ICE USDX 2011-02-14; Eurex fixed income 2018-12-10 and
  2019-02-25.
- Two candidate cutovers were **rejected** rather than encoded. A CME NKD 2010
  date was retrievable only from a third-party news aggregator, with the CME
  URL returning 403 and no archive capture. An ICE Sugar 2012-03-12 notice is
  titled "Temporary Change to Opening Time" and is a daylight-saving window
  change, not a normal-week revision.
- `globex_nikkei_225_dollar` carries a dated timeline from 2012-11-18. A
  follow-up sourcing pass found the previously undated close change: CME
  Globex Notice #20150817 of 17 August 2015 moves the CME Equity close to
  16:00 CT effective Monday 2015-09-21, corroborated by CME's own NKD
  contract-specification captures either side of the cutover. NKD therefore
  routes SER-6465 from 2012-11-18, the SER-6554R halt removal from
  2013-03-03, and the 16:00 CT close from 2015-09-20. The same notice
  supplies the original-announcement citation for `globex_equity_index`'s
  existing 2015-09-20 revision. The row stays Partial because the pre-2012
  interval is omitted: no primary source states the pre-2012 evening open, so
  pre-2012 dated queries return no session rather than an inferred grid.
- The Sunday Globex pre-open queue's 16:15→16:00 move remains undated after a
  read of 481 CME Globex Notices spanning 2008 to early 2016. It is bracketed
  by primary evidence but never announced, so it stays omitted from
  `globex_equity_index`, `globex_fx` and `globex_interest_rates` rather than
  being inferred. The same pass established that `globex_fx` and
  `globex_interest_rates` never had a 16:15 CT close to move — both already
  closed at 16:00 CT before 2015 — so there was no gap there to close.
- The five SGX rows are Partial for the same reason: SGX's circular archive
  exposes no day-level hours changes, so every transition is bracketed between
  calendar PDFs but undated. No date was inferred to fill a gap.
- Nifty is deliberately absent. It is an NSE IFSC product now, and SGX's own
  2026 calendar and GIFT Connect product page state different T+1 start times.

## [1.0.0] - 2026-08-22

First stable release. Version 1.0 establishes the canonical string identities,
normal-week schedule scope, date-aware calendar surface, and primary-source
maintenance contract described in the README and schedule verification ledger.

### Added

- **23 major cash-equity venues:** ASX, TMX Australia, NZX, TSE, NSE India,
  BSE India, HKEX, SGX Securities, Bursa Malaysia, SET Thailand, IDX, PSE,
  HOSE, SSE, SZSE, KRX, TWSE, Borsa Istanbul, TSX, JSE, Tadawul, B3, and BMV.
  Together with the US additions below and pre-v1 IQX cleanup, this brings the crate to 93
  source-backed market identities plus the synthetic `Exchange::Unknown`
  fallback (94 `Exchange` variants total).
- **Three live US national exchanges:** LTSE (`ltse`), 24X National Exchange
  (`24x`), and Texas Stock Exchange (`txse`), each with its primary-sourced
  production launch boundary, current accepted-order envelope, regional bulk
  membership, and stable canonical identity. Conditional 24X overnight hours
  remain on the watch list rather than activating early.
- **The additive `ExchangeCalendar` API** for date-aware predicates, session
  scans, maintenance/all-day queries, and candle boundaries. Existing
  `MarketHours` functions and signatures remain unchanged; B3/BMV calendars
  reselect their published grid on every candidate trading day.
- **Date-aware product-family calendars.** `calendar_for_market_hours_key`
  gives every `MarketHoursKey` the complete calendar query surface and
  reselects sourced revisions for each candidate opening day. `CalendarSource`,
  `ExchangeCalendar::source`, `exchange`, and `market_hours_key` preserve
  whether a calendar represents a venue or a product family.
- **Three primary-sourced CME Group family keys:**
  `GlobexInterestRates` (`globex_interest_rates`) for CBOT Treasuries/Fed Funds
  and CME SOFR, `GlobexLivestock` (`globex_livestock`) for LE/GF/HE, and
  `GlobexCryptocurrency` (`globex_cryptocurrency`) for CME non-spot-quoted
  cryptocurrency futures. Each owns its dated family history rather than
  borrowing another product family's clock. There are now 12 keys total: 11
  operator-derived families plus synthetic `AlwaysOpen`.
- **Caller-supplied trade-date overrides.** `DayPolicy` and
  `PolicyCalendar` apply closed dates, early final closes, and late first opens
  to all status, session, candle, and closed-day scans without bundling holiday
  data or modifying `hours_at`. `StaticDayPolicy` and `DayOverride` add a
  validated, allocation-free hard-coded record format; `NoPolicy` preserves
  normal-week behavior. Complex holiday phase replacements remain explicitly
  outside this scalar boundary API rather than being approximated.
- **Trade-date and one-shot state queries.** `trade_date` and
  `is_closed_trade_date` are available on date-aware and policy-aware
  calendars. Profiles without a trade-date concept return no trade date and
  report every trade date closed because no session can be assigned to one;
  their civil-day availability remains available separately. `SessionState`
  distinguishes regular, extended, halt, maintenance, and closed states;
  `is_maintenance` is exactly its maintenance case.
- **A Criterion query benchmark** covering `is_open`, `session_bounds`, daily
  `candle_end`, `trade_date`, and closed-gap `session_state` for
  `GlobexEquityIndex`, plus documented allocation and value/thread-trait
  performance contracts.
- **`hours_for_market_hours_key_as_of`** for primary-sourced point-in-time
  futures product-family snapshots, reusing the same dated tables as the
  corresponding exchange profiles.
- **A complete `MarketHoursKey` identity surface:** `ALL`, `as_str`, `Display`,
  `FromStr`, `Ord`, and `PartialOrd` share the canonical string table used by Serde;
  `ParseMarketHoursKeyError` preserves rejected input for callers.
- **Point-in-time APAC and global-equity schedules back to the January 2010
  audit floor.** Every encoded cutover has a primary-source, day-level
  effective date; temporary pandemic schedules and PSE's two full closure
  dates are preserved. B3 preserves every explicit grid back to January 2010
  before switching to its modern cross-zone rule; BMV preserves its exact 2010
  spring exception before applying the operator's prospective New York-alignment
  policy from November 2010.
- **Regional bulk builders** for Asia-Pacific and other major global equities:
  `hours_for_apac_equities`, `hours_map_apac_equities`,
  `hours_for_global_equities`, and `hours_map_global_equities`.
- **Schedule-maintenance documentation:** the README now publishes an explicit
  source-review cutoff and machine-checked assurance counts, while a dated
  audit report, per-exchange verification ledger, normalized official-source
  registry, and repeatable update guide make freshness, scope, and evidence
  status auditable without overstating future accuracy.

### Changed

- **Breaking: `ExchangeCalendar::exchange()` now returns `Option<Exchange>`.**
  Venue calendars return `Some(exchange)` and product-family calendars return
  `None`. Callers that accept both identity kinds should match on
  `ExchangeCalendar::source()`; family-specific callers can use
  `market_hours_key()`.
- **Breaking (behavioral): maintenance uses a four-hour bound and trade-date
  semantics.** A same-trade-date gap is now `Halt`; an inter-trade-date gap is
  `Maintenance` only when its complete elapsed span is at most four hours and
  both dates are in one ISO week. A sourced continuously traded-week profile
  can also retain an operator-designated gap within that bound as maintenance;
  this covers CME cryptocurrency's Saturday window even though trading on both
  sides carries one trade date. Longer gaps such as CBOT grains' afternoon
  closure and policy-created early-close gaps are `Closed`.
- **Product-family selection is explicit.** The crate does not map symbols,
  roots, product codes, or MICs to keys. Venue-keyed compatibility defaults are
  documented, and callers are warned not to use them for products outside the
  named family. Nine unmodeled families—including CME NKD and the requested
  ICE/Eurex/SGX expansions—remain rejected rather than receiving guessed
  substitutes.
- **Equal `SessionRule` endpoints now mean a complete local-day session.** A
  rule such as Sunday `18:00→18:00` preserves one continuous session through
  Monday 18:00, including exact `session_bounds`; absence is represented by
  omitting the rule. This intentionally breaking 1.0-boundary correction removes
  `SessionRuleError::EmptyInterval`.
- **Canonical string Serde for `Exchange` and `MarketHoursKey`.** Both public
  identity enums now use their stable `snake_case` strings in every format.
  This replaces derive-generated enum ordinals in non-self-describing formats,
  so a future variant insertion or removal cannot silently decode as another
  venue or product family. JSON remains unchanged. Binary payloads written by
  earlier crate versions require a one-time migration to the canonical string
  representation.

### Removed

- **`Exchange::IntelligentcrossIqx` and the `intelligentcross_iqx` wire name.**
  Official materials identify IntelligentCross as an ATS and `IQX` as its
  market-data feed, while available primary evidence cannot state the ATS's
  exact first-live day. Removing the unused, misleading pre-v1 identity avoids
  returning today's schedule for dates before the venue existed. Persisted
  `intelligentcross_iqx` values must be removed or migrated by callers. There
  is no replacement `Exchange` variant: applications that still need the ATS
  must keep an external mapping rather than converting it to another venue or
  to `Exchange::Unknown`.
- **Four raw US schedule slices are no longer public:**
  `US_EQUITY_REGULAR`, `US_EQUITY_EXTENDED`, `NYSE_TEXAS_EXTENDED`, and
  `BLUE_OCEAN_EXTENDED` were implementation details that could bypass
  venue/date routing. Use `hours_for_exchange`, `hours_for_exchange_as_of`, or
  `calendar_for_exchange` and the public query APIs instead.

### Fixed

- **Exact CME cryptocurrency weekend semantics.** The key-backed calendar now
  joins adjacent one-midnight storage pieces into the two physical session
  blocks around Saturday maintenance and assigns both to the following open
  business date. Ordinarily that is Monday and the daily bar is Friday
  16:01 Pre-Open→Monday 16:00 CT; when caller policy closes Monday, weekend trading
  remains open, receives Tuesday's trade date, and the bar ends Tuesday at
  16:00. Friday 16:00 remains the final weekly close before the next trade-date
  week enters Pre-Open at 16:01. The weekday 16:01–16:02 and Saturday
  03:45–04:00 order-entry phases are now correctly `Extended`; the preceding
  closed portions remain `Maintenance`. Detached fixed snapshots preserve exact open/closed
  state but retain table-piece bounds because only the key calendar carries
  the family identity required for coalescing, weekly boundaries, and trade
  dates.
- **Civil-time discontinuities in public queries.** A rule occurrence whose
  local open and close both collapse onto the same instant in a DST gap is now
  omitted instead of producing a zero-width session or candle. Partially
  skipped rules resolve to the first real wall-clock second, and a wholly
  skipped civil date is correctly treated as an empty, closed calendar day.
- **Conditional future schedules are held until confirmed.** Announced Nasdaq,
  EDGX, NYSE Arca, MEMX, 24X, and FINRA TRF overnight expansions are monitored
  in the update guide but do not activate from provisional target dates. MX2
  Options, IEX Options, MRX's additional sessions, and GIX are likewise watched
  without premature identities or selectors. Runtime coverage will change only
  after the required readiness, regulatory, clearing, SIP, and production-day
  confirmations.
- **Bursa Malaysia, IDX, HOSE, and BMV January-2010 history.** Bursa's morning
  continuous session now runs through 12:30 under the pre-audit-floor v2 manual
  and its dated successors. IDX now includes its archived 09:10–09:30 pre-open
  before the 2013 expansion. HOSE retains the operator's exact January-2010
  phase table plus its sourced September 2010, March 2012, and July 2013
  revisions. BMV preserves the exact March 2010 early interval and uses the
  operator's prospective New York-alignment rule from November 1, 2010 through
  a date-aware Mexico City/New York offset selector.
- **Venue-union phases across APAC, China, and Mexico.** HKEX now treats its
  executable Extended Morning Session as continuous Regular trading and dates
  CAS from 2016-07-25. SET includes the exact 2025-05-06 DR night launch and
  post-midnight trade-date boundary. TSE/TWSE include ToSTNeT/block tails;
  SSE/SZSE retain 15:00–15:30 block trading from the audit floor; HOSE retains
  every era's put-through tail; IDX restores Negotiated Market through 16:30;
  and BMV preserves its sourced HD/ID `:06`, `:10`, and `:20` transitions.
  Security eligibility may be narrower than these exchange-availability unions.
- **Calendar candle boundaries now honor the profile's close semantics.**
  Always-open profiles no longer receive artificial daily, weekly, or monthly
  closes; adjacent phase handoffs are ignored when the requested session kind
  remains open; and a venue's first launch-day candle start no longer requires
  a pre-launch close.
- **ASX's pre-Service Release 15 staggered opening envelope.** The historical
  profile now keeps the opening-auction phase through the sourced latest Group
  5 transition at 10:09:15 Sydney time while retaining 10:00 as the earliest
  continuous-trading edge. The current post-June 23, 2025 profile is unchanged.
- **Primary-sourced European cash-equity phases and ICE Canada history.** LSE
  SETS now includes pre-trading, conservative randomized opening and midday
  auction edges, and its sourced 2012 closing-price crossing session. SIX and BME now use
  the operators' conservative two-minute and 30-second randomized opening
  edges, respectively; their pre-TAL closing auctions retain the same sourced
  maximum envelopes. Nasdaq Stockholm, Helsinki, and Copenhagen now use the
  market model's five-second randomized opening edge. SIX and BME retain
  their pre-TAL profiles before the sourced June 22, 2020 and December 4, 2023
  launches; Vienna now includes complete 2010/2017/2019/2020 ATX history. LSE,
  Xetra, SIX, BME, all three Nasdaq Nordic markets, Vienna, Euronext Milan, and
  Euronext Dublin now have complete primary-sourced histories for their stated
  scopes. ICE Futures Canada
  now preserves the sourced January-2010 19:00 pre-open / 20:00–13:15 CT
  continuous grid, its 2011 opening change, 2012 and 2013 close revisions,
  and the 2016 move to 13:20 before closing on the 2018 IFUS transfer's actual
  Sunday opening day.
- **Product-scoped international derivatives schedules.** Eurex benchmark
  futures now preserve the January-2010 07:30 pre-trading / 07:50 continuous
  grid and classify the fixed-UTC Asian pre-trading/opening auction separately
  from post-2018 continuous trading across CET/CEST; EEX is
  narrowed to Nordic Zonal Power Futures with its 2024 launch; ICE Europe
  identities now model Brent and FTSE 100 Futures. ICE Endex preserves the
  transferred Dutch TTF contract's sourced 2013 WebICE grid, 2026 opening eve,
  and recurring DST rule; IFAD models Murban
  in its New-York-locked schedule. SGX is narrowed to Three-Month SORA Futures
  with its sourced 2024 launch and auction gaps. Binance Futures is narrowed to
  USDⓈ-M perpetuals and is closed before the exact sourced 2019-09-13 04:00 UTC
  platform launch. Xetra now includes the sourced 2025 Extended Retail envelope
  for DAX shares. Each profile's exact product scope and historical boundary are
  recorded in the verification ledger.
- **Nasdaq-family equity schedules and history.** Nasdaq BX/Texas now preserves
  its sourced January-2010 08:00–19:00 ET grid and exact April 18, 2011 move to
  07:00–19:00, while PSX uses 08:00–17:00 and is
  closed before its sourced October 8, 2010 launch; its launch-day 09:00 open
  moves to 08:00 on the sourced December 13, 2010 date. Nasdaq Stock Market
  history now retains the 07:00 pre-market before its sourced March 18, 2013
  move to 04:00. Nasdaq's announced Night Session remains monitored but is not
  encoded until the operator files its final readiness confirmation.
- **Cboe US-equity accepted-order envelopes.** BZX and BYX retain their exact
  2014-12-02/2014-12-01 06:00 queue onsets; BZX moves to a 02:30 queue on
  2025-05-01. EDGX retains its exact 2021 queue changes. EDGA/EDGX still begin
  on their 2010-07-02 first-symbol production day, but an undated original
  queue onset is now disclosed as Partial instead of being silently excluded.
- **NYSE-family accepted-order envelopes and Texas continuity.** Current
  profiles now include the operator-published 06:30 queues for NYSE, American,
  National, and Texas and 02:30 for Arca. NYSE Texas is correctly modeled as
  the continuing CHX/NYSE Chicago identity: 07:00–17:00 at the January-2010
  floor, Pillar from 2019-11-04, and a non-substantive 2025 rename—not a new
  2025 launch. Incomplete staged/legacy queue histories remain Partial.
- **IEX and Blue Ocean ATS identity/history.** The `iex` exchange identity is closed
  before its sourced August 19, 2016 first production-symbol launch instead of
  inheriting predecessor-ATS history. Blue Ocean is
  closed before its October 5, 2021 production launch and is explicitly scoped
  to the production ATS service rather than its earlier beta/testing phase;
  its primary-sourced new-order trading window is 20:00–04:00. The live
  filing's sub-minute resting-book cleanup is outside that stated scope.
- **Primary-sourced US listed-equity-options queues.** Seventeen identifiers now
  include their current generic 06:00, 07:00, or 07:30 order-acceptance phase
  before the 09:30–16:00 ordinary-stock-options RTH; MEMX correctly rejects
  pre-09:30 orders. Exact launch history remains, while each unavailable queue
  onset is explicitly Partial instead of receiving an inferred date.
- **Euronext cash-market clocks and pre-open phases.** Paris, Amsterdam,
  Brussels, Lisbon, Dublin, and Milan now use the operator's published 07:30
  CET pre-open. Paris, Amsterdam, Brussels, and Lisbon use the published
  nominal 09:00 continuous-session start and 17:35 Trading-at-Last handoff;
  the per-security 0–30-second randomized auction uncross is explicitly outside
  the exchange-level schedule. Lisbon
  and Dublin retain their venue IANA zones while translating Euronext's
  Central-European clock one hour earlier locally, eliminating the former
  one-hour UTC shift. Legacy Euronext markets change pre-opening on the sourced
  March 20, 2023 date, while Milan changes on March 27. Milan's complete history
  is recorded; Dublin now preserves its operator-published pre-floor timetable
  through successive ISE order-book models and the 2018 calendar. All six rows
  now have complete history within their documented exchange-level scopes.
- **CME equity-index matching, queues, and scope.** The pre-November-18, 2012
  profile no longer receives the later 15:30–16:15 CT post-halt session. That
  extension begins on CME's published Sunday effective date, the 16:15→16:00
  close change begins on September 20, 2015's opening day, and the 15:15–15:30
  halt is removed from the exact 2021-06-27 opening. The fixed-current profile
  now includes the Sunday 16:00 and weekday 16:45 Pre-Open queues. The dated
  history retains the exact 2010 weekday-queue change but omits the Sunday
  queue where its 16:15→16:00 onset day is unavailable, so the family is
  explicitly Partial. Its scope excludes full-size `SP`, NKD, BTIC, and TACO.
- **MEMX and MIAX Pearl Equities launch and early-session history.** Both
  venues are now closed before their sourced September 2020 production
  launches. MEMX retains its exact 2020-10-05 shortening to a 17:00 close,
  2023-02-01 restoration to 20:00, and 07:00 pre-market before the exchange's
  May 19, 2025 production launch of 04:00 trading; MIAX Pearl retains its
  regular-hours-only profile before its sourced February 20, 2025 Early and
  Late Trading Session launch.
- **CBOT standard-size grain/oilseed history and accepted-order phases.** The
  January-2010 profile now uses the published matching, morning Pre-Open, and
  PCP phases; PCP expands on 2010-04-19 and the morning queue moves to 08:00 on
  2011-12-27. The sourced 17:00–14:00 electronic regime begins Sunday
  2012-05-20, CME's 19:00–07:45 / 08:30–13:15 regime begins Sunday 2013-04-07,
  and the 13:20 close begins Sunday 2015-07-05. The fixed-current profile also
  includes the published 16:00/16:45 evening queues and 14:30–16:00 PCP.
  Their post-2012 onset chain is incomplete, so those phases are omitted from
  the affected dated eras and the history is Partial. Mini grains are excluded.
- **CME Group interest-rate, livestock, and cryptocurrency family history.**
  Interest rates retain the January-2010 17:30→16:00 CT matching grid and
  queues, the exact 2010 weekday-queue change, and the 2011-10-02 move to a
  17:00 match open. Livestock retains its overnight baseline, the 2014-10-27
  removal of evening trading, the 2016-02-29 move to 08:30–13:05, and the exact
  2020 move to an 08:00 Pre-Open; its fixed-current profile also includes PCP.
  Cryptocurrency is closed before the 2017-12-17 Bitcoin launch, preserves the
  five-day matching era, and switches at the exact 2026-05-29 24/7 transition
  with its 16:01 weekday and 03:45 Saturday Pre-Open phases, plus the temporary
  2026-08-01 maintenance extension and restoration. All three histories are
  Partial only for named older PCP/Pre-Open onset gaps; no date is inferred.
- **COMEX, NYMEX, CME FX, and ICE U.S. product scopes and queues.** The shared
  NYMEX energy/PGM and COMEX metals profile preserves the sourced January-2010
  16:15 CT close and September 2015 move to 16:00, while its fixed-current
  snapshot includes Sunday 16:00 and weekday 16:45 Pre-Open. Its scope is NYMEX
  CL/MCL/QM, NG/MNG/QG, HO/RB/BZ, and PL/PA plus COMEX GC/MGC, SI/SIL, and
  HG/MHG, excluding alternate session types and differently specified products.
  Standard-grid CME FX retains its 17:00→16:00 matching history and exact 2010
  weekday-queue change; current Sunday Pre-Open is included. Both CME histories
  are Partial only because the Sunday queue's earlier onset lacks a source day.
  ICE U.S. now represents NYSE FANG+ Index Futures with its exact November 2017
  launch, 30-minute Pre-Open queues as Extended, and matching as Regular instead
  of an uncited venue-wide default.
- **CFE's 2013 phased extensions, 2014 launch, and 2018 system-migration
  history.** The profile now retains CFE's 07:00 CT morning open before the
  sourced October 28, 2013 launch of its Monday–Thursday 15:29–15:30 pre-open
  and 15:30–16:15 session, then moves the morning open to 02:00 on the sourced
  November 4 second phase while retaining that pre-open.
  The nearly 24-hour profile starts on the sourced Sunday, June 22, 2014 launch
  rather than June 1, including its exact 16:15–17:00 Sunday pre-open. Its
  continuous Monday–Thursday 15:30–08:30 ETH comes directly from CFE-2014-010.
  A further sourced revision on February 25, 2018
  restores the 16:00–17:00 CT daily break introduced with CFE's system migration
  and classifies its 15:15–15:30 and later opening queues as extended rather
  than closed. Randomized queue starts use the sourced conservative latest edge
  of three seconds from 2018-02-25 and six seconds from 2018-08-12.
  The January-2010 08:30–15:15 baseline plus the December 2010 07:20 and
  September 2011 07:00 morning extensions are now date-aware, completing the
  VIX-futures normal-week history from the audit floor.
- **FINRA TRF session classification and 2026 opening-time history.** Carteret,
  Chicago, and the FINRA/NYSE TRF now classify 09:30–16:00 ET as regular and
  their before/after-RTH reporting windows as extended. Their point-in-time
  profiles retain the former 08:00 ET system open before March 30, 2026 and
  apply FINRA's 04:00 ET open from that effective date. Chicago is now closed
  before its sourced September 10, 2018 facility launch; that launch was
  test-security-only through September 21. FINRA's later overnight expansion
  remains monitored but unencoded while implementation follows the SIP rollout.
- **Calendar queries remain total at Chrono's representational bounds.** Local
  date resolution and forward/backward scans now use checked, inward-bounded
  arithmetic, and a bar that cannot advance past `DateTime<Utc>::MAX_UTC`
  returns `None` instead of overflowing.
- **Oversized intraday intervals clamp to the enclosing session close** instead
  of overflowing, and candle starts now exist exactly when their paired ends
  do. Weekly/monthly walks also retain the last representable valid close.

## [0.2.2] - 2026-08-21

Documentation only; no code, no API, and no dependency change. The published
`0.2.1` README carried claims that `0.2.1` itself had made false.

### Fixed

- **The README no longer says the crate logs through `tracing`.** Two claims
  survived the dependency's removal in `0.2.1` and shipped with it: the
  coverage table's always-open row stated that `Exchange::Unknown` "also logs a
  one-shot `tracing::warn!`", and the dependency paragraph listed `tracing`
  itself as a dependency. Neither has been true since `0.2.1`.
- **The version and MSRV badges are read from crates.io** rather than
  hardcoded. The version badge had been left at `0.2.0` across the `0.2.1`
  release — the same class of duplicated fact that `AGENTS.md` warns about —
  and pointed at a yanked version from the crate's own front page. Both badges
  now track the published crate and cannot drift again.

## [0.2.1] - 2026-08-21

A dependency removal and the lint attribute that fell out of it. No change to
any returned value. `0.2.0` is yanked in favour of this release.

### Removed

- **`tracing` is no longer a dependency.** It backed a single one-shot
  `warn!`, fired when a caller passed `Exchange::Unknown` to
  `hours_for_exchange` and received the documented 24×7 UTC fallback. That
  variant is reachable only deliberately — it is not a `Default`, and neither
  `FromStr` nor serde will produce it from an unrecognized name (both error) —
  so the warning reported a choice the caller had just made, to a subscriber
  most consumers of a pure-computation crate never install. The dependency
  tree is now `chrono`, `chrono-tz`, and `serde` alone — 25 locked packages
  down to 19, since `tracing-attributes` also pinned a second `syn` major
  alongside the one `serde_derive` uses. Returned values are unchanged:
  `hours_for_exchange(Exchange::Unknown)` still yields the 24×7 UTC fallback,
  as the contract suite's always-open invariant pins. Only the log line is
  gone.

### Changed

- `hours_for_exchange` is now `#[must_use]`, matching every other query in the
  crate. It was the sole public query without the attribute: the removed
  `warn!` was a side effect, so `clippy::must_use_candidate` never fired on
  it. **This is visible downstream** — a caller that discards the returned
  `MarketHours` now gets an `unused_must_use` warning, which is an error under
  `#![deny(warnings)]`. The signature and the returned value are unchanged, so
  any call that uses its result is unaffected.

## [0.2.0] - 2026-08-20 [YANKED]

API corrections published the same day as `0.1.0`. Both versions are now
**yanked** — `0.1.0` as superseded by this release, `0.2.0` in turn by
`0.2.1`, which is the release to use. The breaking changes below are therefore
breaking only against versions no longer available for selection; they remain
listed here because they are real and shipped.

### Changed

- **Breaking:** `Exchange` is `#[non_exhaustive]`, mirroring `MarketHoursKey`.
  Dependents must match it with a wildcard arm; in exchange, future venue
  additions are minor releases instead of breaking ones. Enumerate the
  variants of the compiled version via the new `Exchange::ALL`.
- **Breaking (behavioral):** the day's last intraday bar now ends at the daily
  close itself. Previously, when a `Minutes`/`Hours` bar ended exactly at a
  daily close followed by a maintenance break, `candle_end` snapped the end to
  the next session open (CME's last bar reported 17:00 CT instead of 16:00) —
  a V1-inherited exception that contradicted the crate's own end-exclusive
  close convention by counting closed time as bar time. Intraday bars now
  simply clamp to the enclosing session close, everywhere.
- **Breaking (behavioral):** `is_maintenance` now classifies the enclosing
  gap, not the distance to the reopen. It is true exactly when `t` lies inside
  a closed gap between two sessions whose whole close-to-reopen span is
  shorter than six hours. The old rule — closed and reopening within 90
  minutes — missed the front of every break longer than its threshold (the
  first 30 minutes of ICE's two-hour break, the first half of Eurex's
  three-hour gap, almost four hours of CBOT grains' 13:20→19:00 CT afternoon)
  and wrongly flagged pre-open windows (CME Sunday 15:30–17:00 CT, US
  equities 02:30–04:00 ET) that belong to overnight or weekend closures.

### Added

- `Exchange::ALL` — every variant, in declaration (`Ord`) order.
- `Exchange::as_str`, `Display`, and `FromStr` — one canonical `snake_case`
  name per venue, identical to the serde wire form, so string-keyed callers
  parse (`"nyse_arca".parse::<Exchange>()`) instead of pattern-matching. An
  unrecognized name is a `ParseExchangeError` (new public type) carrying the
  offending input, never a silent `Exchange::Unknown`.

## [0.1.0] - 2026-08-20 [YANKED]

Initial release.

### Added

- **69 venues** across US equities and options, US and international futures,
  EU equities, and always-open crypto, each schedule transcribed from a primary
  source and cited beside its table.
- **Session queries** — `is_open` / `is_open_with` over regular, extended, or
  both; `session_bounds`; `next_session_after`; `is_maintenance`;
  `is_closed_all_day_*`; `normal_week_open_seconds`.
- **Calendar-aware bar boundaries** — `candle_start` / `candle_end` across
  intraday, daily, weekly, and monthly `CalendarResolution`, clamped to real
  sessions so no bar spans a closed period and a daily bar closes at the
  session close rather than at midnight.
- **Point-in-time hours** — `hours_for_exchange_as_of` returns the schedule in
  effect at an instant, with cutovers compared in the venue's own local zone.
  A cutover exists only where a primary source states a day-level effective
  date; changes that are real but undated are recorded as known gaps rather
  than given invented dates.
- **Shared futures profiles** addressable by product family through
  `MarketHoursKey`, via `session_profile` / `hours_for_market_hours_key`.
- **Bulk builders** with deterministic `BTreeMap` iteration order.
- **Validated `SessionRule` construction** (`new` / `validate`) over a stated
  seconds-since-midnight domain, fence-checked against every shipped table.

### Notes

- Timestamps are `chrono::DateTime<Utc>` in and out; a venue's local zone and
  its DST rules stay internal. Opens resolve to the earliest valid instant and
  closes to the latest, so a session stays inclusive while its end-exclusive
  close lands on the true boundary.
- Boundary queries return `Option`. `None` means no session of the requested
  kind exists in the search horizon — a venue queried before its go-live date,
  or a zero-length interval — never a fabricated degenerate pair.
- This is a **normal-week** model: holidays, early closes, half-days, and
  product-level variations are out of scope. The profiles are exchange-level
  defaults and a best-effort model, not an authority. Have a human verify a
  venue against its currently published schedule before trading on it.
- No runtime state, no I/O, no clock reads, no floats, and
  `#![forbid(unsafe_code)]`.

[Unreleased]: https://github.com/SharurTrading/exchange-hours-rs/compare/v1.0.0...HEAD
[1.0.0]: https://github.com/SharurTrading/exchange-hours-rs/compare/v0.2.2...v1.0.0
[0.2.2]: https://github.com/SharurTrading/exchange-hours-rs/compare/v0.2.1...v0.2.2
[0.2.1]: https://github.com/SharurTrading/exchange-hours-rs/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/SharurTrading/exchange-hours-rs/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/SharurTrading/exchange-hours-rs/releases/tag/v0.1.0
