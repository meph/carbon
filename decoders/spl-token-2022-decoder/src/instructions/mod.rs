



use super::SplToken2022Decoder;
pub mod initialize_mint;
pub mod initialize_account;
pub mod initialize_multisig;
pub mod transfer;
pub mod approve;
pub mod revoke;
pub mod set_authority;
pub mod mint_to;
pub mod burn;
pub mod close_account;
pub mod freeze_account;
pub mod thaw_account;
pub mod transfer_checked;
pub mod approve_checked;
pub mod mint_to_checked;
pub mod burn_checked;
pub mod initialize_account2;
pub mod sync_native;
pub mod initialize_account3;
pub mod initialize_multisig2;
pub mod initialize_mint2;
pub mod get_account_data_size;
pub mod initialize_immutable_owner;
pub mod amount_to_ui_amount;
pub mod ui_amount_to_amount;
pub mod initialize_mint_close_authority;
pub mod transfer_fee_extension;
pub mod confidential_transfer_extension;
pub mod default_account_state_extension;
pub mod reallocate;
pub mod memo_transfer_extension;
pub mod create_native_mint;
pub mod initialize_non_transferable_mint;
pub mod interest_bearing_mint_extension;
pub mod cpi_guard_extension;
pub mod initialize_permanent_delegate;
pub mod transfer_hook_extension;
pub mod confidential_transfer_fee_extension;
pub mod withdraw_excess_lamports;
pub mod metadata_pointer_extension;
pub mod group_pointer_extension;
pub mod group_member_pointer_extension;

#[derive(carbon_core::InstructionType, serde::Serialize, serde::Deserialize, PartialEq, Eq, Debug, Clone, Hash)]
pub enum SplToken2022Instruction {
    InitializeMint(initialize_mint::InitializeMint),
    InitializeAccount(initialize_account::InitializeAccount),
    InitializeMultisig(initialize_multisig::InitializeMultisig),
    Transfer(transfer::Transfer),
    Approve(approve::Approve),
    Revoke(revoke::Revoke),
    SetAuthority(set_authority::SetAuthority),
    MintTo(mint_to::MintTo),
    Burn(burn::Burn),
    CloseAccount(close_account::CloseAccount),
    FreezeAccount(freeze_account::FreezeAccount),
    ThawAccount(thaw_account::ThawAccount),
    TransferChecked(transfer_checked::TransferChecked),
    ApproveChecked(approve_checked::ApproveChecked),
    MintToChecked(mint_to_checked::MintToChecked),
    BurnChecked(burn_checked::BurnChecked),
    InitializeAccount2(initialize_account2::InitializeAccount2),
    SyncNative(sync_native::SyncNative),
    InitializeAccount3(initialize_account3::InitializeAccount3),
    InitializeMultisig2(initialize_multisig2::InitializeMultisig2),
    InitializeMint2(initialize_mint2::InitializeMint2),
    GetAccountDataSize(get_account_data_size::GetAccountDataSize),
    InitializeImmutableOwner(initialize_immutable_owner::InitializeImmutableOwner),
    AmountToUiAmount(amount_to_ui_amount::AmountToUiAmount),
    UiAmountToAmount(ui_amount_to_amount::UiAmountToAmount),
    InitializeMintCloseAuthority(initialize_mint_close_authority::InitializeMintCloseAuthority),
    TransferFeeExtension(transfer_fee_extension::TransferFeeExtension),
    ConfidentialTransferExtension(confidential_transfer_extension::ConfidentialTransferExtension),
    DefaultAccountStateExtension(default_account_state_extension::DefaultAccountStateExtension),
    Reallocate(reallocate::Reallocate),
    MemoTransferExtension(memo_transfer_extension::MemoTransferExtension),
    CreateNativeMint(create_native_mint::CreateNativeMint),
    InitializeNonTransferableMint(initialize_non_transferable_mint::InitializeNonTransferableMint),
    InterestBearingMintExtension(interest_bearing_mint_extension::InterestBearingMintExtension),
    CpiGuardExtension(cpi_guard_extension::CpiGuardExtension),
    InitializePermanentDelegate(initialize_permanent_delegate::InitializePermanentDelegate),
    TransferHookExtension(transfer_hook_extension::TransferHookExtension),
    ConfidentialTransferFeeExtension(confidential_transfer_fee_extension::ConfidentialTransferFeeExtension),
    WithdrawExcessLamports(withdraw_excess_lamports::WithdrawExcessLamports),
    MetadataPointerExtension(metadata_pointer_extension::MetadataPointerExtension),
    GroupPointerExtension(group_pointer_extension::GroupPointerExtension),
    GroupMemberPointerExtension(group_member_pointer_extension::GroupMemberPointerExtension),
}

