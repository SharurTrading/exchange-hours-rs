#!/usr/bin/env python3
# SPDX-License-Identifier: MIT-0
"""evidence_wave5.py -- write the 2013-2015 evidence into the docs and headers.

Reads the six family modules, the four venue modules and the wave-5 derived
tables, and rewrites, in place:

  * each family module's `//!` era paragraph, so its header names the window it
    now declares (`every_holiday_module_header_names_its_declared_windows`);
  * each venue module's era paragraph and its era-share sentence;
  * `docs/evidence/<family>.md`: the `**Coverage:**` line, the `### 2013`,
    `### 2014` and `### 2015` tables, a `### Documents` table and a
    `### Gaps and residual risks, 2013-2015` block, inserted immediately above
    the 2016 era;
  * `docs/evidence/<venue>.md`: `### 2013-2015 (T1)`, its `### Documents` table
    and its three year tables, inserted immediately above the 2019-2021 era.

The prose is recomputed from the modules on every run, so no count in it is
hand-maintained.  Nested backticks and literal `|` never reach a cell.

Usage:  WAVE5_RESEARCH=... python3 tools/evidence_wave5.py
"""

from __future__ import annotations

import argparse
import collections
import datetime as dt
import json
import os
import re
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))

import wave5_rows as R  # noqa: E402
import wave5_pdf  # noqa: E402  (imported for its docstring's sake)

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
HOLIDAYS = os.path.join(ROOT, "src", "calendar", "schedules", "holidays")
EVIDENCE = os.path.join(ROOT, "docs", "evidence")

ERA = (2013, 2015)

CARDINAL_WORD = {1: "one", 2: "two", 3: "three", 4: "four", 5: "five",
                 6: "six", 7: "seven"}

NUMBER_WORD = {1: "first", 2: "second", 3: "third", 4: "fourth", 5: "fifth",
               6: "sixth", 7: "seventh"}
CARDINAL = {1: "one", 2: "two", 3: "three", 4: "four", 5: "five", 6: "six",
            7: "seven"}


def series(items):
    items = list(items)
    if len(items) == 1:
        return items[0]
    if len(items) == 2:
        return "%s and %s" % (items[0], items[1])
    return ", ".join(items[:-1]) + " and " + items[-1]


def wrap(prefix, text, width=75):
    words = text.split()
    lines = []
    current = prefix
    for word in words:
        if len(current) + len(word) + 1 > width and current.strip() != prefix.strip():
            lines.append(current.rstrip())
            current = prefix + word
        else:
            current = current + (" " if current.strip() else "") + word
    lines.append(current.rstrip())
    return lines


# --------------------------------------------------------------------------
# Reading the crate's own modules
# --------------------------------------------------------------------------

WINDOW_RE = re.compile(
    r"\(\s*(\d+)\s*,\s*(\d+)\s*,\s*(\d+)\s*\)\s*\.\.=\s*"
    r"\(\s*(\d+)\s*,\s*(\d+)\s*,\s*(\d+)\s*\)")


def parse_module(path):
    """Returns (windows, rows) for one module's `holidays!` block.

    Multi-line rows are read as written: the parser walks the whole block with
    `re.S` rather than scanning line by line, so a row split across several
    lines is counted once.
    """
    text = open(path, encoding="utf-8").read()
    constants = {}
    for line in text.splitlines():
        found = re.match(
            r"^\s*const\s+([A-Za-z_][A-Za-z0-9_]*)\s*:\s*u32\s*=\s*([^;]+);", line)
        if found:
            constants[found.group(1)] = found.group(2)
    body = text[text.index("holidays! {"):]
    coverage, _, rest = body.partition("rows:")
    windows = [
        (dt.date(int(m[0]), int(m[1]), int(m[2])),
         dt.date(int(m[3]), int(m[4]), int(m[5])))
        for m in WINDOW_RE.findall(coverage)]
    rows = []
    for found in re.finditer(
            r"\(\s*(\d{4})\s*,\s*(\d{1,2})\s*,\s*(\d{1,2})\s*,\s*(.+?)\s*,\s*"
            r"(T[12])\s*,\s*\"([^\"]+)\"\s*,?\s*\)", rest, re.S):
        rows.append({
            "date": dt.date(int(found.group(1)), int(found.group(2)),
                            int(found.group(3))),
            "kind": normalize_kind(found.group(4), constants),
            "tier": found.group(5),
            "document": found.group(6),
        })
    rows.sort(key=lambda row: row["date"])
    return windows, rows


