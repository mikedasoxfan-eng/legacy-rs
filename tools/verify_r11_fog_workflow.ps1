$ErrorActionPreference='Stop'
Push-Location (Split-Path $PSScriptRoot -Parent)
try {
 $libs=Get-ChildItem tools/retromcp-r11/libraries -Recurse -Filter '*.jar' | Where-Object {$_.Name -notmatch 'sources|natives|^lwjgl-2'} | ForEach-Object {$_.FullName}
 $cp='tools/oracle11/probes/build;tools/oracle11/build/input;tools/retromcp-r11/minecraft/jars/deobfuscated.jar;tools/oracle11/build;tools/oracle11/build/lwjgl-unsealed.jar;'+($libs -join ';')
 New-Item -ItemType Directory -Force target/r11 | Out-Null
 javac -cp $cp -d tools/oracle11/probes/build tools/oracle11/probes/FogWorkflowProbe11.java
 if ($LASTEXITCODE) {throw 'Fog probe compilation failed'}
 & $env:ORACLE_JAVA -cp $cp net.minecraft.src.FogWorkflowProbe11 target/r11/fog-workflow11.bin
 if ($LASTEXITCODE) {throw 'Original fog probe failed'}
 $saved=[Environment]::GetEnvironmentVariable('FOG_WORKFLOW11_FIXTURE')
 try {
  $env:FOG_WORKFLOW11_FIXTURE=(Resolve-Path target/r11/fog-workflow11.bin).Path
  cargo test -p client11 --lib fog_workflow_matches_original_11 -- --ignored --nocapture
  if ($LASTEXITCODE) {throw 'Fog workflow comparison failed'}
 } finally {[Environment]::SetEnvironmentVariable('FOG_WORKFLOW11_FIXTURE',$saved)}
 Write-Output 'R11 FOG WORKFLOW EXACT'
} finally {Pop-Location}
