//! LRU-based deduplication cache.
use lru::LruCache;
use std::num::NonZeroUsize;

/// Deduplication cache using LRU eviction.
pub struct DedupCache {
    cache: LruCache<u64, ()>,
}

impl DedupCache {
    pub fn new(capacity: usize) -> Self {
        Self {
            cache: LruCache::new(NonZeroUsize::new(capacity).expect("capacity > 0")),
        }
    }

    /// Returns true if the content is a duplicate (already seen).
    pub fn is_duplicate(&mut self, hash: u64) -> bool {
        if self.cache.contains(&hash) {
            true
        } else {
            self.cache.put(hash, ());
            false
        }
    }
}
