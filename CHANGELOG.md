<!-- SPDX-License-Identifier: MIT-0 -->

# Changelog

## [Unreleased]

### Fixed

- Correct the Brent evidence comparison to ICE's printed trading hours;
  its settlement calculation is not a session close. OANDA rules are unchanged.

- BME opening and closing auction calls now accept orders without reporting
  the market open, on normal days and half days. The sourced uncross windows
  and Trading-at-Last remain executable.
- LSE's five withheld 2025 spring/summer holiday dates now answer as sourced
  closures, recovered from a verbatim public mirror of the operator table.

### Added

- Nasdaq Copenhagen, Helsinki and Stockholm cash holiday tables for 2026–2027, including Stockholm's half-day phases through 13:30 local time.

- Archived OANDA UK FX and spot-metals holidays from 2024-02-21 through
  2024-12-31: Christmas FX closure and fourteen literal metals arrangements,
  including Thanksgiving and Christmas reopens. Earlier normal-week history
  remains unresolved under #332.
- Archived OANDA UK US-index and Treasury-CFD holiday arrangements for 2024,
  preserving literal reopens and retaining the omitted Treasury Christmas Eve
  arrangement as a typed gap.
- Archived OANDA UK commodity and Brent holiday arrangements from
  2024-02-21 through 2024-12-31, preserving the separate group scopes and
  retaining unpublished Brent reopenings and special-day hours as typed gaps.
- Euronext Amsterdam's own 2010-2026 holiday table: 108 rows, with the
  two 2026 December half-day instants explicitly withheld until publication.
- BME principal-share cash holidays from 2010 through 2026, with complete
  half-day phase replacements and typed gaps for unpublished 2026 year-end
  timetables.

## [1.1.0] - 2026-10-11

### Changed

