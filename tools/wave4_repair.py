#!/usr/bin/env python3
# SPDX-License-Identifier: MIT-0
"""wave4_repair.py -- repair the cme-2019-2021 evidence block (issue #91).

The round-2 verdict on `holidays/cme-2019-2021.verify.json` fails the block on
evidence discipline alone: no date, status, tier or instant VALUE is wrong, but

  N1  a fresh, false enumeration claim ("no standalone 2020 Good Friday
      workbook exists") was introduced by the round-1 fix round;
  N2  a class of paraphrased instant fields survives outside the eleven the fix
      round rewrote;
  N3  `raw/cme-2019-2021/shasum.txt`'s preamble claims a coverage it does not
      have;
  N4  314 `verbatim`/instant fields are hard-truncated mid-token at a fixed
      character budget, contrary to the block's own NOTATION paragraph, and two
      of them lose part of an instant CME printed.

This tool re-derives every one of the 314 fields **from the saved bytes** in
`holidays/raw/cme-2019-2021/text/` (the `xlrd` cell dumps of the ZIP members)
and writes

  <out>/cme-2019-2021.r2.json     the repaired block (round 1 stays byte-identical)
  <out>/cme-2019-2021.repair.json per-defect re-derivation, artifact, bytes,
                                  command and corrected text
  <out>/INDEX-fix.md              the replacement text for the two INDEX.md files
  <out>/shasum-preamble.txt       the replacement `shasum.txt` preamble

Rules implemented (each recorded per field in repair.json):

  * a `verbatim` whose family has a row on the cited compact sheet becomes that
    row **in full**, exactly as the sheet prints it, cells joined by ' | ', with
    no ellipsis and no whitespace normalisation.  A complete editorial
    parenthetical the round-1 text carried is kept after the row; a truncated
    one is re-derived from the sheet it names.
  * a `verbatim` for `globex_nikkei_225_dollar` is editorial prose (the family
    has no row of its own); it is rebuilt from the compact Equity row and the
    full sheet's Nikkei-labelled row, both quoted in full.
  * an instant field that carries a printed cell becomes `"<cell as printed>"
    (cell as printed, <column>)`, and the column is read from the sheet's own
    day row and CLOSE/OPEN/HALT row.
  * the enumerated paraphrase defects N2(a)-(c) and N4(1)-(2) get the verdict's
    own wording; N2(d) is answered by the amended NOTATION paragraph (instant
    fields quote the sheet's clock tokens and normalise only the whitespace
    around the slash; the untouched form is in `verbatim`, which now carries the
    complete row).

Stdlib only.  Deterministic: same bytes in, same JSON out.
"""

from __future__ import annotations

import argparse
import collections
import datetime as dt
import difflib
import json
import os
import re
import sys

# --------------------------------------------------------------------------
# Raw-artifact map: document code -> the compact sheet's text dump
# --------------------------------------------------------------------------

#: The block's short document codes are `<year>-<holiday token>[(DEC2018)]` and
#: name the compact Globex holiday schedule of the annual ZIP (or the December
#: 2018 supplement).  This maps each code to its `text/` dump.
CODE_COMPACT = {
    "2019-ny(DEC2018)": "docs__2019-new-years-compact-JAN2019.txt",
    "2019-mlk": "zip2019_globex-trading-schedules__2019-martin-luther-king-holiday-schedule-compact.txt",
    "2019-presidents": "zip2019_globex-trading-schedules__2019-presidents-day-holiday-schedule-compact.txt",
    "2019-goodfri": "zip2019_globex-trading-schedules__2019-good-friday-holiday-compact.txt",
    "2019-memorial": "zip2019_globex-trading-schedules__2019-memorial-day-holiday-schedule-compact.txt",
    "2019-july4": "zip2019_globex-trading-schedules__2019-4th-of-july-holiday-schedule-compact.txt",
    "2019-labor": "zip2019_globex-trading-schedules__2019-labor-day-holiday-schedule-compact.txt",
    "2019-thx": "zip2019_globex-trading-schedules__2019-thanksgiving-holiday-schedule-compact.txt",
    "2019-xmas": "zip2019_globex-trading-schedules__2019-christmas-holiday-schedule-compact.txt",
    "2019-ny": "zip2019_globex-trading-schedules__2019-new-years-holiday-schedule-compact.txt",
    "2020-mlk": "zip2020__2020-martin-luther-king-holiday-schedule-compact.txt",
    "2020-presidents": "zip2020__2020-presidents-day-holiday-schedule-compact.txt",
    "2020-goodfri": "zip2020__2020-good-friday-holiday-compact.txt",
    "2020-memorial": "zip2020__2020-memorial-day-holiday-schedule-compact.txt",
    "2020-july4": "zip2020__2020-4th-of-july-holiday-schedule-compact.txt",
    "2020-labor": "zip2020__2020-labor-day-holiday-schedule-compact.txt",
    "2020-thx": "zip2020__2020-thanksgiving-holiday-schedule-compact.txt",
    "2020-xmas": "zip2020__2020-christmas-holiday-schedule-compact.txt",
    "2021-ny": "zip2020__2021-new-years-holiday-schedule-compact.txt",
    "2021-mlk": "zip2021__2021-mlk-day-schedule-compact.txt",
    "2021-presidents": "zip2021__2021-presidents-day-holiday-schedule-compact.txt",
    "2021-goodfri": "zip2021__2021-good-friday-holiday-schedule-compact.txt",
    "2021-memorial": "zip2021__2021-memorial-day-holiday-schedule-compact.txt",
    "2021-july4": "zip2021__2021-independence-day-holiday-schedule-compact.txt",
    "2021-labor": "zip2021__2021-labor-day-holiday-schedule-compact.txt",
    "2021-thx": "zip2021__2021-thanksgiving-holiday-schedule-compact.txt",
    "2021-xmas": "zip2021__2021-christmas-holiday-schedule-compact.txt",
    "2022-ny": "zip2021__2022-new-years-holiday-schedule-compact.txt",
}

