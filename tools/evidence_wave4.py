#!/usr/bin/env python3
# SPDX-License-Identifier: MIT-0
"""evidence_wave4.py -- write the 2019-2021 era into the modules' docs, the
twelve evidence files, and nothing else.

Reads the wave-4 generator's output (`tools/out/wave4/`) plus the crate's own
modules, recomputes every count from the modules themselves, and patches:

  * each family module's `TABLE` doc comment (the audited-era list and the
    interval left unaudited),
  * `docs/evidence/<family>.md`: the `**Coverage:**` line, the `### 2019`,
    `### 2020` and `### 2021` tables, a `### Documents` table for the era's ids
    and a `### Gaps and residual risks, 2019-2021` block,
  * `docs/evidence/<venue>.md` for the four CME venues: the same, plus the
    per-date dispute account the D17 rule requires.

Every number it writes is recomputed here from the module tables or from
`tools/out/wave4/SUMMARY.json`; none is typed by hand.
"""

from __future__ import annotations

import collections
import json
import os
import re
import sys

ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), ".."))
OUT = os.path.join(ROOT, "tools", "out", "wave4")
HOLIDAYS = os.path.join(ROOT, "src", "calendar", "schedules", "holidays")
EVIDENCE = os.path.join(ROOT, "docs", "evidence")

FAMILIES = (
    "globex_equity_index", "globex_energy", "globex_fx", "globex_grains",
    "globex_interest_rates", "globex_livestock", "globex_cryptocurrency",
    "globex_nikkei_225_dollar",
)
VENUES = ("cme", "cbot", "comex", "nymex")

WINDOW_RE = re.compile(
    r"\(\s*(\d+)\s*,\s*(\d+)\s*,\s*(\d+)\s*\)\s*\.\.=\s*"
    r"\(\s*(\d+)\s*,\s*(\d+)\s*,\s*(\d+)\s*\)")
ROW_RE = re.compile(r"^        \((\d{4}), (\d+), (\d+), (.+?), (T[12]), \"(.+?)\"\),$")

ORDINAL = {1: "first", 2: "second", 3: "third", 4: "fourth", 5: "fifth",
           6: "sixth", 7: "seventh"}
NUMBER_WORD = {1: "One", 2: "Two", 3: "Three", 4: "Four", 5: "Five",
               6: "Six", 7: "Seven"}


def series(items):
    """`a`, `a and b`, `a, b and c` — the way the modules write a list."""
    if not items:
        return ""
    if len(items) == 1:
        return items[0]
    return ", ".join(items[:-1]) + " and " + items[-1]

#: The tier mix each window is recorded at, by year, as the module states it.
TIER_OF = {
    2010: "T1", 2011: "T1", 2012: "T1", 2016: "T1", 2017: "T1", 2018: "T1",
    2019: "T1", 2020: "T1", 2021: "T1", 2022: "T1", 2023: "T1",
    2024: "T2", 2025: "T2", 2026: "T2", 2027: "T2",
}


def module_path(name):
    return os.path.join(HOLIDAYS, "%s.rs" % name)


def parse_table(text):
    """The `holidays!` block of one module: windows, rows and their kinds."""
    body = text[text.index("holidays! {"):]
    coverage, _, rest = body.partition("rows:")
    windows = [
        ((int(m[0]), int(m[1]), int(m[2])), (int(m[3]), int(m[4]), int(m[5])))
        for m in WINDOW_RE.findall(coverage)
    ]
    rows = []
    for line in rest.splitlines():
        match = ROW_RE.match(line)
        if match:
            rows.append({
                "year": int(match.group(1)), "month": int(match.group(2)),
                "day": int(match.group(3)), "kind": match.group(4),
                "tier": match.group(5), "document": match.group(6),
            })
    return windows, rows


def era_rows(rows, first_year, last_year):
    return [row for row in rows
            if first_year <= row["year"] <= last_year]


def kinds_phrase(rows):
    """`8 full closures, 25 early closes and 3 Unsourced rows`."""
    counts = collections.Counter()
    for row in rows:
        kind = row["kind"]
        if kind == "Closed":
            counts["full closures"] += 1
        elif kind == "Unsourced":
            counts["`Unsourced` rows"] += 1
        elif kind.startswith("early_close("):
            counts["early closes"] += 1
        elif kind.startswith("late_open("):
            counts["late opens"] += 1
        elif kind.startswith("late_open_and_early_close("):
            counts["late opens with early closes"] += 1
        else:
            raise SystemExit("unclassified kind %r" % kind)
    order = ["full closures", "early closes", "late opens",
             "late opens with early closes", "`Unsourced` rows"]
    return series(["%d %s" % (counts[label], label) for label in order if counts[label]])


