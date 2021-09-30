use crate::{common::SakResult, err_res};
use k256::{
    ecdsa::{
        signature::{Signer, Verifier},
        Signature, SigningKey, VerifyingKey,
    },
    SecretKey,
};
use rand_core::OsRng;
use tokio::{io::AsyncReadExt, net::TcpStream};
use std::convert::{TryInto};

const SAKSAHA: &[u8; 7] = b"saksaha";

pub struct WhoAreYou {
    // signature,
    p2p_port: usize,
}

pub struct WhoAreYouAck {}

impl WhoAreYou {
    pub fn create(signing_key: SigningKey) -> Vec<u8> {
        let buf = [0; 1024];

        // let signing_key = SigningKey::random(&mut OsRng); // Serialize with `::to_bytes()`

        let message = SAKSAHA;
        let signature: Signature = signing_key.sign(message);
        let a = signature.to_der().to_bytes().to_vec();

        println!("44, {}", a.len());

        // let b: [u8; 1024] = a.to_vec().try_into()
        //     .unwrap();

        // return &b[0..245];

        // let signature2 = Signature::from_der(&a).unwrap();
        // let verify_key = VerifyingKey::from(&signing_key);
        // let b = verify_key.verify(message, &signature).unwrap();

        // let c = verify_key.verify(message, &signature2).unwrap();

        // println!("22, {:?}", b);

        // Signature::from_scalars(r, s)

        // let verify_key = VerifyingKey::from(&signing_key);
        // verify_key.verify(message,)

        a
    }

    pub async fn parse(stream: &mut TcpStream) -> SakResult<WhoAreYou> {
        let mut buf = vec![0; 1024];

        loop {
            let n = match stream.read(&mut buf).await {
                Ok(n) => n,
                Err(err) => {
                    return err_res!(
                        "Error parsing `who_are_you` request`, err: {}",
                        err
                    );
                }
            };

            if n == 0 {
                let w = WhoAreYou { p2p_port: 0 };

                println!("55: {:?}", buf);
                return Ok(w);
            }
        }
    }
}

pub async fn receive() {
    // [32, 31, 23, 14, 41, 23, 41, 41, 32];
}

pub async fn initiate() {
    // [32, 31, 23, 14, 41, 23, 41, 41, 32];
}
