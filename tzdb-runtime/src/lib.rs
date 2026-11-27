// SPDX-License-Identifier: MIT-0

//! Automatically load signed IANA time zone releases for `exchange-hours`.
//!
//! The updater owns the I/O and polling. Obtain a snapshot once at the start
//! of each calendar run and apply it to the calendars used in that run.

mod iana;

use std::{
    error::Error,
    fmt, io,
    sync::{Arc, RwLock, mpsc},
    thread,
    time::{Duration, SystemTime},
};

use exchange_hours::TimeZoneSnapshot;

pub use iana::IanaSource;

/// The interval between checks in [`TimeZoneUpdater::start_latest`].
pub const DEFAULT_CHECK_INTERVAL: Duration = Duration::from_hours(1);

/// A release source's failure, preserved in [`UpdateError::Source`].
pub type SourceError = Box<dyn Error + Send + Sync>;

/// A source of complete time zone snapshots, with its own verification policy.
///
/// [`IanaSource`] is the signed IANA implementation. This trait also supports
/// deployment-specific release mirrors with their own verified transport.
pub trait ReleaseSource: Send + 'static {
    /// Check for a stable release newer than `active_release`.
    ///
    /// Return `Ok(None)` when the source confirms the active release is
    /// current. At startup `active_release` is `None` and a snapshot is
    /// required. A snapshot's release must be an IANA release identifier.
    ///
    /// # Errors
    ///
    /// Return an error when the release check or snapshot validation fails.
    fn check(
        &mut self,
        active_release: Option<&str>,
    ) -> Result<Option<TimeZoneSnapshot>, SourceError>;
}

/// Failure to start, refresh or inspect the updater.
#[derive(Debug)]
#[non_exhaustive]
pub enum UpdateError {
    /// A zero interval would start a continuous polling loop.
    ZeroInterval,
    /// The configured source failed its check or validation.
    Source(SourceError),
    /// The configured source returned no snapshot on startup.
    NoInitialRelease,
    /// The source returned an identifier outside the stable IANA format.
    InvalidRelease(String),
    /// The source reported a release older than the active release.
    ReleaseRegressed {
        /// The snapshot still in use.
        active: String,
        /// The version returned by the source.
        candidate: String,
    },
    /// The updater could not start its worker thread.
    WorkerSpawn(io::Error),
    /// The updater's synchronized state could not be accessed.
    StateUnavailable,
}

impl fmt::Display for UpdateError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ZeroInterval => formatter.write_str("the time zone check interval is zero"),
            Self::Source(error) => write!(formatter, "time zone release check failed: {error}"),
            Self::NoInitialRelease => {
                formatter.write_str("no initial time zone release was provided")
            }
            Self::InvalidRelease(release) => write!(formatter, "invalid IANA release {release:?}"),
            Self::ReleaseRegressed { active, candidate } => {
                write!(
                    formatter,
                    "IANA release regressed from {active} to {candidate}"
                )
            }
            Self::WorkerSpawn(error) => write!(formatter, "starting time zone worker: {error}"),
            Self::StateUnavailable => formatter.write_str("time zone updater state is unavailable"),
        }
    }
}

impl Error for UpdateError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Source(error) => Some(error.as_ref()),
            Self::WorkerSpawn(error) => Some(error),
            _ => None,
        }
    }
}

/// Observable status from the most recently completed check.
#[derive(Clone, Debug)]
pub struct UpdateStatus {
    /// Release returned by [`TimeZoneUpdater::snapshot`] at this observation.
    pub active_release: String,
    /// When the latest check finished, regardless of outcome.
    pub last_checked_at: SystemTime,
    /// When the last successful check finished, including an unchanged release.
    pub last_success_at: SystemTime,
    /// Why the latest check failed, if it did. The active release stays usable.
    pub last_error: Option<Arc<UpdateError>>,
}

#[derive(Debug)]
struct ActiveRelease {
    snapshot: Arc<TimeZoneSnapshot>,
    last_checked_at: SystemTime,
    last_success_at: SystemTime,
    last_error: Option<Arc<UpdateError>>,
}

/// Starts with a verified release and checks for new releases in the background.
///
/// Dropping the updater signals the worker to stop. An in-progress check can
/// finish, and snapshots previously obtained from it remain valid.
#[derive(Debug)]
pub struct TimeZoneUpdater {
    state: Arc<RwLock<ActiveRelease>>,
    _stop: mpsc::Sender<()>,
}

impl TimeZoneUpdater {
    /// Verify the latest IANA release now, then check for updates each hour.
    ///
    /// Requires `curl`, `gpgv` and `zic` on `PATH`, and HTTPS access to
    /// `data.iana.org`. The release verification key is bundled.
    ///
    /// # Errors
    ///
    /// Returns an error if no verified snapshot can be obtained or polling
    /// cannot start.
    pub fn start_latest() -> Result<Self, UpdateError> {
        Self::start(IanaSource, DEFAULT_CHECK_INTERVAL)
    }

