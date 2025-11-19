# Build Prompt: 18 Advanced Features for Etrid Mobile Wallet

## Instructions for Claude Code

**CRITICAL**: Use ultrathink (extended thinking) and launch 8 parallel subagents to build all features simultaneously. Each agent should handle 2-3 related features.

---

## Context

The Etrid mobile wallet already has a foundation built:
- **Location**: `/Users/macbook/Desktop/etrid/apps/wallet-mobile/etrid-wallet/`
- **Stack**: React Native 0.72 + Expo 49 + TypeScript
- **Existing**: 126 files, 25,231 lines (auth, home, send/receive, staking, governance, ATM, Ledger, backend)

Reference these existing files for patterns:
- `src/theme/theme.ts` - Design system (colors, spacing, typography)
- `src/services/` - Service pattern examples
- `src/hooks/` - Hook pattern examples
- `src/screens/` - Screen pattern examples
- `backend/` - Express + PostgreSQL backend

---

## Task

Build 18 additional features using 8 parallel Task agents. Each agent should create complete, production-ready code including screens, components, services, hooks, types, and backend endpoints.

### Agent Configuration

Launch these 8 agents IN PARALLEL (single message with 8 Task tool calls):

```
Agent 1: Fiat On/Off Ramp + AU Bloccard
Agent 2: Social Features + Username System
Agent 3: NFT Marketplace + Advanced Trading
Agent 4: Lending/Borrowing + Savings Goals
Agent 5: Multi-Sig + Enhanced Security + Privacy
Agent 6: Business Accounts + Merchant Tools
Agent 7: dApp Browser + DAO Management
Agent 8: Portfolio Analytics + Notifications + Themes + Metaverse
```

---

## Feature Specifications

### AGENT 1: Fiat On/Off Ramp + AU Bloccard

#### Fiat On/Off Ramp
Create complete fiat-to-crypto and crypto-to-fiat functionality.

**Screens to create:**
- `src/screens/fiat/FiatRampScreen.tsx` - Main hub
- `src/screens/fiat/BuyCryptoScreen.tsx` - Card payment flow
- `src/screens/fiat/SellCryptoScreen.tsx` - Sell to bank
- `src/screens/fiat/DCASetupScreen.tsx` - Dollar-cost averaging
- `src/screens/fiat/PaymentMethodsScreen.tsx` - Manage cards/banks

**Components:**
- `src/components/fiat/FiatAmountInput.tsx` - Currency conversion
- `src/components/fiat/PaymentMethodCard.tsx` - Saved method
- `src/components/fiat/DCAScheduleItem.tsx` - Recurring purchase
- `src/components/fiat/BankAccountCard.tsx` - Linked bank

**Services & Hooks:**
- `src/services/FiatRampService.ts` - MoonPay/Wyre integration
- `src/services/DCAService.ts` - Scheduled purchases
- `src/hooks/useFiatRamp.ts`
- `src/hooks/useDCA.ts`
- `src/hooks/usePaymentMethods.ts`

**Backend:**
- `backend/src/routes/fiat.ts` - Fiat endpoints
- Database tables: `payment_methods`, `dca_schedules`, `fiat_transactions`

#### AU Bloccard (Secured Crypto-Backed Debit Card)

**CRITICAL CONCEPT**: This is NOT a credit card. It's a secured debit card where crypto is collateral (like a CD account). Users deposit crypto, get spending limit based on 60% LTV.

**Key Logic:**
```typescript
// Spending limit calculation
const spendingLimit = collateralValueUSD * 0.6; // 60% LTV

// Health factor calculation
const healthFactor = (collateralValueUSD / amountSpent) * 100;
// >150% = Safe (green)
// 120-150% = Warning (yellow)
// <120% = Danger (red) - must add collateral
```

**Screens to create:**
- `src/screens/bloccard/AUBloccardScreen.tsx` - Main card management
- `src/screens/bloccard/BloccardSetupScreen.tsx` - Application flow
- `src/screens/bloccard/CollateralManageScreen.tsx` - Add/remove collateral
- `src/screens/bloccard/BloccardTransactionsScreen.tsx` - Spending history
- `src/screens/bloccard/BloccardSettingsScreen.tsx` - Limits, freeze

