

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x09")]
pub struct CloseAccount{
}

pub struct CloseAccountInstructionAccounts {
    pub close_account: solana_sdk::pubkey::Pubkey,
    pub destination: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub signers: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CloseAccount {
    type ArrangedAccounts = CloseAccountInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let close_account = accounts.get(0)?;
        let destination = accounts.get(1)?;
        let owner = accounts.get(2)?;
        let signers = accounts.get(3)?;

        Some(CloseAccountInstructionAccounts {
            close_account: close_account.pubkey,
            destination: destination.pubkey,
            owner: owner.pubkey,
            signers: signers.pubkey,
        })
    }
}