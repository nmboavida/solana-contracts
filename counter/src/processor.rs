use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::instruction::CounterInstruction;
use crate::state::Counter;


pub struct Processor {}

impl Processor {
    pub fn process_instruction(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8]
    ) -> ProgramResult {
        let instruction = (
            CounterInstruction::try_from_slice(instruction_data)
            .map_err(|_| ProgramError::InvalidInstructionData)
        )?;

        match instruction {
            CounterInstruction::Increment => {
                msg!("Instruction: Increment");
                let accounts_iter = &mut accounts.iter();
                
                // next_account_info is exposed through Solana sdk
                // Can be used go through an iterator of accounts
                let counter_ai = next_account_info(accounts_iter)?; // it's an AccountInfo

                /* Pulling the data from the raw account info and pack it into an object
                 Remember counter_ai is an AccountInfo, which is a struct exposed by Solana sdk
                 try_from_slice is the method inherited by Counter from the BorshDeserialize trait
                */
                let mut counter = Counter::try_from_slice(&counter_ai.data.borrow())?;

                // Borsh library actually copies  the content of the buffer into a new object
                counter.count += 1; // In the local copy of the data buffer, the counter has increased

                // Write it back to the account
                counter.serialize(&mut *counter_ai.data.borrow_mut())?;
            }
        }

        Ok(())
    }
}