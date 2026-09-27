// SPDX-License-Identifier: MIT-0

//! CME, CBOT, COMEX and NYMEX venue holiday rows, 2025-2027.
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
//! them are the documents the family tables already cite — CME's own
//! trading-hours service, at tier **T2** — and nothing in this module rests on
//! an artifact a family module does not already carry. Each venue declares one
//! coverage window, `(2025, 1, 1) ..= (2027, 12, 31)`: the support floor
//! onward. The pre-2025 eras the tables once carried left in Stage 5 of the
//! release plan (docs/plans/2026-09-12-path-to-release.md, #117); their rows
//! and their audit history live in Git history, and every artifact behind them
//! stays in the research store. Dates before the floor are refused by the
//! Stage 2B coverage contract, so no removed row could answer anything the
//! crate still asks. There is accordingly no venue evidence of its own to add:
//! the four venue evidence files record the derivation, the routing, the
//! intersection rule and every date the intersection drops.
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
//! | `cme` | `globex_equity_index`, `globex_energy`, `globex_fx`, `globex_grains`, `globex_interest_rates`, `globex_livestock` | the plan's six. The two CME families left out are `globex_nikkei_225_dollar` and `globex_cryptocurrency`; what each would cost is stated below |
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
//! **What the two excluded families would cost is known.** The two families'
//! 2025-2027 answers differ from the routed families' on measured dates, not
//! on an assumption. `globex_nikkei_225_dollar`'s answers differ from
//! `globex_equity_index`'s on twenty-three dates in the window: fifteen where
//! both state different rows — the three Stage-4 Saturday trade dates
//! (2026-06-22, 2026-07-06 and 2027-06-21), where Nikkei states a
//! complete-day Saturday row, among them — four 2025 Sunday-eve merges
//! and the 2025-06-20 Juneteenth-eve merge, where only equity index states, and
//! three dates where only Nikkei states (its
//! no-prior-evening-leg block rows for 2025-01-02, 2025-12-26 and
//! 2026-01-02). On the Saturdays the venue already reports `Unsourced`, so
//! routing Nikkei could add no stated row there; the rest of the disagreement
//! is a re-routing decision's to weigh, not this module's to settle.
//! `globex_cryptocurrency` moves no answer in the window: it shares all nine
//! 2025-2027 closures, states nothing on thirty of the sixty-one dates the
//! venue withholds, and every one of its other thirty-one rows lands on a
//! date the venue already reports `Unsourced`, so no audited-normal date
//! could flip.
//!
//! # Two kinds of row
//!
//! **Where every routed family states the same row**, the venue ships that row.
//! In the shipped window that is the nine Globex full closures. On each, all six CME families, both CBOT families, or
//! the single energy family behind COMEX/NYMEX agree to the status, and the
//! venue is closed. A routed
//! family whose table does not cover an era **abstains** there: it has no
//! answer, so it cannot make the families that do cover the era dispute one,
//! and it neither supplies nor withholds a venue row. A family whose table does
//! cover a date and holds no row has audited it normal, which is an answer and
//! does dispute a row.
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
//! disagreement behind each row is named in the venue's evidence file. The same
//! marker ships where every routed family states `Unsourced` itself, because
//! that is agreement too: the families say the same thing, and the venue
//! repeats their answer rather than inventing a row or claiming the date was
//! audited normal.
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
