# Minecraft 1.0

Minecraft 1.0 support was introduced in legacy-rs 3.0.0. The 3.0.1 and 3.0.2 releases corrected End portal, dragon, End sky/fog and record playback behavior; see the [changelog](../CHANGELOG.md).

Start with `beta.exe --mc 1.0`. Original resources come from the launcher's `versions\1.0\1.0.jar`; saves and options default to `1.0\` beneath the working directory.

The version adds the End dimension and dragon, Nether fortresses, mushroom islands, enchanting, brewing and potions, hardcore mode, breeding, new mobs and their renderers. Its implementation lives in `crates/worldgen10` and `crates/client10`.

## Verification

The 3.0.2 release records a complete 2,006-section original-code suite. The 3.1.0 candidate additionally passes the workspace library regression batch. These are separate results: the historical full original-code suite was not rerun during the documentation and version-number packaging change.

`tools/verify10.sh` drives the original 1.0 comparisons, with `tools/oracle10` providing the Java harness and `tools/r10` retaining class coverage and change-map data. Original game files and the oracle runtime are local prerequisites; they are not bundled. [Development guide](development.md) · [Reports](../reports/README.md)
