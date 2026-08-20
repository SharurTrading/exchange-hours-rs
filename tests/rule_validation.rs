// Copyright (C) 2026 Kevin Monaghan. All rights reserved.
//
// This file is proprietary and confidential.
// Unauthorized copying, use, modification, distribution, or disclosure of this file,
// via any medium, is strictly prohibited except under a written agreement with the
// copyright owner.

//! The [`SessionRule`] domain contract: what [`SessionRule::new`] and
//! [`SessionRule::validate`] accept and reject.
//!
//! The SSM domain is `open_ssm in 0..86_400`, `close_ssm in 0..=86_400` (the
//! `86_400` sentinel closes a same-day session exactly at next local
//! midnight — the shape the 24×7 profile uses), a non-empty interval, and at
//! least one enabled weekday. Struct-literal and serde construction bypass the
//! checks by design; `validate` is the recheck for those paths.

use exchange_hours::{SessionRule, SessionRuleError};

const MON_FRI: [bool; 7] = [true, true, true, true, true, false, false];
const NO_DAYS: [bool; 7] = [false; 7];

#[test]
fn new_accepts_a_same_day_rule() {
    let rule = SessionRule::new(MON_FRI, 9 * 3600 + 30 * 60, 16 * 3600).expect("valid rule");
    assert_eq!(rule.open_ssm, 9 * 3600 + 30 * 60);
    assert_eq!(rule.close_ssm, 16 * 3600);
    assert_eq!(rule.days, MON_FRI);
}

#[test]
fn new_accepts_a_wrap_rule() {
    // 17:00 -> 08:30 next day (Globex shape).
    SessionRule::new(MON_FRI, 17 * 3600, 8 * 3600 + 30 * 60).expect("wrap rules are valid");
}

#[test]
fn new_accepts_the_full_day_sentinel_close() {
    // 0 -> 86_400 is the always-open shape: closes exactly at next midnight.
    SessionRule::new([true; 7], 0, 86_400).expect("the 86_400 close sentinel is valid");
}

#[test]
fn new_rejects_an_out_of_range_open() {
    assert_eq!(
        SessionRule::new(MON_FRI, 86_400, 3600),
        Err(SessionRuleError::OpenOutOfRange { open_ssm: 86_400 })
    );
}

#[test]
fn new_rejects_an_out_of_range_close() {
    assert_eq!(
        SessionRule::new(MON_FRI, 3600, 86_401),
        Err(SessionRuleError::CloseOutOfRange { close_ssm: 86_401 })
    );
}

#[test]
fn new_rejects_an_empty_interval() {
    assert_eq!(
        SessionRule::new(MON_FRI, 3600, 3600),
        Err(SessionRuleError::EmptyInterval { ssm: 3600 })
    );
}

#[test]
fn new_rejects_an_all_disabled_mask() {
    assert_eq!(
        SessionRule::new(NO_DAYS, 3600, 7200),
        Err(SessionRuleError::NoEnabledDays)
    );
}

#[test]
fn validate_rechecks_a_literal_or_deserialized_rule() {
    // Struct literals bypass `new`; `validate` is the recheck.
    let out_of_domain = SessionRule {
        days: MON_FRI,
        open_ssm: 200_000,
        close_ssm: 3600,
    };
    assert_eq!(
        out_of_domain.validate(),
        Err(SessionRuleError::OpenOutOfRange { open_ssm: 200_000 })
    );

    let deserialized: SessionRule = serde_json::from_str(
        r#"{"days":[true,true,true,true,true,false,false],"open_ssm":34200,"close_ssm":57600}"#,
    )
    .expect("valid JSON shape");
    assert_eq!(deserialized.validate(), Ok(()));

    // Serde does not enforce the domain; `validate` is the recheck that must
    // reject an out-of-domain payload.
    let out_of_domain_wire: SessionRule = serde_json::from_str(
        r#"{"days":[true,true,true,true,true,false,false],"open_ssm":86400,"close_ssm":3600}"#,
    )
    .expect("serde accepts the shape regardless of domain");
    assert_eq!(
        out_of_domain_wire.validate(),
        Err(SessionRuleError::OpenOutOfRange { open_ssm: 86_400 })
    );
}

#[test]
fn error_display_names_the_violated_bound() {
    let err = SessionRule::new(MON_FRI, 90_000, 3600).expect_err("out of range");
    let msg = err.to_string();
    assert!(
        msg.contains("90000"),
        "message should carry the value: {msg}"
    );
    assert!(
        msg.contains("86400"),
        "message should carry the bound: {msg}"
    );
}