- Three identities turned **served** under the OANDA programme's consumer
  admission (SharurPlatform#1046): the `eurex` and `eurex_fixed_income` keys —
  Germany 40 / Euro 50 and Germany 10Y Bund — whose tables the Eurex venue's
  calendar work already ships at the same audited depth, and `ice_us_sugar`,
  whose table the IFUS notices waves had already built. Review cadence moves
  to monthly for all three; the served-scope inventory gains their rows and
  the verdict tally moves to thirty complete and six incomplete.

### Added

- `Exchange::Oanda` and the first two OANDA-book keys, `oanda_fx` (all FX
  products except TRY pairs: Sunday 17:05 to Friday 16:59 New York time with
  the six-minute 16:59–17:05 daily break) and `oanda_try_fx` (a daily
  Monday–Friday 08:00–17:00 Central European session with an overnight gap).
  The books trade straight through the daily maintenance halts of the futures
  clocks they hedge on, so they are book keys, not futures mappings. The
  `oanda_fx` holiday table ships the operator's three stated trade-date
  closures (2025-01-01, 2025-12-25, 2026-01-01) over its
  2025-01-01..2026-10-30 audited window, and the venue's table mirrors it —
  the first wave of the OANDA-listed market coverage programme
  (SharurPlatform#1046, oanda-rs#30).
- Four additional OANDA-book keys for the UK hours page's spot metals,
  US-anchored CFDs and Brent listings:
  `oanda_metals` (XAU/XAG and crosses, Sunday 18:05 to Friday 16:59 New York
  time), `oanda_us_index_cfd` and `oanda_us_commodity_cfd` (four US indices and five
  US energy/commodity-metals listings on one continuous clock — Chicago
  17:01 to 15:59, the same instants as New York 18:01 to 16:59 — split into
  two keys because the operator's holiday page gives different early closes), and
  `oanda_brent` (London-anchored by the page's own dagger note, Sunday 23:01
  to Friday 21:59 with its two stated breaks). Holiday tables ship each
  group's printed rows from the operator's 2025-01-01..2026-10-30 dated
  sections, including typed withholdings where the page's own
  regular-hours vocabulary makes its silence a gap.

- `oanda_us_bond_cfd` separates the four Treasury listings from the index
  book. Good Friday 2026 closes bonds at 11:14 ET and indices at 09:14 ET;
  Carter's Day 2025 closes bonds at 13:15 ET. This does not activate any
  incomplete OANDA scope or certify the remaining product programme (#332).

### Fixed

- The US-index key's exact holiday scope is restricted to its four US
  indices. Japan 225's two listings share the normal clock but require
  independent holiday-scope evidence before exact consumer routing (#332).
- The current OANDA `***` instruction follows each row's named local zone:
  Sydney for Australia 200's marked close and Frankfurt for the German,
  Euro 50 and Bund marked openings. The older guide's US-season Australian
  selector belongs to its older grid; its historical change date stays
  unsourced (#332).
- Carter's Day 2025 index close resolves to 09:30 ET from the UK page's
  dated 24-hour context and stated 18:00 reopening. The following trade
  date preserves that literal 18:00 open; Memorial's incomplete minute
  and the remaining historical/scope gaps stay withheld (#332).
- OANDA holiday instants now preserve the operator's exact minutes: the
  missing MLK 2025 closes, New Year 18:04 reopens, 2025 US 18:00 and Brent
  20:00 ET reopens, and Holy Thursday's 17:00 ET close (#330).
- Bond CFDs retain ordinary trading on the index-only Independence Eve
  notice. The incomplete Memorial Day 2026 reopen refuses the following
  index/bond trade date until the operator clarifies it (#332).
- Brent's Monday session bounds include the Sunday 23:01 London opening
  (#331). Replacement-session searches select the nearest chronological
  occurrence across normal rules and replacement blocks (#329).

- The OANDA order-acceptance scope is explicit: off-hours pending-order
  storage is a broker facility outside the published session phases (#333).

## [1.0.2] - 2026-10-10

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

- The ICE Futures U.S. holiday notices the earlier waves recorded as
  unretrieved or nonexistent were recovered on 2026-10-09 UTC through the
  operator's own notices listing service (the machine channel behind the
  client-side notices page, found by reading the page's JS bundle): the 2025
  Independence Day notice, the 2025 Christmas notice, the 2026 MLK notice and
  the 2026 Presidents Day notice. The 2025 dates now state their instants —
  FANG+ closes at 13:15 NY on 2025-07-03 and 2025-12-24 and 13:00 NY on
  2025-07-04, the dollar index at 13:00 on 2025-07-04 and 13:45 on
  2025-12-24, and 2025-12-26 is audited normal for both index families per
  the Christmas notice's `Regular Hours`; Cotton gains rows for its own
  2025-07-07 late open at 08:00 NY, the 2025-12-24 early close at 13:05 NY
  and the 2025-12-26 late open at 07:30 NY, Coffee, Cocoa and Sugar 11 gain
  the same Christmas rows, and FCOJ gains the 2025-12-24 early close. The
  2026 MLK and Presidents Day notices (issued December 2025 — the earlier
  "no such notice exists" record was an archive-channel failure) give FANG+
  the 13:00 NY early close on 2026-01-19 and 2026-02-16 and leave the dollar
  index regular both days. The `iceus` venue gains one `Unsourced` row
  (2025-07-07, Cotton's late open) and its counts move to 49 rows / 42
  withheld; 2026-12-28 stays withheld, no 2026 Christmas notice having
  issued at the re-check (#168).
- The `cfe` 2017-07-03 withholding closed as data on 2026-10-10 UTC: the
  controlling circular CFEIC17-022 ("Modified Trading Hours for Independence
  Day Holiday", June 16, 2017) was recovered from Common Crawl CC-MAIN-2017-34
  after the `cfecirculars.com` channel proved dead (NXDOMAIN, zero Wayback and
  zero Common Crawl captures) and the operator's own circulars index in the
  Wayback Machine named it; it states `Trading in all CFE products will close
  at 12:15 p.m. on Monday, July 3, 2017`, so the date ships `early close
  12:15 CT` and the `cfe` audited windows ship zero `Unsourced` dates.
- The `lse` five-date 2025 gap and the `cme` 2023-01-16 marker survived their
  next channels, recorded negative: the LSEG business-days page's only
  2024-2025 captures (2024-05-19, 2025-08-07) are client-side shells stating
  no dates, and the CME mirror sweep found no holiday or trading-hours page
  anywhere in the archived `cmegroup.cn`, a parked `cmegroup.fr`, empty
  `education`/`advisories` paths, and an HTTP 500 from the one further
  archived trading-hours-service capture querying a January 2023 window.
- The ICE monitoring entry point moved: `ice.com/trading-hours` 404s live and
  the section navigation now names `ice.com/holiday-hours`, whose IFUS entry
  re-fetched byte-identical to the held calendar (verified 2026-10-10 UTC).

- The `nasdaq` venue's four withheld holiday dates closed as data on the
  operator's own Equity Trader Alerts (retrieved 2026-10-09 UTC): 2010-11-26
  and 2011-11-25 now ship `early close 13:00` keyed to alerts 2010-73 and
  2011-54, whose Early Closing Schedules print the Nasdaq day session and
  closing cross at 1:00 p.m. (both alerts still serve live from the
  operator's Trader News pages); 2012-10-30 ships `closed` keyed to alert
  2012-45, which states the Hurricane Sandy Tuesday closure unconditionally;
  and 2025-01-09 ships `closed` keyed to alert 2025-1 (Wayback replay of
  capture 20250122151135), which states the National Day of Mourning closure.
  No date inside the `nasdaq` audited window is withheld any more, so the
  identity reads complete to 2026-12-31.
- The `nse_india` Muhurat hunt's 2026-10-09 UTC re-check is recorded:
  completed apex-host Wayback CDX sweeps (the prior pass's two apex files
  were error pages), the per-year issuance windows 2011/2012-2014/2017/2019
  negative again, the `/content/cmtr/` path family empty, and the operator's
  own live circulars channel listing 414 CM circulars 2026-09-22..2026-10-09
  with no Muhurat item — the seven withheld Muhurat dates stay withheld and
  the 2026-11-08 watch is re-dated. Ledger rows re-reviewed 2026-10-09.

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
