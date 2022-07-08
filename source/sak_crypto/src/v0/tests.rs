#[cfg(test)]
mod test {
    use crate::{aes_encrypt, derive_aes_key, PublicKey};
    use base64ct::{Base64, Encoding};
    use k256::SecretKey;
    use k256::{
        ecdh::EphemeralSecret,
        ecdsa::{
            signature::{Signer, Verifier},
            Signature, SigningKey, VerifyingKey,
        },
        elliptic_curve::sec1::ToEncodedPoint,
        EncodedPoint,
    };
    use k256::{elliptic_curve::ecdh::SharedSecret, Secp256k1};
    use rand_core::OsRng;
    use sha3::{Digest, Sha3_256};
    use std::{fmt::Write, num::ParseIntError};

    pub fn init() {
        const RUST_LOG_ENV: &str = "
            sak_,
            saksaha
        ";

        if std::env::var("RUST_LOG").is_err() {
            std::env::set_var("RUST_LOG", RUST_LOG_ENV);
        }

        sak_logger::init(false);
    }

    #[test]
    fn it_creates_signature() {
        let signing_key = SigningKey::random(&mut OsRng); // Serialize with `::to_bytes()`
        let message = b"ECDSA proves knowledge of a secret number in the context of a single message";

        let signature: Signature = signing_key.sign(message);
        let verify_key = VerifyingKey::from(&signing_key); // Serialize with `::to_encoded_point()`

        assert!(verify_key.verify(message, &signature).is_ok());
    }

    #[test]
    fn it_creates_shared_secret() {
        // Alice
        let alice_secret = EphemeralSecret::random(&mut OsRng);
        let alice_pk_bytes = EncodedPoint::from(alice_secret.public_key());

        let sk = SecretKey::random(&mut OsRng);
        let sk_bytes = sk.to_bytes();

        print!("secret key: {:?}", sk);
        print!("secret key bytes: {:?}", alice_pk_bytes);

        let sk_rec = SecretKey::from_bytes(sk_bytes).unwrap();
        print!("recovered secret key {:?}", sk_rec);

        let public_key = sk.public_key();
        let enc_point = public_key.to_encoded_point(false);
        let enc_point_bytes = enc_point.as_bytes();

        print!("public key: {:?}", public_key);
        print!("encoded point: {:?}", enc_point);
        print!("encoded point as bytes: {:?}", enc_point_bytes);

        let public_key_rec =
            PublicKey::from_sec1_bytes(enc_point_bytes).unwrap();
        let enc_point_rec = public_key_rec.to_encoded_point(false);

        print!("public key rec: {:?}", public_key_rec);
        print!("encoded point rec: {:?}", &enc_point_rec);

        assert_eq!(enc_point, enc_point_rec);

        // k256::elliptic_curve::ecdh::diffie_hellman(
        //     // public_key.to_secret_scalar(),
        //     // her_public.as_affine(),
        // );
    }

    #[test]
    fn test_ecies_variant() {
        init();

        sak_test_utils::init_test_config(&vec!["test".to_string()]).unwrap();

        let (bob_pk, bob_sk) = {
            let sk = crate::generate_key();
            let pk = sk.public_key();
            (pk, sk)
        };

        // alice is the sender of the message
        let (e_pk, e_sk) = {
            let sk = crate::generate_key();
            let pk = sk.public_key();

            (pk, sk)
        };

        let msg = "hello";
        println!("msg: {}", msg);

        let aes_key = derive_aes_key(e_sk, bob_pk);
        println!("aes_key: {:?}", aes_key);

        let cipher_text = aes_encrypt(&aes_key, msg.as_bytes()).unwrap();
        println!("cipher_text: {:?}", cipher_text);

        let mut msg_to_send = Vec::new();

        msg_to_send.extend_from_slice(&e_pk.to_encoded_point(false).to_bytes());
        msg_to_send.extend(cipher_text);

        println!("msg_to_send: {:?}", msg_to_send);
    }
}
