@echo off
call "C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\VC\Auxiliary\Build\vcvars64.bat" >nul 2>&1
cd /d "C:\c\Users\herpe\magpie-vsig-spike\.scratch\vsig-harness"
set HARNESS_BASE=C:\c\Users\herpe\magpie-vsig-spike
cargo run --quiet --bin corpus
if errorlevel 1 (exit /b 1)
cargo run --quiet --bin corpus
exit /b %errorlevel%
