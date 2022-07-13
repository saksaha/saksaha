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
            return Err(format!("Fail to parse byte array into scalar").into());
        }
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
        println!("powe4444");

        ScalarExt::parse_arr(&arr)
    }
}
