

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x13")]
pub struct InitializeMultisig2{
    pub no_of_signers_required: u8,
}

pub struct InitializeMultisig2InstructionAccounts {
    pub initialize_account: solana_sdk::pubkey::Pubkey,
    pub signer1: solana_sdk::pubkey::Pubkey,
    pub signer2: solana_sdk::pubkey::Pubkey,
    pub signer3: solana_sdk::pubkey::Pubkey,
    pub signer4: solana_sdk::pubkey::Pubkey,
    pub signer5: solana_sdk::pubkey::Pubkey,
    pub signer6: solana_sdk::pubkey::Pubkey,
    pub signer7: solana_sdk::pubkey::Pubkey,
    pub signer8: solana_sdk::pubkey::Pubkey,
    pub signer9: solana_sdk::pubkey::Pubkey,
    pub signer10: solana_sdk::pubkey::Pubkey,
    pub signer11: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializeMultisig2 {
    type ArrangedAccounts = InitializeMultisig2InstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let initialize_account = accounts.get(0)?;
        let signer1 = accounts.get(1)?;
        let signer2 = accounts.get(2)?;
        let signer3 = accounts.get(3)?;
        let signer4 = accounts.get(4)?;
        let signer5 = accounts.get(5)?;
        let signer6 = accounts.get(6)?;
        let signer7 = accounts.get(7)?;
        let signer8 = accounts.get(8)?;
        let signer9 = accounts.get(9)?;
        let signer10 = accounts.get(10)?;
        let signer11 = accounts.get(11)?;

        Some(InitializeMultisig2InstructionAccounts {
            initialize_account: initialize_account.pubkey,
            signer1: signer1.pubkey,
            signer2: signer2.pubkey,
            signer3: signer3.pubkey,
            signer4: signer4.pubkey,
            signer5: signer5.pubkey,
            signer6: signer6.pubkey,
            signer7: signer7.pubkey,
            signer8: signer8.pubkey,
            signer9: signer9.pubkey,
            signer10: signer10.pubkey,
            signer11: signer11.pubkey,
        })
    }
}