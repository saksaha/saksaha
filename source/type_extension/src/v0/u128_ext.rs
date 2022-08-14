use crate::TypeExtError;
use std::convert::TryInto;

const U128_BYTE_LEN: usize = 16;

pub fn convert_u8_slice_into_u128(arr: &[u8]) -> Result<u128, TypeExtError> {
    let padded_u8_bytes: [u8; 16] = {
        let len = arr.len();

        if len > U128_BYTE_LEN {
            return Err(format!(
                "Number is too large (>8bytes), num: {:?}",
                arr
            )
            .into());
        }

        let diff = U128_BYTE_LEN - len;

        let mut v = vec![0u8; diff];
        v.extend(arr);

        match v.try_into() {
            Ok(a) => a,
            Err(err) => {
                return Err(format!(
                    "Error padding zeros to val, err: {:?}",
                    err,
                )
                .into());
            }
        }
    };

    Ok(u128::from_be_bytes(padded_u8_bytes))
}

pub fn convert_vec_into_u8_32(v: Vec<u8>) -> Result<[u8; 32], TypeExtError> {
    let arr: [u8; 32] = match v.try_into() {
        Ok(a) => a,
        Err(err) => {
            return Err(format!(
                "Cannot convert cm into an array, vec: {:?}",
                err,
            )
            .into())
        }
    };

    Ok(arr)
}
