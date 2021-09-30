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

const SAKSAHA: &[u8; 7] = b"saksaha";

pub struct WhoAreYou {
    // signature,
    p2p_port: usize,
}

pub struct WhoAreYouAck {}

impl WhoAreYou {
    pub fn create() -> [u8; 1024] {
        let buf = [0; 1024];

        let signing_key = SigningKey::random(&mut OsRng); // Serialize with `::to_bytes()`
        let message = SAKSAHA;

        // let message = b"ECDSA proves knowled";

        // Note: the signature type must be annotated or otherwise inferrable as
        // `Signer` has many impls of the `Signer` trait (for both regular and
        // recoverable signature types).
        let signature: Signature = signing_key.sign(message);
        let verify_key = VerifyingKey::from(&signing_key);
        let b = verify_key.verify(message, &signature);

        // signature.
        let a = signature.to_der().to_bytes();

        let signature2 = Signature::from_der(&a).unwrap();
        let verify_key = VerifyingKey::from(&signing_key);
        let b = verify_key.verify(message, &signature).unwrap();

        let c = verify_key.verify(message, &signature2).unwrap();

        println!("22");

        // Signature::from_scalars(r, s)

        // let verify_key = VerifyingKey::from(&signing_key);
        // verify_key.verify(message,)

        buf
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
