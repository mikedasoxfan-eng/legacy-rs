$ErrorActionPreference='Stop'
Push-Location (Split-Path (Split-Path $PSScriptRoot -Parent) -Parent)
try {
 $libs=Get-ChildItem tools/retromcp-r11/libraries -Recurse -Filter '*.jar' | Where-Object {$_.Name -notmatch 'sources|natives|^lwjgl-2'} | ForEach-Object {$_.FullName}
 $cp='tools/oracle11/probes/build;tools/oracle11/build/input;tools/retromcp-r11/minecraft/jars/deobfuscated.jar;tools/oracle11/build;tools/oracle11/build/lwjgl-unsealed.jar;'+($libs -join ';')
 javac -encoding UTF-8 -cp $cp -d tools/oracle11/probes/build tools/oracle11/probes/PotionNames11.java
 if($LASTEXITCODE){throw 'Potion original probe compilation failed'}
 & $env:ORACLE_JAVA -cp $cp net.minecraft.src.PotionNames11 target/r11/potion-names11.bin
 if($LASTEXITCODE){throw 'Original potion probe failed'}
 $env:POTION_NAMES11_FIXTURE=(Resolve-Path target/r11/potion-names11.bin).Path
 cargo test -p worldgen11 --test potion_names11 -- --ignored --nocapture
 if($LASTEXITCODE){throw 'Original potion name/tooltip comparison failed'}
}finally{Pop-Location}
