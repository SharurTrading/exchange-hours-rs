<!-- SPDX-License-Identifier: MIT-0 -->

# Date exceptions and holiday calendars

Normal-week profiles and date exceptions solve different problems. A permanent
or recurring change to a venue's weekday phases belongs in its sourced profile
timeline. A closure, delayed first open, shortened final close, emergency halt,
or other arrangement for named trade dates belongs in an exception layer. A
one-off holiday must never be encoded as though the venue changed its normal
profile from that date onward.

## The boundary layer

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
application-owned calendars outside the crate while avoiding a different ad-hoc
record shape in every consumer.

These scalar overrides are intentionally not described as a complete holiday
calendar. They cannot express an extra intraday pause, a special reopen, a
regular-only close while extended trading continues, or a holiday trade date
that contains several disjoint blocks spanning multiple civil dates. Those
cases need complete replacement sessions rather than boundary clipping, and
they are what the exception layer below exists for.

For example, CME's [2015 Thanksgiving schedule](https://www.cmegroup.com/tools-information/holiday-calendar/files/2015-thanksgiving-holiday-schedule.pdf)
contains a pause and reopen across the holiday window: for Equity Products it
opens Wednesday 17:00 CT *for trade date Friday 27 November*, halts into a
pre-open at Thursday 12:00 CT, resumes trading at Thursday 17:00 CT, and closes
early at Friday 12:15 CT. One trade date, two disjoint tradeable blocks, three
civil dates. Nasdaq's
[2011 early-close notice](https://www.nasdaqtrader.com/TraderNews.aspx?id=ETA2011-54)
is recorded in this repository as having kept different extended sessions open
after the 13:00 ET regular close on Nasdaq, BX, and PSX — recorded, not
verified: see the retrieval note below, which establishes that the alert itself
is unreachable. Neither arrangement can be reduced truthfully to one
venue-wide `early_close_ssm`. Both are pinned as fence tests in
`tests/session_exceptions.rs`.

Retrieval note, 2026-09-02. The CME PDF above was read through the public web
archive because cmegroup.com serves an anti-scraping block:
`https://web.archive.org/web/2016id_/https://www.cmegroup.com/tools-information/holiday-calendar/files/2015-thanksgiving-holiday-schedule.pdf`.
The Nasdaq alert was **not** retrievable: ETA2011-54 has no snapshot in the
archive's CDX index for `nasdaqtrader.com/TraderNews.aspx` (2011-51, -52, -53,
-58 and -60 are held; -54 is not), and nasdaqtrader.com serves a bot
interstitial. What is sourced for that date is the NASDAQ OMX holiday calendar
captured 2011-11-10, which lists 24 November 2011 as closed and 25 November
2011 as an "Early Market Close" for NASDAQ, BX and PSX and directs readers to
the trader alerts for system operating times:
`https://web.archive.org/web/20111110145837id_/http://www.nasdaqtrader.com/trader.aspx?id=calendar`.
The fence test's wall clocks for the continuing extended session are therefore
labelled in place as fixture values exercising the shape, not as sourced times.
This costs nothing at runtime — the crate ships no exception data either way —
but the gap stays visible rather than being implied away.

## The exception layer — shipped for caller-owned data

The replacement-session engine and its public surface ship. **No exception data
ships with them**, exactly as `DayPolicy` shipped: the crate provides the model,
the validation, and the engine integration, and the caller owns every record.

A provider implements `SessionExceptionSource` and distinguishes four states per
venue-local trade date:

- `KnownNormal` — the date was audited and has no exception;
- `Closed` — no session belongs to the trade date;
- `ReplaceSessions` — a complete ordered set of regular/extended/order-entry
  blocks with explicit local-day offsets and a trade-date assignment; and
- `OutOfCoverage` — the provider has no authoritative answer for the date.

That distinction prevents a missing record outside the audited window from
silently becoming an ordinary weekday. Runtime queries necessarily serve the
normal week for both `KnownNormal` and `OutOfCoverage` — a deterministic engine
has no third answer for an unknown date — so the distinction is exposed rather
than acted on: `PolicyCalendar::session_exception_on` and the provider's own
coverage window are how a caller refuses to trade an unaudited date.

Each provider is scoped to one exact `CalendarSource` and publishes its first
and last covered trade dates. Attaching a provider to a calendar with a
different identity returns `ExceptionScopeError` instead of letting one venue's
holiday topology drive another's answers. Source, revision, review-date, and
finality metadata stay with the caller's dataset, which is where the evidence
chain lives.

`StaticSessionExceptions` is the allocation-free, `const`-constructible table
format — the `StaticDayPolicy` analogue. Its constructor validates coverage
bounds, strictly increasing record dates, record membership in the coverage
window, non-empty replacements, block ordering, and every block's domain.
Closed and replaced are mutually exclusive by construction: a record is one
`SessionExceptionRecord` variant, and an empty block set is rejected in favour
of a closed record.

`ExceptionBlock` uses the same representation as a normal `SessionRule` —
venue-local seconds since midnight, end-exclusive closes, `open_ssm >=
close_ssm` wrapping into the next local day, and the asymmetric DST bias where
opens resolve earliest and closes latest. `open_day_offset` places the block's
opening local day relative to its trade date, within
`ExceptionBlock::MIN_DAY_OFFSET..=ExceptionBlock::MAX_DAY_OFFSET`. Like a
`SessionRule`, a block spans at most one local midnight; longer continuous
trading is stated as adjacent blocks.

A trade date is named by the local date of its final close, so nothing in a
record may close after it: a block at offset `0` is rejected if it wraps. That
costs no expressiveness, because a block covering one whole local day is
stated as `open_ssm = 0`, `close_ssm = 86_400`, which does not wrap. Blocks at
a negative offset wrap freely — that is how an evening open reaches its own
trade date.

**Precedence, and it does not vary.** The exception layer resolves the trading
day first: a replaced or closed trade date deletes its normal-week occurrences
outright. The caller's `DayPolicy` then overlays the result exactly as it
overlays a normal week — a closed date removes it, an early close clips it, a
late open delays it. Two replacement layers never compose: attaching a provider
replaces any provider already attached.

The identity conventions survive the new layer. A replacement record's
trade-date assignment overrides every derived convention, and CME
cryptocurrency's following-open-business-day rule skips a date the exception
layer closes just as it skips one a `DayPolicy` closes. A profile with no final
daily close has no trade-date identity, so it ignores both overlays rather than
inventing one.

**Sourcing policy, recorded 2026-08-31.** This project uses only publicly
available operator and regulator material, and it encodes the schedule facts
those documents state — opening and closing times, phase boundaries, effective
days. Reading a published schedule and encoding the times it states is the use
those publications are made for, and public availability of the source is what
gates the work — nothing else does. This is the standard every new or revised
literal is held to; it does not assert that every literal already in the crate
meets it. Where adjacent primary support is still missing, the ledger says so on
the row and [sources.md](sources.md) marks the source set "Missing/uncited",
and those gaps stay visible rather than being papered over by this policy. A source behind a
member portal or an authenticated feed is out of scope as a data source; its
existence and publication date may still be cited as evidence that a change
occurred, as the SGX Titan newsletters are.

Built-in data is still not included, and that is a scope decision rather than a
missing feature. A complete backfill is an evidence project, not a table-size
problem:

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

Complex holiday schedules remain caller-owned. Do not approximate them with
normal-week profile revisions or with a scalar boundary override that deletes a
valid phase — the replacement layer exists so that neither approximation is
ever necessary.
