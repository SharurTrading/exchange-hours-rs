// SPDX-License-Identifier: MIT-0

//! Pins the IANA data used to compile the built-in time zones.

fn main() {
    assert_eq!(
        jiff_tzdb::VERSION,
        "2026f",
        "review the bundled IANA data before updating it"
    );
}
