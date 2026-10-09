# Verification reports

Reports retain the source reviews and measured comparisons behind release claims. Read [verification methodology](../docs/minecraft-1.1-verification.md) and [development prerequisites](../docs/development.md) before interpreting or reproducing them.

## Release reports

- [legacy-rs v3.1.0](releases/v3.1.0.md): four-version Windows package, source provenance, artifact hashes and headless checks.
- [Changelog](../CHANGELOG.md): historical release results and fixes.

## Minecraft 1.1 evidence

| Record | Purpose |
|---|---|
| [Final validation](r11-progress/final-validation.json) | Runtime candidate, source/report hash pins, gates and historical binary hash |
| [Full production log](r11-progress/final-production11.log) | Client/simulation tests, regressions and corrected package checks |
| [Raw-only class review](r11-progress/raw-only11-completion-review.json) | Dispositions for 308 raw-only changes |
| [Binding audit](r11-progress/raw-bindings11.json) | Renamed callsite and receiver resolution |
| [Raw source screening](r11-progress/raw-source-screen11.json) | Changes outside the normalized class denominator |
| [Parent final review](r11-progress/parent-final11-review.json) | Final source/evidence integration |
| [World-name NBT log](r11-progress/worldname-utf16-11-repro.log) | Original modified UTF-8 and UTF-16 cases |
| [Potion names log](r11-progress/potion-names11-repro.log) | Original potion metadata and locale corpus |
| [Unicode world lifecycle](r11-progress/unicode-world-lifecycle11.log) | Native save, reopen, list, rename and delete |
| [Final GUI reproduction](r11-progress/remaining-gui11-final-repro.log) | Original world-creation and tooltip callbacks |
| [Remote CI status](r11-progress/remote-ci-status.json) | GitHub billing/spending-limit runner block |

`r11-progress/` keeps its historical name to preserve registered paths and evidence hashes. Intermediate checkpoint files are historical measurements; the final validation and release report identify which results apply to the release.

## Earlier versions

Beta 1.8.1 subsystem reports are indexed in [its version document](../docs/beta-1.8.1.md), including [terrain](../docs/b181-terrain-report.md), [world and blocks](../docs/b181-world-blocks-report.md) and [entities](../docs/entities181-report.md). Other files in this directory retain earlier integration rounds. Minecraft 1.0 release verification is summarized in [its version document](../docs/minecraft-1.0.md) and the changelog.

A historical full-suite result and a later library regression run have different coverage. Reports preserve that distinction, along with source candidates, fixture prerequisites and rendering tolerances.
