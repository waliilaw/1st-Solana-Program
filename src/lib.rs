use borsh::{BorshDeserialize, BorshSerialize};

use solana_program::{
    account_info::{AccountInfo , next_account_info},
    entrypoint::ProgramResult, entrypoint,
    pubkey::Pubkey,
    msg    
};

#[derive(BorshDeserialize , BorshSerialize)]
enum InstructionType {
    Increment(u32),
    Decrement(u32)
}

#[derive(BorshDeserialize , BorshSerialize)]
struct Counter {
   counter : u32
}

entrypoint!(counter_contract);

pub fn counter_contract(
    program_id : &Pubkey,
    accounts : &[AccountInfo],
    instruction_data : &[u8]
 ) -> ProgramResult {
    let acc: &AccountInfo<'_> = next_account_info(&mut accounts.iter())?;

    let instruction_type = InstructionType::try_from_slice(instruction_data)?;
    let mut counter_data = Counter::try_from_slice(&acc.data.borrow())?;

    match instruction_type {
        InstructionType::Increment(value ) => {
            msg!("Applying Increse");
            counter_data.counter += value;

        } ,
        InstructionType::Decrement(value ) => {
            msg!("Applying Decrease");
             counter_data.counter -= value;
        } 
    }

    counter_data.serialize(&mut *acc.data.borrow_mut())?;
    msg!("Contract Successful");
    Ok(())
}           