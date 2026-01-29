# Core Principles

These principles are intended to hold for 10+ years.

## 1. Language-Agnostic Core

The core engine must not contain any language-specific logic.

### Do:
- Generic keystroke processing
- Buffer state management
- Plugin interface for language packs

### Don't:
- Hardcode Telex/VNI rules in core
- Assume Vietnamese-specific behavior
- Add language tied to English assumptions

## 2. Language Packs Are Independent

Each language is a separate, versioned crate.

### Do:
- Load language packs dynamically
- Support multiple language packs simultaneously
- Keep language data isolated

### Don't:
- Cross-pollinate language logic
- Make core depend on specific language pack

## 3. No Forced Auto-Correction

Users must be in control.

### Do:
- Provide optional suggestions
- Allow users to accept/reject
- Preserve user intent

### Don't:
- Automatically "fix" what user typed
- Assume user made a mistake
- Hide corrections from user awareness

## 4. Local-First, No Mandatory Cloud

Privacy and reliability.

### Do:
- Work offline by default
- Store data locally
- Make cloud features explicitly opt-in

### Don't:
- Require network for basic operation
- Send keystrokes to remote servers
- Break when offline

## 5. Latency > Intelligence

Speed is a feature. Slow IME is broken IME.

### Do:
- Profile hot paths
- Keep state in memory
- Avoid blocking operations

### Don't:
- Add heavy computation before UI update
- Make network calls on keystroke path
- Optimize prematurely (but do profile)

## Decision Framework

When faced with a design choice, ask:

1. Does this violate language-agnostic core?
2. Is this respectful of user intent?
3. Does this require cloud? If so, is it optional?
4. Does this add latency? Can it be avoided?

If answer violates any principle, reject the approach.
