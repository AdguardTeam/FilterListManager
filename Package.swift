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
      url: "https://github.com/AdguardTeam/FilterListManager/releases/download/v0.7.8@swift-5/AdGuardFLM-0.7.8.zip",
      checksum: "408ca94aaa65487ebac6ecab1e559da769d0008007659b819f3ccf0464d771f2"
    ),
  ]
)

