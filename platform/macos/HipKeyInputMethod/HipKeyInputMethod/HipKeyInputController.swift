import Cocoa
import InputMethodKit

class HipKeyInputController: IMKInputController {
    private var engine: OpaquePointer?
    private var lastCommitted: String = ""

    override init!(server: IMKServer!, delegate: Any!, client inputClient: Any!) {
        super.init(server: server, delegate: delegate, client: inputClient)
        engine = hipkey_engine_create()
        if engine != nil {
            _ = hipkey_engine_set_language_pack_vi(engine, 0)
        }
        NSLog("HipKey: Input controller created")
    }

    deinit {
        if engine != nil {
            hipkey_engine_destroy(engine)
        }
    }

    override func handle(_ event: NSEvent!, client sender: Any!) -> Bool {
        guard let event = event else { return false }
        guard event.type == .keyDown else { return false }

        let keyCode = mapKeyCode(event)
        let modifiers = event.modifierFlags
        let shift = modifiers.contains(.shift)
        let ctrl = modifiers.contains(.control)
        let alt = modifiers.contains(.option)
        let meta = modifiers.contains(.command)

        if meta || ctrl {
            return false
        }

        let result = hipkey_process_keystroke(
            engine,
            keyCode,
            shift, ctrl, alt, meta
        )

        switch result {
        case HIPKEY_EVENT_BUFFER_CHANGED:
            updateComposing(client: sender)
            return true

        case HIPKEY_EVENT_COMMIT:
            commitAndClear(client: sender)
            return true

        case HIPKEY_EVENT_PASS_THROUGH:
            return false

        case HIPKEY_EVENT_CANDIDATES_UPDATED:
            showCandidates(client: sender)
            return true

        default:
            return false
        }
    }

    override func commitComposition(_ sender: Any!) {
        commitAndClear(client: sender)
    }

    private func updateComposing(client: Any?) {
        guard let client = client as? IMKTextInputProtocol else { return }
        guard let composingPtr = hipkey_get_composing_text(engine) else { return }
        let composing = String(cString: composingPtr)
        hipkey_string_free(composingPtr)

        client.setMarkedText(composing, selection: NSRange(location: composing.count, length: 0),
                             replacementRange: NSRange(location: NSNotFound, length: 0))
    }

    private func commitAndClear(client: Any?) {
        guard let client = client as? IMKTextInputProtocol else { return }

        if let ptr = hipkey_get_composing_text(engine) {
            let text = String(cString: ptr)
            hipkey_string_free(ptr)
            if !text.isEmpty {
                client.insertText(text, replacementRange: NSRange(location: NSNotFound, length: 0))
            }
        }

        hipkey_commit(engine)
        hipkey_clear(engine)
        client.setMarkedText("", selection: NSRange(location: 0, length: 0),
                             replacementRange: NSRange(location: NSNotFound, length: 0))
    }

    private func showCandidates(client: Any?) {
        guard let client = client as? IMKTextInputProtocol else { return }

        let candidateList = hipkey_get_candidates(engine)
        guard candidateList.len > 0, let candidates = candidateList.candidates else {
            return
        }

        var displayText = ""
        for i in 0..<Int(candidateList.len) {
            let candidate = candidates[i]
            if let textPtr = candidate.text {
                let text = String(cString: textPtr)
                let number = i < 9 ? "\(i + 1). " : ""
                displayText += "\(number)\(text)\n"
            }
        }

        hipkey_candidate_list_free(candidateList)

        if !displayText.isEmpty {
            client.setMarkedText(displayText, selection: NSRange(location: 0, length: 0),
                                 replacementRange: NSRange(location: NSNotFound, length: 0))
        }
    }

    private func mapKeyCode(_ event: NSEvent) -> UInt32 {
        let chars = event.charactersIgnoringModifiers
        if let char = chars?.first {
            let scalar = char.asciiValue ?? 0
            if scalar >= 0x20 && scalar <= 0x7E {
                return UInt32(scalar)
            }
        }

        switch event.keyCode {
        case 51: return 0x08  // Backspace
        case 117: return 0x7F // Delete
        case 36: return 0x0D  // Enter
        case 53: return 0x1B  // Escape
        case 48: return 0x09  // Tab
        case 49: return 0x20  // Space
        case 126: return 0x11 // Arrow Up
        case 125: return 0x12 // Arrow Down
        case 123: return 0x13 // Arrow Left
        case 124: return 0x14 // Arrow Right
        default: return UInt32(event.keyCode)
        }
    }
}
