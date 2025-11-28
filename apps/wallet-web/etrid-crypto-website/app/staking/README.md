# Ëtrid Protocol Nominator Portal

A comprehensive staking interface for delegated staking in the Ëtrid Protocol ecosystem. This portal enables users to discover validators, create nominations, track rewards, and calculate potential earnings.

## Overview

The Nominator Portal provides a complete solution for users to participate in Ëtrid's proof-of-stake consensus mechanism through delegated staking. Users can select validators, manage their stakes, and monitor performance without running validator infrastructure.

## Features

### 1. Nominator Dashboard (`nominator-dashboard.tsx`)

**Purpose**: Central hub for managing all staking activities and monitoring portfolio performance.

**Key Features**:
- **Portfolio Overview**: Real-time statistics including total staked, active nominations, total rewards, and estimated APY
- **Active Nominations**: Comprehensive view of all current validator delegations with performance metrics
- **Performance Alerts**: Automated notifications for validator issues (low uptime, commission changes, etc.)
- **Quick Actions**: Fast access to validator browser, rewards tracker, and APY calculator
- **Nomination Status**: Track active, waiting, and inactive nominations with visual indicators
- **Detailed Metrics**: Per-validator breakdown showing stake amount, APY, commission, rewards, and uptime

**User Flow**:
1. User lands on dashboard to see portfolio overview
2. Review performance alerts for any validator issues
3. Monitor active nominations and rewards accumulation
4. Access quick actions to browse validators or track rewards
5. Manage individual nominations (add stake, unstake, view details)

### 2. Validator Browser (`validator-browser.tsx`)

**Purpose**: Discover and compare validators for optimal delegation decisions.

**Key Features**:
- **Advanced Search**: Find validators by name or address
- **Multi-criteria Filtering**:
  - Minimum APY threshold
  - Maximum commission rate
  - Minimum uptime percentage
  - Verified validators only
  - Risk level (low/medium/high)
- **Flexible Sorting**: Sort by APY, commission, total stake, uptime, or nominator count
- **Validator Comparison**: Select multiple validators to compare side-by-side
- **Risk Assessment**: Color-coded risk scores based on validator metrics
- **Detailed Validator Cards**: Complete validator information including:
  - APY and commission rates
  - Total stake and nominator count
  - Uptime statistics with visual progress bars
  - Verification status
  - Performance badges (high APY, low commission, high uptime)

**User Flow**:
1. Browse available validators with default sorting
2. Apply filters to narrow down options (APY, commission, uptime, risk)
3. Search for specific validators by name or address
4. Select validators for comparison (up to 16)
5. Review detailed metrics for each validator
6. Navigate to nomination manager with pre-selected validators
7. View validator details or explore on blockchain explorer

### 3. Nomination Manager (`nomination-manager.tsx`)

**Purpose**: Create new nominations and manage existing stakes.

**Key Features**:
- **Dual Interface**:
  - **New Nomination Tab**: Create fresh delegations
  - **Manage Existing Tab**: Modify current stakes
- **Multi-validator Support**: Delegate to up to 16 validators simultaneously
- **Flexible Staking**:
  - Add stake to existing nominations
  - Unstake with 28-day unbonding period
  - Rebond unbonding tokens
- **Pending Actions Tracker**: Monitor queued transactions and upcoming executions
- **Unbonding Dashboard**: Track tokens in unbonding period with countdown timers
- **Staking Summary**: Real-time calculation of total staked, average APY, and total rewards
- **Educational Alerts**: Inline guidance about staking mechanics and requirements

**User Flow**:

**Creating New Nomination**:
1. Navigate to "New Nomination" tab
2. Add validators using the nomination form (opens validator selector)
3. Allocate stake amounts per validator (minimum 10 ETD)
4. Review summary showing total stake and estimated APY
5. Submit nomination (becomes active in next era ~6 hours)

**Managing Existing Nominations**:
1. Switch to "Manage Existing" tab
2. Review current nominations with performance metrics
3. Add more stake to performing validators
4. Unstake from underperforming validators (28-day unbonding)
5. Track unbonding tokens with visual progress indicators
6. View aggregated staking summary

### 4. Rewards Tracker (`rewards-tracker.tsx`)

**Purpose**: Monitor reward history and analyze staking performance.

