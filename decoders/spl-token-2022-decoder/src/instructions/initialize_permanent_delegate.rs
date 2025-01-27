

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x23")]
pub struct InitializePermanentDelegate{
    pub delegate: solana_sdk::pubkey::Pubkey,
}

pub struct InitializePermanentDelegateInstructionAccounts {
    pub mint: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializePermanentDelegate {
    type ArrangedAccounts = InitializePermanentDelegateInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let mint = accounts.get(0)?;

        Some(InitializePermanentDelegateInstructionAccounts {
            mint: mint.pubkey,
        })
    }
}