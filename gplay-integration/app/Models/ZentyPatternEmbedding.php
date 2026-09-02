<?php

namespace App\Models;

use Illuminate\Database\Eloquent\Factories\HasFactory;
use Illuminate\Database\Eloquent\Model;

class ZentyPatternEmbedding extends Model
{
    use HasFactory;

    protected $table = 'zenty_pattern_embeddings';

    protected $fillable = [
        'cluster_id',
        'model_workload_name',
        'pattern_vector',
        'avg_power_draw_watt',
        'peak_power_draw_watt',
        'recommended_peak_limit_watt',
        'dynamic_throttle_ratio',
        'predicted_spike_in_minutes',
    ];

    protected $casts = [
        'avg_power_draw_watt' => 'float',
        'peak_power_draw_watt' => 'float',
        'recommended_peak_limit_watt' => 'float',
        'dynamic_throttle_ratio' => 'float',
        'predicted_spike_in_minutes' => 'integer',
    ];

    public function cluster()
    {
        return $this->belongsTo(ZentyCluster::class, 'cluster_id', 'cluster_id');
    }
}
