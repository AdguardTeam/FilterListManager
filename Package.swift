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
      url: "https://github.com/AdguardTeam/FilterListManager/releases/download/v0.7.7@swift-5/AdGuardFLM-0.7.7.zip",
      checksum: "4915966524c9b241aa0e2300a2d22c05d249ad198e51f1e32efb398dbae2e75c"
    ),
  ]
)

