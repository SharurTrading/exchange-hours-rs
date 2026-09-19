<!-- SPDX-License-Identifier: MIT-0 -->

# `coinbase_derivatives` — evidence

- **Kind:** `Exchange`
- **Owner module:** [`coinbase_derivatives.rs`](../../src/calendar/schedules/futures/us/coinbase_derivatives.rs)
- **Source sets:** [`US-COINBASE-DERIVATIVES`](../schedules/sources.md#us-coinbase-derivatives)
- **Ledger row:** [verification.md](../schedules/verification.md)

## Ledger basis (moved from docs/schedules/verification.md on 2026-09-12 UTC)

**Gap: order-entry** — the trading session is sourced; what is undated is a queue or post-close phase in which no trade can print. Closed before FairX's exact 2021-06-28 08:00 CT launch, which the operator's homepage banner states; the first session opens at the launch, with no Sunday-evening session before it. The 23x5 grid, Sunday–Friday 17:00–16:00 CT with a daily 16:00–17:00 break, is stated by the four 2021-06-04 launch certifications and restated by every later dated filing through #2026-24, so no revision separates the launch from the 2026-09-11 review date. The Pre-Open phase is documented from 2021 without a time and its 16:50 CT start is first witnessed in a 2025 capture, so it enters at a knowledge-bound 2026-09-11 review row (verified current, onset undated); the dated profiles before it carry no Pre-Open. Since #2026-24 (on or after trade date 2026-05-04) most CDE futures trade 24x7; this default is the grid that copper, platinum, nano crude oil, natural gas and Mag7 + Crypto equity index futures retain. The 24x7 family, with its weekly Friday, quarterly weekend and ad-hoc maintenance windows, is out of scope; no product-family key is claimed.

## Revision rows

- 2026-09-11 — T1 — 2026-09-11 review: verified current, onset undated — the knowledge-bound row that adds the 16:50–17:00 CT Pre-Open queue to the sourced 23x5 grid.

The row's own tier is the tier of the artifact it rests on: the operator's own
market-hours documentation page, read in a 2025 capture. The row makes no onset
claim and never moves forward; a Coinbase Derivatives artifact that states the
Pre-Open on a day-level effective date replaces it.

Two further boundaries are in `profile_at` rather than in the `revisions!`
block, because neither is a venue-local-midnight revision and neither is a
revision row. The launch is an exact instant, 2021-06-28 13:00:00 UTC
(08:00 CDT), stated at T1 by FairX's own homepage banner; everything before it
is a sourced closure. The one-off launch-day profile then hands over to the
recurring 23x5 grid on 2021-06-29, the first full day, and the Monday-evening
session that runs into the next day is identical in both, so the handover never
splits a running session.

## Dated selectors

Day-level boundaries this identity's `profile_at` selects on directly, outside
any `revisions!` block. They are invisible to the module-declaration fences, so
they are recorded here in revision-row grammar and checked against
`HISTORICAL_CUTOVERS` / `HISTORICAL_INSTANT_CUTOVERS` in
`tests/contract/session_invariants/historical_expectations.rs`.

- 2021-06-28 — T1 — FairX homepage banner, 2021-06-22 capture — an exact-instant boundary at 2021-06-28 13:00:00 UTC (08:00 CDT), selected in `profile_at` rather than as a tuple; `HISTORICAL_INSTANT_CUTOVERS` records the same instant.
- 2021-06-29 — T1 — the same four 2021-06-04 launch certifications — the one-off launch-day profile hands over to the recurring 23x5 grid on the first full day; the Monday-evening session is identical in both, so the handover splits no running session.

## Holidays

**Coverage:** 2021-06-28..2026-09-07 (inclusive trade dates). Tier: T1 throughout.

The venue profile is CDE's recurring 23x5 futures grid, so every row states what the operator's
notice states for the product groups on that grid. The window opens at the venue's first trade
date, FairX's launch Monday 2021-06-28, and closes at 2026-09-07, where the table the crate first
shipped stopped; the notices for 2026-09-08 onward belong to the published-future refresh.

On a half day the groups can print different instants. Ordinary half days are the Friday after
Thanksgiving, Christmas Eve and New Year's Eve: in 2021-11-26, 2023-11-24, 2025-11-28 and
2025-12-24 the Equity group printed 12:15 CT while Energy, and later Metals, printed 12:45 or
13:45 CT; in 2024-12-24 Energy and Metal printed 12:45 CT while the 23x5 Crypto group traded to
16:00 CT. **The row carries the earliest instant**, because a venue row must never report a window
in which no product on the grid can print. The other group's instant is quoted in the same line,
so the residual under-report is visible: for 2024-12-24 the 23x5 Crypto group really traded to
16:00 CT, and for 2025-11-28 the Energy & Metal group really traded to 13:45 CT.

### Documents

| Document | Window | Replay or service URL | Capture or retrieval, UTC | Tier | sha256 |
|---|---|---|---|---|---|
| `CDE-MN-21-03` | 2021-07-05 .. 2021-07-05 | <https://assets.ctfassets.net/k3n74unfin40/5j6yZybmG6MbpPTLPauK7g/9c1d755fbccae82d8d8603ec935741c0/Market_Notice_21-03.pdf> | retrieved 2026-09-19 02:02-02:45 UTC (live) | T1 | `5a73460829fc98288c754c7c0cf4230a716d5e4f3decac57bb76f32679dc57c3` |
| `CDE-MN-21-04` | 2021-09-06 .. 2021-09-06 | <https://assets.ctfassets.net/k3n74unfin40/3J67latBawdEG4jEjzpYnR/2a719ed20a044e29a419dc1411cfca70/Market_Notice_21-04.pdf> | retrieved 2026-09-19 02:02-02:45 UTC (live) | T1 | `18872d6f3993a3e22f2dc0f982889b7b71d8971148ddedf6cfcba8fd0fe89c1d` |
| `CDE-MN-21-06` | 2021-11-25 .. 2021-11-26 | <https://assets.ctfassets.net/k3n74unfin40/2tUXFAck99wsMx6mdOxxrE/147cfabf288adaed696fc153b2aad05d/Market_Notice_21-06_-_Thanksgiving_Schedule.pdf> | retrieved 2026-09-19 02:02-02:45 UTC (live) | T1 | `25ffa93abcd57af4fd46250baeac927075fe9d6b40e63f0f73461e4f8bf592af` |
| `CDE-MN-21-07` | 2021-12-24 .. 2021-12-24 | <https://assets.ctfassets.net/k3n74unfin40/7xnJ9WM8jTyCMzVhj7u7hh/058dfe0ec720df4bc6580071eb25acf0/Market_Notice_21-07.pdf> | retrieved 2026-09-19 02:02-02:45 UTC (live) | T1 | `1cdafb6827367d7ee86f27b3c70d7e2fd0708c51eccb59cd7b20289905d7bad8` |
| `CDE-MN-22-01` | 2022-01-17 .. 2022-01-17 | <https://assets.ctfassets.net/k3n74unfin40/3eN7Oh4yWCpE24Bqa9Z6OT/85b4cac3126c48d0361a97cb0faf17c4/Market_Notice_22-01.pdf> | retrieved 2026-09-19 02:02-02:45 UTC (live) | T1 | `d8e683ec405f991f107814bfd71b3733936263e9834fd875f465f654e24d3176` |
| `CDE-MN-22-02` | 2022-02-21 .. 2022-02-21 | <https://assets.ctfassets.net/k3n74unfin40/2W6sAOxys14tR9HqrJdn4b/b83a557ad4f3886b9f616188541435e6/Market_Notice_22-02.pdf> | retrieved 2026-09-19 02:02-02:45 UTC (live) | T1 | `5ea1e7872f16692720cd1ad98e898c94f5e02e3884418653ec1048fe6b5d0db9` |
| `CDE-MN-22-04` | 2022-04-15 .. 2022-04-15 | <https://assets.ctfassets.net/k3n74unfin40/2o0RNydeKJeHydTx6t7pHr/b01e70592a80966724c514479a3c77a1/Market_Notice_22-04-1.pdf> | retrieved 2026-09-19 02:02-02:45 UTC (live) | T1 | `c1807219193507dae8759a91f93d350b380df9f341068e33b54ccaddf3fd477c` |
| `CDE-MN-22-05` | 2022-05-30 .. 2022-05-30 | <https://assets.ctfassets.net/k3n74unfin40/78Ip0rSxgcXRKuodjWf6Co/ff22d52cae720bf0fc3f8b4bc11d3fd6/Market_Notice_22-05.pdf> | retrieved 2026-09-19 02:02-02:45 UTC (live) | T1 | `3fb42b97a01ed8b3d1d2ab587abcfe943e0bd2b730fc9d666d391df25fdfd86e` |
| `CDE-MN-22-06` | 2022-06-20 .. 2022-06-20 | <https://assets.ctfassets.net/k3n74unfin40/1AF9j5X7IGcMNTzYbYjEr8/e0c532ce277200a7dd667433cd56c7c1/Market_Notice_22-06.pdf> | retrieved 2026-09-19 02:02-02:45 UTC (live) | T1 | `1e7bb387263b0d13e9676e92ec2dea93c4dd2d7b4e638db49797142492a2d97f` |
| `CDE-MN-22-07` | 2022-07-04 .. 2022-07-04 | <https://assets.ctfassets.net/k3n74unfin40/4wYR4rvG3WqUT3wX7xpYK0/1fcb914490242560a6e96686c103c56c/Market_Notice__22-07__.pdf> | retrieved 2026-09-19 02:02-02:45 UTC (live) | T1 | `e9662aff64a82fbbddaf975db72039ac2b249ee7473f41dcd88764283bb434b8` |
| `CDE-MN-22-08` | 2022-09-05 .. 2022-09-05 | <https://assets.ctfassets.net/k3n74unfin40/5kTsBSbrdFquUSRaE0Y3Bu/9338b85c0c15cab3b99969b5f08e77a6/Market_Notice_22-08.pdf> | retrieved 2026-09-19 02:02-02:45 UTC (live) | T1 | `a64f6953582926df8209a487d3bfda7bc3786179744a19025bd00013c88ef825` |
| `CDE-MN-22-11` | 2022-12-26 .. 2022-12-26 | <https://assets.ctfassets.net/k3n74unfin40/1bcPFyFDQbRNqR6RhKwdX3/744286f7573b8b1b043a919670d70641/Market_Notice__22-11.pdf> | retrieved 2026-09-19 02:02-02:45 UTC (live) | T1 | `b4981e4fe3c6d388612716b1c72721ff5870825f867b67e75d8f3c9e036f8927` |
| `CDE-MN-23-01` | 2023-01-02 .. 2023-01-02 | <https://assets.ctfassets.net/k3n74unfin40/4Kwy5eQqAODwNauwpzOy5Q/ac41fe91f469a2ce779319d3f3e4b19f/Market_Notice_23-01.pdf> | retrieved 2026-09-19 02:02-02:45 UTC (live) | T1 | `e5057eb09b731a68839253744028d6040dedaf5e7fc70d9b7a6b29c5b23accd5` |
| `CDE-MN-23-02` | 2023-01-16 .. 2023-01-16 | <https://assets.ctfassets.net/k3n74unfin40/3HeoJwlxc8HjhN6SLYjqKy/2a120c0ee39ad4ec05e09d1a70defc93/Market_Notice_23-02.pdf> | retrieved 2026-09-19 02:02-02:45 UTC (live) | T1 | `ecd6968cb969aa0294585a9ec7fd08eac2b46a8a73057bb879fddd045509c55c` |
| `CDE-MN-23-03` | 2023-02-20 .. 2023-02-20 | <https://assets.ctfassets.net/k3n74unfin40/TB4RZvutilimzGnn3KZiu/c5b8b6c9bc19810379f77d53e48998b0/Market_Notice_23-03.pdf> | retrieved 2026-09-19 02:02-02:45 UTC (live) | T1 | `44b6dfdd9203d303d79bf4c5330ffa424bd6e79511bbdb9f92e09c1c14c2d349` |
| `CDE-MN-23-07` | 2023-04-07 .. 2023-04-07 | <https://assets.ctfassets.net/k3n74unfin40/3FPP5Zka1eTdppqoYhUZrs/66f61225d7890421b8718b19ed7f7155/Market_Notice_23-07.pdf> | retrieved 2026-09-19 02:02-02:45 UTC (live) | T1 | `c01b86456bc364bda6376714b84f1ad8c688cdc106afcd6b1cf0bde4b8a206ee` |
| `CDE-MN-23-09` | 2023-05-29 .. 2023-05-29 | <https://assets.ctfassets.net/k3n74unfin40/5u2uRMkdHX3Ugj6g4uhc0y/a1646aa1344490307dce4b003e454f1f/Market_Notice_23-09.pdf> | retrieved 2026-09-19 02:02-02:45 UTC (live) | T1 | `cef3d02dfb08659cac3e71731ab5d60ec1b5a4aca78ec8373b9497b20ffce9a9` |
| `CDE-MN-23-10` | 2023-06-19 .. 2023-06-19 | <https://assets.ctfassets.net/k3n74unfin40/6MOugUw3S4bMDnOcve6sgP/441982361827158977c87d8c4059ebda/Market_Notice_23-10.pdf> | retrieved 2026-09-19 02:02-02:45 UTC (live) | T1 | `7424b3e500c74135226622581cc747d002fb26e57cab24b8bbd8517255f8cf5d` |
| `CDE-MN-23-11` | 2023-07-04 .. 2023-07-04 | <https://assets.ctfassets.net/k3n74unfin40/2VBzXQYE9eXPjNDoul0lTC/bdeb4b8493b91c1793d6536258d83a22/Market_Notice_23-11.pdf> | retrieved 2026-09-19 02:02-02:45 UTC (live) | T1 | `65c59d6d37f028c1bb43dba87778b62b09f9fa99d80367c9ecd34b2cf4b2c984` |
| `CDE-MN-23-13` | 2023-09-04 .. 2023-09-04 | <https://assets.ctfassets.net/k3n74unfin40/1bYBIHxporz5fEvaUrQbRq/cf93dfb37ea24423a38242035643fb6b/Market_Notice_23-13.pdf> | retrieved 2026-09-19 02:02-02:45 UTC (live) | T1 | `e6ee3bde85e13c35c8a117219820d672ad18aa969baa839d37ecaa67115a4f6b` |
| `CDE-MN-23-16` | 2023-11-23 .. 2023-11-24 | <https://assets.ctfassets.net/k3n74unfin40/o77I9xXmB5oKk1iqS7a2E/921a498e23a9abd48c5ea8d236508e35/Market_Notice_23-16_.pdf> | retrieved 2026-09-19 02:02-02:45 UTC (live) | T1 | `0480b2b4c3f2aae413886246c02c8d593f22605ff11d3620e284e35dc931e3e2` |
| `CDE-MN-23-19` | 2023-12-25 .. 2023-12-25 | <https://assets.ctfassets.net/k3n74unfin40/3AbzZqebgUWjKmcpFuXHHL/d5d6d0dcfa1b822da19ca0662527b3da/Market_Notice_23-19.pdf> | retrieved 2026-09-19 02:02-02:45 UTC (live) | T1 | `ffd455d5e97d4c3ae179e2c923f7c14c73bbab502f6399fe93de701a852bc0c0` |
| `CDE-MN-23-20` | 2024-01-01 .. 2024-01-01 | <https://assets.ctfassets.net/k3n74unfin40/2z8C92dcb4Xr5vJlMhBN7O/d9842fb6981e0378554fe4c8a4f6395e/Market_Notice_23-20.pdf> | retrieved 2026-09-19 02:02-02:45 UTC (live) | T1 | `757166da5be7cd12e458d589ee782acd4a3824a5b35334f023e1a7bf2fe8ad35` |
| `CDE-MN-24-01` | 2024-01-15 .. 2024-01-15 | <https://assets.ctfassets.net/k3n74unfin40/5vsfhWqUY3CvI6jHSRo69u/046ee817dac781c70d55c8fb2bea1178/Market_Notice_24-01.pdf> | retrieved 2026-09-19 02:02-02:45 UTC (live) | T1 | `793ebdad7cc4aca12c1a7a36a7da2257f7b4754b4697c8484447e97836ccde9d` |
| `CDE-MN-24-02` | 2024-02-19 .. 2024-02-19 | <https://assets.ctfassets.net/k3n74unfin40/3rVNx1qvDfkqrk3C1AApvg/4eb2f5edb5db91258bf30950aebd210b/Market_Notice_24-02__Presidents-_Day.pdf> | retrieved 2026-09-19 02:02-02:45 UTC (live) | T1 | `a6c9ae9deb9b0b6dbec6fb83256fcf6e7b4add00a71aed96b91d5f8f6169800e` |
| `CDE-MN-24-04` | 2024-03-29 .. 2024-03-29 | <https://assets.ctfassets.net/k3n74unfin40/3mCWZ94h0Md4NUTgwUIGyd/bacf10959ef4a75deb51a9e492f4f3c9/Market_Notice_24-04.pdf> | retrieved 2026-09-19 02:02-02:45 UTC (live) | T1 | `c03e8e5bef2d017bd3b71d63077b5324e059db0c742e3975824f4925c8bd6041` |
| `CDE-MN-24-09` | 2024-05-27 .. 2024-05-27 | <https://assets.ctfassets.net/k3n74unfin40/1DNtOMWU6jTujck9FuQvW2/d7389961e4a1d0225cb3616a6956d264/Market_Notice_24-09__Memorial_Day.pdf> | retrieved 2026-09-19 02:02-02:45 UTC (live) | T1 | `f3547bfed329242b5939836dc4b992aac79ff6309f94bd989ab7809799f7331b` |
| `CDE-MN-24-12` | 2024-06-19 .. 2024-06-19 | <https://assets.ctfassets.net/k3n74unfin40/3qWRHv5qyUppIBwZXGdAAs/108f4e2cb2379887811a5a9dbdbb59ab/Market_Notice_24-12__Juneteenth.pdf> | retrieved 2026-09-19 02:02-02:45 UTC (live) | T1 | `fe2a89a8a4aae9372f0d4fd410a62f345d43bfb630abfe931c51b6c858674b20` |
| `CDE-MN-24-13` | 2024-07-04 .. 2024-07-04 | <https://assets.ctfassets.net/k3n74unfin40/mBPAEFh6TJB3oEtUjI70F/65936d9de4dd1255965c70b3330a49a4/Market_Notice_24-13__Independence_Day.pdf> | retrieved 2026-09-19 02:02-02:45 UTC (live) | T1 | `abf2c18aecc0e8ed381c8579e730f6d9d63255e9dff0db695719303f5d3ee002` |
| `CDE-MN-24-16` | 2024-09-02 .. 2024-09-02 | <https://assets.ctfassets.net/k3n74unfin40/7gKjRY61IbchrmbIkFUWUG/935416c088fe48f01845b56d6615bdd9/Market_Notice_24-16__Labor_Day.pdf> | retrieved 2026-09-19 02:02-02:45 UTC (live) | T1 | `0b39ec30c81ce2eeecb7d387850d354eca6d17b1db548a8c19a2e6845b414e44` |
| `CDE-MN-24-21` | 2024-11-28 .. 2024-11-29 | <https://assets.ctfassets.net/k3n74unfin40/5LqbVzBHS4xEypz6J5M4s4/0b107fb67289db11819d88acdb420421/Market_Notice_24-21_Thanksgiving.pdf> | retrieved 2026-09-19 02:02-02:45 UTC (live) | T1 | `4cecb30797437bc88ecdab2e92c271093c38096d77ce83b7f56cfa3db42e61ff` |
| `CDE-MN-24-23` | 2024-12-24 .. 2024-12-25 | <https://assets.ctfassets.net/k3n74unfin40/4M0jGaNd1zO9eeamF3LLyg/01bdbe21710c7cf6283447107337e02f/Market_Notice_24-23_Christmas.pdf> | retrieved 2026-09-19 02:02-02:45 UTC (live) | T1 | `afe928a517ff00149b5281a794c28eb740d6bbd3b044c4dc77ff605918cce4c2` |
| `CDE-MN-24-25` | 2025-01-01 .. 2025-01-01 | <https://assets.ctfassets.net/k3n74unfin40/2vlwxnd4tklT8Nk5wkZ0M9/5a38c34116702fa7e691fdcfcbe6d989/Market_Notice_24-25_New_Years_Day.pdf> | retrieved 2026-09-19 02:02-02:45 UTC (live) | T1 | `41b602c904f69adbea9ec6c30074938b87de9a431b9110285c4d28af761c474a` |
| `CDE-MN-24-26` | n/a | <https://assets.ctfassets.net/k3n74unfin40/9OQS9VAhJ2ivUiXjnNM4J/d817c78546bcda19b3daa6163d39e73e/Market_Notice_24-26_Early_Close_on_Dec-24.pdf> | retrieved 2026-09-19 02:02-02:45 UTC (live) | T1 | `cc365da86ef0af2ce43819208b7c3daff26f070319ca777d0cd71d6c8f66a355` |
| `CDE-MN-24-27` | n/a | <https://assets.ctfassets.net/k3n74unfin40/6bdOsBLIXmHyAM5HcQs6ep/56d02a0085be8b8da4696bc655e83fd3/Market_Notice_24-27_U.S._National_Day_of_Mourning.pdf> | retrieved 2026-09-19 02:02-02:45 UTC (live) | T1 | `7cadc40b10df7ab2eece838b9d45536a2eef7bebe3d56cb29070d079d0dcd809` |
| `CDE-MN-25-01` | 2025-01-20 .. 2025-01-20 | <https://assets.ctfassets.net/k3n74unfin40/1CkT7Lv5cA2G7jN6b3TGxK/9b21090199763c2c81c5726f76ec4b87/Market_Notice_25-01__MLK.pdf> | retrieved 2026-09-19 02:02-02:45 UTC (live) | T1 | `6250247db03bc5c595a894bf60d2e1e931ce5dde5add6205ffd5fa70a08cd507` |
| `CDE-MN-25-03` | 2025-02-17 .. 2025-02-17 | <https://assets.ctfassets.net/k3n74unfin40/HRCLVTq5c3Aymj34wr885/434e11816f4537e6de3b384cd51637c8/Market_Notice_25-03__Presidents-_Day.pdf> | retrieved 2026-09-19 02:02-02:45 UTC (live) | T1 | `ee52b39083b6ea106efa5785053ff0143adab94232f157d5e1bc7c118b077080` |
| `CDE-MN-25-15` | 2025-04-18 .. 2025-04-18 | <https://assets.ctfassets.net/k3n74unfin40/52cGUHIs2GVIWyBF2aDrfF/83161f6ad1317cca4f1be654151d40de/25-15__Good_Friday.pdf> | retrieved 2026-09-19 02:02-02:45 UTC (live) | T1 | `a2cd1b6dc73761ac357b4f3f6cca2166396652ee67ca5c65c628f3801f68e8aa` |
| `CDE-MN-25-18` | 2025-05-26 .. 2025-05-26 | <https://assets.ctfassets.net/k3n74unfin40/2fPRQKeU5nkKOvK26l8Asc/2a7af5c5893a58a2f3013065a54a3572/Market_Notice_25-18__Memorial_Day.pdf> | retrieved 2026-09-19 02:02-02:45 UTC (live) | T1 | `da63c79eb196850a5e09c034dc6716507dfceb6d1f27af13206727b19ce3325b` |
| `CDE-MN-25-20` | 2025-06-19 .. 2025-06-19 | <https://assets.ctfassets.net/k3n74unfin40/2kXyopqCU858pWtLcDjuWJ/6e42751bff50238b748ff7bb9a7286d4/Market_Notice_25-20__Juneteenth.pdf> | retrieved 2026-09-19 02:02-02:45 UTC (live) | T1 | `83a2ede3216fc304bbdd4766982b0c609864cd288ba69d496ca0db3caad94fff` |
| `CDE-MN-25-21` | 2025-07-04 .. 2025-07-04 | <https://assets.ctfassets.net/k3n74unfin40/2kaf1nLCbdKjY27wP3alhw/130dc3c520c16d33d5180f64b928854f/Market_Notice_25-21__Independence_Day_Holiday.pdf> | retrieved 2026-09-19 02:02-02:45 UTC (live) | T1 | `6dcb0d5224654bee8f90849c424c5dde88b1ab5c6cffc932a7082924e614b364` |
| `CDE-MN-25-29` | 2025-09-01 .. 2025-09-01 | <https://assets.ctfassets.net/k3n74unfin40/7ojwVPGtodkdB4iZANQFKu/5c72b3329776b53973cb00d115eb2f58/CDE_Market_Notice_25-29_Labor_Day_Holiday.pdf> | retrieved 2026-09-19 02:02-02:45 UTC (live) | T1 | `9fc3dd980675d0fadfa44502f054bb58dacc7450a8f61f4a0efd60647688d219` |
| `CDE-MN-25-37` | 2025-11-27 .. 2025-11-28 | <https://assets.ctfassets.net/k3n74unfin40/6ZRvjcwtmiDg4i46Dl35SP/527990b1872111261a81a77bc2f18377/Market_Notice_25-37__Thanksgiving_Schedule.pdf> | retrieved 2026-09-19 02:02-02:45 UTC (live) | T1 | `3826ec0405ed0398ed31cb2523b7dc9d315f6443bdcd3454af97d602c61a46c7` |
| `CDE-MN-25-41` | 2025-12-24 .. 2025-12-25 | <https://assets.ctfassets.net/k3n74unfin40/1Dh6mmmzrARNnjQcFy5lob/ce2f407a6c1d9291cf204b1b65e3a35c/Market_Notice_25-41__Christmas.pdf> | retrieved 2026-09-19 02:02-02:45 UTC (live) | T1 | `d8097f3af89a2702fc2f97f46bba8185e92eb4c4630a9ae066020979033b56d9` |
| `CDE-MN-25-42` | 2026-01-01 .. 2026-01-01 | <https://assets.ctfassets.net/k3n74unfin40/46Q4yxPzeFtSLYsDm4hz6C/f586118d66500c05dddcacceac04be97/Market_Notice_25-42_New_Years_Day.pdf> | retrieved 2026-09-19 02:02-02:45 UTC (live) | T1 | `ac81272ecfc006a634cf855723f8a6d6875476555427d8612d90761172c12b0e` |
| `CDE-MN-26-01` | 2026-01-19 .. 2026-01-19 | <https://images.ctfassets.net/k3n74unfin40/jJEoZRkAZ0JlPC3UcJ2CL/d71ac89c5cd10783b9a7e11f449d1424/Market_Notice_26-01_MLK_Holiday.pdf> | retrieved 2026-09-19 02:02-02:45 UTC (live) | T1 | `d4b206391f70a015002af78c04c46bdce0ed08f87005b5b8e383cc7c5cd01f38` |
| `CDE-MN-26-05` | 2026-02-16 .. 2026-02-16 | <https://assets.ctfassets.net/k3n74unfin40/46lI9u5Ahz4bX22aj4H29u/025010e407b86208a1442f4c6967f2a6/Market_Notice_26-05_Presidents-_Day_Holiday.pdf> | retrieved 2026-09-19 02:02-02:45 UTC (live) | T1 | `9cc8ffb0b88a2a007d31243cd27ffc5c696585ddba8ec06ca562e63c1a448c34` |
| `CDE-MN-26-12` | 2026-04-03 .. 2026-04-03 | <https://assets.ctfassets.net/k3n74unfin40/6DezMFxYrU5eD4x51kbfEv/914143ffbf3349e88befe96d5c2e35cc/CDE_Market_Notice_26-12_Good_Friday_Holiday.pdf> | retrieved 2026-09-19 02:02-02:45 UTC (live) | T1 | `3226ba73e92d91f252e8c163d49f9b329567c77f701dfba6a7f85413a25de150` |
| `CDE-MN-26-23` | 2026-05-25 .. 2026-05-25 | <https://assets.ctfassets.net/k3n74unfin40/6NUx3bRH0pdR2mSYCSW0pD/9cb91cc5b9165e39079e17fac0a03f8f/CDE_Market_Notice_26-23_2026_Memorial_Day.pdf> | retrieved 2026-09-19 02:02-02:45 UTC (live) | T1 | `b41bc8ee48084084fe5dbe8ac77be1f98688fad1b4b5a134d4fa6cd79f5c0869` |
| `CDE-MN-26-27` | n/a | <https://assets.ctfassets.net/k3n74unfin40/sISYGuk6L7FuXllMBlsIY/1d17b56138c2b6907e4cd02d07f7239b/CDE_Market_Notice_26-27_2026_Juneteenth_Schedule.pdf> | retrieved 2026-09-19 02:02-02:45 UTC (live) | T1 | `e13cb644cebef3c851e83c14132860c8b6abc9b026a0dd23b36c5b55ae072b79` |
| `CDE-MN-26-27.1` | 2026-06-19 .. 2026-06-19 | <https://assets.ctfassets.net/k3n74unfin40/4Vy8yQyOCHG8dWW20SjNTU/033e076b88302e28249198bd72b518b5/CDE_Market_Notice_26-27.1_Amendment_to_26.27_2026_Juneteenth_Schedule_for_Gold_Silver_24x7.pdf> | retrieved 2026-09-19 02:02-02:45 UTC (live) | T1 | `c141366e2f50eeacfd26def14e0ad6a9de448b0f319d79a5007933769881c10f` |
| `CDE-MN-26-29` | 2026-07-03 .. 2026-07-03 | <https://assets.ctfassets.net/k3n74unfin40/70J0u2Lv7TOspmrGMI30QI/95ee852baac3d551aabe41acb334837d/CDE_Market_Notice_26-29__2026_Independence_Day_Schedule.pdf> | retrieved 2026-09-19 02:02-02:45 UTC (live) | T1 | `481e0f35da824519f4046129cc66142ae004b90fe19b2749b2073f8b89dea736` |
| `CDE-MN-26-36` | 2026-09-07 .. 2026-09-07 | <https://assets.ctfassets.net/k3n74unfin40/3QC7hwlE6hRHicbNapLlWn/e36b89318d9dbcfc17c931f70912e4c4/Market_Notice_26-36__2026_Labor_Day.pdf> | retrieved 2026-09-19 02:02-02:45 UTC (live) | T1 | `c21dc783a6e6c560540126a30572e932654d8531f4e7225f0b3221514116616d` |
| `CDE-NOTICES-INDEX-2026-09-19` | 2021-06-01 .. 2026-09-10 | <https://www.coinbase.com/derivatives/market-notices> | retrieved 2026-09-19 02:02 UTC via the public reader (direct coinbase.com returns 403); raw bytes `cde_market_notices_raw.html`, sha256 `54816ca2d73db15058f89afecb88861b13fe7f16c2a22bcc3092cc4ddc5d0dbc` | T1 | `54816ca2d73db15058f89afecb88861b13fe7f16c2a22bcc3092cc4ddc5d0dbc` |

All bytes are in the research store under `holidays/raw/cde-2021-2025/` with a URL, UTC retrieval
time and sha256 per artifact in its `INDEX.md`; the operator statements behind every row are the
`OPS` literal in that directory's `build_block.py`, and the verified result is
`holidays/cde-2021-2026.json`.

### 2021

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2021-07-05 | closed | `Markets will be closed from 16:00 CT Friday, July 2nd` | `CDE-MN-21-03` | T1 | Monday 7/5 is the observed holiday the notice's subject names; markets are closed from 16:00 CT the preceding Friday to 17:00 CT that Monday, and the Monday-evening session that follows belongs to the next trade date |
| 2021-09-06 | closed | `Markets will be closed from 16:00 CT Friday, September 2nd` | `CDE-MN-21-04` | T1 | Monday 9/6 is the observed holiday the notice's subject names; markets are closed from 16:00 CT the preceding Friday to 17:00 CT that Monday, and the Monday-evening session that follows belongs to the next trade date |
| 2021-11-25 | closed | `Closed for holiday` | `CDE-MN-21-06` | T1 | CDE's own Trade Date column names Thursday 11/25; its OPEN and CLOSE cells both print the quoted text for Energy Products, Equity Products |
| 2021-11-26 | early close | Equity Products `11/26 12:15 CT`; Energy Products `11/26 12:45 CT` | `CDE-MN-21-06` | T1 | CDE's own Trade Date column names Friday 11/26; the venue row takes the earliest close among the groups on the 23x5 grid (Energy Products, Equity Products), so it never reports a window in which no product prints |
| 2021-12-24 | closed | `Closed for holiday` | `CDE-MN-21-07` | T1 | CDE's own Trade Date column names Friday 12/24; its OPEN and CLOSE cells both print the quoted text for Energy Products, Equity Products |

### 2022

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2022-01-17 | closed | `Closed for holiday` | `CDE-MN-22-01` | T1 | CDE's own Trade Date column names Monday 1/17; its OPEN and CLOSE cells both print the quoted text for Equity Products |
| 2022-02-21 | closed | `Closed for holiday` | `CDE-MN-22-02` | T1 | CDE's own Trade Date column names Monday 2/21; its OPEN and CLOSE cells both print the quoted text for Equity Products |
| 2022-04-15 | closed | `Closed for holiday` | `CDE-MN-22-04` | T1 | CDE's own Trade Date column names Friday 4/15; its OPEN and CLOSE cells both print the quoted text for Equity Products |
| 2022-05-30 | closed | `Closed for holiday` | `CDE-MN-22-05` | T1 | CDE's own Trade Date column names Monday 5/30; its OPEN and CLOSE cells both print the quoted text for Equity Products |
| 2022-06-20 | closed | `Closed for holiday` | `CDE-MN-22-06` | T1 | CDE's own Trade Date column names Monday 6/20; its OPEN and CLOSE cells both print the quoted text for Equity Products |
| 2022-07-04 | closed | `Closed for holiday` | `CDE-MN-22-07` | T1 | CDE's own Trade Date column names Monday 7/4; its OPEN and CLOSE cells both print the quoted text for Equity Products |
| 2022-09-05 | closed | `Closed for holiday` | `CDE-MN-22-08` | T1 | CDE's own Trade Date column names Monday 9/5; its OPEN and CLOSE cells both print the quoted text for Crypto Products, Equity Products |
| 2022-11-24 | unsourced | &mdash; no status claimed | `CDE-NOTICES-INDEX-2026-09-19` | T1 | notice `CDE-MN-22-10` ("Market Notice - Thanksgiving Holiday Schedule 2022") governs this date; the operator lists it but its PDF is unreachable, so the date is carried as not audited rather than claimed closed |
| 2022-11-25 | unsourced | &mdash; no status claimed | `CDE-NOTICES-INDEX-2026-09-19` | T1 | notice `CDE-MN-22-10` ("Market Notice - Thanksgiving Holiday Schedule 2022") governs this date; the operator lists it but its PDF is unreachable, so the date is carried as not audited rather than claimed closed |
| 2022-12-26 | closed | `Closed for holiday` | `CDE-MN-22-11` | T1 | CDE's own Trade Date column names Monday 12/26; its OPEN and CLOSE cells both print the quoted text for Equity Products |

### 2023

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2023-01-02 | closed | `Closed for holiday` | `CDE-MN-23-01` | T1 | CDE's own Trade Date column names Monday 1/2; its OPEN and CLOSE cells both print the quoted text for Equity Products |
| 2023-01-16 | closed | `Closed for holiday` | `CDE-MN-23-02` | T1 | CDE's own Trade Date column names Monday 1/16; its OPEN and CLOSE cells both print the quoted text for Equity Products |
| 2023-02-20 | closed | `Closed for holiday` | `CDE-MN-23-03` | T1 | CDE's own Trade Date column names Monday 2/20; its OPEN and CLOSE cells both print the quoted text for Equity Products |
| 2023-04-07 | closed | `Closed for holiday` | `CDE-MN-23-07` | T1 | CDE's own Trade Date column names Friday 4/7; its OPEN and CLOSE cells both print the quoted text for Equity Products |
| 2023-05-29 | closed | `Closed for holiday` | `CDE-MN-23-09` | T1 | CDE's own Trade Date column names Monday 5/29; its OPEN and CLOSE cells both print the quoted text for Equity Products |
| 2023-06-19 | closed | `Closed for holiday` | `CDE-MN-23-10` | T1 | CDE's own Trade Date column names Monday 6/19; its OPEN and CLOSE cells both print the quoted text for Equity Products |
| 2023-07-04 | closed | `Closed for holiday` | `CDE-MN-23-11` | T1 | CDE's own Trade Date column names Tuesday 7/4; its OPEN and CLOSE cells both print the quoted text for Equity Products |
| 2023-09-04 | closed | `Closed for holiday` | `CDE-MN-23-13` | T1 | CDE's own Trade Date column names Monday 9/4; its OPEN and CLOSE cells both print the quoted text for Equity Products |
| 2023-11-23 | closed | `Closed for holiday` | `CDE-MN-23-16` | T1 | CDE's own Trade Date column names Thursday 11/23; its OPEN and CLOSE cells both print the quoted text for Equity Products |
| 2023-11-24 | early close | Equity Products `11/24 12:15 CT`; Energy Products `11/24 12:45 CT`; Crypto Products `11/24 12:45 CT` | `CDE-MN-23-16` | T1 | CDE's own Trade Date column names Friday 11/24; the venue row takes the earliest close among the groups on the 23x5 grid (Crypto Products, Energy Products, Equity Products), so it never reports a window in which no product prints |
| 2023-12-25 | closed | `Closed for holiday` | `CDE-MN-23-19` | T1 | CDE's own Trade Date column names Monday 12/25; its OPEN and CLOSE cells both print the quoted text for Equity Products |

### 2024

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2024-01-01 | closed | `Closed for holiday` | `CDE-MN-23-20` | T1 | CDE's own Trade Date column names Monday 1/1; its OPEN and CLOSE cells both print the quoted text for Equity Products |
| 2024-01-15 | closed | `Closed for` | `CDE-MN-24-01` | T1 | CDE's own Trade Date column names Monday 1/15; its OPEN and CLOSE cells both print the quoted text for Crypto Products |
| 2024-02-19 | closed | `Closed for` | `CDE-MN-24-02` | T1 | CDE's own Trade Date column names Monday 2/19; its OPEN and CLOSE cells both print the quoted text for Crypto Products |
| 2024-03-29 | closed | `Closed for holiday` | `CDE-MN-24-04` | T1 | CDE's own Trade Date column names Friday 3/29; its OPEN and CLOSE cells both print the quoted text for Crypto Products |
| 2024-05-27 | closed | `Closed for` | `CDE-MN-24-09` | T1 | CDE's own Trade Date column names Monday 5/27; its OPEN and CLOSE cells both print the quoted text for Crypto Products |
| 2024-06-19 | closed | `Closed for` | `CDE-MN-24-12` | T1 | CDE's own Trade Date column names Wednesday 6/19; its OPEN and CLOSE cells both print the quoted text for Energy Products |
| 2024-07-04 | closed | `Closed for` | `CDE-MN-24-13` | T1 | CDE's own Trade Date column names Thursday 7/4; its OPEN and CLOSE cells both print the quoted text for Energy Products |
| 2024-09-02 | closed | `Closed for` | `CDE-MN-24-16` | T1 | CDE's own Trade Date column names Monday 9/2; its OPEN and CLOSE cells both print the quoted text for Energy Products |
| 2024-11-28 | closed | `Closed for` | `CDE-MN-24-21` | T1 | CDE's own Trade Date column names Thursday 11/28; its OPEN and CLOSE cells both print the quoted text for Energy Products |
| 2024-11-29 | early close | Energy Products `11/29 13:45 CT`; Metal Products `11/29 13:45 CT`; Crypto Products `11/29 13:45 CT` | `CDE-MN-24-21` | T1 | CDE's own Trade Date column names Friday 11/29; the venue row takes the earliest close among the groups on the 23x5 grid (Crypto Products, Energy Products, Metal Products), so it never reports a window in which no product prints |
| 2024-12-24 | early close | Energy Products `12/24 12:45 CT`; Metal Products `12/24 12:45 CT`; Crypto Products trades to 16:00 CT (`12/23 17:00 CT 12/24 16:00 CT`) | `CDE-MN-24-23` | T1 | CDE's own Trade Date column names Tuesday 12/24; the venue row takes the earliest close among the groups on the 23x5 grid (Crypto Products, Energy Products, Metal Products), so it never reports a window in which no product prints |
| 2024-12-25 | closed | `Closed for` | `CDE-MN-24-23` | T1 | CDE's own Trade Date column names Wednesday 12/25; its OPEN and CLOSE cells both print the quoted text for Energy Products |

### 2025

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2025-01-01 | closed | `Closed for holiday` | `CDE-MN-24-25` | T1 | CDE's own Trade Date column names Wednesday 1/1; its OPEN and CLOSE cells both print the quoted text for Crypto Products |
| 2025-01-20 | closed | `Closed for` | `CDE-MN-25-01` | T1 | CDE's own Trade Date column names Monday 1/20; its OPEN and CLOSE cells both print the quoted text for Energy Products |
| 2025-02-17 | closed | `Closed for` | `CDE-MN-25-03` | T1 | CDE's own Trade Date column names Monday 2/17; its OPEN and CLOSE cells both print the quoted text for Energy Products |
| 2025-04-18 | closed | `Closed for` | `CDE-MN-25-15` | T1 | CDE's own Trade Date column names Friday 4/18; its OPEN and CLOSE cells both print the quoted text for Energy Products |
| 2025-05-26 | closed | `Closed for` | `CDE-MN-25-18` | T1 | CDE's own Trade Date column names Monday 5/26; its OPEN and CLOSE cells both print the quoted text for Energy & Metal |
| 2025-06-19 | closed | `Closed for holiday` | `CDE-MN-25-20` | T1 | CDE's own Trade Date column names Thursday 6/19; its OPEN and CLOSE cells both print the quoted text for Energy & Metal |
| 2025-07-04 | closed | `Closed for holiday` | `CDE-MN-25-21` | T1 | CDE's own Trade Date column names Friday 7/4; its OPEN and CLOSE cells both print the quoted text for Energy & Metal |
| 2025-09-01 | closed | `Closed for holiday` | `CDE-MN-25-29` | T1 | CDE's own Trade Date column names Monday 9/1; its OPEN and CLOSE cells both print the quoted text for Energy & Metal |
| 2025-11-27 | closed | `Closed for holiday` | `CDE-MN-25-37` | T1 | CDE's own Trade Date column names Thursday 11/27; its OPEN and CLOSE cells both print the quoted text for Energy & Metal |
| 2025-11-28 | early close | Energy & Metal `11/28 13:45 CT`; Equity `11/28 12:15 CT`; 23x5 Crypto trades to 16:00 CT (`11/27 17:00 CT 11/28 16:00 CT`) | `CDE-MN-25-37` | T1 | CDE's own Trade Date column names Friday 11/28; the venue row takes the earliest close among the groups on the 23x5 grid (23x5 Crypto, Energy & Metal, Equity), so it never reports a window in which no product prints |
| 2025-12-24 | early close | Energy & Metal `12/24 12:45 CT`; Equity `12/24 12:15 CT` | `CDE-MN-25-41` | T1 | CDE's own Trade Date column names Wednesday 12/24; the venue row takes the earliest close among the groups on the 23x5 grid (Energy & Metal, Equity), so it never reports a window in which no product prints |
| 2025-12-25 | closed | `Closed for holiday` | `CDE-MN-25-41` | T1 | CDE's own Trade Date column names Thursday 12/25; its OPEN and CLOSE cells both print the quoted text for Energy & Metal |

### 2026

| Trade date | Kind | Instant as printed | Document | Tier | Derived from |
|---|---|---|---|---|---|
| 2026-01-01 | closed | `Closed for holiday` | `CDE-MN-25-42` | T1 | CDE's own Trade Date column names Thursday 1/1; its OPEN and CLOSE cells both print the quoted text for Energy & Metal |
| 2026-01-19 | closed | `Closed for holiday` | `CDE-MN-26-01` | T1 | CDE's own Trade Date column names Monday 1/19; its OPEN and CLOSE cells both print the quoted text for Energy, Metal & Equity |
| 2026-02-16 | closed | `Closed for holiday` | `CDE-MN-26-05` | T1 | CDE's own Trade Date column names Monday 2/16; its OPEN and CLOSE cells both print the quoted text for Energy, Metal & Equity |
| 2026-04-03 | closed | `Closed for holiday` | `CDE-MN-26-12` | T1 | CDE's own Trade Date column names Friday 4/3; its OPEN and CLOSE cells both print the quoted text for Energy, Metal & Equity |
| 2026-05-25 | closed | `Closed for holiday` | `CDE-MN-26-23` | T1 | CDE's own Trade Date column names Monday 5/25; its OPEN and CLOSE cells both print the quoted text for Energy, Metal & Equity |
| 2026-06-19 | closed | `Closed for holiday` | `CDE-MN-26-27.1` | T1 | CDE's own Trade Date column names Friday 6/19; its OPEN and CLOSE cells both print the quoted text for 23x5 Products |
| 2026-07-03 | closed | `Closed for holiday` | `CDE-MN-26-29` | T1 | CDE's own Trade Date column names Friday 7/3; its OPEN and CLOSE cells both print the quoted text for 23x5 Products |
| 2026-09-07 | closed | `Closed for holiday` | `CDE-MN-26-36` | T1 | CDE's own Trade Date column names Monday 9/7; its OPEN and CLOSE cells both print the quoted text for 23x5 Products |

### Gaps and residual risks, 2021-2026

- **Notice 24-12 is headed "IN DRAFT".** It is the only notice in the corpus whose own
  published PDF carries that watermark, and it is the notice behind trade date 2024-06-19. The
  operator lists it at T1 with a posted date of 06/05/2024 and its grid is unambiguous, and a
  closure shipped in error errs toward closed rather than toward reporting a window in which no
  product traded, so the row stands. Residual risk: a later final revision is not held. Closing
  condition: a non-draft copy of notice 24-12.

- **Trade dates 2022-11-24 and 2022-11-25 ship `Unsourced`.** The operator's own listing carries
  notice 22-10, "Market Notice - Thanksgiving Holiday Schedule 2022", category Holiday, posted
  12/12/2022. Its href points at
  `info.fairx.com/coinbase-derivatives-market-notice-22-10-thanksgiving-holiday-schedule-2022`,
  where the host no longer completes a TLS handshake, and the Wayback Machine holds no capture of
  it: a CDX prefix query over `assets.ctfassets.net/k3n74unfin40*` returns 144 `Market_Notice`
  artifacts from 21-01 to 25-45 and none of them is 22-10, and a query over `info.fairx.com*`
  returns exactly one unrelated 2022 capture. `Unsourced` clips nothing, so the 23x5 grid applies
  unchanged; the crate simply declines to certify the date. Closing condition: any surviving copy
  of notice 22-10, or a later notice that restates the outgoing 2022 schedule.
- **Trade date 2021-12-31 is carried as audited normal and ships no row.** The operator published
  no notice for it: the 2021 listing runs 21-01 to 21-07 with no gaps, and 21-07 (issued
  2021-12-21, the last 2021 notice) covers Christmas only. New Year's Day 2022 fell on a Saturday
  and the 23x5 grid has no Saturday session. Residual risk: a closure the operator never
  published would be under-reported here.
- **Notice 24-26 keys no row.** "On December 24, 2024, Coinbase Derivatives Crypto Futures markets
  closed early at 14:30 CT due to a technical issue." That is an unplanned venue incident, not a
  published holiday schedule, and it names the crypto tier; trade date 2024-12-24 takes its row
  from notice 24-23, the published Christmas schedule.
- **Notice 24-27 keys no row.** It states that the 2025-01-09 session (a National Day of Mourning)
  "will observe a normal trading day", so 2025-01-09 is an audited-normal date.
- **The half-day rows under-report the later-closing groups.** See the `## Holidays` preamble: the
  venue row carries the earliest instant on 2021-11-26, 2023-11-24, 2024-11-29, 2024-12-24,
  2025-11-28 and 2025-12-24. A caller trading a group that prints a later close on those dates
  should use its own key; no key is claimed for the 24x7 crypto tier or the 24x5 equity-index PSF
  group.
- **Trade date 2021-09-03 is carried as audited normal.** Notice 21-04 says markets are "closed
  from 16:00 CT Friday, September 2nd", but 2 September 2021 was a Thursday and the Friday before
  Labor Day was the 3rd; the notice's subject ("Labor Day Observed Monday, September 6th, 2021"),
  its reopening sentence and the identical construction in notice 21-03 ("closed from 16:00 CT
  Friday, July 2nd") all place the closure on the Monday. The operator's date is a typo; the
  affected trade date is 2021-09-06, which ships `closed`.
- **The 2026-09-08 onward notices are not retrieved here.** They belong to the published-future
  refresh, not to this window; the last row this table ships is 2026-09-07.

## Sources

Row review: 2026-09-11 (UTC) is the date the ledger row was last reviewed as a
whole. Per-source retrieval dates were not recorded before the 2026-09-12
migration; where a later targeted review, capture or document date is recorded
beside a source below, that date governs for that source, and dates are added
as each source is re-verified.

The documents below stand behind the row and behind the narrative moved below.

- <https://www.cftc.gov/filings/ptc/ptc060421lmxdcm001.pdf> — FairX launch certification #2021-06E, filed 2021-06-04, one of the four that state the 23x5 grid at launch — T1.
- <https://www.cftc.gov/filings/orgrules/rules0416261571.pdf> — Coinbase Derivatives filing #2026-24, which states which products keep the 23x5 grid after the 24x7 move — T1.
- <https://docs.cdp.coinbase.com/derivatives/introduction/market-hours> — the operator's market-hours documentation, the current-schedule entry point and the page that witnesses the 16:50 CT Pre-Open — T1. The page is CDE's "Derivatives Market Hours & 24x7" document, whose own opening sentence dates its content ("Starting May 9, 2025, Coinbase Derivatives, LLC (CDE) will enable 24x7 trading for select cryptocurrency futures products"); under **Regular Market Hours for 23x5 Crypto Products** and again under **Regular Market Hours for Energy and Metals Products** it states "Pre-open quoting begins daily at 4:50 PM CT, 10 minutes before the market opens."
- Research-store artifact `holidays/raw/cfe-eurex-ice-cde-smfe-2026-2027/cde_market_hours.md` — the saved bytes of that page, **captured 2026-09-12 04:23 UTC** (sha256 `ff8cace80f2f6bb9e463c6ac762c8f50f79fa76ce7b205c14e42d29e847d13a5`), carrying both Pre-Open sentences quoted above — T1. This is the earliest capture of the page held in the research store; the "2025 capture" the basis note and the gap bullet refer to is the 2025-dated *content* of this document, not a separately saved 2025 retrieval. No archived 2025 capture is linked, because web.archive.org was unreachable for the whole 2026-09-12 retrieval session (recorded in that task's `INDEX.md`).
- <https://www.coinbase.com/derivatives> — the exchange product directory — T1.
- <https://help.coinbase.com/derivatives/general/market-notices> — the exchange's market notices, the watch channel — T1.
- <https://web.archive.org/web/20210622222253/https://www.fairx.com/> — the 2021-06-22 capture of FairX's homepage banner, naming Monday 2021-06-28 at 09:00 ET and fixing the year — T1 through a verbatim public mirror.
- <https://web.archive.org/web/20210802230534/https://www.fairx.com/> — the 2021-08-02 capture, reporting the venue open — T1 through a verbatim public mirror.
- <https://www.cftc.gov/IndustryOversight/IndustryFilings/TradingOrganizations/43304> — the CFTC DCM record tying FairX, Coinbase Derivatives and the 2023 legal-name change to one contract market — T1.

## Gaps and residual risks

- **order-entry** — the Pre-Open queue's onset is undated. Coinbase Derivatives
  documents a Pre-Open phase from 2021 without a time, and the 16:50 CT start is
  first witnessed in a 2025 capture of the market-hours page, so no artifact
  states when it began. The crate therefore withholds the queue from every dated
  profile and admits it only at the knowledge-bound 2026-09-11 review row. No
  trade can print in the queue, so the gap never touches an executable window.
  Closing condition: a Coinbase Derivatives or FairX artifact that states the
  Pre-Open in session language on a day-level effective date. Served identity,
  so tracked as an issue (LAW-FOLLOW-UPS-ARE-ISSUES).
- **The 24x7 family is deliberately out of scope.** Since #2026-24, on or after
  trade date 2026-05-04, most CDE futures trade 24x7 with weekly Friday,
  quarterly weekend and ad-hoc maintenance windows. The venue default stays the
  23x5 grid that copper, platinum, nano crude oil, natural gas and the
  Mag7 + Crypto equity index futures retain. A 24x7 product family needs its own
  `MarketHoursKey` and its own sourced windows; none is claimed here, and the
  caller must not read the venue default as covering those roots.
- **A golden-fixture consequence, recorded so it is not rediscovered.** The
  2026-08-22 fixture instant in `tests/golden/normal_week_grids.txt` predates the
  knowledge-bound row, so it renders the dated grid without the Pre-Open.

## Module narrative (moved from src/calendar/schedules/futures/us/coinbase_derivatives.rs on 2026-09-12 UTC)

The venue default is CDE's recurring 23x5 futures grid: Sunday through
Friday, 17:00-16:00 CT, with the daily 16:00-17:00 break. The four launch
certifications filed 2021-06-04 state that grid, and every later dated
filing through #2026-24 restates it, so no revision separates launch from
the 2026-09-11 review date. Since #2026-24 (on or after trade date 2026-05-04) most CDE futures
trade 24x7; this grid is the one copper, platinum, nano crude oil, natural
gas and Mag7 + Crypto equity index futures retain. The 24x7 family needs its
own product-family key at the caller.
<https://www.cftc.gov/filings/ptc/ptc060421lmxdcm001.pdf>
<https://www.cftc.gov/filings/orgrules/rules0416261571.pdf>
<https://docs.cdp.coinbase.com/derivatives/introduction/market-hours>

ORDER ENTRY, NOT TRADING. Pre-Open quoting accepts orders for the coming
session and nothing matches until the 17:00 open. The Pre-Open phase is
documented from 2021 without a time, and its 16:50 start is first witnessed
in a 2025 capture of the market-hours page, so no source dates its onset: it
enters only at the knowledge-bound row below, and the dated profiles before
it carry no Pre-Open.

The dated grid from launch: the sourced trading session, no Pre-Open.

FairX, as CDE then traded, opened for trading on Monday 2021-06-28 at 09:00
ET with no Sunday-evening session before it. The operator's homepage banner
names the day and time; its captures from 2021-06-22 fix the year, and the
2021-08-02 capture reports the venue open. The launch-day profile starts
that first session at the launch instant, so its bounds never reach back to
a Sunday 17:00 that did not trade. The Monday-evening session that runs into
the next day is identical in both profiles, so the day-level switch to the
full grid never splits it.
<https://web.archive.org/web/20210622222253/https://www.fairx.com/>
<https://web.archive.org/web/20210802230534/https://www.fairx.com/>

2021-06-28 13:00:00 UTC, 08:00 CDT. An exact instant is required; this
launch is not a venue-local-midnight revision.

Knowledge-bound row, dated at the UTC date of the review that verified the
16:50 Pre-Open (LAW-UTC-DATES). It adds only that queue, makes no onset
claim, never moves forward, and a sourced onset day replaces it. One
consequence is visible in `tests/golden/normal_week_grids.txt`, whose
2026-08-22 fixture instant renders the dated grid without the Pre-Open.
