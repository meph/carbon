

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x1e")]
pub struct MemoTransferExtension{
}

pub struct MemoTransferExtensionInstructionAccounts {
}

impl carbon_core::deserialize::ArrangeAccounts for MemoTransferExtension {
    type ArrangedAccounts = MemoTransferExtensionInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {

        Some(MemoTransferExtensionInstructionAccounts {
        })
    }
}