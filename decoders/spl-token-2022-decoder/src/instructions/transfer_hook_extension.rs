

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x24")]
pub struct TransferHookExtension{
}

pub struct TransferHookExtensionInstructionAccounts {
}

impl carbon_core::deserialize::ArrangeAccounts for TransferHookExtension {
    type ArrangedAccounts = TransferHookExtensionInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {

        Some(TransferHookExtensionInstructionAccounts {
        })
    }
}