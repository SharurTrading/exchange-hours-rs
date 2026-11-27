// SPDX-License-Identifier: MIT-0

use std::{
    fs::{self, File, OpenOptions},
    io::{self, Read},
    path::Path,
    process::Command,
};

use exchange_hours::{EMBEDDED_TZDB_VERSION, TimeZoneSnapshot};
use flate2::read::GzDecoder;

use crate::{ReleaseSource, SourceError, compare_releases, validate_release};

const IANA: &str = "https://data.iana.org/time-zones";
const SIGNING_KEY: &[u8] = include_bytes!("../keys/iana-tzdb.gpg");
const SIGNING_FINGERPRINT: &str = "7E3792A9D8ACF7D633BC1588ED97E90E62AA7E34";
const REGION_FILES: [&str; 10] = [
    "africa",
    "antarctica",
    "asia",
    "australasia",
    "europe",
    "northamerica",
    "southamerica",
    "etcetera",
    "factory",
    "backward",
];

/// The official IANA stable release source, using the bundled signing key.
///
/// See [`crate::TimeZoneUpdater::start_latest`] for the default polling setup.
/// To choose another interval, pass `IanaSource` to
/// [`crate::TimeZoneUpdater::start`].
#[derive(Clone, Copy, Debug, Default)]
pub struct IanaSource;

impl IanaSource {
    /// Read the stable release identifier currently published by IANA.
    ///
    /// This discovery check uses HTTPS and `curl`. It does not download the
    /// release archive or require `gpgv` or `zic`.
    ///
    /// # Errors
    ///
    /// Returns an error if the publication cannot be fetched or its identifier
    /// does not follow the stable IANA format.
    pub fn latest_release() -> Result<String, SourceError> {
        let staging = tempfile::tempdir()?;
        let path = staging.path().join("latest");
        download(&format!("{IANA}/tzdb/version"), &path, 128, 2)?;
        let mut version = fs::read_to_string(path)?;
        if version.ends_with('\n') {
            version.pop();
        }
        validate_release(&version)?;
        Ok(version)
    }
}

impl ReleaseSource for IanaSource {
    fn check(
        &mut self,
        active_release: Option<&str>,
    ) -> Result<Option<TimeZoneSnapshot>, SourceError> {
        let version = Self::latest_release()?;
        compare_releases(&version, active_release.unwrap_or(EMBEDDED_TZDB_VERSION))?;
        if active_release == Some(version.as_str()) {
            return Ok(None);
        }

        let temporary = tempfile::tempdir()?;
        let staging = fs::canonicalize(temporary.path())?;
        let archive = staging.join("release.tar.gz");
        let signature = staging.join("release.tar.gz.asc");
        let release_url = format!("{IANA}/releases/tzdata{version}.tar.gz");
        download(&release_url, &archive, 2 * 1024 * 1024, 0)?;
        download(&format!("{release_url}.asc"), &signature, 8 * 1024, 0)?;
        verify_signature(&staging, &archive, &signature)?;
        unpack_sources(&archive, &staging)?;
        let packaged_version = fs::read_to_string(staging.join("version"))?;
        if packaged_version
            .strip_suffix('\n')
            .unwrap_or(&packaged_version)
            != version
        {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("signed archive version {packaged_version:?} differs from IANA release {version}"),
            ).into());
        }
        let zoneinfo = staging.join("zoneinfo");
        fs::create_dir(&zoneinfo)?;
        // IANA's default build includes backward links (including US/Central)
        // and uses these regional source files, without backzone or leap data.
        // Fat output also carries transitions explicitly for older zic versions.
        run(
            "zic",
            Command::new("zic")
                .args(["-b", "fat", "-d"])
                .arg(&zoneinfo)
                .args(REGION_FILES.map(|name| staging.join(name))),
        )?;
        let snapshot = TimeZoneSnapshot::from_tzif(version, |name| fs::read(zoneinfo.join(name)))?;
        Ok(Some(snapshot))
    }
}

fn download(url: &str, destination: &Path, limit: u64, retries: u8) -> io::Result<()> {
    run(
        "curl",
        Command::new("curl")
            .args([
                "-q",
                "--fail",
                "--silent",
                "--show-error",
                "--proto",
                "=https",
                "--tlsv1.2",
                "--connect-timeout",
                "10",
                "--max-time",
                "30",
                "--header",
                "Cache-Control: no-cache",
                "--max-filesize",
            ])
            .arg(limit.to_string())
            .arg("--retry")
            .arg(retries.to_string())
            .args(["--retry-max-time", "45"])
            .arg("--output")
            .arg(destination)
            .arg(url),
    )?;
    if fs::metadata(destination)?.len() > limit {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("download exceeded the size limit for {url}"),
        ));
    }
    Ok(())
}

fn verify_signature(staging: &Path, archive: &Path, signature: &Path) -> io::Result<()> {
    let keyring = staging.join("iana-tzdb.gpg");
    fs::write(&keyring, SIGNING_KEY)?;
    let report = run(
        "gpgv",
        Command::new("gpgv")
            .args(["--status-fd", "1", "--homedir"])
            .arg(staging)
            .arg("--keyring")
            .arg(keyring)
            .arg(signature)
            .arg(archive),
    )?;
    let report = String::from_utf8_lossy(&report);
    let mut valid = false;
    for line in report.lines() {
        let mut fields = line.split_ascii_whitespace();
        if fields.next() != Some("[GNUPG:]") {
            continue;
        }
        match fields.next() {
            Some("VALIDSIG") => valid |= fields.next() == Some(SIGNING_FINGERPRINT),
            Some("EXPKEYSIG" | "EXPSIG" | "REVKEYSIG") => {
                return Err(io::Error::new(io::ErrorKind::InvalidData, line));
            }
            _ => {}
        }
    }
    if valid {
        return Ok(());
    }
    Err(io::Error::new(
        io::ErrorKind::InvalidData,
        "archive lacks a valid signature by the pinned IANA release key",
    ))
}

fn unpack_sources(archive: &Path, staging: &Path) -> io::Result<()> {
    // Write only required root-level regular files, with exclusive creation.
    // Archive paths, links, ownership and modes are never passed to the filesystem.
    let contents = GzDecoder::new(File::open(archive)?).take(32 * 1024 * 1024);
    let mut archive = tar::Archive::new(contents);
    for entry in archive.entries()? {
        let mut entry = entry?;
        let name = REGION_FILES
            .iter()
            .copied()
            .chain(["version"])
            .find(|name| entry.path_bytes().as_ref() == name.as_bytes());
        let Some(name) = name else { continue };
        if !entry.header().entry_type().is_file() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("archive source {name} must be a regular file"),
            ));
        }
        let mut output = OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(staging.join(name))?;
        io::copy(&mut entry, &mut output)?;
    }
    Ok(())
}

fn run(tool: &str, command: &mut Command) -> io::Result<Vec<u8>> {
    let output = command
        .output()
        .map_err(|error| io::Error::new(error.kind(), format!("starting {tool}: {error}")))?;
    if output.status.success() {
        return Ok(output.stdout);
    }
    Err(io::Error::other(format!(
        "{tool} exited with {}: {}",
        output.status,
        String::from_utf8_lossy(&output.stderr).trim(),
    )))
}
