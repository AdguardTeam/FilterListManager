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
      url: "https://github.com/AdguardTeam/FilterListManager/releases/download/v1.5.10@swift-5/AdGuardFLM-1.5.10.zip",
      checksum: "c3d07ba3e52dc0c8cdb965b8735464cda9addd83e7b85cb1b65544243ecde396"
    ),
  ]
)