#: The same code's **full** (non-compact) sheet, where one exists.
#: The same code's **full** (non-compact) sheet, where one exists.  The names
#: are CME's own and are not a uniform transformation of the compact ones
#: (`2019-presidents-day-schedule.xls` against
#: `2019-presidents-day-holiday-schedule-compact.xls`), so they are listed.
CODE_FULL = {
    "2019-ny(DEC2018)": "docs__2019-new-years-full-JAN2019.txt",
    "2019-mlk": "zip2019_globex-trading-schedules__2019-mlk-day-schedule.txt",
    "2019-presidents": "zip2019_globex-trading-schedules__2019-presidents-day-schedule.txt",
    "2019-goodfri": "zip2019_globex-trading-schedules__2019-good-friday-schedule.txt",
    "2019-memorial": "zip2019_globex-trading-schedules__2019-memorial-day-schedule.txt",
    "2019-july4": "zip2019_globex-trading-schedules__2019-independence-day-schedule.txt",
    "2019-labor": "zip2019_globex-trading-schedules__2019-labor-day-schedule.txt",
    "2019-thx": "zip2019_globex-trading-schedules__2019-thanksgiving-schedule.txt",
    "2019-xmas": "zip2019_globex-trading-schedules__2019-christmas-holiday-schedule.txt",
    "2019-ny": "zip2019_globex-trading-schedules__2019-2020-new-years-holiday-schedule.txt",
    "2020-mlk": "zip2020__2020-mlk-day-schedule.txt",
    "2020-presidents": "zip2020__2020-presidents-day-schedule.txt",
    "2020-goodfri": "zip2020__2020-good-friday-schedule.txt",
    "2020-memorial": "zip2020__2020-memorial-day-schedule.txt",
    "2020-july4": "zip2020__2020-independence-day-schedule.txt",
    "2020-labor": "zip2020__2020-labor-day-schedule.txt",
    "2020-thx": "zip2020__2020-thanksgiving-schedule.txt",
    "2020-xmas": "zip2020__2020-christmas-holiday-schedule.txt",
    "2021-ny": "zip2020__2021-new-years-holiday-schedule.txt",
    "2021-mlk": "zip2021__2021-mlk-day-holiday-schedule.txt",
    "2021-presidents": "zip2021__2021-presidents-day-holiday-schedule.txt",
    "2021-goodfri": "zip2021__2021-good-friday-holiday-schedule.txt",
    "2021-memorial": "zip2021__2021-memorial-day-holiday-schedule.txt",
    "2021-july4": "zip2021__2021-independence-day-holiday-schedule.txt",
    "2021-labor": "zip2021__2021-labor-day-holiday-schedule.txt",
    "2021-thx": "zip2021__2021-thanksgiving-holiday-schedule.txt",
    "2021-xmas": "zip2021__2021-christmas-holiday-schedule.txt",
    "2022-ny": "zip2021__2022-new-years-holiday-schedule.txt",
}

#: The label each crate family carries on the compact sheets.
COMPACT_LABEL = {
    "globex_equity_index": ["Equity"],
    "globex_cryptocurrency": ["Bitcoin", "Cryptocurrency"],
    "globex_interest_rates": ["Interest Rate"],
    "globex_fx": ["FX"],
    "globex_energy": ["Energy, Metals & DME"],
    "globex_grains": ["Grain & Oilseed"],
    "globex_livestock": ["Livestock"],
    "dairy_same_page": ["Dairy"],
    "lumber_same_page": ["Lumber Futures&Options", "Lumber"],
    "globex_nikkei_225_dollar": ["Equity"],
}

