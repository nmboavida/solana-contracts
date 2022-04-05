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


pub fn asssert_with_msg(statement: bool, err: ProgramError, msg: &str) -> ProgramResult {
    if !statement {
        msg!(msg);
        Err(err)
    } else {
        Ok(())
    }
}


pub struct Processor {}

impl Processor {
    pub fn process_instruction(
        _program_id: &Pubkey,
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
                let counter_ai = next_account_info(accounts_iter)?; // AccountInfo
                // We not only pass the counter but also the authority
                let authority_ai = next_account_info(accounts_iter)?;

                // Only if there is an authority signature
                // Checks if there is a signature in the instrucitons that corresponds to the authority_ai Pubkey
                asssert_with_msg(
                    authority_ai.is_signer,
                    ProgramError::MissingRequiredSignature,
                    "Authority must sign."
                )?;

                // Fetches the data from the counter_ai
                let mut counter = Counter::try_from_slice(&counter_ai.data.borrow())?;
                
                if counter.count == 0 {
                    // Set the counter authority if it's the first time counter is being used
                    counter.authority = *authority_ai.key;
                }
                // Checks that claiming authority is the actual authority for the requested data buffer
                asssert_with_msg(
                    //counter.authority is really &counter_ai.data.authority 
                    // authority_ai.key is the pubkey from the claimed authority according to the parameters
                    counter.authority == *authority_ai.key,
                    ProgramError::MissingRequiredSignature,
                    "Attempted to increment with an invalid authority"
                )?;

                counter.count += 1;
                counter.serialize(&mut *counter_ai.data.borrow_mut())?;
            }
        }

        Ok(())
    }
}