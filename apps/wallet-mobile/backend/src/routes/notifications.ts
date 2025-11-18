import { Router, Response } from 'express';
import { AuthenticatedRequest, ApiResponse } from '../types';
import { asyncHandler } from '../middleware/errorHandler';
import { authenticateJWT } from '../middleware/auth';
import { validate, paginationSchema } from '../middleware/validation';
import db from '../database/client';

const router = Router();

/**
 * GET /api/v1/notifications
 * Get user notifications
 */
router.get(
  '/',
  authenticateJWT,
  validate(paginationSchema, 'query'),
  asyncHandler(async (req: AuthenticatedRequest, res: Response) => {
    const { page, limit } = req.query as any;
    const offset = (page - 1) * limit;

    const countResult = await db.query(
      `SELECT COUNT(*) as total FROM notifications WHERE user_id = $1`,
      [req.user!.id]
    );

    const result = await db.query(
      `SELECT * FROM notifications
       WHERE user_id = $1
       ORDER BY created_at DESC
       LIMIT $2 OFFSET $3`,
      [req.user!.id, limit, offset]
    );

    res.json({
      success: true,
      data: result.rows,
      pagination: {
        page,
        limit,
        total: parseInt(countResult.rows[0].total),
        totalPages: Math.ceil(parseInt(countResult.rows[0].total) / limit),
      },
      timestamp: new Date().toISOString(),
    });
  })
);

/**
 * GET /api/v1/notifications/unread
 * Get unread notifications count
 */
router.get(
  '/unread',
  authenticateJWT,
  asyncHandler(async (req: AuthenticatedRequest, res: Response) => {
    const result = await db.query(
      `SELECT COUNT(*) as count FROM notifications
       WHERE user_id = $1 AND is_read = false`,
      [req.user!.id]
    );

    res.json({
      success: true,
      data: { unread_count: parseInt(result.rows[0].count) },
      timestamp: new Date().toISOString(),
    });
  })
);

/**
 * POST /api/v1/notifications/:id/read
 * Mark notification as read
 */
router.post(
  '/:id/read',
  authenticateJWT,
  asyncHandler(async (req: AuthenticatedRequest, res: Response) => {
    const { id } = req.params;

    await db.query(
      `UPDATE notifications
       SET is_read = true, read_at = NOW()
       WHERE id = $1 AND user_id = $2`,
      [id, req.user!.id]
    );

    res.json({
      success: true,
      data: { message: 'Notification marked as read' },
      timestamp: new Date().toISOString(),
    });
  })
);

/**
 * POST /api/v1/notifications/read-all
 * Mark all notifications as read
 */
router.post(
  '/read-all',
  authenticateJWT,
  asyncHandler(async (req: AuthenticatedRequest, res: Response) => {
    await db.query(
      `UPDATE notifications
       SET is_read = true, read_at = NOW()
       WHERE user_id = $1 AND is_read = false`,
      [req.user!.id]
    );

    res.json({
      success: true,
      data: { message: 'All notifications marked as read' },
      timestamp: new Date().toISOString(),
    });
  })
);

export default router;
