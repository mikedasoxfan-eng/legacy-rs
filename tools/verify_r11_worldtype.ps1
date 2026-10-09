$ErrorActionPreference='Stop'
Push-Location (Split-Path $PSScriptRoot -Parent)
try {
    if (!$env:ORACLE_JAVA) { throw 'Set ORACLE_JAVA to Java 8u51' }
    $cp='tools/oracle11/probes/build;tools/oracle11/build;tools/retromcp-r11/minecraft/jars/deobfuscated.jar'
    javac -cp $cp -d tools/oracle11/probes/build tools/oracle11/probes/WorldTypeProbe11.java
    if ($LASTEXITCODE) { throw 'Java probe compilation failed' }
    cargo build --release -p worldgen11 --bin genlayer11_probe
    if ($LASTEXITCODE) { throw 'Rust probe compilation failed' }
    $run=[guid]::NewGuid().ToString('N')
    foreach ($seed in @('0','42','-1')) {
        $base="target/r11/worldtype/$run/$seed"
        New-Item -ItemType Directory -Force "$base/java-save","$base/rust-save" | Out-Null
        & $env:ORACLE_JAVA -cp $cp net.minecraft.src.WorldTypeProbe11 $seed "$base/java.bin" "$base/java-save"
        if ($LASTEXITCODE) { throw 'Original probe failed' }
        & target/release/genlayer11_probe.exe $seed "$base/rust.bin" worldtype "$base/rust-save"
        if ($LASTEXITCODE) { throw 'Rust probe failed' }
        $j=(Get-FileHash -LiteralPath "$base/java.bin").Hash
        $r=(Get-FileHash -LiteralPath "$base/rust.bin").Hash
        if ($j -ne $r) { throw "World type mismatch: $seed" }
        Write-Host "EXACT MATCH: worldtype $seed $j ($((Get-Item -LiteralPath "$base/java.bin").Length) bytes) $base"
    }
} finally { Pop-Location }
