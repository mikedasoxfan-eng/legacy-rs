$ErrorActionPreference='Stop'
Push-Location (Split-Path $PSScriptRoot -Parent)
try {
 $libs=Get-ChildItem tools/retromcp-r11/libraries -Recurse -Filter '*.jar' | Where-Object {$_.Name -notmatch 'sources|natives|^lwjgl-2'} | ForEach-Object {$_.FullName}
 $cp='tools/oracle11/probes/build;tools/oracle11/build/input;tools/retromcp-r11/minecraft/jars/deobfuscated.jar;tools/oracle11/build;tools/oracle11/build/lwjgl-unsealed.jar;'+($libs -join ';')
 New-Item -ItemType Directory -Force target/r11 | Out-Null
 javac -cp $cp -d tools/oracle11/probes/build tools/oracle11/probes/SkyHorizonProbe11.java
 if ($LASTEXITCODE) {throw 'Sky probe compilation failed'}
 & $env:ORACLE_JAVA -cp $cp net.minecraft.src.SkyHorizonProbe11 target/r11/sky-horizon11.bin
 if ($LASTEXITCODE) {throw 'Original sky probe failed'}
 $saved=[Environment]::GetEnvironmentVariable('SKY_HORIZON11_FIXTURE')
 try {
  $env:SKY_HORIZON11_FIXTURE=(Resolve-Path target/r11/sky-horizon11.bin).Path
  cargo test -p client11 --lib sky_horizon_matches_original_11 -- --ignored --nocapture
  if ($LASTEXITCODE) {throw 'Sky workflow comparison failed'}
 } finally {[Environment]::SetEnvironmentVariable('SKY_HORIZON11_FIXTURE',$saved)}
 Write-Output 'R11 SKY WORKFLOW EXACT'
} finally {Pop-Location}
