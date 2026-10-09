param([string]$McpPath='tools/retromcp-r11', [switch]$SkipRust)
$ErrorActionPreference='Stop'
$repoRoot=Split-Path $PSScriptRoot -Parent
Push-Location $repoRoot
try {
 $mcpRoot=(Resolve-Path $McpPath).Path
 $jar=Join-Path $mcpRoot 'minecraft/jars/deobfuscated.jar'
 $source=Join-Path $repoRoot 'tools/oracle11/probes/fontdraw'
 $build=Join-Path $source 'build'
 New-Item -ItemType Directory -Force $build,'target/r11' | Out-Null
 $cp=$build+';'+$jar+';'+(Join-Path $mcpRoot 'minecraft/jars/bin/*')
 & javac -encoding UTF-8 -cp $cp -d $build (Join-Path $source 'org/lwjgl/opengl/GL11.java') (Join-Path $source 'FontDraw11.java') (Join-Path $source 'BidiProbe11.java') (Join-Path $source 'BidiBoundaryProbe11.java')
 if($LASTEXITCODE -ne 0){throw 'Bidi probes compile failed'}
 if(!$env:ORACLE_JAVA){throw 'ORACLE_JAVA must name the pinned Mojang Java8u51 runtime'}
 & $env:ORACLE_JAVA -Xmx1G -cp $cp net.minecraft.src.BidiProbe11 target/r11/bidi11.bin target/r11/java8-bidi-classes.bin
 if($LASTEXITCODE -ne 0){throw 'Bidi corpus failed'}
 & $env:ORACLE_JAVA -Xmx1G -cp $cp net.minecraft.src.BidiBoundaryProbe11 target/r11/bidi-boundary11.bin
 if($LASTEXITCODE -ne 0){throw 'Bidi boundary corpus failed'}
 & python tools/r11/generate_bidi_data.py --verify-java-data target/r11/java8-bidi-classes.bin
 if($LASTEXITCODE -ne 0){throw 'Bidi class data differs'}
 if(!$SkipRust){
  $priorFixtures=$env:BIDI11_FIXTURES
  try {
   $env:BIDI11_FIXTURES=(Resolve-Path target/r11/bidi11.bin).Path+';'+(Resolve-Path target/r11/bidi-boundary11.bin).Path
   & cargo test --release -p client11 --lib gui::bidi11::tests::original_bytecode_corpora -- --ignored --nocapture
   if($LASTEXITCODE -ne 0){throw 'Production Bidi comparison failed'}
  } finally {$env:BIDI11_FIXTURES=$priorFixtures}
 }
 Get-FileHash $jar,target/r11/bidi11.bin,target/r11/bidi-boundary11.bin,target/r11/java8-bidi-classes.bin -Algorithm SHA256 | Format-List
 Write-Output 'R11 BIDI ORIGINAL CORPORA EXACT'
} finally {Pop-Location}
