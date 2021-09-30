use crate::{common::SakResult, err_res, p2p::peer_op};
use k256::ecdsa::{
    signature::{Signer, Verifier},
    Signature, SigningKey, VerifyingKey,
};
use std::convert::TryInto;
use tokio::{io::AsyncReadExt, net::TcpStream};

pub const MESSAGE: &[u8; 7] = b"saksaha";

#[derive(Copy, Clone)]
pub enum Type {
    SYN = 0x0,
    ACK,
}

pub struct WhoAreYou {
    pub sig: Signature,
    pub peer_op_port: u16,
}

impl WhoAreYou {
    pub fn new(sig: Signature, peer_op_port: u16) -> WhoAreYou {
        WhoAreYou { sig, peer_op_port }
    }

    pub fn to_bytes(&self) -> SakResult<[u8; 128]> {
        let mut buf = [0; 128];

        buf[0] = Type::SYN as u8;

        let sig_bytes = self.sig.to_der().to_bytes();
        let sig_len = sig_bytes.len();

        if sig_len == 70 {
            buf[1..71].copy_from_slice(&sig_bytes);
        } else {
            return err_res!(
                "Signature does not fit the size, len: {}",
                sig_len
            );
        }

        buf[71..73].copy_from_slice(&self.peer_op_port.to_be_bytes());

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
        }

        let sig: Signature = match buf[1..71].try_into() {
            Ok(b) => match Signature::from_der(b) {
                Ok(s) => s,
                Err(err) => {
                    return err_res!(
                        "Error recovering signature, err: {}",
                        err
                    );
                }
            },
            Err(err) => {
                return err_res!("Error parsing signature, err: {}", err);
            }
        };

        let peer_op_port: u16 = match buf[71..73].try_into() {
            Ok(p) => u16::from_be_bytes(p),
            Err(err) => {
                return err_res!("Error parsing peer_op_port");
            }
        };

        let way = WhoAreYou { sig, peer_op_port };

        Ok(way)
    }
}

pub struct WhoAreYouAck {
    way: WhoAreYou,
}

impl WhoAreYouAck {
    pub fn new(sig: Signature, peer_op_port: u16) -> WhoAreYouAck {
        let way = WhoAreYou::new(sig, peer_op_port);

        WhoAreYouAck {
            way,
        }
    }

    pub fn to_bytes(&self) -> SakResult<[u8; 128]> {
        return self.way.to_bytes();
    }

    pub async fn parse(stream: &mut TcpStream) -> SakResult<WhoAreYouAck> {
        let mut buf = [0; 128];

        match stream.read(&mut buf).await {
            Ok(b) => b,
            Err(err) => {
                return err_res!("Error reading whoAreYouAck, err: {}", err);
            }
        };

        // println!("22, {:?}", buf);

        let way_ack = WhoAreYouAck {};

        Ok(waya)
    }
}
