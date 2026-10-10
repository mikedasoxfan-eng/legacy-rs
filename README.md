<div align="center">

# legacy-rs

**Minecraft Beta 1.7.3, Beta 1.8.1, 1.0 and 1.1, ported to Rust.**

<sub>Four versions in one singleplayer client. Formerly beta-rs.</sub>

[![release](https://img.shields.io/badge/release-v3.1.0-2ea44f?style=flat&labelColor=3b3b3b)](https://github.com/mikedasoxfan-eng/legacy-rs/releases/tag/v3.1.0)
[![CI](https://github.com/mikedasoxfan-eng/legacy-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/mikedasoxfan-eng/legacy-rs/actions/workflows/ci.yml)
[![Minecraft](https://img.shields.io/badge/Minecraft-Beta_1.7.3_%2B_Beta_1.8.1_%2B_1.0_%2B_1.1-6aa84f?style=flat&labelColor=3b3b3b)](#versions-and-saves)
[![Rust](https://img.shields.io/badge/built_with-Rust-b7410e?style=flat&logo=rust&logoColor=white&labelColor=3b3b3b)](#build-from-source)
[![wgpu](https://img.shields.io/badge/graphics-wgpu-4b6ec2?style=flat&labelColor=3b3b3b)](#architecture)
[![oracle](https://img.shields.io/badge/oracle-original_code_comparisons-2ea44f?style=flat&logo=openjdk&logoColor=white&labelColor=3b3b3b)](#how-verification-works)
[![GUI](https://img.shields.io/badge/GUI-pixel_comparisons-2ea44f?style=flat&labelColor=3b3b3b)](#rendering-and-gui-comparisons)

[![1.1 tests](https://img.shields.io/badge/1.1_retained_tests-67_client_%2B_114_simulation-2ea44f?style=flat&labelColor=3b3b3b)](#minecraft-11-release-evidence)
[![Rust lines](https://img.shields.io/badge/Rust-434,017_lines-b7410e?style=flat&labelColor=3b3b3b)](#repository-layout)
[![Mojang assets](https://img.shields.io/badge/Mojang_assets_bundled-0_bytes-2ea44f?style=flat&labelColor=3b3b3b)](#original-game-resources)
[![multiplayer](https://img.shields.io/badge/multiplayer-out_of_scope-9e9e9e?style=flat&labelColor=3b3b3b)](#scope-and-differences)
[![Herobrine](https://img.shields.io/badge/Herobrine-not_found-a33?style=flat&labelColor=3b3b3b)](#what-you-can-play)

**[Download v3.1.0](https://github.com/mikedasoxfan-eng/legacy-rs/releases/tag/v3.1.0)** · **[Documentation](docs/README.md)** · **[Verification reports](reports/README.md)** · **[Changelog](CHANGELOG.md)**

</div>

Each version has its own simulation and renderer, checked against the matching original game bytecode. The aim is to preserve the selected version's mechanics, save data, menus and rendering behavior, including quirks that later Minecraft releases changed.

## Download and play

1. Download `legacy-rs-3.1.0-windows-x64.exe` from the release, or extract the Windows ZIP into its own folder.
2. Install the Minecraft versions you want to play through the official Minecraft launcher. Enable historical versions for the Betas, and run each selected version once to obtain its resources.
3. Run the executable and select a version. The ZIP names the executable `beta.exe`; the standalone download works the same way.

The published build is for **Windows x64**. The renderer uses wgpu and requires a compatible graphics device. Textures, sounds and fonts come from your own Minecraft installation; no Mojang game assets are bundled. See [installation and troubleshooting](docs/playing.md) for the expected paths.

To select a version directly:

```powershell
.\beta.exe --mc b1.7.3
.\beta.exe --mc b1.8.1
.\beta.exe --mc 1.0
.\beta.exe --mc 1.1
```

### Original game resources

The game reads the selected version's textures, fonts and language files from your installed original JAR, and sounds/music from the legacy resource directory.

| Version | Expected original JAR |
|---|---|
| Beta 1.7.3 | `%APPDATA%\.minecraft\versions\b1.7.3\b1.7.3.jar` |
| Beta 1.8.1 | `%APPDATA%\.minecraft\versions\b1.8.1\b1.8.1.jar` |
| Minecraft 1.0 | `%APPDATA%\.minecraft\versions\1.0\1.0.jar` |
| Minecraft 1.1 | `%APPDATA%\.minecraft\versions\1.1\1.1.jar` |

Sounds and music are loaded from `%APPDATA%\.minecraft\resources`. For a custom installation, set `BETA_JAR` to the matching JAR; this changes the asset path, while `--mc` still chooses the game implementation:

```powershell
$env:BETA_JAR = 'D:\Minecraft\versions\1.1\1.1.jar'
.\beta.exe --mc 1.1
```

A missing JAR or sound directory needs the original resources installed in the expected location. The download includes no replacement texture, font or sound pack. See the [playing guide](docs/playing.md) for troubleshooting.

## What you can play

| System | Included behavior |
|---|---|
| Terrain and exploration | Version-specific Overworld and Nether generation, caves, population, lighting and structures; the End in 1.0 and 1.1 |
| Survival | Movement, collision, mining, placement, tools, damage, death, respawn, beds, portals and inventories |
| Blocks and mechanisms | Redstone, rails, pistons, liquids, crops, furnaces, dispensers and tile-entity save/load behavior |
| Entities | Version-specific mobs and AI, spawning, items, projectiles, vehicles and riding |
| Menus and HUD | World selection/creation, options, controls, statistics, achievements, container screens and the selected version's HUD |
| Graphics | Chunk meshes, block and entity models, held items, camera effects, particles, sky, clouds, fog and weather |
| Audio | Positional effects, sound pools, records and music timing using installed resources |
| Persistence | Original-format worlds, player inventory/state, options, statistics and screenshots |

Beta 1.7.3 keeps classic Beta terrain and survival. Beta 1.8.1 adds the Adventure Update's biomes and structures, hunger, sprinting, experience, creative mode, new mobs and lightmap rendering. Minecraft 1.0 adds hardcore, enchanting, brewing, the End and dragon, Nether fortresses, breeding and the corresponding blocks, mobs and screens.

**Minecraft 1.1** adds Superflat selection and world-type persistence, creative spawn eggs, language selection, translated menus/tooltips, Unicode font pages and bidirectional text. Its port also follows the original changes to world generation, entity/player interactions, crafting, biome colors, fog and item rendering. [1.1 version notes](docs/minecraft-1.1.md)

## Versions and saves

| Minecraft version | Highlights | Default data directory |
|---|---|---|
| Beta 1.7.3 | Classic Beta terrain, survival, redstone and pistons | Executable folder |
| Beta 1.8.1 | Adventure Update terrain and structures, hunger, sprinting, creative mode | `b1.8.1\` |
| 1.0 | The End and dragon, enchanting, brewing, hardcore mode | `1.0\` |
| 1.1 | Superflat worlds, spawn eggs, language selection, Unicode and bidirectional text | `1.1\` |

Each directory contains its own `saves\`, options, statistics and screenshots. A plain launch or `--mc <version>` launch uses the executable folder; commands with additional options use the caller's working directory. Worlds use the original version's save format. Back up a world before moving it between clients, and open it with the matching Minecraft version. [Playing guide](docs/playing.md) · [Minecraft 1.0](docs/minecraft-1.0.md) · [Minecraft 1.1](docs/minecraft-1.1.md)

### Moving worlds between clients

Copy a backed-up world into the matching version's `saves\` directory, or open a specific folder with `--world-dir`. Use the same Minecraft version on both sides when exchanging saves.

```powershell
.\beta.exe --mc 1.1 --world-dir 'D:\Minecraft\saves\My World'
```

### Controls

Gameplay keys are rebindable in Options → Controls. The selected version supplies its original defaults and debug shortcuts.

| Key | Action |
|---|---|
| W A S D | Move |
| Space / Left Shift | Jump / sneak |
| Mouse | Look, with sensitivity and invert settings |
| Left / right click | Mine / place or use |
| Middle click | Pick block |
| 1–9 / mouse wheel | Choose hotbar slot |
| E / Q | Inventory / drop item |
| F1 / F2 / F3 | Hide HUD / save screenshot / debug information |
| F5 / F8 | Third-person view / smooth camera |
| Esc | Open the game menu |

Render-distance shortcuts depend on the version: Beta 1.7.3 uses F; Minecraft 1.0 and 1.1 use F3+F. Render distance is also available in Video Settings.

### Command-line options

The ZIP calls the game `beta.exe`. Substitute the standalone download's filename when using that executable.

| Option | Meaning |
|---|---|
| `--mc <version>` | Choose `b1.7.3`, `b1.8.1`, `1.0` or `1.1` |
| `--seed <number or text>` | Create and enter a world with this seed |
| `--world <name>` | Enter or create a named world |
| `--world-dir <path>` | Open a specific save folder |
| `--no-save` | Prevent world writes |
| `--options-dir <path>` | Select the options/statistics/screenshots directory |
| `--time <ticks>` | Set starting time; 0 is sunrise and 6000 is noon |
| `--weather <rain,thunder,flash>` | Set initial weather strengths |
| `--shot <file.png> [--shot-ticks n]` | Capture a screenshot after the specified ticks and exit |
| `--play-script <file.txt>` | Replay scripted player input for unattended checks |
| `--server [--view-distance n]` | Exercise dedicated-server-style chunk loading |
| `--version` / `--help` | Print version information / selected-client help |

For information without opening a game window:

```powershell
.\beta.exe --version
.\beta.exe --help --mc 1.1
```

Keep `--help` first when checking Beta 1.7.3. Beta 1.8.1 also offers an optional `--mc b1.8.1e` enhanced mode with additional rendering and comfort settings, separate from the original-version verification claims.

## How verification works

The reference harness invokes the installed original Java classes and compares their outputs with Rust: world generation, block and entity behavior, saved data, screen callbacks, text layout and rendering inputs. GUI checks also compare captured pixels, with documented allowances for graphics-driver rounding on some 3D content.

Details that matter when preserving original behavior:

- **Reference runtime:** Math functions can differ in their final bits between Java runtimes. The harness configuration prescribes the expected oracle environment.
- **Randomness:** Replays control seeds and state for world ticks, entities and other randomized behavior so both implementations receive the same sequence.
- **Iteration order:** Java collection traversal can affect redstone updates, chunk ticks and event ordering. The simulation includes compatible collection behavior where the original depends on it.
- **Numeric precision:** The port preserves distinctions between float and double operations, casts, overflow and signed values.
- **Stateful callbacks:** A final image or inventory can hide a different callback order. Corpora also retain intermediate state changes, events and lifecycle behavior.

### What the harnesses exercise

| Area | Examples of comparisons |
|---|---|
| Terrain | Seeded chunks, population, structures, biome boundaries and loading order |
| World simulation | Lighting, weather, random ticks, plants, fire, redstone and fluids |
| Saves | Region files, level.dat, NBT, original-to-Rust and Rust-to-original reads |
| Entities and player | Tick-by-tick movement, AI, damage, item use, inventories, beds and portals |
| Containers | Click semantics, stack limits, recipes, furnace and dispenser state |
| Rendering | Chunk vertices, special blocks, entity poses, held items, particles and sky/fog inputs |
| Screens | Callbacks, inventory layouts, tooltips, statistics, achievements and text wrapping |
| Native workflows | Client input, initial save, reopen, selection, rename and deletion |

The harnesses and coverage differ by version. `tools/verify.sh` serves Beta 1.7.3; `tools/verify181.sh`, `tools/verify10.sh` and `tools/verify11.sh` serve later versions, alongside focused comparisons and GUI checks. See [development prerequisites](docs/development.md) before reproducing them; original game files and prepared local fixtures are required.

### Minecraft 1.1 release evidence

| Retained check | Result |
|---|---|
| Complete 1.1 client library, including prepared fixture tests | 67 passed |
| 1.1 simulation library | 114 passed |
| Cross-version workspace library regressions | 249 passed |
| Potion names and tooltips | 159,452 original calls matched |
| World-name NBT, including unmatched UTF-16 surrogates | 65,550 original cases matched |
| Unicode world-creation callbacks | 96 original callbacks matched |
| Achievement tooltips | 3,021 original callbacks across 57 locales; original formatter exceptions matched |
| Native world lifecycle | Initial save, reopen, list, rename and delete checked |
| Source and evidence provenance | 509 hash pins matched |
| v3.1.0 package | Workspace release build, four headless version dispatches and uploaded artifact hashes passed |

The class audit covers 144 normalized changed classes, with a separate review of 308 raw-only class changes so identifier normalization cannot silently hide a behavioral difference. [1.1 methodology](docs/minecraft-1.1-verification.md) · [Retained evidence](reports/README.md) · [v3.1.0 package report](reports/releases/v3.1.0.md)

These are recorded results for identified source candidates and corpora. The release's metadata-only rebuild did not rerun every older version's full original-code suite. Earlier full-suite results remain documented in the [changelog](CHANGELOG.md) and version reports.

### Rendering and GUI comparisons

GUI harnesses drive the original and port through matching scenes and compare captured pixels. Renderer-input comparisons help distinguish simulation or geometry errors from rasterization differences.

Some 3D block icons and filtered panorama pixels can differ by one color level because graphics drivers round lighting and filtering differently. Relevant comparisons use documented tolerances. Text, callback, layout and numeric comparisons retain their own exact expectations; a tolerance in one rendering case does not apply to all checks.

### Scope and differences

Verification establishes agreement for the audited changes and tested cases. It does not promise identical pixels on every GPU or cover every possible Java API path. Multiplayer networking, an account launcher and online skins are outside this project's scope. The `--server` testing option selects server-style chunk loading; it does not host a multiplayer server.

Platform-specific information also differs: debug memory figures describe the native process rather than a Java heap. The recorded GitHub Actions validation could not start because GitHub reported an account payment or spending-limit block; the configured four-version simulation command passed locally. [Recorded CI status](reports/r11-progress/remote-ci-status.json) distinguishes that runner limitation from a successful remote run.

## Build from source

Install Rust with the Windows MSVC toolchain and its C++ build tools, then run:

```powershell
cargo build --release -p client -p give
.\target\release\beta.exe
```

The `client` package builds the `beta` executable, including all four versions. `give.exe` is an optional offline inventory editor; close the game before using it. [Build and verification guide](docs/development.md)

### Local tests

Run simulation libraries serially because vanilla decorators share singleton state:

```powershell
cargo test --release -p worldgen -p worldgen181 -p worldgen10 -p worldgen11 --lib -- --test-threads=1
cargo test --workspace --lib -- --test-threads=1
```

Ignored original-fixture tests need their prepared corpora. A normal test run alone does not establish that those tests passed. For 1.1, `tools/verify_r11_final.ps1` replays the complete retained fixture batch when its prerequisites are present; `-PackageOnly` rebuilds and checks the command line headlessly.

### Offline inventory editor

`give.exe` edits a world's player inventory while the game is closed. It retains the previous file as `level.dat.bak`; run it without arguments for usage.

```powershell
.\give.exe '1.0\saves\My World' 120 12
```

This supplies twelve End portal frames to that 1.0 world. The utility edits saved NBT directly and is separate from the selected game's original commands.

## Architecture

The **world worker** advances the simulation, loads and saves chunks, prepares meshes and publishes snapshots. The **window thread** handles input, screens, sound and drawing. Player input crosses into the simulation as messages, while mouse look can be applied between ticks for rendering.

Version-specific simulation crates own terrain, blocks, entities, player state, containers and persistence. Client crates adapt those states into the matching screens, sounds and rendered scene. The executable entry point dispatches to the selected client; default data directories keep saves and settings separate.

| Component | Responsibility |
|---|---|
| Terrain and population modules | Noise, biome selection, caves, ores, vegetation and structures |
| `world/` | Loaded chunks, lighting, ticking, weather, redstone, rails and portals |
| Entity and player modules | Physics, AI, spawning, inventory, controllers and persistence |
| Storage/NBT modules | Original-format world and player data |
| `app.rs` and `worker.rs` | Window loop, client state and worker coordination |
| `render/` | Block meshes, models, camera, held items, targeting, weather and sky |
| `gui/` | Screen callbacks, settings, font layout and GUI drawing |
| `world_effects.rs` and `sound.rs` | Ordered particles/sounds and audio output |

wgpu provides graphics output; cpal provides audio output through the client's mixer. The GUI rendering layer preserves fixed-function drawing behavior used by the original screens. Module arrangements differ between versions as their systems evolve.

### Original quirks

The subsystem notes explain details such as piston progress saved from the preceding state, quick sticky-piston pulses leaving a block behind, tile-entity invalidation and deferred insertion, shared block bounds, and collection-dependent redstone update order.

These are version-specific behaviors to verify and preserve. [Pistons and tile entities](docs/pistons.md) · [Containers](docs/containers.md) · [First-person view](docs/first-person.md) · [Weather](docs/weather.md)

## Repository layout

| Path | Purpose |
|---|---|
| `crates/worldgen`, `crates/client` | Beta 1.7.3 simulation and executable entry point |
| `crates/worldgen181`, `crates/client181` | Beta 1.8.1 |
| `crates/worldgen10`, `crates/client10` | Minecraft 1.0 |
| `crates/worldgen11`, `crates/client11` | Minecraft 1.1 |
| `crates/give` | Offline inventory editor |
| `tools/oracle*`, `tools/verify*` | Original-code harnesses and comparisons |
| `docs/` | Playing, development, version and subsystem documentation |
| `reports/` | Retained test evidence, audits and release verification |

## License

legacy-rs is a personal, unofficial project and is **not** released under an open-source license: much of it follows the structure of Mojang's decompiled game code, so the code is for personal use only and may not be redistributed. Minecraft's textures, sounds and fonts are never included; they are read from your own installation at runtime.

Not affiliated with or endorsed by Mojang or Microsoft. Minecraft is a trademark of Mojang Synergies AB.