**Key Features**:
- **Flexible Timeframes**: View rewards over 7 days, 30 days, 90 days, or all-time
- **Validator Filtering**: Focus on specific validator performance
- **Summary Statistics**:
  - Total rewards earned
  - Average APY
  - Best performing validator
  - Total reward events
- **Interactive Charts** (via RewardChart component):
  - Daily rewards accumulation
  - Cumulative earnings over time
  - APY trends
  - Per-validator breakdown
- **Detailed History**: Transaction-level reward log with:
  - Date and era information
  - Validator details
  - Reward amount and APY
  - Blockchain explorer links
- **Validator Performance Breakdown**: Aggregate rewards per validator
- **Projected Earnings**: Future rewards estimate based on current performance
- **Export Functionality**: Download reward history for tax reporting

**User Flow**:
1. Select timeframe (7d/30d/90d/all-time)
2. Optionally filter by specific validator
3. Review summary statistics at a glance
4. Analyze reward trends with interactive charts
5. Examine validator performance breakdown
6. Browse detailed reward history
7. Review projected future earnings
8. Export data for external analysis

### 5. Validator Card Component (`validator-card.tsx`)

**Purpose**: Reusable component displaying comprehensive validator information.

**Key Features**:
- **Status Indicators**:
  - Verification badge
  - Active/waiting/inactive status
  - Risk score with color coding
- **Key Metrics Display**:
  - APY percentage with visual prominence
  - Commission rate
  - Nominator count
  - Total stake
- **Performance Indicators**:
  - Uptime percentage with progress bar
  - Blocks produced
  - Last reward amount
- **Performance Badges**: Automatic badges for high APY, low commission, high uptime
- **Alert Badges**: Warning for low uptime or other issues
- **Action Buttons**:
  - Nominate validator
  - View detailed information
  - Open in blockchain explorer
- **Selection Support**: Checkbox for multi-validator comparison

### 6. Nomination Form Component (`nomination-form.tsx`)

**Purpose**: Streamlined interface for creating new delegations.

**Key Features**:
- **Balance Display**: Real-time available and remaining balance
- **Validator Selection**:
  - Modal dialog with searchable validator list
  - Verified validator indicators
  - APY and commission preview
  - Support for up to 16 validators
- **Stake Allocation**:
  - Individual amount per validator
  - "Distribute Evenly" quick action
  - Minimum stake validation (10 ETD)
  - Balance overflow prevention
- **Live Summary**:
  - Total stake calculation
  - Weighted average APY
  - Stake distribution visualization
- **Projected Earnings**: Estimated returns for 30/90/365 days
- **Validation & Warnings**:
  - Minimum stake requirements
  - Balance overflow alerts
  - Educational tooltips
- **Form State Management**: Persistent selections during session

### 7. Reward Chart Component (`reward-chart.tsx`)

**Purpose**: Interactive data visualizations for reward analysis.

**Key Features**:
- **Four Chart Types**:
  1. **Daily Rewards**: Area chart showing day-by-day earnings
  2. **Cumulative**: Line chart of total accumulated rewards
  3. **APY Trend**: Historical APY performance over time
  4. **Breakdown**: Stacked bar chart showing per-validator contributions
- **Dynamic Data**: Adapts to selected timeframe (7d/30d/90d/all-time)
- **Validator Filtering**: Show all validators or focus on specific one
- **Interactive Tooltips**: Hover for detailed data points
- **Responsive Design**: Scales for mobile and desktop viewing
- **Color-coded Validators**: Easy visual distinction between validators
- **Professional Styling**: Gradient fills and smooth animations

### 8. APY Calculator Component (`apy-calculator.tsx`)

**Purpose**: Calculate potential staking returns with customizable parameters.

**Key Features**:
- **Customizable Inputs**:
  - Stake amount (ETD)
  - Expected APY (slider 5-20%)
  - Validator commission (slider 0-20%)
  - Duration (1-10 years)
  - Additional stake amount and frequency
- **Quick Returns Display**:
  - Daily, weekly, monthly, quarterly, yearly projections
- **Long-term Projections**:
  - 1-year, 2-year, 5-year compounded returns
  - Principal vs. earnings breakdown
- **Compound Interest Calculations**: Daily compounding with additional stakes
- **Additional Stake Options**:
  - Daily, weekly, monthly, or yearly contributions
  - Automatic calculation of total growth
