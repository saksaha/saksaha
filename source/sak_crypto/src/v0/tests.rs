#[cfg(test)]
mod test {
    pub type PublicKey = k256::PublicKey;
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
    fn a() {
        // alice
        let alice_secret = {
            let alice_secret_bytes = {
                let secret = String::from(
                    "7297b903877a957748b74068d63d6d5661481975240\
                    99fc1df5cd9e8814c66c7",
                );

                crate::decode_hex(&secret).unwrap()
            };

            let s = SecretKey::from_bytes(alice_secret_bytes).unwrap();
            s
            // let encoded_point = EncodedPoint::from(s.public_key());
            // encoded_point
        };

        let bob_secret = {
            let secret_bytes = {
                let secret = String::from("0");

                crate::decode_hex(&secret).unwrap()
            };

            let s = SecretKey::from_bytes(secret_bytes).unwrap();
            s
            // let encoded_point = EncodedPoint::from(s.public_key());
            // encoded_point
        };

        // let a = EncodedPoint::from(bob_secret.public_key());
        let p = bob_secret.public_key();

        // shared_secret
        let ss = k256::elliptic_curve::ecdh::diffie_hellman(
            alice_secret.to_secret_scalar(),
            p.as_affine(),
        );

    }
}
