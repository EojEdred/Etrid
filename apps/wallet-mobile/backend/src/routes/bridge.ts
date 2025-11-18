import { Router, Response } from 'express';
import { AuthenticatedRequest, ApiResponse } from '../types';
import { asyncHandler } from '../middleware/errorHandler';
import { authenticateJWT } from '../middleware/auth';
import { validate, bridgeTransferSchema } from '../middleware/validation';
import BridgeService from '../services/BridgeService';
import db from '../database/client';

const router = Router();

/**
 * GET /api/v1/bridge/chains
 * Get supported chains
 */
router.get(
  '/chains',
  asyncHandler(async (req: AuthenticatedRequest, res: Response) => {
    const chains = [
      { id: 'BTC', name: 'Bitcoin', assets: ['BTC'] },
      { id: 'ETH', name: 'Ethereum', assets: ['ETH', 'USDT', 'USDC'] },
      { id: 'BSC', name: 'Binance Smart Chain', assets: ['BNB', 'USDT'] },
      { id: 'MATIC', name: 'Polygon', assets: ['MATIC', 'USDT'] },
      { id: 'FLARE', name: 'Ëtrid FlareChain', assets: ['ETR'] },
    ];

    res.json({
      success: true,
      data: { chains },
      timestamp: new Date().toISOString(),
    });
  })
);

/**
 * GET /api/v1/bridge/rate
 * Get exchange rate
 */
router.get(
  '/rate',
  asyncHandler(async (req: AuthenticatedRequest, res: Response) => {
    const { from, to } = req.query;

    if (!from || !to) {
      res.status(400).json({
        success: false,
        error: { code: 'MISSING_PARAMS', message: 'from and to parameters required' },
        timestamp: new Date().toISOString(),
      });
      return;
    }

    const rate = await BridgeService.getExchangeRate(from as string, to as string);

    res.json({
      success: true,
      data: { from, to, rate, fee_percent: 0.5 },
      timestamp: new Date().toISOString(),
    });
  })
);

/**
 * POST /api/v1/bridge/transfer
 * Submit bridge transfer
 */
router.post(
  '/transfer',
  authenticateJWT,
  validate(bridgeTransferSchema),
  asyncHandler(async (req: AuthenticatedRequest, res: Response) => {
    const { from_chain, to_chain, from_address, to_address, from_asset, amount } = req.body;

    const exchangeRate = await BridgeService.getExchangeRate(from_asset, 'ETR');
    const bridgeFee = parseFloat(amount) * 0.005; // 0.5% fee
    const amountTo = (parseFloat(amount) - bridgeFee) * exchangeRate;

    const result = await db.query(
      `INSERT INTO bridge_transfers
         (user_id, from_chain, to_chain, from_address, to_address,
          from_asset, to_asset, amount_from, amount_to, exchange_rate, bridge_fee)
       VALUES ($1, $2, $3, $4, $5, $6, 'ETR', $7, $8, $9, $10)
       RETURNING *`,
      [
        req.user!.id,
        from_chain,
        to_chain,
        from_address,
        to_address,
        from_asset,
        amount,
        amountTo.toString(),
        exchangeRate,
        bridgeFee.toString(),
      ]
    );

    const transfer = result.rows[0];

    // Start bridge process
    await BridgeService.initiateBridge(transfer);

    res.json({
      success: true,
      data: {
        transfer_id: transfer.id,
        status: 'pending',
        amount_from: amount,
        amount_to: amountTo.toString(),
        fee: bridgeFee.toString(),
        estimated_time: '15-30 minutes',
      },
      timestamp: new Date().toISOString(),
    });
  })
);

/**
 * GET /api/v1/bridge/transfers/:id/status
 * Get transfer status
 */
router.get(
  '/transfers/:id/status',
  asyncHandler(async (req: AuthenticatedRequest, res: Response) => {
    const { id } = req.params;

    const result = await db.query(
      `SELECT * FROM bridge_transfers WHERE id = $1`,
      [id]
    );

    if (result.rows.length === 0) {
      res.status(404).json({
        success: false,
        error: { code: 'NOT_FOUND', message: 'Transfer not found' },
        timestamp: new Date().toISOString(),
      });
      return;
    }

    res.json({
      success: true,
      data: result.rows[0],
      timestamp: new Date().toISOString(),
    });
  })
);

/**
 * GET /api/v1/bridge/transfers
 * Get user's bridge transfers
 */
router.get(
  '/transfers',
  authenticateJWT,
  asyncHandler(async (req: AuthenticatedRequest, res: Response) => {
    const result = await db.query(
      `SELECT * FROM bridge_transfers
       WHERE user_id = $1
       ORDER BY created_at DESC
       LIMIT 50`,
      [req.user!.id]
    );

    res.json({
      success: true,
      data: result.rows,
      timestamp: new Date().toISOString(),
    });
  })
);

export default router;
