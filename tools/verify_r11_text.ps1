param([string]$McpPath='tools/retromcp-r11',[switch]$SkipRust)
$ErrorActionPreference='Stop'
$repoRoot=Split-Path $PSScriptRoot -Parent
Push-Location $repoRoot
try {
 $mcpRoot=(Resolve-Path $McpPath).Path
 & ./tools/verify_r11_bidi.ps1 -McpPath $mcpRoot -SkipRust
 $jar=Join-Path $mcpRoot 'minecraft/jars/deobfuscated.jar'
 $source=Join-Path $repoRoot 'tools/oracle11/probes/fontdraw'
 $build=Join-Path $source 'build'
 $libs=Join-Path $repoRoot 'target/r11/font-libs'
 & python tools/r11/prepare_font_libs11.py $mcpRoot $libs
 if($LASTEXITCODE -ne 0){throw 'Library preparation failed'}
 $otherLibs=(Get-ChildItem (Join-Path $mcpRoot 'libraries') -Recurse -Filter '*.jar' | Where-Object {$_.Name -notmatch 'sources|natives|^lwjgl'} | ForEach-Object FullName) -join ';'
 $cp=$build+';'+$jar+';'+(Join-Path $libs '*')+';'+$otherLibs
 & javac -encoding UTF-8 -cp $cp -d $build (Join-Path $source 'FontBidiDraw11.java') (Join-Path $source 'FontWrap11.java') (Join-Path $source 'LanguageScreen11.java')
 if($LASTEXITCODE -ne 0){throw 'Text probes compile failed'}
 foreach($probe in @(@('FontBidiDraw11','font-bidi-draw11.bin'),@('FontWrap11','font-wrap11.bin'),@('LanguageScreen11','language-screen11.bin'))){
  & $env:ORACLE_JAVA -Xmx1G -cp $cp ('net.minecraft.src.'+$probe[0]) ('target/r11/'+$probe[1])
  if($LASTEXITCODE -ne 0){throw ($probe[0]+' failed')}
 }
 if(!$SkipRust){
  $savedEnv=@{};foreach($key in @('BIDI11_FIXTURES','FONT_BIDI11_FIXTURE','FONT_WRAP11_FIXTURE','LANGUAGE_SCREEN11_FIXTURE')){$savedEnv[$key]=[Environment]::GetEnvironmentVariable($key)}
  try {
   $env:BIDI11_FIXTURES=(Resolve-Path target/r11/bidi11.bin).Path+';'+(Resolve-Path target/r11/bidi-boundary11.bin).Path
   $env:FONT_BIDI11_FIXTURE=(Resolve-Path target/r11/font-bidi-draw11.bin).Path
   $env:FONT_WRAP11_FIXTURE=(Resolve-Path target/r11/font-wrap11.bin).Path
   $env:LANGUAGE_SCREEN11_FIXTURE=(Resolve-Path target/r11/language-screen11.bin).Path
   foreach($test in @('gui::bidi11::tests::original_bytecode_corpora','gui::font::bidi_draw_tests::original_public_draw_methods','gui::font::bidi_draw_tests::original_wrapped_draw_methods','gui::screens::language::tests::original_language_callbacks')){
    & cargo test --release -p client11 --lib $test -- --ignored --nocapture
    if($LASTEXITCODE -ne 0){throw ('Production comparison failed: '+$test)}
   }
  } finally {foreach($key in $savedEnv.Keys){[Environment]::SetEnvironmentVariable($key,$savedEnv[$key])}}
 }
 Get-FileHash $jar,target/r11/font-bidi-draw11.bin,target/r11/font-wrap11.bin,target/r11/language-screen11.bin -Algorithm SHA256 | Format-List
 Write-Output 'R11 TEXT AND LANGUAGE FOCUSED EXACT'
} finally {Pop-Location}
