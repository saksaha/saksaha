use crate::TypeExtError;
use std::convert::TryInto;

const U128_BYTE_LEN: usize = 16;

pub fn convert_u8_slice_into_u128(arr: &[u8]) -> Result<u128, TypeExtError> {
    let padded_u8_bytes: [u8; 16] = {
        let len = arr.len();

        if len > U128_BYTE_LEN {
            return Err(format!("Number is too large (>8bytes), num: {:?}", arr).into());
        }

        let diff = U128_BYTE_LEN - len;

        let mut v = vec![0u8; diff];
        v.extend(arr);

        match v.try_into() {
            Ok(a) => a,
            Err(err) => {
                return Err(format!("Error padding zeros to val, err: {:?}", err,).into());
            }
        }
    };

    Ok(u128::from_be_bytes(padded_u8_bytes))
}

pub fn convert_u128_into_u8_slice(v: u128) -> Result<Box<[u8]>, TypeExtError> {
    let arr = v.to_be_bytes();
    let arr = Box::new(arr);

    Ok(arr)
}
