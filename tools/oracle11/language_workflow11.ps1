$ErrorActionPreference='Stop'
Push-Location (Split-Path (Split-Path $PSScriptRoot -Parent) -Parent)
try {
 $libs=Get-ChildItem tools/retromcp-r11/libraries -Recurse -Filter '*.jar' | Where-Object {$_.Name -notmatch 'sources|natives|^lwjgl-2'} | ForEach-Object {$_.FullName}
 $cp='tools/oracle11/probes/build;tools/oracle11/build/input;tools/retromcp-r11/minecraft/jars/deobfuscated.jar;tools/oracle11/build;tools/oracle11/build/lwjgl-unsealed.jar;'+($libs -join ';')
 javac -encoding UTF-8 -cp $cp -d tools/oracle11/probes/build tools/oracle11/probes/LanguageWorkflow11.java
 if($LASTEXITCODE){throw 'Language workflow compile failed'}
 & $env:ORACLE_JAVA -cp $cp net.minecraft.src.LanguageWorkflow11 target/r11/language-workflow11.bin target/r11/language-workflow11-java
 if($LASTEXITCODE){throw 'Original language workflow failed'}
 Get-FileHash target/r11/language-workflow11.bin
}finally{Pop-Location}
