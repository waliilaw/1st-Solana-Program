use borsh::BorshDeserialize;
use borsh_derive::{BorshDeserialize, BorshSerialize};
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

entrypoint!(counter_contract);

pub fn counter_contract(
    program_id : &Pubkey,
    accounts : &[AccountInfo],
    instruction_data : &[u8]
 ) -> ProgramResult {
    let acc: &AccountInfo<'_> = next_account_info(&mut accounts.iter())?;

    let instruction_type = InstructionType::try_from_slice(instruction_data)?;

    match instruction_type {
        InstructionType::Increment(value ) => {
            acc.data += value 
        } ,
        InstructionType::Decrement(value ) => {
            acc.data -= value
        } 
    }
}           