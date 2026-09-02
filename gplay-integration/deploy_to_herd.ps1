# PowerShell Script: Deploy ZentyElastis Modules to Herd/gplay
$ErrorActionPreference = "Stop"

$SourceDir = "$PSScriptRoot"
$GPlayDir = "C:\Users\UseR\Herd\gplay"

Write-Host "============================================================" -ForegroundColor Cyan
Write-Host "DEPLOYING ZENTYELASTIS INTEGRATION TO GPLAY (Herd)" -ForegroundColor Green
Write-Host "============================================================" -ForegroundColor Cyan

# 1. Salin Migration
$MigrationSource = Join-Path $SourceDir "database\migrations\2026_09_02_000001_create_zenty_elastis_tables.php"
$MigrationDest = Join-Path $GPlayDir "database\migrations"
Copy-Item -Path $MigrationSource -Destination $MigrationDest -Force
Write-Host "Migration disalin ke: $MigrationDest" -ForegroundColor Yellow

# 2. Salin Models
$ModelsSource = Join-Path $SourceDir "app\Models\*"
$ModelsDest = Join-Path $GPlayDir "app\Models"
Copy-Item -Path $ModelsSource -Destination $ModelsDest -Force
Write-Host "Models (ZentyCluster, ZentyTelemetryLog, etc.) disalin ke: $ModelsDest" -ForegroundColor Yellow

# 3. Salin Controller
$ControllerSource = Join-Path $SourceDir "app\Http\Controllers\ZentyElastisApiController.php"
$ControllerDest = Join-Path $GPlayDir "app\Http\Controllers"
Copy-Item -Path $ControllerSource -Destination $ControllerDest -Force
Write-Host "Controller ZentyElastisApiController disalin ke: $ControllerDest" -ForegroundColor Yellow

# 4. Tambahkan Routes ke routes/api.php jika belum ada
$ApiPhpPath = Join-Path $GPlayDir "routes\api.php"
$ApiContent = Get-Content $ApiPhpPath -Raw -Encoding UTF8

if ($ApiContent -notmatch "v1/zenty") {
    $RoutesFile = Join-Path $SourceDir "routes\zenty_api_routes.php"
    $RouteSnippet = "`n" + (Get-Content $RoutesFile -Raw -Encoding UTF8)
    Add-Content -Path $ApiPhpPath -Value $RouteSnippet -Encoding UTF8
    Write-Host "Routes /api/v1/zenty/* berhasil ditambahkan ke routes/api.php" -ForegroundColor Yellow
} else {
    Write-Host "Routes /api/v1/zenty/* sudah terdaftar sebelumnya." -ForegroundColor Gray
}

# 5. Gabungkan Terjemahan JSON (EN, ID, MS)
function Merge-JsonTranslation($patchFile, $targetFile) {
    if ((Test-Path $patchFile) -and (Test-Path $targetFile)) {
        $patchData = Get-Content $patchFile -Raw -Encoding UTF8 | ConvertFrom-Json
        $targetData = Get-Content $targetFile -Raw -Encoding UTF8 | ConvertFrom-Json
        
        foreach ($prop in $patchData.PSObject.Properties) {
            $targetData | Add-Member -MemberType NoteProperty -Name $prop.Name -Value $prop.Value -Force
        }
        
        $targetData | ConvertTo-Json -Depth 10 | Set-Content -Path $targetFile -Encoding UTF8
        Write-Host "Translation merged: $(Split-Path $targetFile -Leaf)" -ForegroundColor Yellow
    }
}

Merge-JsonTranslation (Join-Path $SourceDir "lang\en_zenty_patch.json") (Join-Path $GPlayDir "lang\en.json")
Merge-JsonTranslation (Join-Path $SourceDir "lang\id_zenty_patch.json") (Join-Path $GPlayDir "lang\id.json")
Merge-JsonTranslation (Join-Path $SourceDir "lang\ms_zenty_patch.json") (Join-Path $GPlayDir "lang\ms.json")

Write-Host "SEMUA FILE BERHASIL DISINKRONISASI KE C:\Users\UseR\Herd\gplay!" -ForegroundColor Green
Write-Host "Jalankan 'php artisan migrate' di C:\Users\UseR\Herd\gplay untuk membuat tabel baru." -ForegroundColor Cyan
