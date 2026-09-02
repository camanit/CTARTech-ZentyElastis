<?php

namespace App\Models;

use Illuminate\Database\Eloquent\Factories\HasFactory;
use Illuminate\Database\Eloquent\Model;

class ZentyEnergyAuditLedger extends Model
{
    use HasFactory;

    protected $table = 'zenty_energy_audit_ledgers';

    protected $fillable = [
        'cluster_id',
        'merkle_root_hash',
        'total_kwh_consumed',
        'total_co2_kg_emitted',
        'water_liters_used',
        'avg_pue',
        'audit_period_start',
        'audit_period_end',
        'verification_status',
    ];

    protected $casts = [
        'total_kwh_consumed' => 'float',
        'total_co2_kg_emitted' => 'float',
        'water_liters_used' => 'float',
        'avg_pue' => 'float',
        'audit_period_start' => 'datetime',
        'audit_period_end' => 'datetime',
    ];

    public function cluster()
    {
        return $this->belongsTo(ZentyCluster::class, 'cluster_id', 'cluster_id');
    }
}
