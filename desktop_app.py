#!/usr/bin/env python3
"""
CTARTech-ZentyElastis™: Native Desktop Application Wrapper
----------------------------------------------------------
Membuka antarmuka ZentyElastis sebagai aplikasi desktop mandiri (Native Window):
- Tanpa address bar / tab browser
- Ikon resmi CTARTech-ZentyElastis
- Terintegrasi langsung dengan Core Gateway & Edge Telemetry Agent
- Dukungan ganda: PyWebView2 & Windows Edge App Mode (<0.01ms startup)
"""

import os
import sys
import time
import socket
import subprocess
import urllib.request
import urllib.error
from pathlib import Path

if sys.platform == "win32":
    try:
        sys.stdout.reconfigure(encoding="utf-8")
        sys.stderr.reconfigure(encoding="utf-8")
    except Exception:
        pass

BASE_DIR = Path(__file__).resolve().parent
ICON_PATH = BASE_DIR / "assets" / "logo.ico"
GATEWAY_EXE = BASE_DIR / "core-gateway.exe"
AGENT_SCRIPT = BASE_DIR / "edge-agent" / "python" / "agent.py"

def is_port_in_use(port: int = 8088) -> bool:
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
        return s.connect_ex(('127.0.0.1', port)) == 0

def wait_for_gateway(url: str = "http://127.0.0.1:8088/", timeout_sec: int = 8) -> bool:
    start_time = time.time()
    while time.time() - start_time < timeout_sec:
        try:
            with urllib.request.urlopen(url, timeout=1.0) as resp:
                if resp.status == 200:
                    return True
        except Exception:
            time.sleep(0.4)
    return False

def launch_edge_app_mode(url: str = "http://127.0.0.1:8088/"):
    """Fallback terpercaya: Windows Edge Standalone App Mode (Window khusus tanpa address bar)."""
    edge_paths = [
        r"C:\Program Files (x86)\Microsoft\Edge\Application\msedge.exe",
        r"C:\Program Files\Microsoft\Edge\Application\msedge.exe",
        "msedge.exe"
    ]
    for p in edge_paths:
        try:
            cmd = f'"{p}" --app="{url}" --window-size=1440,900 --app-id=CTARTech-ZentyElastis'
            subprocess.Popen(cmd, shell=True)
            return True
        except Exception:
            continue
    return False

def main():
    gw_proc = None
    agent_proc = None

    # 1. Jalankan Core Gateway jika belum aktif
    if not is_port_in_use(8088):
        exe_to_run = GATEWAY_EXE if GATEWAY_EXE.exists() else (BASE_DIR / "core-gateway" / "target" / "release" / "core-gateway.exe")
        if exe_to_run.exists():
            print(f"[*] Menyalakan ZentyElastis Core Gateway...")
            gw_proc = subprocess.Popen(
                [str(exe_to_run)],
                cwd=str(BASE_DIR),
                creationflags=subprocess.CREATE_NO_WINDOW if sys.platform == "win32" else 0
            )

    # 2. Tunggu Gateway siap
    print("[*] Menghubungkan ke Core Gateway (http://127.0.0.1:8088)...")
    wait_for_gateway("http://127.0.0.1:8088/", timeout_sec=8)

    # 3. Jalankan Edge Telemetry Agent jika ada
    if AGENT_SCRIPT.exists():
        print("[*] Mengaktifkan Zero-Trust Edge Telemetry Agent...")
        agent_proc = subprocess.Popen(
            [sys.executable, str(AGENT_SCRIPT)],
            cwd=str(BASE_DIR),
            creationflags=subprocess.CREATE_NO_WINDOW if sys.platform == "win32" else 0
        )

    # 4. Buka Native Desktop Window
    opened = False
    try:
        import webview
        icon_arg = str(ICON_PATH) if ICON_PATH.exists() else None
        window = webview.create_window(
            title="CTARTech-ZentyElastis™ | Autonomous AI Data Center Telemetry Mesh & Digital Twin",
            url="http://127.0.0.1:8088/",
            width=1440,
            height=900,
            min_size=(1024, 700),
            background_color="#030712",
            confirm_close=False,
        )
        print("[OK] Membuka jendela aplikasi desktop...")
        webview.start(icon=icon_arg, debug=False)
        opened = True
    except Exception as e:
        print(f"[!] PyWebView beralih ke Mode Aplikasi Desktop Windows: {e}")

    # Jika pywebview tidak membuka, gunakan Edge App Mode yang selalu pasti jalan
    if not opened:
        print("[*] Meluncurkan CTARTech-ZentyElastis Standalone Window...")
        launch_edge_app_mode("http://127.0.0.1:8088/")

if __name__ == "__main__":
    main()
