

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x22")]
pub struct CpiGuardExtension{
}

pub struct CpiGuardExtensionInstructionAccounts {
}

impl carbon_core::deserialize::ArrangeAccounts for CpiGuardExtension {
    type ArrangedAccounts = CpiGuardExtensionInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {

        Some(CpiGuardExtensionInstructionAccounts {
        })
    }
}