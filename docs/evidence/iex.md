<!-- SPDX-License-Identifier: MIT-0 -->

# `iex` — evidence

- **Kind:** `Exchange`
- **Owner module:** [`ats.rs`](../../src/calendar/schedules/equities/us/ats.rs)
- **Source sets:** [`US-IEX`](../schedules/sources.md#us-iex)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

The exchange identity is closed before its 2016-08-19 first production-symbol launch; the symbol phase-in ended and predecessor ATS ceased on 2016-09-02. Current 08:00–17:00 System Hours are primary-sourced. **Systems in scope (2026-09-02):** the equities matching system (08:00–17:00 System Hours) is the envelope and the predecessor IEX ATS is out of scope by identity. IEX Options is a facility of the same SRO, Investors Exchange LLC, and is modeled nowhere; it is on the discrepancy list, with no unconditional launch day in the reviewed material.

## Revision rows

- 2016-08-19 — T1 — IEX Trading Alert 2016-042 — the exchange launches with its first production symbols on 08:00–17:00 ET System Hours.

## Sources

No per-URL retrieval date is recorded in this repository; every link below
was read at or before the row's `Reviewed on` date in the ledger.

- <https://www.iex.io/resources/trading/trading-hours-holidays> — IEX trading hours and holidays, the current 08:00–17:00 System Hours.
- <https://www.iex.io/resources/regulation> — the current IEX regulation and rule-book hub.
- <https://www.sec.gov/files/rules/sro/iex/2016/34-78447.pdf> — the SEC's initial exchange-hours correction, which defines System Hours around the 09:30–16:00 regular session.
- <https://iextrading.com/trading/alerts/2016/042/> — Trading Alert 2016-042, the 2016-08-19 first production-symbol launch and the symbol-by-symbol phase-in.
- <https://iextrading.com/trading/alerts/2016/049/> — Trading Alert 2016-049, the 2016-09-02 all-symbol transition and predecessor-ATS cessation.
- <https://www.federalregister.gov/documents/2026/08/25/2026-17281/> — an IEX Options filing captioned "Self-Regulatory Organizations; Investors Exchange LLC".
- <https://www.federalregister.gov/documents/2026/08/12/2026-16383/> — a 2026-08-12 fee filing describing changes "in Preparation for the Launch of IEX Options Later This Year".
- <https://www.federalregister.gov/documents/2026/09/01/2026-17806/> — the options listing plan amendment adding Investors Exchange LLC as a plan sponsor.
- <https://www.iex.io/options> — the operator's IEX Options page, the watch source for a launch day.

## Gaps and residual risks

- **None below the launch.** The exchange identity is `CLOSED` before the sourced
  2016-08-19 launch, so nothing is carried and the horizon is `—`. The
  predecessor IEX ATS is out of scope by identity and its history is not
  conflated with this row.
- **System coverage (2026-09-02), discrepancy #4.** IEX Options is a facility of
  the same SRO, Investors Exchange LLC — its filings are captioned "Investors
  Exchange LLC" and amend IEX Rules 22.250/22.260 governing "IEX Options
  Members" — and is modelled nowhere. No unconditional launch day is stated in
  the reviewed material, so nothing is encodable, only monitorable
  (LAW-NO-FABRICATED-DATES). Routing: a new identity once an unconditional launch
  day is sourced; until then, an explicit scope exclusion on this row plus a
  watch-list entry.

## Module narrative (moved from src/calendar/schedules/equities/us/ats.rs on 2026-09-12 UTC)

Investors Exchange's initial exchange rules and living hours table define
System Hours as 08:00–17:00 ET around the 09:30–16:00 regular session.
Trading Alert 2016-042 dates the exchange launch to 2016-08-19 and records
the symbol-by-symbol phase-in through 2016-09-02, when the predecessor ATS
ceased. The stable `iex` exchange identity is therefore closed before the
first non-test securities transitioned; ATS history is not conflated with it.
Both System Hours wings are executable: the exchange rules run the same
continuous order book in the pre-market and post-market sessions as in the
regular one, so trades print throughout and neither wing is order entry.
https://www.sec.gov/files/rules/sro/iex/2016/34-78447.pdf
https://www.iex.io/resources/trading/trading-hours-holidays
https://iextrading.com/trading/alerts/2016/042/

Blue Ocean ATS 20:00→04:00 ET new-order trading window, Sunday through Thursday.

The window matches continuously — Blue Ocean's overnight book crosses and
prints throughout it, so it is a tradeable extended session, not order entry.

The live SEC Form ATS-N ends new-order acceptance at 04:00. It permits
resting-book clearing matches for less than a further minute, but that
bounded cleanup is outside this new-order session scope. Friday night is
excluded because the reporting facility is unavailable on Saturday.
<https://www.sec.gov/Archives/edgar/data/1795131/000090266426001359/xslATS-N_X01/primary_doc.xml>

The `blue_ocean_ats` profile is scoped to Blue Ocean's production ATS
service, whose official launch was 2021-10-05. The operator describes an
earlier June 2021 beta without a day-level start; testing/beta activity is
outside this production-service identity and is not backfilled as trading.
The launch-era, 2023, and live ATS-N filings all end new-order acceptance at
04:00. The live filing's sub-minute resting-book cleanup is excluded from the
stated new-order trading-window scope, so it creates no schedule revision.
https://blueocean-tech.io/2021/10/05/announcing-launch-of-blue-ocean-ats-afterhours-trading/
https://blueocean-tech.io/timeline/
https://www.sec.gov/Archives/edgar/data/1795131/000153949721000764/primary_doc.xml
https://www.sec.gov/Archives/edgar/data/1795131/000153949723000091/primary_doc.xml
