/**
 * Test setup file
 * Runs before each test suite
 */

// Extend Jest timeout for blockchain operations
jest.setTimeout(30000);

// Mock console methods to reduce noise in tests
global.console = {
  ...console,
  // Uncomment to suppress console.log during tests
  // log: jest.fn(),
  // Uncomment to suppress console.debug during tests
  // debug: jest.fn(),
  // Keep error and warn for debugging
  error: console.error,
  warn: console.warn,
};

// Global test utilities
export {};
