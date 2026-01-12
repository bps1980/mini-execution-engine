// src/events.rs

/// A single event emitted by a contract during execution.
#[derive(Debug, Clone)]
pub struct Event {
    pub contract: String,
    pub name: String,
    pub data: Vec<u8>,
}

/// In-memory event log for a single execution engine instance.
#[derive(Debug, Default)]
pub struct EventLog {
    pub events: Vec<Event>,
}

impl EventLog {
    /// Create a new, empty event log.
    pub fn new() -> Self {
        EventLog { events: Vec::new() }
    }

    /// Emit a new event.
    pub fn emit(&mut self, contract: &str, name: &str, data: Vec<u8>) {
        self.events.push(Event {
            contract: contract.to_string(),
            name: name.to_string(),
            data,
        });
    }

    /// Get all events.
    pub fn all(&self) -> &[Event] {
        &self.events
    }

    /// Clear all events.
    pub fn clear(&mut self) {
        self.events.clear();
    }
}
