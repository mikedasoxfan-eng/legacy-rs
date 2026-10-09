# legacy-rs

Minecraft **Beta 1.7.3, Beta 1.8.1, 1.0 and 1.1**, ported to Rust in one singleplayer client. Each version has its own simulation and renderer, with behavior checked against the original game bytecode. Formerly beta-rs.

**[Download v3.1.0](https://github.com/mikedasoxfan-eng/legacy-rs/releases/tag/v3.1.0)** · **[Documentation](docs/README.md)** · **[Verification reports](reports/README.md)** · **[Changelog](CHANGELOG.md)**

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

## Versions and saves

| Minecraft version | Highlights | Default data directory |
|---|---|---|
| Beta 1.7.3 | Classic Beta terrain, survival, redstone and pistons | Current working directory |
| Beta 1.8.1 | Adventure Update terrain and structures, hunger, sprinting, creative mode | `b1.8.1\` |
| 1.0 | The End and dragon, enchanting, brewing, hardcore mode | `1.0\` |
| 1.1 | Superflat worlds, spawn eggs, language selection, Unicode and bidirectional text | `1.1\` |

Each directory contains its own `saves\`, options, statistics and screenshots. Run from the game's folder to keep these files together. Worlds use the original version's save format. Back up a world before moving it between clients, and open it with the matching Minecraft version. [Playing guide](docs/playing.md) · [Minecraft 1.0](docs/minecraft-1.0.md) · [Minecraft 1.1](docs/minecraft-1.1.md)

Controls follow the selected version and can be changed in Options: WASD to move, Space to jump, Shift to sneak, mouse buttons to break/use blocks, E for inventory, Q to drop, and 1–9 for the hotbar. F2 saves a screenshot; Esc opens the game menu.

## Verification

The reference harness invokes the installed original Java classes and compares their outputs with Rust: world generation, block and entity behavior, saved data, screen callbacks, text layout and rendering inputs. GUI checks also compare captured pixels, with documented allowances for graphics-driver rounding on some 3D content.

Minecraft 1.1 verification includes the complete retained client and simulation test sets, original-code corpora for language and Unicode handling, native save lifecycle checks, and regression tests across the four version libraries. The [1.1 verification document](docs/minecraft-1.1-verification.md) explains the class audit and its boundaries; the [release report](reports/releases/v3.1.0.md) records the build and package checks.

Verification establishes agreement for the audited changes and tested cases. It does not promise identical pixels on every GPU or cover every possible Java API path. Multiplayer networking, an account launcher and online skins are outside this project's scope. The `--server` testing option selects server-style chunk loading; it does not host a multiplayer server.

## Build from source

Install Rust with the Windows MSVC toolchain and its C++ build tools, then run:

```powershell
cargo build --release -p client -p give
.\target\release\beta.exe
```

The `client` package builds the `beta` executable, including all four versions. `give.exe` is an optional offline inventory editor; close the game before using it. [Build and verification guide](docs/development.md)

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
