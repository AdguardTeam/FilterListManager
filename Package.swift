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
      url: "https://github.com/AdguardTeam/FilterListManager/releases/download/v0.8.13@swift-5/AdGuardFLM-0.8.13.zip",
      checksum: "a3783adc6cd8b17e1843ac6795059b1374b8a0dde9ab24dba4ded1fa13c0d031"
    ),
  ]
)

