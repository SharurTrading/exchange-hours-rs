#!/usr/bin/env python3
# SPDX-License-Identifier: MIT-0
"""gen_wave5_tests.py -- append the 2013-2015 era tests to the six family suites.

Reads the encoding plan (`tools/out/wave5/ROWS.json`), the repaired block and
each test file's own helpers, and appends, per family:

  * `era_2013_2015_sweeps_every_shipped_row_kind_and_instant` — the walk over the
    whole window, exercising the instant each row states on both sides of it;
  * `era_2013_2015_window_edges_answer_as_the_module_declares`;
  * `era_2013_2015_window_is_declared_in_order_and_bounds_every_row` — the whole
    window list, with the era's neighbours probed;
  * `era_2013_2015_rows_are_the_audited_date_kind_and_tier_set` — a handwritten
    `(date, kind, tier)` table compared in order, which the count sweep alone
    cannot see past.

It refuses to run twice.  The window lists in the earlier eras' own tests are
updated in the same pass, because a new declared window is a fact every one of
them asserts.

Usage:  WAVE5_RESEARCH=... python3 tools/gen_wave5_tests.py
"""

from __future__ import annotations

import argparse
import json
import os
import re
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))

import wave5_rows as R  # noqa: E402

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
FAMILY_TESTS = os.path.join(ROOT, "tests", "futures_family_boundaries")
VENUE_TEST = os.path.join(ROOT, "tests", "venue_sessions",
                          "holidays_cme_venues.rs")

#: The helper each family's test file already defines for a CT instant.
CT = {
    "globex_equity_index": "ct",
    "globex_energy": "ct",
    "globex_fx": "ct",
    "globex_grains": "ct",
    "globex_interest_rates": "ct",
    "globex_livestock": "ct",
}

#: How each suite builds its `ExchangeCalendar`: `call` names a zero-argument
#: helper the suite already defines, `expr` is spelled out.
CALENDAR = {
    "globex_equity_index": ("call", "equity_index"),
    "globex_energy": ("call", "calendar"),
    "globex_fx": ("call", "fx"),
    "globex_grains": ("expr", "calendar_for_market_hours_key(ZC)"),
    "globex_interest_rates": ("call", "rates"),
    "globex_livestock": ("call", "calendar"),
}

#: How each suite's `day` helper takes its date: `tuple` is
#: `day((2015, 12, 31))`, `parts` is `day(2015, 12, 31)`.
DAY_STYLE = {
    "globex_equity_index": "parts",
    "globex_energy": "tuple",
    "globex_fx": "parts",
    "globex_grains": "tuple",
    "globex_interest_rates": "parts",
    "globex_livestock": "tuple",
}

WINDOW = [(2010, 1, 1, 2012, 12, 31), (2013, 1, 1, 2015, 12, 31),
          (2016, 1, 1, 2018, 12, 31), (2019, 1, 1, 2021, 12, 31),
          (2022, 1, 1, 2024, 12, 31), (2025, 1, 1, 2027, 12, 31)]


def kind_expr(row):
    if row["kind"] == "Closed":
        return "HolidayKind::Closed"
    if row["kind"] == "Unsourced":
        return "HolidayKind::Unsourced"
    if row["kind"] == "EarlyClose":
        return ("HolidayKind::EarlyClose {\n            close_ssm: %s,\n        }"
                % ssm(row["close_ssm"]))
    if row["kind"] == "LateOpen":
        return ("HolidayKind::LateOpen {\n            open_ssm: %s,\n        }"
                % ssm(row["open_ssm"]))
    return ("HolidayKind::LateOpenAndEarlyClose {\n"
            "            open_ssm: %s,\n            close_ssm: %s,\n        }"
            % (ssm(row["open_ssm"]), ssm(row["close_ssm"])))


def ssm(value):
    hours, rest = divmod(value, 3_600)
    minutes = rest // 60
    parts = []
    if hours:
        parts.append("%d * 3_600" % hours)
    if minutes:
        parts.append("%d * 60" % minutes)
    return " + ".join(parts) if parts else "0"


