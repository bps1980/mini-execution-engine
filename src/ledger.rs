// src/ledger.rs

use crate::events::Event;

/// A single ledger entry representing an executed transaction or operation.
#[derive(Debug, Clone)]
pub struct LedgerEntry {
    pub index: u64,
    pub events: Vec<Event>,
}

/// A mock ledger built on top of an in-memory list of entries.
///
/// In a real chain, this would correspond to blocks or commits.
#[derive(Debug, Default)]
pub struct Ledger {
    entries: Vec<LedgerEntry>,
}

impl Ledger {
    pub fn new() -> Self {
        Ledger { entries: Vec::new() }
    }

    /// Append a new ledger entry with the given events.
    pub fn append(&mut self, events: Vec<Event>) {
        let index = self.entries.len() as u64;
        let entry = LedgerEntry { index, events };
        self.entries.push(entry);
    }

    /// Get all entries.
    pub fn entries(&self) -> &[LedgerEntry] {
        &self.entries
    }

    /// Get last entry, if any.
    pub fn last(&self) -> Option<&LedgerEntry> {
        self.entries.last()
    }

    /// Total number of entries.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}
