$ErrorActionPreference='Stop'
Push-Location (Split-Path $PSScriptRoot -Parent)
try {
    bash tools/oracle11/oracle.sh sheeprender11 sheep-render11.bin
    if ($LASTEXITCODE) { throw 'Original sheep renderer probe failed' }
    $saved=[Environment]::GetEnvironmentVariable('SHEEP_RENDER11_FIXTURE')
    try {
        $env:SHEEP_RENDER11_FIXTURE=(Resolve-Path tools/oracle11/out/sheep-render11.bin).Path
        cargo test --release -p client11 --lib sheep_live_bridge_models_and_renderer_match_original_11 -- --ignored --nocapture
        if ($LASTEXITCODE) { throw 'Sheep live renderer comparison failed' }
    } finally { [Environment]::SetEnvironmentVariable('SHEEP_RENDER11_FIXTURE',$saved) }
    Write-Output 'R11 SHEEP LIVE RENDER EXACT'
} finally { Pop-Location }
