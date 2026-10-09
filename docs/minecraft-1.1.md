# Minecraft 1.1

legacy-rs 3.1.0 adds Minecraft 1.1 alongside the three existing versions. Start with `beta.exe --mc 1.1`, or choose Minecraft 1.1 in the version picker. The client reads `versions\1.1\1.1.jar` from your Minecraft installation and stores saves, options, statistics and screenshots under `1.1\`. Plain launches use the executable folder; commands with additional options use the caller's working directory.

## Changes from 1.0

- Superflat world selection, world-type persistence and the corresponding terrain behavior.
- Creative spawn eggs and their item rendering and interaction behavior.
- Language selection, translated screens and tooltips, Unicode font pages and bidirectional text.
- Version-specific gameplay changes, including golden apple crafting, entity/player interactions and sheep behavior.
- World generation, biome/color inputs, fog and rendering adjustments from the original 1.1 code.

The simulation is in `crates/worldgen11`; the client is in `crates/client11`. The executable entry point and version picker are shared with the older versions. Each version keeps its own default data directory.

## Verification and limitations

The original-code audit covers the normalized 1.0-to-1.1 change map and separately resolves raw changes hidden by identifier normalization. Retained corpora exercise text rendering, menu callbacks, potion names, world-name NBT and actual native save operations. Read the [verification document](minecraft-1.1-verification.md) for results and reproduction prerequisites, and the [release report](../reports/releases/v3.1.0.md) for package checks.

The project supports singleplayer. Finite rendering comparisons use the repository's documented driver-rounding allowances; their results do not establish identical output for every GPU and frame. No original game assets or oracle fixtures extracted from an installed game are included in the download.
