# Installing and playing

Download the Windows x64 executable or ZIP from [releases](https://github.com/mikedasoxfan-eng/legacy-rs/releases). Extract the ZIP into a dedicated folder and run `beta.exe`. The standalone executable has a versioned filename; substitute that filename in the commands below.

## Original resources

Install and run the selected version once through the official Minecraft launcher. The client looks for these JARs:

| Version | Default JAR |
|---|---|
| Beta 1.7.3 | `%APPDATA%\.minecraft\versions\b1.7.3\b1.7.3.jar` |
| Beta 1.8.1 | `%APPDATA%\.minecraft\versions\b1.8.1\b1.8.1.jar` |
| 1.0 | `%APPDATA%\.minecraft\versions\1.0\1.0.jar` |
| 1.1 | `%APPDATA%\.minecraft\versions\1.1\1.1.jar` |

Textures and fonts are read from the matching JAR. Sounds are read from `%APPDATA%\.minecraft\resources`. An installation in a custom launcher directory may require setting `BETA_JAR` to the matching JAR. That override selects the asset file; `--mc` still selects the game version.

```powershell
$env:BETA_JAR = 'D:\Minecraft\versions\1.1\1.1.jar'
.\beta.exe --mc 1.1
```

If resources are missing, check these paths first. Changing the graphics backend cannot fix a missing JAR or sound directory.

## Saves and options

Default paths are relative to the process's working directory. Run from the extracted folder. Beta 1.7.3 writes `saves\`, `options.txt`, `stats\`, `screenshots\` and `texturepacks\` there. The later versions place their data under `b1.8.1\`, `1.0\` and `1.1\` respectively.

Back up worlds before importing them. Use the matching version when exchanging worlds with the original game. An explicit save directory can be opened with:

```powershell
.\beta.exe --mc 1.1 --world-dir 'D:\Minecraft\saves\My World'
```

`--no-save` prevents world writes. `--options-dir <path>` selects an options/statistics/screenshots directory. See the selected client's help for further options:

```powershell
.\beta.exe --help --mc 1.1
.\beta.exe --version
```

These two commands print information without opening a game window. Keep `--help` first when checking Beta 1.7.3.

## Inventory editor

`give.exe` edits a world's player inventory while the game is closed and retains the previous file as `level.dat.bak`. Its help prints when run without arguments. Example for a 1.0 world:

```powershell
.\give.exe '1.0\saves\My World' 120 12
```

This supplies twelve End portal frames. It edits the save directly; it is separate from the original game's commands.

## Graphics and scope

The client uses wgpu. If startup reports a graphics initialization failure, retain the diagnostic output and check that the device and driver support an available backend. Some 3D GUI pixels can differ by a color level due to driver rounding; see [verification](minecraft-1.1-verification.md).

This is a singleplayer client. The launcher, authentication, multiplayer networking and online skins are outside scope. Beta 1.8.1 also has an optional `--mc b1.8.1e` enhanced mode; its extra rendering and comfort settings are separate from the original-version verification claims.
