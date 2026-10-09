param([string]$Output='target/r11/itempass-render11.bin')
$ErrorActionPreference='Stop'
Push-Location (Split-Path (Split-Path $PSScriptRoot -Parent) -Parent)
try {
    if (!$env:ORACLE_JAVA) {throw 'Source toolchain env.ps1 first; Java8u51 required'}
    $libs=Get-ChildItem tools/retromcp-r11/libraries -Recurse -Filter '*.jar' | Where-Object {$_.Name -notmatch 'sources|natives|^lwjgl-2'} | ForEach-Object {$_.FullName}
    $cp='tools/oracle11/probes/build;tools/oracle11/build/input;tools/retromcp-r11/minecraft/jars/deobfuscated.jar;tools/oracle11/build;tools/oracle11/build/lwjgl-unsealed.jar;'+($libs -join ';')
    javac -cp $cp -d tools/oracle11/probes/build tools/oracle11/probes/ItemPassRenderProbe11.java
    if ($LASTEXITCODE) {throw 'Probe compile failed'}
    New-Item -ItemType Directory -Force (Split-Path $Output -Parent) | Out-Null
    & $env:ORACLE_JAVA -cp $cp net.minecraft.src.ItemPassRenderProbe11 $Output
    if ($LASTEXITCODE) {throw 'Original probe failed'}
    Get-FileHash $Output
    javac -cp $cp -d tools/oracle11/probes/build tools/oracle11/probes/HeldItemPassProbe11.java
    if ($LASTEXITCODE) {throw 'Held probe compile failed'}
    $held=[System.IO.Path]::ChangeExtension($Output,'held.bin')
    & $env:ORACLE_JAVA -cp $cp net.minecraft.src.HeldItemPassProbe11 $held
    if ($LASTEXITCODE) {throw 'Original held probe failed'}
    Get-FileHash $held
} finally {Pop-Location}
