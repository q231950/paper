import XCTest
@testable import Paper

final class PaperTests: XCTestCase {
    func testExample() throws {
        let result = simple_addition(3, 2)

        XCTAssertEqual(result, 5)
    }
}
