use std::{fmt::Display, str::FromStr};

use async_graphql::InputObject;
use ethers::types::{Address, Bytes, U256};
use hex::{self, FromHex, FromHexError};
use sea_orm::prelude::Decimal;
use service::vault::contract::{deposit_automator, withdraw_automator};

// TODO: Can we create Address type?
// TODO: Can we create Signature type?

#[derive(InputObject)]
pub struct ERC2612PermitInput {
    owner: String,
    token: String,
    value: Decimal,
    deadline: Decimal,
    v: String,
    r: String,
    s: String,
}

impl TryFrom<ERC2612PermitInput> for withdraw_automator::Erc2612Permit {
    type Error = String;

    fn try_from(value: ERC2612PermitInput) -> Result<Self, Self::Error> {
        Ok(withdraw_automator::Erc2612Permit {
            owner: value
                .owner
                .parse::<Address>()
                .map_err(|_| "Failed to parse owner address".to_string())?,
            token: value
                .token
                .parse::<Address>()
                .map_err(|_| "Failed to parse token address".to_string())?,
            value: decimal_to_u256(value.value)?,
            deadline: decimal_to_u256(value.deadline)?,
            v: decode_hex(value.v)
                .map_err(|_| "Failed to parse v".to_string())?
                .first()
                .cloned()
                .ok_or("Failed to parse v".to_string())?,
            r: decode_hex(value.r)
                .map_err(|_| "Failed to parse r".to_string())?
                .try_into()
                .map_err(|_| "Failed to parse r".to_string())?,
            s: decode_hex(value.s)
                .map_err(|_| "Failed to parse s".to_string())?
                .try_into()
                .map_err(|_| "Failed to parse s".to_string())?,
        })
    }
}

impl TryFrom<ERC2612PermitInput> for deposit_automator::Erc2612Permit {
    type Error = String;

    fn try_from(value: ERC2612PermitInput) -> Result<Self, Self::Error> {
        Ok(deposit_automator::Erc2612Permit {
            owner: value
                .owner
                .parse::<Address>()
                .map_err(|_| "Failed to parse owner address".to_string())?,
            token: value
                .token
                .parse::<Address>()
                .map_err(|_| "Failed to parse token address".to_string())?,
            value: decimal_to_u256(value.value)?,
            deadline: decimal_to_u256(value.deadline)?,
            v: decode_hex(value.v)
                .map_err(|_| "Failed to parse v".to_string())?
                .first()
                .cloned()
                .ok_or("Failed to parse v".to_string())?,
            r: decode_hex(value.r)
                .map_err(|_| "Failed to parse r".to_string())?
                .try_into()
                .map_err(|_| "Failed to parse r".to_string())?,
            s: decode_hex(value.s)
                .map_err(|_| "Failed to parse s".to_string())?
                .try_into()
                .map_err(|_| "Failed to parse s".to_string())?,
        })
    }
}

// Withdraw

#[derive(InputObject)]
pub struct RelayQueueWithdrawAndSyncInput {
    erc_2612_permit: ERC2612PermitInput,
    queue_withdraw_and_sync_params: RelayQueueWithdrawAndSyncStructInput,
    queue_withdraw_and_sync_permit: String,
}

#[derive(InputObject)]
pub struct RelayWithdrawInput {
    erc_2612_permit: ERC2612PermitInput,
    withdraw_params: RelayWithdrawStructInput,
    withdraw_permit: String,
}

#[derive(InputObject)]
pub struct RelayRemoveQueuedWithdrawInput {
    erc_2612_permit: ERC2612PermitInput,
    remove_queued_withdraw_params: RelayRemoveQueuedWithdrawStructInput,
    remove_queued_withdraw_permit: String,
}

#[derive(InputObject)]
pub struct RelayQueueWithdrawAndSyncStructInput {
    pub vault: String,
    pub token_id: Decimal,
    pub shares: Decimal,
    pub min_unit_price: Decimal,
    pub expiry: Decimal,
    pub lz_sync_fees: Vec<Decimal>,
    pub lz_withdraw_fees_total: Decimal,
    pub keeper_fee: Decimal,
    pub signer_nonce: Decimal,
}

