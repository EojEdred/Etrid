import AnalyticsService from '../services/AnalyticsService';

interface PerformanceMetric {
  name: string;
  startTime: number;
  endTime?: number;
  duration?: number;
  metadata?: Record<string, any>;
}

/**
 * Performance monitoring utility
 */
class PerformanceMonitor {
  private metrics: Map<string, PerformanceMetric> = new Map();
  private slowOperationThreshold = 1000; // 1 second

  /**
   * Start tracking a metric
   */
  public start(name: string, metadata?: Record<string, any>): void {
    this.metrics.set(name, {
      name,
      startTime: Date.now(),
      metadata,
    });
  }

  /**
   * End tracking a metric
   */
  public end(name: string): number | null {
    const metric = this.metrics.get(name);
    if (!metric) {
      console.warn(`Performance metric not found: ${name}`);
      return null;
    }

    const endTime = Date.now();
    const duration = endTime - metric.startTime;

    metric.endTime = endTime;
    metric.duration = duration;

    // Log slow operations
    if (duration > this.slowOperationThreshold) {
      console.warn(`Slow operation detected: ${name} took ${duration}ms`);
      this.logSlowOperation(metric);
    }

    // Report to analytics
    AnalyticsService.trackPerformance(
      this.categorizeMetric(name),
      duration,
      metric.metadata?.screen,
      metric.metadata
    );

    return duration;
  }

  /**
   * Measure async function execution time
   */
  public async measure<T>(
    name: string,
    fn: () => Promise<T>,
    metadata?: Record<string, any>
  ): Promise<T> {
    this.start(name, metadata);
    try {
      const result = await fn();
      this.end(name);
      return result;
    } catch (error) {
      this.end(name);
      throw error;
    }
  }

  /**
   * Measure sync function execution time
   */
  public measureSync<T>(
    name: string,
    fn: () => T,
    metadata?: Record<string, any>
  ): T {
    this.start(name, metadata);
    try {
      const result = fn();
      this.end(name);
      return result;
    } catch (error) {
      this.end(name);
      throw error;
    }
  }

  /**
   * Track screen render time
   */
  public trackScreenRender(screenName: string): () => void {
    const metricName = `screen_render_${screenName}`;
    this.start(metricName, { screen: screenName });

    return () => {
      this.end(metricName);
    };
  }

  /**
   * Track API call latency
   */
  public async trackAPICall<T>(
    endpoint: string,
    method: string,
    fn: () => Promise<T>
  ): Promise<T> {
    return this.measure(`api_${method}_${endpoint}`, fn, {
      endpoint,
      method,
    });
  }

  /**
   * Track component render time
   */
  public trackComponentRender(componentName: string): () => void {
    const metricName = `component_render_${componentName}`;
    this.start(metricName, { component: componentName });

    return () => {
      this.end(metricName);
    };
  }

  /**
   * Get all metrics
   */
  public getMetrics(): PerformanceMetric[] {
    return Array.from(this.metrics.values());
  }

  /**
   * Get metrics by category
   */
  public getMetricsByCategory(category: string): PerformanceMetric[] {
    return this.getMetrics().filter((metric) => metric.name.startsWith(category));
  }

  /**
   * Get slow operations
   */
  public getSlowOperations(): PerformanceMetric[] {
    return this.getMetrics().filter(
      (metric) => metric.duration && metric.duration > this.slowOperationThreshold
    );
  }

  /**
   * Clear metrics
   */
  public clear(): void {
    this.metrics.clear();
  }

  /**
   * Set slow operation threshold
   */
  public setSlowOperationThreshold(ms: number): void {
    this.slowOperationThreshold = ms;
  }

  /**
   * Get performance report
   */
  public getReport(): {
    totalMetrics: number;
    slowOperations: number;
    averageDuration: number;
    categories: Record<string, { count: number; avgDuration: number }>;
  } {
    const metrics = this.getMetrics();
    const slowOps = this.getSlowOperations();

    const totalDuration = metrics.reduce(
      (sum, m) => sum + (m.duration || 0),
      0
    );
    const avgDuration = metrics.length > 0 ? totalDuration / metrics.length : 0;

    // Group by category
    const categories: Record<string, { count: number; totalDuration: number }> = {};
    metrics.forEach((metric) => {
      const category = this.categorizeMetric(metric.name);
      if (!categories[category]) {
        categories[category] = { count: 0, totalDuration: 0 };
      }
      categories[category].count++;
      categories[category].totalDuration += metric.duration || 0;
    });

    const categoryStats: Record<string, { count: number; avgDuration: number }> = {};
    Object.entries(categories).forEach(([cat, stats]) => {
      categoryStats[cat] = {
        count: stats.count,
        avgDuration: stats.count > 0 ? stats.totalDuration / stats.count : 0,
      };
    });

    return {
      totalMetrics: metrics.length,
      slowOperations: slowOps.length,
      averageDuration: avgDuration,
      categories: categoryStats,
    };
  }

  /**
   * Log performance report
   */
  public logReport(): void {
    const report = this.getReport();
    console.log('=== Performance Report ===');
    console.log(`Total Metrics: ${report.totalMetrics}`);
    console.log(`Slow Operations: ${report.slowOperations}`);
    console.log(`Average Duration: ${report.averageDuration.toFixed(2)}ms`);
    console.log('Categories:');
    Object.entries(report.categories).forEach(([cat, stats]) => {
      console.log(
        `  ${cat}: ${stats.count} ops, ${stats.avgDuration.toFixed(2)}ms avg`
      );
    });
  }

  // Helper methods
  private categorizeMetric(name: string): 'screen_load' | 'api_call' | 'transaction_time' {
    if (name.startsWith('screen_')) return 'screen_load';
    if (name.startsWith('api_')) return 'api_call';
    return 'transaction_time';
  }

  private logSlowOperation(metric: PerformanceMetric): void {
    console.warn('=== Slow Operation Detected ===');
    console.warn(`Name: ${metric.name}`);
    console.warn(`Duration: ${metric.duration}ms`);
    console.warn(`Metadata:`, metric.metadata);
  }
}

export default new PerformanceMonitor();
