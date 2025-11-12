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
      url: "https://github.com/AdguardTeam/FilterListManager/releases/download/v2.3.1@swift-5/AdGuardFLM-2.3.1.zip",
      checksum: "49844a69cb28f3e3a0feb5a2635759a88349cff9940a3096dace5e0b37709472"
    ),
  ]
)

