
 
use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)] 
 

#[carbon(discriminator = "0x4ac3c5153bbc7760")] 
pub struct MintAccount { 
        pub mint_authority: Option<solana_sdk::pubkey::Pubkey>, 
        pub supply: u64, 
        pub decimals: u8, 
        pub is_initialized: bool, 
        pub freeze_authority: Option<solana_sdk::pubkey::Pubkey>, 
}