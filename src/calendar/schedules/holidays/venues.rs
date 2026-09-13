// SPDX-License-Identifier: MIT-0

//! CME, CBOT, COMEX and NYMEX venue holiday rows, 2025-2027.
//!
//! `Exchange::Cme`, `Cbot`, `Comex` and `Nymex` each route several product
//! families onto one schedule, and a holiday row is a statement about **one**
//! clock. A venue row may therefore be stated only where the families that
//! route to the venue say the same thing: this module is the **intersection**
//! of those families' tables (design memo D17), not a union and not a
//! compromise between them.
//!
//! Every row here is **derived, not retrieved**. The operator statements behind
//! them are the `CME-SVC-*` documents the family tables already cite — CME
//! Group's own `trading-hours-by-product` service at tier T2 — and nothing in
//! this module rests on an artifact a family module does not already carry.
//! There is accordingly no venue evidence of its own to add: the four venue
//! evidence files record the derivation, the routing, the intersection rule and
//! every date the intersection drops.
//!
//! # The routing
//!
//! | Venue | Families | Where that routing is decided |
//! |---|---|---|
//! | `cme` | `globex_equity_index`, `globex_energy`, `globex_fx`, `globex_grains`, `globex_interest_rates`, `globex_livestock` | every key resolving to a CME-owned profile |
//! | `cbot` | `globex_grains`, `globex_interest_rates` | `hours_for_exchange`'s `Cbot` arm |
//! | `comex` | `globex_energy`, metals half | `hours_for_exchange`'s `Comex` arm |
//! | `nymex` | `globex_energy`, energy half | `hours_for_exchange`'s `Nymex` arm |
//!
//! COMEX and NYMEX share one family key, because CME prints the two halves as
//! one product row on every date in this window: no date is dropped for a
//! disagreement between `GC` and `CL`, and the two venues' tables are identical
//! by construction rather than by copying.
//!
//! # Two kinds of row
//!
//! **Where every routed family states the same row**, the venue ships that row.
//! In 2025-2027 that is the nine Globex full closures: all six CME families,
//! both CBOT families, or the single energy family behind COMEX/NYMEX agree to
//! the status, and the venue is closed.
//!
//! **Where they disagree**, the venue ships [`HolidayKind::Unsourced`]. That is
//! neither silence nor a compromise instant. The coverage window is contiguous,
//! so a date carrying no row is the positive claim that it was audited normal,
//! which on these dates is false; and any single instant would be wrong for
//! someone, because a venue row that copied the shallowest early close would
//! delete the deepest family's sourced trading while one that copied the
//! deepest would delete the shallowest family's post-close phase. `Unsourced`
//! clips nothing, answers nothing, and tells the caller what the crate knows:
//! the date is special and the venue has no one answer for it. ICE Futures
//! U.S.'s venue table, which shipped first, is the precedent, and the
//! disagreement behind each row is named in the venue's evidence file.
//!
//! Evidence: [`docs/evidence/cme.md`](../../../../../docs/evidence/cme.md),
//! [`cbot.md`](../../../../../docs/evidence/cbot.md),
//! [`comex.md`](../../../../../docs/evidence/comex.md),
//! [`nymex.md`](../../../../../docs/evidence/nymex.md).

use super::fences::early_close;
use super::{
    EvidenceTier::T2, HolidayKind::Closed, HolidayKind::Unsourced, HolidayTable, holidays,
};