impl<'a> carbon_core::instruction::InstructionDecoder<'a> for SplToken2022Decoder {
    type InstructionType = SplToken2022Instruction;

    fn decode_instruction(
        &self,
        instruction: &solana_sdk::instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        carbon_core::try_decode_instructions!(instruction,
            SplToken2022Instruction::InitializeMint => initialize_mint::InitializeMint,
            SplToken2022Instruction::InitializeAccount => initialize_account::InitializeAccount,
            SplToken2022Instruction::InitializeMultisig => initialize_multisig::InitializeMultisig,
            SplToken2022Instruction::Transfer => transfer::Transfer,
            SplToken2022Instruction::Approve => approve::Approve,
            SplToken2022Instruction::Revoke => revoke::Revoke,
            SplToken2022Instruction::SetAuthority => set_authority::SetAuthority,
            SplToken2022Instruction::MintTo => mint_to::MintTo,
            SplToken2022Instruction::Burn => burn::Burn,
            SplToken2022Instruction::CloseAccount => close_account::CloseAccount,
            SplToken2022Instruction::FreezeAccount => freeze_account::FreezeAccount,
            SplToken2022Instruction::ThawAccount => thaw_account::ThawAccount,
            SplToken2022Instruction::TransferChecked => transfer_checked::TransferChecked,
            SplToken2022Instruction::ApproveChecked => approve_checked::ApproveChecked,
            SplToken2022Instruction::MintToChecked => mint_to_checked::MintToChecked,
            SplToken2022Instruction::BurnChecked => burn_checked::BurnChecked,
            SplToken2022Instruction::InitializeAccount2 => initialize_account2::InitializeAccount2,
            SplToken2022Instruction::SyncNative => sync_native::SyncNative,
            SplToken2022Instruction::InitializeAccount3 => initialize_account3::InitializeAccount3,
            SplToken2022Instruction::InitializeMultisig2 => initialize_multisig2::InitializeMultisig2,
            SplToken2022Instruction::InitializeMint2 => initialize_mint2::InitializeMint2,
            SplToken2022Instruction::GetAccountDataSize => get_account_data_size::GetAccountDataSize,
            SplToken2022Instruction::InitializeImmutableOwner => initialize_immutable_owner::InitializeImmutableOwner,
            SplToken2022Instruction::AmountToUiAmount => amount_to_ui_amount::AmountToUiAmount,
            SplToken2022Instruction::UiAmountToAmount => ui_amount_to_amount::UiAmountToAmount,
            SplToken2022Instruction::InitializeMintCloseAuthority => initialize_mint_close_authority::InitializeMintCloseAuthority,
            SplToken2022Instruction::TransferFeeExtension => transfer_fee_extension::TransferFeeExtension,
            SplToken2022Instruction::ConfidentialTransferExtension => confidential_transfer_extension::ConfidentialTransferExtension,
            SplToken2022Instruction::DefaultAccountStateExtension => default_account_state_extension::DefaultAccountStateExtension,
            SplToken2022Instruction::Reallocate => reallocate::Reallocate,
            SplToken2022Instruction::MemoTransferExtension => memo_transfer_extension::MemoTransferExtension,
            SplToken2022Instruction::CreateNativeMint => create_native_mint::CreateNativeMint,
            SplToken2022Instruction::InitializeNonTransferableMint => initialize_non_transferable_mint::InitializeNonTransferableMint,
            SplToken2022Instruction::InterestBearingMintExtension => interest_bearing_mint_extension::InterestBearingMintExtension,
            SplToken2022Instruction::CpiGuardExtension => cpi_guard_extension::CpiGuardExtension,
            SplToken2022Instruction::InitializePermanentDelegate => initialize_permanent_delegate::InitializePermanentDelegate,
            SplToken2022Instruction::TransferHookExtension => transfer_hook_extension::TransferHookExtension,
            SplToken2022Instruction::ConfidentialTransferFeeExtension => confidential_transfer_fee_extension::ConfidentialTransferFeeExtension,
            SplToken2022Instruction::WithdrawExcessLamports => withdraw_excess_lamports::WithdrawExcessLamports,
            SplToken2022Instruction::MetadataPointerExtension => metadata_pointer_extension::MetadataPointerExtension,
            SplToken2022Instruction::GroupPointerExtension => group_pointer_extension::GroupPointerExtension,
            SplToken2022Instruction::GroupMemberPointerExtension => group_member_pointer_extension::GroupMemberPointerExtension,
        )
    }
}