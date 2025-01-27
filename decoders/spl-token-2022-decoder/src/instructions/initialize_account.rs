

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x01")]
pub struct InitializeAccount{
}

pub struct InitializeAccountInstructionAccounts {
    pub account_to_initialize: solana_sdk::pubkey::Pubkey,
    pub mint: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializeAccount {
    type ArrangedAccounts = InitializeAccountInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let account_to_initialize = accounts.get(0)?;
        let mint = accounts.get(1)?;
        let owner = accounts.get(2)?;
        let rent = accounts.get(3)?;

        Some(InitializeAccountInstructionAccounts {
            account_to_initialize: account_to_initialize.pubkey,
            mint: mint.pubkey,
            owner: owner.pubkey,
            rent: rent.pubkey,
        })
    }
}