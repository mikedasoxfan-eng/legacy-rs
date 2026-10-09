# Minecraft 1.1 verified completion

**100%: 2,190 / 2,190 normalized changed lines; 144 / 144 classes; 35 / 35 verification sections.**

The denominator and class weighting use the repository's r10 change-map technique. Each class has a committed implementation or documented semantic disposition, complete raw hunk review, explicit original-code evidence and hash pins. The additional conservative audit resolves all 308 raw-only class deltas:305 equivalent original bytecode classes plus 3 explicitly verified/disposed residues; 1,798 renamed callsites and 925 receiver witnesses have no unresolved bindings. Recalculate with `python tools/r11/progress.py`; source/evidence edits invalidate credit.

| Area | Verified lines | Completed classes |
|---|---:|---:|
| gui | 591 / 591 | 31 / 31 |
| worldgen | 302 / 302 | 21 / 21 |
| blocks | 308 / 308 | 34 / 34 |
| world | 68 / 68 | 7 / 7 |
| entities | 779 / 779 | 34 / 34 |
| player | 32 / 32 | 4 / 4 |
| rendering | 110 / 110 | 13 / 13 |

Final runtime candidate: `b4eef854dd54f72ab75af309113ca5e7054635f7`. The full current 1.1 client library passes 67 tests, including all retained original corpora and headless GPU Unicode-font upload; simulation passes 114 tests. Potion names/tooltips match 159,452 original calls. Unicode world names match 65,550 original NBT cases and 96 original creation callbacks, with actual native first-save/reopen/list/rename/delete coverage. Older-version library regressions pass. Release workspace build and all four packaged CLI version dispatches pass. Reproduce with `tools/verify_r11_final.ps1` after preparing the original fixtures using their registered commands.

GitHub CI could not start because GitHub reports failed account payments or a spending-limit block. The updated four-version CI simulation command passed locally. See [recorded CI status](../reports/r11-progress/remote-ci-status.json); this is an external runner limitation, not a claimed successful remote CI run.

Scope follows the repository: singleplayer, installed original resources, and the 1.0-to1.1 delta. Multiplayer networking is excluded. Finite original rendering corpora retain the repository's established GPU rounding tolerance; this does not claim every possible inherited Java API or GPU frame. No Mojang game assets are bundled.
