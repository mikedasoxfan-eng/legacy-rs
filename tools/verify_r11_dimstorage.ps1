$ErrorActionPreference='Stop'
Push-Location (Split-Path $PSScriptRoot -Parent)
try {
    if (!$env:ORACLE_JAVA) { throw 'Set ORACLE_JAVA to Java 8u51' }
    $cp='tools/oracle11/probes/build;tools/oracle11/build;tools/retromcp-r11/minecraft/jars/deobfuscated.jar'
    javac -cp $cp -d tools/oracle11/probes/build tools/oracle11/probes/DimensionStorageProbe11.java
    if ($LASTEXITCODE) { throw 'Java probe compilation failed' }
    cargo build --release -p worldgen11 --bin genlayer11_probe
    if ($LASTEXITCODE) { throw 'Rust probe compilation failed' }
    $run=[guid]::NewGuid().ToString('N')
    foreach ($seed in @('0','42','-1')) {
        $base="target/r11/dimstorage/$run/$seed"
        New-Item -ItemType Directory -Force "$base/java-save","$base/rust-save" | Out-Null
        & $env:ORACLE_JAVA -cp $cp net.minecraft.src.DimensionStorageProbe11 $seed "$base/java.bin" "$base/java-save" write
        if ($LASTEXITCODE) { throw 'Original write/reopen failed' }
        & target/release/genlayer11_probe.exe $seed "$base/rust.bin" dimstorage "$base/rust-save" write
        if ($LASTEXITCODE) { throw 'Rust write/reopen failed' }
        & $env:ORACLE_JAVA -cp $cp net.minecraft.src.DimensionStorageProbe11 $seed "$base/java-read-rust.bin" "$base/rust-save" read
        if ($LASTEXITCODE) { throw 'Original reading Rust save failed' }
        & target/release/genlayer11_probe.exe $seed "$base/rust-read-java.bin" dimstorage "$base/java-save" read
        if ($LASTEXITCODE) { throw 'Rust reading original save failed' }
        $expected=(Get-FileHash -LiteralPath "$base/java.bin").Hash
        foreach ($kind in @('rust','java-read-rust','rust-read-java')) {
            Write-Host "== dimstorage $kind seed $seed"
            if ((Get-FileHash -LiteralPath "$base/$kind.bin").Hash -ne $expected) { throw "Dimension storage mismatch: $kind $seed" }
            Write-Host "EXACT MATCH $expected ($((Get-Item -LiteralPath "$base/$kind.bin").Length) bytes) $base"
        }
    }
} finally { Pop-Location }
