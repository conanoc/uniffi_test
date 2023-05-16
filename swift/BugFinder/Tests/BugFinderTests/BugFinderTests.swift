import XCTest
@testable import BugFinder

final class BugFinderTests: XCTestCase {
    func testStore() async throws {
        print("test starts")
        let store = await createStore()
        do {
            let session = await store.session()
            let count = await session.count()
            print("count: \(count)")
        }
        await store.close()
        print("store closed")
    }
}
