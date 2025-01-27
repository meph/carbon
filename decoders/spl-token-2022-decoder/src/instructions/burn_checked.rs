

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x0f")]
pub struct BurnChecked{
    pub amount: u64,
    pub decimals: u8,
}

pub struct BurnCheckedInstructionAccounts {
    pub burn_account: solana_sdk::pubkey::Pubkey,
    pub token_mint: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub signers: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for BurnChecked {
    type ArrangedAccounts = BurnCheckedInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let burn_account = accounts.get(0)?;
        let token_mint = accounts.get(1)?;
        let owner = accounts.get(2)?;
        let signers = accounts.get(3)?;

        Some(BurnCheckedInstructionAccounts {
            burn_account: burn_account.pubkey,
            token_mint: token_mint.pubkey,
            owner: owner.pubkey,
            signers: signers.pubkey,
        })
    }
}