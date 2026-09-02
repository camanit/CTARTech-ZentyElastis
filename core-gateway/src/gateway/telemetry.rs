use crate::breaker::{evaluate_actuation, ActuationStatus};
use crate::gplay::GPlayAiClient;
use crate::license::{verify_license_file, LicenseStatus};
use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeviceTelemetryPayload {
    pub device_id: String,
    pub wattage: f32,
    pub temperature_c: f32,
    pub vram_used_mb: Option<u64>,
    pub vram_total_mb: Option<u64>,
    pub state_transition: Option<String>,
    pub timestamp: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GuardResponse {
    pub status: String, // ALLOW, THROTTLE, BLOCK
    pub reason: String,
    pub latency_us: u64,
    pub deepoptiflex_advice: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HealthResponse {
    pub status: String,
    pub engine: String,
    pub target_latency: String,
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

/// Handler utama untuk menerima telemetri masuk berkecepatan tinggi (<0.1ms)
pub async fn ingest_telemetry_handler(
    State(state): State<AppState>,
    Json(payload): Json<DeviceTelemetryPayload>,
) -> impl IntoResponse {
    // 1. Evaluasi Actuation Assurance (Evaluasi dalam mikrodetik)
    let decision = evaluate_actuation(payload.temperature_c, payload.wattage);

    // 2. Tambah counter metrik
    {
        let mut count = state.total_ingested.lock().unwrap();
        *count += 1;
    }

    // 3. Ambil saran cerdas jika lisensi aktif
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
