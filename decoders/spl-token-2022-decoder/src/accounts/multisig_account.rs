
 
use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)] 
 

#[carbon(discriminator = "0x4d09b4c7b7f69c51")] 
pub struct MultisigAccount { 
        pub num_of_signers_required: u8, 
        pub num_of_valid_signers_required: u8, 
        pub is_initialized: bool, 
        pub signers: [solana_sdk::pubkey::Pubkey; 11], 
}