<!-- SPDX-License-Identifier: MIT-0 -->

# Changelog

## [Unreleased]

### Added

- The coverage engine's **bridged residual** (issue #296, Tier 1; the charter
  amendment of 2026-10-06 UTC): a span **between two audited holiday windows**
  whose normal week the identity sources no longer refuses whole dates — the
  session questions answer from the sourced normal week the timeline serves,
  the holiday layer stays honestly absent, and the metadata reports the span
  as `CoverageGapReason::HolidayWindowsBridged` rather than complete. The
  spans that now answer: `sgx_securities`'s 2020-01-02..2024-12-31 capture gap
  (1,826 days), `nse_india`'s 2018 unrecovered year (365 days) and `cfe`'s two
  between-window spans 2015-01-03..2015-02-14 and 2016-01-21..2017-04-09 (488
  days) — 2,679 days lifted from whole-date refusal — and eight of their
  neighbouring window dates that previously refused as resolution edges
  (`sgx_securities` 2019-12-31, 2020-01-01 and 2025-01-01; `nse_india`
  2011-12-31, 2013-01-01, 2017-12-30, 2017-12-31 and 2019-01-01) are fully
  covered again. One-flank spans (below the first window, above the last)
  refuse unchanged, and the residuals are disclosed in the owners' evidence
  files with their #213 and NSE-list closers.
- `cfe` / `cfe_vix` holiday history extended below the 2017 window: the
  operator's own holiday press releases of 2014-2017 and the CFE information
  circulars IC15-011, IC15-023, IC15-028 and IC15-039 (recovered 2026-10-06
  UTC from the operator's `ir.cboe.com` news directory and the archived
  `CFEinfocirc` series, one Wayback `id_` replay each) add the audited windows
  2014-12-24..2015-01-02 and 2015-02-15..2016-01-20 (twenty rows) and extend
  the 2017-2026 rows with the pre-migration next-day legs the operator's own
  charts and notices print (seven `LateOpen` rows and the 2017-11-24 combined
  half day). The audited row set moves from 113 to 140 over three windows,
  with 2017-07-03 still the one `Unsourced` date.
