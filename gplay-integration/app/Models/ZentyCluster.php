<?php

namespace App\Models;

use Illuminate\Database\Eloquent\Factories\HasFactory;
use Illuminate\Database\Eloquent\Model;

class ZentyCluster extends Model
{
    use HasFactory;

    protected $table = 'zenty_clusters';

    protected $fillable = [
        'cluster_id',
        'client_name',
        'tier',
        'max_nodes',
        'hardware_type',
        'status',
        'public_key_fingerprint',
        'license_expires_at',
    ];

    protected $casts = [
        'license_expires_at' => 'datetime',
        'max_nodes' => 'integer',
    ];

    public function telemetryLogs()
    {
        return $this->hasMany(ZentyTelemetryLog::class, 'cluster_id', 'cluster_id');
    }

    public function patternEmbeddings()
    {
        return $this->hasMany(ZentyPatternEmbedding::class, 'cluster_id', 'cluster_id');
    }

    public function auditLedgers()
    {
        return $this->hasMany(ZentyEnergyAuditLedger::class, 'cluster_id', 'cluster_id');
    }
}
