@echo off
title Hentikan CTARTech-ZentyElastis
chcp 65001 > nul
color 0C

echo ===================================================================
echo   🛑 Mematikan Semua Proses CTARTech-ZentyElastis
echo ===================================================================
echo.

echo [*] Menghentikan Core Gateway (core-gateway.exe)...
taskkill /F /IM core-gateway.exe > nul 2>&1

echo [*] Menghentikan Edge Telemetry Agent...
taskkill /F /FI "WINDOWTITLE eq ZentyElastis Edge Agent*" > nul 2>&1

echo.
echo [✓] Semua modul ZentyElastis berhasil dihentikan.
echo.
timeout /t 3 > nul
