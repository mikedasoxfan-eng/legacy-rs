# Minecraft 1.1 checkpoint and progress accounting

The 1.1 port is unfinished. This checkpoint saves the isolated client11/worldgen11 crates, dispatcher, oracle tooling and focused changes; it is not a release or a claim of full-suite parity.

The existing r10 technique weights added/changed classes by normalized changed lines and credits complete classes only when their covering oracle sections are EXACT. The 1.0→1.1 normalized inventory has144classes and2190weightedlines. The conservative inventory adds candidates that normalization hides; that broader review remains mandatory.

Reproduce the current recorded percentage:

```powershell
python tools/r11/progress.py
```

At this checkpoint the registered whole-class completion credit is0/2190lines,0/144classes,0.0%. `coverage.json` and `ported.json` are empty deliberately: bounded focused comparisons do not establish entire-class completion. This is a certification/accounting number, not the fraction of code written. The earlier70% chat estimate was not calculated with this method.

Validation already obtained includes workspace compilation; focused original-bytecode biome/populated-grid/structure, worldtype/save/spawn, gameplay, font draw and language checks. The Bidi production implementation matches233985cases; original public font drawing matches1792cases; language callback/button comparison matches168selections at3resolutions. Original inputs and generated builds remain outside Git.

Remaining gates include completing per-class behavior audits and explicit coverage registration; text wrapping/formatting; rendered language screens and startup/restart; sheep animation and spawn-egg rendering; complete world/dimension/controller workflows; final stable-candidate1.1suite; older-version regressions; CI/documentation/package updates. The ongoing broad run spans source changes and is diagnostic only.

Local checkpoint commits preserve work. No remote publishing or release is implied.
