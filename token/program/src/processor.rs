use borsh::{BorshDeserialize};
use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey
};

use crate::instruction::TokenInstruction;
use crate::state::{AccountTag, Mint, TokenAccount};

pub struct Processor {}

pub fn asssert_with_msg(statement: bool, err: ProgramError, msg: &str) -> ProgramResult {
    if !statement {
        msg!(msg);
        Err(err)
    } else {
        Ok(())
    }
}

impl Processor {
    pub fn process_instruction(
        _program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {

        let instruction = TokenInstruction::try_from_slice(instruction_data)
            .map_err(|_| ProgramError::InvalidInstructionData)?;
        
        let accounts_iter = &mut accounts.iter();

        match instruction {
            TokenInstruction::InitializeMint => {

                let mint_ai = next_account_info(accounts_iter)?; // AccountInfo object
                let mint_authority = next_account_info(accounts_iter)?; // AccountInfo object
                let mut mint = Mint::load_unchecked(mint_ai)?; // Mint object
                
                asssert_with_msg(
                    mint_authority.is_signer,
                    ProgramError::MissingRequiredSignature,
                    "Mint authority must sign!"
                )?;

                mint.tag = AccountTag::Mint;
                mint.authority = *mint_authority.key;
                mint.supply = 0;
                mint.save(mint_ai) // serializes data and returns it
            }
            TokenInstruction::InitializeTokenAccount => {
                let token_account_ai = next_account_info(accounts_iter)?; // AccountInfo object
                let mint_ai = next_account_info(accounts_iter)?;  // AccountInfo object
                //let mint = Mint::load(mint_ai)?; // validated Mint object
                let owner = next_account_info(accounts_iter)?;  // AccountInfo object
                let mut token_account = TokenAccount::load_unchecked(token_account_ai)?; // TokenAccount object

                token_account.tag = AccountTag::TokenAccount;
                token_account.owner = *owner.key;
                token_account.mint = *mint_ai.key;
                token_account.amount = 0;
                token_account.save(token_account_ai)
            }
            TokenInstruction::Mint {amount} => {
                msg!("Instruction Mint");
                let token_account_ai = next_account_info(accounts_iter)?;
                let mint_ai = next_account_info(accounts_iter)?;
                let mint_authority = next_account_info(accounts_iter)?;

                let mut token_account = TokenAccount::load(token_account_ai)?;
                let mut mint = Mint::load(mint_ai)?;

                asssert_with_msg(
                    mint_authority.is_signer,
                    ProgramError::MissingRequiredSignature,
                    "Mint authority must sign."
                )?;

                // unsafe --> check for numerical overflow
                mint.supply += amount;
                token_account.amount += amount;

                token_account.save(token_account_ai)?;
                mint.save(mint_ai) //todo check this out
            }
            TokenInstruction::Burn {amount} => {
                msg!("Instruction Burn");
                let token_account_ai = next_account_info(accounts_iter)?;
                let mint_ai = next_account_info(accounts_iter)?;
                let owner = next_account_info(accounts_iter)?;

                let mut token_account = TokenAccount::load(token_account_ai)?;
                let mut mint = Mint::load(mint_ai)?;

                asssert_with_msg(
                    owner.is_signer, // This checks if signature of owner is present
                    ProgramError::MissingRequiredSignature,
                    "Token owner must sign."
                )?;
                // Assert that owner is the token account owner
                asssert_with_msg(
                    token_account.owner == *owner.key,
                    ProgramError::MissingRequiredSignature,
                    "Token owner mistmatch."
                )?;
                // Assert that burned amount does not surpass token_account.amount
                asssert_with_msg(
                    token_account.amount >=amount,
                    ProgramError::InvalidAccountData,
                    "Cannot burn amount superior than the amount in the token_account."
                )?;

                mint.supply -= amount;
                token_account.amount -= amount;
                token_account.save(token_account_ai)?;
                mint.save(mint_ai) //todo check this out
            }
            TokenInstruction::Transfer {amount} => {
                msg!("Instruction Transfer");
                let src_token_account_ai = next_account_info(accounts_iter)?;
                let dst_token_account_ai = next_account_info(accounts_iter)?;
                let owner = next_account_info(accounts_iter)?;

                let mut src_token_account = TokenAccount::load(src_token_account_ai)?;
                let mut dst_token_account = TokenAccount::load(dst_token_account_ai)?;

                asssert_with_msg(
                    owner.is_signer,
                    ProgramError::MissingRequiredSignature,
                    "Token owner must sign"
                )?;
                // Assert that owner is the token account owner
                asssert_with_msg(
                    src_token_account.owner == *owner.key,
                    ProgramError::MissingRequiredSignature,
                    "Token owner mistmatch."
                )?;
                // Assert that owner is the token account owner
                asssert_with_msg(
                    src_token_account.mint == dst_token_account.mint,
                    ProgramError::InvalidAccountData,
                    "Token account mints do not match."
                )?;

                src_token_account.amount -= amount;
                dst_token_account.amount += amount;
                
                src_token_account.save(src_token_account_ai)?;
                dst_token_account.save(src_token_account_ai) // todo check this out
            }
        }
    }
 }

