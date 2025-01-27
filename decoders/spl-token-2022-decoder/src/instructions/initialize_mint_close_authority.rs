

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x19")]
pub struct InitializeMintCloseAuthority{
    pub close_authority: Option<solana_sdk::pubkey::Pubkey>,
}

pub struct InitializeMintCloseAuthorityInstructionAccounts {
    pub mint: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializeMintCloseAuthority {
    type ArrangedAccounts = InitializeMintCloseAuthorityInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let mint = accounts.get(0)?;

        Some(InitializeMintCloseAuthorityInstructionAccounts {
            mint: mint.pubkey,
        })
    }
}