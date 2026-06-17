/**
 * hip-key C API
 *
 * Language-agnostic input method engine.
 * Build: link against libhip_key_ffi.a (static) or libhip_key_ffi.dylib/.so (dynamic)
 */

#ifndef HIP_KEY_H
#define HIP_KEY_H

#include <stdint.h>
#include <stdbool.h>

#ifdef __cplusplus
extern "C" {
#endif

/* ========== Opaque Types ========== */

typedef struct HipKeyEngine HipKeyEngine;

/* ========== Result Codes ========== */

typedef enum HipKeyResult {
    HIPKEY_SUCCESS         =  0,
    HIPKEY_ERROR           = -1,
    HIPKEY_INVALID_ARGUMENT = -2,
    HIPKEY_NOT_READY       = -3,
} HipKeyResult;

/* ========== Engine Events ========== */

typedef enum HipKeyEngineEvent {
    HIPKEY_EVENT_BUFFER_CHANGED    = 1,
    HIPKEY_EVENT_CANDIDATES_UPDATED = 2,
    HIPKEY_EVENT_COMMIT            = 3,
    HIPKEY_EVENT_PASS_THROUGH      = 4,
    HIPKEY_EVENT_ERROR             = -1,
} HipKeyEngineEvent;

/* ========== Candidates ========== */

typedef struct HipKeyCandidate {
    char* text;
    float confidence;
} HipKeyCandidate;

typedef struct HipKeyCandidateList {
    HipKeyCandidate* candidates;
    size_t len;
} HipKeyCandidateList;

/* ========== Input Method IDs ========== */

#define HIPKEY_VI_TELEX 0
#define HIPKEY_VI_VNI   1

/* ========== Engine Lifecycle ========== */

/**
 * Create a new engine instance.
 * Returns NULL on failure.
 * Must be destroyed with hipkey_engine_destroy().
 */
HipKeyEngine* hipkey_engine_create(void);

/**
 * Destroy an engine instance and free all resources.
 * Passing NULL is safe (no-op).
 */
void hipkey_engine_destroy(HipKeyEngine* engine);

/* ========== Language Pack ========== */

/**
 * Set the Vietnamese language pack on the engine.
 * method: HIPKEY_VI_TELEX (0) or HIPKEY_VI_VNI (1)
 */
HipKeyResult hipkey_engine_set_language_pack_vi(HipKeyEngine* engine, uint32_t method);

/* ========== Input Processing ========== */

/**
 * Process a keystroke through the engine.
 *
 * key_code: ASCII char code (0x20-0x7E), or special:
 *   0x08 = Backspace, 0x7F = Delete, 0x0D = Enter,
 *   0x1B = Escape, 0x09 = Tab, 0x20 = Space
 *   0x11-0x14 = Arrow Up/Down/Left/Right
 * shift, ctrl, alt, meta: modifier key state
 */
HipKeyEngineEvent hipkey_process_keystroke(
    HipKeyEngine* engine,
    uint32_t key_code,
    bool shift,
    bool ctrl,
    bool alt,
    bool meta
);

/* ========== Buffer Access ========== */

/**
 * Get the current composing text.
 * Caller must free the returned string with hipkey_string_free().
 * Returns NULL on error.
 */
char* hipkey_get_composing_text(HipKeyEngine* engine);

/**
 * Get the committed text.
 * Caller must free the returned string with hipkey_string_free().
 * Returns NULL on error.
 */
char* hipkey_get_committed_text(HipKeyEngine* engine);

/**
 * Commit the current composition.
 * The composing text is moved to committed.
 */
HipKeyResult hipkey_commit(HipKeyEngine* engine);

/**
 * Get the last committed text (from a Commit event).
 * Caller must free the returned string with hipkey_string_free().
 * Returns NULL if no commit has occurred.
 */
char* hipkey_get_last_committed(HipKeyEngine* engine);

/* ========== Candidates ========== */

/**
 * Get the current candidate list.
 * Caller must free with hipkey_candidate_list_free().
 */
HipKeyCandidateList hipkey_get_candidates(HipKeyEngine* engine);

/**
 * Free a candidate list and all its strings.
 */
void hipkey_candidate_list_free(HipKeyCandidateList list);

/* ========== State ========== */

/**
 * Check if the engine has an active composition.
 */
bool hipkey_is_composing(HipKeyEngine* engine);

/**
 * Clear all engine state (composition, candidates, etc.).
 */
HipKeyResult hipkey_clear(HipKeyEngine* engine);

/* ========== Memory Management ========== */

/**
 * Free a string returned by any hipkey function.
 * Passing NULL is safe (no-op).
 */
void hipkey_string_free(char* s);

/* ========== Agent API ========== */

typedef struct HipKeyActionResult {
    bool success;
    char* display_text;
    char* commit_text;
    bool should_commit;
} HipKeyActionResult;

/**
 * Enable the agent (intent detection + action automation).
 */
HipKeyResult hipkey_agent_enable(HipKeyEngine* engine);

/**
 * Disable the agent.
 */
HipKeyResult hipkey_agent_disable(HipKeyEngine* engine);

/**
 * Check if the agent is enabled.
 */
bool hipkey_agent_is_enabled(HipKeyEngine* engine);

/**
 * Process text through the agent. Returns a HipKeyActionResult.
 * The caller must free the result with hipkey_agent_action_result_free().
 */
HipKeyActionResult hipkey_agent_process(HipKeyEngine* engine, const char* text);

/**
 * Get the display_text pointer from a HipKeyActionResult.
 * Does not transfer ownership; use hipkey_agent_action_result_free to free.
 */
char* hipkey_agent_action_result_display_text(HipKeyActionResult result);

/**
 * Free a HipKeyActionResult and its strings.
 */
void hipkey_agent_action_result_free(HipKeyActionResult result);

#ifdef __cplusplus
}
#endif

#endif /* HIP_KEY_H */
