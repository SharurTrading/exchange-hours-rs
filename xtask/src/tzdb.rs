// SPDX-License-Identifier: MIT-0

//! IANA release discovery for the repository freshness check.

use std::{io, process::Command};

pub(super) fn latest_release() -> io::Result<String> {
    let response = Command::new("curl")
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
            "--retry",
            "2",
            "--retry-max-time",
            "45",
            "--max-filesize",
            "128",
            "--header",
            "Cache-Control: no-cache",
            "https://data.iana.org/time-zones/tzdb/version",
        ])
        .output()?;
    if !response.status.success() {
        return Err(io::Error::other(format!(
            "curl exited with {}: {}",
            response.status,
            String::from_utf8_lossy(&response.stderr).trim()
        )));
    }
    let version = String::from_utf8(response.stdout)
        .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))?;
    let version = version.strip_suffix('\n').unwrap_or(&version);
    // IANA numbers releases a through z, then za through zz, then zza, etc.
    if let [y1, y2, y3, y4, prefix @ .., suffix] = version.as_bytes()
        && [y1, y2, y3, y4].into_iter().all(u8::is_ascii_digit)
        && prefix.iter().all(|&byte| byte == b'z')
        && suffix.is_ascii_lowercase()
    {
        Ok(version.to_owned())
    } else {
        Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("invalid IANA release identifier {version:?}"),
        ))
    }
}
