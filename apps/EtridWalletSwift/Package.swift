// swift-tools-version: 5.9
import PackageDescription

let package = Package(
    name: "EtridWalletSwift",
    platforms: [.iOS(.v17)],
    products: [
        .library(
            name: "EtridWalletSwift",
            targets: ["EtridWalletSwift"]),
    ],
    dependencies: [
        .package(url: "https://github.com/WalletConnect/WalletConnectSwiftV2", from: "1.0.0"),
    ],
    targets: [
        .target(
            name: "EtridWalletSwift",
            dependencies: [
                .product(name: "WalletConnectSwiftV2", package: "WalletConnectSwiftV2")
            ]),
    ]
)
