use std::str::FromStr;

use error_stack::{FutureExt, IntoReport, Result, ResultExt};
use ethers::{
    abi::Address,
    types::{H160, RecoveryMessage, Signature},
};
use serde::Serialize;

use crate::error::Error;

pub fn recover_address(message: &String, signature: &str) -> Result<Address, Error> {
    let signature = Signature::from_str(signature).unwrap();

    signature
        .recover(RecoveryMessage::Data(message.as_bytes().to_vec()))
        .into_report()
        .change_context(Error::InvalidSignature)
}

pub fn verify_address<T>(address: &H160, message: &T, signature: &str) -> Result<bool, Error>
where
    T: Serialize,
{
    let message = serde_json::to_string(&message).unwrap();

    let recovered_address = recover_address(&message, signature)?;
    Ok(recovered_address.eq(address))
}
