
use super::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone)]
pub enum ExtensionTypeWithFields {
    Uninitialized,
    TransferFeeConfig
                {
                    extension_length: u16,
                    transfer_fee_config_authority: solana_sdk::pubkey::Pubkey,
                    withdraw_withheld_authority: solana_sdk::pubkey::Pubkey,
                    withheld_amount: u64,
                    older_transfer_fee: TransferFee,
                    newer_transfer_fee: TransferFee,
                }
    ,
    TransferFeeAmount
                {
                    extension_length: u16,
                    withheld_amount: u64,
                }
    ,
    MintCloseAuthority
                {
                    extension_length: u16,
                    close_authority: solana_sdk::pubkey::Pubkey,
                }
    ,
    ConfidentialTransferMint
                {
                    extension_length: u16,
                    authority: solana_sdk::pubkey::Pubkey,
                    auto_approve_new_accounts: bool,
                    auditor_elgamal_pubkey: solana_sdk::pubkey::Pubkey,
                }
    ,
    ConfidentialTransferAccount
                {
                    extension_length: u16,
                    approved: bool,
                    elgamal_pubkey: solana_sdk::pubkey::Pubkey,
                    #[serde(with = "BigArray")]
                    pending_balance_lo: [u8; 64],
                    #[serde(with = "BigArray")]
                    pending_balance_hi: [u8; 64],
                    #[serde(with = "BigArray")]
                    available_balance: [u8; 64],
                    #[serde(with = "BigArray")]
                    decryptable_available_balance: [u8; 36],
                    allow_confidential_credits: bool,
                    allow_non_confidential_credits: bool,
                    pending_balance_credit_counter: u64,
                    maximum_pending_balance_credit_counter: u64,
                    expected_pending_balance_credit_counter: u64,
                    actual_pending_balance_credit_counter: u64,
                }
    ,
    DefaultAccountState
                {
                    extension_length: u16,
                    default_account_state: AccountState,
                }
    ,
    ImmutableOwner
                {
                    extension_length: u16,
                }
    ,
    MemoTransfer
                {
                    extension_length: u16,
                    require_incoming_transfer_memos: bool,
                }
    ,
    NonTransferable
                {
                    extension_length: u16,
                }
    ,
    InterestBearingConfig
                {
                    extension_length: u16,
                    rate_authority: solana_sdk::pubkey::Pubkey,
                    initialization_timestamp: i64,
                    pre_update_average_rate: i16,
                    last_update_timestamp: i64,
                    current_rate: i16,
                }
    ,
    CpiGuard
                {
                    extension_length: u16,
                    lock_cpi: bool,
                }
    ,
    PermanentDelegate
                {
                    extension_length: u16,
                    delegate: solana_sdk::pubkey::Pubkey,
                }
    ,
    NonTransferableAccount
                {
                    extension_length: u16,
                }
    ,
    TransferHook
                {
                    extension_length: u16,
                    authority: solana_sdk::pubkey::Pubkey,
                    program_id: solana_sdk::pubkey::Pubkey,
                }
    ,
    TransferHookAccount
                {
                    extension_length: u16,
                    transferring: bool,
                }
    ,
    ConfidentialTransferFeeConfig
                {
                    extension_length: u16,
                    authority: solana_sdk::pubkey::Pubkey,
                    withdraw_withheld_authority_elgamal_pubkey: solana_sdk::pubkey::Pubkey,
                    harvest_to_mint_enabled: bool,
                    #[serde(with = "BigArray")]
                    withheld_amount: [u8; 64],
                }
    ,
    ConfidentialTransferFeeAmount
                {
                    extension_length: u16,
                    #[serde(with = "BigArray")]
                    withheld_amount: [u8; 64],
                }
    ,
    MetadataPointer
                {
                    extension_length: u16,
                    authority: solana_sdk::pubkey::Pubkey,
                    metadata_address: solana_sdk::pubkey::Pubkey,
                }
    ,
    TokenMetadata
                {
                    extension_length: u16,
                    update_authority: solana_sdk::pubkey::Pubkey,
                    mint: solana_sdk::pubkey::Pubkey,
                    name: String,
                    symbol: String,
                    uri: String,
                    additional_metadata: std::collections::HashMap<String, String>,
                }
    ,
    GroupPointer
                {
                    extension_length: u16,
                    authority: solana_sdk::pubkey::Pubkey,
                    group_address: solana_sdk::pubkey::Pubkey,
                }
    ,
    TokenGroup
                {
                    extension_length: u16,
                    update_authority: solana_sdk::pubkey::Pubkey,
                    mint: solana_sdk::pubkey::Pubkey,
                    size: u32,
                    max_size: u32,
                }
    ,
    GroupMemberPointer
                {
                    extension_length: u16,
                    authority: solana_sdk::pubkey::Pubkey,
                    member_address: solana_sdk::pubkey::Pubkey,
                }
    ,
    TokenGroupMember
                {
                    extension_length: u16,
                    mint: solana_sdk::pubkey::Pubkey,
                    group: solana_sdk::pubkey::Pubkey,
                    member_number: u32,
                }
    ,
}


