/**
 * Firebase Crashlytics / Error Reporting
 *
 * For web, we use console error reporting and can integrate with
 * third-party services like Sentry. Firebase Crashlytics is primarily
 * for mobile apps (iOS/Android).
 *
 * This module provides a unified interface for error reporting that can
 * be extended to support various error tracking services.
 */

/**
 * Error severity levels
 */
export enum ErrorSeverity {
  DEBUG = 'debug',
  INFO = 'info',
  WARNING = 'warning',
  ERROR = 'error',
  FATAL = 'fatal',
}

/**
 * Error context information
 */
export interface ErrorContext {
  userId?: string;
  screen?: string;
  action?: string;
  [key: string]: any;
}

/**
 * Error log entry
 */
interface ErrorLog {
  timestamp: number;
  severity: ErrorSeverity;
  message: string;
  error?: Error;
  context?: ErrorContext;
}

// Store recent errors for debugging
const errorLogs: ErrorLog[] = [];
const MAX_ERROR_LOGS = 100;

/**
 * Report an error
 *
 * @param error Error object or message
 * @param context Additional context
 * @param severity Error severity level
 */
export function reportError(
  error: Error | string,
  context?: ErrorContext,
  severity: ErrorSeverity = ErrorSeverity.ERROR
): void {
  const errorMessage = typeof error === 'string' ? error : error.message;
  const errorObj = typeof error === 'string' ? new Error(error) : error;

  // Log to console
  console.error(`[${severity.toUpperCase()}] ${errorMessage}`, {
    error: errorObj,
    context,
  });

  // Store in local log
  addErrorLog({
    timestamp: Date.now(),
    severity,
    message: errorMessage,
    error: errorObj,
    context,
  });

  // In production, send to error tracking service
  if (process.env.NODE_ENV === 'production') {
    sendToErrorTracking(errorObj, context, severity);
  }
}

/**
 * Report a non-fatal error
 *
 * @param error Error object or message
 * @param context Additional context
 */
export function reportNonFatalError(
  error: Error | string,
  context?: ErrorContext
): void {
  reportError(error, context, ErrorSeverity.WARNING);
}

/**
 * Report a fatal error
 *
 * @param error Error object or message
 * @param context Additional context
 */
export function reportFatalError(
  error: Error | string,
  context?: ErrorContext
): void {
  reportError(error, context, ErrorSeverity.FATAL);
}

/**
 * Log a message (for breadcrumbs)
 *
 * @param message Log message
 * @param severity Severity level
 */
export function log(
  message: string,
  severity: ErrorSeverity = ErrorSeverity.INFO
): void {
  console.log(`[${severity.toUpperCase()}] ${message}`);

  addErrorLog({
    timestamp: Date.now(),
    severity,
    message,
  });
}

/**
 * Set user context for error reports
 *
 * @param userId User identifier (should be anonymized/hashed)
 * @param properties Additional user properties
 */
export function setUserContext(
  userId: string,
  properties?: Record<string, any>
): void {
  if (process.env.NODE_ENV === 'production') {
    // Send to error tracking service
    console.log('Setting user context:', { userId, ...properties });
    // Sentry.setUser({ id: userId, ...properties });
  }
}

/**
 * Clear user context
 */
export function clearUserContext(): void {
  if (process.env.NODE_ENV === 'production') {
    console.log('Clearing user context');
    // Sentry.setUser(null);
  }
}

/**
 * Set custom attribute for error reports
 *
 * @param key Attribute key
 * @param value Attribute value
 */
export function setAttribute(key: string, value: string | number | boolean): void {
  if (process.env.NODE_ENV === 'production') {
    console.log(`Setting attribute: ${key} = ${value}`);
    // Sentry.setTag(key, value);
  }
}

/**
 * Set multiple custom attributes
 *
 * @param attributes Key-value pairs of attributes
 */
export function setAttributes(attributes: Record<string, string | number | boolean>): void {
  Object.entries(attributes).forEach(([key, value]) => {
    setAttribute(key, value);
  });
}

/**
 * Add breadcrumb for error context
 *
 * @param message Breadcrumb message
 * @param data Additional data
 * @param category Breadcrumb category
 */
