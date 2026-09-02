@echo off
title CTARTech-ZentyElastis Desktop Application
chcp 65001 > nul
color 0B

cd /d "%~dp0"

echo ===================================================================
echo   ⚡ CTARTech-ZentyElastis: Native Desktop Application
echo   Autonomous AI Data Center Telemetry Mesh & Digital Twin
echo ===================================================================
echo.

:: Pastikan pywebview terinstal
python -c "import webview" > nul 2>&1
if errorlevel 1 (
    echo [*] Memasang pustaka antarmuka desktop (pywebview)...
    pip install pywebview pillow > nul 2>&1
)

echo [*] Menjalankan CTARTech-ZentyElastis Desktop...
python desktop_app.py

