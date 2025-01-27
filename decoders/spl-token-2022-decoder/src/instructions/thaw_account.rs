

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x0b")]
pub struct ThawAccount{
}

pub struct ThawAccountInstructionAccounts {
    pub thaw_account: solana_sdk::pubkey::Pubkey,
    pub token_mint: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub signers: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ThawAccount {
    type ArrangedAccounts = ThawAccountInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let thaw_account = accounts.get(0)?;
        let token_mint = accounts.get(1)?;
        let owner = accounts.get(2)?;
        let signers = accounts.get(3)?;

        Some(ThawAccountInstructionAccounts {
            thaw_account: thaw_account.pubkey,
            token_mint: token_mint.pubkey,
            owner: owner.pubkey,
            signers: signers.pubkey,
        })
    }
}