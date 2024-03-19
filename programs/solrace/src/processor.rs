use solana_program::{
  account_info::{next_account_info, AccountInfo}, 
  entrypoint::ProgramResult, 
  pubkey::Pubkey,
};
use spl_token::instruction::transfer;

pub fn process_spl_token_transfer(
  accounts: &[AccountInfo],
  amount: u64,
) -> ProgramResult {
  let account_info_iter = &mut accounts.iter();
  let source_account = next_account_info(account_info_iter)?;
  let destination_account = next_account_info(account_info_iter)?;
  let authority_account = next_account_info(account_info_iter)?;
  let token_program_account = next_account_info(account_info_iter)?;

  let transfer_instruction = transfer(
      token_program_account.key,
      source_account.key,
      destination_account.key,
      authority_account.key,
      &[&authority_account.key],
      amount,
  )?;

  solana_program::program::invoke_signed(
      &transfer_instruction,
      &[
          source_account.clone(),
          destination_account.clone(),
          authority_account.clone(),
          token_program_account.clone(),
      ],
      &[],
  )?;

  Ok(())
}
