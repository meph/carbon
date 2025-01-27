

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x1b")]
pub struct ConfidentialTransferExtension{
}

pub struct ConfidentialTransferExtensionInstructionAccounts {
}

impl carbon_core::deserialize::ArrangeAccounts for ConfidentialTransferExtension {
    type ArrangedAccounts = ConfidentialTransferExtensionInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {

        Some(ConfidentialTransferExtensionInstructionAccounts {
        })
    }
}