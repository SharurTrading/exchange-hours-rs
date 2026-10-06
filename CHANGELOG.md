<!-- SPDX-License-Identifier: MIT-0 -->

# Changelog

## [Unreleased]

### Fixed

- `nse_india`: the five Muhurat Trading days whose instants the operator has
  published now ship as `ReplacementBlocks` rows restating each printed
  special-session schedule, keyed to the operator's own capital-market
  circulars recovered from the Wayback archive (circulars 98/2020 for
  2020-11-14, 124/2022 for 2022-10-24, 139/2023 for 2023-11-12, 147/2024 for
  2024-11-01 and 124/2025 for 2025-10-21; one Wayback `id_` replay each). The
  withheld (`Unsourced`) Muhurat count moves from fourteen to nine; 2025-10-21
  trades its printed 13:15-15:05 schedule instead of refusing.
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
