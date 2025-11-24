//
//  StorageManager.swift
//  EtridWallet
//
//  File-based persistent storage manager
//

import Foundation

actor StorageManager {
    // MARK: - Singleton
    static let shared = StorageManager()

    // MARK: - Storage Locations
    private let documentsDirectory: URL
    private let cacheDirectory: URL

    private init() {
        documentsDirectory = FileManager.default.urls(for: .documentDirectory, in: .userDomainMask)[0]
        cacheDirectory = FileManager.default.urls(for: .cachesDirectory, in: .userDomainMask)[0]
    }

    // MARK: - Save/Load
    /// Saves codable object to file
    /// - Parameters:
    ///   - object: Object to save
    ///   - filename: Filename
    ///   - inCache: Save in cache directory instead of documents
    func save<T: Encodable>(_ object: T, filename: String, inCache: Bool = false) throws {
        let directory = inCache ? cacheDirectory : documentsDirectory
        let fileURL = directory.appendingPathComponent(filename)

        let data = try JSONEncoder().encode(object)
        try data.write(to: fileURL, options: [.atomic, .completeFileProtection])
    }

    /// Loads codable object from file
    /// - Parameters:
    ///   - type: Type to decode
    ///   - filename: Filename
    ///   - inCache: Load from cache directory
    /// - Returns: Decoded object or nil
    func load<T: Decodable>(_ type: T.Type, filename: String, inCache: Bool = false) throws -> T? {
        let directory = inCache ? cacheDirectory : documentsDirectory
        let fileURL = directory.appendingPathComponent(filename)

        guard FileManager.default.fileExists(atPath: fileURL.path) else {
            return nil
        }

        let data = try Data(contentsOf: fileURL)
        return try JSONDecoder().decode(T.self, from: data)
    }

    /// Saves data to file
    /// - Parameters:
    ///   - data: Data to save
    ///   - filename: Filename
    ///   - inCache: Save in cache
    func saveData(_ data: Data, filename: String, inCache: Bool = false) throws {
        let directory = inCache ? cacheDirectory : documentsDirectory
        let fileURL = directory.appendingPathComponent(filename)

        try data.write(to: fileURL, options: [.atomic, .completeFileProtection])
    }

    /// Loads data from file
    /// - Parameters:
    ///   - filename: Filename
    ///   - inCache: Load from cache
    /// - Returns: Data or nil
    func loadData(filename: String, inCache: Bool = false) throws -> Data? {
        let directory = inCache ? cacheDirectory : documentsDirectory
        let fileURL = directory.appendingPathComponent(filename)

        guard FileManager.default.fileExists(atPath: fileURL.path) else {
            return nil
        }

        return try Data(contentsOf: fileURL)
    }

    // MARK: - Delete
    /// Deletes file
    /// - Parameters:
    ///   - filename: Filename to delete
    ///   - inCache: Delete from cache
    func delete(filename: String, inCache: Bool = false) throws {
        let directory = inCache ? cacheDirectory : documentsDirectory
        let fileURL = directory.appendingPathComponent(filename)

        if FileManager.default.fileExists(atPath: fileURL.path) {
            try FileManager.default.removeItem(at: fileURL)
        }
    }

    /// Deletes all files in directory
    /// - Parameter inCache: Clear cache directory
    func deleteAll(inCache: Bool = false) throws {
        let directory = inCache ? cacheDirectory : documentsDirectory
        let contents = try FileManager.default.contentsOfDirectory(at: directory, includingPropertiesForKeys: nil)

        for fileURL in contents {
            try FileManager.default.removeItem(at: fileURL)
        }
    }

    // MARK: - File Management
    /// Checks if file exists
    /// - Parameters:
    ///   - filename: Filename
    ///   - inCache: Check in cache
    /// - Returns: True if exists
    func fileExists(_ filename: String, inCache: Bool = false) -> Bool {
        let directory = inCache ? cacheDirectory : documentsDirectory
        let fileURL = directory.appendingPathComponent(filename)
        return FileManager.default.fileExists(atPath: fileURL.path)
    }

    /// Gets file size
    /// - Parameters:
    ///   - filename: Filename
    ///   - inCache: Check in cache
    /// - Returns: Size in bytes
    func fileSize(_ filename: String, inCache: Bool = false) -> Int64? {
        let directory = inCache ? cacheDirectory : documentsDirectory
        let fileURL = directory.appendingPathComponent(filename)

        guard let attributes = try? FileManager.default.attributesOfItem(atPath: fileURL.path),
              let size = attributes[.size] as? Int64 else {
            return nil
        }

        return size
    }

    /// Lists all files
    /// - Parameter inCache: List cache files
    /// - Returns: Array of filenames
    func listFiles(inCache: Bool = false) throws -> [String] {
        let directory = inCache ? cacheDirectory : documentsDirectory
        let contents = try FileManager.default.contentsOfDirectory(at: directory, includingPropertiesForKeys: nil)
        return contents.map { $0.lastPathComponent }
    }

    // MARK: - Directory Info
    /// Gets total storage used
    /// - Parameter inCache: Check cache directory
    /// - Returns: Total bytes used
    func totalStorageUsed(inCache: Bool = false) -> Int64 {
        let directory = inCache ? cacheDirectory : documentsDirectory

        guard let contents = try? FileManager.default.contentsOfDirectory(
            at: directory,
            includingPropertiesForKeys: [.fileSizeKey]
        ) else {
            return 0
        }

        var totalSize: Int64 = 0

        for fileURL in contents {
            if let attributes = try? FileManager.default.attributesOfItem(atPath: fileURL.path),
               let size = attributes[.size] as? Int64 {
                totalSize += size
            }
        }

        return totalSize
    }

    /// Gets available storage space
    /// - Returns: Available bytes
    func availableStorageSpace() -> Int64? {
        guard let attributes = try? FileManager.default.attributesOfFileSystem(
            forPath: documentsDirectory.path
        ),
              let freeSize = attributes[.systemFreeSize] as? Int64 else {
            return nil
        }

        return freeSize
    }
}
