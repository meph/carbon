
use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x15")]
pub struct CreateCollectionV2{
    pub create_collection_v2_args: CreateCollectionV2Args,
}

pub struct CreateCollectionV2InstructionAccounts {
    pub collection: solana_sdk::pubkey::Pubkey,
    pub update_authority: solana_sdk::pubkey::Pubkey,
    pub payer: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CreateCollectionV2 {
    type ArrangedAccounts = CreateCollectionV2InstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let collection = accounts.get(0)?;
        let update_authority = accounts.get(1)?;
        let payer = accounts.get(2)?;
        let system_program = accounts.get(3)?;

        Some(CreateCollectionV2InstructionAccounts {
            collection: collection.pubkey,
            update_authority: update_authority.pubkey,
            payer: payer.pubkey,
            system_program: system_program.pubkey,
        })
    }
}