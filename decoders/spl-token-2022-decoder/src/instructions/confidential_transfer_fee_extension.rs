

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x25")]
pub struct ConfidentialTransferFeeExtension{
}

pub struct ConfidentialTransferFeeExtensionInstructionAccounts {
}

impl carbon_core::deserialize::ArrangeAccounts for ConfidentialTransferFeeExtension {
    type ArrangedAccounts = ConfidentialTransferFeeExtensionInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {

        Some(ConfidentialTransferFeeExtensionInstructionAccounts {
        })
    }
}