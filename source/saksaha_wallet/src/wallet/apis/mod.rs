mod apis;
mod coin;
mod user;

use crate::WalletError;
pub(crate) use apis::*;

pub(crate) async fn decode_hex_string_to_u64(
    val: &String,
) -> Result<u64, WalletError> {
    let v = val.trim_start_matches("0x");

    let lsb = &v[0..16];

    let v = u64::from_str_radix(lsb, 16)?;

    Ok(v)
}
