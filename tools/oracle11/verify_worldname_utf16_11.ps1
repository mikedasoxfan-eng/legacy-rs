param([string]$McpPath=(Join-Path $PSScriptRoot '../retromcp-r11'),[switch]$SkipRust)
$ErrorActionPreference='Stop'
$taskRoot=(Resolve-Path (Join-Path $PSScriptRoot '../..')).Path
$taskMcp=(Resolve-Path $McpPath).Path
$taskBuild=Join-Path $taskRoot 'target/r11/worldname-build'
$taskVectors=Join-Path $taskRoot 'target/r11/worldname-vectors'
New-Item -ItemType Directory -Force $taskBuild,$taskVectors | Out-Null
$taskJar=Join-Path $taskMcp 'minecraft/jars/deobfuscated.jar'
$taskLibs=@(Get-ChildItem (Join-Path $taskMcp 'libraries') -Recurse -Filter '*.jar' | Where-Object {$_.Name -notmatch 'sources|natives'} | ForEach-Object FullName)
$taskCp=(@($taskJar)+$taskLibs) -join ';'
if (-not $env:ORACLE_JAVA) {throw 'Set ORACLE_JAVA to pinned Java 8u51.'}
if ((& $env:ORACLE_JAVA -version 2>&1 | Out-String) -notmatch '1\.8\.0_51') {throw 'Pinned Java8u51 required.'}
& javac -cp $taskCp -d $taskBuild (Join-Path $PSScriptRoot 'src/net/minecraft/src/WorldNameUtf16_11.java')
if ($LASTEXITCODE -ne 0){throw 'Original WorldInfo probe compilation failed.'}
& $env:ORACLE_JAVA -cp ($taskBuild+';'+$taskCp) net.minecraft.src.WorldNameUtf16_11 $taskVectors
if ($LASTEXITCODE -ne 0){throw 'Original WorldInfo/NBT callback failed.'}
$taskActual=(Get-FileHash (Join-Path $taskVectors 'worldname_utf16_11.txt')).Hash
$taskExpected=(Get-FileHash (Join-Path $taskRoot 'crates/worldgen11/src/fixtures/worldname_utf16_11.txt')).Hash
if ($taskActual -ne $taskExpected){throw 'WorldInfo UTF16 original fixture drift.'}
Write-Output 'ORIGINAL1.1 WORLDINFO NBT UTF16 EXACT: 65550 cases'
if (-not $SkipRust){Push-Location $taskRoot;try{& cargo test --release -p worldgen11 --lib world_name_utf16_matches_original_11_nbt_and_reopen -- --nocapture;if($LASTEXITCODE -ne 0){throw 'WorldInfo production parity failed.'};& cargo test --release -p worldgen11 --lib nbt::tests -- --test-threads=1;if($LASTEXITCODE -ne 0){throw 'Existing NBT checks failed.'}}finally{Pop-Location}}
