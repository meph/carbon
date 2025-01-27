

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x05")]
pub struct Revoke{
}

pub struct RevokeInstructionAccounts {
    pub source: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub signers: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Revoke {
    type ArrangedAccounts = RevokeInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let source = accounts.get(0)?;
        let owner = accounts.get(1)?;
        let signers = accounts.get(2)?;

        Some(RevokeInstructionAccounts {
            source: source.pubkey,
            owner: owner.pubkey,
            signers: signers.pubkey,
        })
    }
}