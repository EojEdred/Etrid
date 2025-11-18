# Ëtrid Mobile Wallet Backend API

Production-ready Node.js/TypeScript backend for the Ëtrid Mobile DeFi Wallet.

## Features

- RESTful API with Express.js
- PostgreSQL database with connection pooling
- Redis caching for performance
- JWT authentication with signature verification
- Rate limiting and security middleware
- Blockchain integration with Polkadot.js
- ATM withdrawal integration (Coinme, Bitcoin Depot, CoinFlip)
- GPU rental marketplace (Vast.ai, RunPod)
- Cross-chain bridge support
- Staking and governance features
- Push notifications via Expo
- Email notifications via SendGrid
- SMS notifications via Twilio
- Comprehensive error handling and logging
- Docker containerization
- TypeScript with strict type checking

## Tech Stack

- **Runtime**: Node.js 18+
- **Language**: TypeScript 5.3
- **Framework**: Express.js 4.18
- **Database**: PostgreSQL 14
- **Cache**: Redis 7
- **Authentication**: JWT + Polkadot signature verification
- **Blockchain**: Polkadot.js API
- **Testing**: Jest
- **Containerization**: Docker & Docker Compose

## Project Structure

```
backend/
├── src/
│   ├── config/           # Configuration files
│   ├── database/         # Database client and schema
│   ├── middleware/       # Express middleware (auth, validation, errors)
│   ├── repositories/     # Database repositories
│   ├── routes/           # API route handlers
│   ├── services/         # Business logic services
│   ├── types/            # TypeScript type definitions
│   ├── utils/            # Utility functions
│   └── server.ts         # Main server file
├── scripts/              # Database migration scripts
├── logs/                 # Application logs
├── Dockerfile            # Docker configuration
├── docker-compose.yml    # Multi-container setup
└── package.json          # Dependencies
```

## Installation

### Prerequisites

- Node.js 18+ and npm
- PostgreSQL 14+
- Redis 7+
- Docker & Docker Compose (optional)

### Local Development

1. **Clone the repository**

```bash
cd /Users/macbook/Desktop/etrid/apps/wallet-mobile/backend
```

2. **Install dependencies**

```bash
npm install
```

3. **Configure environment variables**

```bash
cp .env.example .env
# Edit .env with your configuration
```

4. **Set up database**

```bash
# Create database
createdb etrid_wallet

# Run migrations
psql -d etrid_wallet -f src/database/schema.sql
```

5. **Start development server**

```bash
npm run dev
```

The API will be available at `http://localhost:3000`

### Docker Deployment

1. **Configure environment**

```bash
cp .env.example .env
# Edit .env with production values
```

2. **Start all services**

```bash
docker-compose up -d
```

3. **Check service health**

```bash
docker-compose ps
docker-compose logs -f api
```

4. **Stop services**

```bash
docker-compose down
```

## API Endpoints

### Authentication

- `GET /api/v1/auth/nonce/:address` - Get nonce for signing
- `POST /api/v1/auth/login` - Login with signature
- `POST /api/v1/auth/refresh` - Refresh JWT token
- `POST /api/v1/auth/logout` - Logout

### Accounts

- `GET /api/v1/accounts/:address/balance` - Get balance
- `GET /api/v1/accounts/:address/portfolio` - Get portfolio
- `GET /api/v1/accounts/:address/transactions` - Get transaction history
- `POST /api/v1/accounts/:address/transfer` - Submit transfer
- `GET /api/v1/accounts/:address/activity` - Get activity summary
- `GET /api/v1/accounts/:address/stats` - Get detailed stats

### Staking

- `GET /api/v1/staking/validators` - Get validator list
- `GET /api/v1/staking/validators/:address` - Get validator details
- `GET /api/v1/staking/:address/positions` - Get staking positions
- `GET /api/v1/staking/:address/rewards` - Get rewards history
- `POST /api/v1/staking/:address/stake` - Stake tokens
- `POST /api/v1/staking/:address/unstake` - Unstake tokens

### Governance

- `GET /api/v1/governance/proposals` - Get active proposals
- `GET /api/v1/governance/proposals/:id` - Get proposal details
- `POST /api/v1/governance/proposals/:id/vote` - Submit vote
- `GET /api/v1/governance/:address/votes` - Get vote history

### ATM Withdrawals

- `GET /api/v1/atm/locations` - Find nearby ATMs
- `POST /api/v1/atm/withdraw` - Create withdrawal
- `GET /api/v1/atm/withdrawals/:code/status` - Check status
- `GET /api/v1/atm/withdrawals` - Get withdrawal history