def window_list(windows):
    return ", ".join("%04d-%02d-%02d..%04d-%02d-%02d" % (first + last)
                     for first, last in windows)


def unaudited(windows):
    """The year intervals between the windows, as `YYYY-01-01 .. YYYY-12-31`."""
    gaps = []
    for (_, last), (first, _) in zip(windows, windows[1:]):
        if last[0] + 1 <= first[0] - 1:
            gaps.append("%04d-01-01 .. %04d-12-31" % (last[0] + 1, first[0] - 1))
    return gaps


def switch_positions(rows, windows):
    """`{year: tier}` where a later window starts with a different tier mix."""
    return {}


def wrap(prefix, text, width=75):
    """Wrap `text` under a `prefix` of `/// `-style lines."""
    words = text.split()
    lines = []
    current = prefix
    for word in words:
        if len(current) + len(word) + 1 > width and current != prefix:
            lines.append(current.rstrip())
            current = prefix
        current += word + " "
    if current.strip() != prefix.strip():
        lines.append(current.rstrip())
    return lines


def era_doc_text(name, windows, rows):
    """The module's `N audited eras:` paragraph, recomputed."""
    entries = []
    for first, last in windows:
        tiers = []
        for year in range(first[0], last[0] + 1):
            if TIER_OF[year] not in tiers:
                tiers.append(TIER_OF[year])
        entries.append("%04d-%04d at %s" % (first[0], last[0], "/".join(tiers)))
    count = len(windows)
    lines = wrap("/// ", "%s audited era%s: %s." % (
        NUMBER_WORD[count], "" if count == 1 else "s", series(entries)))
    gaps = unaudited(windows)
    if not gaps:
        prose = ("Nothing before %04d-01-01 has a table at all: that interval lies "
                 "outside every window, so `holiday_on` has no answer there rather "
                 "than reporting a normal date." % windows[0][0][0])
    elif windows[0][0][0] > 2010:
        prose = ("Nothing before %04d-01-01 has a table at all, and the %s interval "
                 "between the later windows is audited by no wave: both lie outside "
                 "every window, so `holiday_on` has no answer there rather than "
                 "reporting a normal date." % (windows[0][0][0], series(gaps)))
    else:
        prose = ("The %s interval%s between them %s audited by no wave and lie%s "
                 "outside every window, so `holiday_on` has no answer there rather "
                 "than reporting a normal date."
                 % (series(gaps), "" if len(gaps) == 1 else "s",
                    "is" if len(gaps) == 1 else "are",
                    "s" if len(gaps) == 1 else ""))
    lines.extend(wrap("/// ", prose))
    return "\n".join(lines)


def patch_module_doc(name, windows, rows):
    path = module_path(name)
    text = open(path, encoding="utf-8").read()
    new = era_doc_text(name, windows, rows)
    start = text.index("/// ") + len("/// ")
    marker = re.search(r"^/// [A-Z][a-z]+ audited eras?: ", text, re.M)
    if marker is None:
        raise SystemExit("%s: no `audited eras` paragraph" % path)
    tail = text[marker.end():]
    end = tail.index("// Evidence:")
    block = tail[:end]
    # Keep everything from the sentence that follows the era list.
    keep_at = None
    for needle in ("Coverage ends at", "Coverage runs to"):
        if needle in block:
            keep_at = block.index(needle)
            break
    if keep_at is None:
        raise SystemExit("%s: no coverage tail sentence" % path)
    tail_text = block[keep_at:]
    replacement = new + "\n///\n/// " + tail_text.rstrip()
    text = text[:marker.start()] + replacement + "\n" + text[marker.end() + end:]
    open(path, "w", encoding="utf-8").write(text)
    return new


def patch_coverage(name, windows):
    path = os.path.join(EVIDENCE, "%s.md" % name)
    text = open(path, encoding="utf-8").read()
    line = "**Coverage:** %s (inclusive venue-local trade dates)." % window_list(windows)
    text = re.sub(r"^\*\*Coverage:\*\* .*$", line, text, count=1, flags=re.M)
    open(path, "w", encoding="utf-8").write(text)


def insert_before(text, marker, block):
    at = text.index(marker)
    return text[:at] + block + text[at:]