**Components:**
- `src/components/bloccard/BloccardVisual.tsx` - Virtual card (animated flip)
- `src/components/bloccard/CollateralMeter.tsx` - Health factor gauge
- `src/components/bloccard/SpendingLimitBar.tsx` - Available limit
- `src/components/bloccard/CollateralAssetItem.tsx` - Locked asset
- `src/components/bloccard/BloccardTransaction.tsx` - Purchase item

**Services & Hooks:**
- `src/services/BloccardService.ts` - Card operations
- `src/services/CollateralService.ts` - Health factor monitoring
- `src/hooks/useBloccard.ts`
- `src/hooks/useCollateral.ts`
- `src/hooks/useBloccardTransactions.ts`

**Backend:**
- `backend/src/routes/bloccard.ts`
- Database tables: `bloccard_accounts`, `bloccard_collateral`, `bloccard_transactions`, `collateral_price_history`

---

### AGENT 2: Social Features + Username System

#### Username System
Allow users to register usernames like `eoj.etrid` that resolve to their address.

**Screens:**
- `src/screens/social/UsernameSetupScreen.tsx` - Claim username
- `src/screens/social/ContactsScreen.tsx` - Address book
- `src/screens/social/SocialFeedScreen.tsx` - Activity feed
- `src/screens/social/BillSplitScreen.tsx` - Split expenses
- `src/screens/social/SocialRecoveryScreen.tsx` - Guardian setup

**Components:**
- `src/components/social/UsernameInput.tsx` - Availability checker
- `src/components/social/ContactCard.tsx` - Contact display
- `src/components/social/BillSplitItem.tsx` - Split request
- `src/components/social/GuardianCard.tsx` - Recovery guardian
- `src/components/social/ActivityFeedItem.tsx` - Social tx

**Services & Hooks:**
- `src/services/UsernameService.ts` - Register/resolve
- `src/services/ContactsService.ts` - Address book
- `src/services/BillSplitService.ts` - Group expenses
- `src/services/SocialRecoveryService.ts` - Guardians
- `src/hooks/useUsername.ts`
- `src/hooks/useContacts.ts`
- `src/hooks/useBillSplit.ts`
- `src/hooks/useSocialRecovery.ts`

**Backend:**
- `backend/src/routes/social.ts`
- Database tables: `usernames`, `contacts`, `bill_splits`, `social_recovery_guardians`

---

### AGENT 3: NFT Marketplace + Advanced Trading

#### NFT Marketplace & Gallery

**Screens:**
- `src/screens/nft/NFTGalleryScreen.tsx` - User collection
- `src/screens/nft/NFTMarketplaceScreen.tsx` - Browse/buy
- `src/screens/nft/NFTDetailScreen.tsx` - Single NFT
- `src/screens/nft/NFTMintScreen.tsx` - Create NFT
- `src/screens/nft/NFTAuctionScreen.tsx` - Auctions

**Components:**
- `src/components/nft/NFTCard.tsx` - Thumbnail with price
- `src/components/nft/NFTGrid.tsx` - Masonry layout
- `src/components/nft/NFTAttributeList.tsx` - Metadata
- `src/components/nft/AuctionTimer.tsx` - Countdown
- `src/components/nft/BidHistory.tsx` - Bid list

**Services & Hooks:**
- `src/services/NFTService.ts`
- `src/services/NFTMarketplaceService.ts`
- `src/services/NFTMintService.ts`
- `src/hooks/useNFTGallery.ts`
- `src/hooks/useNFTMarketplace.ts`
- `src/hooks/useNFTAuction.ts`

#### Advanced Trading

**Screens:**
- `src/screens/trading/TradingScreen.tsx` - Main interface
- `src/screens/trading/ChartScreen.tsx` - Full TradingView
- `src/screens/trading/OrdersScreen.tsx` - Open/filled
- `src/screens/trading/DCABotScreen.tsx` - DCA bot
- `src/screens/trading/AlertsScreen.tsx` - Price alerts

**Components:**
- `src/components/trading/TradingChart.tsx` - Charts
- `src/components/trading/OrderBook.tsx` - Depth
- `src/components/trading/OrderForm.tsx` - Limit/market/stop
- `src/components/trading/PositionCard.tsx` - Open position
- `src/components/trading/IndicatorSelector.tsx` - MA, RSI, MACD

