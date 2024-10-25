use arch_program::{
    account::AccountInfo,
    entrypoint,
    helper::get_state_transition_tx,
    input_to_sign::InputToSign,
    instruction::Instruction,
    msg,
    program::{
        get_bitcoin_tx, get_network_xonly_pubkey, invoke, next_account_info, set_return_data,
        set_transaction_to_sign, validate_utxo_ownership,
    },
    program_error::ProgramError,
    pubkey::Pubkey,
    system_instruction::SystemInstruction,
    transaction_to_sign::TransactionToSign,
    utxo::UtxoMeta,
};
use borsh::{BorshDeserialize, BorshSerialize};

entrypoint!(process_instruction);
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> Result<(), ProgramError> {
    let params: LiquidityPoolParams = borsh::from_slice(instruction_data).unwrap();

    match params {
        LiquidityPoolParams::InitializePool((
            open_pool_params,
            user_psbt,
            param_utxos,
            serialized_utxos,
        )) => {
            let utxos = serialized_utxos
                .iter()
                .map(|serialized_utxo| UtxoMeta::from_slice(serialized_utxo))
                .collect::<Vec<UtxoMeta>>();

            if accounts.len() != 2 {
                panic!("account length mismatch");
            }

            let account_iter = &mut accounts.iter();
            let liquidity_pool_account = next_account_info(account_iter)?;
            assert!(
                liquidity_pool_account.is_writable,
                "liquidity_pool_account must be writable"
            );
            let mut pool: LiquidityPool =
                borsh::from_slice(&liquidity_pool_account.data.try_borrow().unwrap()).unwrap();
            let caller_account = next_account_info(account_iter)?;
            assert!(caller_account.is_signer, "caller_account must be signer");

            initialize_pool(
                &mut pool,
                open_pool_params,
                caller_account.key,
                user_psbt,
                param_utxos,
                &utxos,
            );

            liquidity_pool_account
                .data
                .try_borrow_mut()
                .unwrap()
                .copy_from_slice(&borsh::to_vec(&pool).unwrap());
        }
        _ => {}
    }

    Ok(())
}

pub fn initialize_pool(
    pool: &mut LiquidityPool,
    params: OpenPoolParams,
    caller: &Pubkey,
    user_psbt: Vec<u8>,
    param_utxos: OpenPoolUtxos,
    utxos: &Vec<UtxoMeta>,
) {
}

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize)]
pub enum LiquidityPoolParams {
    InitializePool((OpenPoolParams, Vec<u8>, OpenPoolUtxos, Vec<[u8; 36]>)),
    IncreaseLiquidity,
    DecreaseLiquidity,
    Swap,
}

use std::collections::HashMap;

#[derive(Debug, BorshSerialize, BorshDeserialize, Default)]
pub struct LiquidityPool {
    pub balance_sats: u128,
    pub balance_rune: u128,
    pub fee: Option<u8>,
    pub positions: HashMap<String, LiquidityPosition>,
    pub rune_name: String,
    pub rune_id: (u64, u32),
}

#[derive(Debug, BorshSerialize, BorshDeserialize, Clone, Copy)]
pub struct LiquidityPosition(pub f64);

#[derive(Debug, BorshSerialize, BorshDeserialize, Default, Clone)]
pub struct OpenPoolParams {
    /// The initial balance of BTC in the pool (in sats)
    pub balance_sats: u64,
    /// The initial balance of runes in the pool
    pub balance_rune: u64,
    /// The trading fee that is incurred during swaps, in basis points
    pub fee: Option<u8>,
    /// Name of the rune that will be held in this pool
    pub rune_name: String,
    /// Id of the rune that will be held in this pool
    pub rune_id: (u64, u32),
}

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize)]
pub struct OpenPoolUtxos {
    pub btc_utxo: String,
    pub rune_utxo: String,
}