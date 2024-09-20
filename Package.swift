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
      url: "https://github.com/AdguardTeam/FilterListManager/releases/download/v0.6.2@swift-5/AdGuardFLM-0.6.2.zip",
      checksum: "61f18fe9a0a1a8832daa569711d60d4251d34517aa53f58b32de5cf0951c7c19"
    ),
  ]
)

