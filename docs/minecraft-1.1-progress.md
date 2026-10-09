# Minecraft 1.1 verified progress

**20.18% verified complete: 442 / 2,190 normalized changed lines; 47 / 144 classes.**

This uses the repository's r10 changed-line weighting, with the original 1.0-to-1.1 denominator unchanged. Each credited class has a review covering its complete 1.1 delta, a committed implementation, explicit original-bytecode check coverage and passing recorded results pinned to source hashes. Declaration-only changes receive r10's no-behavior-change credit. Partially verified and unreviewed classes receive zero credit, even when substantial implementation exists. This is a conservative verified-completion measure, not a code-written estimate or a release claim. The earlier 70% estimate is withdrawn.

Recalculate from the repository root:

```powershell
python tools/r11/progress.py
```

The calculator reads `changes.json`, `completion-review.json`, `coverage.json`, `ported.json`, `sections.json`, and `source-snapshots.json` under `tools/r11`. Changing covered source or evidence hashes invalidates its saved credit. Detailed hunk reviews and check transcripts are under `reports/r11-progress`. The broader raw source audit has 308 reviewed classes in the central scope ledger; normalization can conceal changes, so that review remains a release gate.

| Area | Verified lines | Total lines | Completed classes |
|---|---:|---:|---:|
| GUI | 24 | 591 | 6 / 31 |
| World generation | 106 | 302 | 16 / 21 |
| Blocks and items | 190 | 308 | 16 / 34 |
| World and saves | 34 | 68 | 3 / 7 |
| Entities | 80 | 779 | 5 / 34 |
| Player | 8 | 32 | 1 / 4 |
| Rendering | 0 | 110 | 0 / 13 |

Fresh checks for this accounting update: 29 gameplay tests passed serially; all 30 regenerated original-bytecode fixtures matched; three world-type/save streams matched; five complete options.txt language cases matched; five retained original biome fixture comparisons matched. Prior bounded population, layer, spawn and depth-fog comparisons are retained with per-hunk review and current source hashes. Their finite test inputs establish the documented coverage; they are not all-input formal proofs.

Font/Bidi/wrapping and language callbacks have extensive passing comparisons but retain unresolved integration gates, so their incomplete classes earn no credit yet. Other remaining gates include targeted layer branches, sheep animation/status handling, spawn-egg rendering, world/dimension/controller workflows, final stable-candidate 1.1 verification, older-version regressions, CI, documentation and packaging. Unreachable new AI internals remain uncredited pending a recorded scope disposition. The diagnostic broad suite spanning source changes is not used for completion credit.
