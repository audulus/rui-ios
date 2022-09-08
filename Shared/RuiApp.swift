
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
            ZStack {
                Rectangle().fill(.black).edgesIgnoringSafeArea(.all)
                RuiSwiftUIView(appState: model.appState)
            }
        }
    }
}
