<!-- SPDX-License-Identifier: MIT-0 -->

# Date exceptions and holiday calendars

Normal-week profiles and date exceptions solve different problems. A permanent
or recurring change to a venue's weekday phases belongs in its sourced profile
timeline. A closure, delayed first open, shortened final close, emergency halt,
or other arrangement for named trade dates belongs in an exception layer. A
one-off holiday must never be encoded as though the venue changed its normal
profile from that date onward.

## What v1 provides

`DayPolicy` is a deterministic, trade-date-keyed boundary overlay.
`StaticDayPolicy` gives callers an allocation-free, validated table format for
hard-coded `DayOverride` records. It supports:

- a completely closed trade date;
- a later first open for the trade date;
- an earlier final close for the trade date; or
- both boundary changes on the same trade date.

Records are exact dates, not inferred weekday or holiday rules. The table is
opt-in: callers apply it with `ExchangeCalendar::with_day_policy`, and
`hours_at` continues to return the unmodified normal-week profile. This keeps
application-owned or licensed calendars outside the crate while avoiding a
different ad-hoc record shape in every consumer.

These scalar overrides are intentionally not described as a complete holiday
calendar. They cannot express an extra intraday pause, a special reopen, a
regular-only close while extended trading continues, or a holiday trade date
that contains several disjoint blocks spanning multiple civil dates. Those
cases need complete replacement sessions rather than boundary clipping.

For example, CME's [2015 Thanksgiving schedule](https://www.cmegroup.com/tools-information/holiday-calendar/files/2015-thanksgiving-holiday-schedule.pdf)
contains a pause and reopen across the holiday window. Nasdaq's
[2011 early-close notice](https://www.nasdaqtrader.com/TraderNews.aspx?id=ETA2011-54)
kept different extended sessions open after the 13:00 ET regular close on
Nasdaq, BX, and PSX. Neither arrangement can be reduced truthfully to one
venue-wide `early_close_ssm`.

## Path to built-in calendars

A future built-in exception provider must be additive and opt-in. Before any
operator data drives runtime answers, its model must distinguish:

- `KnownNormal` — the date was audited and has no exception;
- `Closed` — no session belongs to the trade date;
- `ReplaceSessions` — a complete ordered set of regular/extended blocks with
  explicit local-day offsets and a trade-date assignment; and
- `OutOfCoverage` — the provider has no authoritative answer for the date.

That distinction prevents a missing record outside the audited window from
silently becoming an ordinary weekday. Each provider must be scoped to one
exact `CalendarSource`, publish its first and last covered trade dates, and
retain source, revision, review-date, and finality metadata. Replacement blocks
must use the same local-time resolution and end-exclusive-close rules as normal
profiles.

**Sourcing policy, recorded 2026-08-31.** This project uses only publicly
available operator and regulator material, and it encodes the schedule facts
those documents state — opening and closing times, phase boundaries, effective
days. It does not reproduce or redistribute the documents themselves, so
redistribution rights over an operator's publications are not what gates this
work — public availability is. Every literal in the crate is a fact read from a
public primary source and cited back to it, which is the use those publications
are made for. A source behind a
member portal or an authenticated feed is out of scope as a data source; its
existence and publication date may still be cited as evidence that a change
occurred, as the SGX Titan newsletters are.

Built-in data is not included in v1. A complete backfill is an evidence
project, not a table-size problem:

- holiday topology differs by venue, segment, and futures product family;
- operator notices are sometimes revised;
- exceptional closures such as weather events or national days of mourning
  must be retained explicitly;
- a calendar that exists only behind authentication is out of scope under the
  sourcing policy above, so its venue stays `OutOfCoverage` rather than being
  filled from a non-public source; and
- a future calendar is finite and may change after publication.

## Future dates

An announced holiday may be recorded as monitoring metadata, but it must not
drive `is_open` until the operator's required conditions are satisfied and the
detailed schedule has been revalidated. CME states on its
[holiday and trading-hours page](https://www.cmegroup.com/trading-hours.html)
that holiday schedules are subject to change and are usually finalized about
two weeks before the holiday. NYSE publishes a longer forward calendar, while
its [official 2026 yearly calendar](https://www.nyse.com/publicdocs/nyse/ICE_NYSE_2026_Yearly_Trading_Calendar.pdf)
marks its dates subject to change. A future provider therefore needs an explicit
announced/final distinction; the normal runtime path may consume only the
final side.

## Maintenance checklist

For each exception dataset:

1. Define the exact exchange, segment, or product-family scope.
2. Audit a continuous date range, including dates confirmed normal.
3. Cite the operator notice beside every exceptional record and retain revised
   notices in the evidence chain.
4. Test every block edge, phase kind, trade-date assignment, daily/weekly bar
   boundary, and the first date outside coverage.
5. Recheck future entries inside the operator's finalization window.
6. Confirm the source is primary and publicly available without
   authentication, and record that alongside its citation.

Until that contract is implemented, complex holiday schedules remain
caller-owned. Do not approximate them with normal-week profile revisions or
with a scalar boundary override that deletes a valid phase.
