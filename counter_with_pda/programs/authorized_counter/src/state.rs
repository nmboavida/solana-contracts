use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{pubkey::Pubkey};

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
pub struct Counter {
    pub authority: Pubkey, // we add authority to the counter
    pub count: u64,
}