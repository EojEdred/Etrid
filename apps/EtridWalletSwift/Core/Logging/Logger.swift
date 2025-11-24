//
//  Logger.swift
//  EtridWallet
//
//  Created on 2025-01-22.
//  Copyright © 2025 Ëtrid. All rights reserved.
//

import Foundation
import OSLog

/// Centralized logging system with privacy-safe logging and file rotation
actor Logger {
    static let shared = Logger()

    // MARK: - Types

    /// Log severity levels
    enum LogLevel: Int, Comparable {
        case debug = 0
        case info = 1
        case warning = 2
        case error = 3
        case critical = 4

        var emoji: String {
            switch self {
            case .debug: return "🔍"
            case .info: return "ℹ️"
            case .warning: return "⚠️"
            case .error: return "❌"
            case .critical: return "🔥"
            }
        }

        var osLogType: OSLogType {
            switch self {
            case .debug: return .debug
            case .info: return .info
            case .warning: return .default
            case .error: return .error
            case .critical: return .fault
            }
        }

        static func < (lhs: LogLevel, rhs: LogLevel) -> Bool {
            lhs.rawValue < rhs.rawValue
        }
    }

    /// Log categories for filtering and organization
    enum Category: String, CaseIterable {
        case network = "Network"
        case crypto = "Crypto"
        case ui = "UI"
        case security = "Security"
        case transaction = "Transaction"
        case wallet = "Wallet"
        case storage = "Storage"
        case analytics = "Analytics"
        case performance = "Performance"
        case general = "General"

        var subsystem: String {
            "com.etrid.wallet.\(rawValue.lowercased())"
        }
    }

    // MARK: - Properties

    private let osLog: OSLog
    private var logFileURL: URL?
    private let maxLogFileSize: Int = 5_000_000 // 5 MB
    private let maxLogFiles: Int = 5
    private var minLogLevel: LogLevel = .debug
    private var enabledCategories: Set<Category> = Set(Category.allCases)
    private let dateFormatter: DateFormatter
    private let fileManager = FileManager.default

    // MARK: - Initialization

    private init() {
        self.osLog = OSLog(subsystem: "com.etrid.wallet", category: "app")

        self.dateFormatter = DateFormatter()
        self.dateFormatter.dateFormat = "yyyy-MM-dd HH:mm:ss.SSS"
        self.dateFormatter.timeZone = TimeZone.current

        setupLogFile()
    }

    // MARK: - Configuration

    /// Set minimum log level (logs below this level will be ignored)
    func setMinimumLogLevel(_ level: LogLevel) {
        self.minLogLevel = level
    }

    /// Enable/disable specific categories
    func setCategory(_ category: Category, enabled: Bool) {
        if enabled {
            enabledCategories.insert(category)
        } else {
            enabledCategories.remove(category)
        }
    }

    /// Enable all categories
    func enableAllCategories() {
        enabledCategories = Set(Category.allCases)
    }

    // MARK: - Logging

    /// Main logging method
    func log(
        _ message: String,
        level: LogLevel = .info,
        category: Category = .general,
        file: String = #file,
        function: String = #function,
        line: Int = #line
    ) {
        // Check if we should log this
        guard level >= minLogLevel && enabledCategories.contains(category) else {
            return
        }

        // Sanitize sensitive information
        let sanitized = sanitizeMessage(message)

        // Create log entry
        let fileName = (file as NSString).lastPathComponent
        let timestamp = dateFormatter.string(from: Date())
        let logMessage = "[\(timestamp)] [\(category.rawValue)] \(level.emoji) \(sanitized) (\(fileName):\(line) \(function))"

        // Log to system
        logToOS(sanitized, level: level, category: category)

        // Log to file
        appendToFile(logMessage)

        // Print to console in debug builds
        #if DEBUG
        print(logMessage)
        #endif
    }

    /// Log with custom metadata
    func log(
        _ message: String,
        level: LogLevel = .info,
        category: Category = .general,
        metadata: [String: Any] = [:],
        file: String = #file,
        function: String = #function,
        line: Int = #line
    ) {
        var fullMessage = message

        if !metadata.isEmpty {
            let metadataString = metadata.map { "\($0.key)=\($0.value)" }.joined(separator: ", ")
            fullMessage += " | \(metadataString)"
        }

        log(fullMessage, level: level, category: category, file: file, function: function, line: line)
    }

    // MARK: - Convenience Methods

    func debug(_ message: String, category: Category = .general, file: String = #file, function: String = #function, line: Int = #line) {
        log(message, level: .debug, category: category, file: file, function: function, line: line)
    }

    func info(_ message: String, category: Category = .general, file: String = #file, function: String = #function, line: Int = #line) {
        log(message, level: .info, category: category, file: file, function: function, line: line)
    }

    func warning(_ message: String, category: Category = .general, file: String = #file, function: String = #function, line: Int = #line) {
        log(message, level: .warning, category: category, file: file, function: function, line: line)
    }

    func error(_ message: String, category: Category = .general, file: String = #file, function: String = #function, line: Int = #line) {
        log(message, level: .error, category: category, file: file, function: function, line: line)
    }

    func critical(_ message: String, category: Category = .general, file: String = #file, function: String = #function, line: Int = #line) {
        log(message, level: .critical, category: category, file: file, function: function, line: line)
    }

    // MARK: - Privacy & Sanitization

    /// Sanitize message to remove sensitive information
    private func sanitizeMessage(_ message: String) -> String {
        var sanitized = message

        // Redact Ethereum private keys (0x followed by 64 hex characters)
        sanitized = sanitized.replacingOccurrences(
            of: "0x[a-fA-F0-9]{64}",
            with: "0x[PRIVATE_KEY_REDACTED]",
            options: .regularExpression
        )

        // Redact mnemonics (12-24 words)
        sanitized = sanitized.replacingOccurrences(
            of: "\\b([a-z]+\\s){11,23}[a-z]+\\b",
            with: "[MNEMONIC_REDACTED]",
            options: .regularExpression
        )

        // Redact potential passwords (password=xxx, pass=xxx, etc.)
        sanitized = sanitized.replacingOccurrences(
            of: "(password|pass|pwd|secret|token)\\s*[=:]\\s*[^\\s,]+",
            with: "$1=[REDACTED]",
            options: [.regularExpression, .caseInsensitive]
        )

        // Redact JWT tokens
        sanitized = sanitized.replacingOccurrences(
            of: "eyJ[A-Za-z0-9-_=]+\\.[A-Za-z0-9-_=]+\\.?[A-Za-z0-9-_.+/=]*",
            with: "[JWT_REDACTED]",
            options: .regularExpression
        )

        // Redact email addresses (partial)
        sanitized = sanitized.replacingOccurrences(
            of: "([a-zA-Z0-9._-]+)@([a-zA-Z0-9._-]+\\.[a-zA-Z0-9._-]+)",
            with: "$1***@$2",
            options: .regularExpression
        )

        return sanitized
    }

    // MARK: - File Logging

    private func setupLogFile() {
        guard let documentsURL = fileManager.urls(for: .documentDirectory, in: .userDomainMask).first else {
            return
        }

        let logsDirectory = documentsURL.appendingPathComponent("Logs", isDirectory: true)

        // Create logs directory if needed
        if !fileManager.fileExists(atPath: logsDirectory.path) {
            try? fileManager.createDirectory(at: logsDirectory, withIntermediateDirectories: true)
        }

        // Set current log file
        let dateString = DateFormatter.logFileName.string(from: Date())
        logFileURL = logsDirectory.appendingPathComponent("etrid-\(dateString).log")

        // Rotate old logs
        rotateLogsIfNeeded()
    }

    private func appendToFile(_ message: String) {
        guard let logFileURL = logFileURL else { return }

        let logLine = message + "\n"

        if let data = logLine.data(using: .utf8) {
            if fileManager.fileExists(atPath: logFileURL.path) {
                // Append to existing file
                if let fileHandle = try? FileHandle(forWritingTo: logFileURL) {
                    fileHandle.seekToEndOfFile()
                    fileHandle.write(data)
                    try? fileHandle.close()
                }
            } else {
                // Create new file
                try? data.write(to: logFileURL)
            }

            // Check if rotation needed
            checkFileSize()
        }
    }

    private func checkFileSize() {
        guard let logFileURL = logFileURL else { return }

        if let attributes = try? fileManager.attributesOfItem(atPath: logFileURL.path),
           let fileSize = attributes[.size] as? Int,
           fileSize > maxLogFileSize {
            rotateLogFile()
        }
    }

    private func rotateLogFile() {
        guard let logFileURL = logFileURL else { return }

        let timestamp = DateFormatter.logFileName.string(from: Date())
        let rotatedURL = logFileURL.deletingLastPathComponent()
            .appendingPathComponent("etrid-\(timestamp)-rotated.log")

        try? fileManager.moveItem(at: logFileURL, to: rotatedURL)

        // Create new file
        try? "".write(to: logFileURL, atomically: true, encoding: .utf8)

        rotateLogsIfNeeded()
    }

    private func rotateLogsIfNeeded() {
        guard let documentsURL = fileManager.urls(for: .documentDirectory, in: .userDomainMask).first else {
            return
        }

        let logsDirectory = documentsURL.appendingPathComponent("Logs", isDirectory: true)

        guard let logFiles = try? fileManager.contentsOfDirectory(
            at: logsDirectory,
            includingPropertiesForKeys: [.creationDateKey],
            options: .skipsHiddenFiles
        ) else {
            return
        }

        // Sort by creation date
        let sortedLogs = logFiles.sorted { url1, url2 in
            let date1 = (try? url1.resourceValues(forKeys: [.creationDateKey]))?.creationDate ?? Date.distantPast
            let date2 = (try? url2.resourceValues(forKeys: [.creationDateKey]))?.creationDate ?? Date.distantPast
            return date1 > date2
        }

        // Delete old logs
        if sortedLogs.count > maxLogFiles {
            for logFile in sortedLogs.dropFirst(maxLogFiles) {
                try? fileManager.removeItem(at: logFile)
            }
        }
    }

    private func logToOS(_ message: String, level: LogLevel, category: Category) {
        let categoryLog = OSLog(subsystem: category.subsystem, category: category.rawValue)
        os_log("%{public}@", log: categoryLog, type: level.osLogType, message)
    }

    // MARK: - Export

    /// Get all log file URLs
    func getLogFiles() -> [URL] {
        guard let documentsURL = fileManager.urls(for: .documentDirectory, in: .userDomainMask).first else {
            return []
        }

        let logsDirectory = documentsURL.appendingPathComponent("Logs", isDirectory: true)

        guard let logFiles = try? fileManager.contentsOfDirectory(
            at: logsDirectory,
            includingPropertiesForKeys: [.creationDateKey],
            options: .skipsHiddenFiles
        ) else {
            return []
        }

        return logFiles.sorted { url1, url2 in
            let date1 = (try? url1.resourceValues(forKeys: [.creationDateKey]))?.creationDate ?? Date.distantPast
            let date2 = (try? url2.resourceValues(forKeys: [.creationDateKey]))?.creationDate ?? Date.distantPast
            return date1 > date2
        }
    }

    /// Export all logs as a single file
    func exportLogs() -> URL? {
        let logFiles = getLogFiles()
        guard !logFiles.isEmpty else { return nil }

        var combinedLogs = ""

        for logFile in logFiles {
            if let content = try? String(contentsOf: logFile) {
                combinedLogs += "=== \(logFile.lastPathComponent) ===\n"
                combinedLogs += content
                combinedLogs += "\n\n"
            }
        }

        guard let tempURL = fileManager.urls(for: .cachesDirectory, in: .userDomainMask).first else {
            return nil
        }

        let exportURL = tempURL.appendingPathComponent("etrid-logs-export.txt")
        try? combinedLogs.write(to: exportURL, atomically: true, encoding: .utf8)

        return exportURL
    }

    /// Clear all log files
    func clearLogs() {
        let logFiles = getLogFiles()
        for logFile in logFiles {
            try? fileManager.removeItem(at: logFile)
        }
        setupLogFile()
    }
}

