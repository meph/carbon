

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x0e")]
pub struct MintToChecked{
    pub amount: u64,
    pub decimals: u8,
}

pub struct MintToCheckedInstructionAccounts {
    pub token_mint: solana_sdk::pubkey::Pubkey,
    pub mint_to: solana_sdk::pubkey::Pubkey,
    pub mint_authority: solana_sdk::pubkey::Pubkey,
    pub signers: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for MintToChecked {
    type ArrangedAccounts = MintToCheckedInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let token_mint = accounts.get(0)?;
        let mint_to = accounts.get(1)?;
        let mint_authority = accounts.get(2)?;
        let signers = accounts.get(3)?;

        Some(MintToCheckedInstructionAccounts {
            token_mint: token_mint.pubkey,
            mint_to: mint_to.pubkey,
            mint_authority: mint_authority.pubkey,
            signers: signers.pubkey,
        })
    }
}