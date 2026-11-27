// SPDX-License-Identifier: MIT-0

//! Exercise the CI command with a controlled response from the HTTPS client.

#![cfg(unix)]

use std::{fs, os::unix::fs::PermissionsExt, process::Command};

#[test]
fn freshness_requires_an_exact_match_and_successful_discovery() {
    let bin = tempfile::tempdir().unwrap();
    let curl = bin.path().join("curl");
    fs::write(
        &curl,
        r#"#!/bin/sh
test "$IANA_FIXTURE" != unavailable || exit 22
printf '%s\n' "$IANA_FIXTURE"
"#,
    )
    .unwrap();
    fs::set_permissions(curl, fs::Permissions::from_mode(0o755)).unwrap();
    let current = exchange_hours::EMBEDDED_TZDB_VERSION;
    for (response, success, message) in [
        (current, true, current),
        ("9999z", false, "latest stable release is 9999z"),
        ("9999za", false, "latest stable release is 9999za"),
        ("9999aa", false, "Could not verify"),
        ("preview", false, "Could not verify"),
        ("unavailable", false, "Could not verify"),
    ] {
        let output = Command::new(env!("CARGO_BIN_EXE_xtask"))
            .arg("verify-embedded-tzdb")
            .env("PATH", bin.path())
            .env("IANA_FIXTURE", response)
            .output()
            .unwrap();
        let report = if success {
            output.stdout
        } else {
            output.stderr
        };
        let report = String::from_utf8(report).unwrap();
        assert_eq!(output.status.success(), success, "{report}");
        assert!(report.contains(message), "{report}");
    }
}
