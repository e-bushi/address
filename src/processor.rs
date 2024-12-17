use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
    pubkey::Pubkey,
    msg,
};

use borsh::BorshDeserialize;

use crate::instructions::create::process_initialize_address;
use crate::state::AddressInfo;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("Creating Address");

    // let instruction = AddressInfo::try_from_slice(instruction_data)?;

    // msg!("Instruction data: {:?}", instruction);

    // Replace the if let with a match to handle the error case explicitly
    let address_info = AddressInfo::try_from_slice(instruction_data)
        .map_err(|e| {
            msg!("Failed to deserialize instruction data: {:?}", e);
            ProgramError::InvalidInstructionData
        })?;
    
    process_initialize_address(program_id, accounts, address_info)
}