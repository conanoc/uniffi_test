## How to test

1. run `./build-xcframework.sh`
2. open folder `./swift/BugFinder` in Xcode
3. run `testStore()` in `BugFinderTests.swift`

## What happens?

- `testStore()` in `BugFinderTests.swift` crashes at several points.
- `close_test()` in `src/lib.rs` works fine.
