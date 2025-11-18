import { Router, Response } from 'express';
import { AuthenticatedRequest, ApiResponse } from '../types';
import { asyncHandler } from '../middleware/errorHandler';
import { authenticateJWT } from '../middleware/auth';
import { validate, addressSchema, stakeSchema, unstakeSchema, paginationSchema } from '../middleware/validation';
import BlockchainService from '../services/BlockchainService';
import cacheService from '../services/CacheService';
import db from '../database/client';
import Joi from 'joi';

const router = Router();

/**
 * GET /api/v1/staking/:address/positions
 * Get staking positions for an address
 */
router.get(
  '/:address/positions',
  validate(Joi.object({ address: addressSchema }), 'params'),
  asyncHandler(async (req: AuthenticatedRequest, res: Response) => {
    const { address } = req.params;

    const result = await db.query(
      `SELECT
         sp.*,
         v.name as validator_name,
         v.commission_rate,
         v.apy as validator_apy,
         v.is_active as validator_active
       FROM staking_positions sp
       LEFT JOIN validators v ON sp.validator_address = v.address
       WHERE sp.user_id = (SELECT id FROM users WHERE address = $1)
       ORDER BY sp.created_at DESC`,
      [address]
    );

    const response: ApiResponse = {
      success: true,
      data: result.rows,
      timestamp: new Date().toISOString(),
    };

    res.json(response);
  })
);

/**
 * GET /api/v1/staking/:address/rewards
 * Get rewards history
 */
router.get(
  '/:address/rewards',
  validate(Joi.object({ address: addressSchema }), 'params'),
  validate(paginationSchema, 'query'),
  asyncHandler(async (req: AuthenticatedRequest, res: Response) => {
    const { address } = req.params;
    const { page, limit } = req.query as any;
    const offset = (page - 1) * limit;

    // Get total rewards
    const totalResult = await db.query(
      `SELECT
         COALESCE(SUM(rewards_earned), 0) as total_rewards,
         COALESCE(SUM(rewards_claimed), 0) as total_claimed,
         COUNT(*) as position_count
       FROM staking_positions
       WHERE user_id = (SELECT id FROM users WHERE address = $1)`,
      [address]
    );

    // Get recent reward updates (from transaction history)
    const rewardsResult = await db.query(
      `SELECT
         t.*,
         sp.validator_address,
         v.name as validator_name
       FROM transactions t
       LEFT JOIN staking_positions sp ON t.metadata->>'position_id' = sp.id::text
       LEFT JOIN validators v ON sp.validator_address = v.address
       WHERE t.user_id = (SELECT id FROM users WHERE address = $1)
         AND t.tx_type = 'stake'
         AND t.metadata->>'action' = 'claim_rewards'
       ORDER BY t.created_at DESC
       LIMIT $2 OFFSET $3`,
      [address, limit, offset]
    );

    const response: ApiResponse = {
      success: true,
      data: {
        summary: totalResult.rows[0],
        rewards: rewardsResult.rows,
      },
      timestamp: new Date().toISOString(),
    };

    res.json(response);
  })
);

/**
 * POST /api/v1/staking/:address/stake
 * Submit stake transaction
 */
router.post(
  '/:address/stake',
  authenticateJWT,
  validate(Joi.object({ address: addressSchema }), 'params'),
  validate(stakeSchema),
  asyncHandler(async (req: AuthenticatedRequest, res: Response) => {
    const { address } = req.params;
    const { validator_address, amount, auto_compound } = req.body;

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

    // Verify validator exists and is active
    const validatorResult = await db.query(
      'SELECT * FROM validators WHERE address = $1 AND is_active = true',
      [validator_address]
    );

    if (validatorResult.rows.length === 0) {
      const response: ApiResponse = {
        success: false,
        error: {
          code: 'INVALID_VALIDATOR',
          message: 'Validator not found or inactive',
        },
        timestamp: new Date().toISOString(),
      };
      res.status(400).json(response);
      return;
    }

    const validator = validatorResult.rows[0];

    // Submit stake transaction to blockchain
    const txHash = await BlockchainService.submitStake({
      from: address,
      validator: validator_address,
      amount,
    });

    // Create staking position
    const positionResult = await db.query(
      `INSERT INTO staking_positions
         (user_id, validator_address, validator_name, amount, apy, auto_compound, tx_hash, metadata)
       VALUES
         ((SELECT id FROM users WHERE address = $1), $2, $3, $4, $5, $6, $7, $8)
       RETURNING *`,
      [
        address,
        validator_address,
        validator.name,
        amount,
        validator.apy,
        auto_compound,
        txHash,
        { commission_rate: validator.commission_rate },
      ]
    );

    // Invalidate balance cache
    await cacheService.invalidateBalance(address);

    const response: ApiResponse = {
      success: true,
      data: {
        position: positionResult.rows[0],
        txHash,
        status: 'pending',
      },
      timestamp: new Date().toISOString(),
    };

    res.json(response);
  })
);

