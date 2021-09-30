use crate::{common::SakResult, err_res};
use k256::{
    ecdsa::{
        signature::{Signer, Verifier},
        Signature, SigningKey, VerifyingKey,
    },
};
use tokio::{io::AsyncReadExt, net::TcpStream};

const MESSAGE: &[u8; 7] = b"saksaha";

#[derive(Copy, Clone)]
pub enum Type {
    SYN = 0x0,
    ACK,
}

pub struct WhoAreYou {
    signing_key: SigningKey,
    disc_port: u16,
    peer_op_port: u16,
}

pub struct WhoAreYouAck;

impl WhoAreYou {
    pub fn to_bytes(&self) -> SakResult<[u8; 128]> {
        let mut buf = [0; 128];

        buf[0] = Type::SYN as u8;

        let sig: Signature = self.signing_key.sign(MESSAGE);
        let sig_bytes = sig.to_der().to_bytes();
        let len = sig_bytes.len();

        if len == 70 {
            buf[1..71].copy_from_slice(&sig_bytes);
        } else {
            return err_res!("Signature does not fit the size, len: {}", len);
        }

        buf[71..73].copy_from_slice(&self.disc_port.to_be_bytes());
        buf[72..74].copy_from_slice(&self.peer_op_port.to_be_bytes());

        Ok(buf)
    }

    pub async fn parse(stream: &mut TcpStream) -> SakResult<WhoAreYou> {
        let mut buf = [0; 128];

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
                break;
            }
        };

        let signing_key = '';
        let disc_port = '';
        let peer_op_port = '';

        let way = WhoAreYou {

        };

        Ok(way)
    }
}
