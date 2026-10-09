<!-- SPDX-License-Identifier: MIT-0 -->

# Changelog

## [Unreleased]

### Changed

- The four CME venue holiday tables (`cme`, `cbot`, `comex`, `nymex`) now
  carry their profile clock's rows verbatim under the 2026-10-09 charter
  amendment completing the profile-clock rule (#153, #242): `cbot` mirrors
  `globex_grains`, `cme` mirrors `globex_equity_index`, and `comex`/`nymex`
  mirror `globex_energy`. Every holiday date a venue's clock answers now
  states its actual arrangement — full close, early close, late open or
  replacement day — where the earlier intersection rule withheld it as
  `Unsourced` because two routed families print different instants; the
  dissenting family's row stays in its own family table and the venue
  evidence files name the disagreements. CBOT's Thanksgiving now answers
  closed instead of refusing as unsourced.

### Fixed

- The globex futures families' remaining not-worked-up holiday markers are
  resolved where the operator's own documents state the arrangement: the
  three Juneteenth `Unsourced` markers (2019, 2020 and 2021) are deleted in
  every family — CME's own consolidated annual bundles name no Juneteenth
  arrangement in those years, so the dates are audited normal — and
  2023-02-20 and 2023-04-07 are worked up at T1 from CME's own summary
  sheets `files/presidents-day.pdf` and `files/good-friday.pdf` for
  `globex_energy`, `globex_equity_index`, `globex_livestock`,
  `globex_cryptocurrency` (Good Friday only; its Presidents' Day sheet
  prints the ordinary close) and `globex_fx` (same shape). 2023-01-16 stays
  a recorded gap in every family (no operator document states it; channels
  re-enumerated) and `globex_nikkei_225_dollar` keeps its 2022-05-30 and
  thirteen 2024 markers (no `THBP-B` capture exists in the archive — swept
  2026-10-09). Ledger rows re-reviewed 2026-10-09.

## 1.0.1 — 2026-10-08

### Removed

- The crate archive no longer ships the repository's one-off Python
  verification tooling under `tools/`. The package carries the library, its
  evidence, and the maintenance documents only.

## 1.0.0 — 2026-10-08

Initial release.
