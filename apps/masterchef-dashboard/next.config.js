/** @type {import('next').NextConfig} */
const nextConfig = {
  output: 'standalone',
  reactStrictMode: true,
  swcMinify: true,

  // Enable static export for CDN hosting (optional)
  // output: 'export',
  // trailingSlash: true,

  // Image optimization
  images: {
    unoptimized: true,
  },

  // Environment variables exposed to browser
  env: {
    NEXT_PUBLIC_APP_NAME: 'MasterChef Dashboard',
    NEXT_PUBLIC_APP_VERSION: '1.0.0',
  },

  // Headers for security
  async headers() {
    return [
      {
        source: '/:path*',
        headers: [
          {
            key: 'X-Frame-Options',
            value: 'DENY',
          },
          {
            key: 'X-Content-Type-Options',
            value: 'nosniff',
          },
          {
            key: 'Referrer-Policy',
            value: 'strict-origin-when-cross-origin',
          },
        ],
      },
    ];
  },
};

module.exports = nextConfig;
