#!/usr/bin/env python3
# SPDX-License-Identifier: MIT-0
"""wave5_reconcile.py -- reconcile every earlier PDF capture against the cited one.

Repair item 3 of `holidays/cme-2013-2015.verify.json` requires the PDF half of
CME's in-place revision history to be retrieved and reconciled the way the
`.xls` half already was.  This tool reads

  * the round-0 CDX enumeration (`holidays/raw/cme-2013-2015/cdx_holiday_calendar.json`),
  * the download manifests (`names.txt`, `fix/todo.tsv`, `verify-r2/new/`),
  * the retrieved captures (`holidays/raw/cme-2013-2015-repair-r2/retrieval.json`),
  * the cited captures (`holidays/raw/cme-2013-2015/txt/`), and
  * the block's own record of every date and family (`holidays/cme-2013-2015.json`),

and emits, per PDF URL, every distinct-digest status-200 capture in CDX order
with its retrieved sha256, the `Last updated` footer each carries, and — for
every earlier capture — a per-section diff of the CT clock values against the
cited capture, so a reader can see exactly which revisions moved a session
value and which are cosmetic.

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

CITED_DIR = "cme-2013-2015/txt"
NEW_DIR = "cme-2013-2015-repair-r2/txt"


def load_cdx(path):
    with open(path) as handle:
        rows = json.load(handle)
    header = rows[0]
    return [dict(zip(header, row)) for row in rows[1:]]


def manifests(root):
    held = set()
    with open(os.path.join(root, "cme-2013-2015", "names.txt")) as handle:
        for line in handle:
            line = line.strip()
            if line and "__" in line:
                base, stamp = line.rsplit("__", 1)
                held.add((base + ".pdf", stamp))
    with open(os.path.join(root, "cme-2013-2015-fix", "todo.tsv")) as handle:
        for line in handle:
            parts = line.rstrip("\n").split("\t")
            if len(parts) >= 2:
                held.add((os.path.basename(parts[1]), parts[0]))
    newdir = os.path.join(root, "cme-2013-2015-verify-r2", "new")
    if os.path.isdir(newdir):
        for name in os.listdir(newdir):
            if name.endswith(".pdf") and "__" in name:
                base, stamp = name[:-4].rsplit("__", 1)
                held.add((base, stamp))
    return held


def repr_of(path):
    """The comparison key: section kind -> sorted distinct CT tokens per day."""
    out = collections.defaultdict(lambda: collections.defaultdict(set))
    for section, day, lines in W.blocks(path):
        if section is None:
            continue  # a day heading before any section heading carries no lines
        kind = W.section_kind(section)
        for line in lines:
            for token in W.ct_clocks(line):
                out[kind][day].add(token)
    return {kind: {day: sorted(tokens) for day, tokens in days.items()}
            for kind, days in out.items()}


def diffs(earlier, cited):
    out = []
    for kind in sorted(set(earlier) | set(cited)):
        a = earlier.get(kind, {})
        b = cited.get(kind, {})
        for day in sorted(set(a) | set(b)):
            if a.get(day) != b.get(day):
                out.append({"section": kind, "day": day,
                            "earlier": a.get(day), "cited": b.get(day)})
    return out


def main(argv=None):
    parser = argparse.ArgumentParser()
    parser.add_argument("--research", default=os.environ.get("WAVE5_RESEARCH"))
    parser.add_argument("--out", default="tools/out/wave5")
    args = parser.parse_args(argv)
    if not args.research:
        print("WAVE5_RESEARCH must name the research store root", file=sys.stderr)
        return 2
    root = os.path.join(args.research, "holidays", "raw")

    with open(os.path.join(args.research, "holidays",
                           "cme-2013-2015.json")) as handle:
        block = json.load(handle)
    cited = {}
    for code, entry in block["documents"].items():
        name = os.path.basename(entry["file"])
        if name.endswith(".pdf"):
            cited[name] = entry

    cdx = load_cdx(os.path.join(root, "cme-2013-2015", "cdx_holiday_calendar.json"))
    held = manifests(root)
    retrieval = {}
    retrieval_path = os.path.join(root, "cme-2013-2015-repair-r2", "retrieval.json")
    if os.path.exists(retrieval_path):
        with open(retrieval_path) as handle:
            for item in json.load(handle):
                base = item["base"]
                if not base.lower().endswith(".pdf"):
                    base += ".pdf"
                retrieval[(base, item["timestamp"])] = item

    by_url = collections.defaultdict(list)
    for row in cdx:
        if row["statuscode"] != "200" or not row["original"].lower().endswith(".pdf"):
            continue
        name = row["original"].rsplit("/", 1)[-1]
        by_url[name].append(row)

    report = {"task": "cme-2013-2015", "retrieved_on_utc": "2026-09-17",
              "urls": [], "counts": {}}
    total_captures = 0
    total_earlier = 0
    total_divergent = 0
    divergent_dates = []

    for name in sorted(cited):
        rows = sorted(by_url.get(name, []), key=lambda r: r["timestamp"])
        cited_stamp = cited[name]["doc_id_long"].rsplit("@", 1)[-1]
        cited_path = os.path.join(root, CITED_DIR, "%s__%s.txt" % (
            name[:-4], cited_stamp))
        cited_repr = repr_of(cited_path) if os.path.exists(cited_path) else None
        url_entry = {
            "file": name,
            "cited_capture": cited[name]["capture_utc"],
            "cited_sha256": cited[name].get("sha256"),
            "captures": [],
        }
        for row in rows:
            stamp = row["timestamp"]
            item = retrieval.get((name, stamp))
            entry = {
                "capture": stamp,
                "cdx_digest": row["digest"],
                "cdx_length": row["length"],
                "held_before_this_wave": (name, stamp) in held,
                "cited": stamp == cited_stamp,
            }
            if item:
                entry["sha256"] = item.get("sha256")
                entry["bytes"] = item.get("bytes")
                path = os.path.join(root, NEW_DIR, "%s__%s.txt" % (name[:-4], stamp))
                entry["last_updated"] = W.last_updated(path)
                if cited_repr is not None and not entry["cited"]:
                    delta = diffs(repr_of(path), cited_repr)
                    entry["differs_from_cited"] = bool(delta)
                    entry["differences"] = delta
                    total_earlier += 1
                    if delta:
                        total_divergent += 1
                        divergent_dates.append({
                            "file": name, "capture": stamp,
                            "earlier_last_updated": entry["last_updated"],
                            "cited_last_updated": W.last_updated(cited_path),
                            "differences": delta,
                        })
            url_entry["captures"].append(entry)
            total_captures += 1
        report["urls"].append(url_entry)

    report["counts"] = {
        "cited_pdf_urls": len(cited),
        "distinct_digest_captures": total_captures,
        "earlier_captures_reconciled": total_earlier,
        "earlier_captures_with_a_session_value_difference": total_divergent,
        "retrieved_for_this_repair": len(retrieval),
    }
    report["divergent"] = divergent_dates

    os.makedirs(args.out, exist_ok=True)
    with open(os.path.join(args.out, "reconciliation.json"), "w") as handle:
        json.dump(report, handle, indent=1)
        handle.write("\n")
    print(json.dumps(report["counts"], indent=1))
    for item in divergent_dates:
        print("DIVERGENT %s @%s (earlier %s vs cited %s): %d date/section pair(s)"
              % (item["file"], item["capture"], item["earlier_last_updated"],
                 item["cited_last_updated"], len(item["differences"])))
        for diff in item["differences"]:
            print("    %-10s %-22s earlier=%s cited=%s"
                  % (diff["section"], diff["day"],
                     ",".join(diff["earlier"] or []),
                     ",".join(diff["cited"] or [])))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
