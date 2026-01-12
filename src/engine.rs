// src/engine.rs

use crate::{
    contract::{Contract, Context},
    events::EventLog,
    ledger::Ledger,
    state::StateStore,
    errors::Result,
};

/// Core execution engine for running contracts against a state store and event log.
pub struct ExecutionEngine {
    state: StateStore,
    events: EventLog,
    ledger: Ledger,
}

impl ExecutionEngine {
    /// Create a new execution engine with empty state and event log.
    pub fn new(state: StateStore, events: EventLog) -> Self {
        ExecutionEngine {
            state,
            events,
            ledger: Ledger::new(),
        }
    }

    /// Create a fresh context bound to this engine's state and event log.
    pub fn context(&mut self) -> Context<'_> {
        Context::new(&mut self.state, &mut self.events)
    }

    /// Run a contract with a given input payload.
    ///
    /// This method:
    /// - creates a context
    /// - calls `execute` on the contract
    /// - appends emitted events to the ledger
    pub fn run<C: Contract>(&mut self, contract: &mut C, input: Vec<u8>) -> Result<Vec<u8>> {
        {
            let mut ctx = self.context();
            let output = contract.execute(&mut ctx, input)?;
            // snapshot events for this execution into the ledger
            let events_snapshot = self.events.all().to_vec();
            self.ledger.append(events_snapshot);
            Ok(output)
        }
    }

    /// Access the underlying state store (immutable).
    pub fn state(&self) -> &StateStore {
        &self.state
    }

    /// Access the underlying state store (mutable).
    pub fn state_mut(&mut self) -> &mut StateStore {
        &mut self.state
    }

    /// Access the event log (immutable).
    pub fn events(&self) -> &EventLog {
        &self.events
    }

    /// Access the event log (mutable).
    pub fn events_mut(&mut self) -> &mut EventLog {
        &mut self.events
    }

    /// Access the ledger (immutable).
    pub fn ledger(&self) -> &Ledger {
        &self.ledger
    }

    /// Reset events (but keep state and ledger).
    pub fn clear_events(&mut self) {
        self.events.clear();
    }
}
