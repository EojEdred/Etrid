/**
 * Authentication System
 * JWT-based auth for multi-tenant operations center
 */

const jwt = require('jsonwebtoken');
const bcrypt = require('bcrypt');
const crypto = require('crypto');

class AuthSystem {
  constructor(config) {
    this.config = config;
    this.jwtSecret = config.auth?.jwtSecret || this.generateSecret();
    this.jwtExpiry = config.auth?.jwtExpiry || '7d';
    this.saltRounds = 10;
    this.sessions = new Map(); // In production, use Redis
  }

  generateSecret() {
    // Generate secure secret if not provided
    return crypto.randomBytes(64).toString('hex');
  }

  /**
   * Register new user
   */
  async register(userData) {
    const { email, password, name, organization } = userData;

    // Validate input
    if (!email || !password || !name) {
      throw new Error('Email, password, and name are required');
    }

    if (password.length < 8) {
      throw new Error('Password must be at least 8 characters');
    }

    if (!this.isValidEmail(email)) {
      throw new Error('Invalid email address');
    }

    // Check if user exists
    const existingUser = await this.getUserByEmail(email);
    if (existingUser) {
      throw new Error('User already exists');
    }

    // Hash password
    const passwordHash = await bcrypt.hash(password, this.saltRounds);

    // Create user
    const user = {
      id: this.generateUserId(),
      email: email.toLowerCase(),
      passwordHash,
      name,
      organization: organization || null,
      role: 'user', // user, admin
      tier: 'free', // free, pro, enterprise
      createdAt: Date.now(),
      lastLogin: null,
      active: true,
      emailVerified: false,
      apiKey: this.generateApiKey(),
      settings: {
        notifications: {
          email: true,
          telegram: false,
          discord: false
        },
        theme: 'dark',
        timezone: 'UTC'
      }
    };

    // Store user (in production, use database)
    await this.saveUser(user);

    // Send verification email
    await this.sendVerificationEmail(user);

    // Return user without sensitive data
    return this.sanitizeUser(user);
  }

  /**
   * Login user
   */
  async login(email, password, remember = false) {
    // Get user
    const user = await this.getUserByEmail(email.toLowerCase());
    if (!user) {
      throw new Error('Invalid credentials');
    }

    // Check if active
    if (!user.active) {
      throw new Error('Account is disabled');
    }

    // Verify password
    const validPassword = await bcrypt.compare(password, user.passwordHash);
    if (!validPassword) {
      throw new Error('Invalid credentials');
    }

    // Update last login
    user.lastLogin = Date.now();
    await this.saveUser(user);

    // Generate tokens
    const accessToken = this.generateAccessToken(user);
    const refreshToken = remember ? this.generateRefreshToken(user) : null;

    // Create session
    const session = {
      userId: user.id,
      accessToken,
      refreshToken,
      createdAt: Date.now(),
      expiresAt: Date.now() + (remember ? 30 * 24 * 60 * 60 * 1000 : 7 * 24 * 60 * 60 * 1000),
      userAgent: null, // Set from request
      ipAddress: null  // Set from request
    };

    this.sessions.set(accessToken, session);

    return {
      user: this.sanitizeUser(user),
      accessToken,
      refreshToken,
      expiresIn: remember ? '30d' : '7d'
    };
  }

  /**
   * Verify access token
   */
  verifyToken(token) {
    try {
      const decoded = jwt.verify(token, this.jwtSecret);

      // Check session exists
      const session = this.sessions.get(token);
      if (!session) {
        throw new Error('Session not found');
      }

      // Check expiry
      if (session.expiresAt < Date.now()) {
        this.sessions.delete(token);
        throw new Error('Session expired');
      }

      return decoded;
    } catch (err) {
      throw new Error('Invalid token: ' + err.message);
    }
  }

  /**
   * Refresh access token
   */
  async refreshAccessToken(refreshToken) {
    try {
      const decoded = jwt.verify(refreshToken, this.jwtSecret);
      const user = await this.getUserById(decoded.userId);

      if (!user || !user.active) {
        throw new Error('Invalid user');
      }

      // Generate new access token
      const newAccessToken = this.generateAccessToken(user);

      // Update session
      const session = Array.from(this.sessions.values())
        .find(s => s.userId === user.id && s.refreshToken === refreshToken);

      if (session) {
        this.sessions.delete(session.accessToken);
        session.accessToken = newAccessToken;
        session.createdAt = Date.now();
        this.sessions.set(newAccessToken, session);
      }

      return {
        accessToken: newAccessToken,
        expiresIn: '7d'
      };
    } catch (err) {
      throw new Error('Invalid refresh token');
    }
  }

