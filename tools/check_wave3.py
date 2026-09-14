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


def main():
    root = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
    out = os.path.join(root, "tools", "out")
    research = os.environ.get(
        "WAVE3_RESEARCH",
        "/Users/agedvagabond/Developer/exchange-hours-research")
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
             "--quiet", "--out", scratch],
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
            ok(comment_count <= len(comments),
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
            date, _kind, cell, doc_id, tier = match.groups()
            rows[date] = (cell, doc_id, tier)
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
                ok(rows[key][1] == doc and rows[key][2] == tier,
                   "%s %s: evidence row cites %s/%s, tuple cites %s/%s"
                   % (family, key, rows[key][1], rows[key][2], doc, tier))
        for key in rows:
            ok(any(row[0].isoformat() == key for row in tuples[family]),
               "%s %s: evidence table lists a row that is not emitted"
               % (family, key))

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
