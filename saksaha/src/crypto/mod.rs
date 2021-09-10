use k256::{EncodedPoint, PublicKey, SecretKey, ecdh::EphemeralSecret, ecdsa::{
        signature::{Signer, Verifier},
        Signature, SigningKey, VerifyingKey,
    }, elliptic_curve::sec1::ToEncodedPoint};
use rand_core::OsRng; // requires 'getrandom' feature
use std::{fmt::Write, num::ParseIntError};

pub fn decode_hex(s: &str) -> Result<Vec<u8>, ParseIntError> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
        .collect()
}

pub fn encode_hex(bytes: &[u8]) -> String {
    let mut s = String::with_capacity(bytes.len() * 2);
    for &b in bytes {
        write!(&mut s, "{:02x}", b).unwrap();
    }
    s
}

pub fn generate_key() -> SecretKey {
    let secret = SecretKey::random(&mut OsRng);
    return secret;
}

pub fn to_hex(secret: EphemeralSecret) {
    // let pk = secret.public_key();
    // secret.
    // EncodedPoint::from(secret);
    // let pk = EncodedPoint::from(secret.public_key());

}

#[test]
fn b() {
    // Signing
    let signing_key = SigningKey::random(&mut OsRng); // Serialize with `::to_bytes()`
    let message = b"ECDSA proves knowledge of a secret number in the context of a single message";

    // Note: the signature type must be annotated or otherwise inferrable as
    // `Signer` has many impls of the `Signer` trait (for both regular and
    // recoverable signature types).
    let signature: Signature = signing_key.sign(message);

    // Verification
    let verify_key = VerifyingKey::from(&signing_key); // Serialize with `::to_encoded_point()`
    assert!(verify_key.verify(message, &signature).is_ok());
}

#[test]
fn a() {
    // Alice
    let alice_secret = EphemeralSecret::random(&mut OsRng);
    let alice_pk_bytes = EncodedPoint::from(alice_secret.public_key());

    let sk = SecretKey::random(&mut OsRng);
    let sk_bytes = sk.to_bytes();

    print!("secret key: {:?}\n", sk);
    print!("secret key bytes: {:?}\n", alice_pk_bytes);

    let sk_rec = SecretKey::from_bytes(sk_bytes).unwrap();
    print!("recovered secret key {:?}\n", sk_rec);

    let public_key = sk.public_key();
    let enc_point = public_key.to_encoded_point(false);
    let enc_point_bytes = enc_point.as_bytes();

    print!("public key: {:?}\n", public_key);
    print!("encoded point: {:?}\n", enc_point);
    print!("encoded point as bytes: {:?}\n", enc_point_bytes);

    let public_key_rec = PublicKey::from_sec1_bytes(enc_point_bytes).unwrap();
    let enc_point_rec = public_key_rec.to_encoded_point(false);

    print!("public key rec: {:?}\n", public_key_rec);
    print!("encoded point rec: {:?}\n", &enc_point_rec);

    assert_eq!(enc_point, enc_point_rec);
    // PublicKey::fr

    // // Bob
    // let bob_secret = EphemeralSecret::random(&mut OsRng);
    // let bob_pk_bytes = EncodedPoint::from(bob_secret.public_key());

    // // Alice decodes Bob's serialized public key and computes a shared secret from it
    // let bob_public = PublicKey::from_sec1_bytes(bob_pk_bytes.as_ref())
    //     .expect("bob's public key is invalid!"); // In real usage, don't panic, handle this!

    // let alice_shared = alice_secret.diffie_hellman(&bob_public);
    // let a = alice_shared.as_bytes().as_slice();

    // let mut f = String::new();
    // for &b in a {
    //     write!(&mut f, "{:02x}", b).unwrap();
    // }

    // // print!("44 {:?}\n", alice_secret);

    // // c.join(" ");
    // // println!("5 {}", b);
    // // a.join("");

    // print!("31 {:x?}\n", alice_shared.as_bytes());
    // print!("33 {:x?}\n", a);

    // // for n in 0..a.

    // // Bob deocdes Alice's serialized public key and computes the same shared secret
    // let alice_public = PublicKey::from_sec1_bytes(alice_pk_bytes.as_ref())
    //     .expect("alice's public key is invalid!"); // In real usage, don't panic, handle this!

    // let bob_shared = bob_secret.diffie_hellman(&alice_public);

    // print!("44 {:?}\n", bob_shared.as_bytes());

    // // Both participants arrive on the same shared secret
    // assert_eq!(alice_shared.as_bytes(), bob_shared.as_bytes());
}
