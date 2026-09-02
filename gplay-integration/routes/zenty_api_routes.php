<?php

use App\Http\Controllers\ZentyElastisApiController;
use Illuminate\Support\Facades\Route;

/*
|--------------------------------------------------------------------------
| CTARTech-ZentyElastis: AI Data Gateway & Telemetry Mesh Routes
| Tambahkan blok ini ke dalam routes/api.php di Herd/gplay
|--------------------------------------------------------------------------
*/

Route::prefix('v1/zenty')->group(function () {
    // Health check gateway
    Route::get('/ping', [ZentyElastisApiController::class, 'ping'])->name('api.zenty.ping');

    // Telemetry synchronization dari Rust Core Gateway
    Route::post('/telemetry/sync', [ZentyElastisApiController::class, 'syncTelemetry'])->name('api.zenty.telemetry.sync');

    // DeepOptiFlex AI predictive recommendations
    Route::get('/recommendations', [ZentyElastisApiController::class, 'getRecommendations'])->name('api.zenty.recommendations');

    // Cluster health and status for Digital Twin dashboard
    Route::get('/cluster/{cluster_id}/health', [ZentyElastisApiController::class, 'getClusterHealth'])->name('api.zenty.cluster.health');
});
