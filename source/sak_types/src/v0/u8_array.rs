use std::convert::TryInto;

use crate::TypesError;

pub struct U8Array;

impl U8Array {
    pub fn new_empty_32() -> [u8; 32] {
        [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ]
    }

    pub fn from_int(v: u64) -> Result<[u8; 32], TypesError> {
        let ret: [u8; 32] = match v.to_le_bytes() {
            Ok(v) => v,
            Err(err) => return Err(format!("aflwkej").into()),
        };

        Ok(ret)
    }
}
