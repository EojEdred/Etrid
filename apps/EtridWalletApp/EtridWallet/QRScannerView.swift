import SwiftUI
import AVFoundation

struct QRScannerView: View {
    @Environment(\.dismiss) var dismiss
    @State private var scannedCode: String?
    let onCodeScanned: (String) -> Void

    var body: some View {
        NavigationView {
            ZStack {
                ScannerViewController(scannedCode: $scannedCode)
                    .edgesIgnoringSafeArea(.all)

                VStack {
                    Spacer()

                    if let code = scannedCode {
                        Text("Scanned: \(code)")
                            .padding()
                            .background(Color.black.opacity(0.7))
                            .foregroundColor(.white)
                            .cornerRadius(10)
                            .padding()
                    }
                }
            }
            .navigationTitle("Scan QR Code")
            .navigationBarTitleDisplayMode(.inline)
            .toolbar {
                ToolbarItem(placement: .navigationBarTrailing) {
                    Button("Done") {
                        dismiss()
                    }
                }
            }
            .onChange(of: scannedCode) { oldValue, newValue in
                if let code = newValue {
                    onCodeScanned(code)
                    dismiss()
                }
            }
        }
    }
}

struct ScannerViewController: UIViewControllerRepresentable {
    @Binding var scannedCode: String?
    
    func makeUIViewController(context: Context) -> ScannerVC {
        let controller = ScannerVC()
        controller.delegate = context.coordinator
        return controller
    }
    
    func updateUIViewController(_ uiViewController: ScannerVC, context: Context) {}
    
    func makeCoordinator() -> Coordinator {
        Coordinator(scannedCode: $scannedCode)
    }
    
    class Coordinator: NSObject, AVCaptureMetadataOutputObjectsDelegate {
        @Binding var scannedCode: String?
        
        init(scannedCode: Binding<String?>) {
            _scannedCode = scannedCode
        }
        
        func metadataOutput(_ output: AVCaptureMetadataOutput, didOutput metadataObjects: [AVMetadataObject], from connection: AVCaptureConnection) {
            if let metadataObject = metadataObjects.first as? AVMetadataMachineReadableCodeObject,
               let stringValue = metadataObject.stringValue {
                scannedCode = stringValue
            }
        }
    }
}

class ScannerVC: UIViewController {
    var delegate: AVCaptureMetadataOutputObjectsDelegate?
    private var captureSession: AVCaptureSession?
    private var previewLayer: AVCaptureVideoPreviewLayer?
    
    override func viewDidLoad() {
        super.viewDidLoad()
        setupCamera()
    }
    
    private func setupCamera() {
        captureSession = AVCaptureSession()
        
        guard let videoCaptureDevice = AVCaptureDevice.default(for: .video),
              let videoInput = try? AVCaptureDeviceInput(device: videoCaptureDevice),
              let captureSession = captureSession,
              captureSession.canAddInput(videoInput) else {
            return
        }
        
        captureSession.addInput(videoInput)
        
        let metadataOutput = AVCaptureMetadataOutput()
        
        if captureSession.canAddOutput(metadataOutput) {
            captureSession.addOutput(metadataOutput)
            metadataOutput.setMetadataObjectsDelegate(delegate, queue: DispatchQueue.main)
            metadataOutput.metadataObjectTypes = [.qr]
        }
        
        previewLayer = AVCaptureVideoPreviewLayer(session: captureSession)
        previewLayer?.frame = view.layer.bounds
        previewLayer?.videoGravity = .resizeAspectFill
        
        if let previewLayer = previewLayer {
            view.layer.addSublayer(previewLayer)
        }
        
        DispatchQueue.global(qos: .userInitiated).async {
            captureSession.startRunning()
        }
    }
    
    override func viewWillDisappear(_ animated: Bool) {
        super.viewWillDisappear(animated)
        captureSession?.stopRunning()
    }
}
