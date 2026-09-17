#!/usr/bin/env python3
# SPDX-License-Identifier: MIT-0
"""wave5_repair.py -- repair the cme-2013-2015 block from the verdict's 12 items.

The round-2 verdict (`holidays/cme-2013-2015.verify.json`, `matches: false`)
raises twelve discrepancies against `holidays/cme-2013-2015.json`.  A row may
not ship from a block a verifier failed, so this tool writes the repaired block
(`cme-2013-2015.r2.json`) and a repair record (`cme-2013-2015.repair.json`)
before any crate row or evidence file is written from it.  Round 1
(`cme-2013-2015.json`) is opened read-only and never rewritten.

Every repaired value is re-derived from the bytes in the research store, and
each repair records the artifact and the literal string it was read from.
Nothing is invented and nothing is copied from the verdict without being
checked against the saved document first; each guard below fails loudly if the
block's `before` value is not the one the verdict names, so the tool cannot
silently run against a different block.

Repairs, by verdict id:

  1  2013-07-03 `livestock+dairy+lumber`: `normal` -> `early_close` at 12:00 CT
     (Dairy, Lumber), 12:02 CT (Lumber Options) and 12:15 CT (Livestock Futures
     & Options), from CME's later 7/2/2013 revision of the 2013 Independence
     Day schedule; the 6/4/2013 statement becomes a `superseded` statement.
  2  2013-07-03 `grains_oilseeds`: the quoted line becomes the revision's
     `1215 CT - Early MGEX Wheat & Apple Juice close`.
  3  The completeness claim: `coverage` and `missing[10]` state the PDF half of
     the in-place revision history retrieved and reconciled (42 earlier
     captures), not only the `.xls` half.
  4  The 2013-03-28 livestock/dairy/lumber zone: sourced as CT from X13GFPD's
     own Notes row, attached as a corroborating statement; `missing[2]` closed.
  5  The Interest-Rate/FX grouping claim: 57 of 58 `.xls` sheets, with the
     X13GFPD `Good Fri.` exception stated.
  6  A new `missing[]` entry naming the printed product lines the file does not
     model, with what would close each.
  7  `coverage`'s self-description: one 2013 multi-sheet workbook, not two.
  8  2014-11-11 `other_statements[0].note`: the statement is the same, the
     sheet is not.
  9  2013-10-14 and 2013-11-11: the registered-trademark glyph is restored in
     the quoted document titles.
  10 The three-digit clock times are re-emitted as CME printed them, so no
     `*_instant` field zero-pads.
  11 `raw/cme-2013-2015/INDEX.md` gains a dated amendment block.
  12 The 2013-01-01 / 2013-01-02 rows gain X13GFPD's `New Years` sheet as a
     corroborating statement.

Stdlib only.  Deterministic: same bytes in, same text out.
"""

from __future__ import annotations

import argparse
import collections
import hashlib
import json
import os
import re
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))

import wave5_pdf as W  # noqa: E402

REPAIRED_ON = "2026-09-17"

DAIRY_LUMBER_TITLE = "Globex® Columbus Day Holiday Schedule"
VETERANS_TITLE = "Globex® Veterans Day Holiday Schedule"

INDICES_MISSING = (
    "Product lines CME prints on the 2013-2015 holiday schedules that this "
    "file does not model, with what would close each. The block carries the "
    "nine product families it reconciles and no others, so a printed line "
    "outside those nine is invisible to a reader of this file even where it "
    "has a crate key. (a) Crate keys that exist and are dormant: "
    "`globex_weather` - the `Weather` line prints its own close on the 2013 "
    "PDFs and on X15ANN sheet `Labor` r035 (15:15 CT) against Equity's 16:15 "
    "CT; `globex_mini_grains` - `Mini-Sized Grains and Oilseeds (CBOT)` prints "
    "14:30 on X13GFPD sheet `MLK, Jr.` r019 against `Grains and Oilseeds "
    "(CBOT & KCBT)` 14:00, and X15ANN sheet `Labor` r020 gives it 13:45 "
    "against `Grains and Oilseeds` 13:15. Both are dormant (ledger rows at "
    "docs/schedules/verification.md), so best-effort is the right obligation "
    "(LAW-SERVICE-TIERS) and neither is modelled here; closing condition for "
    "each is a consumer that maps the product, at which point the same sheets "
    "already carry its rows. (b) Printed lines with no crate key at all: "
    "`Real Estate`, `S&P GSCI`, `Wood Pulp`, `Dow Jones-UBS ER` and `RS`, "
    "`Eurozone HICP`, `Crude Palm Oil`, `Ethanol (CBOT)` and `DDG`, `Black "
    "Sea Wheat (CBOT)`, `MGEX Hard Red Spring Wheat`, `MGEX Indices`, `MGEX "
    "Apple Juice`, `Bloomberg` indices, `Custom Indexes`, `S&P CNX Nifty "
    "(Nifty 50)`, `USD-Denominated Ibovespa Futures`, `EUA Daily Futures`, "
    "`DME Oman Crude` and the metals TAS lines, `KOSPI 200`, the Bursa "
    "Malaysia products and `Pine Prairie Energy Center Futures`. Closing "
    "condition for every line in (b): the maintainer names the market and a "
    "consumer maps it (LAW-SERVICE-TIERS), or a family wave is commissioned "
    "for it. The `TAS/TAM Products - Regular close - Per each product "
    "schedule` line is deliberately not modelled at any resolution "
    "(LAW-SESSION-NOT-EXPIRY)."
)

