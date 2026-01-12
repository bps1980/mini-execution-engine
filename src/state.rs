// src/state.rs

use std::collections::HashMap;

/// Simple in-memory key-value state store.
///
/// Keys and values are opaque byte vectors to keep the store generic.
#[derive(Debug, Default)]
pub struct StateStore {
    inner: HashMap<Vec<u8>, Vec<u8>>,
}

impl StateStore {
    /// Create a new, empty state store.
    pub fn new() -> Self {
        StateStore {
            inner: HashMap::new(),
        }
    }

    /// Get a value by key.
    pub fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
        self.inner.get(key).cloned()
    }

    /// Set a key to a given value.
    pub fn set(&mut self, key: Vec<u8>, value: Vec<u8>) {
        self.inner.insert(key, value);
    }

    /// Check if a key exists.
    pub fn contains_key(&self, key: &[u8]) -> bool {
        self.inner.contains_key(key)
    }

    /// Remove a key.
    pub fn remove(&mut self, key: &[u8]) {
        self.inner.remove(key);
    }

    /// Clear the entire store.
    pub fn clear(&mut self) {
        self.inner.clear();
    }
}
