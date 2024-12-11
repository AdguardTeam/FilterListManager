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
      url: "https://github.com/AdguardTeam/FilterListManager/releases/download/v0.8.26@swift-5/AdGuardFLM-0.8.26.zip",
      checksum: "135ee0e65bee404bc8aecabde502e2cff6d96f5bf67fa17531ff275c490b6701"
    ),
  ]
)

