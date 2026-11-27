// SPDX-License-Identifier: MIT-0

//! Integration tests for signed release discovery and worker snapshot activation.

use std::{io, sync::mpsc, time::Duration};

use exchange_hours::{Exchange, ExchangeCalendar, TimeZoneSnapshot, TimeZoneSnapshotError};
use exchange_hours_tzdb::{ReleaseSource, SourceError, TimeZoneUpdater, UpdateError};

const DEADLINE: Duration = Duration::from_secs(5);

struct ScriptedSource {
    started: mpsc::Sender<Option<String>>,
    results: mpsc::Receiver<Result<Option<TimeZoneSnapshot>, SourceError>>,
}

impl ReleaseSource for ScriptedSource {
    fn check(
        &mut self,
        active_release: Option<&str>,
    ) -> Result<Option<TimeZoneSnapshot>, SourceError> {
        self.started.send(active_release.map(str::to_owned))?;
        self.results.recv_timeout(DEADLINE)?
    }
}

fn snapshot(release: &str) -> Result<TimeZoneSnapshot, TimeZoneSnapshotError<io::Error>> {
    TimeZoneSnapshot::from_tzif(release, |name| {
        jiff_tzdb::get(name)
            .map(|(_, bytes)| bytes)
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, name))
    })
}

#[test]
fn automatic_refresh_retains_complete_snapshot_across_failures() {
    let (started, checks) = mpsc::channel();
    let (steps, results) = mpsc::channel();
    steps.send(Ok(Some(snapshot("2090z").unwrap()))).unwrap();
    let updater = TimeZoneUpdater::start(
        ScriptedSource { started, results },
        Duration::from_millis(5),
    )
    .unwrap();
    assert_eq!(checks.recv_timeout(DEADLINE).unwrap(), None);
    assert_eq!(
        checks.recv_timeout(DEADLINE).unwrap().as_deref(),
        Some("2090z")
    );
    let initial = updater.status().unwrap();
    let pinned = updater.snapshot().unwrap();
    let refusal = ExchangeCalendar::new(Exchange::Nyse)
        .with_time_zone_snapshot(&pinned)
        .is_open("2029-03-07T15:00Z".parse().unwrap())
        .unwrap_err();
    assert!(initial.last_error.is_none());

    let invalid = TimeZoneSnapshot::from_tzif("2090za", |_| Ok::<_, io::Error>(b"invalid"));
    steps.send(Err(Box::new(invalid.unwrap_err()))).unwrap();
    assert_eq!(
        checks.recv_timeout(DEADLINE).unwrap().as_deref(),
        Some("2090z")
    );
    assert_eq!(pinned, updater.snapshot().unwrap());
    let rejected = updater.status().unwrap();
    assert!(matches!(
        rejected.last_error.as_deref(),
        Some(UpdateError::Source(_))
    ));
    assert_eq!(rejected.last_success_at, initial.last_success_at);

    steps.send(Ok(Some(snapshot("2090za").unwrap()))).unwrap();
    assert_eq!(
        checks.recv_timeout(DEADLINE).unwrap().as_deref(),
        Some("2090za")
    );
    assert_eq!(updater.snapshot().unwrap().release(), "2090za");
    assert_eq!(pinned.release(), "2090z");
    drop(pinned);
    assert_eq!(refusal.tzdb_release(), "2090z");
    assert!(refusal.normal_week_baseline().is_some());
    assert!(updater.status().unwrap().last_error.is_none());

    steps.send(Ok(Some(snapshot("2090z").unwrap()))).unwrap();
    assert_eq!(
        checks.recv_timeout(DEADLINE).unwrap().as_deref(),
        Some("2090za")
    );
    assert!(matches!(
        updater.status().unwrap().last_error.as_deref(),
        Some(UpdateError::ReleaseRegressed { active, candidate })
            if active == "2090za" && candidate == "2090z"
    ));
    assert_eq!(updater.snapshot().unwrap().release(), "2090za");
    steps.send(Ok(None)).unwrap();
    assert_eq!(
        checks.recv_timeout(DEADLINE).unwrap().as_deref(),
        Some("2090za")
    );
    assert!(updater.status().unwrap().last_error.is_none());
}

#[test]
fn startup_does_not_fall_back_after_failure() {
    let (started, checks) = mpsc::channel();
    let (steps, results) = mpsc::channel();
    steps
        .send(Err(io::Error::other("source unavailable").into()))
        .unwrap();
    let result = TimeZoneUpdater::start(ScriptedSource { started, results }, DEADLINE);
    assert_eq!(checks.recv_timeout(DEADLINE).unwrap(), None);
    assert!(matches!(result, Err(UpdateError::Source(_))));
}

#[test]
#[ignore = "requires network access plus curl, gpgv and zic on PATH"]
fn opens_current_signed_iana_release() {
    let updater = TimeZoneUpdater::start_latest().unwrap();
    let snapshot = updater.snapshot().unwrap();
    assert_eq!(updater.status().unwrap().active_release, snapshot.release());
    let date = "2026-01-06T15:00:00Z".parse().unwrap();
    let calendar = ExchangeCalendar::new(Exchange::Nyse);
    assert_eq!(
        calendar
            .with_time_zone_snapshot(&snapshot)
            .session_bounds(date),
        calendar.session_bounds(date),
    );
}