/**
 * POST /api/v1/staking/:address/unstake
 * Submit unstake transaction
 */
router.post(
  '/:address/unstake',
  authenticateJWT,
  validate(Joi.object({ address: addressSchema }), 'params'),
  validate(unstakeSchema),
  asyncHandler(async (req: AuthenticatedRequest, res: Response) => {
    const { address } = req.params;
    const { position_id, amount } = req.body;

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

    // Get staking position
    const positionResult = await db.query(
      `SELECT sp.* FROM staking_positions sp
       JOIN users u ON sp.user_id = u.id
       WHERE sp.id = $1 AND u.address = $2 AND sp.status = 'active'`,
      [position_id, address]
    );

    if (positionResult.rows.length === 0) {
      const response: ApiResponse = {
        success: false,
        error: {
          code: 'POSITION_NOT_FOUND',
          message: 'Staking position not found or not active',
        },
        timestamp: new Date().toISOString(),
      };
      res.status(404).json(response);
      return;
    }

    const position = positionResult.rows[0];
    const unstakeAmount = amount || position.amount;

    // Submit unstake transaction to blockchain
    const txHash = await BlockchainService.submitUnstake({
      from: address,
      validator: position.validator_address,
      amount: unstakeAmount,
    });

    // Update staking position
    if (parseFloat(unstakeAmount) >= parseFloat(position.amount)) {
      // Full unstake
      await db.query(
        `UPDATE staking_positions
         SET status = 'unbonding', end_date = NOW(), updated_at = NOW()
         WHERE id = $1`,
        [position_id]
      );
    } else {
      // Partial unstake
      await db.query(
        `UPDATE staking_positions
         SET amount = amount - $1, updated_at = NOW()
         WHERE id = $2`,
        [unstakeAmount, position_id]
      );
    }

    // Invalidate balance cache
    await cacheService.invalidateBalance(address);

    const response: ApiResponse = {
      success: true,
      data: {
        txHash,
        status: 'pending',
        amount: unstakeAmount,
        unbonding_period_days: position.unbonding_period_days,
      },
      timestamp: new Date().toISOString(),
    };

    res.json(response);
  })
);

/**
 * GET /api/v1/staking/validators
 * Get validator list
 */
router.get(
  '/validators',
  asyncHandler(async (req: AuthenticatedRequest, res: Response) => {
    // Check cache
    const cached = await cacheService.getCachedValidators();
    if (cached) {
      const response: ApiResponse = {
        success: true,
        data: { validators: cached, fromCache: true },
        timestamp: new Date().toISOString(),
      };
      res.json(response);
      return;
    }

    // Fetch from database
    const result = await db.query(
      `SELECT
         v.*,
         COUNT(DISTINCT sp.user_id) as delegator_count,
         COALESCE(SUM(sp.amount), 0) as delegated_stake
       FROM validators v
       LEFT JOIN staking_positions sp ON v.address = sp.validator_address AND sp.status = 'active'
       WHERE v.is_active = true
       GROUP BY v.id
       ORDER BY v.apy DESC NULLS LAST, v.total_stake DESC`
    );

    // Cache the result
    await cacheService.cacheValidators(result.rows);

    const response: ApiResponse = {
      success: true,
      data: {
        validators: result.rows,
        count: result.rows.length,
      },
      timestamp: new Date().toISOString(),
    };

    res.json(response);
  })
);

/**
 * GET /api/v1/staking/validators/:address
 * Get validator details
 */
router.get(
  '/validators/:address',
  validate(Joi.object({ address: addressSchema }), 'params'),
  asyncHandler(async (req: AuthenticatedRequest, res: Response) => {
    const { address } = req.params;

    const result = await db.query(
      `SELECT
         v.*,
         COUNT(DISTINCT sp.user_id) as delegator_count,
         COALESCE(SUM(sp.amount), 0) as delegated_stake
       FROM validators v
       LEFT JOIN staking_positions sp ON v.address = sp.validator_address AND sp.status = 'active'
       WHERE v.address = $1
       GROUP BY v.id`,
      [address]
    );

    if (result.rows.length === 0) {
      const response: ApiResponse = {
        success: false,
        error: {
          code: 'VALIDATOR_NOT_FOUND',
          message: 'Validator not found',
        },
        timestamp: new Date().toISOString(),
      };
      res.status(404).json(response);
      return;
    }

    const response: ApiResponse = {
      success: true,
      data: result.rows[0],
      timestamp: new Date().toISOString(),
    };

    res.json(response);
  })
);

export default router;
