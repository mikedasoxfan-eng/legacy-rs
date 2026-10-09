$ErrorActionPreference='Stop'
Push-Location (Split-Path (Split-Path $PSScriptRoot -Parent) -Parent)
try {
 $cp='tools/oracle11/probes/build;tools/oracle11/build/input;tools/retromcp-r11/minecraft/jars/deobfuscated.jar;tools/oracle11/build;tools/oracle11/build/lwjgl-unsealed.jar'
 javac -encoding UTF-8 -cp $cp -d tools/oracle11/probes/build tools/oracle11/probes/MinecraftConvert11.java
 if($LASTEXITCODE){throw 'Minecraft conversion probe compilation failed'}
 & $env:ORACLE_JAVA -cp $cp net.minecraft.src.MinecraftConvert11 target/r11/minecraft-convert11.txt
 if($LASTEXITCODE){throw 'Original conversion callback failed'}
 $actual=(Get-Content target/r11/minecraft-convert11.txt -Raw).Replace("`r`n","`n")
 $expected=(Get-Content crates/client11/src/fixtures/minecraft-convert11.txt -Raw).Replace("`r`n","`n")
 if($actual -ne $expected){throw 'Original conversion callback fixture drift'}
 cargo test -p client11 --lib conversion11_tests -- --nocapture
 if($LASTEXITCODE){throw 'Minecraft conversion settings comparison failed'}
 Write-Output 'MINECRAFT CONVERSION CALLBACK EXACT: unchanged private callback selects DEFAULT; worker uses matching settings'
}finally{Pop-Location}
