// swift-tools-version: 5.9
import PackageDescription

let package = Package(
    name: "EtridSDK",
    platforms: [
        .iOS(.v15),
        .macOS(.v12)
    ],
    products: [
        .library(
            name: "EtridSDK",
            targets: ["EtridSDK"]),
    ],
    dependencies: [
        .package(url: "https://github.com/apple/swift-crypto.git", from: "3.0.0"),
        .package(url: "https://github.com/daltoniam/Starscream.git", from: "4.0.0"),
    ],
    targets: [
        .target(
            name: "EtridSDK",
            dependencies: [
                .product(name: "Crypto", package: "swift-crypto"),
                "Starscream"
            ]),
        .testTarget(
            name: "EtridSDKTests",
            dependencies: ["EtridSDK"]),
    ]
)
