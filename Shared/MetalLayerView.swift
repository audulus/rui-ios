
import SwiftUI
import Metal
import QuartzCore

struct MetalLayerView: UIViewRepresentable {

    var appState: OpaquePointer

    func makeUIView(context: Context) -> UIView {
        let view = UIView()

        var metalLayer = CAMetalLayer()

        setup_surface(appState, &metalLayer)

        view.layer.addSublayer(metalLayer)
        return view
    }

    func updateUIView(_ uiView: UIView, context: Context) {
        // do nothing
    }

}

