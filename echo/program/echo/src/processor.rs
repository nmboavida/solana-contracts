use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{AccountInfo, next_account_info},
    pubkey::Pubkey, entrypoint::ProgramResult, msg,
    program_error::ProgramError, 
};

struct Processor {}

impl Processor {
    pub fn process_instruction(
        _program_id: &Pubkey,
        accounts: &[Pubkey],
        data: &[u8],
    ) -> ProgramResult {
        // iterate-assign account

        let instruction = TokenInstruction::try_from_slice(data)
    }

}