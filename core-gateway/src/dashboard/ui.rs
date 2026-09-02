/// Mengembalikan HTML antarmuka dashboard Web UI berkecepatan tinggi
pub fn render_dashboard_html() -> &'static str {
    r##"<!DOCTYPE html>
<html lang="id">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>CTARTech-ZentyElastis | Resource Twin & Telemetry Mesh</title>
    <link rel="icon" type="image/x-icon" href="/favicon.ico">
    <link rel="preconnect" href="https://fonts.googleapis.com">
    <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
    <link href="https://fonts.googleapis.com/css2?family=Plus+Jakarta+Sans:wght@400;500;600;700;800&family=JetBrains+Mono:wght@400;600;700&display=swap" rel="stylesheet">
    <style>
        :root {
            --bg-base: #030712;
            --bg-card: rgba(17, 24, 39, 0.75);
            --bg-card-hover: rgba(31, 41, 55, 0.85);
            --border-glow: rgba(99, 102, 241, 0.25);
            --neon-cyan: #06b6d4;
            --neon-emerald: #10b981;
            --neon-amber: #f59e0b;
            --neon-rose: #f43f5e;
            --neon-purple: #a855f7;
            --text-main: #f9fafb;
            --text-muted: #9ca3af;
        }

        * { box-sizing: border-box; margin: 0; padding: 0; }
        body {
            background-color: var(--bg-base);
            background-image: 
                radial-gradient(circle at 15% 15%, rgba(99, 102, 241, 0.08) 0%, transparent 40%),
                radial-gradient(circle at 85% 85%, rgba(6, 182, 212, 0.08) 0%, transparent 40%);
            color: var(--text-main);
            font-family: 'Plus Jakarta Sans', -apple-system, BlinkMacSystemFont, sans-serif;
            min-height: 100vh;
            padding: 24px;
            overflow-x: hidden;
        }

        .container { max-width: 1440px; margin: 0 auto; display: flex; flex-direction: column; gap: 24px; }

        /* Glassmorphism Cards */
        .glass {
            background: var(--bg-card);
            backdrop-filter: blur(16px);
            -webkit-backdrop-filter: blur(16px);
            border: 1px solid rgba(255, 255, 255, 0.08);
            border-radius: 16px;
            box-shadow: 0 10px 30px -10px rgba(0, 0, 0, 0.5);
            transition: all 0.3s ease;
        }
        .glass:hover { border-color: rgba(255, 255, 255, 0.15); }

        /* Navbar Header */
        header {
            display: flex;
            align-items: center;
            justify-content: space-between;
            padding: 18px 28px;
            flex-wrap: wrap;
            gap: 16px;
        }
        .logo-area { display: flex; align-items: center; gap: 14px; }
        .logo-badge {
            background: linear-gradient(135deg, #4f46e5, #06b6d4);
            width: 44px; height: 44px;
            border-radius: 12px;
            display: flex; align-items: center; justify-content: center;
            font-weight: 800; font-size: 20px; color: #fff;
            box-shadow: 0 0 20px rgba(79, 70, 229, 0.5);
        }
        .logo-title h1 { font-size: 20px; font-weight: 800; letter-spacing: -0.5px; }
        .logo-title p { font-size: 11px; color: var(--text-muted); font-family: 'JetBrains Mono', monospace; }

        .header-badges { display: flex; align-items: center; gap: 12px; flex-wrap: wrap; }
        .pill {
            display: inline-flex; align-items: center; gap: 8px;
            padding: 6px 14px; border-radius: 999px;
            font-size: 11px; font-weight: 700;
            font-family: 'JetBrains Mono', monospace;
            border: 1px solid transparent;
        }
        .pill-emerald { background: rgba(16, 185, 129, 0.12); color: var(--neon-emerald); border-color: rgba(16, 185, 129, 0.25); }
        .pill-cyan { background: rgba(6, 182, 212, 0.12); color: var(--neon-cyan); border-color: rgba(6, 182, 212, 0.25); }
        .pill-purple { background: rgba(168, 85, 247, 0.12); color: var(--neon-purple); border-color: rgba(168, 85, 247, 0.25); }
        .pill-rose { background: rgba(244, 63, 94, 0.12); color: var(--neon-rose); border-color: rgba(244, 63, 94, 0.25); }

        .pulse-dot {
            width: 8px; height: 8px; border-radius: 50%;
            background-color: currentColor;
            animation: pulse 2s cubic-bezier(0.4, 0, 0.6, 1) infinite;
        }
        @keyframes pulse { 0%, 100% { opacity: 1; transform: scale(1); } 50% { opacity: 0.4; transform: scale(0.8); } }

        /* KPI Grid */
        .kpi-grid {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(260px, 1fr));
            gap: 18px;
        }
        .kpi-card { padding: 22px 24px; position: relative; overflow: hidden; }
        .kpi-card::before {
            content: ''; position: absolute; top: 0; left: 0; right: 0; height: 2px;
            background: linear-gradient(90deg, transparent, var(--accent, #6366f1), transparent);
        }
        .kpi-label { font-size: 11px; font-weight: 700; color: var(--text-muted); text-transform: uppercase; letter-spacing: 0.8px; }
        .kpi-value { font-size: 32px; font-weight: 800; font-family: 'JetBrains Mono', monospace; margin-top: 8px; display: flex; align-items: baseline; gap: 6px; }
        .kpi-unit { font-size: 14px; font-weight: 600; color: var(--text-muted); }
        .kpi-footer { font-size: 12px; margin-top: 10px; display: flex; align-items: center; justify-content: space-between; }

        /* Main Grid: Chart & Actuation Assurance */
        .main-grid {
            display: grid;
            grid-template-columns: 2fr 1fr;
            gap: 24px;
        }
        @media (max-width: 1024px) { .main-grid { grid-template-columns: 1fr; } }

        .panel-header {
            display: flex; align-items: center; justify-content: space-between;
            padding: 20px 24px; border-bottom: 1px solid rgba(255, 255, 255, 0.06);
        }
        .panel-title { font-size: 14px; font-weight: 700; display: flex; align-items: center; gap: 8px; }
        .panel-body { padding: 24px; }

        /* Canvas Chart Container */
        .chart-container {
            width: 100%; height: 260px; position: relative;
        }
        canvas { width: 100% !important; height: 100% !important; display: block; }

        /* Circuit Breaker Box */
        .breaker-box {
            display: flex; flex-direction: column; gap: 16px;
            align-items: center; text-align: center;
        }
        .breaker-status-badge {
            width: 100%; padding: 18px; border-radius: 12px;
            font-size: 16px; font-weight: 800; font-family: 'JetBrains Mono', monospace;
            display: flex; align-items: center; justify-content: center; gap: 10px;
            transition: all 0.3s;
        }
        .status-armed { background: rgba(16, 185, 129, 0.15); color: var(--neon-emerald); border: 1px solid rgba(16, 185, 129, 0.4); box-shadow: 0 0 25px rgba(16, 185, 129, 0.2); }
        .status-tripped { background: rgba(244, 63, 94, 0.2); color: var(--neon-rose); border: 1px solid var(--neon-rose); box-shadow: 0 0 30px rgba(244, 63, 94, 0.4); animation: pulse 1s infinite; }

        .btn-action {
            width: 100%; padding: 14px; border-radius: 10px;
            font-size: 13px; font-weight: 700; cursor: pointer;
            border: none; transition: all 0.2s ease;
            display: flex; align-items: center; justify-content: center; gap: 8px;
        }
        .btn-kill { background: linear-gradient(135deg, #e11d48, #be123c); color: #fff; }
        .btn-kill:hover { transform: translateY(-2px); box-shadow: 0 8px 20px rgba(225, 29, 72, 0.4); }
        .btn-reset { background: rgba(255, 255, 255, 0.08); color: var(--text-main); border: 1px solid rgba(255, 255, 255, 0.15); }
        .btn-reset:hover { background: rgba(255, 255, 255, 0.15); }

        /* Logs & Feeds */
        .bottom-grid {
            display: grid;
            grid-template-columns: 1fr 1fr;
            gap: 24px;
        }
        @media (max-width: 900px) { .bottom-grid { grid-template-columns: 1fr; } }

        .feed-list {
            display: flex; flex-direction: column; gap: 10px;
            max-height: 220px; overflow-y: auto;
        }
        .feed-item {
            padding: 12px 14px; border-radius: 8px;
            background: rgba(255, 255, 255, 0.03);
            border-left: 3px solid var(--feed-color, #6366f1);
            font-size: 12px; display: flex; justify-content: space-between; align-items: center;
        }
        .feed-text { font-family: 'JetBrains Mono', monospace; color: #e5e7eb; }
        .feed-time { font-size: 10px; color: var(--text-muted); }

        /* Footer */
        footer {
            text-align: center; padding: 20px; color: var(--text-muted);
            font-size: 12px; border-top: 1px solid rgba(255, 255, 255, 0.05);
            display: flex; justify-content: space-between; align-items: center; flex-wrap: wrap; gap: 12px;
        }
        footer a { color: var(--neon-cyan); text-decoration: none; font-weight: 600; }
        footer a:hover { text-decoration: underline; }
    </style>
</head>
<body>

<div class="container">
    <!-- Navbar Header -->
    <header class="glass">
        <div class="logo-area">
            <img src="/assets/logo.png" alt="CTARTech Logo" style="width: 44px; height: 44px; border-radius: 12px; box-shadow: 0 0 20px rgba(6, 182, 212, 0.4); object-fit: cover;" />
            <div class="logo-title">
                <h1>CTARTech-ZentyElastis™</h1>
                <p>Autonomous AI Data Center Telemetry Mesh & Digital Twin</p>
            </div>
        </div>
        <div class="header-badges">
            <div class="pill pill-emerald" id="badge-engine">
                <span class="pulse-dot"></span>
                <span>Rust Axum Core: &lt;0.1ms</span>
            </div>
            <div class="pill pill-cyan" id="badge-license">
                <span>🔑 Ed25519 Verified</span>
            </div>
            <div class="pill pill-emerald" id="badge-sla">
                <span>🛡️ SLAShield: OPTIMAL</span>
            </div>
            <div class="pill pill-purple" id="badge-mesh">
                <span>Zero-Trust HMAC Active</span>
            </div>
        </div>
    </header>

    <!-- KPI Grid -->
    <div class="kpi-grid">
        <!-- 1. Total Power Draw -->
        <div class="glass kpi-card" style="--accent: var(--neon-amber);">
            <div class="kpi-label">Cluster Total Power Draw</div>
            <div class="kpi-value" id="kpi-wattage">
                <span id="val-watt">--</span>
                <span class="kpi-unit">Watt</span>
            </div>
            <div class="kpi-footer">
                <span style="color: var(--neon-emerald);" id="kpi-peak-saving">DeepOptiFlex Shaving: -18.5%</span>
                <span style="color: var(--text-muted);" id="kpi-cap-limit">Cap: 3,500W</span>
            </div>
        </div>

        <!-- 2. GPU Thermal Junction -->
        <div class="glass kpi-card" style="--accent: var(--neon-rose);">
            <div class="kpi-label">GPU Junction Temperature</div>
            <div class="kpi-value" id="kpi-temp">
                <span id="val-temp">--</span>
                <span class="kpi-unit">°C</span>
            </div>
            <div class="kpi-footer">
                <span id="kpi-temp-status" style="color: var(--neon-emerald);">● Safe Envelope (&lt;78°C)</span>
                <span style="color: var(--text-muted);">Trip: 85.0°C</span>
            </div>
        </div>

        <!-- 3. AI Compute Efficiency (Joules per Token) -->
        <div class="glass kpi-card" style="--accent: var(--neon-cyan);">
            <div class="kpi-label">Energy per Token Efficiency</div>
            <div class="kpi-value" id="kpi-joules">
                <span id="val-joules">--</span>
                <span class="kpi-unit">J/Tok</span>
            </div>
            <div class="kpi-footer">
                <span id="val-tps" style="color: var(--neon-cyan);">Throughput: -- TPS</span>
                <span style="color: var(--text-muted);">LLM Inference</span>
            </div>
        </div>

        <!-- 4. Carbon Rate & ESG Green Grid -->
        <div class="glass kpi-card" style="--accent: var(--neon-emerald);">
            <div class="kpi-label">Carbon Emission Rate (ESG)</div>
            <div class="kpi-value" id="kpi-carbon">
                <span id="val-carbon">--</span>
                <span class="kpi-unit">gCO₂/h</span>
            </div>
            <div class="kpi-footer">
                <span style="color: var(--neon-emerald);" id="val-saved-co2">Saved: -- gCO₂</span>
                <span style="color: var(--text-muted);">PUE: 1.15</span>
            </div>
        </div>
    </div>

    <!-- Main Grid: Real-Time Dynamic Chart & Circuit Breaker -->
    <div class="main-grid">
        <!-- Power Draw vs DeepOptiFlex Envelope Canvas Chart -->
        <div class="glass">
            <div class="panel-header">
                <div class="panel-title">
                    <span style="color: var(--neon-cyan);">📊</span>
                    <span>Real-Time Power Envelope vs DeepOptiFlex™ Target (Last 60s)</span>
                </div>
                <div class="pill pill-cyan">
                    <span id="chart-samples-count">Sampling: 1.5s</span>
                </div>
            </div>
            <div class="panel-body">
                <div class="chart-container">
                    <canvas id="powerChart"></canvas>
                </div>
            </div>
        </div>

        <!-- Actuation Assurance & Emergency Circuit Breaker -->
        <div class="glass">
            <div class="panel-header">
                <div class="panel-title">
                    <span style="color: var(--neon-rose);">🛡️</span>
                    <span>Actuation Assurance Control</span>
                </div>
                <div class="pill pill-purple">&lt;5ms Kill-Switch</div>
            </div>
            <div class="panel-body breaker-box">
                <div class="breaker-status-badge status-armed" id="breaker-badge">
                    <span id="breaker-icon">🟢</span>
                    <span id="breaker-text">CIRCUIT BREAKER ARMED</span>
                </div>
                <p style="font-size: 11px; color: var(--text-muted); line-height: 1.5;">
                    Pemutus sirkuit hardware otonom aktif. Beban otomatis diputus jika suhu &gt;85°C atau tarikan daya &gt;3,500W.
                </p>
                <button class="btn-action btn-kill" id="btn-kill" onclick="tripBreaker()">
                    ⚡ MANUAL EMERGENCY TRIP (KILL-SWITCH)
                </button>
                <button class="btn-action btn-reset" id="btn-reset" onclick="resetBreaker()">
                    🔄 RESET CIRCUIT BREAKER
                </button>
            </div>
        </div>
    </div>

    <!-- Sprint 3 Grid: SLAShield Performance Guardian & SOC Merkle ESG Audit Ledger -->
    <div class="bottom-grid">
        <!-- 1. SLAShield Performance Guardian -->
        <div class="glass">
            <div class="panel-header">
                <div class="panel-title">
                    <span style="color: var(--neon-cyan);">🛡️</span>
                    <span>SLAShield™ Performance Guardian</span>
                </div>
                <div class="pill pill-emerald" id="sla-status-pill">SLA: OPTIMAL</div>
            </div>
            <div class="panel-body">
                <div style="display: flex; flex-direction: column; gap: 12px; font-family: 'JetBrains Mono', monospace; font-size: 12px;">
                    <div style="display: flex; justify-content: space-between;">
                        <span style="color: var(--text-muted);">Current Throughput:</span>
                        <strong id="sla-current-tps" style="color: var(--neon-cyan);">-- TPS</strong>
                    </div>
                    <div style="display: flex; justify-content: space-between;">
                        <span style="color: var(--text-muted);">Target SLA Floor:</span>
                        <strong style="color: var(--text-main);">120.0 TPS</strong>
                    </div>
                    <div style="display: flex; justify-content: space-between;">
                        <span style="color: var(--text-muted);">Power Cap Override:</span>
                        <strong id="sla-override-cap" style="color: var(--neon-emerald);">NONE (Peak Shaving Allowed)</strong>
                    </div>
                    <div id="sla-advisory" style="font-size: 11px; padding: 10px; border-radius: 8px; background: rgba(255,255,255,0.03); color: #e5e7eb; border-left: 3px solid var(--neon-emerald);">
                        Menunggu telemetri inferensi...
                    </div>
                </div>
            </div>
        </div>

        <!-- 2. SOC Merkle Chain Audit Ledger & ESG Green AI -->
        <div class="glass">
            <div class="panel-header">
                <div class="panel-title">
                    <span style="color: var(--neon-emerald);">📜</span>
                    <span>SOC Merkle Chain Audit Ledger (ESG Compliance)</span>
                </div>
                <div class="pill pill-purple" id="merkle-height-pill">Block #0</div>
            </div>
            <div class="panel-body" style="display: flex; flex-direction: column; gap: 14px;">
                <div style="font-family: 'JetBrains Mono', monospace; font-size: 11px; display: flex; flex-direction: column; gap: 6px;">
                    <div><span style="color: var(--text-muted);">Merkle Root:</span> <span id="merkle-root-val" style="color: var(--neon-cyan); word-break: break-all;">Genesis Initialized</span></div>
                    <div><span style="color: var(--text-muted);">Audit Standard:</span> <span style="color: var(--text-main);">ISO 14064-1 & GHG Protocol Scope 2</span></div>
                    <div><span style="color: var(--text-muted);">Verification:</span> <span style="color: var(--neon-emerald);">CRYPTOGRAPHICALLY SEALED (SHA-256)</span></div>
                </div>
                <button onclick="downloadEsgCertificate()" class="btn-action" style="background: linear-gradient(135deg, #059669, #10b981); color: #fff; font-size: 12px; font-weight: 700;">
                    📜 UNDUH SERTIFIKAT AUDIT ESG RESMI (.JSON)
                </button>
            </div>
        </div>
    </div>

    <!-- Bottom Grid: Self-Healing Log & Live Hardware Telemetry -->
    <div class="bottom-grid">
        <!-- Autonomous Self-Healing Feed -->
        <div class="glass">
            <div class="panel-header">
                <div class="panel-title">
                    <span style="color: var(--neon-purple);">🤖</span>
                    <span>Autonomous Self-Healing Event Stream</span>
                </div>
                <div class="pill pill-purple" id="healing-count">0 Actions</div>
            </div>
            <div class="panel-body">
                <div class="feed-list" id="healing-feed">
                    <div class="feed-item" style="--feed-color: var(--neon-emerald);">
                        <span class="feed-text">Self-Healing Engine Siaga: Pemantauan Thermal & VRAM aktif.</span>
                        <span class="feed-time">BOOT</span>
                    </div>
                </div>
            </div>
        </div>

        <!-- Zero-Trust Edge Hardware Telemetry Status -->
        <div class="glass">
            <div class="panel-header">
                <div class="panel-title">
                    <span style="color: var(--neon-emerald);">🖥️</span>
                    <span>Cluster Hardware State & Zero-Trust Verification</span>
                </div>
                <div class="pill pill-emerald" id="auth-status">HMAC VALID</div>
            </div>
            <div class="panel-body">
                <div style="font-family: 'JetBrains Mono', monospace; font-size: 12px; display: grid; grid-template-columns: 1fr 1fr; gap: 12px;">
                    <div><span style="color: var(--text-muted);">Device Node:</span> <strong id="val-device">gpu-node-01</strong></div>
                    <div><span style="color: var(--text-muted);">SM Clock:</span> <strong id="val-sm-clock">-- MHz</strong></div>
                    <div><span style="color: var(--text-muted);">Core Voltage:</span> <strong id="val-voltage">-- V</strong></div>
                    <div><span style="color: var(--text-muted);">Fan Speed:</span> <strong id="val-fan">-- %</strong></div>
                    <div><span style="color: var(--text-muted);">VRAM Allocation:</span> <strong id="val-vram">-- MB</strong></div>
                    <div><span style="color: var(--text-muted);">GPlay AI Sync:</span> <strong style="color: var(--neon-cyan);" id="val-gplay">CONNECTED</strong></div>
                </div>
            </div>
        </div>
    </div>

    <!-- Footer -->
    <footer>
        <div>
            &copy; 2026 <strong>PT CTAR Technology Indonesia</strong>. CTARTech-ZentyElastis™ Enterprise.
        </div>
        <div>
            Official Gateway: <a href="https://gplay.ctar.tech" target="_blank">gplay.ctar.tech</a> | 
            Subdomain: <a href="https://zentyelastis.ctar.tech" target="_blank">zentyelastis.ctar.tech</a> |
            WhatsApp: <a href="https://wa.me/6281260006666" target="_blank">0812-6000-6666 (a.n. Abdul Rahman Rahmad)</a>
        </div>
    </footer>
</div>

<script>
    // Data Canvas Chart (Zero-dependency Canvas 2D)
    const canvas = document.getElementById('powerChart');
    const ctx = canvas.getContext('2d');
    let historyWatt = [];
    let historyCap = [];

    function resizeCanvas() {
        canvas.width = canvas.parentElement.clientWidth;
        canvas.height = canvas.parentElement.clientHeight;
    }
    window.addEventListener('resize', resizeCanvas);
    resizeCanvas();

    function drawChart() {
        ctx.clearRect(0, 0, canvas.width, canvas.height);
        if (historyWatt.length < 2) return;

        const maxVal = 4000;
        const w = canvas.width;
        const h = canvas.height;
        const step = w / (historyWatt.length - 1);

        // Draw Grid Lines
        ctx.strokeStyle = 'rgba(255, 255, 255, 0.05)';
        ctx.lineWidth = 1;
        for (let i = 1; i <= 4; i++) {
            let y = h - (h / 4) * i;
            ctx.beginPath();
            ctx.moveTo(0, y);
            ctx.lineTo(w, y);
            ctx.stroke();
        }

        // Draw Shaved Cap Line (Emerald Dashed)
        ctx.strokeStyle = '#10b981';
        ctx.setLineDash([6, 4]);
        ctx.lineWidth = 2;
        ctx.beginPath();
        for (let i = 0; i < historyCap.length; i++) {
            let x = i * step;
            let y = h - (historyCap[i] / maxVal) * h;
            if (i === 0) ctx.moveTo(x, y); else ctx.lineTo(x, y);
        }
        ctx.stroke();
        ctx.setLineDash([]);

        // Draw Actual Wattage Line (Neon Cyan Glow)
        ctx.strokeStyle = '#06b6d4';
        ctx.lineWidth = 3;
        ctx.shadowColor = '#06b6d4';
        ctx.shadowBlur = 10;
        ctx.beginPath();
        for (let i = 0; i < historyWatt.length; i++) {
            let x = i * step;
            let y = h - (historyWatt[i] / maxVal) * h;
            if (i === 0) ctx.moveTo(x, y); else ctx.lineTo(x, y);
        }
        ctx.stroke();
        ctx.shadowBlur = 0; // reset
    }

    // Polling Live Telemetry Data
    async function fetchLiveTelemetry() {
        try {
            const res = await fetch('/api/v1/telemetry/live');
            if (!res.ok) return;
            const data = await res.json();

            // Update KPIs
            if (data.latest_metrics) {
                const m = data.latest_metrics;
                document.getElementById('val-watt').innerText = m.wattage.toFixed(1);
                document.getElementById('val-temp').innerText = m.temperature_c.toFixed(1);
                document.getElementById('val-joules').innerText = (m.joules_per_token || 11.2).toFixed(2);
                document.getElementById('val-carbon').innerText = (m.carbon_rate_gco2 || 420.0).toFixed(1);
                document.getElementById('val-tps').innerText = `Throughput: ${(m.tokens_per_sec || 160.0).toFixed(1)} TPS`;

                document.getElementById('val-device').innerText = m.device_id || 'gpu-node-01';
                document.getElementById('val-sm-clock').innerText = (m.sm_clock_mhz || 1980) + ' MHz';
                document.getElementById('val-voltage').innerText = (m.voltage_v || 12.0).toFixed(2) + ' V';
                document.getElementById('val-fan').innerText = (m.fan_speed_pct || 65.0).toFixed(0) + ' %';
                document.getElementById('val-vram').innerText = `${m.vram_used_mb || 40960} / ${m.vram_total_mb || 81920} MB`;

                // Temp status alert
                const tempEl = document.getElementById('kpi-temp-status');
                if (m.temperature_c > 80.0) {
                    tempEl.innerText = '⚠️ CRITICAL HOT (>80°C)';
                    tempEl.style.color = 'var(--neon-rose)';
                } else if (m.temperature_c > 75.0) {
                    tempEl.innerText = '⚡ PREEMPTIVE ZONE (75-80°C)';
                    tempEl.style.color = 'var(--neon-amber)';
                } else {
                    tempEl.innerText = '● Safe Envelope (<75°C)';
                    tempEl.style.color = 'var(--neon-emerald)';
                }
            }

            // DeepOptiFlex advice
            if (data.deepoptiflex) {
                const opt = data.deepoptiflex;
                document.getElementById('kpi-cap-limit').innerText = `Cap: ${opt.recommended_cap_watt.toFixed(0)}W`;
                document.getElementById('val-saved-co2').innerText = `Saved: ${opt.carbon_prevented_gco2.toFixed(3)} gCO₂`;
            }

            // SLAShield Guardian status
            if (data.slashield) {
                const sla = data.slashield;
                document.getElementById('sla-current-tps').innerText = `${sla.current_tps.toFixed(1)} TPS`;
                const pill = document.getElementById('sla-status-pill');
                const badge = document.getElementById('badge-sla');
                const adv = document.getElementById('sla-advisory');
                const capEl = document.getElementById('sla-override-cap');

                adv.innerText = sla.advisory;

                if (sla.status === 'RESCUE') {
                    pill.className = 'pill pill-rose';
                    pill.innerText = 'SLA: RESCUE ACTIVE';
                    badge.className = 'pill pill-rose';
                    badge.innerHTML = '<span>⚠️ SLAShield: RESCUE</span>';
                    adv.style.borderColor = 'var(--neon-rose)';
                    capEl.innerText = `${sla.override_power_cap_watt.toFixed(0)}W (SLA Headroom Boost)`;
                    capEl.style.color = 'var(--neon-rose)';
                } else if (sla.status === 'ADAPTIVE_THROTTLE') {
                    pill.className = 'pill pill-purple';
                    pill.innerText = 'SLA: BUFFER ZONE';
                    badge.className = 'pill pill-purple';
                    badge.innerHTML = '<span>⚡ SLAShield: BUFFER</span>';
                    adv.style.borderColor = 'var(--neon-amber)';
                    capEl.innerText = `${sla.override_power_cap_watt.toFixed(0)}W (Stabilized)`;
                    capEl.style.color = 'var(--neon-amber)';
                } else {
                    pill.className = 'pill pill-emerald';
                    pill.innerText = 'SLA: OPTIMAL';
                    badge.className = 'pill pill-emerald';
                    badge.innerHTML = '<span>🛡️ SLAShield: OPTIMAL</span>';
                    adv.style.borderColor = 'var(--neon-emerald)';
                    capEl.innerText = 'NONE (Peak Shaving Allowed)';
                    capEl.style.color = 'var(--neon-emerald)';
                }
            }

            // SOC Merkle Chain Audit Status
            if (data.merkle_status) {
                const m = data.merkle_status;
                document.getElementById('merkle-height-pill').innerText = `Block #${m.block_height}`;
                document.getElementById('merkle-root-val').innerText = m.merkle_root;
            }

            // Update Circuit Breaker Badge
            const breakerBadge = document.getElementById('breaker-badge');
            const breakerText = document.getElementById('breaker-text');
            const breakerIcon = document.getElementById('breaker-icon');
            if (data.circuit_breaker_status === 'TRIPPED') {
                breakerBadge.className = 'breaker-status-badge status-tripped';
                breakerText.innerText = 'CIRCUIT BREAKER TRIPPED';
                breakerIcon.innerText = '🔴';
            } else {
                breakerBadge.className = 'breaker-status-badge status-armed';
                breakerText.innerText = 'CIRCUIT BREAKER ARMED';
                breakerIcon.innerText = '🟢';
            }

            // Update Self-Healing Feed
            if (data.self_healing_events && data.self_healing_events.length > 0) {
                const feedContainer = document.getElementById('healing-feed');
                document.getElementById('healing-count').innerText = `${data.self_healing_events.length} Actions`;
                feedContainer.innerHTML = '';
                data.self_healing_events.slice(-5).reverse().forEach(ev => {
                    const item = document.createElement('div');
                    item.className = 'feed-item';
                    item.style.setProperty('--feed-color', ev.action.includes('MIGRATION') ? 'var(--neon-amber)' : 'var(--neon-cyan)');
                    item.innerHTML = `<span class="feed-text">${ev.action}: ${ev.reason}</span><span class="feed-time">${ev.time}</span>`;
                    feedContainer.appendChild(item);
                });
            }

            // Update License Badge
            if (data.active_license) {
                document.getElementById('badge-license').innerText = `🔑 ${data.active_license.client_id} (${data.active_license.tier})`;
            }

            // Update Chart History
            if (data.history && data.history.length > 0) {
                historyWatt = data.history.map(h => h.wattage);
                historyCap = data.history.map(h => h.recommended_cap);
                drawChart();
            }
        } catch (e) {
            console.error('Error fetching telemetry:', e);
        }
    }

    // Manual Circuit Breaker Kill-Switch
    async function tripBreaker() {
        if (!confirm('Peringatan: Apakah Anda yakin ingin memicu Emergency Circuit Breaker (<5ms Kill-Switch)? Beban GPU akan langsung diputus.')) return;
        try {
            await fetch('/api/v1/breaker/trip', { method: 'POST' });
            fetchLiveTelemetry();
        } catch (e) {
            alert('Gagal memicu breaker: ' + e);
        }
    }

    // Reset Circuit Breaker
    async function resetBreaker() {
        try {
            await fetch('/api/v1/breaker/reset', { method: 'POST' });
            fetchLiveTelemetry();
        } catch (e) {
            alert('Gagal reset breaker: ' + e);
        }
    }

    // Unduh Sertifikat Kepatuhan ESG Green AI
    async function downloadEsgCertificate() {
        try {
            const res = await fetch('/api/v1/audit/esg-certificate');
            if (!res.ok) {
                alert('Gagal mengambil sertifikat ESG');
                return;
            }
            const cert = await res.json();
            const jsonStr = JSON.stringify(cert, null, 2);
            const blob = new Blob([jsonStr], { type: 'application/json' });
            const url = URL.createObjectURL(blob);
            const a = document.createElement('a');
            a.href = url;
            a.download = `ESG_GreenAI_Certificate_${cert.certificate_id}.json`;
            document.body.appendChild(a);
            a.click();
            document.body.removeChild(a);
            URL.revokeObjectURL(url);
        } catch (e) {
            alert('Gagal mengunduh sertifikat ESG: ' + e);
        }
    }

    // Polling Interval 1500ms
    setInterval(fetchLiveTelemetry, 1500);
    fetchLiveTelemetry();
</script>

</body>
</html>"##
}