#[derive(InputObject)]
pub struct RelayWithdrawStructInput {
    pub vault: String,
    pub token_id: Decimal,
    pub shares: Decimal,
    pub min_unit_price: Decimal,
    pub deadline: Decimal,
    pub lz_withdraw_fees: Vec<Decimal>,
    pub signer_nonce: Decimal,
}

#[derive(InputObject)]
pub struct RelayRemoveQueuedWithdrawStructInput {
    pub vault: String,
    pub index: Decimal,
    pub deadline: Decimal,
    pub signer_nonce: Decimal,
}

impl TryFrom<RelayQueueWithdrawAndSyncInput> for withdraw_automator::RelayQueueWithdrawAndSyncDTO {
    type Error = String;

    fn try_from(value: RelayQueueWithdrawAndSyncInput) -> Result<Self, Self::Error> {
        Ok(withdraw_automator::RelayQueueWithdrawAndSyncDTO {
            erc_2612_permit: value.erc_2612_permit.try_into()?,
            queue_withdraw_and_sync_params: value.queue_withdraw_and_sync_params.try_into()?,
            queue_withdraw_and_sync_permit: std::convert::Into::<Bytes>::into(
                decode_hex(value.queue_withdraw_and_sync_permit)
                    .map_err(|_| "Failed to convert queue_withdraw_and_sync_permit")?,
            ),
        })
    }
}

impl TryFrom<RelayWithdrawInput> for withdraw_automator::RelayWithdrawDTO {
    type Error = String;

    fn try_from(value: RelayWithdrawInput) -> Result<Self, Self::Error> {
        Ok(withdraw_automator::RelayWithdrawDTO {
            erc_2612_permit: value.erc_2612_permit.try_into()?,
            withdraw_params: value.withdraw_params.try_into()?,
            withdraw_permit: std::convert::Into::<Bytes>::into(
                decode_hex(value.withdraw_permit)
                    .map_err(|_| "Failed to convert withdraw_permit")?,
            ),
        })
    }
}

impl TryFrom<RelayRemoveQueuedWithdrawInput> for withdraw_automator::RelayRemoveQueuedWithdrawDTO {
    type Error = String;

    fn try_from(value: RelayRemoveQueuedWithdrawInput) -> Result<Self, Self::Error> {
        Ok(withdraw_automator::RelayRemoveQueuedWithdrawDTO {
            erc_2612_permit: value.erc_2612_permit.try_into()?,
            remove_queued_withdraw_params: value.remove_queued_withdraw_params.try_into()?,
            remove_queued_withdraw_permit: std::convert::Into::<Bytes>::into(
                decode_hex(value.remove_queued_withdraw_permit)
                    .map_err(|_| "Failed to parse remove_queued_withdraw_permit".to_string())?,
            ),
        })
    }
}

impl TryFrom<RelayQueueWithdrawAndSyncStructInput>
    for withdraw_automator::RelayQueueWithdrawAndSync
{
    type Error = String;

    fn try_from(value: RelayQueueWithdrawAndSyncStructInput) -> Result<Self, Self::Error> {
        Ok(withdraw_automator::RelayQueueWithdrawAndSync {
            vault: value
                .vault
                .parse::<Address>()
                .map_err(|_| "Failed to parse vault address".to_string())?,
            token_id: decimal_to_u256(value.token_id)?,
            shares: decimal_to_u256(value.shares)?,
            min_unit_price: decimal_to_u256(value.min_unit_price)?,
            expiry: decimal_to_u256(value.expiry)?,
            lz_sync_fees: value
                .lz_sync_fees
                .into_iter()
                .map(decimal_to_u256)
                .collect::<Result<Vec<_>, _>>()?,
            lz_withdraw_fees_total: decimal_to_u256(value.lz_withdraw_fees_total)?,
            keeper_fee: decimal_to_u256(value.keeper_fee)?,
            signer_nonce: decimal_to_u256(value.signer_nonce)?,
        })
    }
}

impl TryFrom<RelayWithdrawStructInput> for withdraw_automator::RelayWithdraw {
    type Error = String;

    fn try_from(value: RelayWithdrawStructInput) -> Result<Self, Self::Error> {
        Ok(withdraw_automator::RelayWithdraw {
            vault: value
                .vault
                .parse::<Address>()
                .map_err(|_| "Failed to parse vault address".to_string())?,
            token_id: decimal_to_u256(value.token_id)?,
            shares: decimal_to_u256(value.shares)?,
            min_unit_price: decimal_to_u256(value.min_unit_price)?,
            deadline: decimal_to_u256(value.deadline)?,
            lz_withdraw_fees: value
                .lz_withdraw_fees
                .into_iter()
                .map(decimal_to_u256)
                .collect::<Result<Vec<_>, _>>()?,
            signer_nonce: decimal_to_u256(value.signer_nonce)?,
        })
    }
}

