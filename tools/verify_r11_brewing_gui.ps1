$ErrorActionPreference='Stop'
Push-Location (Split-Path $PSScriptRoot -Parent)
try {
    python tools/r11/gui2_oracle.py gui2brewing10 target/r11/gui-brewing11.bin
    if ($LASTEXITCODE) { throw 'Original GUI probe failed' }
    $saved=[Environment]::GetEnvironmentVariable('GUI_BREWING11_FIXTURE')
    try {
        $env:GUI_BREWING11_FIXTURE=(Resolve-Path target/r11/gui-brewing11.bin).Path
        cargo test --release -p client11 --lib brewing_background_matches_original_11 -- --ignored --nocapture
        if ($LASTEXITCODE) { throw 'GUI brewing comparison failed' }
    } finally { [Environment]::SetEnvironmentVariable('GUI_BREWING11_FIXTURE',$saved) }
    Write-Output 'R11 BREWING GUI EXACT'
} finally { Pop-Location }
