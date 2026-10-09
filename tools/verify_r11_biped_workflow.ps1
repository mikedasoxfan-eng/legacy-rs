$ErrorActionPreference='Stop'
Push-Location (Split-Path $PSScriptRoot -Parent)
try {
 $libs=Get-ChildItem tools/retromcp-r11/libraries -Recurse -Filter '*.jar' | Where-Object {$_.Name -notmatch 'sources|natives|^lwjgl-2'} | ForEach-Object {$_.FullName}
 $cp='tools/oracle11/probes/build;tools/oracle11/build/input;tools/retromcp-r11/minecraft/jars/deobfuscated.jar;tools/oracle11/build;tools/oracle11/build/lwjgl-unsealed.jar;'+($libs -join ';')
 New-Item -ItemType Directory -Force target/r11 | Out-Null
 javac -cp $cp -d tools/oracle11/probes/build tools/oracle11/probes/BipedWorkflowProbe11.java
 if ($LASTEXITCODE) {throw 'Biped probe compilation failed'}
 & $env:ORACLE_JAVA -cp $cp net.minecraft.src.BipedWorkflowProbe11 target/r11/biped-workflow11.bin
 if ($LASTEXITCODE) {throw 'Original biped probe failed'}
 $saved=[Environment]::GetEnvironmentVariable('BIPED_WORKFLOW11_FIXTURE')
 try {
  $env:BIPED_WORKFLOW11_FIXTURE=(Resolve-Path target/r11/biped-workflow11.bin).Path
  cargo test -p client11 --lib biped_workflow_matches_original_11 -- --ignored --nocapture
  if ($LASTEXITCODE) {throw 'Biped workflow comparison failed'}
 } finally {[Environment]::SetEnvironmentVariable('BIPED_WORKFLOW11_FIXTURE',$saved)}
 Write-Output 'R11 BIPED WORKFLOW EXACT'
} finally {Pop-Location}
