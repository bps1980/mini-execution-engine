// src/contract.rs

use crate::{events::EventLog, state::StateStore, errors::Result};

/// Execution context passed into contract methods.
/// Gives access to state and event log.
pub struct Context<'a> {
    pub state: &'a mut StateStore,
    pub events: &'a mut EventLog,
}

impl<'a> Context<'a> {
    pub fn new(state: &'a mut StateStore, events: &'a mut EventLog) -> Self {
        Context { state, events }
    }
}

/// Trait that all contracts must implement.
pub trait Contract {
    /// Called once to initialize contract state.
    fn init(&mut self, ctx: &mut Context) -> Result<()>;

    /// Called to execute a contract operation.
    ///
    /// `input` is a generic byte payload; contracts decide how to decode it.
    /// Returns an opaque byte vector as output.
    fn execute(&mut self, ctx: &mut Context, input: Vec<u8>) -> Result<Vec<u8>>;
}
