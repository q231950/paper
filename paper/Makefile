apple:
	cargo swift package --name Paper --platforms ios --platforms macos
	rm -rf paper-package/RustFramework.xcframework
	cp -r ./Paper/RustFramework.xcframework ./paper-package/RustFramework.xcframework

	rm -rf paper-package/Sources/Paper/Paper.swift
	cp ./Paper/Sources/Paper/paper.swift ./paper-package/Sources/Paper/Paper.swift

apple-release:
	cargo swift package --name Paper --platforms ios --release
	rm -rf paper-package/RustFramework.xcframework
	cp -r ./Paper/RustFramework.xcframework ./paper-package/RustFramework.xcframework

	rm -rf paper-package/Sources/Paper/Paper.swift
	cp ./Paper/Sources/Paper/paper.swift ./paper-package/Sources/Paper/Paper.swift
