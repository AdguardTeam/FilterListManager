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
      url: "https://github.com/AdguardTeam/FilterListManager/releases/download/v0.5.16@swift-5/AdGuardFLM-0.5.16.zip",
      checksum: "d9988a0729c64cad1f8bd18676e8bfb32e0a02ec39b13b4fea57640592d28407"
    ),
  ]
)

