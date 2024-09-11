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
      url: "https://github.com/AdguardTeam/FilterListManager/releases/download/v0.5.17@swift-5/AdGuardFLM-0.5.17.zip",
      checksum: "ce955f8f9b2475f9ef0f22c14fd8f6354bd82a5b71440e0656aa20bd2f9f890d"
    ),
  ]
)

