import { Router, Response } from 'express';
import { AuthenticatedRequest, ApiResponse } from '../types';
import { asyncHandler } from '../middleware/errorHandler';
import { authenticateJWT } from '../middleware/auth';
import { validate, gpuSearchSchema, gpuRentSchema } from '../middleware/validation';
import GPUService from '../services/GPUService';
import db from '../database/client';

const router = Router();

/**
 * GET /api/v1/gpu/search
 * Search available GPUs
 */
router.get(
  '/search',
  validate(gpuSearchSchema, 'query'),
  asyncHandler(async (req: AuthenticatedRequest, res: Response) => {
    const { min_vram, max_price, gpu_type, provider } = req.query;

    const gpus = await GPUService.searchGPUs({
      min_vram: min_vram ? parseInt(min_vram as string) : undefined,
      max_price: max_price ? parseFloat(max_price as string) : undefined,
      gpu_type: gpu_type as string,
      provider: provider as string,
    });

    res.json({
      success: true,
      data: {
        gpus,
        count: gpus.length,
      },
      timestamp: new Date().toISOString(),
    });
  })
);

/**
 * GET /api/v1/gpu/:id
 * Get GPU details
 */
router.get(
  '/:id',
  asyncHandler(async (req: AuthenticatedRequest, res: Response) => {
    const { id } = req.params;

    const gpu = await GPUService.getGPUDetails(id);

    if (!gpu) {
      res.status(404).json({
        success: false,
        error: { code: 'NOT_FOUND', message: 'GPU not found' },
        timestamp: new Date().toISOString(),
      });
      return;
    }

    res.json({
      success: true,
      data: gpu,
      timestamp: new Date().toISOString(),
    });
  })
);

/**
 * POST /api/v1/gpu/:id/rent
 * Rent GPU
 */
router.post(
  '/:id/rent',
  authenticateJWT,
  validate(gpuRentSchema),
  asyncHandler(async (req: AuthenticatedRequest, res: Response) => {
    const { id } = req.params;
    const { duration_hours } = req.body;

    // Get GPU details
    const gpu = await GPUService.getGPUDetails(id);

    if (!gpu || !gpu.availability) {
      res.status(400).json({
        success: false,
        error: { code: 'GPU_UNAVAILABLE', message: 'GPU not available' },
        timestamp: new Date().toISOString(),
      });
      return;
    }

    const totalCost = parseFloat(gpu.price_per_hour) * duration_hours;

    // Create rental
    const result = await db.query(
      `INSERT INTO gpu_rentals
         (user_id, gpu_id, gpu_name, provider, duration_hours,
          price_per_hour, total_cost, vram_gb, gpu_count)
       VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
       RETURNING *`,
      [
        req.user!.id,
        id,
        gpu.name,
        gpu.provider,
        duration_hours,
        gpu.price_per_hour,
        totalCost.toString(),
        gpu.vram_gb,
        gpu.gpu_count,
      ]
    );

    const rental = result.rows[0];

    // Provision GPU
    const provisionedGPU = await GPUService.provisionGPU(gpu.provider, id, duration_hours);

    // Update rental with connection details
    await db.query(
      `UPDATE gpu_rentals
       SET ssh_host = $1, ssh_port = $2, ssh_username = $3,
           ssh_password = $4, status = 'provisioning', provider_instance_id = $5
       WHERE id = $6`,
      [
        provisionedGPU.ssh_host,
        provisionedGPU.ssh_port,
        provisionedGPU.ssh_username,
        provisionedGPU.ssh_password,
        provisionedGPU.instance_id,
        rental.id,
      ]
    );

    res.json({
      success: true,
      data: {
        rental_id: rental.id,
        status: 'provisioning',
        total_cost: totalCost.toString(),
        duration_hours,
        connection: {
          ssh_host: provisionedGPU.ssh_host,
          ssh_port: provisionedGPU.ssh_port,
          ssh_username: provisionedGPU.ssh_username,
          ssh_password: provisionedGPU.ssh_password,
        },
        estimated_ready_time: '2-5 minutes',
      },
      timestamp: new Date().toISOString(),
    });
  })
);

/**
 * GET /api/v1/gpu/rentals
 * Get user's GPU rentals
 */
router.get(
  '/rentals',
  authenticateJWT,
  asyncHandler(async (req: AuthenticatedRequest, res: Response) => {
    const result = await db.query(
      `SELECT * FROM gpu_rentals
       WHERE user_id = $1
       ORDER BY created_at DESC`,
      [req.user!.id]
    );

    res.json({
      success: true,
      data: result.rows,
      timestamp: new Date().toISOString(),
    });
  })
);

/**
 * POST /api/v1/gpu/rentals/:id/terminate
 * Terminate GPU rental
 */
router.post(
  '/rentals/:id/terminate',
  authenticateJWT,
  asyncHandler(async (req: AuthenticatedRequest, res: Response) => {
    const { id } = req.params;

    const result = await db.query(
      `SELECT * FROM gpu_rentals WHERE id = $1 AND user_id = $2`,
      [id, req.user!.id]
    );

    if (result.rows.length === 0) {
      res.status(404).json({
        success: false,
        error: { code: 'NOT_FOUND', message: 'Rental not found' },
        timestamp: new Date().toISOString(),
      });
      return;
    }

    const rental = result.rows[0];

    // Terminate instance
    await GPUService.terminateInstance(rental.provider, rental.provider_instance_id);

    // Update status
    await db.query(
      `UPDATE gpu_rentals
       SET status = 'completed', end_time = NOW()
       WHERE id = $1`,
      [id]
    );

    res.json({
      success: true,
      data: { message: 'Rental terminated successfully' },
      timestamp: new Date().toISOString(),
    });
  })
);

export default router;
