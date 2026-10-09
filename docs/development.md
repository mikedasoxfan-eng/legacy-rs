# Building and verifying

## Windows build

Use Rust's MSVC toolchain and the corresponding C++ build tools. From the repository root:

```powershell
cargo build --release -p client -p give
.\target\release\beta.exe --version
.\target\release\beta.exe --help --mc 1.1
```

The `client` package builds `beta.exe` and links all four version clients. `give.exe` is the separate offline inventory editor. `cargo build --release --workspace` also builds the workspace's verification tools. Original resources are needed for gameplay; see [playing](playing.md).

## Tests without original files

Run the simulation libraries serially because vanilla decorators share singleton state:

```powershell
cargo test --release -p worldgen -p worldgen181 -p worldgen10 -p worldgen11 --lib -- --test-threads=1
cargo test --workspace --lib -- --test-threads=1
```

Ignored fixture tests require separately prepared original corpora. An ordinary library test run does not establish that those tests passed.

## Original-code verification

The oracle invokes classes from the corresponding installed original JAR. Full comparisons also require locally prepared decompiled sources, Java compiler/runtime prerequisites and, for rendering checks, a graphics device. Original binaries, assets and generated oracle fixtures are not committed or included in releases.

| Version | Main comparison harness | Original-code harness |
|---|---|---|
| Beta 1.7.3 | `tools/verify.sh`, `tools/verify_gui.sh` | `tools/oracle` |
| Beta 1.8.1 | `tools/verify181.sh`, `tools/verify_gui181.sh` | `tools/oracle181` |
| 1.0 | `tools/verify10.sh` | `tools/oracle10` |
| 1.1 | `tools/verify11.sh`, focused `tools/verify_r11_*` scripts | `tools/oracle11` |

The shell harnesses require Bash and their documented local Java/runtime configuration. For the completed 1.1 fixture replay, `tools/verify_r11_final.ps1` runs the retained client/simulation corpora, potion checks, workspace regressions and package checks. Its fixture paths are explicit; prepare them before replaying. The script does not download original game files or generate every fixture automatically.

```powershell
.\tools\verify_r11_final.ps1
.\tools\verify_r11_final.ps1 -PackageOnly
python tools/r11/verify_pins11.py
```

Keep evidence tied to the source and binary actually checked. Existing final-validation records are historical and should not be silently replaced after a new build. Record later package hashes and changed scope in a release report. [Report index](../reports/README.md)

## Architecture

Version-specific simulation crates own world generation, blocks, entities, player state and persistence. Client crates handle input, screens, sound and rendering. The world worker advances the simulation; the window thread processes input and drawing. The main `client` executable dispatches to the selected version library.

The original subsystem notes in this directory describe the Beta 1.7.3 foundation. Later crates adapt that foundation to each version; a historical note alone is not evidence that every described behavior is identical in later versions.
