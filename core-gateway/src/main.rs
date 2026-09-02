mod breaker;
mod gateway;
mod gplay;
mod license;

use gateway::{app_router, AppState};
use gplay::GPlayAiClient;
use license::verify_license_file;
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
    println!("⚡ CTARTech-ZentyElastis: Core Runtime Gateway (Axum Engine <0.1ms)");
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

    // 3. Konfigurasi State Aplikasi
    let state = AppState {
        gplay_client: GPlayAiClient::new(
            Some("https://gplay.ctar.tech".to_string()),
            Some("gplay_live_api_token".to_string()),
        ),
        license_status: Arc::new(Mutex::new(initial_license)),
        total_ingested: Arc::new(Mutex::new(0)),
    };

    // 4. Siapkan Router Axum
    let app = app_router(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    println!("🚀 Server aktif mendengarkan di: http://{}", addr);
    println!("📡 Endpoint Telemetri: POST http://{}/api/v1/telemetry/ingest", addr);
    println!("💓 Endpoint Health   : GET  http://{}/api/v1/health", addr);
    println!("🌐 GPlay AI Gateway  : https://gplay.ctar.tech");
    println!("===================================================================");

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Gagal mengikat port 8000");

    axum::serve(listener, app)
        .await
        .expect("Runtime server mengalami kendala");
}
