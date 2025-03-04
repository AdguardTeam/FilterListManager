// swift-tools-version: 5.4
import PackageDescription

let package = Package(
  name: "AdGuardFLM",
  platforms: [
    .iOS("11.2"), .macOS("10.15")
  ],
  products: [
    .library(name: "AdGuardFLMLib", targets: ["AdGuardFLMLib"])
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
      url: "https://github.com/AdguardTeam/FilterListManager/releases/download/v1.4.1@swift-5/AdGuardFLM-1.4.1.zip",
      checksum: "010b89709384c7014a2dfa980bf75931d8c5c428e3bc18b1e250a83ac78d1d18"
    ),
  ]
)

