# Changelog

All notable changes to beta-rs. Versions follow [Semantic Versioning](https://semver.org);
"exact" always means checked bit for bit against the original b1.7.3 bytecode.

## [Unreleased] - 2.0.0

Minecraft Beta 1.8.1 (the Adventure Update), next to Beta 1.7.3 in the same program.

### Versions
- Starting `beta.exe` without arguments shows a version picker (Beta 1.7.3 / Beta 1.8.1);
  `beta.exe --mc b1.8.1` starts Beta 1.8.1 directly.
- Beta 1.8.1 keeps its own saves, options, statistics and screenshots in `b1.8.1\`.
  Beta 1.7.3 is unchanged and still checked bit for bit.

### Beta 1.8.1 (in progress; see the README for the exact verified share)
- Rendering: the lightmap (smooth day/night, torch flicker, the Brightness option), b1.8.1
  smooth lighting and block shapes (vines, stems, panes, fence gates, ...), sky, clouds, fog
  (including the void fog near bedrock), rain and snow, first-person hand with arm sway,
  chests as animated tile entities, lit signs and moving pistons, depth particles.
- Tile entities in play: animated chests, lit signs and moving pistons, the spinning mob inside
  monster spawners; distant mobs are hidden at the original render range.
- Play-tested from the title screen: world creation (survival/creative), the creative
  inventory and flying, sprinting, Nether portals and the Nether.
- World generation, entities, the player (hunger, sprinting, experience, creative), blocks,
  items and screens are being ported area by area; each part counts only once the original
  b1.8.1 bytecode agrees with it bit for bit.

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
