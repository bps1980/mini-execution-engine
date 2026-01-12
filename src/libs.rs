// src/lib.rs

pub mod contract;
pub mod engine;
pub mod state;
pub mod events;
pub mod ledger;
pub mod errors;

pub use crate::contract::{Contract, Context};
pub use crate::engine::ExecutionEngine;
pub use crate::state::StateStore;
pub use crate::events::{Event, EventLog};
pub use crate::ledger::{Ledger, LedgerEntry};
pub use crate::errors::{Error, Result};
