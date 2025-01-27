

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x00")]
pub struct InitializeMint{
    pub decimals: u8,
    pub mint_authority: solana_sdk::pubkey::Pubkey,
    pub freeze_authority: Option<solana_sdk::pubkey::Pubkey>,
}

pub struct InitializeMintInstructionAccounts {
    pub mint: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializeMint {
    type ArrangedAccounts = InitializeMintInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let mint = accounts.get(0)?;
        let rent = accounts.get(1)?;

        Some(InitializeMintInstructionAccounts {
            mint: mint.pubkey,
            rent: rent.pubkey,
        })
    }
}