- **Visual Growth Projection**:
  - Area chart showing total value over time
  - Separate visualization of earnings vs. principal
  - Month-by-month breakdown
- **Effective APY Display**: Real APY after commission deduction
- **Interactive Sliders**: Smooth, real-time calculation updates

## Technical Architecture

### Tech Stack

- **React 18+**: Modern hooks-based components
- **TypeScript**: Full type safety and IntelliSense support
- **Next.js 15**: App router with server and client components
- **Tailwind CSS**: Utility-first styling with custom design system
- **Radix UI**: Accessible, unstyled component primitives
- **Recharts**: Declarative charting library for data visualization
- **Lucide React**: Consistent icon system

### Component Structure

```
app/staking/
├── nominator-dashboard.tsx    # Main dashboard page
├── validator-browser.tsx      # Validator discovery and filtering
├── nomination-manager.tsx     # Create and manage nominations
├── rewards-tracker.tsx        # Reward history and analytics
└── README.md                  # This documentation

components/staking/
├── validator-card.tsx         # Validator display component
├── nomination-form.tsx        # Nomination creation form
├── reward-chart.tsx          # Interactive reward charts
└── apy-calculator.tsx        # Staking calculator component
```

### State Management

- **Local State**: React useState for component-level data
- **Derived State**: useMemo for calculated values (APY, totals, etc.)
- **Form State**: Controlled inputs with validation
- **Mock Data**: Currently uses static data; ready for API integration

### Styling Approach

- **Mobile-First**: Responsive design starting from mobile breakpoints
- **Dark Mode**: Full support via next-themes
- **Design System**: Consistent spacing, colors, and typography
- **Accessibility**: ARIA labels, keyboard navigation, screen reader support

## Integration Points

### Required API Endpoints (To Be Implemented)

```typescript
// Validator endpoints
GET /api/validators                    // List all validators
GET /api/validators/:address           // Validator details
GET /api/validators/:address/stats     // Performance statistics

// Nomination endpoints
POST /api/nominations                  // Create nomination
GET /api/nominations/:address          // User's nominations
PUT /api/nominations/:id/stake         // Add stake
PUT /api/nominations/:id/unstake       // Remove stake
DELETE /api/nominations/:id            // Cancel nomination

// Rewards endpoints
GET /api/rewards/:address              // User's reward history
GET /api/rewards/:address/summary      // Aggregated statistics
GET /api/rewards/:address/projections  // Future earnings estimates

// User endpoints
GET /api/users/:address/balance        // Available balance
GET /api/users/:address/staking-info   // Staking summary
```

### Blockchain Integration

The portal will integrate with:
- **Polkadot.js API**: For Substrate-based staking operations
- **Wallet Extensions**: Polkadot.js extension, Talisman, SubWallet
- **On-chain Queries**: Real-time validator and nomination data
- **Transaction Signing**: Secure extrinsic submission

### Data Models

```typescript
interface Validator {
  id: string;
  name: string;
  address: string;
  apy: number;
  commission: number;
  totalStake: string;
  nominatorCount: number;
  uptime: number;
  blocksProduced: number;
  isVerified: boolean;
  status: 'active' | 'waiting' | 'inactive';
  lastReward: string;
  riskScore: 'low' | 'medium' | 'high';
}

interface Nomination {
  id: string;
  validatorId: string;
  validatorName: string;
  validatorAddress: string;
  stakedAmount: string;
  apy: number;
  commission: number;
  status: 'active' | 'waiting' | 'unbonding';
  unbondingPeriod?: string;
  rewards: string;
  uptime: number;
}

interface RewardEntry {
  id: string;
  date: string;
  era: number;
  validatorName: string;
  validatorAddress: string;
  amount: string;
  apy: number;
  type: 'staking' | 'commission';
  txHash: string;
}
```

## User Experience Design

### Design Principles

1. **Clarity**: Information hierarchy emphasizes key metrics
2. **Guidance**: Inline help and educational tooltips throughout
3. **Transparency**: Full disclosure of fees, risks, and timelines
4. **Efficiency**: Quick actions and batch operations
5. **Trust**: Verification badges and risk assessments

### Mobile Optimization