    /// Verify the initial snapshot synchronously, then poll the given source.
    ///
    /// The worker waits `interval` after one check completes before starting
    /// the next. The source owns any network and tool deadlines.
    ///
    /// # Errors
    ///
    /// Returns an error if the interval is zero, the first check cannot supply
    /// a valid snapshot, or polling cannot start.
    pub fn start(mut source: impl ReleaseSource, interval: Duration) -> Result<Self, UpdateError> {
        if interval.is_zero() {
            return Err(UpdateError::ZeroInterval);
        }
        let snapshot = check(&mut source, None)?.ok_or(UpdateError::NoInitialRelease)?;
        let checked_at = check_completed_at();
        let state = Arc::new(RwLock::new(ActiveRelease {
            snapshot: Arc::new(snapshot),
            last_checked_at: checked_at,
            last_success_at: checked_at,
            last_error: None,
        }));
        let (stop, stopped) = mpsc::channel();
        let worker_state = Arc::clone(&state);
        thread::Builder::new()
            .name("exchange-hours-tzdb".to_owned())
            .spawn(move || poll(&worker_state, &mut source, interval, &stopped))
            .map_err(UpdateError::WorkerSpawn)?;
        Ok(Self { state, _stop: stop })
    }

    /// Pin the currently active snapshot for one run, background job or replay.
    ///
    /// # Errors
    ///
    /// Returns an error if the updater's shared state is unavailable.
    pub fn snapshot(&self) -> Result<Arc<TimeZoneSnapshot>, UpdateError> {
        let state = self
            .state
            .read()
            .map_err(|_| UpdateError::StateUnavailable)?;
        Ok(Arc::clone(&state.snapshot))
    }

    /// Inspect the active release and outcome of the last completed check.
    ///
    /// # Errors
    ///
    /// Returns an error if the updater's shared state is unavailable.
    pub fn status(&self) -> Result<UpdateStatus, UpdateError> {
        let state = self
            .state
            .read()
            .map_err(|_| UpdateError::StateUnavailable)?;
        Ok(UpdateStatus {
            active_release: state.snapshot.release().to_owned(),
            last_checked_at: state.last_checked_at,
            last_success_at: state.last_success_at,
            last_error: state.last_error.clone(),
        })
    }
}

fn poll(
    state: &RwLock<ActiveRelease>,
    source: &mut impl ReleaseSource,
    interval: Duration,
    stopped: &mpsc::Receiver<()>,
) {
    while stopped.recv_timeout(interval) == Err(mpsc::RecvTimeoutError::Timeout) {
        let active = match state.read() {
            Ok(state) => Arc::clone(&state.snapshot),
            Err(_) => break,
        };
        let checked = check(source, Some(active.release()));
        let checked_at = check_completed_at();
        let Ok(mut state) = state.write() else { break };
        state.last_checked_at = checked_at;
        match checked {
            Ok(snapshot) => {
                if let Some(snapshot) = snapshot {
                    state.snapshot = Arc::new(snapshot);
                }
                state.last_success_at = checked_at;
                state.last_error = None;
            }
            Err(error) => state.last_error = Some(Arc::new(error)),
        }
    }
}

#[expect(
    clippy::disallowed_methods,
    reason = "the I/O companion timestamps refresh status outside the calendar engine"
)]
fn check_completed_at() -> SystemTime {
    SystemTime::now()
}

fn check(
    source: &mut impl ReleaseSource,
    active_release: Option<&str>,
) -> Result<Option<TimeZoneSnapshot>, UpdateError> {
    let Some(snapshot) = source.check(active_release).map_err(UpdateError::Source)? else {
        return Ok(None);
    };
    let candidate = snapshot.release();
    validate_release(candidate)?;
    if let Some(active) = active_release {
        compare_releases(candidate, active)?;
        if candidate == active {
            return Ok(None);
        }
    }
    Ok(Some(snapshot))
}

fn compare_releases(candidate: &str, active: &str) -> Result<(), UpdateError> {
    if candidate < active {
        return Err(UpdateError::ReleaseRegressed {
            active: active.to_owned(),
            candidate: candidate.to_owned(),
        });
    }
    Ok(())
}

// IANA versions are a four-digit year, then a through z, za through zz, etc.
// These names sort in release order. See https://www.iana.org/time-zones/tz-link.
fn validate_release(release: &str) -> Result<(), UpdateError> {
    let [y1, y2, y3, y4, prefix @ .., last] = release.as_bytes() else {
        return Err(UpdateError::InvalidRelease(release.to_owned()));
    };
    if [y1, y2, y3, y4].into_iter().all(u8::is_ascii_digit)
        && prefix.iter().all(|byte| *byte == b'z')
        && last.is_ascii_lowercase()
    {
        return Ok(());
    }
    Err(UpdateError::InvalidRelease(release.to_owned()))
}
