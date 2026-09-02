mod audit;
mod breaker;
mod dashboard;
mod deepoptiflex;
mod gateway;
mod gplay;
mod license;
mod slashield;

use audit::MerkleAuditLedger;
use deepoptiflex::{DeepOptiFlexEngine, DeepOptiFlexPolicy};
use gateway::{app_router, AppState};
use gplay::GPlayAiClient;
use license::verify_license_file;
use slashield::{SLAShieldGuardian, SLAShieldPolicy};
use std::collections::VecDeque;
use std::net::SocketAddr;
use std::path::Path;
use std::sync::{Arc, Mutex};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    // 1. Inisialisasi logging terstruktur
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "core_gateway=info,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    println!("===================================================================");
    println!("⚡ CTARTech-ZentyElastis: Core Runtime Gateway & Digital Twin Web UI");
    println!("   DeepOptiFlex™ & SLAShield™ Guardian with SOC Merkle Chain Audit");
    println!("===================================================================");

    // 2. Deteksi lisensi lokal otomatis jika ada
    let license_path = Path::new("license.lic");
    let pubkey_path = Path::new("public_key.pem");
    let initial_license = if license_path.exists() && pubkey_path.exists() {
        match verify_license_file(license_path, pubkey_path) {
            Ok(status) => {
                println!("🔑 [LISENSI TERVERIFIKASI] Klien: {} | Expire: {} hari", status.client_id, status.days_remaining);
                Some(status)
            }
            Err(e) => {
                println!("⚠️ [LISENSI PERINGATAN] {}", e);
                None
            }
        }
    } else {
        println!("ℹ️ Tidak ada file 'license.lic' lokal. Berjalan dalam mode Community Edition (Free).");
        None
    };

    // 3. Konfigurasi State Aplikasi, SLAShield™ & SOC Merkle Audit Ledger
    let state = AppState {
        gplay_client: GPlayAiClient::new(
            Some("https://gplay.ctar.tech".to_string()),
            Some("gplay_live_api_token".to_string()),
        ),
        license_status: Arc::new(Mutex::new(initial_license)),
        total_ingested: Arc::new(Mutex::new(0)),
        deepoptiflex: DeepOptiFlexEngine::new(DeepOptiFlexPolicy::default()),
        slashield: SLAShieldGuardian::new(SLAShieldPolicy::default()),
        audit_ledger: MerkleAuditLedger::new(),
        manual_trip: Arc::new(Mutex::new(false)),
        latest_metrics: Arc::new(Mutex::new(None)),
        latest_deepoptiflex: Arc::new(Mutex::new(None)),
        latest_slashield: Arc::new(Mutex::new(None)),
        telemetry_history: Arc::new(Mutex::new(VecDeque::with_capacity(60))),
        self_healing_feed: Arc::new(Mutex::new(Vec::new())),
    };

    // 4. Background Sync Worker ke GPlay AI Gateway
    let sync_state = state.clone();
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(5));
        loop {
            interval.tick().await;
            let maybe_metric = sync_state.latest_metrics.lock().unwrap().clone();
            if let Some(metric) = maybe_metric {
                let client_name = {
                    let lic_guard = sync_state.license_status.lock().unwrap();
                    lic_guard.as_ref().map(|l| l.client_id.clone()).unwrap_or_else(|| "CTARTech Enterprise Node".to_string())
                };

                let metric_json = serde_json::json!({
                    "device_id": metric.device_id,
                    "wattage": metric.wattage,
                    "temperature_c": metric.temperature_c,
                    "voltage_v": metric.voltage_v,
                    "fan_speed_pct": metric.fan_speed_pct,
                    "sm_clock_mhz": metric.sm_clock_mhz,
                    "mem_clock_mhz": metric.mem_clock_mhz,
                    "gpu_utilization_pct": metric.gpu_utilization_pct,
                    "vram_used_mb": metric.vram_used_mb,
                    "vram_total_mb": metric.vram_total_mb,
                    "tokens_per_sec": metric.tokens_per_sec,
                    "joules_per_token": metric.joules_per_token,
                    "carbon_rate_gco2": metric.carbon_rate_gco2,
                    "timestamp": metric.timestamp,
                });

                match sync_state.gplay_client.sync_telemetry_batch(
                    "zenty-cluster-01",
                    &client_name,
                    "Enterprise",
                    vec![metric_json],
                ).await {
                    Ok(_) => {
                        println!("🌐 [GPLAY AI SYNC] Berhasil mengirim stream telemetri ke GPlay Gateway ({})", sync_state.gplay_client.endpoint_url);
                    }
                    Err(_e) => {
                        // Fallback silang aman
                    }
                }
            }
        }
    });

    // 5. Siapkan Router Axum
    let app = app_router(state);

    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8088);

    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    println!("🚀 Server aktif mendengarkan di: http://{}", addr);
    println!("📊 Web UI Dashboard    : http://{}/", addr);
    println!("📡 Telemetry Ingest     : POST http://{}/api/v1/telemetry/ingest", addr);
    println!("💓 Live Telemetry API   : GET  http://{}/api/v1/telemetry/live", addr);
    println!("📜 Merkle ESG Ledger    : GET  http://{}/api/v1/audit/esg-certificate", addr);
    println!("🌐 GPlay AI Gateway     : https://gplay.ctar.tech");
    println!("===================================================================");

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .unwrap_or_else(|e| panic!("Gagal mengikat port {}: {}", port, e));

    axum::serve(listener, app)
        .await
        .expect("Runtime server mengalami kendala");
}
