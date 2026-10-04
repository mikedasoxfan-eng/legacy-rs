# beta-rs

Minecraft Beta 1.7.3, singleplayer, rewritten in Rust (wgpu for graphics), aiming to be an
exact copy of the original: the same worlds from the same seeds, the same mob behaviour, the
same physics, the same screens pixel for pixel.

Nothing here is guessed. Every part is a line-by-line port of the original client, and
`tools/verify.sh` proves it: the original game code runs next to the port as a reference
("oracle"), on the same Java version Mojang shipped it with (8u51), and the results are
compared bit for bit — world generation, lighting, block ticks, weather, mobs, the player,
redstone, portals, chunk meshes and vertex colours, sky colours, saves. `tools/verify_gui.sh`
does the same for the menus and HUD, screenshot against screenshot.

## You need your own copy of the game

No Mojang code or assets are included. The game reads textures, sounds and fonts from your
own installation (`%APPDATA%\.minecraft\versions\b1.7.3\b1.7.3.jar` and
`%APPDATA%\.minecraft\resources`). The reference checks additionally need the original
client decompiled with RetroMCP under `tools/retromcp` and Mojang's Java 8u51.

## Play

```
cargo build --release -p client
target\release\beta.exe
```

or `play.bat`. Worlds are saved in `saves\` next to where it runs, in the original format.

## Layout

- `crates/worldgen` — the simulation: terrain and population (Overworld and Nether), lighting,
  saves, block and world ticks, redstone and rails, entities and mobs, the player, portals.
- `crates/client` — the window: renderer (blocks, mobs, sky, weather), the GUI (title screen,
  options, world list, pause menu, HUD, achievements), sound, input.
- `tools/oracle` — the reference harness around the original classes.

## Status

Working and verified: terrain, caves, ores, structures, the Nether, lighting, saves and chunk
loading, weather, every mob, the player (movement, health, mining, placing, items, beds),
portals and respawning, block rendering for every block, menus and HUD.

In progress: inventory and crafting screens, the first-person hand and held item, particles
and world sounds, pistons and mob spawners. Not yet: minecarts and boats, signs, paintings,
maps, fishing, records, statistics.

Unofficial; not affiliated with Mojang or Microsoft.
