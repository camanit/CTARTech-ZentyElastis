use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ActuationStatus {
    ALLOW,
    THROTTLE,
    BLOCK,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActuationDecision {
    pub status: ActuationStatus,
    pub reason: String,
    pub latency_us: u64, // mikrodetik untuk membuktikan performa sub-milidetik (<0.1ms)
}

const CRITICAL_TEMPERATURE_C: f32 = 85.0;
const WARNING_TEMPERATURE_C: f32 = 80.0;
const CRITICAL_WATTAGE: f32 = 3500.0;
const WARNING_WATTAGE: f32 = 3000.0;

/// Mengevaluasi telemetri fisik masuk terhadap batas keselamatan hardware (Actuation Assurance)
/// Dijalankan dalam waktu mikrodetik (<0.1ms) di memori Rust Axum.
pub fn evaluate_actuation(temperature_c: f32, wattage: f32) -> ActuationDecision {
    let start = std::time::Instant::now();

    // 1. Pemeriksaan Ambang Batas Kritis (Emergency Circuit Breaker)
    if temperature_c > CRITICAL_TEMPERATURE_C || wattage > CRITICAL_WATTAGE {
        let elapsed_us = start.elapsed().as_micros() as u64;
        return ActuationDecision {
            status: ActuationStatus::BLOCK,
            reason: format!(
                "CRITICAL OVERLOAD: Temp {:.1}°C (max {:.1}°C) or Power {:.1}W (max {:.1}W). Emergency Circuit Breaker Triggered!",
                temperature_c, CRITICAL_TEMPERATURE_C, wattage, CRITICAL_WATTAGE
            ),
            latency_us: elapsed_us,
        };
    }

    // 2. Pemeriksaan Ambang Batas Peringatan (SLAShield™ & DeepOptiFlex™ Throttling)
    if temperature_c > WARNING_TEMPERATURE_C || wattage > WARNING_WATTAGE {
        let elapsed_us = start.elapsed().as_micros() as u64;
        return ActuationDecision {
            status: ActuationStatus::THROTTLE,
            reason: format!(
                "PEAK POWER WARNING: Temp {:.1}°C or Power {:.1}W nearing limit. Activating SLAShield™ dynamic workload throttling.",
                temperature_c, wattage
            ),
            latency_us: elapsed_us,
        };
    }

    // 3. Normal - Parameter aman diizinkan
    let elapsed_us = start.elapsed().as_micros() as u64;
    ActuationDecision {
        status: ActuationStatus::ALLOW,
        reason: "Telemetry within safe operational envelope. Actuation approved.".to_string(),
        latency_us: elapsed_us,
    }
}
