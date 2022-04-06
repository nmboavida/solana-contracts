use borsh::{BorshDeserialize, BorshSerialize};


#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub enum EchoInstruction {
    Echo { data: Vec<u8> },
    InitAuthorizedEcho {
        buffer_seed: u64,
        buffer_size: usize,
    },
    AuthorizedEcho { data: Vec<u8> },
    InitVendingMachineEcho {
        // number of tokens required to change the buffer
        price: u64,
        buffer_size: usize,
    },
    VendingMachineEcho { data: Vec<u8> }
}