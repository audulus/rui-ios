
import SwiftUI
import Metal
import QuartzCore

struct MetalLayerView: UIViewRepresentable {

    var appState: OpaquePointer

    func makeCoordinator() -> Coordinator {
        Coordinator()
    }

    class Coordinator: NSObject {
        var layer: CAMetalLayer?
    }

    func makeUIView(context: Context) -> UIView {
        let view = UIView()
        view.frame.size = CGSize(width: 1024, height: 768)

        var metalLayer = CAMetalLayer()
        metalLayer.device = MTLCreateSystemDefaultDevice()!
        metalLayer.pixelFormat = .bgra8Unorm_srgb
        metalLayer.frame = view.layer.frame
        metalLayer.framebufferOnly = true

        context.coordinator.layer = metalLayer

        setup_surface(appState, &metalLayer)

        view.layer.addSublayer(metalLayer)
        return view
    }

    func updateUIView(_ uiView: UIView, context: Context) {
        // do nothing
        print("updateUIView")

        let size = uiView.frame.size
        let scale = UIScreen.main.scale
        uiView.contentScaleFactor = scale

        print("uiView size: \(size)")
        print("scale: \(scale)")

        if let metalLayer = context.coordinator.layer {
            metalLayer.frame = uiView.layer.frame

            metalLayer.drawableSize = CGSize(width: size.width * scale, height: size.height * scale)

            print("drawableSize: \(metalLayer.drawableSize)")
        }

        render(appState, Float(size.width), Float(size.height), Float(scale))
    }

}

