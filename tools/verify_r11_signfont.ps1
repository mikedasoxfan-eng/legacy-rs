param([switch]$SkipRust)
$ErrorActionPreference='Stop'
Push-Location (Split-Path $PSScriptRoot -Parent)
try {
    if (!$env:ORACLE_JAVA) { throw 'Set ORACLE_JAVA to Java 8u51' }
    $taskBash=(Get-Command bash -ErrorAction Stop).Source
    & $taskBash tools/oracle11/oracle.sh signfont11 sign-font11.bin
    if ($LASTEXITCODE) { throw 'Original sign/font probe failed' }
    $taskFixture=(Resolve-Path tools/oracle11/out/sign-font11.bin).Path
    & $taskBash tools/oracle11/oracle.sh normalpack11 normal-pack11.bin
    if ($LASTEXITCODE) { throw 'Original normal-packing probe failed' }
    if (!$SkipRust) {
        $taskPrevious=[Environment]::GetEnvironmentVariable('SIGN_FONT11_FIXTURE')
        $taskPreviousNormal=[Environment]::GetEnvironmentVariable('NORMAL_PACK11_FIXTURE')
        try {
            $env:SIGN_FONT11_FIXTURE=$taskFixture
            $env:NORMAL_PACK11_FIXTURE=(Resolve-Path tools/oracle11/out/normal-pack11.bin).Path
            & cargo test --release -p client11 --lib original_normal_packing -- --ignored --nocapture
            if ($LASTEXITCODE) { throw 'Normal-packing comparison failed' }
            & cargo test --release -p client11 --lib original_sign_then_gui_shared_font -- --ignored --nocapture
            if ($LASTEXITCODE) { throw 'Sign/font comparison failed' }
            & cargo test --release -p client11 --lib unicode_sign_pages_upload_headlessly -- --ignored --nocapture
            if ($LASTEXITCODE) { throw 'Unicode sign GPU upload failed' }
        } finally {
            [Environment]::SetEnvironmentVariable('SIGN_FONT11_FIXTURE',$taskPrevious)
            [Environment]::SetEnvironmentVariable('NORMAL_PACK11_FIXTURE',$taskPreviousNormal)
        }
    }
    Get-FileHash -LiteralPath $taskFixture -Algorithm SHA256 | Format-List
    Write-Output 'R11 SIGN FONT EXACT'
} finally {Pop-Location}
