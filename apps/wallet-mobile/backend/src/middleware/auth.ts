import { Response, NextFunction } from 'express';
import jwt from 'jsonwebtoken';
import config from '../config';
import logger from '../utils/logger';
import { AuthenticatedRequest } from '../types';

/**
 * JWT Authentication Middleware
 */
export const authenticateJWT = (
  req: AuthenticatedRequest,
  res: Response,
  next: NextFunction
): void => {
  const authHeader = req.headers.authorization;

  if (!authHeader) {
    res.status(401).json({
      success: false,
      error: {
        code: 'NO_TOKEN',
        message: 'No authorization token provided',
      },
    });
    return;
  }

  const token = authHeader.split(' ')[1]; // Bearer <token>

  if (!token) {
    res.status(401).json({
      success: false,
      error: {
        code: 'INVALID_TOKEN_FORMAT',
        message: 'Invalid token format. Use: Bearer <token>',
      },
    });
    return;
  }

  try {
    const decoded = jwt.verify(token, config.jwt.secret) as {
      id: string;
      address: string;
      email?: string;
    };

    req.user = decoded;
    next();
  } catch (error: any) {
    logger.warn('JWT verification failed', {
      error: error.message,
      token: token.substring(0, 20) + '...',
    });

    if (error.name === 'TokenExpiredError') {
      res.status(401).json({
        success: false,
        error: {
          code: 'TOKEN_EXPIRED',
          message: 'Token has expired',
        },
      });
      return;
    }

    res.status(403).json({
      success: false,
      error: {
        code: 'INVALID_TOKEN',
        message: 'Invalid or malformed token',
      },
    });
  }
};

/**
 * Optional authentication - doesn't fail if no token provided
 */
export const optionalAuth = (
  req: AuthenticatedRequest,
  res: Response,
  next: NextFunction
): void => {
  const authHeader = req.headers.authorization;

  if (!authHeader) {
    next();
    return;
  }

  const token = authHeader.split(' ')[1];

  if (!token) {
    next();
    return;
  }

  try {
    const decoded = jwt.verify(token, config.jwt.secret) as {
      id: string;
      address: string;
      email?: string;
    };

    req.user = decoded;
  } catch (error) {
    // Silently fail - this is optional auth
  }

  next();
};

/**
 * Generate JWT token
 */
export const generateToken = (payload: {
  id: string;
  address: string;
  email?: string;
}): string => {
  return jwt.sign(payload, config.jwt.secret, {
    expiresIn: config.jwt.expiresIn,
  });
};

/**
 * Generate refresh token
 */
export const generateRefreshToken = (payload: {
  id: string;
  address: string;
}): string => {
  return jwt.sign(payload, config.jwt.refreshSecret, {
    expiresIn: config.jwt.refreshExpiresIn,
  });
};

/**
 * Verify refresh token
 */
export const verifyRefreshToken = (token: string): {
  id: string;
  address: string;
} | null => {
  try {
    return jwt.verify(token, config.jwt.refreshSecret) as {
      id: string;
      address: string;
    };
  } catch (error) {
    return null;
  }
};
