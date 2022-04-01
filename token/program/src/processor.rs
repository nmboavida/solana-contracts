use borsh::{BorshDeserialize,  BorshSerialize};
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
                let mint_ai = next_account_info(accounts_iter)?;
                let mint_authority = next_account_info(accounts_iter)?;
                let mut mint = Mint::load_unchecked(mint_ai)?;
                
                asssert_with_msg(
                    mint_authority.is_signer,
                    ProgramError::MissingRequiredSignature,
                    "Mint authority must sign!"
                );

                mint.tag = AccountTag::Mint;
                mint.authority = *mint_authority.key;
                mint.supply = 0;
                mint.save(mint_ai)
            }
            TokenInstruction::InitializeTokenAccount => {
                let token_account_ai = next_account_info(accounts_iter)?;
                let mint_ai = next_account_info(accounts_iter)?;
                let mint = Mint::load(mint_ai)?;
                let owner = next_account_info(accounts_iter)?;
                let mut token_account = TokenAccount::load_unchecked(token_account_ai)?;

                token_account.tag = AccountTag::TokenAccount;
                token_account.owner = *owner.key;
                token_account.mint = *mint_ai.key;
                token_account.amount = 0;
                token_account.save(token_account_ai)
            }
            TokenInstruction::Mint {amount} => {}
            TokenInstruction::Burn {amount} => {}
            TokenInstruction::Transfer {amount} => {}
        }
    }
 }

