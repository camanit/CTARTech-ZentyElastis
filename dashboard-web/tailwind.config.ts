import type { Config } from "tailwindcss";

const config: Config = {
  content: [
    "./src/pages/**/*.{js,ts,jsx,tsx,mdx}",
    "./src/components/**/*.{js,ts,jsx,tsx,mdx}",
    "./src/app/**/*.{js,ts,jsx,tsx,mdx}",
  ],
  theme: {
    extend: {
      colors: {
        background: "#080b11",
        surface: "#0e1420",
        border: "rgba(255, 255, 255, 0.08)",
        cyber: {
          cyan: "#06b6d4",
          emerald: "#10b981",
          purple: "#8b5cf6",
          amber: "#f59e0b",
          rose: "#f43f5e",
        },
      },
      fontFamily: {
        sans: ["var(--font-inter)", "sans-serif"],
        mono: ["var(--font-jetbrains)", "monospace"],
      },
      boxShadow: {
        neon: "0 0 25px rgba(6, 182, 212, 0.25)",
        "neon-green": "0 0 25px rgba(16, 185, 129, 0.25)",
      },
    },
  },
  plugins: [],
};
export default config;
