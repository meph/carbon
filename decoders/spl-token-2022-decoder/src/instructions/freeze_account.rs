

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x0a")]
pub struct FreezeAccount{
}

pub struct FreezeAccountInstructionAccounts {
    pub freeze_account: solana_sdk::pubkey::Pubkey,
    pub mint: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub signers: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for FreezeAccount {
    type ArrangedAccounts = FreezeAccountInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let freeze_account = accounts.get(0)?;
        let mint = accounts.get(1)?;
        let owner = accounts.get(2)?;
        let signers = accounts.get(3)?;

        Some(FreezeAccountInstructionAccounts {
            freeze_account: freeze_account.pubkey,
            mint: mint.pubkey,
            owner: owner.pubkey,
            signers: signers.pubkey,
        })
    }
}