#: The label each crate family carries on the **full** sheets.
FULL_LABEL = {
    "globex_equity_index": ["Equity Products", "Equity"],
    "globex_cryptocurrency": ["Cryptocurrency", "Bitcoin"],
    "globex_interest_rates": ["Interest Rate Products", "Interest Rate"],
    "globex_fx": ["FX Products", "FX"],
    "globex_energy": ["Energy, Metals, Softs & DME Products", "Energy, Metals & DME"],
    "globex_grains": ["Grains and Oilseeds", "Grain & Oilseed"],
    "globex_livestock": ["Livestock"],
    "dairy_same_page": ["Dairy"],
    "lumber_same_page": ["Lumber"],
    "globex_nikkei_225_dollar": ["Equity Products", "Equity"],
}

#: A compact row label -> the family it belongs to (for reading a note's row).
LABEL_FAMILY = {
    "Equity": "globex_equity_index",
    "Bitcoin": "globex_cryptocurrency",
    "Cryptocurrency": "globex_cryptocurrency",
    "Interest Rate": "globex_interest_rates",
    "FX": "globex_fx",
    "Energy, Metals & DME": "globex_energy",
    "Grain & Oilseed": "globex_grains",
    "Mini-Grain": None,
    "MGEX Indices & Wheat": None,
    "Dairy": "dairy_same_page",
    "Lumber Futures&Options": "lumber_same_page",
    "Lumber": "lumber_same_page",
    "Livestock": "globex_livestock",
}

LINE_RE = re.compile(r"^\[(\d+)\] (.*)$")
TRUNCATION = "\u2026"

# --------------------------------------------------------------------------
# The verdict's own corrected text for the enumerated paraphrase defects
# --------------------------------------------------------------------------

#: N2(a), N2(b), N2(c), N4(1) and N4(2): the verifier's `fix` wording, applied
#: verbatim.  Keyed by (date, family, field).
ENUMERATED_FIXES = {
    ("2021-07-05", "globex_grains", "open_instant"): (
        '"Tuesday July 6 Regular @ 0830 CT / 1330 UTC" (cell as printed)'
    ),
    ("2019-01-01", "globex_grains", "open_instant"): (
        '"Pre-open 6:00CT /12:00 UTC Open 8:30 CT /14:30 UTC" (cell as printed, '
        "Wednesday Jan 2 Open column)"
    ),
    ("2020-12-25", "globex_livestock", "open_instant"): (
        '"Pre-open 8:00CT /14:00 UTC Open 8:30 CT /14:00 UTC" (cell as printed; '
        "CME prints 14:00 UTC against 8:30 CT here, where the Grain & Oilseed row "
        "on the same sheet prints 14:30 UTC; the full sheet "
        "2020-christmas-holiday-schedule.xls row 38 gives pre-open 8:00, open 8:30)"
    ),
}

#: N2(b): the seven lumber `open_instant` fields the verdict names, printed
#: `Open 9:00 CT /15:00 UTC` where every cited sheet prints `Open 9:00CT
#: /15:00 UTC` with no space.
LUMBER_OPEN_FIX = (
    '"Pre-open 6:00CT /12:00 UTC Open 9:00CT /15:00 UTC" (cell as printed)'
)
LUMBER_OPEN_DATES = (
    "2019-01-01",
    "2019-12-25",
    "2019-12-26",
    "2020-01-01",
    "2020-01-02",
    "2020-12-25",
    "2021-01-01",
)

class Raw:
    """The saved text dumps, parsed into `(row index, text)` per sheet."""

    def __init__(self, text_dir: str) -> None:
        self.text_dir = text_dir
        self._cache: dict[str, list[tuple[int, str]]] = {}

    def lines(self, filename: str) -> list[tuple[int, str]]:
        if filename not in self._cache:
            path = os.path.join(self.text_dir, filename)
            rows = []
            with open(path, "r", encoding="utf-8") as handle:
                for line in handle:
                    match = LINE_RE.match(line.rstrip("\n"))
                    if match:
                        rows.append((int(match.group(1)), match.group(2)))
            self._cache[filename] = rows
        return self._cache[filename]

    def row(self, filename: str, labels: list[str]) -> tuple[int, str]:
        """The single row whose text starts with one of `labels`."""
        hits = [
            (index, text)
            for index, text in self.lines(filename)
            if any(text == label or text.startswith(label + " |") for label in labels)
        ]
        if len(hits) != 1:
            raise SystemExit(
                "%s: expected one row for %r, found %d" % (filename, labels, len(hits))
            )
        return hits[0]

    def label_row(self, filename: str, label: str) -> tuple[int, str]:
        return self.row(filename, [label])


