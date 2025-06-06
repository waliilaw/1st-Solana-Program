use solana_program::{
    account_info::{AccountInfo , next_account_info},
    entrypoint::ProgramResult, entrypoint,
    pubkey::Pubkey,
    msg    
};

enum 

entrypoint!(counter_contract);

pub fn counter_contract(
    program_id : &Pubkey,
    accounts : &[AccountInfo],
    instruction_data : &[u8]
 ) -> ProgramResult {
    let acc: &AccountInfo<'_> = next_account_info(&mut accounts.iter())?;
}           