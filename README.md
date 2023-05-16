## How to test

1. run `./build-xcframework.sh`
2. open folder `./swift/BugFinder` in Xcode
3. run `testStore()` in `BugFinderTests.swift`

## What happens?

- `testStore()` in `BugFinderTests.swift` hangs on `store.close()`.
- `close_test()` in `src/lib.rs` works fine.
