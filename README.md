# CTARTech-ZentyElastis ⚡
> **Next-Gen GPU Power Optimization, Zero-Latency Telemetry Engine & Digital Twin**

[![GitHub license](https://img.shields.io/badge/license-Open--Core-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-Axum%20%3C0.1ms-orange.svg)](https://www.rust-lang.org/)
[![AI Gateway](https://img.shields.io/badge/AI%20Gateway-gplay.ctar.tech-green.svg)](https://gplay.ctar.tech)
[![Compliance](https://img.shields.io/badge/Compliance-ISO%2027001%20%7C%20UU%20PDP-brightgreen.svg)](#-kepatuhan-standar-enterprise)

CTARTech-ZentyElastis adalah sistem orkestrasi dan manajemen daya cerdas untuk kluster GPU/TPU/CPU berskala enterprise dan pusat data kecerdasan buatan (*AI Data Centers*). Sistem ini mengombinasikan backend **Rust Axum berlatensi ultra-rendah (<0.1ms)** dengan **GPlay AI Data Gateway (`gplay.ctar.tech`)** sebagai pusat analitik pola beban komputasi dan optimasi konsumsi energi (*Peak Shaving*).

---

## 🏛️ Arsitektur Sistem 3-Tier

```text
+-----------------------------------------------------------------------------------+
|               1. EDGE TELEMETRY & AGENT AUTODISCOVERY LAYER                       |
|  - Vendor-Agnostic Agents (NVIDIA NVML, AMD ROCm, TPU, CPU Cluster OS)           |
|  - Multi-Dimensional Telemetry: Clocks, Wattage, Temp, Throttle Reasons, VRAM,    |
|    Energy per Token (Joules/Token), Power Quality (Voltage/PF), ESG Metrics       |
+-----------------------------------------------------------------------------------+
                                         |
                                         v (Encrypted Streaming)
+-----------------------------------------------------------------------------------+
|               🛡️ SECURITY & TELEMETRY MESH (EDGE-TO-CORE HARNESS)                 |
|  - Zero-Trust Device Attestation & Cryptographic Payload Integrity (HMAC-SHA256) |
|  - Anti-Replay & Anti-Tampering Nonce Verification Guard                          |
|  - Autonomous Telemetry & Self-Healing Engine (Closed-Loop Auto-Remediation)      |
|    * Preemptive Workload Migration (Pengalihan beban sebelum thermal threshold)   |
|    * Autonomous VRAM Zombie Cache Purge                                           |
|    * Dynamic Hardware Power Cap Pinning                                           |
+-----------------------------------------------------------------------------------+
                                         |
                                         v (<0.1ms Verified Streaming)
+-----------------------------------------------------------------------------------+
|               2. RUST CORE RUNTIME GATEWAY (Axum Engine <0.1ms Latency)           |
|  - Zero-Leakage License Verifier (Offline Ed25519 Cryptographic Check)            |
|  - Power DC GPU Ontology & Semantic Graph Connector                               |
|  - DeepOptiFlex™ AI-Driven Power & Peak Management Engine                         |
|  - SLAShield™ Guarantee Guard (SLA Protection & Automated Workload Throttling)    |
|  - Emergency Circuit Breaker (Zero Trust Instant Hardware Kill-Switch Protocol)   |
|  - SOC Merkle Chain Audit Ledger (Tamper-Proof Carbon & Energy Compliance)        |
+-----------------------------------------------------------------------------------+
             |                                              |
             v (Raw Streaming Ingestion)                    v (Secure mTLS / gRPC / HTTPS)
+-------------------------------+      +--------------------------------------------+
|   LOCAL TIME-SERIES BUFFER    |      |       🌐 gplay.ctar.tech (GPlay AI Gateway)|
|  - Real-time Wattage & Temp   |      |  - Central AI Knowledge & Vector Bank      |
|  - Carbon & Water Raw Metrics |      |  - Riwayat Pola Beban Kerja LLM Embeddings |
|  - Immediate Actuation Data   |      |  - DeepOptiFlex™ Intelligence Feeder       |
|  - SOC Merkle Chain Log       |      |  - Predictive AI Workload Scaling Model    |
+-------------------------------+      +--------------------------------------------+
                                                            |
                                                            v
+-----------------------------------------------------------------------------------+
|               3. ENTERPRISE DASHBOARD & CONTROL PLANE (Next.js 14)                |
|  - 3D Power & GPU Resource Twin Interactive Heatmaps (WebGL/Three.js 60 FPS)      |
|  - Multi-Tenant Power Budgeting & Token-Based Billing Integration                 |
|  - Live Kanban Infrastructure Views & Multi-Channel Alert Gateway                 |
+-----------------------------------------------------------------------------------+
```

---

## 🚀 Fitur Unggulan

- **Sub-Milisecond Gateway**: Backend berbasis Rust Axum memproses puluhan ribu paket metrik per detik dengan latensi evaluasi `<0.1ms`.
- **Security & Telemetry Mesh**: Perlindungan **Zero-Trust Edge-to-Core Harness** dengan autentikasi perangkat kriptografis (HMAC-SHA256) dan verifikasi anti-replay.
- **Autonomous Self-Healing & Auto-Remediation**: Sistem otonom yang memulihkan dirinya sendiri—mencakup *preemptive workload migration* (pengalihan antrean prompt LLM sebelum GPU overheating), *VRAM zombie cache purge*, dan *dynamic power cap pinning*.
- **Telemetri Multi-Dimensi Tingkat Lanjut**:
  - **Diagnostik Throttling**: Deteksi alasan GPU melambat (*SW Power Cap, Thermal Slowdown, HW Slowdown*).
  - **Metrik Energi AI**: Mengukur **Joules per Token** untuk penagihan komputasi presisi (*Token-Based Billing*).
  - **Kualitas Kelistrikan & Pendingin**: Monitoring voltase PSU, faktor daya ($\cos \phi$), dan suhu pendingin cair (*liquid cooling*).
- **Actuation Assurance & Instant Circuit Breaker**: Evaluasi ambang batas fisik mandiri (*hardware fail-safe*). Memutus aliran daya atau membatasi beban dalam `<5ms` jika suhu inti atau daya listrik melampaui batas bahaya.
- **Pusat AI Data Tunggal (GPlay AI - `gplay.ctar.tech`)**: Seluruh pola antrean prompt LLM dan tarikan daya di-cluster menjadi *vector embeddings* di GPlay AI untuk memprediksi lonjakan panas sebelum terjadi.
- **DeepOptiFlex™ Dynamic Peak Shaving**: Memangkas puncak tarikan daya listrik hingga **15–25%** pada saat beban puncak (*peak hours*), menghemat puluhan hingga ratusan juta rupiah tagihan listrik data center.
- **SLAShield™ Guardian**: Memastikan pemotongan daya tidak menyebabkan *latency spike* atau pelanggaran SLA klien korporat.
- **Lisensi Air-Gapped Ed25519**: Klien perbankan & BUMN dapat memverifikasi lisensi 100% offline tanpa perlu koneksi internet ke luar server tertutup (*Zero-Leakage Compliance*).
- **SOC Merkle Chain Audit Ledger**: Setiap perubahan beban daya dan jejak emisi karbon ($CO_2$) dicatat ke rantai hash Merkle lokal yang *tamper-proof* dan siap diaudit.

---

## 🗺️ Roadmap Implementasi (10 Minggu / 5 Sprint)

| Fase | Durasi | Fokus Utama | Deliverables Kunci |
| :--- | :--- | :--- | :--- |
| **Fase 1** | Minggu 1–2 | Core Ingestion & Autodiscovery | Rust Axum Ingestion Gateway, Edge Telemetry Agent, Ring Buffer. |
| **Fase 2** | Minggu 3–4 | Offline License & Actuation | Ed25519 Keygen & Verifier, Actuation Assurance, Emergency Circuit Breaker. |
| **Fase 3** | Minggu 5–6 | GPlay AI Integration | Konektor `gplay.ctar.tech`, Vector Pattern Store, DeepOptiFlex™ Dynamic Enforcer. |
| **Fase 4** | Minggu 7–8 | Green-Grid Ledger & Security | SOC Merkle Audit Ledger, Layer 3/4 & 7 Anti-DDoS, ISO & UU PDP Compliance. |
| **Fase 5** | Minggu 9–10| Enterprise Control Plane | Next.js 14 3D Resource Twin (WebGL 60 FPS), Multi-Tenant Billing, Pilot Run. |

---

## 📂 Struktur Repositori

```text
CTARTech-ZentyElastis/
├── core-gateway/                  # Rust Axum Ingestion & Actuation Engine (<0.1ms)
├── edge-agent/                    # Agen ringan pengumpul telemetri GPU/CPU
│   └── python/                    # Python NVML / psutil edge client
├── tools/                         # Developer Enclave (Air-gapped Keygen & Issuer)
│   └── license-issuer/            # Ed25519 Offline Keygen
├── Desain Arsitektur Sistem CTARTech-Elastis.txt # Cetak biru arsitektur lengkap
└── README.md
```

---

## 💻 Panduan Cepat (Quick Start)

### 1. Menjalankan Core Gateway (Rust)

```bash
cd core-gateway
cargo run --release
```
Gateway akan aktif mendengarkan telemetri pada `http://127.0.0.1:8000`.

### 2. Menjalankan Edge Agent (Python)

```bash
cd edge-agent/python
pip install requests psutil
python agent.py
```

### 3. Menerbitkan Lisensi Offline (Developer Side)

```bash
cd tools/license-issuer
python keygen.py --client "PT Enterprise Maju" --nodes 32 --days 365
```

---

## 🛡️ Kepatuhan Standar Enterprise

| Standar / Regulasi | Deskripsi Fokus | Implementasi di CTARTech-ZentyElastis |
| :--- | :--- | :--- |
| **ISO/IEC 27001** | *Information Security Management System* | Enkripsi end-to-end (AES-256 & TLS 1.3), mTLS antar node, isolasi data tenant. |
| **ISO 22301** | *Business Continuity Management (BCM)* | Emergency Circuit Breaker otomatis dan degradasi lisensi bertahap (*Graceful Fail-Safe*). |
| **UU PDP No. 27/2022 & GDPR** | *Perlindungan Data Pribadi* | Zero-Leakage air-gapped runtime, data masking metrik ke GPlay AI. |
| **Green Grid / ESG** | *Efisiensi Daya & Emisi Karbon* | SOC Merkle Chain Audit Ledger untuk pencatatan emisi $CO_2$ dan efisiensi air pendingin. |

---

## 📄 Model Lisensi: Open-Core (Community vs Enterprise)

- **Community Edition (Open-Source)**: Edge Ingestion Agent, Client SDKs, dan pemantauan dasar (s/d 8 GPU) bebas digunakan oleh komunitas pengembang.
- **Enterprise Edition (Commercial)**: Fitur lanjutan (*DeepOptiFlex™*, *SLAShield™*, *Air-Gapped Ed25519 License*, *SOC Merkle Audit*, dan integrasi penuh *GPlay AI*) dilindungi hak cipta untuk kebutuhan korporasi.

---

## 👥 Tim & Ekosistem
Dikembangkan oleh tim **CTARTech** bekerjasama dengan ekosistem data cerdas **[GPlay AI](https://gplay.ctar.tech)**.