**Services & Hooks:**
- `src/services/TradingService.ts`
- `src/services/ChartService.ts`
- `src/services/AlertService.ts`
- `src/hooks/useTrading.ts`
- `src/hooks/useChart.ts`
- `src/hooks/useOrders.ts`
- `src/hooks/usePriceAlerts.ts`

**Backend:**
- `backend/src/routes/nft.ts`
- `backend/src/routes/trading.ts`
- Database tables: `nft_listings`, `nft_bids`, `nft_sales`, `orders`, `price_alerts`, `dca_bots`

---

### AGENT 4: Lending/Borrowing + Savings Goals

#### Lending/Borrowing Marketplace

**Screens:**
- `src/screens/lending/LendingScreen.tsx` - Supply/borrow hub
- `src/screens/lending/SupplyScreen.tsx` - Deposit to earn
- `src/screens/lending/BorrowScreen.tsx` - Borrow against collateral
- `src/screens/lending/P2PLendingScreen.tsx` - Peer-to-peer
- `src/screens/lending/LoanDetailScreen.tsx` - Loan management

**Components:**
- `src/components/lending/APYCard.tsx` - Supply/borrow rates
- `src/components/lending/HealthFactorGauge.tsx` - Loan health
- `src/components/lending/CollateralSlider.tsx` - Adjust collateral
- `src/components/lending/LoanOffer.tsx` - P2P listing
- `src/components/lending/InterestAccrual.tsx` - Real-time interest

**Services & Hooks:**
- `src/services/LendingService.ts` - ReserveVaultWrapper integration
- `src/services/P2PLendingService.ts`
- `src/hooks/useLending.ts`
- `src/hooks/useHealthFactor.ts`
- `src/hooks/useP2PLending.ts`

#### Savings Goals

**Screens:**
- `src/screens/savings/SavingsGoalsScreen.tsx` - All goals
- `src/screens/savings/GoalDetailScreen.tsx` - Single goal
- `src/screens/savings/CreateGoalScreen.tsx` - New goal
- `src/screens/savings/AutoSaveRulesScreen.tsx` - Automation

**Components:**
- `src/components/savings/GoalCard.tsx` - Progress ring
- `src/components/savings/GoalProgressBar.tsx` - Visual progress
- `src/components/savings/MilestoneMarker.tsx` - Milestones
- `src/components/savings/AutoSaveRule.tsx` - Rule config

**Services & Hooks:**
- `src/services/SavingsGoalService.ts`
- `src/services/AutoSaveService.ts`
- `src/hooks/useSavingsGoals.ts`
- `src/hooks/useAutoSave.ts`

**Backend:**
- `backend/src/routes/lending.ts`
- `backend/src/routes/savings.ts`
- Database tables: `lending_positions`, `p2p_loans`, `savings_goals`, `goal_contributions`, `auto_save_rules`

---

### AGENT 5: Multi-Sig + Enhanced Security + Privacy

#### Multi-Signature Wallets

**Screens:**
- `src/screens/multisig/MultiSigScreen.tsx` - Wallets list
- `src/screens/multisig/CreateMultiSigScreen.tsx` - Setup wizard
- `src/screens/multisig/MultiSigDetailScreen.tsx` - Management
- `src/screens/multisig/PendingApprovalsScreen.tsx` - Awaiting signatures

**Components:**
- `src/components/multisig/MultiSigCard.tsx` - Wallet with signers
- `src/components/multisig/SignerAvatar.tsx` - Signer status
- `src/components/multisig/ApprovalItem.tsx` - Pending tx
- `src/components/multisig/SignatureProgress.tsx` - X of Y signed

**Services & Hooks:**
- `src/services/MultiSigService.ts`
- `src/hooks/useMultiSig.ts`
- `src/hooks/usePendingApprovals.ts`

#### Enhanced Security

**Screens:**
- `src/screens/security/SecurityCenterScreen.tsx` - Security hub
- `src/screens/security/WhitelistScreen.tsx` - Whitelisted addresses
- `src/screens/security/TimelockScreen.tsx` - Delayed withdrawals
- `src/screens/security/SpendingLimitsScreen.tsx` - Daily/weekly limits
- `src/screens/security/PanicModeScreen.tsx` - Emergency lockdown

