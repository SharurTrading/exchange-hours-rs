<!-- SPDX-License-Identifier: MIT-0 -->

# `idx` — evidence

- **Kind:** `Exchange`
- **Owner module:** [`idx.rs`](../../src/calendar/schedules/equities/apac/idx.rs)
- **Source sets:** [`APAC-IDX`](../schedules/sources.md#apac-idx)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

The archived 2010 baseline and sourced 2013 expansion, 2020 temporary shortening, and 2023 restoration are date-aware. The restored venue union retains Negotiated Market availability through 16:30.

## Revision rows

- 2013-01-02 — T1 — IDX 2013 annual report — sessions expand to the 09:00 open, with the regular close at 15:50 and post-trading through 16:15.
- 2020-03-30 — T1 — OJK Siaran Pers Perubahan Jam Perdagangan — temporary pandemic shortening: 09:00–11:30 and 13:30–14:50, with pre-opening 08:45–09:00 and closing phases to 15:15.
- 2023-04-03 — T1 — IDX ikhtisar dan sejarah BEI — the pre-pandemic table is restored, with Negotiated Market availability through 16:30.

## Sources

Retrieval dates: these sources were last opened on the row's reviewed-on date
(2026-08-22, UTC); per-source retrieval dates were not recorded before the
2026-09-12 migration and are added as each source is re-verified.

- <https://www.idx.id/en/products-services/trading-hours-and-mechanism/> — IDX trading hours and mechanism. Under Board decree II-A Kep-00196/BEI/12-2024 the table prints "Pre opening (Input) 08.45.00 – 08.57.59" and "Pre opening (Matching) 08.58.00 – 08.59.59"; the Cash and Negotiated Markets both start at 09.00.00.
- <https://web.archive.org/web/20221220175625/https://www.idx.co.id/media/10022/peraturan_ii_a_perdagangan_efek_bersifat_ekuitas.pdf> — IDX rulebook Kep-00061/BEI/07-2021: "pukul 08.45.00 sampai dengan 08.55.00 digunakan oleh Anggota Bursa Efek untuk memasukkan penawaran jual dan/atau permintaan beli", followed by matching from 08.55.01. Clause IV.4.1.2 keeps the Negotiated Market running continuously to 16:30.
- <https://www.idx.co.id/Media/1208/2013.pdf> — IDX 2013 annual report, dating the 2013 expansion and printing the prior continuous sessions.
- <https://web.archive.org/web/20100831234522id_/http://www.idx.co.id/MainMenu/Trading/JamPerdagangan/tabid/214/lang/en-US/language/en-US/Default.aspx> — archived copy of IDX's own 2010 trading-hours page, printing 09:10–09:25 order input followed by price forming and allocation through 09:29:59.
- <https://ojk.go.id/id/berita-dan-kegiatan/info-terkini/Pages/Siaran-Pers-Perubahan-Jam-Perdagangan-di-Bursa-Efek.aspx> — OJK press release shortening all weekday sessions effective 2020-03-30.
- <https://www.idx.id/id/tentang-bei/ikhtisar-dan-sejarah-bei> — IDX history page, dating the 2023-04-03 restoration.

## Gaps and residual risks

- The archived 2010 trading-hours page was captured on 2010-08-31, after the January-2010 floor. It is treated as supplying the January-2010 baseline because nothing dates a change inside 2010 before the capture; the residual risk is recorded here rather than modelled as a cutover (AGENTS.md, *Carry the earliest sourced state back to the floor*).
- The pre-2013 static profile uses the contiguous nominal 09:10–09:30 envelope rather than the split input/allocation sub-phases the 2010 page prints.
- The pre-opening order-entry boundary is pinned at 08:55, the earliest matching start IDX has ever documented, so no second in which a trade could print is marked order entry under any regime this profile family spans.
- The close-side window stays `extended` in full: Pre-closing 15:50–16:00 is an input phase for the Regular Market, but the Negotiated Market runs continuously to 16:30, so negotiated trades print throughout it.
