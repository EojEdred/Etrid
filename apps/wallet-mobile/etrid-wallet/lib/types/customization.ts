// Customization Types

export interface Theme {
  id: string
  name: string
  colors: ThemeColors
  builtIn: boolean
  userId?: string
  createdAt?: Date
}

export interface ThemeColors {
  primary: string
  background: string
  card: string
  text: string
  textSecondary: string
  accent: string
  border: string
  success: string
  warning: string
  error: string
}

export interface CustomTheme {
  name: string
  colors: ThemeColors
}

export interface Widget {
  id: string
  type: WidgetType
  position: WidgetPosition
  enabled: boolean
  config?: WidgetConfig
}

export type WidgetType =
  | 'balance'
  | 'price_ticker'
  | 'portfolio_chart'
  | 'quick_actions'
  | 'recent_transactions'
  | 'market_overview'
  | 'staking_rewards'
  | 'nft_gallery'

export interface WidgetPosition {
  row: number
  col: number
  width: number
  height: number
}

export interface WidgetConfig {
  title?: string
  assets?: string[]
  timeframe?: string
  showLabels?: boolean
  chartType?: 'line' | 'bar' | 'pie'
}

export interface WidgetLayout {
  widgets: Widget[]
  gridColumns: number
  gridRows: number
}

export interface UserPreferences {
  userId: string
  themeId: string
  widgetLayout: WidgetLayout
  currency: string
  language: string
  autoDarkMode: boolean
  darkModeStart?: string
  darkModeEnd?: string
}

export const BUILT_IN_THEMES: Theme[] = [
  {
    id: 'light',
    name: 'Light',
    builtIn: true,
    colors: {
      primary: '#3b82f6',
      background: '#ffffff',
      card: '#f9fafb',
      text: '#111827',
      textSecondary: '#6b7280',
      accent: '#8b5cf6',
      border: '#e5e7eb',
      success: '#10b981',
      warning: '#f59e0b',
      error: '#ef4444',
    },
  },
  {
    id: 'dark',
    name: 'Dark',
    builtIn: true,
    colors: {
      primary: '#3b82f6',
      background: '#0f172a',
      card: '#1e293b',
      text: '#f1f5f9',
      textSecondary: '#94a3b8',
      accent: '#8b5cf6',
      border: '#334155',
      success: '#10b981',
      warning: '#f59e0b',
      error: '#ef4444',
    },
  },
  {
    id: 'midnight',
    name: 'Midnight Blue',
    builtIn: true,
    colors: {
      primary: '#0ea5e9',
      background: '#020617',
      card: '#0c1525',
      text: '#e0f2fe',
      textSecondary: '#7dd3fc',
      accent: '#06b6d4',
      border: '#1e3a5f',
      success: '#14b8a6',
      warning: '#fbbf24',
      error: '#f87171',
    },
  },
  {
    id: 'forest',
    name: 'Forest Green',
    builtIn: true,
    colors: {
      primary: '#22c55e',
      background: '#052e16',
      card: '#14532d',
      text: '#dcfce7',
      textSecondary: '#86efac',
      accent: '#84cc16',
      border: '#166534',
      success: '#4ade80',
      warning: '#facc15',
      error: '#fb7185',
    },
  },
  {
    id: 'cyberpunk',
    name: 'Cyberpunk',
    builtIn: true,
    colors: {
      primary: '#ec4899',
      background: '#18181b',
      card: '#27272a',
      text: '#fdf4ff',
      textSecondary: '#e879f9',
      accent: '#06b6d4',
      border: '#3f3f46',
      success: '#2dd4bf',
      warning: '#fbbf24',
      error: '#f43f5e',
    },
  },
]
