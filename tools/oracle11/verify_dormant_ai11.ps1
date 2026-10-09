param([switch]$Regenerate,[switch]$SkipRust)
$ErrorActionPreference='Stop'
$taskRoot=(Resolve-Path (Join-Path $PSScriptRoot '../..')).Path
Push-Location $taskRoot
try {
    & (Join-Path $PSScriptRoot 'verify_eggstate11.ps1') -SkipRust
    & python tools/oracle11/audit_dormant_ai11.py ../retromcp-1.1 target/r11/ai-dormant-static-review.json
    if($LASTEXITCODE -ne 0){throw 'Original source/bytecode AI graph audit failed'}
    $taskLibraries=@(Get-ChildItem ../retromcp-1.1/libraries -Recurse -Filter '*.jar' | Where-Object {$_.Name -notmatch 'sources|natives'} | ForEach-Object FullName)
    $taskCp=(@('target/r11/eggstate-build','../retromcp-1.1/minecraft/jars/deobfuscated.jar','tools/oracle11/build')+$taskLibraries)-join ';'
    & javac -cp $taskCp -d target/r11/eggstate-build tools/oracle11/src/net/minecraft/src/DormantAi11.java
    if($LASTEXITCODE -ne 0){throw 'Dormant AI probe compilation failed'}
    & $env:ORACLE_JAVA -cp $taskCp net.minecraft.src.DormantAi11 target/r11/ai-dormant
    if($LASTEXITCODE -ne 0){throw 'Original dormant AI graph assertions failed'}
    $taskActual='target/r11/ai-dormant/ai_dormant11.txt'
    $taskFixture='tools/r11/data/ai_dormant11.txt'
    if($Regenerate){Copy-Item -LiteralPath $taskActual -Destination $taskFixture}
    if((Get-FileHash $taskActual).Hash -ne (Get-FileHash $taskFixture).Hash){throw 'Original AI graph fixture drift'}
    Write-Output 'ORIGINAL AI GRAPH SNAPSHOT EXACT: 240 cases'
    if(-not $SkipRust){cargo test --release -p worldgen11 --lib egg_constructor_save_tick_matches_original_11_bytecode -- --test-threads=1;if($LASTEXITCODE -ne 0){throw 'Production20-mob constructor/save/tick parity failed'}}
} finally {Pop-Location}
