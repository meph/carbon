

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x07")]
pub struct MintTo{
    pub amount: u64,
}

pub struct MintToInstructionAccounts {
    pub mint: solana_sdk::pubkey::Pubkey,
    pub mint_to: solana_sdk::pubkey::Pubkey,
    pub mint_authority: solana_sdk::pubkey::Pubkey,
    pub signers: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for MintTo {
    type ArrangedAccounts = MintToInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let mint = accounts.get(0)?;
        let mint_to = accounts.get(1)?;
        let mint_authority = accounts.get(2)?;
        let signers = accounts.get(3)?;

        Some(MintToInstructionAccounts {
            mint: mint.pubkey,
            mint_to: mint_to.pubkey,
            mint_authority: mint_authority.pubkey,
            signers: signers.pubkey,
        })
    }
}