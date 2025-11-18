import { Router, Response } from 'express';
import { AuthenticatedRequest, ApiResponse, PaginatedResponse } from '../types';
import { asyncHandler } from '../middleware/errorHandler';
import { authenticateJWT } from '../middleware/auth';
import { validate, addressSchema, paginationSchema, transferSchema } from '../middleware/validation';
import BlockchainService from '../services/BlockchainService';
import cacheService from '../services/CacheService';
import db from '../database/client';
import Joi from 'joi';

const router = Router();

/**
 * GET /api/v1/accounts/:address/balance
 * Get account balance
 */
router.get(
  '/:address/balance',
  validate(Joi.object({ address: addressSchema }), 'params'),
  asyncHandler(async (req: AuthenticatedRequest, res: Response) => {
    const { address } = req.params;

    // Check cache first
    const cached = await cacheService.getCachedBalance(address);
    if (cached) {
      const response: ApiResponse = {
        success: true,
        data: { ...cached, fromCache: true },
        timestamp: new Date().toISOString(),
      };
      res.json(response);
      return;
    }

    // Fetch from blockchain
    const balance = await BlockchainService.getBalance(address);

    // Cache the result
    await cacheService.cacheBalance(address, balance);

    const response: ApiResponse = {
      success: true,
      data: balance,
      timestamp: new Date().toISOString(),
    };

    res.json(response);
  })
);

/**
 * GET /api/v1/accounts/:address/portfolio
 * Get complete portfolio (all assets)
 */
router.get(
  '/:address/portfolio',
  validate(Joi.object({ address: addressSchema }), 'params'),
  asyncHandler(async (req: AuthenticatedRequest, res: Response) => {
    const { address } = req.params;

    const portfolio = await BlockchainService.getPortfolio(address);

    const response: ApiResponse = {
      success: true,
      data: portfolio,
      timestamp: new Date().toISOString(),
    };

    res.json(response);
  })
);

/**
 * GET /api/v1/accounts/:address/transactions
 * Get transaction history (paginated)
 */
router.get(
  '/:address/transactions',
  validate(Joi.object({ address: addressSchema }), 'params'),
  validate(paginationSchema, 'query'),
  asyncHandler(async (req: AuthenticatedRequest, res: Response) => {
    const { address } = req.params;
    const { page, limit } = req.query as any;

    const offset = (page - 1) * limit;

    // Get total count
    const countResult = await db.query(
      `SELECT COUNT(*) as total
       FROM transactions
       WHERE from_address = $1 OR to_address = $1`,
      [address]
    );

    const total = parseInt(countResult.rows[0].total);

    // Get transactions
    const result = await db.query(
      `SELECT
         id,
         tx_hash,
         block_number,
         from_address,
         to_address,
         amount,
         asset,
         fee,
         status,
         tx_type,
         metadata,
         created_at,
         confirmed_at
       FROM transactions
       WHERE from_address = $1 OR to_address = $1
       ORDER BY created_at DESC
       LIMIT $2 OFFSET $3`,
      [address, limit, offset]
    );

    const response: PaginatedResponse<any> = {
      data: result.rows,
      pagination: {
        page,
        limit,
        total,
        totalPages: Math.ceil(total / limit),
      },
    };

    res.json({
      success: true,
      ...response,
      timestamp: new Date().toISOString(),
    });
  })
);

/**
 * POST /api/v1/accounts/:address/transfer
 * Submit transfer transaction
 */
