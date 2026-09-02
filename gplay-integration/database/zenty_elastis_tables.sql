-- ==============================================================
-- CTARTech-ZentyElastis: AI Data Gateway & Telemetry Mesh Tables
-- Import file SQL ini langsung via phpMyAdmin di Hosting / cPanel
-- Database Engine: MySQL / MariaDB (InnoDB, utf8mb4_unicode_ci)
-- ==============================================================

SET FOREIGN_KEY_CHECKS = 0;

-- --------------------------------------------------------
-- 1. Tabel: zenty_clusters (Registrasi Kluster GPU / Klien)
-- --------------------------------------------------------
CREATE TABLE IF NOT EXISTS `zenty_clusters` (
  `id` bigint(20) UNSIGNED NOT NULL AUTO_INCREMENT,
  `cluster_id` varchar(191) NOT NULL,
  `client_name` varchar(255) NOT NULL,
  `tier` varchar(50) NOT NULL DEFAULT 'Community',
  `max_nodes` int(11) NOT NULL DEFAULT 8,
  `hardware_type` varchar(100) NOT NULL DEFAULT 'NVIDIA_GPU',
  `status` varchar(50) NOT NULL DEFAULT 'ACTIVE',
  `public_key_fingerprint` varchar(255) DEFAULT NULL,
  `license_expires_at` timestamp NULL DEFAULT NULL,
  `created_at` timestamp NULL DEFAULT NULL,
  `updated_at` timestamp NULL DEFAULT NULL,
  PRIMARY KEY (`id`),
  UNIQUE KEY `zenty_clusters_cluster_id_unique` (`cluster_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- --------------------------------------------------------
-- 2. Tabel: zenty_telemetry_logs (Riwayat Telemetri Multi-Dimensi)
-- --------------------------------------------------------
CREATE TABLE IF NOT EXISTS `zenty_telemetry_logs` (
  `id` bigint(20) UNSIGNED NOT NULL AUTO_INCREMENT,
  `cluster_id` varchar(191) NOT NULL,
  `device_id` varchar(191) NOT NULL,
  `wattage` double(10,2) NOT NULL,
  `temperature_c` double(6,2) NOT NULL,
  `voltage_v` double(6,2) DEFAULT NULL,
  `fan_speed_pct` double(5,2) DEFAULT NULL,
  `sm_clock_mhz` int(11) DEFAULT NULL,
  `mem_clock_mhz` int(11) DEFAULT NULL,
  `gpu_utilization_pct` double(5,2) DEFAULT NULL,
  `vram_used_mb` bigint(20) UNSIGNED DEFAULT NULL,
  `vram_total_mb` bigint(20) UNSIGNED DEFAULT NULL,
  `tokens_per_sec` double(10,2) DEFAULT NULL,
  `joules_per_token` double(10,4) DEFAULT NULL,
  `carbon_rate_gco2` double(10,2) DEFAULT NULL,
  `throttle_reasons` longtext CHARACTER SET utf8mb4 COLLATE utf8mb4_bin DEFAULT NULL CHECK (json_valid(`throttle_reasons`)),
  `state_transition` varchar(100) DEFAULT NULL,
  `source_timestamp` bigint(20) UNSIGNED NOT NULL,
  `created_at` timestamp NULL DEFAULT NULL,
  `updated_at` timestamp NULL DEFAULT NULL,
  PRIMARY KEY (`id`),
  KEY `zenty_telemetry_logs_cluster_id_index` (`cluster_id`),
  KEY `zenty_telemetry_logs_device_id_index` (`device_id`),
  KEY `zenty_telemetry_logs_source_timestamp_index` (`source_timestamp`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- --------------------------------------------------------
-- 3. Tabel: zenty_pattern_embeddings (Bank Data Vektor Pola AI)
-- --------------------------------------------------------
CREATE TABLE IF NOT EXISTS `zenty_pattern_embeddings` (
  `id` bigint(20) UNSIGNED NOT NULL AUTO_INCREMENT,
  `cluster_id` varchar(191) NOT NULL,
  `model_workload_name` varchar(100) NOT NULL DEFAULT 'LLM_INFERENCE',
  `pattern_vector` longtext DEFAULT NULL,
  `avg_power_draw_watt` double(10,2) NOT NULL,
  `peak_power_draw_watt` double(10,2) NOT NULL,
  `recommended_peak_limit_watt` double(10,2) NOT NULL,
  `dynamic_throttle_ratio` double(4,2) NOT NULL DEFAULT 1.00,
  `predicted_spike_in_minutes` int(11) NOT NULL DEFAULT 15,
  `created_at` timestamp NULL DEFAULT NULL,
  `updated_at` timestamp NULL DEFAULT NULL,
  PRIMARY KEY (`id`),
  KEY `zenty_pattern_embeddings_cluster_id_index` (`cluster_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- --------------------------------------------------------
-- 4. Tabel: zenty_energy_audit_ledgers (SOC Merkle Chain Audit)
-- --------------------------------------------------------
CREATE TABLE IF NOT EXISTS `zenty_energy_audit_ledgers` (
  `id` bigint(20) UNSIGNED NOT NULL AUTO_INCREMENT,
  `cluster_id` varchar(191) NOT NULL,
  `merkle_root_hash` varchar(191) NOT NULL,
  `total_kwh_consumed` double(12,4) NOT NULL,
  `total_co2_kg_emitted` double(12,4) NOT NULL,
  `water_liters_used` double(12,2) NOT NULL DEFAULT 0.00,
  `avg_pue` double(4,2) NOT NULL DEFAULT 1.15,
  `audit_period_start` timestamp NOT NULL,
  `audit_period_end` timestamp NOT NULL,
  `verification_status` varchar(50) NOT NULL DEFAULT 'VERIFIED',
  `created_at` timestamp NULL DEFAULT NULL,
  `updated_at` timestamp NULL DEFAULT NULL,
  PRIMARY KEY (`id`),
  UNIQUE KEY `zenty_energy_audit_ledgers_merkle_root_hash_unique` (`merkle_root_hash`),
  KEY `zenty_energy_audit_ledgers_cluster_id_index` (`cluster_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

SET FOREIGN_KEY_CHECKS = 1;
