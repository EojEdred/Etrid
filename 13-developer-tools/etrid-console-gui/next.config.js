/** @type {import('next').NextConfig} */
const nextConfig = {
  output: 'export', // Add this line for static HTML export
  images: {
    // Disable image optimization for static export (Tauri / `out/`).
    unoptimized: true,
    domains: ["placehold.co", "ui-avatars.com", "cdnjs.cloudflare.com", "cdn.jsdelivr.net"],
  },
  serverExternalPackages: ["@polkadot"],
  webpack: (config, { isServer }) => {
    // Workaround: the production minifier can emit invalid JS for some crypto deps
    // (e.g. `\00` in template strings), which breaks Tauri/WebKit at runtime.
    config.optimization.minimize = false;
    config.optimization.minimizer = [];

    // Handle React Native specific modules for web build
    if (!isServer) {
      config.resolve.fallback = {
        ...config.resolve.fallback,
        fs: false,
        net: false,
        tls: false,
      };

      // Alias react-native modules to false to exclude them from web build
      config.resolve.alias = {
        ...config.resolve.alias,
        '@react-native-async-storage/async-storage': false,
        '@react-native-community/netinfo': false,
        'react-native$': false,
        'react-native-web$': false,
        '@react-native-async-storage/async-storage/jest/async-storage-mock': false,
      };

      // Define global variables for React Native compatibility
      config.plugins = config.plugins || [];
      config.plugins.push(
        new (require('webpack')).DefinePlugin({
          'global.TYPED_ARRAY_SUPPORT': true,
          'process.browser': true,
          'process.env.TARGET_BROWSER': '"chrome"',
        })
      );
    }

    return config;
  },
};

module.exports = nextConfig;
