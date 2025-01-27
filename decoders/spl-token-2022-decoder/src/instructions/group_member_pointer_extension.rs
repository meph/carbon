

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x29")]
pub struct GroupMemberPointerExtension{
}

pub struct GroupMemberPointerExtensionInstructionAccounts {
}

impl carbon_core::deserialize::ArrangeAccounts for GroupMemberPointerExtension {
    type ArrangedAccounts = GroupMemberPointerExtensionInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {

        Some(GroupMemberPointerExtensionInstructionAccounts {
        })
    }
}