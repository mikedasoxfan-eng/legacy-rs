# Minecraft 1.1 verified progress

**25.89% verified complete: 567 / 2,190 normalized changed lines; 60 / 144 classes.**

This uses the repository's r10 changed-line weighting, with the original 1.0-to-1.1 denominator unchanged. Each credited class has a review covering its complete 1.1 delta, a committed implementation, explicit original-bytecode check coverage and passing recorded results pinned to source hashes. Declaration-only changes receive r10's no-behavior-change credit. Partially verified and unreviewed classes receive zero credit, even when substantial implementation exists. This is a conservative verified-completion measure, not a code-written estimate or a release claim. The earlier 70% estimate is withdrawn.

Recalculate from the repository root:

```powershell
python tools/r11/progress.py
```

The calculator reads `changes.json`, `completion-review.json`, `coverage.json`, `ported.json`, `sections.json`, and `source-snapshots.json` under `tools/r11`. Changing covered source or evidence hashes invalidates its saved credit. Detailed hunk reviews and check transcripts are under `reports/r11-progress`. The broader raw source audit has 308 reviewed classes in the central scope ledger; normalization can conceal changes, so that review remains a release gate.

| Area | Verified lines | Total lines | Completed classes |
|---|---:|---:|---:|
| GUI | 24 | 591 | 6 / 31 |
| World generation | 202 | 302 | 19 / 21 |
| Blocks and items | 199 | 308 | 21 / 34 |
| World and saves | 50 | 68 | 5 / 7 |
| Entities | 83 | 779 | 7 / 34 |
| Player | 8 | 32 | 1 / 4 |
| Rendering | 1 | 110 | 1 / 13 |

Fresh checks for this accounting update: 36 gameplay tests passed serially; all 37 regenerated original-bytecode fixture/table files matched; three world-type/save streams matched; five complete options.txt language cases matched; five original biome comparisons matched. Population, complete layer graph/fixed-manager/raw-flat, spawn and depth-fog comparisons were rerun on the current source. Six layer-neighborhood streams additionally match 619,920 output cells; nine dimension-save sections match native and both crossload directions. Direct original packed-normal comparisons match 11,331 vectors. These finite inputs establish the documented coverage; they are not all-input formal proofs.

Compared with the previous measured checkpoint, the additional 125 credited lines are three layer classes (96), WorldInfo and WorldProviderHell (16), seven gameplay classes (12), and Tessellator (1). The denominator is unchanged. Source and evidence hash pins were refreshed only after the relevant current-source comparisons passed. Implementations are committed in 5c9741d, 00abcf3, 31835cf and fbe6cf4.

Font/Bidi/wrapping, shared sign font state and language callbacks have extensive passing comparisons but retain unresolved integration gates, so their incomplete classes earn no credit yet. Other remaining gates include flat-provider interface coverage, stronghold diagnostic output, sheep animation/status handling, spawn-egg rendering, world/dimension/controller workflows, final stable-candidate 1.1 verification, older-version regressions, CI, documentation and packaging. Unreachable new AI internals remain uncredited pending a recorded scope disposition. The diagnostic broad suite spanning source changes is not used for completion credit.
