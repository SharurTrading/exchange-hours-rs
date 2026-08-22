<!-- SPDX-License-Identifier: MIT-0 -->

# Schedule source registry

This registry gives each schedule a stable set of monitoring entry points. A
source-set ID should stay stable even when an operator redesigns its site; fix
the URLs beneath the existing ID instead of renaming it. For a review, open
both the living current/rulebook material and the notice/evidence channel.

This is an index, not the evidence archive. Exact circulars, filings, historical
tables, quotations, and day-level effective dates remain beside the profile in
the linked owner code. A search result, HTTP success, or third-party calendar is
not verification. “Missing/uncited” below means the modeled literal does not yet
have sufficient adjacent primary support; a monitoring page alone does not
upgrade its basis.

## Synthetic profiles

<a id="synthetic-24x7"></a>

### `SYNTHETIC-24X7`

- **Official current/rulebook:** none; this is library policy.
- **Notices/evidence:** none.
- **Status:** synthetic. Do not treat it as evidence for any real venue.

## United States equities and options

<a id="us-nasdaq-equities"></a>

### `US-NASDAQ-EQUITIES`

- **Official current/rulebook:** [Nasdaq systems hours](https://www.nasdaqtrader.com/content/technicalsupport/nasdaq_sys_hours.pdf), [Nasdaq Equity 1](https://listingcenter.nasdaq.com/rulebook/nasdaq/rules/Nasdaq%20Equity%201), [Nasdaq Equity 2](https://listingcenter.nasdaq.com/rulebook/nasdaq/rules/Nasdaq%20Equity%202), [Nasdaq Texas Equity 1](https://listingcenter.nasdaq.com/rulebook/nasdaqtx/rules/Nasdaq%20Texas%20Equity%201), and [PSX legacy rules](https://listingcenter.nasdaq.com/rulebook/phlx/rules/phlx-psx-legacy-3000).
- **Notices/evidence:** [Nasdaq news-alert RSS feeds](https://www.nasdaqtrader.com/trader.aspx?id=newsrss), [ETA2013-21](https://www.nasdaqtrader.com/TraderNews.aspx?id=ETA2013-21) for Nasdaq's 2013 early open, [ETA2010-56](https://www.nasdaqtrader.com/TraderNews.aspx?id=ETA2010-56) and [SR-Phlx-2010-172](https://www.sec.gov/files/rules/sro/phlx/2010/34-63492.pdf) for PSX, BX's [January-2010 08:00–19:00 circular](https://www.nasdaqtrader.com/content/newsalerts/2009/bx_infocirculars/QQQQ_01152009.pdf) and exact [ETA2011-20 implementation alert](https://www.nasdaqtrader.com/TraderNews.aspx?id=ETA2011-20), plus the [2026 BX-to-Nasdaq-Texas conversion alert](https://nasdaqtrader.com/TraderNews.aspx?id=ETA2026-8) and [ETA2026-46](https://www.nasdaqtrader.com/TraderNews.aspx?id=ETA2026-46) for Nasdaq's future Night Session.
- **Status:** Nasdaq, Nasdaq Texas/BX, and PSX current and January-2010-or-launch histories are primary-supported within their stated scopes. The stable `nasdaq_bx` wire name is not renamed with the legal venue.

<a id="us-cboe-equities"></a>

### `US-CBOE-EQUITIES`

- **Official current/rulebook:** [Cboe hours and holidays](https://www.cboe.com/about/hours) and the [US equities rule-book hub](https://www.cboe.com/markets/us/equities/membership).
- **Notices/evidence:** [US equities schedule notices](https://www.cboe.com/markets/us/equities/notices/schedule-update), [BZX's January-2010 baseline filing](https://www.sec.gov/rules/sro/bats/2009/34-59963.pdf), the SEC-filed [BYX launch record](https://www.sec.gov/files/rules/sro/byx/2010/34-63097.pdf) and operator [launch notice](https://cdn.cboe.com/resources/fee_schedule/2010/BATS-Announces-BATS-Y-Exchange-BYX-Pricing-Effective-October-15-2010-and-New-B2B-TRIM-SLIM-and-One-Under-Routing-Strategies.pdf), the [EDGA/EDGX exchange-launch release](https://www.globenewswire.com/news-release/2010/07/21/425534/9381/en/Direct-Edge-Launches-Exchange-Operations.html), the [four distinct 2016 early-open dates](https://cdn.cboe.com/resources/release_notes/2016/Update-Bats-to-Begin-Equity-Order-Matching-and-Routing-at-7-am-ET.pdf), the [BZX/BYX 2018 late-close dates](https://cdn.cboe.com/resources/release_notes/2018/BZX-Exchange-and-BYX-Exchange-to-Extend-Post-Market-Session-Hours-to-8PM-ET.pdf), the [EDGX 2021 early-hours announcement](https://www.cboe.com/insights/posts/cboe-edgx-equities-exchange-to-introduce-early-trading-hours-beginning-march-8/), the [BZX 2025 early-hours announcement](https://www.cboe.com/insights/posts/early-birds-and-night-owls-how-extended-trading-hours-are-reshaping-u-s-equities-markets-), and Cboe's [current EDGX opening-process specification](https://www.cboe.com/document/tech-spec/document/technical-specifications/cboe-titanium-u.s.-equities-opening-process), which announces the 2026-12-07 business-date overnight launch.
- **Status:** all four equity rows have primary-supported January-2010-or-launch baselines and dated phase changes. Their profiles model executable session envelopes, not earlier connectivity or order-acceptance queues. Recheck EDGX's announced future date and regulatory-readiness conditions before release.

<a id="us-nyse-equities"></a>

### `US-NYSE-EQUITIES`

- **Official current/rulebook:** [NYSE hours and calendars](https://www.nyse.com/markets/hours-calendars), [NYSE market rule books](https://www.nyse.com/regulation/rules), and [NYSE Texas](https://www.nyse.com/markets/nyse-texas).
- **Notices/evidence:** [NYSE trader updates](https://www.nyse.com/trader-update/history), the SEC's [pre-2010 Arca three-session evidence](https://www.sec.gov/files/rules/sro/nysearca/2008/34-57505.pdf), [NYSE American's Pillar-hours update](https://www.nyse.com/publicdocs/nyse/markets/nyse-american/Pillar_Update_NYSE_American_March_2017.pdf) and [2017-07-24 launch update](https://www.nyse.com/publicdocs/nyse/markets/nyse-american/Pillar_Update_NYSE_American_Weekend_Test_Update_July21_2017.pdf), the SEC's NSX [2010 late-close filing](https://www.sec.gov/files/rules/sro/nsx/2010/34-62643.pdf), [2014 short-close filing](https://www.sec.gov/files/rules/sro/nsx/2014/34-72215.pdf), [2014 cessation filing](https://www.sec.gov/files/rules/sro/nsx/2014/34-72107.pdf), [2015 resumption approval](https://www.sec.gov/files/rules/sro/nsx/2015/34-76640.pdf), [2015 phased-relaunch record](https://www.sec.gov/Archives/edgar/vprr/1601/16019238.pdf), and [2017 cessation filing](https://www.sec.gov/files/rules/sro/nsx/2017/34-80018.pdf), plus NYSE National's [official filing recording the 2018-05-21 relaunch](https://www.nyse.com/publicdocs/nyse/markets/nyse-national/rule-filings/filings/2020/SR-NYSENat-2020-05.pdf).
- **Status:** check NYSE Tape A, Arca, American, National, and Texas separately; a shared operator does not imply shared phases. The `nyse` row is scoped to Tape A and excludes its queued pre-open plus the separate Tapes B/C early session. Arca is supported at the January-2010 floor, American's pre-/post-Pillar continuous envelopes are dated, and National's legacy grids, 2014 short close, dormant intervals, and relaunches are date-aware. Discrete legacy American off-hours crosses are outside the interval model.

<a id="us-memx"></a>

### `US-MEMX`

- **Official current/rulebook:** [MEMX market hours and holidays](https://info.memxtrading.com/market-hours-and-holiday-schedule/) and the current [MEMX rulebook](https://info.memxtrading.com/regulation/memx-rules/).
- **Notices/evidence:** monitor MEMX [rules and filings](https://info.memxtrading.com/regulation/rules-and-filings/) and the SEC's [current MEMX SRO filings](https://www.sec.gov/rules-regulations/self-regulatory-organization-rulemaking/national-securities-exchanges?sro_organization=192766); retain the official [day-one record](https://memx.com/insights/day-1) for the 2020-09-21 live launch and the [early-open announcement](https://memx.com/insights/pre-market-share-gains-and-new-options-active-risk-feature), which states that 04:00 ET began 2025-05-19.
- **Status:** the equity launch boundary, current schedule, and 2025 cutover are primary-supported. MEMX Options is closed before its sourced 2023-09-27 stock-options launch and then uses the operator-supported ordinary individual-stock-options RTH envelope; exact artifacts remain beside `options.rs`.

<a id="us-miax"></a>

### `US-MIAX`

- **Official current/rulebook:** [MIAX markets](https://www.miaxglobal.com/markets) and [all-options-exchanges rulebooks](https://www.miaxglobal.com/markets/us-options/all-options-exchanges/rulebooks).
- **Notices/evidence:** the official [Pearl Equities market history](https://www.miaxglobal.com/company/markets/us-equities) records the 2020-09-29 live launch; the [expansion alert](https://www.miaxglobal.com/alert/2025/01/17/miax-pearl-equities-changes-expand-trading-hours-adopting-early-and-late) and [RC-2025-02](https://www.miaxglobal.com/sites/default/files/circular-files/MIAX_Pearl_Equities_RC_2025_02_0.pdf) establish the 2025-02-20 early/late change. Continue monitoring the [all-options-exchanges alerts](https://www.miaxglobal.com/markets/us-options/all-options-exchange/alerts).
- **Status:** the Pearl Equities launch boundary, current schedule, and 2025 expansion are primary-supported. MIAX, Pearl, Emerald, and Sapphire Options each own their sourced launch boundary and ordinary individual-stock-options RTH profile; exact launch artifacts remain beside `options.rs`.

<a id="us-iex"></a>

### `US-IEX`

- **Official current/rulebook:** [IEX trading hours and holidays](https://www.iex.io/resources/trading/trading-hours-holidays) and the current [IEX regulation and rule-book hub](https://www.iex.io/resources/regulation).
- **Notices/evidence:** the SEC's [initial exchange-hours correction](https://www.sec.gov/files/rules/sro/iex/2016/34-78447.pdf), [IEX Trading Alert 2016-042](https://iextrading.com/trading/alerts/2016/042/) for the 2016-08-19 first production-symbol launch and phase-in, and [Trading Alert 2016-049](https://iextrading.com/trading/alerts/2016/049/) for the 2016-09-02 all-symbol transition and ATS cessation; monitor the exchange's current [information and regulatory circulars](https://www.iex.io/resources/regulation/circulars).
- **Status:** the `iex` row represents Investors Exchange, not its predecessor ATS. It is closed before the sourced 2016-08-19 exchange launch, notes the symbol phase-in through 2016-09-02, and uses primary-supported 08:00–17:00 System Hours.

<a id="us-intelligentcross"></a>

### `US-INTELLIGENTCROSS`

- **Official current/rulebook:** the SEC's [current live IntelligentCross Form ATS-N](https://www.sec.gov/Archives/edgar/data/1708826/000170882626000008/xslATS-N_X01/primary_doc.xml) and the [Form ATS-N filings index](https://www.sec.gov/about/divisions-offices/division-trading-markets/alternative-trading-systems/form-ats-n-filings-information).
- **Notices/evidence:** the operator's archived [2018-08-18 FAQ](https://web.archive.org/web/20180818115449id_/http://intelligentcross.com/faq) and [first public electronic ATS-N on 2019-07-22](https://www.sec.gov/Archives/edgar/data/1708826/000170882619000008/xslATS-N_X01/primary_doc.xml) contain the same hours table; the operator's [December 2018 release](https://www.imperativex.com/news/imperative-execution-closes-9-million-series-a-financing-round) dates the ATS launch only to September 2018. Monitor later amendments through the SEC [Form ATS-N filings index](https://www.sec.gov/about/divisions-offices/division-trading-markets/alternative-trading-systems/form-ats-n-filings-information).
- **Status:** **partial history.** Current, launch-era, and 2019 filings support 09:00–09:30 order acceptance and 09:30–16:00 matching. No primary source states the exact September-2018 production-launch day, so no inception selector is fabricated.

<a id="us-blue-ocean"></a>

### `US-BLUE-OCEAN`

- **Official current/rulebook:** the SEC's [live Blue Ocean Form ATS-N](https://www.sec.gov/Archives/edgar/data/1795131/000090266426001359/xslATS-N_X01/primary_doc.xml) and the [Form ATS-N filings index](https://www.sec.gov/about/divisions-offices/division-trading-markets/alternative-trading-systems/form-ats-n-filings-information).
- **Notices/evidence:** [operator launch announcement](https://blueocean-tech.io/2021/10/05/announcing-launch-of-blue-ocean-ats-afterhours-trading/), the operator's [timeline](https://blueocean-tech.io/timeline/), which identifies only the month of the earlier beta, and the SEC's [2021](https://www.sec.gov/Archives/edgar/data/1795131/000153949721000764/xslATS-N_X01/primary_doc.xml) and [2023](https://www.sec.gov/Archives/edgar/data/1795131/000153949723000091/xslATS-N_X01/primary_doc.xml) live ATS-N filings that described matching through 04:00.
- **Status:** the row is scoped to production new-order ATS service. Its 2021-10-05 launch and 20:00–04:00 Sunday–Thursday window are primary-supported, and the profile is closed before launch. The live ATS-N's sub-minute resting-book cleanup and earlier beta/testing are explicitly outside scope rather than stretched into an unsupported interval.

<a id="us-finra-trf"></a>

### `US-FINRA-TRF`

- **Official current/rulebook:** [FINRA TRF hub](https://www.finra.org/filing-reporting/trade-reporting-facility-trf), [Rule 6380A](https://www.finra.org/rules-guidance/rulebooks/finra-rules/6380a), and [Rule 6380B](https://www.finra.org/rules-guidance/rulebooks/finra-rules/6380b).
- **Notices/evidence:** FINRA's [Chicago launch notice](https://www.finra.org/filing-reporting/trf/technical-notices/reminder-finranasdaq-trf-chicago), [Regulatory Notice 25-15](https://www.finra.org/rules-guidance/notices/25-15) for the 2026-03-30 change, and [SR-FINRA-2026-015](https://www.finra.org/sites/default/files/2026-07/SR-FINRA-2026-015.pdf), whose future implementation is tied to a SIP date.
- **Status:** these are reporting facilities, not matching venues. Chicago is closed before its sourced 2018-09-10 facility launch; it accepted test securities only through 2018-09-21. The currently published 2026-12-06 effective date is modeled for all three TRFs; move that revision if FINRA or the SIP schedule moves it.

<a id="us-cboe-options"></a>

### `US-CBOE-OPTIONS`

- **Official current/rulebook:** [Cboe US options hours](https://www.cboe.com/about/hours/us-options), the [US options rule-book hub](https://www.cboe.com/markets/us/options/membership), the [options FIX specification](https://www.cboe.com/document/tech-spec/document/technical-specifications/cboe-titanium-u.s.-options-fix-specification), and the [C1 opening-process specification](https://www.cboe.com/document/tech-spec/document/technical-specifications/cboe-titanium-u.s.-options-opening-process).
- **Notices/evidence:** monitor [US options notices](https://www.cboe.com/us/options/notices/); exact pre-floor C1 evidence and the sourced C2, BZX Options, and EDGX Options launch artifacts remain beside `options.rs`.
- **Status:** each row is narrowly scoped to the published 09:30–16:00 regular-session envelope for ordinary individual-stock options. ETF, ETN, index, FLEX, floor-only, and venue-designated extended-hours classes are excluded. C1 has a pre-floor primary baseline; C2, BZX Options, and EDGX Options are closed before their exact sourced launches.

<a id="us-nyse-options"></a>

### `US-NYSE-OPTIONS`

- **Official current/rulebook:** [NYSE hours and calendars](https://www.nyse.com/markets/hours-calendars) and [NYSE options rule books](https://www.nyse.com/regulation/rules).
- **Notices/evidence:** monitor [NYSE trader updates](https://www.nyse.com/trader-update/history); the coordinated 2006 individual-stock-options close filings for Arca/PCX and American/Amex remain beside `options.rs`.
- **Status:** Arca Options and American Options are scoped to ordinary individual-stock options' published 09:30–16:00 regular-session envelope. Their primary baseline predates January 2010; product-specific extended classes are excluded.

<a id="us-nasdaq-options"></a>

### `US-NASDAQ-OPTIONS`

- **Official current/rulebook:** [Nasdaq options trading hours](https://www.nasdaqtrader.com/Trader.aspx?id=optionshours) and [Nasdaq rulebooks](https://listingcenter.nasdaq.com/rulebook).
- **Notices/evidence:** monitor Nasdaq's [news-alert RSS feeds](https://www.nasdaqtrader.com/trader.aspx?id=newsrss); exact PHLX/ISE/NOM baselines and MRX/GEMX/BX Options launch artifacts remain beside `options.rs`.
- **Status:** each row is scoped to ordinary individual-stock options' published 09:30–16:00 regular-session envelope. PHLX, ISE, and NOM have pre-floor baselines; MRX, GEMX, and BX Options are closed before exact sourced launches. Product-specific extended classes are excluded.

<a id="us-box-options"></a>

### `US-BOX-OPTIONS`

- **Official current/rulebook:** [BOX Options](https://boxexchange.com/) and the current [BOX rulebook and filings hub](https://boxexchange.com/regulatory/rulebook-filings/).
- **Notices/evidence:** monitor the consolidated [BOX notices](https://boxexchange.com/notices/); the coordinated 2006 individual-stock-options close filing remains beside `options.rs`.
- **Status:** BOX is scoped to ordinary individual-stock options' published 09:30–16:00 regular-session envelope, with a primary baseline predating January 2010. Product-specific extended classes are excluded.

## Futures, energy, and crypto

<a id="us-cme-group"></a>

### `US-CME-GROUP`

- **Official current/rulebook:** [CME Group trading hours](https://www.cmegroup.com/trading-hours.html), [CME Group rulebooks](https://www.cmegroup.com/market-regulation/rulebook.html), [Globex overview](https://www.cmegroup.com/solutions/market-access/globex.html), the current [CME FX futures grid](https://www.cmegroup.com/trading/why-futures/welcome-to-cme-fx-futures.html), [COMEX Gold specifications](https://www.cmegroup.com/markets/metals/precious/gold.contractSpecs.html), and [NYMEX Light Sweet Crude specifications](https://www.cmegroup.com/markets/energy/crude-oil/light-sweet-crude.contractSpecs.html).
- **Notices/evidence:** CME's [October-2009 E-mini equity-index guide](https://www.cmegroup.com/content/dam/cmegroup/education/modules/files/EQ240_EQ_for_AIT.pdf), [Chadv12-423](https://www.cmegroup.com/tools-information/lookups/advisories/clearing/Chadv12-423.html) and the corresponding [2012 Globex notice](https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20121022.html), plus the [2015 equity-index close-change advisory](https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20150914.html); for CBOT, the [2009 hours announcement](https://www.cmegroup.com/media-room/press-releases/2009/6/05/cme_group_announcesadditionalagricultureethanolelectronictrading.html), [2012 expansion announcement](https://www.cmegroup.com/media-room/press-releases/2012/5/18/cme_group_to_startexpandedcbotgrainandoilseedtradinghoursmay20.html), [2012 market-data notice](https://www.cmegroup.com/tools-information/lookups/advisories/market-data/20120518.html), [SER-6617](https://www.cmegroup.com/rulebook/files/ser_6617_cbot_grain_oilseed_hours_2013_final.pdf), and [SER-7395R](https://www.cmegroup.com/tools-information/lookups/advisories/ser/SER-7395R.html); for energy/metals, the [January-2010 baseline notice](https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20090130.html), [SER-5391](https://www.cmegroup.com/tools-information/lookups/advisories/market-regulation/SER-5391.html), and [2015 close-change notice](https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20150907.html); and for FX, the [2009-02-08 Sunday-open change](https://www.cmegroup.com/tools-information/lookups/advisories/electronic-trading/20081229.html), [2010 product guide](https://www.cmegroup.com/trading/fx/files/FX248-2010_FX_Product_Guide_and_Calendar.pdf), [2018 product report](https://www.cmegroup.com/trading/fx/fx-report/files/q1-2018-cme-fx-products.pdf), [2020 brochure](https://www.cmegroup.com/trading/fx/files/emfx-brochure-q3-2020.pdf), and [current calendar-spread FAQ](https://www.cmegroup.com/articles/faqs/frequently-asked-questions-cme-fx-futures-calendar-spreads.html).
- **Status:** product specifications control. E-mini equity-index, CBOT grain/oilseed, COMEX Gold, NYMEX benchmark-energy, and major CME FX rows have primary-supported January-2010 baselines and all modeled normal-week revisions. Every scope is a named product family rather than a venue-wide clock.

<a id="us-cfe"></a>

### `US-CFE`

- **Official current/rulebook:** [CFE trading hours](https://www.cboe.com/about/hours/us-futures), [VIX futures specifications](https://www.cboe.com/tradable-products/vix/vix-futures/specifications), and the current [CFE regulation/rule-book hub](https://www.cboe.com/us/futures/regulation/).
- **Notices/evidence:** monitor the current [CFE regulatory circulars](https://www.cboe.com/markets/us/futures/regulation/circulars/cfe/regulatory/); retain [SR-CFE-2010-013](https://cdn.cboe.com/resources/regulation/rule_filings/approved/2010/SR-CFE-2010-013.pdf), [SR-CFE-2011-019](https://cdn.cboe.com/resources/regulation/rule_filings/approved/2011/SR-CFE-2011-019.pdf), the [2013 phased-hours announcement](https://ir.cboe.com/news/news-details/2013/CBOE-Futures-Exchange-Announces-Launch-Dates-For-VIX-Futures-Extended-Trading-Hours-09-30-2013/default.aspx), [IC13-041](https://cdn.cboe.com/resources/regulation/circulars/general/CFE-IC-2013-041.pdf) for the exact phase dates and weekday pre-open, the [official 2013 launch retrospective](https://ir.cboe.com/news/news-details/2014/2013-Trading-Volume-Reaches-New-All-Time-High-At-CBOE-Futures-Exchange-01-02-2014/default.aspx), [2014 round-the-clock launch announcement](https://ir.cboe.com/news/news-details/2014/CBOE-Futures-Exchange-Set-For-June-22-Launch-Of-24-Hour-VIX-Futures-Trading-06-09-2014/default.aspx), [SR-CFE-2014-010](https://cdn.cboe.com/resources/regulation/rule_filings/approved/2014/SR-CFE-2014-010.pdf), [IC14-036](https://cdn.cboe.com/resources/regulation/circulars/general/CFE-IC-2014-036.pdf) for the resulting Sunday and weekday pre-opens, [RG-CFE-2014-020](https://cdn.cboe.com/resources/regulation/circulars/regulatory/RG-CFE-2014-020.pdf) for the exact Sunday launch, [SR-CFE-2017-017](https://cdn.cboe.com/resources/regulation/rule_filings/approved/2017/SR-CFE-2017-017.pdf), [RG18-005](https://cdn.cboe.com/resources/regulation/circulars/regulatory/RG-CFE-2018-005.pdf) for the 2018-02-25 migration, and [CFE-2021-028](https://cdn.cboe.com/resources/regulation/rule_filings/pending/2021/21-028-VX-VXM-and-AMERIBOR-Trading-Hours.pdf) for the current queues and trading phases.
- **Status:** Primary. VIX-futures normal-week history is primary-supported from the January-2010 floor, including the 2010 and 2011 morning extensions, the exact 2013 weekday and 2014 Sunday old-system pre-open onsets, and every later modeled regime. The venue default is specifically VIX futures; other CFE contracts require their own check.

<a id="eu-eurex"></a>

### `EU-EUREX`

- **Official current/rulebook:** [Eurex trading hours](https://www.eurex.com/ex-en/trade/trading-hours) and the current [contract-specification Annex C](https://www.eurex.com/resource/blob/2824010/3b94b95cdf5f31cc635294659a5e9786/data/2026_05_04_eurex_d_kontraktspezifikationen_annexe_en.pdf).
- **Notices/evidence:** monitor [Eurex circulars](https://www.eurex.com/ex-en/find/circulars); retain the official [2009](https://www.eurex.com/resource/blob/296888/978b08fe3a240a0b4a8fb62a2647a197/data/cs_history_26102009_en.pdf.pdf), [2013](https://www.eurex.com/resource/blob/337416/2598be26dcb9b5521549169d5e8b9e8e/data/2013_09_25_cs_history_en.pdf.pdf), [2015](https://www.eurex.com/resource/blob/298554/d1fb9a8ac7a259104ea157830860e080/data/2015_10_28_cs_1_history_en.pdf), and [2017](https://www.eurex.com/resource/blob/317412/ff50dcdf5143258c382b4f682cbaf37b/data/2017_08_01_cs_1_history_en.pdf) archived specifications, [Circular 088/2018 and its Annex C](https://www.eurex.com/resource/blob/1412768/e61a2c41d65ad165af7909002223b943/data/er18088e.pdf), and the [Asian-hours launch phase diagram](https://www.eurex.com/resource/blob/1448250/29a4179e4d28742af5d0ee85f9af89f8/data/Eurex%20Asian%20Trading%20Hours_Nov%202018.pdf).
- **Status:** Primary for FESX/FDAX/FDXM benchmark-index futures. The archive pins the January-2010 07:30 pre-trading / 07:50–22:00 continuous grid; the 2018 notice supplies the day-level extension cutover, and the launch diagram distinguishes the first 15 minutes of pre-trading/opening auction from continuous trading.

<a id="eu-eex"></a>

### `EU-EEX`

- **Official current/rulebook:** the current [EEX rules and regulations](https://www.eex.com/en/trading-resources/trading-information/rules-and-regulations), [trading-hours documents](https://www.eex.com/en/trading-resources/trading-information/trading-forms-and-documentation), and [derivatives timetable](https://www.eex.com/fileadmin/EEX/Downloads/Trading/Trading_Hours/20250701_Trading_Hours_on_EEX_Derivatives_Markets_.pdf).
- **Notices/evidence:** monitor the official [EEX newsroom](https://www.eex.com/en/newsroom) and [downloads index](https://www.eex.com/en/downloads); retain the [Nordic Zonal Power Futures customer information](https://www.eex.com/fileadmin/Global/News/EEX/EEX_Customer_Information/2024/20240109_EEX_Customer_Information_Nordic_Zonal_Futures.pdf), which gives the 2024-03-25 launch and 08:00–18:00 CE(S)T grid.
- **Status:** Primary for the specifically named Nordic Zonal Power Futures scope; other EEX products require their own profile.

<a id="ice-derivatives"></a>

### `ICE-DERIVATIVES`

- **Official current/rulebook:** [ICE trading hours](https://www.ice.com/trading-hours), the [ICE product directory](https://www.ice.com/products), [NYSE FANG+ Index Futures](https://www.ice.com/products/66380320/NYSE-FANG-Index-Future), [Brent Crude Futures](https://www.ice.com/products/219/Brent-Crude-Futures), [FTSE 100 Index Futures](https://www.ice.com/products/38716764/FTSE-100-INDEX-), and [Murban Crude Oil Futures](https://www.ice.com/products/75443578/Murban-Crude-Oil-Futures/).
- **Notices/evidence:** monitor [ICE holiday hours](https://www.ice.com/holiday-hours) and exchange-specific circular channels. Retain the [FANG+ launch notice](https://www.ice.com/publicdocs/futures_us/exchange_notices/ICE_Futures_US_FANG%2BFuture_20170926.pdf), [2010 Brent DST circular](https://www.ice.com/publicdocs/circulars/10070.pdf), FTSE circulars [14/146](https://www.ice.com/publicdocs/circulars/14146.pdf), [15/016](https://www.ice.com/publicdocs/circulars/15016.pdf), and [15/169](https://www.ice.com/publicdocs/circulars/15169.pdf), and IFAD's [Murban launch](https://www.ice.com/publicdocs/abu_dhabi/circulars/IFAD%20Circular%20-%2021003%20-%20Trading%20information%20publication.pdf) and [2026 DST](https://www.ice.com/publicdocs/abu_dhabi/circulars/2026.03_-_IFAD_Trading_Hours_Change_Final.pdf) circulars.
- **Status:** ICE has no venue-wide clock. The named FANG+, Brent, FTSE 100, and Murban scopes are primary-supported; every other contract family requires a separate review.

<a id="ice-endex"></a>

### `ICE-ENDEX`

- **Official current/rulebook:** [ICE Endex market resources](https://www.ice.com/endex/market-resources), the [Dutch TTF Natural Gas Futures specification](https://www.ice.com/products/27996665/Dutch-TTF-Gas-Futures), and the current [Operating Time Schedule](https://www.ice.com/publicdocs/endex/ICE_Endex_Operating_Schedule.pdf).
- **Notices/evidence:** monitor [ICE Endex circulars](https://www.ice.com/endex/circulars); retain the predecessor's [2010 Dutch TTF specification](https://www.ice.com/publicdocs/circulars/10010%20attach%201%20-%20TTF%20Nat%20Gas%20Contract%20Spec.pdf), the [2013 transfer circular](https://www.ice.com/publicdocs/circulars/13107.pdf), corroborating [13/134](https://www.ice.com/publicdocs/circulars/13134.pdf), the [March 2014 operating schedule](https://www.ice.com/publicdocs/endex/circulars/ICE-Endex-Derivatives-Rules-V21-2-201403-Appendix-B-1-Operating-Time-Schedule.pdf), the [2015-07-01 Endex rulebook](https://www.ice.com/publicdocs/endex/ICE_Endex_Rules.pdf), and [E26004](https://www.ice.com/publicdocs/endex/circulars/E26004.pdf).
- **Status:** current Dutch TTF phases, the 2013 transfer boundary, continuous 08:00–18:00 trading through the 2014/2015 primary artifacts, and the 2026 extension/DST selector are primary-supported. No primary source dates the historical 07:45 pre-open onset, so the row remains Partial.

<a id="ice-canada-legacy"></a>

### `ICE-CANADA-LEGACY`

- **Official current/rulebook:** ICE's [historical IFUS volume page](https://www.ice.com/historical-volumes-ifus-futures) records that ICE Futures Canada ceased operations, while the current [Canola contract](https://www.ice.com/products/251/Canola) is an ICE Futures U.S. product.
- **Notices/evidence:** retain the [2009 trading calendar](https://www.ice.com/publicdocs/futures_canada/member_notices/Trading_Calendar_2009.pdf), the [2011 pre-open/open change](https://www.ice.com/publicdocs/futures_canada/member_notices/Feb1_2011_revised_trading_hours.pdf), the [2012 close extension](https://www.ice.com/publicdocs/futures_canada/member_notices/June_13_2012_ICE_Futures_Canada_notice-Trading_Hours_and_Settlement_Time_Change.pdf), the [2013 close restoration](https://www.ice.com/publicdocs/futures_canada/member_notices/April_8_2013_Reminder_Closing_time_and_Settlement_time_changes_today.pdf), the [2016 13:20 extension evidence](https://www.ice.com/publicdocs/futures_canada/member_notices/2016_01_18_Reminder_Canola_Trade_At_Settlement.pdf), the corroborating [2017 IFCA holiday schedule](https://www.ice.com/publicdocs/futures_canada/member_notices/2017_11_27_Christmas_2017_and_New_Years_2018_Schedules.pdf), and the [2018 transfer notice](https://www.ice.com/publicdocs/futures_us/exchange_notices/ICE_Futures_US-Notice-Canola-20180501.pdf).
- **Status:** Primary for the legacy Winnipeg Canola identity from the January-2010 audit floor through its final IFCA session. The notices explicitly pin the 2011, 2012, and 2013 opening days or trade-date/session relationship; the 2016 close extension and 2018 IFUS transfer complete the history.

<a id="apac-sgx-derivatives"></a>

### `APAC-SGX-DERIVATIVES`

- **Official current/rulebook:** the [Three-Month SORA Futures product page](https://www.sgx.com/derivatives/products/stir-products?cc=SORA), [Rule 4.1.5](https://rulebook.sgx.com/rulebook/415-trading-hours-opening-and-closing-routines-and-closing-range), and the current [SGX derivatives calendar](https://api2.sgx.com/sites/default/files/2026-01/SGX%20Calendar%202026_2.pdf).
- **Notices/evidence:** monitor the [SGX rulebook latest updates](https://rulebook.sgx.com/latest-updates?page=0) and [derivatives regulatory notices](https://rulebook.sgx.com/rulebook/regulatory-notices); retain the [2024-07-29 launch announcement](https://links.sgx.com/1.0.0/corporate-announcements/LG3YO2RZCGZ92J0B/359e83de092b9d70d54305133c92a82e16f676fc43ef4aa06a6976d8bc771fdf).
- **Status:** Primary for Three-Month SORA Futures, including closed-before-launch history. Other SGX derivatives remain contract-specific.

<a id="crypto-binance"></a>

### `CRYPTO-BINANCE`

- **Official current/rulebook:** the [Binance USDⓈ-M Futures API](https://developers.binance.com/en/docs/products/derivatives-trading-usds-futures/Introduction), [Exchange Information](https://developers.binance.com/docs/derivatives/usds-margined-futures/market-data/rest-api/Exchange-Information), and current operator launch specifications that state 24/7 trading.
- **Notices/evidence:** monitor [Binance support announcements](https://www.binance.com/en/support/announcement); retain the [archived official launch article](https://arquivo.pt/noFrame/replay/20200608065459id_/https://www.binance.com/en/support/articles/360033314152) and Binance's [official announcement-channel link](https://t.me/binance_announcements/799).
- **Status:** Primary for the USDⓈ-M perpetual platform's normal availability: closed before 2019-09-13 04:00 UTC, then 24×7. Ad-hoc maintenance, incidents, and individual contract listing/delisting windows are excluded.

## Asia-Pacific cash equities

<a id="apac-asx"></a>

### `APAC-ASX`

- **Official current/rulebook:** [ASX cash-market hours](https://www.asx.com.au/markets/market-resources/trading-hours-calendar/cash-market-trading-hours) and ASX Operating Rules Procedures Appendix 4013.
- **Notices/evidence:** [Service Release 15 notice](https://www.asxonline.com/public/notices/2025/may/0473.25.05.html) and its [marked procedure amendments](https://www.asxonline.com/content/dam/asxonline/public/notices/2025/april/asx-sr15asx-operating-rule-procedure-amendments.pdf).

<a id="apac-tmx-australia"></a>

### `APAC-TMX-AUSTRALIA`

- **Official current/rulebook:** [TMX Australia hours](https://www.tmxaustralia.com/about/hours) and [Operating Rules and Procedures](https://cdn.cboe.com/resources/au/tmx/participant_resources/Operating_Rules_Procedures_Clean.pdf).
- **Notices/evidence:** no consolidated schedule-notice feed is indexed here; reopen the [current hours page](https://www.tmxaustralia.com/about/hours) and every dated ASIC/Cboe/TMX artifact linked beside the owner profiles.

<a id="apac-nzx"></a>

### `APAC-NZX`

- **Official current/rulebook:** [NZX trading hours](https://www.nzx.com/learning/help-reference/trading-hours) and [anatomy of a trading day](https://www.nzx.com/learning/issuer-participant-resources/nzx-trading/anatomy-of-a-trading-day).
- **Notices/evidence:** [announcement 350919](https://www.nzx.com/announcements/350919) and [announcement 353837](https://www.nzx.com/announcements/353837).

<a id="apac-jpx"></a>

### `APAC-JPX`

- **Official current/rulebook:** [TSE domestic trading hours](https://www.jpx.co.jp/english/equities/trading/domestic/01.html) and [equities trading system](https://www.jpx.co.jp/english/systems/equities-trading/01.html).
- **Notices/evidence:** JPX's [official trading-hours transition table](https://www.jpx.co.jp/english/equities/trading/domestic/tvdivq0000006blj-att/tradinghours_eg.pdf).

<a id="apac-india-cash"></a>

### `APAC-INDIA-CASH`

- **Official current/rulebook:** [NSE closing-auction session](https://www.nseindia.com/static/products-services/closing-auction-session), the [SEBI CAS circular](https://www.sebi.gov.in/legal/circulars/jan-2026/introduction-of-closing-auction-session-cas-in-the-equity-cash-segment-and-certain-modifications-in-the-pre-open-auction-session_99122.html), and BSE's [notices and circulars portal](https://www.bseindia.com/markets/MarketInfo/NoticesCirculars.aspx).
- **Notices/evidence:** search the official [NSE exchange circulars](https://www.nseindia.com/resources/exchange-communication-circulars?PageSpeed=noscript) and [BSE notices and circulars](https://www.bseindia.com/markets/MarketInfo/NoticesCirculars.aspx); retain each exact annual report, release, and day-level notice beside `nse.rs` or `bse.rs`.
- **Status:** keep NSE and BSE historical evidence distinct even where the current CAS envelope is coordinated.

<a id="apac-hkex"></a>

### `APAC-HKEX`

- **Official current/rulebook:** [HKEX securities trading hours](https://www.hkex.com.hk/Services/Trading-hours-and-Severe-Weather-Arrangements/Trading-Hours/Securities-Market?sc_lang=en).
- **Notices/evidence:** monitor [HKEX market communications](https://www.hkex.com.hk/News/Market-Communications?sc_lang=en) and [regulatory announcements](https://www.hkex.com.hk/News/Regulatory-Announcements?sc_lang=en); the exact 2011, 2012, and 2019 releases remain beside the owner timeline.

<a id="apac-sgx-securities"></a>

### `APAC-SGX-SECURITIES`

- **Official current/rulebook:** [SGX-ST Regulatory Notice 8.2.1](https://rulebook.sgx.com/rulebook/regulatory-notice-821-trading-hours-market-phases-application-market-phases-and-principles).
- **Notices/evidence:** no consolidated schedule-only feed is indexed here; reopen [Regulatory Notice 8.2.1](https://rulebook.sgx.com/rulebook/regulatory-notice-821-trading-hours-market-phases-application-market-phases-and-principles) and the exact 2011 rule and 2017/2019 operator announcements beside the owner timeline.

<a id="apac-bursa"></a>

### `APAC-BURSA`

- **Official current/rulebook:** Bursa Malaysia's stable [Securities rules hub](https://www.bursamalaysia.com/regulation/securities/rules_of_bursa_malaysia_securities), [Trading Manual v36](https://www.bursamalaysia.com/sites/5d809dcf39fba22790cad230/assets/65e1a47ccd34aaccd96e5ef0/POs_Trading_Manual_v36_4_March_2024.pdf), and [market-phase/TOP example](https://www.bursamalaysia.com/sites/5bb54be15f36ca0af339077a/assets/5bb57dc75f36ca0c341f041c/Example_on_Theoretical_Opening_Price__TOP_.pdf).
- **Notices/evidence:** no consolidated schedule-only feed is indexed here; start from the [Securities rules hub](https://www.bursamalaysia.com/regulation/securities/rules_of_bursa_malaysia_securities). Retain the pre-audit-floor [Trading Manual v2](https://www.bursamalaysia.com/sites/5bb54be15f36ca0af339077a/assets/5bb55ac75f36ca0c3028d8e7/Amended_Participating_Organisations__Trading_Manual.pdf), dated [v3](https://www.bursamalaysia.com/sites/5bb54be15f36ca0af339077a/assets/5cda944139fba22dab508ab1/rules_bms_cir_rr2_110411.pdf), [v5](https://www.bursamalaysia.com/sites/5bb54be15f36ca0af339077a/assets/5bb55ab65f36ca0c3028d8c2/1._Amendments_to_the_Rules_of_Bursa_Malaysia_Securities_Berhad_in_relation_to_Market_Making_and_Margin_Financing.pdf), and [v29](https://www.bursamalaysia.com/sites/5d809dcf39fba22790cad230/assets/60b1b8e85b711a63ee7f1395/POs_Trading_Manual_v28n_29.pdf) manuals beside the current v36 table.
- **Status:** the 2009 v2 manual was in force at the January-2010 floor, and the dated amendment/manual sequence retains the modeled normal-lot grid through the current table. The row is Primary for the normal-week scope.

<a id="apac-set"></a>

### `APAC-SET`

- **Official current/rulebook:** [SET trading hours](https://www.set.or.th/en/market/information/trading-procedure/trading-hours).
- **Notices/evidence:** [2024 session-change notice](https://www.set.or.th/en/market/news-and-alert/newsdetails?id=86864800&symbol=SET) and SET market alerts.

<a id="apac-idx"></a>

### `APAC-IDX`

- **Official current/rulebook:** [IDX trading hours and mechanism](https://www.idx.id/en/products-services/trading-hours-and-mechanism/).
- **Notices/evidence:** no consolidated schedule-notice feed is indexed here; reopen the [IDX trading-hours page](https://www.idx.id/en/products-services/trading-hours-and-mechanism/), retain the archived operator's [2010 trading-hours table](https://web.archive.org/web/20100831234522id_/http://www.idx.co.id/MainMenu/Trading/JamPerdagangan/tabid/214/lang/en-US/language/en-US/Default.aspx), and check every dated IDX/OJK artifact linked beside the owner timeline.
- **Status:** the archived operator table supplies the January-2010 pre-open and continuous-session baseline; the 2013 expansion, 2020 temporary schedule, and 2023 restoration are primary-supported. The row is Primary.

<a id="apac-pse"></a>

### `APAC-PSE`

- **Official current/rulebook:** [PSE investing/trading overview](https://www.pse.com.ph/investing-at-pse/).
- **Notices/evidence:** use the official [PSE EDGE disclosure portal](https://edge.pse.com.ph/) as the living search entry point; retain the exact 2011–2013, pandemic, and 2024 circulars beside the owner timeline.

<a id="apac-hose"></a>

### `APAC-HOSE`

- **Official current/rulebook:** [HOSE trading-hours table](https://staticfile.hsx.vn/Uploads/UploadDocuments/2372209/2.Trading%20hours.pdf).
- **Notices/evidence:** no consolidated schedule-notice feed is indexed here; reopen the [official trading-hours table](https://staticfile.hsx.vn/Uploads/UploadDocuments/2372209/2.Trading%20hours.pdf), retain the archived operator's [January-2010 phase table](https://web.archive.org/web/20100215053559id_/http://www.hsx.vn:80/hsx/Uploaded/quy_dinh_file/2.Thoi%20gian%20giao%20dich..pdf) and [2010-09-13 change notice](https://web.archive.org/web/20100830155813id_/http://www.hsx.vn/hsx/Modules/News/NewsDetail.aspx?id=48784), and check the exact 2012/2013 annual-report evidence beside the owner timeline.
- **Status:** the January-2010 baseline and every modeled 2010, 2012, and 2013 phase revision are primary-supported with day-level effective dates. The row is Primary.

<a id="apac-china-cash"></a>

### `APAC-CHINA-CASH`

- **Official current/rulebook:** the current [SSE trading rule](https://www.sse.com.cn/lawandrules/sselawsrules2025/stocks/exchange/c/c_20260424_10816482.shtml) and [SZSE trading rule](https://www.szse.cn/lawrules/rule/trade/current/t20260424_620190.html).
- **Notices/evidence:** monitor the current [SSE rule](https://www.sse.com.cn/lawandrules/sselawsrules2025/stocks/exchange/c/c_20260424_10816482.shtml) and [SZSE rule](https://www.szse.cn/lawrules/rule/trade/current/t20260424_620190.html); exact dated releases remain beside their separate owner timelines.
- **Status:** monitor both exchanges independently even when a national rule change is coordinated.

<a id="apac-krx"></a>

### `APAC-KRX`

- **Official current/rulebook:** [KRX cash-equity trading hours](https://global.krx.co.kr/contents/GLB/06/0602/0602020204/GLB0602020204T1.jsp) and [KRX rules](https://law.krx.co.kr/las/LawBon.jsp?lawid=000111).
- **Notices/evidence:** use the official [KRX rules service](https://law.krx.co.kr/las/LawBon.jsp?lawid=000111) as the living entry point; the exact brochure remains beside the 2016/2019 revisions.

<a id="apac-twse"></a>

### `APAC-TWSE`

- **Official current/rulebook:** [TWSE trading system](https://www.twse.com.tw/en/products/system/trading.html).
- **Notices/evidence:** [TWSE company/history timeline](https://www.twse.com.tw/en/about/company/history.html) for the 2020 continuous-trading launch.

## Europe, Americas, Africa, and Middle East cash equities

<a id="eu-fese-secondary"></a>

### `EU-FESE-SECONDARY`

- **Official current/rulebook:** no venue-primary source belongs to this set; the indexed material is the secondary [FESE 2025 trading-hours table](https://www.fese.eu/app/uploads/2024/07/trading-hours-2025-1.pdf).
- **Notices/evidence:** re-download the [FESE trading-hours table](https://www.fese.eu/app/uploads/2024/07/trading-hours-2025-1.pdf) only for corroboration, then verify each literal against its venue's source set.
- **Status:** secondary only. It can cross-check captured European phases but cannot by itself satisfy LAW-PRIMARY-SOURCES or establish an effective date.

<a id="eu-lse"></a>

### `EU-LSE`

- **Official current/rulebook:** [LSE SETS](https://www.londonstockexchange.com/equities-trading/asset-classes/shares-trading/sets), the [equities technical library](https://www.londonstockexchange.com/resources/equities-trading-resources?tab=technical-library), and the operator's [SETS-aligned trading-day timetable](https://docs.londonstockexchange.com/sites/default/files/documents/international-order-book-introduction-sheet.pdf).
- **Notices/evidence:** obtain current MIT201 and Business Parameters documents from the [equities technical library](https://www.londonstockexchange.com/resources/equities-trading-resources?tab=technical-library); retain [notice N15/12](https://docs.londonstockexchange.com/sites/default/files/documents/n1512_attach1.pdf), the operator's MIT201/MIT501 production history for the 2012-04-30 CPX launch, and [notice N01/16](https://docs.londonstockexchange.com/sites/default/files/documents/n0116.pdf) for the 2016-03-21 midday auction.
- **Status:** SETS has a primary January-2010 baseline and exact 2012 CPX and 2016 midday-auction revisions. Randomized uncrosses use the documented conservative latest edge.

<a id="eu-xetra"></a>

### `EU-XETRA`

- **Official current/rulebook:** [Xetra calendar and hours](https://www.cashmarket.deutsche-boerse.com/cash-en/trading/trading-calendar-and-trading-hours), [continuous trading with auctions](https://www.cashmarket.deutsche-boerse.com/cash-en/trading/Xetra/continuous-trading-with-auctions), and [FWB rules](https://www.cashmarket.deutsche-boerse.com/cash-en/Stay-Informed/rules-and-regulations-for-the-fwb).
- **Notices/evidence:** monitor FWB [rules and regulations](https://www.cashmarket.deutsche-boerse.com/cash-en/Stay-Informed/rules-and-regulations-for-the-fwb); retain the operator's [January-2010-era market model](https://cashmarket.deutsche-boerse.com/resource/blob/197910/0890768f3f753299e4c268b80fe7944d/data/207_08e.pdf), [Trade-at-Close launch material](https://www.cashmarket.deutsche-boerse.com/cash-en/Stay-Informed/newsroom/press-releases/Xetra-Trade-at-Close-enables-trading-at-the-official-closing-price-2346762), and [Extended Xetra Retail circular](https://www.cashmarket.deutsche-boerse.com/cash-en/Stay-Informed/circulars-newsletters/deutsche-boerse-circulars/Introduction-of-the-Extended-Xetra-Retail-Service-early-and-late-trading-Planned-changes-to-the-trading-process-valid-from-1-December-2025-4793480).
- **Status:** the DAX-share January-2010 phases, 2020-11-24 Trade-at-Close launch, and participant-restricted 2025-12-01 Extended Retail cutover are primary-supported.

<a id="eu-six"></a>

### `EU-SIX`

- **Official current/rulebook:** [SIX trading hours](https://www.six-group.com/en/products-services/the-swiss-stock-exchange/trading/trading-provisions/trading-hours.html), the current [SIX Trading Guide](https://www.six-group.com/dam/download/the-swiss-stock-exchange/trading/trading-provisions/regulation/trading-guides-upcoming/trading-hours.pdf), and the [SIX download centre](https://www.six-group.com/en/products-services/the-swiss-stock-exchange/trading/download-center.html).
- **Notices/evidence:** archived operator [Directive 1](https://web.archive.org/web/20081123115341id_/http://www.six-swiss-exchange.com/download/trading/regulation/directives/swx_dir01_en.pdf) and the [2009 Equity Market Product Guide](https://web.archive.org/web/20090824132532id_/http://www.six-swiss-exchange.com:80/download/marketpulse/news/newsboard/product_guides/product_guide_equities_en.pdf) establish the January-2010 grid; [SMR8.2 participant readiness](https://www.six-group.com/dam/download/the-swiss-stock-exchange/trading/participation/SWXess-maintenance-releases/smr82_participant_readiness.pdf) makes Trading-At-Last effective 2020-06-22.
- **Status:** the shares profile, including its two-minute randomized opening and closing edges, is primary-supported from January 2010; ETF/ETP timing differs and is outside scope.

<a id="eu-euronext"></a>

### `EU-EURONEXT`

- **Official current/rulebook:** [Euronext trading hours and holidays](https://www.euronext.com/en/trading/trading-hours-holidays), the [regulated-market manuals](https://www.euronext.com/en/regulation/euronext-regulated-markets), and the current [4-01/4-03 timing appendix](https://www.euronext.com/sites/default/files/2026-07/appendix%20to%20Euronext%20Instructions%204-01%204-03%20Trading%20Manuals_0.xlsx).
- **Notices/evidence:** monitor [cash-market notices](https://www.euronext.com/en/products-services/cash-market-notices) and the [regulated-market manual hub](https://www.euronext.com/en/regulation/euronext-regulated-markets); retain the operator's [Optiq migration weekend guide](https://connect.euronext.com/sites/default/files/it-documentation/Go-Live%20Weekend%20Guidelines%20-%20Borsa%20Italiana%20Optiq%20Migration.pdf), which distinguishes the 2023-03-20 legacy-market and 2023-03-27 Milan changes.
- **Status:** current core-share phases and their Central-European clock are primary-supported. Milan has complete sourced history. Paris, Amsterdam, Brussels, and Lisbon remain Partial only because the operator dates the 30-second randomization onset to “late 2015,” not a day. Dublin remains Partial because no operator/regulator-hosted detailed January-2010 phase table or complete amendment chain was recovered.

<a id="eu-bme"></a>

### `EU-BME`

- **Official current/rulebook:** [BME trading hours](https://www.bolsasymercados.es/en/bme-exchange/trading-hours.html) and the [Regulation Explorer](https://www.bolsasymercados.es/bme-exchange/en/Regulation/Regulation-Explorer).
- **Notices/evidence:** the operator's [pre-floor SIBE circular](https://www.bolsasymercados.es/dam/descargas/regulacion/renta-variable/sociedad-de-bolsas/circular/2001/c20011uk.pdf) establishes the baseline; [Circular 1/2023](https://www.bolsasymercados.es/dam/descargas/regulacion/renta-variable/sociedad-de-bolsas/circular/2023/circular-1-23-english.pdf) and [Operating Instruction 47/2023](https://www.bolsasymercados.es/dam/descargas/regulacion/renta-variable/sociedad-de-bolsas/instrucciones-operativas/2023/oi-47-2023-application-of-tal-phase-for-fixing-instruments.pdf) establish the 2023-12-04 Trading-At-Last launch.
- **Status:** principal-share phases, the conservative 30-second auction envelope, and the 2023 Trading-At-Last revision are primary-supported through the complete audit window.

<a id="eu-nasdaq-nordic"></a>

### `EU-NASDAQ-NORDIC`

- **Official current/rulebook:** [Nasdaq European trading hours](https://www.nasdaq.com/european-market-activity/trading-hours), [Nordic member rules](https://www.nasdaq.com/market-regulation/nordic/member-rules), and the current [Nasdaq Nordic Market Model 2026:03](https://www.nasdaq.com/docs/2026/06/17/Nasdaq_Nordic_Market_Model_2026_03_Clean.pdf), section 3.1.
- **Notices/evidence:** retain the operator's [2010 INET market-model launch](https://www.globenewswire.com/news-release/2010/01/25/151379/0/en/INET-Nordic-NASDAQ-OMX-Market-Model.html), [2015 randomized-auction change](https://www.globenewswire.com/news-release/2015/11/16/787323/0/en/IT-INET-REMINDER-Introduction-of-functional-changes-to-INET-auctions-61-15.html), and Copenhagen's [2019 Trading@Closing Price notice](https://view.news.eu.nasdaq.com/view?id=b6276fe1aed34c7412a4d454976025d2d&lang=da); monitor [Nordic member rules](https://www.nasdaq.com/market-regulation/nordic/member-rules) for later changes.
- **Status:** Stockholm, Helsinki, and Copenhagen have primary January-2010 baselines, exact randomized-auction revisions, and current post-trading phases; Copenhagen's closing-price session is also date-aware.

<a id="eu-vienna"></a>

### `EU-VIENNA`

- **Official current/rulebook:** [Vienna trading hours](https://www.wienerborse.at/en/trading/trading-information/trading-hours/), the [current German hours table](https://www.wienerborse.at/handel/handelsinformationen/handelszeiten/), and the [trading-system/model hub](https://www.wienerborse.at/en/trading/trading-information/trading-system/).
- **Notices/evidence:** the archived operator [2009 market model](https://web.archive.org/web/20090219151827id_/http://en.wienerborse.at/static/cms/sites/wbag/media/en/pdf/marketplace_products/feinspez_xetra_marktmodell.pdf) establishes the January-2010 grid; the owner module retains exact 2017 T7 and 2019 close-extension sources, and the [Trade-at-Close launch notice](https://www.wienerborse.at/en/news/vienna-stock-exchange-news/vienna-stock-exchange-extends-trading-hours/) makes TAC effective 2020-12-01.
- **Status:** ATX phases, recurring third-Friday settlement schedule, and 2017/2019/2020 revisions are primary-supported through the complete audit window. Published nominal deterministic auction edges are documented beside the table.

<a id="eu-bist"></a>

### `EU-BIST`

- **Official current/rulebook:** [Borsa İstanbul Equity Market procedure](https://www.borsaistanbul.com/files/equity-market-procedure.pdf).
- **Notices/evidence:** no stable consolidated announcements-feed URL is indexed here; reopen the [Equity Market procedure](https://www.borsaistanbul.com/files/equity-market-procedure.pdf) and every exact 2012–2019 circular or announcement beside `bist.rs`.

<a id="amer-tsx"></a>

### `AMER-TSX`

- **Official current/rulebook:** [TSX trading hours](https://www.tsx.com/en/trading/calendars-and-trading-hours/trading-hours).
- **Notices/evidence:** TMX notices plus the [OSC bulletin](https://www.osc.ca/sites/default/files/pdfs/bulletins/oscb_20050114_2802.pdf) establishing the pre-2010 close-side phases.

<a id="africa-jse"></a>

### `AFRICA-JSE`

- **Official current/rulebook:** the stable [JSE trading and market-data technical library](https://clientportal.jse.co.za/technical-library/trading-and-market-data-documentation), its current Volume 00E v4.09 and session-time workbook, the [Equities Directives](https://www.jse.co.za/media/document/market-regulation/equities-directives), and the [Market Regulation hub](https://www.jse.co.za/regulation/market-regulation).
- **Notices/evidence:** monitor [JSE service hotlines](https://clientportal.jse.co.za/communication/jse-service-hotlines); Release 7.8 became effective 2026-08-17 without changing ZA01 session times, and exact notices for every modeled 2012–2021 change remain beside `jse.rs`.

<a id="mideast-tadawul"></a>

### `MIDEAST-TADAWUL`

- **Official current/rulebook:** [Saudi Exchange trading cycle and times](https://www.saudiexchange.sa/wps/portal/saudiexchange/rules-guidance/capital-market-overview/trading-cycle-and-times?locale=en).
- **Notices/evidence:** no consolidated schedule-notice feed is indexed here; reopen the official [trading cycle and times](https://www.saudiexchange.sa/wps/portal/saudiexchange/rules-guidance/capital-market-overview/trading-cycle-and-times?locale=en) and every report, press-agency release, and pandemic notice beside `tadawul.rs`.

<a id="amer-b3"></a>

### `AMER-B3`

- **Official current/rulebook:** [B3 trading-hours notices](https://www.b3.com.br/pt_br/noticias/horarios-de-negociacao.htm) and the current operator circulars linked beside the profiles.
- **Notices/evidence:** monitor B3's [trading-hours notices](https://www.b3.com.br/pt_br/noticias/horarios-de-negociacao.htm); the January-2010 baseline, every old-grid switch, and recurring New York-reference rule remain beside `b3.rs`.

<a id="amer-bmv"></a>

### `AMER-BMV`

- **Official current/rulebook:** [BMV operating manual](https://www.bmv.com.mx/docs-pub/MARCO_NORMATIVO/CTEN_MNBM/MANUAL_OPERATIVO.PDF) and [Manual v1.95](https://www.bmv.com.mx/docs-pub/MARCO_NORMATIVO/CTEN_MERMO/20260102_V1.95_Act_Tarifas.pdf), which prints the normal and US-daylight-time grids.
- **Notices/evidence:** use the current [BMV operating manual](https://www.bmv.com.mx/docs-pub/MARCO_NORMATIVO/CTEN_MNBM/MANUAL_OPERATIVO.PDF) as the living entry point. Retain the exact [2010 spring interval](https://web.archive.org/web/20130908220405id_/http://www.bmv.com.mx/wb3/wb/BMV/BMV_repositorio/_vtp/BMV/BMV_8aa_sistema_electronico_de_negocios/_rid/223/_mto/3/20100218_DST_Cambio_de_horario.pdf) and the [2010 prospective New York-alignment notice](https://web.archive.org/web/20101122224905id_/http://www.bmv.com.mx:80/wb3/wb/BMV/BMV_repositorio/_vtp/BMV/BMV_8aa_sistema_electronico_de_negocios/_rid/223/_mto/3/20101013_Aviso_Cambio_de_Horario.pdf) beside the recurring selector; later dated notices are corroborating samples of the same rule.
- **Status:** the exact 2010 spring exception and the operator's prospective alignment policy from 2010-11-01 support the full January-2010-on normal-week timeline. The date-aware selector derives each later mismatch interval from the Mexico City/New York offset relation; the row is Primary.
