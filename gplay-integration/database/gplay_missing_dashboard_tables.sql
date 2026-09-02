-- =========================================================================
-- GPlay AI Platform: Fix Missing Dashboard & RBAC Tables
-- Eksekusi SQL ini di phpMyAdmin database 'u289640181_gplay'
-- Ini akan memperbaiki error 500 pada:
-- - /dashboard/audit-trail (audit_trail_logs)
-- - /dashboard/billing (invoices)
-- - /dashboard/team (tenant_team_members)
-- - /dashboard/sso (sso_configurations)
-- =========================================================================

SET FOREIGN_KEY_CHECKS = 0;

-- 1. Tambahkan kolom pendukung di tabel tenants jika belum ada
ALTER TABLE `tenants` 
  ADD COLUMN IF NOT EXISTS `custom_domain` varchar(255) NULL AFTER `email`,
  ADD COLUMN IF NOT EXISTS `custom_domain_status` varchar(50) NOT NULL DEFAULT 'unverified' AFTER `custom_domain`,
  ADD COLUMN IF NOT EXISTS `cname_target` varchar(255) NULL DEFAULT 'custom.gplay.test' AFTER `custom_domain_status`,
  ADD COLUMN IF NOT EXISTS `notes` text NULL AFTER `cname_target`;

-- 2. Tabel: tenant_team_members (RBAC Team)
CREATE TABLE IF NOT EXISTS `tenant_team_members` (
  `id` bigint(20) UNSIGNED NOT NULL AUTO_INCREMENT,
  `tenant_id` bigint(20) UNSIGNED NOT NULL,
  `user_id` bigint(20) UNSIGNED DEFAULT NULL,
  `email` varchar(255) NOT NULL,
  `name` varchar(255) DEFAULT NULL,
  `role` enum('admin','developer','analyst','auditor','billing_viewer') NOT NULL DEFAULT 'developer',
  `invited_by_email` varchar(255) DEFAULT NULL,
  `status` enum('pending','active','revoked') NOT NULL DEFAULT 'active',
  `permissions` text DEFAULT NULL,
  `last_active_at` timestamp NULL DEFAULT NULL,
  `created_at` timestamp NULL DEFAULT NULL,
  `updated_at` timestamp NULL DEFAULT NULL,
  PRIMARY KEY (`id`),
  UNIQUE KEY `tenant_team_members_tenant_id_email_unique` (`tenant_id`,`email`),
  KEY `tenant_team_members_tenant_id_role_index` (`tenant_id`,`role`),
  KEY `tenant_team_members_tenant_id_status_index` (`tenant_id`,`status`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- 3. Tabel: audit_trail_logs (Immutable ISO 27001 / UU PDP Audit)
CREATE TABLE IF NOT EXISTS `audit_trail_logs` (
  `id` bigint(20) UNSIGNED NOT NULL AUTO_INCREMENT,
  `tenant_id` bigint(20) UNSIGNED NOT NULL,
  `actor_user_id` bigint(20) UNSIGNED DEFAULT NULL,
  `actor_email` varchar(255) DEFAULT NULL,
  `actor_role` varchar(50) DEFAULT NULL,
  `action` varchar(100) NOT NULL,
  `resource_type` varchar(80) DEFAULT NULL,
  `resource_id` varchar(80) DEFAULT NULL,
  `description` text DEFAULT NULL,
  `ip_address` varchar(45) DEFAULT NULL,
  `user_agent` text DEFAULT NULL,
  `payload_hash` varchar(64) DEFAULT NULL,
  `severity` enum('info','warning','critical') NOT NULL DEFAULT 'info',
  `created_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
  PRIMARY KEY (`id`),
  KEY `audit_trail_logs_tenant_id_created_at_index` (`tenant_id`,`created_at`),
  KEY `audit_trail_logs_tenant_id_action_index` (`tenant_id`,`action`),
  KEY `audit_trail_logs_tenant_id_severity_index` (`tenant_id`,`severity`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- 4. Tabel: sso_configurations (SSO / SAML 2.0 / OIDC)
CREATE TABLE IF NOT EXISTS `sso_configurations` (
  `id` bigint(20) UNSIGNED NOT NULL AUTO_INCREMENT,
  `tenant_id` bigint(20) UNSIGNED NOT NULL,
  `provider` enum('okta','azure_ad','google_workspace','custom_saml') NOT NULL DEFAULT 'okta',
  `protocol` enum('saml2','oidc') NOT NULL DEFAULT 'saml2',
  `idp_entity_id` varchar(255) DEFAULT NULL,
  `idp_sso_url` varchar(255) DEFAULT NULL,
  `idp_certificate` text DEFAULT NULL,
  `oidc_client_id` varchar(255) DEFAULT NULL,
  `oidc_client_secret` text DEFAULT NULL,
  `oidc_discovery_url` varchar(255) DEFAULT NULL,
  `sp_entity_id` varchar(255) DEFAULT NULL,
  `sp_acs_url` varchar(255) DEFAULT NULL,
  `attr_email` varchar(100) NOT NULL DEFAULT 'email',
  `attr_name` varchar(100) NOT NULL DEFAULT 'name',
  `attr_role` varchar(100) DEFAULT NULL,
  `is_active` tinyint(1) NOT NULL DEFAULT 0,
  `force_sso` tinyint(1) NOT NULL DEFAULT 0,
  `last_auth_at` timestamp NULL DEFAULT NULL,
  `created_at` timestamp NULL DEFAULT NULL,
  `updated_at` timestamp NULL DEFAULT NULL,
  PRIMARY KEY (`id`),
  UNIQUE KEY `sso_configurations_tenant_id_unique` (`tenant_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- 5. Tabel: invoices (Billing & Invoices)
CREATE TABLE IF NOT EXISTS `invoices` (
  `id` bigint(20) UNSIGNED NOT NULL AUTO_INCREMENT,
  `tenant_id` bigint(20) UNSIGNED NOT NULL,
  `invoice_number` varchar(255) NOT NULL,
  `amount` decimal(12,2) NOT NULL,
  `currency` varchar(10) NOT NULL DEFAULT 'USD',
  `package_tier` varchar(50) NOT NULL DEFAULT 'pro',
  `status` varchar(50) NOT NULL DEFAULT 'unpaid',
  `payment_method` varchar(100) DEFAULT NULL,
  `due_date` timestamp NULL DEFAULT NULL,
  `paid_at` timestamp NULL DEFAULT NULL,
  `notes` text DEFAULT NULL,
  `created_at` timestamp NULL DEFAULT NULL,
  `updated_at` timestamp NULL DEFAULT NULL,
  PRIMARY KEY (`id`),
  UNIQUE KEY `invoices_invoice_number_unique` (`invoice_number`),
  KEY `invoices_tenant_id_index` (`tenant_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

SET FOREIGN_KEY_CHECKS = 1;
