use crate::TrptError;
use bytes::Bytes;
use std::convert::TryInto;

pub(crate) fn convert_bytes_into_u8_32(
    b: Bytes,
) -> Result<[u8; 32], TrptError> {
    if b.len() != 32 {
        return Err(format!("cm has invalid length, len: {}", b.len()).into());
    }

    let ret: [u8; 32] = (&b[..]).try_into()?;

    Ok(ret)
}
