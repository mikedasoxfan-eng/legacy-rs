param([switch]$SkipPackage,[switch]$PackageOnly)
$ErrorActionPreference='Stop'
Push-Location (Split-Path $PSScriptRoot -Parent)
try {
 if(!$PackageOnly){
 # Replay freshly retained original fixtures in one compiled production library.
 $fixtures=@{
  FONT_BIDI11_FIXTURE='target/r11/font-bidi-draw11.bin';FONT_WRAP11_FIXTURE='target/r11/font-wrap11.bin';LANGUAGE_SCREEN11_FIXTURE='target/r11/language-screen11.bin'
  HELD_WORKFLOW11_FIXTURE='target/r11/held-workflow11.bin';BIPED_WORKFLOW11_FIXTURE='target/r11/biped-workflow11.bin';SKY_HORIZON11_FIXTURE='target/r11/sky-horizon11.bin'
  SIGN_FONT11_FIXTURE='tools/oracle11/out/sign-font11.bin';NORMAL_PACK11_FIXTURE='tools/oracle11/out/normal-pack11.bin';SHEEP_RENDER11_FIXTURE='tools/oracle11/out/sheep-render11.bin'
  ITEMPASS11_FIXTURE='target/r11/itempass-render11.bin';ITEMPASS11_HELD_FIXTURE='target/r11/itempass-render11.held.bin';BURNING_LIGHT11_FIXTURE='target/r11/burning-light11.bin';CREATIVE_FOG11_FIXTURE='target/r11/creative-fog11.bin'
  GUI_BREWING11_FIXTURE='target/r11/gui-brewing11.bin';SILVERFISH_WORKFLOW11_FIXTURE='target/r11/silverfish-workflow11.bin';FOG_WORKFLOW11_FIXTURE='target/r11/fog-workflow11.bin'
  LANGUAGE_WORKFLOW11_FIXTURE='target/r11/language-workflow11.bin';SMALL_GUI_MENUDRAW11_FIXTURE='target/r11/small-gui-menudraw11.bin';SMALL_GUI_LANGUAGE11_FIXTURE='target/r11/small-gui-language11.bin';CREATE_WORLD_TYPE11_FIXTURE='target/r11/create-world-type-gui11.bin'
  GUI_STAT_LANGUAGE11_FIXTURE='target/r11/gui-stat-language11.bin';INVENTORY_POTENCY11_FIXTURE='target/r11/inventory-potency11.bin';UNICODE_INPUT11_FIXTURE='target/r11/unicode-input11.bin';CONTAINER_RELEASE11_FIXTURE='target/r11/container-release11.bin';INGAME_VERSION11_FIXTURE='target/r11/ingame-version11.bin'
  POTION_NAMES11_FIXTURE='target/r11/potion-names11.bin';CREATE_WORLD_UNICODE11_FIXTURE='target/r11/create-world-unicode11.bin';ACHIEVEMENT_TOOLTIP11_FIXTURE='target/r11/achievement-tooltip11.bin'
 }
 $saved=@{};foreach($key in $fixtures.Keys){$saved[$key]=[Environment]::GetEnvironmentVariable($key);[Environment]::SetEnvironmentVariable($key,(Resolve-Path $fixtures[$key]).Path)}
 $saved['FONT_PRIMITIVES11_DIRECTORY']=[Environment]::GetEnvironmentVariable('FONT_PRIMITIVES11_DIRECTORY');$env:FONT_PRIMITIVES11_DIRECTORY=(Resolve-Path target/r11).Path
 $saved['BIDI11_FIXTURES']=[Environment]::GetEnvironmentVariable('BIDI11_FIXTURES');$env:BIDI11_FIXTURES=(Resolve-Path target/r11/bidi11.bin).Path+';'+(Resolve-Path target/r11/bidi-boundary11.bin).Path
 try {
  cargo test -p client11 --lib -- --include-ignored --test-threads=1 --nocapture
  if($LASTEXITCODE){throw 'Complete1.1 production client library checks failed'}
  cargo test -p worldgen11 --lib -- --test-threads=1 --nocapture
  if($LASTEXITCODE){throw 'Complete1.1 simulation library checks failed'}
  cargo test -p worldgen11 --test potion_names11 -- --ignored --nocapture
  if($LASTEXITCODE){throw 'Potion name original corpus failed'}
  cargo test --workspace --lib -- --test-threads=1
  if($LASTEXITCODE){throw 'Cross-version library regressions failed'}
 }finally{foreach($key in $saved.Keys){[Environment]::SetEnvironmentVariable($key,$saved[$key])}}
 }
 if(!$SkipPackage){
  cargo build --release --workspace
  if($LASTEXITCODE){throw 'Release workspace packaging build failed'}
  & target/release/beta.exe --version
  if($LASTEXITCODE){throw 'Packaged version smoke failed'}
  foreach($version in @('b1.7.3','b1.8.1','1.0','1.1')) {
   & target/release/beta.exe --help --mc $version
   if($LASTEXITCODE){throw "Packaged version dispatch failed: $version"}
  }
  Get-FileHash target/release/beta.exe -Algorithm SHA256
 }
 if($PackageOnly){Write-Output 'R11 PACKAGE CHECKS PASS'}elseif($SkipPackage){Write-Output 'R11 PRODUCTION SUITE AND CROSS-VERSION REGRESSIONS PASS'}else{Write-Output 'R11 FINAL PRODUCTION SUITE, CROSS-VERSION REGRESSIONS AND PACKAGE CHECKS PASS'}
}finally{Pop-Location}


