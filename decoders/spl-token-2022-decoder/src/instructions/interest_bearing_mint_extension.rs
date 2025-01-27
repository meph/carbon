

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x21")]
pub struct InterestBearingMintExtension{
}

pub struct InterestBearingMintExtensionInstructionAccounts {
}

impl carbon_core::deserialize::ArrangeAccounts for InterestBearingMintExtension {
    type ArrangedAccounts = InterestBearingMintExtensionInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {

        Some(InterestBearingMintExtensionInstructionAccounts {
        })
    }
}