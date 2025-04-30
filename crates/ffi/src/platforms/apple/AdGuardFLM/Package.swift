// swift-tools-version: 5.4

import PackageDescription

let package = Package(
    name: "AdGuardFLM",
    platforms: [
        .iOS("11.2"),
        .macOS("10.15")
    ],
    products: [
        .library(
            name: "AdGuardFLMLib",
            targets: ["AdGuardFLMLib"]
        )
    ],
    dependencies: [
        .package(url: "https://github.com/apple/swift-protobuf.git", from: "1.28.2")
    ],
    targets: [
        .target(
            name: "AdGuardFLMLib",
            dependencies: [
                .product(name: "SwiftProtobuf", package: "swift-protobuf"),
                .target(name: "AdGuardFLM")
            ]
        ),
        .binaryTarget(
            name: "AdGuardFLM",
            path: "../build/framework/AdGuardFLM.xcframework.zip"
        ),
        .testTarget(
            name: "AdGuardFLMLibTests",
            dependencies: ["AdGuardFLMLib"]
        ),
    ]
)
