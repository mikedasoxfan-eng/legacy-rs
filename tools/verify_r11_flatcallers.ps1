$ErrorActionPreference='Stop'
Push-Location (Split-Path $PSScriptRoot -Parent)
try {
    if (!$env:ORACLE_JAVA) {throw 'Set ORACLE_JAVA to Java 8u51'}
    $cp='tools/oracle11/probes/build;tools/oracle11/build;tools/retromcp-r11/minecraft/jars/deobfuscated.jar'
    javac -cp $cp -d tools/oracle11/probes/build tools/oracle11/probes/FlatCallerProbe11.java
    if ($LASTEXITCODE) {throw 'Java compilation failed'}
    cargo build --release -p worldgen11 --bin genlayer11_probe
    if ($LASTEXITCODE) {throw 'Rust compilation failed'}
    foreach ($seed in @('0','42','-1','123456789','9223372036854775807','-9223372036854775808')) {
        $base="target/r11/flatcallers/$seed"
        New-Item -ItemType Directory -Force $base | Out-Null
        & $env:ORACLE_JAVA -cp $cp net.minecraft.src.FlatCallerProbe11 $seed "$base/java.bin" "$base/java-leaf.bin"
        if ($LASTEXITCODE) {throw 'Original probe failed'}
        & target/release/genlayer11_probe.exe $seed "$base/rust.bin" flatcallers
        if ($LASTEXITCODE) {throw 'Rust probe failed'}
        $j=(Get-FileHash "$base/java.bin").Hash
        $r=(Get-FileHash "$base/rust.bin").Hash
        if ($j -ne $r) {throw "Mismatch $seed"}
        Write-Host "EXACT MATCH: flatcallers $seed $j ($((Get-Item "$base/java.bin").Length) bytes)"
    }
} finally {Pop-Location}
