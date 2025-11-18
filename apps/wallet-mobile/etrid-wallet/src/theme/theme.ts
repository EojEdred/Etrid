import { DefaultTheme } from 'react-native-paper';

export const colors = {
  // Brand Colors
  primary: '#6C5CE7',      // Purple - Ëtrid brand
  secondary: '#00B894',    // Green - success, positive
  accent: '#FD79A8',       // Pink - highlights
  
  // Backgrounds
  background: '#FFFFFF',   // Light background
  backgroundDark: '#1E1E1E', // Dark background
  surface: '#F7F7F7',      // Card/surface color
  surfaceDark: '#2D2D2D',  // Dark mode surface
  
  // Text
  text: '#2D3436',         // Dark text
  textLight: '#DFE6E9',    // Light text (dark mode)
  textSecondary: '#636E72',// Secondary text
  
  // Status
  success: '#00B894',      // Success/positive
  warning: '#FDCB6E',      // Warning
  error: '#D63031',        // Error/negative
  info: '#74B9FF',         // Info
  
  // Grays
  gray100: '#F7F7F7',
  gray200: '#DFE6E9',
  gray300: '#B2BEC3',
  gray400: '#636E72',
  gray500: '#2D3436',
  
  // Chart Colors
  chart: {
    etr: '#6C5CE7',
    btc: '#F7931A',
    eth: '#627EEA',
    sol: '#14F195',
    usdt: '#26A17B',
  }
};

export const spacing = {
  xs: 4,
  sm: 8,
  md: 16,
  lg: 24,
  xl: 32,
  xxl: 48,
};

export const borderRadius = {
  sm: 4,
  md: 8,
  lg: 16,
  xl: 24,
  full: 9999,
};

export const typography = {
  h1: {
    fontSize: 32,
    fontWeight: 'bold' as const,
    lineHeight: 40,
  },
  h2: {
    fontSize: 24,
    fontWeight: 'bold' as const,
    lineHeight: 32,
  },
  h3: {
    fontSize: 20,
    fontWeight: '600' as const,
    lineHeight: 28,
  },
  body: {
    fontSize: 16,
    fontWeight: 'normal' as const,
    lineHeight: 24,
  },
  bodySmall: {
    fontSize: 14,
    fontWeight: 'normal' as const,
    lineHeight: 20,
  },
  caption: {
    fontSize: 12,
    fontWeight: 'normal' as const,
    lineHeight: 16,
  },
};

export const theme = {
  ...DefaultTheme,
  colors: {
    ...DefaultTheme.colors,
    primary: colors.primary,
    accent: colors.accent,
    background: colors.background,
    surface: colors.surface,
    text: colors.text,
    error: colors.error,
  },
  roundness: borderRadius.md,
};

export const darkTheme = {
  ...DefaultTheme,
  dark: true,
  colors: {
    ...DefaultTheme.colors,
    primary: colors.primary,
    accent: colors.accent,
    background: colors.backgroundDark,
    surface: colors.surfaceDark,
    text: colors.textLight,
    error: colors.error,
  },
  roundness: borderRadius.md,
};
