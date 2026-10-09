$ErrorActionPreference='Stop'
Push-Location (Split-Path $PSScriptRoot -Parent)
try {
 & tools/oracle11/itempass_render11.ps1
 if ($LASTEXITCODE) {throw 'Original item renderer generation failed'}
 $env:ITEMPASS11_FIXTURE=(Resolve-Path target/r11/itempass-render11.bin).Path
 $env:ITEMPASS11_HELD_FIXTURE=(Resolve-Path target/r11/itempass-render11.held.bin).Path
 cargo test -p client11 --lib item_passes_match_original_11 -- --ignored --nocapture
 if ($LASTEXITCODE) {throw 'Item renderer comparison failed'}
} finally {
 Remove-Item Env:ITEMPASS11_FIXTURE,Env:ITEMPASS11_HELD_FIXTURE -ErrorAction SilentlyContinue
 Pop-Location
}
