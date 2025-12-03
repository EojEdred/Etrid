export const colors = {
  // Primary brand colors (Cyan/Turquoise - matching iOS)
  primary: '#66D9E6',
  primaryLight: '#8DE4ED',
  primaryDark: '#00B8D9',

  // Background colors (dark theme)
  background: '#0A1A2E',
  backgroundLight: '#1A2E44',
  backgroundDark: '#050D17',
  surface: '#1A3A5C',
  surfaceLight: '#2A4A6C',

  // Text colors
  text: '#ffffff',
  textSecondary: '#9ca3af',
  textMuted: '#6b7280',

  // Accent colors
  accent: '#00d9ff',
  accentLight: '#4de4ff',
  accentDark: '#00b8d9',

  // Semantic colors
  success: '#10b981',
  successLight: '#34d399',
  error: '#ef4444',
  errorLight: '#f87171',
  warning: '#f59e0b',
  warningLight: '#fbbf24',
  info: '#3b82f6',
  infoLight: '#60a5fa',

  // Token colors
  etr: '#66D9E6',
  edsc: '#10b981',

  // Utility colors
  border: '#1A3A5C',
  borderLight: '#2A4A6C',
  divider: 'rgba(102, 217, 230, 0.2)',
  overlay: 'rgba(0, 0, 0, 0.6)',

  // Glass morphism
  glass: 'rgba(102, 217, 230, 0.1)',
  glassStrong: 'rgba(102, 217, 230, 0.2)',

  // Gradients
  gradientStart: '#0A1A2E',
  gradientEnd: '#1A3A5C',
};

export type ColorKey = keyof typeof colors;
