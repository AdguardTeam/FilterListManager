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
      url: "https://github.com/AdguardTeam/FilterListManager/releases/download/v0.5.20@swift-5/AdGuardFLM-0.5.20.zip",
      checksum: "65ca85b694194bd042c60f917bf14ece447d85c5c8cfe9561439dabd90542fc3"
    ),
  ]
)

