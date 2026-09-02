import type { Metadata } from "next";
import "./globals.css";

export const metadata: Metadata = {
  metadataBase: new URL("https://zentyelastis.ctar.tech"),
  title: {
    default: "CTARTech-ZentyElastis™ | Autonomous AI Data Center Telemetry Mesh & Digital Twin",
    template: "%s | CTARTech-ZentyElastis™"
  },
  description: "Next-generation sub-millisecond GPU power optimization, DeepOptiFlex™ predictive peak shaving (-18.5%), SLAShield™ latency protection, and SOC Merkle ESG Green-AI Compliance for hyperscale AI clusters.",
  keywords: [
    "CTARTech",
    "ZentyElastis",
    "AI Data Center",
    "GPU Power Optimization",
    "DeepOptiFlex",
    "SLAShield",
    "NVIDIA H100",
    "NVIDIA Blackwell",
    "Rust Axum Gateway",
    "Telemetry Mesh",
    "Digital Twin",
    "ESG Carbon Compliance",
    "ISO 14064-1",
    "GHG Protocol",
    "PT CTAR Technology Indonesia"
  ],
  authors: [{ name: "PT CTAR Technology Indonesia", url: "https://ctar.tech" }],
  creator: "PT CTAR Technology Indonesia",
  publisher: "PT CTAR Technology Indonesia",
  robots: {
    index: true,
    follow: true,
    googleBot: {
      index: true,
      follow: true,
      "max-video-preview": -1,
      "max-image-preview": "large",
      "max-snippet": -1,
    },
  },
  openGraph: {
    type: "website",
    locale: "en_US",
    alternateLocale: "id_ID",
    url: "https://zentyelastis.ctar.tech",
    siteName: "CTARTech-ZentyElastis™",
    title: "CTARTech-ZentyElastis™ | Autonomous AI Data Center Telemetry Mesh",
    description: "Sub-millisecond Rust core gateway, DeepOptiFlex™ predictive peak shaving, and cryptographic ESG audit ledger for hyperscale AI GPU clusters.",
    images: [
      {
        url: "/logo.png",
        width: 800,
        height: 800,
        alt: "CTARTech-ZentyElastis Logo",
      },
    ],
  },
  twitter: {
    card: "summary_large_image",
    title: "CTARTech-ZentyElastis™ | Autonomous AI Data Center Power Optimization",
    description: "Predictive peak power shaving & SLA latency shield for AI GPU clusters. Sub-millisecond Rust Axum Core.",
    images: ["/logo.png"],
    creator: "@ctartech",
  },
  alternates: {
    canonical: "https://zentyelastis.ctar.tech",
  },
  icons: {
    icon: "/favicon.ico",
    apple: "/logo.png",
  },
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en" className="scroll-smooth">
      <body className="antialiased min-h-screen selection:bg-cyan-500 selection:text-black">
        {children}
      </body>
    </html>
  );
}
