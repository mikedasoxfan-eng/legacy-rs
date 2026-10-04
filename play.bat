@echo off
rem Minecraft Beta 1.7.3 (Rust) - world viewer.
rem Options: --seed <number or text>   --time <ticks, 0 = sunrise, 6000 = noon>
cd /d "%~dp0"
if not exist target\release\beta.exe cargo build --release -p client
target\release\beta.exe %*
