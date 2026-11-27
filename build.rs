// SPDX-License-Identifier: MIT-0

//! Pins the IANA data used to compile the built-in time zones.

fn main() {
    println!(
        "cargo::rustc-env=EXCHANGE_HOURS_EMBEDDED_TZDB={}",
        jiff_tzdb::VERSION
    );
    assert_eq!(
        jiff_tzdb::VERSION,
        "2026f",
        "review the bundled IANA data before updating it"
    );
}