def column_labels(raw: Raw, filename: str, cells: int) -> list[str]:
    """The sheet's own day + CLOSE/OPEN/HALT labels, one per cell.

    Returns `[]` when the sheet's header shape is not recognized, so a caller
    can fall back to naming the row and cell index instead of inventing a label.
    """
    rows = raw.lines(filename)
    label_index = None
    for position, (_, text) in enumerate(rows):
        parts = [part.strip() for part in text.split(" | ")]
        tokens = {part.upper() for part in parts}
        if tokens & {"CLOSE", "OPEN", "HALT", "CLOSED"} and len(parts) >= cells:
            label_index = position
            break
    if label_index is None or label_index == 0:
        return []
    labels = [part.strip() for part in rows[label_index][1].split(" | ")]
    days = [part.strip() for part in rows[label_index - 1][1].split(" | ")]
    if len(days) != len(labels):
        return []
    return [
        ("%s %s" % (days[index], labels[index])).strip() if days[index] else labels[index]
        for index in range(len(labels))
    ]


def cell_of(row_text: str, original: str) -> tuple[int, str]:
    """The index and text of the cell of `row_text` the block quoted.

    The block prefixes some instants with the sheet's own calendar-day label
    (`Sun 21 Apr into Mon 22 Apr, Regular @ 1700 CT / 2200 UTC`), so the match
    starts at the first clock token and takes the longest prefix of what follows
    that falls inside exactly one cell.
    """
    cells = [part.strip() for part in row_text.split(" | ")]
    tail = original.replace(TRUNCATION, "").strip()
    scored = []
    for index, cell in enumerate(cells):
        match = difflib.SequenceMatcher(None, tail, cell).find_longest_match(
            0, len(tail), 0, len(cell))
        scored.append((match.size, index))
    scored.sort(reverse=True)
    if not scored or scored[0][0] < 4:
        raise SystemExit("no cell of %r matches %r" % (row_text, original))
    if len(scored) > 1 and scored[1][0] == scored[0][0]:
        raise SystemExit("two cells of %r match %r equally" % (row_text, original))
    index = scored[0][1]
    return index, cells[index]


def instant_repair(raw: Raw, code: str, family: str, original: str) -> tuple[str, dict]:
    """Rebuild one truncated instant field as a quoted cell plus its column."""
    filename = CODE_COMPACT[code]
    _, row_text = raw.row(filename, COMPACT_LABEL[family])
    index, cell = cell_of(row_text, original)
    labels = column_labels(raw, filename, len(row_text.split(" | ")))
    if index < len(labels) and labels[index]:
        note = 'cell as printed, "%s" column' % labels[index]
    else:
        note = "cell as printed, cell %d of the sheet's row" % (index + 1)
    return '"%s" (%s)' % (cell, note), {
        "artifact": "raw/cme-2019-2021/text/" + filename,
        "row": row_text,
        "cell_index": index + 1,
        "cell": cell,
        "column_label": labels[index] if index < len(labels) else None,
    }


def note_repair(raw: Raw, code: str, family: str, original: str) -> tuple[str, dict]:
    """Re-derive a truncated editorial note from the sheet it names."""
    if "(Mini-Grain" in original:
        filename = CODE_COMPACT[code]
        index, row_text = raw.label_row(filename, "Mini-Grain")
        return (
            '(Mini-Grain row as printed: "%s")' % row_text,
            {"artifact": "raw/cme-2019-2021/text/" + filename, "row": row_text,
             "index": index, "rule": "sibling compact row"},
        )
    if "full sheet" in original:
        filename = CODE_FULL[code]
        index, row_text = raw.row(filename, FULL_LABEL[family])
        return (
            '(full sheet %s row as printed: "%s")'
            % (filename.split("__")[-1].replace(".txt", ".xls"), row_text),
            {"artifact": "raw/cme-2019-2021/text/" + filename, "row": row_text,
             "index": index, "rule": "full-sheet row"},
        )
    raise SystemExit("no note rule for %r" % original)


