use borsh::{BorshDeserialize,  BorshSerialize};
use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey
};

use crate::instruction::TokenInstruction;
use crate::state::{AccountTag, Mint, TokenAccount};

pub struct Processor {}

impl Processor {
    pub fn process_instruction(
        _program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {

        let instruction = TokenInstruction::try_from_slice(instruction_data)
            .map_err(|_| ProgramError::InvalidInstructionData)?;
        
        let account_iter = &mut accounts.iter();

        match instruction {
            TokenInstruction::InitializeMint => {}
            TokenInstruction::InitializeTokenAccount => {}
            TokenInstruction::Mint {amount} => {}
            TokenInstruction::Burn {amount} => {}
            TokenInstruction::Transfer {amount} => {}
        }
    }
 }