**Components:**
- `src/components/security/SecurityScoreCard.tsx` - Overall rating
- `src/components/security/WhitelistItem.tsx` - Approved address
- `src/components/security/TimelockSetting.tsx` - Delay config
- `src/components/security/LimitSlider.tsx` - Limit adjuster

**Features to implement:**
- Duress PIN (shows fake low balance)
- Panic mode (freeze all + alert guardians)
- Inactivity auto-lock

**Services & Hooks:**
- `src/services/SecurityService.ts`
- `src/hooks/useSecurity.ts`
- `src/hooks/useWhitelist.ts`
- `src/hooks/usePanicMode.ts`

#### Privacy Features

**Screens:**
- `src/screens/privacy/PrivacyScreen.tsx` - Privacy settings
- `src/screens/privacy/StealthAddressScreen.tsx` - Generate stealth addresses
- `src/screens/privacy/CoinMixScreen.tsx` - Privacy mixing

**Components:**
- `src/components/privacy/StealthAddressCard.tsx` - One-time address
- `src/components/privacy/MixingProgress.tsx` - Anonymization status
- `src/components/privacy/PrivacyLevel.tsx` - Low/Medium/High

**Services & Hooks:**
- `src/services/PrivacyService.ts`
- `src/services/MetadataService.ts` - TX metadata scrubbing
- `src/hooks/usePrivacy.ts`
- `src/hooks/useStealthAddress.ts`
- `src/hooks/useCoinMix.ts`

**Backend:**
- `backend/src/routes/multisig.ts`
- `backend/src/routes/security.ts`
- `backend/src/routes/privacy.ts`
- Database tables: `multisig_wallets`, `multisig_signers`, `security_settings`, `whitelisted_addresses`, `stealth_addresses`

---

### AGENT 6: Business Accounts + Merchant Tools

#### Business Accounts

**Screens:**
- `src/screens/business/BusinessDashboardScreen.tsx` - Overview
- `src/screens/business/TeamManagementScreen.tsx` - Roles/permissions
- `src/screens/business/InvoicingScreen.tsx` - Create invoices
- `src/screens/business/PayrollScreen.tsx` - Batch payments
- `src/screens/business/ExpenseTrackingScreen.tsx` - Expense reports

**Components:**
- `src/components/business/TeamMemberCard.tsx` - Member with role
- `src/components/business/InvoiceItem.tsx` - Invoice summary
- `src/components/business/PayrollBatch.tsx` - Batch payment
- `src/components/business/ExpenseChart.tsx` - Category breakdown

**Services & Hooks:**
- `src/services/BusinessService.ts`
- `src/services/InvoiceService.ts`
- `src/services/PayrollService.ts`
- `src/hooks/useBusiness.ts`
- `src/hooks/useTeam.ts`
- `src/hooks/useInvoices.ts`
- `src/hooks/usePayroll.ts`

#### Merchant Tools

**Screens:**
- `src/screens/merchant/MerchantDashboardScreen.tsx` - Sales overview
- `src/screens/merchant/POSScreen.tsx` - Point of sale
- `src/screens/merchant/PaymentLinksScreen.tsx` - Link generator
- `src/screens/merchant/ProductCatalogScreen.tsx` - Products
- `src/screens/merchant/RefundsScreen.tsx` - Refund management

**Components:**
- `src/components/merchant/POSKeypad.tsx` - Amount entry
- `src/components/merchant/PaymentLinkCard.tsx` - Shareable link
- `src/components/merchant/ProductItem.tsx` - Catalog item
- `src/components/merchant/RefundItem.tsx` - Refund request

**Services & Hooks:**
- `src/services/MerchantService.ts`
- `src/services/POSService.ts`
- `src/services/PaymentLinkService.ts`
- `src/hooks/useMerchant.ts`
- `src/hooks/usePOS.ts`
- `src/hooks/usePaymentLinks.ts`

**Backend:**
- `backend/src/routes/business.ts`
- `backend/src/routes/merchant.ts`
- Database tables: `business_accounts`, `team_members`, `invoices`, `payroll_batches`, `merchant_accounts`, `products`, `payment_links`

