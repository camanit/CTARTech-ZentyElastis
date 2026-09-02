<?php

namespace App\Http\Controllers;

use App\Models\ZentyCluster;
use App\Models\ZentyTelemetryLog;
use App\Models\ZentyPatternEmbedding;
use App\Models\ZentyEnergyAuditLedger;
use Illuminate\Http\Request;
use Illuminate\Support\Facades\Validator;
use Illuminate\Support\Facades\Log;

class ZentyElastisApiController extends Controller
{
    /**
     * Ping / Health check untuk ZentyElastis AI Data Gateway
     */
    public function ping(Request $request)
    {
        $lang = $request->header('Accept-Language', 'id');
        app()->setLocale($lang);

        return response()->json([
            'status' => 'online',
            'gateway' => 'GPlay AI Data Gateway (gplay.ctar.tech)',
            'subsystem' => 'CTARTech-ZentyElastis Telemetry Mesh',
            'version' => '1.0.0',
            'timestamp' => now()->toIso8601String(),
            'message' => __('zenty.gateway_online')
        ]);
    }

    /**
     * Sinkronisasi batch telemetri dari Rust Core Gateway ke GPlay AI Bank Data
     */
    public function syncTelemetry(Request $request)
    {
        $validator = Validator::make($request->all(), [
            'cluster_id' => 'required|string',
            'metrics' => 'required|array',
            'metrics.*.device_id' => 'required|string',
            'metrics.*.wattage' => 'required|numeric',
            'metrics.*.temperature_c' => 'required|numeric',
            'metrics.*.timestamp' => 'required|integer',
        ]);

        if ($validator->fails()) {
            return response()->json([
                'success' => false,
                'error' => 'VALIDATION_ERROR',
                'messages' => $validator->errors()
            ], 422);
        }

        $clusterId = $request->input('cluster_id');
        $metrics = $request->input('metrics');

        // Pastikan cluster terdaftar atau buat entri otomatis
        $cluster = ZentyCluster::firstOrCreate(
            ['cluster_id' => $clusterId],
            [
                'client_name' => $request->input('client_name', 'Enterprise Cluster Node'),
                'tier' => $request->input('tier', 'Enterprise'),
                'hardware_type' => $request->input('hardware_type', 'NVIDIA_GPU'),
                'max_nodes' => count($metrics),
                'status' => 'ACTIVE'
            ]
        );

        $insertedLogs = [];
        $totalWattage = 0;
        $totalTemp = 0;

        foreach ($metrics as $metric) {
            $log = ZentyTelemetryLog::create([
                'cluster_id' => $clusterId,
                'device_id' => $metric['device_id'],
                'wattage' => $metric['wattage'],
                'temperature_c' => $metric['temperature_c'],
                'voltage_v' => $metric['voltage_v'] ?? null,
                'fan_speed_pct' => $metric['fan_speed_pct'] ?? null,
                'sm_clock_mhz' => $metric['sm_clock_mhz'] ?? null,
                'mem_clock_mhz' => $metric['mem_clock_mhz'] ?? null,
                'gpu_utilization_pct' => $metric['gpu_utilization_pct'] ?? null,
                'vram_used_mb' => $metric['vram_used_mb'] ?? null,
                'vram_total_mb' => $metric['vram_total_mb'] ?? null,
                'tokens_per_sec' => $metric['tokens_per_sec'] ?? null,
                'joules_per_token' => $metric['joules_per_token'] ?? null,
                'carbon_rate_gco2' => $metric['carbon_rate_gco2'] ?? null,
                'throttle_reasons' => $metric['throttle_reasons'] ?? null,
                'state_transition' => $metric['state_transition'] ?? 'ACTIVE',
                'source_timestamp' => $metric['timestamp'],
            ]);
            $insertedLogs[] = $log->id;
            $totalWattage += $metric['wattage'];
            $totalTemp += $metric['temperature_c'];
        }

        $nodeCount = max(count($metrics), 1);
        $avgWattage = $totalWattage / $nodeCount;
        $avgTemp = $totalTemp / $nodeCount;

        // Hitung batas dinamis DeepOptiFlex (Peak Shaving ~18.5%)
        $recommendedLimit = $totalWattage * 0.815;

        // Simpan / update vektor pola beban
        ZentyPatternEmbedding::create([
            'cluster_id' => $clusterId,
            'model_workload_name' => $request->input('workload_name', 'LLM_INFERENCE'),
            'avg_power_draw_watt' => $avgWattage,
            'peak_power_draw_watt' => $totalWattage,
            'recommended_peak_limit_watt' => $recommendedLimit,
            'dynamic_throttle_ratio' => 0.82,
            'predicted_spike_in_minutes' => 15
        ]);

        return response()->json([
            'success' => true,
            'synced_records' => count($insertedLogs),
            'cluster_status' => $cluster->status,
            'deepoptiflex_recommendation' => [
                'current_total_wattage' => $totalWattage,
                'recommended_peak_limit_watt' => $recommendedLimit,
                'peak_saving_pct' => 18.5,
                'action' => $totalWattage > $recommendedLimit ? 'APPLY_PEAK_SHAVING' : 'MAINTAIN_ENVELOPE',
                'message' => __('zenty.peak_shaving_recommendation', ['pct' => 18.5])
            ]
        ]);
    }

    /**
     * Mengambil rekomendasi prediktif DeepOptiFlex dari GPlay AI
     */
    public function getRecommendations(Request $request)
    {
        $clusterId = $request->query('cluster_id', 'default');
        $latestEmbedding = ZentyPatternEmbedding::where('cluster_id', $clusterId)
            ->latest()
            ->first();

        $limit = $latestEmbedding ? $latestEmbedding->recommended_peak_limit_watt : 3500.0;
        $ratio = $latestEmbedding ? $latestEmbedding->dynamic_throttle_ratio : 0.85;

        return response()->json([
            'success' => true,
            'cluster_id' => $clusterId,
            'recommended_peak_limit_watt' => $limit,
            'dynamic_throttle_ratio' => $ratio,
            'predicted_spike_in_minutes' => 15,
            'intelligence_source' => 'GPlay AI Knowledge Engine v2.4 (gplay.ctar.tech)',
            'advice' => __('zenty.deepoptiflex_advice', ['limit' => number_format($limit, 1)])
        ]);
    }

    /**
     * Mengambil status kesehatan kluster untuk dasbor Digital Twin
     */
    public function getClusterHealth(Request $request, $cluster_id)
    {
        $cluster = ZentyCluster::where('cluster_id', $cluster_id)->first();
        if (!$cluster) {
            return response()->json(['success' => false, 'message' => __('zenty.cluster_not_found')], 404);
        }

        $latestTelemetry = ZentyTelemetryLog::where('cluster_id', $cluster_id)
            ->latest('source_timestamp')
            ->limit(32)
            ->get();

        return response()->json([
            'success' => true,
            'cluster' => $cluster,
            'nodes_reporting' => $latestTelemetry->count(),
            'recent_telemetry' => $latestTelemetry
        ]);
    }
}
