param([switch]$Regenerate, [switch]$SkipRust)
$ErrorActionPreference = 'Stop'
$taskRoot = (Resolve-Path (Join-Path $PSScriptRoot '../..')).Path
Push-Location $taskRoot
try {
    if (-not $env:ORACLE_JAVA) { throw 'Load work/toolchains/env.ps1 first.' }
    if ((& $env:ORACLE_JAVA -version 2>&1 | Out-String) -notmatch '1\.8\.0_51') { throw 'Original oracle requires Java 8u51.' }
    if (-not (Test-Path 'tools/oracle11/build/net/minecraft/src/Mode_entities10.class')) { throw 'Build the main oracle11 helper classes first.' }
    & python tools/oracle11/prepare_eggstate11.py
    if ($LASTEXITCODE -ne 0) { throw 'Original Entity preservation assertion failed.' }
    $taskLibraries = @(Get-ChildItem ../retromcp-1.1/libraries -Recurse -Filter '*.jar' | Where-Object { $_.Name -notmatch 'sources|natives' } | ForEach-Object FullName)
    # Only the injected Entity precedes the original jar; all game methods come from that jar.
    $taskCp = (@('target/r11/eggstate-build','../retromcp-1.1/minecraft/jars/deobfuscated.jar','tools/oracle11/build') + $taskLibraries) -join ';'
    & javac -cp $taskCp -d target/r11/eggstate-build tools/oracle11/src/net/minecraft/src/OracleHooks.java tools/oracle11/src/net/minecraft/src/EggState11.java
    if ($LASTEXITCODE -ne 0) { throw 'Egg state probe compilation failed.' }
    & $env:ORACLE_JAVA -cp $taskCp net.minecraft.src.EggState11 target/r11/eggstate-clean
    if ($LASTEXITCODE -ne 0) { throw 'Original egg state probe failed.' }
    $taskActual = 'target/r11/eggstate-clean/eggstate11.bin'
    $taskFixture = 'crates/worldgen11/src/fixtures/eggstate11.bin'
    if ($Regenerate) { Copy-Item -LiteralPath $taskActual -Destination $taskFixture }
    if ((Get-FileHash $taskActual).Hash -ne (Get-FileHash $taskFixture).Hash) { throw 'Original-bytecode egg state fixture drift.' }
    Write-Output 'ORIGINAL BYTECODE EXACT: 960 constructor/tick/save/reload cases with post-load ticks'
    if (-not $SkipRust) {
        & cargo test --release -p worldgen11 --lib egg_ -- --test-threads=1
        if ($LASTEXITCODE -ne 0) { throw 'Egg state parity failed.' }
    }
} finally { Pop-Location }
