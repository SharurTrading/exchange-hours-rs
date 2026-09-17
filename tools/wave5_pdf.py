#!/usr/bin/env python3
# SPDX-License-Identifier: MIT-0
"""wave5_pdf.py -- a small reader for CME's 2013-2015 holiday-schedule PDFs.

The wave-5 evidence store holds two PDF corpora: the cited 32 round-0 captures
(`holidays/raw/cme-2013-2015/txt/`) and the 77 earlier distinct-digest captures
retrieved for the repair (`holidays/raw/cme-2013-2015-repair-r2/txt/`).  Both
are `pdftotext -layout` renderings, so this module turns one into a list of
`(section, date, lines)` blocks: the product section the sheet prints, the day
heading under it, and the printed lines that follow.

Stdlib only, deterministic.
"""

from __future__ import annotations

import os
import re

#: Every product-section heading CME printed on a 2013-2015 holiday sheet.
SECTION_RE = re.compile(
    r"^(?:"
    r"CME & CBOT Equity Products"
    r"|Equity Products"
    r"|CME Group Equity Products"
    r"|CME & CBOT Interest Rate & FX Products"
    r"|Interest Rate & FX Products"
    r"|CME Group Interest Rate Products"
    r"|CME Group FX Products"
    r"|NYMEX, COMEX ?(?:&|and) ?(?:Dubai Mercantile \(DME\)|DME) Products"
    r"|NYMEX, COMEX® and DME Products on CME Globex"
    r"|Energy, Metals & DME Products"
    r"|CBOT, KCBT, MGEX Grain & Agricultural Products"
    r"|Grain, Oilseed & MGEX Products"
    r"|Livestock, Dairy & Lumber Products"
    r"|Other CME Group Products(?: on CME Globex)?"
    r"|Agricultural, GSCI.*"
    r"|Korea Exchange \(KRX\)"
    r"|Bursa Malaysia Derivatives \(BMD\)"
    r")\s*$")

DAY_RE = re.compile(
    r"^(?:Mon|Tues|Tue|Wed|Thurs|Thu|Fri|Sat|Sun)(?:day)?\.?,?\s+"
    r"(?:Jan|Feb|Mar|Apr|May|Jun|Jul|Aug|Sep|Oct|Nov|Dec)[a-z]*\.?\s+\d{1,2}"
    r"(?:\s*,?\s*\d{4})?$")

CLOCK_CT = re.compile(r"\b(\d{3,4})\s*CT\b")

#: Section heading -> the crate families its line carries.
SECTION_FAMILIES = {
    "equity": ("globex_equity_index",),
    "rates_fx": ("globex_interest_rates", "globex_fx"),
    "energy": ("globex_energy",),
    "grains": ("globex_grains",),
    "livestock": ("globex_livestock",),
    "other": (),
}


def section_kind(heading):
    low = heading.lower()
    if "equity products" in low:
        return "equity"
    if "interest rate" in low or "fx products" in low:
        return "rates_fx"
    if "energy" in low or "nymex" in low:
        return "energy"
    if "grain" in low:
        return "grains"
    if "livestock" in low:
        return "livestock"
    return "other"


def blocks(path):
    """Yields ``(section, day, [line, ...])`` for every day heading."""
    with open(path, errors="replace") as handle:
        text = handle.read()
    section = None
    day = None
    lines = []
    for raw in text.splitlines():
        stripped = raw.strip()
        if not stripped:
            continue
        if SECTION_RE.match(stripped):
            if day is not None:
                yield section, day, lines
            section, day, lines = stripped, None, []
            continue
        if DAY_RE.match(stripped):
            if day is not None:
                yield section, day, lines
            day, lines = stripped, []
            continue
        if section is not None and day is not None:
            lines.append(stripped)
    if day is not None:
        yield section, day, lines


def ct_clocks(line):
    """Returns the CT clock times on a line, as three/four digit strings."""
    return CLOCK_CT.findall(line)


def last_updated(path):
    with open(path, errors="replace") as handle:
        text = handle.read()
    match = re.search(r"Last updated\s+(\d{1,2}/\d{1,2}/\d{4})", text)
    return match.group(1) if match else None


def corpus(root, subdir):
    """Returns {basename: [path, ...]} for one of the saved PDF text corpora."""
    directory = os.path.join(root, subdir)
    out = {}
    if not os.path.isdir(directory):
        return out
    for name in sorted(os.listdir(directory)):
        if not name.endswith(".txt"):
            continue
        stem = name[:-4]
        if "__" not in stem:
            continue
        base = stem.rsplit("__", 1)[0]
        out.setdefault(base, []).append(os.path.join(directory, name))
    return out
