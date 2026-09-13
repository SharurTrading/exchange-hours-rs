// SPDX-License-Identifier: MIT-0

//! CME, CBOT, COMEX and NYMEX venue holiday rows, 2010-2027.
//!
//! One module per venue: `cme.rs`, `cbot.rs`, `comex.rs`, `nymex.rs`.
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
//! The routing is a **decision recorded here, not a fact the crate can derive**.
//! `hours_for_exchange(Exchange::Cme, _)` resolves to a schedule, not to a
//! membership list, and by the consumer contract the map from product families
//! to venues belongs to `SharurPlatform`. The list below is the plan's Wave 7
//! decision (`docs/plans/2026-09-12-path-to-release.md` §3 2.1); each venue's
//! evidence file states what the choice costs and what would change it.
//!
//! | Venue | Families | Why these |
//! |---|---|---|
//! | `cme` | `globex_equity_index`, `globex_energy`, `globex_fx`, `globex_grains`, `globex_interest_rates`, `globex_livestock` | the plan's six. `globex_nikkei_225_dollar` is left out because it agrees with `globex_equity_index` on every date in this window, so it would change neither an answer nor a recorded disagreement; `globex_cryptocurrency` is left out because, although it shares all nine closures, it states nothing on seventeen of the other dates and would therefore join seventeen more disagreements without moving an answer |
//! | `cbot` | `globex_grains`, `globex_interest_rates` | the two families whose sessions the venue profile is built from |
//! | `comex` | `globex_energy`, metals half | the venue's documented scope |
//! | `nymex` | `globex_energy`, energy half | the venue's documented scope |
//!
//! COMEX and NYMEX share one family key, because CME prints the two halves as
//! one product row on every date in this window: no date is dropped for a
//! disagreement between `GC` and `CL`. Their two tables hold the same rows —
//! they are separate tables rather than one alias, because a venue's table is a
//! decision about that venue — and
//! `the_energy_venues_carry_the_family_table_unchanged` holds each against the
//! family's own answers.
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
//! someone, because on 2026-12-24 the families' sourced closes are 12:05, 12:15
//! and 12:45 CT at once, so a row at either end stops someone early or runs
//! someone past their own close. `Unsourced`
//! clips nothing, answers nothing, and tells the caller what the crate knows:
//! the date is special and the venue has no one answer for it. ICE Futures
//! U.S.'s venue table, which shipped first, is the precedent, and the
//! disagreement behind each row is named in the venue's evidence file.
//!
//! Evidence: [`docs/evidence/cme.md`](../../../../../docs/evidence/cme.md),
//! [`cbot.md`](../../../../../docs/evidence/cbot.md),
//! [`comex.md`](../../../../../docs/evidence/comex.md),
//! [`nymex.md`](../../../../../docs/evidence/nymex.md).

mod cbot;
mod cme;
mod comex;
mod nymex;

pub(crate) use cbot::CBOT;
pub(crate) use cme::CME;
pub(crate) use comex::COMEX;
pub(crate) use nymex::NYMEX;
