

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x18")]
pub struct UiAmountToAmount{
    pub ui_amount: String,
}

pub struct UiAmountToAmountInstructionAccounts {
    pub mint: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UiAmountToAmount {
    type ArrangedAccounts = UiAmountToAmountInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let mint = accounts.get(0)?;

        Some(UiAmountToAmountInstructionAccounts {
            mint: mint.pubkey,
        })
    }
}