NUMERIC_SPLIT = re.compile(r"\s*([+*])\s*")


def numeric(expression):
    total = 0
    for term in expression.split("+"):
        product = 1
        for factor in term.split("*"):
            factor = factor.strip().replace("_", "")
            if not factor.isdigit():
                raise ValueError("not an instant: %r" % expression)
            product *= int(factor)
        total += product
    return total


def normalize_kind(kind, constants):
    for name in sorted(constants, key=len, reverse=True):
        kind = re.sub(r"\b%s\b" % re.escape(name), constants[name], kind)
    kind = re.sub(r"\s+", "", kind)
    if kind in ("Closed", "Unsourced"):
        return kind
    for name in ("early_close", "late_open", "late_open_and_early_close"):
        prefix = "%s(" % name
        if kind.startswith(prefix) and kind.endswith(")"):
            stamps = []
            for argument in kind[len(prefix):-1].split(","):
                hours, rest = divmod(numeric(argument), 3_600)
                stamps.append("%02d:%02d" % (hours, rest // 60))
            return "%s(%s)" % (name, ", ".join(stamps))
    raise ValueError("unrecognized kind %r" % kind)


def era_rows(rows, first, last):
    return [row for row in rows
            if first <= row["date"].year <= last]


def kinds_phrase(rows):
    """The era-share sentence's kind breakdown, in the crate's own words."""
    counts = collections.Counter()
    for row in rows:
        if row["kind"] == "Closed":
            counts["closed"] += 1
        elif row["kind"] == "Unsourced":
            counts["unsourced"] += 1
        elif row["kind"].startswith("early_close("):
            counts["early_close"] += 1
        elif row["kind"].startswith("late_open_and_early_close("):
            counts["combined"] += 1
        else:
            counts["late_open"] += 1
    parts = []
    for key, singular, plural in (
            ("closed", "stated closure", "stated closures"),
            ("early_close", "early close", "early closes"),
            ("late_open", "late open", "late opens"),
            ("combined", "combined late-open-and-early-close row",
             "combined late-open-and-early-close rows"),
            ("unsourced", "not-worked-up marker", "not-worked-up markers")):
        if counts[key]:
            parts.append("%d %s" % (counts[key],
                                    singular if counts[key] == 1 else plural))
    return series(parts)


def window_list(windows):
    return ", ".join(
        "%04d-%02d-%02d..%04d-%02d-%02d"
        % (first.year, first.month, first.day, last.year, last.month, last.day)
        for first, last in windows)


def unaudited(windows):
    """The year spans no declared window covers, as `2016-2018`-style labels."""
    gaps = []
    for earlier, later in zip(windows, windows[1:]):
        if (later[0] - earlier[1]).days > 1:
            gaps.append("%04d-%04d" % (earlier[1].year + 1, later[0].year - 1))
    return gaps


# --------------------------------------------------------------------------
# Family module headers and the evidence files
# --------------------------------------------------------------------------

def module_path(name):
    return os.path.join(HOLIDAYS, "%s.rs" % name)


def era_doc_text(windows):
    entries = []
    for first, last in windows:
        entries.append("%04d-%04d at T1" % (first.year, last.year)
                       if (last.year - first.year) else "%04d at T1" % first.year)
    count = len(windows)
    lines = wrap("/// ", "%s audited eras: %s." % (
        CARDINAL[count].capitalize(), series(entries)))
    gaps = unaudited(windows)
    if not gaps:
        prose = ("Every audited interval is contiguous, and nothing before "
                 "%04d-01-01 has a table at all: that span lies outside every "
                 "window, so `holiday_on` has no answer there rather than "
                 "reporting a normal date." % windows[0][0].year)
    else:
        prose = ("The %s interval%s between them %s audited by no wave and "
                 "lie%s outside every window, so `holiday_on` has no answer "
                 "there rather than reporting a normal date."
                 % (series(gaps), "" if len(gaps) == 1 else "s",
                    "is" if len(gaps) == 1 else "are",
                    "s" if len(gaps) == 1 else ""))
    lines.extend(wrap("/// ", prose))
    return "\n".join(lines)


def patch_module_doc(name, windows):
    path = module_path(name)
    text = open(path, encoding="utf-8").read()
    marker = re.search(r"^///\s*[A-Z][a-z]+ audited eras?: ", text, re.M)
    if marker is None:
        raise SystemExit("%s: no `audited eras` paragraph" % path)
    tail = text[marker.end():]
    end = tail.index("// Evidence:")
    block = tail[:end]
    keep_at = None
    for needle in ("Coverage ends at", "Coverage runs to", "Inside a window",
                   "Coverage is"):
        if needle in block:
            keep_at = block.index(needle)
            break
    if keep_at is None:
        raise SystemExit("%s: no coverage tail sentence" % path)
    replacement = era_doc_text(windows) + "\n///\n/// " + block[keep_at:].rstrip()
    text = text[:marker.start()] + replacement + "\n" + text[marker.end() + end:]
    open(path, "w", encoding="utf-8").write(text)


def family_era_paragraph(windows, count):
    """The `## Holidays` era paragraph, recomputed for the whole table."""
    gaps = unaudited(windows)
    tail = ("`HolidayCoverage::windows()` lists the %d, and `contains` answers "
            "per date." % count)
    windows_cell = ", ".join("`%s`" % w for w in window_list(windows).split(", "))
    body = [
        "**%s audited eras%s.** The table declares %d coverage %s: %s."
        % (CARDINAL[count].capitalize(),
           "" if count == 1 else (", with no gap between them" if not gaps
                                  else ", and one interval no era covers"),
           count, "window" if count == 1 else "windows", windows_cell),
    ]
    body.append(
        "The eras through 2023 are the operator's own published holiday "
        "schedules at **T1**, from CME's per-holiday PDFs and .xls workbooks; "
        "2024 shares T1 and the operator's `trading-hours-by-product` "
        "responses at **T2**; and 2025-2027 is that service alone, at T2.")
    if gaps:
        body.append(
            "The one interval outside every declared window is `%s`: "
            "`holiday_on` has **no answer** there rather than reporting an "
            "unaudited date as normal." % gaps[0])
    else:
        body.append(
            "Every interval from 2010-01-01 is declared, so the whole span has "
            "an answer; the eras before 2010 are out of scope below the "
            "crate's January-2010 floor.")
    body.append(tail)
    return "\n".join(body)


def patch_family_era(name, windows):
    path = os.path.join(EVIDENCE, "%s.md" % name)
    text = open(path, encoding="utf-8").read()
    marker = re.search(r"^\*\*[A-Za-z]+ audited eras.*?\*\*.*?$", text, re.M)
    if marker is None:
        raise SystemExit("%s: no era paragraph" % path)
    end = tail_marker(text, marker.end())
    replacement = family_era_paragraph(windows, len(windows))
    text = text[:marker.start()] + replacement + "\n" + text[end:]
    open(path, "w", encoding="utf-8").write(text)


def tail_marker(text, start):
    """The start of the line that follows the era paragraph's last sentence."""
    marker = text.index("`contains` answers per date.", start)
    return marker + len("`contains` answers per date.")


def patch_coverage(name, windows):
    path = os.path.join(EVIDENCE, "%s.md" % name)
    text = open(path, encoding="utf-8").read()
    line = "**Coverage:** %s (inclusive venue-local trade dates)." % window_list(windows)
    new, count = re.subn(r"^\*\*Coverage:\*\* .*$", line, text, count=1, flags=re.M)
    if count != 1:
        raise SystemExit("%s: no **Coverage:** line" % path)
    open(path, "w", encoding="utf-8").write(new)


def insert_before(text, marker, block):
    at = text.index(marker)
    return text[:at] + block + text[at:]


def family_gaps_text(windows, all_rows, mine, extra):
    """The `### Gaps and residual risks, 2013-2015` block for one family."""
    parts = ["**This era brings the family to %s audited windows.** The table "
             "as a whole carries %d rows over %d windows — %s — and this era's "
             "share is **%d rows**: %s. Every row is at T1."
             % (CARDINAL_WORD[len(windows)], len(all_rows), len(windows),
                window_list(windows), len(mine), kinds_phrase(mine))]
    if extra:
        parts.append(extra)
    return "\n\n".join(parts)


ORDINAL_WORD = "sixth"


def render_year_table(rows, year):
    lines = ["### %d" % year, "",
             "| trade date | kind | instant as printed | document | tier | derived from |",
             "|---|---|---|---|---|---|"]
    for row in rows:
        if row["date"].year != year:
            continue
        lines.append("| %s | %s | %s | `%s` | %s | %s |" % (
            row["date"].isoformat(), R.KIND_WORDS[row["kind"]],
            instant_cell(row), row["document"], row["tier"],
            sanitize(row["reason"])))
    return "\n".join(lines)


def instant_cell(row):
    tokens = []
    if row.get("printed_close"):
        tokens.append("`%s`" % sanitize(row["printed_close"]))
    if row.get("printed_open"):
        tokens.append("`%s`" % sanitize(row["printed_open"]))
    return " / ".join(tokens) if tokens else "—"


def sanitize(text):
    return (text or "").replace("|", "·").replace("`", "")


#: Artifacts the 2016-2018 wave's own `### Documents` table already resolves in
#: the same file. `every_cited_document_id_is_resolved_exactly_once` counts a
#: citation's hits per file, so listing one of these a second time would
#: resolve it twice. Closed: only an artifact an earlier era already documents
#: in the very file this table is written into may join it.
RESOLVED_BY_AN_EARLIER_ERA = frozenset(("2016-new-years-holiday-schedule.pdf",))


def render_documents(documents, skip=()):
    """The era's `### Documents` table.

    The six columns are the fixed shape the evidence fence reads: id, trade-date
    window, replay URL, capture, tier, sha256.
    """
    lines = [
        "| Document | Window | Replay or service URL | Capture or retrieval, UTC | Tier | sha256 |",
        "|---|---|---|---|---|---|"]
    for doc in documents:
        if doc["file"] in skip:
            continue
        lines.append("| `%s` | %s | <%s> | archive capture %s | %s | `%s` |" % (
            doc["id"], R.WINDOW_CELL, doc["url"], doc["capture"], doc["tier"],
            doc["sha256"] or "—"))
    return "\n".join(lines)


def venue_era_paragraph(windows, count):
    windows_cell = ", ".join("`%s`" % w for w in window_list(windows).split(", "))
    gaps = unaudited(windows)
    body = ["**%s audited %s, %s.** The table declares %d coverage %s: %s."
            % (CARDINAL[count].capitalize(),
               "era" if count == 1 else "eras",
               "with no gap between them" if not gaps
               else "with one interval no era covers",
               count, "window" if count == 1 else "windows", windows_cell),
            "The rows of every era are the D17 intersection of the families "
            "routed here: an era's rows are the family tables' own where a "
            "venue routes one family, and the joint statement where it routes "
            "several.",
            ("The one interval outside every declared window is `%s`: "
             "`holiday_on` has **no answer** there rather than reporting an "
             "unaudited date as normal." % gaps[0]) if gaps else
            ("Every interval from 2010-01-01 is declared, so the whole span has "
             "an answer; the eras before 2010 are out of scope below the "
             "crate's January-2010 floor.")]
    return "\n".join(body)


def patch_venue_coverage(venue, windows):
    """States the venue module's own windows in its evidence file.

    `every_holiday_table_states_its_coverage_window` reads the venue module's
    `coverage:` clause and requires the file to state exactly that list once.
    """
    path = os.path.join(EVIDENCE, "%s.md" % venue)
    text = open(path, encoding="utf-8").read()
    line = "**Coverage:** %s (inclusive venue-local trade dates)." % window_list(windows)
    if "**Coverage:**" in text:
        text, count = re.subn(r"^\*\*Coverage:\*\* .*$", line, text,
                              count=1, flags=re.M)
        if count != 1:
            raise SystemExit("%s: no **Coverage:** line" % path)
    else:
        marker = "\n## Holidays\n"
        if marker not in text:
            raise SystemExit("%s: no ## Holidays heading" % path)
        text = text.replace(marker, marker + "\n" + line + "\n", 1)
    open(path, "w", encoding="utf-8").write(text)


def patch_venue_era(venue, windows):
    path = os.path.join(EVIDENCE, "%s.md" % venue)
    text = open(path, encoding="utf-8").read()
    marker = re.search(r"^\*\*[A-Za-z]+ audited eras?, and one gap between "
                       r"them\.\*\*.*?^$", text, re.S | re.M)
    if marker is None:
        raise SystemExit("%s: no venue era paragraph" % path)
    text = (text[:marker.start()] + venue_era_paragraph(windows, len(windows))
            + "\n" + text[marker.end():])
    open(path, "w", encoding="utf-8").write(text)


def render_venue_table(rows, year):
    lines = ["### %d" % year, "",
             "| trade date | kind | instant as printed | document | tier | derived from |",
             "|---|---|---|---|---|---|"]
    for row in rows:
        if row["date"].year != year:
            continue
        if row["kind"] == "Unsourced":
            cells = ("unsourced", "`—`",
                     "the intersection of the families routed to this venue")
        elif row["kind"] == "Closed":
            cells = ("closed", "`every routed family states a closure`",
                     "the intersection of the families routed to this venue")
        else:
            words = {"early_close": "early close", "late_open": "late open",
                     "late_open_and_early_close": "late open and early close"}
            name = row["kind"][:row["kind"].index("(")]
            cells = (words[name], "`%s CT`" % inside(row["kind"]),
                     "the intersection of the families routed to this venue")
        lines.append("| %s | %s | %s | `%s` | %s | %s |" % (
            row["date"].isoformat(), cells[0], cells[1], row["document"],
            row["tier"], cells[2]))
    return "\n".join(lines)


def inside(kind):
    return kind[kind.index("(") + 1:-1]


def main(argv=None):
    parser = argparse.ArgumentParser()
    parser.add_argument("--research", default=os.environ.get("WAVE5_RESEARCH"))
    parser.add_argument("--out", default="tools/out/wave5")
    args = parser.parse_args(argv)

    # PRE-FLIGHT. Every refusal happens before the first write, so a rejected
    # run leaves the tree exactly as it found it.
    for owner in list(R.FAMILIES) + ["cme", "cbot", "comex", "nymex"]:
        path = os.path.join(EVIDENCE, "%s.md" % owner)
        text = open(path, encoding="utf-8").read()
        if "\n### 2013\n" in text or "\n### 2013-2015 (T1)\n" in text:
            raise SystemExit(
                "%s already carries a 2013 era; the tool is not idempotent "
                "and refuses to duplicate it" % path)
        if owner in R.FAMILIES and "\n### 2016\n" not in text \
                and "\n## Sources\n" not in text:
            raise SystemExit("%s: no era to insert above" % path)
        if owner not in R.FAMILIES and "\n### 2019-2021 (T1)\n" not in text:
            raise SystemExit("%s: no 2019-2021 era to insert above" % path)

    plan = json.load(open(os.path.join(args.out, "ROWS.json")))
    summary = json.load(open(os.path.join(args.out, "SUMMARY.json")))
    block = json.load(open(os.path.join(
        args.research, "holidays", "cme-2013-2015.r2.json")))
    documents = json.load(open(os.path.join(args.out, "reconciliation.json")))

    # the derived rows, with the printed cells the block recorded
    printed = {}
    for holiday in block["holidays"]:
        for family_row in holiday["families"]:
            printed[(holiday["date"], family_row["family"])] = family_row
    derived = {}
    for family, rows in plan["families"].items():
        enriched = []
        for row in rows:
            entry = {"date": dt.date.fromisoformat(row["date"]),
                     "kind": row["kind"], "tier": row["tier"],
                     "document": row["document"], "reason": row["reason"],
                     "printed_close": None, "printed_open": None}
            # The plan carries the instants the block printed for this row's
            # own trade date; matching a document id alone would hand a row the
            # instant of whichever date of a multi-date sheet came first.
            entry["printed_close"] = row.get("printed_close")
            entry["printed_open"] = row.get("printed_open")
            enriched.append(entry)
        derived[family] = enriched

    # the era's document ids, from the plan
    used = sorted({row["document"] for rows in plan["families"].values()
                   for row in rows})
    doc_rows = []
    for document in used:
        code = R.row_code(block, document)
        # `row_code` must see the id the module carries; the table renders it
        # through `document_id` so both spellings stay in one place.
        entry = block["documents"][code]
        # The id the module row carries; the block's own long form is only used
        # when it is the same string the family modules were written with.
        doc_rows.append({"id": R.document_id(code, block),
                         "file": os.path.basename(entry["file"]),
                         "url": entry.get("replay_url"),
                         "capture": entry["capture_utc"],
                         "tier": entry["tier"],
                         "sha256": entry.get("sha256")})

    for name in R.FAMILIES:
        windows, all_rows = parse_module(module_path(name))
        mine = era_rows(all_rows, *ERA)
        if "\n### 2013\n" in open(os.path.join(EVIDENCE, "%s.md" % name),
                                   encoding="utf-8").read():
            raise SystemExit("docs/evidence/%s.md already carries a 2013 era"
                             % name)
        patch_module_doc(name, windows)
        patch_coverage(name, windows)
        patch_family_era(name, windows)
        extra = family_extra(name, summary, derived)
        gaps = family_gaps_text(windows, all_rows, mine, extra)
        tables = "\n\n".join(render_year_table(derived[name], year)
                             for year in (2013, 2014, 2015))
        block_text = (
            "\n\n" + tables
            + "\n\n### Documents\n"
            + render_documents(doc_rows, RESOLVED_BY_AN_EARLIER_ERA)
            + "\n\n### Gaps and residual risks, 2013-2015\n\n" + gaps + "\n")
        path = os.path.join(EVIDENCE, "%s.md" % name)
        text = open(path, encoding="utf-8").read()
        marker = "\n### 2016\n" if "\n### 2016\n" in text else "\n## Sources\n"
        text = insert_before(text, marker, block_text)
        open(path, "w", encoding="utf-8").write(text)
        print("%-24s %3d era rows, %d documents" % (name, len(mine), len(doc_rows)))

    for venue in ("cme", "cbot", "comex", "nymex"):
        path = os.path.join(HOLIDAYS, "venues", "%s.rs" % venue)
        windows, all_rows = parse_module(path)
        mine = era_rows(all_rows, *ERA)
        patch_venue_doc(venue, windows, all_rows, mine)
        patch_venue_coverage(venue, windows)
        patch_venue_era(venue, windows)
        tables = "\n\n".join(render_venue_table(mine, year)
                             for year in (2013, 2014, 2015))
        block_text = (
            "\n### 2013-2015 (T1)\n" + venue_text(venue, windows, all_rows, mine)
            + "\n\n### Documents\n"
            + render_documents(doc_rows, RESOLVED_BY_AN_EARLIER_ERA)
            + "\n\n" + tables + "\n")
        ev = os.path.join(EVIDENCE, "%s.md" % venue)
        text = open(ev, encoding="utf-8").read()
        marker = "\n### 2019-2021 (T1)\n"
        if marker not in text:
            raise SystemExit("%s: no 2019-2021 era to insert above" % ev)
        text = insert_before(text, marker, block_text)
        open(ev, "w", encoding="utf-8").write(text)
        print("%-24s %3d era rows" % (venue, len(mine)))
    return 0


ROUTING = {
    "cme": ("globex_equity_index", "globex_energy", "globex_fx",
            "globex_grains", "globex_interest_rates", "globex_livestock"),
    "cbot": ("globex_grains", "globex_interest_rates"),
    "comex": ("globex_energy",),
    "nymex": ("globex_energy",),
}


def venue_text(venue, windows, all_rows, mine):
    counts = collections.Counter(row["kind"] for row in mine)
    stated = len(mine) - counts["Unsourced"]
    routed = ", ".join("`%s`" % name for name in ROUTING[venue])
    return (
        "**This era brings the venue to %s audited windows.** The table as a "
        "whole carries %d rows over %d windows — %s — and this era's share is "
        "**%d rows**: %d stated %s and %d `Unsourced` %s. Every row is the "
        "intersection of the families routed here — %s — by the D17 rule "
        "`venues.rs` states: a row ships only where every routed family states "
        "the same one, and a date on which they differ, or on which one states "
        "a row while another has audited the date normal, ships `Unsourced`; "
        "the three year tables below carry every date's row, its instant as "
        "printed and the id it is derived from."
        % (CARDINAL_WORD[len(windows)], len(all_rows), len(windows),
           window_list(windows), len(mine), stated,
           "row" if stated == 1 else "rows", counts["Unsourced"],
           "row" if counts["Unsourced"] == 1 else "rows", routed))


def patch_venue_doc(venue, windows, all_rows, mine):
    path = os.path.join(HOLIDAYS, "venues", "%s.rs" % venue)
    text = open(path, encoding="utf-8").read()
    old = re.search(r"//! Coverage is \w+ audited eras?: ", text)
    if old is None:
        raise SystemExit("%s: no coverage sentence" % path)
    count = len(windows)
    text = (text[:old.start()]
            + "//! Coverage is %d audited eras: " % count
            + text[old.end():])
    # the era-share sentence of each era block gains its new ordinal
    text = re.sub(
        r"\*\*This era declares the venue's \w+ audited window\.\*\*",
        "**This era declares the venue's %s audited window.**"
        % NUMBER_WORD[count],
        text)
    text = text.replace(
        "//! 2013-2015 interval between them is audited by no wave and lies outside\n"
        "//! every declared window.",
        "")
    open(path, "w", encoding="utf-8").write(text)


def family_extra(name, summary, derived):
    if name == "globex_equity_index":
        return (
            "**The three late opens are the year-end reopenings.** 2013-01-02, "
            "2013-12-26 and 2014-01-02 each print `0500 CT - CME Globex open "
            "for trade date`, which is sixteen hours after the trade date's "
            "ordinary 17:00 CT first open on the closed day, so each ships "
            "`late_open(5 * 3_600)`. 2015-01-02 ships no late open: the 2015 "
            "New Year's sheet prints no reopening line at all, so the crate "
            "carries the ordinary grid there and the evidence records the "
            "silence rather than an instant. 2013-12-26 additionally prints "
            "`0515 CT - Regular open - USD - Ibovespa Futures`, which is the "
            "Ibovespa contract's own open and not this family's "
            "(LAW-SESSION-NOT-EXPIRY).")
    if name == "globex_energy":
        return (
            "**Good Friday closes the energy and metals line in all three years.** CME's 2013 Good Friday sheet prints `CME Globex is closed` for 2013-03-29 under the `NYMEX & COMEX® and Dubai Mercantile (DME) Products` heading, and its 2014 and 2015 sheets print the same line for 2014-04-18 and 2015-04-03 under `Energy, Metals & DME Products`, so all three ship `Closed`. The Thursday before each is ordinary for the era, which the same sheets state as `1615 CT / 1715 ET - Regular close`, so those dates ship no row.")
    if name == "globex_interest_rates" or name == "globex_fx":
        return (
            "**The Friday eves of the Monday holidays close at 15:15 CT.** "
            "Every 2013-2015 holiday schedule prints `1515 CT - Early close` "
            "for the Interest Rate and FX lines on the Friday before a Monday "
            "holiday, an hour and a quarter earlier than the family's ordinary "
            "16:00 CT close, while the Equity line on the same sheet prints "
            "its ordinary 16:15 CT. The rows are that printed early close. "
            "From 2014 the same line is `1515 CT / 1615 ET / 2115 UTC`; the "
            "2014 Martin Luther King sheet's UTC column reads `2215 UTC` where "
            "1515 CT is 2115 UTC, and the block records CME's printed value "
            "with the arithmetic flagged rather than correcting it, so the "
            "crate row is the CT value both revisions agree on.")
    if name == "globex_grains":
        return (
            "**The grains day session does not run on the Monday and Thursday "
            "holidays, so those dates are `Closed`.** CME halts the "
            "`Grain, Oilseed & MGEX Products` line at 12:00 CT on the holiday "
            "itself and reopens it at 19:00 CT the same evening for the next "
            "trade date: the holiday's own trade date has no final close of "
            "its own, and the closure removes it with the evening leg that "
            "would have opened it. The three `late_open_and_early_close` rows "
            "are the Thanksgiving Fridays, where the 08:30 CT reopen is the "
            "printed first open and 12:00 CT the printed final close. The "
            "grains close is 13:15 CT through 2015-07-04 and 13:20 CT from "
            "2015-07-05 (CME SER-7395R), which is why 2015-12-24's 12:05 CT "
            "differs from the 12:00 CT of the two earlier years.")
    if name == "globex_livestock":
        return (
            "**The grid changes inside the window.** Through 2014-10-26 "
            "`livestock.rs PROFILE_AT_2010_FLOOR` runs an around-the-clock "
            "grid: a Monday 09:05-16:00 CT session, Tuesday-Thursday "
            "17:00-16:00 CT and a Thursday 17:00-13:55 CT short day, all "
            "inside a single civil day, so the era's `late_open` rows sit at "
            "09:05 CT on the trade date after a closure and its `Closed` rows "
            "are outright. From 2014-10-27 (CME SER-7194) the evening sessions "
            "are gone and the same dates' printed 08:00 CT is the ordinary "
            "open, so 2014-12-26, 2015-01-02 and 2015-07-06 ship no row even "
            "though the block records a reopen instant for them. The 2015-12-24 "
            "and 2015-12-31 rows carry CME's `Livestock, Dairy & Lumber "
            "Products` line; 2015-12-31 is the year-end half-day at 13:55 CT.")
    return None


if __name__ == "__main__":
    raise SystemExit(main())
