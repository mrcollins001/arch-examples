pub const ELF_PATH: &str = "./program/target/sbf-solana-solana/release/counter_program.so";

pub mod counter_deployment;
pub mod counter_helpers;
pub mod counter_instructions;
#[cfg(test)]
pub mod errors_and_panics;
#[cfg(test)]
pub mod happy_path;
