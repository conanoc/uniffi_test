import XCTest
@testable import BugFinder

final class BugFinderTests: XCTestCase {
    func testAsync() async throws {
        print("waiting addAsyncNormal...")
        var sum = await addAsyncNormal(left: 1, right: 2)
        print("got sum = \(sum)")
        print("waiting addAsync...")
        sum = await addAsync(left: 1, right: 2)
        print("got sum = \(sum)")
    }
}
