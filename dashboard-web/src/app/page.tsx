"use client";

import React, { useState, useEffect } from "react";
import {
  Zap,
  Shield,
  Activity,
  Cpu,
  Lock,
  Leaf,
  Layers,
  ArrowRight,
  Download,
  AlertTriangle,
  CheckCircle2,
  ExternalLink,
  MessageSquare,
  CreditCard,
  Building,
  Terminal,
  RefreshCw,
  Power
} from "lucide-react";

export default function HomePage() {
  // Simulator State untuk interaktivitas pengguna di Landing Page
  const [powerLoad, setPowerLoad] = useState(1920);
  const [isBreakerTripped, setIsBreakerTripped] = useState(false);
  const [tps, setTps] = useState(168.5);

  // Kalkulasi DeepOptiFlex & SLAShield dinamis
  const powerCap = Math.round(powerLoad * 0.815); // -18.5%
  const temp = Math.min(88, Math.round(45 + (powerLoad / 3500) * 42));
  const joulesPerToken = (powerLoad / tps).toFixed(2);
  const carbonRate = ((powerLoad * 1.15 * 0.432) / 2.0).toFixed(1);

  // Status SLAShield
  let slaStatus = "OPTIMAL";
  let slaColor = "text-emerald-400 bg-emerald-950/60 border-emerald-500/30";
  if (tps < 120) {
    slaStatus = "RESCUE ACTIVE";
    slaColor = "text-rose-400 bg-rose-950/60 border-rose-500/30";
  } else if (tps < 140) {
    slaStatus = "BUFFER ZONE";
    slaColor = "text-amber-400 bg-amber-950/60 border-amber-500/30";
  }

  // Fungsi unduh sertifikat ESG demo
  const handleDownloadEsg = () => {
    const cert = {
      certificate_id: `ESG-ZENTY-BLOCK-12-${Date.now().toString().slice(-6)}`,
      issued_to: "PT CTAR Technology Indonesia — Enterprise AI Client",
      authority: "PT CTAR Technology Indonesia (Sovereign ESG Registry)",
      merkle_root: "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
      compliance_standard: "ISO 14064-1 & GHG Protocol Scope 2 AI Optimization",
      cumulative_energy_saved_kwh: ((powerLoad - powerCap) * 24 * 30 / 1000).toFixed(2),
      cumulative_carbon_prevented_kg_co2: (((powerLoad - powerCap) * 24 * 30 * 0.432) / 1000).toFixed(2),
      verification_status: "CRYPTOGRAPHICALLY_VERIFIED",
      verified_at: new Date().toISOString()
    };
    const blob = new Blob([JSON.stringify(cert, null, 2)], { type: "application/json" });
    const url = URL.createObjectURL(blob);
    const a = document.createElement("a");
    a.href = url;
    a.download = `ESG_Compliance_Certificate_${cert.certificate_id}.json`;
    a.click();
    URL.revokeObjectURL(url);
  };

  return (
    <div className="min-h-screen text-slate-100 selection:bg-cyan-500 selection:text-black">
      {/* 1. TOP ANNOUNCEMENT BAR */}
      <div className="border-b border-cyan-500/20 bg-cyan-950/40 px-4 py-2 text-center text-xs text-cyan-300 backdrop-blur-md">
        <span>🚀 <strong>Rilis Resmi V0.1 Aktif:</strong> Sub-Millisecond Rust Axum Core & DeepOptiFlex™ Predictive Peak Shaving.</span>
        <a href="#demo" className="ml-2 font-semibold underline hover:text-white">Coba Demo Interaktif &rarr;</a>
      </div>

      {/* 2. NAVBAR */}
      <nav className="sticky top-0 z-50 border-b border-white/10 bg-slate-950/80 backdrop-blur-xl">
        <div className="mx-auto flex max-w-7xl items-center justify-between px-6 py-4">
          <div className="flex items-center gap-3">
            <img src="/logo.png" alt="CTARTech Logo" className="h-10 w-10 rounded-xl object-cover shadow-neon" />
            <div>
              <span className="text-lg font-bold tracking-tight text-white">
                CTARTech-<span className="gradient-text">ZentyElastis™</span>
              </span>
              <p className="text-[10px] uppercase tracking-wider text-slate-400">Autonomous AI DC Telemetry Mesh</p>
            </div>
          </div>

          <div className="hidden md:flex items-center gap-6 text-sm text-slate-300">
            <a href="#arsitektur" className="hover:text-cyan-400 transition-colors">Arsitektur</a>
            <a href="#deepoptiflex" className="hover:text-cyan-400 transition-colors">DeepOptiFlex™</a>
            <a href="#slashield" className="hover:text-cyan-400 transition-colors">SLAShield™</a>
            <a href="#merkle-esg" className="hover:text-cyan-400 transition-colors">Audit ESG</a>
            <a href="#pricing" className="hover:text-cyan-400 transition-colors">Lisensi</a>
            <a href="https://gplay.ctar.tech" target="_blank" className="flex items-center gap-1 text-purple-400 hover:text-purple-300 transition-colors">
              <span>GPlay AI</span>
              <ExternalLink className="h-3 w-3" />
            </a>
          </div>

          <div className="flex items-center gap-3">
            <a
              href="https://wa.me/6281260006666?text=Halo%20CTARTech,%20saya%20tertarik%20dengan%20solusi%20Enterprise%20ZentyElastis"
              target="_blank"
              className="flex items-center gap-2 rounded-xl bg-gradient-to-r from-cyan-500 to-emerald-500 px-4 py-2 text-xs font-semibold text-black shadow-neon transition-all hover:scale-105"
            >
              <MessageSquare className="h-4 w-4" />
              <span>Hubungi Enterprise</span>
            </a>
          </div>
        </div>
      </nav>

      {/* 3. HERO SECTION */}
      <section className="relative overflow-hidden px-6 pt-20 pb-16">
        <div className="mx-auto max-w-5xl text-center">
          <div className="mb-6 inline-flex items-center gap-2 rounded-full border border-cyan-500/30 bg-cyan-950/50 px-4 py-1.5 text-xs text-cyan-300 shadow-neon">
            <Zap className="h-3.5 w-3.5 animate-pulse text-cyan-400" />
            <span>Rust Axum Core Latency: &lt;0.1ms | Zero-Trust Hardware Harness</span>
          </div>

          <h1 className="text-4xl font-extrabold tracking-tight sm:text-6xl sm:leading-tight">
            Optimasi Daya & Pelindung Latensi{" "}
            <span className="gradient-text">Pusat Data AI Generasi Baru</span>
          </h1>

          <p className="mx-auto mt-6 max-w-3xl text-lg text-slate-300 leading-relaxed">
            Platform telemetri otonom dan digital twin pertama untuk kluster GPU AI skala besar (NVIDIA H100 / A100 / Blackwell). 
            Dilengkapi mesin prediktif <strong>DeepOptiFlex™</strong> untuk pemotongan lonjakan daya listrik hingga <strong>-18.5%</strong>, 
            penjaga SLA inferensi <strong>SLAShield™</strong>, serta sertifikasi kepatuhan karbon <strong>SOC Merkle Chain Audit Ledger</strong>.
          </p>

          <div className="mt-10 flex flex-wrap items-center justify-center gap-4">
            <a
              href="#demo"
              className="flex items-center gap-2 rounded-xl bg-cyan-500 px-6 py-3.5 text-sm font-bold text-black shadow-neon transition-all hover:bg-cyan-400 hover:scale-105"
            >
              <Activity className="h-4 w-4" />
              <span>Buka Live Telemetry Simulator</span>
            </a>
            <a
              href="#pricing"
              className="flex items-center gap-2 rounded-xl border border-white/20 bg-white/5 px-6 py-3.5 text-sm font-semibold text-white backdrop-blur-md transition-all hover:bg-white/10 hover:border-cyan-400"
            >
              <Shield className="h-4 w-4 text-emerald-400" />
              <span>Tinjau Skema Lisensi Enterprise</span>
            </a>
          </div>

          {/* Quick Badges */}
          <div className="mt-12 flex flex-wrap items-center justify-center gap-6 text-xs text-slate-400">
            <div className="flex items-center gap-1.5">
              <CheckCircle2 className="h-4 w-4 text-emerald-400" />
              <span>Zero-Trust HMAC-SHA256</span>
            </div>
            <div className="flex items-center gap-1.5">
              <CheckCircle2 className="h-4 w-4 text-emerald-400" />
              <span>Offline Air-Gapped Ed25519</span>
            </div>
            <div className="flex items-center gap-1.5">
              <CheckCircle2 className="h-4 w-4 text-emerald-400" />
              <span>ISO 14064-1 & GHG Protocol</span>
            </div>
            <div className="flex items-center gap-1.5">
              <CheckCircle2 className="h-4 w-4 text-emerald-400" />
              <span>&lt;5ms Emergency Hardware Kill-Switch</span>
            </div>
          </div>
        </div>
      </section>

      {/* 4. INTERACTIVE LIVE SIMULATOR (DEMO) */}
      <section id="demo" className="px-6 py-12">
        <div className="mx-auto max-w-6xl">
          <div className="glass rounded-3xl p-6 sm:p-8 border border-cyan-500/30 shadow-2xl relative overflow-hidden">
            <div className="flex flex-wrap items-center justify-between gap-4 border-b border-white/10 pb-6">
              <div className="flex items-center gap-3">
                <div className="h-3 w-3 rounded-full bg-emerald-400 animate-ping" />
                <h3 className="text-xl font-bold text-white">Live Resource Twin Simulator (Edge-to-Core)</h3>
              </div>
              <div className="flex items-center gap-2">
                <span className={`inline-flex items-center gap-1.5 rounded-full border px-3 py-1 text-xs font-mono font-semibold ${slaColor}`}>
                  🛡️ SLAShield: {slaStatus}
                </span>
                <span className="rounded-full border border-purple-500/30 bg-purple-950/60 px-3 py-1 text-xs font-mono text-purple-300">
                  Zero-Trust HMAC Active
                </span>
              </div>
            </div>

            {/* 4 Metric Cards */}
            <div className="mt-8 grid grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-4">
              {/* Power */}
              <div className="glass-card rounded-2xl p-5">
                <span className="text-xs uppercase text-slate-400">Cluster Total Power</span>
                <div className="mt-2 flex items-baseline gap-2">
                  <span className="text-3xl font-mono font-extrabold text-amber-400">{isBreakerTripped ? 0 : powerLoad.toFixed(1)}</span>
                  <span className="text-xs text-slate-400">Watt</span>
                </div>
                <div className="mt-3 flex justify-between text-xs">
                  <span className="text-emerald-400 font-semibold">Peak Shave: -18.5%</span>
                  <span className="text-slate-400">Cap: {isBreakerTripped ? 0 : powerCap}W</span>
                </div>
              </div>

              {/* Temp */}
              <div className="glass-card rounded-2xl p-5">
                <span className="text-xs uppercase text-slate-400">GPU Junction Temp</span>
                <div className="mt-2 flex items-baseline gap-2">
                  <span className="text-3xl font-mono font-extrabold text-rose-400">{isBreakerTripped ? 28 : temp}</span>
                  <span className="text-xs text-slate-400">°C</span>
                </div>
                <div className="mt-3 flex justify-between text-xs">
                  <span className={temp > 78 ? "text-rose-400 font-bold" : "text-emerald-400"}>
                    {temp > 78 ? "⚠️ Preemptive Zone" : "● Safe Envelope"}
                  </span>
                  <span className="text-slate-400">Trip: 85.0°C</span>
                </div>
              </div>

              {/* AI Efficiency */}
              <div className="glass-card rounded-2xl p-5">
                <span className="text-xs uppercase text-slate-400">Energy Per Token</span>
                <div className="mt-2 flex items-baseline gap-2">
                  <span className="text-3xl font-mono font-extrabold text-cyan-400">{isBreakerTripped ? 0 : joulesPerToken}</span>
                  <span className="text-xs text-slate-400">J/Tok</span>
                </div>
                <div className="mt-3 flex justify-between text-xs">
                  <span className="text-cyan-400">Throughput: {isBreakerTripped ? 0 : tps.toFixed(1)} TPS</span>
                  <span className="text-slate-400">LLM Inference</span>
                </div>
              </div>

              {/* Carbon */}
              <div className="glass-card rounded-2xl p-5">
                <span className="text-xs uppercase text-slate-400">Carbon Rate (ESG)</span>
                <div className="mt-2 flex items-baseline gap-2">
                  <span className="text-3xl font-mono font-extrabold text-emerald-400">{isBreakerTripped ? 0 : carbonRate}</span>
                  <span className="text-xs text-slate-400">gCO₂/h</span>
                </div>
                <div className="mt-3 flex justify-between text-xs">
                  <span className="text-emerald-400 font-semibold">PUE: 1.15 Target</span>
                  <span className="text-slate-400">ISO 14064-1</span>
                </div>
              </div>
            </div>

            {/* Interactive Controls Bar */}
            <div className="mt-8 rounded-2xl bg-black/40 p-6 border border-white/5 flex flex-col md:flex-row items-center justify-between gap-6">
              <div className="w-full md:w-1/2">
                <div className="flex justify-between text-xs font-mono text-slate-300 mb-2">
                  <span>Simulasi Beban GPU (Watts):</span>
                  <strong className="text-cyan-400">{powerLoad} Watt</strong>
                </div>
                <input
                  type="range"
                  min="800"
                  max="3400"
                  step="50"
                  value={powerLoad}
                  disabled={isBreakerTripped}
                  onChange={(e) => setPowerLoad(Number(e.target.value))}
                  className="w-full h-2 bg-slate-800 rounded-lg appearance-none cursor-pointer accent-cyan-500"
                />
              </div>

              <div className="w-full md:w-1/3">
                <div className="flex justify-between text-xs font-mono text-slate-300 mb-2">
                  <span>Throughput Inferensi (TPS):</span>
                  <strong className="text-purple-400">{tps} TPS</strong>
                </div>
                <input
                  type="range"
                  min="80"
                  max="240"
                  step="5"
                  value={tps}
                  disabled={isBreakerTripped}
                  onChange={(e) => setTps(Number(e.target.value))}
                  className="w-full h-2 bg-slate-800 rounded-lg appearance-none cursor-pointer accent-purple-500"
                />
              </div>

              <div className="flex items-center gap-3">
                <button
                  onClick={() => setIsBreakerTripped(!isBreakerTripped)}
                  className={`flex items-center gap-2 rounded-xl px-4 py-2.5 text-xs font-bold transition-all ${
                    isBreakerTripped
                      ? "bg-emerald-500 text-black shadow-neon-green hover:bg-emerald-400"
                      : "bg-rose-600 text-white shadow-lg hover:bg-rose-500"
                  }`}
                >
                  <Power className="h-4 w-4" />
                  <span>{isBreakerTripped ? "RESET CIRCUIT BREAKER" : "TRIGGER KILL-SWITCH"}</span>
                </button>
              </div>
            </div>

            {/* Merkle Certificate Action */}
            <div className="mt-6 flex flex-wrap items-center justify-between gap-4 border-t border-white/10 pt-6">
              <div className="text-xs text-slate-400 font-mono">
                <span>Merkle Root: </span>
                <span className="text-cyan-300">e3b0c44298fc1c149afbf4c8...3b1c</span> (SHA-256 Tamper-Proof Chain)
              </div>
              <button
                onClick={handleDownloadEsg}
                className="flex items-center gap-2 rounded-xl bg-emerald-600/30 border border-emerald-500/40 px-4 py-2 text-xs font-semibold text-emerald-300 hover:bg-emerald-600/50 transition-colors"
              >
                <Download className="h-4 w-4" />
                <span>Unduh Sertifikat Kepatuhan ESG (.JSON)</span>
              </button>
            </div>
          </div>
        </div>
      </section>

      {/* 5. 6 PILAR ARSITEKTUR KELAS INDUSTRI */}
      <section id="arsitektur" className="px-6 py-20 bg-slate-950/60 border-t border-white/5">
        <div className="mx-auto max-w-6xl">
          <div className="text-center max-w-3xl mx-auto">
            <span className="text-xs font-bold uppercase tracking-wider text-cyan-400">Infrastruktur Standar Militer & Perbankan</span>
            <h2 className="mt-3 text-3xl font-extrabold text-white sm:text-4xl">
              6 Pilar Teknologi Otonom ZentyElastis™
            </h2>
            <p className="mt-4 text-slate-400 text-sm leading-relaxed">
              Didesain khusus untuk beban kerja komputasi AI intensif, data center sovereign, dan kluster GPU hyperscaler tanpa kompromi performa.
            </p>
          </div>

          <div className="mt-14 grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
            {/* 1 */}
            <div className="glass-card rounded-2xl p-6">
              <div className="h-10 w-10 rounded-xl bg-cyan-500/20 text-cyan-400 flex items-center justify-center mb-4">
                <Zap className="h-5 w-5" />
              </div>
              <h4 className="text-base font-bold text-white">Rust Axum Core Gateway</h4>
              <p className="mt-2 text-xs text-slate-400 leading-relaxed">
                Pemrosesan telemetri ingest dengan latensi sub-milidetik (&lt;0.1ms). Mampu menampung 50.000+ metrik/detik dengan konsumsi memori minimal (&lt;30MB RAM).
              </p>
            </div>

            {/* 2 */}
            <div id="deepoptiflex" className="glass-card rounded-2xl p-6 border-cyan-500/30">
              <div className="h-10 w-10 rounded-xl bg-emerald-500/20 text-emerald-400 flex items-center justify-center mb-4">
                <Activity className="h-5 w-5" />
              </div>
              <h4 className="text-base font-bold text-white">DeepOptiFlex™ Predictive Shaving</h4>
              <p className="mt-2 text-xs text-slate-400 leading-relaxed">
                Algoritma prediktif berbasis intelligence feed GPlay AI yang secara dinamis memangkas beban puncak daya listrik sebesar 15% - 25% tanpa mematikan proses inferensi.
              </p>
            </div>

            {/* 3 */}
            <div id="slashield" className="glass-card rounded-2xl p-6 border-purple-500/30">
              <div className="h-10 w-10 rounded-xl bg-purple-500/20 text-purple-400 flex items-center justify-center mb-4">
                <Shield className="h-5 w-5" />
              </div>
              <h4 className="text-base font-bold text-white">SLAShield™ Guarantee Guard</h4>
              <p className="mt-2 text-xs text-slate-400 leading-relaxed">
                Penjaga Time-To-First-Token (TTFT) dan throughput TPS inferensi AI. Jika power capping mengancam SLA, sistem otomatis menaikkan kuota daya (+15% SLA Headroom Boost).
              </p>
            </div>

            {/* 4 */}
            <div id="merkle-esg" className="glass-card rounded-2xl p-6">
              <div className="h-10 w-10 rounded-xl bg-emerald-500/20 text-emerald-400 flex items-center justify-center mb-4">
                <Leaf className="h-5 w-5" />
              </div>
              <h4 className="text-base font-bold text-white">SOC Merkle Chain Audit Ledger</h4>
              <p className="mt-2 text-xs text-slate-400 leading-relaxed">
                Buku besar audit hijau berbasis pohon kriptografi SHA-256. Setiap kilowatt-hour daya dan gram CO₂ yang dihemat tercatat tamper-proof sesuai ISO 14064-1 & GHG Protocol.
              </p>
            </div>

            {/* 5 */}
            <div className="glass-card rounded-2xl p-6">
              <div className="h-10 w-10 rounded-xl bg-amber-500/20 text-amber-400 flex items-center justify-center mb-4">
                <Lock className="h-5 w-5" />
              </div>
              <h4 className="text-base font-bold text-white">Offline Air-Gapped Ed25519</h4>
              <p className="mt-2 text-xs text-slate-400 leading-relaxed">
                Penegakan lisensi kriptografi asimetris 100% offline. Server di pusat data terisolasi (air-gapped) tidak perlu koneksi internet eksternal untuk verifikasi integritas.
              </p>
            </div>

            {/* 6 */}
            <div className="glass-card rounded-2xl p-6">
              <div className="h-10 w-10 rounded-xl bg-rose-500/20 text-rose-400 flex items-center justify-center mb-4">
                <AlertTriangle className="h-5 w-5" />
              </div>
              <h4 className="text-base font-bold text-white">Actuation Assurance &amp; Kill-Switch</h4>
              <p className="mt-2 text-xs text-slate-400 leading-relaxed">
                Mekanisme pertahanan sirkuit darurat (&lt;5ms instant trip) dan self-healing engine otonom (Preemptive Workload Migration, VRAM zombie cache purge, dynamic cap pinning).
              </p>
            </div>
          </div>
        </div>
      </section>

      {/* 6. PRICING & LISENSI ENTERPRISE */}
      <section id="pricing" className="px-6 py-20">
        <div className="mx-auto max-w-6xl">
          <div className="text-center max-w-3xl mx-auto">
            <span className="text-xs font-bold uppercase tracking-wider text-emerald-400">Model Lisensi Terbuka (Open-Core)</span>
            <h2 className="mt-3 text-3xl font-extrabold text-white sm:text-4xl">
              Skema Lisensi &amp; Pengadaan Korporasi
            </h2>
            <p className="mt-4 text-slate-400 text-sm leading-relaxed">
              Pilih edisi yang sesuai dengan skala pusat data AI Anda. Dari riset open-source hingga komputasi perbankan berlisensi sovereign.
            </p>
          </div>

          <div className="mt-14 grid grid-cols-1 md:grid-cols-3 gap-8">
            {/* Community */}
            <div className="glass-card rounded-3xl p-8 flex flex-col justify-between">
              <div>
                <span className="text-xs font-bold uppercase text-slate-400">Community Edition</span>
                <div className="mt-4 flex items-baseline gap-1">
                  <span className="text-4xl font-extrabold text-white">Gratis</span>
                  <span className="text-xs text-slate-400">/ Open-Source</span>
                </div>
                <p className="mt-3 text-xs text-slate-400">Bebas digunakan untuk riset, lab GPU, dan pengembang independen.</p>
                <ul className="mt-6 space-y-3 text-xs text-slate-300">
                  <li className="flex items-center gap-2"><CheckCircle2 className="h-4 w-4 text-emerald-400" /> Hingga 8 GPU Node</li>
                  <li className="flex items-center gap-2"><CheckCircle2 className="h-4 w-4 text-emerald-400" /> Edge Telemetry Agent (NVML/ROCm)</li>
                  <li className="flex items-center gap-2"><CheckCircle2 className="h-4 w-4 text-emerald-400" /> Rust Core Gateway &lt;0.1ms</li>
                  <li className="flex items-center gap-2"><CheckCircle2 className="h-4 w-4 text-emerald-400" /> Emergency Circuit Breaker Dasar</li>
                </ul>
              </div>
              <a
                href="https://github.com/camanit/CTARTech-ZentyElastis"
                target="_blank"
                className="mt-8 block w-full rounded-xl border border-white/20 bg-white/5 py-3 text-center text-xs font-semibold text-white hover:bg-white/10 transition-colors"
              >
                Unduh via GitHub (v0.1)
              </a>
            </div>

            {/* Enterprise */}
            <div className="glass-card rounded-3xl p-8 border-2 border-cyan-500 shadow-neon relative flex flex-col justify-between">
              <div className="absolute -top-3.5 left-1/2 -translate-x-1/2 rounded-full bg-gradient-to-r from-cyan-500 to-emerald-500 px-4 py-1 text-[11px] font-bold text-black uppercase tracking-wider">
                Paling Banyak Digunakan
              </div>
              <div>
                <span className="text-xs font-bold uppercase text-cyan-400">Enterprise Edition</span>
                <div className="mt-4 flex items-baseline gap-1">
                  <span className="text-4xl font-extrabold text-white">Hubungi Kami</span>
                  <span className="text-xs text-slate-400">/ Node Kluster</span>
                </div>
                <p className="mt-3 text-xs text-slate-400">Untuk penyedia cloud GPU, AI startup skala besar, dan data center swasta.</p>
                <ul className="mt-6 space-y-3 text-xs text-slate-200">
                  <li className="flex items-center gap-2"><CheckCircle2 className="h-4 w-4 text-cyan-400" /> <strong>Node GPU Tanpa Batas</strong></li>
                  <li className="flex items-center gap-2"><CheckCircle2 className="h-4 w-4 text-cyan-400" /> <strong>DeepOptiFlex™ Predictive Shaving</strong> (-18.5%)</li>
                  <li className="flex items-center gap-2"><CheckCircle2 className="h-4 w-4 text-cyan-400" /> <strong>SLAShield™ Guarantee Guard</strong></li>
                  <li className="flex items-center gap-2"><CheckCircle2 className="h-4 w-4 text-cyan-400" /> <strong>SOC Merkle Chain Audit Ledger</strong> (ISO 14064-1)</li>
                  <li className="flex items-center gap-2"><CheckCircle2 className="h-4 w-4 text-cyan-400" /> Integrasi Sinkronisasi <strong>gplay.ctar.tech</strong></li>
                  <li className="flex items-center gap-2"><CheckCircle2 className="h-4 w-4 text-cyan-400" /> Dukungan SLA Teknis 24/7</li>
                </ul>
              </div>
              <a
                href="https://wa.me/6281260006666?text=Halo%20CTARTech,%20saya%20ingin%20mengajukan%20penawaran%20lisensi%20Enterprise%20ZentyElastis"
                target="_blank"
                className="mt-8 block w-full rounded-xl bg-cyan-500 py-3 text-center text-xs font-bold text-black hover:bg-cyan-400 shadow-neon transition-all"
              >
                Ajukan Lisensi Enterprise
              </a>
            </div>

            {/* Sovereign Air-Gapped */}
            <div className="glass-card rounded-3xl p-8 flex flex-col justify-between">
              <div>
                <span className="text-xs font-bold uppercase text-purple-400">Sovereign Air-Gapped</span>
                <div className="mt-4 flex items-baseline gap-1">
                  <span className="text-4xl font-extrabold text-white">Custom PoC</span>
                  <span className="text-xs text-slate-400">/ On-Premise</span>
                </div>
                <p className="mt-3 text-xs text-slate-400">Untuk sektor perbankan, BUMN, pertahanan, dan institusi pemerintahan.</p>
                <ul className="mt-6 space-y-3 text-xs text-slate-300">
                  <li className="flex items-center gap-2"><CheckCircle2 className="h-4 w-4 text-purple-400" /> 100% Offline Air-Gapped Deployment</li>
                  <li className="flex items-center gap-2"><CheckCircle2 className="h-4 w-4 text-purple-400" /> Ed25519 Custom Key Authority</li>
                  <li className="flex items-center gap-2"><CheckCircle2 className="h-4 w-4 text-purple-400" /> Audit Kepatuhan UU PDP No. 27/2022</li>
                  <li className="flex items-center gap-2"><CheckCircle2 className="h-4 w-4 text-purple-400" /> Dedicated Source Code Escrow</li>
                </ul>
              </div>
              <a
                href="https://wa.me/6281260006666?text=Halo%20CTARTech,%20saya%20ingin%20berdiskusi%20mengenai%20PoC%20Sovereign%20Air-Gapped%20ZentyElastis"
                target="_blank"
                className="mt-8 block w-full rounded-xl border border-purple-500/40 bg-purple-950/40 py-3 text-center text-xs font-semibold text-purple-300 hover:bg-purple-900/50 transition-colors"
              >
                Jadwalkan PoC / Pilot Run
              </a>
            </div>
          </div>
        </div>
      </section>

      {/* 7. CONTACT & RESMI PAYMENT */}
      <section className="px-6 py-16 bg-slate-950 border-t border-white/5">
        <div className="mx-auto max-w-5xl glass rounded-3xl p-8 border border-white/10">
          <div className="grid grid-cols-1 md:grid-cols-2 gap-8 items-center">
            <div>
              <span className="text-xs font-bold uppercase text-cyan-400">Kontak Resmi &amp; Pembayaran Lisensi</span>
              <h3 className="mt-2 text-2xl font-bold text-white">PT CTAR Technology Indonesia</h3>
              <p className="mt-3 text-xs text-slate-400 leading-relaxed">
                Untuk pengadaan lisensi korporat (*Enterprise License*), permohonan PoC / *Pilot Deployment*, konsultasi teknis data center, atau deposit billing komputasi:
              </p>

              <div className="mt-6 space-y-3 font-mono text-xs">
                <div className="flex items-center gap-3">
                  <div className="h-8 w-8 rounded-lg bg-emerald-500/20 text-emerald-400 flex items-center justify-center">
                    <MessageSquare className="h-4 w-4" />
                  </div>
                  <div>
                    <span className="text-slate-400 block text-[10px]">WhatsApp Hotline (Fast Response):</span>
                    <a href="https://wa.me/6281260006666" target="_blank" className="text-emerald-400 hover:underline font-bold">
                      0812-6000-6666 (a.n. Abdul Rahman Rahmad)
                    </a>
                  </div>
                </div>

                <div className="flex items-center gap-3">
                  <div className="h-8 w-8 rounded-lg bg-purple-500/20 text-purple-400 flex items-center justify-center">
                    <CreditCard className="h-4 w-4" />
                  </div>
                  <div>
                    <span className="text-slate-400 block text-[10px]">Allo Bank Account (Official Billing):</span>
                    <strong className="text-purple-300">081260006666 (a.n. Abdul Rahman Rahmad)</strong>
                  </div>
                </div>
              </div>
            </div>

            <div className="rounded-2xl bg-black/50 p-6 border border-white/5 space-y-4">
              <h4 className="text-sm font-bold text-white flex items-center gap-2">
                <Building className="h-4 w-4 text-cyan-400" />
                <span>Ekosistem Subdomain Resmi CTARTech</span>
              </h4>
              <div className="space-y-3 text-xs">
                <a
                  href="https://zentyelastis.ctar.tech"
                  className="flex items-center justify-between p-3 rounded-xl bg-white/5 hover:bg-white/10 transition-colors border border-white/5"
                >
                  <div>
                    <strong className="text-cyan-300 block">zentyelastis.ctar.tech</strong>
                    <span className="text-[10px] text-slate-400">Portal Utama Telemetri Mesh &amp; Digital Twin</span>
                  </div>
                  <ExternalLink className="h-4 w-4 text-slate-400" />
                </a>

                <a
                  href="https://gplay.ctar.tech"
                  className="flex items-center justify-between p-3 rounded-xl bg-white/5 hover:bg-white/10 transition-colors border border-white/5"
                >
                  <div>
                    <strong className="text-purple-300 block">gplay.ctar.tech</strong>
                    <span className="text-[10px] text-slate-400">Central AI Data Gateway &amp; Vector Knowledge</span>
                  </div>
                  <ExternalLink className="h-4 w-4 text-slate-400" />
                </a>

                <a
                  href="https://ctar.tech"
                  className="flex items-center justify-between p-3 rounded-xl bg-white/5 hover:bg-white/10 transition-colors border border-white/5"
                >
                  <div>
                    <strong className="text-emerald-300 block">ctar.tech</strong>
                    <span className="text-[10px] text-slate-400">Portal Induk Inovasi Teknologi Indonesia</span>
                  </div>
                  <ExternalLink className="h-4 w-4 text-slate-400" />
                </a>
              </div>
            </div>
          </div>
        </div>
      </section>

      {/* 8. FOOTER */}
      <footer className="border-t border-white/10 bg-black/80 px-6 py-8 text-center text-xs text-slate-500">
        <p>&copy; 2026 <strong>PT CTAR Technology Indonesia</strong>. Seluruh Hak Cipta Dilindungi Undang-Undang.</p>
        <p className="mt-2 text-[11px] text-slate-600">
          CTARTech-ZentyElastis™ adalah merek dagang terdaftar. Memenuhi kepatuhan ISO 14064-1, GHG Protocol Scope 2, dan UU PDP No. 27/2022.
        </p>
      </footer>
    </div>
  );
}
