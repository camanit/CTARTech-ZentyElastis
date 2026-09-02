use crate::breaker::{evaluate_actuation, ActuationStatus};
use crate::gplay::GPlayAiClient;
use crate::license::{verify_license_file, LicenseStatus};
use axum::{
    extract::{Json, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use hmac::{Hmac, Mac};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use std::sync::{Arc, Mutex};
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;

type HmacSha256 = Hmac<Sha256>;

const EDGE_HARNESS_SECRET: &str = "zenty_mesh_edge_secret_key_2026";
const MAX_TIMESTAMP_SKEW_SEC: u64 = 30;

/// Payload Telemetri Multi-Dimensi Tingkat Lanjut
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeviceTelemetryPayload {
    pub device_id: String,

    // Core Power & Thermal
    pub wattage: f32,
    pub temperature_c: f32,
    pub voltage_v: Option<f32>,
    pub fan_speed_pct: Option<f32>,

    // GPU Compute & Clocks
    pub sm_clock_mhz: Option<u32>,
    pub mem_clock_mhz: Option<u32>,
    pub gpu_utilization_pct: Option<f32>,
    pub vram_used_mb: Option<u64>,
    pub vram_total_mb: Option<u64>,
    pub throttle_reasons: Option<Vec<String>>,

    // AI & Green-Grid Layer
    pub tokens_per_sec: Option<f32>,
    pub joules_per_token: Option<f32>,
    pub carbon_rate_gco2: Option<f32>,

    pub state_transition: Option<String>,
    pub timestamp: u64,
    pub nonce: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GuardResponse {
    pub status: String, // ALLOW, THROTTLE, BLOCK
    pub reason: String,
    pub latency_us: u64,
    pub deepoptiflex_advice: Option<String>,
    pub auto_remediation_action: Option<String>, // Self-Healing Engine Action
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HealthResponse {
    pub status: String,
    pub engine: String,
    pub target_latency: String,
    pub security_mesh: String,
    pub active_license: Option<LicenseStatus>,
    pub gplay_gateway: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApplyLicenseRequest {
    pub license_file_path: String,
    pub public_key_path: String,
}

#[derive(Clone)]
pub struct AppState {
    pub gplay_client: GPlayAiClient,
    pub license_status: Arc<Mutex<Option<LicenseStatus>>>,
    pub total_ingested: Arc<Mutex<u64>>,
}

/// Verifikasi Zero-Trust Edge-to-Core Harness (HMAC-SHA256 & Anti-Replay Guard)
fn verify_edge_harness(headers: &HeaderMap, payload: &DeviceTelemetryPayload) -> Result<(), String> {
    // 1. Anti-Replay Guard: Cek skew timestamp
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    let diff = if now >= payload.timestamp {
        now - payload.timestamp
    } else {
        payload.timestamp - now
    };

    if diff > MAX_TIMESTAMP_SKEW_SEC {
        return Err(format!(
            "Anti-Replay Alert: Timestamp skew {:.0}s melebihi batas toleransi {}s",
            diff, MAX_TIMESTAMP_SKEW_SEC
        ));
    }

    // 2. Cek HMAC Signature jika disertakan di header
    if let Some(sig_header) = headers.get("X-Zenty-Signature") {
        let sig_str = sig_header.to_str().map_err(|_| "Header X-Zenty-Signature bukan string valid")?;
        
        let mut mac = HmacSha256::new_from_slice(EDGE_HARNESS_SECRET.as_bytes())
            .map_err(|e| format!("Inisialisasi HMAC gagal: {}", e))?;

        let canonical_msg = format!("{}:{}:{:.1}:{:.1}", payload.device_id, payload.timestamp, payload.wattage, payload.temperature_c);
        mac.update(canonical_msg.as_bytes());

        let expected_sig = hex::encode(mac.finalize().into_bytes());
        if sig_str != expected_sig {
            return Err("Zero-Trust Violation: X-Zenty-Signature HMAC tidak cocok! Kemungkinan manipulasi paket.".to_string());
        }
    }

    Ok(())
}

/// Autonomous Telemetry & Self-Healing Engine (Auto-Remediation Evaluator)
fn evaluate_self_healing(payload: &DeviceTelemetryPayload) -> Option<String> {
    // Skenario 1: Preemptive Workload Migration jika suhu mendekati batas (78°C)
    if payload.temperature_c >= 78.0 && payload.temperature_c < 85.0 {
        return Some("PREEMPTIVE_WORKLOAD_MIGRATION: Mengalihkan antrean prompt ke GPU dingin sebelum terjadi thermal throttling!".to_string());
    }

    // Skenario 2: VRAM Zombie Cache Purge jika VRAM > 90% tapi tidak ada token yang diproses
    if let (Some(used), Some(total)) = (payload.vram_used_mb, payload.vram_total_mb) {
        let usage_pct = (used as f32 / total as f32) * 100.0;
        let tps = payload.tokens_per_sec.unwrap_or(0.0);
        if usage_pct > 90.0 && tps == 0.0 {
            return Some("AUTONOMOUS_VRAM_CACHE_PURGE: Terdeteksi alokasi zombie cache VRAM saat idle. Menjalankan otonom memory flush.".to_string());
        }
    }

    // Skenario 3: Dynamic Power Cap Pinning jika terdeteksi throttle karena Power Cap
    if let Some(reasons) = &payload.throttle_reasons {
        if reasons.iter().any(|r| r.contains("POWER_CAP") || r.contains("HW_SLOWDOWN")) {
            return Some("DYNAMIC_POWER_CAP_PINNING: Mengatur ulang power envelope GPU (-10%) untuk meredakan lonjakan beban PSU.".to_string());
        }
    }

    None
}

/// Handler utama penerima telemetri dengan Security Mesh & Self-Healing
pub async fn ingest_telemetry_handler(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<DeviceTelemetryPayload>,
) -> impl IntoResponse {
    // 1. Validasi Zero-Trust Edge-to-Core Harness
    if let Err(err_msg) = verify_edge_harness(&headers, &payload) {
        tracing::warn!("🚨 [ZERO-TRUST REJECT] {}", err_msg);
        return (
            StatusCode::UNAUTHORIZED,
            Json(GuardResponse {
                status: "BLOCK".to_string(),
                reason: err_msg,
                latency_us: 0,
                deepoptiflex_advice: None,
                auto_remediation_action: None,
            }),
        );
    }

    // 2. Evaluasi Actuation Assurance (Evaluasi dalam mikrodetik)
    let decision = evaluate_actuation(payload.temperature_c, payload.wattage);

    // 3. Evaluasi Autonomous Self-Healing Engine
    let self_healing_action = evaluate_self_healing(&payload);

    // 4. Tambah counter metrik
    {
        let mut count = state.total_ingested.lock().unwrap();
        *count += 1;
    }

    // 5. Ambil saran cerdas jika lisensi aktif
    let advice = {
        let lic = state.license_status.lock().unwrap();
        if lic.as_ref().map(|l| l.premium_features_enabled).unwrap_or(false) {
            let rec = state.gplay_client.get_predictive_recommendation(payload.wattage, 1);
            Some(rec.advice)
        } else {
            None
        }
    };

    let status_str = match decision.status {
        ActuationStatus::ALLOW => "ALLOW",
        ActuationStatus::THROTTLE => "THROTTLE",
        ActuationStatus::BLOCK => "BLOCK",
    };

    (
        StatusCode::OK,
        Json(GuardResponse {
            status: status_str.to_string(),
            reason: decision.reason,
            latency_us: decision.latency_us,
            deepoptiflex_advice: advice,
            auto_remediation_action: self_healing_action,
        }),
    )
}

/// Health check endpoint
pub async fn health_handler(State(state): State<AppState>) -> impl IntoResponse {
    let lic = state.license_status.lock().unwrap().clone();
    (
        StatusCode::OK,
        Json(HealthResponse {
            status: "HEALTHY".to_string(),
            engine: "Rust Axum Sub-Millisecond Core".to_string(),
            target_latency: "<0.1ms".to_string(),
            security_mesh: "Zero-Trust Edge-to-Core Harness + Self-Healing Active".to_string(),
            active_license: lic,
            gplay_gateway: state.gplay_client.endpoint_url.clone(),
        }),
    )
}

/// Status Lisensi Ed25519
pub async fn license_status_handler(State(state): State<AppState>) -> impl IntoResponse {
    let lic = state.license_status.lock().unwrap().clone();
    (StatusCode::OK, Json(lic))
}

/// Apply file lisensi baru
pub async fn apply_license_handler(
    State(state): State<AppState>,
    Json(req): Json<ApplyLicenseRequest>,
) -> impl IntoResponse {
    match verify_license_file(&req.license_file_path, &req.public_key_path) {
        Ok(status) => {
            let mut current = state.license_status.lock().unwrap();
            *current = Some(status.clone());
            (
                StatusCode::OK,
                Json(serde_json::json!({
                    "success": true,
                    "message": "Lisensi berhasil diverifikasi dan diterapkan!",
                    "details": status
                })),
            )
        }
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "success": false,
                "message": format!("Gagal memverifikasi lisensi: {}", e)
            })),
        ),
    }
}

pub fn app_router(state: AppState) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        .route("/api/v1/health", get(health_handler))
        .route("/api/v1/telemetry/ingest", post(ingest_telemetry_handler))
        .route("/api/v1/license/status", get(license_status_handler))
        .route("/api/v1/license/apply", post(apply_license_handler))
        .layer(TraceLayer::new_for_http())
        .layer(cors)
        .with_state(state)
}
