

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x03")]
pub struct Transfer{
    pub amount: u64,
}

pub struct TransferInstructionAccounts {
    pub source: solana_sdk::pubkey::Pubkey,
    pub destination: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
    pub signers: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Transfer {
    type ArrangedAccounts = TransferInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let source = accounts.get(0)?;
        let destination = accounts.get(1)?;
        let authority = accounts.get(2)?;
        let signers = accounts.get(3)?;

        Some(TransferInstructionAccounts {
            source: source.pubkey,
            destination: destination.pubkey,
            authority: authority.pubkey,
            signers: signers.pubkey,
        })
    }
}