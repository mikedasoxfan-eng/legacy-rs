$ErrorActionPreference='Stop'
Push-Location (Split-Path (Split-Path $PSScriptRoot -Parent) -Parent)
try {
 $libs=Get-ChildItem tools/retromcp-r11/libraries -Recurse -Filter '*.jar' | Where-Object {$_.Name -notmatch 'sources|natives|^lwjgl-2'} | ForEach-Object {$_.FullName}
 $cp='tools/oracle11/probes/build;tools/oracle11/build/input;tools/retromcp-r11/minecraft/jars/deobfuscated.jar;tools/oracle11/build;tools/oracle11/build/lwjgl-unsealed.jar;'+($libs -join ';')
 javac -encoding UTF-8 -cp $cp -d tools/oracle11/probes/build tools/oracle11/gui2/Gui2Fixture10.java tools/oracle11/gui2/Mode_gui2menudraw10.java tools/oracle11/probes/GuiStatLanguage11.java tools/oracle11/probes/InventoryPotency11.java tools/oracle11/probes/SmallGuiLanguage11.java tools/oracle11/probes/CreateWorldType11.java
 if($LASTEXITCODE){throw 'Small GUI/stat original probe compilation failed'}
 # The general replay Keyboard stub returns KEY18. Use unchanged LWJGL Keyboard
 # for its real key-name table; preserve all other headless device seams.
 Add-Type -AssemblyName System.IO.Compression.FileSystem
 $zip=[System.IO.Compression.ZipFile]::OpenRead((Resolve-Path tools/oracle11/build/lwjgl-unsealed.jar))
 try {foreach($entry in $zip.Entries){if($entry.FullName -like 'org/lwjgl/input/Keyboard*.class'){
  $path=Join-Path (Join-Path (Get-Location) 'tools/oracle11/probes/build/keynames') $entry.FullName
  [System.IO.Directory]::CreateDirectory((Split-Path $path -Parent))|Out-Null
  [System.IO.Compression.ZipFileExtensions]::ExtractToFile($entry,$path,$true)
 }}}finally{$zip.Dispose()}
 & $env:ORACLE_JAVA '-Djava.library.path=../retromcp-1.1/libraries/natives' -cp ('tools/oracle11/probes/build/keynames;'+$cp) net.minecraft.src.GuiStatLanguage11 target/r11/gui-stat-language11.bin
 if($LASTEXITCODE){throw 'Original statistics language probe failed'}
 & $env:ORACLE_JAVA -cp $cp net.minecraft.src.InventoryPotency11 target/r11/inventory-potency11.bin
 if($LASTEXITCODE){throw 'Original inventory potency probe failed'}
 & $env:ORACLE_JAVA -cp $cp net.minecraft.src.SmallGuiLanguage11 target/r11/small-gui-language11.bin target/r11/small-gui-language11-java
 if($LASTEXITCODE){throw 'Original small GUI language probe failed'}
 & $env:ORACLE_JAVA -cp $cp net.minecraft.src.CreateWorldType11 target/r11/create-world-type-gui11.bin
 if($LASTEXITCODE){throw 'Original create-world type GUI probe failed'}
 # Device seams missing from the generic replay GL recorder; no game method changes.
 $gl=Get-Content tools/oracle11/headless/org/lwjgl/opengl/GL11.java -Raw
 $at=$gl.LastIndexOf('}')
 $gl=$gl.Substring(0,$at)+"public static void glViewport(int x,int y,int w,int h){}`npublic static void glCopyTexSubImage2D(int t,int l,int xo,int yo,int x,int y,int w,int h){}`n"+$gl.Substring($at)
 New-Item -ItemType Directory -Force tools/oracle11/probes/build/menudraw/org/lwjgl/opengl,tools/oracle11/probes/build/menudraw/build | Out-Null
 [System.IO.File]::WriteAllText((Join-Path (Get-Location) 'tools/oracle11/probes/build/menudraw/org/lwjgl/opengl/GL11.java'),$gl,[System.Text.UTF8Encoding]::new($false))
 javac -encoding UTF-8 -cp $cp -d tools/oracle11/probes/build/menudraw/build tools/oracle11/probes/build/menudraw/org/lwjgl/opengl/GL11.java
 if($LASTEXITCODE){throw 'Menu draw GL seam compile failed'}
 & $env:ORACLE_JAVA -cp ('tools/oracle11/probes/build/menudraw/build;'+$cp) net.minecraft.src.Mode_gui2menudraw10 target/r11/small-gui-menudraw11.bin
 if($LASTEXITCODE){throw 'Original menu decorations probe failed'}
 Get-FileHash target/r11/gui-stat-language11.bin,target/r11/inventory-potency11.bin,target/r11/small-gui-language11.bin,target/r11/create-world-type-gui11.bin,target/r11/small-gui-menudraw11.bin
 $env:GUI_STAT_LANGUAGE11_FIXTURE=(Resolve-Path target/r11/gui-stat-language11.bin).Path
 $env:INVENTORY_POTENCY11_FIXTURE=(Resolve-Path target/r11/inventory-potency11.bin).Path
 $env:SMALL_GUI_LANGUAGE11_FIXTURE=(Resolve-Path target/r11/small-gui-language11.bin).Path
 $env:SMALL_GUI_MENUDRAW11_FIXTURE=(Resolve-Path target/r11/small-gui-menudraw11.bin).Path
 $env:CREATE_WORLD_TYPE11_FIXTURE=(Resolve-Path target/r11/create-world-type-gui11.bin).Path
 # One incremental debug client library build, followed by the same focused
 # production-call checks used for the retained passing release evidence.
 cargo test -p client11 --lib language_delta11_tests -- --ignored --nocapture
 if($LASTEXITCODE){throw 'Statistics/achievement/lifecycle/inventory comparison failed'}
 cargo test -p client11 --lib language_menu11_tests -- --ignored --nocapture
 if($LASTEXITCODE){throw 'Menu/options/language-button comparison failed'}
 cargo test -p client11 --lib worldtype_gui11_tests -- --ignored --nocapture
 if($LASTEXITCODE){throw 'Create-world selector/settings comparison failed'}
}finally{Pop-Location}