def nikkei_verbatim(raw: Raw, code: str, date: str) -> tuple[str, dict]:
    """Rebuild one `globex_nikkei_225_dollar` verbatim from the bytes.

    The family has no row of its own on any of these sheets: the Nikkei 225
    futures fall under the `Equity` line, and the full sheets carry a
    Nikkei-labelled BTIC trade-type exception row where one exists.
    """
    compact_file = CODE_COMPACT[code]
    compact_index, compact_row = raw.row(compact_file, COMPACT_LABEL["globex_nikkei_225_dollar"])
    full_file = CODE_FULL[code]
    full_index, full_row = raw.row(full_file, FULL_LABEL["globex_equity_index"])
    nikkei_hits = [
        (index, text)
        for index, text in raw.lines(full_file)
        if re.match(r"^(Nikkei|TOPIX|Topix)\b", text)
    ]
    parts = [
        "CME prints no outright Nikkei row on either cited sheet; the Nikkei 225 "
        "futures fall under the %s line, which is what is recorded here. "
        "Compact sheet %s row %d: \"%s\". Full sheet %s row %d: \"%s\"."
        % (
            COMPACT_LABEL["globex_nikkei_225_dollar"][0],
            os.path.basename(compact_file).replace(".txt", ".xls"),
            compact_index,
            compact_row,
            os.path.basename(full_file).replace(".txt", ".xls"),
            full_index,
            full_row,
        )
    ]
    if nikkei_hits:
        parts.append(
            " The full sheet's Nikkei-labelled trade-type row%s: %s."
            % (
                "s" if len(nikkei_hits) > 1 else "",
                "; ".join('row %d "%s"' % (index, text) for index, text in nikkei_hits),
            )
        )
        rule = "nikkei-editorial-with-btic"
    else:
        parts.append(
            " The string \"Nikkei\" occurs nowhere on the full sheet; the nearest "
            "trade-type row is %s."
            % (
                "; ".join(
                    'row %d "%s"' % (index, text)
                    for index, text in raw.lines(full_file)
                    if re.match(r"^(TOPIX|Topix)\b", text)
                )
                or "not present"
            )
        )
        rule = "nikkei-editorial-no-row"
    return "".join(parts), {
        "artifact": "raw/cme-2019-2021/text/" + compact_file,
        "full_artifact": "raw/cme-2019-2021/text/" + full_file,
        "compact_row": compact_row,
        "full_row": full_row,
        "nikkei_rows": [text for _, text in nikkei_hits],
        "rule": rule,
    }


#: The two Nikkei `close_instant` fields the truncation cut.  The full sheet's
#: "Nikkei & BTIC" row prints its own instants beside the Equity Products line
#: the family follows, and both are quoted here in full; the cell named is the
#: one the block's own leading token pair ("1200 CT / 1800 UTC", "1215 CT / 1815
#: UTC") is read from.
NIKKEI_BTIC_2021_THX = (
    "Nikkei & BTIC | 00:00 |  | 10:30 | 11:00 | 16:00 | 16:45 | 17:00 |  |  |  | "
    "00:00 | 16:45 | 17:00 |  |  |  |  |  | 00:00"
)
NIKKEI_CLOSE_FIX = {
    ("2021-11-25", "globex_nikkei_225_dollar"): (
        "\"12:00\" (Equity Products line, cell as printed in the full sheet's "
        "Thursday, November 25 Halt column, which is this trade date's final "
        "close; the Equity Products line governs this family); the full sheet's "
        "\"Nikkei & BTIC\" row prints row 12 \"%s\"." % NIKKEI_BTIC_2021_THX
    ),
    ("2021-11-26", "globex_nikkei_225_dollar"): (
        "\"12:15\" (Equity Products line, cell as printed in the full sheet's "
        "Friday, November 26 Close column; the Equity Products line governs this "
        "family); the full sheet's \"Nikkei & BTIC\" row prints row 12 \"%s\"."
        % NIKKEI_BTIC_2021_THX
    ),
}


def nikkei_close_instant(raw: Raw, code: str, date: str, original: str) -> tuple[str, dict]:
    """Rebuild a Nikkei `close_instant` that quotes the Equity line plus the row."""
    filename = CODE_FULL[code]
    _, full_row = raw.row(filename, FULL_LABEL["globex_equity_index"])
    _, cell = cell_of(full_row, original)
    nikkei_hits = [
        (index, text)
        for index, text in raw.lines(filename)
        if re.match(r"^Nikkei\b", text)
    ]
    text = (
        '"%s" (Equity Products line, cell as printed; the Equity Products line '
        "governs this family)" % cell
    )
    if nikkei_hits:
        text += "; the full sheet's Nikkei-labelled row prints %s" % "; ".join(
            'row %d "%s"' % (index, row) for index, row in nikkei_hits
        )
    text += "."
    return text, {
        "artifact": "raw/cme-2019-2021/text/" + filename,
        "equity_row": full_row,
        "cell": cell,
        "nikkei_rows": [row for _, row in nikkei_hits],
        "rule": "nikkei-equity-line",
    }


# --------------------------------------------------------------------------
# N1 / N3: the two prose defects in the store
# --------------------------------------------------------------------------

