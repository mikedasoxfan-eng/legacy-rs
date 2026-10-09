# Changelog

All notable changes to legacy-rs (formerly beta-rs). Versions follow [Semantic Versioning](https://semver.org);
"exact" always means checked bit for bit against the original bytecode of that version.

## [3.1.0] - 2026-10-09

### Added
- Minecraft **1.1** in the version picker and `--mc 1.1`, with its own `1.1/` data directory.
- Superflat world selection and persistence, spawn eggs, language selection, Unicode and bidirectional text, and the original 1.1 gameplay/world/rendering changes.
- Completed original-code audit of the 1.0-to-1.1 delta, including raw-only class changes and renamed callsite binding checks.
- Playing, building, version and verification documentation, with an indexed reports directory and a dedicated release verification report.

### Changed
- Rewrote the README for all four supported versions and current downloads; removed development progress badges and tables.
- Published a Windows ZIP alongside the standalone executable and offline inventory editor, with SHA-256 checksums.

### Verified
- Retained 1.1 production client suite: 67 tests; simulation: 114 tests; cross-version workspace libraries: 249 tests.
- Original potion names/tooltips: 159,452 calls; world-name NBT: 65,550 cases; Unicode world creation: 96 callbacks; achievement tooltips: 3,021 callbacks.
- Release packaging checks and evidence provenance are recorded in [the v3.1.0 report](reports/releases/v3.1.0.md). Older releases' historical full oracle suites remain recorded separately.

## [3.0.2] - 2026-10-06

Minecraft 1.0 fixes found by playing through the new features in the real client: new end-to-end
play tests set up a situation (`--setup`) and play it with a script, in the actual game.

### Fixed
- The End was empty and frozen: drawing the Ender Dragon crashed the game's world thread. The same
  crash happened when throwing an eye of ender or an ender pearl, or near a magma cube.
- Killing the dragon now builds the exit portal (its portal blocks were removed as soon as they were
  placed) and drops the dragon egg; walking in shows the end poem and credits, then the Overworld.
- Dying in the End respawns you in the Overworld (it sent you to the Nether).
- The End's sky and fog now follow the original End (it used the Overworld's sun angle, biome and fog
  colour); the compass and clock spin there as in the original.
- After changing dimension the game no longer races through catch-up ticks.
- All eleven music discs play.
- Four checks that still differed from the original now match: potion effect expiry, an item icon,
  and aiming a bucket at water.

### Tests
- `tools/verify_playtest10.sh`: end-to-end play tests of the real 1.0 client (End portal, dragon and
  credits, Nether round trip, death in the End, hardcore death, every new item, block, screen and mob).

### Verified
- The complete 1.0 suite on this build: 2,006 sections, every one matching the original (the final rerun 3.0.0 and 3.0.1 still owed). Beta 1.7.3 and Beta 1.8.1: ALL CASES EXACT, screens included.

## [3.0.1] - 2026-10-06

### Fixed
- Minecraft 1.0: stepping into an End portal now takes you to the End (onto the obsidian platform, as in
  the original). The portal raised the right event, but the client never acted on it.

### Added
- `give.exe`, a small separate tool that puts items into a singleplayer world's inventory (with a
  backup), for things the original game has no way to hand you, like End portal frames. New stacks go
  into the hotbar, since 1.0's creative inventory screen shows only the hotbar.

## [3.0.0] - 2026-10-06

The project is now called **legacy-rs** (formerly beta-rs), because it is no longer only Beta. The
program file is still `beta.exe` and every save and options folder stays where it was.

Minecraft 1.0 (the first full release), next to Beta 1.7.3 and Beta 1.8.1 in the same program.

