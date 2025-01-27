

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x17")]
pub struct AmountToUiAmount{
    pub amount: u64,
}

pub struct AmountToUiAmountInstructionAccounts {
    pub mint: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for AmountToUiAmount {
    type ArrangedAccounts = AmountToUiAmountInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let mint = accounts.get(0)?;

        Some(AmountToUiAmountInstructionAccounts {
            mint: mint.pubkey,
        })
    }
}