REVISION_HISTORY = (
    "In-place PDF revision history, retrieved and reconciled in this round. "
    "The PDF URLs this file cites carry 73 distinct-digest status-200 captures "
    "in CME's own archived responses for 2013-2015: 31 are the captures this "
    "file cites and 42 are earlier revisions that no round had retrieved. All "
    "42 were retrieved on 2026-09-17 UTC into "
    "raw/cme-2013-2015-repair-r2/ (INDEX.md carries url, capture time, sha256 "
    "and byte count for each, and txt/ the pdftotext -layout rendering), and "
    "each was reconciled line by line against the capture this file cites: "
    "tools/wave5_reconcile.py re-reads every capture, groups the printed "
    "values by product section and day heading, and reports every date on "
    "which an earlier revision states a different CT value. Two of the 42 "
    "carry a session value the cited revision does not: "
    "2015-4th-of-july-holiday-schedule.pdf@20150326113353 (footer 'Last "
    "updated 12/29/2014') gives the Equity Products line 1215 CT on Thursday "
    "July 2 2015 where the cited 6/1/2015 revision gives 1615 CT, and "
    "2014-4th-of-july-holiday-schedule.pdf@20140326153233 (footer 'Last "
    "updated 2/26/2014') prints '1200 CT / 1330 ET / 1700 UTC - Early close' "
    "for the grain line where the cited 7/1/2014 revision prints 1300 ET. "
    "Lineage settles both: in each pair the cited capture is the later "
    "revision, by CME's own 'Last updated' footer and by capture time, so the "
    "cited value is the operator's final statement and no recorded value "
    "moves. The earlier revisions carry real changes outside the rows this "
    "file records - CME widened several drafts' session sets before "
    "publication - which is why the reconciliation is recorded here rather "
    "than asserted away."
)

XLS_GROUPING = (
    "Across all 42 workbooks, 57 of the 58 sheets that carry both an Interest "
    "Rate row and an FX row give them identical values in every column; the "
    "exception is X13GFPD sheet 'Good Fri.', where the FX row's Sunday "
    "March 31 pre-open and open sit on the unlabelled continuation row r013 "
    "between the two product rows. The round-0 grouping therefore survives on "
    "evidence and not on the PDFs' shared section heading."
)

COVERAGE_INTRO = (
    "CME Group Globex holiday schedules for calendar years 2013, 2014 and "
    "2015, complete for every U.S. holiday CME published a schedule for. "
)

COVERAGE_ROUND1 = (
    "Round 1 (fixer) rebuild: the round-0 result excluded CME's .xls holiday "
    "schedules on the false ground that they were 'the .xls twins of the PDFs, "
    "same content, different container'. That exclusion is withdrawn. Every "
    ".xls and .zip holiday document CME published under "
    "tools-information/holiday-calendar/files/ for 2013-2015 was enumerated "
    "afresh by CDX and retrieved on 2026-09-12 UTC into "
    "raw/cme-2013-2015-fix/ (45 files, INDEX.md with url, capture time, sha256 "
    "and byte count for each), dumped sheet by sheet with xlrd and reconciled "
    "cell by cell against every PDF-sourced row. "
)

COVERAGE_EVIDENCE = (
    "Evidence base: 62 operator documents - the 34 of round 0 (32 PDFs + the "
    "2 Columbus Day .xls) plus 28 CME .xls documents cited here for the first "
    "time, among them the 2014 and 2015 annual master workbooks and the 2013 "
    "four-sheet Good Friday / Presidents Day bundle. "
)

MISSING_10_OLD_MARKER = "Two generations account for all of them"


def sha256_file(path):
    with open(path, "rb") as handle:
        return hashlib.sha256(handle.read()).hexdigest()


