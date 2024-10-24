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
      url: "https://github.com/AdguardTeam/FilterListManager/releases/download/v0.7.10@swift-5/AdGuardFLM-0.7.10.zip",
      checksum: "c242ffeeb10ed127d68deb6600c9ca1abe7755b296a7c11c71b545af8193d02a"
    ),
  ]
)

