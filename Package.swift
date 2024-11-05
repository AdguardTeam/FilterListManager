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
      url: "https://github.com/AdguardTeam/FilterListManager/releases/download/v0.8.9@swift-5/AdGuardFLM-0.8.9.zip",
      checksum: "e3dd15260756edd3f1211ed654f66637df4ce9733bc1a4ccc99e14053ccb2e1d"
    ),
  ]
)

