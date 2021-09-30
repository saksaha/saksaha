use crate::{common::SakResult, err_res};
use k256::{
    ecdsa::{
        signature::{Signer, Verifier},
        Signature, SigningKey, VerifyingKey,
    },
    SecretKey,
};
use rand_core::OsRng;
use std::convert::TryInto;
use tokio::{io::AsyncReadExt, net::TcpStream};

const MESSAGE: &[u8; 7] = b"saksaha";

#[derive(Copy, Clone)]
pub enum Type {
    SYN = 0x0,
    ACK,
}

pub struct WhoAreYou;

pub struct WhoAreYouAck;

impl WhoAreYou {
    pub fn create(
        signing_key: SigningKey,
        disc_port: u16,
        peer_op_port: u16,
    ) -> SakResult<[u8; 128]> {
        let mut buf = [0; 128];

        buf[0] = Type::SYN as u8;

        let sig: Signature = signing_key.sign(MESSAGE);
        let sig_bytes = sig.to_der().to_bytes();
        let len = sig_bytes.len();

        if len == 70 {
            buf[1..71].copy_from_slice(&sig_bytes);
        } else {
            return err_res!("Signature does not fit the size, len: {}", len);
        }

        buf[71..73].copy_from_slice(&disc_port.to_be_bytes());
        buf[72..74].copy_from_slice(&peer_op_port.to_be_bytes());


        Ok(buf)

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
    }

    pub async fn parse(stream: &mut TcpStream) -> SakResult<WhoAreYou> {
        // let mut buf = vec![0; 1024];
        let mut buf = [0; 256];

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
                let w = WhoAreYou {};

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