---

### AGENT 7: dApp Browser + DAO Management

#### dApp Browser

**Screens:**
- `src/screens/dapp/DAppBrowserScreen.tsx` - WebView browser
- `src/screens/dapp/DAppListScreen.tsx` - Popular dApps
- `src/screens/dapp/BookmarksScreen.tsx` - Saved dApps
- `src/screens/dapp/WalletConnectScreen.tsx` - Active sessions

**Components:**
- `src/components/dapp/DAppCard.tsx` - dApp with icon
- `src/components/dapp/BrowserNavBar.tsx` - URL, back/forward
- `src/components/dapp/WalletConnectModal.tsx` - Connection approval
- `src/components/dapp/TransactionApproval.tsx` - Sign tx modal

**Features:**
- WebView with injected web3 provider
- WalletConnect v2 pairing
- Transaction signing flow
- dApp permissions

**Services & Hooks:**
- `src/services/DAppBrowserService.ts` - Web3 injection
- `src/services/WalletConnectService.ts` - WC v2
- `src/hooks/useDAppBrowser.ts`
- `src/hooks/useWalletConnect.ts`

#### DAO Management

**Screens:**
- `src/screens/dao/DAOListScreen.tsx` - User's DAOs
- `src/screens/dao/DAODashboardScreen.tsx` - Single DAO
- `src/screens/dao/CreateDAOScreen.tsx` - Creation wizard
- `src/screens/dao/DAOProposalsScreen.tsx` - Proposals
- `src/screens/dao/DAOTreasuryScreen.tsx` - Treasury

**Components:**
- `src/components/dao/DAOCard.tsx` - DAO summary
- `src/components/dao/ProposalCard.tsx` - With voting
- `src/components/dao/TreasuryChart.tsx` - Asset allocation
- `src/components/dao/MemberList.tsx` - Members

**Services & Hooks:**
- `src/services/DAOService.ts`
- `src/services/DAOProposalService.ts`
- `src/hooks/useDAOs.ts`
- `src/hooks/useDAOProposals.ts`
- `src/hooks/useDAOTreasury.ts`

**Backend:**
- `backend/src/routes/dapp.ts`
- `backend/src/routes/dao.ts`
- Database tables: `dapp_bookmarks`, `walletconnect_sessions`, `daos`, `dao_members`, `dao_proposals`, `dao_treasury`

---

### AGENT 8: Portfolio Analytics + Notifications + Themes + Metaverse

#### Portfolio Analytics

**Screens:**
- `src/screens/analytics/AnalyticsDashboardScreen.tsx` - Hub
- `src/screens/analytics/RiskMetricsScreen.tsx` - Risk analysis
- `src/screens/analytics/TaxReportScreen.tsx` - Tax harvesting
- `src/screens/analytics/PerformanceScreen.tsx` - Historical

**Components:**
- `src/components/analytics/RiskScoreCard.tsx` - Risk rating
- `src/components/analytics/DiversificationChart.tsx` - Allocation
- `src/components/analytics/TaxLossCard.tsx` - Harvesting opportunity
- `src/components/analytics/PerformanceChart.tsx` - P&L
- `src/components/analytics/CorrelationMatrix.tsx` - Correlations

**Features:**
- Sharpe ratio, volatility, beta
- Diversification score
- Tax loss harvesting suggestions

**Services & Hooks:**
- `src/services/AnalyticsService.ts`
- `src/services/TaxService.ts`
- `src/hooks/useAnalytics.ts`
- `src/hooks/useRiskMetrics.ts`
- `src/hooks/useTaxReport.ts`

#### Smart Notifications

**Screens:**
- `src/screens/notifications/NotificationCenterScreen.tsx` - All notifications
- `src/screens/notifications/AlertSettingsScreen.tsx` - Configure

**Components:**
- `src/components/notifications/NotificationItem.tsx` - Single notification
- `src/components/notifications/AlertToggle.tsx` - Enable/disable
- `src/components/notifications/ThresholdInput.tsx` - Thresholds

**Alert Types:**
- Price alerts, whale alerts, governance alerts, staking alerts, security alerts

**Services & Hooks:**
- `src/services/NotificationService.ts`
- `src/services/AlertEngine.ts`
- `src/hooks/useNotifications.ts`
- `src/hooks/useAlertSettings.ts`

