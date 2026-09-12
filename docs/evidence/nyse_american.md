<!-- SPDX-License-Identifier: MIT-0 -->

# `nyse_american` — evidence

- **Kind:** `Exchange`
- **Owner module:** [`nyse.rs`](../../src/calendar/schedules/equities/us/nyse.rs)
- **Source sets:** [`US-NYSE-EQUITIES`](../schedules/sources.md#us-nyse-equities)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

**Gap: executable** — reclassified 2026-09-02, having been recorded as order-entry. The acceptance edge is rulebook text, not an operator system setting: NYSE American Rule 7.34E(a)(1) (82 FR 10814) begins acceptance 30 minutes before the 07:00 Early Trading Session, and the Commission records the production day plainly — "NYSE American's cash equities market transitioned to Pillar on July 24, 2017" (83 FR 13553) — which is the dated revision the crate already carries. What remains is the January-2010 off-hours crossing topology, and it is executable rather than order-entry: at the floor NYSE Amex participated in the NYSE Off-Hours Trading Facility crossing sessions, whose CS I leg was eliminated in 2009 and whose CS II leg ran until 18:30 on 2024-01-31 (89 FR 14909). The crate models no post-16:00 phase here, so it errs toward closed; the exact floor-era phase table and its amendment chain are still unestablished. **Systems in scope (2026-09-02):** the American equities Pillar system (06:30–20:00) is the envelope and NYSE American Options is `nyse_american_options`; the Off-Hours Trading Facility under Rule 7.39E, decommissioned 2022-09-01, is in neither place and is on the discrepancy list.

## Revision rows

- 2017-07-24 — T1 — NYSE American Pillar update 2017-07-21 — the cash-equity market transitions to Pillar, adding the 06:30–07:00 acceptance queue around the 07:00–20:00 execution grid.

## Sources

No per-URL retrieval date is recorded in this repository; every link below
was read at or before the row's `Reviewed on` date in the ledger.

- <https://www.nyse.com/trade/hours-calendars?os=.> — NYSE American hours and calendars, the current 06:30–20:00 envelope.
- <https://www.federalregister.gov/documents/2017/02/15/2017-02990/self-regulatory-organizations-nyse-mkt-llc-notice-of-filing-of-proposed-rule-change-to-adopt-new> — SR-NYSEMKT-2017-01 (82 FR 10814), adopting Rule 7.34E(a)(1).
- <https://www.federalregister.gov/documents/2018/03/29/2018-06339/self-regulatory-organizations-new-york-stock-exchange-llc-notice-of-filing-of-amendment-no-1-and> — 83 FR 13553: "NYSE American's cash equities market transitioned to Pillar on July 24, 2017."
- <https://www.nyse.com/publicdocs/nyse/markets/nyse-american/Pillar_Update_NYSE_American_March_2017.pdf> — the operator's March-2017 Pillar update.
- <https://www.nyse.com/publicdocs/nyse/markets/nyse-american/Pillar_Update_NYSE_American_Weekend_Test_Update_July21_2017.pdf> — the 2017-07-21 weekend-test update naming the production day.
- <https://www.sec.gov/rules/sro/nyseamex/2010/34-61890.pdf> — a 2010 NYSE Amex filing from the off-hours crossing era.
- <https://www.federalregister.gov/documents/2024/02/29/2024-04168/self-regulatory-organizations-new-york-stock-exchange-llc-notice-of-filing-and-immediate> — 89 FR 14909, which records Crossing Session II running until 18:30 on 2024-01-31.
- <https://www.federalregister.gov/documents/full_text/text/2022/08/18/2022-17750.txt> — SR-NYSEAMER-2022-35 (87 FR 50894), deleting Rule 7.39E and designating 2022-09-01 as the decommissioning day.

## Gaps and residual risks

- **executable** — at the January-2010 floor NYSE Amex participated in the NYSE
  Off-Hours Trading Facility crossing sessions. Crossing Session I was eliminated
  in 2009, below the floor; Crossing Session II ran until 18:30 on 2024-01-31.
  The crate models no post-16:00 phase here, so it errs toward closed. The exact
  floor-era phase table and its complete amendment chain have not been
  established, and widening the executable envelope from two endpoints is exactly
  the inference this crate refuses. Closing condition: the floor-era phase table
  plus the amendment chain, and the same scope decision the `nyse` row needs.
  Dormant identity, so it is recorded here rather than opened as an issue
  (LAW-FOLLOW-UPS-ARE-ISSUES).
- **Order-entry half closed.** Rule 7.34E(a)(1) is rulebook text, not an
  operator system setting, and the Commission records the production day plainly.
- **Horizon carried below the first dated row.** The baseline below 2017-07-24 is the 09:30–16:00
  core session, carried rather than sourced at a named day; no reviewed artifact
  in this row's material states it on a floor-era date, so the ledger horizon is
  2017-07-24, the first day at which this row's state is sourced. Closing
  condition: a floor-era NYSE Amex rulebook edition or hours publication that
  states the continuous session, which would move the horizon down to the
  January-2010 floor.
- **System coverage (2026-09-02), discrepancy #3.** The Off-Hours Trading
  Facility under Rule 7.39E, decommissioned 2022-09-01, is in neither the
  envelope nor a modelled identity, and needs the same decision as the `nyse`
  entry on its pre-2022-09-01 history. NYSE American Options is
  `nyse_american_options`.

> Shared module. The narrative for
> [`nyse.rs`](../../src/calendar/schedules/equities/us/nyse.rs)
> lives in [`nyse`](nyse.md#module-narrative-moved-from-srccalendarschedulesequitiesusnysers-on-2026-09-12-utc).
> Sibling identities: [`nyse_arca`](nyse_arca.md), [`nyse_national`](nyse_national.md), [`nyse_texas`](nyse_texas.md).
