use crate::{common::Result, crypto::Crypto, err};
use k256::ecdsa::Signature;
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
    pub public_key_bytes: [u8; 65],
    pub peer_op_port: u16,
}

impl WhoAreYou {
    pub fn new(
        sig: Signature,
        peer_op_port: u16,
        public_key_bytes: [u8; 65],
    ) -> WhoAreYou {
        WhoAreYou {
            sig,
            peer_op_port,
            public_key_bytes,
        }
    }

    pub fn to_bytes(&self) -> Result<[u8; 140]> {
        let mut buf = [0; 140];

        buf[0] = Type::SYN as u8;

        let sig_bytes = self.sig.to_der().to_bytes();
        let sig_len = sig_bytes.len();

        if sig_len == 71 {
            buf[1..72].copy_from_slice(&sig_bytes);
        } else {
            return err!(
                "Signature does not fit the size, len: {}",
                sig_len
            );
        }

        buf[72..74].copy_from_slice(&self.peer_op_port.to_be_bytes());

        buf[74..139].copy_from_slice(&self.public_key_bytes);

        Ok(buf)
    }

    pub fn get_peer_id(&self) -> String {
        Crypto::encode_hex(&self.public_key_bytes)
    }

    pub async fn parse(stream: &mut TcpStream) -> Result<WhoAreYou> {
        let mut buf = [0; 140];

        match stream.read_exact(&mut buf).await {
            Ok(b) => b,
            Err(err) => {
                return err!("Error reading whoAreYou, err: {}", err);
            }
        };

        let sig: Signature = match buf[1..72].try_into() {
            Ok(b) => match Signature::from_der(b) {
                Ok(s) => s,
                Err(err) => {
                    return err!(
                        "Error recovering signature, err: {}",
                        err
                    );
                }
            },
            Err(err) => {
                return err!("Error parsing signature, err: {}", err);
            }
        };

        let peer_op_port: u16 = match buf[72..74].try_into() {
            Ok(p) => u16::from_be_bytes(p),
            Err(err) => {
                return err!("Error parsing peer_op_port, err: {}", err);
            }
        };

        let mut public_key_bytes = [0; 65];
        public_key_bytes.copy_from_slice(&buf[74..139]);

        let way = WhoAreYou {
            sig,
            peer_op_port,
            public_key_bytes,
        };

        Ok(way)
    }
}

pub struct WhoAreYouAck {
    pub way: WhoAreYou,
}

impl WhoAreYouAck {
    pub fn new(
        sig: Signature,
        peer_op_port: u16,
        public_key_bytes: [u8; 65],
    ) -> WhoAreYouAck {
        let way = WhoAreYou::new(sig, peer_op_port, public_key_bytes);

        WhoAreYouAck { way }
    }

    pub fn to_bytes(&self) -> Result<[u8; 140]> {
        return self.way.to_bytes();
    }

    pub async fn parse(stream: &mut TcpStream) -> Result<WhoAreYouAck> {
        let way = match WhoAreYou::parse(stream).await {
            Ok(w) => w,
            Err(err) => {
                return err!("Error parsing WhoAreYouAck, err: {}", err);
            }
        };

        let way_ack = WhoAreYouAck { way };

        Ok(way_ack)
    }
}
