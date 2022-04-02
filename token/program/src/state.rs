use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
    pubkey::Pubkey
};

/* Think of state.rs as essentially building the data model of your program
The data model is composed of data objects (structs) that can have helper methods.
Helper methods serve can serve to validate input and perform deserialization/serialization operations
*/

/* In general when building systems, it's a good idea to have 
 some distinguishing tag for your different account types.
This can be infered if the account types have different sizes.
However if different account types have the same size, there are potentially
 malicioius things attackers can do to the program.
AccountTag allows us to distinguish between Mint account and TokenAccounts
*/
#[derive(BorshDeserialize, BorshSerialize, Debug, Clone, PartialEq)]
pub enum AccountTag {
    Uninitialized,
    Mint,
    TokenAccount
}

// We need 2 structs for our token program: Mint and TokenAccount

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
pub struct Mint {
    pub tag: AccountTag,
    pub authority: Pubkey,
    pub supply: u64,
}

impl Mint {
    // Helper functions
    pub fn load_unchecked(ai: &AccountInfo) -> Result<Self, ProgramError> {
        return Ok(
            Self::try_from_slice(
                &ai.data.borrow()
            )?
        )
    }

    fn validate(&self) -> ProgramResult {
        if self.tag != AccountTag::Mint {
            return Err(ProgramError::InvalidAccountData)
        }
        return Ok(())
    }

    pub fn load(ai: &AccountInfo) -> Result<Self, ProgramError> {
        let mint = Self::try_from_slice(&ai.data.borrow())?;
        mint.validate()?;
        return Ok(mint)
    }

    pub fn save(&self, ai: &AccountInfo) -> ProgramResult {
        return Ok(
            self.serialize(
                &mut *ai.data.borrow_mut()
            )?
        )
    }
}

// Token Account

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
pub struct TokenAccount {
    pub tag: AccountTag,
    pub owner: Pubkey,
    pub mint: Pubkey, // specifies which Token Type / Mint (e.g. USDC, BTC, etc.)
    pub amount: u64,
}

impl TokenAccount {
        // Helper functions
        pub fn load_unchecked(ai: &AccountInfo) -> Result<Self, ProgramError> {
            return Ok(
                Self::try_from_slice(
                    &ai.data.borrow()
                )?
            )
        }
    
        fn validate(&self) -> ProgramResult {
            if self.tag != AccountTag::TokenAccount {
                return Err(ProgramError::InvalidAccountData)
            }
            return Ok(())
        }
    
        pub fn load(ai: &AccountInfo) -> Result<Self, ProgramError> {
            let mint = Self::try_from_slice(&ai.data.borrow())?;
            mint.validate()?;
            return Ok(mint)
        }
    
        pub fn save(&self, ai: &AccountInfo) -> ProgramResult {
            return Ok(
                self.serialize(
                    &mut *ai.data.borrow_mut()
                )?
            )
        }
}