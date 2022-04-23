
import SwiftUI

@main
struct RuiApp: App {
    var body: some Scene {
        DocumentGroup(newDocument: RuiDocument()) { file in
            ContentView(document: file.$document)
        }
    }
}