/// The `Exchange::Cme` table: the intersection of the six CME families.
///
/// Nine rows state a status — the Globex full closures — and the other
/// thirty-two are `Unsourced`. Coverage runs to 2027-12-31, the end of the
/// families' own audited window; 2028-01-01 sits outside it in every family
/// table and ships no row here either.
// Evidence: docs/evidence/cme.md
pub(crate) static CME: &HolidayTable = holidays! {
    coverage: (2025, 1, 1) ..= (2027, 12, 31),
    rows: [
        // 2025-01-01 - T2 - CME-SVC-2024-12-31 - all six families closed.
        (2025, 1, 1, Closed, T2, "CME-SVC-2024-12-31"),
        // 2025-01-02 - T2 - CME-SVC-2024-12-31 - grains late open 08:30 CT;
        // no row in equity index, energy, FX, interest rates or livestock.
        (2025, 1, 2, Unsourced, T2, "CME-SVC-2024-12-31"),
        // 2025-01-20 - T2 - CME-SVC-2025-01-19 - equity index and interest
        // rates early close 12:00 CT, energy 13:30 CT, grains and livestock
        // closed, no row in FX.
        (2025, 1, 20, Unsourced, T2, "CME-SVC-2025-01-19"),
        // 2025-02-17 - T2 - CME-SVC-2025-02-16 - as 2025-01-20.
        (2025, 2, 17, Unsourced, T2, "CME-SVC-2025-02-16"),
        // 2025-04-18 - T2 - CME-SVC-2025-04-17 - all six families closed.
        (2025, 4, 18, Closed, T2, "CME-SVC-2025-04-17"),
        // 2025-05-26 - T2 - CME-SVC-2025-05-25 - as 2025-01-20.
        (2025, 5, 26, Unsourced, T2, "CME-SVC-2025-05-25"),
        // 2025-06-19 - T2 - CME-SVC-2025-06-18 - as 2025-01-20.
        (2025, 6, 19, Unsourced, T2, "CME-SVC-2025-06-18"),
        // 2025-07-03 - T2 - CME-SVC-2025-07-03 - equity index early close
        // 12:15 CT; no row in any of the other five families.
        (2025, 7, 3, Unsourced, T2, "CME-SVC-2025-07-03"),
        // 2025-07-04 - T2 - CME-SVC-2025-07-03 - equity index, energy, FX and
        // interest rates early close 12:00 CT, grains and livestock closed.
        (2025, 7, 4, Unsourced, T2, "CME-SVC-2025-07-03"),
        // 2025-09-01 - T2 - CME-SVC-2025-08-31 - as 2025-01-20.
        (2025, 9, 1, Unsourced, T2, "CME-SVC-2025-08-31"),
        // 2025-11-27 - T2 - CME-SVC-2025-11-26-SAT - as 2025-01-20.
        (2025, 11, 27, Unsourced, T2, "CME-SVC-2025-11-26-SAT"),
        // 2025-11-28 - T2 - CME-SVC-2025-11-26 - equity index, interest rates
        // and livestock early close 12:05-12:15 CT, energy and FX 13:45 CT,
        // grains a 08:30 CT late open into a 12:05 CT close.
        (2025, 11, 28, Unsourced, T2, "CME-SVC-2025-11-26"),
        // 2025-11-29 - T2 - CME-SVC-2025-11-26-SAT - all six families closed.
        (2025, 11, 29, Closed, T2, "CME-SVC-2025-11-26-SAT"),
        // 2025-12-24 - T2 - CME-SVC-2025-12-24 - early closes at 12:05 CT for
        // grains, 12:15 CT for equity index, interest rates and livestock, and
        // 12:45 CT for energy and FX.
        (2025, 12, 24, Unsourced, T2, "CME-SVC-2025-12-24"),
        // 2025-12-25 - T2 - CME-SVC-2025-12-24 - all six families closed.
        (2025, 12, 25, Closed, T2, "CME-SVC-2025-12-24"),
        // 2025-12-26 - T2 - CME-SVC-2025-12-24 - grains late open 08:30 CT;
        // no row in any of the other five families.
        (2025, 12, 26, Unsourced, T2, "CME-SVC-2025-12-24"),
        // 2026-01-01 - T2 - CME-SVC-2025-12-31 - all six families closed.
        (2026, 1, 1, Closed, T2, "CME-SVC-2025-12-31"),
        // 2026-01-02 - T2 - CME-SVC-2025-12-31 - grains late open 08:30 CT;
        // no row in any of the other five families.
        (2026, 1, 2, Unsourced, T2, "CME-SVC-2025-12-31"),
        // 2026-01-19 - T2 - CME-SVC-2026-01-18 - as 2025-01-20.
        (2026, 1, 19, Unsourced, T2, "CME-SVC-2026-01-18"),
        // 2026-02-16 - T2 - CME-SVC-2026-02-15 - as 2025-01-20.
        (2026, 2, 16, Unsourced, T2, "CME-SVC-2026-02-15"),
        // 2026-04-03 - T2 - CME-SVC-2026-04-01 - equity index early close
        // 08:15 CT, FX and interest rates 10:15 CT, energy, grains and
        // livestock closed.
        (2026, 4, 3, Unsourced, T2, "CME-SVC-2026-04-01"),
        // 2026-05-25 - T2 - CME-SVC-2026-05-24 - as 2025-01-20.
        (2026, 5, 25, Unsourced, T2, "CME-SVC-2026-05-24"),
        // 2026-06-19 - T2 - CME-SVC-2026-06-18 - equity index, energy, FX and
        // interest rates early close 12:00 CT, grains and livestock closed.
        (2026, 6, 19, Unsourced, T2, "CME-SVC-2026-06-18"),
        // 2026-07-03 - T2 - CME-SVC-2026-07-03 - as 2026-06-19.
        (2026, 7, 3, Unsourced, T2, "CME-SVC-2026-07-03"),
        // 2026-09-07 - T2 - CME-SVC-2026-09-06 - as 2025-01-20.
        (2026, 9, 7, Unsourced, T2, "CME-SVC-2026-09-06"),
        // 2026-11-26 - T2 - CME-SVC-2026-11-25 - as 2025-01-20.
        (2026, 11, 26, Unsourced, T2, "CME-SVC-2026-11-25"),
        // 2026-11-27 - T2 - CME-SVC-2026-11-25 - as 2025-11-28.
        (2026, 11, 27, Unsourced, T2, "CME-SVC-2026-11-25"),
        // 2026-12-24 - T2 - CME-SVC-2026-12-22 - as 2025-12-24, except that
        // livestock closes at 12:05 CT with grains.
        (2026, 12, 24, Unsourced, T2, "CME-SVC-2026-12-22"),
        // 2026-12-25 - T2 - CME-SVC-2026-12-24 - all six families closed.
        (2026, 12, 25, Closed, T2, "CME-SVC-2026-12-24"),
        // 2027-01-01 - T2 - CME-SVC-2026-12-31 - all six families closed.
        (2027, 1, 1, Closed, T2, "CME-SVC-2026-12-31"),
        // 2027-01-18 - T2 - CME-SVC-2027-01-17 - as 2025-01-20.
        (2027, 1, 18, Unsourced, T2, "CME-SVC-2027-01-17"),
        // 2027-02-15 - T2 - CME-SVC-2027-02-14 - as 2025-01-20.
        (2027, 2, 15, Unsourced, T2, "CME-SVC-2027-02-14"),
        // 2027-03-26 - T2 - CME-SVC-2027-03-25 - all six families closed.
        (2027, 3, 26, Closed, T2, "CME-SVC-2027-03-25"),
        // 2027-05-31 - T2 - CME-SVC-2027-05-30 - as 2025-01-20.
        (2027, 5, 31, Unsourced, T2, "CME-SVC-2027-05-30"),
        // 2027-06-18 - T2 - CME-SVC-2027-06-17 - as 2026-06-19.
        (2027, 6, 18, Unsourced, T2, "CME-SVC-2027-06-17"),
        // 2027-07-05 - T2 - CME-SVC-2027-07-04 - equity index 12:00 CT,
        // energy and interest rates 13:30 CT early close, grains and livestock
        // closed, no row in FX.
        (2027, 7, 5, Unsourced, T2, "CME-SVC-2027-07-04"),
        // 2027-07-06 - T2 - CME-SVC-2027-07-04 - grains late open 08:30 CT;
        // no row in any of the other five families.
        (2027, 7, 6, Unsourced, T2, "CME-SVC-2027-07-04"),
        // 2027-09-06 - T2 - CME-SVC-2027-09-05 - as 2025-01-20.
        (2027, 9, 6, Unsourced, T2, "CME-SVC-2027-09-05"),
        // 2027-11-25 - T2 - CME-SVC-2027-11-24 - as 2025-01-20.
        (2027, 11, 25, Unsourced, T2, "CME-SVC-2027-11-24"),
        // 2027-11-26 - T2 - CME-SVC-2027-11-24 - as 2025-11-28.
        (2027, 11, 26, Unsourced, T2, "CME-SVC-2027-11-24"),
        // 2027-12-24 - T2 - CME-SVC-2027-12-22 - all six families closed.
        (2027, 12, 24, Closed, T2, "CME-SVC-2027-12-22"),
    ],
};

