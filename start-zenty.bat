@echo off
title CTARTech-ZentyElastis Desktop Application
chcp 65001 > nul
color 0B

cd /d "%~dp0"

echo ===================================================================
echo   ⚡ CTARTech-ZentyElastis™: Native Desktop Application
echo   Autonomous AI Data Center Telemetry Mesh & Digital Twin
echo ===================================================================
echo.

:: 1. Cek & Jalankan Core Gateway (Rust Axum Engine port 8088)
tasklist /FI "IMAGENAME eq core-gateway.exe" 2>NUL | find /I /N "core-gateway.exe">NUL
if "%ERRORLEVEL%"=="0" (
    echo [✓] Core Gateway sudah aktif di latar belakang.
) else (
    echo [*] Menyalakan Core Gateway (port 8088)...
    if exist "core-gateway.exe" (
        start "" /MIN "core-gateway.exe"
    ) else if exist "core-gateway\target\release\core-gateway.exe" (
        start "" /MIN "core-gateway\target\release\core-gateway.exe"
    )
    timeout /t 2 /nobreak > nul
)

:: 2. Cek & Jalankan Edge Telemetry Agent jika belum aktif
tasklist /FI "WINDOWTITLE eq ZentyElastis Edge Agent*" 2>NUL | find /I /N "python">NUL
if "%ERRORLEVEL%"=="1" (
    echo [*] Menyalakan Edge Telemetry Agent (Streaming Live Metrics)...
    start "ZentyElastis Edge Agent" /MIN python edge-agent\python\agent.py
)

echo [*] Meluncurkan jendela Desktop Application ZentyElastis...
:: Jalankan mode aplikasi mandiri (Borderless Standalone App Window tanpa URL bar)
start "" msedge.exe --app="http://127.0.0.1:8088/" --window-size=1440,900

echo.
echo [✓] APLIKASI DESKTOP BERHASIL DIBUKA!
echo.
echo ===================================================================
echo   Jendela aplikasi mandiri telah aktif di layar Anda.
echo   Tekan tombol apa saja untuk menutup jendela console ini.
echo ===================================================================
timeout /t 3 > nul
exit
