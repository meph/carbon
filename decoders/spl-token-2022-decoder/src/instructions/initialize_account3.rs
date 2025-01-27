

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x12")]
pub struct InitializeAccount3{
    pub owner: solana_sdk::pubkey::Pubkey,
}

pub struct InitializeAccount3InstructionAccounts {
    pub initialize_account: solana_sdk::pubkey::Pubkey,
    pub associated_mint: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializeAccount3 {
    type ArrangedAccounts = InitializeAccount3InstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let initialize_account = accounts.get(0)?;
        let associated_mint = accounts.get(1)?;

        Some(InitializeAccount3InstructionAccounts {
            initialize_account: initialize_account.pubkey,
            associated_mint: associated_mint.pubkey,
        })
    }
}