#!/bin/sh

set -e

cargo build
cargo run --bin uniffi-bindgen generate uniffi/bug_finder.udl --language swift -o out --lib-file target/debug/libbug_finder.dylib

cp out/bug_finder.swift swift/BugFinder/Sources/BugFinder/

mkdir -p out/include
cp out/bug_finderFFI.h out/include/bug_finderFFI.h
cp out/bug_finderFFI.modulemap out/include/module.modulemap

rm -rf out/bug_finderFFI.xcframework
xcodebuild -create-xcframework \
  -library target/debug/libbug_finder.a \
  -headers out/include \
  -output out/bug_finderFFI.xcframework
