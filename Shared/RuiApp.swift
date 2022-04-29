
import SwiftUI

class AppModel {
    var appState = new_context()!

    init() {}

    deinit {
        delete_context(appState)
    }
}

@main
struct RuiApp: App {

    let model = AppModel()

    var body: some Scene {
        WindowGroup {
            RuiSwiftUIView(appState: model.appState)
        }
    }
}