def clock(value):
    hours, rest = divmod(value, 3_600)
    return (hours, rest // 60, rest % 60)


def row_table(rows):
    out = []
    for row in rows:
        date = row["date"]
        year, month, day = (int(p) for p in date.split("-"))
        out.append("    ((%d, %d, %d), %s, EvidenceTier::%s),"
                   % (year, month, day, kind_expr(row), row["tier"]))
    return "\n".join(out)


def sweep_body(family, rows):
    venue = CALENDAR[family]
    ct = CT[family]
    counts = {
        "closed": len([r for r in rows if r["kind"] == "Closed"]),
        "early": len([r for r in rows if r["kind"] == "EarlyClose"]),
        "late": len([r for r in rows if r["kind"] == "LateOpen"]),
        "both": len([r for r in rows if r["kind"] == "LateOpenAndEarlyClose"]),
    }
    out = []
    out.append("    let venue = %s;" % CALENDAR_EXPR[family])
    out.append("    let (mut closed, mut early, mut late, mut both) = "
               "(0_usize, 0_usize, 0_usize, 0_usize);")
    out.append("    let mut date = %s;" % D(family, 2013, 1, 1))
    out.append("    while date <= %s {" % D(family, 2015, 12, 31))
    out.append("        if let Some(row) = venue.holiday_on(date) {")
    out.append("            assert_eq!(row.tier(), EvidenceTier::T1, "
               "\"{date}\");")
    out.append("            assert!(!row.document_id().is_empty(), "
               "\"{date} cites no artifact\");")
    out.append("            match row.kind() {")
    out.append("                HolidayKind::Closed => {")
    out.append("                    closed += 1;")
    out.append("                    assert!(")
    out.append("                        venue.is_closed_trade_date(date, "
               "SessionKind::Both),")
    out.append("                        \"{date}\"")
    out.append("                    );")
    out.append("                    assert!(")
    out.append("                        !venue.is_open(ct_on(day_before(date), "
               "%s))," % CT_EXPR(family, "17", "0", "0"))
    out.append("                        \"{date}: the eve leg is gone\"")
    out.append("                    );")
    out.append("                }")
    out.append("                HolidayKind::EarlyClose { close_ssm } => {")
    out.append("                    early += 1;")
    out.append("                    let (h, m, %s) = (%s);"
               % (_var(S_VAR, family, 's'), "close_ssm / 3_600, (close_ssm % 3_600) / 60, "
                                 "close_ssm % 60"))
    out.append("                    let cutoff = ct_on(date, %s);"
               % CT_EXPR(family))
    out.append("                    assert!(")
    out.append("                        venue.is_open(cutoff - "
               "Duration::seconds(1)),")
    out.append("                        \"{date}\"")
    out.append("                    );")
    out.append("                    assert!(!venue.is_open(cutoff), "
               "\"{date}: end-exclusive\");")
    out.append("                    assert_eq!(")
    out.append("                        venue.trade_date(cutoff - "
               "Duration::seconds(1)),")
    out.append("                        Some(date),")
    out.append("                        \"{date}\"")
    out.append("                    );")
    out.append("                    assert_eq!(")
    out.append("                        venue.candle_end(cutoff - "
               "Duration::minutes(1), CalendarResolution::Daily),")
    out.append("                        Some(cutoff),")
    out.append("                        \"{date}\"")
    out.append("                    );")
    out.append("                }")
    out.append("                HolidayKind::LateOpen { open_ssm } => {")
    out.append("                    late += 1;")
    out.append("                    let (h, m, %s) = (%s);"
               % (_var(S_VAR, family, 's'), "open_ssm / 3_600, (open_ssm % 3_600) / 60, "
                                 "open_ssm % 60"))
    out.append("                    let open = ct_on(date, %s);"
               % CT_EXPR(family))
    out.append("                    assert!(!venue.is_open(open - "
               "Duration::seconds(1)), \"{date}\");")
    out.append("                    assert!(venue.is_open(open), "
               "\"{date}: matching starts at the printed instant\");")
    out.append("                    assert_eq!(")
    out.append("                        venue.trade_date(open + "
               "Duration::hours(1)),")
    out.append("                        Some(date),")
    out.append("                        \"{date}: keyed to its own trade date\"")
    out.append("                    );")
    out.append("                }")
    out.append("                HolidayKind::LateOpenAndEarlyClose "
               "{ open_ssm, close_ssm } => {")
    out.append("                    both += 1;")
    out.append("                    let (oh, om, %s) = (%s);"
               % (_var(SEC_VAR, family, 'os'), "open_ssm / 3_600, (open_ssm % 3_600) / 60, "
                                   "open_ssm % 60"))
    out.append("                    let (ch, cm, %s) = (%s);"
               % (_var(CSEC_VAR, family, 'cs'), "close_ssm / 3_600, (close_ssm % 3_600) / 60, "
                                    "close_ssm % 60"))
    out.append("                    let open = ct_on(date, %s);"
               % CT_EXPR(family, "oh", "om", "os"))
    out.append("                    let cutoff = ct_on(date, %s);"
               % CT_EXPR(family, "ch", "cm", "cs"))
    out.append("                    assert!(!venue.is_open(open - "
               "Duration::seconds(1)), \"{date}\");")
    out.append("                    assert!(venue.is_open(open), "
               "\"{date}\");")
    out.append("                    assert!(venue.is_open(cutoff - "
               "Duration::seconds(1)), \"{date}\");")
    out.append("                    assert!(!venue.is_open(cutoff), "
               "\"{date}: end-exclusive\");")
    out.append("                }")
    out.append("                other => panic!(\"{date}: this era ships no "
               "{other:?}\"),")
    out.append("            }")
    out.append("        }")
    out.append("        date = date.succ_opt().expect(\"the era ends well "
               "before the bound\");")
    out.append("    }")
    out.append("    assert_eq!(")
    out.append("        (closed, early, late, both),")
    out.append("        (%d, %d, %d, %d)," % (counts["closed"], counts["early"],
                                              counts["late"], counts["both"]))
    out.append("        \"the era's shape\"")
    out.append("    );")
    return "\n".join(out)


#: Whether a suite's `ct_on` takes a `(h, m, s)` tuple or separate arguments.
CT_ON_STYLE = {"globex_energy": "parts"}


#: The seconds binding must be *used* when `ct_on` takes three arguments and
#: unused when it takes two.
S_VAR = {name: ("_s" if style == "parts" else "s")
         for name, style in CT_ON_STYLE.items()}
SEC_VAR = {name: ("_os" if style == "parts" else "os")
           for name, style in CT_ON_STYLE.items()}
CSEC_VAR = {name: ("_cs" if style == "parts" else "cs")
            for name, style in CT_ON_STYLE.items()}


def _var(table, family, fallback):
    return table.get(family, fallback)


def CT_EXPR(family, h="h", m="m", s="s"):
    """A `ct_on` time argument in the suite's own shape."""
    if CT_ON_STYLE.get(family) == "parts":
        return "%s, %s" % (h, m)
    return "(%s, %s, %s)" % (h, m, s)


def CT_ON(family, _unused, hour, minute):
    """A fixed `ct_on(...)` probe at 09:00 CT on the last day of the era."""
    date = D(family, 2015, 12, 31)
    if CT_ON_STYLE.get(family) == "parts":
        return "ct_on(%s, %d, %d)" % (date, hour, minute)
    return "ct_on(%s, (%d, %d, 0))" % (date, hour, minute)


CALENDAR_EXPR = {name: ("%s()" % target if style == "call" else target)
                 for name, (style, target) in CALENDAR.items()}


def D(family, year, month, day):
    """One date literal in the suite's own `day` style."""
    if DAY_STYLE[family] == "tuple":
        return "day((%d, %d, %d))" % (year, month, day)
    return "day(%d, %d, %d)" % (year, month, day)


def family_block(family, rows):
    table = row_table(rows)
    last = rows[-1]["date"]
    last_year, last_month, last_day = (int(p) for p in last.split("-"))
    first = rows[0]["date"]
    first_year, first_month, first_day = (int(p) for p in first.split("-"))
    windows = ",\n".join(
        "            (%s, %s)" % (D(family, w[0], w[1], w[2]),
                                D(family, w[3], w[4], w[5]))
        for w in WINDOWS_BY_FAMILY.get(family, DEFAULT_WINDOWS))
    return '''
// ---------------------------------------------------------------------------
// The 2013-2015 rows.
// ---------------------------------------------------------------------------

/// The era-wide sweep: every row the 2013-2015 window ships, read from the
/// module, with both sides of every instant it states.
///
/// The walk covers the whole window, so a dropped or added row fails on the
/// era's total, a moved instant fails on the side of the instant it moved from,
/// and a kind this family does not ship fails outright. The count tuple is the
/// era's shape as the block records it; the handwritten table below pins the
/// date set the counts cannot see.
#[test]
fn era_2013_2015_sweeps_every_shipped_row_kind_and_instant() {
%(sweep)s
}

/// The era is its own declared window: 2013-01-01 is inside it and 2012-12-31
/// and 2016-01-01 belong to the waves either side and lie outside it.
#[test]
fn era_2013_2015_window_edges_answer_as_the_module_declares() {
    let venue = %(family_calendar)s;
    let coverage = venue
        .holiday_coverage()
        .expect("%(family)s ships a table");
    let era = (%(d2013_1_1)s, %(d2015_12_31)s);

    assert!(
        coverage.windows().contains(&era),
        "the 2013-2015 window is declared as a window of its own"
    );
    assert!(coverage.contains(era.0));
    assert!(coverage.contains(era.1));

    // The era's own edges answer for themselves: its first day is the shipped
    // New Year closure, and its last is the last date the window declares.
    assert_eq!(
        venue.holiday_on(%(d2013_1_1)s).map(Holiday::kind),
        Some(HolidayKind::Closed)
    );
    assert!(
        coverage.contains(%(d2015_12_31)s),
        "the era's last day is inside the declared window"
    );
    // The neighbouring dates are their own eras' business: 2012-12-31 is
    // audited normal by the wave below, 2016-01-01 belongs to the wave above,
    // and neither is this era.
    assert_eq!(venue.holiday_on(%(d2012_12_31)s), None);
    assert!(!(era.0 <= %(d2016_1_1)s && %(d2016_1_1)s <= era.1));
    // A date below the January-2010 floor is outside every window, so this
    // table has no answer for it at all.
    assert!(!coverage.contains(%(d2009_12_31)s));
    assert_eq!(venue.holiday_on(%(d2009_12_31)s), None);
}

/// The family's coverage names its windows in order, and the 2013-2015 window
/// is one of them: every row the era ships lies inside it, and no row ships on
/// the era's outer neighbours.
#[test]
fn era_2013_2015_window_is_declared_in_order_and_bounds_every_row() {
    let venue = %(family_calendar)s;
    let coverage = venue
        .holiday_coverage()
        .expect("%(family)s ships a table");

    assert_eq!(
        coverage.windows(),
        vec![
%(windows)s,
        ]
    );

    let mut rows = 0_usize;
    let mut date = %(d2013_1_1)s;
    while date <= %(d2015_12_31)s {
        if venue.holiday_on(date).is_some() {
            assert!(coverage.contains(date), "{date} ships outside its window");
            rows += 1;
        }
        date = date.succ_opt().expect("the era ends well before the bound");
    }
    assert_eq!(rows, %(rows)d, "the era's rows");
    assert_eq!(venue.holiday_on(%(d2012_12_31)s), None);
    assert!(!coverage.contains(%(d2028_1_1)s));
    assert_eq!(venue.holiday_on(%(d2028_1_1)s), None);
}

/// The 2013-2015 rows as the block records them: date, kind and tier in order,
/// handwritten here rather than read back from the module. The era-wide sweep
/// counts kinds and instants, which a row moved to another audited date with
/// the same kind and instant would leave unchanged; this pins the date set
/// itself, in the order `holiday_on` must answer it.
const ERA_2013_2015_ROWS: &[((i32, u32, u32), HolidayKind, EvidenceTier)] = &[
%(table)s
];

/// The era's audited date, kind and tier set, in order.
#[test]
fn era_2013_2015_rows_are_the_audited_date_kind_and_tier_set() {
    let venue = %(family_calendar)s;
    let mut index = 0_usize;
    let mut date = %(d2013_1_1)s;
    while date <= %(d2015_12_31)s {
        if let Some(row) = venue.holiday_on(date) {
            let (expected, kind, tier) = *ERA_2013_2015_ROWS.get(index).unwrap_or_else(|| {
                panic!("{date}: a row ships in the 2013-2015 window that the block does not record")
            });
            assert_eq!(
                (date.year(), date.month(), date.day()),
                expected,
                "the 2013-2015 rows must ship in order, with none added"
            );
            assert_eq!(row.kind(), kind, "{date}");
            assert_eq!(row.tier(), tier, "{date}");
            index += 1;
        }
        date = date.succ_opt().expect("the era ends well before the bound");
    }
    assert_eq!(index, ERA_2013_2015_ROWS.len(), "every recorded row ships");
}
''' % dict(sweep=sweep_body(family, rows), family=family,
         family_calendar=CALENDAR_EXPR[family],
         windows=windows,
         rows=len(rows), table=table,
         d2013_1_1=D(family, 2013, 1, 1),
         d2015_12_31=D(family, 2015, 12, 31),
         d2012_12_31=D(family, 2012, 12, 31),
         d2016_1_1=D(family, 2016, 1, 1),
         d2009_12_31=D(family, 2009, 12, 31),
         d2028_1_1=D(family, 2028, 1, 1),
         last_probe=CT_ON(family, None, 9, 0))


#: The windows each family's table declares, as the tests must list them.
WINDOWS_BY_FAMILY = {
    "globex_livestock": [(2010, 1, 1, 2012, 12, 31), (2013, 1, 1, 2015, 12, 31),
                         (2019, 1, 1, 2021, 12, 31), (2022, 1, 1, 2024, 12, 31),
                         (2025, 1, 1, 2027, 12, 31)],
}
DEFAULT_WINDOWS = WINDOW


def update_window_lists(path):
    """Adds the 2013-2015 window to every `coverage.windows()` assertion."""
    text = open(path, encoding="utf-8").read()
    if "day((2013, 1, 1)), day((2015, 12, 31))" in text:
        return text, 0
    # Insert the new window directly above the 2016-2018 line, keeping the
    # file's own indentation.  `str.replace` rather than a regex: the lines are
    # literal and a regex here has repeatedly mis-parsed the parentheses.
    count = 0
    pieces = []
    for line in text.split("\n"):
        stripped = line.strip()
        for needle, new in (
                ("(day(2016, 1, 1), day(2018, 12, 31)),",
                 "(day(2013, 1, 1), day(2015, 12, 31)),"),
                ("(day((2016, 1, 1)), day((2018, 12, 31))),",
                 "(day((2013, 1, 1)), day((2015, 12, 31))),"),
                # globex_livestock declared no 2016-2018 era, so its list goes
                # straight from 2012 to 2019.
                ("(day((2019, 1, 1)), day((2021, 12, 31))),",
                 "(day((2013, 1, 1)), day((2015, 12, 31))),")):
            if stripped == needle and "2013, 1, 1" not in "".join(pieces[-3:]):
                indent = line[: len(line) - len(line.lstrip())]
                pieces.append("%s%s" % (indent, new))
                count += 1
                break
        pieces.append(line)
    return "\n".join(pieces), count
    return text, count


def main(argv=None):
    parser = argparse.ArgumentParser()
    parser.add_argument("--research", default=os.environ.get("WAVE5_RESEARCH"))
    parser.add_argument("--out", default="tools/out/wave5")
    args = parser.parse_args(argv)
    plan = json.load(open(os.path.join(args.out, "ROWS.json")))

    for family in R.FAMILIES:
        path = os.path.join(FAMILY_TESTS, "holidays_%s.rs" % family)
        text = open(path, encoding="utf-8").read()
        if "ERA_2013_2015_ROWS" in text:
            raise SystemExit("%s already carries the era tests" % path)
        text, count = update_window_lists(path)
        if count == 0:
            raise SystemExit("%s: no coverage.windows() assertion found" % path)
        rows = plan["families"][family]
        # The era tests subtract instants, so `Duration` must be in scope.
        if "Duration" not in text.split("fn ")[0]:
            text, added = re.subn(
                r"^(use chrono::\{)([^}]*)(\};)$",
                lambda m: "%s%s, Duration%s" % (m.group(1), m.group(2), m.group(3)),
                text, count=1, flags=re.M)
            if added == 0:
                raise SystemExit("%s: no `use chrono::` line to widen" % path)
        open(path, "w", encoding="utf-8").write(
            text.rstrip("\n") + "\n" + family_block(family, rows))
        print("%-24s +4 era tests, %d window lists updated"
              % (family, count))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