/// The `Exchange::Cbot` table: `globex_grains` ∩ `globex_interest_rates`.
///
/// The two families trade the same building around different sessions, and the
/// day session is where they touch: every one of the nine full closures below
/// keeps both closed, while every CBOT holiday early close moves the two by a
/// different amount — the grain day session ends at 12:05 CT while the rates
/// overnight leg halts at 12:00 CT — so those dates ship `Unsourced`.
// Evidence: docs/evidence/cbot.md
pub(crate) static CBOT: &HolidayTable = holidays! {
    coverage: (2025, 1, 1) ..= (2027, 12, 31),
    rows: [
        // 2025-01-01 - T2 - CME-SVC-2024-12-31 - both families closed.
        (2025, 1, 1, Closed, T2, "CME-SVC-2024-12-31"),
        // 2025-01-02 - T2 - CME-SVC-2024-12-31 - grains late open 08:30 CT;
        // no row in interest rates.
        (2025, 1, 2, Unsourced, T2, "CME-SVC-2024-12-31"),
        // 2025-01-20 - T2 - CME-SVC-2025-01-19 - grains closed, interest rates
        // early close 12:00 CT.
        (2025, 1, 20, Unsourced, T2, "CME-SVC-2025-01-19"),
        // 2025-02-17 - T2 - CME-SVC-2025-02-16 - as 2025-01-20.
        (2025, 2, 17, Unsourced, T2, "CME-SVC-2025-02-16"),
        // 2025-04-18 - T2 - CME-SVC-2025-04-17 - both families closed.
        (2025, 4, 18, Closed, T2, "CME-SVC-2025-04-17"),
        // 2025-05-26 - T2 - CME-SVC-2025-05-25 - as 2025-01-20.
        (2025, 5, 26, Unsourced, T2, "CME-SVC-2025-05-25"),
        // 2025-06-19 - T2 - CME-SVC-2025-06-18 - as 2025-01-20.
        (2025, 6, 19, Unsourced, T2, "CME-SVC-2025-06-18"),
        // 2025-07-04 - T2 - CME-SVC-2025-07-03 - as 2025-01-20.
        (2025, 7, 4, Unsourced, T2, "CME-SVC-2025-07-03"),
        // 2025-09-01 - T2 - CME-SVC-2025-08-31 - as 2025-01-20.
        (2025, 9, 1, Unsourced, T2, "CME-SVC-2025-08-31"),
        // 2025-11-27 - T2 - CME-SVC-2025-11-26-SAT - as 2025-01-20.
        (2025, 11, 27, Unsourced, T2, "CME-SVC-2025-11-26-SAT"),
        // 2025-11-28 - T2 - CME-SVC-2025-11-26 - grains 08:30 CT late open
        // into a 12:05 CT close, interest rates 12:15 CT early close.
        (2025, 11, 28, Unsourced, T2, "CME-SVC-2025-11-26"),
        // 2025-11-29 - T2 - CME-SVC-2025-11-26-SAT - both families closed.
        (2025, 11, 29, Closed, T2, "CME-SVC-2025-11-26-SAT"),
        // 2025-12-24 - T2 - CME-SVC-2025-12-24 - grains 12:05 CT, interest
        // rates 12:15 CT.
        (2025, 12, 24, Unsourced, T2, "CME-SVC-2025-12-24"),
        // 2025-12-25 - T2 - CME-SVC-2025-12-24 - both families closed.
        (2025, 12, 25, Closed, T2, "CME-SVC-2025-12-24"),
        // 2025-12-26 - T2 - CME-SVC-2025-12-24 - grains late open 08:30 CT;
        // no row in interest rates.
        (2025, 12, 26, Unsourced, T2, "CME-SVC-2025-12-24"),
        // 2026-01-01 - T2 - CME-SVC-2025-12-31 - both families closed.
        (2026, 1, 1, Closed, T2, "CME-SVC-2025-12-31"),
        // 2026-01-02 - T2 - CME-SVC-2025-12-31 - grains late open 08:30 CT;
        // no row in interest rates.
        (2026, 1, 2, Unsourced, T2, "CME-SVC-2025-12-31"),
        // 2026-01-19 - T2 - CME-SVC-2026-01-18 - as 2025-01-20.
        (2026, 1, 19, Unsourced, T2, "CME-SVC-2026-01-18"),
        // 2026-02-16 - T2 - CME-SVC-2026-02-15 - as 2025-01-20.
        (2026, 2, 16, Unsourced, T2, "CME-SVC-2026-02-15"),
        // 2026-04-03 - T2 - CME-SVC-2026-04-01 - grains closed, interest rates
        // early close 10:15 CT.
        (2026, 4, 3, Unsourced, T2, "CME-SVC-2026-04-01"),
        // 2026-05-25 - T2 - CME-SVC-2026-05-24 - as 2025-01-20.
        (2026, 5, 25, Unsourced, T2, "CME-SVC-2026-05-24"),
        // 2026-06-19 - T2 - CME-SVC-2026-06-18 - as 2025-01-20.
        (2026, 6, 19, Unsourced, T2, "CME-SVC-2026-06-18"),
        // 2026-07-03 - T2 - CME-SVC-2026-07-03 - as 2025-01-20.
        (2026, 7, 3, Unsourced, T2, "CME-SVC-2026-07-03"),
        // 2026-09-07 - T2 - CME-SVC-2026-09-06 - as 2025-01-20.
        (2026, 9, 7, Unsourced, T2, "CME-SVC-2026-09-06"),
        // 2026-11-26 - T2 - CME-SVC-2026-11-25 - as 2025-01-20.
        (2026, 11, 26, Unsourced, T2, "CME-SVC-2026-11-25"),
        // 2026-11-27 - T2 - CME-SVC-2026-11-25 - as 2025-11-28.
        (2026, 11, 27, Unsourced, T2, "CME-SVC-2026-11-25"),
        // 2026-12-24 - T2 - CME-SVC-2026-12-22 - grains 12:05 CT, interest
        // rates 12:15 CT.
        (2026, 12, 24, Unsourced, T2, "CME-SVC-2026-12-22"),
        // 2026-12-25 - T2 - CME-SVC-2026-12-24 - both families closed.
        (2026, 12, 25, Closed, T2, "CME-SVC-2026-12-24"),
        // 2027-01-01 - T2 - CME-SVC-2026-12-31 - both families closed.
        (2027, 1, 1, Closed, T2, "CME-SVC-2026-12-31"),
        // 2027-01-18 - T2 - CME-SVC-2027-01-17 - as 2025-01-20.
        (2027, 1, 18, Unsourced, T2, "CME-SVC-2027-01-17"),
        // 2027-02-15 - T2 - CME-SVC-2027-02-14 - as 2025-01-20.
        (2027, 2, 15, Unsourced, T2, "CME-SVC-2027-02-14"),
        // 2027-03-26 - T2 - CME-SVC-2027-03-25 - both families closed.
        (2027, 3, 26, Closed, T2, "CME-SVC-2027-03-25"),
        // 2027-05-31 - T2 - CME-SVC-2027-05-30 - as 2025-01-20.
        (2027, 5, 31, Unsourced, T2, "CME-SVC-2027-05-30"),
        // 2027-06-18 - T2 - CME-SVC-2027-06-17 - as 2025-01-20.
        (2027, 6, 18, Unsourced, T2, "CME-SVC-2027-06-17"),
        // 2027-07-05 - T2 - CME-SVC-2027-07-04 - grains closed, interest rates
        // early close 13:30 CT.
        (2027, 7, 5, Unsourced, T2, "CME-SVC-2027-07-04"),
        // 2027-07-06 - T2 - CME-SVC-2027-07-04 - grains late open 08:30 CT;
        // no row in interest rates.
        (2027, 7, 6, Unsourced, T2, "CME-SVC-2027-07-04"),
        // 2027-09-06 - T2 - CME-SVC-2027-09-05 - as 2025-01-20.
        (2027, 9, 6, Unsourced, T2, "CME-SVC-2027-09-05"),
        // 2027-11-25 - T2 - CME-SVC-2027-11-24 - as 2025-01-20.
        (2027, 11, 25, Unsourced, T2, "CME-SVC-2027-11-24"),
        // 2027-11-26 - T2 - CME-SVC-2027-11-24 - as 2025-11-28.
        (2027, 11, 26, Unsourced, T2, "CME-SVC-2027-11-24"),
        // 2027-12-24 - T2 - CME-SVC-2027-12-22 - both families closed.
        (2027, 12, 24, Closed, T2, "CME-SVC-2027-12-22"),
    ],
};

