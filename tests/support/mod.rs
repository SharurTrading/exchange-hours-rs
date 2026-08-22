// SPDX-License-Identifier: MIT-0

//! Public-surface-only fixtures shared by integration-test targets.

use chrono::{DateTime, TimeZone, Utc};
use chrono_tz::Tz;

pub(crate) fn local(tz: Tz, date: (i32, u32, u32), time: (u32, u32, u32)) -> DateTime<Utc> {
    let failure = format!("local fixture is nonexistent or ambiguous in {tz}: {date:?} {time:?}");
    tz.with_ymd_and_hms(date.0, date.1, date.2, time.0, time.1, time.2)
        .single()
        .expect(&failure)
        .with_timezone(&Utc)
}
