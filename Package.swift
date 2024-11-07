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
      url: "https://github.com/AdguardTeam/FilterListManager/releases/download/v0.8.17@swift-5/AdGuardFLM-0.8.17.zip",
      checksum: "47efab8cebd44d4d867e04b1a7f0fef52fba43c19a8796b92a827cf134581d85"
    ),
  ]
)

