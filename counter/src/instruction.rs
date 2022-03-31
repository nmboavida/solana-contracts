use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub enum CounterInstruction {
    /* The Borsh Deserialize trait implements the following methods:
        try_from_slice ==> Deserializes the instance from a slice of bytes.
    */

    Increment, // unsigned byte
}