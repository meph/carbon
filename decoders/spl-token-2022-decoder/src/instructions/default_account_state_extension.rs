

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x1c")]
pub struct DefaultAccountStateExtension{
}

pub struct DefaultAccountStateExtensionInstructionAccounts {
}

impl carbon_core::deserialize::ArrangeAccounts for DefaultAccountStateExtension {
    type ArrangedAccounts = DefaultAccountStateExtensionInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {

        Some(DefaultAccountStateExtensionInstructionAccounts {
        })
    }
}