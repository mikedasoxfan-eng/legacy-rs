# Minecraft 1.1 verification

The completed 1.0-to-1.1 audit covers **2,190 normalized changed lines across 144 classes and 35 verification sections**. Each class has an implementation or documented semantic disposition, raw-hunk review, original-code evidence and source/evidence hash pins.

## Audit method

The change map follows the repository's 1.0 technique: compare decompiled versions after normalizing renamed identifiers, then require evidence for every relevant class. A second audit addresses changes that normalization could hide: 308 raw-only classes resolve to 305 bytecode-equivalent classes and three explicitly reviewed residues. The binding review covers 1,798 renamed callsites and 925 receiver witnesses, with no unresolved bindings.

The retained machine records live under `tools/r11` and `reports/r11-progress`. The historical directory name is retained because registered evidence paths and hash pins depend on it. Those files are audit records, not a current development roadmap.

## Runtime results

| Check | Recorded result |
|---|---|
| Complete retained 1.1 client library, including ignored fixture tests | 67 passed |
| 1.1 simulation library | 114 passed |
| Potion names and tooltips | 159,452 original calls matched |
| World-name NBT, including unmatched UTF-16 surrogates | 65,550 original cases matched |
| Unicode world-creation callbacks | 96 original callbacks matched |
| Achievement tooltips | 3,021 original draw callbacks; 57 locales; 26 Greek formatter exceptions matched |
| Native world lifecycle | First save, reopen, list, rename and delete checked |
| Cross-version workspace libraries | 249 tests passed |
| Package | Release build and all four CLI dispatches passed |

The full runtime evidence was collected for candidate `b4eef854dd54f72ab75af309113ca5e7054635f7`. [Final validation](../reports/r11-progress/final-validation.json) pins its source files, reports and then-built binary. The [v3.1.0 release report](../reports/releases/v3.1.0.md) records the subsequent metadata-only version bump and new package hashes; the historical binary hash remains attached to its original validation.

## Reproduction

See [development prerequisites](development.md). Prepare the original corpora with the commands registered in `tools/r11/sections.json` and its linked evidence records. Then run `tools/verify_r11_final.ps1` from PowerShell with Rust, the Java oracle prerequisites and original resources available. `-PackageOnly` rebuilds the workspace and checks command-line dispatch without launching a game window.

`python tools/r11/verify_pins11.py` validates source/evidence hashes against the Git index. The legacy `python tools/r11/progress.py` command recalculates the class ledger and validates completion evidence; it is retained for audit reproducibility.

## Boundaries

The result covers the repository's singleplayer scope and the audited version delta. Multiplayer networking is excluded. Original rendering comparisons use finite corpora and established graphics-driver rounding tolerances; they do not establish equivalence for every inherited Java API path or possible GPU frame.

GitHub Actions could not start the recorded validation run because GitHub reported an account payment or spending-limit block. The configured four-version simulation command passed locally. [Recorded runner status](../reports/r11-progress/remote-ci-status.json) documents the external limitation; remote CI is not claimed as passed.
