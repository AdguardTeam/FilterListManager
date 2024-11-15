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
      url: "https://github.com/AdguardTeam/FilterListManager/releases/download/v0.8.21@swift-5/AdGuardFLM-0.8.21.zip",
      checksum: "2bc0fda489bff3bf5fbeaed08e93e73166354fa7fc4b443a8f425be87c031279"
    ),
  ]
)

