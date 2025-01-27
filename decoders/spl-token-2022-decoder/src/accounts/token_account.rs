
use super::super::types::*;
 
use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)] 
 

#[carbon(discriminator = "0xdc83ec1091cecf36")] 
pub struct TokenAccount { 
        pub mint: solana_sdk::pubkey::Pubkey, 
        pub owner: solana_sdk::pubkey::Pubkey, 
        pub amount: u64, 
        pub delegate: Option<solana_sdk::pubkey::Pubkey>, 
        pub state: AccountState, 
        pub is_native: Option<u64>, 
        pub delegated_amount: u64, 
        pub close_authority: Option<solana_sdk::pubkey::Pubkey>, 
}