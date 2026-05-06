import Cocoa
import InputMethodKit

@main
class HipKeyAppDelegate: NSObject, NSApplicationDelegate {
    func applicationDidFinishLaunching(_ notification: Notification) {
        let server = IMKServer(name: "HipKeyInputMethod", bundleIdentifier: Bundle.main.bundleIdentifier)
        guard server != nil else {
            NSLog("HipKey: Failed to create IMKServer")
            return
        }
        NSLog("HipKey: Input method server started")
    }
}
