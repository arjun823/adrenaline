@echo off
set PYO3_USE_ABI3_FORWARD_COMPATIBILITY=1
powershell -NoProfile -ExecutionPolicy Bypass -File build.ps1
echo Build script finished!