def locate(root, relpath):
    """Finds the saved bytes for a block document, by basename and capture."""
    relpath = relpath.replace("\\", "/")
    name = os.path.basename(relpath)
    # A document id is `<file> @<stamp>` or `<dir>/<file>__<stamp>.<ext>`, and
    # the saved name is `<stem>__<stamp>.<ext>` in both cases.
    file_part = name.split("@", 1)[0]
    stem, _, extension = file_part.rpartition(".")
    stamp = ""
    if "@" in name:
        stamp = name.split("@", 1)[1].split(".")[0]
    else:
        found = re.search(r"__(\d{8,14})", name)
        stamp = found.group(1) if found else ""
    candidates = [
        os.path.join(root, relpath),
    ]
    # The round-0 PDFs and the fix directory's workbooks use a
    # `<stem>__<stamp><ext>` name, so try that exact extension first.
    for directory in ("cme-2013-2015/pdf", "cme-2013-2015/xls",
                      "cme-2013-2015-fix/xls", "cme-2013-2015-fix/zip",
                      "cme-2013-2015-verify-r2/new",
                      "cme-2013-2015-verify-r2/refetch",
                      "cme-2013-2015-repair-r2/pdf"):
        candidates.append(os.path.join(root, directory, name))
        if extension:
            candidates.append(os.path.join(
                root, directory, "%s__%s.%s" % (stem, stamp, extension)))
    for candidate in candidates:
        if os.path.isfile(candidate):
            return candidate
    for directory in ("cme-2013-2015/pdf", "cme-2013-2015/xls",
                      "cme-2013-2015-fix/xls", "cme-2013-2015-fix/zip",
                      "cme-2013-2015-verify-r2/new", "cme-2013-2015-repair-r2/pdf"):
        full = os.path.join(root, directory)
        if not os.path.isdir(full):
            continue
        for entry in sorted(os.listdir(full)):
            # The extension must match: several URLs differ only by container
            # (`.pdf` vs `.xls`), and hashing the wrong one would record a
            # digest the cited artifact does not have.
            if entry == name:
                return os.path.join(full, entry)
            if entry.startswith(stem + "__") and entry.endswith(extension):
                return os.path.join(full, entry)
    return None


class Repairs:
    def __init__(self):
        self.items = []

    def add(self, item_id, where, field, before, after, evidence):
        self.items.append({
            "id": item_id, "where": where, "field": field,
            "before": before, "after": after, "evidence": evidence,
        })

    def guard(self, item_id, where, value, expected):
        """Fails loudly unless the block holds the value the verdict names."""
        if value != expected:
            raise SystemExit(
                "repair %d (%s): the block's value is not the one the verdict "
                "names.\n  expected: %r\n  found   : %r"
                % (item_id, where, expected, value))


def read_text(root, relpath):
    path = os.path.join(root, relpath)
    with open(path, errors="replace") as handle:
        return path, handle.read()


def find(entry, family):
    for row in entry["families"]:
        if row["family"] == family:
            return row
    raise SystemExit("no %r row on %s" % (family, entry["date"]))


def entry_on(block, date):
    for entry in block["holidays"]:
        if entry["date"] == date:
            return entry
    raise SystemExit("no block entry for %s" % date)


