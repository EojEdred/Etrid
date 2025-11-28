import { WidgetLayout, Widget, WidgetType } from '../types/customization'

export class CustomizationService {
  private static instance: CustomizationService

  static getInstance(): CustomizationService {
    if (!CustomizationService.instance) {
      CustomizationService.instance = new CustomizationService()
    }
    return CustomizationService.instance
  }

  async getWidgetLayout(): Promise<WidgetLayout> {
    try {
      const response = await fetch('/api/customization/widgets')
      if (!response.ok) throw new Error('Failed to fetch widget layout')
      return await response.json()
    } catch (error) {
      console.error('Error fetching widget layout:', error)
      return this.getDefaultLayout()
    }
  }

  async updateWidgetLayout(layout: WidgetLayout): Promise<void> {
    try {
      const response = await fetch('/api/customization/widgets', {
        method: 'PUT',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(layout),
      })
      if (!response.ok) throw new Error('Failed to update widget layout')
    } catch (error) {
      console.error('Error updating widget layout:', error)
      throw error
    }
  }

  async resetToDefault(): Promise<WidgetLayout> {
    try {
      const response = await fetch('/api/customization/widgets/reset', {
        method: 'POST',
      })
      if (!response.ok) throw new Error('Failed to reset widget layout')
      return await response.json()
    } catch (error) {
      console.error('Error resetting widget layout:', error)
      return this.getDefaultLayout()
    }
  }

  getDefaultLayout(): WidgetLayout {
    return {
      gridColumns: 2,
      gridRows: 6,
      widgets: [
        {
          id: 'balance-1',
          type: 'balance',
          position: { row: 0, col: 0, width: 2, height: 1 },
          enabled: true,
        },
        {
          id: 'quick-actions-1',
          type: 'quick_actions',
          position: { row: 1, col: 0, width: 2, height: 1 },
          enabled: true,
        },
        {
          id: 'chart-1',
          type: 'portfolio_chart',
          position: { row: 2, col: 0, width: 2, height: 2 },
          enabled: true,
          config: {
            timeframe: '24h',
            chartType: 'line',
          },
        },
        {
          id: 'transactions-1',
          type: 'recent_transactions',
          position: { row: 4, col: 0, width: 2, height: 2 },
          enabled: true,
        },
      ],
    }
  }

  getAvailableWidgets(): Array<{ type: WidgetType; name: string; description: string }> {
    return [
      {
        type: 'balance',
        name: 'Balance Card',
        description: 'Display your total portfolio value',
      },
      {
        type: 'price_ticker',
        name: 'Price Ticker',
        description: 'Live price updates for your assets',
      },
      {
        type: 'portfolio_chart',
        name: 'Portfolio Chart',
        description: 'Visual representation of your portfolio',
      },
      {
        type: 'quick_actions',
        name: 'Quick Actions',
        description: 'Fast access to common actions',
      },
      {
        type: 'recent_transactions',
        name: 'Recent Transactions',
        description: 'Your latest transactions',
      },
      {
        type: 'market_overview',
        name: 'Market Overview',
        description: 'Overall market trends',
      },
      {
        type: 'staking_rewards',
        name: 'Staking Rewards',
        description: 'Your staking earnings',
      },
      {
        type: 'nft_gallery',
        name: 'NFT Gallery',
        description: 'Featured NFTs from your collection',
      },
    ]
  }
}

export const customizationService = CustomizationService.getInstance()
