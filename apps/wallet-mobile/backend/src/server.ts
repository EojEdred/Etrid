import express, { Application } from 'express';
import helmet from 'helmet';
import cors from 'cors';
import rateLimit from 'express-rate-limit';
import config from './config';
import logger from './utils/logger';
import db from './database/client';
import cacheService from './services/CacheService';
import { errorHandler, notFoundHandler } from './middleware/errorHandler';

// Import routes
import authRoutes from './routes/auth';
import accountRoutes from './routes/accounts';
import stakingRoutes from './routes/staking';
import governanceRoutes from './routes/governance';
import atmRoutes from './routes/atm';
import bridgeRoutes from './routes/bridge';
import gpuRoutes from './routes/gpu';
import notificationRoutes from './routes/notifications';

class Server {
  public app: Application;

  constructor() {
    this.app = express();
    this.initializeMiddleware();
    this.initializeRoutes();
    this.initializeErrorHandling();
  }

  private initializeMiddleware(): void {
    // Security middleware
    if (config.security.helmetEnabled) {
      this.app.use(helmet());
    }

    // CORS
    this.app.use(
      cors({
        origin: config.security.corsOrigin,
        credentials: true,
      })
    );

    // Body parsing
    this.app.use(express.json({ limit: '10mb' }));
    this.app.use(express.urlencoded({ extended: true, limit: '10mb' }));

    // Rate limiting
    const limiter = rateLimit({
      windowMs: config.rateLimit.windowMs,
      max: config.rateLimit.maxRequests,
      message: {
        success: false,
        error: {
          code: 'RATE_LIMIT_EXCEEDED',
          message: 'Too many requests, please try again later',
        },
      },
      standardHeaders: true,
      legacyHeaders: false,
    });

    this.app.use('/api/', limiter);

    // Request logging
    this.app.use((req, res, next) => {
      logger.info('Incoming request', {
        method: req.method,
        url: req.url,
        ip: req.ip,
        userAgent: req.get('user-agent'),
      });
      next();
    });
  }

  private initializeRoutes(): void {
    // Health check
    this.app.get('/health', (req, res) => {
      res.json({
        success: true,
        data: {
          status: 'ok',
          timestamp: new Date().toISOString(),
          version: config.apiVersion,
          uptime: process.uptime(),
        },
      });
    });

    // API version check
    this.app.get('/version', (req, res) => {
      res.json({
        success: true,
        data: {
          version: config.apiVersion,
          environment: config.env,
        },
      });
    });

    // API routes
    const apiPrefix = `/api/${config.apiVersion}`;
    this.app.use(`${apiPrefix}/auth`, authRoutes);
    this.app.use(`${apiPrefix}/accounts`, accountRoutes);
    this.app.use(`${apiPrefix}/staking`, stakingRoutes);
    this.app.use(`${apiPrefix}/governance`, governanceRoutes);
    this.app.use(`${apiPrefix}/atm`, atmRoutes);
    this.app.use(`${apiPrefix}/bridge`, bridgeRoutes);
    this.app.use(`${apiPrefix}/gpu`, gpuRoutes);
    this.app.use(`${apiPrefix}/notifications`, notificationRoutes);
  }

  private initializeErrorHandling(): void {
    // 404 handler
    this.app.use(notFoundHandler);

    // Error handler
    this.app.use(errorHandler);
  }

  public async start(): Promise<void> {
    try {
      // Test database connection
      logger.info('Testing database connection...');
      const dbConnected = await db.testConnection();
      if (!dbConnected) {
        throw new Error('Failed to connect to database');
      }

      // Connect to Redis
      logger.info('Connecting to Redis...');
      await cacheService.connect();

      // Start server
      this.app.listen(config.port, () => {
        logger.info(`Server started successfully`, {
          port: config.port,
          environment: config.env,
          nodeVersion: process.version,
        });
      });
    } catch (error: any) {
      logger.error('Failed to start server', { error: error.message });
      process.exit(1);
    }
  }

  public async shutdown(): Promise<void> {
    logger.info('Shutting down server...');

    try {
      await db.close();
      await cacheService.disconnect();
      logger.info('Server shutdown complete');
      process.exit(0);
    } catch (error: any) {
      logger.error('Error during shutdown', { error: error.message });
      process.exit(1);
    }
  }
}

// Create server instance
const server = new Server();

// Start server
server.start();

// Graceful shutdown
process.on('SIGTERM', () => server.shutdown());
process.on('SIGINT', () => server.shutdown());

// Handle unhandled promise rejections
process.on('unhandledRejection', (reason: any) => {
  logger.error('Unhandled Promise Rejection', {
    reason: reason?.message || reason,
    stack: reason?.stack,
  });
});

// Handle uncaught exceptions
process.on('uncaughtException', (error: Error) => {
  logger.error('Uncaught Exception', {
    error: error.message,
    stack: error.stack,
  });
  server.shutdown();
});

export default server;
