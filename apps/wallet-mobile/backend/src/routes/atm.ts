import { Router, Response } from 'express';
import { AuthenticatedRequest, ApiResponse } from '../types';
import { asyncHandler } from '../middleware/errorHandler';
import { authenticateJWT } from '../middleware/auth';
import { validate, atmWithdrawalSchema, atmLocationSchema } from '../middleware/validation';
import ATMService from '../services/ATMService';
import db from '../database/client';

const router = Router();

/**
 * GET /api/v1/atm/locations
 * Get nearby ATMs
 */
router.get(
  '/locations',
  validate(atmLocationSchema, 'query'),
  asyncHandler(async (req: AuthenticatedRequest, res: Response) => {
    const { lat, lng, radius, partner } = req.query as any;

    const locations = await ATMService.findNearbyATMs({
      lat: parseFloat(lat),
      lng: parseFloat(lng),
      radius: parseInt(radius),
      partner,
    });

    const response: ApiResponse = {
      success: true,
      data: {
        locations,
        count: locations.length,
      },
      timestamp: new Date().toISOString(),
    };

    res.json(response);
  })
);

/**
 * POST /api/v1/atm/withdraw
 * Create ATM withdrawal
 */
router.post(
  '/withdraw',
  authenticateJWT,
  validate(atmWithdrawalSchema),
  asyncHandler(async (req: AuthenticatedRequest, res: Response) => {
    const { amount_usd, asset, atm_partner, atm_location_id } = req.body;

    // Get exchange rate
    const exchangeRate = await ATMService.getExchangeRate(asset, 'USD');

    // Calculate crypto amount
    const amount_crypto = amount_usd / exchangeRate;

    // Calculate fee (typically 5-10%)
    const feePercent = 0.08; // 8%
    const fee = amount_usd * feePercent;

    // Generate unique withdrawal code
    const withdrawalCode = ATMService.generateWithdrawalCode();

    // Create withdrawal in database
    const result = await db.query(
      `INSERT INTO atm_withdrawals
         (user_id, withdrawal_code, amount_usd, amount_crypto, asset, fee,
          exchange_rate, atm_partner, atm_location_id, expires_at)
       VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, NOW() + INTERVAL '30 minutes')
       RETURNING *`,
      [
        req.user!.id,
        withdrawalCode,
        amount_usd,
        amount_crypto.toString(),
        asset,
        fee,
        exchangeRate,
        atm_partner,
        atm_location_id,
      ]
    );

    const withdrawal = result.rows[0];

    // Submit to partner API
    await ATMService.submitWithdrawal(atm_partner, withdrawal);

    const response: ApiResponse = {
      success: true,
      data: {
        withdrawal_code: withdrawalCode,
        amount_usd,
        amount_crypto: amount_crypto.toString(),
        fee,
        exchange_rate: exchangeRate,
        expires_at: withdrawal.expires_at,
        status: 'pending',
      },
      timestamp: new Date().toISOString(),
    };

    res.json(response);
  })
);

/**
 * GET /api/v1/atm/withdrawals/:code/status
 * Check withdrawal status
 */
router.get(
  '/withdrawals/:code/status',
  asyncHandler(async (req: AuthenticatedRequest, res: Response) => {
    const { code } = req.params;

    const result = await db.query(
      `SELECT * FROM atm_withdrawals WHERE withdrawal_code = $1`,
      [code]
    );

    if (result.rows.length === 0) {
      res.status(404).json({
        success: false,
        error: { code: 'NOT_FOUND', message: 'Withdrawal not found' },
        timestamp: new Date().toISOString(),
      });
      return;
    }

    const withdrawal = result.rows[0];

    // Check if expired
    if (new Date() > new Date(withdrawal.expires_at) && withdrawal.status === 'pending') {
      await db.query(
        `UPDATE atm_withdrawals SET status = 'expired' WHERE id = $1`,
        [withdrawal.id]
      );
      withdrawal.status = 'expired';
    }

    const response: ApiResponse = {
      success: true,
      data: withdrawal,
      timestamp: new Date().toISOString(),
    };

    res.json(response);
  })
);

/**
 * GET /api/v1/atm/withdrawals
 * Get user's withdrawal history
 */
router.get(
  '/withdrawals',
  authenticateJWT,
  asyncHandler(async (req: AuthenticatedRequest, res: Response) => {
    const result = await db.query(
      `SELECT * FROM atm_withdrawals
       WHERE user_id = $1
       ORDER BY created_at DESC
       LIMIT 50`,
      [req.user!.id]
    );

    const response: ApiResponse = {
      success: true,
      data: result.rows,
      timestamp: new Date().toISOString(),
    };

    res.json(response);
  })
);

export default router;
