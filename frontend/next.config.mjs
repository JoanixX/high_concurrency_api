/** @type {import('next').NextConfig} */
const nextConfig = {
  // build autocontenido para docker/deploy sin node_modules
  output: 'standalone',
  images: {
    remotePatterns: [],
    formats: ['image/avif', 'image/webp'],
  },
  experimental: {
    // tree-shaking más agresivo para estas librerías
    optimizePackageImports: ['@tanstack/react-query', 'zustand', 'lucide-react'],
  },
};

export default nextConfig;