impl TryFrom<RelayRemoveQueuedWithdrawStructInput>
    for withdraw_automator::RelayRemoveQueuedWithdraw
{
    type Error = String;

    fn try_from(value: RelayRemoveQueuedWithdrawStructInput) -> Result<Self, Self::Error> {
        Ok(withdraw_automator::RelayRemoveQueuedWithdraw {
            vault: value
                .vault
                .parse::<Address>()
                .map_err(|_| "Failed to parse vault address".to_string())?,
            index: decimal_to_u256(value.index)?,
            deadline: decimal_to_u256(value.deadline)?,
            signer_nonce: decimal_to_u256(value.signer_nonce)?,
        })
    }
}

// Deposit

#[derive(InputObject)]
pub struct RelayQueueDepositAndSyncInput {
    erc_2612_permit: ERC2612PermitInput,
    queue_deposit_and_sync_params: RelayQueueDepositAndSyncStructInput,
    queue_deposit_and_sync_permit: String,
}

#[derive(InputObject)]
pub struct RelayDepositInput {
    erc_2612_permit: ERC2612PermitInput,
    deposit_params: RelayDepositStructInput,
    deposit_permit: String,
}

#[derive(InputObject)]
pub struct RelayRemoveQueuedDepositInput {
    erc_2612_permit: ERC2612PermitInput,
    remove_queued_deposit_params: RelayRemoveQueuedDepositStructInput,
    remove_queued_deposit_permit: String,
}

#[derive(InputObject)]
pub struct RelayQueueDepositAndSyncStructInput {
    pub vault: String,
    pub token_id: Decimal,
    pub deposit_amount: Decimal,
    pub max_unit_price: Decimal,
    pub expiry: Decimal,
    pub lz_sync_fees: Vec<Decimal>,
    pub keeper_fee: Decimal,
    pub permit_amount_for_all_fees: Decimal,
    pub signer_nonce: Decimal,
    pub referrer: String,
}

#[derive(InputObject)]
pub struct RelayDepositStructInput {
    pub vault: String,
    pub token_id: Decimal,
    pub deposit_amount: Decimal,
    pub max_unit_price: Decimal,
    pub deadline: Decimal,
    pub permit_amount_for_all_fees: Decimal,
    pub signer_nonce: Decimal,
    pub referrer: String,
}

#[derive(InputObject)]
pub struct RelayRemoveQueuedDepositStructInput {
    pub vault: String,
    pub index: Decimal,
    pub deadline: Decimal,
    pub permit_amount_for_all_fees: Decimal,
    pub signer_nonce: Decimal,
}

impl TryFrom<RelayQueueDepositAndSyncInput> for deposit_automator::RelayQueueDepositAndSyncDTO {
    type Error = String;

    fn try_from(value: RelayQueueDepositAndSyncInput) -> Result<Self, Self::Error> {
        Ok(deposit_automator::RelayQueueDepositAndSyncDTO {
            erc_2612_permit: value.erc_2612_permit.try_into()?,
            queue_deposit_and_sync_params: value.queue_deposit_and_sync_params.try_into()?,
            queue_deposit_and_sync_permit: std::convert::Into::<Bytes>::into(
                decode_hex(value.queue_deposit_and_sync_permit)
                    .map_err(|_| "Failed to convert queue_deposit_and_sync_permit")?,
            ),
        })
    }
}

impl TryFrom<RelayDepositInput> for deposit_automator::RelayDepositDTO {
    type Error = String;

    fn try_from(value: RelayDepositInput) -> Result<Self, Self::Error> {
        Ok(deposit_automator::RelayDepositDTO {
            erc_2612_permit: value.erc_2612_permit.try_into()?,
            deposit_params: value.deposit_params.try_into()?,
            deposit_permit: std::convert::Into::<Bytes>::into(
                decode_hex(value.deposit_permit).map_err(|_| "Failed to convert deposit_permit")?,
            ),
        })
    }
}

