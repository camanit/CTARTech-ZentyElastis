import sys
import re

if sys.platform == "win32":
    try:
        sys.stdout.reconfigure(encoding="utf-8")
        sys.stderr.reconfigure(encoding="utf-8")
    except Exception:
        pass

docs_path = r"C:\Users\UseR\Herd\gplay\resources\views\api-docs.blade.php"

with open(docs_path, "r", encoding="utf-8") as f:
    content = f.read()

# 1. Update Sidebar Navigation
sidebar_target = '<a href="#sec-66" class="block px-3 py-1.5 rounded-lg text-slate-300 hover:bg-slate-900 hover:text-cyan-400 font-medium">66. Automated IT Self-Healing Script Generator API</a>'
sidebar_addition = '''<a href="#sec-66" class="block px-3 py-1.5 rounded-lg text-slate-300 hover:bg-slate-900 hover:text-cyan-400 font-medium">66. Automated IT Self-Healing Script Generator API</a>
                      <div class="pt-3 pb-1 text-[10px] font-bold text-emerald-400 tracking-wider">--- CTARTECH-ZENTYELASTIS GPU AI & TELEMETRY MESH ---</div>
                      <a href="#sec-67" class="block px-3 py-1.5 rounded-lg text-slate-300 hover:bg-slate-900 hover:text-emerald-400 font-medium">67. ZentyElastis GPU Telemetry Sync API</a>
                      <a href="#sec-68" class="block px-3 py-1.5 rounded-lg text-slate-300 hover:bg-slate-900 hover:text-emerald-400 font-medium">68. DeepOptiFlex™ Dynamic Peak Shaving API</a>
                      <a href="#sec-69" class="block px-3 py-1.5 rounded-lg text-slate-300 hover:bg-slate-900 hover:text-emerald-400 font-medium">69. ZentyElastis Cluster Health & Digital Twin API</a>
                      <a href="#sec-70" class="block px-3 py-1.5 rounded-lg text-slate-300 hover:bg-slate-900 hover:text-emerald-400 font-medium">70. ZentyElastis Gateway Ping & Attestation API</a>'''

if sidebar_target in content and "sec-67" not in content:
    content = content.replace(sidebar_target, sidebar_addition)
    print("✅ Sidebar updated in api-docs.blade.php")
else:
    print("ℹ️ Sidebar already updated or target not found.")

# 2. Update Main Documentation Content
sections_to_add = '''
            <!-- 67. ZentyElastis GPU Telemetry Sync API -->
            <section id="sec-67" class="glass p-8 rounded-2xl border border-emerald-500/30 space-y-4">
                <div class="flex items-center gap-3">
                    <span class="px-2.5 py-1 rounded bg-emerald-500/20 text-emerald-400 font-bold text-xs font-mono">POST</span>
                    <code class="text-sm text-white">/api/v1/zenty/telemetry/sync</code>
                    <span class="text-[10px] px-2 py-0.5 rounded bg-emerald-500/10 text-emerald-400 border border-emerald-500/20 font-bold">GPU TELEMETRY MESH</span>
                </div>
                <h2 class="text-xl font-bold text-white">67. ZentyElastis GPU Telemetry Sync API</h2>
                <p class="text-slate-400 text-xs">Pusat sinkronisasi batch telemetri multi-dimensi (Wattage, Junction Temp, GPU Clocks, Throttle Reasons, VRAM, Joules/Token, Voltage) dari Rust Core Gateway ke GPlay AI Bank Data dengan latensi sub-milidetik.</p>
            </section>

            <!-- 68. DeepOptiFlex™ Dynamic Peak Shaving API -->
            <section id="sec-68" class="glass p-8 rounded-2xl border border-purple-500/30 space-y-4">
                <div class="flex items-center gap-3">
                    <span class="px-2.5 py-1 rounded bg-purple-500/20 text-purple-400 font-bold text-xs font-mono">GET</span>
                    <code class="text-sm text-white">/api/v1/zenty/recommendations</code>
                    <span class="text-[10px] px-2 py-0.5 rounded bg-purple-500/10 text-purple-400 border border-purple-500/20 font-bold">AI PEAK SHAVING</span>
                </div>
                <h2 class="text-xl font-bold text-white">68. DeepOptiFlex™ Dynamic Peak Shaving API</h2>
                <p class="text-slate-400 text-xs">Menyuplai batas daya dinamis (dynamic peak limit) dan prediksi lonjakan beban komputasi AI dari model prediktif GPlay AI untuk menghemat biaya operasional listrik data center hingga 15–25%.</p>
            </section>

            <!-- 69. ZentyElastis Cluster Health & Digital Twin API -->
            <section id="sec-69" class="glass p-8 rounded-2xl border border-cyan-500/30 space-y-4">
                <div class="flex items-center gap-3">
                    <span class="px-2.5 py-1 rounded bg-cyan-500/20 text-cyan-400 font-bold text-xs font-mono">GET</span>
                    <code class="text-sm text-white">/api/v1/zenty/cluster/{cluster_id}/health</code>
                    <span class="text-[10px] px-2 py-0.5 rounded bg-cyan-500/10 text-cyan-400 border border-cyan-500/20 font-bold">DIGITAL TWIN STATUS</span>
                </div>
                <h2 class="text-xl font-bold text-white">69. ZentyElastis Cluster Health & Digital Twin API</h2>
                <p class="text-slate-400 text-xs">Menyediakan status kesehatan real-time seluruh node kluster GPU, agregasi metrik daya aktif, dan histori beban untuk visualisasi 3D Resource Twin pada dasbor enterprise.</p>
            </section>

            <!-- 70. ZentyElastis Gateway Ping & Attestation API -->
            <section id="sec-70" class="glass p-8 rounded-2xl border border-blue-500/30 space-y-4">
                <div class="flex items-center gap-3">
                    <span class="px-2.5 py-1 rounded bg-blue-500/20 text-blue-400 font-bold text-xs font-mono">GET</span>
                    <code class="text-sm text-white">/api/v1/zenty/ping</code>
                    <span class="text-[10px] px-2 py-0.5 rounded bg-blue-500/10 text-blue-400 border border-blue-500/20 font-bold">ZERO-TRUST HARNESS</span>
                </div>
                <h2 class="text-xl font-bold text-white">70. ZentyElastis Gateway Ping & Attestation API</h2>
                <p class="text-slate-400 text-xs">Endpoint verifikasi status aktif (liveness check) GPlay AI Gateway dan handshake Zero-Trust Edge-to-Core Harness dengan dukungan respon multibahasa (ID, EN, MY).</p>
            </section>
'''

if "id=\"sec-67\"" not in content:
    pattern = r"(</section>\s*</main>)"
    replacement = r"\n" + sections_to_add + r"\n        </main>"
    content, count = re.subn(pattern, replacement, content, count=1)
    if count > 0:
        print("✅ Added Sections 67, 68, 69, 70 to api-docs.blade.php")
    else:
        print("❌ Could not match closing </main> tag.")
else:
    print("ℹ️ Sections 67-70 already present.")

with open(docs_path, "w", encoding="utf-8") as f:
    f.write(content)

print("🎉 File api-docs.blade.php updated successfully!")