/// The `Exchange::Comex` table: the COMEX metals half of `globex_energy`.
///
/// Metals and energy are one key and the operator prints them as one product
/// row on every date in this window, so the intersection is total: the venue
/// carries the family's thirty-six rows unchanged, and no date is withheld.
// Evidence: docs/evidence/comex.md
pub(crate) static COMEX: &HolidayTable = holidays! {
    coverage: (2025, 1, 1) ..= (2027, 12, 31),
    rows: [
        // 2025-01-01 - T2 - CME-SVC-2024-12-31 - metals closed.
        (2025, 1, 1, Closed, T2, "CME-SVC-2024-12-31"),
        // 2025-01-20 - T2 - CME-SVC-2025-01-19 - metals early close 13:30 CT.
        (2025, 1, 20, early_close(13 * 3_600 + 30 * 60), T2, "CME-SVC-2025-01-19"),
        // 2025-02-17 - T2 - CME-SVC-2025-02-16 - metals early close 13:30 CT.
        (2025, 2, 17, early_close(13 * 3_600 + 30 * 60), T2, "CME-SVC-2025-02-16"),
        // 2025-04-18 - T2 - CME-SVC-2025-04-17 - metals closed.
        (2025, 4, 18, Closed, T2, "CME-SVC-2025-04-17"),
        // 2025-05-26 - T2 - CME-SVC-2025-05-25 - metals early close 13:30 CT.
        (2025, 5, 26, early_close(13 * 3_600 + 30 * 60), T2, "CME-SVC-2025-05-25"),
        // 2025-06-19 - T2 - CME-SVC-2025-06-18 - metals early close 13:30 CT.
        (2025, 6, 19, early_close(13 * 3_600 + 30 * 60), T2, "CME-SVC-2025-06-18"),
        // 2025-07-04 - T2 - CME-SVC-2025-07-03 - metals early close 12:00 CT.
        (2025, 7, 4, early_close(12 * 3_600), T2, "CME-SVC-2025-07-03"),
        // 2025-09-01 - T2 - CME-SVC-2025-08-31 - metals early close 13:30 CT.
        (2025, 9, 1, early_close(13 * 3_600 + 30 * 60), T2, "CME-SVC-2025-08-31"),
        // 2025-11-27 - T2 - CME-SVC-2025-11-26 - metals early close 13:30 CT.
        (2025, 11, 27, early_close(13 * 3_600 + 30 * 60), T2, "CME-SVC-2025-11-26"),
        // 2025-11-28 - T2 - CME-SVC-2025-11-26 - metals early close 13:45 CT.
        (2025, 11, 28, early_close(13 * 3_600 + 45 * 60), T2, "CME-SVC-2025-11-26"),
        // 2025-11-29 - T2 - CME-SVC-2025-11-26-SAT - metals closed.
        (2025, 11, 29, Closed, T2, "CME-SVC-2025-11-26-SAT"),
        // 2025-12-24 - T2 - CME-SVC-2025-12-24 - metals early close 12:45 CT.
        (2025, 12, 24, early_close(12 * 3_600 + 45 * 60), T2, "CME-SVC-2025-12-24"),
        // 2025-12-25 - T2 - CME-SVC-2025-12-24 - metals closed.
        (2025, 12, 25, Closed, T2, "CME-SVC-2025-12-24"),
        // 2026-01-01 - T2 - CME-SVC-2025-12-31 - metals closed.
        (2026, 1, 1, Closed, T2, "CME-SVC-2025-12-31"),
        // 2026-01-19 - T2 - CME-SVC-2026-01-18 - metals early close 13:30 CT.
        (2026, 1, 19, early_close(13 * 3_600 + 30 * 60), T2, "CME-SVC-2026-01-18"),
        // 2026-02-16 - T2 - CME-SVC-2026-02-15 - metals early close 13:30 CT.
        (2026, 2, 16, early_close(13 * 3_600 + 30 * 60), T2, "CME-SVC-2026-02-15"),
        // 2026-04-03 - T2 - CME-SVC-2026-04-01 - metals closed.
        (2026, 4, 3, Closed, T2, "CME-SVC-2026-04-01"),
        // 2026-05-25 - T2 - CME-SVC-2026-05-24 - metals early close 13:30 CT.
        (2026, 5, 25, early_close(13 * 3_600 + 30 * 60), T2, "CME-SVC-2026-05-24"),
        // 2026-06-19 - T2 - CME-SVC-2026-06-18 - metals early close 12:00 CT.
        (2026, 6, 19, early_close(12 * 3_600), T2, "CME-SVC-2026-06-18"),
        // 2026-07-03 - T2 - CME-SVC-2026-07-03 - metals early close 12:00 CT.
        (2026, 7, 3, early_close(12 * 3_600), T2, "CME-SVC-2026-07-03"),
        // 2026-09-07 - T2 - CME-SVC-2026-09-06 - metals early close 13:30 CT.
        (2026, 9, 7, early_close(13 * 3_600 + 30 * 60), T2, "CME-SVC-2026-09-06"),
        // 2026-11-26 - T2 - CME-SVC-2026-11-25 - metals early close 13:30 CT.
        (2026, 11, 26, early_close(13 * 3_600 + 30 * 60), T2, "CME-SVC-2026-11-25"),
        // 2026-11-27 - T2 - CME-SVC-2026-11-25 - metals early close 13:45 CT.
        (2026, 11, 27, early_close(13 * 3_600 + 45 * 60), T2, "CME-SVC-2026-11-25"),
        // 2026-12-24 - T2 - CME-SVC-2026-12-22 - metals early close 12:45 CT.
        (2026, 12, 24, early_close(12 * 3_600 + 45 * 60), T2, "CME-SVC-2026-12-22"),
        // 2026-12-25 - T2 - CME-SVC-2026-12-24 - metals closed.
        (2026, 12, 25, Closed, T2, "CME-SVC-2026-12-24"),
        // 2027-01-01 - T2 - CME-SVC-2026-12-31 - metals closed.
        (2027, 1, 1, Closed, T2, "CME-SVC-2026-12-31"),
        // 2027-01-18 - T2 - CME-SVC-2027-01-17 - metals early close 13:30 CT.
        (2027, 1, 18, early_close(13 * 3_600 + 30 * 60), T2, "CME-SVC-2027-01-17"),
        // 2027-02-15 - T2 - CME-SVC-2027-02-14 - metals early close 13:30 CT.
        (2027, 2, 15, early_close(13 * 3_600 + 30 * 60), T2, "CME-SVC-2027-02-14"),
        // 2027-03-26 - T2 - CME-SVC-2027-03-25 - metals closed.
        (2027, 3, 26, Closed, T2, "CME-SVC-2027-03-25"),
        // 2027-05-31 - T2 - CME-SVC-2027-05-30 - metals early close 13:30 CT.
        (2027, 5, 31, early_close(13 * 3_600 + 30 * 60), T2, "CME-SVC-2027-05-30"),
        // 2027-06-18 - T2 - CME-SVC-2027-06-17 - metals early close 12:00 CT.
        (2027, 6, 18, early_close(12 * 3_600), T2, "CME-SVC-2027-06-17"),
        // 2027-07-05 - T2 - CME-SVC-2027-07-04 - metals early close 13:30 CT.
        (2027, 7, 5, early_close(13 * 3_600 + 30 * 60), T2, "CME-SVC-2027-07-04"),
        // 2027-09-06 - T2 - CME-SVC-2027-09-05 - metals early close 13:30 CT.
        (2027, 9, 6, early_close(13 * 3_600 + 30 * 60), T2, "CME-SVC-2027-09-05"),
        // 2027-11-25 - T2 - CME-SVC-2027-11-24 - metals early close 13:30 CT.
        (2027, 11, 25, early_close(13 * 3_600 + 30 * 60), T2, "CME-SVC-2027-11-24"),
        // 2027-11-26 - T2 - CME-SVC-2027-11-24 - metals early close 13:45 CT.
        (2027, 11, 26, early_close(13 * 3_600 + 45 * 60), T2, "CME-SVC-2027-11-24"),
        // 2027-12-24 - T2 - CME-SVC-2027-12-22 - metals closed.
        (2027, 12, 24, Closed, T2, "CME-SVC-2027-12-22"),
    ],
};

