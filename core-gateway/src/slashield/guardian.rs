use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum SLAStatus {
    OPTIMAL,           // Kinerja prima di atas target SLA
    ADAPTIVE_THROTTLE, // Mendekati batas SLA, optimasi daya dikurangi
    RESCUE,            // Bahaya pelanggaran SLA! Batas daya dinaikkan darurat
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SLAShieldPolicy {
    pub min_target_tps: f32,       // Default: 120.0 TPS
    pub max_acceptable_latency_ms: f32, // Default: 45.0 ms
    pub rescue_headroom_pct: f32,  // Default: 15.0%
}

impl Default for SLAShieldPolicy {
    fn default() -> Self {
        Self {
            min_target_tps: 120.0,
            max_acceptable_latency_ms: 45.0,
            rescue_headroom_pct: 15.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SLAShieldDecision {
    pub status: SLAStatus,
    pub current_tps: f32,
    pub target_tps: f32,
    pub override_power_cap_watt: Option<f64>,
    pub advisory: String,
    pub latency_us: u64,
}

#[derive(Clone)]
pub struct SLAShieldGuardian {
    pub policy: SLAShieldPolicy,
    history_tps: Arc<Mutex<Vec<f32>>>,
}

impl SLAShieldGuardian {
    pub fn new(policy: SLAShieldPolicy) -> Self {
        Self {
            policy,
            history_tps: Arc::new(Mutex::new(Vec::with_capacity(30))),
        }
    }

    /// Mengevaluasi throughput AI inference dan mencegah pelanggaran SLA korporat
    pub fn evaluate(&self, current_tps: f32, current_wattage: f64, current_cap: f64) -> SLAShieldDecision {
        let start = std::time::Instant::now();

        {
            let mut hist = self.history_tps.lock().unwrap();
            if hist.len() >= 30 {
                hist.remove(0);
            }
            hist.push(current_tps);
        }

        let elapsed_us = start.elapsed().as_micros() as u64;

        // 1. Kondisi RESCUE: Throughput anjlok di bawah ambang batas minimal
        if current_tps < self.policy.min_target_tps {
            let rescue_cap = current_cap * (1.0 + (self.policy.rescue_headroom_pct as f64 / 100.0));
            return SLAShieldDecision {
                status: SLAStatus::RESCUE,
                current_tps,
                target_tps: self.policy.min_target_tps,
                override_power_cap_watt: Some(rescue_cap.min(3500.0)),
                advisory: format!(
                    "⚠️ SLAShield™ RESCUE ACTIVE: Throughput {:.1} TPS di bawah target ({:.1} TPS). Menambah kuota daya ke {:.0}W demi menjaga SLA.",
                    current_tps, self.policy.min_target_tps, rescue_cap
                ),
                latency_us: elapsed_us,
            };
        }

        // 2. Kondisi ADAPTIVE_THROTTLE: Throughput mendekati batas (rentang buffer +15%)
        let warning_threshold = self.policy.min_target_tps * 1.15;
        if current_tps < warning_threshold {
            let adjusted_cap = current_wattage.max(current_cap);
            return SLAShieldDecision {
                status: SLAStatus::ADAPTIVE_THROTTLE,
                current_tps,
                target_tps: self.policy.min_target_tps,
                override_power_cap_watt: Some(adjusted_cap),
                advisory: format!(
                    "⚡ SLAShield™ BUFFER: Throughput {:.1} TPS mendekati batas SLA. Menstabilkan envelope daya di {:.0}W.",
                    current_tps, adjusted_cap
                ),
                latency_us: elapsed_us,
            };
        }

        // 3. Kondisi OPTIMAL: Performa inferensi aman, penghematan DeepOptiFlex diizinkan 100%
        SLAShieldDecision {
            status: SLAStatus::OPTIMAL,
            current_tps,
            target_tps: self.policy.min_target_tps,
            override_power_cap_watt: None,
            advisory: format!(
                "● SLAShield™ OPTIMAL: Performa komputasi prima ({:.1} TPS). Optimasi DeepOptiFlex diizinkan penuh.",
                current_tps
            ),
            latency_us: elapsed_us,
        }
    }
}
