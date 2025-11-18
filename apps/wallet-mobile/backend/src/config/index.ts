import dotenv from 'dotenv';
import path from 'path';

// Load environment variables
dotenv.config();

interface Config {
  env: string;
  port: number;
  apiVersion: string;
  database: {
    host: string;
    port: number;
    name: string;
    user: string;
    password: string;
    maxConnections: number;
  };
  redis: {
    url: string;
    password?: string;
    db: number;
  };
  jwt: {
    secret: string;
    expiresIn: string;
    refreshSecret: string;
    refreshExpiresIn: string;
  };
  blockchain: {
    wsUrl: string;
    httpUrl: string;
    syncInterval: number;
  };
  atm: {
    coinme: {
      apiKey: string;
      apiSecret: string;
      baseUrl: string;
    };
    bitcoinDepot: {
      apiKey: string;
      baseUrl: string;
    };
    coinflip: {
      apiKey: string;
      baseUrl: string;
    };
  };
  gpu: {
    vastAi: {
      apiKey: string;
    };
    runpod: {
      apiKey: string;
    };
  };
  notifications: {
    expo: {
      accessToken: string;
    };
    twilio: {
      accountSid: string;
      authToken: string;
      phoneNumber: string;
    };
    sendgrid: {
      apiKey: string;
      fromEmail: string;
    };
  };
  analytics: {
    mixpanel: {
      token: string;
    };
    ga: {
      trackingId: string;
    };
  };
  rateLimit: {
    windowMs: number;
    maxRequests: number;
  };
  security: {
    corsOrigin: string;
    helmetEnabled: boolean;
    bcryptRounds: number;
  };
  logging: {
    level: string;
    filePath: string;
  };
  cache: {
    balanceTTL: number;
    priceTTL: number;
    validatorTTL: number;
    proposalTTL: number;
  };
}

const config: Config = {
  env: process.env.NODE_ENV || 'development',
  port: parseInt(process.env.PORT || '3000'),
  apiVersion: process.env.API_VERSION || 'v1',

  database: {
    host: process.env.DB_HOST || 'localhost',
    port: parseInt(process.env.DB_PORT || '5432'),
    name: process.env.DB_NAME || 'etrid_wallet',
    user: process.env.DB_USER || 'postgres',
    password: process.env.DB_PASSWORD || '',
    maxConnections: parseInt(process.env.DB_MAX_CONNECTIONS || '20'),
  },

  redis: {
    url: process.env.REDIS_URL || 'redis://localhost:6379',
    password: process.env.REDIS_PASSWORD,
    db: parseInt(process.env.REDIS_DB || '0'),
  },

  jwt: {
    secret: process.env.JWT_SECRET || 'change_this_secret_in_production',
    expiresIn: process.env.JWT_EXPIRES_IN || '24h',
    refreshSecret: process.env.JWT_REFRESH_SECRET || 'change_this_refresh_secret',
    refreshExpiresIn: process.env.JWT_REFRESH_EXPIRES_IN || '7d',
  },

  blockchain: {
    wsUrl: process.env.FLARECHAIN_WS_URL || 'wss://flarechain.etrid.io',
    httpUrl: process.env.FLARECHAIN_HTTP_URL || 'https://rpc.etrid.io',
    syncInterval: parseInt(process.env.CHAIN_SYNC_INTERVAL || '6000'),
  },

  atm: {
    coinme: {
      apiKey: process.env.COINME_API_KEY || '',
      apiSecret: process.env.COINME_API_SECRET || '',
      baseUrl: process.env.COINME_BASE_URL || 'https://api.coinme.com/v1',
    },
    bitcoinDepot: {
      apiKey: process.env.BITCOIN_DEPOT_API_KEY || '',
      baseUrl: process.env.BITCOIN_DEPOT_BASE_URL || 'https://api.bitcoindepot.com/v1',
    },
    coinflip: {
      apiKey: process.env.COINFLIP_API_KEY || '',
      baseUrl: process.env.COINFLIP_BASE_URL || 'https://api.coinflip.tech/v1',
    },
  },

  gpu: {
    vastAi: {
      apiKey: process.env.VAST_AI_API_KEY || '',
    },
    runpod: {
      apiKey: process.env.RUNPOD_API_KEY || '',
    },
  },

  notifications: {
    expo: {
      accessToken: process.env.EXPO_ACCESS_TOKEN || '',
    },
    twilio: {
      accountSid: process.env.TWILIO_ACCOUNT_SID || '',
      authToken: process.env.TWILIO_AUTH_TOKEN || '',
      phoneNumber: process.env.TWILIO_PHONE_NUMBER || '',
    },
    sendgrid: {
      apiKey: process.env.SENDGRID_API_KEY || '',
      fromEmail: process.env.FROM_EMAIL || 'noreply@etrid.io',
    },
  },

  analytics: {
    mixpanel: {
      token: process.env.MIXPANEL_TOKEN || '',
    },
    ga: {
      trackingId: process.env.GA_TRACKING_ID || '',
    },
  },

  rateLimit: {
    windowMs: parseInt(process.env.RATE_LIMIT_WINDOW_MS || '900000'), // 15 minutes
    maxRequests: parseInt(process.env.RATE_LIMIT_MAX_REQUESTS || '100'),
  },

  security: {
    corsOrigin: process.env.CORS_ORIGIN || '*',
    helmetEnabled: process.env.HELMET_ENABLED === 'true',
    bcryptRounds: parseInt(process.env.BCRYPT_ROUNDS || '12'),
  },

  logging: {
    level: process.env.LOG_LEVEL || 'info',
    filePath: process.env.LOG_FILE_PATH || './logs/app.log',
  },

  cache: {
    balanceTTL: parseInt(process.env.CACHE_BALANCE_TTL || '300'),
    priceTTL: parseInt(process.env.CACHE_PRICE_TTL || '60'),
    validatorTTL: parseInt(process.env.CACHE_VALIDATOR_TTL || '600'),
    proposalTTL: parseInt(process.env.CACHE_PROPOSAL_TTL || '120'),
  },
};

// Validate critical configuration
if (config.env === 'production') {
  if (config.jwt.secret === 'change_this_secret_in_production') {
    throw new Error('JWT_SECRET must be changed in production!');
  }
  if (!config.database.password) {
    throw new Error('DB_PASSWORD is required in production!');
  }
}

export default config;
