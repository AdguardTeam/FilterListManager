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
      url: "https://github.com/AdguardTeam/FilterListManager/releases/download/v0.7.4@swift-5/AdGuardFLM-0.7.4.zip",
      checksum: "b9ac9d3cd78e6911aaf6742670ae6479e4531f1b525d281e3bbbd71807ef188f"
    ),
  ]
)

