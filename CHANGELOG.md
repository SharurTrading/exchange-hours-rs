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
  byte-identical to its 2009-09-12 and 2009-11-12 captures, so the formerly
  carried region 2010-01-01..2010-01-11 is sourced and nothing below the
  floor is carried.

## 1.0.0 — 2026-10-05

Initial release.