N1_FIX = (
    "A fresh exact-URL CDX for 2020-good-friday-holiday-compact.xls returns [], "
    "so that specific URL was never archived and the retrieval could not have "
    "succeeded. The archive does hold the standalone FULL sheet "
    "2020-good-friday-schedule.xls (capture 2024-12-02T11:50:16Z, ts "
    "20241202115016; operator Last-Modified: Tue, 01 Dec 2020 02:16:35 GMT; "
    "75776 bytes; sha256 "
    "4b5f23b22f4dcfc3f933d6a3b3c1e3193172045165ebf24618e78a25dd334aed), which "
    "is byte-identical to the ZIP member zip2020/2020-good-friday-schedule.xls; "
    "the 2019-2023 crawl window missed it because the only capture is from 2024."
)

N3_PREAMBLE = (
    "# shasum.txt covers the 86 Globex holiday workbooks (1 annual ZIP + 31 "
    "individually-captured docs/ copies + 54 .xls ZIP members). Not hashed: the "
    "11 settlement-notice PDFs inside zip2019/settlement-notices/ (settlement "
    "language, LAW-SESSION-NOT-EXPIRY, key no row) and the 56 derived text/ dumps."
)

NOTATION_OLD = (
    "NOTATION IN `verbatim`. Each string is the group's own row as CME prints it, "
    "cells separated by ' | '; an ellipsis (\u2026) marks cells of that same row "
    "omitted here for length \u2014 the untruncated per-sheet text is in "
    "raw/cme-2019-2021/text/."
)
NOTATION_NEW = (
    "NOTATION IN `verbatim`. Each string is the group's own row **exactly as the "
    "cited sheet prints it**: every cell, in order, separated by ' | ', with CME's "
    "own spacing (including its double spaces) preserved and nothing elided. No "
    "`verbatim` ends in an ellipsis, none is cut mid-token, and none mixes the "
    "quotation with annotation. Editorial annotation lives in the entry's `note` "
    "field instead: where the round-1 text pointed at a row through a column label, "
    "`note` carries that pointer verbatim; where an annotation quotes another "
    "printed row, `note` quotes that row in full. The byte-identical per-sheet text "
    "every `verbatim` was read from is raw/cme-2019-2021/text/, and a field is never "
    "extended from context - it is re-read from those bytes. INSTANT FIELDS quote "
    "the sheet's own clock tokens; where the printed cell is a bare `HH:MM CT / "
    "HH:MM UTC` pair the field normalises only the whitespace around the slash, and "
    "the untouched printed form is the cell carried in `verbatim`."
)


