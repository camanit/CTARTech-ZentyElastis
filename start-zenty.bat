@echo off
title CTARTech-ZentyElastis Desktop Application
color 0B

cd /d "%~dp0"

echo ===================================================================
echo   CTARTech-ZentyElastis: Native Desktop Application
echo   Autonomous AI Data Center Telemetry Mesh and Digital Twin
echo ===================================================================
echo.

:: 1. Cek apakah Core Gateway sudah aktif di port 8088
netstat -ano | findstr ":8088" | findstr "LISTENING" > nul 2>&1
if "%ERRORLEVEL%"=="0" (
    echo [*] Core Gateway aktif di port 8088
) else (
    echo [*] Menyalakan Core Gateway di port 8088
    if exist "core-gateway.exe" start "ZentyGateway" /MIN "core-gateway.exe"
    if not exist "core-gateway.exe" if exist "core-gateway\target\release\core-gateway.exe" start "ZentyGateway" /MIN "core-gateway\target\release\core-gateway.exe"
    timeout /t 2 /nobreak > nul
)

:: 2. Cek dan Jalankan Edge Telemetry Agent jika belum aktif
tasklist /FI "WINDOWTITLE eq ZentyElastis Edge Agent*" 2>NUL | find /I /N "python">NUL
if "%ERRORLEVEL%"=="1" (
    echo [*] Menyalakan Edge Telemetry Agent
    start "ZentyElastis Edge Agent" /MIN python edge-agent\python\agent.py
)

:: 3. Cari jalur absolut Microsoft Edge atau Google Chrome untuk mode Standalone App
set "APP_RUNNER="

if exist "C:\Program Files (x86)\Microsoft\Edge\Application\msedge.exe" set "APP_RUNNER=C:\Program Files (x86)\Microsoft\Edge\Application\msedge.exe"
if not defined APP_RUNNER if exist "C:\Program Files\Microsoft\Edge\Application\msedge.exe" set "APP_RUNNER=C:\Program Files\Microsoft\Edge\Application\msedge.exe"
if not defined APP_RUNNER if exist "C:\Program Files\Google\Chrome\Application\chrome.exe" set "APP_RUNNER=C:\Program Files\Google\Chrome\Application\chrome.exe"
if not defined APP_RUNNER if exist "C:\Program Files (x86)\Google\Chrome\Application\chrome.exe" set "APP_RUNNER=C:\Program Files (x86)\Google\Chrome\Application\chrome.exe"

echo [*] Meluncurkan jendela Desktop Application ZentyElastis
if defined APP_RUNNER (
    echo [*] Menggunakan Standalone Window
    start "" "%APP_RUNNER%" --app="http://127.0.0.1:8088/" --window-size=1440,900
)
if not defined APP_RUNNER (
    echo [*] Membuka browser default
    start http://127.0.0.1:8088/
)

echo.
echo ===================================================================
echo   APLIKASI DESKTOP ZENTYELASTIS BERHASIL DIBUKA!
echo   - Alamat Gateway : http://127.0.0.1:8088/
echo   - Status Mesin   : Online
echo ===================================================================
echo.
echo Jendela ini boleh kamu tutup atau biarkan saja.
pause
