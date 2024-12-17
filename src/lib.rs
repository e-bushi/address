use solana_program::entrypoint;
use processor::process_instruction;

pub mod processor;
pub mod instructions;
pub mod state;

entrypoint!(process_instruction);
