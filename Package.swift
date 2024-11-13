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
      url: "https://github.com/AdguardTeam/FilterListManager/releases/download/v0.8.19@swift-5/AdGuardFLM-0.8.19.zip",
      checksum: "0a2203c5e4e7056eeb2ad00a738f6380c05c442c4c135420606b191523a14b59"
    ),
  ]
)

