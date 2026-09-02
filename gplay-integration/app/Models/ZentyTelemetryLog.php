<?php

namespace App\Models;

use Illuminate\Database\Eloquent\Factories\HasFactory;
use Illuminate\Database\Eloquent\Model;

class ZentyTelemetryLog extends Model
{
    use HasFactory;

    protected $table = 'zenty_telemetry_logs';

    protected $fillable = [
        'cluster_id',
        'device_id',
        'wattage',
        'temperature_c',
        'voltage_v',
        'fan_speed_pct',
        'sm_clock_mhz',
        'mem_clock_mhz',
        'gpu_utilization_pct',
        'vram_used_mb',
        'vram_total_mb',
        'tokens_per_sec',
        'joules_per_token',
        'carbon_rate_gco2',
        'throttle_reasons',
        'state_transition',
        'source_timestamp',
    ];

    protected $casts = [
        'throttle_reasons' => 'array',
        'wattage' => 'float',
        'temperature_c' => 'float',
        'voltage_v' => 'float',
        'fan_speed_pct' => 'float',
        'tokens_per_sec' => 'float',
        'joules_per_token' => 'float',
        'carbon_rate_gco2' => 'float',
        'vram_used_mb' => 'integer',
        'vram_total_mb' => 'integer',
        'source_timestamp' => 'integer',
    ];

    public function cluster()
    {
        return $this->belongsTo(ZentyCluster::class, 'cluster_id', 'cluster_id');
    }
}
