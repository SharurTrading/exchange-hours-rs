<!-- SPDX-License-Identifier: MIT-0 -->

# exchange-hours-tzdb

**Use the latest stable IANA release for live trading.** This companion checks
at startup and every hour while the updater is alive. Pin the returned snapshot
at the start of each live operation to keep every calendar on one release.

**Use the precompiled `exchange-hours` calendar for backtesting.** Pin the
calendar crate version and record `exchange_hours::EMBEDDED_TZDB_VERSION` with
the results. To replay a particular live run that used different rules, use
the same crate version and load that run's archived time zone data.

```rust
use exchange_hours::{Exchange, ExchangeCalendar};
use exchange_hours_tzdb::TimeZoneUpdater;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let updater = TimeZoneUpdater::start_latest()?;
    let snapshot = updater.snapshot()?;
    let nyse = ExchangeCalendar::new(Exchange::Nyse).with_time_zone_snapshot(&snapshot);
    let at = "2026-01-06T15:00:00Z".parse()?;
    let _bounds = nyse.session_bounds(at)?;
    Ok(())
}
```

Read `updater.status()` for the active release, the last check and last
successful check, and any error from the last check. A failed startup returns
an error. A failed background refresh leaves the previous verified release
active and the updater retries at the next interval. Each process begins with
a fresh check. Use `TimeZoneUpdater::start(IanaSource, interval)` to select a
different interval.

## Deployment

The updater requires HTTPS access to `data.iana.org` and trusted `curl` (8.4
or later), `gpgv`, and `zic` on `PATH`. The runtime path is intended for macOS
and Linux. Provide a writable temporary directory for release staging. All
calendar queries use the in-memory snapshot obtained from the updater.

The updater reads the version published by IANA, downloads the matching
versioned data archive and signature, verifies the signature and the
archive's release identifier, and compiles the IANA regional sources and
legacy aliases to TZif. It activates the release after all zones used by
`exchange-hours` can be read and parsed. See the [IANA release format].

The live source uses IANA's standard data. Jiff's precompiled data uses
IANA's rearguard format. For Dublin both return GMT in winter, IST in summer
and the same UTC session boundaries. Code inspecting Jiff's daylight-saving
flag directly sees opposite flags because the standard data represents Irish
winter time with a negative daylight-saving adjustment. See
[Jiff's data generation notes](https://docs.rs/jiff-tzdb/latest/jiff_tzdb/#data-generation).

HTTPS attempts have a 10-second connect timeout and a 30-second total timeout.
Release discovery retries transient fetch failures up to twice and starts
those retries within a 45-second window. A final attempt can finish after
that window. Invalid release data or signatures fail the rest of the check.

The bundled verification key is Paul Eggert's IANA release key with
fingerprint `7E3792A9D8ACF7D633BC1588ED97E90E62AA7E34`. A release signed only
by a different key fails verification. The key is available from [OpenPGP].

[IANA release format]: https://www.iana.org/time-zones/tz-link
[OpenPGP]: https://keys.openpgp.org/vks/v1/by-fingerprint/7E3792A9D8ACF7D633BC1588ED97E90E62AA7E34
