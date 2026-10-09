param([string]$McpPath=(Join-Path $PSScriptRoot '../retromcp-r11'),[switch]$SkipRust)
$ErrorActionPreference='Stop'
$taskRoot=(Resolve-Path (Join-Path $PSScriptRoot '../..')).Path
$taskMcp=(Resolve-Path $McpPath).Path
$taskBuild=Join-Path $taskRoot 'target/r11/dragon-owner-build'
$taskVectors=Join-Path $taskRoot 'target/r11/dragon-owner-vectors'
New-Item -ItemType Directory -Force $taskBuild,$taskVectors | Out-Null
$taskJar=Join-Path $taskMcp 'minecraft/jars/deobfuscated.jar'
$taskLibs=@(Get-ChildItem (Join-Path $taskMcp 'libraries') -Recurse -Filter '*.jar' | Where-Object {$_.Name -notmatch 'sources|natives'} | ForEach-Object FullName)
$taskCp=(@($taskJar)+$taskLibs) -join ';'
if (-not $env:ORACLE_JAVA) {throw 'Set ORACLE_JAVA to pinned Java 8u51.'}
if ((& $env:ORACLE_JAVA -version 2>&1 | Out-String) -notmatch '1\.8\.0_51') {throw 'Pinned Java8u51 required.'}
$taskSources=@('SheepSlice11.java','DragonOwner11.java') | ForEach-Object {Join-Path $PSScriptRoot "src/net/minecraft/src/$_"}
& javac -cp $taskCp -d $taskBuild @taskSources
if ($LASTEXITCODE -ne 0){throw 'Original probe compilation failed.'}
& $env:ORACLE_JAVA -cp ($taskBuild+';'+$taskCp) net.minecraft.src.DragonOwner11 $taskVectors
if ($LASTEXITCODE -ne 0){throw 'Original dragon callback failed.'}
$taskActual=(Get-FileHash (Join-Path $taskVectors 'dragonowner11.txt')).Hash
$taskExpected=(Get-FileHash (Join-Path $taskRoot 'crates/worldgen11/src/fixtures/dragonowner11.txt')).Hash
if ($taskActual -ne $taskExpected){throw 'Dragon original fixture drift.'}
Write-Output 'ORIGINAL1.1 DRAGON OWNER EXACT: 216 cases'
if (-not $SkipRust){Push-Location $taskRoot;try{& cargo test --release -p worldgen11 --lib dragon_part_attacker_gate_matches_original_11_bytecode -- --nocapture;if($LASTEXITCODE -ne 0){throw 'Dragon production parity failed.'}}finally{Pop-Location}}
