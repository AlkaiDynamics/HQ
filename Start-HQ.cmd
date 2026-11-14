@echo off
cd /d "%~dp0"
set HQ_OPEN_BROWSER=1
if exist runtime\node.exe (
  runtime\node.exe native-bridge\server.mjs
) else (
  node native-bridge\server.mjs
)
if errorlevel 1 pause
