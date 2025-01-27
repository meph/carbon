

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x1a")]
pub struct TransferFeeExtension{
}

pub struct TransferFeeExtensionInstructionAccounts {
}

impl carbon_core::deserialize::ArrangeAccounts for TransferFeeExtension {
    type ArrangedAccounts = TransferFeeExtensionInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {

        Some(TransferFeeExtensionInstructionAccounts {
        })
    }
}