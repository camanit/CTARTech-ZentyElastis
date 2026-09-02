use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkloadPatternEmbedding {
    pub cluster_id: String,
    pub timestamp: u64,
    pub avg_temperature_c: f32,
    pub total_wattage: f32,
    pub active_nodes: u32,
    pub prompt_queue_density: f32,
    pub carbon_intensity_gco2_kwh: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeepOptiFlexRecommendation {
    pub recommended_peak_limit_watt: f32,
    pub dynamic_throttle_ratio: f32,
    pub predicted_spike_in_minutes: u32,
    pub advice: String,
}

#[derive(Debug, Clone)]
pub struct GPlayAiClient {
    pub endpoint_url: String,
    pub api_key: String,
}

impl GPlayAiClient {
    pub fn new(endpoint_url: Option<String>, api_key: Option<String>) -> Self {
        Self {
            endpoint_url: endpoint_url.unwrap_or_else(|| "https://gplay.ctar.tech".to_string()),
            api_key: api_key.unwrap_or_else(|| "gplay_enterprise_token".to_string()),
        }
    }

    /// Menghasilkan rekomendasi cerdas DeepOptiFlex dari GPlay AI Data Gateway
    pub fn get_predictive_recommendation(
        &self,
        current_wattage: f32,
        active_nodes: u32,
    ) -> DeepOptiFlexRecommendation {
        // Simulasi kalkulasi model AI terpusat gplay.ctar.tech
        let dynamic_limit = (active_nodes as f32) * 450.0 * 0.85; // 15% peak shave
        DeepOptiFlexRecommendation {
            recommended_peak_limit_watt: dynamic_limit,
            dynamic_throttle_ratio: if current_wattage > dynamic_limit { 0.85 } else { 1.0 },
            predicted_spike_in_minutes: 15,
            advice: format!(
                "GPlay AI Intelligence: Peak shave recommendation aktif. Target maks: {:.1}W",
                dynamic_limit
            ),
        }
    }
}
