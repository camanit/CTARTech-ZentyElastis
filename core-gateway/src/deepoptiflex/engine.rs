use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

/// Parameter kebijakan optimasi DeepOptiFlex™
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeepOptiFlexPolicy {
    /// Target persentase pemotongan beban puncak (default: 18.5%)
    pub peak_shave_target_pct: f64,
    /// Batas daya keras maksimum hardware (Watt)
    pub max_hardware_ceiling_watt: f64,
    /// Ambang peringatan sebelum throttling (Watt)
    pub warning_threshold_watt: f64,
    /// Rasio pelambatan dinamis (0.50 - 1.00)
    pub dynamic_clamp_ratio: f64,
    /// Mode operasi: ONLINE_GPLAY_SYNC atau AIR_GAPPED_LOCAL
    pub mode: String,
}

impl Default for DeepOptiFlexPolicy {
    fn default() -> Self {
        Self {
            peak_shave_target_pct: 18.5,
            max_hardware_ceiling_watt: 3500.0,
            warning_threshold_watt: 2800.0,
            dynamic_clamp_ratio: 0.815,
            mode: "AIR_GAPPED_LOCAL".to_string(),
        }
    }
}

/// Hasil evaluasi optimasi daya per siklus
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationResult {
    pub current_wattage: f64,
    pub recommended_cap_watt: f64,
    pub power_saved_watt: f64,
    pub savings_percentage: f64,
    pub is_peak_shaving_active: bool,
    pub advisory_message: String,
    pub carbon_prevented_gco2: f64,
}

/// Mesin kalkulasi prediktif DeepOptiFlex™
#[derive(Debug, Clone)]
pub struct DeepOptiFlexEngine {
    policy: Arc<Mutex<DeepOptiFlexPolicy>>,
    wattage_window: Arc<Mutex<VecDeque<f64>>>,
    window_capacity: usize,
}

impl DeepOptiFlexEngine {
    pub fn new(policy: DeepOptiFlexPolicy) -> Self {
        Self {
            policy: Arc::new(Mutex::new(policy)),
            wattage_window: Arc::new(Mutex::new(VecDeque::with_capacity(60))),
            window_capacity: 60,
        }
    }

    /// Evaluasi sampel daya baru dan hitung envelope peak shaving
    pub fn evaluate_sample(&self, current_wattage: f64) -> OptimizationResult {
        let policy = self.policy.lock().unwrap().clone();
        let mut history = self.wattage_window.lock().unwrap();

        if history.len() >= self.window_capacity {
            history.pop_front();
        }
        history.push_back(current_wattage);

        // Hitung Exponential Moving Average (EMA) sederhana
        let count = history.len() as f64;
        let sum: f64 = history.iter().sum();
        let moving_avg = if count > 0.0 { sum / count } else { current_wattage };

        // Kalkulasi batas daya yang direkomendasikan
        let baseline = moving_avg.max(current_wattage);
        let recommended_cap = (baseline * (1.0 - (policy.peak_shave_target_pct / 100.0)))
            .min(policy.max_hardware_ceiling_watt);

        let power_saved = if current_wattage > recommended_cap {
            current_wattage - recommended_cap
        } else {
            0.0
        };

        let is_active = current_wattage > policy.warning_threshold_watt;

        let advisory = if current_wattage >= policy.max_hardware_ceiling_watt {
            format!("CRITICAL: Daya melampaui batas keras {:.1}W! Actuation Assurance memicu clamping darurat.", policy.max_hardware_ceiling_watt)
        } else if is_active {
            format!("DeepOptiFlex™ AKTIF: Memotong lonjakan daya puncak sebesar {:.1}% (Hemat {:.1}W).", policy.peak_shave_target_pct, power_saved)
        } else {
            format!("DeepOptiFlex™ SIAGA: Pola beban normal ({:.1}W). Envelope daya stabil.", current_wattage)
        };

        // Estimasi pencegahan emisi karbon (~450g CO2 per kWh di Indonesia)
        let carbon_prevented = (power_saved / 1000.0) * (450.0 / 3600.0);

        OptimizationResult {
            current_wattage,
            recommended_cap_watt: recommended_cap,
            power_saved_watt: power_saved,
            savings_percentage: if current_wattage > 0.0 { (power_saved / current_wattage) * 100.0 } else { 0.0 },
            is_peak_shaving_active: is_active,
            advisory_message: advisory,
            carbon_prevented_gco2: carbon_prevented,
        }
    }

    pub fn get_policy(&self) -> DeepOptiFlexPolicy {
        self.policy.lock().unwrap().clone()
    }
}
