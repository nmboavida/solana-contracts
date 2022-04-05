use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub enum TrackerInstruction {
    Initialize,
    Increment, // idea here is to be able to increment the counter (authorized_counter program) via pda and cpi
}