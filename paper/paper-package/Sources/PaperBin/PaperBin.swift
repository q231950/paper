// The Swift Programming Language
// https://docs.swift.org/swift-book

import Foundation
import ArgumentParser
import Paper

@main
struct Login: ParsableCommand {
    @Argument(help: "The username")
    var username = ""

    @Argument(help: "The password")
    var password = ""

    mutating func run() throws {
        let configuration = Configuration(username: username, password: password, shouldScrape: true, libraryType: .hamburgPublic)
        let scraper = LibraryScraper()

        

        Task {
            do {
                let account = try await scraper.fetchAccount(configuration: configuration)

                // Account
                debugPrint("Signed in as \(account.name)")

                // Loans
                for loan in account.loans.loans {
                    debugPrint(loan.title)
                }

                // Renewal
                if let loan = account.loans.loans.last {
                    print(loan)
                    _ = RenewalService()
//                    let result = try await renewalService.renew(itemNumber: loan.itemNumber, configuration: configuration)
//                    print(result)
                }

            } catch {
                print(error)
            }
        }

        RunLoop.main.run()
    }
}
