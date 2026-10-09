$ErrorActionPreference='Stop'
Push-Location (Split-Path $PSScriptRoot -Parent)
try {
 $cp='tools/oracle11/probes/build;tools/oracle11/build;tools/retromcp-r11/minecraft/jars/deobfuscated.jar'
 javac -encoding UTF-8 -cp $cp -d tools/oracle11/probes/build tools/oracle11/probes/LanguageFormatProbe11.java
 if ($LASTEXITCODE) {throw 'Original helper compile failed'}
 & $env:ORACLE_JAVA -cp $cp net.minecraft.src.LanguageFormatProbe11 ../language-format11-original.tsv
 if ($LASTEXITCODE) {throw 'Original helper failed'}
 New-Item -ItemType Directory -Force target/r11/formatter | Out-Null
 python tools/r11/format_corpus11.py ../language-format11-original.tsv target/r11/formatter/strings.bin
 if ($LASTEXITCODE) {throw 'Prepare corpus failed'}
 rustc --edition=2021 -Awarnings tools/r11/format_check11.rs -o target/r11/formatter/format_check11.exe
 if ($LASTEXITCODE) {throw 'Rust helper compile failed'}
 & target/r11/formatter/format_check11.exe target/r11/formatter/strings.bin
 if ($LASTEXITCODE) {throw 'Formatter mismatch'}
}finally{Pop-Location}
