// swift-tools-version: 5.9
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription

let package = Package(
    name: "Paper",
    platforms: [.iOS(.v13), .macOS(.v10_15)],
    products: [
        // Products define the executables and libraries a package produces, making them visible to other packages.
        .library(
            name: "Paper",
            targets: ["Paper", "PaperFFI"])
    ],
    dependencies: [
        .package(url: "https://github.com/apple/swift-argument-parser", from: "1.0.0")
    ],
    targets: [
        .target(
            name: "Paper",
            dependencies: ["PaperFFI"],
            path: "paper/paper-package/Sources/Paper"
        ),
        .binaryTarget(name: "PaperFFI", path: "paper/paper-package/RustFramework.xcframework"),
        .testTarget(name: "PaperTests", dependencies: ["Paper"]),
    ]
)