router.post(
  '/:address/transfer',
  authenticateJWT,
  validate(Joi.object({ address: addressSchema }), 'params'),
  validate(transferSchema),
  asyncHandler(async (req: AuthenticatedRequest, res: Response) => {
    const { address } = req.params;
    const { to, amount, asset, memo } = req.body;

    // Verify user owns this address
    if (req.user?.address !== address) {
      const response: ApiResponse = {
        success: false,
        error: {
          code: 'UNAUTHORIZED',
          message: 'You do not own this address',
        },
        timestamp: new Date().toISOString(),
      };
      res.status(403).json(response);
      return;
    }

    // Submit transaction to blockchain
    const txHash = await BlockchainService.submitTransfer({
      from: address,
      to,
      amount,
      asset,
      memo,
    });

    // Store in database
    await db.query(
      `INSERT INTO transactions (user_id, tx_hash, from_address, to_address, amount, asset, tx_type, metadata)
       VALUES ($1, $2, $3, $4, $5, $6, 'transfer', $7)`,
      [req.user.id, txHash, address, to, amount, asset, { memo }]
    );

    // Invalidate balance cache
    await cacheService.invalidateBalance(address);

    const response: ApiResponse = {
      success: true,
      data: {
        txHash,
        status: 'pending',
        from: address,
        to,
        amount,
        asset,
      },
      timestamp: new Date().toISOString(),
    };

    res.json(response);
  })
);

/**
 * GET /api/v1/accounts/:address/activity
 * Get account activity summary
 */
router.get(
  '/:address/activity',
  validate(Joi.object({ address: addressSchema }), 'params'),
  asyncHandler(async (req: AuthenticatedRequest, res: Response) => {
    const { address } = req.params;

    const result = await db.query(
      `SELECT
         COUNT(*) as total_transactions,
         COUNT(*) FILTER (WHERE tx_type = 'transfer') as transfers,
         COUNT(*) FILTER (WHERE tx_type = 'stake') as stakes,
         COUNT(*) FILTER (WHERE tx_type = 'vote') as votes,
         COUNT(*) FILTER (WHERE status = 'confirmed') as confirmed,
         COUNT(*) FILTER (WHERE status = 'pending') as pending,
         COALESCE(SUM(amount) FILTER (WHERE tx_type = 'transfer' AND from_address = $1), 0) as total_sent,
         COALESCE(SUM(amount) FILTER (WHERE tx_type = 'transfer' AND to_address = $1), 0) as total_received
       FROM transactions
       WHERE from_address = $1 OR to_address = $1`,
      [address]
    );

    const response: ApiResponse = {
      success: true,
      data: result.rows[0],
      timestamp: new Date().toISOString(),
    };

    res.json(response);
  })
);

/**
 * GET /api/v1/accounts/:address/stats
 * Get detailed account statistics
 */
router.get(
  '/:address/stats',
  validate(Joi.object({ address: addressSchema }), 'params'),
  asyncHandler(async (req: AuthenticatedRequest, res: Response) => {
    const { address } = req.params;

    // Get balance
    const balance = await BlockchainService.getBalance(address);

    // Get transaction stats
    const txStats = await db.query(
      `SELECT
         COUNT(*) as total_transactions,
         COUNT(*) FILTER (WHERE created_at > NOW() - INTERVAL '24 hours') as transactions_24h,
         COUNT(*) FILTER (WHERE created_at > NOW() - INTERVAL '7 days') as transactions_7d,
         COUNT(*) FILTER (WHERE created_at > NOW() - INTERVAL '30 days') as transactions_30d
       FROM transactions
       WHERE from_address = $1 OR to_address = $1`,
      [address]
    );

    // Get staking stats
    const stakingStats = await db.query(
      `SELECT
         COUNT(*) as active_positions,
         COALESCE(SUM(amount), 0) as total_staked,
         COALESCE(SUM(rewards_earned), 0) as total_rewards
       FROM staking_positions
       WHERE user_id = (SELECT id FROM users WHERE address = $1)
         AND status = 'active'`,
      [address]
    );

    // Get governance stats
    const govStats = await db.query(
      `SELECT COUNT(*) as votes_cast
       FROM governance_votes
       WHERE user_id = (SELECT id FROM users WHERE address = $1)`,
      [address]
    );

    const response: ApiResponse = {
      success: true,
      data: {
        balance,
        transactions: txStats.rows[0],
        staking: stakingStats.rows[0],
        governance: govStats.rows[0],
      },
      timestamp: new Date().toISOString(),
    };

    res.json(response);
  })
);

export default router;
