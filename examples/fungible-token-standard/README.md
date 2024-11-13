# Understanding the Fungible Token Standard on Arch Network

## Introduction

The Fungible Token Standard program is a crucial piece of infrastructure for the Arch Network ecosystem that enables the creation and management of fungible tokens. Similar to ERC-20 on Ethereum, this standard provides a consistent interface for implementing fungible tokens on Arch Network.

## Core Components

### Token Mint Details
The foundation of the standard is the TokenMintDetails struct which manages token metadata and supply:

```
pub struct TokenMintDetails {
    owner: [u8; 32],
    pub status: MintStatus,
    pub supply: u64,             // in lowest denomination
    pub circulating_supply: u64, // in lowest denomination
    pub ticker: String,
    pub decimals: u8,
    token_metadata: HashMap<String, [u8; 32]>,
}
```

This structure maintains critical token information including total supply, circulating supply, decimals for precision, and token metadata.

### Token Balance Management 
Account balances are handled through the TokenBalance struct:

```
pub struct TokenBalance {
    pub owner: [u8; 32],
    pub mint_account: [u8; 32],
    pub current_balance: u64, // in smallest denomination of token
}
```

## Key Features

### 1. Token Initialization
- Create new token with configurable:
  - Total supply
  - Decimal precision
  - Token symbol/ticker
  - Owner address
  - Custom metadata

### 2. Token Operations
The standard supports three core operations:

#### Minting

```
pub fn mint_tokens(
    balance_account: &AccountInfo<'_>,
    mint_account: &AccountInfo<'_>,
    owner_account: &AccountInfo<'_>,
    program_id: &Pubkey,
    mint_input: MintInput,
) -> Result<(), ProgramError> {
    /* ------------------------- Balance account checks ------------------------- */
    let mut token_balance_data = balance_account
        .data
        .try_borrow_mut()
        .map_err(|_| ProgramError::AccountBorrowFailed)?;

    let mut token_balance = TokenBalance::deserialize(&mut &token_balance_data[..])
        .map_err(|_| ProgramError::InvalidAccountData)?;

    if balance_account.owner != program_id {
        return Err(ProgramError::Custom(501));
    };

    if token_balance.mint_account != mint_account.key.serialize() {
        return Err(ProgramError::Custom(503));
    }

    if token_balance.owner != owner_account.key.serialize() {
        return Err(ProgramError::Custom(502));
    }

    /* --------------------------- MINT ACCOUNT CHECKS -------------------------- */

    let mut mint_data = mint_account
        .data
        .try_borrow_mut()
        .map_err(|_| ProgramError::AccountBorrowFailed)?;

    let mut mint_details = TokenMintDetails::deserialize(&mut &mint_data[..])
        .map_err(|_| ProgramError::InvalidAccountData)?;

    if mint_account.owner != program_id {
        return Err(ProgramError::Custom(504));
    }

    if mint_details.status == MintStatus::Finished {
        return Err(ProgramError::Custom(502));
    }

    /* -------------------------- OWNER ACCOUNT CHECKS -------------------------- */
    if !owner_account.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    /* -------------------------------- EXECUTION ------------------------------- */

    add_mint_to_circulating_supply(&mut mint_details, &mint_input)?;
    msg!("circulating_supply: {}", mint_details.circulating_supply);

    if mint_details.circulating_supply == mint_details.supply {
        mint_details.status = MintStatus::Finished;
    }

    token_balance.increase_balance(mint_input.amount, &mint_details);

    let new_serialized_token_balance = borsh::to_vec(&token_balance).unwrap();

    let new_serialized_mint_details = borsh::to_vec(&mint_details).unwrap();

    /* -------------------------- UPDATE TOKEN BALANCE -------------------------- */

    if new_serialized_token_balance.len() > token_balance_data.len() {
        balance_account.realloc(new_serialized_token_balance.len(), true)?;
    }
    token_balance_data.copy_from_slice(&new_serialized_token_balance);

    /* ---------------------------- UPDATE MINT DATA ---------------------------- */

    if new_serialized_mint_details.len() > mint_data.len() {
        mint_account.realloc(new_serialized_mint_details.len(), true)?;
    }
    mint_data.copy_from_slice(&new_serialized_mint_details);

    Ok(())
}
```


Minting allows new tokens to be created up to the maximum supply. The process includes:
- Supply validation
- Owner verification
- Balance updates
- Mint status management

#### Transfers

Transfers enable token movement between accounts with:
- Balance verification
- Ownership checks
- Atomic updates

#### Balance Management
The standard provides precise balance tracking with:
- Decimal support
- Overflow protection
- Underflow prevention
- Atomic operations

## Security Features

1. **Access Control**
- Owner verification for sensitive operations
- Program ownership validation
- Signer verification

2. **Supply Management**
- Maximum supply enforcement
- Circulating supply tracking
- Mint completion status

3. **Error Handling**

## Use Cases

1. **Stablecoins**
- Fiat-pegged tokens
- Commodity-backed tokens
- Synthetic assets

2. **Governance**
- DAO voting tokens
- Protocol governance
- Staking mechanisms

3. **DeFi Applications**
- Liquidity pool tokens
- Yield farming rewards
- Collateral assets

4. **Gaming & NFTs**
- In-game currencies
- Reward tokens
- Fractional NFT ownership

5. **Real World Assets**
- Security tokens
- Real estate tokens
- Revenue sharing tokens

## Testing Framework

The standard includes comprehensive testing:


Tests cover:
- Token initialization
- Minting operations
- Transfer scenarios
- Error conditions
- Edge cases

## Benefits for Arch Network

1. **Standardization**
- Consistent token interface
- Predictable behavior
- Ecosystem compatibility

2. **Security**
- Audited codebase
- Battle-tested patterns
- Built-in safeguards

3. **Flexibility**
- Configurable parameters
- Extensible design
- Custom metadata support

4. **Efficiency**
- Optimized operations
- Minimal storage
- Gas-efficient

## Future Development

1. **Enhanced Features**
- Token burning
- Permit/approval system
- Batch operations

2. **Cross-Chain Integration**
- Bridge support
- Multi-chain compatibility
- Atomic swaps

3. **Advanced Functionality**
- Time-locked tokens
- Vesting schedules
- Automated distributions

## Conclusion

The Fungible Token Standard provides a robust foundation for token-based applications on Arch Network. Its combination of security, flexibility, and efficiency makes it an essential tool for developers building on the platform. As the ecosystem grows, the standard will continue to evolve to meet the needs of increasingly sophisticated decentralized applications.