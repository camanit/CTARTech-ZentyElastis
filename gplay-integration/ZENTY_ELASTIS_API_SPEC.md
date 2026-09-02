# GPlay AI Data Gateway: CTARTech-ZentyElastis API Specification
> **Base URL**: `https://gplay.ctar.tech/api/v1/zenty` (or `http://localhost/api/v1/zenty` on local Herd)  
> **Auth**: `Authorization: Bearer <GPLAY_API_KEY>`  
> **Supported Languages**: `id` (Bahasa Indonesia), `en` (English), `ms` (Bahasa Melayu) via `Accept-Language` header.

---

## 1. Endpoints

### 1.1. Health Check & Ping
* **Method**: `GET /ping`
* **Headers**: `Accept-Language: id`
* **Response (200 OK)**:
```json
{
  "status": "online",
  "gateway": "GPlay AI Data Gateway (gplay.ctar.tech)",
  "subsystem": "CTARTech-ZentyElastis Telemetry Mesh",
  "version": "1.0.0",
  "timestamp": "2026-09-02T12:40:00+07:00",
  "message": "GPlay AI Data Gateway online dan siap menerima sinkronisasi telemetri."
}
```

---

### 1.2. Telemetry Batch Sync (From Rust Gateway)
* **Method**: `POST /telemetry/sync`
* **Headers**: 
  - `Content-Type: application/json`
  - `Authorization: Bearer <API_KEY>`
* **Request Payload**:
```json
{
  "cluster_id": "cluster-h100-bank-mandiri",
  "client_name": "PT Bank Mandiri AI Data Center",
  "tier": "Enterprise",
  "hardware_type": "NVIDIA_GPU",
  "workload_name": "LLM_INFERENCE",
  "metrics": [
    {
      "device_id": "gpu-node-01",
      "wattage": 1920.5,
      "temperature_c": 74.2,
      "voltage_v": 12.05,
      "fan_speed_pct": 68.0,
      "sm_clock_mhz": 1980,
      "mem_clock_mhz": 2619,
      "gpu_utilization_pct": 91.2,
      "vram_used_mb": 42100,
      "vram_total_mb": 81920,
      "throttle_reasons": ["NONE"],
      "tokens_per_sec": 168.5,
      "joules_per_token": 11.398,
      "carbon_rate_gco2": 415.0,
      "state_transition": "ACTIVE_LLM_INFERENCE",
      "timestamp": 1788327307
    }
  ]
}
```
* **Response (200 OK)**:
```json
{
  "success": true,
  "synced_records": 1,
  "cluster_status": "ACTIVE",
  "deepoptiflex_recommendation": {
    "current_total_wattage": 1920.5,
    "recommended_peak_limit_watt": 1565.2,
    "peak_saving_pct": 18.5,
    "action": "APPLY_PEAK_SHAVING",
    "message": "DeepOptiFlex dynamic peak shaving aktif: Target pemotongan daya sebesar 18.5%."
  }
}
```

---

### 1.3. Get Predictive Recommendations (DeepOptiFlex™)
* **Method**: `GET /recommendations?cluster_id=cluster-h100-bank-mandiri`
* **Response (200 OK)**:
```json
{
  "success": true,
  "cluster_id": "cluster-h100-bank-mandiri",
  "recommended_peak_limit_watt": 1565.2,
  "dynamic_throttle_ratio": 0.82,
  "predicted_spike_in_minutes": 15,
  "intelligence_source": "GPlay AI Knowledge Engine v2.4 (gplay.ctar.tech)",
  "advice": "Rekomendasi cerdas GPlay AI: Jaga batas daya puncak di bawah 1,565.2 W."
}
```

---

### 1.4. Get Cluster Health & Status
* **Method**: `GET /cluster/{cluster_id}/health`
* **Response (200 OK)**:
```json
{
  "success": true,
  "cluster": {
    "cluster_id": "cluster-h100-bank-mandiri",
    "client_name": "PT Bank Mandiri AI Data Center",
    "tier": "Enterprise",
    "max_nodes": 64,
    "status": "ACTIVE"
  },
  "nodes_reporting": 1,
  "recent_telemetry": [...]
}
```
