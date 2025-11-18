import db from '../database/client';
import { User, CreateUserDTO } from '../types';
import logger from '../utils/logger';

class UserRepository {
  /**
   * Find user by ID
   */
  async findById(id: string): Promise<User | null> {
    try {
      const result = await db.query<User>(
        'SELECT * FROM users WHERE id = $1',
        [id]
      );
      return result.rows[0] || null;
    } catch (error: any) {
      logger.error('Error finding user by ID', { id, error: error.message });
      throw error;
    }
  }

  /**
   * Find user by address
   */
  async findByAddress(address: string): Promise<User | null> {
    try {
      const result = await db.query<User>(
        'SELECT * FROM users WHERE address = $1',
        [address]
      );
      return result.rows[0] || null;
    } catch (error: any) {
      logger.error('Error finding user by address', { address, error: error.message });
      throw error;
    }
  }

  /**
   * Find user by email
   */
  async findByEmail(email: string): Promise<User | null> {
    try {
      const result = await db.query<User>(
        'SELECT * FROM users WHERE email = $1',
        [email]
      );
      return result.rows[0] || null;
    } catch (error: any) {
      logger.error('Error finding user by email', { email, error: error.message });
      throw error;
    }
  }

  /**
   * Create new user
   */
  async create(data: CreateUserDTO): Promise<User> {
    try {
      const result = await db.query<User>(
        `INSERT INTO users (address, email, phone)
         VALUES ($1, $2, $3)
         RETURNING *`,
        [data.address, data.email || null, data.phone || null]
      );
      return result.rows[0];
    } catch (error: any) {
      logger.error('Error creating user', { data, error: error.message });
      throw error;
    }
  }

  /**
   * Update user
   */
  async update(id: string, data: Partial<User>): Promise<User | null> {
    try {
      const fields: string[] = [];
      const values: any[] = [];
      let paramCount = 1;

      Object.entries(data).forEach(([key, value]) => {
        if (value !== undefined) {
          fields.push(`${key} = $${paramCount}`);
          values.push(value);
          paramCount++;
        }
      });

      if (fields.length === 0) {
        return this.findById(id);
      }

      values.push(id);

      const result = await db.query<User>(
        `UPDATE users SET ${fields.join(', ')}
         WHERE id = $${paramCount}
         RETURNING *`,
        values
      );

      return result.rows[0] || null;
    } catch (error: any) {
      logger.error('Error updating user', { id, data, error: error.message });
      throw error;
    }
  }

  /**
   * Update last login
   */
  async updateLastLogin(id: string): Promise<void> {
    try {
      await db.query(
        'UPDATE users SET last_login = CURRENT_TIMESTAMP WHERE id = $1',
        [id]
      );
    } catch (error: any) {
      logger.error('Error updating last login', { id, error: error.message });
      throw error;
    }
  }

  /**
   * Update KYC status
   */
  async updateKycStatus(
    id: string,
    status: 'pending' | 'verified' | 'rejected',
    level?: number
  ): Promise<User | null> {
    try {
      const result = await db.query<User>(
        `UPDATE users
         SET kyc_status = $1,
             kyc_level = COALESCE($2, kyc_level),
             kyc_verified_at = CASE WHEN $1 = 'verified' THEN CURRENT_TIMESTAMP ELSE kyc_verified_at END
         WHERE id = $3
         RETURNING *`,
        [status, level, id]
      );
      return result.rows[0] || null;
    } catch (error: any) {
      logger.error('Error updating KYC status', { id, status, error: error.message });
      throw error;
    }
  }

  /**
   * Enable 2FA
   */
  async enable2FA(id: string, secret: string): Promise<void> {
    try {
      await db.query(
        'UPDATE users SET two_factor_enabled = true, two_factor_secret = $1 WHERE id = $2',
        [secret, id]
      );
    } catch (error: any) {
      logger.error('Error enabling 2FA', { id, error: error.message });
      throw error;
    }
  }

  /**
   * Disable 2FA
   */
  async disable2FA(id: string): Promise<void> {
    try {
      await db.query(
        'UPDATE users SET two_factor_enabled = false, two_factor_secret = NULL WHERE id = $1',
        [id]
      );
    } catch (error: any) {
      logger.error('Error disabling 2FA', { id, error: error.message });
      throw error;
    }
  }

  /**
   * Get user statistics
   */
  async getUserStats(userId: string): Promise<any> {
    try {
      const result = await db.query(
        `SELECT
           (SELECT COUNT(*) FROM transactions WHERE user_id = $1 AND status = 'confirmed') as transaction_count,
           (SELECT COUNT(*) FROM staking_positions WHERE user_id = $1 AND status = 'active') as active_stakes,
           (SELECT COALESCE(SUM(amount), 0) FROM staking_positions WHERE user_id = $1 AND status = 'active') as total_staked,
           (SELECT COALESCE(SUM(rewards_earned), 0) FROM staking_positions WHERE user_id = $1) as total_rewards,
           (SELECT COUNT(*) FROM governance_votes WHERE user_id = $1) as votes_cast`,
        [userId]
      );
      return result.rows[0];
    } catch (error: any) {
      logger.error('Error getting user stats', { userId, error: error.message });
      throw error;
    }
  }

  /**
   * Delete user (soft delete by marking as inactive)
   */
  async delete(id: string): Promise<void> {
    try {
      await db.query('DELETE FROM users WHERE id = $1', [id]);
    } catch (error: any) {
      logger.error('Error deleting user', { id, error: error.message });
      throw error;
    }
  }
}

export default new UserRepository();
