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
      url: "https://github.com/AdguardTeam/FilterListManager/releases/download/v0.8.15@swift-5/AdGuardFLM-0.8.15.zip",
      checksum: "915f0302efcb4ad52819c87f48bf191b24c31212acf3660a498d7befa1041e88"
    ),
  ]
)

