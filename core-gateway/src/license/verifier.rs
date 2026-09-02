use base64::engine::general_purpose::STANDARD as BASE64;
use base64::Engine;
use ed25519_dalek::pkcs8::DecodePublicKey;
use ed25519_dalek::{Signature, VerifyingKey};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicensePayload {
    pub issuer: String,
    pub client_id: String,
    pub tier: String,
    pub max_nodes: u32,
    pub features: Vec<String>,
    pub issued_at: u64,
    pub expires_at: u64,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicenseContainer {
    pub version: String,
    pub algorithm: String,
    pub payload: LicensePayload,
    pub signature_b64: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicenseStatus {
    pub is_valid: bool,
    pub client_id: String,
    pub tier: String,
    pub max_nodes: u32,
    pub expires_at: u64,
    pub days_remaining: i64,
    pub premium_features_enabled: bool,
    pub message: String,
}

/// Menghasilkan canonical JSON yang identik dengan generator python
pub fn to_canonical_json(payload: &LicensePayload) -> Result<Vec<u8>, serde_json::Error> {
    // Serialisasi dengan urutan key dan tanpa whitespace
    let value = serde_json::to_value(payload)?;
    let mut canonical_map = serde_json::Map::new();

    if let serde_json::Value::Object(map) = value {
        let mut sorted_keys: Vec<_> = map.keys().cloned().collect();
        sorted_keys.sort();
        for k in sorted_keys {
            if let Some(v) = map.get(&k) {
                canonical_map.insert(k, v.clone());
            }
        }
    }

    serde_json::to_vec(&serde_json::Value::Object(canonical_map))
}

/// Memverifikasi lisensi secara 100% offline menggunakan public_key.pem
pub fn verify_license_file<P: AsRef<Path>>(
    license_path: P,
    public_key_path: P,
) -> Result<LicenseStatus, String> {
    // 1. Baca file kunci publik (dukung format PEM maupun DER)
    let pub_pem = fs::read_to_string(public_key_path)
        .map_err(|e| format!("Gagal membaca public key: {}", e))?;

    let der_base64: String = pub_pem
        .lines()
        .filter(|line| !line.starts_with("-----"))
        .collect();

    let der_bytes = BASE64
        .decode(der_base64.trim())
        .map_err(|e| format!("Gagal decode PEM base64: {}", e))?;

    let verifying_key = VerifyingKey::from_public_key_der(&der_bytes)
        .map_err(|e| format!("Format public key Ed25519 DER tidak valid: {}", e))?;

    // 2. Baca file lisensi
    let lic_content = fs::read_to_string(license_path)
        .map_err(|e| format!("Gagal membaca file lisensi: {}", e))?;

    let container: LicenseContainer = serde_json::from_str(&lic_content)
        .map_err(|e| format!("Format file lisensi korup / tidak valid: {}", e))?;

    // 3. Verifikasi signature Ed25519
    let sig_bytes = BASE64
        .decode(&container.signature_b64)
        .map_err(|e| format!("Gagal decode signature base64: {}", e))?;

    let signature = Signature::from_slice(&sig_bytes)
        .map_err(|e| format!("Signature Ed25519 tidak valid: {}", e))?;

    let canonical_bytes = to_canonical_json(&container.payload)
        .map_err(|e| format!("Gagal kanonikalisasi payload lisensi: {}", e))?;

    use ed25519_dalek::Verifier;
    verifying_key
        .verify(&canonical_bytes, &signature)
        .map_err(|e| format!("Tanda tangan kriptografis TIDAK COCOK: {}", e))?;

    // 4. Periksa masa berlaku
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    let days_remaining = (container.payload.expires_at as i64 - now as i64) / 86400;
    let is_expired = now > container.payload.expires_at;

    if is_expired {
        return Ok(LicenseStatus {
            is_valid: false,
            client_id: container.payload.client_id,
            tier: container.payload.tier,
            max_nodes: container.payload.max_nodes,
            expires_at: container.payload.expires_at,
            days_remaining,
            premium_features_enabled: false,
            message: "Lisensi telah kadaluarsa! Fitur DeepOptiFlex dikunci.".to_string(),
        });
    }

    Ok(LicenseStatus {
        is_valid: true,
        client_id: container.payload.client_id,
        tier: container.payload.tier,
        max_nodes: container.payload.max_nodes,
        expires_at: container.payload.expires_at,
        days_remaining,
        premium_features_enabled: true,
        message: format!("Lisensi valid untuk {} hari ke depan.", days_remaining),
    })
}
