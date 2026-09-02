@echo off
title CTARTech-ZentyElastis Launcher
chcp 65001 > nul
color 0B

echo ===================================================================
echo   ⚡ CTARTech-ZentyElastis: AI Data Center Telemetry Engine
echo   Zero-Trust Edge-to-Core Harness ^& DeepOptiFlex Predictive Mesh
echo ===================================================================
echo.

cd /d "%~dp0"

:: 1. Cek binary release Core Gateway
set "GATEWAY_EXE=core-gateway\target\release\core-gateway.exe"
if not exist "%GATEWAY_EXE%" (
    echo [!] Binary release belum ditemukan, mencari binary debug...
    set "GATEWAY_EXE=core-gateway\target\debug\core-gateway.exe"
)

if not exist "%GATEWAY_EXE%" (
    echo [!] Mengompilasi Core Gateway terlebih dahulu...
    cd core-gateway
    cargo build --release
    cd ..
    set "GATEWAY_EXE=core-gateway\target\release\core-gateway.exe"
)

:: 2. Salin kunci dan lisensi jika ada
if exist "tools\license-issuer\keys\public_key.pem" (
    copy /y "tools\license-issuer\keys\public_key.pem" "public_key.pem" > nul 2>&1
)
if exist "tools\license-issuer\license.lic" (
    copy /y "tools\license-issuer\license.lic" "license.lic" > nul 2>&1
)

:: 3. Matikan proses lama jika masih berjalan
taskkill /F /IM core-gateway.exe > nul 2>&1

echo [*] Menjalankan Core Gateway (Rust Axum Engine di port 8088)...
start "ZentyElastis Core Gateway" /MIN "%GATEWAY_EXE%"

:: Tunggu 2 detik agar gateway aktif
timeout /t 2 /nobreak > nul

echo [*] Menjalankan Zero-Dependency Edge Telemetry Agent...
start "ZentyElastis Edge Agent" /MIN python edge-agent\python\agent.py

echo.
echo [✓] SISTEM ZENTYELASTIS BERHASIL AKTIF!
echo     - Core Gateway : http://127.0.0.1:8088
echo     - Health Check : http://127.0.0.1:8088/health
echo     - Telemetry Ingest : http://127.0.0.1:8088/api/v1/telemetry/ingest
echo.
echo [*] Membuka antarmuka monitoring di browser...
start http://127.0.0.1:8088/health

echo.
echo ===================================================================
echo   Tekan tombol apa saja untuk menutup jendela launcher ini.
echo   (ZentyElastis akan tetap berjalan di latar belakang).
echo   Untuk mematikan sistem, jalankan 'stop-zenty.bat'.
echo ===================================================================
pause > nul
