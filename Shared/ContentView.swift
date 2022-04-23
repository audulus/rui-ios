
import SwiftUI
import Metal

struct MetalLayerView: UIViewRepresentable {

    var appState: OpaquePointer

    func makeUIView(context: Context) -> UIView {
        let view = UIView()
        view.layer.addSublayer(CAMetalLayer())
        return view
    }

    func updateUIView(_ uiView: UIView, context: Context) {
        // do nothing
    }

}

struct ContentView: View {
    @Binding var document: RuiDocument

    var body: some View {
        MetalLayerView(appState: document.model.appState)
    }
}

struct ContentView_Previews: PreviewProvider {
    static var previews: some View {
        ContentView(document: .constant(RuiDocument()))
    }
}
