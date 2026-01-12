// examples/simple_token.rs

use mini_execution_engine::{
    Contract, Context, ExecutionEngine, StateStore, EventLog, Result,
};

struct SimpleToken;

impl Contract for SimpleToken {
    fn init(&mut self, ctx: &mut Context) -> Result<()> {
        // Set an initial total supply
        let supply: u64 = 1000;
        ctx.state
            .set(b"total_supply".to_vec(), supply.to_be_bytes().to_vec());
        Ok(())
    }

    fn execute(&mut self, ctx: &mut Context, input: Vec<u8>) -> Result<Vec<u8>> {
        // For this example: just emit an event and echo the input
        ctx.events.emit("SimpleToken", "execute", input.clone());
        Ok(input)
    }
}

fn main() -> Result<()> {
    let state = StateStore::new();
    let events = EventLog::new();
    let mut engine = ExecutionEngine::new(state, events);

    let mut contract = SimpleToken;

    {
        let mut ctx = engine.context();
        contract.init(&mut ctx)?;
    }

    let payload = b"hello-token".to_vec();
    let output = engine.run(&mut contract, payload.clone())?;

    println!("Output: {:?}", String::from_utf8_lossy(&output));
    println!("Events: {:?}", engine.events().events);
    println!("Ledger entries: {}", engine.ledger().len());

    Ok(())
}
