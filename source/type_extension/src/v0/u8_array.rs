use crate::TypeExtError;
use std::convert::TryInto;

pub struct U8Array;

impl U8Array {
    pub fn new_empty_32() -> [u8; 32] {
        [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0,
        ]
    }

    pub fn from_int(v: u64) -> [u8; 32] {
        let arr: [u8; 8] = v.to_le_bytes();

        let mut ret = [0u8; 32];

        let _ = &ret[24..].copy_from_slice(&arr);

        ret
    }

    pub fn from_hex_string(str: String) -> Result<[u8; 32], TypeExtError> {
        let mut tmp = str.clone();

        if str.len() > 64 {
            return Err(format!("The length of hex string should be less than 65").into());
        }

        for _ in 0..(64 - str.len()) {
            tmp = format!("0{}", str);
        }

        let tmp = tmp.as_bytes().to_vec();

        let mut res: [u8; 32] = [0; 32];

        for idx in 0..32 {
            res[idx] = tmp[idx];
        }

        Ok([0; 32])
    }
}
