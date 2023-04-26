import XCTest
@testable import BugFinder

final class BugFinderTests: XCTestCase {
    func testStore() async throws {
        let store = await createStore()
        let count = await store.count()
        print("count: \(count)")
        await store.close()
        print("store closed")
    }
}
