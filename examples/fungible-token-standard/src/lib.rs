#[cfg(test)]
pub mod deploy;
#[cfg(test)]
pub mod helpers;
#[cfg(test)]
pub mod instruction;
#[cfg(test)]
pub mod tests_mint;
#[cfg(test)]
pub mod tests_transfer;
#[cfg(test)]
pub mod standard_tests {

    pub const MINT_FILE_PATH: &str = ".mint.json";
    pub const MINT_OWNER_FILE_PATH: &str = ".mint_owner.json";

    use anyhow::{anyhow, Result};
    use arch_program::{
        account::AccountMeta, instruction::Instruction, pubkey::Pubkey,
        system_instruction::SystemInstruction,
    };
    use borsh::BorshSerialize;
    use ebpf_counter::{
        counter_helpers::{assign_ownership_to_program, generate_new_keypair, init_logging},
        counter_instructions::{
            build_and_send_block, build_transaction, fetch_processed_transactions,
        },
    };
    use fungible_token_standard_program::{
        mint::{InitializeMintInput, TokenMintDetails},
        token_account::TokenBalance,
    };
    use sdk::processed_transaction::Status;
    use sdk::{
        constants::{NODE1_ADDRESS, PROGRAM_FILE_PATH},
        helper::{
            get_processed_transaction, read_account_info, send_utxo, sign_and_send_instruction,
        },
    };
    use serial_test::serial;
    pub const ELF_PATH: &str =
        "./program/target/sbf-solana-solana/release/fungible_token_standard_program.so";

    use bitcoin::key::Keypair;
    use bitcoin::key::UntweakedPublicKey;
}
