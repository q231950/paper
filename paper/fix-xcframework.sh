#!/bin/sh
#
# Fixes the XCFramework that cargo-swift 0.9.0 creates, such that Xcode finds the FFI module:
# - cargo-swift moves the headers of every library to `headers`, Info.plist expects `Headers`
# - cargo-swift moves the module map and header to `Headers/RustFramework`, which Xcode copies
#   to `include/RustFramework` where only a search of subdirectories finds the module map.
#   Moving them to `Headers` puts the module map right into the `include` search path.
# - cargo-swift names the header after the crate (`paperFFI.h`), the module map expects the
#   header of the uniffi namespace (`PaperFFI.h`)
#
# Usage: sh fix-xcframework.sh paper-package/RustFramework.xcframework

set -e

# Renames in two steps, a rename that only changes the case does nothing on a case-insensitive
# file system
rename() {
    mv "$1" "$1.tmp"
    mv "$1.tmp" "$2"
}

lowercased() {
    echo "$1" | tr '[:upper:]' '[:lower:]'
}

for library in "$1"/*/; do
    library="${library%/}"

    if [ -d "$library/headers" ]; then
        rename "$library/headers" "$library/Headers"
    fi

    for modulemap in "$library"/Headers/*/module.modulemap; do
        [ -f "$modulemap" ] || continue

        directory=$(dirname "$modulemap")
        mv "$directory"/* "$library/Headers/"
        rmdir "$directory"
    done

    modulemap="$library/Headers/module.modulemap"
    [ -f "$modulemap" ] || continue

    expected=$(sed -n 's/.*header "\(.*\)".*/\1/p' "$modulemap")

    for header in "$library"/Headers/*.h; do
        [ -f "$header" ] || continue

        name=$(basename "$header")
        if [ "$name" != "$expected" ] && [ "$(lowercased "$name")" = "$(lowercased "$expected")" ]; then
            rename "$header" "$library/Headers/$expected"
        fi
    done
done
