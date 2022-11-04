use crate::{CryptoError, Scalar};
use std::convert::TryInto;

pub struct ScalarExt;

impl ScalarExt {
    pub fn parse_arr(arr: &[u8; 32]) -> Result<Scalar, CryptoError> {
        let s = Scalar::from_bytes(arr);

        if s.is_some().into() {
            let ret = s.unwrap();
            return Ok(ret);
        } else {
            return Err(format!(" 111 Fail to parse byte array into scalar").into());
        }
    }

    pub fn parse_arr_wide(arr1: &[u8; 32], arr2: &[u8; 32]) -> Result<Scalar, CryptoError> {
        let ret = {
            let mut r: [u8; 64] = [0; 64];
            let (one, two) = r.split_at_mut(arr1.len());
            one.copy_from_slice(arr1);
            two.copy_from_slice(arr2);
            r
        };

        let s = Scalar::from_bytes_wide(&ret);

        Ok(s)
    }

    pub fn parse_vec(v: Vec<u8>) -> Result<Scalar, CryptoError> {
        let arr = {
            let ret: [u8; 32] = match v.try_into() {
                Ok(r) => r,
                Err(_) => {
                    return Err(format!(
                        "Could not convert sibling cm into \
                                        an array"
                    )
                    .into())
                }
            };

            ret
        };

        ScalarExt::parse_arr(&arr)
    }

    pub fn parse_string(s: String) -> Result<Scalar, CryptoError> {
        let sb = { s.as_bytes().to_vec() };

        ScalarExt::parse_vec(sb)
    }

    pub fn parse_u64(v: u64) -> Result<Scalar, CryptoError> {
        let bytes: [u8; 8] = v.to_le_bytes();
        let mut arr = [0u8; 32];
        let _ = &arr[24..].copy_from_slice(&bytes);

        let s = Scalar::from_bytes(&arr);

        if s.is_some().into() {
            let ret = s.unwrap();
            return Ok(ret);
        } else {
            return Err(format!("Fail to parse byte array into scalar").into());
        }
    }

    pub fn into_u64(s: Scalar) -> Result<u64, CryptoError> {
        let bytes = s.to_bytes();

        let arr: [u8; 8] = bytes[24..32].try_into()?;

        let v = u64::from_le_bytes(arr);

        Ok(v)
    }
}
