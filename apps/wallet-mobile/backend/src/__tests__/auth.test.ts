import request from 'supertest';
import { server } from '../server';

describe('Auth API', () => {
  describe('POST /api/v1/auth/login', () => {
    it('should return 401 for invalid signature', async () => {
      const response = await request(server.app)
        .post('/api/v1/auth/login')
        .send({
          address: '5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY',
          signature: 'invalid_signature',
          message: 'Sign this message',
        });

      expect(response.status).toBe(401);
      expect(response.body.success).toBe(false);
      expect(response.body.error.code).toBe('INVALID_SIGNATURE');
    });

    it('should validate request body', async () => {
      const response = await request(server.app)
        .post('/api/v1/auth/login')
        .send({
          address: 'invalid_address',
        });

      expect(response.status).toBe(400);
      expect(response.body.success).toBe(false);
      expect(response.body.error.code).toBe('VALIDATION_ERROR');
    });
  });

  describe('GET /api/v1/auth/nonce/:address', () => {
    it('should return nonce for valid address', async () => {
      const address = '5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY';
      const response = await request(server.app).get(`/api/v1/auth/nonce/${address}`);

      expect(response.status).toBe(200);
      expect(response.body.success).toBe(true);
      expect(response.body.data).toHaveProperty('nonce');
      expect(response.body.data).toHaveProperty('message');
    });
  });

  describe('POST /api/v1/auth/refresh', () => {
    it('should return 401 for invalid refresh token', async () => {
      const response = await request(server.app)
        .post('/api/v1/auth/refresh')
        .send({
          refreshToken: 'invalid_token',
        });

      expect(response.status).toBe(401);
      expect(response.body.success).toBe(false);
    });
  });
});