- `nse_india` 2012 holiday list recovered: the operator's annual circular
  66/2011 (dated 2011-12-09, download `NSE/CMTR/19539`), which states the
  complete 2012 capital-market trading-holiday list, surfaced in the Wayback
  archive on 2026-10-06 UTC, so the 2012 window merges and fifteen rows ship
  (fourteen closures plus the year's own withheld Muhurat date 2012-11-13).
  Three more Muhurat circulars recovered the same day (121/2010, 65/2015 and
  56/2016) turn 2010-11-05, 2015-11-11 and 2016-10-30 into `ReplacementBlocks`
  days restating each printed special-session schedule — eight Muhurat days
  now ship blocks, seven dates stay withheld (the Muhurat dates of 2011, 2012,
  2013, 2014, 2017 and 2019 plus the 2026-11-08 banner date), and 2018 is the
  one unrecovered year, the bridged residual.
- **Refusal context (issue #296, Tier 3)**: every
  `CalendarQueryError` still refuses exactly as before, and now carries the
  sourced normal-week baseline its date sits inside —
  `CalendarQueryError::normal_week_baseline()` returns the refused date's
  weekday and the normal-week windows the identity's timeline serves there
  (`NormalWeekBaseline`, with `Display` rendering "normal week Thursday: open
  09:15-15:30, …"), and the `UnresolvedGap` display appends "the holiday
  arrangement is unsourced". The baseline is absent below the support floor
  and where the ledger records the week as carried.

### Fixed

- `nse_india`: the five Muhurat Trading days whose instants the operator has
  published now ship as `ReplacementBlocks` rows restating each printed
  special-session schedule, keyed to the operator's own capital-market
  circulars recovered from the Wayback archive (circulars 98/2020 for
  2020-11-14, 124/2022 for 2022-10-24, 139/2023 for 2023-11-12, 147/2024 for
  2024-11-01 and 124/2025 for 2025-10-21; one Wayback `id_` replay each). The
  withheld (`Unsourced`) Muhurat count moves from fourteen to nine; 2025-10-21
  trades its printed 13:15-15:05 schedule instead of refusing. The same day's
  second pass (recorded in the Added entries above) extends the recovery to
  eight shipped Muhurat days and seven withheld dates.
- `tadawul`: the ledger horizon moves from 2010-01-12 to the 2010-01-01
  support floor — the operator's Trading Times page bytes of 2010-01-12 are
  byte-identical to its 2009-09-12 and 2009-11-12 captures, and the
  pre-floor 2008-11-19 English print states the identical grid, so the
  formerly carried region 2010-01-01..2010-01-11 is sourced and nothing below
  the floor is carried.
- `asx`: the carried normal week 2010-01-01..2013-09-15 is no longer a refusal
  — the 2026-10-05 no-changes sweep of the operator's archived market-phases
  pages found the identical staggered grid printed 2009-02-01, 2010-01-06 and
  2011-03-25 through 2013-09-02, so the horizon sits at the 2010-01-01 floor
  and the formerly carried dates answer from the floor-sourced grid.
- `sgx_securities`: the normal-week horizon moved from 2011-08-01 to the floor
  — the outgoing pre-2011-08-01 grid is stated in the operator's own Practice
  Note 8.2.1 amendment (issue date 1 August 2011) and the sweep found no
  declared hours change inside the carried span; the 2010-2013 and 2020-2024
  holiday capture gaps still refuse, now as holiday-coverage gaps (#213).
- `euronext_paris`: the carried span 2010-01-01..2010-12-23 answers from the
  legacy grid — the operator's own PAR_20101126_06372_EUR notice of 26/11/2010
  restates the ordinary 07:15 pre-opening and 09:00 opening inside the span and
  no surviving 2010 channel declares a normal-week change, so the horizon sits
  at the floor.
- `borsa_istanbul`: the carried span 2010-01-01..2010-03-24 is verified — the
  operator's own 2010 annual report dates the era's last session-hours changes
  to 19 October and 13 November 2009 (both pre-floor) and the 2010-04-30
  change left the equity envelope byte-identical, so the horizon sits at the
  floor; the holiday window keeps its 2012-03-02 opening.
- `nzx`: the carried region 2010-01-01..2010-01-04 contains no trade date and
  the pre-floor 2009-12-04 print states the identical grid, so the horizon
  sits at the floor and the region's carried refusal shape retires; its two
  weekday closures remain the 2010-01-05 sheet's own rows.
  byte-identical to its 2009-09-12 and 2009-11-12 captures, so the formerly
  carried region 2010-01-01..2010-01-11 is sourced and nothing below the
  floor is carried.
- evidence, `comex`, `nymex`, `globex_energy` and `globex_fx`: the no-changes
  sweep records now state the 2010-2011 notice leg's own counts — 114 held,
  113 readable — instead of the 2008-2011 series-wide 242-readable figure,
  and `globex_fx` names each zero-byte replay's own store INDEX (#304). No
  row, window, tier or instant moves: the horizon conclusions stand on every
  2010-2011 month being covered by readable notices.
- evidence, `sgx_securities`: restores the fourth domain-lineage pass record
  (2026-10-06 UTC, the predecessor domains `ses.com.sg`, `info.sgx.com` and
  `sgx.com/others/` — all negative; artifacts under
  `holidays/raw/equities/sgx_securities/domain-lineage-2026-10-06/`) that the
  #301 rebase dropped, and the coverage intro now dates `tadawul`'s Horizon
  move to its 2026-10-06 domain-lineage sweep rather than the 2026-10-05
  equities half (#306). No row, window or instant moves.

## 1.0.0 — 2026-10-05

Initial release.
