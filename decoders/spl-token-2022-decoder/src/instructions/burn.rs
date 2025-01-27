

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x08")]
pub struct Burn{
    pub amount: u64,
}

pub struct BurnInstructionAccounts {
    pub burn_account: solana_sdk::pubkey::Pubkey,
    pub mint: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub signers: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Burn {
    type ArrangedAccounts = BurnInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let burn_account = accounts.get(0)?;
        let mint = accounts.get(1)?;
        let owner = accounts.get(2)?;
        let signers = accounts.get(3)?;

        Some(BurnInstructionAccounts {
            burn_account: burn_account.pubkey,
            mint: mint.pubkey,
            owner: owner.pubkey,
            signers: signers.pubkey,
        })
    }
}