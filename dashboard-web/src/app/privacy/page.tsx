"use client";

import React, { useState } from "react";
import Link from "next/link";
import { ArrowLeft, Shield, Lock, EyeOff, CheckCircle2, Building, MessageSquare, CreditCard } from "lucide-react";

export default function PrivacyPage() {
  const [lang, setLang] = useState<"en" | "id">("en");

  return (
    <div className="min-h-screen text-slate-100 selection:bg-cyan-500 selection:text-black">
      {/* Navbar */}
      <nav className="border-b border-white/10 bg-slate-950/80 backdrop-blur-xl sticky top-0 z-50">
        <div className="mx-auto flex max-w-5xl items-center justify-between px-6 py-4">
          <Link href="/" className="flex items-center gap-2 text-xs font-semibold text-cyan-400 hover:text-cyan-300 transition-colors">
            <ArrowLeft className="h-4 w-4" />
            <span>{lang === "en" ? "Return to Home" : "Kembali ke Beranda"}</span>
          </Link>

          <div className="flex items-center gap-3">
            <div className="flex items-center rounded-xl border border-white/10 bg-white/5 p-1 text-xs">
              <button
                onClick={() => setLang("en")}
                className={`rounded-lg px-2.5 py-1 font-bold transition-all ${
                  lang === "en" ? "bg-cyan-500 text-black shadow-neon" : "text-slate-400 hover:text-white"
                }`}
              >
                EN
              </button>
              <button
                onClick={() => setLang("id")}
                className={`rounded-lg px-2.5 py-1 font-bold transition-all ${
                  lang === "id" ? "bg-cyan-500 text-black shadow-neon" : "text-slate-400 hover:text-white"
                }`}
              >
                ID
              </button>
            </div>
          </div>
        </div>
      </nav>

      {/* Main Content */}
      <main className="mx-auto max-w-4xl px-6 py-14">
        <div className="mb-10 text-center">
          <div className="inline-flex items-center gap-2 rounded-full border border-emerald-500/30 bg-emerald-950/40 px-3.5 py-1 text-xs text-emerald-300 mb-4">
            <Shield className="h-3.5 w-3.5" />
            <span>Zero-Trust Privacy Standard</span>
          </div>
          <h1 className="text-3xl font-extrabold text-white sm:text-4xl">
            {lang === "en" ? "Privacy Policy" : "Kebijakan Privasi"}
          </h1>
          <p className="mt-3 text-xs text-slate-400">
            {lang === "en"
              ? "Last Updated: September 2026 | Compliant with Indonesian PDP Act No. 27/2022 & ISO 27001"
              : "Terakhir Diperbarui: September 2026 | Memenuhi UU PDP No. 27/2022 & Standar ISO 27001"}
          </p>
        </div>

        <div className="glass rounded-3xl p-8 sm:p-12 space-y-8 text-sm leading-relaxed text-slate-300">
          {/* Section 1 */}
          <section>
            <h2 className="text-lg font-bold text-white flex items-center gap-2">
              <span className="text-emerald-400">1.</span>
              <span>{lang === "en" ? "Core Principle: Zero Personal Data Collection" : "Prinsip Utama: Nol Pengumpulan Data Pribadi"}</span>
            </h2>
            <p className="mt-2 text-slate-400">
              {lang === "en"
                ? "CTARTech-ZentyElastis™ is strictly an infrastructure-level GPU power, thermal, and compute performance telemetry platform. Our software does NOT collect, inspect, log, or store prompt contents, user prompts, training datasets, model weights, API keys, or personally identifiable information (PII) of your end-users."
                : "CTARTech-ZentyElastis™ adalah platform telemetri infrastruktur fisik daya dan suhu GPU. Sistem kami TIDAK PERNAH mengumpulkan, membaca, atau menyimpan isi prompt AI, dataset pelatihan model, bobot model (weights), maupun data pribadi pengguna akhir Anda."}
            </p>
          </section>

          {/* Section 2 */}
          <section>
            <h2 className="text-lg font-bold text-white flex items-center gap-2">
              <span className="text-emerald-400">2.</span>
              <span>{lang === "en" ? "Scope of Telemetry Data Processed" : "Jenis Data Telemetri yang Diproses"}</span>
            </h2>
            <p className="mt-2 text-slate-400">
              {lang === "en"
                ? "The Edge Agent only captures physical sensor and operational compute telemetry, including:"
                : "Edge Agent hanya membaca metrik sensor fisik perangkat keras, antara lain:"}
            </p>
            <ul className="mt-3 space-y-2 list-disc list-inside text-xs text-slate-300">
              <li>
                <strong>Power & Electrical Metrics:</strong>{" "}
                {lang === "en" ? "Wattage draw, core voltage (V), power factor, and throttling triggers." : "Konsumsi daya (Watt), tegangan (Volt), dan alasan pembatasan daya (throttle reasons)."}
              </li>
              <li>
                <strong>Thermal & Physical Metrics:</strong>{" "}
                {lang === "en" ? "GPU junction temperature (°C), fan speed percentage, and cooling flow state." : "Suhu junction GPU (°C), kecepatan kipas (%), dan status sistem pendingin."}
              </li>
              <li>
                <strong>Execution Performance:</strong>{" "}
                {lang === "en" ? "SM clock speed (MHz), memory clock (MHz), VRAM allocation (MB), Tokens-Per-Second (TPS), and Joules-per-Token." : "Kecepatan clock GPU (MHz), alokasi VRAM (MB), throughput token (TPS), dan efisiensi Joule/Token."}
              </li>
              <li>
                <strong>ESG Accounting:</strong>{" "}
                {lang === "en" ? "Carbon emission rates (gCO2/h) based on localized grid emission factors and PUE 1.15." : "Laju emisi karbon (gCO2/h) berbasis koefisien emisi gardu listrik lokal."}
              </li>
            </ul>
          </section>

          {/* Section 3 */}
          <section>
            <h2 className="text-lg font-bold text-white flex items-center gap-2">
              <span className="text-emerald-400">3.</span>
              <span>{lang === "en" ? "100% Offline Air-Gapped Data Sovereign Guarantee" : "Jaminan Kedaulatan Data 100% Offline"}</span>
            </h2>
            <p className="mt-2 text-slate-400">
              {lang === "en"
                ? "For clients deploying the Enterprise or Sovereign Air-Gapped Edition, all telemetry processing, DeepOptiFlex™ optimizations, and Merkle ledger recording occur 100% within your local on-premise hardware boundaries. No telemetry packets leave your private network without your explicit cryptographic authorization."
                : "Bagi klien edisi Enterprise atau Sovereign Air-Gapped, seluruh proses analisis telemetri dan pembukuan Merkle berjalan 100% di dalam infrastruktur lokal Anda. Tidak ada satu pun paket data yang dikirim ke server luar tanpa persetujuan Anda."}
            </p>
          </section>

          {/* Section 4 */}
          <section>
            <h2 className="text-lg font-bold text-white flex items-center gap-2">
              <span className="text-emerald-400">4.</span>
              <span>{lang === "en" ? "Cryptographic Integrity & Tamper-Proof Logs" : "Integritas Kriptografis & Keamanan Log"}</span>
            </h2>
            <p className="mt-2 text-slate-400">
              {lang === "en"
                ? "Every telemetry packet transmitted from Edge Agents to the Rust Core Gateway is authenticated using HMAC-SHA256 signatures with monotonic nonces to prevent replay attacks. Historical energy audit logs are sealed in an immutable Merkle Tree chain, ensuring that sustainability reports cannot be forged or manipulated."
                : "Setiap paket data telemetri diamankan dengan otentikasi HMAC-SHA256 dan anti-replay nonce. Seluruh riwayat penghematan energi disegel dalam rantai pohon kriptografi Merkle Tree SHA-256 yang kebal pemalsuan."}
            </p>
          </section>

          {/* Section 5 */}
          <section>
            <h2 className="text-lg font-bold text-white flex items-center gap-2">
              <span className="text-emerald-400">5.</span>
              <span>{lang === "en" ? "Compliance with Personal Data Protection Laws" : "Kepatuhan Terhadap Regulasi Privasi"}</span>
            </h2>
            <p className="mt-2 text-slate-400">
              {lang === "en"
                ? "PT CTAR Technology Indonesia strictly adheres to Indonesian Law No. 27 of 2022 on Personal Data Protection (UU PDP), ISO/IEC 27001 Information Security Management, and ISO 14064-1 Greenhouse Gas Verification standards."
                : "PT CTAR Technology Indonesia mematuhi Undang-Undang No. 27 Tahun 2022 tentang Pelindungan Data Pribadi (UU PDP), standar manajemen keamanan informasi ISO/IEC 27001, serta standar verifikasi emisi gas rumah kaca ISO 14064-1."}
            </p>
          </section>

          {/* Section 6 - Contact */}
          <section className="rounded-2xl bg-black/50 p-6 border border-white/5 space-y-4">
            <h3 className="text-base font-bold text-white flex items-center gap-2">
              <Building className="h-4 w-4 text-emerald-400" />
              <span>Data Protection Officer &amp; Inquiries</span>
            </h3>
            <p className="text-xs text-slate-400">
              {lang === "en"
                ? "For data sovereignty audits, security inquiries, or enterprise compliance requests, contact our designated officer:"
                : "Untuk audit kedaulatan data, pertanyaan keamanan, atau kepatuhan regulasi, hubungi petugas resmi kami:"}
            </p>
            <div className="space-y-2 font-mono text-xs text-slate-300">
              <div className="flex items-center gap-2">
                <MessageSquare className="h-4 w-4 text-emerald-400" />
                <span>WhatsApp: 0812-6000-6666 (a.n. Abdul Rahman Rahmad)</span>
              </div>
              <div className="flex items-center gap-2">
                <CreditCard className="h-4 w-4 text-purple-400" />
                <span>PT CTAR Technology Indonesia</span>
              </div>
            </div>
          </section>
        </div>
      </main>

      {/* Footer */}
      <footer className="border-t border-white/10 bg-black/80 px-6 py-6 text-center text-xs text-slate-500">
        <p>&copy; 2026 PT CTAR Technology Indonesia. All Rights Reserved.</p>
      </footer>
    </div>
  );
}
