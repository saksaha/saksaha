use crate::tests::test_utils;
use sak_crypto::{decode_hex, PublicKey, SakKey, ScalarBytes, SecretKey, ToEncodedPoint};
use sak_logger::SakLogger;

#[test]
fn test_recover_secret_key_from_the_credential() {
    test_utils::init_test().unwrap();

    let (secret_key, _public_key) = SakKey::generate();

    println!("secret_key: {:?}", secret_key.to_bytes());

    let secret_key_str: String = sak_crypto::encode_hex(&secret_key.to_bytes() as &[u8]);

    println!("secret_key: {:?}", secret_key_str);

    let secret_key_array = decode_hex(&secret_key_str).unwrap();

    let recovered_secret_key = SecretKey::from_bytes(secret_key_array).unwrap();

    assert_eq!(secret_key.to_bytes(), recovered_secret_key.to_bytes());
}

#[test]
fn test_recover_public_key_from_the_credential() {
    test_utils::init_test().unwrap();

    let (_secret_key, public_key) = SakKey::generate();

    let public_key_array = public_key.to_encoded_point(false).as_bytes().to_owned();

    println!("public_key: {:?}", public_key_array);
    println!("public_key (len): {:?}", public_key_array.len());

    let recovered_public_key = PublicKey::from_sec1_bytes(&public_key_array).unwrap();

    assert_eq!(
        public_key.to_encoded_point(false).as_bytes(),
        recovered_public_key.to_encoded_point(false).as_bytes()
    )
}
