// SPDX-License-Identifier: MIT-0

//! Small Exchange, Inc. venue history (now Kraken Derivatives Exchange).

use chrono_tz::America;

use crate::calendar::SessionRule;
use crate::calendar::rule::MON_FRI;
use crate::calendar::schedules::StaticHoursProfile;
use crate::calendar::schedules::timeline::{Revision, local_date, revisions, select_revision};

// Small Exchange listed its first contracts (SM75, SPRE and SFX) for trade date Monday 2020-05-18, and trades executed that morning. Narrative:
// docs/evidence/small_exchange.md.
static LAUNCH_REGULAR: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 7 * 3600,
    close_ssm: 16 * 3600,
}];
// ORDER ENTRY, NOT TRADING. Pre-Open quoting, ending in a no-cancel minute,
// accepts orders for the coming session and nothing matches until the open.
static LAUNCH_ORDER_ENTRY: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 6 * 3600 + 30 * 60,
    close_ssm: 7 * 3600,
}];
static LAUNCH: StaticHoursProfile = StaticHoursProfile {
    tz: America::Chicago,
    regular: LAUNCH_REGULAR,
    extended: &[],
    order_entry: LAUNCH_ORDER_ENTRY,
    has_daily_close: true,
    has_weekend_close: true,
};

// The S5C certification, dated 2024-11-21 and listing S5C for trade date 2024-11-25, states the Exchange's hours as 08:30-15:00 CT Monday-Friday. Narrative:
// docs/evidence/small_exchange.md.
static S5C_REGULAR: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 8 * 3600 + 30 * 60,
    close_ssm: 15 * 3600,
}];
// Same Pre-Open queue as at launch, so the same classification.
static S5C_ORDER_ENTRY: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 8 * 3600,
    close_ssm: 8 * 3600 + 30 * 60,
}];
static S5C_ERA: StaticHoursProfile = StaticHoursProfile {
    tz: America::Chicago,
    regular: S5C_REGULAR,
    extended: &[],
    order_entry: S5C_ORDER_ENTRY,
    has_daily_close: true,
    has_weekend_close: true,
};

// Every listed contract was delisted as of the close of business on
// 2025-03-21 (SMFE 2025-001), and the CFTC records the last trade as
// 2025-01-10. With nothing listed the venue accepts no orders, so it is closed
// from the next trade date until a relisting states its own date.
// https://www.cftc.gov/sites/default/files/filings/orgrules/25/03/rules03212518130.pdf
// https://www.cftc.gov/csl/26-21/download
static CLOSED: StaticHoursProfile = StaticHoursProfile {
    tz: America::Chicago,
    regular: &[],
    extended: &[],
    order_entry: &[],
    has_daily_close: true,
    has_weekend_close: true,
};

// Evidence: docs/evidence/small_exchange.md
static REVISIONS: &[Revision] = revisions![
    (
        2020,
        5,
        18,
        &LAUNCH,
        "SMFE 2020-003..005 launch certifications"
    ),
    (
        2024,
        11,
        4,
        &S5C_ERA,
        "SMFE info hub, last 07:00-16:00 capture 2024-11-03, keyed to the Monday; SMFE 2024-010"
    ),
    (2025, 3, 24, &CLOSED, "SMFE 2025-001 delisting"),
];

pub(crate) fn profile_at(as_of: chrono::DateTime<chrono::Utc>) -> &'static StaticHoursProfile {
    select_revision(local_date(as_of, America::Chicago), &CLOSED, REVISIONS)
}
