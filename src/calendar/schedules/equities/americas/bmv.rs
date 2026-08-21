// SPDX-License-Identifier: MIT-0

//! BMV cash-equity grids, including its New York reference-zone regime.

use chrono_tz::America;

use super::super::StaticHoursProfile;
use crate::calendar::SessionRule;
use crate::calendar::rule::MON_FRI;

static BMV_REGULAR_NORMAL: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 8 * 3600 + 30 * 60,
    close_ssm: 15 * 3600,
}];
static BMV_REGULAR_EARLY: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 7 * 3600 + 30 * 60,
    close_ssm: 14 * 3600,
}];
static BMV_EXTENDED_NORMAL: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 8 * 3600,
    close_ssm: 8 * 3600 + 30 * 60,
}];
static BMV_EXTENDED_EARLY: &[SessionRule] = &[SessionRule {
    days: MON_FRI,
    open_ssm: 7 * 3600,
    close_ssm: 7 * 3600 + 30 * 60,
}];

// BMV publishes a normal grid and an hour-earlier grid used to remain aligned
// with New York. Cancellation-only setup and the special HD/ID closing-price
// order stage are excluded from this generic cash-equity order-book profile.
// Manual v1.39 pins the normal grid; v1.90, effective 2023-11-06, prints both
// grids and states the New York alignment rule.
// Sources:
// https://www.bmv.com.mx/docs-pub/MARCO_NORMATIVO/CTEN_MNBM/MANUAL_OPERATIVO.PDF
// https://www.bmv.com.mx/docs-pub/MARCO_NORMATIVO/CTEN_MERMO/20170522_V1%2053_Filtros%20valores%20menores%20a%20un%20peso_%28Esp%29.pdf
// https://www.bmv.com.mx/docs-pub/MARCO_NORMATIVO/CTEN_MERMO/20231106_V1.90_Horario%20DC_Filtros%201E_%20Avisos%20en%20Susp_ReagruapTarif%20Anexo12%28ESP%29.pdf
// https://www.bmv.com.mx/docs-pub/MARCO_NORMATIVO/CTEN_MERMO/20260102_V1.95_Act_Tarifas.pdf
pub(crate) static BMV_PROFILE_NORMAL: StaticHoursProfile = StaticHoursProfile {
    tz: America::Mexico_City,
    regular: BMV_REGULAR_NORMAL,
    extended: BMV_EXTENDED_NORMAL,
    has_daily_close: true,
    has_weekend_close: true,
};
pub(crate) static BMV_PROFILE_EARLY: StaticHoursProfile = StaticHoursProfile {
    tz: America::Mexico_City,
    regular: BMV_REGULAR_EARLY,
    extended: BMV_EXTENDED_EARLY,
    has_daily_close: true,
    has_weekend_close: true,
};

use crate::calendar::schedules::timeline::{
    Revision, effective_date, local_date, reference_delta_seconds, select_revision,
};

pub(crate) const CURRENT: &StaticHoursProfile = &BMV_PROFILE_EARLY;

// BMV's 2010 spring notice gives an exact bounded early grid. Its 2010 fall
// notice gives another exact table and, crucially, states that BMV will align
// its hours with New York whenever the two countries' clock-change dates
// differ in the future. The recurring selector therefore starts with the
// notice's first affected session on 2010-11-01. Later official notices and
// Manual v1.90 corroborate the same one-hour grid and policy.
// https://web.archive.org/web/20130908220405id_/http://www.bmv.com.mx/wb3/wb/BMV/BMV_repositorio/_vtp/BMV/BMV_8aa_sistema_electronico_de_negocios/_rid/223/_mto/3/20100218_DST_Cambio_de_horario.pdf
// https://web.archive.org/web/20101122224905id_/http://www.bmv.com.mx:80/wb3/wb/BMV/BMV_repositorio/_vtp/BMV/BMV_8aa_sistema_electronico_de_negocios/_rid/223/_mto/3/20101013_Aviso_Cambio_de_Horario.pdf
// https://web.archive.org/web/20150510152627id_/http://www.bmv.com.mx/wb3/wb/BMV/BMV_repositorio/_vtp/BMV/BMV_1139_bmv_informa/_rid/223/_mto/3/Aviso_Importante_Horario_Operacion_2014.pdf
// https://web.archive.org/web/20180413023907id_/http://www.bmv.com.mx:80/docs-pub/SALA_PRENSA/CTEN_NOTI/Aviso_Importante_Horario_Operaci%C3%B3n_ING.pdf
// https://web.archive.org/web/20210310164243id_/https://www.bmv.com.mx/docs-pub/SALA_PRENSA/CTEN_NOTI/Aviso_Importante_Horario_Operaci%C3%B3n%20marzo%202021.pdf
static SOURCED_REVISIONS: &[Revision] = &[
    Revision {
        effective: effective_date(2010, 3, 16),
        profile: &BMV_PROFILE_EARLY,
    },
    Revision {
        effective: effective_date(2010, 4, 1),
        profile: &BMV_PROFILE_NORMAL,
    },
];

const REFERENCE_GRID: chrono::NaiveDate = effective_date(2010, 11, 1);

/// Resolves BMV's sourced bounded exceptions and recurring offset regime.
pub(crate) fn profile_at(as_of: chrono::DateTime<chrono::Utc>) -> &'static StaticHoursProfile {
    let day = local_date(as_of, America::Mexico_City);
    if day < REFERENCE_GRID {
        return select_revision(day, &BMV_PROFILE_NORMAL, SOURCED_REVISIONS);
    }
    if reference_delta_seconds(as_of, America::Mexico_City, America::New_York) == 2 * 3600 {
        &BMV_PROFILE_EARLY
    } else {
        &BMV_PROFILE_NORMAL
    }
}
