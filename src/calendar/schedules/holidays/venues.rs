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
//! them are the documents the family tables already cite: CME Group's own
//! holiday-calendar PDFs, at tier **T1** for the 2010-2012 rows; CME's own
//! published Globex holiday schedules, at tier **T1** for the 2016-2018 rows,
//! which are the design memo's D17 intersection of the routed families' T1 rows
//! rather than a retrieval of their own; and its `trading-hours-by-product`
//! service, at tier **T2** for the 2025-2027 ones. Nothing in this module rests
//! on an artifact a family module does not already carry, and each venue
//! declares the three eras as three coverage windows — the 2013-2015 and
//! 2019-2024 intervals between them are audited by no wave and ship no row.
//! There is accordingly no venue evidence of its own to add: the four venue
//! evidence files record the derivation, the routing, the intersection rule and
//! every date the intersection drops.
//!
//! On the 2016-2018 era's **thirty-six dates** the CME and CBOT tables state
//! nine `Closed` rows — the dates every routed family shut — and withhold the
//! other twenty-seven as [`HolidayKind::Unsourced`], while `comex` and `nymex`
//! carry `globex_energy`'s thirty-one rows unchanged. A date is disputed
//! wherever the routed families do not state the same row: on eighteen of them
//! the four financial families halt at 12:00 CT while `globex_grains` is shut
//! outright, on the three Thanksgiving Fridays and 2018-12-24 the families'
//! closes are 12:05, 12:15 and 12:45 CT at once, and on 2016-12-23, 2017-12-22,
//! 2017-07-03, 2018-07-03 and 2018-12-26 `globex_grains` states a row every
//! financial family audited normal. A routed family **with no table for an
//! era abstains** rather than disputing: `globex_livestock` covers 2010-2012
//! and 2025-2027 and not 2016-2018, so it neither supplies nor withholds a
//! venue row there, and the families that do cover the era decide each date.
//! Metals and energy are one key and CME prints them as one product row, so the
//! `globex_energy` rows carry through untouched.
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
//! | `cme` | `globex_equity_index`, `globex_energy`, `globex_fx`, `globex_grains`, `globex_interest_rates`, `globex_livestock` | the plan's six. `globex_nikkei_225_dollar` is left out because it agrees with `globex_equity_index` on every date the two now share — the same thirty-four rows in 2016-2018 and the same thirty-one in 2025-2027 — so it would change neither an answer nor a recorded disagreement; `globex_cryptocurrency` is left out because, although it shares all nine 2025-2027 closures, it states nothing on seventeen of the other dates and would therefore join seventeen more disagreements without moving an answer |
//! | `cbot` | `globex_grains`, `globex_interest_rates` | the two families whose sessions the venue profile is built from |
//! | `comex` | `globex_energy`, metals half | the venue's documented scope |
//! | `nymex` | `globex_energy`, energy half | the venue's documented scope |
//!
//! COMEX and NYMEX share one family key, because CME prints the two halves as
//! one product row on every date either table audits: no date is dropped for a
//! disagreement between `GC` and `CL`. Their two tables hold the same rows —
//! they are separate tables rather than one alias, because a venue's table is a
//! decision about that venue — and
//! `the_energy_venues_carry_the_family_table_unchanged` holds each against the
//! family's own answers.
//!
//! # Two kinds of row
//!
//! **Where every routed family states the same row**, the venue ships that row.
//! In 2025-2027 that is the nine Globex full closures; in 2010-2012 it is the
//! six that CME published for those years. On each, all six CME families, both
//! CBOT families, or the single energy family behind COMEX/NYMEX agree to the
//! status, and the venue is closed. 2016-2018 ships no such row: a routed family
//! with no answer in an era leaves the venue without a unanimous one either, so
//! those dates take the `Unsourced` treatment below.
//!
//! **Where they disagree**, the venue ships [`HolidayKind::Unsourced`]. That is
//! neither silence nor a compromise instant. An audited window is contiguous,
//! so a date carrying no row inside one is the positive claim that it was
//! audited normal, which on these dates is false; and any single instant would
//! be wrong for someone, because on 2026-12-24 the families' sourced closes are 12:05, 12:15
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
