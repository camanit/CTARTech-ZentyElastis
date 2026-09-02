use crate::breaker::{evaluate_actuation, ActuationStatus};
use crate::dashboard::render_dashboard_html;
use crate::deepoptiflex::{DeepOptiFlexEngine, OptimizationResult};
use crate::gplay::GPlayAiClient;
use crate::license::{verify_license_file, LicenseStatus};
use axum::{
    extract::{Json, State},
    http::{HeaderMap, StatusCode},
    response::{Html, IntoResponse},
    routing::{get, post},
    Router,
};
use hmac::{Hmac, Mac};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use std::collections::VecDeque;
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TelemetryHistoryPoint {
    pub timestamp: u64,
    pub wattage: f32,
    pub temperature_c: f32,
    pub recommended_cap: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SelfHealingEvent {
    pub action: String,
    pub reason: String,
    pub time: String,
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

#[derive(Serialize, Deserialize, Debug)]
pub struct LiveTelemetryResponse {
    pub circuit_breaker_status: String,
    pub latest_metrics: Option<DeviceTelemetryPayload>,
    pub deepoptiflex: Option<OptimizationResult>,
    pub history: Vec<TelemetryHistoryPoint>,
    pub self_healing_events: Vec<SelfHealingEvent>,
    pub active_license: Option<LicenseStatus>,
    pub total_samples_ingested: u64,
}

#[derive(Clone)]
pub struct AppState {
    pub gplay_client: GPlayAiClient,
    pub license_status: Arc<Mutex<Option<LicenseStatus>>>,
    pub total_ingested: Arc<Mutex<u64>>,
    pub deepoptiflex: DeepOptiFlexEngine,
    pub manual_trip: Arc<Mutex<bool>>,
    pub latest_metrics: Arc<Mutex<Option<DeviceTelemetryPayload>>>,
    pub latest_deepoptiflex: Arc<Mutex<Option<OptimizationResult>>>,
    pub telemetry_history: Arc<Mutex<VecDeque<TelemetryHistoryPoint>>>,
    pub self_healing_feed: Arc<Mutex<Vec<SelfHealingEvent>>>,
}

/// Membangun Router Axum dengan Zero-Trust Security Mesh & Web Dashboard
pub fn app_router(state: AppState) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        // 1. Root & Web UI Dashboard
        .route("/", get(dashboard_handler))
        .route("/dashboard", get(dashboard_handler))
        .route("/favicon.ico", get(favicon_handler))
        .route("/assets/logo.png", get(logo_handler))

        // 2. Telemetry Ingest & Real-time API
        .route("/api/v1/telemetry/ingest", post(ingest_telemetry_handler))
        .route("/api/v1/telemetry/live", get(live_telemetry_handler))

        // 3. Actuation Assurance & Circuit Breaker Controls
        .route("/api/v1/breaker/trip", post(trip_breaker_handler))
        .route("/api/v1/breaker/reset", post(reset_breaker_handler))

        // 4. System Health & Licensing
        .route("/health", get(health_handler))
        .route("/api/v1/license/status", get(license_status_handler))
        .route("/api/v1/license/apply", post(apply_license_handler))
        .layer(TraceLayer::new_for_http())
        .layer(cors)
        .with_state(state)
}

/// Handler rendering Web Dashboard HTML
pub async fn dashboard_handler() -> Html<&'static str> {
    Html(render_dashboard_html())
}

/// Handler serving favicon.ico directly from compiled binary
pub async fn favicon_handler() -> impl IntoResponse {
    let ico = include_bytes!("../../../assets/logo.ico");
    ([(axum::http::header::CONTENT_TYPE, "image/x-icon")], ico.to_vec())
}

/// Handler serving logo.png directly from compiled binary
pub async fn logo_handler() -> impl IntoResponse {
    let png = include_bytes!("../../../assets/logo.png");
    ([(axum::http::header::CONTENT_TYPE, "image/png")], png.to_vec())
}

