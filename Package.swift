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
      url: "https://github.com/AdguardTeam/FilterListManager/releases/download/v2.0.0-rc.2@swift-5/AdGuardFLM-2.0.0-rc.2.zip",
      checksum: "6d6aebcc624105c2a86bf27453da35f3c191f13bbff9cb5b87ffeb1b1e7e1226"
    ),
  ]
)

