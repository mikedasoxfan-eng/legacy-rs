param(
    [string]$McpPath = (Join-Path $PSScriptRoot '../retromcp-r11'),
    [switch]$Regenerate,
    [switch]$SkipRust
)
$ErrorActionPreference = 'Stop'
$taskRoot = (Resolve-Path (Join-Path $PSScriptRoot '../..')).Path
$taskMcp = (Resolve-Path -LiteralPath $McpPath).Path
$taskJar = Join-Path $taskMcp 'minecraft/jars/deobfuscated.jar'
$taskBuild = Join-Path $taskRoot 'target/r11/gameplay-build'
$taskVectors = Join-Path $taskRoot 'target/r11/gameplay-vectors'
New-Item -ItemType Directory -Force $taskBuild, $taskVectors | Out-Null
$taskLibraries = @(Get-ChildItem (Join-Path $taskMcp 'libraries') -Recurse -Filter '*.jar' |
    Where-Object { $_.Name -notmatch 'sources|natives' } | ForEach-Object FullName)
$taskClassPath = (@($taskJar) + $taskLibraries) -join ';'
$taskJavac = if ($env:JAVA_HOME) { Join-Path $env:JAVA_HOME 'bin/javac.exe' } else { 'javac' }
$taskJava = $env:ORACLE_JAVA
if (-not $taskJava) { throw 'Set ORACLE_JAVA to the pinned Java 8u51 executable first.' }
$taskVersion = (& $taskJava -version 2>&1 | Out-String)
if ($taskVersion -notmatch '1\.8\.0_51') { throw 'Gameplay oracle execution requires Java 8u51.' }
$taskSources = @('GameplaySlice11.java', 'BowSlice11.java', 'SheepSlice11.java', 'DoorSlice11.java', 'BlockSlice11.java', 'EggSlice11.java', 'SmallGameplay11.java', 'SmallBlocks11.java', 'EntityGuards11.java','LeavesWire11.java','InventoryTags11.java','AnimalDelta11.java','IndirectDamage11.java','BlockTint11.java','Landing11.java','GoldenFood11.java','PickBlock11.java') | ForEach-Object {
    Join-Path $PSScriptRoot "src/net/minecraft/src/$_"
}
& $taskJavac -cp $taskClassPath -d $taskBuild @taskSources
if ($LASTEXITCODE -ne 0) { throw 'Gameplay fixture compilation failed.' }
foreach ($taskClass in @('GameplaySlice11', 'BowSlice11', 'SheepSlice11', 'DoorSlice11', 'BlockSlice11', 'EggSlice11', 'SmallGameplay11', 'SmallBlocks11', 'EntityGuards11','LeavesWire11','InventoryTags11','AnimalDelta11','IndirectDamage11','BlockTint11','Landing11','GoldenFood11','PickBlock11')) {
    & $taskJava -cp ($taskBuild + ';' + $taskClassPath) "net.minecraft.src.$taskClass" $taskVectors *> (Join-Path $taskBuild "$taskClass.log")
    if ($LASTEXITCODE -ne 0) { throw "Original-bytecode fixture $taskClass failed." }
}
$taskFiles = @('xp11.txt','offers11.txt','tags11.txt','remainders11.txt','damage11.txt',
    'bows11.txt','bowenchant11.txt','arrowhits11.txt','arrownbt11.txt','sheep_actions11.txt','sheep_heads11.txt','sheep_ticks11.txt','shearing11.txt','magma11.txt','doors11.txt','blockboxes11.txt','farmland11.txt','farmwalking11.txt','gate11.txt','railadded11.txt','web11.txt','eggs11.txt','eggnames11.txt','creative11.txt','creative11.bin','egguse11.txt','eggstats11.txt','eggstackuse11.txt','egginvalidside11.txt','brewing11.txt','creeperrecords11.txt','snow11.txt','stem11.txt','trapdoor11.txt','detector11.txt','wolfguard11.txt','recipes11.bin','sheep_status11.txt','living_guard11.txt','squid_guard11.txt','leavesdrops11.txt','wireconnect11.txt','wirerise11.txt','wireremove11.txt','containertags11.txt','inventoryalias11.txt','stacknulltags11.txt','stacknames11.txt','animalfind11.txt','animalattack11.txt','indirectdamage11.txt','blocktint11.txt','landing11.txt','goldfood11.txt','pickblock11.txt')
foreach ($taskFile in $taskFiles) {
    $taskSource = Join-Path $taskVectors $taskFile
    $taskDestination = if ($taskFile -eq 'recipes11.bin') {
        Join-Path $taskRoot 'crates/worldgen11/src/container/recipes.bin'
    } elseif ($taskFile -eq 'creative11.bin') { Join-Path $taskRoot 'crates/worldgen11/src/block/creative11.bin' } else { Join-Path $taskRoot "crates/worldgen11/src/fixtures/$taskFile" }
    if ($Regenerate) { Copy-Item -LiteralPath $taskSource -Destination $taskDestination }
    $taskActual = (Get-FileHash -LiteralPath $taskSource -Algorithm SHA256).Hash
    $taskExpected = (Get-FileHash -LiteralPath $taskDestination -Algorithm SHA256).Hash
    if ($taskActual -ne $taskExpected) { throw "Original-bytecode vector drift: $taskFile" }
    Write-Output "ORIGINAL BYTECODE EXACT: $taskFile"
}
Write-Output ('Reference jar SHA256: ' + (Get-FileHash -LiteralPath $taskJar -Algorithm SHA256).Hash)
if (-not $SkipRust) {
    Push-Location $taskRoot
    try {
        & cargo test --release -p worldgen11 --lib original_11 -- --nocapture
        if ($LASTEXITCODE -ne 0) { throw 'Gameplay Rust parity tests failed.' }
    } finally { Pop-Location }
}
