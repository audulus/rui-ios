
import SwiftUI
import Metal
import QuartzCore
import MetalKit

class Renderer: NSObject, MTKViewDelegate {

    var appState: OpaquePointer

    init(appState: OpaquePointer) {
        self.appState = appState
    }

    func mtkView(_ view: MTKView, drawableSizeWillChange size: CGSize) {
        // do nothing
    }

    func draw(in view: MTKView) {
        let size = view.frame.size
        let scale = view.contentScaleFactor
        update(appState, Float(size.width), Float(size.height))
        render(appState, Float(size.width), Float(size.height), Float(scale))
    }


}

struct MetalLayerView: UIViewRepresentable {

    var appState: OpaquePointer

    func makeCoordinator() -> Coordinator {
        Coordinator()
    }

    class Coordinator: NSObject {
        var renderer: Renderer?
    }

    func makeUIView(context: Context) -> UIView {
        let view = MTKView(frame: CGRect(x: 0, y: 0, width: 1024, height: 768), device: MTLCreateSystemDefaultDevice())
        var metalLayer = view.layer as! CAMetalLayer
        setup_surface(appState, &metalLayer)

        let renderer = Renderer(appState: appState)
        context.coordinator.renderer = renderer
        view.delegate = renderer

        return view
    }

    func updateUIView(_ uiView: UIView, context: Context) {
        // do nothing

    }

}

