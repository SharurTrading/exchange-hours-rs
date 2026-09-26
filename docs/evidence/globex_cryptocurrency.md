<!-- SPDX-License-Identifier: MIT-0 -->

# `globex_cryptocurrency` — evidence

- **Kind:** `MarketHoursKey`
- **Owner module:** [`cryptocurrency.rs`](../../src/calendar/schedules/futures/us/cryptocurrency.rs)
- **Source sets:** [`US-CME-GROUP`](../schedules/sources.md#us-cme-group)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

**Gap: order-entry** — the trading session is sourced; what is undated is a queue or post-close phase in which no trade can print. CME non-spot-quoted cryptocurrency futures. The exact current 24/7 phases, 2026-05-29 transition, multi-day bounds, weekly close, and following-open-business-day convention are retained, as are the three one-day Saturday maintenance extensions CME's Globex notices state for this family's channels 326/327 — 2026-08-01 to 09:00 CT (notice 20260727), 2026-08-29 to 06:00 and 2026-09-19 to 08:00 CT (notice 20260824, restated by 20260831) — each without a replacement Pre-Open and each reverting to the 02:00–04:00 standard window; the September row is forward-dated on the operator's statement, and the two later rows were added on 2026-09-06 (#61). The 2017–2026 matching grid is exact, but primary evidence does not date the five-day era's Sunday/weekday Pre-Open onset, so dated history omits those queues. The 2026-08-31 review confirmed this at the source: the bitcoin contract specification captured 2017-12-14 — carrying the launch statement for trade date 2017-12-18 — and the 2017-12-22 and 2018-01-04 captures publish the Globex matching grid only and state no Pre-Open. Later member-product listings remain catalog data.

## Revision rows

- 2017-12-17 — T1 — CME SER-8051R — five-day launch grid, 17:00–16:00 CT.
- 2026-05-29 — T1 — CME filing 26-114 — one-day bridge into the 24/7 grid.
- 2026-05-30 — T1 — CME filing 26-114 — permanent 24/7 normal week.
- 2026-08-01 — T1 — CME Globex notice 20260727 — Saturday reopen 09:00 CT.
- 2026-08-02 — T1 — CME Globex notice 20260727 — revert to the standard window.
- 2026-08-29 — T1 — CME Globex notice 20260824 — Saturday reopen 06:00 CT.
- 2026-08-30 — T1 — CME Globex notice 20260824 — revert to the standard window.
- 2026-09-19 — T1 — CME Globex notice 20260824 — Saturday reopen 08:00 CT.
- 2026-09-20 — T1 — CME Globex notice 20260824 — revert to the standard window.

## Holidays

**Coverage:** 2019-01-01..2021-12-31, 2022-01-01..2024-12-31, 2025-01-01..2027-12-31 (inclusive venue-local trade dates).

**Three contiguous audited eras.** The table declares 3 coverage windows:
`2019-01-01..2021-12-31`, from CME's own published Globex holiday schedules at **T1**;
`2022-01-01..2024-12-31`, from the 2022 workbooks and 2023 one-pagers at T1 and CME's service
responses at **T2**; and `2025-01-01..2027-12-31`, from the trading-hours service at T2. They are
adjacent, so no gap lies between them. Everything before 2019 lies outside every declared window,
and `holiday_on` therefore has **no answer** there rather than reporting an unaudited date as
normal: the 2013-2015 era is declared by the six families CME published a holiday schedule for,
but not by this one, because the years before its 2017-12-17 launch list no cryptocurrency futures
at all and no crypto line exists on any 2013-2015 sheet to source; the eras before 2010 are out of
scope below the crate's January-2010 floor. `HolidayCoverage::windows()` lists the 3, and `contains`
answers per date.
**Zone.** Verbatim from the operator page: "Trading hours are subject to change
and are in U.S. Central Time unless otherwise stated." This channel prints no
Eastern column, so no ET value is asserted anywhere below; every instant is CT
as printed.

**Event vocabulary**, verbatim from the same page: `preopen` — "Order Entry,
modification, and cancel are allowed. No order matching."; `open` — "Start of
continuous trading phase. Order matching begins."; `closed` — "Final Close of
the date. Day and GTD (current trade date) orders are eliminated." A `closed`
event is therefore the operator's own statement of which trade date ended, and
`/TD <date>` is the trade date the service prints beside an event.

**Family label.** The service groups products, and `Cryptocurrency (BTC)` — the
Bitcoin future as the group's representative product — is the line this key is
built from. No separate line for ETH, MBT or MET is published in this channel.

**How an event-date record became a trade-date row.** CME keys a holiday by the
civil day its events fall on and prints each event's own trade date. The crate
keys by the trade date. So a record whose events all carry the *following*
business date is evidence that the holiday has **no trade date of its own**, and
the date is not a `normal` day. Which row states that absence depends on the era
and on whether the preceding evening's leg ran: a `closed` row on the holiday
itself where the operator published no session for it at all, or the following
business day's merged `replacement blocks` row where the operator ran matching
across it and only re-labelled the span. The table below gives each shape and the
row it becomes. The retrieval's own status words map onto that as follows:

| Retrieval status and note | Crate row |
|---|---|
| `closed` with `no events published` | `closed` |
| `closed` `[N17]` — only a 16:00 CT pre-open and a 17:00 CT open, both already carrying the next business date | `closed` |
| `modified` `[N3]` — no final close on the date and a 16:00 CT rather than 16:45 CT pre-open, the whole span rolling into the next business day | no row on the holiday; `replacement blocks` on that next business day |
| `modified` `[N10]` — 24/7 era, Monday or Thursday holiday: the 16:00 CT final close is omitted outright | `closed` |
| `closed` `[N19]` — 24/7 era, Friday holiday: the 16:00 CT final close is printed but carries the following Monday's trade date | `closed` |
| `early_close` with `close_instant` | `early close` at that instant |
| `modified` `[N6]` — a normal final close with no evening re-open, the leg the *neighbouring* `closed` row already removes | no row |
| `modified` `[N18]` / `normal` `[N8]` — the date settles its own trade date and only the re-open rolls | no row |

### Interpretive notes

The mapping table above annotates its retrieval shapes with the retrieval's own
short codes — `N3`, `N6`, `N8`, `N10`, `N17`, `N18` and `N19` — and the 2025 and
2026 sections below cite `N3`, `N6` and `N18` again in their interpretive steps.
Those codes are defined in the research store's `raw/cme-2025-2027/INDEX.md`
(`N1`..`N16`, `EC`), in that file's round-1 addendum (`N17`, `N18`) and in
`raw/cme-2025-2027-repair/INDEX.md` (`N19`), none of which is committed, so a
reader following a row to its reasoning currently reaches a dead reference. Every
code this file's mapping table and interpretive steps print is defined here beside
the bytes that exhibit it; `N1` is the one code the store defines that this file
never prints, and it is left undefined at the end of this section. Artifact
paths are relative to the research store's `holidays/` directory, and every
quotation below is a verbatim substring of the file at the path named beside it —
the JSON, or the reader-original `.md` where the path names one — given with the
product id and the `eventDate` it was read from. The modelled line throughout is
product `8478` (Bitcoin Futures; the service prints its `globex` code as `BTC`
and its schedule `groupCode` as `BF`).

| Note | In one line | Crate row |
|---|---|---|
| `N3` | five-day era, Monday or Thursday holiday: the normal 16:00-17:00 CT matching pause is printed as a 16:00 CT `preopen` rather than as a final close, and every event on the span carries the following business day's trade date | no row on the holiday; the merged span ships as `HolidayKind::ReplacementBlocks` on that following business day |
| `N6` | a final close at the normal 16:00 CT but no evening re-open | no row (the missing leg belongs to the neighbouring `closed` row) |
| `N8` | the published event list — times, types and printed trade dates — equals the family's normal grid for that weekday | no row (audited normal) |
| `N10` | 24/7 era, Monday or Thursday holiday: the 16:00 CT final close is omitted outright | `HolidayKind::Closed` on that trade date |
| `N17` | no day session: only a 16:00 CT pre-open and a 17:00 CT open, both already carrying the next business day's trade date | `HolidayKind::Closed` on that trade date |
| `N18` | the published event times equal the normal grid, but the re-open carries a trade date that skips the following closed holiday | no row (the closure it rolls past carries one) |
| `N19` | 24/7 era, Friday holiday: the 16:00 CT final close is printed but carries the following Monday's trade date | `HolidayKind::Closed` on that trade date |

**`N3` — the ordinary daily 16:00-17:00 CT matching pause, no early close: the operator prints its normal 16:00 CT boundary as a `preopen` rather than as a final close, so no final close falls on the date and its trade date rolls to the next business day.**

- **Evidence.** `raw/cme-2025-2027/arc/thbp_2025-01-19_2025-01-21_20241220155340.json` — document `CME-SVC-2025-01-19`, sha256 `4f2ab56af14e7b3a6978e7fa6db8e2cfc63a82d06428cd88844f0e5bcf534f40`, archive capture 2024-12-20T15:53:40Z. Product `8478`, `eventDate` `2025-01-20` (Monday, Martin Luther King Jr. Day):

  ```json
  {"groupCode":"BF","eventDate":"2025-01-20","events":[{"tradingDate":"2025-01-21","eventTime":"16:00","marketEventType":"preopen"},{"tradingDate":"2025-01-21","eventTime":"17:00","marketEventType":"open"}]}
  ```

  The Sunday this family's five-day week opens on, and the trade date every event in the response names, from the same artifact:

  ```json
  {"groupCode":"BF","eventDate":"2025-01-19","events":[{"tradingDate":"2025-01-21","eventTime":"16:00","marketEventType":"preopen"},{"tradingDate":"2025-01-21","eventTime":"17:00","marketEventType":"open"}]}
  {"groupCode":"BF","eventDate":"2025-01-21","events":[{"tradingDate":"2025-01-21","eventTime":"16:00","marketEventType":"closed"},{"tradingDate":"2025-01-22","eventTime":"16:45","marketEventType":"preopen"},{"tradingDate":"2025-01-22","eventTime":"17:00","marketEventType":"open"}]}
  ```

  No event anywhere in that response carries trade date 2025-01-20: the request covers exactly these three `eventDate` records, and every event in them carries 2025-01-21 or 2025-01-22.

- **Crate row.** No row on the holiday 2025-01-20, and the span ships as `HolidayKind::ReplacementBlocks` on the trade date the operator labels it with, 2025-01-21 — `(2025, 1, 21, ReplacementBlocks(&MERGED_SESSION_BLOCKS), T2, "CME-SVC-2025-01-19")` in `src/calendar/schedules/holidays/globex_cryptocurrency.rs`. 2025-02-18, 2025-05-27, 2025-09-02, 2026-01-20, 2026-02-17 and 2026-05-26 carry the same kind and block table; 2025-06-20 uses `MERGED_SESSION_AFTER_WEEKDAY_BLOCKS`, because its span opens on a Wednesday evening and the `-2` queue is the operator's ordinary 16:45 CT; and 2025-11-28 uses `MERGED_THANKSGIVING_FRIDAY_BLOCKS` for the `07:00-07:30` CT pause its finalised publication adds. That the holiday keys no row is the crate's interpretive step, and its point is what it does **not** do: the 16:00 CT event on 2025-01-20 is a `preopen`, an order-entry-only event by the operator's own vocabulary quoted above, and the crate does not read it as a session close (LAW-SESSION-NOT-EXPIRY). What makes the reading right is the trade dates the operator prints: matching ran from the Sunday 17:00 CT open through the holiday's own 16:00 CT pause and resumed at 17:00 CT, and the operator attached 2025-01-21 to every part of that span. A `closed` row on 2025-01-20 would delete an evening and a day the operator ran; an early close would invent a boundary no event states. What a scalar boundary cannot state — the reassignment of the span's trade date — is what the `ReplacementBlocks` row states instead, so the merge ships rather than being recorded as a gap.

- **Falsified by.** A CME publication printing an early final close on `eventDate` 2025-01-20, or printing any event on it carrying trade date 2025-01-20.

**`N6` — a final close at the normal 16:00 CT but no evening re-open.**

- **Evidence.** `raw/cme-2025-2027/arc/thbp_2025-04-17_2025-04-19_20241220155340.json` — document `CME-SVC-2025-04-17`, sha256 `865a1d4f08102e00151bd87ab2b8e8a7720e9203a17aaaba24627ade3ed26e74`, archive capture 2024-12-20T15:53:40Z. Product `8478`, `eventDate` `2025-04-17`, whose record holds exactly one event:

  ```json
  {"groupCode":"BF","eventDate":"2025-04-17","events":[{"tradingDate":"2025-04-17","eventTime":"16:00","marketEventType":"closed"}]}
  ```

  An ordinary day of the same era prints three, as the `N3` quotation above shows: the same 16:00 CT close, then a 16:45/17:00 CT re-open carrying the next trade date. The next record in this response is the closure that missing leg would have served:

  ```json
  {"groupCode":"BF","eventDate":"2025-04-18","events":[]}
  ```

- **Crate row.** No row — this file records 2025-04-17, 2025-07-03 and 2025-12-31 as `modified` `[N6]` records that ship nothing. What makes that right: the only event the operator omitted is the evening re-open, and that leg's trade date is the *next* date — 2025-04-18 here — which the neighbouring `closed` row already removes in full, so the missing leg is not this date's to state. The 16:00 CT event is the family's ordinary final close at its ordinary instant, so there is no early close to carry either, and the next date's own row is where the closure belongs.

- **Falsified by.** A later publication printing an evening re-open on `eventDate` 2025-04-17, which would make its 16:00 CT instant a pause rather than the day's end and would mean 2025-04-18 held a session that the `Closed` row deletes.

**`N8` — the published event list — times, types and printed trade dates — equals the family's normal grid for that weekday, so nothing about the date changed.**

- **Evidence.** `raw/cme-2025-2027-repair/live/probeA_2026-06-17_2026-06-19.md` — research id `D66`, sha256 `f9ee38678821e32e8f54f8084366024173462b108376658be423581c1ba77e09`, retrieved 2026-09-12T08:55:13Z. This is the reader-original `.md`; the `json/probeA_2026-06-17_2026-06-19.json` beside it is a derived re-serialisation, and the sha256 above hashes the `.md`, so the `.md` is what is cited and what the quotation below is a substring of. The corresponding archived capture is **superseded** for this date: `raw/cme-2025-2027/arc/thbp_2026-06-17_2026-06-19_20260129012310.json` was taken 2026-01-29, before CME moved this family to 24/7 on 2026-05-29, and prints the retired `16:45 preopen; 17:00 open` pair. Product `8478`, `eventDate` `2026-06-17` (Wednesday):

  ```json
  {"groupCode":"BF","eventDate":"2026-06-17","events":[{"tradingDate":"2026-06-17","eventTime":"16:00","marketEventType":"closed"},{"tradingDate":"2026-06-18","eventTime":"16:01","marketEventType":"preopen"},{"tradingDate":"2026-06-18","eventTime":"16:02","marketEventType":"open"}]}
  ```

  The baseline is the reference week `raw/cme-2025-2027/live/normal/normalweek_main.json` — sha256 `d3bd6e890bdc427d2ebd3ece48dedc56eb231178be82c69ab3ac427f0b09dc0a`, indexed in `raw/cme-2025-2027/INDEX.md` as "the normal-grid baseline every holiday row is measured against" — product `8478`, `eventDate` `2026-10-21`, the same weekday:

  ```json
  {"groupCode":"BF","eventDate":"2026-10-21","events":[{"tradingDate":"2026-10-21","eventTime":"16:00","marketEventType":"closed"},{"tradingDate":"2026-10-22","eventTime":"16:01","marketEventType":"preopen"},{"tradingDate":"2026-10-22","eventTime":"16:02","marketEventType":"open"}]}
  ```

  Same three times, same three types, and in both the 16:00 CT close carries the date's own trade date while the re-open carries the next calendar day's.

- **Crate row.** No row — 2026-06-17 is inside the 2025-2027 window and ships nothing, which is the crate's audited-normal claim. What makes that right: every answer the crate gives for the date is the ordinary 24/7 grid the response prints, so a row could only change a correct one; the roll past 2026-06-17 is the family's ordinary roll; and the next closure, 2026-06-19, carries its own `Closed` row.

- **Falsified by.** A publication whose `eventDate` 2026-06-17 events differ in time, event type or printed trade date from the same weekday's grid in the reference week.

**`N10` — 24/7 era, Monday or Thursday holiday: the 16:00 CT daily final close is omitted outright, so trading is continuous across the date and its trade date rolls.**

- **Evidence.** `raw/cme-2025-2027/live/thbp/thbp_2026-09-06_2026-09-08.json` — document `CME-SVC-2026-09-06`, sha256 `01fb78ffaac10eac466fed53674214222f05aed518b9d93a4b42cf8957147bca`, live retrieval 2026-09-12T04:30Z. Product `8478`, `eventDate` `2026-09-07` (Monday, Labor Day), whose record holds two events where an ordinary day holds three:

  ```json
  {"groupCode":"BF","eventDate":"2026-09-07","events":[{"tradingDate":"2026-09-08","eventTime":"16:01","marketEventType":"preopen"},{"tradingDate":"2026-09-08","eventTime":"16:02","marketEventType":"open"}]}
  ```

  The next record in the same response shows the complete grid, for contrast:

  ```json
  {"groupCode":"BF","eventDate":"2026-09-08","events":[{"tradingDate":"2026-09-08","eventTime":"16:00","marketEventType":"closed"},{"tradingDate":"2026-09-09","eventTime":"16:01","marketEventType":"preopen"},{"tradingDate":"2026-09-09","eventTime":"16:02","marketEventType":"open"}]}
  ```

  The Thursday case prints the same two-event shape, in `raw/cme-2025-2027/live/thbp/thbp_2026-11-25_2026-11-27.json` — document `CME-SVC-2026-11-25`, product `8478`, `eventDate` `2026-11-26`:

  ```json
  {"groupCode":"BF","eventDate":"2026-11-26","events":[{"tradingDate":"2026-11-27","eventTime":"16:01","marketEventType":"preopen"},{"tradingDate":"2026-11-27","eventTime":"16:02","marketEventType":"open"}]}
  ```

- **Crate row.** `HolidayKind::Closed` on trade date 2026-09-07 — `(2026, 9, 7, HolidayKind::Closed, T2, "CME-SVC-2026-09-06")`, and the same kind for 2026-11-26 and for this file's 2027 Monday and Thursday holidays. This is the crate's interpretive step, and its honest limit is recorded under the 2026 gaps above: the operator prints **no** 16:00 CT event here at all, so the crate reads an omission, not a printed close. What makes the reading right is the trade dates the operator does print: both events that bracket the missing boundary carry 2026-09-08, so the holiday has no trade date of its own to key a row to, and the neighbouring record's own 16:00 CT close carries 2026-09-08 as well. `Closed` then makes the 24/7 roll skip the date, which is what the printed trade dates already show happening. The residue is the 60-second 16:00-16:01 CT maintenance minute the crate's normal week keeps, recorded as a gap; it is not a session boundary the operator printed.

- **Falsified by.** A CME publication printing the 16:00 CT final close on `eventDate` 2026-09-07, or any event on it carrying trade date 2026-09-07.

**`N17` — no day session on the holiday: the operator publishes only a 16:00 CT pre-open and a 17:00 CT open, both already carrying the next business day's trade date.**

- **Evidence.** `raw/cme-2025-2027/arc/thbp_2025-12-31_2026-01-02_20260619114105.json` — document `CME-SVC-2025-12-31`, sha256 `0ed61f8328eda4746265cc8e197f10cd53aec06c2b393927bab27c913993d314`, archive capture 2026-06-19T11:41:05Z. Product `8478`, `eventDate` `2026-01-01`:

  ```json
  {"groupCode":"BF","eventDate":"2026-01-01","events":[{"tradingDate":"2026-01-02","eventTime":"16:00","marketEventType":"preopen"},{"tradingDate":"2026-01-02","eventTime":"17:00","marketEventType":"open"}]}
  ```

  The evening before it, in the same response, is an `N6`-shaped record for its own trade date with no re-open:

  ```json
  {"groupCode":"BF","eventDate":"2025-12-31","events":[{"tradingDate":"2025-12-31","eventTime":"16:00","marketEventType":"closed"}]}
  ```

- **Crate row.** `HolidayKind::Closed` on trade date 2026-01-01 — `(2026, 1, 1, HolidayKind::Closed, T2, "CME-SVC-2025-12-31")`, and the same shape for 2025-01-01 and 2025-12-25. Why that is right: the family's five-day grid has one session per trade date, opening 17:00 CT the previous evening; the 2025-12-31 record shows that leg not running, and both events printed against 2026-01-01 carry 2026-01-02, so nothing is assignable to the holiday. The 17:00 CT open is the next trade date's session beginning — the shape the ordinary week already has — not a session belonging to 2026-01-01, and no expiry, settlement or order-entry cutoff is being read as a close (LAW-SESSION-NOT-EXPIRY).

- **Falsified by.** An event on `eventDate` 2026-01-01 carrying trade date 2026-01-01, or an evening re-open on 2025-12-31 carrying it.

**`N18` — the published event times equal the family's normal grid for that weekday, but the re-open carries a trade date that skips the following closed holiday, so the date settles its own trade date and only the re-open rolls past the closure.**

- **Evidence.** Two records exhibit it. First, `raw/cme-2025-2027-repair/live/probeA_2026-07-02_2026-07-04.md` — research id `D67`, sha256 `0e8b38b67779644a22d41de94e315e599198d6c8c739ef35196163779aacfd4b`, retrieved 2026-09-12T08:55:14Z; the reader-original `.md`, cited as for `N8` above, and again the archived 2026-01-29 capture `raw/cme-2025-2027/arc/thbp_2026-07-02_2026-07-04_20260129012216.json` is superseded by it. Product `8478`, `eventDate` `2026-07-02` (Thursday, the eve of the closed Friday 2026-07-03):

  ```json
  {"groupCode":"BF","eventDate":"2026-07-02","events":[{"tradingDate":"2026-07-02","eventTime":"16:00","marketEventType":"closed"},{"tradingDate":"2026-07-06","eventTime":"16:01","marketEventType":"preopen"},{"tradingDate":"2026-07-06","eventTime":"16:02","marketEventType":"open"}]}
  ```

  Second, `raw/cme-2025-2027/live/thbp/thbp_2027-12-22_2027-12-25.json` — document `CME-SVC-2027-12-22`, sha256 `5edc4dd588a32faa74f841494c10a3df48692dca29843c3581bad3e18c30fef9`, live retrieval 2026-09-12T04:30Z; product `8478`, `eventDate` `2027-12-23` (Thursday, the eve of the closed Friday 2027-12-24):

  ```json
  {"groupCode":"BF","eventDate":"2027-12-23","events":[{"tradingDate":"2027-12-23","eventTime":"16:00","marketEventType":"closed"},{"tradingDate":"2027-12-27","eventTime":"16:01","marketEventType":"preopen"},{"tradingDate":"2027-12-27","eventTime":"16:02","marketEventType":"open"}]}
  ```

  The reference week `raw/cme-2025-2027/live/normal/normalweek_main.json` (sha256 `d3bd6e890bdc427d2ebd3ece48dedc56eb231178be82c69ab3ac427f0b09dc0a`) carries the ordinary Thursday grid whose times both records keep:

  ```json
  {"groupCode":"BF","eventDate":"2026-10-22","events":[{"tradingDate":"2026-10-22","eventTime":"16:00","marketEventType":"closed"},{"tradingDate":"2026-10-23","eventTime":"16:01","marketEventType":"preopen"},{"tradingDate":"2026-10-23","eventTime":"16:02","marketEventType":"open"}]}
  ```

- **Crate row.** No row on either `N18` date — 2026-07-02 and 2027-12-23 both ship nothing. What makes that right: each settles its own trade date at the operator's ordinary 16:00 CT close, so every answer the crate gives for it is already the ordinary week's; the closure each re-open rolls past is the **following** date and carries its own `Closed` row — `(2026, 7, 3, HolidayKind::Closed, T2, "CME-SVC-2026-07-03")` and `(2027, 12, 24, HolidayKind::Closed, T2, "CME-SVC-2027-12-22")`. The second closure is exhibited twice over in the one artifact: the 2027-12-24 record is itself the `N19` shape below, printing all three of its events against 2027-12-27 —

  ```json
  {"groupCode":"BF","eventDate":"2027-12-24","events":[{"tradingDate":"2027-12-27","eventTime":"16:00","marketEventType":"closed"},{"tradingDate":"2027-12-27","eventTime":"16:01","marketEventType":"preopen"},{"tradingDate":"2027-12-27","eventTime":"16:02","marketEventType":"open"}]}
  ```

  — so the crate reads that closure from the holiday's own bytes as well as from the eve's re-open.

- **Falsified by.** The 16:00 CT close on such a date carrying the following Monday's trade date instead of its own, which is `N19` and not `N18`, or the re-open carrying the day's own trade date, which is `N8` and not `N18`.

**`N19` — 24/7 era, Friday holiday: the 16:00 CT final close *is* published, but it carries the following Monday's trade date, so the holiday settles no trade date of its own and the whole day rolls into the next business date.**

- **Evidence.** `raw/cme-2025-2027/live/thbp/thbp_2026-12-24_2026-12-26.json` — document `CME-SVC-2026-12-24`, sha256 `bdc1fe831adb794bcf8aeb7e99baf6af2009d1ff9969d0a48b18b2ebc2e1e829`, live retrieval 2026-09-12T04:30Z. Product `8478`, `eventDate` `2026-12-25` (Friday, Christmas Day):

  ```json
  {"groupCode":"BF","eventDate":"2026-12-25","events":[{"tradingDate":"2026-12-28","eventTime":"16:00","marketEventType":"closed"},{"tradingDate":"2026-12-28","eventTime":"16:01","marketEventType":"preopen"},{"tradingDate":"2026-12-28","eventTime":"16:02","marketEventType":"open"}]}
  ```

  The contrast is inside the same artifact, one day earlier, where the same three times are printed and the 16:00 CT close carries the date's own trade date:

  ```json
  {"groupCode":"BF","eventDate":"2026-12-24","events":[{"tradingDate":"2026-12-24","eventTime":"16:00","marketEventType":"closed"},{"tradingDate":"2026-12-28","eventTime":"16:01","marketEventType":"preopen"},{"tradingDate":"2026-12-28","eventTime":"16:02","marketEventType":"open"}]}
  ```

  The baseline is the reference week `raw/cme-2025-2027/live/normal/normalweek_main.json` (sha256 `d3bd6e890bdc427d2ebd3ece48dedc56eb231178be82c69ab3ac427f0b09dc0a`), whose ordinary Friday is what this shape is measured against:

  ```json
  {"groupCode":"BF","eventDate":"2026-10-23","events":[{"tradingDate":"2026-10-23","eventTime":"16:00","marketEventType":"closed"},{"tradingDate":"2026-10-26","eventTime":"16:01","marketEventType":"preopen"},{"tradingDate":"2026-10-26","eventTime":"16:02","marketEventType":"open"}]}
  ```

- **Crate row.** `HolidayKind::Closed` on trade date 2026-12-25 — `(2026, 12, 25, HolidayKind::Closed, T2, "CME-SVC-2026-12-24")`, and the same kind for every other 24/7-era Friday holiday this file's 2026 and 2027 tables carry. What makes the reading right: the operator's `closed` event is its own final-close statement in session language, and the trade date printed beside it is 2026-12-28 rather than the date the events fall on, while the ordinary Friday grid prints the Friday's own trade date at that same instant — so the difference is the operator's own statement that this date settles nothing. `Closed` then makes the 24/7 roll skip the date, which is what the printed 2026-12-28 on all three events already says. Unlike `N10`, nothing is missing here: the final close is printed, so no part of the ordinary grid's shape is lost, and no expiry, settlement or order-entry instant is read as a close.

- **Falsified by.** The 16:00 CT close on such a Friday carrying the Friday's own trade date, which is the ordinary `N8` shape, or the operator printing no 16:00 CT event on it at all, which is the `N10` shape.

**`N1` is deliberately not defined here.** This file never prints the code: the shape it names — the service publishing no events for the date — appears only as the unnumbered `closed` with `no events published` row of the mapping table above, and in the `closed` rows this file's 2019-2025 tables derive from that row, whose own text states the shape in full. Defining it would put a code in a reader's path that no row of this file cites.

### 2019

| trade date | kind | instant as printed | document | tier | derived from |
|---|---|---|---|---|---|
| 2019-01-01 | closed | `Closed for New Year's` | `2019-new-years-holiday-schedule-compact.xls @2018-01-07T04:13:43Z` | T1 | CME prints the closure on this date, so the session whose final close would have fallen here — the leg that opened the previous evening at 17:00 CT — is removed with it, and the crate's trade date is the operator's own event date. |
| 2019-01-21 | early close | `1200 CT / 1800 UTC` | `2019-holiday-calendars.zip#globex-trading-schedules/2019-martin-luther-king-holiday-schedule-compact.xls @2021-01-26T09:48:37Z` | T1 | The cited sheet's `Bitcoin` line prints `12:00 CT` as this date's own final close, so the leg that opened the previous evening at 17:00 CT is clipped there and the crate's trade date is the date that close falls on — the operator's event date. |
| 2019-02-18 | early close | `1200 CT / 1800 UTC` | `2019-holiday-calendars.zip#globex-trading-schedules/2019-presidents-day-holiday-schedule-compact.xls @2021-01-26T09:48:37Z` | T1 | The cited sheet's `Bitcoin` line prints `12:00 CT` as this date's own final close, so the leg that opened the previous evening at 17:00 CT is clipped there and the crate's trade date is the date that close falls on — the operator's event date. |
| 2019-04-19 | closed | `Closed for Good Friday` | `2019-holiday-calendars.zip#globex-trading-schedules/2019-good-friday-holiday-compact.xls @2021-01-26T09:48:37Z` | T1 | CME prints the closure on this date, so the session whose final close would have fallen here — the leg that opened the previous evening at 17:00 CT — is removed with it, and the crate's trade date is the operator's own event date. |
| 2019-05-27 | early close | `1200 CT / 1700 UTC` | `2019-holiday-calendars.zip#globex-trading-schedules/2019-memorial-day-holiday-schedule-compact.xls @2021-01-26T09:48:37Z` | T1 | The cited sheet's `Bitcoin` line prints `12:00 CT` as this date's own final close, so the leg that opened the previous evening at 17:00 CT is clipped there and the crate's trade date is the date that close falls on — the operator's event date. |
| 2019-06-19 | unsourced | no CME document covers this date (see `2019-holiday-calendars.zip @2021-01-26T09:48:37Z`) | `2019-holiday-calendars.zip @2021-01-26T09:48:37Z` | T1 | The operator published no Juneteenth schedule for 2019, 2020 or 2021: the year's own consolidated bundle `2019-holiday-calendars.zip @2021-01-26T09:48:37Z` — CME's account of every Globex holiday schedule it published that year — carries no Juneteenth sheet, the archived holiday-calendar.html index pages name none, and a fresh 2018-2027 prefix CDX enumeration finds no `juneteenth` filename before 2022. Silence inside the window would read as audited normal on a date the operator later marks as a holiday, so the row is `Unsourced`, which clips nothing. |
| 2019-07-03 | early close | `1215 CT / 1715 UTC` | `2019-holiday-calendars.zip#globex-trading-schedules/2019-4th-of-july-holiday-schedule-compact.xls @2021-01-26T09:48:37Z` | T1 | The cited sheet's `Bitcoin` line prints `12:15 CT` as this date's own final close, so the leg that opened the previous evening at 17:00 CT is clipped there and the crate's trade date is the date that close falls on — the operator's event date. |
| 2019-07-04 | early close | `1200 CT / 1700 UTC` | `2019-holiday-calendars.zip#globex-trading-schedules/2019-4th-of-july-holiday-schedule-compact.xls @2021-01-26T09:48:37Z` | T1 | The cited sheet's `Bitcoin` line prints `12:00 CT` as this date's own final close, so the leg that opened the previous evening at 17:00 CT is clipped there and the crate's trade date is the date that close falls on — the operator's event date. |
| 2019-09-02 | early close | `1200 CT / 1700 UTC` | `2019-holiday-calendars.zip#globex-trading-schedules/2019-labor-day-holiday-schedule-compact.xls @2021-01-26T09:48:37Z` | T1 | The cited sheet's `Bitcoin` line prints `12:00 CT` as this date's own final close, so the leg that opened the previous evening at 17:00 CT is clipped there and the crate's trade date is the date that close falls on — the operator's event date. |
| 2019-11-28 | early close | `1200 CT / 1800 UTC` | `2019-holiday-calendars.zip#globex-trading-schedules/2019-thanksgiving-holiday-schedule-compact.xls @2021-01-26T09:48:37Z` | T1 | The cited sheet's `Bitcoin` line prints `12:00 CT` as this date's own final close, so the leg that opened the previous evening at 17:00 CT is clipped there and the crate's trade date is the date that close falls on — the operator's event date. |
| 2019-11-29 | early close | `1215 CT / 1815 UTC` | `2019-holiday-calendars.zip#globex-trading-schedules/2019-thanksgiving-holiday-schedule-compact.xls @2021-01-26T09:48:37Z` | T1 | The cited sheet's `Bitcoin` line prints `12:15 CT` as this date's own final close, so the leg that opened the previous evening at 17:00 CT is clipped there and the crate's trade date is the date that close falls on — the operator's event date. |
| 2019-12-24 | early close | `1215 CT / 1815 UTC` | `2019-holiday-calendars.zip#globex-trading-schedules/2019-christmas-holiday-schedule-compact.xls @2021-01-26T09:48:37Z` | T1 | The cited sheet's `Bitcoin` line prints `12:15 CT` as this date's own final close, so the leg that opened the previous evening at 17:00 CT is clipped there and the crate's trade date is the date that close falls on — the operator's event date. |
| 2019-12-25 | closed | `Closed for Christmas` | `2019-holiday-calendars.zip#globex-trading-schedules/2019-christmas-holiday-schedule-compact.xls @2021-01-26T09:48:37Z` | T1 | CME prints the closure on this date, so the session whose final close would have fallen here — the leg that opened the previous evening at 17:00 CT — is removed with it, and the crate's trade date is the operator's own event date. |

### 2020

| trade date | kind | instant as printed | document | tier | derived from |
|---|---|---|---|---|---|
| 2020-01-01 | closed | `Closed for New Year's` | `2019-holiday-calendars.zip#globex-trading-schedules/2019-new-years-holiday-schedule-compact.xls @2021-01-26T09:48:37Z` | T1 | CME prints the closure on this date, so the session whose final close would have fallen here — the leg that opened the previous evening at 17:00 CT — is removed with it, and the crate's trade date is the operator's own event date. |
| 2020-01-20 | early close | `1200 CT / 1800 UTC` | `2020-holiday-calendars.zip#2020-martin-luther-king-holiday-schedule-compact.xls @2026-07-30T11:18:34Z` | T1 | The cited sheet's `Bitcoin` line prints `12:00 CT` as this date's own final close, so the leg that opened the previous evening at 17:00 CT is clipped there and the crate's trade date is the date that close falls on — the operator's event date. |
| 2020-02-17 | early close | `1200 CT / 1800 UTC` | `2020-holiday-calendars.zip#2020-presidents-day-holiday-schedule-compact.xls @2026-07-30T11:18:34Z` | T1 | The cited sheet's `Bitcoin` line prints `12:00 CT` as this date's own final close, so the leg that opened the previous evening at 17:00 CT is clipped there and the crate's trade date is the date that close falls on — the operator's event date. |
| 2020-04-10 | closed | `Closed for Good Friday` | `2020-holiday-calendars.zip#2020-good-friday-holiday-compact.xls @2026-07-30T11:18:34Z` | T1 | CME prints the closure on this date, so the session whose final close would have fallen here — the leg that opened the previous evening at 17:00 CT — is removed with it, and the crate's trade date is the operator's own event date. |
| 2020-05-25 | early close | `1200 CT / 1700 UTC` | `2020-holiday-calendars.zip#2020-memorial-day-holiday-schedule-compact.xls @2026-07-30T11:18:34Z` | T1 | The cited sheet's `Bitcoin` line prints `12:00 CT` as this date's own final close, so the leg that opened the previous evening at 17:00 CT is clipped there and the crate's trade date is the date that close falls on — the operator's event date. |
| 2020-06-19 | unsourced | no CME document covers this date (see `2020-holiday-calendars.zip @2026-07-30T11:18:34Z`) | `2020-holiday-calendars.zip @2026-07-30T11:18:34Z` | T1 | The operator published no Juneteenth schedule for 2019, 2020 or 2021: the year's own consolidated bundle `2020-holiday-calendars.zip @2026-07-30T11:18:34Z` — CME's account of every Globex holiday schedule it published that year — carries no Juneteenth sheet, the archived holiday-calendar.html index pages name none, and a fresh 2018-2027 prefix CDX enumeration finds no `juneteenth` filename before 2022. Silence inside the window would read as audited normal on a date the operator later marks as a holiday, so the row is `Unsourced`, which clips nothing. |
| 2020-07-03 | early close | `1200 CT / 1700 UTC` | `2020-holiday-calendars.zip#2020-4th-of-july-holiday-schedule-compact.xls @2026-07-30T11:18:34Z` | T1 | The cited sheet's `Bitcoin` line prints `12:00 CT` as this date's own final close, so the leg that opened the previous evening at 17:00 CT is clipped there and the crate's trade date is the date that close falls on — the operator's event date. |
| 2020-09-07 | early close | `1200 CT / 1700 UTC` | `2020-holiday-calendars.zip#2020-labor-day-holiday-schedule-compact.xls @2026-07-30T11:18:34Z` | T1 | The cited sheet's `Bitcoin` line prints `12:00 CT` as this date's own final close, so the leg that opened the previous evening at 17:00 CT is clipped there and the crate's trade date is the date that close falls on — the operator's event date. |
| 2020-11-26 | early close | `1200 CT / 1800 UTC` | `2020-holiday-calendars.zip#2020-thanksgiving-holiday-schedule-compact.xls @2026-07-30T11:18:34Z` | T1 | The cited sheet's `Bitcoin` line prints `12:00 CT` as this date's own final close, so the leg that opened the previous evening at 17:00 CT is clipped there and the crate's trade date is the date that close falls on — the operator's event date. |
| 2020-11-27 | early close | `1215 CT / 1815 UTC` | `2020-holiday-calendars.zip#2020-thanksgiving-holiday-schedule-compact.xls @2026-07-30T11:18:34Z` | T1 | The cited sheet's `Bitcoin` line prints `12:15 CT` as this date's own final close, so the leg that opened the previous evening at 17:00 CT is clipped there and the crate's trade date is the date that close falls on — the operator's event date. |
| 2020-12-24 | early close | `1215 CT / 1815 UTC` | `2020-holiday-calendars.zip#2020-christmas-holiday-schedule-compact.xls @2026-07-30T11:18:34Z` | T1 | The cited sheet's `Bitcoin` line prints `12:15 CT` as this date's own final close, so the leg that opened the previous evening at 17:00 CT is clipped there and the crate's trade date is the date that close falls on — the operator's event date. |
| 2020-12-25 | closed | `Closed for Christmas` | `2020-holiday-calendars.zip#2020-christmas-holiday-schedule-compact.xls @2026-07-30T11:18:34Z` | T1 | CME prints the closure on this date, so the session whose final close would have fallen here — the leg that opened the previous evening at 17:00 CT — is removed with it, and the crate's trade date is the operator's own event date. |

### 2021

| trade date | kind | instant as printed | document | tier | derived from |
|---|---|---|---|---|---|
| 2021-01-01 | closed | `Closed for New Year's` | `2020-holiday-calendars.zip#2021-new-years-holiday-schedule-compact.xls @2026-07-30T11:18:34Z` | T1 | CME prints the closure on this date, so the session whose final close would have fallen here — the leg that opened the previous evening at 17:00 CT — is removed with it, and the crate's trade date is the operator's own event date. |
| 2021-01-18 | early close | `1200 CT / 1800 UTC` | `2021-holiday-calendars.zip#2021-mlk-day-schedule-compact.xls @2026-08-30T10:03:27Z` | T1 | The cited sheet's `Bitcoin` line prints `12:00 CT` as this date's own final close, so the leg that opened the previous evening at 17:00 CT is clipped there and the crate's trade date is the date that close falls on — the operator's event date. |
| 2021-02-15 | early close | `1200 CT / 1800 UTC` | `2021-holiday-calendars.zip#2021-presidents-day-holiday-schedule-compact.xls @2026-08-30T10:03:27Z` | T1 | The cited sheet's `Bitcoin` line prints `12:00 CT` as this date's own final close, so the leg that opened the previous evening at 17:00 CT is clipped there and the crate's trade date is the date that close falls on — the operator's event date. |
| 2021-04-02 | early close | `0815 CT / 1315 UTC` | `2021-holiday-calendars.zip#2021-good-friday-holiday-schedule-compact.xls @2026-08-30T10:03:27Z` | T1 | The cited sheet's `Bitcoin` line prints `08:15 CT` as this date's own final close, so the leg that opened the previous evening at 17:00 CT is clipped there and the crate's trade date is the date that close falls on — the operator's event date. |
| 2021-05-31 | early close | `1200 CT / 1700 UTC` | `2021-holiday-calendars.zip#2021-memorial-day-holiday-schedule-compact.xls @2026-08-30T10:03:27Z` | T1 | The cited sheet's `Bitcoin` line prints `12:00 CT` as this date's own final close, so the leg that opened the previous evening at 17:00 CT is clipped there and the crate's trade date is the date that close falls on — the operator's event date. |
| 2021-06-19 | unsourced | no CME document covers this date (see `2021-holiday-calendars.zip @2026-08-30T10:03:27Z`) | `2021-holiday-calendars.zip @2026-08-30T10:03:27Z` | T1 | The operator published no Juneteenth schedule for 2019, 2020 or 2021: the year's own consolidated bundle `2021-holiday-calendars.zip @2026-08-30T10:03:27Z` — CME's account of every Globex holiday schedule it published that year — carries no Juneteenth sheet, the archived holiday-calendar.html index pages name none, and a fresh 2018-2027 prefix CDX enumeration finds no `juneteenth` filename before 2022. Silence inside the window would read as audited normal on a date the operator later marks as a holiday, so the row is `Unsourced`, which clips nothing. |
| 2021-07-05 | early close | `1200 CT / 1700 UTC` | `2021-holiday-calendars.zip#2021-independence-day-holiday-schedule-compact.xls @2026-08-30T10:03:27Z` | T1 | The cited sheet's `Bitcoin` line prints `12:00 CT` as this date's own final close, so the leg that opened the previous evening at 17:00 CT is clipped there and the crate's trade date is the date that close falls on — the operator's event date. |
| 2021-09-06 | early close | `1200 CT / 1700 UTC` | `2021-holiday-calendars.zip#2021-labor-day-holiday-schedule-compact.xls @2026-08-30T10:03:27Z` | T1 | The cited sheet's `Bitcoin` line prints `12:00 CT` as this date's own final close, so the leg that opened the previous evening at 17:00 CT is clipped there and the crate's trade date is the date that close falls on — the operator's event date. |
| 2021-11-25 | early close | `1200 CT / 1800 UTC` | `2021-holiday-calendars.zip#2021-thanksgiving-holiday-schedule-compact.xls @2026-08-30T10:03:27Z` | T1 | The cited sheet's `Bitcoin` line prints `12:00 CT` as this date's own final close, so the leg that opened the previous evening at 17:00 CT is clipped there and the crate's trade date is the date that close falls on — the operator's event date. |
| 2021-11-26 | early close | `1245 CT / 1845 UTC` | `2021-holiday-calendars.zip#2021-thanksgiving-holiday-schedule-compact.xls @2026-08-30T10:03:27Z` | T1 | The cited sheet's `Bitcoin` line prints `12:45 CT` as this date's own final close, so the leg that opened the previous evening at 17:00 CT is clipped there and the crate's trade date is the date that close falls on — the operator's event date. |
| 2021-12-24 | closed | `Closed for Christmas` | `2021-holiday-calendars.zip#2021-christmas-holiday-schedule-compact.xls @2026-08-30T10:03:27Z` | T1 | CME prints the closure on this date, so the session whose final close would have fallen here — the leg that opened the previous evening at 17:00 CT — is removed with it, and the crate's trade date is the operator's own event date. |

### Documents
This era's rows cite the ids below — CME Group's own Globex holiday schedules, all at **T1**. The 2020 and 2021 sheets and the three `Unsourced` Juneteenth rows are members of their year's consolidated annual bundle; the 2019 sheets likewise, and 1-2 January 2019 come from the December-2018 supplement CME published before the bundle. Each id resolves to the URL the bytes were read at — an Internet Archive raw replay of CME's own file — with the capture time in UTC, the tier and the sha256. All of them resolve in the research store's `holidays/raw/cme-2019-2021/`, whose `INDEX.md` carries the byte counts and whose `shasum.txt` hashes every workbook. The next `### Documents` table in this file is the 2022-2024 era's.
| Document | Window | Replay or service URL | Capture or retrieval, UTC | Tier | sha256 |
|---|---|---|---|---|---|
| `2019-holiday-calendars.zip @2021-01-26T09:48:37Z` | 2019-01-01 .. 2021-12-31 | <https://web.archive.org/web/20210126094837id_/https://www.cmegroup.com/tools-information/holiday-calendar/files/2019-holiday-calendars.zip> | archive capture 2021-01-26T09:48:37Z | T1 | `1e861e355238903b013c1288f4eb9e8026e6ddd5fc1001dfd2acdbfcc1832e05` |
| `2019-holiday-calendars.zip#globex-trading-schedules/2019-4th-of-july-holiday-schedule-compact.xls @2021-01-26T09:48:37Z` | 2019-01-01 .. 2021-12-31 | <https://web.archive.org/web/20210126094837id_/https://www.cmegroup.com/tools-information/holiday-calendar/files/2019-holiday-calendars.zip> | archive capture 2021-01-26T09:48:37Z | T1 | `8546b2a9e42c92a4bcf2d2e8209906f482467aeb549ed2a02eb95690954ddd16` |
| `2019-holiday-calendars.zip#globex-trading-schedules/2019-christmas-holiday-schedule-compact.xls @2021-01-26T09:48:37Z` | 2019-01-01 .. 2021-12-31 | <https://web.archive.org/web/20210126094837id_/https://www.cmegroup.com/tools-information/holiday-calendar/files/2019-holiday-calendars.zip> | archive capture 2021-01-26T09:48:37Z | T1 | `8ccc652c9e420f20000110bf9e6522ce5ab3a4ac1acd6cca465320da42409dfc` |
| `2019-holiday-calendars.zip#globex-trading-schedules/2019-good-friday-holiday-compact.xls @2021-01-26T09:48:37Z` | 2019-01-01 .. 2021-12-31 | <https://web.archive.org/web/20210126094837id_/https://www.cmegroup.com/tools-information/holiday-calendar/files/2019-holiday-calendars.zip> | archive capture 2021-01-26T09:48:37Z | T1 | `cfaa08b30242010a65af781d3e485f72642a9ed55ecfe8fbad1aa9b3b20b7ad9` |
| `2019-holiday-calendars.zip#globex-trading-schedules/2019-labor-day-holiday-schedule-compact.xls @2021-01-26T09:48:37Z` | 2019-01-01 .. 2021-12-31 | <https://web.archive.org/web/20210126094837id_/https://www.cmegroup.com/tools-information/holiday-calendar/files/2019-holiday-calendars.zip> | archive capture 2021-01-26T09:48:37Z | T1 | `68b3ac93f8b9077bb82b561d8043c0661475de44f5e5b869b900faa33cadf758` |
| `2019-holiday-calendars.zip#globex-trading-schedules/2019-martin-luther-king-holiday-schedule-compact.xls @2021-01-26T09:48:37Z` | 2019-01-01 .. 2021-12-31 | <https://web.archive.org/web/20210126094837id_/https://www.cmegroup.com/tools-information/holiday-calendar/files/2019-holiday-calendars.zip> | archive capture 2021-01-26T09:48:37Z | T1 | `d439d73aa100a9e960a121cac158d2813dceb570f7af80f9cf163e17abedc42a` |
| `2019-holiday-calendars.zip#globex-trading-schedules/2019-memorial-day-holiday-schedule-compact.xls @2021-01-26T09:48:37Z` | 2019-01-01 .. 2021-12-31 | <https://web.archive.org/web/20210126094837id_/https://www.cmegroup.com/tools-information/holiday-calendar/files/2019-holiday-calendars.zip> | archive capture 2021-01-26T09:48:37Z | T1 | `4d55425087a171b10b37a634d9d95cba0ae863e4e2cba282599ef1016b404fbe` |
| `2019-holiday-calendars.zip#globex-trading-schedules/2019-new-years-holiday-schedule-compact.xls @2021-01-26T09:48:37Z` | 2019-01-01 .. 2021-12-31 | <https://web.archive.org/web/20210126094837id_/https://www.cmegroup.com/tools-information/holiday-calendar/files/2019-holiday-calendars.zip> | archive capture 2021-01-26T09:48:37Z | T1 | `6625c29097309fb5c209459438b0e868d88623a350324db9404430cf3f13d7fe` |
| `2019-holiday-calendars.zip#globex-trading-schedules/2019-presidents-day-holiday-schedule-compact.xls @2021-01-26T09:48:37Z` | 2019-01-01 .. 2021-12-31 | <https://web.archive.org/web/20210126094837id_/https://www.cmegroup.com/tools-information/holiday-calendar/files/2019-holiday-calendars.zip> | archive capture 2021-01-26T09:48:37Z | T1 | `cd6c895435ef2e7a14408ee74ab4fba140d462abefe00d2ee0333577253049c3` |
| `2019-holiday-calendars.zip#globex-trading-schedules/2019-thanksgiving-holiday-schedule-compact.xls @2021-01-26T09:48:37Z` | 2019-01-01 .. 2021-12-31 | <https://web.archive.org/web/20210126094837id_/https://www.cmegroup.com/tools-information/holiday-calendar/files/2019-holiday-calendars.zip> | archive capture 2021-01-26T09:48:37Z | T1 | `99ea9b63ef7d756116312e0061275b1ee7cd50ac547a55f09098a4e58973cb31` |
| `2019-new-years-holiday-schedule-compact.xls @2018-01-07T04:13:43Z` | 2019-01-01 .. 2021-12-31 | <https://web.archive.org/web/20180107041343id_/http://www.cmegroup.com/tools-information/holiday-calendar/files/2019-new-years-holiday-schedule-compact.xls> | archive capture 2018-01-07T04:13:43Z | T1 | `2684a5f1b3a9f65802f6911ca6089e2cb68c3cdf2520dfaf3cdcf3105328c188` |
| `2020-holiday-calendars.zip @2026-07-30T11:18:34Z` | 2019-01-01 .. 2021-12-31 | <https://web.archive.org/web/20260730111834id_/https://www.cmegroup.com/tools-information/holiday-calendar/files/2020-holiday-calendars.zip> | archive capture 2026-07-30T11:18:34Z | T1 | `5263a4a5e9076bc0e69c7cd0e3fd9f82dc4d80b66f1b56f14070f08c1d762b59` |
| `2020-holiday-calendars.zip#2020-4th-of-july-holiday-schedule-compact.xls @2026-07-30T11:18:34Z` | 2019-01-01 .. 2021-12-31 | <https://web.archive.org/web/20260730111834id_/https://www.cmegroup.com/tools-information/holiday-calendar/files/2020-holiday-calendars.zip> | archive capture 2026-07-30T11:18:34Z | T1 | `05fee0fd5ec2e9d403ac32ad103612569c8f269299b4bfef931c50c311b8740a` |
| `2020-holiday-calendars.zip#2020-christmas-holiday-schedule-compact.xls @2026-07-30T11:18:34Z` | 2019-01-01 .. 2021-12-31 | <https://web.archive.org/web/20260730111834id_/https://www.cmegroup.com/tools-information/holiday-calendar/files/2020-holiday-calendars.zip> | archive capture 2026-07-30T11:18:34Z | T1 | `d342b6b0a075591255b45b0dc58dfede5bf5cc28077aa8b362591b82224c24ae` |
| `2020-holiday-calendars.zip#2020-good-friday-holiday-compact.xls @2026-07-30T11:18:34Z` | 2019-01-01 .. 2021-12-31 | <https://web.archive.org/web/20260730111834id_/https://www.cmegroup.com/tools-information/holiday-calendar/files/2020-holiday-calendars.zip> | archive capture 2026-07-30T11:18:34Z | T1 | `aa0936e9278904f40bc23abba57c2c379207af49d0cb6971f07289010fe1745e` |
| `2020-holiday-calendars.zip#2020-labor-day-holiday-schedule-compact.xls @2026-07-30T11:18:34Z` | 2019-01-01 .. 2021-12-31 | <https://web.archive.org/web/20260730111834id_/https://www.cmegroup.com/tools-information/holiday-calendar/files/2020-holiday-calendars.zip> | archive capture 2026-07-30T11:18:34Z | T1 | `85acc34db0b86dfe09824c3cfe2f4c7fb21f02ce8ad5593587afe7affa84221f` |
| `2020-holiday-calendars.zip#2020-martin-luther-king-holiday-schedule-compact.xls @2026-07-30T11:18:34Z` | 2019-01-01 .. 2021-12-31 | <https://web.archive.org/web/20260730111834id_/https://www.cmegroup.com/tools-information/holiday-calendar/files/2020-holiday-calendars.zip> | archive capture 2026-07-30T11:18:34Z | T1 | `bacf30ad37e6843f7162a6457e17dec7e0024474e9b8e0d35f9ef12fc9671d25` |
| `2020-holiday-calendars.zip#2020-memorial-day-holiday-schedule-compact.xls @2026-07-30T11:18:34Z` | 2019-01-01 .. 2021-12-31 | <https://web.archive.org/web/20260730111834id_/https://www.cmegroup.com/tools-information/holiday-calendar/files/2020-holiday-calendars.zip> | archive capture 2026-07-30T11:18:34Z | T1 | `f3052963d22aca969eefb81ff8996662fc1b0a17f0bfdf53e8df37b0af5ddc5b` |
| `2020-holiday-calendars.zip#2020-presidents-day-holiday-schedule-compact.xls @2026-07-30T11:18:34Z` | 2019-01-01 .. 2021-12-31 | <https://web.archive.org/web/20260730111834id_/https://www.cmegroup.com/tools-information/holiday-calendar/files/2020-holiday-calendars.zip> | archive capture 2026-07-30T11:18:34Z | T1 | `a42697d72f03fdcc9229a17f99320418fd61a52ceb7bb82636c01cb8d818606d` |
| `2020-holiday-calendars.zip#2020-thanksgiving-holiday-schedule-compact.xls @2026-07-30T11:18:34Z` | 2019-01-01 .. 2021-12-31 | <https://web.archive.org/web/20260730111834id_/https://www.cmegroup.com/tools-information/holiday-calendar/files/2020-holiday-calendars.zip> | archive capture 2026-07-30T11:18:34Z | T1 | `bcf6b5e616b91663abe532953d47dd85af9fb05e26cc4d9fe72329414ef598b4` |
| `2020-holiday-calendars.zip#2021-new-years-holiday-schedule-compact.xls @2026-07-30T11:18:34Z` | 2019-01-01 .. 2021-12-31 | <https://web.archive.org/web/20260730111834id_/https://www.cmegroup.com/tools-information/holiday-calendar/files/2020-holiday-calendars.zip> | archive capture 2026-07-30T11:18:34Z | T1 | `f7b782e1effb08ff84b75daf4a7d59e1cd7d64645c8504cf10d54af9d2199928` |
| `2021-holiday-calendars.zip @2026-08-30T10:03:27Z` | 2019-01-01 .. 2021-12-31 | <https://web.archive.org/web/20260830100327id_/https://www.cmegroup.com/tools-information/holiday-calendar/files/2021-holiday-calendars.zip> | archive capture 2026-08-30T10:03:27Z | T1 | `0ee0860a3a0e035eb9d079419aca3cafcc4256d296fa916647987c396dda8c59` |
| `2021-holiday-calendars.zip#2021-christmas-holiday-schedule-compact.xls @2026-08-30T10:03:27Z` | 2019-01-01 .. 2021-12-31 | <https://web.archive.org/web/20260830100327id_/https://www.cmegroup.com/tools-information/holiday-calendar/files/2021-holiday-calendars.zip> | archive capture 2026-08-30T10:03:27Z | T1 | `e80ee0b766f582091d04f04e1a79595c988848dec269e37d0045d7dc33a68235` |
| `2021-holiday-calendars.zip#2021-good-friday-holiday-schedule-compact.xls @2026-08-30T10:03:27Z` | 2019-01-01 .. 2021-12-31 | <https://web.archive.org/web/20260830100327id_/https://www.cmegroup.com/tools-information/holiday-calendar/files/2021-holiday-calendars.zip> | archive capture 2026-08-30T10:03:27Z | T1 | `2d54d45609bc774e61d65d92d9a3165372950999400dd5b102b7a9319c560356` |
| `2021-holiday-calendars.zip#2021-independence-day-holiday-schedule-compact.xls @2026-08-30T10:03:27Z` | 2019-01-01 .. 2021-12-31 | <https://web.archive.org/web/20260830100327id_/https://www.cmegroup.com/tools-information/holiday-calendar/files/2021-holiday-calendars.zip> | archive capture 2026-08-30T10:03:27Z | T1 | `6444dd5b3513a79c4bce6b9db0746c969e432a3c2cdb5bb918c80ad44bf8fe9b` |
| `2021-holiday-calendars.zip#2021-labor-day-holiday-schedule-compact.xls @2026-08-30T10:03:27Z` | 2019-01-01 .. 2021-12-31 | <https://web.archive.org/web/20260830100327id_/https://www.cmegroup.com/tools-information/holiday-calendar/files/2021-holiday-calendars.zip> | archive capture 2026-08-30T10:03:27Z | T1 | `613623b6d3bb4ac76120d284e198799e4eedf6870119449df762a613ab53888e` |
| `2021-holiday-calendars.zip#2021-memorial-day-holiday-schedule-compact.xls @2026-08-30T10:03:27Z` | 2019-01-01 .. 2021-12-31 | <https://web.archive.org/web/20260830100327id_/https://www.cmegroup.com/tools-information/holiday-calendar/files/2021-holiday-calendars.zip> | archive capture 2026-08-30T10:03:27Z | T1 | `75b06b120bd92f77aa85f8acaca2ec661e06dba98bb64f3863a8d24987475968` |
| `2021-holiday-calendars.zip#2021-mlk-day-schedule-compact.xls @2026-08-30T10:03:27Z` | 2019-01-01 .. 2021-12-31 | <https://web.archive.org/web/20260830100327id_/https://www.cmegroup.com/tools-information/holiday-calendar/files/2021-holiday-calendars.zip> | archive capture 2026-08-30T10:03:27Z | T1 | `83724afe5e7f073bf58d3fd0118269c92d7d1c0c8c18ff716fbebe5e51b7c64c` |
| `2021-holiday-calendars.zip#2021-presidents-day-holiday-schedule-compact.xls @2026-08-30T10:03:27Z` | 2019-01-01 .. 2021-12-31 | <https://web.archive.org/web/20260830100327id_/https://www.cmegroup.com/tools-information/holiday-calendar/files/2021-holiday-calendars.zip> | archive capture 2026-08-30T10:03:27Z | T1 | `7a8d5ce35c639998abf64723de2cc2d649dc015b1aa6cfc3bc326ab5cdfd0c4f` |
| `2021-holiday-calendars.zip#2021-thanksgiving-holiday-schedule-compact.xls @2026-08-30T10:03:27Z` | 2019-01-01 .. 2021-12-31 | <https://web.archive.org/web/20260830100327id_/https://www.cmegroup.com/tools-information/holiday-calendar/files/2021-holiday-calendars.zip> | archive capture 2026-08-30T10:03:27Z | T1 | `df6cdbc4c996109fe5a5829b5e8aaf622c20c6f16f1464dcdd9d5250c4163a47` |
### Gaps and residual risks, 2019-2021
**This era declares the family's third audited window.** The table as a whole carries 74 rows over 3 windows — 2019-01-01..2021-12-31, 2022-01-01..2024-12-31, 2025-01-01..2027-12-31 — and this era's share is **36 rows**: 8 full closures, 25 early closes and 3 `Unsourced` rows. Every row is at T1. Every date before the first window lies outside it, so `holiday_coverage` reports it as unaudited rather than as audited normal.
**Juneteenth 2019, 2020 and 2021 — three `Unsourced` rows.** CME published no Juneteenth schedule in any of the three years. Each row cites that year's own consolidated bundle — `2019-holiday-calendars.zip @2021-01-26T09:48:37Z`, `2020-holiday-calendars.zip @2026-07-30T11:18:34Z` and `2021-holiday-calendars.zip @2026-08-30T10:03:27Z` — whose member lists are CME's own account of every Globex holiday schedule it published that year and which carry no Juneteenth sheet; the four archived `holiday-calendar.html` index pages name none either, and a fresh 2018-2027 prefix CDX enumeration (`raw/cme-2019-2021-fix/cdx/cdx-files-2018-2027.json`, 369 rows, 340 distinct filenames) finds no `juneteenth` filename before 2022. Inside a contiguous window silence is the positive claim that a date was audited normal, which is false for a date the operator later marks as a holiday, so all three ship `Unsourced`, which clips nothing. 2021-06-19 is a **Saturday**: no family has a trade date there and the row changes no answer, and the row is keyed to the operator's own calendar date for the holiday rather than to an observed date CME never states. Closing condition: a CME holiday schedule naming Juneteenth in one of these three years.
**Columbus Day and Veterans Day — six dates with no row.** 2019-10-14, 2019-11-11, 2020-10-12, 2020-11-11, 2021-10-11 and 2021-11-11 lie inside this window and carry no row, so the family's ordinary week stands there. CME published settlement-time and OTC-clearing advisories for these dates — the 2019 ZIP's `settlement-notices/*-settlement-times.pdf` members and, for example, `2021-veterans-day-advisory.pdf` — but never a Globex trading schedule for them. A settlement notice is not session language (LAW-SESSION-NOT-EXPIRY), so no row is keyed to one and the block's `missing` register records the dates as gaps rather than as sourced normality. Closing condition: a CME Globex holiday schedule naming one of these dates.

### 2022

| trade date | kind | instant as printed | document | tier | derived from |
|---|---|---|---|---|---|
| 2022-04-15 | closed | `"Globex Closed"` | `2022-good-friday-holiday-schedule.xls @2022-07-04T06:55:01Z` | T1 | CME prints no session on this date, so the trade date whose final close would have fallen here — the leg that opened the previous evening at 17:00 CT — is removed; the crate keys the venue-local date of the final close, which is the operator's own date here |
| 2022-11-25 | early close | `12:45` | `2022-thanksgiving-holiday-schedule.FINAL-20221122.xls @2022-11-22T06:08:01Z` | T1 | CME prints `12:45` as the final close of the session that opened the previous evening at 17:00 CT, so the row is an early close on the trade date the close falls on, which is the block's own date |
| 2022-12-26 | closed | `"Globex Closed"` | `2022-christmas-holiday-schedule.xls @2022-07-04T06:54:30Z` | T1 | CME prints no session on this date, so the trade date whose final close would have fallen here — the leg that opened the previous evening at 17:00 CT — is removed; the crate keys the venue-local date of the final close, which is the operator's own date here |

### 2023

| trade date | kind | instant as printed | document | tier | derived from |
|---|---|---|---|---|---|
| 2023-01-02 | closed | `"Globex Closed"` | `2023-new-years-holiday-schedule.xls @2022-07-04T06:55:01Z` | T1 | CME prints no session on this date, so the trade date whose final close would have fallen here — the leg that opened the previous evening at 17:00 CT — is removed; the crate keys the venue-local date of the final close, which is the operator's own date here |
| 2023-01-16 | unsourced | `hasEvents:false` — the artifact's own empty event list | `CME-SVC-2023-01-15` | T2 | the operator published no document for this date and the cited artifact is the negative control that returns an empty event list, so the row is `unsourced` and clips nothing rather than reading as audited normal |
| 2023-02-20 | unsourced | `hasEvents:false` — the artifact's own empty event list | `CME-SVC-2023-02-19` | T2 | the operator published no document for this date and the cited artifact is the negative control that returns an empty event list, so the row is `unsourced` and clips nothing rather than reading as audited normal |
| 2023-04-07 | unsourced | `hasEvents:false` — the artifact's own empty event list | `CME-SVC-2023-04-06` | T2 | the operator published no document for this date and the cited artifact is the negative control that returns an empty event list, so the row is `unsourced` and clips nothing rather than reading as audited normal |
| 2023-11-24 | early close | `12:45` | `thanksgiving-day-2023.pdf @2023-12-03T20:59:29Z` | T1 | CME prints `12:45` as the final close of the session that opened the previous evening at 17:00 CT, so the row is an early close on the trade date the close falls on, which is the block's own date |
| 2023-12-25 | closed | `16:00 (PREOPEN)` / `17:00 (OPEN)` — the only clocks in this date's own entry are the evening re-open that carries the next trade date | `christmas-day-2023.pdf @2026-07-19T09:52:48Z` | T1 | CME prints no session on this date, so the trade date whose final close would have fallen here — the leg that opened the previous evening at 17:00 CT — is removed; the crate keys the venue-local date of the final close, which is the operator's own date here |

### 2024

| trade date | kind | instant as printed | document | tier | derived from |
|---|---|---|---|---|---|
| 2024-01-01 | closed | `16:00 (PREOPEN)` / `17:00 (OPEN)` — the only clocks in this date's own entry are the evening re-open that carries the next trade date | `new-years-day-2024.pdf @2026-08-11T16:57:16Z` | T1 | CME prints no session on this date, so the trade date whose final close would have fallen here — the leg that opened the previous evening at 17:00 CT — is removed; the crate keys the venue-local date of the final close, which is the operator's own date here |
| 2024-03-29 | closed | `events []` — the artifact's own empty event list for this date | `CME-SVC-2024-03-28` | T2 | CME prints no session on this date, so the trade date whose final close would have fallen here — the leg that opened the previous evening at 17:00 CT — is removed; the crate keys the venue-local date of the final close, which is the operator's own date here |
| 2024-11-29 | early close | `12:45` | `CME-SVC-2024-11-27` | T2 | CME prints `12:45` as the final close of the session that opened the previous evening at 17:00 CT, so the row is an early close on the trade date the close falls on, which is the block's own date |
| 2024-12-24 | early close | `12:45` | `CME-SVC-2024-12-24` | T2 | CME prints `12:45` as the final close of the session that opened the previous evening at 17:00 CT, so the row is an early close on the trade date the close falls on, which is the block's own date |
| 2024-12-25 | closed | `16:00 (PREOPEN)` / `17:00 (OPEN)` — the only clocks in this date's own entry are the evening re-open that carries the next trade date | `CME-SVC-2024-12-24` | T2 | CME prints no session on this date, so the trade date whose final close would have fallen here — the leg that opened the previous evening at 17:00 CT — is removed; the crate keys the venue-local date of the final close, which is the operator's own date here |
### Documents

This era's rows cite the ids below: CME Group's own published holiday schedules at **T1** (2022 per-asset-class workbooks, 2023 one-pagers, and the 2024 Good Friday and year-end service windows as archived), and responses of CME's own `trading-hours-by-product` service at **T2**. Each id resolves to the URL it was read at — an Internet Archive raw replay for a saved capture, the operator's own endpoint for a live retrieval — with the capture or retrieval time in UTC, the tier and the sha256. All thirty artifacts resolve in the research store's `holidays/raw/cme-2022-2024/` and `holidays/raw/cme-2022-2024-fix/`, whose `INDEX.md` files carry the byte counts.

| Document | Window | Replay or service URL | Capture or retrieval, UTC | Tier | sha256 |
|---|---|---|---|---|---|
| `2022-good-friday-holiday-schedule.xls @2022-07-04T06:55:01Z` | 2022-01-01 .. 2024-12-31 | <https://web.archive.org/web/20220704065501id_/https://www.cmegroup.com/tools-information/holiday-calendar/files/2022-good-friday-holiday-schedule.xls> | archive capture 2022-07-04T06:55:01Z | T1 | `a82936ab14d1b1f7041583123289c4de401c66ace4eea7e90fa9f60c1a3f3b7e` |
| `2022-thanksgiving-holiday-schedule.FINAL-20221122.xls @2022-11-22T06:08:01Z` | 2022-01-01 .. 2024-12-31 | <https://web.archive.org/web/20221122060801id_/https://www.cmegroup.com/tools-information/holiday-calendar/files/2022-thanksgiving-holiday-schedule.xls> | archive capture 2022-11-22T06:08:01Z | T1 | `64341a65350de982a6a05760504132e173611151ae3164c6f9437509a406edcb` |
| `2022-christmas-holiday-schedule.xls @2022-07-04T06:54:30Z` | 2022-01-01 .. 2024-12-31 | <https://web.archive.org/web/20220704065430id_/https://www.cmegroup.com/tools-information/holiday-calendar/files/2022-christmas-holiday-schedule.xls> | archive capture 2022-07-04T06:54:30Z | T1 | `2dd1d531514989845dcb6ce6d767db5dd3f956ac6777ab1cfe36d6843f3762e7` |
| `2023-new-years-holiday-schedule.xls @2022-07-04T06:55:01Z` | 2022-01-01 .. 2024-12-31 | <https://web.archive.org/web/20220704065501id_/https://www.cmegroup.com/tools-information/holiday-calendar/files/2023-new-years-holiday-schedule.xls> | archive capture 2022-07-04T06:55:01Z | T1 | `eefafd1066f406edbe6167ddf8ad13c0697b124a337c0893ce5a5da200c783d3` |
| `CME-SVC-2023-01-15` | 2022-01-01 .. 2024-12-31 | <https://web.archive.org/web/20240708161438id_/https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2023-01-15&toEventDate=2023-01-17&isProtected&_t=1720455278636> | archive capture 2024-07-08T16:14:38Z | T2 | `507fd196a7654ddd916218b2aa24a146eeaca4e1f7cb7daded4e7a9c689cda82` |
| `CME-SVC-2023-02-19` | 2022-01-01 .. 2024-12-31 | <https://web.archive.org/web/20240708161438id_/https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2023-02-19&toEventDate=2023-02-21&isProtected&_t=1720455278640> | archive capture 2024-07-08T16:14:38Z | T2 | `d063238a83e8cb84d4484a86b26cb1d976f1ccfdb91972eecc08dcc0cec49412` |
| `CME-SVC-2023-04-06` | 2022-01-01 .. 2024-12-31 | <https://web.archive.org/web/20240708161438id_/https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2023-04-06&toEventDate=2023-04-08&isProtected&_t=1720455278642> | archive capture 2024-07-08T16:14:38Z | T2 | `0543d5f6d2efd4aa6f4132ce9f5425b9ba9f64de08eccd677b78d6b43c6e9d4f` |
| `thanksgiving-day-2023.pdf @2023-12-03T20:59:29Z` | 2022-01-01 .. 2024-12-31 | <https://web.archive.org/web/20231203205929id_/https://www.cmegroup.com/trading-hours/files/thanksgiving-day-2023.pdf> | archive capture 2023-12-03T20:59:29Z | T1 | `99e187b3f3899e1062d662e148d5978e0cd075b961e6fc79550d4393812307e8` |
| `christmas-day-2023.pdf @2026-07-19T09:52:48Z` | 2022-01-01 .. 2024-12-31 | <https://web.archive.org/web/20260719095248id_/https://www.cmegroup.com/trading-hours/files/christmas-day-2023.pdf> | archive capture 2026-07-19T09:52:48Z | T1 | `edcde0fcf61d3414cee2a332453db861e0e2d7edf81d89a5f379c44b2e72d5cf` |
| `new-years-day-2024.pdf @2026-08-11T16:57:16Z` | 2022-01-01 .. 2024-12-31 | <https://web.archive.org/web/20260811165716id_/https://www.cmegroup.com/trading-hours/files/new-years-day-2024.pdf> | archive capture 2026-08-11T16:57:16Z | T1 | `34e60f8c97623df30e00f0ad8e4eeda20b99001b6c35d64f735828fecec5b9b8` |
| `CME-SVC-2024-03-28` | 2022-01-01 .. 2024-12-31 | <https://web.archive.org/web/20240708161439id_/https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2024-03-28&toEventDate=2024-03-30&isProtected&_t=1720455278672> | archive capture 2024-07-08T16:14:39Z | T2 | `9b41709219e36f56296843fe589e362a7513e9a98132a682a1b956f2e40c593b` |
| `CME-SVC-2024-11-27` | 2022-01-01 .. 2024-12-31 | <https://web.archive.org/web/20240708161439id_/https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2024-11-27&toEventDate=2024-11-29&isProtected&_t=1720455278685> | archive capture 2024-07-08T16:14:39Z | T2 | `f6a15f26991d25f8c6821fa6c967d0e76b2ad3f775bed62a2a001ec2fd7e389a` |
| `CME-SVC-2024-12-24` | 2022-01-01 .. 2024-12-31 | <https://web.archive.org/web/20241220155340id_/https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2024-12-24&toEventDate=2024-12-26&isProtected&_t=1734710019537> | archive capture 2024-12-20T15:53:40Z | T2 | `183c85160e31d53fde30c422048b8e778937d8866f3ea1c45e2dd570ea9db1c8` |
### Gaps and residual risks, 2022-2024

**The three 2023 dates the operator published nothing for — 2023-01-16, 2023-02-20 and 2023-04-07.** They ship `Unsourced`, cited to the T2 captures `CME-SVC-2023-01-15`, `CME-SVC-2023-02-19` and `CME-SVC-2023-04-06`, each of which is the operator's own machine channel read as bytes and returns an empty event list for its window: 2023 Dr. Martin Luther King, Jr. Day (Monday 16 January 2023), 2023 Presidents Day (Monday 20 February 2023) and 2023 Good Friday (Friday 7 April 2023). The block's `missing` register records the channels searched — `holiday-calendar/files/*.xls`, of which only a compact MGEX/DME workbook exists for MLK, and `trading-hours/files/<holiday>-2023.pdf`, which 404s — so the gap is "not worked up", not "no source exists". `Unsourced` clips nothing, so each of these dates still resolves to the family's ordinary week. Closing condition: a CME holiday schedule for 2023 covering this date at T1, or a T2 window that carries its events. The day after each of the three is a second, unmodelled gap: 2023-01-17 and 2023-02-21 (and 2023-04-10 for `globex_grains`) may have lost their prior-evening leg the way 2024-01-02 and 2023-12-26 did, and no artifact this crate read states whether they did, so they ship no row and the family's ordinary week stands there.

**The 2024-04-01 re-open.** The Good Friday window `CME-SVC-2024-03-28` ends at 2024-03-30 and prints empty event lists for both 2024-03-29 and 2024-03-30, so CME published nothing this crate read for the Sunday 2024-03-31 evening leg that would carry trade date 2024-04-01. Inside a contiguous window silence would read as audited normal, so 2024-04-01 ships **no row** and is a declared gap rather than an audited one. Closing condition: a CME service response covering 2024-03-31, or a T1 statement of that Sunday's re-open. For `globex_grains` the same silence is what withholds a possible late open on 2024-04-01.

**Normal-week notes that ship no row.** Where a printed token falls outside the family's ordinary week but moves no boundary a scalar holiday row can state, the date ships nothing and the token is recorded here: the 2022 New Year's workbooks print the `Nikkei/TOPIX BTIC` `Close 00:00` on 2022-01-01, a BTIC close for the next trade date; the 2023 and 2024 Independence Day and New Year grain entries print the next trade date's `06:00 (PREOPEN)`, which is the no-evening-leg marker the six `globex_grains` late opens are read from — those rows are in that family's own table and evidence file and this family has none; and the 2024-12-31 grain entry points at `2025-01-02 06:00 preopen`, outside this window, so no row ships and the 2025-2027 table must state that trade date.

### 2025

Trade dates 2025-01-01 .. 2025-12-31 fall entirely in the five-day
17:00–16:00 CT era, which closes over the weekend, so a `closed` row removes the
complete trading day including the session that opened the previous evening.

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2025-01-01 | closed | `16:00 preopen; 17:00 open` — CT; no final close on this date | `CME-SVC-2024-12-31` | T2 | eventDate 2025-01-01, CME trade date 2025-01-02; eventDate 2024-12-31 prints `16:00 closed` for trade date 2024-12-31 with no evening re-open |
| 2025-01-21 | replacement blocks | `16:00 preopen; 17:00 open` on 2025-01-19 and again on 2025-01-20, then `16:00 closed` on 2025-01-21 — CT | `CME-SVC-2025-01-19` | T2 | eventDate 2025-01-19 and eventDate 2025-01-20 each print their own `16:00 preopen` and `17:00 open` with CME trade date 2025-01-21, and eventDate 2025-01-21 prints `16:00 closed` for 2025-01-21; the holiday 2025-01-20 has no final close of its own, so the whole span from the Sunday 17:00 CT open through the Tuesday 16:00 CT close is this trade date |
| 2025-02-18 | replacement blocks | `16:00 preopen; 17:00 open` on 2025-02-16 and again on 2025-02-17, then `16:00 closed` on 2025-02-18 — CT | `CME-SVC-2025-02-16` | T2 | eventDate 2025-02-16 and eventDate 2025-02-17 each print their own `16:00 preopen` and `17:00 open` with CME trade date 2025-02-18, and eventDate 2025-02-18 prints `16:00 closed` for 2025-02-18; the holiday 2025-02-17 has no final close of its own, so the span from the Sunday 17:00 CT open through the Tuesday 16:00 CT close is this trade date |
| 2025-04-18 | closed | `no events published` | `CME-SVC-2025-04-17` | T2 | eventDate 2025-04-18; eventDate 2025-04-17 prints `16:00 closed` for trade date 2025-04-17 with no evening re-open |
| 2025-05-27 | replacement blocks | `16:00 preopen; 17:00 open` on 2025-05-25 and again on 2025-05-26, then `16:00 closed` on 2025-05-27 — CT | `CME-SVC-2025-05-25` | T2 | eventDate 2025-05-25 and eventDate 2025-05-26 each print their own `16:00 preopen` and `17:00 open` with CME trade date 2025-05-27, and eventDate 2025-05-27 prints `16:00 closed` for 2025-05-27; the holiday 2025-05-26 has no final close of its own, so the span from the Sunday 17:00 CT open through the Tuesday 16:00 CT close is this trade date |
| 2025-06-20 | replacement blocks | `16:45 preopen; 17:00 open` on 2025-06-18, `16:00 preopen; 17:00 open` on 2025-06-19, then `16:00 closed` on 2025-06-20 — CT | `CME-SVC-2025-06-18` | T2 | eventDate 2025-06-18 prints `16:00 closed` for trade date 2025-06-18 and then its own `16:45 preopen` and `17:00 open` for CME trade date 2025-06-20; eventDate 2025-06-19 prints `16:00 preopen` and `17:00 open` for 2025-06-20, and eventDate 2025-06-20 prints `16:00 closed` for 2025-06-20. The holiday 2025-06-19 has no final close of its own, so the span opens on the Wednesday evening, whose queue is the family's ordinary weekday 16:45 CT |
| 2025-07-04 | early close | `12:00 closed` — 12:00 CT | `CME-SVC-2025-07-03` | T2 | eventDate 2025-07-04, CME trade date 2025-07-04 |
| 2025-09-02 | replacement blocks | `16:00 preopen; 17:00 open` on 2025-08-31 and again on 2025-09-01, then `16:00 closed` on 2025-09-02 — CT | `CME-SVC-2025-08-31` | T2 | eventDate 2025-08-31 and eventDate 2025-09-01 each print their own `16:00 preopen` and `17:00 open` with CME trade date 2025-09-02, and eventDate 2025-09-02 prints `16:00 closed` for 2025-09-02; the holiday 2025-09-01 has no final close of its own, so the span from the Sunday 17:00 CT open through the Tuesday 16:00 CT close is this trade date |
| 2025-11-28 | replacement blocks | `16:45 preopen; 17:00 open` on 2025-11-26, `16:00 preopen; 17:00 open` on 2025-11-27, then `07:00 preopen; 07:30 open; 13:45 closed` on 2025-11-28 — CT | `CME-SVC-2025-11-26` | T2 | eventDate 2025-11-26 prints `16:00 closed` for trade date 2025-11-26 and then its own `16:45 preopen` and `17:00 open` for CME trade date 2025-11-28; eventDate 2025-11-27 prints `16:00 preopen` and `17:00 open` for 2025-11-28; eventDate 2025-11-28 prints `07:00 preopen`, `07:30 open` and `13:45 closed`, all for 2025-11-28. The Thursday holiday publishes no final close, and the finalised publication adds the 07:00-07:30 CT pause the pre-holiday capture lacks |
| 2025-11-29 | closed | `no events published` | `CME-SVC-2025-11-26-SAT` | T2 | eventDate 2025-11-29; the Saturday-extended window publishes an empty schedule for all ten products, and CME's 2025 Globex table states the period as "27 - 29 November 2025" |
| 2025-12-24 | early close | `12:45 closed` — 12:45 CT, no evening re-open | `CME-SVC-2025-12-24` | T2 | eventDate 2025-12-24, CME trade date 2025-12-24 |
| 2025-12-25 | closed | `16:00 preopen; 17:00 open` — CT; no final close on this date | `CME-SVC-2025-12-24` | T2 | eventDate 2025-12-25, CME trade date 2025-12-26 |

**Interpretive steps, 2025.**

- Saturday 2025-11-29 is a sourced closure — the live service publishes a
  2025-11-29 schedule for all ten headline products and every one of them is
  empty — and it ships a `closed` row even though the five-day week has no
  Saturday trade date for it to remove. One audited operator closure ships in
  every family that routes to the venue, so the D17 venue intersection is
  computed from one uniform input rather than from eight family judgements.
- The six Monday and Thursday holidays 2025-01-20, 2025-02-17, 2025-05-26,
  2025-06-19, 2025-09-01 and 2025-11-27 carry `modified` `[N3]` records: they key
  **no row of their own**, and the span each merges ships as the
  `ReplacementBlocks` row on the following business day — 2025-01-21,
  2025-02-18, 2025-05-27, 2025-06-20, 2025-09-02 and 2025-11-28 — which is the
  trade date every one of the operator's own events on that span prints. See the
  trade-date merge below.
- Columbus Day and Veterans Day appear nowhere in CME's Globex holiday list;
  Globex trades a normal session on both. Inside the coverage window they
  therefore read as normal, which is a statement, not an omission.
- 2025-04-17, 2025-07-03 and 2025-12-31 carry `modified` `[N6]` records — a
  normal 16:00 CT final close with no evening re-open. The missing leg is
  deleted by the neighbouring `closed` row, so no row of their own.

**The trade-date merge, 2025.** On the six holidays above CME printed a 16:00 CT
*pre-open* where a normal day prints a 16:00 CT final close, with every event
carrying the following business date. No `[N6]` predecessor exists on the
preceding evening, so the previous 17:00 CT open was normal: matching ran from
that open through to 16:00 CT, the queue moved to 16:00 CT, and matching resumed
at 17:00 CT — and what changed is the trade-date label the operator attached to
the whole span. The nine trade dates that own such a span are read from the
events themselves, never inferred: the crate's row is keyed to the date CME
prints beside the events, and the holiday's own date keeps no row because the
operator gives it no final close and therefore no trade date. On 2025-11-28 the
finalised publication `CME-SVC-2025-11-26` adds a `07:00 preopen; 07:30 open`
pause ahead of the 13:45 CT close, so that day ships six blocks: the `-1` leg
ends at 07:00 CT, the 07:00-07:30 CT queue matches nothing, and the final leg
runs 07:30-13:45 CT.

**Gaps, 2025.**

- **Order entry — the days that still carry no block row.** On the full-closure
  dates the pre-open starts at 16:00 CT rather than the normal 16:45 CT. The six
  merged spans above now state their own 16:00 CT queues as `order_entry`
  blocks; what remains unstated is that deviation on the closure dates
  2025-01-01, 2025-04-18, 2025-07-04, 2025-12-24 and 2025-12-25, whose rows are
  scalar. It changes no `is_open` answer, and the family's five-day Pre-Open
  onset is undated in the normal week in any case (#123). Closing condition: a
  block row on the affected trade date, or an order-entry boundary on
  `DayPolicy`.
- **Residual risk — the pre-finalisation captures.** 2025-01-01, 2025-01-20,
  2025-02-17, 2025-04-18, 2025-05-26, 2025-06-19, 2025-07-04 and 2025-09-01 rest
  on the single archive capture 2024-12-20T15:53:40Z of the service, which is a
  pre-holiday publication; all eight of those windows now key or bound a row —
  2025-01-01, 2025-04-18 and 2025-07-04 ship rows of their own, and the five
  merged dates 2025-01-21, 2025-02-18, 2025-05-27, 2025-06-20 and 2025-09-02
  ship the spans their holidays merge into — so the risk is live rather than
  historical. CME states on the same page that the schedule is subject to change
  and is usually finalised about two weeks before. The service no longer answers
  for those windows — re-probed 2026-09-12, they fall past its retention edge
  and return the products with empty schedules. Closing condition: any later
  archived call of the service over those windows, or a CME notice restating the
  finalised Globex hours. 2025-11-27/28 and 2025-12-24/25 are **not** exposed to
  this: both rest on post-holiday captures, and 2025-11-28's own six-block day
  comes from the finalised `CME-SVC-2025-11-26` capture of 2026-01-29.
- **Tier.** No T1 per-asset-class rendering of these hours exists. The operator
  page renders the table client-side from this same service, so the archived
  HTML carries no rows.

### 2026

Trade dates through 2026-05-25 are the five-day era; from trade date 2026-05-30
the family is 24/7 and assigns a block to the following open business date, so
from 2026-06-19 onward a `closed` row deletes no trading — it makes that roll
skip the date.

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2026-01-01 | closed | `16:00 preopen; 17:00 open` — CT; no final close on this date | `CME-SVC-2025-12-31` | T2 | eventDate 2026-01-01, CME trade date 2026-01-02; eventDate 2025-12-31 prints `16:00 closed` for trade date 2025-12-31 with no evening re-open |
| 2026-01-20 | replacement blocks | `16:00 preopen; 17:00 open` on 2026-01-18 and again on 2026-01-19, then `16:00 closed` on 2026-01-20 — CT | `CME-SVC-2026-01-18` | T2 | eventDate 2026-01-18 and eventDate 2026-01-19 each print their own `16:00 preopen` and `17:00 open` with CME trade date 2026-01-20, and eventDate 2026-01-20 prints `16:00 closed` for 2026-01-20; the holiday 2026-01-19 has no final close of its own, so the span from the Sunday 17:00 CT open through the Tuesday 16:00 CT close is this trade date |
| 2026-02-17 | replacement blocks | `16:00 preopen; 17:00 open` on 2026-02-15 and again on 2026-02-16, then `16:00 closed` on 2026-02-17 — CT | `CME-SVC-2026-02-15` | T2 | eventDate 2026-02-15 and eventDate 2026-02-16 each print their own `16:00 preopen` and `17:00 open` with CME trade date 2026-02-17, and eventDate 2026-02-17 prints `16:00 closed` for 2026-02-17; the holiday 2026-02-16 has no final close of its own, so the span from the Sunday 17:00 CT open through the Tuesday 16:00 CT close is this trade date |
| 2026-04-03 | early close | `10:15 closed` — 10:15 CT | `CME-SVC-2026-04-01` | T2 | eventDate 2026-04-03, CME trade date 2026-04-03 |
| 2026-05-26 | replacement blocks | `16:00 preopen; 17:00 open` on 2026-05-24 and again on 2026-05-25, then `16:00 closed` on 2026-05-26 — CT | `CME-SVC-2026-05-24` | T2 | eventDate 2026-05-24 and eventDate 2026-05-25 each print their own `16:00 preopen` and `17:00 open` with CME trade date 2026-05-26, and eventDate 2026-05-26 prints `16:00 closed` for 2026-05-26; the holiday 2026-05-25 has no final close of its own, so the span from the Sunday 17:00 CT open through the Tuesday 16:00 CT close is this trade date |
| 2026-06-19 | closed | `16:00 closed /TD 2026-06-22; 16:01 preopen /TD 2026-06-22; 16:02 open /TD 2026-06-22` — CT | `CME-SVC-2026-06-18` | T2 | eventDate 2026-06-19, CME trade date 2026-06-22 |
| 2026-07-03 | closed | `16:00 closed /TD 2026-07-06; 16:01 preopen /TD 2026-07-06; 16:02 open /TD 2026-07-06` — CT | `CME-SVC-2026-07-03` | T2 | eventDate 2026-07-03, CME trade date 2026-07-06 |
| 2026-09-07 | closed | `16:01 preopen; 16:02 open` — CT; the 16:00 CT final close is omitted | `CME-SVC-2026-09-06` | T2 | eventDate 2026-09-07, CME trade date 2026-09-08 |
| 2026-11-26 | closed | `16:01 preopen; 16:02 open` — CT; the 16:00 CT final close is omitted | `CME-SVC-2026-11-25` | T2 | eventDate 2026-11-26, CME trade date 2026-11-27 |
| 2026-12-25 | closed | `16:00 closed /TD 2026-12-28; 16:01 preopen /TD 2026-12-28; 16:02 open /TD 2026-12-28` — CT | `CME-SVC-2026-12-24` | T2 | eventDate 2026-12-25, CME trade date 2026-12-28 |

**Interpretive steps, 2026.**

- The reference week 2026-10-18 .. 2026-10-24, pulled from the same service, is
  what "normal" means here: an ordinary Friday prints
  `16:00 closed /TD 2026-10-23; 16:01 preopen /TD 2026-10-26; 16:02 open /TD 2026-10-26`.
  On the six 24/7-era holiday Fridays the 16:00 CT close instead carries the
  following Monday, which is how the holiday's own trade date is known to be
  absent even though the event times are unchanged.
- 2026-06-17 and 2026-07-02 are `normal` and `modified` `[N18]` respectively:
  both settle their own trade date and only the re-open rolls, so neither keys a
  row. The roll they describe is produced by the 2026-06-19 and 2026-07-03
  `closed` rows.
- The Saturday sessions CME publishes on 2026-06-20 and 2026-07-04
  — `05:00 open; 17:00 closed` for trade date the following Monday — belong to
  the 24-hour *non*-cryptocurrency groups. Cryptocurrency trades its ordinary
  24/7 Saturday grid on both, so neither is a row or a gap for this family.
- 2026-11-27 and 2026-12-24, early closes for most other families, carry no
  cryptocurrency record: the 24/7 grid runs normally on both.

**Gaps, 2026.**

- **Pre-Open served as executable — corrected 2026-09-26 UTC; no longer a gap.**
  Until this change the crate carried both Pre-Open queues (weekday 16:01-16:02 CT,
  Saturday 03:45-04:00 CT) in `extended`, so `is_open` answered true and
  `session_state` answered `OpenExtended` for 60 seconds every day and 15 minutes
  every Saturday in windows this file's own vocabulary calls *"Order Entry,
  modification, and cancel are allowed. **No order matching.**"* The narrative above
  already stated the correct geometry — matching maintenance 16:00-16:02 CT with the
  Pre-Open from 16:01 — so the code was the outlier, not the sources. Both queues are
  now `order_entry` rules, matching resumes at the 16:02 and 04:00 `open`s, and the
  `ECBTC` profile reads the same two queues the same way. The corrected queues are
  still *refused* rather than served while the `#93` declaration below stands.
- **Order entry — the day that still carries no block row**, as 2025:
  2026-01-01. The three merged spans 2026-01-20, 2026-02-17 and 2026-05-26 state
  their own 16:00 CT queues as `order_entry` blocks, so the deviation is stated
  for them.
- **Executable hours — the 60-second maintenance minute.** On the 24/7-era
  Monday and Thursday holidays 2026-09-07 and 2026-11-26 CME omits the 16:00 CT
  final close outright, so its ordinary 16:00–16:02 CT maintenance window is
  absent that day. The crate's normal week keeps a 16:00–16:02 CT non-matching
  window — the 16:01–16:02 CT order-entry queue sits inside it — so `is_open`
  answers false for the 16:00–16:01 CT minute the operator published as
  continuous. The scalar vocabulary cannot delete a gap; recorded, not modelled,
  with a replacement row on the affected trade date the closing condition. This
  is the half of #93 that survives the merged trade dates. The Friday
  holidays are not affected — there the 16:00 CT close *is* printed.
- **Tier**, as 2025.

**The trade-date merge, 2026.** The three five-day-era holidays 2026-01-19,
2026-02-16 and 2026-05-25 merge exactly as the six 2025 dates do, and the spans
they merge ship as the rows 2026-01-20, 2026-02-17 and 2026-05-26 — the trade
dates the operator's own events print. From trade date 2026-05-30 the family is
24/7 and the mechanism changes: see the interpretive steps above.

### 2027

The whole year is the 24/7 era.

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2027-01-01 | closed | `16:00 closed /TD 2027-01-04; 16:01 preopen /TD 2027-01-04; 16:02 open /TD 2027-01-04` — CT | `CME-SVC-2026-12-31` | T2 | eventDate 2027-01-01, CME trade date 2027-01-04 |
| 2027-01-18 | closed | `16:01 preopen; 16:02 open` — CT; the 16:00 CT final close is omitted | `CME-SVC-2027-01-17` | T2 | eventDate 2027-01-18, CME trade date 2027-01-19 |
| 2027-02-15 | closed | `16:01 preopen; 16:02 open` — CT; the 16:00 CT final close is omitted | `CME-SVC-2027-02-14` | T2 | eventDate 2027-02-15, CME trade date 2027-02-16 |
| 2027-03-26 | closed | `16:00 closed /TD 2027-03-29; 16:01 preopen /TD 2027-03-29; 16:02 open /TD 2027-03-29` — CT | `CME-SVC-2027-03-25` | T2 | eventDate 2027-03-26, CME trade date 2027-03-29 |
| 2027-05-31 | closed | `16:01 preopen; 16:02 open` — CT; the 16:00 CT final close is omitted | `CME-SVC-2027-05-30` | T2 | eventDate 2027-05-31, CME trade date 2027-06-01 |
| 2027-06-18 | closed | `16:00 closed /TD 2027-06-21; 16:01 preopen /TD 2027-06-21; 16:02 open /TD 2027-06-21` — CT | `CME-SVC-2027-06-17` | T2 | eventDate 2027-06-18, CME trade date 2027-06-21 |
| 2027-07-05 | closed | `16:01 preopen; 16:02 open` — CT; the 16:00 CT final close is omitted | `CME-SVC-2027-07-04` | T2 | eventDate 2027-07-05, CME trade date 2027-07-06 |
| 2027-09-06 | closed | `16:01 preopen; 16:02 open` — CT; the 16:00 CT final close is omitted | `CME-SVC-2027-09-05` | T2 | eventDate 2027-09-06, CME trade date 2027-09-07 |
| 2027-11-25 | closed | `16:01 preopen; 16:02 open` — CT; the 16:00 CT final close is omitted | `CME-SVC-2027-11-24` | T2 | eventDate 2027-11-25, CME trade date 2027-11-26 |
| 2027-12-24 | closed | `16:00 closed /TD 2027-12-23; 16:01 preopen /TD 2027-12-27; 16:02 open /TD 2027-12-27` — CT, on eventDate 2027-12-23 | `CME-SVC-2027-12-22` | T2 | eventDate 2027-12-23, CME trade date 2027-12-27 — the re-open skips 2027-12-24; the same document prints `no events published` for eventDate 2027-12-24 for the nine other product groups, with no cryptocurrency record because the 24/7 grid never stops |

**Interpretive steps, 2027.**

- CME's own 2027 Christmas holiday date is Thursday 2027-12-23, with Globex shut
  on Friday 2027-12-24 and 25 December falling on a Saturday. Cryptocurrency
  settles its own trade date on 2027-12-23 — `16:00 closed /TD 2027-12-23` — so
  that date keys **no** row. The row is 2027-12-24, and it is keyed from the
  trade date the 16:01/16:02 re-open carries: 2027-12-27, skipping the Friday.
  This is the one row in the table whose *only* evidence is the trade date
  printed beside a neighbouring day's events, which is precisely what the
  trade-date key is for.
- 2028-01-01 is a Saturday and prints the ordinary 24/7 Saturday grid
  `02:00 closed; 03:45 preopen; 04:00 open` for trade date 2028-01-03. It is
  outside the coverage window and is not a row.
- 2027-12-31 carries no cryptocurrency record: the 24/7 grid runs normally.

**Gaps, 2027.**

- **Executable hours — the 60-second maintenance minute**, as 2026, on
  2027-01-18, 2027-02-15, 2027-05-31, 2027-07-05, 2027-09-06 and 2027-11-25.
- **Sourced but unrepresentable — the Saturday session 2027-06-19.** CME
  publishes `05:00 open; 17:00 closed` for trade date 2027-06-21 for the 24-hour
  groups after the Juneteenth Friday. Cryptocurrency is not among them and runs
  its normal Saturday grid, so this is recorded only so a reader does not look
  for a row.
- **Tier**, as 2025.
- **Forward-dated rows.** Every 2027 row and the 2026 rows from 2026-09-07 on
  are encoded ahead of their effective day on the operator's published future,
  which LAW-NO-FABRICATED-DATES permits for an unconditional, fully sourced
  date. They must be confirmed against the operator before each day
  (LAW-WATCH); CME states the schedule is subject to change and is usually
  finalised about two weeks before.

### Documents

Every id below resolves to the CME Group trading-hours service, tier T2.
`[THBP-A]` expands to
`https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true`
and `WA:<ts>id_/` to `https://web.archive.org/web/<ts>id_/`. `cmegroup.com`
returns HTTP 403 to the retrieving machine, so live calls were read through the
public reader `https://r.jina.ai/<url>` and history through the Wayback Machine
with `id_` replay. Bytes and manifests: `holidays/raw/cme-2025-2027*` in the
research store.

| Document | Window | Replay or service URL | Capture or retrieval, UTC | Tier | sha256 |
|---|---|---|---|---|---|
| `CME-SVC-2024-12-31` | 2024-12-31 .. 2025-01-02 | <https://web.archive.org/web/20241220155340id_/https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2024-12-31&toEventDate=2025-01-02> | archive capture 2024-12-20T15:53:40Z | T2 | `375c70eecd19c5c6204ecb408d1b3210a9da4c9a03b85ef1c7dbbcde1397ab63` |
| `CME-SVC-2025-01-19` | 2025-01-19 .. 2025-01-21 | <https://web.archive.org/web/20241220155340id_/https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2025-01-19&toEventDate=2025-01-21> | archive capture 2024-12-20T15:53:40Z | T2 | `4f2ab56af14e7b3a6978e7fa6db8e2cfc63a82d06428cd88844f0e5bcf534f40` |
| `CME-SVC-2025-02-16` | 2025-02-16 .. 2025-02-18 | <https://web.archive.org/web/20241220155340id_/https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2025-02-16&toEventDate=2025-02-18> | archive capture 2024-12-20T15:53:40Z | T2 | `5bec2ca6b4999a534e4d9818035aaa18ec8626b6c912cf7e3d2c57015536f2fa` |
| `CME-SVC-2025-04-17` | 2025-04-17 .. 2025-04-19 | <https://web.archive.org/web/20241220155340id_/https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2025-04-17&toEventDate=2025-04-19> | archive capture 2024-12-20T15:53:40Z | T2 | `865a1d4f08102e00151bd87ab2b8e8a7720e9203a17aaaba24627ade3ed26e74` |
| `CME-SVC-2025-05-25` | 2025-05-25 .. 2025-05-27 | <https://web.archive.org/web/20241220155340id_/https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2025-05-25&toEventDate=2025-05-27> | archive capture 2024-12-20T15:53:40Z | T2 | `5f42869879c826f5949b79236aabb3d26d74e7565d92d7cc5e8784c63973210b` |
| `CME-SVC-2025-06-18` | 2025-06-18 .. 2025-06-20 | <https://web.archive.org/web/20241220155340id_/https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2025-06-18&toEventDate=2025-06-20> | archive capture 2024-12-20T15:53:40Z | T2 | `a572706907175776255261103b393493ebdf5a8106ec5374d129145bdf89105e` |
| `CME-SVC-2025-07-03` | 2025-07-03 .. 2025-07-05 | <https://web.archive.org/web/20241220155340id_/https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2025-07-03&toEventDate=2025-07-05> | archive capture 2024-12-20T15:53:40Z | T2 | `b80cd4bfed0ae72865bfacc1936e107eb8febfcc94b37fcce1d05505c659147b` |
| `CME-SVC-2025-08-31` | 2025-08-31 .. 2025-09-02 | <https://web.archive.org/web/20241220155340id_/https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2025-08-31&toEventDate=2025-09-02> | archive capture 2024-12-20T15:53:40Z | T2 | `e075762ed34a86048d94900e10edba10d95b3d766052908ffbb5133f6b64bab0` |
| `CME-SVC-2025-11-26` | 2025-11-26 .. 2025-11-28 | <https://web.archive.org/web/20260129012309id_/https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2025-11-26&toEventDate=2025-11-28> | archive capture 2026-01-29T01:23:09Z | T2 | `6c4c598791058dd9a11aff0ddb072c761a436c6d1054b891def74c6935f020f1` |
| `CME-SVC-2025-11-26-SAT` | 2025-11-26 .. 2025-11-29 | <https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2025-11-26&toEventDate=2025-11-29> | live retrieval 2026-09-12T08:55:12Z | T2 | `2e9f34f20085de3ccbdff1dc29cb7463bcff93713ef0c550740d6f15e0635ab7` |
| `CME-SVC-2025-12-24` | 2025-12-24 .. 2025-12-26 | <https://web.archive.org/web/20260129012159id_/https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2025-12-24&toEventDate=2025-12-26> | archive capture 2026-01-29T01:21:59Z | T2 | `322a2be989b67f5f4cc0ec12fd63a393383d574badd4aacc87a0c9637533d386` |
| `CME-SVC-2025-12-31` | 2025-12-31 .. 2026-01-02 | <https://web.archive.org/web/20260619114105id_/https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2025-12-31&toEventDate=2026-01-02> | archive capture 2026-06-19T11:41:05Z | T2 | `0ed61f8328eda4746265cc8e197f10cd53aec06c2b393927bab27c913993d314` |
| `CME-SVC-2026-01-18` | 2026-01-18 .. 2026-01-20 | <https://web.archive.org/web/20260619114105id_/https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2026-01-18&toEventDate=2026-01-20> | archive capture 2026-06-19T11:41:05Z | T2 | `5e3ff08bdc7d07474b96b8dc8c18ed0d5e48d12dc4bcad81a5f68820cb2aa89e` |
| `CME-SVC-2026-02-15` | 2026-02-15 .. 2026-02-17 | <https://web.archive.org/web/20260619114105id_/https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2026-02-15&toEventDate=2026-02-17> | archive capture 2026-06-19T11:41:05Z | T2 | `5dd507dd959d0029e838ec88b1bdb63c32444ea36a121de002606f5d7b206e2f` |
| `CME-SVC-2026-04-01` | 2026-04-01 .. 2026-04-03 | <https://web.archive.org/web/20260619114118id_/https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2026-04-01&toEventDate=2026-04-03> | archive capture 2026-06-19T11:41:18Z | T2 | `54bcc271e9ba9737a99a2fe608e658de0c657075284d050fbfec4fe1aee2a2a5` |
| `CME-SVC-2026-05-24` | 2026-05-24 .. 2026-05-26 | <https://web.archive.org/web/20260619114105id_/https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2026-05-24&toEventDate=2026-05-26> | archive capture 2026-06-19T11:41:05Z | T2 | `f7e30d204ce2cbe08e5f486ded6518f623369159f3a36161288a4708288314da` |
| `CME-SVC-2026-06-18` | 2026-06-18 .. 2026-06-20 | <https://web.archive.org/web/20260619113404id_/https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2026-06-18&toEventDate=2026-06-20> | archive capture 2026-06-19T11:34:04Z | T2 | `97fd5da371309f4486a8fb49ff2105c6c1c2396939ab7c76f1a2a1097b6f015c` |
| `CME-SVC-2026-07-03` | 2026-07-03 .. 2026-07-05 | <https://web.archive.org/web/20260619114108id_/https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2026-07-03&toEventDate=2026-07-05> | archive capture 2026-06-19T11:41:08Z | T2 | `4b89a026358e998277f9c1ff7e095e5d4e625cdc45115fd141dc92201833155b` |
| `CME-SVC-2026-09-06` | 2026-09-06 .. 2026-09-08 | <https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2026-09-06&toEventDate=2026-09-08> | live retrieval 2026-09-12T04:30Z | T2 | `01fb78ffaac10eac466fed53674214222f05aed518b9d93a4b42cf8957147bca` |
| `CME-SVC-2026-11-25` | 2026-11-25 .. 2026-11-27 | <https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2026-11-25&toEventDate=2026-11-27> | live retrieval 2026-09-12T04:30Z | T2 | `e1f35a5623b3c5d15e7468b2cb4119e587411a9714f920605dab11bf688756d1` |
| `CME-SVC-2026-12-24` | 2026-12-24 .. 2026-12-26 | <https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2026-12-24&toEventDate=2026-12-26> | live retrieval 2026-09-12T04:30Z | T2 | `bdc1fe831adb794bcf8aeb7e99baf6af2009d1ff9969d0a48b18b2ebc2e1e829` |
| `CME-SVC-2026-12-31` | 2026-12-31 .. 2027-01-02 | <https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2026-12-31&toEventDate=2027-01-02> | live retrieval 2026-09-12T04:30Z | T2 | `7162652821c16f1bd05e3ec533bd5b82af03833c7186a64c7734b0b650364dcd` |
| `CME-SVC-2027-01-17` | 2027-01-17 .. 2027-01-19 | <https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2027-01-17&toEventDate=2027-01-19> | live retrieval 2026-09-12T04:30Z | T2 | `7155c4b7ee8b299b3033eb3daf002b6ceecf0fbd53f6f98a7036048022275743` |
| `CME-SVC-2027-02-14` | 2027-02-14 .. 2027-02-16 | <https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2027-02-14&toEventDate=2027-02-16> | live retrieval 2026-09-12T04:30Z | T2 | `41f5aa8cde3879f8b10490386c134a294a0f1509edde2022a22ec3ffcaed1183` |
| `CME-SVC-2027-03-25` | 2027-03-25 .. 2027-03-27 | <https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2027-03-25&toEventDate=2027-03-27> | live retrieval 2026-09-12T04:30Z | T2 | `9bd7225d440e00139f30892f3914c9b38beb8bf29d4272039b6cd8f2de926880` |
| `CME-SVC-2027-05-30` | 2027-05-30 .. 2027-06-01 | <https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2027-05-30&toEventDate=2027-06-01> | live retrieval 2026-09-12T04:30Z | T2 | `1283649724c30163fa08ba7ab02d1230fa9a7dd0613b8b4b3d96cd1d9dc4febd` |
| `CME-SVC-2027-06-17` | 2027-06-17 .. 2027-06-19 | <https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2027-06-17&toEventDate=2027-06-19> | live retrieval 2026-09-12T04:30Z | T2 | `60c9a2f5106d61039a616986b463cd852861ee4d3b91b11fac8badfa1b97b01c` |
| `CME-SVC-2027-07-04` | 2027-07-04 .. 2027-07-06 | <https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2027-07-04&toEventDate=2027-07-06> | live retrieval 2026-09-12T04:30Z | T2 | `93ff8232886435c94be682bf968aa30749011cdf8dadeb7d2425a3b0b9e0bf71` |
| `CME-SVC-2027-09-05` | 2027-09-05 .. 2027-09-07 | <https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2027-09-05&toEventDate=2027-09-07> | live retrieval 2026-09-12T04:30Z | T2 | `aa08a3bd102812928e69cf1ea4c8a84f738eaa5d14f967acee7d2571e74aedb9` |
| `CME-SVC-2027-11-24` | 2027-11-24 .. 2027-11-26 | <https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2027-11-24&toEventDate=2027-11-26> | live retrieval 2026-09-12T04:30Z | T2 | `6aa7c0fd701a02480dabeac1fbae1a69b56e77643a29e3a9b2223c56e822ce9f` |
| `CME-SVC-2027-12-22` | 2027-12-22 .. 2027-12-25 | <https://www.cmegroup.com/services/trading-hours-by-product?id=316,133,425,300,58,437,22,8478,5201,10191&pageNumber=1&pageSize=999&sortAsc=true&fromEventDate=2027-12-22&toEventDate=2027-12-25> | live retrieval 2026-09-12T04:30Z | T2 | `5edc4dd588a32faa74f841494c10a3df48692dca29843c3581bad3e18c30fef9` |


## Sources

Row review: 2026-09-06 (UTC) is the date the ledger row was last reviewed as a
whole. Per-source retrieval dates were not recorded before the 2026-09-12
migration; where a later targeted review, capture or document date is recorded
beside a source below, that date governs for that source, and dates are added
as each source is re-verified.

- <https://www.cmegroup.com/notices/ser/2017/12/SER-8051R.html> — CME SER-8051R, bitcoin futures launch, the 2017-12-17 revision's source.
- <https://www.cmegroup.com/market-regulation/rule-filings/2017/12/17-417.pdf> — CME rule filing 17-417, the launch filing carrying the original 17:00–16:00 CT weekday grid.
- <https://web.archive.org/web/20171214071544id_/http://www.cmegroup.com/trading/equity-index/us-index/bitcoin_contract_specifications.html> — CME bitcoin contract specification — capture 2017-12-14, carrying the launch statement and publishing the matching grid only.
- <https://www.cmegroup.com/notices/clearing/2021/01/Chadv21-028.pdf> — CME clearing advisory Chadv21-028, the ETH launch.
- <https://www.cmegroup.com/notices/electronic-trading/2021/04/20210426.html> — CME Globex notice 20210426, the MBT launch.
- <https://www.cmegroup.com/notices/electronic-trading/2021/11/20211129.html> — CME Globex notice 20211129, the MET launch.
- <https://www.cmegroup.com/content/dam/cmegroup/market-regulation/rule-filings/2026/5/26-114.pdf> — CME rule filing 26-114, 24/7 cryptocurrency trading, the 2026-05-29 and 2026-05-30 revisions' source.
- <https://www.cmegroup.com/notices/electronic-trading/2026/05/20260525.html> — CME Globex notice 20260525, corroborating the 24/7 transition.
- <https://www.cmegroup.com/articles/faqs/frequently-asked-questions-cryptocurrency-futures.html> — CME cryptocurrency futures FAQ, current grid.
- <https://www.cmegroup.com/notices/electronic-trading/2026/07/20260727.html> — CME Globex notice 20260727, the 2026-08-01 Saturday extension.
- <https://www.cmegroup.com/notices/electronic-trading/2026/08/20260824.html> — CME Globex notice 20260824, the 2026-08-29 and 2026-09-19 Saturday extensions.
- <https://www.cmegroup.com/notices/electronic-trading/2026/08/20260831.html> — CME Globex notice 20260831, restating notice 20260824.

Official origin of the specification capture: <http://www.cmegroup.com/trading/equity-index/us-index/bitcoin_contract_specifications.html>.

## Gaps and residual risks

- **order-entry** — the five-day era's Sunday and weekday Pre-Open onset is undated in the normal week; the 2017-12-14, 2017-12-22 and 2018-01-04 contract-specification captures publish the matching grid only, and the 16:00 CT queues the six merged spans and 2025-11-28 publish are stated by their own `replacement blocks` rows rather than by the week. Closing condition: a CME artifact that states the Pre-Open in session language on a day-level effective date. Served identity, so tracked as issue [#123](https://github.com/SharurTrading/exchange-hours-rs/issues/123) (LAW-FOLLOW-UPS-ARE-ISSUES), which `schedules/sourcing.rs` cites as the declaration's closing condition and `docs/schedules/coverage-2025.md` names in this scope's `Closing issues` cell.
- **forward-dated row** — the 2026-09-19 Saturday extension is encoded ahead of its effective day on CME Globex notice 20260824, restated by notice 20260831. It must be confirmed against the operator before that day (LAW-WATCH).
- **holidays** — the built-in holiday table covers trade dates 2025-01-01 .. 2027-12-31 at T2. Its per-year gaps, interpretive steps and residual risks are recorded under `## Holidays`, beside the rows they are about, rather than pooled here. The load-bearing ones: the 24/7 era's Monday and Thursday holidays serve the 16:00-16:01 CT minute closed (#93), the five-day era's Pre-Open onset is undated in the normal week (#123), five scalar closure dates still carry an unstated 16:00 CT queue, eight 2025 windows rest on a pre-holiday capture, and no T1 rendering of these hours exists.
- **scope** — ETH, MBT and MET joined this already-live family in 2021 and later member-product listings remain catalog data; their individual launch dates are not family-clock revisions.

## Module narrative (moved from src/calendar/schedules/futures/us/cryptocurrency.rs on 2026-09-12 UTC)

2026-08-31 five-day-era Pre-Open review — confirmed knowledge-bound. The CME
bitcoin contract specification captured 2017-12-14, which itself carries the
launch statement "Effective Sunday 17 December 2017 for trade date Monday 18
December 2017 ... CME will launch Bitcoin Futures", publishes only the
matching grid: "CME Globex: Sunday - Friday 6:00 p.m. - 5:00 p.m. (5:00 p.m.
- 4:00 p.m. CT) with a 60-minute break each day beginning at 5:00 p.m. (4:00
p.m. CT)". It states no Pre-Open, and neither do the 2017-12-22 or
2018-01-04 captures. The five-day era's Sunday/weekday Pre-Open onset is
therefore undated at the source, not merely unsearched. Official origin
http://www.cmegroup.com/trading/equity-index/us-index/bitcoin_contract_specifications.html
delivered via
https://web.archive.org/web/20171214071544id_/http://www.cmegroup.com/trading/equity-index/us-index/bitcoin_contract_specifications.html

Bitcoin futures opened Sunday 2017-12-17 at 17:00 CT for trade date Monday
2017-12-18. The launch filing gives the original 17:00-16:00 weekday grid.
ETH, MBT, and MET joined this already-live family in 2021; their individual
launch dates are intentionally not family-clock revisions.

CME filing 26-114 changed all non-spot-quoted cryptocurrency futures to
24/7 Globex trading effective Friday 2026-05-29: matching maintenance is
16:00-16:02 CT Monday-Friday with Pre-Open from 16:01, and 02:00-04:00 CT
Saturday with Pre-Open from 03:45. Three one-day Globex notices then
temporarily extended the Saturday window for the 24/7 markets — 2026-08-01
through 09:00 CT (notice 20260727), 2026-08-29 through 06:00 CT and
2026-09-19 through 08:00 CT (notice 20260824, restated by 20260831) — each
without publishing a replacement Pre-Open, each followed by the standard
02:00-04:00 window. The notices' tables name this family's channels, "CME
Crypto Futures | 74 | 326" and "CME Crypto Options | 327", alongside the
event-contract channels. The September row is forward-dated on the
operator's statement.

`SessionRule` spans at most one local midnight, so the multi-day weekend
session is stored in adjacent pieces. The key-backed calendar joins those
storage-only pieces at query time, while retaining the 02:00-03:45 Saturday
closed break and 03:45-04:00 Pre-Open. Both weekend blocks carry the following
open business date: normally Monday, or Tuesday when a caller policy closes
Monday. The corresponding daily bar runs from Friday 16:01 Pre-Open through
that business date's 16:00 close.
https://www.cmegroup.com/notices/ser/2017/12/SER-8051R.html
https://www.cmegroup.com/market-regulation/rule-filings/2017/12/17-417.pdf
https://www.cmegroup.com/notices/clearing/2021/01/Chadv21-028.pdf
https://www.cmegroup.com/notices/electronic-trading/2021/04/20210426.html
https://www.cmegroup.com/notices/electronic-trading/2021/11/20211129.html
https://www.cmegroup.com/content/dam/cmegroup/market-regulation/rule-filings/2026/5/26-114.pdf
https://www.cmegroup.com/notices/electronic-trading/2026/05/20260525.html
https://www.cmegroup.com/articles/faqs/frequently-asked-questions-cryptocurrency-futures.html
https://www.cmegroup.com/notices/electronic-trading/2026/07/20260727.html
https://www.cmegroup.com/notices/electronic-trading/2026/08/20260824.html
https://www.cmegroup.com/notices/electronic-trading/2026/08/20260831.html
