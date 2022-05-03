
import SwiftUI

class AppModel {
    var appState = AppState()

    init() {}
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
