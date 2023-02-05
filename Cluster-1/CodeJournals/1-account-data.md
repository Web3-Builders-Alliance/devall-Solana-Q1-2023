# Code Journal 1 - [Account Data](https://github.com/solana-developers/program-examples/tree/main/basics/account-data/native/program/src)

This code implements a program on Solana whose purpose is to create an account for storing the AddressInfo data structure using the CPI (cross program invocation) of the Solana system program.

# Code

This program contains the following file structure:
- `lib.rs` - entry point
- `processor.rs` - executes the create statement
- `instructions` - program instructions
   - `create.rs` - instructions for creating an account for data
   - `mod.rs` - declares parts of the current folder
- `state` - data structures
   - `address_info.rs` - structure for address
   - `mod.rs` - declares parts of the current folder

Let's take a closer look at the code

## state/[address_info.rs](https://github.com/solana-developers/program-examples/blob/main/basics/account-data/native/program/src/state/address_info.rs)

Import of dependencies
```rust
// For deserialize/serialize data
use borsh::{ BorshDeserialize, BorshSerialize };
```

AddressInfo's data structure
```rust
// A trait that adds deserialization, serialization, and logging functionality to the data structure
#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct AddressInfo {
    pub name: String,
    pub house_number: u8,
    pub street: String,
    pub city: String,
}

// Implementation of the "instance"'s creation method of data structure
impl AddressInfo {
    pub fn new(
        name: String,
        house_number: u8,
        street: String,
        city: String,
    ) -> Self {
        AddressInfo {
            name,
            house_number,
            street,
            city,
        }
    }
}
```

## [lib.rs](https://github.com/solana-developers/program-examples/blob/main/basics/account-data/native/program/src/lib.rs)

Import of dependencies
```rust
// From the Solana library entrypoint which is used as the entry point for all programs in Solana
use solana_program::entrypoint;

// From processor to be passed to entrypoint
use processor::process_instruction;
```

Importing the rest of the program
```rust
pub mod instructions;
pub mod processor;
pub mod state;
```

Call entrypoint
```rust
entrypoint!(process_instruction);
```

## [processor.rs](https://github.com/solana-developers/program-examples/blob/main/basics/account-data/native/program/src/processor.rs)

Import dependencies
```rust
// For deserialize/serialize of data
use borsh::{ BorshDeserialize, BorshSerialize };
// From SDK Solana
use solana_program::{
    // Accous
    account_info::AccountInfo, 
    // Program result
    entrypoint::ProgramResult, 
    // Program error
    program_error::ProgramError,
    // Publickey
    pubkey::Pubkey,
};

// Instructions from crate
use crate::instructions;
// Address's structure from crate
use crate::state::AddressInfo;
```

Description of the logic of a single program instruction
```rust
pub fn process_instruction(
    // Program's address
    program_id: &Pubkey,
    // Accounts transferred to it
    accounts: &[AccountInfo],
    // Passed data
    instruction_data: &[u8],
) -> ProgramResult {

    // Attempt to deserialize the passed data into an Adress structure with match
    match AddressInfo::try_from_slice(&instruction_data) {
        // If successful, create_address_info from crate is called, passing all the data
        // including the AddressInfo data structure converted from instruction_data
        // In this case, we exit the program with success and return ProgramResult
        Ok(address_info) => return instructions::create::create_address_info(
            program_id, accounts, address_info
        ),
        // If the error is nothing, but its call is below
        Err(_) => {},
    };

    // Throw an error
    Err(ProgramError::InvalidInstructionData)
}
```

## Let's dive into the logic of the instruction - instructions/[create.rs](https://github.com/solana-developers/program-examples/blob/main/basics/account-data/native/program/src/instructions/create.rs)

Import dependencies
```rust
// Deserialize/serialize with borsh
use borsh::{ BorshDeserialize, BorshSerialize };
// From Solana SDK
use solana_program::{
    // Account, method for iteration by accounts
    account_info::{ AccountInfo, next_account_info },
    // Program result
    entrypoint::ProgramResult, 
    // For CPI
    program::invoke,
    // Publickey
    pubkey::Pubkey,
    // Rent account
    rent::Rent,
    // Contains system instructions
    system_instruction,
    // Contains the system program
    system_program,
    // Sysvar account (not used in code)
    sysvar::Sysvar,
};

// Address structure from crate
use crate::state::AddressInfo;
```

Instruction describing the logic of creating an Address account
```rust
pub fn create_address_info(
    // Program address
    program_id: &Pubkey,
    // Accounts
    accounts: &[AccountInfo],
    // Converted earlier AddressInfo from instruction_data
    address_info: AddressInfo,
) -> ProgramResult {
    // Accounts iterator
    let accounts_iter = &mut accounts.iter();
    // Account for AddressInfo
    let address_info_account = next_account_info(accounts_iter)?;
    // The one who pays for the transaction, signer
    let payer = next_account_info(accounts_iter)?;
    // System program
    let system_program = next_account_info(accounts_iter)?;

    // Data size for the AddressInfo data structure
    let account_span = (address_info.try_to_vec()?).len();
    // Required number of lamports to pay rent when storing AddressInfo data
    let lamports_required = (Rent::get()?).minimum_balance(account_span);

    // CPI system instructions for creating an account where AddressInfo will be stored
    invoke(
        // System instruction for creating an account
        &system_instruction::create_account(
            // Public key payer
            &payer.key,
            // Public key of the account where Address Info will be stored
            &address_info_account.key,
            // Required number of lamports
            lamports_required,
            // Account size in bytes
            account_span as u64,
            // Address of the program that will own the account (current program)
            program_id,
        ),
        // Signers
        &[ 
            // Payer
            payer.clone(), 
            // Created Address Info account
            address_info_account.clone(), 
            // System program
            system_program.clone()
        ]
    )?;
    
    // Serialize with borsh and store data in address_info
    address_info.serialize(&mut &mut address_info_account.data.borrow_mut()[..])?;
    Ok(())
}
```