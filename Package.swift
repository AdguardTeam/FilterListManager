// swift-tools-version:5.3
import PackageDescription

let package = Package(
  name: "AdGuardFLM",
  platforms: [
    .iOS("11.2"), .macOS("10.15")
  ],
  products: [
    .library(name: "AdGuardFLM", targets: ["AdGuardFLM"]),
  ],
  targets: [
    .binaryTarget(
      name: "AdGuardFLM",
      url: "https://github.com/AdguardTeam/FilterListManager/releases/download/v0.6.1@swift-5/AdGuardFLM-0.6.1.zip",
      checksum: "501bb7dab6d2bf1d8490c6c9c11c724c49d4f2cff138c3207c4426d6b31e00d9"
    ),
  ]
)