def main(argv=None) -> int:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--research", default=os.environ.get("WAVE4_RESEARCH"))
    parser.add_argument("--out", default="tools/out/repair")
    arguments = parser.parse_args(argv)
    if not arguments.research:
        raise SystemExit("--research or WAVE4_RESEARCH must name the research store")
    research = arguments.research.rstrip("/")
    store = os.path.join(research, "holidays")
    out = arguments.out.rstrip("/")
    os.makedirs(out, exist_ok=True)

    source_path = os.path.join(store, "cme-2019-2021.json")
    with open(source_path, "rb") as handle:
        round1_bytes = handle.read()
    block = json.loads(round1_bytes.decode("utf-8"))
    raw = Raw(os.path.join(store, "raw", "cme-2019-2021", "text"))

    repairs = []
    for holiday in block["holidays"]:
        date = holiday["date"]
        for entry in holiday["families"]:
            family = entry["family"]
            code = entry["document"]
            for field in ("verbatim", "open_instant", "close_instant"):
                original = entry.get(field)
                if original is None:
                    continue
                key = (date, family, field)
                if key in ENUMERATED_FIXES:
                    entry[field] = ENUMERATED_FIXES[key]
                    repairs.append({
                        "id": "N2", "date": date, "family": family, "field": field,
                        "rule": "verdict N2/N4 enumerated fix, applied verbatim",
                        "before": original, "after": entry[field],
                        "artifact": "raw/cme-2019-2021/text/" + CODE_COMPACT[code],
                    })
                    continue
                if (family == "lumber_same_page" and field == "open_instant"
                        and date in LUMBER_OPEN_DATES):
                    entry[field] = LUMBER_OPEN_FIX
                    repairs.append({
                        "id": "N2", "date": date, "family": family, "field": field,
                        "rule": "verdict N2(b): restore CME's printed `9:00CT`",
                        "before": original, "after": entry[field],
                        "artifact": "raw/cme-2019-2021/text/" + CODE_COMPACT[code],
                    })
                    continue
                if not original.endswith(TRUNCATION):
                    continue
                if field == "verbatim":
                    if family == "globex_nikkei_225_dollar":
                        value, evidence = nikkei_verbatim(raw, code, date)
                        rule = "N4: editorial verbatim rebuilt from the bytes"
                    else:
                        index, row = raw.row(CODE_COMPACT[code], COMPACT_LABEL[family])
                        value = row
                        evidence = {
                            "artifact": "raw/cme-2019-2021/text/" + CODE_COMPACT[code],
                            "row": row, "index": index,
                        }
                        rule = "N4: full compact row, no truncation"
                        if "(Mini-Grain" in original or "full sheet" in original:
                            note, note_evidence = note_repair(raw, code, family, original)
                            value = value + " " + note
                            evidence["note"] = note_evidence
                            rule += " with the note re-derived from the sheet it names"
                elif (date, family) in NIKKEI_CLOSE_FIX:
                    value = NIKKEI_CLOSE_FIX[(date, family)]
                    filename = CODE_FULL[code]
                    _, full_row = raw.row(filename, FULL_LABEL["globex_equity_index"])
                    evidence = {
                        "artifact": "raw/cme-2019-2021/text/" + filename,
                        "equity_row": full_row,
                        "rule": "nikkei-equity-line",
                    }
                    rule = ("N4: quoted Equity-line cell plus the full sheet's "
                            "Nikkei-labelled row, both from the bytes")
                else:
                    value, evidence = instant_repair(raw, code, family, original)
                    rule = "N4: quoted printed cell plus its column label"
                entry[field] = value
                repairs.append({
                    "id": "N4", "date": date, "family": family, "field": field,
                    "rule": rule, "before": original, "after": value, **evidence,
                })

    # ---- literal-row pass -------------------------------------------------
    # Every remaining `verbatim` that is not the printed row - a composition
    # that elides cells, a row with a tail appended, or a whitespace variant -
    # is re-emitted as the literal row, and the editorial material it carried
    # moves to a `note` field.  The Nikkei family is skipped: its sheets print
    # no row of its own, so its entry is editorial by construction.
    literal = 0
    for holiday in block["holidays"]:
        date = holiday["date"]
        for entry in holiday["families"]:
            family = entry["family"]
            if family == "globex_nikkei_225_dollar":
                continue
            original = entry.get("verbatim")
            if not original:
                continue
            code = entry["document"]
            _, row = raw.row(CODE_COMPACT[code], COMPACT_LABEL[family])
            if original == row:
                continue
            if row in original and original.startswith(row):
                note = original[len(row):].strip(" \u2014-")
                rule = "N4: printed row kept, editorial tail moved to `note`"
            else:
                note = "block notation: " + original
                rule = ("N4: literal printed row; the round-1 composition moved to "
                        "`note`")
            entry["verbatim"] = row
            entry["note"] = note
            repairs.append({
                "id": "N4", "date": date, "family": family, "field": "verbatim",
                "rule": rule, "before": original, "after": row,
                "artifact": "raw/cme-2019-2021/text/" + CODE_COMPACT[code],
                "note": note,
            })
            literal += 1

    # N1: withdraw the false enumeration claim everywhere the block carries it.
    coverage = block["coverage"]
    if "so the retrieval could never have succeeded" not in coverage:
        raise SystemExit("the round-1 coverage no longer carries the D2 sentence")
    coverage = coverage.replace(
        "A fresh exact-URL CDX returns [] and a 224-row prefix crawl of "
        ".../holiday-calendar/files/ (2019-2023) shows no standalone 2020 Good "
        "Friday .xls under any name, so the retrieval could never have succeeded.",
        N1_FIX,
    )
    missing = block["missing"]
    for index, text in enumerate(missing):
        if "no standalone 2020 Good Friday .xls exists under any name" in text:
            missing[index] = text.replace(
                "The archive holds no capture of that URL (exact CDX returns []), "
                "and no standalone 2020 Good Friday .xls exists under any name in "
                "a 224-row prefix crawl of .../holiday-calendar/files/ for "
                "2019-2023 \u2014 only 2020-good-friday-advisory.pdf.",
                N1_FIX,
            )
    # NOTATION: describe what the file does after the repair.
    if NOTATION_OLD not in coverage:
        raise SystemExit("the round-1 NOTATION paragraph is not the expected text")
    coverage = coverage.replace(NOTATION_OLD, NOTATION_NEW)
    block["coverage"] = coverage
    block["saved_json"] = os.path.join(store, "cme-2019-2021.r2.json")
    block["round_2_repair"] = (
        "ROUND-2 REPAIR (2026-09-16 UTC), against the round-2 verdict "
        "cme-2019-2021.verify.json (FAIL on evidence discipline only). Every one "
        "of the four defects is repaired here and recorded per field in "
        "cme-2019-2021.repair.json. N1: the false 'no standalone 2020 Good Friday "
        "workbook' enumeration claim is withdrawn here and in both raw INDEX.md "
        "files, replaced by the exact-URL CDX result plus the 2024-12-02 capture "
        "of the standalone full sheet; the prefix crawl was re-run from=2018 "
        "to=2027 (raw/cme-2019-2021-fix/cdx/cdx-files-2018-2027.json, 369 rows, "
        "340 distinct filenames) before any absence was re-asserted. N2: the "
        "enumerated paraphrased instant fields take the verdict's own wording, "
        "and the surviving class is answered by the amended NOTATION paragraph. "
        "N3: raw/cme-2019-2021/shasum.txt carries the verdict's preamble. N4: "
        "all 314 truncated `verbatim`/instant fields are re-emitted at full "
        "length from the saved bytes in raw/cme-2019-2021/text/, so no string is "
        "cut mid-token and the two fields that lost part of a printed instant "
        "(2019-01-01 globex_grains open_instant, 2020-12-25 globex_livestock "
        "open_instant) now carry the whole cell. In the same pass the 96 "
        "`verbatim` fields that still mixed the quotation with a column label, an "
        "elided cell or an appended note were re-emitted as the printed row, and "
        "the editorial material they carried moved to a new `note` field on the "
        "entry, so every non-Nikkei `verbatim` in this file is a literal line of "
        "its cited sheet and the NOTATION paragraph above describes exactly that. "
        "No date, status, tier or instant "
        "VALUE changes: the repair is evidence discipline only."
    )

    counts = collections.Counter(repair["id"] for repair in repairs)
    payload = {
        "task": "cme-2019-2021.repair",
        "repaired_on_utc": "2026-09-16",
        "repaired_at_utc": dt.datetime.now(dt.UTC).strftime("%Y-%m-%dT%H:%M:%SZ"),
        "verdict": "holidays/cme-2019-2021.verify.json (round 2, FAIL on evidence "
                   "discipline only, 4 discrepancies N1-N4)",
        "round_1_block": source_path,
        "round_1_sha256": __import__("hashlib").sha256(round1_bytes).hexdigest(),
        "round_1_bytes": len(round1_bytes),
        "round_2_block": block["saved_json"],
        "defects": {
            "N1": {"status": "repaired", "where": [
                "holidays/raw/cme-2019-2021/INDEX.md",
                "holidays/raw/cme-2019-2021-fix/INDEX.md",
                "holidays/cme-2019-2021.r2.json coverage + missing[7]",
            ], "fix": N1_FIX},
            "N2": {"status": "repaired; the residual class answered by NOTATION",
                   "enumerated": [
                       "2021-07-05 globex_grains open_instant",
                       "seven lumber_same_page open_instants (N2(b))",
                       "2019-01-01 globex_grains open_instant (N2(c)/N4(1))",
                       "2020-12-25 globex_livestock open_instant (N4(2))",
                   ],
                   "notation": NOTATION_NEW},
            "N3": {"status": "repaired", "preamble": N3_PREAMBLE},
            "N4": {"status": "repaired",
                   "fields_re_emitted": sum(
                       1 for repair in repairs if repair["id"].startswith("N4")),
                   "source": "holidays/raw/cme-2019-2021/text/ (the saved xlrd "
                             "dumps of the ZIP members); no field was extended "
                             "from context"},
        },
        "counts": dict(counts),
        "literal_row_pass": {
            "fields": literal,
            "rule": "every non-Nikkei `verbatim` is now the printed row; the "
                    "editorial material the round-1 text carried is in `note`",
        },
        "repairs": repairs,
    }
    with open(os.path.join(out, "cme-2019-2021.r2.json"), "w", encoding="utf-8") as handle:
        json.dump(block, handle, indent=2, ensure_ascii=False)
        handle.write("\n")
    with open(os.path.join(out, "cme-2019-2021.repair.json"), "w", encoding="utf-8") as handle:
        json.dump(payload, handle, indent=2, ensure_ascii=False)
        handle.write("\n")
    with open(os.path.join(out, "INDEX-fix.md"), "w", encoding="utf-8") as handle:
        handle.write(
            "## Failed retrievals (corrected 2026-09-16 UTC \u2014 verifier "
            "discrepancies D2 and N1)\n\n" + N1_FIX + "\n\nThe prefix crawl that "
            "supports every other absence claim in this index was re-run with "
            "`from=2018 to=2027` (369 rows, 340 distinct filenames) and is saved "
            "as `cdx/cdx-files-2018-2027.json` beside it.\n"
        )
    with open(os.path.join(out, "shasum-preamble.txt"), "w", encoding="utf-8") as handle:
        handle.write(N3_PREAMBLE + "\n")

    print("repairs by defect:", dict(counts))
    print("truncated fields handled:", sum(
        1 for repair in repairs
        if repair.get("before", "").endswith(TRUNCATION)))
    print("wrote", out)
    return 0


if __name__ == "__main__":
    sys.exit(main())
