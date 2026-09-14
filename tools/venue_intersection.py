#!/usr/bin/env python3
# SPDX-License-Identifier: MIT-0
"""Derive the four CME venue holiday tables from the eight family tables.

Throwaway stage-2.2 wave-3 tool (design memo D17). It parses the `holidays!`
blocks of the eight CME family modules under ``src/calendar/schedules/holidays/``,
applies the routing recorded in ``venues.rs``'s module doc, and prints, per venue
and per trade date in the era, the joint kind plus the family-level evidence for
it. Nothing here reads the venue modules back: the venue rows are a projection of
the family rows.

Usage::

    python3 tools/venue_intersection.py                 # the era report
    python3 tools/venue_intersection.py --era 2016 2018 # any audited era
    python3 tools/venue_intersection.py --emit          # Rust rows + citations
    python3 tools/venue_intersection.py --check         # compare with venues.rs

Deterministic: same files in, same text out.
"""

from __future__ import annotations

import argparse
import datetime as dt
import pathlib
import re
import sys

ROOT = pathlib.Path(__file__).resolve().parents[1]
HOLIDAYS = ROOT / "src" / "calendar" / "schedules" / "holidays"

# The routing recorded in `holidays/venues.rs`'s module doc (a decision, not a
# derivation): venue -> the families whose tables it intersects, in the order
# the doc lists them.
ROUTING: dict[str, list[str]] = {
    "cme": [
        "globex_equity_index",
        "globex_energy",
        "globex_fx",
        "globex_grains",
        "globex_interest_rates",
        "globex_livestock",
    ],
    "cbot": ["globex_grains", "globex_interest_rates"],
    "comex": ["globex_energy"],
    "nymex": ["globex_energy"],
}

ALL_FAMILIES = [
    "globex_equity_index",
    "globex_energy",
    "globex_fx",
    "globex_grains",
    "globex_interest_rates",
    "globex_livestock",
    "globex_cryptocurrency",
    "globex_nikkei_225_dollar",
]

# The venue module each emitted table belongs to.
VENUE_FILE = {
    "cme": "cme.rs",
    "cbot": "cbot.rs",
    "comex": "comex.rs",
    "nymex": "nymex.rs",
}

WINDOW_RE = re.compile(
    r"\(\s*(\d+)\s*,\s*(\d+)\s*,\s*(\d+)\s*\)\s*\.\.=\s*"
    r"\(\s*(\d+)\s*,\s*(\d+)\s*,\s*(\d+)\s*\)"
)
CONST_RE = re.compile(r"^\s*const\s+([A-Za-z_][A-Za-z0-9_]*)\s*:\s*u32\s*=\s*([^;]+);")


def numeric(expression: str) -> int:
    """Reads a `h * 3_600 + m * 60` expression, exactly as the crate writes it."""
    total = 0
    for term in expression.split("+"):
        product = 1
        for factor in term.split("*"):
            factor = factor.strip().replace("_", "")
            if not factor.isdigit():
                raise ValueError(f"not a holiday instant expression: {expression!r}")
            product *= int(factor)
        total += product
    return total


def stamp(seconds: int) -> str:
    """A seconds-since-local-midnight instant as `hh:mm`."""
    return f"{seconds // 3600:02d}:{seconds % 3600 // 60:02d}"


class Row:
    """One `holidays!` tuple: its trade date, its kind, its tier and its id."""

    __slots__ = ("date", "kind", "tier", "document")

    def __init__(self, date: dt.date, kind: str, tier: str, document: str) -> None:
        self.date = date
        self.kind = kind
        self.tier = tier
        self.document = document

    def __repr__(self) -> str:
        return f"Row({self.date}, {self.kind}, {self.tier}, {self.document!r})"


class Table:
    """One family's parsed `holidays!` block."""

    def __init__(self, windows: list[tuple[dt.date, dt.date]], rows: list[Row]) -> None:
        self.windows = windows
        self.rows = {row.date: row for row in rows}
        self.ordered = rows

    def covers(self, date: dt.date) -> bool:
        return any(first <= date <= last for first, last in self.windows)

    def row(self, date: dt.date) -> Row | None:
        return self.rows.get(date)


