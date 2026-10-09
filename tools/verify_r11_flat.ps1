# Run from the repository with javac, Cargo, and ORACLE_JAVA configured.
# Build tools/oracle11 first using its normal build script.
$ErrorActionPreference = 'Stop'
$repo = Split-Path $PSScriptRoot -Parent
Push-Location $repo
try {
    if (!$env:ORACLE_JAVA) { throw 'ORACLE_JAVA must point to the original Minecraft Java 8 runtime.' }
    $cp = 'tools/oracle11/probes/build;tools/oracle11/build;tools/retromcp-r11/minecraft/jars/deobfuscated.jar'
    New-Item -ItemType Directory -Force tools/oracle11/probes/build,target/r11/flat | Out-Null
    javac -cp $cp -d tools/oracle11/probes/build tools/oracle11/src/net/minecraft/src/Mode_structures10.java tools/oracle11/probes/FlatGridProbe11.java
    if ($LASTEXITCODE) { throw 'Java probe compilation failed' }
    cargo build --release -p worldgen11 --bin compare11 --bin terrain_compare11
    if ($LASTEXITCODE) { throw 'Rust compilation failed' }
    foreach ($seed in @(0,42,-1)) {
        foreach ($mode in @('flat-village-layout','flat-village-map','flat-village-blocks','village-map','village-blocks')) {
            $file = "target/r11/flat/$mode-$seed.bin"
            & $env:ORACLE_JAVA -cp $cp net.minecraft.src.Mode_structures10 structures10 $mode $seed $file
            if ($LASTEXITCODE) { throw "Oracle failed: $mode $seed" }
            $result = & target/release/terrain_compare11.exe structures $mode $seed $file 2>&1
            $result | ForEach-Object { Write-Host $_ }
            if ($LASTEXITCODE -or !(($result -join "`n").Contains('EXACT MATCH'))) { throw "Mismatch: $mode $seed" }
        }
    }
    $cases = @(@('0',0,0,2,2),@('0',17,18,22,23),@('42',-29,-21,-24,-16),@('-1',11,10,16,15))
    foreach ($case in $cases) {
        $seed,$x0,$z0,$x1,$z1 = $case
        $file = "target/r11/flat/grid-$seed-$x0-$z0.bin"
        & $env:ORACLE_JAVA -cp $cp net.minecraft.src.FlatGridProbe11 $seed 300 $x0 $z0 $x1 $z1 end $file
        if ($LASTEXITCODE) { throw "Oracle flatgrid failed: $case" }
        & target/release/compare11.exe flatgrid $seed 300 $x0 $z0 $x1 $z1 end $file
        if ($LASTEXITCODE) { throw "Flatgrid mismatch: $case" }
    }
    foreach ($case in @(@('0',-2,-2,2,2),@('42',-6,2,-2,6),@('-1',2,-6,6,-2))) {
        $seed,$x0,$z0,$x1,$z1 = $case
        $file = "target/r11/flat/normal-$seed.bin"
        & $env:ORACLE_JAVA -cp $cp net.minecraft.src.Oracle grid $seed 300 $x0 $z0 $x1 $z1 end $file
        if ($LASTEXITCODE) { throw "Oracle grid failed: $case" }
        & target/release/compare11.exe grid $seed 300 $x0 $z0 $x1 $z1 end $file
        if ($LASTEXITCODE) { throw "Grid mismatch: $case" }
    }
    Write-Host 'R11 FLAT AND POPULATED GRID EXACT'
} finally { Pop-Location }
