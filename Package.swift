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
      url: "https://github.com/AdguardTeam/FilterListManager/releases/download/v0.8.5@swift-5/AdGuardFLM-0.8.5.zip",
      checksum: "32ffa35af00e57d6a20f5d2b0af9f99cc5c72a7b3cde8afd1237f426657bf9ca"
    ),
  ]
)

