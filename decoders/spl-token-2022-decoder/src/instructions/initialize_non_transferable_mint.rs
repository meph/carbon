

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x20")]
pub struct InitializeNonTransferableMint{
}

pub struct InitializeNonTransferableMintInstructionAccounts {
    pub mint: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializeNonTransferableMint {
    type ArrangedAccounts = InitializeNonTransferableMintInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let mint = accounts.get(0)?;

        Some(InitializeNonTransferableMintInstructionAccounts {
            mint: mint.pubkey,
        })
    }
}