// MARK: - Global Convenience Functions

/// Log debug message
func logDebug(_ message: String, category: Logger.Category = .general, file: String = #file, function: String = #function, line: Int = #line) {
    Task {
        await Logger.shared.debug(message, category: category, file: file, function: function, line: line)
    }
}

/// Log info message
func logInfo(_ message: String, category: Logger.Category = .general, file: String = #file, function: String = #function, line: Int = #line) {
    Task {
        await Logger.shared.info(message, category: category, file: file, function: function, line: line)
    }
}

/// Log warning message
func logWarning(_ message: String, category: Logger.Category = .general, file: String = #file, function: String = #function, line: Int = #line) {
    Task {
        await Logger.shared.warning(message, category: category, file: file, function: function, line: line)
    }
}

/// Log error message
func logError(_ message: String, category: Logger.Category = .general, file: String = #file, function: String = #function, line: Int = #line) {
    Task {
        await Logger.shared.error(message, category: category, file: file, function: function, line: line)
    }
}

/// Log critical message
func logCritical(_ message: String, category: Logger.Category = .general, file: String = #file, function: String = #function, line: Int = #line) {
    Task {
        await Logger.shared.critical(message, category: category, file: file, function: function, line: line)
    }
}

// MARK: - Extensions

extension DateFormatter {
    static let logFileName: DateFormatter = {
        let formatter = DateFormatter()
        formatter.dateFormat = "yyyy-MM-dd"
        return formatter
    }()
}
