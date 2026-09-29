// SPDX-License-Identifier: MIT-0

//! B3 cash-equity holiday rows, 2011-2026.
//!
//! Keyed by the venue-local trade date in `America/Sao_Paulo`. B3 cash
//! equities trade Monday-Friday, so every listed holiday lands on its own
//! trade date; holidays that fell on weekends print no session change and
//! ship no row (07 September, 12 October, 02 November and 15 November in the
//! years they fell on a Saturday or Sunday, and 2022-2024 Aniversário de São
//! Paulo after the operator's own calendars stopped closing the market on São
//! Paulo holidays).
//!
//! The whole block is **T1**: the operator's own yearly calendar articles,
//! market-calendar pages and hours notices, read from the Internet Archive's
//! verbatim replays of the operator's pages where the live site no longer
//! serves them (LAW-PUBLIC-SOURCES). The pre-2025 rows key to the
//! announcement articles (`Não haverá negociação nos mercados da Bolsa …` /
//! `Não haverá negociação nos segmentos de renda variável … nas seguintes
//! datas`), the operator's own market-calendar pages (`Dias em que não haverá
//! negociação na Bolsa …`, with the per-day equities statements the 2019-2024
//! pages print as `Não haverá negociação nos mercados de renda variável …`),
//! and the Ash Wednesday hours notices (`Das 13h … fase de negociação`). From
//! 2015-12-21 the normal week is the recurring New York-referenced selector,
//! so each Ash Wednesday sits on whichever grid is in force and only the open
//! moves: every one of them is a **late open** at 13:00, never an early close
//! or a closure. The 12:45-13:00 order-collection window the notices print is
//! not an executable phase and is recorded as a residual in the evidence
//! file.
//!
//! **2010 is an unaudited span, not a claim.** No operator artifact stating
//! the 2010 holiday arrangement was found at the 2026-09-29 retrieval, so the
//! backfill window starts at 2011-01-01 and dates inside 2010 get no answer
//! rather than an audited-normal verdict. The gap and its closing condition
//! are recorded in
//! [`docs/evidence/b3.md`](../../../../../docs/evidence/b3.md).

use super::EvidenceTier::T1;
use super::HolidayKind::Closed;
use super::fences::late_open;
use super::{HolidayTable, holidays};

