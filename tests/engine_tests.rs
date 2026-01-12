// tests/engine_tests.rs

use mini_execution_engine::{
    Contract, Context, ExecutionEngine, StateStore, EventLog, Result,
};

struct SimpleToken;

impl Contract for SimpleToken {
    fn init(&mut self, ctx: &mut Context) -> Result<()> {
        ctx.state.set(b"total_supply".to_vec(), 1000u64.to_be_bytes().to_vec());
        Ok(())
    }

    fn execute(&mut self, ctx: &mut Context, input: Vec<u8>) -> Result<Vec<u8>> {
        // For the test: echo the input and emit an event.
        ctx.events.emit("SimpleToken", "transfer", input.clone());
        Ok(input)
    }
}

#[test]
fn simple_token_flow_works() {
    let state = StateStore::new();
    let events = EventLog::new();

    let mut engine = ExecutionEngine::new(state, events);
    let mut contract = SimpleToken;

    // init
    {
        let mut ctx = engine.context();
        contract.init(&mut ctx).expect("init should succeed");
    }

    // execute
    let input = b"test-transfer".to_vec();
    let output = engine.run(&mut contract, input.clone()).expect("execute should succeed");

    assert_eq!(output, input);

    // Check events
    assert_eq!(engine.events().events.len(), 1);
    let evt = &engine.events().events[0];
    assert_eq!(evt.contract, "SimpleToken");
    assert_eq!(evt.name, "transfer");
}
