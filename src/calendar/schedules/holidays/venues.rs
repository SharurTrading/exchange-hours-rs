// SPDX-License-Identifier: MIT-0

//! CME, CBOT, COMEX and NYMEX venue holiday rows, 2010-2027.
//!
//! One module per venue: `cme.rs`, `cbot.rs`, `comex.rs`, `nymex.rs`.
//!
//! `Exchange::Cme`, `Cbot`, `Comex` and `Nymex` each route several product
//! families onto one schedule, and a holiday row is a statement about **one**
//! clock. **The venue speaks for its profile clock** (AGENTS.md,
//! LAW-HOLIDAY-SCOPE): `cbot`'s is the grain grid, `cme`'s the equity-index
//! grid, `comex`'s and `nymex's` the energy grid. The 2026-10-09 amendment
//! completes the rule the 2026-09-30 decision (issue #153) began and #242
//! applied to clock-normal dates: wherever the clock states a row the venue
//! ships that row verbatim — kind, tier and document id — wherever the clock
//! audits the date normal the venue ships no row, and wherever the clock
//! itself withholds the venue inherits the `Unsourced` marker. Each table is
//! therefore the **verbatim mirror** of one family table, not an intersection:
//! a routed family that prints a different arrangement on a date the clock
//! answers keeps its own sourced row in its own family table, where the
//! consumer's exact-family routing (#118) reads it, and the venue's evidence
//! file names every such disagreement. Nothing here rests on an artifact a
//! family module does not already carry.
//!
//! Each venue declares the six eras as six coverage windows — every interval
//! from 2010-01-01 to 2027-12-31 is inside one of them. The rows behind the
//! mirrors are T1 (CME's own holiday-calendar PDFs and Globex holiday
//! schedules) through 2022-2024 except where the trading-hours service answers
//! (2024-03-29, 2024-12-25 and each family's one inherited 2023-01-16 marker),
//! and T2 (that service) for 2025-2027.
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
//! | Venue | Clock | Families routed |
//! |---|---|---|
//! | `cme` | `globex_equity_index` | `globex_equity_index`, `globex_energy`, `globex_fx`, `globex_grains`, `globex_interest_rates`, `globex_livestock` |
//! | `cbot` | `globex_grains` | `globex_grains`, `globex_interest_rates` |
//! | `comex` | `globex_energy` | `globex_energy` (metals half) |
//! | `nymex` | `globex_energy` | `globex_energy` (energy half) |
//!
//! COMEX and NYMEX share one family key, because CME prints the two halves as
//! one product row on every date the table audits. Their two tables hold the
//! same rows — they are separate tables rather than one alias, because a
//! venue's table is a decision about that venue — and the mirror fence holds
//! each against the family's own answers.
//!
//! **What the two families routed to `cme` but no other venue would cost is
//! known.** `globex_nikkei_225_dollar` states the same rows as
//! `globex_equity_index` on every date both cover before 2025 and would move no
//! venue answer on any date the clock answers; under the clock rule it could
//! not withhold anything, because only the clock answers for the venue. Its
//! thirteen 2024 `Unsourced` markers and 2022-05-30 (the wave did not retrieve
//! the 2024 Nikkei channel; the 2022 Memorial Day sheet merges the line with
//! BTIC) would sit in its own family table exactly as they do now.
//! `globex_cryptocurrency` moves no answer in any era.
//!
//! **What a routed family's disagreement costs the venue is disclosure, not a
//! refusal.** On 2026-12-24 the routed families' sourced closes are 12:05
//! (grains), 12:15 (equity, fx, rates, livestock) and 12:45 CT (energy) at
//! once. The venue answers from its clock — for `cme`, equity's 12:15 CT
//! early close — and the other families' instants stay in their own family
//! tables, where the consumer's exact-family routing reads them; the venue
//! evidence files record the dissent per date so the disclosure survives.

mod cbot;
mod cme;
mod comex;
mod nymex;

pub(crate) use cbot::CBOT;
pub(crate) use cme::CME;
pub(crate) use comex::COMEX;
pub(crate) use nymex::NYMEX;
