import { Router, Response } from 'express';
import { AuthenticatedRequest, ApiResponse } from '../types';
import { asyncHandler } from '../middleware/errorHandler';
import { authenticateJWT } from '../middleware/auth';
import { validate, voteSchema, paginationSchema } from '../middleware/validation';
import BlockchainService from '../services/BlockchainService';
import cacheService from '../services/CacheService';
import db from '../database/client';
import Joi from 'joi';

const router = Router();

/**
 * GET /api/v1/governance/proposals
 * Get active proposals
 */
router.get(
  '/proposals',
  validate(paginationSchema, 'query'),
  asyncHandler(async (req: AuthenticatedRequest, res: Response) => {
    const { page, limit } = req.query as any;
    const offset = (page - 1) * limit;

    const countResult = await db.query(
      `SELECT COUNT(*) as total FROM proposals WHERE status IN ('active', 'passed')`
    );
    const total = parseInt(countResult.rows[0].total);

    const result = await db.query(
      `SELECT * FROM proposals
       WHERE status IN ('active', 'passed')
       ORDER BY
         CASE WHEN status = 'active' THEN 0 ELSE 1 END,
         voting_ends_at DESC
       LIMIT $1 OFFSET $2`,
      [limit, offset]
    );

    res.json({
      success: true,
      data: result.rows,
      pagination: {
        page,
        limit,
        total,
        totalPages: Math.ceil(total / limit),
      },
      timestamp: new Date().toISOString(),
    });
  })
);

/**
 * GET /api/v1/governance/proposals/:id
 * Get proposal details
 */
router.get(
  '/proposals/:id',
  asyncHandler(async (req: AuthenticatedRequest, res: Response) => {
    const { id } = req.params;

    const cached = await cacheService.getCachedProposal(parseInt(id));
    if (cached) {
      res.json({
        success: true,
        data: { ...cached, fromCache: true },
        timestamp: new Date().toISOString(),
      });
      return;
    }

    const result = await db.query(
      `SELECT
         p.*,
         COUNT(gv.id) as vote_count,
         COUNT(gv.id) FILTER (WHERE gv.support = true) as yes_count,
         COUNT(gv.id) FILTER (WHERE gv.support = false) as no_count
       FROM proposals p
       LEFT JOIN governance_votes gv ON p.id = gv.proposal_id
       WHERE p.id = $1
       GROUP BY p.id`,
      [id]
    );

    if (result.rows.length === 0) {
      res.status(404).json({
        success: false,
        error: { code: 'NOT_FOUND', message: 'Proposal not found' },
        timestamp: new Date().toISOString(),
      });
      return;
    }

    await cacheService.cacheProposal(parseInt(id), result.rows[0]);

    res.json({
      success: true,
      data: result.rows[0],
      timestamp: new Date().toISOString(),
    });
  })
);

/**
 * POST /api/v1/governance/proposals/:id/vote
 * Submit vote on proposal
 */
router.post(
  '/proposals/:id/vote',
  authenticateJWT,
  validate(voteSchema),
  asyncHandler(async (req: AuthenticatedRequest, res: Response) => {
    const { id } = req.params;
    const { support, conviction, balance } = req.body;

    // Calculate voting power based on conviction
    const convictionMultiplier = [0.1, 1, 2, 3, 4, 5, 6][conviction];
    const votingPower = parseFloat(balance) * convictionMultiplier;

    // Submit vote to blockchain
    const txHash = await BlockchainService.submitVote({
      proposalId: parseInt(id),
      support,
      conviction,
      balance,
    });

    // Store vote in database
    await db.query(
      `INSERT INTO governance_votes
         (user_id, proposal_id, support, conviction, voting_power, balance_locked, tx_hash)
       VALUES ($1, $2, $3, $4, $5, $6, $7)
       ON CONFLICT (user_id, proposal_id)
       DO UPDATE SET support = $3, conviction = $4, voting_power = $5, tx_hash = $7`,
      [req.user!.id, id, support, conviction, votingPower.toString(), balance, txHash]
    );

    // Invalidate cache
    await cacheService.del(`proposal:${id}`);

    res.json({
      success: true,
      data: { txHash, votingPower: votingPower.toString(), status: 'pending' },
      timestamp: new Date().toISOString(),
    });
  })
);

/**
 * GET /api/v1/governance/:address/votes
 * Get vote history for address
 */
router.get(
  '/:address/votes',
  validate(paginationSchema, 'query'),
  asyncHandler(async (req: AuthenticatedRequest, res: Response) => {
    const { address } = req.params;
    const { page, limit } = req.query as any;
    const offset = (page - 1) * limit;

    const result = await db.query(
      `SELECT
         gv.*,
         p.title as proposal_title,
         p.status as proposal_status
       FROM governance_votes gv
       JOIN proposals p ON gv.proposal_id = p.id
       WHERE gv.user_id = (SELECT id FROM users WHERE address = $1)
       ORDER BY gv.created_at DESC
       LIMIT $2 OFFSET $3`,
      [address, limit, offset]
    );

    res.json({
      success: true,
      data: result.rows,
      timestamp: new Date().toISOString(),
    });
  })
);

export default router;
