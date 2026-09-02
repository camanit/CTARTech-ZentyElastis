import type { Metadata } from "next";
import "./globals.css";

export const metadata: Metadata = {
  title: "CTARTech-ZentyElastis™ | Autonomous AI Data Center Telemetry Mesh & Digital Twin",
  description: "Next-generation sub-millisecond GPU power optimization, DeepOptiFlex™ predictive peak shaving, SLAShield™ latency protection, and SOC Merkle ESG Green-AI Compliance.",
  icons: {
    icon: "/favicon.ico",
  },
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="id" className="scroll-smooth">
      <body className="antialiased min-h-screen selection:bg-cyan-500 selection:text-black">
        {children}
      </body>
    </html>
  );
}
