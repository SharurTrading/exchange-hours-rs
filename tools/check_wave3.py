#!/usr/bin/env python3
# SPDX-License-Identifier: MIT-0
"""check_wave3.py -- independent checks over the wave-3 generator's outputs.

This deliberately shares no code with `tools/wave3_rows.py`: it re-reads the
evidence block, the raw indexes and the emitted files as text, and it re-runs
the generator into a scratch directory to prove the output is reproducible.

Checks:

  1. re-running the generator on the same inputs is byte-identical;
  2. every emitted `(Y, M, D, kind, tier, "doc")` tuple parses, its date is
     strictly ascending within its family, and it lies inside
     2022-01-01..2024-12-31;
  3. each tuple's preceding comment names the same date, tier and document;
  4. every document id used by a tuple appears in `tools/out/documents.md`, and
     every sha256 there reproduces from the saved artifact;
  5. every `instant as printed` token in a family's evidence table is a
     substring of the block's own text for that entry;
  6. every evidence-table row's document and tier match the family's emitted
     tuple for the same trade date.

Exit status is 0 when every check passes and 1 otherwise.
"""

from __future__ import annotations

import datetime as dt
import hashlib
import json
import os
import re
import shutil
import subprocess
import sys
import tempfile

FAMILIES = (
    "globex_equity_index",
    "globex_energy",
    "globex_fx",
    "globex_grains",
    "globex_interest_rates",
    "globex_livestock",
    "globex_cryptocurrency",
    "globex_nikkei_225_dollar",
)

KINDS = (
    re.compile(r"^Closed$"),
    re.compile(r"^Unsourced$"),
    re.compile(r"^early_close\(\d+ \* 3_600( \+ \d+ \* 60)?\)$"),
    re.compile(r"^late_open\(\d+ \* 3_600( \+ \d+ \* 60)?\)$"),
    re.compile(r"^late_open_and_early_close\(\d+ \* 3_600( \+ \d+ \* 60)?, "
               r"\d+ \* 3_600( \+ \d+ \* 60)?\)$"),
)

TUPLE_RE = re.compile(
    r'^        \((\d{4}), (\d{1,2}), (\d{1,2}), (.+), (T[12]), "(.+)"\),$')
COMMENT_RE = re.compile(r"^        // (\d{4}-\d{2}-\d{2}) - (T[12]) - (.+?)"
                        r"( - (.*))?\.?$")
DOC_ROW_RE = re.compile(
    r"^\| `([^`]+)` \| `([^`]+)` \| <([^>]+)> \| archive capture "
    r"([0-9T:Z-]+) \| (T[12]) \| `([0-9a-f]{64})` \|$")
EVIDENCE_ROW_RE = re.compile(
    r"^\| (\d{4}-\d{2}-\d{2}) \| ([a-z ]+) \| (.+) \| `([^`]+)` \| (T[12]) \|")


def sha256_of(path):
    with open(path, "rb") as handle:
        return hashlib.sha256(handle.read()).hexdigest()


def kind_name(expr):
    """The plan's kind name for a kind expression."""
    if expr == "Closed":
        return "Closed"
    if expr == "Unsourced":
        return "Unsourced"
    if expr.startswith("early_close("):
        return "EarlyClose"
    if expr.startswith("late_open_and_early_close("):
        return "LateOpenAndEarlyClose"
    return "LateOpen"


def kind_label(kind):
    """The evidence table's `kind` word for a kind expression."""
    if kind == "Closed":
        return "closed"
    if kind == "Unsourced":
        return "unsourced"
    if kind.startswith("early_close("):
        return "early close"
    if kind.startswith("late_open_and_early_close("):
        return "late open and early close"
    return "late open"


def ssm_of(token):
    """Seconds since midnight for an ``HH:MM CT`` token, else ``None``."""
    match = re.fullmatch(r"(\d{1,2}):(\d{2})\s*CT", token)
    if not match:
        return None
    return int(match.group(1)) * 3600 + int(match.group(2)) * 60


def expr_ssm(expr):
    """Seconds since midnight for a `H * 3_600 + M * 60` expression."""
    hours = re.search(r"(\d+) \* 3_600", expr)
    minutes = re.search(r"\+ (\d+) \* 60", expr)
    return (int(hours.group(1)) if hours else 0) * 3600 + \
        (int(minutes.group(1)) if minutes else 0) * 60