def split_tuples(body: str) -> list[str]:
    """Splits a `(…), (…), …` list into its top-level tuple bodies."""
    tuples: list[str] = []
    depth = 0
    start = None
    in_string = False
    for index, character in enumerate(body):
        if in_string:
            if character == '"':
                in_string = False
            continue
        if character == '"':
            in_string = True
        elif character == "(":
            if depth == 0:
                start = index + 1
            depth += 1
        elif character == ")":
            depth -= 1
            if depth == 0 and start is not None:
                tuples.append(body[start:index])
                start = None
    return tuples


def split_fields(tuple_body: str) -> list[str]:
    """Splits one tuple body on its top-level commas."""
    fields: list[str] = []
    depth = 0
    in_string = False
    current = ""
    for character in tuple_body:
        if in_string:
            current += character
            if character == '"':
                in_string = False
            continue
        if character == '"':
            in_string = True
            current += character
        elif character == "(":
            depth += 1
            current += character
        elif character == ")":
            depth -= 1
            current += character
        elif character == "," and depth == 0:
            fields.append(current.strip())
            current = ""
        else:
            current += character
    fields.append(current.strip())
    return fields


def normalize_kind(kind: str, constants: dict[str, str]) -> str:
    """Reduces one kind expression to the crate's own scalar vocabulary."""
    # Longest name first: QUARTER_PAST_NOON contains NOON.
    for name in sorted(constants, key=len, reverse=True):
        kind = re.sub(rf"\b{re.escape(name)}\b", constants[name], kind)
    kind = re.sub(r"\s+", "", kind)
    if kind in ("Closed", "HolidayKind::Closed"):
        return "Closed"
    if kind in ("Unsourced", "HolidayKind::Unsourced"):
        return "Unsourced"
    for name, arity in (
        ("early_close", 1),
        ("late_open", 1),
        ("late_open_and_early_close", 2),
    ):
        prefix = f"{name}("
        if kind.startswith(prefix) and kind.endswith(")"):
            arguments = kind[len(prefix) : -1].split(",")
            if len(arguments) != arity:
                raise ValueError(f"{name} takes {arity} arguments: {kind!r}")
            stamps = [stamp(numeric(argument)) for argument in arguments]
            return f"{name}({', '.join(stamps)})"
    raise ValueError(f"unrecognized holiday kind: {kind!r}")


def parse_table_text(text: str) -> Table:
    """Parses any module's `holidays!` block, given its source text."""
    constants = {
        match.group(1): match.group(2)
        for match in (CONST_RE.match(line) for line in text.splitlines())
        if match
    }
    body = text[text.index("holidays! {") :]
    coverage_body, _, rest = body.partition("rows:")
    windows = [
        (
            dt.date(int(match[0]), int(match[1]), int(match[2])),
            dt.date(int(match[3]), int(match[4]), int(match[5])),
        )
        for match in WINDOW_RE.findall(coverage_body)
    ]
    if not windows:
        raise ValueError("no coverage windows parsed")
    rows = []
    for tuple_body in split_tuples(rest):
        fields = split_fields(tuple_body)
        if len(fields) != 6:
            raise ValueError(f"a row has {len(fields)} fields: {tuple_body!r}")
        rows.append(
            Row(
                dt.date(int(fields[0]), int(fields[1]), int(fields[2])),
                normalize_kind(fields[3], constants),
                fields[4],
                fields[5].strip().strip('"'),
            )
        )
    return Table(windows, rows)


def parse_module(name: str) -> Table:
    """Parses one family module's `holidays!` block."""
    return parse_table_text((HOLIDAYS / f"{name}.rs").read_text())


class Joint:
    """The intersection's answer for one trade date."""

    __slots__ = ("state", "kind", "document", "tier", "stated", "abstained")

    def __init__(self, state, kind, document, tier, stated, abstained) -> None:
        self.state = state  # "normal" | "agreed" | "disputed"
        self.kind = kind  # the agreed kind, or None
        self.document = document  # the id an agreed or withheld row cites
        self.tier = tier  # that row's tier
        self.stated = stated  # [(family, kind-or-None, document-or-None)]
        self.abstained = abstained  # [family]


