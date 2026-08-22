// SPDX-License-Identifier: MIT-0

//! Contracts for the source registry's repeatable monitoring entry points.

use super::{SOURCES, exchange_rows, market_hours_key_rows};

const SYNTHETIC_SOURCE_SET: &str = "synthetic-24x7";
const OFFICIAL_BULLET: &str = "- **Official current/rulebook:**";
const NOTICES_BULLET: &str = "- **Notices/evidence:**";

fn source_sets() -> impl Iterator<Item = (&'static str, &'static str)> {
    SOURCES.split("\n<a id=\"").skip(1).map(|section| {
        section
            .split_once("\"></a>")
            .expect("every source-set anchor must close before its section body")
    })
}

fn channel_body<'a>(section: &'a str, label: &str, anchor: &str) -> &'a str {
    assert_eq!(
        section.matches(label).count(),
        1,
        "source set must contain exactly one {label} bullet: {anchor}"
    );

    let (_, after_label) = section
        .split_once(label)
        .expect("the counted monitoring-channel bullet must be present");
    after_label
        .split_once("\n- **")
        .map_or(after_label, |(body, _)| body)
}

fn contains_markdown_link(text: &str) -> bool {
    if let Some((before_destination, after_open)) = text.split_once("](")
        && let Some((_, label)) = before_destination.rsplit_once('[')
        && let Some((destination, _)) = after_open.split_once(')')
    {
        return !label.trim().is_empty() && !destination.trim().is_empty();
    }

    false
}

#[test]
fn every_real_source_set_has_two_clickable_monitoring_channels() {
    let mut source_set_count = 0_u8;

    for (anchor, section) in source_sets() {
        source_set_count = source_set_count.saturating_add(1);
        if anchor == SYNTHETIC_SOURCE_SET {
            continue;
        }

        for label in [OFFICIAL_BULLET, NOTICES_BULLET] {
            let body = channel_body(section, label, anchor);
            assert!(
                contains_markdown_link(body),
                "source-set monitoring bullet must contain a Markdown link: {anchor} / {label}"
            );
        }
    }

    assert!(
        source_set_count > 1,
        "source registry must contain real sets"
    );
}

#[test]
fn every_source_set_is_referenced_by_a_ledger_row() {
    let mut ledger_rows = exchange_rows();
    ledger_rows.extend(market_hours_key_rows());

    for (anchor, _) in source_sets() {
        let destination = format!("(sources.md#{anchor})");
        assert!(
            ledger_rows.iter().any(|row| row.contains(&destination)),
            "source-set anchor is not referenced by an exchange or MarketHoursKey row: {anchor}"
        );
    }
}
