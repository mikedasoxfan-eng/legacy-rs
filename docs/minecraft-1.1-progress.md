# Minecraft 1.1 verified progress

**70.04566% verified complete: 1534 / 2,190 normalized changed lines; 93 / 144 classes.**

Runtime checkpoint: `5c8a79e`. This uses the repository's r10 normalized changed-line weighting with the original denominator unchanged. A class earns credit only after its entire singleplayer 1.1 delta has a committed implementation or recorded semantic disposition, a full hunk review, explicit original-bytecode comparisons, and source/evidence hash pins. Partial classes receive zero credit. Recalculate with `python tools/r11/progress.py`. Subsequent source edits invalidate the affected saved credit. Text hash pins normalize CRLF to LF, matching Git storage; binary bytes remain exact.

This is verified delta completion, not release readiness or a claim that every inherited Java API has been reproduced. The earlier unsupported 70% estimate remains withdrawn; this new figure comes from the committed class ledger and direct evidence.

| Area | Verified lines | Total lines | Completed classes |
|---|---:|---:|---:|
| gui | 132 | 591 | 10 / 31 |
| worldgen | 302 | 302 | 21 / 21 |
| blocks | 217 | 308 | 23 / 34 |
| world | 50 | 68 | 5 / 7 |
| entities | 755 | 779 | 26 / 34 |
| player | 12 | 32 | 2 / 4 |
| rendering | 66 | 110 | 6 / 13 |

New evidence closes flat World caller interfaces, stronghold stdout, redstone wire, sheep/status/model integration, squid world guards, tagged inventory callbacks, language drawing/actions/save/reopen, brewing GUI progress, generic RenderItem passes and burning-entity light. The original dormant AI guards are constant false: a whole-jar/source reference audit plus constructor/static-effect and runtime comparisons records their singleplayer semantic disposition. These helpers receive credit for equivalent scoped runtime effects; manually invoking their Java facade algorithms is not claimed implemented. Their original weights remain in the denominator.

Stable-checkpoint checks passed:47 gameplay tests and48 original fixture/table regenerations;34 ordinary client tests; world types, flat population, spawn repair/reopen, full layer/biome graph, layer neighborhoods, dimension saves/native crossloads, sign/font shared state, real flat callers and stronghold diagnostics. Additional direct renderer comparisons cover752 inventory/dropped cases,185 held primitive cases,720 burning-light cases,504 creative/dimension fog cases and480 sheep live-render cases. String formatting matches6976 original String-argument vectors. Language workflow covers1008 list draws,126 frames and18 save/reopens, plus168 callback selections.

Remaining gates include FontRenderer/StringTranslate full class reviews, player/controller/world workflows, held player/first-person integration, remaining GUI/block/entity deltas, conservative raw source audit, a final stable-candidate 1.1 suite, older-version regressions, CI and packaging. The diagnostic broad suite spanning source changes is not used as completion credit. Full frame pixel equivalence and arbitrary dormant Java API parity are not claimed by finite callback comparisons.
