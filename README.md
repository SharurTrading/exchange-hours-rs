<!-- SPDX-License-Identifier: MIT-0 -->

# exchange-hours

[![CI](https://github.com/SharurTrading/exchange-hours-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/SharurTrading/exchange-hours-rs/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/exchange-hours.svg)](https://crates.io/crates/exchange-hours)
[![Docs.rs](https://docs.rs/exchange-hours/badge.svg)](https://docs.rs/exchange-hours)
![Edition](https://img.shields.io/badge/edition-2024-orange)
![MSRV](https://img.shields.io/crates/msrv/exchange-hours.svg)
![Unsafe](https://img.shields.io/badge/unsafe-forbidden-success)
[![License](https://img.shields.io/badge/license-MIT--0-blue)](LICENSE)

**A Rust library that knows when exchanges are open — and where sessions and chart bars
begin and end.**

Give it a UTC timestamp (`chrono::DateTime<Utc>`) and an exchange, and it answers three
questions:

1. **Is the market open right now?** — regular hours, extended hours, or either.
2. **When does the current session start and end?** — and if the market is closed, when
   the next session opens.
3. **Where do chart bars begin and end?** — intraday, daily, weekly, and monthly bar
   boundaries that follow the exchange's real sessions, so a bar never spans a closed
   period and a daily bar closes at the session close, not at midnight.

Everything is a pure function over a `MarketHours` snapshot, date-aware
`ExchangeCalendar`, or caller-overlaid `PolicyCalendar`: no state, no I/O, no
clock reads, no floats, and `#![forbid(unsafe_code)]`. Timestamps go in as UTC
and come out as UTC — each exchange's local time zone, including its DST
quirks, is handled internally.
The internal ownership and extension model is documented in
[ARCHITECTURE.md](ARCHITECTURE.md).

- **93 source-backed market identities**, plus the synthetic `Exchange::Unknown`
  fallback (94 `Exchange` variants total) — covering US equities/options, US and
  international futures, EU and Asia-Pacific equities, other major global cash
  markets, and always-open crypto, with independently fenced point-in-time
  revisions wherever primary evidence states an unconditional day-level boundary.
- **Session queries** — open/closed by regular/extended/both, session bounds, next open, gaps.
- **Product-family calendars** — all 25 operator-derived `MarketHoursKey`
  values have fixed, point-in-time, and date-aware query surfaces.
- **Caller-supplied day policy** — whole trade-date closures, early final
  closes, and late first opens can be overlaid without putting mutable or
  bundled operator data in this crate. `StaticDayPolicy` provides a validated
  hard-coded table format for these boundary-level exceptions.
- **Caller-supplied exception sessions** — a trade date that pauses and
  reopens, ends regular trading while extended continues, or spans several
  civil dates is replaced outright by an ordered `ExceptionBlock` set.
  `StaticSessionExceptions` is the validated table format; the crate ships
  **zero** exception data.
- **Calendar-aware bar boundaries** — intraday bars clamp to the session close so no bar
  spans a closed period; the day's last bar ends at the daily close itself (CME 16:00 CT,
  never the later Pre-Open or matching restart), and daily/weekly/monthly bars close at
  real session closes, not midnight.
- **DST correctness by construction** — local seconds-since-midnight rules, resolved to
  instants with an explicit, asymmetric bias (opens earliest, closes latest).

## Installation

`exchange-hours` 1.0 requires Rust 1.95 or newer. The Quick start constructs
timestamps directly, so its complete dependency set is:

```toml
[dependencies]
exchange-hours = "1"
chrono = "0.4"
chrono-tz = "0.10"
```

## Quick start

This is the compiled doctest in `src/lib.rs`, copied verbatim. CME equity-index futures match
17:00→16:00 CT and accept weekday orders from 16:45; RTH runs 08:30–15:15 CT:

```rust
use chrono::{TimeZone, Utc};
use chrono_tz::US;
use exchange_hours::{
    CalendarResolution, Exchange, candle_end, hours_for_exchange, next_session_after,
    session_bounds,
};

let ct = |y, m, d, hh, mm| {
    US::Central
        .with_ymd_and_hms(y, m, d, hh, mm, 0)
        .single()
        .expect("valid CT instant")
        .with_timezone(&Utc)
};

// Every snapshot request carries the caller's instant: the crate never reads
// a clock, so a backtest and a live query run identical code. A live caller
// passes their own `Utc::now()` at the application edge.
let hours = hours_for_exchange(Exchange::Cme, ct(2026, 8, 24, 10, 0));

// Monday mid-morning sits inside the regular session. Boundary queries
// return `Option`: `None` means no matching session exists in the bounded
// search horizon (for example, on a pre-go-live date).
let monday_10am = ct(2026, 8, 24, 10, 0);
assert!(hours.is_open_regular(monday_10am));
let (open, close) = session_bounds(&hours, monday_10am).expect("CME trades this week");
assert_eq!(open, ct(2026, 8, 24, 8, 30));
assert_eq!(close, ct(2026, 8, 24, 15, 15)); // end-exclusive

// 16:30 CT is the daily maintenance break: closed, inside an inter-trade-date
// gap (16:00→16:45) no longer than the documented four-hour bound.
let monday_evening = ct(2026, 8, 24, 16, 30);
assert!(!hours.is_open(monday_evening));
assert!(hours.is_maintenance(monday_evening));

// After Friday's close the next accepted-order phase is Sunday's 16:00
// Pre-Open, not Saturday. Matching resumes at 17:00.
let friday_after_close = ct(2026, 8, 28, 16, 30);
let (next_open, _) = next_session_after(&hours, friday_after_close).expect("reopens Sunday");
assert_eq!(next_open, ct(2026, 8, 30, 16, 0));

// Bar boundaries follow the same rules: a daily bar closes at the venue's
// session close, not at midnight.
let daily_close = candle_end(&hours, monday_10am, CalendarResolution::Daily);
assert_eq!(daily_close, Some(ct(2026, 8, 24, 16, 0)));
```

For a venue whose grid can change between dates, keep an `ExchangeCalendar`
instead of carrying one fixed snapshot:

```rust
use chrono::{TimeZone, Utc};
use exchange_hours::{Exchange, calendar_for_exchange};

let bmv = calendar_for_exchange(Exchange::Bmv);
// All predicates, session scans, and candle methods take UTC instants and
// reselect BMV's normal/early grid for each candidate trading date.
let instant = Utc.with_ymd_and_hms(2026, 8, 19, 16, 0, 0)
    .single()
    .expect("valid UTC instant");
let snapshot_for_one_instant = bmv.hours_at(instant);
assert_eq!(snapshot_for_one_instant.exchange, Exchange::Bmv);
```

Product-family calendars use the same date-aware surface. Their identity is a
`MarketHoursKey`, exposed through `CalendarSource`, rather than a venue:

```rust
use exchange_hours::{CalendarSource, MarketHoursKey, calendar_for_market_hours_key};

let calendar = calendar_for_market_hours_key(MarketHoursKey::GlobexInterestRates);
assert_eq!(
    calendar.source(),
    CalendarSource::MarketHoursKey(MarketHoursKey::GlobexInterestRates)
);
assert_eq!(calendar.exchange(), None);
```

## Coverage

| Family | Market identities | Local zone | Session shape |
|---|---|---|---|
| US equities and ATS | 19 | `America/New_York` | 09:30–16:00 regular on matching venues; modeled accepted-order envelopes differ by venue. The set includes LTSE (08:00–17:00), 24X's live daytime service (04:00–20:00), and TXSE (08:00–17:00), each closed before its sourced production launch. Announced overnight expansions remain monitored and unencoded until their readiness conditions and live days are confirmed. |
| FINRA TRFs | 3 | `America/New_York` | 09:30–16:00 regular; outside-RTH reporting is extended under the sourced 04:00–20:00 system envelope from 2026-03-30. FINRA's announced overnight expansion remains unencoded while its date depends on the SIP rollout. |
| US options | 18 | `America/New_York` | Ordinary individual-stock options trade 09:30–16:00 regular. Seventeen venues also expose their current generic order-acceptance queue as extended (06:00, 07:00, or 07:30 by operator); MEMX rejects orders before 09:30. Product-specific ETF, ETN, index, FLEX, floor-only, and designated sessions remain outside scope. Exact launch history is retained, while an unknown historical queue-onset day is disclosed as Partial rather than invented. |
| CME Globex futures | 4 | `US/Central` | The count is four compatibility `Exchange` identities (CME, CBOT, COMEX, NYMEX); seven product-family keys cover scoped U.S. equity indexes, NYMEX energy/PGM and COMEX metals, standard-size CBOT grains, standard-grid CME FX, CBOT/CME interest rates, CME livestock, and CME non-spot-quoted cryptocurrency futures. Fixed-current profiles include the published Pre-Open/order-entry and PCP phases. Dated selectors retain only source-dated phase changes, so all seven key histories—and the four venue defaults that reuse them—are Partial where an older phase-onset day is unavailable. Cryptocurrency moved from the five-day 17:00→16:00 grid to 24/7 trading on 2026-05-29. Its weekday maintenance is 16:00–16:02 with Pre-Open from 16:01; Saturday maintenance is 02:00–04:00 with Pre-Open from 03:45. |
| Cboe Futures (CFE) | 1 | `US/Central` | RTH 08:30–15:00 flows into post-settlement 15:00–16:00; conservative latest queue-acceptance edges are Sunday 16:00:06 and Monday–Thursday 16:45:06 before the 17:00→08:30 overnight wrap. |
| EU equities | 14 | 11 European zones | 09:00–17:30 continuous as the continental default, with venue-owned phases: Xetra's DAX-share envelope includes participant-restricted Extended Retail from 07:00 to 22:00; LSE SETS includes 07:00 pre-trading, randomized opening/noon auctions, and CPX to 16:40; central Euronext profiles use the published nominal phase boundaries and exclude per-security randomized uncross seconds; SIX, BME, Vienna, and Nasdaq Nordic books keep their own phases and clocks. |
| Asia-Pacific equities | 17 | 14 IANA zones | ASX, TMX Australia, NZX, TSE, NSE India, BSE India, HKEX, SGX Securities, Bursa Malaysia, SET, IDX, PSE, HOSE, SSE, SZSE, KRX, and TWSE. Venue unions include accepted block/crossing phases; SET also includes the sourced 2025 DR night session. Security eligibility may be narrower than the exchange envelope. |
| Other major global equities | 6 | Toronto / Istanbul / Johannesburg / Riyadh / São Paulo / Mexico City | TSX, Borsa Istanbul, JSE's main/liquid ZA01 segment, Tadawul, B3, and BMV, including their pre-open, closing, trade-at-last, and accepted post-close order phases. B3/BMV grids are date-aware because they follow New York's offset relationship. |
| ICE complex & European energy | 9 | London / Amsterdam / Berlin / Dubai / New York / Winnipeg | Named product-family profiles: Eurex FESX/FDAX/FDXM, EEX Nordic Zonal Power, ICE FANG+, Brent, FTSE 100, Dutch TTF, Murban, and legacy Canola. Date-aware profiles preserve product launches, FTSE extensions, Eurex's fixed-UTC Asian pre-trading/auction and continuous phases, Endex's 2026 extension/DST rule, Murban's New-York-locked grid, and Canola's sourced 2010–2018 eras. |
| Asia-Pacific futures (SGX) | 1 | `Asia/Singapore` | Three-Month SORA Futures: continuous 07:25–17:55 and 18:15→05:15, with opening/closing phases and a 18:00–18:05 gap. Closed before the 2024-07-29 launch. |
| Always-open crypto | 1 | `UTC` | Binance USDⓈ-M perpetuals are normally 24×7 after their exact 2019-09-13 04:00 UTC launch. |

The supplied Databento catalog is covered explicitly: all 50 distinct venue
labels across equities, equity options, futures, and options on futures map to
non-`Unknown` identities. See the checked
[Databento venue crosswalk](docs/schedules/databento-venues.md) for exact vendor
labels, stable enum variants, canonical wire names, and each ledger basis.

Futures hours track the *product family*, not merely the listing venue.
`MarketHoursKey` has 26 variants—25 operator-derived product-family keys plus
the synthetic `AlwaysOpen` key. They reuse profiles and are not additional
venues. `session_profile` exposes each family's fixed-current static table;
`hours_for_market_hours_key` selects the sourced snapshot at the caller's
instant; `calendar_for_market_hours_key` reselects the dated profile while
scanning sessions and candles.

Every key's `snake_case` name is a stable persisted wire identity shared by
`as_str`, `Display`, `FromStr`, and Serde. Renaming one is a breaking change.
The crate deliberately does **not** map symbols, roots, product codes, or MICs
to keys: the caller's sourced instrument catalog must make that selection.

The venue-keyed API retains these explicit defaults for compatibility:

| Venue identity | Default family profile |
|---|---|
| `cme` | `globex_equity_index` |
| `cbot` | `globex_grains` |
| `comex`, `nymex` | `globex_energy` |
| `cfe` | `cfe_vix` |
| `eurex` | `eurex` |
| `iceus` | `ice_us` |
| `sgx` | `sgx` |

Those defaults are the wrong choice for any product outside the named family.
CME interest-rate, livestock, and cryptocurrency products must use their family
keys rather than `Exchange::Cme` or `Exchange::Cbot`. `Exchange::Cbot` resolves
to the standard grain and oilseed grid, so Rough Rice (`ZR`/`OZR`) must select
`globex_rough_rice`: its extended session was cut to Sunday-Thursday
19:00-21:00 CT on 2018-01-21 and no longer wraps past midnight. The same applies to the
two venues whose default now covers only a small slice of what they list:
`Exchange::Iceus` resolves to NYSE FANG+, so Sugar No. 11, Coffee "C", Cocoa,
Cotton No. 2, FCOJ-A and the U.S. Dollar Index must select `ice_us_sugar`,
`ice_us_coffee`, `ice_us_cocoa`, `ice_us_cotton`, `ice_us_orange_juice` or
`ice_us_dollar_index`; `Exchange::Sgx` resolves to Three-Month SORA, so SGX
equity-index products must select the matching grid: `sgx_equity_index_japan`,
`sgx_equity_index_china`, `sgx_equity_index_singapore`,
`sgx_equity_index_taiwan`, or `sgx_equity_index_ntr_usd`. Eurex
fixed income likewise has its own `eurex_fixed_income` key, distinct from the
`eurex` index-futures default, and Nikkei 225 Dollar uses
`globex_nikkei_225_dollar` rather than `globex_equity_index`.

Family selection is exact: consumers must never substitute the nearest venue
or product-family key when a product is outside that key's documented scope.
Nikkei 225 Dollar futures (`NKD`), the six ICE Futures U.S. families, CBOT
Rough Rice, and Eurex fixed income all ship as sourced keys. SGX equity-index products do **not**
share one grid, so they ship as five separate keys and the ambiguous name
`sgx_equity_index` stays rejected rather than resolving to one venue-wide clock.
See
[Ambiguous futures families](docs/schedules/unsupported-families.md).

The table contains 93 source-backed market identities. `Exchange::Unknown` is
an additional synthetic 24×7 UTC fallback and is not counted as an exchange or
trading venue.

## Schedule assurance

**Repository-wide review completed:** `2026-08-22`

That is the 93-identity ledger cutoff. Product-family keys were reviewed in the
same pass and carry their own basis labels in the ledger.

**Hours verified against the exchange at the review date:** `93 of 93` non-synthetic
`Exchange` identities, within each row's documented normal-week scope.

**Full dated history back to January 2010:** `66 of 93` non-synthetic
`Exchange` identities.

**History complete except for one named gap:** `27 of 93` non-synthetic
`Exchange` identities.

In plain terms:

- **All 93 venues are right for today.** Every venue's present-day normal week
  was compared against the operator's own published schedule, inside the scope
  its ledger row states. None is unreviewed, and none is known to be wrong.
- **66 of them are also right for any date back to January 2010.** Ask one of
  these what the hours were on an arbitrary past date and every answer is
  carried by dated primary sources the whole way back.
- **The other 27 are right for today, and right for the past except for one
  specific thing each.** Every one of those 27 rows names its own gap in the
  [ledger](docs/schedules/verification.md), and the gap is bounded: where a
  phase is sourced at both ends, the crate serves the part that is true under
  every sourced state and withholds only the disputed remainder.

**Which window a gap sits in decides what it costs you.** A gap in a phase where
trades print — the regular or extended session — would change whether the crate
reports a market as tradeable. A gap in an order-entry window only changes
whether orders could be *queued* ahead of an open that is itself modelled
correctly; no trade can print in one of those windows on any venue in this crate.
Every `Partial` row states which kind it is, and the split is **35 order-entry
to 12 executable** across the 47 rows in the ledger. The order-entry majority is
the exact *day* an older queue or post-close phase started, with the trading
session itself sourced. The executable twelve — the ICE Futures U.S. keys, CME
Nikkei 225 Dollar and the SGX equity-index keys — are each served
conservatively, erring toward closed rather than claiming hours they cannot
support. A recent executable-only audit of all sixteen US futures product
families found none of them withholding executable time that the current grid
serves. Rows carry this distinction in the ledger, so check there before treating
a `Partial` label as a reason to hesitate.

Those 27 are not all the same, and the ledger says which kind each one is. Most
are **knowledge-bound**: a real exchange change happened and no operator ever
published the day, because the value was an operator system setting no filing
ever fixed — searching harder will not close them. A few are **source-limited**:
the document that would date them exists but is a member-only or password-locked
publication. The remainder are **unfinished searches**, where a dated document
should exist and simply has not been found. `cboe_edga` and `cboe_edgx` used to
be the examples of that last kind; they are not any more. The SEC orders that
wrote the 06:00 order-entry start into Rule 11.1(a)(1) have been identified and
dated, and Direct Edge's own FIX and API specifications supply the earlier
07:00 queue back to launch — leaving a knowledge-bound residue of four months
in late 2010 and early 2011 during which the specifications move acceptance
from 07:00 to 06:00 with no source naming the day. Closing all 27 is the current
priority, ahead of any built-in holiday data — the exception-session engine
ships, its data does not.

Every non-synthetic identity was compared with its official current-hours or
rulebook material and its notice/evidence channel. All 93 current profiles are
primary-supported within their stated scope. The 66 **Primary** rows have no
known modeled-history gap since January 2010 or their sourced launch; 27
**Partial** rows name an older queue, PCP phase, or exact onset that available
primary evidence cannot date. No row relies on Secondary, Pragmatic, or Known
issue evidence. `Exchange::Unknown` is synthetic and is not one of the 93
source-backed identities.

The key surface was audited separately:
**Hours verified at the review date for each product family:** `25 of 25` operator-derived
`MarketHoursKey` values. The key API provides fixed-current snapshots, an
`as_of` selector, and a date-aware calendar for sourced histories. Five key
rows are **Primary** and twenty are **Partial**, because a named historical
queue, PCP amendment day, or undated venue transition cannot be dated from a
primary source.

Queries before January 2010 fall outside all of this. The crate records
amendment history back to January 2010 by design, and an instant before that
floor resolves to that venue's oldest profile on record rather than to an
absence — its pre-launch closure where the venue launched later, or its earliest
grid otherwise. It is not a reviewed answer, and nothing below the floor is
covered by the counts above.

These are backward-looking evidence statements, not promises that an exchange
will remain unchanged after the review date. They cover recurring weekday
phases, time zones, lunch and maintenance gaps, and weekend boundaries. They
exclude built-in holidays, half-days, one-off closures or halts,
severe-weather exceptions, and product-specific variations outside a row's
stated scope. A change confined to a single trade date — an early final
close, a late first open, or a full calendar-day closure — is always a
holiday-class date exception, never a normal-week template change: the
built-in tables and their dated revisions encode only real, recurring
exchange behavior. Callers can apply their own sourced closed-day,
early-close, and late-open boundary data through `DayPolicy`, and holiday
arrangements that replace or split phases through `SessionExceptionSource`.
An early close or a late open is exactly a clipped boundary on an otherwise
normal session, which is what `DayPolicy` applies. An arrangement that
replaces or splits phases is not, and is never approximated by clipping: it
states its own blocks through `SessionExceptionSource`. Neither mechanism
ships with data. The full method, corrections, exclusions, and confidence
levels are recorded in the
[2026-08-22 schedule audit](docs/schedules/audit-2026-08-22.md).

The guarantee is exchange/segment/product-family level, never ticker-level
microstructure. When an auction uncross is randomized per security, the row
states whether the deterministic profile uses the operator's nominal phase
boundary or a conservative venue envelope; it does not predict that day's
per-security uncross second.

CME's post-2026-05-29 cryptocurrency weekend is stored as one-midnight
`SessionRule` pieces, but the identity-aware
`calendar_for_market_hours_key(GlobexCryptocurrency)` joins adjacent pieces
into CME's exact multi-day bounds. Both blocks around Saturday maintenance
carry the following open business date: normally Monday, with a daily bar from
Friday 16:01 Pre-Open through Monday 16:00 CT. If a caller's `DayPolicy` closes Monday,
the weekend trading remains open, rolls to Tuesday, and the daily bar closes
Tuesday at 16:00. Its key calendar also retains the real Friday 16:00 final
weekly close before the next trade-date week enters Pre-Open at 16:01. A fixed
`MarketHours` snapshot has no family identity: its open/closed state is exact,
but its bounds retain the one-midnight storage pieces and its generic weekly
candle and trade-date queries are unavailable. Use the key-backed calendar for
those family-specific results. Product-specific listing dates after the family
began remain instrument-catalog data.

The [source-set registry](docs/schedules/sources.md) records the stable official
pages and notice/evidence entry points to check for each exchange. The
repeatable process is in
[Updating exchange schedules](docs/schedules/updating.md). Exact historical
notices remain cited beside the Rust table they support.

## Historical amendments

`hours_for_exchange` and `hours_for_market_hours_key` return fixed-current
snapshots, while their `_as_of` counterparts select the fixed snapshot at a
UTC instant. For queries that span dates, use `calendar_for_exchange` or
`calendar_for_market_hours_key`: their session and candle methods resolve the
applicable profile again for every candidate trading day.

- **Recorded changes only.** A venue gets a historical cutover only when a primary source
  (exchange notice, rulebook amendment, press release) states a day-level effective date.
  Real changes without a sourced date are documented as Partial gaps rather than given
  invented dates. A fixed current snapshot includes a source-verified current phase;
  an `as_of` selector adds it only from a source-stated effective day. Consequently a
  Partial row's dated history can conservatively omit an undated queue even when its
  date-free current snapshot is exact. The verification ledger names every such interval.
  For Paris, Amsterdam, Brussels, and Lisbon, Euronext's
  per-security 0–30-second auction uncross delay is outside the exchange-level schedule,
  so those profiles use the operator's published nominal phase boundaries. Dublin and
  Milan retain their documented conservative latest-edge envelopes. Every choice
  preserves the exact venue-wide open/closed envelope. IEX and Blue Ocean have sourced
  production launch boundaries, and B3's explicit older grids are fully recorded to
  January 2010.
- **Conditional future changes are not schedules yet.** Nasdaq, EDGX, NYSE
  Arca, MEMX, 24X, and the three FINRA TRFs have official future-session plans
  that still depend on readiness, SIP, clearing, or later filings. MX2 Options,
  IEX Options, MRX's additional sessions, and GIX are also watched as future
  identities/phases. They remain deliberately absent from runtime selectors
  until every condition and actual production day is confirmed.
- **Cutover semantics.** Date-only changes are compared in the venue's **own local zone**.
  The new profile applies from venue-local midnight on its session opening day—often Sunday
  for a Monday trade-date change. When a primary source states an exact intraday instant,
  that instant is preserved rather than rounded to midnight; one nanosecond before it sees
  the old profile.
- **How far back.** The aim is to record every session-defining amendment back to
  **January 2010**; changes before that are out of scope by design. Below a venue's oldest
  recorded profile, `hours_for_exchange` keeps returning that oldest profile at earlier
  instants — it does not manufacture an older row or change. Venues with no recorded change
  return their one grid at every instant.

B3 and BMV use recurring cross-zone selection. B3 chooses its short or long cash-equity
grid from the New York−São Paulo UTC-offset difference; BMV chooses its early or normal
grid from the New York−Mexico City difference. This covers the mismatch weeks created by
the countries' former DST calendars and the post-abolition rules without hard-coding yearly
US transition dates. A `MarketHours` returned for one date remains a fixed
snapshot, so use the appropriate date-aware calendar rather than carrying it
across a transition.

## Trade dates, state, and caller day policies

On `ExchangeCalendar` and `PolicyCalendar`, `trade_date(instant)` returns the
venue-local trade date of the containing session, or `None` while closed.
Wrapped sessions use the date of the trading day's final close, so a normal
Sunday-evening Globex instant maps to Monday. An always-open profile has no
final close, so its trade date is always `None`; consequently
`is_closed_trade_date` is true for every date because no session is assigned to
one. Use `is_closed_all_day_on` or `is_closed_all_day_in_calendar` for
civil-day availability.
`session_state(instant)` returns exactly one of `OpenRegular`, `OpenExtended`,
`Halt`, `Maintenance`, or `Closed`. A halt separates phases of the same trade
date. Maintenance is normally an inter-trade-date gap no longer than four
elapsed hours within one ISO week. A profile explicitly marked as having no
weekend close also retains an operator-designated short maintenance gap inside
one trade date; this covers CME cryptocurrency's Saturday 02:00–03:45 CT
closed interval before its 03:45–04:00 Pre-Open. Longer afternoon gaps, closed
days, and weekends are closed.
`is_maintenance` is exactly the maintenance-state predicate.

The built-in profiles remain normal-week schedules and ship no holiday data.
Implement `DayPolicy`, or construct a validated `StaticDayPolicy` from
hard-coded `DayOverride` records, then call
`ExchangeCalendar::with_day_policy` to create a `PolicyCalendar`. It applies
closed trade dates, early final closes, and late first opens to every
predicate, scan, trade-date, state, and candle query.
Policy dates are trade dates: for an ordinary wrapped session, closing Monday
removes the trading day that would have opened Sunday evening. CME's continuous
cryptocurrency weekend follows the operator's next-open-business-day rule
instead: closing Monday leaves the weekend trading open and assigns it to
Tuesday. Profiles without a final daily close—such as `AlwaysOpen`—have no
trade date and ignore the day-policy overlay rather than inventing one.
`PolicyCalendar::hours_at` intentionally returns the unmodified
sourced snapshot because the policy is a query overlay, not another static
profile.

For a small caller-owned table, no custom trait implementation is needed:

```rust
use chrono::NaiveDate;
use exchange_hours::{
    DayOverride, MarketHoursKey, StaticDayPolicy, calendar_for_market_hours_key,
};

let closed_date = NaiveDate::from_ymd_opt(2026, 4, 20).expect("valid trade date");
let overrides = [DayOverride::closed(closed_date)];
let policy = StaticDayPolicy::new(&overrides).expect("records are sorted and valid");
let calendar = calendar_for_market_hours_key(MarketHoursKey::GlobexEquityIndex)
    .with_day_policy(&policy);
```

The date above is an API example, not bundled operator data; callers remain
responsible for the source and exact family/venue scope of every record.

Boundary clipping is not a complete holiday-session model. It cannot insert an
intraday pause/reopen or close only regular trading while an extended phase
continues. Those arrangements go through the **exception-session layer**, which
ships for caller-owned data alongside `DayPolicy`. Implement
`SessionExceptionSource`, or build a validated `StaticSessionExceptions` table,
then call `ExchangeCalendar::with_session_exceptions`:

```rust
use chrono::NaiveDate;
use exchange_hours::{
    CalendarSource, Exchange, ExceptionBlock, SessionExceptionRecord,
    StaticSessionExceptions, calendar_for_exchange,
};

// A regular-only early close: extended trading continues past 13:00 local.
static BLOCKS: [ExceptionBlock; 2] = [
    ExceptionBlock::regular(0, 9 * 3_600 + 30 * 60, 13 * 3_600),
    ExceptionBlock::extended(0, 13 * 3_600, 17 * 3_600),
];

let half_day = NaiveDate::from_ymd_opt(2026, 11, 27).expect("valid trade date");
let first = NaiveDate::from_ymd_opt(2026, 11, 23).expect("valid trade date");
let last = NaiveDate::from_ymd_opt(2026, 11, 30).expect("valid trade date");
let records = [SessionExceptionRecord::replace_sessions(half_day, &BLOCKS)];
let table = StaticSessionExceptions::new(
    CalendarSource::Exchange(Exchange::Nasdaq),
    first,
    last,
    &records,
)
.expect("records are sorted, in range, and inside the coverage window");
let calendar = calendar_for_exchange(Exchange::Nasdaq)
    .with_session_exceptions(&table)
    .expect("the table is scoped to this calendar");
```

Each record is one venue-local trade date and is exactly one of *known normal*,
*closed*, or *replaced* — never two at once. Blocks state venue-local
seconds-since-midnight with end-exclusive closes, the same asymmetric DST bias
as normal profiles, and an explicit opening-day offset, so a Globex-style day
that opens the previous evening uses `-1`. A table publishes its audited
first/last trade dates, and a date outside that window reports
`DateException::OutOfCoverage` rather than silently reading as an ordinary
weekday. A table scoped to another `CalendarSource` is refused, not applied.

Precedence is fixed: the exception layer resolves the trading day, then a
`DayPolicy` overlays it exactly as it overlays a normal week. Two replacement
layers never compose. The contract, the coverage and finality requirements, and
the maintenance checklist for any exception dataset are in
[Date exceptions and holiday calendars](docs/schedules/date-exceptions.md).

## Best effort — validate before production use

Primary-sourced tables and dated revisions carry their citations beside the data and are
pinned by tests. Every non-synthetic current profile is source-supported within the product or
segment scope stated in the ledger. Rows labeled Primary have complete
January-2010-or-launch history at that scope; Partial rows explicitly identify the older
phase or onset that could not be dated. This crate is a **best-effort model, not an authority**:
exchanges amend hours on short notice, publish product-level exceptions, and run holiday
and half-day schedules that the built-in normal-week tables deliberately omit.
Supply boundary-level exceptions through `DayPolicy` when it can represent
them exactly, and multi-phase holiday schedules through
`SessionExceptionSource`; do not reduce a multi-phase holiday schedule to one
cutoff.
Before trading on
any venue's hours in production, have a human verify the profile against the exchange's
currently published schedule and the relevant contract specifications.

## Design

This is a foundational leaf crate. It depends only on `chrono` + `chrono-tz`
(instant/zone arithmetic and the DST resolver) and `serde` (canonical string
serialization of public identities)—no logging facade, engine, transport,
adapter, async runtime, credential crate, or `tokio`.

Built-in `MarketHours` status/bound/candle queries and `ExchangeCalendar`
status/bound/trade-date/candle queries allocate nothing. `is_open` is
`O(rules)` for a fixed snapshot and `O(rules + log revisions)` for a
date-aware calendar. Session-bound, forward, candle, and trade-date queries
multiply that work by their documented bounded day scans; identity-aware
coalescing may inspect adjacent rule pieces.
`ExchangeCalendar` is `Copy + Send + Sync + 'static`; work performed inside a
caller's `DayPolicy` is outside that guarantee. The Criterion
`calendar_queries` benchmark records `is_open`, `session_bounds`, daily
`candle_end`, `trade_date`, and closed-gap `session_state` costs for
`GlobexEquityIndex`.

## Upgrading from 0.2.x

- The retired `intelligentcross_iqx` value has no replacement `Exchange`
  variant. Remove persisted entries that used it, or keep an application-level
  mapping if the IntelligentCross ATS is still required; never translate it to
  another venue or to `Exchange::Unknown`.
- `Exchange` and `MarketHoursKey` now serialize as canonical `snake_case`
  strings in every Serde format. Earlier binary ordinal payloads must be decoded
  with their original crate version and rewritten using the string form.
- Equal `SessionRule` endpoints now encode one complete local-day session, and
  `SessionRuleError::EmptyInterval` was removed. Omit a rule to represent no
  session.
- `ExchangeCalendar::exchange()` now returns `Option<Exchange>` because the
  same calendar type can represent a `MarketHoursKey`. Existing venue callers
  handle `Some(exchange)`; family calendars return `None`. Use `source()` when
  both identity kinds are valid, or `market_hours_key()` for the family case.
- `SessionState` now uses trade-date-aware gap classification with a four-hour
  maintenance ceiling. Same-trade-date gaps are normally `Halt`; a sourced gap
  inside a continuously traded week can remain `Maintenance` within that bound.
  Longer inter-trade-date and policy-created gaps are `Closed`. Recheck callers
  that persisted or matched the former state labels.
- The raw `US_EQUITY_REGULAR`, `US_EQUITY_EXTENDED`,
  `NYSE_TEXAS_EXTENDED`, and `BLUE_OCEAN_EXTENDED` slices are no longer public.
  Use `hours_for_exchange` or `calendar_for_exchange` so venue and historical
  routing cannot be bypassed.

See the [1.0.0 changelog](CHANGELOG.md#100---2026-08-22) for the complete API,
schedule, and migration record.

## Boundaries & invariants

- **UTC in, UTC out.** Every timestamp argument and timestamp result is `DateTime<Utc>`; the exchange `Tz` is an
  internal detail and callers never pass or receive local times.
- **Closes are end-exclusive.** An instant equal to a close is *closed*. Relaxing this makes
  adjacent sessions overlap and every duration double-count at its boundary.
- **`open_ssm >= close_ssm` means the session wraps** into the next local day. Equal
  endpoints encode one complete local-day span, so a Sunday `18:00→18:00` rule is one
  continuous session through Monday 18:00. Omit the rule to express no session. Any "is it
  open?" answer must therefore consult yesterday's rules as well as today's.
- **Opens bias earliest, closes bias latest.** On an ambiguous fall-back hour both mappings
  are valid; the asymmetry keeps a session inclusive while its end-exclusive close lands on
  the true boundary, and a spring-forward gap snaps to the first instant after it. This is
  the one place a maintainer must not "simplify" to a single bias.
- **Calendar bar starts come from sessions, not durations.** `candle_start` pairs a
  daily/weekly/monthly close with the first session open in that period, so a start can fall
  on the preceding civil day, month, or year. Probe a provider close marker at `close - 1ns`.
- **The `Exchange` matches are exhaustive inside the crate.** `hours_for_exchange` and
  `Exchange::as_str` have no catch-all arms, so a new variant forces a profile decision and
  a canonical name instead of silently inheriting defaults. The enum itself is
  `#[non_exhaustive]` (like `MarketHoursKey`), so venue additions are not breaking changes
  for dependents — match it with a wildcard arm, and enumerate via `Exchange::ALL`.
- **Always-open venues stay categorically separate.** A 24×7 venue is a single `0..86400`
  rule on all seven days with `has_daily_close` / `has_weekend_close` both `false`; it is
  never folded into the CME-style daily-break profiles.
- **One canonical name per identity, and it is stable.** `Exchange` and
  `MarketHoursKey` serialize as `snake_case` strings in every Serde format and
  expose the same names through `as_str`, `Display`, and `FromStr`; each also
  has an `ALL` enumeration. String-keyed callers parse instead of
  pattern-matching (`"nyse_arca".parse::<Exchange>()`). Unrecognized input
  returns `ParseExchangeError` or `ParseMarketHoursKeyError`, never a silent
  `Exchange::Unknown`. A rename that changes one of these strings breaks
  persisted data. Neither identity enum uses variant ordinals, so adding or
  removing a row cannot silently reinterpret another identity.
- **Normal week plus explicit overlays.** Built-in tables contain no holiday,
  half-day, or product-level exception data. `DayPolicy` and the validated
  `StaticDayPolicy` helper let a caller overlay sourced closed trade dates,
  early final closes, and late first opens without changing `hours_at`.
  Multi-phase exceptions are not approximated by that boundary API: they go
  through `SessionExceptionSource` and `StaticSessionExceptions`, which replace
  a whole trade date with an ordered block set and publish their own audited
  coverage window. Both layers are caller-owned; verify contract specs before
  trading on a profile outside its explicitly stated scope.
- **No panics, and absence is `None`.** The public surface is total, and boundary queries
  (`session_bounds*`, `next_session_after*`, `candle_start*`/`candle_end*`,
  `time_end_of_day`) return `Option`: a profile with no session of the requested kind in
  the bounded search horizon (a venue before its go-live date), or a zero-duration candle
  resolution, is `None` — never a fabricated degenerate pair that could leak downstream as
  a real session. Predicates report closed/not-in-maintenance/closed-all-day.
- **`SessionRule` has a stated domain.** `SessionRule::new` / `validate` enforce
  `open_ssm < 86_400`, `close_ssm <= 86_400`, and at least one enabled weekday; every
  shipped table is fence-checked against the same domain. Equal endpoints are intentional
  complete-local-day rules, not empty intervals.

## Testing

Pure and stateless, so the validation class is property/deterministic-fixture, not workload.
Every test is an integration test over the public surface: the crate exposes nothing to tests
that callers do not also get (see [Architecture: Tests](ARCHITECTURE.md#tests)).

- `tests/venue_sessions.rs` and `tests/venue_sessions/` — a thin harness over focused futures,
  equities, candle, wrap, bounds, and correction modules. Together they pin published opens,
  end-exclusive closes, overnight wraps, maintenance gaps, weekend boundaries, always-open
  venues, serde forms, and source-cited data corrections.
- `tests/futures_family_boundaries.rs` and `tests/futures_family_boundaries/` — a thin
  harness over dated-boundary fixtures split by venue family (CME Nikkei, CME family
  queues, ICE, SGX equity index). Each assertion probes an instant on one side of a
  sourced cutover, at venue-local midnight for the revision and to the second for the
  session open, so a mis-keyed revision or a one-minute encoding slip fails here.
- `tests/apac_equities.rs` and `tests/apac_equities/` — a thin integration harness with
  separate modules for current boundaries, regional amendment history, and APAC bulk/name
  contracts across all 17 cash-equity venues.
- `tests/global_equities.rs` and `tests/global_equities/` — a thin harness over current
  baselines, amendment history, and the global bulk/name contract for TSX, Borsa Istanbul,
  JSE, Tadawul, B3, and BMV.
- `tests/schedule_documentation.rs` and `tests/schedule_documentation/` — a
  thin harness over contracts that keep all 94 `Exchange` rows (93
  non-synthetic plus `Unknown`) and 26 `MarketHoursKey` rows (25
  operator-derived plus `AlwaysOpen`) in canonical order; validates their
  review metadata and owner/source links; requires both current and
  notice/evidence channels for every source set; rejects orphaned source sets;
  and prevents the README freshness date from drifting from the ledger.
- `tests/seasonal_calendars.rs` and `tests/seasonal_calendars/` — a thin integration harness
  split into B3, BMV, transition-scan, candle/weekend, Chrono-edge, and compatibility-contract
  modules. The contracts include all-fixed-venue `MarketHours`/`ExchangeCalendar` parity and
  a daily history scan proving venue time zones remain stable.
- `tests/calendar_policies.rs` and `tests/calendar_value_traits.rs` — pin
  date-aware key parity, trade-date/state classification, caller policy
  overlays, bounded all-closed scans, and the calendar's value/thread traits.
- `tests/support/` — fixture construction shared by integration targets; it uses only the
  public crate surface and provides no test-only access to production internals.
- `tests/no_session_contract.rs` — the `None` contract for profiles with no rules.
- `tests/rule_validation.rs` — the `SessionRule` domain: what `new`/`validate` accept and reject.
- `tests/contract/session_invariants.rs` and `tests/contract/session_invariants/` — a thin,
  explicitly configured harness over handwritten identity/history expectations and focused
  property modules. A fixed-seed `splitmix64` sweep plus pinned DST fixtures cover totality,
  determinism, maintenance, ordered bounds, advancing session walks, and the all-venue
  cross-query fence. The production tables are never imported into these expectations.
- `src/lib.rs` doctest — the Quick start example above.

The CI gate, in order — `cargo fmt --all --check`, `cargo clippy --all-targets -- -D warnings`
(clippy **pedantic** plus the panic-family and determinism restriction lints, configured in
`Cargo.toml` and `clippy.toml`), `cargo nextest run`, `cargo test --doc`, `cargo doc` with
warnings denied, and `cargo deny check` for dependency licences and advisories. A second job
builds on the declared MSRV so that floor is exercised rather than asserted. The build
toolchain is pinned in `rust-toolchain.toml`. Run everything the `quality` job runs:

```bash
cargo fmt --all --check && cargo clippy --all-targets -- -D warnings && cargo nextest run --all-targets && cargo test --doc && RUSTDOCFLAGS="-D warnings" cargo doc --no-deps && cargo deny check
```

The MSRV job needs a second toolchain, so it is a separate command:

```bash
cargo +1.95 check --all-targets
```

## License

[MIT-0](LICENSE) (MIT No Attribution): use, copy, modify, and redistribute freely,
with no attribution required and no warranty of any kind — see the best-effort
disclaimer above before trading on these tables.
