

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x28")]
pub struct GroupPointerExtension{
}

pub struct GroupPointerExtensionInstructionAccounts {
}

impl carbon_core::deserialize::ArrangeAccounts for GroupPointerExtension {
    type ArrangedAccounts = GroupPointerExtensionInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {

        Some(GroupPointerExtensionInstructionAccounts {
        })
    }
}