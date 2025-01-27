

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x1f")]
pub struct CreateNativeMint{
}

pub struct CreateNativeMintInstructionAccounts {
    pub funding_account: solana_sdk::pubkey::Pubkey,
    pub mint: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CreateNativeMint {
    type ArrangedAccounts = CreateNativeMintInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let funding_account = accounts.get(0)?;
        let mint = accounts.get(1)?;
        let funding_account = accounts.get(2)?;

        Some(CreateNativeMintInstructionAccounts {
            funding_account: funding_account.pubkey,
            mint: mint.pubkey,
        })
    }
}