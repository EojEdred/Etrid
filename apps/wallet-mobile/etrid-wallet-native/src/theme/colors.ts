export const colors = {
  // Primary brand colors
  primary: '#8b5cf6',
  primaryLight: '#a78bfa',
  primaryDark: '#7c3aed',

  // Background colors (dark theme)
  background: '#1a0033',
  backgroundLight: '#2a1050',
  backgroundDark: '#0a0018',
  surface: '#4a0080',
  surfaceLight: '#6b21a8',

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
  etr: '#8b5cf6',
  edsc: '#10b981',

  // Utility colors
  border: '#4a0080',
  borderLight: '#6b21a8',
  divider: 'rgba(139, 92, 246, 0.2)',
  overlay: 'rgba(0, 0, 0, 0.6)',

  // Glass morphism
  glass: 'rgba(139, 92, 246, 0.1)',
  glassStrong: 'rgba(139, 92, 246, 0.2)',

  // Gradients
  gradientStart: '#1a0033',
  gradientEnd: '#4a0080',
};

export type ColorKey = keyof typeof colors;