#### Themes & Customization

**Screens:**
- `src/screens/settings/ThemeSettingsScreen.tsx` - Theme selection
- `src/screens/settings/CustomizeScreen.tsx` - Widget arrangement

**Components:**
- `src/components/settings/ThemePreview.tsx` - Preview card
- `src/components/settings/ColorPicker.tsx` - Custom accent
- `src/components/settings/WidgetArranger.tsx` - Drag-drop

**Themes:** Light, Dark, Midnight Blue, Forest Green, Custom

**Services & Hooks:**
- `src/services/ThemeService.ts`
- `src/hooks/useTheme.ts`
- `src/hooks/useCustomization.ts`

#### Metaverse Integration

**Screens:**
- `src/screens/metaverse/MetaverseHubScreen.tsx` - Entry point
- `src/screens/metaverse/VirtualGalleryScreen.tsx` - NFT gallery
- `src/screens/metaverse/LandNFTsScreen.tsx` - Virtual land
- `src/screens/metaverse/WearablesScreen.tsx` - Avatar items
- `src/screens/metaverse/EventsScreen.tsx` - Virtual events

**Components:**
- `src/components/metaverse/GalleryViewer.tsx` - 3D view
- `src/components/metaverse/LandPlot.tsx` - Land NFT
- `src/components/metaverse/WearableItem.tsx` - Wearable
- `src/components/metaverse/EventCard.tsx` - Event

**Services & Hooks:**
- `src/services/MetaverseService.ts`
- `src/hooks/useMetaverse.ts`
- `src/hooks/useVirtualGallery.ts`
- `src/hooks/useLandNFTs.ts`

**Backend:**
- `backend/src/routes/analytics.ts`
- `backend/src/routes/notifications.ts`
- `backend/src/routes/metaverse.ts`
- Database tables: `portfolio_snapshots`, `tax_lots`, `notifications`, `alert_settings`, `user_preferences`, `virtual_galleries`

---

## Design System Reference

Use these from `src/theme/theme.ts`:

```typescript
colors: {
  primary: '#6C5CE7',      // Purple - Etrid brand
  secondary: '#00B894',    // Green - success
  accent: '#FD79A8',       // Pink - highlights
  background: '#FFFFFF',
  surface: '#F8F9FA',
  text: '#2D3436',
  textSecondary: '#636E72',
  error: '#FF7675',
  warning: '#FDCB6E',
  success: '#00B894',
}

spacing: { xs: 4, sm: 8, md: 16, lg: 24, xl: 32, xxl: 48 }
borderRadius: { sm: 4, md: 8, lg: 16, xl: 24, full: 9999 }
```

---

## Execution Instructions

1. **Launch all 8 agents in a SINGLE message** with 8 Task tool calls
2. Each agent should use `subagent_type: "general-purpose"`
3. Each agent creates complete, working code
4. Follow existing patterns in the codebase
5. Include TypeScript types for everything
6. Add proper error handling
7. Include loading states
8. Use React Native Paper components where appropriate
9. Follow the color scheme and spacing from theme

---

## Expected Output

Each agent should report:
- Number of files created
- Total lines of code
- List of screens, components, services, hooks
- Any dependencies that need to be added to package.json
- Database migrations needed

**Total expected**: ~75 screens, ~120 components, ~40 hooks, ~25 services, ~50 database tables, 40,000-50,000 lines of code

---

## Navigation Integration

After all agents complete, update `src/navigation/RootNavigator.tsx` to include new screens in appropriate tab stacks:

- **Home Tab**: Dashboard, Notifications
- **Portfolio Tab**: Analytics, NFT Gallery
- **Trade Tab**: Trading, NFT Marketplace
- **DeFi Tab**: Lending, Savings, Staking
- **More Tab**: Business, Merchant, DAO, dApp Browser, Settings, Security, Privacy, Metaverse, Bloccard

---

## Final Steps

After all features are built:
1. Update `package.json` with any new dependencies
2. Run `npm install`
3. Create database migration file with all new tables
4. Test each feature screen renders without errors
5. Commit all changes

---

**START BUILDING NOW - Launch all 8 agents in parallel!**
