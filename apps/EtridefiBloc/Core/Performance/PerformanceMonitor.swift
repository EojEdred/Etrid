//
//  PerformanceMonitor.swift
//  EtridWallet
//
//  Created on 2025-01-22.
//  Copyright © 2025 Ëtrid. All rights reserved.
//

import Foundation
import UIKit

/// Monitor app performance and detect bottlenecks
actor PerformanceMonitor {
    static let shared = PerformanceMonitor()

    // MARK: - Types

    struct PerformanceMetric {
        let id: String
        let operation: String
        let startTime: Date
        var endTime: Date?
        var duration: TimeInterval? {
            guard let endTime = endTime else { return nil }
            return endTime.timeIntervalSince(startTime)
        }
        var metadata: [String: Any]

        init(operation: String, metadata: [String: Any] = [:]) {
            self.id = UUID().uuidString
            self.operation = operation
            self.startTime = Date()
            self.metadata = metadata
        }
    }

    struct MemorySnapshot {
        let timestamp: Date
        let usedMemory: UInt64
        let availableMemory: UInt64
        let totalMemory: UInt64

        var usagePercentage: Double {
            guard totalMemory > 0 else { return 0 }
            return Double(usedMemory) / Double(totalMemory) * 100
        }
    }

    struct CPUSnapshot {
        let timestamp: Date
        let usage: Double
    }

    enum PerformanceThreshold {
        static let slowOperation: TimeInterval = 1.0 // 1 second
        static let verySlowOperation: TimeInterval = 3.0 // 3 seconds
        static let highMemoryUsage: Double = 80.0 // 80%
        static let criticalMemoryUsage: Double = 90.0 // 90%
        static let highCPUUsage: Double = 70.0 // 70%
    }

    // MARK: - Properties

    private var activeMetrics: [String: PerformanceMetric] = [:]
    private var completedMetrics: [PerformanceMetric] = []
    private var memorySnapshots: [MemorySnapshot] = []
    private var cpuSnapshots: [CPUSnapshot] = []
    private let maxStoredMetrics = 100
    private var isMonitoring = false
    private var monitoringTask: Task<Void, Never>?

    // MARK: - Initialization

    private init() {}

    // MARK: - Operation Tracking

    /// Start measuring an operation
    func startMeasuring(_ operation: String, metadata: [String: Any] = [:]) -> String {
        let metric = PerformanceMetric(operation: operation, metadata: metadata)
        activeMetrics[metric.id] = metric

        Task {
            await Logger.shared.debug("Started measuring: \(operation)", category: .performance)
        }

        return metric.id
    }

    /// End measuring an operation
    func endMeasuring(_ id: String, metadata: [String: Any] = [:]) {
        guard var metric = activeMetrics[id] else {
            Task {
                await Logger.shared.warning("Attempted to end unknown metric: \(id)", category: .performance)
            }
            return
        }

        metric.endTime = Date()
        metric.metadata.merge(metadata) { _, new in new }

        guard let duration = metric.duration else { return }

        // Log performance
        Task {
            await Logger.shared.info(
                "⏱️ \(metric.operation) took \(String(format: "%.2f", duration * 1000))ms",
                category: .performance
            )
        }

        // Alert if slow
        if duration > PerformanceThreshold.verySlowOperation {
            Task {
                await Logger.shared.warning(
                    "🐌 Very slow operation: \(metric.operation) took \(String(format: "%.2f", duration))s",
                    category: .performance
                )
            }
        } else if duration > PerformanceThreshold.slowOperation {
            Task {
                await Logger.shared.warning(
                    "⚠️ Slow operation: \(metric.operation) took \(String(format: "%.2f", duration))s",
                    category: .performance
                )
            }
        }

        // Track in analytics
        Task {
            await AnalyticsManager.shared.trackPerformanceMetric(
                AnalyticsManager.PerformanceMetric(
                    operation: metric.operation,
                    duration: duration,
                    timestamp: metric.startTime,
                    success: true,
                    metadata: metric.metadata.mapValues { AnyCodable($0) }
                )
            )
        }

        // Store completed metric
        completedMetrics.append(metric)
        if completedMetrics.count > maxStoredMetrics {
            completedMetrics.removeFirst(completedMetrics.count - maxStoredMetrics)
        }

        // Remove from active
        activeMetrics.removeValue(forKey: id)
    }

    /// Measure an async operation
    func measure<T>(
        _ operation: String,
        metadata: [String: Any] = [:],
        work: () async throws -> T
    ) async rethrows -> T {
        let id = startMeasuring(operation, metadata: metadata)
        defer { endMeasuring(id) }
        return try await work()
    }

    // MARK: - Memory Monitoring

    func captureMemorySnapshot() {
        let snapshot = getCurrentMemoryUsage()
        memorySnapshots.append(snapshot)

        // Keep only recent snapshots
        if memorySnapshots.count > maxStoredMetrics {
            memorySnapshots.removeFirst(memorySnapshots.count - maxStoredMetrics)
        }

        // Alert on high usage
        if snapshot.usagePercentage > PerformanceThreshold.criticalMemoryUsage {
            Task {
                await Logger.shared.critical(
                    "🔴 Critical memory usage: \(String(format: "%.1f", snapshot.usagePercentage))%",
                    category: .performance
                )
            }
        } else if snapshot.usagePercentage > PerformanceThreshold.highMemoryUsage {
            Task {
                await Logger.shared.warning(
                    "⚠️ High memory usage: \(String(format: "%.1f", snapshot.usagePercentage))%",
                    category: .performance
                )
            }
        }
    }

    private func getCurrentMemoryUsage() -> MemorySnapshot {
        var info = mach_task_basic_info()
        var count = mach_msg_type_number_t(MemoryLayout<mach_task_basic_info>.size) / 4

        let kerr: kern_return_t = withUnsafeMutablePointer(to: &info) {
            $0.withMemoryRebound(to: integer_t.self, capacity: 1) {
                task_info(
                    mach_task_self_,
                    task_flavor_t(MACH_TASK_BASIC_INFO),
                    $0,
                    &count
                )
            }
        }

        let usedMemory = kerr == KERN_SUCCESS ? info.resident_size : 0

        // Get total system memory
        let totalMemory = ProcessInfo.processInfo.physicalMemory
        let availableMemory = totalMemory > usedMemory ? totalMemory - usedMemory : 0

        return MemorySnapshot(
            timestamp: Date(),
            usedMemory: usedMemory,
            availableMemory: availableMemory,
            totalMemory: totalMemory
        )
    }

    func getMemoryUsageReport() -> [String: Any] {
        guard let latest = memorySnapshots.last else {
            return [:]
        }

        let avgUsage = memorySnapshots.isEmpty ? 0 : memorySnapshots.reduce(0.0) { $0 + $1.usagePercentage } / Double(memorySnapshots.count)
        let maxUsage = memorySnapshots.map { $0.usagePercentage }.max() ?? 0

        return [
            "current_usage_mb": Double(latest.usedMemory) / 1024.0 / 1024.0,
            "current_usage_percent": latest.usagePercentage,
            "average_usage_percent": avgUsage,
            "max_usage_percent": maxUsage,
            "total_memory_mb": Double(latest.totalMemory) / 1024.0 / 1024.0
        ]
    }

    // MARK: - CPU Monitoring

    func captureCPUSnapshot() {
        let usage = getCurrentCPUUsage()
        let snapshot = CPUSnapshot(timestamp: Date(), usage: usage)

        cpuSnapshots.append(snapshot)

        // Keep only recent snapshots
        if cpuSnapshots.count > maxStoredMetrics {
            cpuSnapshots.removeFirst(cpuSnapshots.count - maxStoredMetrics)
        }

        // Alert on high usage
        if usage > PerformanceThreshold.highCPUUsage {
            Task {
                await Logger.shared.warning(
                    "⚠️ High CPU usage: \(String(format: "%.1f", usage))%",
                    category: .performance
                )
            }
        }
    }

    private func getCurrentCPUUsage() -> Double {
        var totalUsageOfCPU: Double = 0.0
        var threadsList: thread_act_array_t?
        var threadsCount = mach_msg_type_number_t(0)

        let threadsResult = withUnsafeMutablePointer(to: &threadsList) {
            task_threads(mach_task_self_, $0, &threadsCount)
        }

        guard threadsResult == KERN_SUCCESS, let threads = threadsList else {
            return 0.0
        }

        for index in 0..<threadsCount {
            var threadInfo = thread_basic_info()
            var threadInfoCount = mach_msg_type_number_t(THREAD_INFO_MAX)

            let infoResult = withUnsafeMutablePointer(to: &threadInfo) {
                $0.withMemoryRebound(to: integer_t.self, capacity: 1) {
                    thread_info(
                        threads[Int(index)],
                        thread_flavor_t(THREAD_BASIC_INFO),
                        $0,
                        &threadInfoCount
                    )
                }
            }

            guard infoResult == KERN_SUCCESS else { continue }

            let threadBasicInfo = threadInfo
            if threadBasicInfo.flags & TH_FLAGS_IDLE == 0 {
                totalUsageOfCPU += Double(threadBasicInfo.cpu_usage) / Double(TH_USAGE_SCALE) * 100.0
            }
        }

        vm_deallocate(
            mach_task_self_,
            vm_address_t(bitPattern: threads),
            vm_size_t(Int(threadsCount) * MemoryLayout<thread_t>.stride)
        )

        return totalUsageOfCPU
    }

    func getCPUUsageReport() -> [String: Any] {
        guard !cpuSnapshots.isEmpty else {
            return [:]
        }

        let current = cpuSnapshots.last?.usage ?? 0
        let avg = cpuSnapshots.reduce(0.0) { $0 + $1.usage } / Double(cpuSnapshots.count)
        let max = cpuSnapshots.map { $0.usage }.max() ?? 0

        return [
            "current_usage_percent": current,
            "average_usage_percent": avg,
            "max_usage_percent": max
        ]
    }

    // MARK: - Network Latency

    func measureNetworkLatency(to endpoint: String, work: () async throws -> Void) async rethrows {
        let startTime = Date()
        var success = false

        do {
            try await work()
            success = true
        } catch {
            success = false
            throw error
        }

        let duration = Date().timeIntervalSince(startTime)

        Task {
            await AnalyticsManager.shared.trackNetworkLatency(
                endpoint: endpoint,
                duration: duration,
                success: success
            )
        }

        if duration > 5.0 {
            Task {
                await Logger.shared.warning(
                    "Slow network request to \(endpoint): \(String(format: "%.2f", duration))s",
                    category: .network
                )
            }
        }
    }

    // MARK: - Frame Rate Monitoring

    @MainActor
    func startFrameRateMonitoring() {
        // This would integrate with CADisplayLink for real-time FPS monitoring
        // For now, we just log that monitoring started
        Task {
            await Logger.shared.info("Frame rate monitoring started", category: .performance)
        }
    }

    @MainActor
    func stopFrameRateMonitoring() {
        Task {
            await Logger.shared.info("Frame rate monitoring stopped", category: .performance)
        }
    }

    // MARK: - Continuous Monitoring

    func startContinuousMonitoring(interval: TimeInterval = 30.0) {
        guard !isMonitoring else { return }

        isMonitoring = true

        monitoringTask = Task {
            while isMonitoring {
                captureMemorySnapshot()
                captureCPUSnapshot()

                try? await Task.sleep(nanoseconds: UInt64(interval * 1_000_000_000))
            }
        }

        Task {
            await Logger.shared.info("Started continuous performance monitoring", category: .performance)
        }
    }

    func stopContinuousMonitoring() {
        isMonitoring = false
        monitoringTask?.cancel()
        monitoringTask = nil

        Task {
            await Logger.shared.info("Stopped continuous performance monitoring", category: .performance)
        }
    }

    // MARK: - Reporting

    func getPerformanceReport() -> [String: Any] {
        let slowOperations = completedMetrics.filter {
            guard let duration = $0.duration else { return false }
            return duration > PerformanceThreshold.slowOperation
        }

        let avgDuration = completedMetrics.compactMap { $0.duration }.isEmpty ? 0 :
            completedMetrics.compactMap { $0.duration }.reduce(0, +) / Double(completedMetrics.compactMap { $0.duration }.count)

        return [
            "total_operations": completedMetrics.count,
            "slow_operations": slowOperations.count,
            "average_duration_ms": avgDuration * 1000,
            "active_operations": activeMetrics.count,
            "memory": getMemoryUsageReport(),
            "cpu": getCPUUsageReport()
        ]
    }

    func getSlowOperations(threshold: TimeInterval = PerformanceThreshold.slowOperation) -> [PerformanceMetric] {
        return completedMetrics.filter {
            guard let duration = $0.duration else { return false }
            return duration > threshold
        }
    }

    func clearMetrics() {
        completedMetrics.removeAll()
        memorySnapshots.removeAll()
        cpuSnapshots.removeAll()
    }
}

// MARK: - Global Convenience Functions

/// Measure performance of an operation
func measurePerformance<T>(
    _ operation: String,
    work: () async throws -> T
) async rethrows -> T {
    return try await PerformanceMonitor.shared.measure(operation, work: work)
}

/// Start measuring an operation
func startMeasuring(_ operation: String) -> String {
    Task {
        return await PerformanceMonitor.shared.startMeasuring(operation)
    }
    return ""
}

/// End measuring an operation
func endMeasuring(_ id: String) {
    Task {
        await PerformanceMonitor.shared.endMeasuring(id)
    }
}
