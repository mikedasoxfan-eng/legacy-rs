# Beta 1.8.1

beta-rs 2.0.0 adds Minecraft Beta 1.8.1 (the Adventure Update, September 2011) next to
Beta 1.7.3. Both live in the same program and share nothing at run time: each version has
its own simulation crate, its own client and its own oracle, so a change for one can never
move the other. Beta 1.7.3 is still checked by `tools/verify.sh` after every merge.

## Running it

`beta.exe` asks which version to play. `beta.exe --mc b1.8.1` starts Beta 1.8.1 directly,
`--mc b1.7.3` the older one. Beta 1.8.1 reads its textures, sounds and fonts from the
launcher's `old_beta b1.8.1` installation, and keeps everything it writes in `b1.8.1\`
next to the exe:

| Path | What |
|---|---|
| `b1.8.1\saves\` | worlds, in the original b1.8.1 format (McRegion, level.dat with game type and structures) |
| `b1.8.1\options.txt` | options, including the new Brightness slider and key bindings |
| `b1.8.1\stats\` | statistics and achievements, in the original stats file |
| `b1.8.1\screenshots\` | F2 screenshots |

## Layout

```
crates/
  worldgen181/        the b1.8.1 simulation (library still named `worldgen`), ported forward
                      from crates/worldgen change by change: biomes, rivers, structures
                      (villages, strongholds, mineshafts), ravines, the new blocks, hunger,
                      experience, creative mode, Endermen, cave spiders, silverfish,
                      potion effects, critical hits
  client181/          the b1.8.1 client (a library that beta.exe runs for --mc b1.8.1):
                      the lightmap, the new block shapes, the new HUD and screens,
                      the creative inventory, the title panorama
tools/
  verify181.sh        the full b1.8.1 comparison (replays run in parallel; VERIFY181_JOBS)
  verify_*181.sh      focused suites per area, each section reported separately
  oracle181/          the reference harness for the original b1.8.1 bytecode (Mode_*.java)
  b181/               progress tooling: changes.json (every class b1.8.1 added or
                      changed), coverage.json (which checks cover each class),
                      progress.py (the README progress block)
```

## How "done" is counted

`tools/b181/changes.json` lists every class that b1.8.1 added or changed compared with
b1.7.3, with the number of changed lines. A class counts as done only when its port is
merged **and** every oracle section that covers it (`tools/b181/coverage.json`) is EXACT in
the latest run. The README's progress block is computed from those two files by
`tools/b181/progress.py`; nothing in it is estimated.

Not counted, as for Beta 1.7.3:
- multiplayer-only classes (`tools/b181/out_of_scope.json`);
- classes whose b1.8.1 change is only renamed identifiers
  (`tools/b181/no_behaviour_change.json`).

Window and startup code is verified by play-testing, as for Beta 1.7.3: in the `Minecraft`
class that is `startGame`, `loadScreen`, `run`, `main` and the loading-screen text, which draw
the window and have no effect on the game. Everything it does to the game (the `runTick` input
loop, clicks, screens and focus, `preloadWorld`, controllers, world creation and selection, the
debug entity counts) is compared with the original in `tools/verify_minecraft181.sh`. Its
`ChunkProviderLoadOrGenerate` branches never run in Beta 1.8.1 singleplayer, whose `World`
uses `ChunkProvider`.

## Detailed reports

The port was built area by area; each area's report lists every class, what was ported and
the exact check that covers it.

| Report | Area |
|---|---|
| [b181-terrain-report.md](b181-terrain-report.md) | World generation: biomes, rivers, villages, strongholds, mineshafts, ravines |
| [b181-world-blocks-report.md](b181-world-blocks-report.md) | Blocks, world ticking, chests, saves, redstone |
| [entities181-report.md](entities181-report.md) | Mobs, entities, items and their renderers |
| [../reports/](../reports/) | The player, screens and HUD (`b181-player-gui*.md`) and the full-suite integration rounds |

## Known differences

The same one as Beta 1.7.3: 3D block icons, and the blurred 3D panorama behind the title,
can be one colour level off on a few pixels, because the graphics driver rounds lighting
and filtering differently from OpenGL. Those screens count as matching; every other screen
is compared pixel for pixel with zero tolerance.
