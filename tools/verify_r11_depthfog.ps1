# Original World.randomDisplayUpdates bytecode versus Rust; configure ORACLE_JAVA first.
$ErrorActionPreference = 'Stop'
Push-Location (Split-Path $PSScriptRoot -Parent)
try {
    if (!$env:ORACLE_JAVA) { throw 'Set ORACLE_JAVA to the original Minecraft Java 8 runtime.' }
    $cp='tools/oracle11/probes/build;tools/oracle11/build;tools/retromcp-r11/minecraft/jars/deobfuscated.jar'
    New-Item -ItemType Directory -Force tools/oracle11/probes/build,target/r11/depthfog | Out-Null
    javac -cp $cp -d tools/oracle11/probes/build tools/oracle11/probes/DepthFogProbe11.java
    if ($LASTEXITCODE) { throw 'Original probe compilation failed' }
    cargo build --release -p worldgen11 --bin genlayer11_probe
    if ($LASTEXITCODE) { throw 'Rust probe compilation failed' }
    foreach ($seed in @('0','42','-1','123456789','9223372036854775807','-9223372036854775808')) {
        $java="target/r11/depthfog/java-$seed.bin"; $rust="target/r11/depthfog/rust-$seed.bin"
        & $env:ORACLE_JAVA -cp $cp net.minecraft.src.DepthFogProbe11 $seed $java
        if ($LASTEXITCODE) { throw "Original probe failed: $seed" }
        & target/release/genlayer11_probe.exe $seed $rust depthfog
        if ($LASTEXITCODE) { throw "Rust probe failed: $seed" }
        if ((Get-FileHash -LiteralPath $java).Hash -ne (Get-FileHash -LiteralPath $rust).Hash) { throw "Depth fog mismatch: $seed" }
        Write-Host "EXACT MATCH: depthfog $seed ($((Get-Item -LiteralPath $java).Length) bytes)"
    }
    Write-Host 'R11 DEPTH FOG DISPLAY LOOP EXACT'
} finally { Pop-Location }
