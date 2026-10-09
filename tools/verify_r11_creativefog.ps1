$ErrorActionPreference='Stop'
Push-Location (Split-Path $PSScriptRoot -Parent)
try {
 $libs=Get-ChildItem tools/retromcp-r11/libraries -Recurse -Filter '*.jar' | Where-Object {$_.Name -notmatch 'sources|natives|^lwjgl-2'} | ForEach-Object {$_.FullName}
 $cp='tools/oracle11/probes/build;tools/oracle11/build/input;tools/retromcp-r11/minecraft/jars/deobfuscated.jar;tools/oracle11/build;tools/oracle11/build/lwjgl-unsealed.jar;'+($libs -join ';')
 javac -cp $cp -d tools/oracle11/probes/build tools/oracle11/probes/CreativeFogProbe11.java
 if ($LASTEXITCODE) {throw 'Probe compilation failed'}
 & $env:ORACLE_JAVA -cp $cp net.minecraft.src.CreativeFogProbe11 target/r11/creative-fog11.bin
 if ($LASTEXITCODE) {throw 'Original generation failed'}
 $env:CREATIVE_FOG11_FIXTURE=(Resolve-Path target/r11/creative-fog11.bin).Path
 cargo test -p client11 --lib creative_fog_matches_original_11 -- --ignored --nocapture
 if ($LASTEXITCODE) {throw 'Creative fog comparison failed'}
}finally{Remove-Item Env:CREATIVE_FOG11_FIXTURE -ErrorAction SilentlyContinue;Pop-Location}
