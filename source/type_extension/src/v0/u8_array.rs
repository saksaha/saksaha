use std::convert::TryInto;

use crate::TypeExtError;

pub struct U8Array;

pub type U8Arr32 = [u8; 32];

impl U8Array {
    pub fn new_empty_32() -> U8Arr32 {
        [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ]
    }

    pub fn from_int(v: u64) -> U8Arr32 {
        let arr: [u8; 8] = v.to_le_bytes();

        let mut ret = [0u8; 32];

        let _ = &ret[24..].copy_from_slice(&arr);

        ret
    }

    pub fn from_hex_string(str: String) -> Result<U8Arr32, TypeExtError> {
        let mut tmp = str.clone();

        if str.len() > 64 {
            return Err(format!(
                "The length of hex string should be less than 65"
            )
            .into());
        }

        for _ in 0..(64 - str.len()) {
            tmp = format!("0{}", str);
        }

        let tmp = tmp.as_bytes().to_vec();

        let mut res: U8Arr32 = [0; 32];

        for idx in 0..32 {
            res[idx] = tmp[idx];
        }

        Ok([0; 32])
    }
}
