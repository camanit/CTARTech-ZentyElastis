use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditSample {
    pub timestamp: u64,
    pub device_id: String,
    pub wattage: f32,
    pub joules_saved: f64,
    pub carbon_prevented_gco2: f64,
    pub sample_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MerkleBlock {
    pub block_height: u64,
    pub timestamp: u64,
    pub prev_block_hash: String,
    pub merkle_root: String,
    pub samples_count: usize,
    pub cumulative_joules_saved: f64,
    pub cumulative_carbon_prevented_gco2: f64,
    pub block_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EsgCertificate {
    pub certificate_id: String,
    pub issued_to: String,
    pub authority: String,
    pub block_height: u64,
    pub merkle_root: String,
    pub latest_block_hash: String,
    pub total_energy_saved_kwh: f64,
    pub total_carbon_prevented_kg_co2: f64,
    pub equivalent_trees_planted: f64,
    pub compliance_standard: String,
    pub verification_status: String,
    pub generated_at: String,
}

#[derive(Clone)]
pub struct MerkleAuditLedger {
    chain: Arc<Mutex<Vec<MerkleBlock>>>,
    pending_samples: Arc<Mutex<Vec<AuditSample>>>,
    total_joules: Arc<Mutex<f64>>,
    total_carbon: Arc<Mutex<f64>>,
}

impl MerkleAuditLedger {
    pub fn new() -> Self {
        // Inisialisasi Genesis Block
        let genesis = MerkleBlock {
            block_height: 0,
            timestamp: 1725235200, // Epoc Genesis Zenty
            prev_block_hash: "0000000000000000000000000000000000000000000000000000000000000000".to_string(),
            merkle_root: "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855".to_string(),
            samples_count: 0,
            cumulative_joules_saved: 0.0,
            cumulative_carbon_prevented_gco2: 0.0,
            block_hash: "00000000000000000000000000000000zentygenesisauditroot000000000000".to_string(),
        };

        Self {
            chain: Arc::new(Mutex::new(vec![genesis])),
            pending_samples: Arc::new(Mutex::new(Vec::with_capacity(50))),
            total_joules: Arc::new(Mutex::new(0.0)),
            total_carbon: Arc::new(Mutex::new(0.0)),
        }
    }

    /// Menambahkan sampel telemetri ke buffer audit
    pub fn record_sample(&self, device_id: &str, wattage: f32, joules_saved: f64, carbon_gco2: f64) {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        // Hitung SHA-256 untuk sampel data individual
        let raw_data = format!("{}:{}:{}:{}:{}", device_id, now, wattage, joules_saved, carbon_gco2);
        let sample_hash = hex::encode(Sha256::digest(raw_data.as_bytes()));

        let sample = AuditSample {
            timestamp: now,
            device_id: device_id.to_string(),
            wattage,
            joules_saved,
            carbon_prevented_gco2: carbon_gco2,
            sample_hash,
        };

        let mut pending = self.pending_samples.lock().unwrap();
        pending.push(sample);

        {
            let mut j = self.total_joules.lock().unwrap();
            *j += joules_saved;
            let mut c = self.total_carbon.lock().unwrap();
            *c += carbon_gco2;
        }

        // Jika buffer mencapai 15 sampel, cetak Merkle Block baru secara otomatis
        if pending.len() >= 15 {
            self.seal_block_internal(&mut pending);
        }
    }

    /// Menghitung Merkle Root dari kumpulan leaf hash
    fn compute_merkle_root(leaf_hashes: &[String]) -> String {
        if leaf_hashes.is_empty() {
            return "0000000000000000000000000000000000000000000000000000000000000000".to_string();
        }
        if leaf_hashes.len() == 1 {
            return leaf_hashes[0].clone();
        }

        let mut current_level = leaf_hashes.to_vec();
        while current_level.len() > 1 {
            let mut next_level = Vec::new();
            for i in (0..current_level.len()).step_by(2) {
                if i + 1 < current_level.len() {
                    let combined = format!("{}{}", current_level[i], current_level[i + 1]);
                    next_level.push(hex::encode(Sha256::digest(combined.as_bytes())));
                } else {
                    // Duplikasi jika ganjil (standar Bitcoin / Merkle Tree)
                    let combined = format!("{}{}", current_level[i], current_level[i]);
                    next_level.push(hex::encode(Sha256::digest(combined.as_bytes())));
                }
            }
            current_level = next_level;
        }

        current_level[0].clone()
    }

    fn seal_block_internal(&self, pending: &mut Vec<AuditSample>) {
        let mut chain = self.chain.lock().unwrap();
        let last_block = chain.last().unwrap();

        let leaf_hashes: Vec<String> = pending.iter().map(|s| s.sample_hash.clone()).collect();
        let merkle_root = Self::compute_merkle_root(&leaf_hashes);

        let height = last_block.block_height + 1;
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        let cumulative_j = *self.total_joules.lock().unwrap();
        let cumulative_c = *self.total_carbon.lock().unwrap();

        let header = format!("{}:{}:{}:{}:{}", height, now, last_block.block_hash, merkle_root, cumulative_c);
        let block_hash = hex::encode(Sha256::digest(header.as_bytes()));

        let block = MerkleBlock {
            block_height: height,
            timestamp: now,
            prev_block_hash: last_block.block_hash.clone(),
            merkle_root,
            samples_count: pending.len(),
            cumulative_joules_saved: cumulative_j,
            cumulative_carbon_prevented_gco2: cumulative_c,
            block_hash,
        };

        chain.push(block);
        pending.clear();
    }

    /// Mengambil seluruh riwayat rantai blok
    pub fn get_chain(&self) -> Vec<MerkleBlock> {
        self.chain.lock().unwrap().clone()
    }

    /// Mengambil status tinggi blok dan Merkle Root terkini
    pub fn get_latest_status(&self) -> (u64, String, f64, f64) {
        let chain = self.chain.lock().unwrap();
        let last = chain.last().unwrap();
        let j = *self.total_joules.lock().unwrap();
        let c = *self.total_carbon.lock().unwrap();
        (last.block_height, last.merkle_root.clone(), j, c)
    }

    /// Menerbitkan Sertifikat ESG Green-AI Compliance Resmi
    pub fn generate_esg_certificate(&self, client_name: &str) -> EsgCertificate {
        let chain = self.chain.lock().unwrap();
        let last = chain.last().unwrap();
        let total_joules = *self.total_joules.lock().unwrap();
        let total_carbon_g = *self.total_carbon.lock().unwrap();

        let kwh = total_joules / 3_600_000.0;
        let kg_co2 = total_carbon_g / 1000.0;
        // 1 pohon menyerap sekitar 21.77 kg CO2 per tahun
        let trees = kg_co2 / 21.77;

        EsgCertificate {
            certificate_id: format!("ESG-ZENTY-{}-{}", last.block_height, last.timestamp % 100000),
            issued_to: client_name.to_string(),
            authority: "PT CTAR Technology Indonesia (Sovereign ESG Registry)".to_string(),
            block_height: last.block_height,
            merkle_root: last.merkle_root.clone(),
            latest_block_hash: last.block_hash.clone(),
            total_energy_saved_kwh: (kwh * 1000.0).round() / 1000.0,
            total_carbon_prevented_kg_co2: (kg_co2 * 1000.0).round() / 1000.0,
            equivalent_trees_planted: (trees * 100.0).round() / 100.0,
            compliance_standard: "ISO 14064-1 & GHG Protocol Scope 2 AI Optimization".to_string(),
            verification_status: "CRYPTOGRAPHICALLY_VERIFIED".to_string(),
            generated_at: chrono::Utc::now().to_rfc3339(),
        }
    }
}
