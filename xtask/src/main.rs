// SPDX-License-Identifier: MIT-0

//! The `cargo xtask` entry point: it wires the process's arguments and
//! standard streams to [`xtask::run`] and maps the outcome to an exit code —
//! 0 for a verification, non-zero for every failure.

use std::path::PathBuf;
use std::process::ExitCode;

fn main() -> ExitCode {
    match xtask::run(
        std::env::args_os().skip(1),
        &workspace_root(),
        std::env::var_os("EXCHANGE_HOURS_RESEARCH"),
    ) {
        xtask::Outcome::Success(report) => {
            println!("{report}");
            ExitCode::SUCCESS
        }
        xtask::Outcome::Failure(report) => {
            eprintln!("{report}");
            ExitCode::FAILURE
        }
    }
}

/// The workspace root: this binary's manifest lives at `<workspace>/xtask`,
/// so the root is its parent directory. `docs/evidence` and the store
/// siblings resolve from there.
fn workspace_root() -> PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap_or(std::path::Path::new("."))
        .to_path_buf()
}
