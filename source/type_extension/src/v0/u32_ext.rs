use crate::TypeExtError;
use std::convert::TryInto;

pub fn convert_vec_into_u8_32(v: Vec<u8>) -> Result<[u8; 32], TypeExtError> {
    let arr: [u8; 32] = match v.try_into() {
        Ok(a) => a,
        Err(err) => {
            return Err(format!(
                "Cannot convert Vec<u8> into an array [u8; 32], err: {:?}",
                err,
            )
            .into())
        }
    };

    Ok(arr)
}
