use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{pubkey::Pubkey};

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
pub struct Echo {
    pub data: Vec<u8>,
}

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
pub struct AuthorizedEcho {
    pub authority: Pubkey,
    pub data: Vec<u8>,
}

// pub struct VendingMachineEcho {}