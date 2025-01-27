

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x16")]
pub struct InitializeImmutableOwner{
}

pub struct InitializeImmutableOwnerInstructionAccounts {
    pub initialize_account: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializeImmutableOwner {
    type ArrangedAccounts = InitializeImmutableOwnerInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let initialize_account = accounts.get(0)?;

        Some(InitializeImmutableOwnerInstructionAccounts {
            initialize_account: initialize_account.pubkey,
        })
    }
}