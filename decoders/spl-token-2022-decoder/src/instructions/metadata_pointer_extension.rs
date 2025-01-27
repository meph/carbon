

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x27")]
pub struct MetadataPointerExtension{
}

pub struct MetadataPointerExtensionInstructionAccounts {
}

impl carbon_core::deserialize::ArrangeAccounts for MetadataPointerExtension {
    type ArrangedAccounts = MetadataPointerExtensionInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {

        Some(MetadataPointerExtensionInstructionAccounts {
        })
    }
}