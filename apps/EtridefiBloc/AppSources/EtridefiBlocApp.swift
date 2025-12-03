import SwiftUI

@main
struct EtridefiBlocApp: App {
    @StateObject private var walletManager = EtridefiBlocManager()

    var body: some Scene {
        WindowGroup {
            RootView()
                .environmentObject(walletManager)
        }
    }
}
