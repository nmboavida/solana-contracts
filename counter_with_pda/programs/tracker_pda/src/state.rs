use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{pubkey::Pubkey};

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
pub struct Tracker {
    pub bump: u8, // bump seed of tracker
    pub auth_bump: u8, // bump seed of auth
    pub counter: Pubkey, // to help aggregation if needed
    pub count: u64,
}