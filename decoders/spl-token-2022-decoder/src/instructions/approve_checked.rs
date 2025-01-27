

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x0d")]
pub struct ApproveChecked{
    pub amount: u64,
    pub decimals: u8,
}

pub struct ApproveCheckedInstructionAccounts {
    pub source: solana_sdk::pubkey::Pubkey,
    pub token_mint: solana_sdk::pubkey::Pubkey,
    pub delegate: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub signers: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ApproveChecked {
    type ArrangedAccounts = ApproveCheckedInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let source = accounts.get(0)?;
        let token_mint = accounts.get(1)?;
        let delegate = accounts.get(2)?;
        let owner = accounts.get(3)?;
        let signers = accounts.get(4)?;

        Some(ApproveCheckedInstructionAccounts {
            source: source.pubkey,
            token_mint: token_mint.pubkey,
            delegate: delegate.pubkey,
            owner: owner.pubkey,
            signers: signers.pubkey,
        })
    }
}