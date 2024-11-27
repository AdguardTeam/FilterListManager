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
      url: "https://github.com/AdguardTeam/FilterListManager/releases/download/v0.8.23@swift-5/AdGuardFLM-0.8.23.zip",
      checksum: "b0631a2b52c720076d12cb1fcb09ecc0d0c9622b03ebbbb36ce9e2825c81254b"
    ),
  ]
)

