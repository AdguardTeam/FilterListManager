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
      url: "https://github.com/AdguardTeam/FilterListManager/releases/download/v0.8.7@swift-5/AdGuardFLM-0.8.7.zip",
      checksum: "224af97865e5094ce523d39aaf876175abf650d97d356a3df0d68bd4a523fbc2"
    ),
  ]
)