impl TryFrom<RelayRemoveQueuedDepositInput> for deposit_automator::RelayRemoveQueuedDepositDTO {
    type Error = String;

    fn try_from(value: RelayRemoveQueuedDepositInput) -> Result<Self, Self::Error> {
        Ok(deposit_automator::RelayRemoveQueuedDepositDTO {
            erc_2612_permit: value.erc_2612_permit.try_into()?,
            remove_queued_deposit_params: value.remove_queued_deposit_params.try_into()?,
            remove_queued_deposit_permit: std::convert::Into::<Bytes>::into(
                decode_hex(value.remove_queued_deposit_permit)
                    .map_err(|_| "Failed to parse remove_queued_deposit_permit".to_string())?,
            ),
        })
    }
}

impl TryFrom<RelayQueueDepositAndSyncStructInput> for deposit_automator::RelayQueueDepositAndSync {
    type Error = String;

    fn try_from(value: RelayQueueDepositAndSyncStructInput) -> Result<Self, Self::Error> {
        Ok(deposit_automator::RelayQueueDepositAndSync {
            vault: value
                .vault
                .parse::<Address>()
                .map_err(|_| "Failed to parse vault address".to_string())?,
            token_id: decimal_to_u256(value.token_id)?,
            deposit_amount: decimal_to_u256(value.deposit_amount)?,
            max_unit_price: decimal_to_u256(value.max_unit_price)?,
            expiry: decimal_to_u256(value.expiry)?,
            lz_sync_fees: value
                .lz_sync_fees
                .into_iter()
                .map(decimal_to_u256)
                .collect::<Result<Vec<_>, _>>()?,
            keeper_fee: decimal_to_u256(value.keeper_fee)?,
            permit_amount_for_all_fees: decimal_to_u256(value.permit_amount_for_all_fees)?,
            signer_nonce: decimal_to_u256(value.signer_nonce)?,
            referrer: value
                .referrer
                .parse::<Address>()
                .map_err(|_| "Failed to parse referrer address".to_string())?,
        })
    }
}

impl TryFrom<RelayDepositStructInput> for deposit_automator::RelayDeposit {
    type Error = String;

    fn try_from(value: RelayDepositStructInput) -> Result<Self, Self::Error> {
        Ok(deposit_automator::RelayDeposit {
            vault: value
                .vault
                .parse::<Address>()
                .map_err(|_| "Failed to parse vault address".to_string())?,
            token_id: decimal_to_u256(value.token_id)?,
            deposit_amount: decimal_to_u256(value.deposit_amount)?,
            max_unit_price: decimal_to_u256(value.max_unit_price)?,
            deadline: decimal_to_u256(value.deadline)?,
            permit_amount_for_all_fees: decimal_to_u256(value.permit_amount_for_all_fees)?,
            signer_nonce: decimal_to_u256(value.signer_nonce)?,
            referrer: value
                .referrer
                .parse::<Address>()
                .map_err(|_| "Failed to parse referrer address".to_string())?,
        })
    }
}

impl TryFrom<RelayRemoveQueuedDepositStructInput> for deposit_automator::RelayRemoveQueuedDeposit {
    type Error = String;

    fn try_from(value: RelayRemoveQueuedDepositStructInput) -> Result<Self, Self::Error> {
        Ok(deposit_automator::RelayRemoveQueuedDeposit {
            vault: value
                .vault
                .parse::<Address>()
                .map_err(|_| "Failed to parse vault address".to_string())?,
            index: decimal_to_u256(value.index)?,
            deadline: decimal_to_u256(value.deadline)?,
            permit_amount_for_all_fees: decimal_to_u256(value.permit_amount_for_all_fees)?,
            signer_nonce: decimal_to_u256(value.signer_nonce)?,
        })
    }
}

// Helpers

/// Convert a Decimal to a U256.
fn decimal_to_u256(value: Decimal) -> Result<U256, String> {
    U256::from_str_radix(&value.to_string(), 10)
        .map_err(|_| "Failed to parse decimal to U256".to_string())
}

/// Decode a hex string into a byte vector.
fn decode_hex<T: AsRef<str> + Display>(value: T) -> Result<Vec<u8>, String> {
    let data = value.as_ref();
    let data = data.strip_prefix("0x").unwrap_or(data);

    FromHex::from_hex(data).map_err(|_| "Failed to parse hex".to_string())
}