export function addBreadcrumb(
  message: string,
  data?: Record<string, any>,
  category?: string
): void {
  console.log(`[Breadcrumb] ${category || 'general'}: ${message}`, data);

  if (process.env.NODE_ENV === 'production') {
    // Sentry.addBreadcrumb({
    //   message,
    //   data,
    //   category,
    //   level: 'info',
    // });
  }
}

/**
 * Handle uncaught errors
 */
export function setupGlobalErrorHandlers(): void {
  if (typeof window === 'undefined') {
    return;
  }

  // Handle uncaught errors
  window.addEventListener('error', (event) => {
    reportError(event.error || event.message, {
      filename: event.filename,
      lineno: event.lineno,
      colno: event.colno,
    });
  });

  // Handle unhandled promise rejections
  window.addEventListener('unhandledrejection', (event) => {
    reportError(
      event.reason || 'Unhandled Promise Rejection',
      {
        promise: event.promise,
      },
      ErrorSeverity.ERROR
    );
  });

  console.log('Global error handlers set up');
}

/**
 * Get recent error logs
 *
 * @param count Number of recent logs to return
 * @returns Array of recent error logs
 */
export function getRecentErrorLogs(count: number = 10): ErrorLog[] {
  return errorLogs.slice(-count);
}

/**
 * Clear error logs
 */
export function clearErrorLogs(): void {
  errorLogs.length = 0;
}

/**
 * Export error logs for debugging
 *
 * @returns JSON string of error logs
 */
export function exportErrorLogs(): string {
  return JSON.stringify(errorLogs, null, 2);
}

// Private helper functions

/**
 * Add error to local log
 */
function addErrorLog(log: ErrorLog): void {
  errorLogs.push(log);

  // Keep only recent logs
  if (errorLogs.length > MAX_ERROR_LOGS) {
    errorLogs.shift();
  }
}

/**
 * Send error to tracking service (Sentry, etc.)
 */
function sendToErrorTracking(
  error: Error,
  context?: ErrorContext,
  severity?: ErrorSeverity
): void {
  // Placeholder for error tracking service integration
  // Uncomment and configure when using Sentry or similar service

  /*
  if (typeof Sentry !== 'undefined') {
    Sentry.withScope((scope) => {
      // Add context
      if (context) {
        Object.entries(context).forEach(([key, value]) => {
          scope.setExtra(key, value);
        });
      }

      // Set severity
      if (severity) {
        scope.setLevel(mapSeverityToSentryLevel(severity));
      }

      // Capture exception
      Sentry.captureException(error);
    });
  }
  */

  // Log that error would be sent in production
  console.log('Would send to error tracking:', {
    error: error.message,
    context,
    severity,
  });
}

/**
 * Map our severity levels to Sentry levels
 */
function mapSeverityToSentryLevel(severity: ErrorSeverity): string {
  const mapping: Record<ErrorSeverity, string> = {
    [ErrorSeverity.DEBUG]: 'debug',
    [ErrorSeverity.INFO]: 'info',
    [ErrorSeverity.WARNING]: 'warning',
    [ErrorSeverity.ERROR]: 'error',
    [ErrorSeverity.FATAL]: 'fatal',
  };

  return mapping[severity] || 'error';
}

/**
 * Test crash reporting (development only)
 */
export function testCrashReporting(): void {
  if (process.env.NODE_ENV === 'production') {
    console.warn('Test crash reporting should not be used in production');
    return;
  }

  console.log('Testing crash reporting...');

  // Test non-fatal error
  reportNonFatalError('This is a test non-fatal error', {
    test: true,
    feature: 'crash_reporting',
  });

  // Test regular error
  reportError(new Error('This is a test error'), {
    test: true,
    feature: 'crash_reporting',
  });

  console.log('Crash reporting tests complete. Check console and error tracking service.');
}

// Initialize global error handlers
if (typeof window !== 'undefined') {
  setupGlobalErrorHandlers();
}

// Export for integration with Sentry or other services
export default {
  reportError,
  reportNonFatalError,
  reportFatalError,
  log,
  setUserContext,
  clearUserContext,
  setAttribute,
  setAttributes,
  addBreadcrumb,
  setupGlobalErrorHandlers,
  getRecentErrorLogs,
  clearErrorLogs,
  exportErrorLogs,
  testCrashReporting,
};
