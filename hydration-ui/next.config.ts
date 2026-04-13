import type { NextConfig } from "next";

const nextConfig: NextConfig = {
  env: {
    apiBaseUrl: "http://127.0.0.1:9527/api",
  },
  /* config options here */
  allowedDevOrigins: ["172.29.101.146"],
};

export default nextConfig;
