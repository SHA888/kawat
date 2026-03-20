//! Fallback comparison logic.
//! Mirrors trafilatura external.py:compare_extraction().
//!
//! Decision tree:
//! 1. If recall mode AND own > 10× min_size → use own
//! 2. Run readability on backup tree
//! 3. Compare lengths with specific heuristics (see external.py lines 45-108)
//! 4. If result still unclean → try justext rescue
//! 5. Pick best result

/// Compare trafilatura's own extraction with readability and justext fallbacks.
pub fn compare_extraction(
    _own_text: &str,
    _own_len: usize,
    _html: &str,
    _min_extracted_size: usize,
    _focus: &str,
) -> (String, usize) {
    // TODO: port the exact heuristic from external.py
    todo!("compare_extraction")
}
