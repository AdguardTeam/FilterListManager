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
      url: "https://github.com/AdguardTeam/FilterListManager/releases/download/v0.8.25@swift-5/AdGuardFLM-0.8.25.zip",
      checksum: "5be360a566e5208ac5620361cee6e4dfe236a884e9babf31600358cb3fd46b93"
    ),
  ]
)

