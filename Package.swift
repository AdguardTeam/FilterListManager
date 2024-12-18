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
      url: "https://github.com/AdguardTeam/FilterListManager/releases/download/v0.8.27@swift-5/AdGuardFLM-0.8.27.zip",
      checksum: "b0082453b201ad8c2dd043d4860a0f7c59f186524994f58eddf157e85976914a"
    ),
  ]
)

