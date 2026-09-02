<?php

use Illuminate\Database\Migrations\Migration;
use Illuminate\Database\Schema\Blueprint;
use Illuminate\Support\Facades\Schema;

return new class extends Migration
{
    /**
     * Run the migrations.
     * CTARTech-ZentyElastis: AI Data Gateway & Knowledge Bank Tables
     */
    public function up(): void
    {
        // 1. Tabel Registrasi Kluster GPU / Node Klien
        if (!Schema::hasTable('zenty_clusters')) {
            Schema::create('zenty_clusters', function (Blueprint $table) {
                $table->id();
                $table->string('cluster_id')->unique()->index();
                $table->string('client_name');
                $table->string('tier')->default('Community'); // Community, Pro, Enterprise
                $table->integer('max_nodes')->default(8);
                $table->string('hardware_type')->default('NVIDIA_GPU'); // NVIDIA_GPU, AMD_ROCM, TPU, CPU
                $table->string('status')->default('ACTIVE'); // ACTIVE, SUSPENDED, DEGRADED
                $table->string('public_key_fingerprint')->nullable();
                $table->timestamp('license_expires_at')->nullable();
                $table->timestamps();
            });
        }

        // 2. Tabel Log Telemetri Multi-Dimensi (Time-Series Aggregations)
        if (!Schema::hasTable('zenty_telemetry_logs')) {
            Schema::create('zenty_telemetry_logs', function (Blueprint $table) {
                $table->id();
                $table->string('cluster_id')->index();
                $table->string('device_id')->index();
                $table->float('wattage', 10, 2);
                $table->float('temperature_c', 6, 2);
                $table->float('voltage_v', 6, 2)->nullable();
                $table->float('fan_speed_pct', 5, 2)->nullable();
                $table->integer('sm_clock_mhz')->nullable();
                $table->integer('mem_clock_mhz')->nullable();
                $table->float('gpu_utilization_pct', 5, 2)->nullable();
                $table->unsignedBigInteger('vram_used_mb')->nullable();
                $table->unsignedBigInteger('vram_total_mb')->nullable();
                $table->float('tokens_per_sec', 10, 2)->nullable();
                $table->float('joules_per_token', 10, 4)->nullable();
                $table->float('carbon_rate_gco2', 10, 2)->nullable();
                $table->json('throttle_reasons')->nullable();
                $table->string('state_transition')->nullable();
                $table->unsignedBigInteger('source_timestamp')->index();
                $table->timestamps();
            });
        }

        // 3. Tabel Bank Data Vektor Pola Beban Kerja AI (Pattern Embeddings)
        if (!Schema::hasTable('zenty_pattern_embeddings')) {
            Schema::create('zenty_pattern_embeddings', function (Blueprint $table) {
                $table->id();
                $table->string('cluster_id')->index();
                $table->string('model_workload_name')->default('LLM_INFERENCE'); // LLM_TRAINING, LLM_INFERENCE, HPC
                $table->text('pattern_vector')->nullable(); // Vector embedding representation
                $table->float('avg_power_draw_watt', 10, 2);
                $table->float('peak_power_draw_watt', 10, 2);
                $table->float('recommended_peak_limit_watt', 10, 2);
                $table->float('dynamic_throttle_ratio', 4, 2)->default(1.0);
                $table->integer('predicted_spike_in_minutes')->default(15);
                $table->timestamps();
            });
        }

        // 4. Tabel SOC Merkle Chain Energy & Carbon Audit Ledger
        if (!Schema::hasTable('zenty_energy_audit_ledgers')) {
            Schema::create('zenty_energy_audit_ledgers', function (Blueprint $table) {
                $table->id();
                $table->string('cluster_id')->index();
                $table->string('merkle_root_hash')->unique();
                $table->float('total_kwh_consumed', 12, 4);
                $table->float('total_co2_kg_emitted', 12, 4);
                $table->float('water_liters_used', 12, 2)->default(0.0);
                $table->float('avg_pue', 4, 2)->default(1.15);
                $table->timestamp('audit_period_start');
                $table->timestamp('audit_period_end');
                $table->string('verification_status')->default('VERIFIED');
                $table->timestamps();
            });
        }
    }

    /**
     * Reverse the migrations.
     */
    public function down(): void
    {
        Schema::dropIfExists('zenty_energy_audit_ledgers');
        Schema::dropIfExists('zenty_pattern_embeddings');
        Schema::dropIfExists('zenty_telemetry_logs');
        Schema::dropIfExists('zenty_clusters');
    }
};