def kind_ssms(kind):
    """The instants a kind expression encodes, as a list."""
    if kind in ("Closed", "Unsourced"):
        return []
    if kind.startswith("early_close("):
        return [expr_ssm(kind[len("early_close("):-1])]
    if kind.startswith("late_open("):
        return [expr_ssm(kind[len("late_open("):-1])]
    inner = kind[len("late_open_and_early_close("):-1]
    head, tail = inner.split(", ", 1)
    return [expr_ssm(head), expr_ssm(tail)]


def main():
    root = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
    out = os.path.join(root, "tools", "out")
    research = os.environ.get("WAVE3_RESEARCH")
    if not research:
        raise SystemExit(
            "set WAVE3_RESEARCH to the research store's root (the directory holding "
            "`holidays/cme-2022-2024.r2.json` and `holidays/raw/`); this wave's block "
            "and every artifact it cites live there and are deliberately not committed"
        )
    holidays = os.path.join(research, "holidays")
    r2 = os.path.join(holidays, "cme-2022-2024.r2.json")
    block_path = r2 if os.path.exists(r2) else os.path.join(
        holidays, "cme-2022-2024.json")
    with open(block_path, "r", encoding="utf-8") as handle:
        block = json.load(handle)

    failures = []
    checks = 0

    def ok(condition, message):
        nonlocal checks
        checks += 1
        if not condition:
            failures.append(message)

    # 1. determinism ------------------------------------------------------
    scratch = tempfile.mkdtemp(prefix="wave3-check-")
    try:
        subprocess.run(
            [sys.executable, os.path.join(root, "tools", "wave3_rows.py"),
             "--quiet", "--out", scratch, "--research", research],
            check=True, capture_output=True,
        )
        names = sorted(os.listdir(out))
        ok(sorted(os.listdir(scratch)) == names,
           "rerun produced a different file set")
        for name in names:
            with open(os.path.join(out, name), "rb") as a, \
                    open(os.path.join(scratch, name), "rb") as b:
                ok(a.read() == b.read(),
                   "rerun is not byte-identical for %s" % name)
    finally:
        shutil.rmtree(scratch, ignore_errors=True)

    # 2/3. tuples and comments -------------------------------------------
    tuples = {}
    for family in FAMILIES:
        path = os.path.join(out, "%s.rows.rs" % family)
        with open(path, "r", encoding="utf-8") as handle:
            lines = handle.readlines()
        rows = []
        comments = []
        for line in lines:
            match = COMMENT_RE.match(line.rstrip("\n"))
            if match and not match.group(1) is None:
                comments.append((match.group(1), match.group(2),
                                 match.group(3).strip()))
                continue
            match = TUPLE_RE.match(line.rstrip("\n"))
            if match:
                year, month, day, kind, tier, doc = match.groups()
                rows.append((dt.date(int(year), int(month), int(day)), kind,
                             tier, doc, len(comments)))
        ok(rows, "%s: no rows parsed" % family)
        dates = [row[0] for row in rows]
        ok(dates == sorted(set(dates)),
           "%s: trade dates are not strictly ascending" % family)
        for date in dates:
            ok(dt.date(2022, 1, 1) <= date <= dt.date(2024, 12, 31),
               "%s: row %s is outside 2022-01-01..2024-12-31" % (family, date))
        for date, kind, tier, doc, comment_count in rows:
            ok(any(pattern.match(kind) for pattern in KINDS),
               "%s %s: kind %r is not one of the crate's kinds"
               % (family, date, kind))
            ok(tier in ("T1", "T2"), "%s %s: tier %r" % (family, date, tier))
            ok(doc.strip() != "", "%s %s: empty document id" % (family, date))
            ok(comment_count >= 1,
               "%s %s: tuple has no citation comment" % (family, date))
            if comment_count:
                cdate, ctier, cdoc = comments[comment_count - 1]
                ok(cdate == date.isoformat(),
                   "%s: comment %s precedes row %s" % (family, cdate, date))
                ok(ctier == tier and cdoc == doc,
                   "%s %s: comment (tier=%s doc=%s) does not match the tuple"
                   % (family, date, ctier, cdoc))
        tuples[family] = rows

    # 4. documents --------------------------------------------------------
    documents = {}
    with open(os.path.join(out, "documents.md"), "r", encoding="utf-8") as handle:
        for line in handle:
            match = DOC_ROW_RE.match(line.rstrip("\n"))
            if not match:
                continue
            doc_id, file, url, capture, tier, sha = match.groups()
            documents[doc_id] = (file, url, capture, tier, sha)
    ok(documents, "documents.md parsed no rows")
    used = {row[3] for rows in tuples.values() for row in rows}
    used |= {row[3] for family in FAMILIES for row in tuples[family]}
    for doc_id in sorted(used):
        ok(doc_id in documents,
           "document id %r is used by a row but is not in documents.md" % doc_id)
    for doc_id, (file, url, capture, tier, sha) in sorted(documents.items()):
        stamp = re.sub(r"[^0-9]", "", capture)
        ok(url.startswith("https://web.archive.org/web/%sid_/" % stamp),
           "%s: replay URL %s does not carry the capture's own timestamp"
           % (doc_id, url))
        found = None
        for candidate in (
            os.path.join(holidays, "raw", "cme-2022-2024", file),
            os.path.join(holidays, "raw", "cme-2022-2024-fix", file),
        ):
            if os.path.exists(candidate):
                found = candidate
                break
        ok(found is not None, "%s: saved artifact %s not found" % (doc_id, file))
        if found:
            ok(sha256_of(found) == sha,
               "%s: sha256 does not reproduce from %s" % (doc_id, found))

    # 5/6. evidence tables -------------------------------------------------
    entries = {}
    for holiday in block["holidays"]:
        for entry in holiday["families"]:
            entries.setdefault(entry["family"], {})[holiday["date"]] = entry
    for family in FAMILIES:
        path = os.path.join(out, "%s.evidence.md" % family)
        with open(path, "r", encoding="utf-8") as handle:
            text = handle.read()
        rows = {}
        for line in text.splitlines():
            match = EVIDENCE_ROW_RE.match(line)
            if not match:
                continue
            date, label, cell, doc_id, tier = match.groups()
            rows[date] = (cell, doc_id, tier, label)
            entry = (entries.get(family) or {}).get(date)
            # A row keyed to a trade date that follows a closure cites the
            # closure entry, so the haystack is that entry: the block's own text
            # whose printed open the row states.
            haystacks = []
            if entry is not None:
                haystacks.append(json.dumps(entry))
            cited_file = documents.get(doc_id, (None,))[0]
            if cited_file:
                for other_date, other in (entries.get(family) or {}).items():
                    if other["document"].split(" @")[0].strip() == cited_file:
                        haystacks.append(json.dumps(other))
            tokens = re.findall(r"`([^`]+)`", cell)
            ok(tokens, "%s %s: evidence row has no printed token" % (family, date))
            for token in tokens:
                if token in ("closed", "unknown"):
                    continue
                ok(bool(haystacks),
                   "%s %s: evidence row cites no block entry" % (family, date))
                ok(any(token in haystack for haystack in haystacks),
                   "%s %s: printed token %r is not a substring of the cited "
                   "entry's text" % (family, date, token))
        for row_date, kind, tier, doc, _ in tuples[family]:
            key = row_date.isoformat()
            ok(key in rows, "%s %s: shipped row has no evidence-table row"
               % (family, key))
            if key in rows:
                cell, cell_doc, cell_tier, cell_label = rows[key]
                ok(cell_label == kind_label(kind),
                   "%s %s: evidence kind %r does not match %s"
                   % (family, key, cell_label, kind))
                ok(cell_doc == doc and cell_tier == tier,
                   "%s %s: evidence row cites %s/%s, tuple cites %s/%s"
                   % (family, key, cell_doc, cell_tier, doc, tier))
                printed = [ssm_of(token)
                           for token in re.findall(r"`([^`]+)`", cell)]
                printed = [value for value in printed if value is not None]
                expected = kind_ssms(kind)
                if kind in ("Closed", "Unsourced"):
                    ok(not printed,
                       "%s %s: %s row prints an instant" % (family, key, kind))
                else:
                    ok(sorted(printed) == sorted(expected),
                       "%s %s: %s encodes %s but the evidence row prints %s"
                       % (family, key, kind, sorted(expected), sorted(printed)))
        for key in rows:
            ok(any(row[0].isoformat() == key for row in tuples[family]),
               "%s %s: evidence table lists a row that is not emitted"
               % (family, key))

    # 6b. ROWS.json is the encoding contract and must agree with the .rows.rs ----
    with open(os.path.join(out, "ROWS.json"), "r", encoding="utf-8") as handle:
        plan = json.load(handle)
    ok(plan["window"] == ["2022-01-01", "2024-12-31"],
       "ROWS.json: window is %r" % (plan["window"],))
    planned_rows = 0
    per_kind = {}
    per_tier = {}
    per_year = {}
    per_family = {}
    for family_plan in plan["families"]:
        family = family_plan["family"]
        ok(family in FAMILIES, "ROWS.json: unknown family %r" % family)
        ok(family_plan["coverage"] == [[2022, 1, 1, 2024, 12, 31]],
           "ROWS.json %s: coverage is %r" % (family, family_plan["coverage"]))
        planned = family_plan["rows"]
        emitted = tuples.get(family, [])
        ok(len(planned) == len(emitted),
           "ROWS.json %s: %d planned rows, %d emitted"
           % (family, len(planned), len(emitted)))
        documents_in_plan = {row["document"] for row in planned}
        for doc_id in documents_in_plan:
            ok(doc_id in documents,
               "ROWS.json %s: document %r is not in documents.md"
               % (family, doc_id))
        dates = [dt.date.fromisoformat(row["date"]) for row in planned]
        ok(dates == sorted(set(dates)),
           "ROWS.json %s: dates are not strictly ascending" % family)
        for row in planned:
            date = dt.date.fromisoformat(row["date"])
            ok(dt.date(2022, 1, 1) <= date <= dt.date(2024, 12, 31),
               "ROWS.json %s: %s is outside the window" % (family, row["date"]))
            open_ssm, close_ssm = row["open_ssm"], row["close_ssm"]
            if row["kind"] == "Closed" or row["kind"] == "Unsourced":
                ok(open_ssm is None and close_ssm is None,
                   "ROWS.json %s %s: %s carries an instant"
                   % (family, row["date"], row["kind"]))
            elif row["kind"] == "EarlyClose":
                ok(open_ssm is None and isinstance(close_ssm, int)
                   and 0 <= close_ssm <= 86400,
                   "ROWS.json %s %s: EarlyClose instants are %r/%r"
                   % (family, row["date"], open_ssm, close_ssm))
            elif row["kind"] == "LateOpen":
                ok(close_ssm is None and isinstance(open_ssm, int)
                   and 0 <= open_ssm < 86400,
                   "ROWS.json %s %s: LateOpen instants are %r/%r"
                   % (family, row["date"], open_ssm, close_ssm))
            elif row["kind"] == "LateOpenAndEarlyClose":
                ok(isinstance(open_ssm, int) and isinstance(close_ssm, int)
                   and 0 <= open_ssm < 86400 and 0 <= close_ssm <= 86400,
                   "ROWS.json %s %s: combined instants are %r/%r"
                   % (family, row["date"], open_ssm, close_ssm))
            else:
                ok(False, "ROWS.json %s %s: unknown kind %r"
                   % (family, row["date"], row["kind"]))
            ok(row["printed"].strip() != "" and row["reason"].strip() != ""
               and row["source"].strip() != "",
               "ROWS.json %s %s: printed/reason/source is empty"
               % (family, row["date"]))
            planned_rows += 1
            per_kind[row["kind"]] = per_kind.get(row["kind"], 0) + 1
            per_tier[row["tier"]] = per_tier.get(row["tier"], 0) + 1
            per_year[row["date"][:4]] = per_year.get(row["date"][:4], 0) + 1
        per_family[family] = len(planned)
        # row-for-row agreement with the emitted tuple lines
        for row, (date, kind, tier, doc, _) in zip(planned, emitted):
            ok(row["date"] == date.isoformat() and row["kind"] == kind_name(kind)
               and row["tier"] == tier and row["document"] == doc,
               "ROWS.json %s %s disagrees with the emitted tuple (%s/%s/%s/%s)"
               % (family, row["date"], row["kind"], row["tier"],
                  row["document"], kind))
            reason = row["reason"]
            with open(os.path.join(out, "%s.rows.rs" % family), "r",
                      encoding="utf-8") as handle:
                body = handle.read()
            ok(reason in body,
               "ROWS.json %s %s: reason %r is not in the row file"
               % (family, row["date"], reason))
    for key, value in (("rows_per_family", per_family),
                       ("rows_per_kind", per_kind),
                       ("rows_per_tier", per_tier),
                       ("rows_per_year", per_year)):
        ok(plan["counts"][key] == value,
           "ROWS.json counts.%s is %r, recomputed %r"
           % (key, plan["counts"][key], value))
    ok(plan["counts"]["total"] == planned_rows,
       "ROWS.json counts.total is %r, recomputed %d"
       % (plan["counts"]["total"], planned_rows))
    ok(len(plan["families"]) == len(FAMILIES),
       "ROWS.json: %d families" % len(plan["families"]))
    plan_docs = {doc["id"]: doc for doc in plan["documents"]}
    ok(set(plan_docs) == set(documents),
       "ROWS.json documents differ from documents.md: %r"
       % (sorted(set(plan_docs) ^ set(documents)),))
    for doc_id, doc in plan_docs.items():
        if doc_id in documents:
            file, url, capture, tier, sha = documents[doc_id]
            ok(doc["file"] == file and doc["url"] == url
               and doc["capture_utc"] == capture and doc["tier"] == tier
               and doc["sha256"] == sha,
               "ROWS.json %s: document row disagrees with documents.md" % doc_id)

    # 7. the emitted rows match the real `holidays!` macro's fragment shape ----
    rustc = shutil.which("rustc")
    if rustc:
        head = (
            "macro_rules! holidays {\n"
            "    (coverage: [$(($fy:expr, $fm:expr, $fd:expr) ..= "
            "($ly:expr, $lm:expr, $ld:expr)),* $(,)?],\n"
            "     rows: [$(($y:expr, $mo:expr, $d:expr, $k:expr, $t:expr, "
            "$doc:literal)),* $(,)?] $(,)?) => { () };\n"
            "}\n"
        )
        body = []
        for family in FAMILIES:
            coverage = "[(2022, 1, 1) ..= (2024, 12, 31)]"
            rows = []
            with open(os.path.join(out, "%s.rows.rs" % family), "r",
                      encoding="utf-8") as handle:
                for line in handle:
                    stripped = line.strip()
                    if stripped.startswith("// coverage:"):
                        coverage = stripped[len("// coverage:"):].strip()
                    if TUPLE_RE.match(line.rstrip("\n")):
                        rows.append(stripped)
            ok(bool(rows), "%s: no rows to compile" % family)
            body.append("static %s: () = holidays! { coverage: %s, rows: [%s] };\n"
                        % (family.upper(), coverage, "".join(rows)))
        source = head + "".join(body)
        scratch = tempfile.mkdtemp(prefix="wave3-rust-")
        try:
            src = os.path.join(scratch, "rows.rs")
            with open(src, "w", encoding="utf-8") as handle:
                handle.write(source)
            finished = subprocess.run(
                [rustc, "--edition", "2021", "--crate-type", "lib",
                 "--emit=metadata", "-o", os.path.join(scratch, "rows.rmeta"),
                 src],
                capture_output=True, text=True,
            )
            ok(finished.returncode == 0,
               "the emitted rows do not satisfy the `holidays!` fragment shape:\n%s"
               % finished.stderr[-2000:])
        finally:
            shutil.rmtree(scratch, ignore_errors=True)
    else:
        print("note: rustc not found; the `holidays!` fragment-shape check was skipped")

    print("checks run: %d" % checks)
    if failures:
        print("FAIL (%d)" % len(failures))
        for failure in failures:
            print("  - %s" % failure)
        return 1
    print("PASS: every check passed")
    return 0


if __name__ == "__main__":
    sys.exit(main())
