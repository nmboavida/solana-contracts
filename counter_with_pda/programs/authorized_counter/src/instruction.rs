use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    instruction::{AccountMeta, Instruction},
    program_error::ProgramError,
    pubkey::Pubkey,
};

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub enum CounterInstruction {
    Increment, // unsigned byte
}

pub fn increment(
    program_id: Pubkey,
    counter: Pubkey,
    authority: Pubkey,
    instruction: CounterInstruction
) -> Result<Instruction, ProgramError> {
    Ok(Instruction {
        accounts: vec![
            AccountMeta::new(counter, false), // new::() -> Writable
            AccountMeta::new_readonly(authority, true), // authority will sign but is readonly
        ],
        data: instruction.try_to_vec()?, // serializex instruction into a vec
        program_id,
    })
}