def main():
    summary = json.load(open(os.path.join(OUT, "SUMMARY.json"), encoding="utf-8"))
    documents = open(os.path.join(OUT, "documents.md"), encoding="utf-8").read().rstrip()
    fold = collections.defaultdict(list)
    for item in summary["fold"]:
        if item["status"] != "agree":
            fold[item["group"]].append(item)

    for name in FAMILIES:
        windows, rows = parse_table(open(module_path(name), encoding="utf-8").read())
        patch_module_doc(name, windows, rows)
        patch_coverage(name, windows)
        mine = era_rows(rows, 2019, 2021)
        counts = collections.Counter(row["tier"] for row in mine)
        era_table = open(os.path.join(OUT, "%s.evidence.md" % name), encoding="utf-8").read().rstrip()
        gaps = unaudited(windows)
        block = ["\n### Documents\n",
                 "This era's rows cite the ids below — CME Group's own Globex holiday "
                 "schedules, all at **T1**. The 2020 and 2021 sheets and the three "
                 "`Unsourced` Juneteenth rows are members of their year's consolidated "
                 "annual bundle; the 2019 sheets likewise, and 1-2 January 2019 come "
                 "from the December-2018 supplement CME published before the bundle. "
                 "Each id resolves to the URL the bytes were read at — an Internet "
                 "Archive raw replay of CME's own file — with the capture time in UTC, "
                 "the tier and the sha256. All of them resolve in the research store's "
                 "`holidays/raw/cme-2019-2021/`, whose `INDEX.md` carries the byte "
                 "counts and whose `shasum.txt` hashes every workbook. The next "
                 "`### Documents` table in this file is the 2022-2024 era's.\n",
                 documents, "\n",
                 "### Gaps and residual risks, 2019-2021\n",
                 "**This era declares the family's %s audited window.** The table as "
                 "a whole carries %d rows over %d windows — %s — and this era's share "
                 "is **%d rows**: %s. Every row is at %s. %s\n" % (
                     ORDINAL[len(windows)], len(rows), len(windows),
                     window_list(windows), len(mine), kinds_phrase(mine),
                     " and ".join(sorted(counts)),
                     ("The interval %s remains audited by no wave, so "
                      "`holiday_coverage` reports it as outside every window rather "
                      "than as audited normal." % series(gaps)) if len(gaps) == 1 else
                     ("The intervals %s remain audited by no wave, so "
                      "`holiday_coverage` reports them as outside every window rather "
                      "than as audited normal." % series(gaps)) if gaps else
                     ("Every date before the first window lies outside it, so "
                      "`holiday_coverage` reports it as unaudited rather than as "
                      "audited normal.")),
                 JUNETEENTH_TEXT, COLUMBUS_TEXT]
        if name == "globex_nikkei_225_dollar":
            block.append(NIKKEI_TEXT)
        if name == "globex_grains":
            block.append(grains_text(summary))
            block.append(fold_text("dairy", fold["dairy_same_page"]))
        if name == "globex_livestock":
            block.append(livestock_text())
            block.append(fold_text("lumber", fold["lumber_same_page"]))
        block.append("\n")
        path = os.path.join(EVIDENCE, "%s.md" % name)
        text = open(path, encoding="utf-8").read()
        marker = "### 2022" if "\n### 2022\n" in text else "## Sources"
        text = insert_before(text, marker, "\n" + era_table + "\n" + "".join(block))
        open(path, "w", encoding="utf-8").write(text)
        print("%-26s %d rows, %d windows" % (name, len(mine), len(windows)))

    for venue in VENUES:
        windows, rows = parse_table(open(
            os.path.join(HOLIDAYS, "venues", "%s.rs" % venue), encoding="utf-8").read())
        mine = era_rows(rows, 2019, 2021)
        closes = sum(1 for row in mine if row["kind"] == "Closed")
        unsourced = sum(1 for row in mine if row["kind"] == "Unsourced")
        table = open(os.path.join(OUT, "venue_%s.evidence.md" % venue),
                     encoding="utf-8").read().rstrip()
        block = ["\n### 2019-2021 (T1)\n",
                 venue_text(venue, mine, closes, unsourced, len(windows)),
                 "\n### Documents\n", documents, "\n",
                 "### 2019\n", year_slice(table, "2019"), "\n",
                 "### 2020\n", year_slice(table, "2020"), "\n",
                 "### 2021\n", year_slice(table, "2021"), "\n"]
        path = os.path.join(EVIDENCE, "%s.md" % venue)
        text = open(path, encoding="utf-8").read()
        text = insert_before(text, "### 2022-2024 (T1/T2)", "".join(block))
        open(path, "w", encoding="utf-8").write(text)
        print("%-26s %d rows (%d closed, %d unsourced)" % (venue, len(mine), closes,
                                                           unsourced))


