$ErrorActionPreference='Stop'
Push-Location (Split-Path $PSScriptRoot -Parent)
try {
 $libs=Get-ChildItem tools/retromcp-r11/libraries -Recurse -Filter '*.jar' | Where-Object {$_.Name -notmatch 'sources|natives|^lwjgl-2'} | ForEach-Object {$_.FullName}
 $cp='tools/oracle11/probes/build;tools/oracle11/build/input;tools/retromcp-r11/minecraft/jars/deobfuscated.jar;tools/oracle11/build;tools/oracle11/build/lwjgl-unsealed.jar;'+($libs -join ';')
 New-Item -ItemType Directory -Force target/r11 | Out-Null
 javac -cp $cp -d tools/oracle11/probes/build tools/oracle11/probes/HeldWorkflowProbe11.java
 if ($LASTEXITCODE) {throw 'Held workflow probe compilation failed'}
 & $env:ORACLE_JAVA -cp $cp net.minecraft.src.HeldWorkflowProbe11 target/r11/held-workflow11.bin
 if ($LASTEXITCODE) {throw 'Original held workflow probe failed'}
 $saved=[Environment]::GetEnvironmentVariable('HELD_WORKFLOW11_FIXTURE')
 try {
  $env:HELD_WORKFLOW11_FIXTURE=(Resolve-Path target/r11/held-workflow11.bin).Path
  cargo test -p client11 --lib held_workflows_match_original_11 -- --ignored --nocapture
  if ($LASTEXITCODE) {throw 'Held workflow comparison failed'}
 } finally {[Environment]::SetEnvironmentVariable('HELD_WORKFLOW11_FIXTURE',$saved)}
 Write-Output 'R11 HELD WORKFLOW EXACT'
} finally {Pop-Location}