### Versions
- The version picker offers Minecraft 1.0; `beta.exe --mc 1.0` starts it directly. It keeps its own saves,
  options and statistics in `1.0\`. Beta 1.7.3 and Beta 1.8.1 are unchanged
  (apart from the program name in their help text).
- Plain launches keep saves next to the exe, whatever folder they are started from.

### Minecraft 1.0
- Everything 1.0 added or changed in singleplayer: the End and the Ender Dragon, enchanting, brewing
  and potions, Nether fortresses and blazes, magma cubes, mooshrooms, snow golems, villagers, breeding,
  mushroom islands, hardcore mode and the rest. All 16,910 changed lines of the 1.0 classes are
  ported and checked bit for bit against the original bytecode (761 focused oracle sections, all exact).
- The final rerun of the whole 1.0 suite (about 1,400 sections, including the older checks carried
  forward from Beta 1.8.1) on the release build, and of the Beta 1.7.3 and 1.8.1 suites, is still to come.

## [2.0.0] - 2026-10-05

Minecraft Beta 1.8.1 (the Adventure Update), next to Beta 1.7.3 in the same program.

### Versions
- Starting `beta.exe` without arguments shows a version picker (Beta 1.7.3 / Beta 1.8.1);
  `beta.exe --mc b1.8.1` starts Beta 1.8.1 directly.
- Beta 1.8.1 keeps its own saves, options, statistics and screenshots in `b1.8.1\`.
  Beta 1.7.3 is unchanged and still checked bit for bit.

### Beta 1.8.1
- Rendering: the lightmap (smooth day/night, torch flicker, the Brightness option), b1.8.1
  smooth lighting and block shapes (vines, stems, panes, fence gates, ...), sky, clouds, fog
  (including the void fog near bedrock), rain and snow, first-person hand with arm sway,
  chests as animated tile entities, lit signs and moving pistons, depth particles.
- Tile entities in play: animated chests, lit signs and moving pistons, the spinning mob inside
  monster spawners; distant mobs are hidden at the original render range.
- Play-tested from the title screen: world creation (survival/creative), the creative
  inventory and flying, sprinting, Nether portals and the Nether.
- Everything else b1.8.1 added or changed in singleplayer: world generation (biomes, rivers,
  villages, strongholds, mineshafts, ravines), entities (Endermen, cave spiders, silverfish, XP
  orbs, potion effects, critical hits), the player (hunger, sprinting, experience, creative),
  blocks and items (vines, stems, melons, panes, fence gates, food, the bow), and the new
  screens. All 20,982 changed lines of the b1.8.1 classes are ported and checked bit
  for bit against the original bytecode (1067 oracle sections).

## [1.0.0] - 2026-10-04

The first complete release: everything in singleplayer Minecraft Beta 1.7.3.

### World
- Overworld and Nether generation: terrain, biomes, caves, ores, lakes, dungeons, trees,
  plants and snow, identical to the original for any seed.
- Lighting, chunk loading and unloading, and saving in the original McRegion format
  (worlds move between beta-rs and the real game in both directions).
- Pre-McRegion (Alpha) saves are converted on load, as the original does.
- World ticking: block ticks, random ticks, fire, liquids, crop and sapling growth, weather,
  lightning, sleeping through the night.
- Redstone: wire, torches (with burnout), repeaters, levers, buttons, pressure plates, doors,
  rails, detector and powered rails, note blocks, normal and sticky pistons.
- Tile entities: chests, furnaces, dispensers, mob spawners, note blocks, jukeboxes, signs,
  moving pistons, with the original object lifecycle.

### Entities and the player
- Every mob with its AI, spawning, items, arrows, TNT, falling blocks, paintings, minecarts
  (plain, chest, furnace) and boats, with NBT.
- The player: movement, swimming, health, damage, death and respawn, mining, placing,
  every usable item, beds, Nether portals, riding, fishing.
- Statistics and achievements counted exactly as the original counts them, saved in the
  original stats file.

### Client
- The original menus, HUD, inventory and container screens, achievements, statistics,
  sign editing, chat and the death screen, pixel for pixel.
- Block, mob, item, vehicle, sign, painting and map rendering; sky, clouds, rain, snow, fog;
  every particle; the first-person hand and item, block outline, mining cracks, view bobbing
  and third person.
- World sounds and music in the original order; jukebox records.
- Video settings including Fancy/Fast, smooth lighting, render distance, the frame-rate limit
  and the 3D anaglyph mode; texture packs with live reloading.
- F2 screenshots, the F3 debug screen with its frame-time graph, the "Taking Inventory" hint,
  the level-save-conflict screen.

### Verification
- `tools/verify.sh`: 165 sections, all `EXACT`.
- `tools/verify_gui.sh`: 100 screen scenes against the real client: 92 identical, 8 one colour
  shade off on 3D block icons (graphics-driver rounding in item lighting).

### Known differences
- 3D block icons on the achievement and statistics screens can be one shade off on a few
  hundred pixels.
