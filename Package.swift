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
      url: "https://github.com/AdguardTeam/FilterListManager/releases/download/v0.8.11@swift-5/AdGuardFLM-0.8.11.zip",
      checksum: "d3c84d6a00de6df4f1e769d85b7208d362d5e4e74d44229093eca75b57f7ded1"
    ),
  ]
)

