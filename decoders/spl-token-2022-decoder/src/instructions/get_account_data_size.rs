
use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x15")]
pub struct GetAccountDataSize{
    pub extension_type: ExtensionTypeArray,
}

pub struct GetAccountDataSizeInstructionAccounts {
    pub mint: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for GetAccountDataSize {
    type ArrangedAccounts = GetAccountDataSizeInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let mint = accounts.get(0)?;

        Some(GetAccountDataSizeInstructionAccounts {
            mint: mint.pubkey,
        })
    }
}