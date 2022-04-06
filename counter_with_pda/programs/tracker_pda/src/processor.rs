use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::{invoke_signed, invoke},
    program_error::ProgramError,
    pubkey::Pubkey,
    system_instruction,
    system_program::ID as SYSTEM_PROGRAM_ID,
    sysvar::{rent::Rent, Sysvar},
};
use authorized_counter::instruction::CounterInstruction;

use crate::instruction::TrackerInstruction;
use crate::state::Tracker;

pub fn assert_with_msg(statement: bool, err: ProgramError, msg: &str) -> ProgramResult {
    if !statement {
        msg!(msg);
        Err(err)
    } else {
        Ok(())
    }
}

/* The tacker program itself has no ability to authorize any kind of state change to any account that is owned by the counter program
   because accounts can only be modified by the program owner itself, so the only way to do it is by making a CPI.
   However because the CPI requires an authorize operation the only you can make this work through a different program 
   is using a PDA to sign 
*/
pub struct Processor {}

impl Processor {
    pub fn process_instruction(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8]
    ) -> ProgramResult { 
        let instruction = TrackerInstruction::try_from_slice(instruction_data)
            .map_err(|_| ProgramError::InvalidInstructionData)?;
        
        let accounts_iter = &mut accounts.iter();

        match instruction {
            TrackerInstruction::Initialize => {
                msg!("Instruction: Initialize");
                let tracker_ai = next_account_info(accounts_iter)?;
                let user = next_account_info(accounts_iter)?;
                let authority = next_account_info(accounts_iter)?;
                let counter = next_account_info(accounts_iter)?;
                let system_program = next_account_info(accounts_iter)?;

                let (authority_key, auth_bump) = Pubkey::find_program_address(
                        &[counter.key.as_ref()], // seeds
                        &program_id, // program_id
                );

                let (tracker_key, tracker_bump) = Pubkey::find_program_address(
                    &[user.key.as_ref()], // seeds
                    &program_id, // program_id
                );

                // use invoke when no need for PDA to sign
                // use invoke_sign when need for PDS to sign
                invoke_signed(
                    &system_instruction::create_account(
                        user.key, // from_pubkey
                        tracker_ai.key, // to_pubkey
                        Rent::get()?.minimum_balance(42), // lamports 
                        42, // space
                        &program_id, // owner
                    ), // instruction
                    &[user.clone(), tracker_ai.clone(), system_program.clone()], // account_infos
                    &[&[user.key.as_ref(), counter.key.as_ref(), &[tracker_bump]]],
                )?;

                assert_with_msg(
                    *system_program.key == SYSTEM_PROGRAM_ID, // * stands for derefencing, typically used in assertions
                    ProgramError::InvalidArgument,
                    "Invalid passed in for system programs"
                )?;

                assert_with_msg(
                    tracker_key == *tracker_ai.key,
                    ProgramError::InvalidArgument,
                    "Invalid PDA seeds for tracker"
                )?;

                assert_with_msg(
                    authority_key == *authority.key,
                    ProgramError::InvalidArgument,
                    "Invalid PDA seeds for authority"
                )?;

                let mut tracker = Tracker::try_from_slice(&tracker_ai.data.borrow())?;
                tracker.bump = tracker_bump;
                tracker.auth_bump = auth_bump;

                // Useful for client side queries
                tracker.counter = *counter.key;
                tracker.count = 0;

                tracker.serialize(&mut *tracker_ai.data.borrow_mut())?
            }
            TrackerInstruction::Increment => {
                msg!("Instruction: Increment");
                // Decode account info
                let tracker_ai = next_account_info(accounts_iter)?;
                let user = next_account_info(accounts_iter)?;
                let counter_program = next_account_info(accounts_iter)?;
                let counter = next_account_info(accounts_iter)?;
                let authority = next_account_info(accounts_iter)?;

                // Deserialize account data
                let mut tracker = Tracker::try_from_slice(&tracker_ai.data.borrow())?;
                
                // Validate auth seeds
                let authority_seeds = &[counter.key.as_ref(), &[tracker.auth_bump]];
                let auth_key = Pubkey::create_program_address(
                    authority_seeds, // seeds: &[&[u8]]
                    program_id, // program_id: &Pubkey
                )?;

                assert_with_msg(
                    auth_key == *authority.key, //statement
                    ProgramError::InvalidArgument, // err
                    "Invalid PDA seeds for authority"
                )?;

                // Validate tracker seeds
                let tracker_seeds = &[user.key.as_ref(), counter.key.as_ref(), &[tracker.bump]];
                let tracker_key = Pubkey::create_program_address(
                    tracker_seeds, // seeds: &[&[u8]]
                    program_id, // program_id: &Pubkey
                )?;

                assert_with_msg(
                    tracker_key == *tracker_ai.key, //statement
                    ProgramError::InvalidArgument, // err
                    "Invalid PDA seeds for tracker"
                )?;
                
                // CPI to authorized_counter
                invoke_signed( // -> ProgramResult
                    &authorized_counter::instruction::increment(
                        *counter_program.key, // program_id: Pubkey
                        *counter.key, // counter: Pubkey -> ALl the accounts that are used by the downstream progrma from the CPI
                        // also need to be passed in to the instruction that we are implementing, that makes the CPI
                        *authority.key, // authority: Pubkey
                        CounterInstruction::Increment // instruction: CounterInstruction
                    )?, // instruction: &Instruction
                    &[ // We need to pass in the actual data buffers that the downstream program will interact with
                    // in this case it is going to be all the accounts that are also passed in with the Pubkeys
                        counter_program.clone(),
                        counter.clone(),
                        authority.clone(),
                    ], // account_infos: &[AccountInfo]
                    &[
                        &[counter.key.as_ref(), &[tracker.auth_bump]]
                    ], // signers_seeds: &[&[&[u8]]] --> this is a Ref to a List of Ref to a List of Refs of bytes
                    // The idea is that every set of signers is going to be a list of byte strings
                    // It's possible to have multiple PDAs signing for the same instruction 
                    // hence the multiple lists of lists of bytestrings - 3 layer nested object
                )?;

                tracker.count += 1; // potential bug: the tracker only updates via CPI calls and not calls disrectly made
                // to the authorized_counter program so there's a chance that the tracker.count differs eventually from the count 
                // controlled by the downstream program
                msg!("User Count {}", tracker.count);
                tracker.serialize(&mut *tracker_ai.data.borrow_mut())?;
            } 
        }
        Ok(())
    }
}