/// The `Exchange::Nymex` table: the NYMEX energy half of `globex_energy`.
///
/// Byte-identical to [`COMEX`] by construction rather than by copying: the two
/// venues route the same single family, because the operator publishes the
/// metals and energy halves as one row on every date in this window.
// Evidence: docs/evidence/nymex.md
pub(crate) static NYMEX: &HolidayTable = holidays! {
    coverage: (2025, 1, 1) ..= (2027, 12, 31),
    rows: [
        // 2025-01-01 - T2 - CME-SVC-2024-12-31 - energy closed.
        (2025, 1, 1, Closed, T2, "CME-SVC-2024-12-31"),
        // 2025-01-20 - T2 - CME-SVC-2025-01-19 - energy early close 13:30 CT.
        (2025, 1, 20, early_close(13 * 3_600 + 30 * 60), T2, "CME-SVC-2025-01-19"),
        // 2025-02-17 - T2 - CME-SVC-2025-02-16 - energy early close 13:30 CT.
        (2025, 2, 17, early_close(13 * 3_600 + 30 * 60), T2, "CME-SVC-2025-02-16"),
        // 2025-04-18 - T2 - CME-SVC-2025-04-17 - energy closed.
        (2025, 4, 18, Closed, T2, "CME-SVC-2025-04-17"),
        // 2025-05-26 - T2 - CME-SVC-2025-05-25 - energy early close 13:30 CT.
        (2025, 5, 26, early_close(13 * 3_600 + 30 * 60), T2, "CME-SVC-2025-05-25"),
        // 2025-06-19 - T2 - CME-SVC-2025-06-18 - energy early close 13:30 CT.
        (2025, 6, 19, early_close(13 * 3_600 + 30 * 60), T2, "CME-SVC-2025-06-18"),
        // 2025-07-04 - T2 - CME-SVC-2025-07-03 - energy early close 12:00 CT.
        (2025, 7, 4, early_close(12 * 3_600), T2, "CME-SVC-2025-07-03"),
        // 2025-09-01 - T2 - CME-SVC-2025-08-31 - energy early close 13:30 CT.
        (2025, 9, 1, early_close(13 * 3_600 + 30 * 60), T2, "CME-SVC-2025-08-31"),
        // 2025-11-27 - T2 - CME-SVC-2025-11-26 - energy early close 13:30 CT.
        (2025, 11, 27, early_close(13 * 3_600 + 30 * 60), T2, "CME-SVC-2025-11-26"),
        // 2025-11-28 - T2 - CME-SVC-2025-11-26 - energy early close 13:45 CT.
        (2025, 11, 28, early_close(13 * 3_600 + 45 * 60), T2, "CME-SVC-2025-11-26"),
        // 2025-11-29 - T2 - CME-SVC-2025-11-26-SAT - energy closed.
        (2025, 11, 29, Closed, T2, "CME-SVC-2025-11-26-SAT"),
        // 2025-12-24 - T2 - CME-SVC-2025-12-24 - energy early close 12:45 CT.
        (2025, 12, 24, early_close(12 * 3_600 + 45 * 60), T2, "CME-SVC-2025-12-24"),
        // 2025-12-25 - T2 - CME-SVC-2025-12-24 - energy closed.
        (2025, 12, 25, Closed, T2, "CME-SVC-2025-12-24"),
        // 2026-01-01 - T2 - CME-SVC-2025-12-31 - energy closed.
        (2026, 1, 1, Closed, T2, "CME-SVC-2025-12-31"),
        // 2026-01-19 - T2 - CME-SVC-2026-01-18 - energy early close 13:30 CT.
        (2026, 1, 19, early_close(13 * 3_600 + 30 * 60), T2, "CME-SVC-2026-01-18"),
        // 2026-02-16 - T2 - CME-SVC-2026-02-15 - energy early close 13:30 CT.
        (2026, 2, 16, early_close(13 * 3_600 + 30 * 60), T2, "CME-SVC-2026-02-15"),
        // 2026-04-03 - T2 - CME-SVC-2026-04-01 - energy closed.
        (2026, 4, 3, Closed, T2, "CME-SVC-2026-04-01"),
        // 2026-05-25 - T2 - CME-SVC-2026-05-24 - energy early close 13:30 CT.
        (2026, 5, 25, early_close(13 * 3_600 + 30 * 60), T2, "CME-SVC-2026-05-24"),
        // 2026-06-19 - T2 - CME-SVC-2026-06-18 - energy early close 12:00 CT.
        (2026, 6, 19, early_close(12 * 3_600), T2, "CME-SVC-2026-06-18"),
        // 2026-07-03 - T2 - CME-SVC-2026-07-03 - energy early close 12:00 CT.
        (2026, 7, 3, early_close(12 * 3_600), T2, "CME-SVC-2026-07-03"),
        // 2026-09-07 - T2 - CME-SVC-2026-09-06 - energy early close 13:30 CT.
        (2026, 9, 7, early_close(13 * 3_600 + 30 * 60), T2, "CME-SVC-2026-09-06"),
        // 2026-11-26 - T2 - CME-SVC-2026-11-25 - energy early close 13:30 CT.
        (2026, 11, 26, early_close(13 * 3_600 + 30 * 60), T2, "CME-SVC-2026-11-25"),
        // 2026-11-27 - T2 - CME-SVC-2026-11-25 - energy early close 13:45 CT.
        (2026, 11, 27, early_close(13 * 3_600 + 45 * 60), T2, "CME-SVC-2026-11-25"),
        // 2026-12-24 - T2 - CME-SVC-2026-12-22 - energy early close 12:45 CT.
        (2026, 12, 24, early_close(12 * 3_600 + 45 * 60), T2, "CME-SVC-2026-12-22"),
        // 2026-12-25 - T2 - CME-SVC-2026-12-24 - energy closed.
        (2026, 12, 25, Closed, T2, "CME-SVC-2026-12-24"),
        // 2027-01-01 - T2 - CME-SVC-2026-12-31 - energy closed.
        (2027, 1, 1, Closed, T2, "CME-SVC-2026-12-31"),
        // 2027-01-18 - T2 - CME-SVC-2027-01-17 - energy early close 13:30 CT.
        (2027, 1, 18, early_close(13 * 3_600 + 30 * 60), T2, "CME-SVC-2027-01-17"),
        // 2027-02-15 - T2 - CME-SVC-2027-02-14 - energy early close 13:30 CT.
        (2027, 2, 15, early_close(13 * 3_600 + 30 * 60), T2, "CME-SVC-2027-02-14"),
        // 2027-03-26 - T2 - CME-SVC-2027-03-25 - energy closed.
        (2027, 3, 26, Closed, T2, "CME-SVC-2027-03-25"),
        // 2027-05-31 - T2 - CME-SVC-2027-05-30 - energy early close 13:30 CT.
        (2027, 5, 31, early_close(13 * 3_600 + 30 * 60), T2, "CME-SVC-2027-05-30"),
        // 2027-06-18 - T2 - CME-SVC-2027-06-17 - energy early close 12:00 CT.
        (2027, 6, 18, early_close(12 * 3_600), T2, "CME-SVC-2027-06-17"),
        // 2027-07-05 - T2 - CME-SVC-2027-07-04 - energy early close 13:30 CT.
        (2027, 7, 5, early_close(13 * 3_600 + 30 * 60), T2, "CME-SVC-2027-07-04"),
        // 2027-09-06 - T2 - CME-SVC-2027-09-05 - energy early close 13:30 CT.
        (2027, 9, 6, early_close(13 * 3_600 + 30 * 60), T2, "CME-SVC-2027-09-05"),
        // 2027-11-25 - T2 - CME-SVC-2027-11-24 - energy early close 13:30 CT.
        (2027, 11, 25, early_close(13 * 3_600 + 30 * 60), T2, "CME-SVC-2027-11-24"),
        // 2027-11-26 - T2 - CME-SVC-2027-11-24 - energy early close 13:45 CT.
        (2027, 11, 26, early_close(13 * 3_600 + 45 * 60), T2, "CME-SVC-2027-11-24"),
        // 2027-12-24 - T2 - CME-SVC-2027-12-22 - energy closed.
        (2027, 12, 24, Closed, T2, "CME-SVC-2027-12-22"),
    ],
};
