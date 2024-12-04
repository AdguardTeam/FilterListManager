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
      url: "https://github.com/AdguardTeam/FilterListManager/releases/download/v0.8.24@swift-5/AdGuardFLM-0.8.24.zip",
      checksum: "3f7eb88815c371d27506d20b49af3257022ce8938ce663dba39da916ccc80c39"
    ),
  ]
)