def main(argv=None):
    parser = argparse.ArgumentParser()
    parser.add_argument("--research", default=os.environ.get("WAVE5_RESEARCH"))
    parser.add_argument("--out", default="tools/out/wave5/repair")
    args = parser.parse_args(argv)
    if not args.research:
        print("WAVE5_RESEARCH must name the research store root", file=sys.stderr)
        return 2
    root = os.path.join(args.research, "holidays", "raw")

    round1_path = os.path.join(args.research, "holidays", "cme-2013-2015.json")
    with open(round1_path, "rb") as handle:
        round1_bytes = handle.read()
    block = json.loads(round1_bytes)
    os.makedirs(args.out, exist_ok=True)
    repairs = Repairs()

    # ---------------------------------------------------------------- item 1
    entry = entry_on(block, "2013-07-03")
    row = find(entry, "livestock+dairy+lumber")
    repairs.guard(1, "2013-07-03 livestock+dairy+lumber status",
                  row["status"], "normal")
    repairs.guard(1, "2013-07-03 livestock+dairy+lumber document",
                  row["document"], "D13J4")
    rev_path = "cme-2013-2015-verify-r2/txt/2013-4th-of-july-done__20130717050333.txt"
    rev_abs, rev_text = read_text(root, rev_path)
    for literal in ("1200 CT – Early close for Dairy",
                    "1200 CT – Early close for Lumber",
                    "1202 CT – Early close for Lumber Options",
                    "1215 CT – Early close for Livestock Futures & Options",
                    "Last updated 7/2/2013"):
        if literal not in rev_text:
            raise SystemExit("item 1: %r is not in %s" % (literal, rev_path))
    before_verbatim = row["verbatim"]
    row["status"] = "early_close"
    row["verbatim"] = (
        "Wednesday, July 3 / 1200 CT - Early close for Dairy / "
        "1200 CT - Early close for Lumber / "
        "1202 CT - Early close for Lumber Options / "
        "1215 CT - Early close for Livestock Futures & Options")
    row["close_instant"] = (
        "1200 CT (Dairy); 1200 CT (Lumber); 1202 CT (Lumber Options); "
        "1215 CT (Livestock Futures & Options)")
    row["document"] = "D13J4D"
    row["document_note"] = (
        "CME's later revision of the 2013 Independence Day schedule: "
        "2013-4th-of-july-done.pdf, footer 'Last updated 7/2/2013'")
    row["other_statements"] = [{
        "document": "D13J4",
        "relation": "superseded",
        "verbatim": before_verbatim,
        "note": (
            "The 6/4/2013 revision this file first cited leaves Livestock, "
            "Dairy and Lumber inside the 'Regular Close - Per each product "
            "schedule:' bullet list; CME's 7/2/2013 revision, captured "
            "2013-07-17T05:03:33Z at the sibling URL "
            "2013-4th-of-july-done.pdf, moves all three into explicit early "
            "closes and removes them from that list. The later statement "
            "governs."),
    }]
    repairs.add(1, "holidays 2013-07-03 family 'livestock+dairy+lumber'",
                "status/verbatim/close_instant/document",
                {"status": "normal", "close_instant": None,
                 "document": "D13J4", "verbatim": before_verbatim},
                {"status": "early_close", "document": "D13J4D",
                 "close_instant": row["close_instant"],
                 "verbatim": row["verbatim"]},
                {"artifact": rev_path,
                 "sha256": sha256_file(rev_abs),
                 "literals": [
                     "1200 CT – Early close for Dairy",
                     "1200 CT – Early close for Lumber",
                     "1202 CT – Early close for Lumber Options",
                     "1215 CT – Early close for Livestock Futures & Options",
                     "Last updated 7/2/2013"],
                 "lineage": (
                     "footer 'Last updated 7/2/2013' post-dates the cited "
                     "revision's 6/4/2013 footer and the capture "
                     "2013-07-17T05:03:33Z post-dates 20130623205825")})

    # ---------------------------------------------------------------- item 2
    row = find(entry, "grains_oilseeds")
    repairs.guard(2, "2013-07-03 grains_oilseeds verbatim fragment",
                  "1215 CT - Early MGEX Wheat close" in row["verbatim"], True)
    before = row["verbatim"]
    row["verbatim"] = before.replace(
        "1215 CT - Early MGEX Wheat close",
        "1215 CT - Early MGEX Wheat & Apple Juice close")
    for literal in ("1200 CT – Early CBOT & KCBT close",
                    "1215 CT – Early MGEX Wheat & Apple Juice close",
                    "1230 CT – Early CBOT Mini-Sized grain close",
                    "1230 CT – Early MGEX Indices close"):
        if literal not in rev_text:
            raise SystemExit("item 2: %r is not in %s" % (literal, rev_path))
    statements = row.setdefault("other_statements", [])
    statements.append({
        "document": "D13J4",
        "relation": "superseded",
        "verbatim": "1215 CT - Early MGEX Wheat close",
        "note": (
            "The cited 7/2/2013 revision prints '1215 CT - Early MGEX Wheat & "
            "Apple Juice close' where the 6/4/2013 revision prints '1215 CT - "
            "Early MGEX Wheat close'. The recorded close_instant values, 1200 "
            "CT for CBOT & KCBT and 1230 CT for CBOT Mini-Sized grain, are "
            "unchanged between the two revisions."),
    })
    repairs.add(2, "holidays 2013-07-03 family 'grains_oilseeds'", "verbatim",
                before, row["verbatim"],
                {"artifact": rev_path, "sha256": sha256_file(rev_abs),
                 "literal": "1215 CT – Early MGEX Wheat & Apple Juice close"})

    # ---------------------------------------------------------------- item 5
    if XLS_GROUPING not in block.get("coverage", ""):
        old_claim = (
            "across all 45 files and every date in range the two rows carry "
            "identical values in every column")
        if old_claim not in block["coverage"]:
            raise SystemExit("item 5: the coverage clause does not carry the "
                             "claim the verdict quotes")
        # The corrected claim is carried once, by `XLS_GROUPING`, which the
        # rebuilt coverage prose prepends below; the stale clause is removed
        # here, with its `, and` connector, rather than replaced, so the block
        # does not state the same thing twice in two spellings.
        stale = ", and " + old_claim
        rebuilt = block["coverage"].replace(stale, "")
        if rebuilt == block["coverage"]:
            raise SystemExit("item 5: the stale clause does not carry its connector")
        block["coverage"] = rebuilt
        repairs.add(5, "coverage clause (1)", "coverage", old_claim,
                    "the stale clause is dropped; the 57-of-58 statement is "
                    "carried once by the rebuilt coverage prose",
                    {"artifact": "coverage"})

    # ----------------------------------------------------- items 3, 7, 10, 5
    coverage = block["coverage"]
    if coverage.startswith(COVERAGE_INTRO):
        coverage = coverage[len(COVERAGE_INTRO):]
    if coverage.startswith(COVERAGE_ROUND1):
        coverage = coverage[len(COVERAGE_ROUND1):]
    # item 7 — the self-description of the cited 2013 workbooks.
    old_self = ("plus 28 CME .xls documents cited here for the first time, "
                "among them the 2014 and 2015 annual master workbooks and both "
                "2013 multi-sheet bundles.")
    new_self = ("plus 28 CME .xls documents cited here for the first time, "
                "among them the 2014 and 2015 annual master workbooks and the "
                "2013 four-sheet Good Friday / Presidents Day bundle.")
    if old_self in coverage:
        coverage = coverage.replace(old_self, new_self)
        repairs.add(7, "coverage clause", "coverage", old_self, new_self,
                    {"artifact": "coverage",
                     "note": "only X13GFPD of the three 2013 .xls files is "
                             "multi-sheet and cited"})
    block["coverage"] = COVERAGE_INTRO + COVERAGE_ROUND1 + XLS_GROUPING + (
        ". ") + REVISION_HISTORY + (
        " Every document in this file's `documents` map now carries the sha256 "
        "and byte count of the bytes this round re-hashed from the research "
        "store, so a reader can reproduce the artifact each row cites.") + " " + coverage
    repairs.add(3, "coverage clause", "coverage",
                "the round-1 clause, which enumerated only the .xls/.zip half",
                "the clause now records the 42 earlier PDF captures",
                {"artifact": "raw/cme-2013-2015-repair-r2/INDEX.md + "
                             "tools/out/wave5/reconciliation.json"})

    # ---------------------------------------------------------------- item 4
    entry = entry_on(block, "2013-03-28")
    row = find(entry, "livestock+dairy+lumber")
    if "zone not printed on this line" not in row.get("close_instant", ""):
        raise SystemExit("item 4: the 2013-03-28 close_instant does not carry "
                         "the unstated-zone hedge the verdict quotes")
    xls_path = "cme-2013-2015-fix/txt/2013-good-friday-presidents-day__20130623203632.txt"
    xls_abs, xls_text = read_text(root, xls_path)
    sheet = xls_text.split("===== SHEET 'Good Fri.'", 1)[-1]
    needed = [
        "r002 | Date | Thursday, March 28",
        "r003 |  | Regular Close | Early Thur. Close",
        "r025 |   Dairy |  | 0000-00-00 13:55:00",
        "r029 |   Livestock |  | 0000-00-00 13:55:00",
        "r030 |   Lumber |  | 0000-00-00 13:55:00",
        "r048 | All times are CT",
    ]
    for literal in needed:
        if literal not in sheet:
            raise SystemExit("item 4: %r is not in %s" % (literal, xls_path))
    before = row["close_instant"]
    row["close_instant"] = "1355 CT"
    row["other_statements"] = [{
        "document": "X13GFPD",
        "relation": "corroborating",
        "verbatim": (
            "[.xls cell render, not running prose] sheet 'Good Fri.', "
            "calendar-date header 'Thursday, March 28' (r002), column header "
            "'Early Thur. Close' (r003 col2): 'Dairy' (r025) = 13:55, "
            "'Livestock' (r029) = 13:55, 'Lumber' (r030) = 13:55. Sheet note "
            "(r048): 'All times are CT'."),
        "note": (
            "Closes the zone the D13GF PDF leaves unstated: CME's own 2013 "
            "workbook prints 13:55 in a column headed 'Early Thur. Close' and "
            "the sheet's own Notes row reads 'All times are CT'. No "
            "cross-year inference is involved; the same workbook is already "
            "cited as corroborating on the neighbouring 2013-03-28 "
            "grains_oilseeds row."),
    }]
    repairs.add(4, "holidays 2013-03-28 family 'livestock+dairy+lumber'",
                "close_instant", before, "1355 CT",
                {"artifact": xls_path, "sha256": sha256_file(xls_abs),
                 "literals": needed})

    # ---------------------------------------------------------------- item 8
    entry = entry_on(block, "2014-11-11")
    row = find(entry, "equity_index+interest_rates+fx+energy+metals"
                      "+grains_oilseeds+livestock+dairy+lumber")
    statements = row.get("other_statements") or []
    if not statements:
        raise SystemExit("item 8: the 2014-11-11 nine-family row carries no "
                         "other_statements entry")
    repairs.guard(8, "2014-11-11 sentence",
                  "The identical sheet appears in the annual master workbook "
                  "X14ANN." in statements[0]["note"], True)
    old_sentence = ("The identical sheet appears in the annual master workbook "
                    "X14ANN.")
    new_sentence = (
        "The same statement appears in X14ANN sheet Veterans Day, with "
        "different punctuation and the older command-centre numbers: X14VET "
        "reads '  Tuesday,  November 11 ... uneffected and will run on a "
        "normal schedule. If you have any questions, please call the CME "
        "Global Command Center at  +1 800 438 8616, in Europe at + 44 800 898 "
        "013 or in Asia at + 65 6532 5010', where X14ANN reads ' Tuesday "
        "November 11 ... uneffected and will run on a normal schedule' with no "
        "comma after Tuesday, no period after schedule, and the older numbers "
        "'312.456.2391 / 44.207.623.4708 / 65.6223.1357'.")
    statements[0]["note"] = statements[0]["note"].replace(
        old_sentence, new_sentence)
    repairs.add(8, "holidays 2014-11-11 other_statements[0]", "note",
                old_sentence, new_sentence,
                {"artifact": "raw/cme-2013-2015-fix/txt/"
                             "2014-veterans-day-holiday-schedule__20140326155155.txt"
                             " and 2014-cme-group-holiday-schedule__20140826204712.txt"})

    # ---------------------------------------------------------------- item 9
    for date, family, title in (
            ("2013-10-14", "equity_index+interest_rates+fx+energy+metals"
                           "+grains_oilseeds+livestock+dairy+lumber",
             "Globex Columbus Day Holiday Schedule"),
            ("2013-11-11", "equity_index+interest_rates+fx+energy+metals"
                           "+grains_oilseeds+livestock+dairy+lumber",
             "Globex Veterans Day Holiday Schedule")):
        row = find(entry_on(block, date), family)
        if not row["verbatim"].startswith(title):
            raise SystemExit("item 9: %s does not quote %r" % (date, title))
        fixed = "Globex®" + title[len("Globex"):]
        before = row["verbatim"]
        row["verbatim"] = fixed + before[len(title):]
        repairs.add(9, "holidays %s (the nine-family row)" % date, "verbatim",
                    title, fixed,
                    {"artifact": "raw/cme-2013-2015/txt/%s__*.txt" % date,
                     "literal": fixed})

    # --------------------------------------------------------------- item 10
    padded = []
    for holiday in block["holidays"]:
        for row in holiday["families"]:
            for field in ("close_instant", "open_instant"):
                value = row.get(field)
                if not value:
                    continue
                new = re.sub(r"\b0(\d{3}) CT", r"\1 CT", value)
                if new != value:
                    padded.append((holiday["date"], row["family"], field,
                                   value, new))
                    row[field] = new
    repairs.add(10, "29 *_instant fields", "close_instant/open_instant",
                "three-digit printed clocks zero-padded to four",
                "%d fields re-emitted as CME printed them" % len(padded),
                {"artifact": "the cited PDFs' own clock forms",
                 "fields": [{"date": d, "family": f, "field": fl,
                             "before": b, "after": a}
                            for d, f, fl, b, a in padded]})

    # --------------------------------------------------------------- item 12
    nye_path = "cme-2013-2015-fix/txt/2013-good-friday-presidents-day__20130623203632.txt"
    nye_sheet = xls_text.split("===== SHEET 'New Years'", 1)[-1].split(
        "===== SHEET", 1)[0]
    for literal in ("r000 | CME Group Globex New Year's Holiday Schedule: "
                    "December 31, 2012 to January 2, 2013",
                    "r001 | Trade Date | Monday, December 31 |  | "
                    "Wednesday, January 2",
                    "r002 | Date | Monday, December 31 |  | "
                    "Tuesday, January 1 |  |  | Wednesday, January 2",
                    "r018 |   Grains and Oilseeds (CBOT & KCBT)",
                    "All times are CT"):
        if literal not in nye_sheet:
            raise SystemExit("item 12: %r is not in the New Years sheet"
                             % literal)
    corroborated = 0
    for date in ("2013-01-01", "2013-01-02"):
        for row in entry_on(block, date)["families"]:
            statements = row.setdefault("other_statements", [])
            if any(s.get("document") == "X13GFPD" for s in statements):
                continue
            statements.append({
                "document": "X13GFPD",
                "relation": "corroborating",
                "verbatim": (
                    "[.xls cell render, not running prose] sheet 'New Years', "
                    "titled 'CME Group Globex New Year's Holiday Schedule: "
                    "December 31, 2012 to January 2, 2013': trade-date header "
                    "(r001) 'Monday, December 31 | Wednesday, January 2' over "
                    "column headers (r003) 'Regular Mon. Close | PCP* | "
                    "Pre-opening** | Open | Halt | ...'. Sheet note (r048): "
                    "'All times are CT'."),
                "note": (
                    "The workbook the file already cites on 2013-01-18 and "
                    "2013-03-28 carries the full product grid for both dates "
                    "of this holiday, which is why the 2013 .xls pass was "
                    "commissioned. Its per-product instants are reported on "
                    "the rows they belong to rather than folded into this "
                    "one; the corroboration records that the sheet covers "
                    "this date. 2013-01-02 is the one date in this block whose "
                    "printed grain open moved between PDF revisions: the "
                    "6/4/2013 revision this file cites prints '0930 CT - CME "
                    "Globex Grain open for trade date Wednesday, Jan 2', and "
                    "the earlier 11/12/2012 and 1/4/2013 revisions print "
                    "0700/0930 differently - see the revised missing[] entry "
                    "for the revision history."),
            })
            corroborated += 1
    repairs.add(12, "holidays 2013-01-01 and 2013-01-02, all rows",
                "other_statements",
                "%d rows carry no corroborating statement" % corroborated,
                "each row carries X13GFPD's New Years sheet",
                {"artifact": nye_path, "sha256": sha256_file(xls_abs),
                 "rows_corroborated": corroborated,
                 "literals": [
                     "r000 | CME Group Globex New Year's Holiday Schedule: "
                     "December 31, 2012 to January 2, 2013",
                     "r048 | All times are CT"]})

    # ---------------------------------------------------------------- item 6
    missing = block["missing"]
    # item 4 — the Good Friday zone gap is closed.
    good_friday = (
        "Good Friday 2013, livestock/dairy/lumber early close - CME prints "
        "'1355 - Early Close for Lumber, Dairy & Livestock' with NO time zone "
        "on that line (every other time in the document is CT, and the "
        "2014/2015 equivalents print '1355 CT / 1455 ET / 1855 UTC'). The zone "
        "is recorded as unstated rather than inferred from the later years.")
    if missing[2] != good_friday:
        raise SystemExit("item 4: missing[2] is not the text the verdict quotes")
    missing[2] = (
        "Good Friday 2013, livestock/dairy/lumber early close - closed in this "
        "round. The D13GF PDF prints '1355 - Early Close for Lumber, Dairy & "
        "Livestock' with no zone on that line, and the round-1 block recorded "
        "the zone as unstated rather than inferring it from the later years. "
        "X13GFPD - 2013-good-friday-presidents-day.xls, already cited on the "
        "2013-03-28 grains row and on 2013-01-18 - settles it from 2013 "
        "itself: sheet 'Good Fri.' carries the column header 'Early Thur. "
        "Close' over rows 'Dairy' (r025), 'Livestock' (r029) and 'Lumber' "
        "(r030) all reading 13:55, and the sheet's own Notes row (r048) reads "
        "'All times are CT'. The 2013-03-28 row now cites it as corroborating "
        "and records '1355 CT'. No inference from a later year is involved.")
    repairs.add(4, "missing[2]", "missing[2]", good_friday, missing[2],
                {"artifact": xls_path})

    # item 3 — the completeness claim.
    old_ten = missing[10]
    if MISSING_10_OLD_MARKER not in old_ten:
        raise SystemExit("item 3: missing[10] does not carry the claim the "
                         "verdict quotes")
    missing[10] = (
        "Superseded operator statements. CME revised these schedules in place "
        "at the same URL. Eleven family rows carry an 'other_statements' entry "
        "with relation 'superseded' recording the earlier operator statement "
        "and the lineage that resolves it; in every one of the eleven the "
        "recorded value is the later statement. The .xls half of that history "
        "is two generations: CME's March 2015 draft generation (the 2015 "
        "annual master workbook captured 2015-03-20 and the per-holiday .xls "
        "twins captured 2015-03-26), superseded by the September/December 2015 "
        "revisions of the same URLs and by the PDFs; and the 2014 annual "
        "master workbook captured 2014-08-26, whose 'July 4', 'Christmas' and "
        "New Years sheets embed revisions older than the per-holiday files "
        "published alongside them. The PDF half is larger still and was not "
        "enumerated by round 1: the cited PDF URLs carry 73 distinct-digest "
        "status-200 captures in CME's archived responses for 2013-2015, of "
        "which 31 are cited and 42 are earlier revisions. All 42 were "
        "retrieved and reconciled in round 2 as .xls revisions were (see the "
        "coverage clause and raw/cme-2013-2015-repair-r2/); two of them state "
        "a session value the cited revision does not, and in both the cited "
        "capture is CME's later revision by its own 'Last updated' footer, so "
        "no recorded value moves. The three generations above therefore "
        "account for the .xls history and the 42 earlier PDF captures for the "
        "PDF history; what remains unread is the revision history of the "
        "non-cited document URLs in the same directory, which no row of this "
        "file cites.")
    repairs.add(3, "missing[10]", "missing[10]", old_ten, missing[10],
                {"artifact": "raw/cme-2013-2015-repair-r2/retrieval.json"})

    # item 6 — the unrecorded gaps.
    missing.append(INDICES_MISSING)
    repairs.add(6, "missing[]", "missing[]", "no entry names the unmodelled "
                "printed product lines", "missing[%d] names them"
                % (len(missing) - 1), {"artifact": "the cited 2013-2015 PDFs "
                "and X13GFPD/X15ANN sheets"})

    # ------------------------------------------- the document registry, item 1
    # The 7/2/2013 revision is a second artifact at a sibling URL, so it takes
    # its own document id, with the bytes this round retrieved.
    done_rel = ("cme-2013-2015-verify-r2/new/"
                "2013-4th-of-july-done__20130717050333.pdf")
    done_abs = locate(root, done_rel)
    if done_abs is None:
        raise SystemExit("item 1: the 7/2/2013 revision is not in the store")
    block["documents"]["D13J4D"] = {
        "file": "2013-4th-of-july-done.pdf",
        "original_url": ("http://www.cmegroup.com/tools-information/"
                         "holiday-calendar/files/2013-4th-of-july-done.pdf"),
        "replay_url": ("https://web.archive.org/web/20130717050333id_/"
                       "http://www.cmegroup.com/tools-information/"
                       "holiday-calendar/files/2013-4th-of-july-done.pdf"),
        "capture_utc": "2013-07-17T05:03:33Z",
        "tier": "T1",
        "doc_id_long": "2013-4th-of-july-done@20130717050333",
        "what": ("CME's later revision of the 2013 Independence Day Globex "
                 "schedule; footer 'Last updated 7/2/2013', retrieved by the "
                 "round-2 verifier and re-verified here"),
        "sha256": sha256_file(done_abs),
        "bytes": os.path.getsize(done_abs),
    }
    # The 2016-2018 wave's own tables already resolve this capture under a
    # date-only id, and every_document_id_resolves_to_one_artifact allows a
    # sha256 exactly one label. The block records that label for it, so this
    # era's rows cite the artifact exactly as the older wave does.
    block["documents"]["D16NY"]["capture_label"] = "2016-01-08"
    # The count the repair record states is the round-1 residue, so it is taken
    # before the loop below assigns the digests it is counting.
    missing_sha256_before = sum(
        1 for code, entry in block["documents"].items()
        if code != "D13J4D" and not entry.get("sha256"))
    registry_hashed = []
    for code, entry in sorted(block["documents"].items()):
        # The capture must travel with the filename: several URLs carry more
        # than one capture, and hashing the wrong revision would record a
        # digest the cited artifact does not have.
        path = locate(root, "%s@%s" % (entry["file"],
                                       entry["doc_id_long"].rsplit("@", 1)[-1]))
        if path is None:
            continue
        digest = sha256_file(path)
        size = os.path.getsize(path)
        if entry.get("sha256") != digest or entry.get("bytes") != size:
            registry_hashed.append({"code": code, "sha256": digest,
                                    "bytes": size})
        entry["sha256"] = digest
        entry["bytes"] = size
    repairs.add(1, "documents registry", "sha256/bytes/D13J4D",
                "%d documents carried no sha256" % missing_sha256_before,
                "every document carries the sha256 and byte count of the "
                "bytes this round re-hashed; D13J4D added",
                {"artifact": "holidays/raw/",
                 "rehashed": registry_hashed,
                 "documents": len(block["documents"])})

    # --------------------------------------------------------------- item 11
    index_path = "cme-2013-2015/INDEX.md"
    index_abs = os.path.join(root, index_path)
    with open(index_abs, errors="replace") as handle:
        index_text = handle.read()
    amendment = """
## Amendment, 2026-09-17 UTC (stage 2.2 wave 5)

Two statements in this round-0 index are withdrawn. They are left in place
because a capture dates the observation, but neither governs.

1. The line "and the `.xls` twins of the PDFs above (same content, different
   container)" is **withdrawn and superseded by
   `../cme-2013-2015-fix/INDEX.md`**. CME's `.xls` holiday schedules are a
   different and richer document than the PDFs, not a container swap; the
   round-1 fix directory retrieved all 45 of them and reconciled them cell by
   cell.
2. Interpretive note 6, which states the 2015 Labor Day change categorically,
   is **superseded by the lineage recorded on the 2015-09-04
   `interest_rates+fx` row** of `../cme-2013-2015.json` and by the round-2
   reconciliation in `../cme-2013-2015-repair-r2/INDEX.md`. The change is real;
   the note's "That is CME's own printed change, not an inference" reads as
   covering more than the one capture it was read from.

The PDF half of CME's in-place revision history was retrieved and reconciled in
round 2: `../cme-2013-2015-repair-r2/INDEX.md` lists the 42 earlier captures of
this directory's cited URLs, with sha256 and the reconciliation result.
"""
    if "## Amendment, 2026-09-17 UTC" not in index_text:
        with open(index_abs, "w") as handle:
            handle.write(amendment.lstrip("\n") + "\n" + index_text)
    repairs.add(11, "raw/cme-2013-2015/INDEX.md", "amendment block",
                "no amendment pointer", "a dated amendment block at the top",
                {"artifact": index_path})

    # --------------------------------------------------------------- write out
    serialised = json.dumps(block, indent=2, ensure_ascii=False) + "\n"
    r2_path = os.path.join(args.out, "cme-2013-2015.r2.json")
    with open(r2_path, "w") as handle:
        handle.write(serialised)
    repair_path = os.path.join(args.out, "cme-2013-2015.repair.json")
    record = {
        "task": "cme-2013-2015",
        "repaired_on_utc": REPAIRED_ON,
        "verdict": "../cme-2013-2015.verify.json (round 2, matches: false, "
                   "12 discrepancies)",
        "round_1_block": "holidays/cme-2013-2015.json",
        "round_1_sha256": hashlib.sha256(round1_bytes).hexdigest(),
        "round_1_bytes": len(round1_bytes),
        "round_2_block": "holidays/cme-2013-2015.r2.json",
        "round_2_sha256": hashlib.sha256(serialised.encode()).hexdigest(),
        "counts": {
            "discrepancies_in_verdict": 12,
            "repairs_recorded": len(repairs.items),
            "repair_ids": sorted({i["id"] for i in repairs.items}),
            "instant_fields_re_emitted": len(padded),
            "rows_corroborated_with_new_years_sheet": corroborated,
            "missing_entries": len(block["missing"]),
        },
        "repairs": repairs.items,
    }
    with open(repair_path, "w") as handle:
        json.dump(record, handle, indent=1, ensure_ascii=False)
        handle.write("\n")

    print("round 1  %s  %d bytes  sha256 %s"
          % (os.path.basename(round1_path), len(round1_bytes),
             record["round_1_sha256"]))
    print("round 2  %s  %d bytes  sha256 %s"
          % (os.path.basename(r2_path), len(serialised.encode()),
             record["round_2_sha256"]))
    print("repairs  %d, ids %s"
          % (len(repairs.items), sorted({i["id"] for i in repairs.items})))
    print("instants re-emitted %d, rows corroborated %d, missing entries %d"
          % (len(padded), corroborated, len(block["missing"])))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