/// B3's built-in holiday rows and the windows they were audited over.
///
/// Every pre-2025 row is one line of the operator's yearly calendar
/// announcement, market-calendar page or hours notice named in its citation;
/// the 2025-2026 rows are the operator's 2025 Ofício Circular 149/2024-PRE
/// and 2026 calendar article. A date inside a window with no row is audited
/// normal.
// Evidence: docs/evidence/b3.md
pub(crate) static TABLE: &HolidayTable = holidays! {
    coverage: [
        (2011, 1, 1) ..= (2024, 12, 31),
        (2025, 1, 1) ..= (2026, 12, 31),
    ],
    rows: [
        // 2011-01-25 - T1 - B3-NEWS-CAL-2011 - Aniversário de São Paulo
        // (feriado municipal): the announcement's `Dias sem negociação na
        // BM&FBOVESPA em 2011` list states `Não haverá negociação nos
        // mercados da Bolsa` on the listed dates.
        (2011, 1, 25, Closed, T1, "B3-NEWS-CAL-2011"),
        // 2011-03-07 - T1 - B3-NEWS-CAL-2011 - Carnaval (Monday).
        (2011, 3, 7, Closed, T1, "B3-NEWS-CAL-2011"),
        // 2011-03-08 - T1 - B3-NEWS-CAL-2011 - Carnaval (Tuesday).
        (2011, 3, 8, Closed, T1, "B3-NEWS-CAL-2011"),
        // 2011-03-09 - T1 - B3-NEWS-CAL-2011 - Quarta-Feira de Cinzas: `a
        // negociação nos mercados da BM&FBOVESPA começará às 13h`.
        (2011, 3, 9, late_open(13 * 3_600), T1, "B3-NEWS-CAL-2011"),
        // 2011-04-21 - T1 - B3-NEWS-CAL-2011 - Tiradentes.
        (2011, 4, 21, Closed, T1, "B3-NEWS-CAL-2011"),
        // 2011-04-22 - T1 - B3-NEWS-CAL-2011 - Paixão de Cristo.
        (2011, 4, 22, Closed, T1, "B3-NEWS-CAL-2011"),
        // 2011-06-23 - T1 - B3-NEWS-CAL-2011 - Corpus Christi.
        (2011, 6, 23, Closed, T1, "B3-NEWS-CAL-2011"),
        // 2011-09-07 - T1 - B3-NEWS-CAL-2011 - Independência do Brasil.
        (2011, 9, 7, Closed, T1, "B3-NEWS-CAL-2011"),
        // 2011-10-12 - T1 - B3-NEWS-CAL-2011 - Nossa Senhora Aparecida.
        (2011, 10, 12, Closed, T1, "B3-NEWS-CAL-2011"),
        // 2011-11-02 - T1 - B3-NEWS-CAL-2011 - Finados.
        (2011, 11, 2, Closed, T1, "B3-NEWS-CAL-2011"),
        // 2011-11-15 - T1 - B3-NEWS-CAL-2011 - Proclamação da República.
        (2011, 11, 15, Closed, T1, "B3-NEWS-CAL-2011"),
        // 2011-12-30 - T1 - B3-NEWS-CAL-2011 - Último dia útil do ano
        // (expediente interno nas instituições bancárias): listed under the
        // announcement's own `Dias sem negociação` header.
        (2011, 12, 30, Closed, T1, "B3-NEWS-CAL-2011"),
        // 2012-01-25 - T1 - B3-NEWS-CAL-2012 - Aniversário de São Paulo
        // (feriado municipal).
        (2012, 1, 25, Closed, T1, "B3-NEWS-CAL-2012"),
        // 2012-02-20 - T1 - B3-NEWS-CAL-2012 - Carnaval (Monday).
        (2012, 2, 20, Closed, T1, "B3-NEWS-CAL-2012"),
        // 2012-02-21 - T1 - B3-NEWS-CAL-2012 - Carnaval (Tuesday).
        (2012, 2, 21, Closed, T1, "B3-NEWS-CAL-2012"),
        // 2012-02-22 - T1 - B3-NEWS-CAL-2012 - Quarta-Feira de Cinzas: `a
        // negociação nos mercados da BM&FBOVESPA terá início às 13h`.
        (2012, 2, 22, late_open(13 * 3_600), T1, "B3-NEWS-CAL-2012"),
        // 2012-04-06 - T1 - B3-NEWS-CAL-2012 - Paixão de Cristo.
        (2012, 4, 6, Closed, T1, "B3-NEWS-CAL-2012"),
        // 2012-05-01 - T1 - B3-NEWS-CAL-2012 - Dia do Trabalho.
        (2012, 5, 1, Closed, T1, "B3-NEWS-CAL-2012"),
        // 2012-06-07 - T1 - B3-NEWS-CAL-2012 - Corpus Christi.
        (2012, 6, 7, Closed, T1, "B3-NEWS-CAL-2012"),
        // 2012-07-09 - T1 - B3-NEWS-CAL-2012 - Revolução Constitucionalista
        // (feriado estadual).
        (2012, 7, 9, Closed, T1, "B3-NEWS-CAL-2012"),
        // 2012-09-07 - T1 - B3-NEWS-CAL-2012 - Independência do Brasil.
        (2012, 9, 7, Closed, T1, "B3-NEWS-CAL-2012"),
        // 2012-10-12 - T1 - B3-NEWS-CAL-2012 - Nossa Senhora Aparecida.
        (2012, 10, 12, Closed, T1, "B3-NEWS-CAL-2012"),
        // 2012-11-02 - T1 - B3-NEWS-CAL-2012 - Finados.
        (2012, 11, 2, Closed, T1, "B3-NEWS-CAL-2012"),
        // 2012-11-15 - T1 - B3-NEWS-CAL-2012 - Proclamação da República.
        (2012, 11, 15, Closed, T1, "B3-NEWS-CAL-2012"),
        // 2012-11-20 - T1 - B3-NEWS-CAL-2012 - Consciência Negra.
        (2012, 11, 20, Closed, T1, "B3-NEWS-CAL-2012"),
        // 2012-12-24 - T1 - B3-NEWS-CAL-2012 - Véspera de Natal.
        (2012, 12, 24, Closed, T1, "B3-NEWS-CAL-2012"),
        // 2012-12-25 - T1 - B3-NEWS-CAL-2012 - Natal.
        (2012, 12, 25, Closed, T1, "B3-NEWS-CAL-2012"),
        // 2012-12-31 - T1 - B3-NEWS-CAL-2012 - Véspera de Ano Novo.
        (2012, 12, 31, Closed, T1, "B3-NEWS-CAL-2012"),
        // 2013-01-01 - T1 - B3-REG-CAL-2013 - Confraternização Universal:
        // `Não haverá negociação nos segmentos de renda variável, de renda
        // fixa privada e de derivativos`.
        (2013, 1, 1, Closed, T1, "B3-REG-CAL-2013"),
        // 2013-01-25 - T1 - B3-REG-CAL-2013 - Aniversário de São Paulo
        // (feriado municipal); the page's description states only the
        // non-Bolsa systems run.
        (2013, 1, 25, Closed, T1, "B3-REG-CAL-2013"),
        // 2013-02-11 - T1 - B3-REG-CAL-2013 - Carnaval (Monday).
        (2013, 2, 11, Closed, T1, "B3-REG-CAL-2013"),
        // 2013-02-12 - T1 - B3-REG-CAL-2013 - Carnaval (Tuesday).
        (2013, 2, 12, Closed, T1, "B3-REG-CAL-2013"),
        // 2013-02-13 - T1 - B3-REG-CAL-2013 - Quarta-Feira de Cinzas: `a
        // negociação nos mercados da BM&FBOVESPA terá início às 13:00`.
        (2013, 2, 13, late_open(13 * 3_600), T1, "B3-REG-CAL-2013"),
        // 2013-03-29 - T1 - B3-REG-CAL-2013 - Paixão de Cristo.
        (2013, 3, 29, Closed, T1, "B3-REG-CAL-2013"),
        // 2013-05-01 - T1 - B3-REG-CAL-2013 - Dia do Trabalho.
        (2013, 5, 1, Closed, T1, "B3-REG-CAL-2013"),
        // 2013-05-30 - T1 - B3-REG-CAL-2013 - Corpus Christi.
        (2013, 5, 30, Closed, T1, "B3-REG-CAL-2013"),
        // 2013-07-09 - T1 - B3-REG-CAL-2013 - Revolução Constitucionalista
        // (feriado estadual).
        (2013, 7, 9, Closed, T1, "B3-REG-CAL-2013"),
        // 2013-11-15 - T1 - B3-REG-CAL-2013 - Proclamação da República.
        (2013, 11, 15, Closed, T1, "B3-REG-CAL-2013"),
        // 2013-11-20 - T1 - B3-REG-CAL-2013 - Consciência Negra.
        (2013, 11, 20, Closed, T1, "B3-REG-CAL-2013"),
        // 2013-12-24 - T1 - B3-REG-CAL-2013 - Véspera de Natal.
        (2013, 12, 24, Closed, T1, "B3-REG-CAL-2013"),
        // 2013-12-25 - T1 - B3-REG-CAL-2013 - Natal.
        (2013, 12, 25, Closed, T1, "B3-REG-CAL-2013"),
        // 2013-12-31 - T1 - B3-REG-CAL-2013 - Véspera de Ano Novo.
        (2013, 12, 31, Closed, T1, "B3-REG-CAL-2013"),
        // 2014-01-01 - T1 - B3-REG-CAL-2014-MAR - Confraternização
        // Universal; the page's `Dias em que não haverá negociação na Bolsa
        // em 2014` block and detail table state it.
        (2014, 1, 1, Closed, T1, "B3-REG-CAL-2014-MAR"),
        // 2014-03-03 - T1 - B3-REG-CAL-2014-MAR - Carnaval (Monday).
        (2014, 3, 3, Closed, T1, "B3-REG-CAL-2014-MAR"),
        // 2014-03-04 - T1 - B3-REG-CAL-2014-MAR - Carnaval (Tuesday): the
        // page's summary block prints day icons 03 and 04 beside the two
        // Carnaval labels.
        (2014, 3, 4, Closed, T1, "B3-REG-CAL-2014-MAR"),
        // 2014-03-05 - T1 - B3-NEWS-CINZAS-2014 - Quarta-Feira de Cinzas:
        // `Das 12h45 às 13h – fase de pré-abertura. Das 13h às 17h – fase de
        // negociação` (Segmento BOVESPA), the 2014 short grid's close
        // unchanged.
        (2014, 3, 5, late_open(13 * 3_600), T1, "B3-NEWS-CINZAS-2014"),
        // 2014-04-18 - T1 - B3-REG-CAL-2014-MAR - Paixão de Cristo.
        (2014, 4, 18, Closed, T1, "B3-REG-CAL-2014-MAR"),
        // 2014-04-21 - T1 - B3-REG-CAL-2014-MAR - Tiradentes.
        (2014, 4, 21, Closed, T1, "B3-REG-CAL-2014-MAR"),
        // 2014-05-01 - T1 - B3-REG-CAL-2014-MAR - Dia do Trabalho.
        (2014, 5, 1, Closed, T1, "B3-REG-CAL-2014-MAR"),
        // 2014-06-19 - T1 - B3-REG-CAL-2014-MAR - Corpus Christi.
        (2014, 6, 19, Closed, T1, "B3-REG-CAL-2014-MAR"),
        // 2014-07-09 - T1 - B3-REG-CAL-2014-MAR - Revolução
        // Constitucionalista (feriado estadual).
        (2014, 7, 9, Closed, T1, "B3-REG-CAL-2014-MAR"),
        // 2014-11-20 - T1 - B3-REG-CAL-2014-MAR - Consciência Negra.
        (2014, 11, 20, Closed, T1, "B3-REG-CAL-2014-MAR"),
        // 2014-12-24 - T1 - B3-REG-CAL-2014-MAR - Véspera de Natal.
        (2014, 12, 24, Closed, T1, "B3-REG-CAL-2014-MAR"),
        // 2014-12-25 - T1 - B3-REG-CAL-2014-MAR - Natal.
        (2014, 12, 25, Closed, T1, "B3-REG-CAL-2014-MAR"),
        // 2014-12-31 - T1 - B3-REG-CAL-2014-MAR - Véspera de Ano Novo.
        (2014, 12, 31, Closed, T1, "B3-REG-CAL-2014-MAR"),
        // 2015-01-01 - T1 - B3-NEWS-CAL-2015 - Confraternização Universal:
        // `não haverá negociação nos segmentos de renda variável, de renda
        // fixa e de derivativos nas seguintes datas`.
        (2015, 1, 1, Closed, T1, "B3-NEWS-CAL-2015"),
        // 2015-02-16 - T1 - B3-NEWS-CAL-2015 - Carnaval (Monday).
        (2015, 2, 16, Closed, T1, "B3-NEWS-CAL-2015"),
        // 2015-02-17 - T1 - B3-NEWS-CAL-2015 - Carnaval (Tuesday).
        (2015, 2, 17, Closed, T1, "B3-NEWS-CAL-2015"),
        // 2015-02-18 - T1 - B3-NEWS-CINZAS-2015 - Quarta-Feira de Cinzas:
        // `Das 12h45 às 13h – fase de pré-abertura; Das 13h às 16h55 –
        // sessão contínua de negociação`.
        (2015, 2, 18, late_open(13 * 3_600), T1, "B3-NEWS-CINZAS-2015"),
        // 2015-04-03 - T1 - B3-NEWS-CAL-2015 - Paixão de Cristo.
        (2015, 4, 3, Closed, T1, "B3-NEWS-CAL-2015"),
        // 2015-04-21 - T1 - B3-NEWS-CAL-2015 - Tiradentes.
        (2015, 4, 21, Closed, T1, "B3-NEWS-CAL-2015"),
        // 2015-05-01 - T1 - B3-NEWS-CAL-2015 - Dia do Trabalho.
        (2015, 5, 1, Closed, T1, "B3-NEWS-CAL-2015"),
        // 2015-06-04 - T1 - B3-NEWS-CAL-2015 - Corpus Christi.
        (2015, 6, 4, Closed, T1, "B3-NEWS-CAL-2015"),
        // 2015-07-09 - T1 - B3-NEWS-CAL-2015 - Revolução Constitucionalista
        // (feriado estadual).
        (2015, 7, 9, Closed, T1, "B3-NEWS-CAL-2015"),
        // 2015-09-07 - T1 - B3-NEWS-CAL-2015 - Independência do Brasil.
        (2015, 9, 7, Closed, T1, "B3-NEWS-CAL-2015"),
        // 2015-10-12 - T1 - B3-NEWS-CAL-2015 - Nossa Senhora Aparecida.
        (2015, 10, 12, Closed, T1, "B3-NEWS-CAL-2015"),
        // 2015-11-02 - T1 - B3-NEWS-CAL-2015 - Finados.
        (2015, 11, 2, Closed, T1, "B3-NEWS-CAL-2015"),
        // 2015-11-20 - T1 - B3-NEWS-CAL-2015 - Consciência Negra.
        (2015, 11, 20, Closed, T1, "B3-NEWS-CAL-2015"),
        // 2015-12-24 - T1 - B3-NEWS-CAL-2015 - Véspera de Natal.
        (2015, 12, 24, Closed, T1, "B3-NEWS-CAL-2015"),
        // 2015-12-25 - T1 - B3-NEWS-CAL-2015 - Natal.
        (2015, 12, 25, Closed, T1, "B3-NEWS-CAL-2015"),
        // 2015-12-31 - T1 - B3-NEWS-CAL-2015 - Véspera de Ano-Novo.
        (2015, 12, 31, Closed, T1, "B3-NEWS-CAL-2015"),
        // 2016-01-01 - T1 - B3-PUMA-FER-2016 - Confraternização universal:
        // `Não haverá negociação e liquidação nos mercados da BM&FBOVESPA`.
        (2016, 1, 1, Closed, T1, "B3-PUMA-FER-2016"),
        // 2016-01-25 - T1 - B3-PUMA-FER-2016 - Aniversário de São Paulo
        // (feriado municipal): the page's `Dias em que não haverá negociação
        // na Bolsa` header names the day and its description states only the
        // non-Bolsa systems run (Sisbex, câmbio pronto, registro de balcão).
        (2016, 1, 25, Closed, T1, "B3-PUMA-FER-2016"),
        // 2016-02-08 - T1 - B3-PUMA-FER-2016 - Carnaval (Monday).
        (2016, 2, 8, Closed, T1, "B3-PUMA-FER-2016"),
        // 2016-02-09 - T1 - B3-PUMA-FER-2016 - Carnaval (Tuesday).
        (2016, 2, 9, Closed, T1, "B3-PUMA-FER-2016"),
        // 2016-02-10 - T1 - B3-PUMA-FER-2016 - Quarta-feira de Cinzas: `A
        // negociação e o registro nos mercados da BM&FBOVESPA terão início
        // às 13h`.
        (2016, 2, 10, late_open(13 * 3_600), T1, "B3-PUMA-FER-2016"),
        // 2016-03-25 - T1 - B3-PUMA-FER-2016 - Paixão de Cristo.
        (2016, 3, 25, Closed, T1, "B3-PUMA-FER-2016"),
        // 2016-04-21 - T1 - B3-PUMA-FER-2016 - Tiradentes.
        (2016, 4, 21, Closed, T1, "B3-PUMA-FER-2016"),
        // 2016-05-26 - T1 - B3-PUMA-FER-2016 - Corpus Christi.
        (2016, 5, 26, Closed, T1, "B3-PUMA-FER-2016"),
        // 2016-09-07 - T1 - B3-PUMA-FER-2016 - Independência do Brasil.
        (2016, 9, 7, Closed, T1, "B3-PUMA-FER-2016"),
        // 2016-10-12 - T1 - B3-PUMA-FER-2016 - Nossa Senhora Aparecida.
        (2016, 10, 12, Closed, T1, "B3-PUMA-FER-2016"),
        // 2016-11-02 - T1 - B3-PUMA-FER-2016 - Finados.
        (2016, 11, 2, Closed, T1, "B3-PUMA-FER-2016"),
        // 2016-11-15 - T1 - B3-PUMA-FER-2016 - Proclamação da República.
        (2016, 11, 15, Closed, T1, "B3-PUMA-FER-2016"),
        // 2016-12-30 - T1 - B3-PUMA-FER-2016 - Último dia útil do ano
        // (expediente interno nas instituições bancárias): `Não haverá
        // negociação e liquidação nos mercados da BM&FBOVESPA`.
        (2016, 12, 30, Closed, T1, "B3-PUMA-FER-2016"),
        // 2017-01-25 - T1 - B3-PUMA-FER-2017 - Aniversário de São Paulo
        // (feriado municipal): the page's no-trading header plus a
        // description that states only the non-Bolsa systems run.
        (2017, 1, 25, Closed, T1, "B3-PUMA-FER-2017"),
        // 2017-02-27 - T1 - B3-PUMA-FER-2017 - Carnaval (Monday).
        (2017, 2, 27, Closed, T1, "B3-PUMA-FER-2017"),
        // 2017-02-28 - T1 - B3-PUMA-FER-2017 - Carnaval (Tuesday).
        (2017, 2, 28, Closed, T1, "B3-PUMA-FER-2017"),
        // 2017-03-01 - T1 - B3-PUMA-FER-2017 - Quarta-feira de Cinzas: `A
        // negociação e o registro nos mercados da BM&FBOVESPA terão início
        // às 13h`.
        (2017, 3, 1, late_open(13 * 3_600), T1, "B3-PUMA-FER-2017"),
        // 2017-04-14 - T1 - B3-PUMA-FER-2017 - Paixão de Cristo.
        (2017, 4, 14, Closed, T1, "B3-PUMA-FER-2017"),
        // 2017-04-21 - T1 - B3-PUMA-FER-2017 - Tiradentes.
        (2017, 4, 21, Closed, T1, "B3-PUMA-FER-2017"),
        // 2017-05-01 - T1 - B3-PUMA-FER-2017 - Dia do Trabalho.
        (2017, 5, 1, Closed, T1, "B3-PUMA-FER-2017"),
        // 2017-06-15 - T1 - B3-PUMA-FER-2017 - Corpus Christi.
        (2017, 6, 15, Closed, T1, "B3-PUMA-FER-2017"),
        // 2017-09-07 - T1 - B3-PUMA-FER-2017 - Independência do Brasil.
        (2017, 9, 7, Closed, T1, "B3-PUMA-FER-2017"),
        // 2017-10-12 - T1 - B3-PUMA-FER-2017 - Nossa Senhora Aparecida.
        (2017, 10, 12, Closed, T1, "B3-PUMA-FER-2017"),
        // 2017-11-02 - T1 - B3-PUMA-FER-2017 - Finados.
        (2017, 11, 2, Closed, T1, "B3-PUMA-FER-2017"),
        // 2017-11-15 - T1 - B3-PUMA-FER-2017 - Proclamação da República.
        (2017, 11, 15, Closed, T1, "B3-PUMA-FER-2017"),
        // 2017-11-20 - T1 - B3-PUMA-FER-2017 - Consciência Negra (feriado
        // municipal): the page's no-trading header plus a description that
        // states only the non-Bolsa systems run.
        (2017, 11, 20, Closed, T1, "B3-PUMA-FER-2017"),
        // 2017-12-25 - T1 - B3-PUMA-FER-2017 - Natal: `Não haverá negociação
        // e liquidação nos mercados da BM&FBOVESPA`.
        (2017, 12, 25, Closed, T1, "B3-PUMA-FER-2017"),
        // 2017-12-29 - T1 - B3-PUMA-FER-2017 - Último dia útil do ano
        // (expediente interno nas instituições bancárias): `Não haverá
        // negociação e liquidação nos mercados da BM&FBOVESPA`.
        (2017, 12, 29, Closed, T1, "B3-PUMA-FER-2017"),
        // 2018-01-01 - T1 - B3-PUMA-FER-2018 - Confraternização Universal.
        (2018, 1, 1, Closed, T1, "B3-PUMA-FER-2018"),
        // 2018-01-25 - T1 - B3-PUMA-FER-2018 - Aniversário de São Paulo
        // (feriado municipal): the page's no-trading header plus a
        // description that states only the non-Bolsa systems run.
        (2018, 1, 25, Closed, T1, "B3-PUMA-FER-2018"),
        // 2018-02-12 - T1 - B3-PUMA-FER-2018 - Carnaval (Monday).
        (2018, 2, 12, Closed, T1, "B3-PUMA-FER-2018"),
        // 2018-02-13 - T1 - B3-PUMA-FER-2018 - Carnaval (Tuesday).
        (2018, 2, 13, Closed, T1, "B3-PUMA-FER-2018"),
        // 2018-02-14 - T1 - B3-PUMA-FER-2018 - Quarta-feira de Cinzas: `A
        // negociação e o registro nos mercados da BMF&BOVESPA terão início
        // às 13h`.
        (2018, 2, 14, late_open(13 * 3_600), T1, "B3-PUMA-FER-2018"),
        // 2018-03-30 - T1 - B3-PUMA-FER-2018 - Paixão de Cristo.
        (2018, 3, 30, Closed, T1, "B3-PUMA-FER-2018"),
        // 2018-05-01 - T1 - B3-PUMA-FER-2018 - Dia do Trabalho.
        (2018, 5, 1, Closed, T1, "B3-PUMA-FER-2018"),
        // 2018-05-31 - T1 - B3-PUMA-FER-2018 - Corpus Christi.
        (2018, 5, 31, Closed, T1, "B3-PUMA-FER-2018"),
        // 2018-07-09 - T1 - B3-PUMA-FER-2018 - Revolução Constitucionalista
        // (feriado estadual): the page's no-trading header plus a
        // description that states only the non-Bolsa systems run.
        (2018, 7, 9, Closed, T1, "B3-PUMA-FER-2018"),
        // 2018-09-07 - T1 - B3-PUMA-FER-2018 - Independência do Brasil.
        (2018, 9, 7, Closed, T1, "B3-PUMA-FER-2018"),
        // 2018-10-12 - T1 - B3-PUMA-FER-2018 - Nossa Senhora Aparecida.
        (2018, 10, 12, Closed, T1, "B3-PUMA-FER-2018"),
        // 2018-11-02 - T1 - B3-PUMA-FER-2018 - Finados.
        (2018, 11, 2, Closed, T1, "B3-PUMA-FER-2018"),
        // 2018-11-15 - T1 - B3-PUMA-FER-2018 - Proclamação da República.
        (2018, 11, 15, Closed, T1, "B3-PUMA-FER-2018"),
        // 2018-11-20 - T1 - B3-PUMA-FER-2018 - Consciência Negra (feriado
        // municipal): the page's no-trading header plus a description that
        // states only the non-Bolsa systems run.
        (2018, 11, 20, Closed, T1, "B3-PUMA-FER-2018"),
        // 2018-12-24 - T1 - B3-PUMA-FER-2018 - Véspera de Natal: the page's
        // no-trading header names the day; the description states only the
        // other segments' reduced hours (câmbio pronto, renda fixa to 12h,
        // Cetip UTVM to 13h). The 24 December 2018 capture of the same page
        // (B3-PUMA-FER-2018-DEC) prints the identical row.
        (2018, 12, 24, Closed, T1, "B3-PUMA-FER-2018"),
        // 2018-12-25 - T1 - B3-PUMA-FER-2018 - Natal: `Não haverá negociação
        // e liquidação nos mercados da BM&FBOVESPA`.
        (2018, 12, 25, Closed, T1, "B3-PUMA-FER-2018"),
        // 2018-12-31 - T1 - B3-PUMA-FER-2018 - Expediente interno nas
        // instituições bancárias (sem sessão de negociação na BM&FBOVESPA).
        (2018, 12, 31, Closed, T1, "B3-PUMA-FER-2018"),
        // 2019-01-01 - T1 - B3-PUMA-FER-2019 - Confraternização Universal /
        // New Year's Day: `Não haverá negociação nos mercados de renda
        // variável, renda fixa privada e de derivativos listados …`.
        (2019, 1, 1, Closed, T1, "B3-PUMA-FER-2019"),
        // 2019-01-25 - T1 - B3-PUMA-FER-2019 - Aniversário de São Paulo
        // (feriado municipal), same equities statement.
        (2019, 1, 25, Closed, T1, "B3-PUMA-FER-2019"),
        // 2019-03-04 - T1 - B3-PUMA-FER-2019 - Carnaval (Monday).
        (2019, 3, 4, Closed, T1, "B3-PUMA-FER-2019"),
        // 2019-03-05 - T1 - B3-PUMA-FER-2019 - Carnaval (Tuesday).
        (2019, 3, 5, Closed, T1, "B3-PUMA-FER-2019"),
        // 2019-03-06 - T1 - B3-PUMA-FER-2019 - Quarta-feira de Cinzas: `A
        // negociação e o registro terão início às 13h`.
        (2019, 3, 6, late_open(13 * 3_600), T1, "B3-PUMA-FER-2019"),
        // 2019-04-19 - T1 - B3-PUMA-FER-2019 - Paixão de Cristo.
        (2019, 4, 19, Closed, T1, "B3-PUMA-FER-2019"),
        // 2019-04-21 - T1 - B3-PUMA-FER-2019 - Tiradentes.
        (2019, 4, 21, Closed, T1, "B3-PUMA-FER-2019"),
        // 2019-05-01 - T1 - B3-PUMA-FER-2019 - Dia do Trabalho.
        (2019, 5, 1, Closed, T1, "B3-PUMA-FER-2019"),
        // 2019-06-20 - T1 - B3-PUMA-FER-2019 - Corpus Christi.
        (2019, 6, 20, Closed, T1, "B3-PUMA-FER-2019"),
        // 2019-07-09 - T1 - B3-PUMA-FER-2019 - Revolução Constitucionalista
        // (feriado municipal), same equities statement.
        (2019, 7, 9, Closed, T1, "B3-PUMA-FER-2019"),
        // 2019-09-07 - T1 - B3-PUMA-FER-2019 - Independência do Brasil.
        (2019, 9, 7, Closed, T1, "B3-PUMA-FER-2019"),
        // 2019-10-12 - T1 - B3-PUMA-FER-2019 - Nossa Senhora Aparecida.
        (2019, 10, 12, Closed, T1, "B3-PUMA-FER-2019"),
        // 2019-11-02 - T1 - B3-PUMA-FER-2019 - Finados.
        (2019, 11, 2, Closed, T1, "B3-PUMA-FER-2019"),
        // 2019-11-15 - T1 - B3-PUMA-FER-2019 - Proclamação da República.
        (2019, 11, 15, Closed, T1, "B3-PUMA-FER-2019"),
        // 2019-11-20 - T1 - B3-PUMA-FER-2019 - Consciência Negra (feriado
        // municipal), same equities statement.
        (2019, 11, 20, Closed, T1, "B3-PUMA-FER-2019"),
        // 2019-12-24 - T1 - B3-PUMA-FER-2019 - Véspera de Natal.
        (2019, 12, 24, Closed, T1, "B3-PUMA-FER-2019"),
        // 2019-12-25 - T1 - B3-PUMA-FER-2019 - Natal / Christmas day.
        (2019, 12, 25, Closed, T1, "B3-PUMA-FER-2019"),
        // 2019-12-31 - T1 - B3-PUMA-FER-2019 - Expediente interno nas
        // instituições bancárias (sem sessão de negociação).
        (2019, 12, 31, Closed, T1, "B3-PUMA-FER-2019"),
        // 2020-01-01 - T1 - B3-PUMA-FER-2020 - Confraternização Universal /
        // New Year's Day: `Não haverá negociação nos mercados de renda
        // variável, renda fixa privada e de derivativos listados …`.
        (2020, 1, 1, Closed, T1, "B3-PUMA-FER-2020"),
        // 2020-02-24 - T1 - B3-PUMA-FER-2020 - Carnaval (Monday).
        (2020, 2, 24, Closed, T1, "B3-PUMA-FER-2020"),
        // 2020-02-25 - T1 - B3-PUMA-FER-2020 - Carnaval (Tuesday).
        (2020, 2, 25, Closed, T1, "B3-PUMA-FER-2020"),
        // 2020-02-26 - T1 - B3-PUMA-FER-2020 - Quarta-feira de Cinzas: `A
        // negociação e o registro terão início às 13h`.
        (2020, 2, 26, late_open(13 * 3_600), T1, "B3-PUMA-FER-2020"),
        // 2020-04-10 - T1 - B3-PUMA-FER-2020 - Paixão de Cristo.
        (2020, 4, 10, Closed, T1, "B3-PUMA-FER-2020"),
        // 2020-04-21 - T1 - B3-PUMA-FER-2020 - Tiradentes.
        (2020, 4, 21, Closed, T1, "B3-PUMA-FER-2020"),
        // 2020-05-01 - T1 - B3-PUMA-FER-2020 - Dia do Trabalho.
        (2020, 5, 1, Closed, T1, "B3-PUMA-FER-2020"),
        // 2020-06-11 - T1 - B3-PUMA-FER-2020 - Corpus Christi.
        (2020, 6, 11, Closed, T1, "B3-PUMA-FER-2020"),
        // 2020-07-09 - T1 - B3-PUMA-FER-2020 - Revolução Constitucionalista
        // (feriado municipal), same equities statement.
        (2020, 7, 9, Closed, T1, "B3-PUMA-FER-2020"),
        // 2020-09-07 - T1 - B3-PUMA-FER-2020 - Independência do Brasil
        // (Brasil) e Labor day (feriado norte-americano): the Brazilian leg
        // closes equities.
        (2020, 9, 7, Closed, T1, "B3-PUMA-FER-2020"),
        // 2020-10-12 - T1 - B3-PUMA-FER-2020 - Nossa Senhora Aparecida
        // (Brasil) e Columbus day (feriado norte-americano): the Brazilian
        // leg closes equities.
        (2020, 10, 12, Closed, T1, "B3-PUMA-FER-2020"),
        // 2020-11-02 - T1 - B3-PUMA-FER-2020 - Finados.
        (2020, 11, 2, Closed, T1, "B3-PUMA-FER-2020"),
        // 2020-11-20 - T1 - B3-PUMA-FER-2020 - Consciência Negra (feriado
        // municipal), same equities statement.
        (2020, 11, 20, Closed, T1, "B3-PUMA-FER-2020"),
        // 2020-12-24 - T1 - B3-PUMA-FER-2020 - Véspera de Natal.
        (2020, 12, 24, Closed, T1, "B3-PUMA-FER-2020"),
        // 2020-12-25 - T1 - B3-PUMA-FER-2020 - Natal / Christmas day.
        (2020, 12, 25, Closed, T1, "B3-PUMA-FER-2020"),
        // 2020-12-31 - T1 - B3-PUMA-FER-2020 - Expediente interno nas
        // instituições bancárias (sem sessão de negociação).
        (2020, 12, 31, Closed, T1, "B3-PUMA-FER-2020"),
        // 2021-01-01 - T1 - B3-PUMA-FER-2021-2024 - Confraternização
        // Universal / New Year's Day: `Não haverá negociação nos mercados de
        // renda variável, renda fixa privada, ETF de renda fixa e de
        // derivativos listados …`.
        (2021, 1, 1, Closed, T1, "B3-PUMA-FER-2021-2024"),
        // 2021-01-25 - T1 - B3-PUMA-FER-2021-2024 - Aniversário de São
        // Paulo: same equities statement (the last year the market closed
        // for São Paulo holidays).
        (2021, 1, 25, Closed, T1, "B3-PUMA-FER-2021-2024"),
        // 2021-02-15 - T1 - B3-PUMA-FER-2021-2024 - Carnaval (Monday).
        (2021, 2, 15, Closed, T1, "B3-PUMA-FER-2021-2024"),
        // 2021-02-16 - T1 - B3-PUMA-FER-2021-2024 - Carnaval (Tuesday).
        (2021, 2, 16, Closed, T1, "B3-PUMA-FER-2021-2024"),
        // 2021-02-17 - T1 - B3-PUMA-FER-2021-2024 - Quarta-feira de Cinzas:
        // `A negociação e o registro terão início às 13h`.
        (2021, 2, 17, late_open(13 * 3_600), T1, "B3-PUMA-FER-2021-2024"),
        // 2021-04-02 - T1 - B3-PUMA-FER-2021-2024 - Paixão de Cristo.
        (2021, 4, 2, Closed, T1, "B3-PUMA-FER-2021-2024"),
        // 2021-04-21 - T1 - B3-PUMA-FER-2021-2024 - Tiradentes.
        (2021, 4, 21, Closed, T1, "B3-PUMA-FER-2021-2024"),
        // 2021-06-03 - T1 - B3-PUMA-FER-2021-2024 - Corpus Christi.
        (2021, 6, 3, Closed, T1, "B3-PUMA-FER-2021-2024"),
        // 2021-07-09 - T1 - B3-PUMA-FER-2021-2024 - Revolução
        // Constitucionalista (feriado municipal), same equities statement.
        (2021, 7, 9, Closed, T1, "B3-PUMA-FER-2021-2024"),
        // 2021-09-07 - T1 - B3-PUMA-FER-2021-2024 - Independência do Brasil
        // (Brasil).
        (2021, 9, 7, Closed, T1, "B3-PUMA-FER-2021-2024"),
        // 2021-10-12 - T1 - B3-PUMA-FER-2021-2024 - Nossa Senhora Aparecida
        // (Brasil).
        (2021, 10, 12, Closed, T1, "B3-PUMA-FER-2021-2024"),
        // 2021-11-02 - T1 - B3-PUMA-FER-2021-2024 - Finados.
        (2021, 11, 2, Closed, T1, "B3-PUMA-FER-2021-2024"),
        // 2021-11-15 - T1 - B3-PUMA-FER-2021-2024 - Proclamação da República.
        (2021, 11, 15, Closed, T1, "B3-PUMA-FER-2021-2024"),
        // 2021-12-24 - T1 - B3-PUMA-FER-2021-2024 - Véspera de Natal /
        // Christmas Day (Observed): `Não haverá negociação nos mercados de
        // renda variável …`.
        (2021, 12, 24, Closed, T1, "B3-PUMA-FER-2021-2024"),
        // 2021-12-31 - T1 - B3-PUMA-FER-2021-2024 - Expediente interno nas
        // instituições bancárias (sem sessão de negociação na B3): `Não
        // haverá negociação nos mercados de renda variável …`.
        (2021, 12, 31, Closed, T1, "B3-PUMA-FER-2021-2024"),
        // 2022-01-01 - T1 - B3-PUMA-FER-2021-2024 - Confraternização
        // Universal / New Year's Day, same equities statement.
        (2022, 1, 1, Closed, T1, "B3-PUMA-FER-2021-2024"),
        // 2022-02-28 - T1 - B3-PUMA-FER-2021-2024 - Carnaval (Monday).
        (2022, 2, 28, Closed, T1, "B3-PUMA-FER-2021-2024"),
        // 2022-03-01 - T1 - B3-PUMA-FER-2021-2024 - Carnaval (Tuesday).
        (2022, 3, 1, Closed, T1, "B3-PUMA-FER-2021-2024"),
        // 2022-03-02 - T1 - B3-PUMA-FER-2021-2024 - Quarta-feira de Cinzas:
        // `A negociação e o registro terão início às 13h`.
        (2022, 3, 2, late_open(13 * 3_600), T1, "B3-PUMA-FER-2021-2024"),
        // 2022-04-15 - T1 - B3-PUMA-FER-2021-2024 - Paixão de Cristo.
        (2022, 4, 15, Closed, T1, "B3-PUMA-FER-2021-2024"),
        // 2022-04-21 - T1 - B3-PUMA-FER-2021-2024 - Tiradentes.
        (2022, 4, 21, Closed, T1, "B3-PUMA-FER-2021-2024"),
        // 2022-06-16 - T1 - B3-PUMA-FER-2021-2024 - Corpus Christi.
        (2022, 6, 16, Closed, T1, "B3-PUMA-FER-2021-2024"),
        // 2022-09-07 - T1 - B3-PUMA-FER-2021-2024 - Independência do Brasil.
        (2022, 9, 7, Closed, T1, "B3-PUMA-FER-2021-2024"),
        // 2022-10-12 - T1 - B3-PUMA-FER-2021-2024 - Nossa Senhora Aparecida
        // (Brasil).
        (2022, 10, 12, Closed, T1, "B3-PUMA-FER-2021-2024"),
        // 2022-11-02 - T1 - B3-PUMA-FER-2021-2024 - Finados.
        (2022, 11, 2, Closed, T1, "B3-PUMA-FER-2021-2024"),
        // 2022-11-15 - T1 - B3-PUMA-FER-2021-2024 - Proclamação da República.
        (2022, 11, 15, Closed, T1, "B3-PUMA-FER-2021-2024"),
        // 2022-12-30 - T1 - B3-PUMA-FER-2021-2024 - Expediente interno nas
        // instituições bancárias (sem sessão de negociação): `Não haverá
        // negociação nos mercados de renda variável …`. The 2022-01-25
        // Aniversário de São Paulo ships no row: the same page states
        // `Haverá negociação nos mercados de renda variável …`.
        (2022, 12, 30, Closed, T1, "B3-PUMA-FER-2021-2024"),
        // 2023-02-20 - T1 - B3-PUMA-FER-2021-2024 - Carnaval (Monday).
        (2023, 2, 20, Closed, T1, "B3-PUMA-FER-2021-2024"),
        // 2023-02-21 - T1 - B3-PUMA-FER-2021-2024 - Carnaval (Tuesday).
        (2023, 2, 21, Closed, T1, "B3-PUMA-FER-2021-2024"),
        // 2023-02-22 - T1 - B3-PUMA-FER-2021-2024 - Quarta-feira de Cinzas:
        // `A negociação e o registro terão início às 13h`.
        (2023, 2, 22, late_open(13 * 3_600), T1, "B3-PUMA-FER-2021-2024"),
        // 2023-04-07 - T1 - B3-PUMA-FER-2021-2024 - Paixão de Cristo.
        (2023, 4, 7, Closed, T1, "B3-PUMA-FER-2021-2024"),
        // 2023-04-21 - T1 - B3-PUMA-FER-2021-2024 - Tiradentes.
        (2023, 4, 21, Closed, T1, "B3-PUMA-FER-2021-2024"),
        // 2023-05-01 - T1 - B3-PUMA-FER-2021-2024 - Dia do Trabalho.
        (2023, 5, 1, Closed, T1, "B3-PUMA-FER-2021-2024"),
        // 2023-06-08 - T1 - B3-PUMA-FER-2021-2024 - Corpus Christi.
        (2023, 6, 8, Closed, T1, "B3-PUMA-FER-2021-2024"),
        // 2023-09-07 - T1 - B3-PUMA-FER-2021-2024 - Independência do Brasil.
        (2023, 9, 7, Closed, T1, "B3-PUMA-FER-2021-2024"),
        // 2023-10-12 - T1 - B3-PUMA-FER-2021-2024 - Nossa Senhora Aparecida.
        (2023, 10, 12, Closed, T1, "B3-PUMA-FER-2021-2024"),
        // 2023-11-02 - T1 - B3-PUMA-FER-2021-2024 - Finados.
        (2023, 11, 2, Closed, T1, "B3-PUMA-FER-2021-2024"),
        // 2023-11-15 - T1 - B3-PUMA-FER-2021-2024 - Proclamação da República.
        (2023, 11, 15, Closed, T1, "B3-PUMA-FER-2021-2024"),
        // 2023-12-25 - T1 - B3-PUMA-FER-2021-2024 - Natal/Christmas Day:
        // `Não haverá negociação nos mercados de renda variável …`. The
        // 2023-11-20 Consciência Negra ships no row: the page states
        // `Haverá negociação nos mercados de renda variável …`.
        (2023, 12, 25, Closed, T1, "B3-PUMA-FER-2021-2024"),
        // 2023-12-29 - T1 - B3-PUMA-FER-2021-2024 - Expediente interno nas
        // instituições bancárias (sem sessão de negociação).
        (2023, 12, 29, Closed, T1, "B3-PUMA-FER-2021-2024"),
        // 2024-01-01 - T1 - B3-PUMA-FER-2021-2024 - Confraternização
        // Universal / New Year's Day, same equities statement.
        (2024, 1, 1, Closed, T1, "B3-PUMA-FER-2021-2024"),
        // 2024-02-12 - T1 - B3-PUMA-FER-2021-2024 - Carnaval (Monday).
        (2024, 2, 12, Closed, T1, "B3-PUMA-FER-2021-2024"),
        // 2024-02-13 - T1 - B3-PUMA-FER-2021-2024 - Carnaval (Tuesday).
        (2024, 2, 13, Closed, T1, "B3-PUMA-FER-2021-2024"),
        // 2024-02-14 - T1 - B3-PUMA-FER-2021-2024 - Quarta-feira de Cinzas:
        // `A negociação e o registro terão início às 13h`.
        (2024, 2, 14, late_open(13 * 3_600), T1, "B3-PUMA-FER-2021-2024"),
        // 2024-03-29 - T1 - B3-PUMA-FER-2021-2024 - Paixão de Cristo.
        (2024, 3, 29, Closed, T1, "B3-PUMA-FER-2021-2024"),
        // 2024-05-01 - T1 - B3-PUMA-FER-2021-2024 - Dia do Trabalho.
        (2024, 5, 1, Closed, T1, "B3-PUMA-FER-2021-2024"),
        // 2024-05-30 - T1 - B3-PUMA-FER-2021-2024 - Corpus Christi.
        (2024, 5, 30, Closed, T1, "B3-PUMA-FER-2021-2024"),
        // 2024-11-15 - T1 - B3-PUMA-FER-2021-2024 - Proclamação da República.
        (2024, 11, 15, Closed, T1, "B3-PUMA-FER-2021-2024"),
        // 2024-11-20 - T1 - B3-PUMA-FER-2021-2024 - Consciência Negra: a
        // national holiday from 2024, same equities statement. The
        // 2024-07-09 Dia da Revolução Constitucionalista ships no row: the
        // page states `Haverá negociação nos mercados de renda variável …`.
        (2024, 11, 20, Closed, T1, "B3-PUMA-FER-2021-2024"),
        // 2024-12-24 - T1 - B3-PUMA-FER-2021-2024 - Expediente interno nas
        // instituições bancárias (sem sessão de negociação).
        (2024, 12, 24, Closed, T1, "B3-PUMA-FER-2021-2024"),
        // 2024-12-25 - T1 - B3-PUMA-FER-2021-2024 - Natal.
        (2024, 12, 25, Closed, T1, "B3-PUMA-FER-2021-2024"),
        // 2024-12-31 - T1 - B3-PUMA-FER-2021-2024 - Expediente interno nas
        // instituições bancárias (sem sessão de negociação).
        (2024, 12, 31, Closed, T1, "B3-PUMA-FER-2021-2024"),
        // 2025-01-01 - T1 - B3-OC-149-2024-PRE - Confraternização Universal:
        // Anexo I lists 1 January and §1.1 states no equities trading.
        (2025, 1, 1, Closed, T1, "B3-OC-149-2024-PRE"),
        // 2025-03-03 - T1 - B3-OC-149-2024-PRE - Carnaval (Monday).
        (2025, 3, 3, Closed, T1, "B3-OC-149-2024-PRE"),
        // 2025-03-04 - T1 - B3-OC-149-2024-PRE - Carnaval (Tuesday).
        (2025, 3, 4, Closed, T1, "B3-OC-149-2024-PRE"),
        // 2025-03-05 - T1 - B3-OC-149-2024-PRE - Quarta-feira de Cinzas:
        // Anexo II states continuous equities trading 13:00-17:55 with a
        // 12:45-13:00 pre-opening, so the long grid's close is unchanged and
        // only the open moves.
        (2025, 3, 5, late_open(13 * 3_600), T1, "B3-OC-149-2024-PRE"),
        // 2025-04-18 - T1 - B3-OC-149-2024-PRE - Sexta-feira Santa.
        (2025, 4, 18, Closed, T1, "B3-OC-149-2024-PRE"),
        // 2025-04-21 - T1 - B3-OC-149-2024-PRE - Tiradentes.
        (2025, 4, 21, Closed, T1, "B3-OC-149-2024-PRE"),
        // 2025-05-01 - T1 - B3-OC-149-2024-PRE - Dia do Trabalho.
        (2025, 5, 1, Closed, T1, "B3-OC-149-2024-PRE"),
        // 2025-06-19 - T1 - B3-OC-149-2024-PRE - Corpus Christi.
        (2025, 6, 19, Closed, T1, "B3-OC-149-2024-PRE"),
        // 2025-11-20 - T1 - B3-OC-149-2024-PRE - Dia Nacional de Zumbi e
        // Consciência Negra.
        (2025, 11, 20, Closed, T1, "B3-OC-149-2024-PRE"),
        // 2025-12-24 - T1 - B3-OC-149-2024-PRE - Véspera de Natal: the
        // circular states no trading session; only off-exchange booking runs.
        (2025, 12, 24, Closed, T1, "B3-OC-149-2024-PRE"),
        // 2025-12-25 - T1 - B3-OC-149-2024-PRE - Natal.
        (2025, 12, 25, Closed, T1, "B3-OC-149-2024-PRE"),
        // 2025-12-31 - T1 - B3-OC-149-2024-PRE - Véspera de Ano Novo: the
        // circular states no trading session.
        (2025, 12, 31, Closed, T1, "B3-OC-149-2024-PRE"),
        // 2026-01-01 - T1 - B3-NEWS-CAL-2026 - Confraternização Universal.
        (2026, 1, 1, Closed, T1, "B3-NEWS-CAL-2026"),
        // 2026-02-16 - T1 - B3-NEWS-CAL-2026 - Carnaval (Monday).
        (2026, 2, 16, Closed, T1, "B3-NEWS-CAL-2026"),
        // 2026-02-17 - T1 - B3-NEWS-CAL-2026 - Carnaval (Tuesday).
        (2026, 2, 17, Closed, T1, "B3-NEWS-CAL-2026"),
        // 2026-02-18 - T1 - B3-NEWS-CAL-2026 - Quarta-feira de Cinzas: the
        // article states pre-opening at 12:45 and trading beginning at 13:00,
        // on the long grid, so only the open moves.
        (2026, 2, 18, late_open(13 * 3_600), T1, "B3-NEWS-CAL-2026"),
        // 2026-04-03 - T1 - B3-NEWS-CAL-2026 - Sexta-feira Santa.
        (2026, 4, 3, Closed, T1, "B3-NEWS-CAL-2026"),
        // 2026-04-21 - T1 - B3-NEWS-CAL-2026 - Tiradentes.
        (2026, 4, 21, Closed, T1, "B3-NEWS-CAL-2026"),
        // 2026-05-01 - T1 - B3-NEWS-CAL-2026 - Dia do Trabalho.
        (2026, 5, 1, Closed, T1, "B3-NEWS-CAL-2026"),
        // 2026-06-04 - T1 - B3-NEWS-CAL-2026 - Corpus Christi.
        (2026, 6, 4, Closed, T1, "B3-NEWS-CAL-2026"),
        // 2026-09-07 - T1 - B3-NEWS-CAL-2026 - Independência do Brasil.
        (2026, 9, 7, Closed, T1, "B3-NEWS-CAL-2026"),
        // 2026-10-12 - T1 - B3-NEWS-CAL-2026 - Nossa Senhora Aparecida.
        (2026, 10, 12, Closed, T1, "B3-NEWS-CAL-2026"),
        // 2026-11-02 - T1 - B3-NEWS-CAL-2026 - Finados.
        (2026, 11, 2, Closed, T1, "B3-NEWS-CAL-2026"),
        // 2026-11-20 - T1 - B3-NEWS-CAL-2026 - Dia Nacional de Zumbi e
        // Consciência Negra.
        (2026, 11, 20, Closed, T1, "B3-NEWS-CAL-2026"),
        // 2026-12-24 - T1 - B3-NEWS-CAL-2026 - Véspera de Natal: the article
        // states `Não haverá sessão de negociação`.
        (2026, 12, 24, Closed, T1, "B3-NEWS-CAL-2026"),
        // 2026-12-25 - T1 - B3-NEWS-CAL-2026 - Natal.
        (2026, 12, 25, Closed, T1, "B3-NEWS-CAL-2026"),
        // 2026-12-31 - T1 - B3-NEWS-CAL-2026 - Véspera de Ano Novo: the
        // article states no trading session on B3.
        (2026, 12, 31, Closed, T1, "B3-NEWS-CAL-2026"),
    ],
};