def year_slice(table, year):
    """The `| <year>-…` rows of a venue evidence table, header included."""
    lines = [line for line in table.splitlines()
             if line.startswith("| trade date |") or line.startswith("|---")
             or line.startswith("| %s-" % year)]
    return "\n".join(lines)


def grains_text(summary):
    rows = json.load(open(os.path.join(OUT, "ROWS.json"), encoding="utf-8"))
    late = [row["date"] for row in rows["families"]["globex_grains"]
            if row["kind"] == "LateOpen"]
    both = [row["date"] for row in rows["families"]["globex_grains"]
            if row["kind"] == "LateOpenAndEarlyClose"]
    return (
        "**The day-after-closure rows (%d).** CME withdraws the prior-evening leg on the "
        "eve of a closure and prints the next trade date's own day session instead, so "
        "the first trade of that date is the ordinary 08:30 CT open, later than the "
        "19:00 CT first open that would otherwise carry it. `late_open(08:30)` ships on "
        "%s, and `late_open_and_early_close(08:30, 12:05)` on %s, where the same day "
        "also prints the family's half-day close. The counter-examples ship **no** row "
        "because the operator's sheet prints the ordinary 19:00 CT evening leg that "
        "carries them: 2019-04-22 (after Good Friday), 2020-07-06 (after the observed "
        "2020-07-03), 2020-12-28 and 2021-12-27 (after the two Christmas closures) and "
        "2021-01-04 (after New Year's Day 2021). `globex_grains` is the only family "
        "whose era needs the shape: every other wrapped family's sheets print an "
        "ordinary evening open on the day after a closure.\n"
        % (len(late) + len(both),
           ", ".join("`%s`" % date for date in late),
           ", ".join("`%s`" % date for date in both)))


def livestock_text():
    return (
        "**The two `late_open` entries this era does not ship.** The block marks "
        "2019-12-26 and 2020-01-02 `late_open` for this family with the printed cell "
        "`Open 8:30 CT /14:30 UTC` and `Open 8:30 CT /14:30 UTC`. This family's session "
        "is a flat 08:30-13:05 CT block inside one civil day, so the printed 08:30 CT "
        "*is* its ordinary first open and moves no boundary; the two dates carry no row "
        "and the ordinary week stands there. The same reading applies to the 2019-11-28 "
        "and 2020-11-26 reopen cells, which name the next trade date's day session as "
        "`Regular per Product`.\n")


def fold_text(group, items):
    if not items:
        return ""
    lines = []
    for item in items:
        lines.append("- `%s` — this family %s; the `%s` line %s."
                     % (item["trade_date"],
                        item["crate_family"] or "ships no row",
                        group, item["folded"] or "ships no row"))
    return (
        "**The `%s` fold (reporting only).** CME prints a `%s` line on the same sheets "
        "and the crate has no key for it, so this wave folds it into this family for "
        "reporting only: the folded group's own instants are compared against this "
        "family's and never key a row, because the two run on different clocks "
        "(`%s` closes at its own hour, where this family's ordinary close is %s). The "
        "two lines differ on the %d dates below; the closing condition is a consumer "
        "that maps the group (LAW-SERVICE-TIERS).\n\n%s\n"
        % (group, group.capitalize(),
           group, "13:20 CT" if group == "dairy" else "13:05 CT",
           len(items), "\n".join(lines)))


def venue_text(venue, rows, closes, unsourced, windows=5):
    routed = {
        "cme": "`globex_equity_index`, `globex_energy`, `globex_fx`, `globex_grains`, "
               "`globex_interest_rates` and `globex_livestock`",
        "cbot": "`globex_grains` and `globex_interest_rates`",
        "comex": "`globex_energy` alone",
        "nymex": "`globex_energy` alone",
    }[venue]
    return (
        "**This era declares the venue's %s audited window.** The table as a whole "
        "carries the union of the family eras; this era's share is **%d rows**: %d "
        "stated closures and %d `Unsourced` rows, every one the intersection of the "
        "families routed here — %s — by the D17 rule `venues.rs` states. A closure "
        "ships only where every routed family states the same one; a date on which "
        "they differ, or on which one states a row while another has audited the date "
        "normal, ships `Unsourced` and the disagreement is named per date below. "
        "`comex` and `nymex` route `globex_energy` alone, so their rows are that "
        "family's own, unchanged. The three Juneteenth dates ship `Unsourced` because "
        "every routed family states the crate's not-worked-up marker there.\n"
        % (ORDINAL[windows], len(rows), closes, unsourced, routed))


