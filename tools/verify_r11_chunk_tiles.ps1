$ErrorActionPreference='Stop'
Push-Location (Split-Path $PSScriptRoot -Parent)
try {
 $libs=Get-ChildItem tools/retromcp-r11/libraries -Recurse -Filter '*.jar' | Where-Object {$_.Name -notmatch 'sources|natives|^lwjgl-2'} | ForEach-Object {$_.FullName}
 $cp='tools/oracle11/probes/build;tools/oracle11/build/input;tools/retromcp-r11/minecraft/jars/deobfuscated.jar;tools/oracle11/build;tools/oracle11/build/lwjgl-unsealed.jar;'+($libs -join ';')
 New-Item -ItemType Directory -Force target/r11 | Out-Null
 javac -cp $cp -d tools/oracle11/probes/build tools/oracle11/probes/ChunkTileCoordinates11.java
 if ($LASTEXITCODE) {throw 'Chunk probe compilation failed'}
 & $env:ORACLE_JAVA -cp $cp net.minecraft.src.ChunkTileCoordinates11 target/r11/chunktiles11.txt
 if ($LASTEXITCODE) {throw 'Original chunk probe failed'}
 $actual=[IO.File]::ReadAllText((Resolve-Path target/r11/chunktiles11.txt)).Replace("`r`n","`n")
 $saved=[IO.File]::ReadAllText((Resolve-Path crates/worldgen11/src/fixtures/chunktiles11.txt)).Replace("`r`n","`n")
 if ($actual -cne $saved) {throw 'Original chunk vectors changed'}
 cargo test -p worldgen11 --lib chunk_tile_coordinates_match_original_11 -- --nocapture
 if ($LASTEXITCODE) {throw 'Chunk tile comparison failed'}
 Write-Output 'R11 CHUNK TILE COORDINATES EXACT: 1536 cases'
} finally {Pop-Location}
