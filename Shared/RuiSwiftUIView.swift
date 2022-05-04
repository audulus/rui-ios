
import SwiftUI
import Metal
import QuartzCore
import MetalKit

class Renderer: NSObject, MTKViewDelegate {

    var appState: AppState

    init(appState: AppState) {
        self.appState = appState
    }

    func mtkView(_ view: MTKView, drawableSizeWillChange size: CGSize) {
        // do nothing
    }

    func draw(in view: MTKView) {
        let size = view.frame.size
        let scale = view.contentScaleFactor
        appState.update(Float(size.width), Float(size.height))
        appState.render(Float(size.width), Float(size.height), Float(scale))
    }

}

class RuiView: MTKView {
    var appState: AppState

    init(appState: AppState) {
        self.appState = appState
        super.init(frame: CGRect(x: 0, y: 0, width: 1024, height: 768),
                   device: MTLCreateSystemDefaultDevice())
    }

    required init(coder: NSCoder) {
        fatalError("init(coder:) has not been implemented")
    }

    // MARK: - Event Handling

    /// Keep track of all the active touches so we can give them indices.
    var touches: [UITouch?] = Array<UITouch?>(repeating: nil, count: 16)

    func add(touch: UITouch) -> Int? {
        for i in 0..<touches.count {
            if touches[i] == nil {
                touches[i] = touch
                return i
            }
        }
        return nil
    }

    override func touchesBegan(_ touches: Set<UITouch>, with event: UIEvent?) {
        for touch in touches {
            if let index = add(touch: touch) {
                let p = touch.location(in: self)
                let appEvent = AppEvent(x: Float(p.x),
                                        y: Float(bounds.size.height - p.y),
                                        id: UInt(index),
                                        kind: .TouchBegin)
                appState.process(appEvent)
            }
        }
    }

    override func touchesMoved(_ touches: Set<UITouch>, with event: UIEvent?) {
        for index in 0..<self.touches.count {
            if let touch = self.touches[index] {
                let p = touch.location(in: self)
                let appEvent = AppEvent(x: Float(p.x),
                                        y: Float(bounds.size.height - p.y),
                                        id: UInt(index),
                                        kind: .TouchMove)
                appState.process(appEvent)
            }
        }
    }

    override func touchesEnded(_ touches: Set<UITouch>, with event: UIEvent?) {
        for index in 0..<self.touches.count {
            if let touch = self.touches[index], touches.contains(touch) {

                let p = touch.location(in: self)
                let appEvent = AppEvent(x: Float(p.x),
                                        y: Float(bounds.size.height - p.y),
                                        id: UInt(index),
                                        kind: .TouchEnd)
                appState.process(appEvent)

                self.touches[index] = nil
            }
        }
    }
}

struct RuiSwiftUIView: UIViewRepresentable {

    var appState: AppState

    func makeCoordinator() -> Coordinator {
        Coordinator()
    }

    class Coordinator: NSObject {
        var renderer: Renderer?
    }

    func makeUIView(context: Context) -> RuiView {
        let view = RuiView(appState: appState)
        view.isMultipleTouchEnabled = true
        var metalLayer = view.layer as! CAMetalLayer
        appState.setup_surface(&metalLayer)

        let renderer = Renderer(appState: appState)
        context.coordinator.renderer = renderer
        view.delegate = renderer

        return view
    }

    func updateUIView(_ uiView: RuiView, context: Context) {
        // do nothing

    }

}

