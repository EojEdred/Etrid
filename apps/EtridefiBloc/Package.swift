// swift-tools-version: 5.9
import PackageDescription

let package = Package(
    name: "EtridWalletSwift",
    platforms: [.iOS(.v17), .macOS(.v14)],
    products: [
        .library(
            name: "EtridWalletSwift",
            targets: ["EtridWalletSwift"]),
    ],
    dependencies: [
        // Secp256k1 for cryptographic operations (Bitcoin, Ethereum, Substrate)
        .package(url: "https://github.com/GigaBitcoin/secp256k1.swift", from: "0.15.0"),
        // QR Code generation
        .package(url: "https://github.com/WalletConnect/QRCode", from: "14.3.1"),
        // CryptoSwift for Keccak256 (Ethereum addresses)
        .package(url: "https://github.com/krzyzanowskim/CryptoSwift", from: "1.8.0"),
        // Blake2 for Substrate SS58 addresses
        .package(url: "https://github.com/tesseract-one/Blake2.swift", from: "0.2.0"),
        // Base58 encoding for Bitcoin/Solana
        .package(url: "https://github.com/keefertaylor/Base58Swift", from: "2.1.0"),
    ],
    targets: [
        .target(
            name: "EtridWalletSwift",
            dependencies: [
                .product(name: "P256K", package: "secp256k1.swift"),
                .product(name: "QRCode", package: "QRCode"),
                .product(name: "CryptoSwift", package: "CryptoSwift"),
                .product(name: "Blake2", package: "Blake2.swift"),
                .product(name: "Base58Swift", package: "Base58Swift"),
            ],
            resources: [
                .process("Resources")
            ]
        ),
    ]
)