/// Handler live status telemetri untuk dashboard polling
pub async fn live_telemetry_handler(State(state): State<AppState>) -> impl IntoResponse {
    let is_tripped = *state.manual_trip.lock().unwrap();
    let breaker_status = if is_tripped { "TRIPPED" } else { "ARMED" };
    let latest = state.latest_metrics.lock().unwrap().clone();
    let deepopt = state.latest_deepoptiflex.lock().unwrap().clone();
    let hist: Vec<TelemetryHistoryPoint> = state.telemetry_history.lock().unwrap().iter().cloned().collect();
    let events = state.self_healing_feed.lock().unwrap().clone();
    let lic = state.license_status.lock().unwrap().clone();
    let total = *state.total_ingested.lock().unwrap();

    Json(LiveTelemetryResponse {
        circuit_breaker_status: breaker_status.to_string(),
        latest_metrics: latest,
        deepoptiflex: deepopt,
        history: hist,
        self_healing_events: events,
        active_license: lic,
        total_samples_ingested: total,
    })
}

/// Pemicu darurat Circuit Breaker manual dari dashboard
pub async fn trip_breaker_handler(State(state): State<AppState>) -> impl IntoResponse {
    let mut trip = state.manual_trip.lock().unwrap();
    *trip = true;

    // Catat ke log self-healing
    let mut feed = state.self_healing_feed.lock().unwrap();
    feed.push(SelfHealingEvent {
        action: "MANUAL_KILL_SWITCH".to_string(),
        reason: "Operator memicu pemutus sirkuit darurat via Web Dashboard.".to_string(),
        time: chrono::Local::now().format("%H:%M:%S").to_string(),
    });

    Json(serde_json::json!({
        "success": true,
        "circuit_breaker_status": "TRIPPED",
        "message": "Circuit breaker berhasil diputus. Seluruh aktuasi hardware diblokir."
    }))
}

/// Reset Circuit Breaker setelah kondisi normal
pub async fn reset_breaker_handler(State(state): State<AppState>) -> impl IntoResponse {
    let mut trip = state.manual_trip.lock().unwrap();
    *trip = false;

    let mut feed = state.self_healing_feed.lock().unwrap();
    feed.push(SelfHealingEvent {
        action: "BREAKER_RESET".to_string(),
        reason: "Circuit Breaker dipulihkan ke status ARMED (Normal).".to_string(),
        time: chrono::Local::now().format("%H:%M:%S").to_string(),
    });

    Json(serde_json::json!({
        "success": true,
        "circuit_breaker_status": "ARMED",
        "message": "Circuit breaker berhasil di-reset. Sistem siaga normal."
    }))
}

