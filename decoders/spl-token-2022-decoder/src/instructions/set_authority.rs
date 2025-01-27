
use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x06")]
pub struct SetAuthority{
    pub authority_type: AuthorityType,
    pub new_authority: Option<solana_sdk::pubkey::Pubkey>,
}

pub struct SetAuthorityInstructionAccounts {
    pub mint: solana_sdk::pubkey::Pubkey,
    pub current_authority: solana_sdk::pubkey::Pubkey,
    pub signers: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SetAuthority {
    type ArrangedAccounts = SetAuthorityInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let mint = accounts.get(0)?;
        let current_authority = accounts.get(1)?;
        let signers = accounts.get(2)?;

        Some(SetAuthorityInstructionAccounts {
            mint: mint.pubkey,
            current_authority: current_authority.pubkey,
            signers: signers.pubkey,
        })
    }
}