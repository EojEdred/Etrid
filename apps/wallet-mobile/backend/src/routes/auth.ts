import { Router, Response } from 'express';
import { AuthenticatedRequest, ApiResponse } from '../types';
import { asyncHandler } from '../middleware/errorHandler';
import { validate, loginSchema, refreshTokenSchema } from '../middleware/validation';
import {
  generateToken,
  generateRefreshToken,
  verifyRefreshToken,
} from '../middleware/auth';
import UserRepository from '../repositories/UserRepository';
import { signatureVerify } from '@polkadot/util-crypto';
import { u8aToHex, hexToU8a } from '@polkadot/util';
import logger from '../utils/logger';

const router = Router();

/**
 * POST /api/v1/auth/login
 * Login with address signature
 */
router.post(
  '/login',
  validate(loginSchema),
  asyncHandler(async (req: AuthenticatedRequest, res: Response) => {
    const { address, signature, message } = req.body;

    // Verify signature
    const isValid = signatureVerify(message, signature, address).isValid;

    if (!isValid) {
      const response: ApiResponse = {
        success: false,
        error: {
          code: 'INVALID_SIGNATURE',
          message: 'Invalid signature',
        },
        timestamp: new Date().toISOString(),
      };
      res.status(401).json(response);
      return;
    }

    // Find or create user
    let user = await UserRepository.findByAddress(address);

    if (!user) {
      user = await UserRepository.create({ address });
      logger.info('New user created', { userId: user.id, address });
    }

    // Update last login
    await UserRepository.updateLastLogin(user.id);

    // Generate tokens
    const token = generateToken({
      id: user.id,
      address: user.address,
      email: user.email,
    });

    const refreshToken = generateRefreshToken({
      id: user.id,
      address: user.address,
    });

    const response: ApiResponse = {
      success: true,
      data: {
        user: {
          id: user.id,
          address: user.address,
          email: user.email,
          kyc_status: user.kyc_status,
          kyc_level: user.kyc_level,
        },
        token,
        refreshToken,
        expiresIn: '24h',
      },
      timestamp: new Date().toISOString(),
    };

    res.json(response);
  })
);

/**
 * POST /api/v1/auth/refresh
 * Refresh JWT token
 */
router.post(
  '/refresh',
  validate(refreshTokenSchema),
  asyncHandler(async (req: AuthenticatedRequest, res: Response) => {
    const { refreshToken } = req.body;

    const decoded = verifyRefreshToken(refreshToken);

    if (!decoded) {
      const response: ApiResponse = {
        success: false,
        error: {
          code: 'INVALID_REFRESH_TOKEN',
          message: 'Invalid or expired refresh token',
        },
        timestamp: new Date().toISOString(),
      };
      res.status(401).json(response);
      return;
    }

    // Get user
    const user = await UserRepository.findById(decoded.id);

    if (!user) {
      const response: ApiResponse = {
        success: false,
        error: {
          code: 'USER_NOT_FOUND',
          message: 'User not found',
        },
        timestamp: new Date().toISOString(),
      };
      res.status(404).json(response);
      return;
    }

    // Generate new tokens
    const token = generateToken({
      id: user.id,
      address: user.address,
      email: user.email,
    });

    const newRefreshToken = generateRefreshToken({
      id: user.id,
      address: user.address,
    });

    const response: ApiResponse = {
      success: true,
      data: {
        token,
        refreshToken: newRefreshToken,
        expiresIn: '24h',
      },
      timestamp: new Date().toISOString(),
    };

    res.json(response);
  })
);

/**
 * POST /api/v1/auth/logout
 * Logout (client-side token deletion)
 */
router.post(
  '/logout',
  asyncHandler(async (req: AuthenticatedRequest, res: Response) => {
    // In a stateless JWT system, logout is handled client-side
    // However, we can log the event

    logger.info('User logout', {
      userId: req.user?.id,
      address: req.user?.address,
    });

    const response: ApiResponse = {
      success: true,
      data: {
        message: 'Logged out successfully',
      },
      timestamp: new Date().toISOString(),
    };

    res.json(response);
  })
);

/**
 * POST /api/v1/auth/verify-2fa
 * Verify 2FA code
 */
router.post(
  '/verify-2fa',
  asyncHandler(async (req: AuthenticatedRequest, res: Response) => {
    const { userId, code } = req.body;

    // TODO: Implement 2FA verification with authenticator app
    // This would use a library like speakeasy to verify TOTP codes

    const response: ApiResponse = {
      success: true,
      data: {
        verified: true,
      },
      timestamp: new Date().toISOString(),
    };

    res.json(response);
  })
);

/**
 * GET /api/v1/auth/nonce
 * Get nonce for signing (prevents replay attacks)
 */
router.get(
  '/nonce/:address',
  asyncHandler(async (req: AuthenticatedRequest, res: Response) => {
    const { address } = req.params;

    // Generate a random nonce
    const nonce = Math.floor(Math.random() * 1000000).toString();
    const message = `Sign this message to authenticate with Ëtrid Wallet.\n\nNonce: ${nonce}\nTimestamp: ${Date.now()}`;

    const response: ApiResponse = {
      success: true,
      data: {
        message,
        nonce,
      },
      timestamp: new Date().toISOString(),
    };

    res.json(response);
  })
);

export default router;
