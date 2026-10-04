<div align="center">

# beta-rs

**Minecraft Beta 1.7.3, rewritten in Rust, proven identical to the original.**

[![Minecraft](https://img.shields.io/badge/Minecraft-Beta_1.7.3-6aa84f?style=flat&labelColor=3b3b3b)](#)
[![Rust](https://img.shields.io/badge/built_with-Rust-b7410e?style=flat&logo=rust&logoColor=white&labelColor=3b3b3b)](#)
[![wgpu](https://img.shields.io/badge/graphics-wgpu-4b6ec2?style=flat&labelColor=3b3b3b)](#)
[![oracle](https://img.shields.io/badge/oracle-ALL_CASES_EXACT-2ea44f?style=flat&logo=openjdk&logoColor=white&labelColor=3b3b3b)](#how-exact-is-exact)
[![GUI](https://img.shields.io/badge/GUI_pixel_diff-23_exact,_3_within_1_shade-2ea44f?style=flat&labelColor=3b3b3b)](#the-gui-pixel-diff)

[![progress](https://img.shields.io/badge/goal_progress-76%25-8fbc3a?style=flat&labelColor=3b3b3b)](#progress)
[![verified sections](https://img.shields.io/badge/verified_sections-149-2ea44f?style=flat&labelColor=3b3b3b)](#what-is-checked)
[![Rust lines](https://img.shields.io/badge/Rust-45k_lines-b7410e?style=flat&labelColor=3b3b3b)](#layout)
[![Mojang assets](https://img.shields.io/badge/Mojang_assets_bundled-0_bytes-2ea44f?style=flat&labelColor=3b3b3b)](#you-need-your-own-copy)
[![multiplayer](https://img.shields.io/badge/multiplayer-not_planned-9e9e9e?style=flat&labelColor=3b3b3b)](#scope)
[![Herobrine](https://img.shields.io/badge/Herobrine-not_found-a33?style=flat&labelColor=3b3b3b)](#)

</div>

---

## Progress

```
Overall  ██████████████████████████████▍░░░░░░░░░  76 %
```

Weighted by how much of the game each part is. Only merged work counts; parts other agents
are still building are counted at what is already in. "Verified" means checked bit for bit
against the original game code, not just "looks right".

| Area | Weight | Done | Status |
|---|---:|---:|---|
| World generation (Overworld + Nether: terrain, caves, ores, lakes, dungeons, trees, plants, snow) | 12 | 100 % | ✅ verified |
| Lighting, chunk loading/unloading, saving (McRegion, level.dat) | 8 | 100 % | ✅ verified |
| World ticking (block ticks, random ticks, fire, liquids, growth, weather, lightning) | 8 | 95 % | ✅ verified |
| Mobs and other entities (physics, AI, spawning, items, arrows, TNT, NBT) | 12 | 90 % | ✅ verified (minecarts and boats missing) |
| The player (movement, health, mining, placing, items, beds, portals, respawn) | 10 | 95 % | ✅ verified |
| Block rendering (all 18 render types, smooth lighting, Fancy/Fast) | 8 | 100 % | ✅ verified |
| Mob/item/effect rendering, sky, clouds, rain, snow, fog | 8 | 95 % | ✅ verified |
| Menus, HUD, achievements, death screen | 7 | 90 % | ✅ pixel-exact (statistics screen missing) |
| Redstone (wire, torches, repeaters, levers, buttons, plates, doors, rails, pistons, notes) | 6 | 100 % | ✅ verified |
| Inventory, crafting, chests, furnaces, dispensers | 7 | 15 % | 🟡 inventory simulation done; screens and crafting in progress |
| First-person view (hand, held item, block outline, cracks, bobbing, third person) | 5 | 0 % | 🟡 in progress |
| Particles and world sounds | 5 | 10 % | 🟡 in progress (music, rain and menu sounds work) |
| Tile entities that tick (mob spawners, furnaces, pistons) | 2 | 100 % | ✅ lifecycle, ticking and NBT verified |
| Vehicles, signs, paintings, maps, fishing, records, statistics | 2 | 0 % | 🔴 not started |

---

## What this is

A singleplayer-only rewrite of the Minecraft Beta 1.7.3 client (July 2011) in Rust, with
[wgpu](https://wgpu.rs) for graphics. It aims to be an **exact copy**, not a remake:

- the same world from the same seed, block for block, including every ore vein, cave,
  dungeon chest and lighting value;
- the same mobs doing the same things on the same ticks;
- the same screens, pixel for pixel;
- the same bugs. If the original does something odd, so does this (see
  [Quirks kept on purpose](#quirks-kept-on-purpose)).

Worlds are saved in the original format, so you can copy a world between this and the real
game in either direction.

### Scope

Singleplayer only, Windows, one user. Multiplayer, the server, texture-pack downloading,
the launcher and online skins are out of scope.

---

## How exact is exact?

Every system is a line-by-line port of the decompiled original client, and none of it is
taken on trust. The repository includes a test harness that runs **the original game code
itself** next to the port as a reference (the "oracle") and compares results bit by bit.
That includes every float and double, every random number drawn, and every block, light
value and metadata nibble.

Details that turned out to matter:

- **The exact Java version.** `Math.sin` and friends give different last bits on different
  JVMs. The oracle refuses to run on anything but the Java Mojang's launcher shipped for
  b1.7.3 (**Java 8u51**), because that is the one the real game uses.
- **Pinned randomness.** Vanilla seeds several things from the clock (`World.rand`, the
  random-tick generator, the cave-sound counter, every entity's `Random`, `Math.random`).
  The harness pins all of them on both sides, so whole play sessions can be replayed.
- **Iteration order.** Some vanilla code walks a Java `HashSet` or `HashMap`, e.g. random ticks over
  loaded chunks or redstone neighbour updates. The order depends on Java 8's bucket layout,
  including red-black tree bins, so `world/java_hash.rs` reimplements it.
- **Float vs double.** Java mixes `float` and `double` arithmetic in specific places; the port
  keeps the exact same casts.

### What is checked

`tools/verify.sh` (149 sections; every one must say `EXACT`):

| Category | What is compared |
|---|---|
| Raw terrain | Chunk blocks for many seeds, Overworld and Nether, before population |
| New worlds | The vanilla new-world flow: spawn search, 289-chunk preload, population, lighting |
| Grids | Chunk loading orders with lighting after each chunk / at the end / never |
| Saves | Rust writes → Java reads and Java writes → Rust reads, region files and level.dat |
| Server mode | A 900-tick walk with the real `World.tick`, chunk unload/reload through disk |
| Ticks | Rain, thunder, snow and ice forming, a forest fire, saplings growing over 4 days |
| Entities | 20 frame-by-frame replays: mobs, items, arrows, TNT, falling sand, spawning |
| Player | 23 replays: walking, swimming, damage, death, digging, placing, items, beds, portals |
| Inventory | 10,000 original-bytecode inventory operations |
| Collision | Every block's collision and selection box in every state (10,000 cases) |
| Portals | Teleporter find/build (incl. the ocean platform), the whole `usePortal` sequence |
| Respawn | At spawn, at a bed, with the bed gone |
| Redstone | Scripted circuits, rail layouts, all piston facings and push lengths, sticky pulls, pulses, clocks, entity pushing, note instruments and pitches |
| Moving pistons | 24,840 original-bytecode renderer cases: partial ticks, all facings, stored blocks, head/base branches, Fancy/Fast and smooth lighting |
| Tile lifecycle | Object identity, invalidation during scanning, deferred insertion, lazy creation, chunk unload/reload and NBT |
| Chunk meshes | Every vertex position, UV and colour of real terrain, Fancy/Fast, smooth lighting on/off |
| Block showcase | Every block id × all 16 metadata values plus neighbour arrangements, all four graphics modes |
| Models & renderers | Mob geometry, animation poses, full renderer draw calls, shadows, fire |
| Sky | Sky, fog and cloud colours across times, weather and render distances, and in the Nether |
| Weather | Rain/snow quads, splash particles, lightning bolt strips |

### The GUI pixel diff

`tools/verify_gui.sh` opens the **real** b1.7.3 client for a few seconds, drives it through
26 scenes (title screen, options, video settings, controls, world list, create and rename
world, texture packs, loading screens, in-game HUD states, achievements, death screen), and
takes screenshots. Then it renders the same scenes with the port and diffs them pixel for pixel.
23 are identical; 3 (achievement icons) differ by one colour level on fewer than 100 pixels,
which is GPU rounding in item lighting.

Getting there needed a few fixes:
- **Forced anti-aliasing:** the NVIDIA driver forces 4× anti-aliasing on OpenGL apps, so the
  real client draws into an offscreen framebuffer.
- **Rasterization order:** the port rasterizes the GUI bottom-up, like OpenGL, so linear
  filtering rounds the same way.

---

## You need your own copy

**No Mojang code, textures, sounds or fonts are in this repository.** At runtime the game
reads them from your own installation:

- `%APPDATA%\.minecraft\versions\b1.7.3\b1.7.3.jar`: textures, fonts, language files
- `%APPDATA%\.minecraft\resources\`: sounds and music

The verification harness additionally needs the original client decompiled with
[RetroMCP](https://github.com/MCPHackers/RetroMCP-Java) into `tools/retromcp` (ignored by
git), and Mojang's Java 8u51 runtime, which the official launcher installs.

---

## Playing

```
cargo build --release -p client
target\release\beta.exe
```

or double-click `play.bat`. The game starts at the title screen: **Singleplayer → Create New
World**, exactly like the original. Worlds go to `saves\` next to the exe.

### Controls

Same as the original's defaults, and rebindable in Options → Controls:

| Key | Action |
|---|---|
| W A S D | Move |
| Space / Left Shift | Jump / sneak |
| Mouse | Look (sensitivity and invert from Options) |
| Left / right click | Mine / place or use |
| Middle click | Pick block |
| 1–9, mouse wheel | Hotbar slot |
| Q | Drop item |
| F | Cycle render distance (Shift: backwards) |
| F1 / F2 / F3 | Hide HUD / screenshot / debug info |
| F5 / F8 | Third person (view not drawn yet) / smooth camera |
| Esc | Game menu (pauses and saves) |

### Command line

| Flag | Meaning |
|---|---|
| `--seed <n or text>` | Create and enter a world with this seed directly |
| `--world <name>` / `--world-dir <path>` | Enter a named world / any save folder, including a real game save |
| `--no-save` | Play without writing anything to disk |
| `--time <ticks>`, `--weather ...` | Start at a time of day / with rain or thunder |
| `--server --view-distance n` | Use the dedicated server's chunk loading instead of the client's |
| `--shot <file.png> [--shot-ticks n]` | Take a screenshot after the world has run `n` ticks and exit |
| `--play-script <file.txt>` | Drive the player from a script (keys, mouse, screenshots) for unattended tests |

---

## Layout

```
crates/
  worldgen/           the simulation (25k lines)
    overworld.rs      terrain noise, biomes, surface
    caves.rs          the cave carver
    populate.rs       ores, lakes, dungeons, trees, flowers, snow ...
    features.rs       the feature generators (trees incl. big oaks, lakes, liquids ...)
    nether/           Nether terrain and population
    world/            World: blocks, light, chunk storage, ticks, weather,
                      redstone.rs, rails.rs, teleporter.rs, dimension.rs, java_hash.rs
    entity/           Entity physics, living entities, every mob's AI, spawning, items,
                      projectiles, NBT; player/ (inventory, controller, digging,
                      placement, items, beds, persistence)
    storage.rs        McRegion region files, level.dat, NBT
    bin/compare.rs    the Rust side of every oracle comparison
  client/             the window (20k lines)
    app.rs            Minecraft's main loop: screens, input, camera, HUD wiring
    worker.rs         the world thread: ticks, meshing, entity snapshots, saving
    render/           blocks.rs + blocks_special.rs (RenderBlocks), sky, weather,
                      video settings, entity/ (models, renderers, item icons)
    gui/              a fixed-function GL emulation so screens port line by line;
                      screens/ (title, options, worlds, game menu, achievements, game over),
                      ingame.rs (HUD), font, language files, settings
    sound.rs          SoundManager / SoundPool semantics over cpal
tools/
  verify.sh           the full bit-exact comparison (quick mode: `verify.sh quick`)
  verify_gui.sh       the GUI screenshot diff against the real client
  oracle/             the reference harness: Java programs that call the original
                      classes, plus scene/scripts (showcase, redstone, GUI scenes)
```

The game runs on two threads, like the original's split between the game loop and the
renderer:
- **The world thread** owns the `World`. It ticks at 20 Hz, builds chunk meshes and sends snapshots.
- **The window thread** handles input, screens, sound and drawing.

Player input crosses over as per-frame messages. Mouse look is applied between ticks, as
`EntityRenderer` does in the original.

---

## Quirks kept on purpose

Being exact means keeping these too:

- **Portal lift:** going through a portal re-adds the player's eye height to their position twice
  (`setLocationAndAngles(x, posY, z)` adds `yOffset` again). You arrive a little higher.
- **Misnamed bed check:** `BlockBed.isBlockFootOfBed` is true for the *head* half. Bed rendering depends on it.
- **Slab tops:** double slabs always draw their top face, even when covered.
- **Pick block on mobs:** middle-clicking a mob picks the block at 0,0,0 (usually bedrock → stone).
- **Redstone update order:** wire updates neighbours in Java `HashSet` order, which decides the order
  things switch on.
- **Torch burnout:** a redstone torch that flips 8 times in 100 ticks burns out, sharing one list
  across all torches in the game.
- **Piston save/load:** moving pistons save the previous progress and omit the render-head flag.
- **Sticky quick pulses:** retracting before an extending tile finishes can leave its block behind.
- **Tile identity:** replacing a chunk-map entry leaves the old object in the update list; invalid
  cleanup can remove a newer object at the same position.
- **Leftover block bounds:** a block's size is shared state; a renderer that changes it and doesn't
  reset it leaves it for the next block of that kind.
- **Fog in the Nether:** if it was raining in the Overworld when you left, the Nether fog is darker.

---

## Development notes

- `tools/verify.sh quick` runs a fast subset; the full run takes a while. Every change has to end
  with `ALL CASES EXACT`.
- `beta --play-script` plus `--no-save --seed` gives repeatable gameplay screenshots without
  anyone at the keyboard.
- `tools/oracle/make_showcase.py` regenerates the block showcase;
  `tools/oracle/redstone/*.txt` are circuit scripts (`place`, `remove`, `use`, `tick`,
  `check`), run by both the original and the port.
- [Piston and tile oracle coverage](tools/oracle/PISTONS.md) documents the extended script commands
  and the moving-piston renderer check.
- Large parts were ported in parallel by AI coding agents (Claude and Codex), each given one
  subsystem and the same rule: port literally, prove it against the original, keep every
  existing check exact.

---

## Not yet

Minecarts and boats (placing them works; riding and physics don't), signs, paintings, maps,
fishing, records, the statistics screen, and the 3D anaglyph mode.

---

<sub>Unofficial fan project for personal use. Not affiliated with or endorsed by Mojang or
Microsoft. Minecraft is a trademark of Mojang Synergies AB.</sub>
