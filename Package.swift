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
      url: "https://github.com/AdguardTeam/FilterListManager/releases/download/v0.8.2@swift-5/AdGuardFLM-0.8.2.zip",
      checksum: "dc2cdd0f4a378c15f9f5537f1ef962e662fe84735f339ee380ff2fff72df92a8"
    ),
  ]
)