  /**
   * Logout user
   */
  logout(token) {
    const session = this.sessions.get(token);
    if (session) {
      this.sessions.delete(token);
      if (session.refreshToken) {
        // Remove refresh token too
        for (const [key, sess] of this.sessions) {
          if (sess.refreshToken === session.refreshToken) {
            this.sessions.delete(key);
          }
        }
      }
    }
  }

  /**
   * Change password
   */
  async changePassword(userId, currentPassword, newPassword) {
    const user = await this.getUserById(userId);
    if (!user) {
      throw new Error('User not found');
    }

    // Verify current password
    const validPassword = await bcrypt.compare(currentPassword, user.passwordHash);
    if (!validPassword) {
      throw new Error('Current password is incorrect');
    }

    // Validate new password
    if (newPassword.length < 8) {
      throw new Error('New password must be at least 8 characters');
    }

    // Hash new password
    user.passwordHash = await bcrypt.hash(newPassword, this.saltRounds);
    await this.saveUser(user);

    // Invalidate all sessions except current
    this.invalidateUserSessions(userId);

    return { success: true };
  }

  /**
   * Reset password
   */
  async requestPasswordReset(email) {
    const user = await this.getUserByEmail(email.toLowerCase());
    if (!user) {
      // Don't reveal if user exists
      return { success: true };
    }

    // Generate reset token
    const resetToken = crypto.randomBytes(32).toString('hex');
    const resetExpiry = Date.now() + (60 * 60 * 1000); // 1 hour

    user.resetToken = resetToken;
    user.resetExpiry = resetExpiry;
    await this.saveUser(user);

    // Send reset email
    await this.sendPasswordResetEmail(user, resetToken);

    return { success: true };
  }

  async resetPassword(resetToken, newPassword) {
    // Find user by reset token
    const user = await this.getUserByResetToken(resetToken);
    if (!user) {
      throw new Error('Invalid or expired reset token');
    }

    // Check expiry
    if (user.resetExpiry < Date.now()) {
      throw new Error('Reset token has expired');
    }

    // Validate new password
    if (newPassword.length < 8) {
      throw new Error('Password must be at least 8 characters');
    }

    // Hash new password
    user.passwordHash = await bcrypt.hash(newPassword, this.saltRounds);
    user.resetToken = null;
    user.resetExpiry = null;
    await this.saveUser(user);

    // Invalidate all sessions
    this.invalidateUserSessions(user.id);

    return { success: true };
  }

  /**
   * Update user profile
   */
  async updateProfile(userId, updates) {
    const user = await this.getUserById(userId);
    if (!user) {
      throw new Error('User not found');
    }

    // Allow updating: name, organization, settings
    const allowedFields = ['name', 'organization', 'settings'];

    for (const field of allowedFields) {
      if (updates[field] !== undefined) {
        if (field === 'settings') {
          user.settings = { ...user.settings, ...updates.settings };
        } else {
          user[field] = updates[field];
        }
      }
    }

    await this.saveUser(user);
    return this.sanitizeUser(user);
  }

  /**
   * Verify email
   */
  async verifyEmail(token) {
    // Find user by verification token
    const user = await this.getUserByVerificationToken(token);
    if (!user) {
      throw new Error('Invalid verification token');
    }

    user.emailVerified = true;
    user.verificationToken = null;
    await this.saveUser(user);

    return { success: true };
  }

  /**
   * API Key authentication (for programmatic access)
   */
  async verifyApiKey(apiKey) {
    const user = await this.getUserByApiKey(apiKey);
    if (!user || !user.active) {
      throw new Error('Invalid API key');
    }

    return this.sanitizeUser(user);
  }

  async regenerateApiKey(userId) {
    const user = await this.getUserById(userId);
    if (!user) {
      throw new Error('User not found');
    }

    user.apiKey = this.generateApiKey();
    await this.saveUser(user);

    return { apiKey: user.apiKey };
  }

  // Helper methods

  generateAccessToken(user) {
    return jwt.sign(
      {
        userId: user.id,
        email: user.email,
        role: user.role,
        tier: user.tier
      },
      this.jwtSecret,
      { expiresIn: this.jwtExpiry }
    );
  }

