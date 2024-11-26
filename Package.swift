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
      url: "https://github.com/AdguardTeam/FilterListManager/releases/download/v0.8.22@swift-5/AdGuardFLM-0.8.22.zip",
      checksum: "9e14f0d9ffb9ac23b6b631613461de2abb3d60a8af695799ed4de2130ab79868"
    ),
  ]
)

