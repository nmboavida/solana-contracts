use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub enum TokenInstruction {
    InitializeMint,
    InitializeTokenAccount,
    Mint {amount: u64},
    Burn {amouunt: u64},
    Transfer {amount: u64},
}