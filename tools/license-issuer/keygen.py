#!/usr/bin/env python3
"""
CTARTech-ZentyElastis: Air-Gapped Offline License Issuer
-------------------------------------------------------
Modul ini berfungsi untuk menghasilkan pasangan kunci kriptografi Ed25519
dan menerbitkan file lisensi offline bertanda tangan digital (license.lic)
untuk klien enterprise (perbankan, data center, BUMN).

PENTING: File kunci privat (private_key.pem) HANYA boleh ada di PC lokal
developer dan TIDAK BOLEH didistribusikan atau di-push ke git.
"""

import os
import sys
import json
import time
import base64
import argparse
from pathlib import Path

# Memastikan output terminal mendukung karakter UTF-8 di Windows
if sys.platform == "win32":
    try:
        sys.stdout.reconfigure(encoding="utf-8")
        sys.stderr.reconfigure(encoding="utf-8")
    except Exception:
        pass

from cryptography.hazmat.primitives.asymmetric import ed25519
from cryptography.hazmat.primitives import serialization

KEYS_DIR = Path(__file__).parent / "keys"
PRIVATE_KEY_PATH = KEYS_DIR / "private_key.pem"
PUBLIC_KEY_PATH = KEYS_DIR / "public_key.pem"

def ensure_keypair():
    """Membuat pasangan kunci Ed25519 jika belum tersedia."""
    KEYS_DIR.mkdir(parents=True, exist_ok=True)
    
    if not PRIVATE_KEY_PATH.exists():
        print("🔑 Menghasilkan pasangan kunci Ed25519 baru...")
        private_key = ed25519.Ed25519PrivateKey.generate()
        
        # Simpan private key (PKCS8 format)
        pem_priv = private_key.private_bytes(
            encoding=serialization.Encoding.PEM,
            format=serialization.PrivateFormat.PKCS8,
            encryption_algorithm=serialization.NoEncryption()
        )
        with open(PRIVATE_KEY_PATH, "wb") as f:
            f.write(pem_priv)
            
        # Simpan public key (SubjectPublicKeyInfo format)
        public_key = private_key.public_key()
        pem_pub = public_key.public_bytes(
            encoding=serialization.Encoding.PEM,
            format=serialization.PublicFormat.SubjectPublicKeyInfo
        )
        with open(PUBLIC_KEY_PATH, "wb") as f:
            f.write(pem_pub)
            
        print(f"✅ Kunci privat tersimpan di: {PRIVATE_KEY_PATH}")
        print(f"✅ Kunci publik tersimpan di: {PUBLIC_KEY_PATH}")
    else:
        with open(PRIVATE_KEY_PATH, "rb") as f:
            private_key = serialization.load_pem_private_key(f.read(), password=None)
            
    return private_key

def issue_license(client_id: str, max_nodes: int, days_valid: int, tier: str = "Enterprise", output_path: str = None):
    """Menerbitkan file license.lic bertanda tangan kriptografis Ed25519."""
    private_key = ensure_keypair()
    
    issued_at = int(time.time())
    expires_at = issued_at + (days_valid * 86400)
    
    payload = {
        "issuer": "CTARTech-ZentyElastis",
        "client_id": client_id,
        "tier": tier,
        "max_nodes": max_nodes,
        "features": [
            "DeepOptiFlex_AI",
            "SLAShield_Guard",
            "SOC_Merkle_Audit_Ledger",
            "GPlay_AI_Gateway_Access"
        ],
        "issued_at": issued_at,
        "expires_at": expires_at,
        "status": "ACTIVE"
    }
    
    # Serialisasi canonical JSON (tanpa spasi berlebih, urutan kunci tetap)
    canonical_json = json.dumps(payload, separators=(',', ':'), sort_keys=True).encode('utf-8')
    
    # Tanda tangani dengan kunci privat Ed25519
    signature = private_key.sign(canonical_json)
    
    # Simpan dalam kontainer lisensi
    license_container = {
        "version": "1.0",
        "algorithm": "Ed25519",
        "payload": payload,
        "signature_b64": base64.b64encode(signature).decode('utf-8')
    }
    
    if not output_path:
        output_path = f"license_{client_id.lower().replace(' ', '_')}.lic"
        
    with open(output_path, "w", encoding="utf-8") as f:
        json.dump(license_container, f, indent=2)
        
    print("\n" + "=" * 60)
    print("📜 LISENSI ENTERPRISE BERHASIL DITERBITKAN!")
    print("=" * 60)
    print(f"🏢 Klien         : {client_id}")
    print(f"⭐ Paket         : {tier}")
    print(f"🖥️ Kuota GPU/Node: {max_nodes} Nodes")
    print(f"📅 Berlaku s/d   : {time.strftime('%Y-%m-%d %H:%M:%S', time.localtime(expires_at))}")
    print(f"📁 Lokasi File   : {output_path}")
    print("=" * 60)
    print("💡 Berikan file ini bersama 'public_key.pem' kepada klien untuk dipasang di server mereka.\n")
    return output_path

def verify_license(license_path: str, public_key_path: str = None):
    """Menguji verifikasi lisensi secara offline."""
    if not public_key_path:
        public_key_path = PUBLIC_KEY_PATH
        
    with open(public_key_path, "rb") as f:
        public_key = serialization.load_pem_public_key(f.read())
        
    with open(license_path, "r", encoding="utf-8") as f:
        license_data = json.load(f)
        
    payload = license_data["payload"]
    signature = base64.b64decode(license_data["signature_b64"])
    canonical_json = json.dumps(payload, separators=(',', ':'), sort_keys=True).encode('utf-8')
    
    try:
        public_key.verify(signature, canonical_json)
        now = int(time.time())
        if now > payload["expires_at"]:
            print(f"❌ Lisensi Kadaluarsa! (Berakhir pada: {time.strftime('%Y-%m-%d', time.localtime(payload['expires_at']))})")
            return False
        print(f"✅ Lisensi VALID & TERVERIFIKASI untuk '{payload['client_id']}'!")
        print(f"   Maksimal GPU Node: {payload['max_nodes']} | Expire: {time.strftime('%Y-%m-%d', time.localtime(payload['expires_at']))}")
        return True
    except Exception as e:
        print(f"❌ Tanda Tangan Lisensi TIDAK VALID: {e}")
        return False

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="CTARTech-ZentyElastis Offline License Issuer")
    parser.add_argument("--client", type=str, default="Demo Corp", help="Nama Klien / Perusahaan")
    parser.add_argument("--nodes", type=int, default=32, help="Jumlah kuota GPU/Node")
    parser.add_argument("--days", type=int, default=365, help="Masa berlaku (hari)")
    parser.add_argument("--tier", type=str, default="Enterprise", help="Tipe Paket")
    parser.add_argument("--out", type=str, default=None, help="Path output file .lic")
    parser.add_argument("--verify", type=str, default=None, help="Path file .lic untuk diverifikasi")
    
    args = parser.parse_args()
    
    if args.verify:
        verify_license(args.verify)
    else:
        issue_license(args.client, args.nodes, args.days, args.tier, args.out)
