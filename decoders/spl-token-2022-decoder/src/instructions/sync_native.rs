

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x11")]
pub struct SyncNative{
}

pub struct SyncNativeInstructionAccounts {
    pub native_token_account: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SyncNative {
    type ArrangedAccounts = SyncNativeInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let native_token_account = accounts.get(0)?;

        Some(SyncNativeInstructionAccounts {
            native_token_account: native_token_account.pubkey,
        })
    }
}