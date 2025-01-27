
use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x1d")]
pub struct Reallocate{
    pub extension_type: ExtensionTypeArray,
}

pub struct ReallocateInstructionAccounts {
    pub reallocated_account: solana_sdk::pubkey::Pubkey,
    pub payer: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub signers: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Reallocate {
    type ArrangedAccounts = ReallocateInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let reallocated_account = accounts.get(0)?;
        let payer = accounts.get(1)?;
        let system_program = accounts.get(2)?;
        let owner = accounts.get(3)?;
        let signers = accounts.get(4)?;

        Some(ReallocateInstructionAccounts {
            reallocated_account: reallocated_account.pubkey,
            payer: payer.pubkey,
            system_program: system_program.pubkey,
            owner: owner.pubkey,
            signers: signers.pubkey,
        })
    }
}