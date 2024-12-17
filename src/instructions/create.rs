use borsh::{BorshSerialize};

use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program::invoke,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction,
    sysvar::Sysvar,
    msg,
    program_error::ProgramError,
};


use crate::state::AddressInfo;

pub fn process_initialize_address(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    address_info: AddressInfo,
) -> ProgramResult {

    let mut account_info = accounts.iter();
    let payer_account = next_account_info(&mut account_info)?;
    let address_info_account = next_account_info(&mut account_info)?;
    let system_program = next_account_info(&mut account_info)?;

    let account_span = (address_info.try_to_vec()?).len();
    let lamports_required = Rent::get()?.minimum_balance(account_span);

    msg!("Payer account is signer: {:?}", payer_account.is_signer);
    msg!("Address info account is writable: {:?}", address_info_account.is_writable);
    msg!("Payer account is writable: {:?}", payer_account.is_writable);

    if !payer_account.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    if !address_info_account.is_writable || !payer_account.is_writable {
        return Err(ProgramError::InvalidAccountData);
    }

    if system_program.key != &solana_program::system_program::ID {
        return Err(ProgramError::IncorrectProgramId);
    }

    msg!("Creating account");

    invoke(
        &system_instruction::create_account(
            payer_account.key,
            address_info_account.key,
            lamports_required,
            account_span.try_into().unwrap(),
            program_id,
        ),
        &[
            payer_account.clone(),
            address_info_account.clone(),
            system_program.clone(),
        ],
    )?;

    address_info.serialize(&mut &mut address_info_account.data.borrow_mut()[..])?;

    msg!("Address info created successfully");

    Ok(())
}
