$ErrorActionPreference='Stop'
Push-Location (Split-Path $PSScriptRoot -Parent)
try {
 $libs=Get-ChildItem tools/retromcp-r11/libraries -Recurse -Filter '*.jar' | Where-Object {$_.Name -notmatch 'sources|natives|^lwjgl-2'} | ForEach-Object {$_.FullName}
 $cp='tools/oracle11/probes/build;tools/oracle11/build/input;tools/retromcp-r11/minecraft/jars/deobfuscated.jar;tools/oracle11/build;tools/oracle11/build/lwjgl-unsealed.jar;'+($libs -join ';')
 javac -cp $cp -d tools/oracle11/probes/build tools/oracle11/probes/BurningLightProbe11.java
 if ($LASTEXITCODE) {throw 'Probe compilation failed'}
 & $env:ORACLE_JAVA -cp $cp net.minecraft.src.BurningLightProbe11 target/r11/burning-light11.bin
 if ($LASTEXITCODE) {throw 'Original comparison generation failed'}
 $env:BURNING_LIGHT11_FIXTURE=(Resolve-Path target/r11/burning-light11.bin).Path
 cargo test -p client11 --lib burning_light_matches_original_11 -- --ignored --nocapture
 if ($LASTEXITCODE) {throw 'Burning light comparison failed'}
} finally {Remove-Item Env:BURNING_LIGHT11_FIXTURE -ErrorAction SilentlyContinue;Pop-Location}