- Touch-friendly tap targets (minimum 44x44px)
- Simplified navigation with bottom sheets
- Swipeable cards for validator browsing
- Collapsible sections to reduce scrolling
- Responsive charts that adapt to screen size

### Error Handling

- Form validation with inline error messages
- Network error recovery with retry mechanisms
- Transaction failure handling with clear explanations
- Loading states for async operations
- Empty states with actionable guidance

## Security Considerations

### Best Practices Implemented

- **No Private Keys**: Never store or transmit private keys
- **Wallet Integration**: Use secure browser extension signing
- **Input Validation**: Client-side validation for all user inputs
- **Balance Checks**: Prevent over-staking beyond available balance
- **Risk Warnings**: Clear communication of unbonding periods and risks

### Future Enhancements

- Transaction simulation before signing
- Multi-signature support for institutional users
- Hardware wallet integration (Ledger, Trezor)
- Gas estimation for transaction costs
- Slippage protection for dynamic APY calculations

## Performance Optimization

### Current Optimizations

- **Memoization**: useMemo for expensive calculations
- **Lazy Loading**: Code splitting for chart libraries
- **Debounced Search**: Reduced re-renders during typing
- **Virtualization**: Ready for large validator lists (100+)
- **Optimistic Updates**: Instant UI feedback before confirmation

### Metrics to Monitor

- Time to Interactive (TTI) < 3 seconds
- First Contentful Paint (FCP) < 1.5 seconds
- Largest Contentful Paint (LCP) < 2.5 seconds
- Cumulative Layout Shift (CLS) < 0.1

## Accessibility Features

- **ARIA Labels**: Descriptive labels for screen readers
- **Keyboard Navigation**: Full keyboard support (Tab, Enter, Escape)
- **Focus Management**: Visible focus indicators
- **Color Contrast**: WCAG AA compliance (4.5:1 minimum)
- **Semantic HTML**: Proper heading hierarchy and landmarks
- **Alt Text**: Descriptive text for all icons and images

## Future Roadmap

### Phase 2 Features

- [ ] Multi-currency support (USD, EUR price display)
- [ ] Push notifications for reward payments
- [ ] Validator reputation scoring system
- [ ] Social features (validator reviews, ratings)
- [ ] Portfolio analytics dashboard
- [ ] Tax reporting integration
- [ ] Staking pool support
- [ ] Liquid staking derivatives

### Phase 3 Features

- [ ] Governance voting interface
- [ ] Validator operator tools
- [ ] Advanced charting (candlestick, heatmaps)
- [ ] Automated rebalancing strategies
- [ ] Risk-adjusted portfolio optimization
- [ ] Cross-chain staking support
- [ ] API for third-party integrations

## Development Guidelines

### Running Locally

```bash
# Install dependencies
npm install

# Start development server
npm run dev

# Build for production
npm run build

# Start production server
npm start
```

### Adding New Features

1. Create component in appropriate directory
2. Add TypeScript interfaces for data models
3. Implement responsive styling with Tailwind
4. Add accessibility attributes
5. Write unit tests (when testing framework added)
6. Update this README with feature documentation

### Code Style

- Use functional components with hooks
- Prefer composition over inheritance
- Keep components focused and single-purpose
- Extract reusable logic into custom hooks
- Use TypeScript strict mode
- Follow existing naming conventions

## Testing Strategy

### Recommended Testing Approach

```typescript
// Unit Tests (Jest + React Testing Library)
- Component rendering
- User interactions (clicks, inputs)
- Calculation accuracy (APY, rewards)
- Form validation logic

// Integration Tests
- User flows (browse → select → nominate)
- API integration
- Wallet connection
- Transaction signing

// E2E Tests (Playwright/Cypress)
- Complete nomination workflow
- Reward tracking accuracy
- Multi-validator delegation
- Error recovery scenarios
```

## Contributing

When contributing to the Nominator Portal:

1. Follow the component structure patterns
2. Maintain TypeScript type safety
3. Ensure mobile responsiveness
4. Add accessibility features
5. Update documentation
6. Test across different browsers

## License

Part of the Ëtrid Protocol - Licensed under [Project License]

## Support

For questions or issues:
- Documentation: [Link to docs]
- Discord: [Community link]
- GitHub Issues: [Repository link]

---

**Last Updated**: October 2025
**Version**: 1.0.0
**Maintained By**: Ëtrid Protocol Team