/// Handler penerima telemetri dengan verifikasi Zero-Trust & DeepOptiFlex
pub async fn ingest_telemetry_handler(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<DeviceTelemetryPayload>,
) -> impl IntoResponse {
    // 0. Periksa apakah pemutus sirkuit manual aktif
    let is_manually_tripped = *state.manual_trip.lock().unwrap();
    if is_manually_tripped {
        return (
            StatusCode::FORBIDDEN,
            Json(GuardResponse {
                status: "BLOCK".to_string(),
                reason: "ACTUATION OVERRIDE: Manual Emergency Kill-Switch is active. Reset breaker to resume.".to_string(),
                latency_us: 1,
                deepoptiflex_advice: None,
                auto_remediation_action: Some("EMERGENCY_LOAD_SHED".to_string()),
            }),
        );
    }

    // 1. Zero-Trust Harness: Validasi Timestamp Skew
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let skew = if now > payload.timestamp {
        now - payload.timestamp
    } else {
        payload.timestamp - now
    };

    if skew > MAX_TIMESTAMP_SKEW_SEC {
        return (
            StatusCode::UNAUTHORIZED,
            Json(GuardResponse {
                status: "BLOCK".to_string(),
                reason: format!(
                    "ZERO-TRUST REPLAY ATTACK DETECTED: Timestamp skew is {}s (max {}s)",
                    skew, MAX_TIMESTAMP_SKEW_SEC
                ),
                latency_us: 1,
                deepoptiflex_advice: None,
                auto_remediation_action: None,
            }),
        );
    }

    // 2. Zero-Trust Harness: Validasi Tanda Tangan HMAC-SHA256
    let client_signature = headers
        .get("X-Zenty-Signature")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("");

    let canonical_payload = format!(
        "{}:{:.1}:{:.1}:{}",
        payload.device_id, payload.wattage, payload.temperature_c, payload.timestamp
    );

    let mut mac = match HmacSha256::new_from_slice(EDGE_HARNESS_SECRET.as_bytes()) {
        Ok(m) => m,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(GuardResponse {
                    status: "ERROR".to_string(),
                    reason: "Failed to initialize cryptographic HMAC context".to_string(),
                    latency_us: 0,
                    deepoptiflex_advice: None,
                    auto_remediation_action: None,
                }),
            )
        }
    };
    mac.update(canonical_payload.as_bytes());

    let expected_signature = hex::encode(mac.finalize().into_bytes());

    if client_signature != expected_signature {
        return (
            StatusCode::UNAUTHORIZED,
            Json(GuardResponse {
                status: "BLOCK".to_string(),
                reason: "ZERO-TRUST INTEGRITY VIOLATION: Invalid X-Zenty-Signature HMAC-SHA256 digest".to_string(),
                latency_us: 1,
                deepoptiflex_advice: None,
                auto_remediation_action: None,
            }),
        );
    }

    // 3. Actuation Assurance & Emergency Circuit Breaker
    let decision = evaluate_actuation(payload.temperature_c, payload.wattage);

    // 4. Autonomous Telemetry & Self-Healing Engine
    let mut self_healing_action: Option<String> = None;
    let time_str = chrono::Local::now().format("%H:%M:%S").to_string();

    if payload.temperature_c >= 78.0 && payload.temperature_c <= 85.0 {
        let act = "PREEMPTIVE_WORKLOAD_MIGRATION".to_string();
        self_healing_action = Some(act.clone());
        state.self_healing_feed.lock().unwrap().push(SelfHealingEvent {
            action: act,
            reason: format!("Suhu junction GPU {:.1}°C mencapai batas preemptive. Mengalihkan antrean prompt ke standby node.", payload.temperature_c),
            time: time_str.clone(),
        });
    }

    if let (Some(used_mb), Some(total_mb)) = (payload.vram_used_mb, payload.vram_total_mb) {
        let vram_usage_pct = (used_mb as f64 / total_mb as f64) * 100.0;
        let is_idle_or_finishing = payload.gpu_utilization_pct.unwrap_or(100.0) < 15.0;
        if vram_usage_pct > 90.0 && is_idle_or_finishing {
            let act = "AUTONOMOUS_VRAM_CACHE_PURGE".to_string();
            self_healing_action = Some(act.clone());
            state.self_healing_feed.lock().unwrap().push(SelfHealingEvent {
                action: act,
                reason: format!("VRAM terisi {:.1}% pada kondisi idle. Membebaskan alokasi memori zombie.", vram_usage_pct),
                time: time_str.clone(),
            });
        }
    }

    if payload.wattage > 2800.0 {
        let act = "DYNAMIC_POWER_CAP_PINNING".to_string();
        self_healing_action = Some(act.clone());
        state.self_healing_feed.lock().unwrap().push(SelfHealingEvent {
            action: act,
            reason: format!("Konsumsi daya {:.1}W mendekati ambang batas. Menetapkan hard-cap daya GPU.", payload.wattage),
            time: time_str.clone(),
        });
    }

    // 5. Evaluasi Mesin Prediktif DeepOptiFlex™
    let opt_result = state.deepoptiflex.evaluate_sample(payload.wattage as f64);
    *state.latest_deepoptiflex.lock().unwrap() = Some(opt_result.clone());

    // 6. Rekam ke Ring-Buffer Riwayat (Maksimal 60 sampel)
    {
        let mut hist = state.telemetry_history.lock().unwrap();
        if hist.len() >= 60 {
            hist.pop_front();
        }
        hist.push_back(TelemetryHistoryPoint {
            timestamp: payload.timestamp,
            wattage: payload.wattage,
            temperature_c: payload.temperature_c,
            recommended_cap: opt_result.recommended_cap_watt,
        });
    }

    // Simpan metrik terbaru & update counter
    *state.latest_metrics.lock().unwrap() = Some(payload.clone());
    {
        let mut count = state.total_ingested.lock().unwrap();
        *count += 1;
    }

    let status_str = match decision.status {
        ActuationStatus::ALLOW => "ALLOW",
        ActuationStatus::THROTTLE => "THROTTLE",
        ActuationStatus::BLOCK => "BLOCK",
    };

    let advice = Some(opt_result.advisory_message);

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