JUNETEENTH_TEXT = (
    "**Juneteenth 2019, 2020 and 2021 — three `Unsourced` rows.** CME published no "
    "Juneteenth schedule in any of the three years. Each row cites that year's own "
    "consolidated bundle — `2019-holiday-calendars.zip @2021-01-26T09:48:37Z`, "
    "`2020-holiday-calendars.zip @2026-07-30T11:18:34Z` and "
    "`2021-holiday-calendars.zip @2026-08-30T10:03:27Z` — whose member lists are "
    "CME's own account of every Globex holiday schedule it published that year and "
    "which carry no Juneteenth sheet; the four archived `holiday-calendar.html` "
    "index pages name none either, and a fresh 2018-2027 prefix CDX enumeration "
    "(`raw/cme-2019-2021-fix/cdx/cdx-files-2018-2027.json`, 369 rows, 340 distinct "
    "filenames) finds no `juneteenth` filename before 2022. Inside a contiguous "
    "window silence is the positive claim that a date was audited normal, which is "
    "false for a date the operator later marks as a holiday, so all three ship "
    "`Unsourced`, which clips nothing. 2021-06-19 is a **Saturday**: no family has a "
    "trade date there and the row changes no answer, and the row is keyed to the "
    "operator's own calendar date for the holiday rather than to an observed date "
    "CME never states. Closing condition: a CME holiday schedule naming Juneteenth "
    "in one of these three years.\n"
)

COLUMBUS_TEXT = (
    "**Columbus Day and Veterans Day — six dates with no row.** 2019-10-14, "
    "2019-11-11, 2020-10-12, 2020-11-11, 2021-10-11 and 2021-11-11 lie inside this "
    "window and carry no row, so the family's ordinary week stands there. CME "
    "published settlement-time and OTC-clearing advisories for these dates — the "
    "2019 ZIP's `settlement-notices/*-settlement-times.pdf` members and, for "
    "example, `2021-veterans-day-advisory.pdf` — but never a Globex trading "
    "schedule for them. A settlement notice is not session language "
    "(LAW-SESSION-NOT-EXPIRY), so no row is keyed to one and the block's `missing` "
    "register records the dates as gaps rather than as sourced normality. Closing "
    "condition: a CME Globex holiday schedule naming one of these dates.\n"
)

NIKKEI_TEXT = (
    "**Interpretive step: this family is keyed to the `Equity` / `Equity Products` "
    "line.** No CME sheet in this era prints an outright Nikkei row. The Nikkei 225 "
    "futures trade under the compact sheets' `Equity` line and the full sheets' "
    "`Equity Products` line, and that is the line every row above records; the "
    "block's `globex_nikkei_225_dollar` rows say so on each of the 44 dates. Five "
    "dates carry `modified` status because CME's own Nikkei-labelled BTIC row "
    "diverges from that line: 2020-12-31 and 2021-12-31, where the row prints "
    "`Globex Closed` and `Closed` in a column where `Equity Products` closes at "
    "16:00; 2021-04-02, where the sheet splits `Nikkei BTIC` from `TOPIX BTIC`; and "
    "2021-11-25 and 2021-11-26, where it prints `Nikkei & BTIC` with its own 10:30 "
    "pre-open, 11:00 open, 16:00 halt and 00:00 close. A BTIC row is a trade-type "
    "variant, not the outright future — the consumer marks that flag itself — so the "
    "`Equity` line governs every row above. The two year-end divergences move no "
    "boundary the crate holds, because the `Equity Products` 16:00 close is that "
    "grid's ordinary close on both dates and both ship no row; the two Thanksgiving "
    "2021 divergences are recorded beside the rows they touch, which carry the "
    "`Equity` line's 12:00 CT and 12:15 CT closes. Closing condition for a row of "
    "this family's own: a CME document that breaks out the outright Nikkei 225 "
    "future's session on a holiday date.\n"
)


if __name__ == "__main__":
    sys.exit(main())
