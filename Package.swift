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
      url: "https://github.com/AdguardTeam/FilterListManager/releases/download/v0.6.3@swift-5/AdGuardFLM-0.6.3.zip",
      checksum: "7f8c319f05a935ac40897f100b8c95e6ba2f85af20bcd0f2cf17170e8aa87013"
    ),
  ]
)

