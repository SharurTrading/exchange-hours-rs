<!-- SPDX-License-Identifier: MIT-0 -->

# `unknown` — evidence

- **Kind:** `Exchange`
- **Owner module:** [`futures_profile.rs`](../../src/calendar/futures_profile.rs)
- **Source sets:** [`SYNTHETIC-24X7`](../schedules/sources.md#synthetic-24x7)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

Deliberate UTC 24×7 fallback; not a venue.

## Revision rows

None. This identity resolves to the single `ALWAYS_OPEN_PROFILE` static in `schedules`, which has no dated revision row.

## Sources

No per-URL retrieval date is recorded in this repository; every link below
was read at or before the row's `Reviewed on` date in the ledger.

- None. `unknown` is deterministic library policy, not a venue schedule, and the
  `SYNTHETIC-24X7` source set records no operator document for it.

## Gaps and residual risks

- **None.** There is nothing to source and nothing to date. The profile is
  continuous, has no final daily close, and therefore returns no trade date; it
  is the deliberate fallback a caller reaches when no venue identity applies.
  Do not treat it as evidence for any real venue.