  generateRefreshToken(user) {
    return jwt.sign(
      { userId: user.id, type: 'refresh' },
      this.jwtSecret,
      { expiresIn: '30d' }
    );
  }

  generateUserId() {
    return 'usr_' + crypto.randomBytes(16).toString('hex');
  }

  generateApiKey() {
    return 'etrid_' + crypto.randomBytes(32).toString('hex');
  }

  sanitizeUser(user) {
    const { passwordHash, resetToken, verificationToken, ...safeUser } = user;
    return safeUser;
  }

  isValidEmail(email) {
    const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
    return emailRegex.test(email);
  }

  invalidateUserSessions(userId) {
    for (const [token, session] of this.sessions) {
      if (session.userId === userId) {
        this.sessions.delete(token);
      }
    }
  }

  // Database methods (implement with actual database)

  async getUserByEmail(email) {
    // TODO: Implement with database
    // For now, use in-memory storage via database instance
    if (this.database) {
      return await this.database.getUserByEmail(email);
    }
    return null;
  }

  async getUserById(id) {
    if (this.database) {
      return await this.database.getUserById(id);
    }
    return null;
  }

  async getUserByApiKey(apiKey) {
    if (this.database) {
      return await this.database.getUserByApiKey(apiKey);
    }
    return null;
  }

  async getUserByResetToken(token) {
    if (this.database) {
      return await this.database.getUserByResetToken(token);
    }
    return null;
  }

  async getUserByVerificationToken(token) {
    if (this.database) {
      return await this.database.getUserByVerificationToken(token);
    }
    return null;
  }

  async saveUser(user) {
    if (this.database) {
      return await this.database.saveUser(user);
    }
  }

  async sendVerificationEmail(user) {
    // TODO: Implement email sending
    console.log(`Verification email would be sent to ${user.email}`);
  }

  async sendPasswordResetEmail(user, token) {
    // TODO: Implement email sending
    console.log(`Password reset email would be sent to ${user.email}`);
  }

  /**
   * Middleware for Express
   */
  middleware() {
    return async (req, res, next) => {
      try {
        // Check for token in header or cookie
        const token = req.headers.authorization?.replace('Bearer ', '') ||
                     req.cookies?.accessToken;

        if (!token) {
          return res.status(401).json({ error: 'No token provided' });
        }

        // Verify token
        const decoded = this.verifyToken(token);
        const user = await this.getUserById(decoded.userId);

        if (!user || !user.active) {
          return res.status(401).json({ error: 'Invalid user' });
        }

        // Attach user to request
        req.user = this.sanitizeUser(user);
        req.token = token;

        next();
      } catch (err) {
        res.status(401).json({ error: 'Unauthorized: ' + err.message });
      }
    };
  }

  /**
   * Optional middleware (doesn't fail if no token)
   */
  optionalMiddleware() {
    return async (req, res, next) => {
      try {
        const token = req.headers.authorization?.replace('Bearer ', '') ||
                     req.cookies?.accessToken;

        if (token) {
          const decoded = this.verifyToken(token);
          const user = await this.getUserById(decoded.userId);
          if (user && user.active) {
            req.user = this.sanitizeUser(user);
            req.token = token;
          }
        }
      } catch (err) {
        // Silently fail, just don't attach user
      }
      next();
    };
  }

  /**
   * Role-based access control middleware
   */
  requireRole(role) {
    return (req, res, next) => {
      if (!req.user) {
        return res.status(401).json({ error: 'Unauthorized' });
      }

      if (req.user.role !== role && req.user.role !== 'admin') {
        return res.status(403).json({ error: 'Forbidden' });
      }

      next();
    };
  }

  /**
   * Tier-based access control
   */
  requireTier(minTier) {
    const tierLevels = { free: 0, pro: 1, enterprise: 2 };

    return (req, res, next) => {
      if (!req.user) {
        return res.status(401).json({ error: 'Unauthorized' });
      }

      const userLevel = tierLevels[req.user.tier] || 0;
      const requiredLevel = tierLevels[minTier] || 0;

      if (userLevel < requiredLevel) {
        return res.status(403).json({
          error: 'Upgrade required',
          required: minTier,
          current: req.user.tier
        });
      }

      next();
    };
  }
}

module.exports = { AuthSystem };