def intersect(tables: dict[str, Table], families: list[str], date: dt.date) -> Joint:
    abstained = [family for family in families if not tables[family].covers(date)]
    stated = []
    for family in families:
        if family in abstained:
            continue
        row = tables[family].row(date)
        stated.append(
            (
                family,
                None if row is None else row.kind,
                None if row is None else row.document,
                None if row is None else row.tier,
            )
        )
    values = {kind for _, kind, _, _ in stated}
    if not stated or values == {None}:
        return Joint("normal", None, None, None, stated, abstained)
    if len(values) == 1:
        kind = next(iter(values))
        document, tier = next(
            (doc, tier)
            for _, value, doc, tier in stated
            if value == kind and doc is not None
        )
        return Joint("agreed", kind, document, tier, stated, abstained)
    document, tier = next(
        ((doc, tier) for _, value, doc, tier in stated if value is not None),
        (None, None),
    )
    return Joint("disputed", None, document, tier, stated, abstained)


def describe(joint: Joint) -> str:
    """The per-family summary of one disputed date, in routing order."""
    parts = [f"{family} {human_kind(kind)}" for family, kind, _, _ in joint.stated]
    parts.extend(f"{family} abstains" for family in joint.abstained)
    return "; ".join(parts)


def human_kind(kind: str | None) -> str:
    """One family's kind in the evidence files' own words."""
    if kind is None:
        return "no row"
    if kind == "Closed":
        return "closed"
    if kind == "Unsourced":
        return "unsourced"
    if kind.startswith("early_close("):
        return f"early close {kind[len('early_close(') : -1]} CT"
    if kind.startswith("late_open_and_early_close("):
        inside = kind[len("late_open_and_early_close(") : -1].split(", ")
        return f"late open {inside[0]} CT and early close {inside[1]} CT"
    if kind.startswith("late_open("):
        return f"late open {kind[len('late_open(') : -1]} CT"
    raise ValueError(f"unrecognized kind: {kind!r}")


def document_kind(kind: str | None) -> str:
    """The evidence file's `kind` cell for one shipped kind.

    A venue row that states an instant is the energy half's own row (COMEX and
    NYMEX), and a venue `Unsourced` row is the intersection's residue.
    """
    if kind is None:
        return "unsourced"
    if kind == "Closed":
        return "closed"
    if kind == "Unsourced":
        return "unsourced"
    return human_kind(kind).removesuffix(" CT")


def rust_kind(kind: str) -> str:
    """The module spelling of one kind, as the existing rows write it."""
    if kind in ("Closed", "Unsourced"):
        return kind
    for name in ("early_close", "late_open", "late_open_and_early_close"):
        prefix = f"{name}("
        if kind.startswith(prefix):
            arguments = []
            for value in kind[len(prefix) : -1].split(", "):
                hours, minutes = (int(part) for part in value.split(":"))
                arguments.append(
                    f"{hours} * 3_600" if minutes == 0 else f"{hours} * 3_600 + {minutes} * 60"
                )
            return f"{name}({', '.join(arguments)})"
    raise ValueError(f"unrecognized kind: {kind!r}")


def venue_rows(tables: dict[str, Table], venue: str, era: tuple[dt.date, dt.date]):
    """Every joint answer in the era, in ascending trade-date order."""
    date = era[0]
    while date <= era[1]:
        yield date, intersect(tables, ROUTING[venue], date)
        date += dt.timedelta(days=1)


def shipped(joint: Joint) -> str | None:
    """The kind the venue ships for one joint answer."""
    if joint.state == "agreed":
        return joint.kind
    if joint.state == "disputed":
        return "Unsourced"
    return None


