$ErrorActionPreference='Stop'
Push-Location (Split-Path $PSScriptRoot -Parent)
try {
    if (!$env:ORACLE_JAVA) { throw 'Set ORACLE_JAVA to Java 8u51' }
    $cp='tools/oracle11/probes/build;tools/oracle11/build;tools/retromcp-r11/minecraft/jars/deobfuscated.jar'
    javac -cp $cp -d tools/oracle11/probes/build tools/oracle11/probes/LayerProbe11.java
    if ($LASTEXITCODE) { throw 'Layer compilation failed' }
    cargo build --release -p worldgen11 --bin genlayer11_probe --bin terrain_compare11
    if ($LASTEXITCODE) { throw 'Rust compilation failed' }
    foreach ($seed in @('0','42','-7654321','9223372036854775807','-9223372036854775808','123456789')) {
        foreach ($mode in @('layer','flat','flatworld')) {
            $base="target/r11/percentage-$mode-$seed"
            & $env:ORACLE_JAVA -cp $cp net.minecraft.src.LayerProbe11 $seed "$base-java.bin" $mode
            if ($LASTEXITCODE) { throw 'Original layer failed' }
            & target/release/genlayer11_probe.exe $seed "$base-rust.bin" $mode
            if ($LASTEXITCODE) { throw 'Rust layer failed' }
            if ((Get-FileHash "$base-java.bin").Hash -ne (Get-FileHash "$base-rust.bin").Hash) { throw "Layer mismatch $mode $seed" }
            Write-Output "EXACT $mode $seed $((Get-Item "$base-java.bin").Length) bytes"
        }
    }
    foreach ($seed in @('0','42','-1','9223372036854775807','-9223372036854775808')) {
        $f="target/r11/percentage-biomes-$seed.bin"
        & $env:ORACLE_JAVA -cp $cp net.minecraft.src.Mode_r10world3biomes r10world3biomes $seed $f
        if ($LASTEXITCODE) { throw 'Original biome failed' }
        $result=& target/release/terrain_compare11.exe world3 biomes $seed $f
        $result | Write-Output
        if ($LASTEXITCODE -or !(($result -join "`n").Contains('EXACT MATCH'))) { throw "Biome mismatch $seed" }
    }
    Write-Output 'LAYERS AND BIOMES CURRENT EXACT'
} finally { Pop-Location }
