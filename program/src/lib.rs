use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
account_info::{next_account_info, AccountInfo},
entrypoint,
entrypoint::ProgramResult,
msg,
program_error::ProgramError,
pubkey::Pubkey,
rent::Rent,
sysvar::Sysvar,
};

fn process_instruction(
program_id: &Pubkey,
accounts: &[AccountInfo],
instruction_data: &[u8],
) -> ProgramResult {
if instruction_data.len() == 0 {
return Err(ProgramError::InvalidInstructionData);
}
if instruction_data[0] == 0 {
    return create_campaign(
        program_id,
        accounts,
        &instruction_data[1..instruction_data.len()],
    );
} else if instruction_data[0] == 1 {
    return take_out(
        program_id,
        accounts,
        &instruction_data[1..instruction_data.len()],
    );
} else if instruction_data[0] == 2 {
    return donation(
        program_id,
        accounts,
        &instruction_data[1..instruction_data.len()],
    );
}
msg!("Entry point not found");
Err(ProgramError::InvalidInstructionData)
}

entrypoint!(process_instruction);

#[derive(BorshSerialize, BorshDeserialize, Debug)]
struct CampaignDetails {
pub admin: Pubkey,
pub name: String,
pub description: String,
pub image_link: String,
pub amount_donated: u64,
}

fn create_campaign(
program_id: &Pubkey,
accounts: &[AccountInfo],
instruction_data: &[u8],
) -> ProgramResult {
let accounts_iter = &mut accounts.iter();
let writing_account = next_account_info(accounts_iter)?;
let creator_account = next_account_info(accounts_iter)?;
if !creator_account.is_signer {
msg!("The creator account should be signer");
return Err(ProgramError::IncorrectProgramId);
}
if writing_account.owner != program_id {
    msg!("The writing account is not owned by the program");
    return Err(ProgramError::IncorrectProgramId);
}

let mut input_data = CampaignDetails::try_from_slice(&instruction_data)
    .expect("Serialization of instruction data failed");

if input_data.admin != *creator_account.key {
    msg!("Invalid instruction data");
    return Err(ProgramError::InvalidInstructionData);
}
let rent_exemption = Rent::get()?.minimum_balance(writing_account.data_len());
if **writing_account.lamports.borrow() < rent_exemption {
    msg!("The balance of writing_account should be more than rent_exemption");
    return Err(ProgramError::InsufficientFunds);
}
input_data.amount_donated = 0;
input_data.serialize(&mut &mut writing_account.try_borrow_mut_data()?[..])?;
Ok(())
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
struct WithdrawRequest {
pub amount: u64,
}

fn take_out(
program_id: &Pubkey,
accounts: &[AccountInfo],
instruction_data: &[u8],
) -> ProgramResult {
let accounts_iter = &mut accounts.iter();
let writing_account = next_account_info(accounts_iter)?;
let admin_account = next_account_info(accounts_iter)?;
if writing_account.owner != program_id {
    msg!("The writing account is not owned by the program");
    return Err(ProgramError::IncorrectProgramId);
}
if !admin_account.is_signer {
    msg!("The admin account should be signer");
    return Err(ProgramError::IncorrectProgramId);
}
let campaign_data = CampaignDetails::try_from_slice(*writing_account.data.borrow())
    .


