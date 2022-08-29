use hex_literal::hex;
use hmac::{Hmac, Mac};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

#[tokio::test(flavor = "multi_thread")]
async fn hmac() {
    let mut mac = HmacSha256::new_from_slice(b"my secret and secure key")
        .expect("HMAC can take key of any size");

    mac.update(b"input message");

    let result = mac.finalize();

    let code_bytes = result.into_bytes();

    println!("result1: {:?}", code_bytes);

    mac.update(b"power");

    let code_bytes = result.into_bytes();

    println!("result2: {:?}", code_bytes);
}
