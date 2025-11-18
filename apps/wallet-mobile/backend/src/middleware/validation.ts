import Joi from 'joi';
import { Request, Response, NextFunction } from 'express';
import { ApiResponse } from '../types';

/**
 * Validate request body, query, or params against Joi schema
 */
export const validate = (schema: Joi.ObjectSchema, property: 'body' | 'query' | 'params' = 'body') => {
  return (req: Request, res: Response, next: NextFunction): void => {
    const { error, value } = schema.validate(req[property], {
      abortEarly: false,
      stripUnknown: true,
    });

    if (error) {
      const errors = error.details.map((detail) => ({
        field: detail.path.join('.'),
        message: detail.message,
      }));

      const response: ApiResponse = {
        success: false,
        error: {
          code: 'VALIDATION_ERROR',
          message: 'Request validation failed',
          details: errors,
        },
        timestamp: new Date().toISOString(),
      };

      res.status(400).json(response);
      return;
    }

    // Replace request data with validated data
    req[property] = value;
    next();
  };
};

// ============================================================================
// COMMON VALIDATION SCHEMAS
// ============================================================================

export const addressSchema = Joi.string()
  .pattern(/^[1-9A-HJ-NP-Za-km-z]{48}$/)
  .required()
  .messages({
    'string.pattern.base': 'Invalid SS58 address format',
  });

export const txHashSchema = Joi.string()
  .pattern(/^0x[a-fA-F0-9]{64}$/)
  .required()
  .messages({
    'string.pattern.base': 'Invalid transaction hash format',
  });

export const uuidSchema = Joi.string()
  .uuid()
  .required()
  .messages({
    'string.uuid': 'Invalid UUID format',
  });

export const paginationSchema = Joi.object({
  page: Joi.number().integer().min(1).default(1),
  limit: Joi.number().integer().min(1).max(100).default(20),
});

// ============================================================================
// AUTH SCHEMAS
// ============================================================================

export const loginSchema = Joi.object({
  address: addressSchema,
  signature: Joi.string().required(),
  message: Joi.string().required(),
});

export const refreshTokenSchema = Joi.object({
  refreshToken: Joi.string().required(),
});

// ============================================================================
// TRANSFER SCHEMAS
// ============================================================================

export const transferSchema = Joi.object({
  to: addressSchema,
  amount: Joi.string()
    .pattern(/^\d+(\.\d+)?$/)
    .required()
    .messages({
      'string.pattern.base': 'Amount must be a valid number',
    }),
  asset: Joi.string().valid('ETR', 'BTC', 'ETH', 'USDT').required(),
  memo: Joi.string().max(256).optional(),
});

// ============================================================================
// STAKING SCHEMAS
// ============================================================================

export const stakeSchema = Joi.object({
  validator_address: addressSchema,
  amount: Joi.string()
    .pattern(/^\d+(\.\d+)?$/)
    .required(),
  auto_compound: Joi.boolean().default(false),
});

export const unstakeSchema = Joi.object({
  position_id: uuidSchema,
  amount: Joi.string()
    .pattern(/^\d+(\.\d+)?$/)
    .optional(),
});

// ============================================================================
// GOVERNANCE SCHEMAS
// ============================================================================

export const voteSchema = Joi.object({
  proposal_id: Joi.number().integer().positive().required(),
  support: Joi.boolean().required(),
  conviction: Joi.number().integer().min(0).max(6).default(0),
  balance: Joi.string()
    .pattern(/^\d+(\.\d+)?$/)
    .required(),
});

// ============================================================================
// ATM SCHEMAS
// ============================================================================

export const atmWithdrawalSchema = Joi.object({
  amount_usd: Joi.number().positive().min(20).max(3000).required(),
  asset: Joi.string().valid('ETR', 'BTC', 'ETH').required(),
  atm_partner: Joi.string()
    .valid('Coinme', 'Bitcoin Depot', 'CoinFlip')
    .required(),
  atm_location_id: Joi.string().optional(),
});

export const atmLocationSchema = Joi.object({
  lat: Joi.number().min(-90).max(90).required(),
  lng: Joi.number().min(-180).max(180).required(),
  radius: Joi.number().positive().max(50000).default(5000), // Default 5km
  partner: Joi.string()
    .valid('Coinme', 'Bitcoin Depot', 'CoinFlip', 'all')
    .default('all'),
});

// ============================================================================
// BRIDGE SCHEMAS
// ============================================================================

export const bridgeTransferSchema = Joi.object({
  from_chain: Joi.string()
    .valid('BTC', 'ETH', 'BSC', 'MATIC', 'AVAX')
    .required(),
  to_chain: Joi.string().valid('FLARE').default('FLARE'),
  from_address: Joi.string().required(),
  to_address: addressSchema,
  from_asset: Joi.string().required(),
  amount: Joi.string()
    .pattern(/^\d+(\.\d+)?$/)
    .required(),
});

// ============================================================================
// GPU SCHEMAS
// ============================================================================

export const gpuSearchSchema = Joi.object({
  min_vram: Joi.number().integer().min(4).optional(),
  max_price: Joi.number().positive().optional(),
  gpu_type: Joi.string().optional(),
  provider: Joi.string().valid('Vast.ai', 'RunPod', 'all').default('all'),
});

export const gpuRentSchema = Joi.object({
  gpu_id: Joi.string().required(),
  duration_hours: Joi.number().integer().min(1).max(720).required(), // Max 30 days
});

// ============================================================================
// USER SCHEMAS
// ============================================================================

export const updateProfileSchema = Joi.object({
  email: Joi.string().email().optional(),
  phone: Joi.string()
    .pattern(/^\+?[1-9]\d{1,14}$/)
    .optional(),
  preferences: Joi.object().optional(),
});