def report(tables: dict[str, Table], era: tuple[dt.date, dt.date]) -> None:
    for venue in ROUTING:
        counts: dict[str, int] = {}
        documents: set[str] = set()
        disputed = []
        days = 0
        for date, joint in venue_rows(tables, venue, era):
            days += 1
            key = shipped(joint) or "audited normal (no row)"
            counts[key] = counts.get(key, 0) + 1
            if joint.document:
                documents.add(joint.document)
            if joint.state == "disputed":
                disputed.append((date, joint))
        print(f"== {venue} ({', '.join(ROUTING[venue])}) ==")
        print(f"   era {era[0]}..{era[1]}: {days} trade dates")
        for key in sorted(counts):
            print(f"   {key}: {counts[key]}")
        print(f"   document ids: {', '.join(sorted(documents)) or '-'}")
        for date, joint in disputed:
            print(f"   {date} DISPUTED -> Unsourced [{joint.document}] ({joint.tier})")
            for family, kind, document, _ in joint.stated:
                suffix = "" if document is None else f" [{document}]"
                print(f"      {family}: {human_kind(kind)}{suffix}")
            for family in joint.abstained:
                print(f"      {family}: abstains (no window for this date)")
        print()


def emit(tables: dict[str, Table], era: tuple[dt.date, dt.date]) -> None:
    for venue in ROUTING:
        print(f"// ---- {VENUE_FILE[venue]} ----")
        for date, joint in venue_rows(tables, venue, era):
            kind = shipped(joint)
            if kind is None:
                continue
            if joint.state == "disputed":
                reason = f"disagreement: {describe(joint)}"
            elif kind == "Closed":
                reason = "closed: every routed family states a closure"
            else:
                reason = human_kind(kind)
            print(
                f"        // {date} - {joint.tier} - {joint.document} - {reason}."
            )
            print(
                f"        ({date.year}, {date.month}, {date.day}, "
                f"{rust_kind(kind)}, {joint.tier}, \"{joint.document}\"),"
            )
        print()


def check(tables: dict[str, Table], era: tuple[dt.date, dt.date]) -> int:
    """Compares the derivation with the venue modules' own rows in the era.

    The kind and the tier are compared exactly. The document id is checked
    against the rule rather than against the tool's own pick: a venue row's id
    must be one a routed family cites on that same date. A different admissible
    pick is reported as a note, not a failure.
    """
    failures = 0
    for venue in ROUTING:
        module = parse_table_text((HOLIDAYS / "venues" / VENUE_FILE[venue]).read_text())
        for date, joint in venue_rows(tables, venue, era):
            row = module.row(date)
            expected = shipped(joint)
            actual = None if row is None else row.kind
            if expected != actual:
                failures += 1
                print(f"MISMATCH {venue} {date}: derived kind {expected!r}, module {actual!r}")
                continue
            if expected is None:
                continue
            if row.tier != joint.tier:
                failures += 1
                print(
                    f"MISMATCH {venue} {date}: derived tier {joint.tier!r}, "
                    f"module {row.tier!r}"
                )
            admissible = {
                document
                for _, kind, document, _ in joint.stated
                if kind is not None and document is not None
            }
            if row.document not in admissible:
                failures += 1
                print(
                    f"MISMATCH {venue} {date}: module id {row.document!r} is not one a "
                    f"routed family cites on the date: {sorted(admissible)}"
                )
            elif row.document != joint.document:
                print(
                    f"note {venue} {date}: module cites {row.document!r}, the tool's "
                    f"first-family pick is {joint.document!r} (both admissible)"
                )
    return failures


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--era", nargs=2, type=int, default=[2022, 2024])
    parser.add_argument("--emit", action="store_true")
    parser.add_argument("--check", action="store_true")
    arguments = parser.parse_args()
    era = (dt.date(arguments.era[0], 1, 1), dt.date(arguments.era[1], 12, 31))
    tables = {name: parse_module(name) for name in ALL_FAMILIES}
    for name, table in tables.items():
        print(
            f"{name}: {len(table.ordered)} rows, windows "
            + ", ".join(f"{first}..{last}" for first, last in table.windows),
            file=sys.stderr,
        )
    if arguments.emit:
        emit(tables, era)
        return 0
    report(tables, era)
    if arguments.check:
        return 1 if check(tables, era) else 0
    return 0


if __name__ == "__main__":
    sys.exit(main())
