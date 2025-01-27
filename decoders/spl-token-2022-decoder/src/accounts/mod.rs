 
use carbon_core::account::AccountDecoder; 
use carbon_core::deserialize::CarbonDeserialize;
 

use super::SplToken2022Decoder; 
pub mod mint_account; 
pub mod token_account; 
pub mod multisig_account; 

pub enum SplToken2022Account { 
        MintAccount(mint_account::MintAccount), 
        TokenAccount(token_account::TokenAccount), 
        MultisigAccount(multisig_account::MultisigAccount), 
}


impl<'a> AccountDecoder<'a> for SplToken2022Decoder { 
    type AccountType = SplToken2022Account;
     fn decode_account( &self, account: &solana_sdk::account::Account, ) -> Option<carbon_core::account::DecodedAccount<Self::AccountType>> { 
         
            if let Some(decoded_account) = mint_account::MintAccount::deserialize(account.data.as_slice()) { 
            return Some(carbon_core::account::DecodedAccount { 
                lamports: account.lamports, 
                data: SplToken2022Account::MintAccount(decoded_account), 
                owner: account.owner, 
                executable: account.executable, 
                rent_epoch: account.rent_epoch, 
            }); 
        } 
         
            if let Some(decoded_account) = token_account::TokenAccount::deserialize(account.data.as_slice()) { 
            return Some(carbon_core::account::DecodedAccount { 
                lamports: account.lamports, 
                data: SplToken2022Account::TokenAccount(decoded_account), 
                owner: account.owner, 
                executable: account.executable, 
                rent_epoch: account.rent_epoch, 
            }); 
        } 
         
            if let Some(decoded_account) = multisig_account::MultisigAccount::deserialize(account.data.as_slice()) { 
            return Some(carbon_core::account::DecodedAccount { 
                lamports: account.lamports, 
                data: SplToken2022Account::MultisigAccount(decoded_account), 
                owner: account.owner, 
                executable: account.executable, 
                rent_epoch: account.rent_epoch, 
            }); 
        } 
         
    None 
    } 
}