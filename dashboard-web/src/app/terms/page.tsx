"use client";

import React, { useState } from "react";
import Link from "next/link";
import { ArrowLeft, Shield, FileText, CheckCircle2, Building, MessageSquare, CreditCard } from "lucide-react";

export default function TermsPage() {
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
          <div className="inline-flex items-center gap-2 rounded-full border border-cyan-500/30 bg-cyan-950/40 px-3.5 py-1 text-xs text-cyan-300 mb-4">
            <FileText className="h-3.5 w-3.5" />
            <span>Legal Documentation</span>
          </div>
          <h1 className="text-3xl font-extrabold text-white sm:text-4xl">
            {lang === "en" ? "Terms of Service" : "Syarat & Ketentuan Layanan"}
          </h1>
          <p className="mt-3 text-xs text-slate-400">
            {lang === "en"
              ? "Last Updated: September 2026 | PT CTAR Technology Indonesia"
              : "Terakhir Diperbarui: September 2026 | PT CTAR Technology Indonesia"}
          </p>
        </div>

        <div className="glass rounded-3xl p-8 sm:p-12 space-y-8 text-sm leading-relaxed text-slate-300">
          {/* Section 1 */}
          <section>
            <h2 className="text-lg font-bold text-white flex items-center gap-2">
              <span className="text-cyan-400">1.</span>
              <span>{lang === "en" ? "Acceptance of Terms" : "Penerimaan Ketentuan"}</span>
            </h2>
            <p className="mt-2 text-slate-400">
              {lang === "en"
                ? "By accessing, downloading, deploying, or utilizing the CTARTech-ZentyElastis™ software, Core Runtime Gateway, Edge Agents, or related services provided by PT CTAR Technology Indonesia, you agree to be legally bound by these Terms of Service. If you are entering into this agreement on behalf of a company or entity, you warrant that you have full legal authority to bind that entity."
                : "Dengan mengakses, mengunduh, memasang, atau menggunakan perangkat lunak CTARTech-ZentyElastis™, Core Runtime Gateway, Edge Agent, atau layanan terkait yang disediakan oleh PT CTAR Technology Indonesia, Anda menyatakan setuju untuk terikat secara hukum oleh Syarat & Ketentuan Layanan ini."}
            </p>
          </section>

          {/* Section 2 */}
          <section>
            <h2 className="text-lg font-bold text-white flex items-center gap-2">
              <span className="text-cyan-400">2.</span>
              <span>{lang === "en" ? "Open-Core Licensing Scope" : "Cakupan Lisensi Open-Core"}</span>
            </h2>
            <p className="mt-2 text-slate-400">
              {lang === "en"
                ? "CTARTech-ZentyElastis™ operates under an Open-Core licensing model:"
                : "CTARTech-ZentyElastis™ beroperasi di bawah model lisensi Open-Core:"}
            </p>
            <ul className="mt-3 space-y-2 list-disc list-inside text-xs text-slate-300">
              <li>
                <strong>Community Edition (Open-Source):</strong>{" "}
                {lang === "en"
                  ? "Free for non-commercial research and clusters with up to 8 GPU nodes under the permissive license."
                  : "Gratis untuk riset non-komersial dan kluster hingga 8 node GPU di bawah lisensi publik."}
              </li>
              <li>
                <strong>Enterprise Edition (Commercial):</strong>{" "}
                {lang === "en"
                  ? "Requires an active Ed25519 cryptographic license issued by PT CTAR Technology Indonesia. Grants unlimited GPU scaling, DeepOptiFlex™ predictive peak shaving, SLAShield™ latency protection, and GPlay AI Gateway synchronization."
                  : "Memerlukan file lisensi kriptografis Ed25519 resmi yang diterbitkan oleh PT CTAR Technology Indonesia untuk kluster GPU skala besar."}
              </li>
              <li>
                <strong>Sovereign Air-Gapped Edition:</strong>{" "}
                {lang === "en"
                  ? "Subject to custom on-premise governance, dedicated source-code escrow, and strict compliance with national data sovereignty laws."
                  : "Khusus untuk perbankan, BUMN, pertahanan, dan institusi pemerintahan dengan instalasi 100% offline terisolasi."}
              </li>
            </ul>
          </section>

          {/* Section 3 */}
          <section>
            <h2 className="text-lg font-bold text-white flex items-center gap-2">
              <span className="text-cyan-400">3.</span>
              <span>{lang === "en" ? "Actuation Assurance & Hardware Safety Disclaimer" : "Keselamatan Hardware & Pemutus Sirkuit Otomatis"}</span>
            </h2>
            <p className="mt-2 text-slate-400">
              {lang === "en"
                ? "ZentyElastis™ features an Emergency Circuit Breaker protocol designed to isolate GPU loads within <5ms during critical thermal overloads (>85°C) or extreme power surges (>3,500W). While this mechanism is built to military-grade standards, PT CTAR Technology Indonesia is not liable for indirect physical damages resulting from third-party hardware failures, improper electrical wiring, or unauthorized firmware modifications."
                : "ZentyElastis™ dilengkapi protokol Pemutus Sirkuit Darurat (<5ms) untuk melindungi GPU dari lonjakan suhu (>85°C) dan daya berlebih. Pengguna bertanggung jawab memastikan spesifikasi instalasi kelistrikan data center memenuhi standar keselamatan industri."}
            </p>
          </section>

          {/* Section 4 */}
          <section>
            <h2 className="text-lg font-bold text-white flex items-center gap-2">
              <span className="text-cyan-400">4.</span>
              <span>{lang === "en" ? "Intellectual Property & Anti-Tampering" : "Hak Kekayaan Intelektual & Anti-Pemalsuan"}</span>
            </h2>
            <p className="mt-2 text-slate-400">
              {lang === "en"
                ? "The DeepOptiFlex™, SLAShield™, and SOC Merkle Chain proprietary algorithms, logos, and Ed25519 signing authorities are intellectual property of PT CTAR Technology Indonesia. Tampering with offline cryptographic signatures, reverse engineering private keys, or distributing unauthorized license files constitutes a violation of copyright law."
                : "Seluruh algoritma prediktif, tanda tangan digital Ed25519, dan merek dagang CTARTech-ZentyElastis™ dilindungi hak cipta. Dilarang keras memalsukan file lisensi atau merekayasa balik mekanisme keamanan Zero-Trust."}
            </p>
          </section>

          {/* Section 5 */}
          <section>
            <h2 className="text-lg font-bold text-white flex items-center gap-2">
              <span className="text-cyan-400">5.</span>
              <span>{lang === "en" ? "Governing Law & Dispute Resolution" : "Hukum yang Berlaku & Penyelesaian Sengketa"}</span>
            </h2>
            <p className="mt-2 text-slate-400">
              {lang === "en"
                ? "These Terms of Service are governed by and construed in accordance with the laws of the Republic of Indonesia, including Law No. 11/2008 on Electronic Information and Transactions (UU ITE) and Personal Data Protection Act No. 27/2022 (UU PDP). Any disputes shall be resolved through amicable consensus, or exclusively through the competent jurisdiction of Medan / Jakarta, Indonesia."
                : "Ketentuan Layanan ini diatur berdasarkan hukum Negara Kesatuan Republik Indonesia, termasuk UU ITE No. 11/2008 dan UU PDP No. 27/2022. Sengketa akan diselesaikan secara musyawarah mufakat atau melalui pengadilan yang berwenang di Indonesia."}
            </p>
          </section>

          {/* Section 6 - Contact */}
          <section className="rounded-2xl bg-black/50 p-6 border border-white/5 space-y-4">
            <h3 className="text-base font-bold text-white flex items-center gap-2">
              <Building className="h-4 w-4 text-cyan-400" />
              <span>PT CTAR Technology Indonesia</span>
            </h3>
            <p className="text-xs text-slate-400">
              {lang === "en"
                ? "For inquiries regarding enterprise licensing, technical audits, or legal compliance, contact:"
                : "Untuk pertanyaan mengenai lisensi korporat, audit teknis, atau kepatuhan hukum, hubungi:"}
            </p>
            <div className="space-y-2 font-mono text-xs text-slate-300">
              <div className="flex items-center gap-2">
                <MessageSquare className="h-4 w-4 text-emerald-400" />
                <span>WhatsApp: 0812-6000-6666 (a.n. Abdul Rahman Rahmad)</span>
              </div>
              <div className="flex items-center gap-2">
                <CreditCard className="h-4 w-4 text-purple-400" />
                <span>Billing: Allo Bank 081260006666 (a.n. Abdul Rahman Rahmad)</span>
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
