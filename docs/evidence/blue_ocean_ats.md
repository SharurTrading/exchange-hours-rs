<!-- SPDX-License-Identifier: MIT-0 -->

# `blue_ocean_ats` — evidence

- **Kind:** `Exchange`
- **Owner module:** [`ats.rs`](../../src/calendar/schedules/equities/us/ats.rs)
- **Source sets:** [`US-BLUE-OCEAN`](../schedules/sources.md#us-blue-ocean)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

Production ATS new-order service only: closed before the sourced 2021-10-05 launch, then 20:00–04:00 Sunday–Thursday. The live ATS-N's sub-minute resting-book cleanup and pre-production beta/testing are outside this row's stated scope. **Systems in scope (2026-09-02):** one matching system. The live ATS-N's 18:15 test-order window is excluded as testing, the post-04:00 sub-minute cleanup stays outside the stated new-order scope, and the 19:30 Reference Price is a fixing instant rather than an order-capable window. No discrepancy.

## Revision rows

- 2021-10-05 — T1 — Blue Ocean launch announcement 2021-10-05 — the production ATS new-order service opens on the 20:00–04:00 ET Sunday-through-Thursday window.

## Sources

Row review: 2026-08-22 (UTC) is the date the ledger row was last reviewed as a
whole. Per-source retrieval dates were not recorded before the 2026-09-12
migration; where a later targeted review, capture or document date is recorded
beside a source below, that date governs for that source, and dates are added
as each source is re-verified.

- <https://www.sec.gov/Archives/edgar/data/1795131/000090266426001359/xslATS-N_X01/primary_doc.xml> — the live Blue Ocean Form ATS-N, which ends new-order acceptance at 04:00.
- <https://www.sec.gov/about/divisions-offices/division-trading-markets/alternative-trading-systems/form-ats-n-filings-information> — the Form ATS-N filings index.
- <https://blueocean-tech.io/2021/10/05/announcing-launch-of-blue-ocean-ats-afterhours-trading/> — the operator's launch announcement of 2021-10-05.
- <https://blueocean-tech.io/timeline/> — the operator's timeline, which identifies only the month of the earlier beta.
- <https://www.sec.gov/Archives/edgar/data/1795131/000153949721000764/primary_doc.xml> — the 2021 live ATS-N filing.
- <https://www.sec.gov/Archives/edgar/data/1795131/000153949723000091/primary_doc.xml> — the 2023 live ATS-N filing.

## Gaps and residual risks

- **None below the launch.** The profile is `CLOSED` before the operator-dated
  launch, so nothing is carried and the horizon is `—`.
- **Scope exclusions, recorded rather than assumed.** The June 2021 beta has no
  day-level start and is outside this production-service identity, so it is not
  backfilled as trading. The live ATS-N's sub-minute resting-book cleanup after
  04:00 is outside the stated new-order window scope and creates no schedule
  revision. The 18:15 test-order window is testing, and the 19:30 Reference Price
  — the last SIP print at or before 19:30 ET, which fixes each symbol's ±20%
  band — is a price determination, not an order-capable window
  (LAW-SESSION-NOT-EXPIRY).
- **Friday night.** Friday is excluded from the rule because the reporting
  facility is unavailable on Saturday.
- **System coverage (2026-09-02).** No discrepancy. One matching system.

> Shared module. The narrative for
> [`ats.rs`](../../src/calendar/schedules/equities/us/ats.rs)
> lives in [`iex`](iex.md#module-narrative-moved-from-srccalendarschedulesequitiesusatsrs-on-2026-09-12-utc).
> Sibling identities: [`iex`](iex.md).
