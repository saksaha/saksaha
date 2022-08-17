use crate::IDError;

pub fn make_public_key_short(public_key: &String) -> Result<&str, IDError> {
    if public_key.len() > 6 {
        let k = &public_key[..6];

        return Ok(k);
    } else {
        return Err(format!(
            "Public key is too short, public key: {}",
            public_key,
        )
        .into());
    }
}