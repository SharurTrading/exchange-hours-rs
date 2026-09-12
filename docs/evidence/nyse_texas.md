<!-- SPDX-License-Identifier: MIT-0 -->

# `nyse_texas` — evidence

- **Kind:** `Exchange`
- **Owner module:** [`nyse.rs`](../../src/calendar/schedules/equities/us/nyse.rs)
- **Source sets:** [`US-NYSE-EQUITIES`](../schedules/sources.md#us-nyse-equities)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

Stable CHX/NYSE Chicago identity: 07:00–17:00 from the January-2010 floor, 06:30–20:00 from the 2019-11-04 Pillar migration, and a non-substantive 2025 Texas rename. It did not launch in 2025. **Systems in scope (2026-09-02):** one equities matching system; this SRO operates no options or bond facility. No discrepancy.

## Revision rows

- 2019-11-04 — T1 — NYSE Chicago migration notice — the Pillar migration establishes the 06:30 acceptance edge around the 07:00–20:00 three-session grid.

## Sources

Row review: 2026-08-22 (UTC) is the date the ledger row was last reviewed as a
whole. Per-source retrieval dates were not recorded before the 2026-09-12
migration; where a later targeted review, capture or document date is recorded
beside a source below, that date governs for that source, and dates are added
as each source is re-verified.

- <https://www.nyse.com/markets/nyse-texas> — the operator's NYSE Texas market page.
- <https://www.nyse.com/trade/hours-calendars?os=.> — the current hours and calendars table.
- <https://www.sec.gov/rules/sro/chx/2009/34-60775.pdf> — the CHX filing behind the floor-era 07:00–17:00 grid and its cross-only late crossing session.
- <https://www.sec.gov/files/rules/sro/nysechx/2019/34-86709.pdf> — the NYSE Chicago Pillar hours filing.
- <https://www.nyse.com/publicdocs/nyse/markets/nyse-chicago/NYSE_Chicago_Migration.pdf> — the 2019-11-04 migration notice.
- <https://www.sec.gov/files/rules/sro/nysechx/2019/34-87264.pdf> — the companion 2019 filing.
- <https://www.sec.gov/files/rules/sro/nysechx/2025/34-102507.pdf> — the 2025 continuity order for the Texas conversion and rename.

## Gaps and residual risks

- **None open.** The floor-era CHX grid is sourced by a 2009 filing that
  predates the January-2010 floor, so the baseline is sourced through the floor
  and nothing above it is carried.
- **Identity note.** This is the same registered exchange formerly called NYSE
  Chicago and CHX; the 2025-03-28 conversion and rename were non-substantive and
  it did not launch in 2025. It is unrelated to the `nasdaq_bx` venue the
  operator also renamed Nasdaq Texas.
- **System coverage (2026-09-02).** No discrepancy. One equities matching
  system; this SRO operates no options or bond facility.

> Shared module. The narrative for
> [`nyse.rs`](../../src/calendar/schedules/equities/us/nyse.rs)
> lives in [`nyse`](nyse.md#module-narrative-moved-from-srccalendarschedulesequitiesusnysers-on-2026-09-12-utc).
> Sibling identities: [`nyse_arca`](nyse_arca.md), [`nyse_american`](nyse_american.md), [`nyse_national`](nyse_national.md).
