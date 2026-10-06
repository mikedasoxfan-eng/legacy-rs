# Minecraft 1.0 original-bytecode fixtures

`oracle.sh MODE ...` loads `tools/retromcp-r10/minecraft/jars/deobfuscated.jar`
on Mojang Java 8u51. The launcher runtime is located automatically; `ORACLE_JAVA`
can select another installation of exactly 8u51. Compilation requires `javac`.
The server runner loads the separate original server jar. Run
`bash tools/oracle10/compile_check.sh` to compile main, GUI, visual and server
entry points and validate the RNG hooks.

The harness accepts the inherited `181` mode arguments and their `10` aliases.
Binary magic and reserved fields retain the inherited comparison schema so the
current worldgen10/client10 checkers can read it. Reserved fields for removed
XP-orb phase and fireball shake are zero. Removed sleep spawning and random
model-part selection fail explicitly rather than emulate removed game methods.

Entity, Explosion and World RNG hooks preserve every original method byte,
instruction offset, stack map and exception table except the seven-byte RNG
initializer substitution. `build_entity.py` verifies this by restoring the
patched bytes and comparing the entire class to the original jar. GUI clock
patches recompile the release GuiMainMenu, GuiAchievements and GuiAchievement
sources. Headless GL stubs record calls; pixel captures use real LWJGL in a
separate classpath.

`OracleRunner10` flushes the release threaded file I/O and closes region caches
before exiting. It also writes `<dump>.oracle10.json` with the invocation used
to reproduce diagnostics. Image decoding uses memory instead of temporary disk
caches. The spawning identity recorder uses release HashMap put calls with
border flags; the world's active-chunk recorder retains its HashSet add calls.

Run `python tools/r10/baseline.py` for the full, rendering, in-game and GUI
baseline, or pass repeated `--suite` arguments. Logs, checker metadata and Rust
diagnostic files stay under `target/r10/` (client comparison modes are isolated too); original dumps stay under
`tools/oracle10/out/`; captures stay under `shots10/`. `VERIFY10_KEEP=1` preserves
outputs, and `VERIFY10_JOBS` controls the main runner's parallel jobs (default 6).
`VERIFY10_LOCKED` is reserved for nested suites sharing the top-level lock.

`python tools/r10/diagnose_dump10.py JAVA.bin RUST.bin` reports the first field
using frame schemas, terrain/player-frame schemas or an opt-in original fixture rerun. Text statistics, saved NBT and framebuffer alpha checks report named fields or pixels too.
`ORACLE10_TRACE_BYTE=N` prints the fixture expression at that byte without
altering serialized values or patching additional original game methods.
Some bulk/variable records have only byte-level diagnostics; the baseline names
those limitations explicitly. Checkers that panic and removed-subject modes are
listed as run failures, not completed byte comparisons.

`port_harness.py` reproduces the initial copy and safe identifier renames; it
overwrites subsequent hand fixes, so use it only when starting a new port.
`port_verify.py` regenerates all 31 inherited suites. Recipe/registry extraction
utilities target only the release crates; baseline runs do not invoke them.
