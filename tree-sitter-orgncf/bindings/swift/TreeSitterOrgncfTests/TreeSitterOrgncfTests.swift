import XCTest
import SwiftTreeSitter
import TreeSitterOrgncf

final class TreeSitterOrgncfTests: XCTestCase {
    func testCanLoadGrammar() throws {
        let parser = Parser()
        let language = Language(language: tree_sitter_orgncf())
        XCTAssertNoThrow(try parser.setLanguage(language),
                         "Error loading Organizator Network Configuration grammar")
    }
}
