$ErrorActionPreference='Stop'
Push-Location (Split-Path (Split-Path $PSScriptRoot -Parent) -Parent)
try {
 $libs=Get-ChildItem tools/retromcp-r11/libraries -Recurse -Filter '*.jar' | Where-Object {$_.Name -notmatch 'sources|natives|^lwjgl-2'} | ForEach-Object {$_.FullName}
 $cp='tools/oracle11/probes/build;tools/oracle11/build/input;tools/retromcp-r11/minecraft/jars/deobfuscated.jar;tools/oracle11/build;tools/oracle11/build/lwjgl-unsealed.jar;'+($libs -join ';')
 javac -encoding UTF-8 -cp $cp -d tools/oracle11/probes/build tools/oracle11/probes/UnicodeInput11.java tools/oracle11/probes/ContainerRelease11.java tools/oracle11/probes/IngameVersion11.java
 if($LASTEXITCODE){throw 'Remaining GUI original probes compilation failed'}
 foreach($probe in @(@('UnicodeInput11','unicode-input11'),@('ContainerRelease11','container-release11'),@('IngameVersion11','ingame-version11'))){
  & $env:ORACLE_JAVA -cp $cp ('net.minecraft.src.'+$probe[0]) ('target/r11/'+$probe[1]+'.bin')
  if($LASTEXITCODE){throw ('Original GUI probe failed: '+$probe[0])}
 }
 Get-FileHash target/r11/unicode-input11.bin,target/r11/container-release11.bin,target/r11/ingame-version11.bin
 $env:UNICODE_INPUT11_FIXTURE=(Resolve-Path target/r11/unicode-input11.bin).Path
 $env:CONTAINER_RELEASE11_FIXTURE=(Resolve-Path target/r11/container-release11.bin).Path
 $env:INGAME_VERSION11_FIXTURE=(Resolve-Path target/r11/ingame-version11.bin).Path
 cargo test -p client11 --lib remaining_gui11_tests -- --ignored --nocapture
 if($LASTEXITCODE){throw 'Remaining GUI production comparisons failed'}
}finally{Pop-Location}