### Bridge

- `GET /api/v1/bridge/chains` - Get supported chains
- `GET /api/v1/bridge/rate` - Get exchange rate
- `POST /api/v1/bridge/transfer` - Submit bridge transfer
- `GET /api/v1/bridge/transfers/:id/status` - Get transfer status
- `GET /api/v1/bridge/transfers` - Get transfer history

### GPU Rentals

- `GET /api/v1/gpu/search` - Search available GPUs
- `GET /api/v1/gpu/:id` - Get GPU details
- `POST /api/v1/gpu/:id/rent` - Rent GPU
- `GET /api/v1/gpu/rentals` - Get rental history
- `POST /api/v1/gpu/rentals/:id/terminate` - Terminate rental

### Notifications

- `GET /api/v1/notifications` - Get notifications
- `GET /api/v1/notifications/unread` - Get unread count
- `POST /api/v1/notifications/:id/read` - Mark as read
- `POST /api/v1/notifications/read-all` - Mark all as read

## Environment Variables

See `.env.example` for all available configuration options.

Critical variables:

```bash
# Database
DB_HOST=localhost
DB_PORT=5432
DB_NAME=etrid_wallet
DB_USER=postgres
DB_PASSWORD=your_secure_password

# Redis
REDIS_URL=redis://localhost:6379

# JWT
JWT_SECRET=your_jwt_secret_key
JWT_REFRESH_SECRET=your_refresh_secret

# Blockchain
FLARECHAIN_WS_URL=wss://flarechain.etrid.io
FLARECHAIN_HTTP_URL=https://rpc.etrid.io
```

## Testing

```bash
# Run all tests
npm test

# Run tests in watch mode
npm run test:watch

# Generate coverage report
npm test -- --coverage
```

## Scripts

```bash
npm run dev          # Start development server with hot reload
npm run build        # Build TypeScript to JavaScript
npm start            # Start production server
npm test             # Run tests
npm run lint         # Lint code
npm run lint:fix     # Fix linting issues
npm run migrate      # Run database migrations
```

## Database Schema

### Key Tables

- **users** - User accounts and KYC data
- **transactions** - Transaction history
- **staking_positions** - Active stakes
- **governance_votes** - Voting records
- **atm_withdrawals** - ATM withdrawal requests
- **gpu_rentals** - GPU rental instances
- **bridge_transfers** - Cross-chain transfers
- **validators** - Validator information
- **proposals** - Governance proposals
- **notifications** - User notifications

See `src/database/schema.sql` for complete schema.

## Security Features

- Helmet.js for HTTP header security
- CORS configuration
- Rate limiting (100 requests per 15 minutes)
- JWT token authentication
- Polkadot signature verification
- SQL injection prevention (parameterized queries)
- Input validation with Joi
- Bcrypt password hashing
- Environment variable validation

## Performance Optimizations

- Redis caching for frequently accessed data
- Database connection pooling
- Indexed database queries
- Gzip compression
- Lazy loading of services
- Efficient pagination

## Monitoring & Logging

- Winston logger with file and console transports
- Error tracking and stack traces
- Request/response logging
- Database query performance monitoring
- Health check endpoint (`/health`)
- Structured JSON logs for production

## Production Deployment

### Docker (Recommended)

```bash
# Build and start
docker-compose up -d --build

# View logs
docker-compose logs -f

# Scale API instances
docker-compose up -d --scale api=3
```

### Manual Deployment

```bash
# Build
npm run build

# Set environment
export NODE_ENV=production

# Start
npm start
```

### Kubernetes

Deploy using the provided Kubernetes manifests (coming soon).

## API Response Format

All API responses follow this structure:

```json
{
  "success": true,
  "data": { ... },
  "timestamp": "2025-11-18T12:00:00.000Z"
}
```

Error responses:

```json
{
  "success": false,
  "error": {
    "code": "ERROR_CODE",
    "message": "Human readable message",
    "details": { ... }
  },
  "timestamp": "2025-11-18T12:00:00.000Z"
}
```

## Rate Limiting

- **Global**: 100 requests per 15 minutes per IP
- **Authentication**: 5 login attempts per hour
- **ATM**: 10 withdrawals per day per user
- **Bridge**: 20 transfers per hour per user

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Write tests
5. Submit a pull request

## License

MIT License - see LICENSE file for details

## Support

For issues and questions:
- GitHub Issues: https://github.com/etrid/wallet-backend/issues
- Email: support@etrid.io
- Discord: https://discord.gg/etrid

## Changelog

See CHANGELOG.md for version history.
