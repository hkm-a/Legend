@echo off
call "C:\Program Files\Microsoft Visual Studio\2022\BuildTools\VC\Auxiliary\Build\vcvars64.bat" >nul
cd /d "C:\Users\hkm\Documents\Code\crystal-mir2"
cargo build --bin mir2-server
